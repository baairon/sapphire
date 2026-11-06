//! Device ops of the sampling and warp family: warp_rel, warp, rotate, perspective, perspective_alpha.
//!
//! Kernels live in `fx_device::kernels_warp`. Each op here has `accepts` (shape level), `run` (value level: a constant that
//! puts the node outside the kernel's contract returns `DevError::Unsupported`, so the executor runs the CPU kernel) and a
//! self-check over [`cases`], run by [`selfcheck`] against `kernels::run_any`.
//!
//! Host side scalars (the sincos of a constant angle, perspective's constants, perspective_alpha's ramp scale) are computed
//! here with the same Rust expressions as the CPU kernels.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::kernels;
use crate::kernels2::{cvt_i32, footprint_taken, geom_of, Out};
use crate::types::{Arg, Geom, Image, KParams, MipImage};
use fx_device::cl::{Access, Arg as ClArg, Buffer};
use fx_device::selfcheck::{adversarial_f32, compare_f32, Mismatch, NanPayloads, Tally};
use fx_device::sim::Rng;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

/// The classes this module serves.
pub const CLASSES: &[&str] = &["warp_rel", "warp", "rotate", "perspective", "perspective_alpha"];

pub fn handles(class: &str) -> bool {
    CLASSES.contains(&class)
}

pub fn empty_corpus() -> Vec<SelfCase> {
    Vec::new()
}

fn unsup(m: &str) -> DevError {
    DevError::Unsupported(m.into())
}

fn lane(a: &DevArg, i: usize) -> Option<f32> {
    match a {
        DevArg::Const { lanes, .. } => Some(f32::from_bits(lanes[i.min(3)])),
        _ => None,
    }
}

/// A one word error cell (set by a kernel that met out of contract data) and its read back.
struct ErrCell {
    buf: Buffer<u32>,
}

impl ErrCell {
    fn new(s: &Session) -> Result<ErrCell, DevError> {
        Ok(ErrCell { buf: Buffer::from_slice(&s.context, &[0u32], Access::ReadWrite)? })
    }
    fn arg(&self) -> ClArg<'_> {
        self.buf.arg()
    }
    fn check(&self, s: &Session, what: &str) -> Result<(), DevError> {
        if s.queue.read(&self.buf)?[0] != 0 {
            return Err(unsup(what));
        }
        Ok(())
    }
}

fn dummy(s: &Session) -> Result<Buffer<f32>, DevError> {
    Ok(Buffer::from_slice(&s.context, &[0f32], Access::ReadOnly)?)
}

fn expect_args(shapes: &[ArgShape], n: usize) -> Result<(), &'static str> {
    if shapes.len() != n {
        return Err("unexpected operand count");
    }
    Ok(())
}

fn is_const(s: &ArgShape) -> bool {
    matches!(s, ArgShape::Const { .. })
}

// ---- warp_rel and warp --------------------------------------------------------------------------------------------

/// The footprint path needs the slot 33 prefill (the x86 NaN-model AVX2 body plus the double finish)
/// and the F1 multi-level lattice lookup. Neither is ported exactly yet, so the path runs on the CPU.
const FOOTPRINT_REASON: &str = "footprint path (needs the slot 33 prefill and the F1 multi-level lattice lookup, not yet ported exactly) runs on the CPU";

fn warp_accepts_common(c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    expect_args(shapes, 4)?;
    if !(1..=4).contains(&c.tag) {
        return Err("combination tag outside 1..4");
    }
    if shapes[0] != ArgShape::Mip {
        return Err("the first operand is not a ripmap");
    }
    match shapes[1] {
        ArgShape::Const { .. } | ArgShape::Img { channels: 2 } => {}
        _ => return Err("coordinate operand must be a constant or a 2 channel image"),
    }
    if !is_const(&shapes[2]) || !is_const(&shapes[3]) {
        return Err("wrap and flag operands must be constants");
    }
    Ok(())
}

pub fn warp_rel_accepts(c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    warp_accepts_common(c, shapes)
}

pub fn warp_accepts(c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    warp_accepts_common(c, shapes)
}

/// A ripmap operand: the host object (plane (0, 0) uploaded per call) or the device resident one (plane (0, 0) is the start of its buffer).
pub(crate) enum MipRef<'a> {
    Host(&'a MipImage),
    Dev(&'a crate::types::DevMipVal),
}

impl MipRef<'_> {
    pub(crate) fn width(&self) -> usize {
        match self {
            MipRef::Host(m) => m.width,
            MipRef::Dev(m) => m.width,
        }
    }
    pub(crate) fn height(&self) -> usize {
        match self {
            MipRef::Host(m) => m.height,
            MipRef::Dev(m) => m.height,
        }
    }
    pub(crate) fn channels(&self) -> usize {
        match self {
            MipRef::Host(m) => m.channels,
            MipRef::Dev(m) => m.channels,
        }
    }
    /// (floats of plane (0, 0), level 0 width, level 0 height)
    pub(crate) fn plane0_shape(&self) -> Option<(usize, usize, usize)> {
        match self {
            MipRef::Host(m) => {
                let rp = &m.ripmap;
                Some((rp.planes.get(rp.idx(0, 0))?.len(), *rp.widths.first()?, *rp.heights.first()?))
            }
            MipRef::Dev(m) => Some((m.plane_len(0, 0), *m.widths.first()?, *m.heights.first()?)),
        }
    }
}

fn warp_launch(rel: bool, s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    if args.len() != 4 {
        return Err(unsup("four operands expected"));
    }
    let mipref = match &args[0] {
        DevArg::Mip(m) => MipRef::Host(m),
        DevArg::DevMip(m) => MipRef::Dev(m),
        _ => return Err(unsup("the first operand is not a ripmap")),
    };
    let (w, h) = c.dims;
    if mipref.width() != w || mipref.height() != h || w * h != c.n || w == 0 || h == 0 {
        return Err(unsup("ripmap built for another frame size"));
    }
    let ch = mipref.channels();
    if !(1..=4).contains(&ch) || ch as i32 != c.tag {
        return Err(unsup("tag differs from the ripmap channels"));
    }
    let mx = cvt_i32(lane(&args[2], 0).ok_or_else(|| unsup("wrap is not a constant"))?);
    let my = cvt_i32(lane(&args[2], 1).ok_or_else(|| unsup("wrap is not a constant"))?);
    let flag = lane(&args[3], 0).ok_or_else(|| unsup("flag is not a constant"))?;
    let (const_coords, ca1, ca2, coords) = match &args[1] {
        DevArg::Const { lanes, .. } => (true, f32::from_bits(lanes[0]), f32::from_bits(lanes[1]), None),
        DevArg::Img(d) if d.channels() == 2 && d.n == c.n => (false, 0.0, 0.0, Some(d)),
        _ => return Err(unsup("coordinate operand must be a constant or a 2 channel image of the frame size")),
    };
    if rel {
        if footprint_taken(flag, const_coords) {
            return Err(unsup(FOOTPRINT_REASON));
        }
    } else {
        if !(0..=2).contains(&mx) || !(0..=2).contains(&my) {
            return Err(unsup("wrap mode outside the registered range"));
        }
        if flag != 0.0 && !const_coords {
            return Err(unsup(FOOTPRINT_REASON));
        }
    }
    let (plane_len, wt0, ht0) = mipref.plane0_shape().ok_or_else(|| unsup("ripmap without level 0"))?;
    if plane_len == 0 || plane_len >= i32::MAX as usize || w >= (1 << 24) || h >= (1 << 24) {
        return Err(unsup("frame or ripmap plane outside the kernel's index range"));
    }
    if wt0 >= (1 << 24) || ht0 >= (1 << 24) {
        return Err(unsup("ripmap level 0 outside the kernel's index range"));
    }
    let g: Geom = geom_of(c.kp, w, h);
    let uploaded = match &mipref {
        MipRef::Host(m) => {
            let plane_f: Vec<f32> = m.ripmap.planes[m.ripmap.idx(0, 0)].iter().map(|b| f32::from_bits(*b)).collect();
            Some(DevImage::upload(s, &[plane_f.as_slice()])?)
        }
        MipRef::Dev(_) => None,
    };
    let out = DevImage::alloc(s, ch, c.n)?;
    let err = ErrCell::new(s)?;
    let pb = match (&uploaded, &mipref) {
        (Some(u), _) => u.plane(0),
        (None, MipRef::Dev(m)) => m.dev.plane(0),
        _ => unreachable!("a host ripmap is uploaded"),
    };
    let (a1, a2) = match coords {
        Some(d) => (d.plane(0), d.plane(1)),
        None => (pb, pb),
    };
    let o = |k: usize| out.plane(k.min(ch - 1));
    s.launch(
        "warp_warp_f32",
        c.n,
        &[
            pb.arg(),
            ClArg::U32(plane_len as u32),
            a1.arg(),
            a2.arg(),
            ClArg::I32(const_coords as i32),
            ClArg::F32(ca1),
            ClArg::F32(ca2),
            o(0).arg(),
            o(1).arg(),
            o(2).arg(),
            o(3).arg(),
            ClArg::I32(ch as i32),
            ClArg::I32(w as i32),
            ClArg::I32(h as i32),
            ClArg::I32(wt0 as i32),
            ClArg::I32(ht0 as i32),
            ClArg::I32(mx),
            ClArg::I32(my),
            ClArg::F32(g.aspect),
            ClArg::F32(g.left),
            ClArg::F32(g.right),
            ClArg::F32(g.assumption_e),
            ClArg::F32(g.assumption_h),
            ClArg::I32(g.roi_x),
            ClArg::I32(g.roi_y),
            ClArg::I32(rel as i32),
            err.arg(),
        ],
    )?;
    err.check(s, "a tap outside the ripmap plane (the CPU kernel reports it)")?;
    Ok(out)
}

pub fn warp_rel_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    warp_launch(true, s, c, args)
}

pub fn warp_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    warp_launch(false, s, c, args)
}

// ---- rotate -------------------------------------------------------------------------------------------------------

pub fn rotate_accepts(_c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    expect_args(shapes, 3)?;
    let ok = |s: &ArgShape, ch: usize| matches!(s, ArgShape::Const { .. }) || *s == ArgShape::Img { channels: ch };
    if !ok(&shapes[0], 2) || !ok(&shapes[1], 1) || !ok(&shapes[2], 2) {
        return Err("operands must be constants or images of 2, 1 and 2 channels");
    }
    Ok(())
}

fn sincos_table(s: &Session) -> Result<Buffer<u64>, DevError> {
    let t = libm_sincos::tables();
    let flat: Vec<u64> = t.s.iter().flat_map(|e| [e[0], e[1]]).collect();
    Ok(Buffer::from_slice(&s.context, &flat, Access::ReadOnly)?)
}

pub fn rotate_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    if args.len() != 3 {
        return Err(unsup("three operands expected"));
    }
    let n = c.n;
    if n == 0 || n >= u32::MAX as usize {
        return Err(unsup("frame size outside the kernel's range"));
    }
    let v_const = matches!(args[0], DevArg::Const { .. });
    let a_const = matches!(args[1], DevArg::Const { .. });
    let c_const = matches!(args[2], DevArg::Const { .. });
    let img = |a: &DevArg, ch: usize| -> Result<(), DevError> {
        match a {
            DevArg::Img(d) if d.channels() == ch && d.n == n => Ok(()),
            DevArg::Const { .. } => Ok(()),
            _ => Err(unsup("operand shape")),
        }
    };
    img(&args[0], 2)?;
    img(&args[1], 1)?;
    img(&args[2], 2)?;
    if a_const && v_const && c_const {
        return Err(unsup("all operands constant (outside the contract)"));
    }
    let l = |a: &DevArg, i: usize| lane(a, i).unwrap_or(0.0);
    let two_pi = rotate2d::TWO_PI;
    let (mut sconst, mut cconst) = (0.0f32, 0.0f32);
    if a_const {
        let (sn, cs) = libm_sincos::sincosf_x(two_pi * l(&args[1], 0));
        sconst = sn;
        cconst = cs;
    }
    let (mut va, mut vb, mut ca, mut cb, mut k1, mut k2) = (0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32, 0.0f32);
    let path: i32;
    if v_const {
        va = l(&args[0], 0);
        vb = l(&args[0], 1);
    }
    if c_const {
        ca = l(&args[2], 0);
        cb = l(&args[2], 1);
    }
    if !a_const {
        path = match (v_const, c_const) {
            (true, true) => {
                let (a, b, cc, d) = (va, vb, ca, cb);
                va = a - cc;
                vb = b - d;
                1
            }
            (true, false) => 2,
            (false, true) => 3,
            (false, false) => 4,
        };
    } else {
        path = match (v_const, c_const) {
            (true, false) => 5,
            (false, false) => 6,
            (false, true) => {
                let (cc, d) = (ca, cb);
                let (sn, cs) = (sconst, cconst);
                let k1a = sn.mul_add(d, cc);
                k1 = (-cc).mul_add(cs, k1a);
                let k2a = (-sn).mul_add(cc, d);
                k2 = (-d).mul_add(cs, k2a);
                7
            }
            (true, true) => return Err(unsup("all operands constant (outside the contract)")),
        };
    }
    let st = sincos_table(s)?;
    let dm = dummy(s)?;
    let out = DevImage::alloc(s, 2, n)?;
    let err = ErrCell::new(s)?;
    fn parg<'a>(a: &DevArg<'a>, k: usize, dm: &'a Buffer<f32>) -> ClArg<'a> {
        match a {
            DevArg::Img(d) => d.plane(k).arg(),
            _ => dm.arg(),
        }
    }
    s.launch(
        "warp_rotate_f32",
        n,
        &[
            parg(&args[1], 0, &dm),
            parg(&args[0], 0, &dm),
            parg(&args[0], 1, &dm),
            parg(&args[2], 0, &dm),
            parg(&args[2], 1, &dm),
            ClArg::F32(va),
            ClArg::F32(vb),
            ClArg::F32(ca),
            ClArg::F32(cb),
            ClArg::F32(sconst),
            ClArg::F32(cconst),
            ClArg::F32(k1),
            ClArg::F32(k2),
            st.arg(),
            out.plane(0).arg(),
            out.plane(1).arg(),
            ClArg::U32(n as u32),
            ClArg::I32(path),
            err.arg(),
        ],
    )?;
    err.check(s, "an angle of 2*pi*x >= 8192 in magnitude (double precision sincos routes stay on the CPU)")?;
    Ok(out)
}

// ---- perspective and perspective_alpha ----------------------------------------------------------------------------

fn persp_accepts_common(shapes: &[ArgShape]) -> Result<(), &'static str> {
    expect_args(shapes, 4)?;
    if shapes[0] != (ArgShape::Img { channels: 2 }) {
        return Err("argument 0 must be a 2 channel image");
    }
    if !is_const(&shapes[1]) || !is_const(&shapes[2]) || !is_const(&shapes[3]) {
        return Err("scalar operands must be constants");
    }
    Ok(())
}

pub fn persp_accepts(_c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    persp_accepts_common(shapes)
}

pub fn persp_alpha_accepts(_c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    persp_accepts_common(shapes)
}

fn env_base(name: &str) -> usize {
    std::env::var(name).ok().and_then(|v| v.parse().ok()).unwrap_or(0)
}

fn persp_scalars(args: &[DevArg]) -> Result<(f32, f32, f32), DevError> {
    let g = |i: usize| lane(&args[i], 0).ok_or_else(|| unsup("scalar operand is not a constant"));
    Ok((g(1)?, g(2)?, g(3)?))
}

const TWO_PI_BITS: u32 = 0x40c9_0fdb;

pub fn persp_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    if args.len() != 4 {
        return Err(unsup("four operands expected"));
    }
    let (w, h) = c.dims;
    let (a, gpu_call_a, b) = persp_scalars(args)?;
    let DevArg::Img(uv) = &args[0] else { return Err(unsup("argument 0 is not an image")) };
    if uv.channels() != 2 || uv.n != c.n || w * h != c.n || w == 0 || h == 0 || w >= (1 << 24) || h >= (1 << 24) {
        return Err(unsup("argument 0 must be a 2 channel image of the frame size"));
    }
    if std::env::var("WPG1_PERSP_WHOLE").map(|v| v == "1").unwrap_or(false) {
        return Err(unsup("whole frame chunking stays on the CPU"));
    }
    let base = if c.kp.persp_base >= 0 { c.kp.persp_base as usize } else { env_base("WPG1_PBASE") };
    // constants of perspective::constants after the sincos callee at (-0.25 * gpu_call_a) * 2pi
    let arg = (-0.25f32 * gpu_call_a) * f32::from_bits(TWO_PI_BITS);
    let (r0, r1) = libm_sincos::sincosf_x(arg);
    let (m1, p, m4, m5, m6, m7) = (a * r0, a * r1, r0 * b, r1 * b, r0, r1);
    let out = DevImage::alloc(s, 2, c.n)?;
    s.launch(
        "warp_persp_f32",
        c.n,
        &[
            uv.plane(0).arg(),
            uv.plane(1).arg(),
            out.plane(0).arg(),
            out.plane(1).arg(),
            ClArg::F32(m1),
            ClArg::F32(p),
            ClArg::F32(m4),
            ClArg::F32(m5),
            ClArg::F32(m6),
            ClArg::F32(m7),
            ClArg::F32(b),
            ClArg::I32(w as i32),
            ClArg::I32(h as i32),
            ClArg::U32(base as u32),
            ClArg::I32(c.kp.persp_rowbuf as i32),
        ],
    )?;
    Ok(out)
}

fn env_f(name: &str) -> Option<f64> {
    std::env::var(name).ok().and_then(|v| v.parse::<f64>().ok())
}

fn cvttsd2si32(x: f64) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x <= -2147483649.0 {
        i32::MIN
    } else {
        x as i32
    }
}

pub fn persp_alpha_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    if args.len() != 4 {
        return Err(unsup("four operands expected"));
    }
    let (w, h) = c.dims;
    let (a, gpu_call_a, b) = persp_scalars(args)?;
    let DevArg::Img(uv) = &args[0] else { return Err(unsup("argument 0 is not an image")) };
    if uv.channels() != 2 || uv.n != c.n || w * h != c.n || w == 0 || h == 0 {
        return Err(unsup("argument 0 must be a 2 channel image of the frame size"));
    }
    let (s8, port_info_b, port_info_a) = match c.kp.palpha_cells {
        Some((x, y, z)) => (x, y, z),
        None => match (env_f("WPG1_S8"), env_f("WPG1_S48"), env_f("WPG1_S30")) {
            (Some(x), Some(y), Some(z)) => (x as i32, y as i32, z),
            _ => return Err(unsup("host cells of perspective_alpha are not given")),
        },
    };
    if std::env::var("WPG1_PERSP_WHOLE").map(|v| v == "1").unwrap_or(false) {
        return Err(unsup("whole frame chunking stays on the CPU"));
    }
    // perspective_alpha_with: theta, sincos, x8, then p, edi and ecx
    let theta = (-0.25f32 * gpu_call_a) * f32::from_bits(TWO_PI_BITS);
    let (r0, r1) = libm_sincos::sincosf_x(theta);
    let x8 = r1 * a;
    if x8 == 0.0 {
        return Err(unsup("zero fill route stays on the CPU"));
    }
    let pp = b * r0;
    let den = (b * (port_info_b as f32)) as f64;
    let k_cell_b = f64::from_bits(0x3fd7_0a3d_8000_0000);
    let num = ((s8 as f64) * k_cell_b) * port_info_a;
    let edi = cvttsd2si32(num / den);
    let m = edi.max(4);
    let ecx = if x8 > 0.0 { m.wrapping_neg() } else { m };
    let cf = ecx as f32;
    let out = DevImage::alloc(s, 1, c.n)?;
    s.launch(
        "warp_persp_alpha_f32",
        c.n,
        &[uv.plane(1).arg(), out.plane(0).arg(), ClArg::U32(c.n as u32), ClArg::F32(cf), ClArg::F32(pp), ClArg::F32(r1)],
    )?;
    Ok(out)
}

// ---- the self-check -----------------------------------------------------------------------------------------------

pub enum K2Arg {
    Img(Image),
    Const(usize, [u32; 4]),
    Mip(Box<MipImage>),
}

pub struct K2Case {
    pub name: String,
    pub dims: (usize, usize),
    pub tag: i32,
    pub args: Vec<K2Arg>,
    pub kp: KParams,
    /// the device must run this case (not decline) whenever the CPU kernel succeeds
    pub strict: bool,
}

fn cst(vals: &[f32]) -> K2Arg {
    let mut l = [0u32; 4];
    for (i, v) in vals.iter().enumerate().take(4) {
        l[i] = v.to_bits();
    }
    K2Arg::Const(vals.len(), l)
}

fn unit(r: &mut Rng) -> f32 {
    (r.next_u32() >> 8) as f32 / 16777216.0
}

fn span(r: &mut Rng, lo: f32, hi: f32) -> f32 {
    lo + (hi - lo) * unit(r)
}

fn plane_of(r: &mut Rng, n: usize, f: &mut dyn FnMut(&mut Rng) -> f32) -> Vec<f32> {
    (0..n).map(|_| f(r)).collect()
}

const FRAMES: &[(usize, usize)] = &[(1, 1), (2, 1), (1, 3), (3, 5), (7, 2), (16, 9), (31, 3), (33, 2), (45, 23), (64, 11)];
const FRAMES_PERSP: &[(usize, usize)] = &[(1, 2), (7, 3), (15, 2), (16, 3), (17, 2), (24, 2), (28, 3), (29, 3), (31, 2), (32, 3), (33, 2), (45, 7), (64, 5), (100, 3), (257, 2)];

fn call_cpu(class: &str, tag: i32, args: &[K2Arg], dims: (usize, usize), kp: &KParams) -> Result<Out, crate::types::ExecError> {
    let cpu: Vec<Arg> = args
        .iter()
        .map(|a| match a {
            K2Arg::Img(i) => Arg::Img(i),
            K2Arg::Const(n, l) => Arg::Const { n: *n, lanes: *l },
            K2Arg::Mip(m) => Arg::Mip(m.as_ref()),
        })
        .collect();
    kernels::run_any(class, tag, &cpu, dims.0 * dims.1, dims, kp)
}

fn src_image(r: &mut Rng, ch: usize, n: usize, dirty: bool) -> Image {
    let sp = adversarial_f32(40, r.next_u64());
    let planes = (0..ch)
        .map(|_| {
            (0..n)
                .map(|i| {
                    if dirty && r.below(7) == 0 {
                        sp[(r.next_u32() as usize + i) % sp.len()]
                    } else {
                        span(r, -0.5, 1.5)
                    }
                })
                .collect()
        })
        .collect();
    Image { n, planes }
}

fn geom_variant(k: usize, w: usize) -> Option<Geom> {
    match k % 3 {
        0 => None,
        1 => Some(Geom { left: -0.5, right: w as f32 - 0.5, aspect: 1.0, assumption_e: 0.25, assumption_h: 0.75, roi_x: 3, roi_y: -2 }),
        _ => Some(Geom { left: 1.5, right: w as f32 + 2.25, aspect: 0.75, assumption_e: -1.5, assumption_h: 0.3, roi_x: -5, roi_y: 7 }),
    }
}

fn mip_of(src: &Image, ch: usize, wrap: (i32, i32), dims: (usize, usize), kp: &KParams) -> Option<MipImage> {
    let args = [K2Arg::Img(src.clone()), cst(&[wrap.0 as f32, wrap.1 as f32]), cst(&[0.0])];
    match call_cpu("make_mipmap", ch as i32, &args, dims, kp) {
        Ok(Out::Mip(m)) => Some(m),
        _ => None,
    }
}

fn warp_cases(rel: bool) -> Vec<K2Case> {
    let class = if rel { "warp_rel" } else { "warp" };
    let mut r = Rng::new(if rel { 0x5eed_0001 } else { 0x5eed_0002 });
    let mut out = Vec::new();
    let modes: &[(i32, i32)] = if rel { &[(0, 0), (1, 1), (2, 2), (1, 2), (2, 0), (0, 1), (3, 3), (0, 3)] } else { &[(0, 0), (1, 1), (2, 2), (1, 2), (2, 0), (0, 1)] };
    let mut k = 0usize;
    for &(w, h) in FRAMES {
        let n = w * h;
        for ch in 1..=4usize {
            for &(mx, my) in modes {
                k += 1;
                let mut kp = KParams::standin();
                kp.geom = geom_variant(k, w);
                kp.pad_pixel = [0.25, -0.5, 2.0, 1.0];
                // make_mipmap wrap: the sampler's own modes mostly, another pair sometimes
                let wrap = if k % 5 == 0 { ((k % 3) as i32, ((k / 3) % 3) as i32) } else { (mx.clamp(0, 2), my.clamp(0, 2)) };
                for kind in 0..4 {
                    // kind 0: image coordinates inside the frame, 1: const coordinates, 2: wide range, 3: adversarial
                    let dirty = kind == 3;
                    let src = src_image(&mut r, ch, n, dirty && k % 2 == 0);
                    let Some(mip) = mip_of(&src, ch, wrap, (w, h), &kp) else { continue };
                    let coords: K2Arg = match kind {
                        0 => K2Arg::Img(Image { n, planes: vec![plane_of(&mut r, n, &mut |r| span(r, -1.2, 1.2)), plane_of(&mut r, n, &mut |r| span(r, -1.2, 1.2))] }),
                        1 => cst(&[span(&mut r, -1.0, 1.0), span(&mut r, -1.0, 1.0)]),
                        2 => K2Arg::Img(Image { n, planes: vec![plane_of(&mut r, n, &mut |r| span(r, -9.0, 9.0)), plane_of(&mut r, n, &mut |r| span(r, -9.0, 9.0))] }),
                        _ => {
                            let sp = adversarial_f32(30, r.next_u64());
                            K2Arg::Img(Image {
                                n,
                                planes: vec![
                                    plane_of(&mut r, n, &mut |r| if r.below(3) == 0 { sp[r.next_u32() as usize % sp.len()] } else { span(r, -2.0, 2.0) }),
                                    plane_of(&mut r, n, &mut |r| if r.below(3) == 0 { sp[r.next_u32() as usize % sp.len()] } else { span(r, -2.0, 2.0) }),
                                ],
                            })
                        }
                    };
                    // argument 3: the filter / footprint flag; nonzero is taken only with image coordinates (then the device declines)
                    let flag = if kind == 1 && k % 2 == 0 { 2.0 } else { 0.0 };
                    let args = vec![K2Arg::Mip(Box::new(mip)), coords, cst(&[mx as f32, my as f32]), cst(&[flag])];
                    out.push(K2Case { name: format!("{class} {w}x{h} ch{ch} mode({mx},{my}) wrap{wrap:?} kind{kind}"), dims: (w, h), tag: ch as i32, args, kp, strict: kind < 2 && mx <= 2 && my <= 2 });
                }
            }
        }
    }
    out
}

fn rotate_cases() -> Vec<K2Case> {
    let mut r = Rng::new(0x5eed_0003);
    let mut out = Vec::new();
    for &(w, h) in FRAMES {
        let n = w * h;
        for kind in 0..6 {
            for combo in 0..8 {
                // combo bits: v const, angle const, centre const
                let (vc, ac, cc) = (combo & 1 != 0, combo & 2 != 0, combo & 4 != 0);
                if vc && ac && cc {
                    continue;
                }
                let sp = adversarial_f32(24, r.next_u64());
                let val = |r: &mut Rng, lo: f32, hi: f32| -> f32 {
                    match kind {
                        4 => sp[r.next_u32() as usize % sp.len()],
                        5 => {
                            if r.below(4) == 0 {
                                sp[r.next_u32() as usize % sp.len()]
                            } else {
                                span(r, lo, hi)
                            }
                        }
                        _ => span(r, lo, hi),
                    }
                };
                let (alo, ahi) = match kind {
                    0 => (-1.0, 1.0),
                    1 => (-100.0, 100.0),
                    2 => (-1300.0, 1300.0),
                    _ => (-1.0e5, 1.0e5),
                };
                let v = if vc { cst(&[val(&mut r, -50.0, 50.0), val(&mut r, -50.0, 50.0)]) } else { K2Arg::Img(Image { n, planes: vec![plane_of(&mut r, n, &mut |r| val(r, -50.0, 50.0)), plane_of(&mut r, n, &mut |r| val(r, -50.0, 50.0))] }) };
                let a = if ac { cst(&[val(&mut r, alo, ahi)]) } else { K2Arg::Img(Image { n, planes: vec![plane_of(&mut r, n, &mut |r| val(r, alo, ahi))] }) };
                let c = if cc { cst(&[val(&mut r, -50.0, 50.0), val(&mut r, -50.0, 50.0)]) } else { K2Arg::Img(Image { n, planes: vec![plane_of(&mut r, n, &mut |r| val(r, -50.0, 50.0)), plane_of(&mut r, n, &mut |r| val(r, -50.0, 50.0))] }) };
                out.push(K2Case {
                    name: format!("rotate {w}x{h} v{} a{} c{} kind{kind}", vc as u8, ac as u8, cc as u8),
                    dims: (w, h),
                    tag: 2,
                    args: vec![v, a, c],
                    kp: KParams::standin(),
                    strict: kind <= 2,
                });
            }
        }
    }
    out
}

fn persp_scalar_sets() -> Vec<(f32, f32, f32)> {
    vec![(1.5, 0.2, 0.9), (2.0, -0.3, 1.1), (0.7, 0.0, 1.0), (-1.2, 0.5, 0.8), (3.0, 1.0, 0.05), (1.0, 0.25, 1.0), (0.4, -0.9, 2.5), (1.0e-3, 0.1, 1.0e3), (5.0, 3.3, 0.7)]
}

fn persp_cases() -> Vec<K2Case> {
    let mut r = Rng::new(0x5eed_0004);
    let mut out = Vec::new();
    let bases = [-1i32, 0, 4, 8, 12, 20, 2, 28];
    let mut k = 0usize;
    for &(w, h) in FRAMES_PERSP {
        let n = w * h;
        for (si, &(a, g, b)) in persp_scalar_sets().iter().enumerate() {
            k += 1;
            let mut kp = KParams::standin();
            kp.persp_base = bases[k % bases.len()];
            kp.persp_rowbuf = k % 3 == 0;
            for kind in 0..3 {
                let sp = adversarial_f32(24, r.next_u64());
                let mut f = |r: &mut Rng| -> f32 {
                    match kind {
                        0 => span(r, -3.0, 3.0),
                        1 => span(r, -40.0, 40.0),
                        _ => {
                            if r.below(3) == 0 {
                                sp[r.next_u32() as usize % sp.len()]
                            } else {
                                span(r, -2.0, 2.0)
                            }
                        }
                    }
                };
                let u = plane_of(&mut r, n, &mut f);
                let v = plane_of(&mut r, n, &mut f);
                out.push(K2Case {
                    name: format!("perspective {w}x{h} scalars{si} base{} rowbuf{} kind{kind}", kp.persp_base, kp.persp_rowbuf),
                    dims: (w, h),
                    tag: 2,
                    args: vec![K2Arg::Img(Image { n, planes: vec![u, v] }), cst(&[a]), cst(&[g]), cst(&[b])],
                    kp,
                    strict: true,
                });
            }
        }
    }
    out
}

fn persp_alpha_cases() -> Vec<K2Case> {
    let mut r = Rng::new(0x5eed_0005);
    let mut out = Vec::new();
    let cells: [(i32, i32, f64); 4] = [(1000, 50, 2.5), (4096, 3, 100.0), (7, 1000, 0.01), (100000, 2, 1.0)];
    let mut k = 0usize;
    for &(w, h) in FRAMES_PERSP {
        let n = w * h;
        for (si, &(a, g, b)) in persp_scalar_sets().iter().enumerate() {
            k += 1;
            let mut kp = KParams::standin();
            kp.palpha_cells = Some(cells[k % cells.len()]);
            for kind in 0..3 {
                let sp = adversarial_f32(24, r.next_u64());
                let mut f = |r: &mut Rng| -> f32 {
                    match kind {
                        0 => span(r, -3.0, 3.0),
                        1 => span(r, -40.0, 40.0),
                        _ => {
                            if r.below(3) == 0 {
                                sp[r.next_u32() as usize % sp.len()]
                            } else {
                                span(r, -2.0, 2.0)
                            }
                        }
                    }
                };
                let u = plane_of(&mut r, n, &mut f);
                let v = plane_of(&mut r, n, &mut f);
                out.push(K2Case {
                    name: format!("perspective_alpha {w}x{h} scalars{si} cells{:?} kind{kind}", kp.palpha_cells),
                    dims: (w, h),
                    tag: 1,
                    args: vec![K2Arg::Img(Image { n, planes: vec![u, v] }), cst(&[a]), cst(&[g]), cst(&[b])],
                    kp,
                    strict: true,
                });
            }
        }
    }
    out
}

/// The self-check cases of one class.
pub fn cases(class: &str) -> Vec<K2Case> {
    match class {
        "warp_rel" => warp_cases(true),
        "warp" => warp_cases(false),
        "rotate" => rotate_cases(),
        "perspective" => persp_cases(),
        "perspective_alpha" => persp_alpha_cases(),
        _ => Vec::new(),
    }
}

/// How the cases of a class went: compared on the device, and declined by the device (with the CPU also succeeding).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Coverage {
    pub compared: usize,
    pub declined_cpu_ok: usize,
    pub cpu_error_declined: usize,
}

pub fn selfcheck_cov(session: &Arc<Session>, op: &DeviceOp) -> Result<(Tally, Coverage), String> {
    let mut tally = Tally { outputs: 0, nan_payloads: NanPayloads::Same };
    let mut cov = Coverage::default();
    for case in cases(op.class) {
        let n = case.dims.0 * case.dims.1;
        let call = NodeCall { idx: 0, class: op.class, tag: case.tag, n, dims: case.dims, kp: &case.kp };
        let shapes: Vec<ArgShape> = case
            .args
            .iter()
            .map(|a| match a {
                K2Arg::Img(i) => ArgShape::Img { channels: i.channels() },
                K2Arg::Const(n, _) => ArgShape::Const { n: *n },
                K2Arg::Mip(_) => ArgShape::Mip,
            })
            .collect();
        if let Err(why) = (op.accepts)(&call, &shapes) {
            return Err(format!("{} {}: the corpus case is refused by accepts: {why}", op.class, case.name));
        }
        let want = call_cpu(op.class, case.tag, &case.args, case.dims, &case.kp);
        let uploads: Result<Vec<Option<DevImage>>, DevError> = case
            .args
            .iter()
            .map(|a| match a {
                K2Arg::Img(i) => DevImage::upload(session, &i.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>()).map(Some),
                _ => Ok(None),
            })
            .collect();
        let uploads = uploads.map_err(|e| format!("{} {}: upload: {e}", op.class, case.name))?;
        let dargs: Vec<DevArg> = case
            .args
            .iter()
            .zip(&uploads)
            .map(|(a, u)| match (a, u) {
                (K2Arg::Img(_), Some(d)) => DevArg::Img(d),
                (K2Arg::Const(n, l), _) => DevArg::Const { n: *n, lanes: *l },
                (K2Arg::Mip(m), _) => DevArg::Mip(m.as_ref()),
                _ => unreachable!("an image argument has an upload"),
            })
            .collect();
        let got = (op.run)(session, &call, &dargs);
        let want_img = match want {
            Ok(Out::Img(i)) => Some(i),
            Ok(Out::Mip(_)) => return Err(format!("{} {}: the CPU kernel returned a ripmap", op.class, case.name)),
            Err(_) => None,
        };
        match (want_img, got) {
            (Some(w), Ok(d)) => {
                let planes = d.download().map_err(|e| format!("{} {}: {e}", op.class, case.name))?;
                if planes.len() != w.planes.len() {
                    return Err(format!("{} {}: {} planes on the device, {} on the CPU", op.class, case.name, planes.len(), w.planes.len()));
                }
                for (p, (g, wp)) in planes.iter().zip(&w.planes).enumerate() {
                    let input = |i: usize| format!("plane {p} sample {i}");
                    match compare_f32(op.class, &format!("{} plane {p}", case.name), input, wp, g) {
                        Ok(t) => {
                            tally.outputs += t.outputs;
                            if let (NanPayloads::Same, NanPayloads::Differ(m)) = (&tally.nan_payloads, t.nan_payloads) {
                                tally.nan_payloads = NanPayloads::Differ(m);
                            }
                        }
                        Err(m @ Mismatch { .. }) => return Err(m.to_string()),
                    }
                }
                cov.compared += 1;
            }
            (Some(_), Err(DevError::Unsupported(why))) => {
                if case.strict {
                    return Err(format!("{} {}: the device declined a case the CPU runs: {why}", op.class, case.name));
                }
                cov.declined_cpu_ok += 1;
            }
            (Some(_), Err(e)) => return Err(format!("{} {}: {e}", op.class, case.name)),
            (None, Err(DevError::Unsupported(_))) => cov.cpu_error_declined += 1,
            (None, Ok(_)) => return Err(format!("{} {}: the CPU kernel fails but the device produced an output", op.class, case.name)),
            (None, Err(e)) => return Err(format!("{} {}: {e}", op.class, case.name)),
        }
    }
    if cov.compared == 0 {
        return Err(format!("{}: no corpus case ran on the device", op.class));
    }
    Ok((tally, cov))
}

pub fn selfcheck(session: &Arc<Session>, op: &DeviceOp) -> Result<Tally, String> {
    selfcheck_cov(session, op).map(|r| r.0)
}
