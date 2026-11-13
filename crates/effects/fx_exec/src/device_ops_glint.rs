//! Device kernel of `glint` (`kernels2::glint_k`, crates `glint_wire`, `glint_dispatch`, `glint_worker`): the CPU's own dispatcher
//! and worker run on the host against a device host, so every Call, every host scalar (the double steps, the radii, the strengths)
//! and every pass order is the CPU's. Each Call is launched on the session's in-order queue:
//! the blur (`device_ops_neighbourhood::blur_dev`), the zero fill, the bright pass (`glow_bright3_f32`), the plane copy, the row and
//! column box passes (`device_ops_glint_passes`), the diagonal box sums (`device_ops_glint_diag::diag_box_sum_buf`) and the scaled
//! screen blend (`glint_blend_f32`).
//!
//! Value refusals (`glint_dispatch::refusal_of`) are decided before any allocation or launch; shape refusals are in `accepts`. Each
//! becomes `DevError::Unsupported` so the executor reruns the node on the CPU, which raises the named error.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::device_ops_glint_diag::diag_box_sum_buf;
use crate::device_ops_glint_passes::{glint_cols_dev, glint_rows_dev};
use crate::device_ops_neighbourhood::{blur_dev, const_n, cst, frame_ok, img, is_const, lane, plane_channels, render_scale, BlurJob};
use crate::types::{Image, KParams};
use diagonal_box_sum::{Diagonal, Params as BoxParams};
use fx_device::cl::{Arg as ClArg, Buffer};
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use glint_dispatch::{refusal_of, wrapper, Error as DispatchError, GlintArgs, Host as DispatchHost, Layer, Roi as WrapRoi, Scratch as WrapScratch, ThreeArgs};
use glint_wire::host::Align;
use glint_wire::op::{source_record, THREE_PLANE_LAYOUT};
use glint_worker::{Abort, BlockId, Call, CallbackArgs, Host, Input, Operand, Pass, PassOp, PlaneId, Roi, Scratch, SourceRecord};
use std::collections::HashMap;
use std::sync::Arc;

pub const GLINT: DeviceOp = DeviceOp { class: "glint", accepts: glint_accepts, run: glint_run, corpus: glint_corpus };

/// The classes of this module that may run on a device.
pub const ALLOW: &[&str] = &["glint"];

fn unsup<T>(why: &str) -> Result<T, DevError> {
    Err(DevError::Unsupported(why.to_string()))
}

fn glint_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if c.n.checked_mul(4).map_or(true, |b| b > i32::MAX as usize) {
        return Err("the frame is over the 31 bit byte range of the source record");
    }
    if a.len() != 7 {
        return Err("seven operands expected");
    }
    if c.tag != 3 {
        return Err("the combination tag is not 3");
    }
    match plane_channels(&a[0]) {
        Some(ch) if ch >= 3 => {}
        Some(_) => return Err("fewer than three source planes"),
        None => return Err("the source is not a plain image"),
    }
    if !a[1..].iter().all(is_const) {
        return Err("thresholds, strength, A, B, T0 and T1 must be constants");
    }
    if c.kp.glint_align.blur != 0 {
        return Err("a nonzero blur phase is not replayed on the device");
    }
    Ok(())
}

/// Lane `i` of a constant operand as `glint_lanes` reads it: a one lane constant is replicated.
fn lanes<const N: usize>(x: &DevArg) -> Result<[f32; N], DevError> {
    let mut v = [0.0f32; N];
    for (i, l) in v.iter_mut().enumerate() {
        *l = lane(x, if const_n(x) == 1 { 0 } else { i })?;
    }
    Ok(v)
}

/// Byte phase (mod 32) of ring row `i` of the last scratch block, as `ComposedHost::row_phase`.
fn row_phase(ring_phase: usize, last_alloc: (u64, u32), i: usize) -> u8 {
    let stride = ((last_alloc.0 as usize) + 15) & !15;
    ((ring_phase + i * stride) % 32) as u8
}

struct DevHost<'a> {
    s: &'a Arc<Session>,
    planes: HashMap<PlaneId, &'a Buffer<f32>>,
    scr: &'a DevImage,
    th: [f32; 3],
    align: Align,
    last_alloc: (u64, u32),
    refused: Option<&'static str>,
    err: Option<String>,
    dev_err: Option<DevError>,
}

impl DevHost<'_> {
    fn buf(&self, id: PlaneId) -> Result<&Buffer<f32>, DevError> {
        match self.planes.get(&id) {
            Some(b) => Ok(*b),
            None => unsup("a plane the device host does not hold"),
        }
    }
    fn fail(&mut self, what: String) {
        if self.err.is_none() {
            self.err = Some(what);
        }
    }
    fn route(&mut self, c: &Call, ctx: &CallbackArgs) -> Result<(), DevError> {
        match c {
            Call::Blur { blurred, source, blur_x, roi, flagged, .. } => {
                if *flagged {
                    self.refused = Some(glint_wire::refusal::BLUR_FLAG);
                    return Ok(());
                }
                let Some(source) = source else { return unsup("a blur without a source plane") };
                let x = f32::from_bits(*blur_x);
                let job = BlurJob { w: roi.w as usize, h: roi.h as usize, sx: x, sy: x, gain: 1.0, edge: 1, passes: 3, subpixel: true, aspect: None };
                blur_dev(self.s, self.scr, self.buf(*source)?, self.buf(*blurred)?, &job)
            }
            Call::Fill { output, flag, roi } => {
                if *flag != 0 {
                    self.refused = Some(glint_wire::refusal::BLUR_FLAG);
                    return Ok(());
                }
                let b = self.buf(*output)?;
                let n = (roi.c as i64).checked_mul(roi.h as i64);
                if n != Some(b.len() as i64) {
                    return unsup("partial fill");
                }
                self.s.queue.fill(b, 0.0f32)?;
                Ok(())
            }
            Call::BrightPass { flag, roi } => {
                if *flag != 0 {
                    self.refused = Some(glint_wire::refusal::BLUR_FLAG);
                    return Ok(());
                }
                let th = self.th;
                let Some(k) = glow_layer::bright::scale(&th, 3) else { return Ok(()) };
                let n = (roi.w as usize) * (roi.h as usize);
                let (p0, p1, p2) = (self.buf(PlaneId(11))?, self.buf(PlaneId(12))?, self.buf(PlaneId(13))?);
                self.s.launch("glow_bright3_f32", n, &[p0.arg(), p1.arg(), p2.arg(), ClArg::U32(n as u32), ClArg::F32(th[0]), ClArg::F32(th[1]), ClArg::F32(th[2]), ClArg::F32(k)])?;
                Ok(())
            }
            Call::CheckAbort(_) | Call::Free(_) | Call::Alloc { .. } => Ok(()),
            Call::CopyPlane { layer, plane, bytes } => {
                let (src, dst) = (self.buf(*plane)?, self.buf(*layer)?);
                if (*bytes / 4) as usize != src.len() || src.len() != dst.len() {
                    return unsup("partial copy");
                }
                self.s.queue.copy(src, dst)?;
                Ok(())
            }
            Call::Parallel { op, radii } => self.parallel(*op, *radii, ctx),
        }
    }

    fn parallel(&mut self, op: PassOp, radii: Option<[i32; 3]>, ctx: &CallbackArgs) -> Result<(), DevError> {
        let (w, h) = (ctx.width as usize, ctx.height as usize);
        let plane = |o: Operand| -> Result<PlaneId, DevError> {
            match o {
                Operand::Plane(p) => Ok(p),
                _ => unsup("an operand that is not a plane"),
            }
        };
        match op {
            PassOp::Scale => {
                let (dst, src) = (self.buf(plane(ctx.scale_dest)?)?, self.buf(plane(ctx.scale_source)?)?);
                let n = ctx.element_count as usize;
                if n != dst.len() || n != src.len() {
                    return unsup("a blend over part of a plane");
                }
                let scale = f32::from_bits(ctx.scale_bits);
                self.s.launch("glint_blend_f32", n, &[dst.arg(), src.arg(), ClArg::U32(n as u32), ClArg::F32(scale), ClArg::I32((scale == 1.0) as i32)])?;
                Ok(())
            }
            PassOp::Horizontal | PassOp::Vertical | PassOp::DiagonalBody(_) | PassOp::DiagonalBox(_) if glint_wire::refusal::shift_out_of_range(ctx.shift as i32) => {
                self.fail(glint_wire::refusal::SHIFT.into());
                Ok(())
            }
            PassOp::Horizontal | PassOp::Vertical => {
                let Some(r) = radii else { return unsup("a pass without radii") };
                let (src, dst) = (self.buf(plane(ctx.source)?)?, self.buf(plane(ctx.dest)?)?);
                let (edge, shift) = (ctx.edge_mode as i32, ctx.shift as i32);
                let scratch = [row_phase(self.align.boxsum, self.last_alloc, 0), row_phase(self.align.boxsum, self.last_alloc, 1)];
                let res = if op == PassOp::Horizontal {
                    let cfg = glint_pass_rows::Config { edge, radii: &r, shift };
                    let ph = glint_pass_rows::Alignment { scratch, radii: self.align.radii_rows };
                    glint_rows_dev(self.s, &cfg, &ph, src, dst, w, h)?.map_err(|e| format!("pass0 {e:?}"))
                } else {
                    let cfg = glint_pass_cols::pass_b::Config { edge, radii: &r, shift };
                    let ph = glint_pass_cols::pass_b::Alignment { scratch, radii: self.align.radii_cols };
                    glint_cols_dev(self.s, &cfg, &ph, src, dst, w, h)?.map_err(|e| format!("pass1 {e:?}"))
                };
                if let Err(e) = res {
                    self.fail(e);
                }
                Ok(())
            }
            PassOp::DiagonalBody(_) => unsup("diagonal body route: not served (diagonal mode 0)"),
            PassOp::DiagonalBox(which) => {
                let Some(r) = radii else { return unsup("a box pass without radii") };
                let img = self.buf(plane(ctx.dest)?)?;
                let (kind, name) = if which == Pass::MainDiagonal { (Diagonal::Main, "boxMain") } else { (Diagonal::Anti, "boxAnti") };
                let mut phases = [0u8; 5];
                for (i, p) in phases.iter_mut().enumerate() {
                    *p = row_phase(self.align.boxsum, self.last_alloc, i);
                }
                let p = BoxParams { w: w as i32, h: h as i32, radii: r, shift: ctx.shift as i32, phases, start: 0, count: w as i32 + h as i32 - 1, kind };
                if let Err(e) = diag_box_sum_buf(self.s, img, &p)? {
                    self.fail(format!("{name} {e:?}"));
                }
                Ok(())
            }
        }
    }
}

impl Host for DevHost<'_> {
    fn call(&mut self, c: &Call, args: &CallbackArgs) {
        if self.refused.is_some() || self.err.is_some() || self.dev_err.is_some() {
            return;
        }
        if let Err(e) = self.route(c, args) {
            self.dev_err = Some(e);
        }
    }
    fn alloc(&mut self, _ring: Scratch, bytes: u64, slots: u32) -> Option<BlockId> {
        self.last_alloc = (bytes, slots);
        None
    }
    fn abort(&mut self, _which: Abort) -> i32 {
        0
    }
}

struct DevWire<'a> {
    record: SourceRecord,
    host: DevHost<'a>,
    dsts: [PlaneId; 3],
    err: Option<String>,
}

fn bits3(v: [f32; 3]) -> [u32; 3] {
    v.map(f32::to_bits)
}
fn bits4(v: [f32; 4]) -> [u32; 4] {
    v.map(f32::to_bits)
}

impl DispatchHost for DevWire<'_> {
    fn layout(&mut self) -> i32 {
        THREE_PLANE_LAYOUT
    }
    fn region(&mut self, s: &SourceRecord) -> WrapRoi {
        WrapRoi { w: s.width, h: s.height, c: s.width, d: 0, e: 4 }
    }
    fn copy_plane(&mut self, _dst: PlaneId, _src: PlaneId, _row: i32, _roi: &WrapRoi) {
        self.err.get_or_insert_with(|| "glint: the plane copy detour is not taken by the op".to_string());
    }
    fn worker_three(&mut self, a: &ThreeArgs) {
        if a.saved_index.is_some() {
            self.err.get_or_insert_with(|| "glint: a saved plane is outside the op".to_string());
            return;
        }
        let inp = Input {
            source: self.record,
            blur_x: a.scale.to_bits(),
            blurred: a.blurred,
            sources: a.sources,
            outputs: self.dsts,
            layer: a.layer,
            saved: None,
            a: bits3(a.a),
            b: bits3(a.b),
            c: bits3(a.c),
            pass_scale: bits4(a.pass_scale),
            pass_gain: bits4(a.pass_gain),
            roi: Roi { w: a.roi.w, h: a.roi.h, c: a.roi.c, d: a.roi.d, e: a.roi.e },
            flag: a.flag as i32,
        };
        if let Err(r) = glint_worker::run(&inp, &mut self.host) {
            self.err.get_or_insert_with(|| r.name().to_string());
        }
    }
}

fn glint_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    if a.len() != 7 || c.tag != 3 {
        return unsup("glint takes seven operands and the combination tag 3");
    }
    let (w, h) = c.dims;
    let n = c.n;
    if w.checked_mul(h) != Some(n) || n == 0 || n.checked_mul(4).map_or(true, |b| b > i32::MAX as usize) {
        return unsup("the frame size does not match the sample count");
    }
    if c.kp.glint_align.blur != 0 {
        return unsup("a nonzero blur phase is not replayed on the device");
    }
    let th: [f32; 3] = lanes(&a[1])?;
    let args = GlintArgs { strength: lane(&a[2], 0)?, a: lanes(&a[3])?, b: lanes(&a[4])?, pass_scale: lanes(&a[5])?, pass_gain: lanes(&a[6])? };
    let record = match source_record(w, h, render_scale(c.kp, w)) {
        Ok(r) => r,
        Err(m) => return Err(DevError::Unsupported(m)),
    };
    // the one predicate for the value refusals, before any allocation or launch
    if let Some(r) = refusal_of(&record, &args, 0) {
        return Err(DevError::Unsupported(r.name().to_string()));
    }
    let src_op = img(s, &a[0], n)?;
    let src = src_op.get();
    if src.n != n || src.channels() < 3 {
        return unsup("the source must hold three planes of the frame");
    }
    let work = DevImage::alloc(s, 4, n)?;
    let out = DevImage::alloc(s, 3, n)?;
    let scr = DevImage::alloc(s, 1, 3 * n)?;
    for p in 0..4 {
        s.queue.fill(work.plane(p), 0.0f32)?;
    }
    for p in 0..3 {
        s.queue.fill(out.plane(p), 0.0f32)?;
    }
    let src_ids = [PlaneId(1), PlaneId(2), PlaneId(3)];
    let dsts = [PlaneId(21), PlaneId(22), PlaneId(23)];
    let mut planes: HashMap<PlaneId, &Buffer<f32>> = HashMap::new();
    for k in 0..3 {
        planes.insert(src_ids[k], src.plane(k));
        planes.insert(PlaneId(11 + k as u64), work.plane(1 + k));
        planes.insert(dsts[k], out.plane(k));
    }
    planes.insert(PlaneId(10), work.plane(0));
    let host = DevHost { s, planes, scr: &scr, th, align: c.kp.glint_align, last_alloc: (0, 0), refused: None, err: None, dev_err: None };
    let layer = Layer {
        planes: vec![PlaneId(10), PlaneId(11), PlaneId(12), PlaneId(13)],
        scratch: WrapScratch { kind: 0, src: src_ids },
        args,
    };
    let mut wire = DevWire { record, host, dsts, err: None };
    let res = wrapper(&mut wire, &layer, &record, 0);
    if let Some(e) = wire.host.dev_err.take() {
        return Err(e);
    }
    let guard = |m: String| -> Result<DevImage, DevError> { Err(DevError::Unsupported(m)) };
    match res {
        Err(DispatchError::Refused(r)) => return guard(r.name().to_string()),
        Err(DispatchError::LayoutRefused) => return guard(glint_wire::refusal::LAYOUT.to_string()),
        Err(e) => return guard(format!("glint: {e:?}")),
        Ok(()) => {}
    }
    if let Some(r) = wire.host.refused {
        return guard(r.to_string());
    }
    if let Some(e) = wire.host.err.take().or(wire.err.take()) {
        return guard(e);
    }
    drop(wire);
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// self-check corpus

/// Frame shapes: degenerate sizes, sizes around the 0x40 vector threshold, the recorded sizes, scratch phase 16 and 0, long rows and
/// columns, odd widths.
pub const FRAMES: &[(usize, usize)] = &[
    (1, 1), (2, 1), (1, 2), (2, 2), (3, 3), (7, 3), (8, 2), (9, 2), (63, 5), (64, 32), (65, 4), (5, 65), (66, 66), (50, 7), (68, 9), (9, 68),
    (100, 70), (72, 8), (1199, 2), (1200, 2), (1201, 2), (2, 1201), (7, 7), (9, 9), (33, 3), (65, 3), (257, 5), (5, 257),
];

/// One parameter set of the six constant operands and the host parameters.
pub struct GlintSet {
    pub th: &'static [f32],
    pub strength: f32,
    pub a: &'static [f32],
    pub b: &'static [f32],
    pub t0: &'static [f32],
    pub t1: &'static [f32],
    pub rs: i32,
}

const N: f32 = f32::NAN;

pub const SETS: &[GlintSet] = &[
    // equal thresholds, all passes on
    GlintSet { th: &[0.5], strength: 1.0, a: &[1.0], b: &[1.0], t0: &[1.0, 1.0, 1.0, 1.0], t1: &[0.25, 0.25, 0.25, 0.25], rs: 0 },
    // distinct thresholds, distinct widths and gates, mixed pass scales and gains
    GlintSet { th: &[0.3, 0.6, 0.9], strength: 0.5, a: &[0.5, 1.0, 2.0], b: &[0.25, 1.0, 1.0], t0: &[1.0, 0.5, 1.0, 0.5], t1: &[0.25, 0.0, 0.5, 1.0], rs: 0 },
    // all thresholds 0 (bright pass skipped), strength 3
    GlintSet { th: &[0.0], strength: 3.0, a: &[2.0], b: &[1.0], t0: &[1.0], t1: &[0.5], rs: 0 },
    // strength 0 (blur copy route), one threshold lane 0
    GlintSet { th: &[0.0, 0.5, 0.7], strength: 0.0, a: &[1.0], b: &[1.0], t0: &[1.0], t1: &[0.25], rs: 0 },
    // thresholds at and above 1 (the 2^-7 floor), render scale override
    GlintSet { th: &[1.0, 1.0, 1.0], strength: 0.5, a: &[1.0], b: &[1.0], t0: &[1.0, 1.0, 1.0, 1.0], t1: &[0.25], rs: 400 },
    GlintSet { th: &[1.5, 2.0, 1.25], strength: 1.0, a: &[0.5, 1.0, 2.0], b: &[1.0], t0: &[1.0], t1: &[0.5, 0.25, 0.125, 1.0], rs: 0 },
    // negative threshold and negative gate, A 0 (copy route for the line passes)
    GlintSet { th: &[-0.5, 0.2, 0.1], strength: 1.0, a: &[0.0], b: &[-1.0, 1.0, 1.0], t0: &[1.0], t1: &[0.25], rs: 0 },
    // NaN threshold, NaN A lane, NaN gate lane
    GlintSet { th: &[N, 0.5, 0.4], strength: 1.0, a: &[N, 1.0, 1.0], b: &[1.0, N, 1.0], t0: &[1.0], t1: &[0.25], rs: 0 },
    // each pass scale lane 0 in turn
    GlintSet { th: &[0.4], strength: 1.0, a: &[1.0], b: &[1.0], t0: &[0.0, 1.0, 1.0, 1.0], t1: &[0.25], rs: 0 },
    GlintSet { th: &[0.4], strength: 1.0, a: &[1.0], b: &[1.0], t0: &[1.0, 0.0, 1.0, 1.0], t1: &[0.25], rs: 0 },
    GlintSet { th: &[0.4], strength: 1.0, a: &[1.0], b: &[1.0], t0: &[1.0, 1.0, 0.0, 1.0], t1: &[0.25], rs: 0 },
    GlintSet { th: &[0.4], strength: 1.0, a: &[1.0], b: &[1.0], t0: &[1.0, 1.0, 1.0, 0.0], t1: &[0.25], rs: 0 },
    // a NaN pass scale lane
    GlintSet { th: &[0.4], strength: 1.0, a: &[1.0], b: &[1.0], t0: &[N, 1.0, N, 1.0], t1: &[0.25], rs: 0 },
    // the blend scale of exactly 1.0 (4 * 1 * 0.25 * 1), and strengths above 1
    GlintSet { th: &[0.3], strength: 1.0, a: &[1.0], b: &[1.0], t0: &[1.0, 1.0, 1.0, 1.0], t1: &[0.25, 0.25, 0.25, 0.25], rs: 400 },
    GlintSet { th: &[0.3], strength: 1.0, a: &[1.0], b: &[1.0], t0: &[1.0, 1.0, 1.0, 1.0], t1: &[4.0, 4.0, 4.0, 4.0], rs: 0 },
    // large A (radius at the clamp) and A in (0, 0.5) (radii 0)
    GlintSet { th: &[0.4], strength: 1.0, a: &[50.0, 200.0, 8.0], b: &[1.0], t0: &[1.0], t1: &[0.25], rs: 0 },
    GlintSet { th: &[0.4], strength: 1.0, a: &[0.05, 0.02, 0.01], b: &[1.0], t0: &[1.0], t1: &[0.25], rs: 0 },
    // a gain of 0 with pass scale 0.5 (the copy route of lines with a blend)
    GlintSet { th: &[0.2, 0.2, 0.2], strength: 2.0, a: &[1.0], b: &[0.5], t0: &[0.5, 0.5, 0.5, 0.5], t1: &[0.0, 0.0, 0.0, 0.0], rs: 0 },
    // a NaN strength lane is a decline (the blur plan); not in the corpus
];

const SPECIAL: [u32; 12] = [0x0000_0000, 0x8000_0000, 0x7fc0_0000, 0xffc0_0001, 0x7f80_0001, 0xff81_2345, 0x7f80_0000, 0xff80_0000, 0x0000_0001, 0x007f_ffff, 0x7f7f_ffff, 0xff7f_ffff];

fn ties(t: &[f32]) -> Vec<f32> {
    let mut v = Vec::new();
    for &x in t {
        if x.is_finite() {
            v.extend([x, x.next_up(), x.next_down(), -x]);
        }
    }
    v.extend([0.0, -0.0, 1.0, 0.25, 0.5, 0.9921875]);
    v
}

fn plane_values(n: usize, seed: u64, t: &[f32], plain: bool) -> Vec<f32> {
    let tv = ties(t);
    let adv = adversarial_f32(2000, 71);
    let mut s = 0x9e37_79b9_7f4a_7c15u64 ^ seed.wrapping_mul(0x1234_5678_9abc_def1);
    (0..n)
        .map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let r = (s >> 33) as usize;
            let moderate = ((s >> 40) as f32 / (1u64 << 24) as f32) * 1.5 - 0.25;
            match (plain, r % 16) {
                (_, 0) | (_, 1) => tv[(r >> 4) % tv.len()],
                (false, 2) => f32::from_bits(SPECIAL[(r >> 4) % SPECIAL.len()]),
                (false, 3) => adv[(r >> 4) % adv.len()],
                _ => moderate,
            }
        })
        .collect()
}

/// The host parameters of a case: render scale and the alignment phases.
fn kp_of(set: &GlintSet, variant: usize) -> KParams {
    let mut kp = KParams::standin();
    kp.blur_render_scale = set.rs;
    kp.glint_align = match variant % 6 {
        0 => Align::default(),
        1 => Align { boxsum: 4, ..Align::default() },
        2 => Align { boxsum: 16, ..Align::default() },
        3 => Align { boxsum: 28, ..Align::default() },
        4 => Align { radii_rows: 8, radii_cols: 8, ..Align::default() },
        _ => Align { boxsum: 12, radii_rows: 8, radii_cols: 8, blur: 0 },
    };
    kp
}

fn consts_of(set: &GlintSet) -> Vec<(usize, usize, [u32; 4])> {
    vec![cst(1, set.th), cst(2, &[set.strength]), cst(3, set.a), cst(4, set.b), cst(5, set.t0), cst(6, set.t1)]
}

/// The corpus: every parameter set at the two recorded sizes, a rotating window of sets at every other size.
pub fn glint_corpus() -> Vec<SelfCase> {
    let mut out = Vec::new();
    let mut counter = 0usize;
    for (fi, &(w, h)) in FRAMES.iter().enumerate() {
        let n = w * h;
        let recorded = (w, h) == (64, 32) || (w, h) == (50, 7);
        let long = w.max(h) >= 63;
        let picks: Vec<usize> = if recorded { (0..SETS.len()).collect() } else { (0..5).map(|j| (fi * 3 + j * 4) % SETS.len()).collect() };
        // frames with a line of 63 or more samples reach the vector routes of the line passes: sets 0 and 1 (all passes active) run at
        // every alignment variant, so the scratch phases of each pass change the sums' grouping
        let mut jobs: Vec<(usize, usize)> = picks.iter().map(|&si| (si, usize::MAX)).collect();
        if long && !recorded {
            for si in [0usize, 1] {
                for v in 0..6 {
                    jobs.push((si, v));
                }
            }
        }
        for (si, forced) in jobs {
            let set = &SETS[si];
            counter += 1;
            let ch = if counter % 2 == 0 { 3 } else { 4 };
            let plain = counter % 3 != 0;
            let seed = (fi * 131 + si * 17 + counter) as u64;
            let src = Image { n, planes: (0..ch).map(|p| plane_values(n, seed * 7 + p as u64, set.th, plain)).collect() };
            out.push(SelfCase {
                name: format!("{w}x{h} set {si} from {ch} planes{}", if plain { "" } else { " specials" }),
                dims: (w, h),
                tag: 3,
                args: vec![src],
                consts: consts_of(set),
                kp: Some(kp_of(set, if forced == usize::MAX { counter } else { forced })),
            });
        }
    }
    // a three lane constant source
    for &(w, h) in &[(7usize, 3usize), (64, 32), (50, 7)] {
        for si in [0usize, 1, 13] {
            let set = &SETS[si];
            let mut consts = consts_of(set);
            consts.insert(0, cst(0, &[0.9, 0.2, 0.6]));
            out.push(SelfCase { name: format!("{w}x{h} constant source set {si}"), dims: (w, h), tag: 3, args: vec![], consts, kp: Some(kp_of(set, si)) });
        }
    }
    out
}
