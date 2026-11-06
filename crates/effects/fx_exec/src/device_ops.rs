//! The per-op device table: which ops have a device kernel, which may run on it, and which are recorded as CPU only.
//!
//! * [`DEVICE_OPS`]: ops with a kernel here. Each has `accepts` (which tags, operand shapes and parameters the kernel
//!   covers; anything else runs on the CPU), `run` (the launch) and `corpus` (the self-check cases).
//! * [`ALLOWLIST`]: the ops that may actually run on a device. An op is on it only while its kernel equals the current
//!   CPU kernel bit for bit; a parity change to a CPU kernel removes the op from this list in the same change unless
//!   the kernel is ported too. At backend build time each allowlisted op is also self-checked against the CPU kernel on
//!   the adversarial corpus, and a mismatch keeps it off the device.
//! * [`CPU_ONLY`]: every other op of the executor's dispatch with the reason it has no kernel yet. The test
//!   `every_op_has_a_device_kernel_or_a_cpu_only_entry` fails when an op of `kernels::SUPPORTED` or `kernels2::WIRED`
//!   is in neither table, so a new effect cannot add an op without either a device kernel or a written reason.
//!   Entries are deleted as kernels land.

use crate::backend::NodeCall;
use crate::kernels;
use crate::types::{Arg, Image, KParams};
use fx_device::cl::Arg as ClArg;
use fx_device::selfcheck::{adversarial_f32, compare_f32, Mismatch, NanPayloads, Tally};
use fx_device::{DevError, DevImage, Session};
use crate::device_ops_warp as warp;
use crate::device_ops_ripmap as ripmap;
use std::sync::Arc;

/// What an operand looks like, for `accepts`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgShape {
    Img { channels: usize },
    Const { n: usize },
    Mip,
}

/// An operand as the kernel launch sees it.
pub enum DevArg<'a> {
    Img(&'a DevImage),
    Const { n: usize, lanes: [u32; 4] },
    Mip(&'a crate::types::MipImage),
    /// a ripmap resident on the device (make_mipmap run by the device backend)
    DevMip(&'a crate::types::DevMipVal),
}

/// One self-check case: an input image and the call it is run with.
pub struct SelfCase {
    pub name: String,
    pub dims: (usize, usize),
    pub tag: i32,
    /// the image operands, in plan order among the images
    pub args: Vec<Image>,
    /// constant operands as (operand position, lanes in use, lane bits); the operand list interleaves them with `args`
    pub consts: Vec<(usize, usize, [u32; 4])>,
    /// the host parameters of the case (None: `KParams::standin()`)
    pub kp: Option<KParams>,
}

pub struct DeviceOp {
    /// the plan class name (`kernels::normalize_class` form)
    pub class: &'static str,
    /// Ok when the kernel covers this call, else the reason it does not.
    pub accepts: fn(&NodeCall, &[ArgShape]) -> Result<(), &'static str>,
    /// The launch: the output stays on the device.
    pub run: fn(&Arc<Session>, &NodeCall, &[DevArg]) -> Result<DevImage, DevError>,
    /// The self-check cases of this op.
    pub corpus: fn() -> Vec<SelfCase>,
}

/// Ops with a device kernel.
pub static DEVICE_OPS: &[DeviceOp] = &[
    DeviceOp { class: "copy", accepts: copy_accepts, run: copy_run, corpus: copy_corpus },
    DeviceOp { class: "one_minus", accepts: one_minus_accepts, run: one_minus_run, corpus: one_minus_corpus },
    // sampling and warp ops (device_ops_warp.rs); their self-check runs in device_ops_warp::selfcheck
    DeviceOp { class: "warp_rel", accepts: warp::warp_rel_accepts, run: warp::warp_rel_run, corpus: warp::empty_corpus },
    DeviceOp { class: "warp", accepts: warp::warp_accepts, run: warp::warp_run, corpus: warp::empty_corpus },
    DeviceOp { class: "rotate", accepts: warp::rotate_accepts, run: warp::rotate_run, corpus: warp::empty_corpus },
    DeviceOp { class: "perspective", accepts: warp::persp_accepts, run: warp::persp_run, corpus: warp::empty_corpus },
    DeviceOp { class: "perspective_alpha", accepts: warp::persp_alpha_accepts, run: warp::persp_alpha_run, corpus: warp::empty_corpus },
    // the ripmap family (device_ops_ripmap.rs); make_mipmap leaves its pyramid on the device (Val::DevMip)
    DeviceOp { class: "make_mipmap", accepts: ripmap::make_mipmap_accepts, run: ripmap::make_mipmap_run, corpus: ripmap::empty_corpus },
    DeviceOp { class: "blur_warp", accepts: ripmap::blur_warp_accepts, run: ripmap::blur_warp_run, corpus: ripmap::empty_corpus },
    // pointwise ops (device_ops_pointwise.rs)
    crate::device_ops_pointwise::ADD,
    crate::device_ops_pointwise::SUB,
    crate::device_ops_pointwise::MUL,
    crate::device_ops_pointwise::DIV,
    crate::device_ops_pointwise::MAX,
    crate::device_ops_pointwise::MIN,
    crate::device_ops_pointwise::DIFF,
    crate::device_ops_pointwise::ABS,
    crate::device_ops_pointwise::NEGATE,
    crate::device_ops_pointwise::IF_PLUS,
    crate::device_ops_pointwise::IF_GT,
    crate::device_ops_pointwise::CLAMP,
    crate::device_ops_pointwise::V2,
    crate::device_ops_pointwise::V3,
    crate::device_ops_pointwise::V4,
    crate::device_ops_pointwise::COERCE_UP,
    crate::device_ops_pointwise::COERCE_UP_A1,
    crate::device_ops_pointwise::COERCE_TO,
    crate::device_ops_pointwise::COERCE_TO_ALPHA,
    crate::device_ops_pointwise::COERCE_TO_V3,
    crate::device_ops_pointwise::COERCE_TO_V4,
    crate::device_ops_pointwise::COERCE_UP_TO_V3,
    crate::device_ops_pointwise::REMOVE_ALPHA_OP,
    crate::device_ops_pointwise::V3_TO_V4,
    crate::device_ops_pointwise::V3_TO_V4_IF4,
    crate::device_ops_pointwise::ELEMENT_OF,
    crate::device_ops_pointwise::MAX_ELEMENT,
    crate::device_ops_pointwise::MIN_ELEMENT,
    crate::device_ops_pointwise::SCREEN_SUPER,
    crate::device_ops_pointwise::INTERP,
    crate::device_ops_pointwise::DISSOLVE,
    crate::device_ops_pointwise::COMP_PREMULT,
    crate::device_ops_pointwise::COMP_RGBA,
    crate::device_ops_pointwise::POSTDIVIDE,
    crate::device_ops_pointwise::ROUND,
    crate::device_ops_pointwise::SCALE_RGB,
    crate::device_ops_pointwise::ADD_RGB,
    crate::device_ops_pointwise::SAT_SCALE,
    crate::device_ops_pointwise::HUE_SHIFT,
    crate::device_ops_pointwise::HUE_SHIFT_PRESERVE_LUMA,
    crate::device_ops_pointwise::MONOCHROME,
    crate::device_ops_pointwise::MONOCHROME_A,
    // blur family, sharpen, edge_aware_sharpen and static2 (device_ops_neighbourhood.rs)
    crate::device_ops_neighbourhood::BLUR,
    crate::device_ops_neighbourhood::MASKED_BLUR,
    crate::device_ops_neighbourhood::GRAD,
    crate::device_ops_neighbourhood::SHARPEN,
    crate::device_ops_neighbourhood::EDGE_AWARE_SHARPEN,
    crate::device_ops_neighbourhood::STATIC2,
];

/// Ops allowed to run on a device.
pub const ALLOWLIST: &[&str] = &["copy", "one_minus", "warp_rel", "warp", "rotate", "perspective", "perspective_alpha", "blur", "masked_blur", "grad", "sharpen", "edge_aware_sharpen", "static2", "+", "-", "*", "/", "max", "min", "diff", "abs", "negate", "if+", "if>", "clamp", "v2", "v3", "v4", "coerce_up", "coerce_up_a1", "coerce_to", "coerce_to_alpha", "coerce_to_v3", "coerce_to_v4", "coerce_up_to_v3", "remove_alpha", "v3_to_v4", "v3_to_v4_if4", "element_of", "max_element", "min_element", "screen_super", "interp", "dissolve", "comp_premult", "comp_rgba", "postdivide", "round", "scale_rgb", "add_rgb", "sat_scale", "hue_shift", "hue_shift_preserve_luma", "monochrome", "monochrome_a", "make_mipmap", "blur_warp"];

/// Ops with no device kernel, each with the reason. The list shrinks as kernels land. `get_frame`, `get_prev_dest` and
/// `save_prev_dest` are host data movement (`kernels::HOST_CLASSES`) and are not kernel ops.
pub const CPU_ONLY: &[(&str, &str)] = &[
];

fn copy_calls_are_identity(tag: i32) -> bool {
    let calls = copy_planes::slot52_calls(tag);
    calls.len() == tag.max(0) as usize && calls.iter().enumerate().all(|(i, c)| c.mode == 0 && c.chan == i as i32 && c.out == i as i32)
}

fn one_image(args: &[ArgShape], tag: i32) -> Result<usize, &'static str> {
    if args.len() != 1 {
        return Err("one operand expected");
    }
    if !(1..=4).contains(&tag) {
        return Err("combination tag outside 1..4");
    }
    match args[0] {
        ArgShape::Img { channels } if channels >= tag as usize => Ok(channels),
        ArgShape::Img { .. } => Err("fewer channels than the combination tag"),
        _ => Err("operand is not an image"),
    }
}

fn copy_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    one_image(args, c.tag)?;
    if !copy_calls_are_identity(c.tag) {
        return Err("the copy slot function is not the identity plane map");
    }
    Ok(())
}

fn per_plane(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], kernel: &str) -> Result<DevImage, DevError> {
    let DevArg::Img(src) = &args[0] else { return Err(DevError::Unsupported("operand is not an image".into())) };
    if c.n != src.n || c.n > u32::MAX as usize {
        return Err(DevError::Unsupported("plane length differs from the frame".into()));
    }
    let out = DevImage::alloc(s, c.tag as usize, c.n)?;
    for p in 0..c.tag as usize {
        s.launch(kernel, c.n, &[src.plane(p).arg(), out.plane(p).arg(), ClArg::U32(c.n as u32)])?;
    }
    Ok(out)
}

fn copy_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    per_plane(s, c, args, "copy_f32")
}

fn one_minus_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    one_image(args, c.tag)?;
    if c.kp.vec_one.to_bits() != 1.0f32.to_bits() {
        return Err("the vector constant is not 1.0, the kernel computes 1 - x");
    }
    Ok(())
}

fn one_minus_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    per_plane(s, c, args, "one_minus_f32")
}

/// Frame shapes of the self-check: one pixel, odd sizes, rows, a power of two and an irregular frame.
pub const DIMS: &[(usize, usize)] = &[(1, 1), (2, 1), (1, 3), (3, 5), (7, 1), (31, 1), (33, 2), (16, 16), (257, 1), (45, 23)];

fn unary_corpus() -> Vec<SelfCase> {
    let vals = adversarial_f32(4000, 31);
    let mut out = Vec::new();
    for &(w, h) in DIMS {
        let n = w * h;
        for tag in 1..=4i32 {
            let planes: Vec<Vec<f32>> = (0..tag as usize).map(|p| (0..n).map(|i| vals[(i * 7 + p * 13 + n) % vals.len()]).collect()).collect();
            out.push(SelfCase { name: format!("{w}x{h} tag {tag}"), dims: (w, h), tag, args: vec![Image { n, planes }], consts: vec![], kp: None });
        }
    }
    out
}

fn copy_corpus() -> Vec<SelfCase> {
    unary_corpus()
}

fn one_minus_corpus() -> Vec<SelfCase> {
    unary_corpus()
}

/// A self-check operand that is a constant: an image with `n == 0` whose planes 0..3 hold the four lane words and plane 4 the lane count
/// (see `device_ops_pointwise::const_image`).
fn case_const(i: &Image) -> Option<(usize, [u32; 4])> {
    if i.n == 0 && i.planes.len() == 5 {
        let l = |k: usize| i.planes[k][0].to_bits();
        Some((i.planes[4][0] as usize, [l(0), l(1), l(2), l(3)]))
    } else {
        None
    }
}

/// Runs the op on the device and on the CPU kernel for every corpus case and compares the bits. A device output
/// with a NaN is allowed only where the CPU output has a NaN too (the executor reruns such a node on the CPU).
pub fn selfcheck(session: &Arc<Session>, op: &DeviceOp) -> Result<Tally, String> {
    if warp::handles(op.class) {
        return warp::selfcheck(session, op);
    }
    if ripmap::handles(op.class) {
        return ripmap::selfcheck(session, op);
    }
    let mut tally = Tally { outputs: 0, nan_payloads: NanPayloads::Same };
    for case in (op.corpus)() {
        let kp = case.kp.unwrap_or_else(KParams::standin);
        let n = case.dims.0 * case.dims.1;
        let call = NodeCall { idx: 0, class: op.class, tag: case.tag, n, dims: case.dims, kp: &kp };
        let total = case.args.len() + case.consts.len();
        let mut imgs = case.args.iter();
        let mut order: Vec<Result<&Image, (usize, [u32; 4])>> = Vec::with_capacity(total);
        for pos in 0..total {
            match case.consts.iter().find(|c| c.0 == pos) {
                Some(c) => order.push(Err((c.1, c.2))),
                None => match imgs.next() {
                    Some(i) => order.push(match case_const(i) {
                        Some((cn, lanes)) => Err((cn, lanes)),
                        None => Ok(i),
                    }),
                    None => return Err(format!("{} {}: the corpus case has a gap in its operand positions", op.class, case.name)),
                },
            }
        }
        let shapes: Vec<ArgShape> = order.iter().map(|o| match o {
            Ok(i) => ArgShape::Img { channels: i.channels() },
            Err((cn, _)) => ArgShape::Const { n: *cn },
        }).collect();
        if let Err(why) = (op.accepts)(&call, &shapes) {
            return Err(format!("{} {}: the corpus case is refused by accepts: {why}", op.class, case.name));
        }
        let cpu_args: Vec<Arg> = order.iter().map(|o| match o {
            Ok(i) => Arg::Img(i),
            Err((cn, lanes)) => Arg::Const { n: *cn, lanes: *lanes },
        }).collect();
        let want = match kernels::run_any(op.class, case.tag, &cpu_args, n, case.dims, &kp) {
            Ok(crate::kernels2::Out::Img(i)) => i,
            Ok(_) => return Err(format!("{} {}: the CPU kernel returned a ripmap", op.class, case.name)),
            Err(e) => return Err(format!("{} {}: the CPU kernel failed: {e}", op.class, case.name)),
        };
        let uploads: Result<Vec<DevImage>, DevError> = case
            .args
            .iter()
            .filter(|i| case_const(i).is_none())
            .map(|i| DevImage::upload(session, &i.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>()))
            .collect();
        let uploads = uploads.map_err(|e| format!("{} {}: upload: {e}", op.class, case.name))?;
        let mut up = uploads.iter();
        let dargs: Vec<DevArg> = order.iter().map(|o| match o {
            Ok(_) => DevArg::Img(up.next().expect("one upload per image")),
            Err((cn, lanes)) => DevArg::Const { n: *cn, lanes: *lanes },
        }).collect();
        let got = (op.run)(session, &call, &dargs).and_then(|d| d.download()).map_err(|e| format!("{} {}: {e}", op.class, case.name))?;
        if got.len() != want.planes.len() {
            return Err(format!("{} {}: {} planes on the device, {} on the CPU", op.class, case.name, got.len(), want.planes.len()));
        }
        for (p, (g, w)) in got.iter().zip(&want.planes).enumerate() {
            let input = |i: usize| case.args.first().and_then(|a| a.planes.get(p)).and_then(|pl| pl.get(i).map(|v| format!("plane {p} sample {i} = 0x{:08x}", v.to_bits()))).unwrap_or_default();
            match compare_f32(op.class, &format!("{} plane {p}", case.name), input, w, g) {
                Ok(t) => {
                    tally.outputs += t.outputs;
                    if let (NanPayloads::Same, NanPayloads::Differ(m)) = (&tally.nan_payloads, t.nan_payloads) {
                        tally.nan_payloads = NanPayloads::Differ(m);
                    }
                }
                Err(m @ Mismatch { .. }) => return Err(m.to_string()),
            }
        }
    }
    Ok(tally)
}

/// The kernel ops of the executor's dispatch: every class `kernels::SUPPORTED` and `kernels2::WIRED` serve.
pub fn dispatch_ops() -> Vec<&'static str> {
    let mut v: Vec<&'static str> = Vec::new();
    for c in kernels::SUPPORTED.iter().chain(crate::kernels2::WIRED.iter()) {
        if !v.contains(c) {
            v.push(c);
        }
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_op_has_a_device_kernel_or_a_cpu_only_entry() {
        let mut missing = Vec::new();
        for op in dispatch_ops() {
            let kernel = DEVICE_OPS.iter().any(|d| d.class == op);
            let cpu_only = CPU_ONLY.iter().any(|(c, _)| *c == op);
            if !kernel && !cpu_only {
                missing.push(op);
            }
            assert!(!(kernel && cpu_only), "{op} has both a device kernel and a cpu_only entry: delete the cpu_only entry");
        }
        assert!(missing.is_empty(), "ops with neither a device kernel nor a cpu_only entry with a reason: {missing:?}");
    }

    #[test]
    fn the_tables_are_consistent() {
        let ops = dispatch_ops();
        for (c, why) in CPU_ONLY {
            assert!(ops.contains(c), "cpu_only names `{c}`, which is not an op of the dispatch");
            assert!(why.len() > 12, "cpu_only `{c}` needs a reason");
            assert_eq!(CPU_ONLY.iter().filter(|(x, _)| x == c).count(), 1, "`{c}` listed twice");
        }
        for d in DEVICE_OPS {
            assert!(ops.contains(&d.class), "device op `{}` is not an op of the dispatch", d.class);
            assert_eq!(DEVICE_OPS.iter().filter(|x| x.class == d.class).count(), 1);
        }
        for a in ALLOWLIST {
            assert!(DEVICE_OPS.iter().any(|d| d.class == *a), "allowlisted `{a}` has no kernel");
        }
    }

    #[test]
    fn host_classes_are_not_kernel_ops() {
        for h in kernels::HOST_CLASSES {
            assert!(!dispatch_ops().contains(&h) || CPU_ONLY.iter().any(|(c, _)| *c == h));
        }
    }
}
