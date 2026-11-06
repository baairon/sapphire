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
//!   Entries are deleted as kernels land (see API.md of the delivery).

use crate::backend::NodeCall;
use crate::kernels;
use crate::types::{Arg, Image, KParams};
use fx_device::cl::Arg as ClArg;
use fx_device::selfcheck::{adversarial_f32, compare_f32, Mismatch, NanPayloads, Tally};
use fx_device::{DevError, DevImage, Session};
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
}

/// One self-check case: an input image and the call it is run with.
pub struct SelfCase {
    pub name: String,
    pub dims: (usize, usize),
    pub tag: i32,
    pub args: Vec<Image>,
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
];

/// Ops allowed to run on a device.
pub const ALLOWLIST: &[&str] = &["copy", "one_minus"];

/// Ops with no device kernel, each with the reason. The list shrinks as kernels land. `get_frame`, `get_prev_dest` and
/// `save_prev_dest` are host data movement (`kernels::HOST_CLASSES`) and are not kernel ops.
pub const CPU_ONLY: &[(&str, &str)] = &[
    ("hue_shift", "not ported yet: fused multiply add sites and a per sample hue branch"),
    ("hue_shift_preserve_luma", "not ported yet: fused multiply add chains and per sample branches"),
    ("sat_scale", "not ported yet: f64 luma sums with mul_add per pixel"),
    ("v3_to_v4_if4", "not ported yet: includes a correctly rounded sqrt site and lane routing"),
    ("+", "not ported yet: elementwise, planned next (combination tag lane selection)"),
    ("-", "not ported yet: elementwise, planned next (combination tag lane selection)"),
    ("*", "not ported yet: elementwise, planned next (combination tag lane selection)"),
    ("/", "not ported yet: elementwise, planned next (combination tag lane selection)"),
    ("max", "not ported yet: x86 operand order select, no fmax"),
    ("min", "not ported yet: x86 operand order select, no fmin"),
    ("diff", "not ported yet: x86 compare and NaN order"),
    ("clamp", "not ported yet: median ladders per operand shape"),
    ("abs", "not ported yet: sign bit mask"),
    ("negate", "not ported yet: sign bit mask"),
    ("if+", "not ported yet: compare select with raw lane bits"),
    ("if>", "not ported yet: compare select with raw lane bits"),
    ("comp_rgba", "not ported yet: sixteen fused multiply add sites"),
    ("screen_super", "not ported yet: fused multiply add chains"),
    ("element_of", "not ported yet: truncating index casts and lane gather"),
    ("v2", "not ported yet: lane routing"),
    ("v3", "not ported yet: lane routing"),
    ("v4", "not ported yet: lane routing"),
    ("interp", "not ported yet: fused multiply add chains"),
    ("make_mipmap", "not ported yet: ripmap object with padded layouts, not a plain image"),
    ("blur_warp", "not ported yet: data dependent gathers from a ripmap"),
    ("warp_rel", "not ported yet: bilinear gathers and ported fmod"),
    ("static2", "not ported yet: integer hash"),
    ("monochrome", "not ported yet: f64 weighted sums with mul_add per pixel"),
    ("rotate", "not ported yet: ported sincos and fused chains"),
    ("max_element", "not ported yet: fixed order lane reduction"),
    ("min_element", "not ported yet: fixed order lane reduction"),
    ("blur", "not ported yet: row serial running box sums in CPU order"),
    ("dissolve", "not ported yet: fused multiply add"),
    ("coerce_up", "not ported yet: lane routing with pad fill"),
    ("coerce_up_a1", "not ported yet: lane routing with pad fill"),
    ("coerce_to", "not ported yet: lane routing with pad fill"),
    ("coerce_to_alpha", "not ported yet: lane routing with pad fill"),
    ("coerce_to_v3", "not ported yet: lane routing"),
    ("coerce_to_v4", "not ported yet: lane routing"),
    ("coerce_up_to_v3", "not ported yet: lane routing"),
    ("remove_alpha", "not ported yet: lane routing"),
    ("round", "not ported yet: explicit rounding mode helpers"),
    ("scale_rgb", "not ported yet: f64 steps and rounding snap"),
    ("add_rgb", "not ported yet: elementwise"),
    ("masked_blur", "not ported yet: depends on the blur kernel"),
    ("comp_premult", "not ported yet: fused multiply add chains"),
    ("grad", "not ported yet: depends on the blur kernel"),
    ("v3_to_v4", "not ported yet: lane routing"),
    ("sharpen", "not ported yet: depends on the blur kernel"),
    ("edge_aware_sharpen", "not ported yet: multi iteration driver, least certain class"),
    ("monochrome_a", "not ported yet: f64 weighted sums with mul_add per pixel"),
    ("perspective", "not ported yet: projective gather with x86 NaN first propagation"),
    ("perspective_alpha", "not ported yet: projective gather with x86 NaN first propagation"),
    ("warp", "not ported yet: gather with row buffer semantics"),
    ("postdivide", "not ported yet: elementwise divide by alpha"),
];

fn copy_calls_are_identity(tag: i32) -> bool {
    let calls = copy_slot52::slot52_calls(tag);
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
            out.push(SelfCase { name: format!("{w}x{h} tag {tag}"), dims: (w, h), tag, args: vec![Image { n, planes }] });
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

/// Runs the op on the device and on the CPU kernel for every corpus case and compares the bits. A device output
/// with a NaN is allowed only where the CPU output has a NaN too (the executor reruns such a node on the CPU).
pub fn selfcheck(session: &Arc<Session>, op: &DeviceOp) -> Result<Tally, String> {
    let kp = KParams::standin();
    let mut tally = Tally { outputs: 0, nan_payloads: NanPayloads::Same };
    for case in (op.corpus)() {
        let n = case.dims.0 * case.dims.1;
        let call = NodeCall { idx: 0, class: op.class, tag: case.tag, n, dims: case.dims, kp: &kp };
        let shapes: Vec<ArgShape> = case.args.iter().map(|i| ArgShape::Img { channels: i.channels() }).collect();
        if let Err(why) = (op.accepts)(&call, &shapes) {
            return Err(format!("{} {}: the corpus case is refused by accepts: {why}", op.class, case.name));
        }
        let cpu_args: Vec<Arg> = case.args.iter().map(Arg::Img).collect();
        let want = match kernels::run_any(op.class, case.tag, &cpu_args, n, case.dims, &kp) {
            Ok(crate::kernels2::Out::Img(i)) => i,
            Ok(_) => return Err(format!("{} {}: the CPU kernel returned a ripmap", op.class, case.name)),
            Err(e) => return Err(format!("{} {}: the CPU kernel failed: {e}", op.class, case.name)),
        };
        let uploads: Result<Vec<DevImage>, DevError> = case
            .args
            .iter()
            .map(|i| DevImage::upload(session, &i.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>()))
            .collect();
        let uploads = uploads.map_err(|e| format!("{} {}: upload: {e}", op.class, case.name))?;
        let dargs: Vec<DevArg> = uploads.iter().map(DevArg::Img).collect();
        let got = (op.run)(session, &call, &dargs).and_then(|d| d.download()).map_err(|e| format!("{} {}: {e}", op.class, case.name))?;
        if got.len() != want.planes.len() {
            return Err(format!("{} {}: {} planes on the device, {} on the CPU", op.class, case.name, got.len(), want.planes.len()));
        }
        for (p, (g, w)) in got.iter().zip(&want.planes).enumerate() {
            let input = |i: usize| case.args.first().and_then(|a| a.planes.get(p)).map(|pl| format!("plane {p} sample {i} = 0x{:08x}", pl[i].to_bits())).unwrap_or_default();
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
