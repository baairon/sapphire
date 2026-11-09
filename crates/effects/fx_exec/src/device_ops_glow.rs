//! Device kernel of `glow` (`kernels2::glow_k`, crate `glow_layer`): copy of the source planes, the bright pass on the device, then,
//! unless the threshold display is on, the separable blur of every plane with the glow pass count.
//!
//! Host side: the operand lanes are read as `glow_k` reads them, and the sigmas (`glow_layer::pass_args`) and the bright pass scale
//! (`glow_layer::bright::scale`) are computed by the CPU crate's own functions. Device side: `glow_bright1_f32` or `glow_bright3_f32`
//! (`fx_device::kernels_glow`) in place on the copied planes, then `device_ops_neighbourhood::blur_dev` per plane (the CPU's
//! `blur::blur_core` with the same driver plan, pass order, sum grouping and alignment replay) into fresh output planes.
//! An edge mode the CPU refuses, or a blur call the blur kernels decline, returns `DevError::Unsupported` and the executor runs
//! the node on the CPU.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::device_ops_neighbourhood::{blur_dev, const_n, cst, edge_of, frame_ok, img, is_const, lane, plane_channels, render_scale, BlurJob};
use crate::kernels2::cvt_i32;
use crate::types::{Image, KParams};
use fx_device::cl::Arg as ClArg;
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use glow_layer::{GlowParams, GLOW_PASSES};
use std::sync::Arc;

pub const GLOW: DeviceOp = DeviceOp { class: "glow", accepts: glow_accepts, run: glow_run, corpus: glow_corpus };

/// The classes of this module that may run on a device.
pub const ALLOW: &[&str] = &["glow"];

fn unsup<T>(why: &str) -> Result<T, DevError> {
    Err(DevError::Unsupported(why.to_string()))
}

fn glow_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 8 {
        return Err("eight operands expected");
    }
    if c.tag != 1 && c.tag != 3 {
        return Err("the combination tag is not 1 or 3");
    }
    match plane_channels(&a[0]) {
        Some(ch) if ch >= c.tag as usize => {}
        Some(_) => return Err("fewer channels than the combination tag"),
        None => return Err("the source is not a plain image"),
    }
    if !a[1..].iter().all(is_const) {
        return Err("thresholds, stretch, widths, brightness, subpixel, edge mode and threshold display must be constants");
    }
    Ok(())
}

/// The render parameters exactly as `glow_k` builds them: a one lane constant is replicated, otherwise lane i; stretch lane 1 falls
/// back to lane 0 for a one lane constant; subpixel, edge mode and threshold display are `cvt_i32` of lane 0.
fn glow_params(c: &NodeCall, a: &[DevArg]) -> Result<GlowParams, DevError> {
    let planes = c.tag.max(0) as usize;
    let lanes = |x: &DevArg| -> Result<[f32; 3], DevError> {
        let mut v = [0.0f32; 3];
        for (i, l) in v.iter_mut().enumerate().take(if planes == 1 { 1 } else { 3 }) {
            *l = lane(x, if const_n(x) == 1 { 0 } else { i })?;
        }
        Ok(v)
    };
    Ok(GlowParams {
        thresholds: lanes(&a[1])?,
        width_xy: [lane(&a[2], 0)?, lane(&a[2], if const_n(&a[2]) == 1 { 0 } else { 1 })?],
        width: lanes(&a[3])?,
        brightness: lanes(&a[4])?,
        subpixel: cvt_i32(lane(&a[5], 0)?),
        edge_mode: cvt_i32(lane(&a[6], 0)?),
        show_threshold: cvt_i32(lane(&a[7], 0)?),
        plane_count: planes,
        render_scale: render_scale(c.kp, c.dims.0),
        aspect: c.kp.blur_aspect,
    })
}

fn glow_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    if a.len() != 8 || (c.tag != 1 && c.tag != 3) {
        return unsup("glow takes eight operands and the combination tag 1 or 3");
    }
    let (w, h) = c.dims;
    let n = c.n;
    if w.checked_mul(h) != Some(n) || n == 0 || n > i32::MAX as usize {
        return unsup("the frame size does not match the sample count");
    }
    let planes = c.tag as usize;
    let gp = glow_params(c, a)?;
    let pa = glow_layer::pass_args(&gp);
    let src_op = img(s, &a[0], n)?;
    let src = src_op.get();
    if src.n != n || src.channels() < planes {
        return unsup("source smaller than the combination tag or than the frame");
    }
    // copy: output plane p starts as source plane p (the CPU hands the first `planes` source planes to the pass)
    let work = DevImage::alloc(s, planes, n)?;
    for p in 0..planes {
        s.queue.copy(src.plane(p), work.plane(p))?;
    }
    // bright pass: the one plane call reads threshold lane 0 only; no scale means the planes stay as copied
    let th = if planes == 1 { [pa.thresholds[0], 0.0, 0.0] } else { pa.thresholds };
    if let Some(k) = glow_layer::bright::scale(&th, planes) {
        if planes == 1 {
            s.launch("glow_bright1_f32", n, &[work.plane(0).arg(), ClArg::U32(n as u32), ClArg::F32(th[0]), ClArg::F32(k)])?;
        } else {
            s.launch(
                "glow_bright3_f32",
                n,
                &[work.plane(0).arg(), work.plane(1).arg(), work.plane(2).arg(), ClArg::U32(n as u32), ClArg::F32(th[0]), ClArg::F32(th[1]), ClArg::F32(th[2]), ClArg::F32(k)],
            )?;
        }
    }
    if pa.show_threshold != 0 {
        return Ok(work);
    }
    let edge = edge_of(pa.edge_mode)?;
    let out = DevImage::alloc(s, planes, n)?;
    let scr = DevImage::alloc(s, 1, 3 * n)?;
    for p in 0..planes {
        let job = BlurJob { w, h, sx: pa.sigma_x[p], sy: pa.sigma_y[p], gain: pa.brightness[p], edge, passes: GLOW_PASSES, subpixel: pa.subpixel != 0, aspect: pa.aspect };
        blur_dev(s, &scr, work.plane(p), out.plane(p), &job)?;
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// self-check corpus

/// Frame shapes: widths 1, 2, 3, 7 and 45, rows and columns, and the golden frame sizes.
const DIMS_GLOW: &[(usize, usize)] = &[(1, 1), (2, 1), (3, 2), (7, 1), (1, 7), (16, 16), (33, 2), (45, 23), (64, 32), (50, 7), (129, 3)];

/// One parameter set: thresholds, stretch, widths, brightness (lanes in use), subpixel, edge mode, threshold display, render scale
/// override (0: the frame width) and aspect.
struct GlowSet {
    thresholds: &'static [f32],
    stretch: &'static [f32],
    widths: &'static [f32],
    brightness: &'static [f32],
    subpixel: f32,
    edge: f32,
    show: f32,
    rs: i32,
    aspect: Option<f32>,
}

const SUB: f32 = 1.0e-40;

const SETS: &[GlowSet] = &[
    // recorded style: equal threshold lanes, distinct widths, subpixel on
    GlowSet { thresholds: &[0.5], stretch: &[1.0, 1.0], widths: &[0.4, 0.5, 0.6], brightness: &[1.0, 1.0, 1.0], subpixel: 1.0, edge: 1.0, show: 0.0, rs: 0, aspect: None },
    // distinct thresholds (minss order), stretched, distinct brightness, zero edge
    GlowSet { thresholds: &[0.3, 0.6, 0.9], stretch: &[1.5, 1.0], widths: &[0.2, 0.3, 0.4], brightness: &[1.35, 1.0, 2.0], subpixel: 1.0, edge: 0.0, show: 0.0, rs: 0, aspect: None },
    // one threshold lane 0, one width lane 0 (scale or copy plan), brightness 0, subpixel off
    GlowSet { thresholds: &[0.0, 0.5, 0.7], stretch: &[0.7, 1.0], widths: &[0.0, 0.3, 0.5], brightness: &[0.0, 1.0, 1.35], subpixel: 0.0, edge: 1.0, show: 0.0, rs: 0, aspect: None },
    // every threshold 0: the pass leaves the planes as copied
    GlowSet { thresholds: &[0.0], stretch: &[1.0], widths: &[0.3], brightness: &[2.0], subpixel: 1.0, edge: 1.0, show: 0.0, rs: 0, aspect: None },
    // minimum 1.0: the 2^-7 floor of the scale; render scale override and aspect
    GlowSet { thresholds: &[1.0, 1.0, 1.0], stretch: &[1.0, 1.0], widths: &[0.5, 0.5, 0.5], brightness: &[1.0, 1.0, 1.0], subpixel: 1.0, edge: 0.0, show: 0.0, rs: 400, aspect: Some(0.75) },
    // minimum above 1, negative brightness
    GlowSet { thresholds: &[1.5, 2.0, 1.25], stretch: &[1.0, 1.0], widths: &[0.1, 0.2, 0.3], brightness: &[-1.0, 1.0, 0.5], subpixel: 0.0, edge: 1.0, show: 0.0, rs: 0, aspect: None },
    // negative minimum, aspect
    GlowSet { thresholds: &[-0.5, 0.2, 0.1], stretch: &[1.0, 0.5], widths: &[0.6, 0.3, 0.1], brightness: &[1.0, 1.0, 1.0], subpixel: 1.0, edge: 1.0, show: 0.0, rs: 0, aspect: Some(0.75) },
    // a NaN threshold lane (minss returns the second operand)
    GlowSet { thresholds: &[f32::NAN, 0.5, 0.4], stretch: &[1.0, 1.0], widths: &[0.3, 0.3, 0.3], brightness: &[1.0, 1.0, 1.0], subpixel: 1.0, edge: 0.0, show: 0.0, rs: 0, aspect: None },
    // threshold display: the bright layer is returned unblurred
    GlowSet { thresholds: &[0.55], stretch: &[1.0, 1.0], widths: &[0.3, 0.4, 0.5], brightness: &[1.0, 1.0, 1.0], subpixel: 1.0, edge: 1.0, show: 1.0, rs: 0, aspect: None },
    GlowSet { thresholds: &[0.35, 0.7, 0.5], stretch: &[1.0, 1.0], widths: &[0.3], brightness: &[1.35], subpixel: 1.0, edge: 0.0, show: 1.0, rs: 0, aspect: None },
    // a subnormal threshold, wide glow with the render scale override
    GlowSet { thresholds: &[SUB, 0.5, 0.25], stretch: &[1.0, 1.0], widths: &[0.04, 0.03, 0.02], brightness: &[1.0, 1.0, 1.0], subpixel: 0.0, edge: 0.0, show: 0.0, rs: 400, aspect: None },
    // threshold display with the threshold at the 2^-7 floor and a one lane stretch
    GlowSet { thresholds: &[0.9921875, 0.25, 0.5], stretch: &[0.7], widths: &[0.3], brightness: &[1.0], subpixel: 0.0, edge: 2.0, show: 1.0, rs: 0, aspect: None },
];

/// Specials: signed zeros, NaN of both signs (quiet and signalling payloads), infinities, subnormals, extremes.
const SPECIAL: [u32; 16] = [
    0x0000_0000, 0x8000_0000, 0x7fc0_0000, 0xffc0_0001, 0x7f80_0001, 0xff81_2345, 0x7f80_0000, 0xff80_0000, 0x0000_0001, 0x8000_0001, 0x007f_ffff, 0x7f7f_ffff,
    0xff7f_ffff, 0x3f80_0000, 0x3c00_0000, 0x0080_0000,
];

/// Values on and one ulp around every threshold lane, so each compare sees ties from both sides.
fn ties(t: &[f32]) -> Vec<f32> {
    let mut v = Vec::new();
    for &x in t {
        if x.is_finite() {
            v.extend([x, x.next_up(), x.next_down(), -x]);
        }
    }
    v.extend([0.0, -0.0, 1.0, 0.25]);
    v
}

/// Plane values: `plain` mixes moderate values in [-0.25, 1.25] with the ties (no NaN or infinity, so the blur sums stay finite);
/// otherwise ties, specials and the adversarial set are mixed in as well.
fn plane_values(n: usize, seed: u64, t: &[f32], plain: bool) -> Vec<f32> {
    let tv = ties(t);
    let adv = adversarial_f32(4000, 53);
    let mut s = 0x9e37_79b9_7f4a_7c15u64 ^ seed.wrapping_mul(0x1234_5678_9abc_def1);
    (0..n)
        .map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let r = (s >> 33) as usize;
            let moderate = ((s >> 40) as f32 / (1u64 << 24) as f32) * 1.5 - 0.25;
            match (plain, r % 8) {
                (_, 0) | (_, 1) => tv[(r >> 3) % tv.len()],
                (false, 2) => f32::from_bits(SPECIAL[(r >> 3) % SPECIAL.len()]),
                (false, 3) => adv[(r >> 3) % adv.len()],
                _ => moderate,
            }
        })
        .collect()
}

fn kp_of(set: &GlowSet) -> KParams {
    let mut kp = KParams::standin();
    kp.blur_render_scale = set.rs;
    kp.blur_aspect = set.aspect;
    kp
}

fn consts_of(set: &GlowSet) -> Vec<(usize, usize, [u32; 4])> {
    vec![cst(1, set.thresholds), cst(2, set.stretch), cst(3, set.widths), cst(4, set.brightness), cst(5, &[set.subpixel]), cst(6, &[set.edge]), cst(7, &[set.show])]
}

fn glow_corpus() -> Vec<SelfCase> {
    let mut out = Vec::new();
    for (di, &(w, h)) in DIMS_GLOW.iter().enumerate() {
        let n = w * h;
        for (si, set) in SETS.iter().enumerate() {
            if set.show == 0.0 && set.edge != 0.0 && set.edge != 1.0 {
                continue; // the CPU reports an error for this edge mode
            }
            // (tag, source channels): tag 1 from 1 and 2 planes, tag 3 from 3 and 4 planes
            for (vi, &(tag, ch)) in [(1usize, 1usize), (1, 2), (3, 3), (3, 4)].iter().enumerate() {
                if n > 1000 && (di + si + vi) % 2 == 1 {
                    continue;
                }
                let plain = (si + vi) % 2 == 0 || set.show == 0.0 && vi % 2 == 0;
                let seed = (di * 131 + si * 17 + vi) as u64;
                let src = Image { n, planes: (0..ch).map(|p| plane_values(n, seed * 7 + p as u64, set.thresholds, plain)).collect() };
                out.push(SelfCase {
                    name: format!("{w}x{h} set {si} tag {tag} from {ch} planes{}", if plain { "" } else { " specials" }),
                    dims: (w, h),
                    tag: tag as i32,
                    args: vec![src],
                    consts: consts_of(set),
                    kp: Some(kp_of(set)),
                });
            }
        }
    }
    // a constant source per tag (the CPU lays it out as constant planes)
    for &(w, h) in &[(3usize, 2usize), (16, 16), (45, 23)] {
        for (lanes, si) in [(&[0.7f32][..], 0usize), (&[0.9, 0.2, 0.6][..], 1), (&[0.9, 0.2, 0.6][..], 8)] {
            let set = &SETS[si];
            let mut consts = consts_of(set);
            consts.insert(0, cst(0, lanes));
            out.push(SelfCase { name: format!("{w}x{h} constant source {} set {si}", lanes.len()), dims: (w, h), tag: if lanes.len() == 1 { 1 } else { 3 }, args: vec![], consts, kp: Some(kp_of(set)) });
        }
    }
    out.extend(bright_corpus());
    out
}

/// The bright pass alone (threshold display on): thresholds equal to sample values and one ulp around them, NaN, infinities and
/// subnormals in every plane, tags 1 and 3, every threshold set. Used by the full corpus and by the bright pass test.
pub fn bright_corpus() -> Vec<SelfCase> {
    let mut out = Vec::new();
    let thresholds: &[&[f32]] = &[
        &[0.5],
        &[0.3, 0.6, 0.9],
        &[0.9, 0.6, 0.3],
        &[0.0, 0.5, 0.7],
        &[0.0],
        &[1.0, 1.0, 1.0],
        &[1.5, 2.0, 1.25],
        &[-0.5, 0.2, 0.1],
        &[f32::NAN, 0.5, 0.4],
        &[0.5, f32::NAN, 0.4],
        &[SUB, 0.5, 0.25],
        &[-0.0, 0.0, -0.0],
        &[f32::INFINITY, 0.5, 0.5],
        &[f32::NEG_INFINITY],
        &[0.9921875, 0.9921876, 0.25],
        &[1.0e-38, 3.0e38, 0.75],
    ];
    for &(w, h) in &[(1usize, 1usize), (7, 1), (16, 16), (45, 23), (257, 1)] {
        let n = w * h;
        for (ti, t) in thresholds.iter().enumerate() {
            for (vi, &(tag, ch)) in [(1usize, 1usize), (1, 2), (3, 3), (3, 4)].iter().enumerate() {
                let seed = (w * 7 + h * 3 + ti * 29 + vi) as u64 + 9000;
                let src = Image { n, planes: (0..ch).map(|p| plane_values(n, seed * 5 + p as u64, t, vi % 2 == 1 && ti % 3 == 0)).collect() };
                out.push(SelfCase {
                    name: format!("{w}x{h} bright pass thresholds {ti} tag {tag} from {ch} planes"),
                    dims: (w, h),
                    tag: tag as i32,
                    args: vec![src],
                    consts: vec![cst(1, t), cst(2, &[1.0, 1.0]), cst(3, &[0.3]), cst(4, &[1.0]), cst(5, &[1.0]), cst(6, &[7.0]), cst(7, &[1.0])],
                    kp: None,
                });
            }
        }
    }
    out
}
