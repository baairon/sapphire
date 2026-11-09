//! Device ops of the Glow stage plan: `screen`, `overlay_super` and `alpha_from_rgb_max`. The kernels are in
//! `fx_device::kernels_glow_ops`.
//!
//! * `screen` and `overlay_super` resolve their operands as the `+` family does (counts 3 and 4 or 4 and 3 give three
//!   channels plus a channel 3 extension, a tag of 3 gives three channels, channel clamps, the A constant tested first).
//!   `screen` needs the vector constant to be 1.0 (its counts 3 and 4 extension writes it); `overlay_super` does not read it.
//! * `alpha_from_rgb_max` runs the mode 4 route: three plane copies (or constant fills) of channel `min(i, nch - 1)`, then
//!   channel 3 from the scale (a host value) and the copies: zero, one, `scale * c2` or `scale * max(max(c0, c1), c2)`.
//!
//! The CPU kernels stay the reference: the self-check compares bit for bit on the corpora below.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::device_ops_pointwise::{const_image, gen_cases, Od, DIMS_SMALL, PW_DIMS};
use crate::types::Image;
use fx_device::cl::Arg as ClArg;
use fx_device::selfcheck::adversarial_f32;
use fx_device::sim::Rng;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

const ONE_BITS: u32 = 0x3f80_0000;

fn unsup(why: impl Into<String>) -> DevError {
    DevError::Unsupported(why.into())
}

fn count_of(s: &ArgShape) -> Result<(usize, bool), &'static str> {
    match s {
        ArgShape::Img { channels } if (1..=4).contains(channels) => Ok((*channels, false)),
        ArgShape::Const { n } if (1..=4).contains(n) => Ok((*n, true)),
        ArgShape::Mip => Err("a ripmap operand"),
        _ => Err("operand channel count outside 1..4"),
    }
}

fn shapes_of(args: &[DevArg]) -> Vec<ArgShape> {
    args.iter()
        .map(|a| match a {
            DevArg::Img(i) => ArgShape::Img { channels: i.channels() },
            DevArg::Const { n, .. } => ArgShape::Const { n: *n },
            DevArg::Mip(_) | DevArg::DevMip(_) => ArgShape::Mip,
        })
        .collect()
}

fn plane<'a>(a: &'a DevArg, c: usize) -> Result<ClArg<'a>, DevError> {
    match a {
        DevArg::Img(i) if c < i.channels() => Ok(i.plane(c).arg()),
        _ => Err(unsup("plane operand missing")),
    }
}

fn lanes(a: &DevArg) -> [u32; 4] {
    match a {
        DevArg::Const { lanes, .. } => *lanes,
        DevArg::Img(_) | DevArg::Mip(_) | DevArg::DevMip(_) => [0; 4],
    }
}

fn is_const(a: &DevArg) -> bool {
    matches!(a, DevArg::Const { .. })
}

fn n_u32(c: &NodeCall) -> Result<u32, DevError> {
    if c.n == 0 || c.n > u32::MAX as usize {
        return Err(unsup("sample count out of range"));
    }
    Ok(c.n as u32)
}

fn check_len(c: &NodeCall, args: &[DevArg]) -> Result<(), DevError> {
    for a in args {
        if let DevArg::Img(i) = a {
            if i.n != c.n {
                return Err(unsup("plane length differs from the frame"));
            }
        }
    }
    Ok(())
}

fn fill(s: &Arc<Session>, out: &DevImage, p: usize, bits: u32, n: u32) -> Result<(), DevError> {
    s.launch("pointwise_fill_f32", n as usize, &[out.plane(p).arg(), ClArg::U32(bits), ClArg::U32(n)])?;
    Ok(())
}

// ---- screen and overlay_super: the operand plan of the `+` family -----------------------------------------------

struct BinPlan {
    nch: usize,
    /// Some(true): counts 3 and 4 (B side); Some(false): counts 4 and 3 (A side)
    ext_b_side: Option<bool>,
}

fn bin_plan(c: &NodeCall, args: &[ArgShape], needs_vec_one: bool) -> Result<BinPlan, &'static str> {
    if !(1..=4).contains(&c.tag) {
        return Err("combination tag outside 1..4");
    }
    let tag = c.tag as usize;
    if needs_vec_one && c.kp.vec_one.to_bits() != ONE_BITS {
        return Err("the vector constant is not 1.0 (the counts 3 and 4 extension writes it)");
    }
    if args.len() != 2 {
        return Err("two operands expected");
    }
    let (ca, a_const) = count_of(&args[0])?;
    let (cb, b_const) = count_of(&args[1])?;
    if a_const && b_const {
        return Err("both operands constant: the CPU kernel reads a plane of a constant");
    }
    let r13 = ca == 3 && cb == 4;
    let rbx = ca == 4 && cb == 3;
    let nch = if r13 || rbx || tag == 3 { 3 } else { tag };
    if nch > tag || ((r13 || rbx) && tag < 4) {
        return Err("fewer output planes than the kernel writes");
    }
    Ok(BinPlan { nch, ext_b_side: if r13 { Some(true) } else if rbx { Some(false) } else { None } })
}

/// The two operand route: `kmain` per output channel (mode 1 A constant, 2 B constant, 0 two planes), `kext` for channel 3
/// of the counts 3 and 4 or 4 and 3, zero for the remaining planes (the CPU output starts zeroed).
fn bin_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg], needs_vec_one: bool, kmain: &str, kext: &str) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    let plan = bin_plan(c, &shapes, needs_vec_one).map_err(unsup)?;
    check_len(c, args)?;
    let n = n_u32(c)?;
    let (a, b) = (&args[0], &args[1]);
    let (ca, cb) = (count_of(&shapes[0]).map_err(unsup)?.0, count_of(&shapes[1]).map_err(unsup)?.0);
    let tag = c.tag as usize;
    let out = DevImage::alloc(s, tag, c.n)?;
    for ch in 0..plan.nch {
        let ia = ch.min(ca - 1);
        let ib = ch.min(cb - 1);
        let o = out.plane(ch).arg();
        if is_const(a) {
            s.launch(kmain, c.n, &[plane(b, ib)?, plane(b, ib)?, o, ClArg::U32(n), ClArg::U32(1), ClArg::U32(lanes(a)[ia]), ClArg::U32(0)])?;
        } else if is_const(b) {
            s.launch(kmain, c.n, &[plane(a, ia)?, plane(a, ia)?, o, ClArg::U32(n), ClArg::U32(2), ClArg::U32(0), ClArg::U32(lanes(b)[ib])])?;
        } else {
            s.launch(kmain, c.n, &[plane(a, ia)?, plane(b, ib)?, o, ClArg::U32(n), ClArg::U32(0), ClArg::U32(0), ClArg::U32(0)])?;
        }
    }
    if let Some(b_side) = plan.ext_b_side {
        let src = if b_side { b } else { a };
        let (cbits, cflag) = if is_const(src) { (lanes(src)[3], 1) } else { (0, 0) };
        let sp = if is_const(src) { plane(if b_side { a } else { b }, 0)? } else { plane(src, 3)? };
        s.launch(kext, c.n, &[sp, out.plane(3).arg(), ClArg::U32(n), ClArg::U32(u32::from(b_side)), ClArg::U32(cflag), ClArg::U32(cbits)])?;
    }
    let written = if plan.ext_b_side.is_some() { 4 } else { plan.nch };
    for p in written..tag {
        fill(s, &out, p, 0, n)?;
    }
    Ok(out)
}

fn screen_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    bin_plan(c, a, true).map(|_| ())
}

fn screen_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    bin_run(s, c, a, true, "glow_screen_f32", "glow_screen_ext_f32")
}

fn overlay_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    bin_plan(c, a, false).map(|_| ())
}

fn overlay_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    bin_run(s, c, a, false, "glow_overlay_f32", "glow_overlay_ext_f32")
}

// ---- alpha_from_rgb_max -----------------------------------------------------------------------------------------

fn afrm_accepts(c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    if args.len() != 3 {
        return Err("three operands expected");
    }
    if c.tag != 4 {
        return Err("only the 4 channel output combination is wired");
    }
    count_of(&args[0])?;
    match args[1] {
        ArgShape::Const { n } if (1..=4).contains(&n) => {}
        _ => return Err("the amount operand is not a constant"),
    }
    Ok(())
}

fn afrm_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let shapes = shapes_of(args);
    afrm_accepts(c, &shapes).map_err(unsup)?;
    check_len(c, &args[..1])?;
    let n = n_u32(c)?;
    let src = &args[0];
    let nch = count_of(&shapes[0]).map_err(unsup)?.0;
    let scale = f32::from_bits(lanes(&args[1])[0]);
    let out = DevImage::alloc(s, 4, c.n)?;
    for i in 0..3 {
        let ch = i.min(nch - 1);
        if is_const(src) {
            fill(s, &out, i, lanes(src)[ch], n)?;
        } else {
            s.launch("copy_f32", c.n, &[plane(src, ch)?, out.plane(i).arg(), ClArg::U32(n)])?;
        }
    }
    if scale == 0.0 {
        fill(s, &out, 3, 0, n)?;
    } else if nch == 1 {
        if scale == 1.0 {
            fill(s, &out, 3, ONE_BITS, n)?;
        } else {
            s.launch("glow_afrm_mul_f32", c.n, &[out.plane(2).arg(), out.plane(3).arg(), ClArg::U32(n), ClArg::U32(scale.to_bits())])?;
        }
    } else {
        let use_scale = u32::from(scale != 1.0);
        s.launch(
            "glow_afrm_max_f32",
            c.n,
            &[out.plane(0).arg(), out.plane(1).arg(), out.plane(2).arg(), out.plane(3).arg(), ClArg::U32(n), ClArg::U32(scale.to_bits()), ClArg::U32(use_scale)],
        )?;
    }
    Ok(out)
}

// ---- self-check corpora -----------------------------------------------------------------------------------------

/// Every one or two operand template over images and constants of 1 to 4 channels.
fn templates2() -> Vec<Vec<Od>> {
    let one: Vec<Od> = (1..=4).flat_map(|c| [Od::Img(c), Od::Const(c)]).collect();
    let mut out = Vec::new();
    for a in &one {
        for b in &one {
            out.push(vec![*a, *b]);
        }
    }
    out
}

fn case(name: String, dims: (usize, usize), tag: i32, args: Vec<Image>) -> SelfCase {
    SelfCase { name, dims, tag, args, consts: vec![], kp: None }
}

/// The planes of an operand: channel p takes `vals` rotated by `7 * p`, so equal counts keep the pairs of `vals`.
fn rot_planes(vals: &[f32], ch: usize) -> Image {
    let n = vals.len();
    Image { n, planes: (0..ch).map(|p| (0..n).map(|i| vals[(i + 7 * p) % n]).collect()).collect() }
}

/// Four constant lanes taken from `vals` from position `at`.
fn lanes_from(vals: &[f32], at: usize) -> [u32; 4] {
    let mut l = [0u32; 4];
    for (j, w) in l.iter_mut().enumerate() {
        *w = vals[(at + j * 5) % vals.len()].to_bits();
    }
    l
}

const QNAN_P: u32 = 0x7fc0_0000;
const QNAN_N: u32 = 0xffc0_0000;
const SNAN_P: u32 = 0x7f80_0001;
const NAN_PAY: u32 = 0x7fc0_1234;
const NAN_PAY_N: u32 = 0xffa0_5678;

fn up(x: f32) -> f32 {
    f32::from_bits(x.to_bits() + 1)
}

fn down(x: f32) -> f32 {
    f32::from_bits(x.to_bits() - 1)
}

/// Values for the two operand ops: the branch thresholds 0.5 and 1.0 and their one-ulp neighbours, signed zeros, infinities,
/// subnormals, NaN of both signs with payloads, and a few unit range values.
fn edge_values() -> Vec<f32> {
    let mut v = vec![0.5f32, up(0.5), down(0.5), 1.0, up(1.0), down(1.0), 0.0, -0.0, 0.25, 0.75, 0.3, 0.7, 2.0, -0.5, -1.0, f32::INFINITY, f32::NEG_INFINITY];
    v.extend([1u32, 0x8000_0001, 0x007f_ffff, QNAN_P, QNAN_N, SNAN_P, NAN_PAY, NAN_PAY_N].iter().map(|b| f32::from_bits(*b)));
    v
}

/// Pairs (a, b) whose fused result differs from the two roundings, found by search with the CPU expression `f`.
fn fused_pairs(count: usize, seed: u64, lo: u32, f: fn(f32, f32) -> (f32, f32)) -> Vec<(f32, f32)> {
    let mut rng = Rng::new(seed);
    let mut out = Vec::new();
    let mut tries = 0;
    while out.len() < count && tries < 2_000_000 {
        tries += 1;
        let a = f32::from_bits(lo | (rng.next_u32() & 0x007f_ffff));
        let b = f32::from_bits(lo | (rng.next_u32() & 0x007f_ffff));
        let (fused, plain) = f(a, b);
        if fused.to_bits() != plain.to_bits() {
            out.push((a, b));
        }
    }
    out
}

/// The cross product of `edge_values` as (a, b) pairs, then `extra` pairs.
fn pair_planes(extra: &[(f32, f32)]) -> (Vec<f32>, Vec<f32>) {
    let e = edge_values();
    let (mut av, mut bv) = (Vec::new(), Vec::new());
    for &a in &e {
        for &b in &e {
            av.push(a);
            bv.push(b);
        }
    }
    for &(a, b) in extra {
        av.push(a);
        bv.push(b);
    }
    (av, bv)
}

/// Explicit cases on the edge pairs for every template and tag `accepts` takes (the generic set is thinned to its cap); constant
/// operands cycle their lanes through the edge values.
fn edge_cases(class: &'static str, accepts: fn(&NodeCall, &[ArgShape]) -> Result<(), &'static str>, extra: &[(f32, f32)]) -> Vec<SelfCase> {
    let (av, bv) = pair_planes(extra);
    let e = edge_values();
    let n = av.len();
    let kp = crate::types::KParams::standin();
    let mut out = Vec::new();
    for t in templates2() {
        let shapes: Vec<ArgShape> = t.iter().map(|o| match *o { Od::Img(c) => ArgShape::Img { channels: c }, Od::Const(c) => ArgShape::Const { n: c } }).collect();
        for tag in 1..=4 {
            let call = NodeCall { idx: 0, class, tag, n, dims: (n, 1), kp: &kp };
            if accepts(&call, &shapes).is_err() {
                continue;
            }
            let any_const = t.iter().any(|o| matches!(o, Od::Const(_)));
            // lanes_from takes positions r, r + 5, r + 10 and r + 15: the even rounds still put every edge value in some lane
            let rounds = if any_const { e.len() } else { 1 };
            for r in (0..rounds).step_by(2) {
                let args: Vec<Image> = t
                    .iter()
                    .enumerate()
                    .map(|(k, od)| match *od {
                        Od::Img(ch) => rot_planes(if k == 0 { &av } else { &bv }, ch),
                        Od::Const(nl) => const_image(nl, lanes_from(&e, r)),
                    })
                    .collect();
                out.push(case(format!("{class} edge {t:?} tag {tag} round {r}"), (n, 1), tag, args));
            }
        }
    }
    out
}

fn screen_fused(a: f32, b: f32) -> (f32, f32) {
    let om = 1.0f32 - a;
    (om.mul_add(b, a), om * b + a)
}

fn overlay_fused(a: f32, b: f32) -> (f32, f32) {
    ((-a).mul_add(b, a), a - a * b)
}

fn screen_corpus() -> Vec<SelfCase> {
    let mut v = gen_cases("screen", screen_accepts, &templates2(), &[1, 2, 3, 4], PW_DIMS, 300);
    // operands in [0.5, 1) where the fused result differs from the unfused one
    v.extend(edge_cases("screen", screen_accepts, &fused_pairs(64, 0x5c4e, 0x3f00_0000, screen_fused)));
    v
}

fn overlay_corpus() -> Vec<SelfCase> {
    let mut v = gen_cases("overlay_super", overlay_accepts, &templates2(), &[1, 2, 3, 4], PW_DIMS, 300);
    // operands in [0.5, 1) (the blend branch) where fma(-a, b, a) differs from a - a * b
    v.extend(edge_cases("overlay_super", overlay_accepts, &fused_pairs(64, 0x0e1a, 0x3f00_0000, overlay_fused)));
    v
}

/// Channel values with ties and NaN in every position: every triple over a small set (equal values, signed zeros, NaN of both signs
/// with payloads, infinities, a subnormal).
fn triple_values() -> Vec<[f32; 3]> {
    let s: Vec<f32> = [0x0000_0000u32, 0x8000_0000, 0x3f00_0000, 0x3f80_0000, 0x3f80_0001, 0xbf80_0000, 0x7f80_0000, 0xff80_0000, 0x0000_0001, QNAN_P, QNAN_N, NAN_PAY, SNAN_P]
        .iter()
        .map(|b| f32::from_bits(*b))
        .collect();
    let mut out = Vec::new();
    for &x in &s {
        for &y in &s {
            for &z in &s {
                out.push([x, y, z]);
            }
        }
    }
    out
}

fn afrm_scales() -> Vec<u32> {
    vec![0x0000_0000, 0x8000_0000, 0x3f80_0000, 0x3f80_0001, 0x3f7f_ffff, 0x3f00_0000, 0x3e80_0000, 0xc000_0000, 0x7f80_0000, 0xff80_0000, QNAN_P, NAN_PAY_N, 0x0000_0003, 0x4040_0000]
}

fn afrm_corpus() -> Vec<SelfCase> {
    let mut out = Vec::new();
    let trip = triple_values();
    let n = trip.len();
    let vals = adversarial_f32(1500, 91);
    let e = edge_values();
    let recorded_out = const_image(4, [(-999.0f32).to_bits(); 4]);
    for (si, &sb) in afrm_scales().iter().enumerate() {
        let scale = const_image(1, [sb, 0, 0, 0]);
        for ch in 1..=4usize {
            // the triples on channels 0..3 (channel 3 rotated), every scale
            let planes: Vec<Vec<f32>> = (0..ch).map(|p| (0..n).map(|i| if p < 3 { trip[i][p] } else { trip[(i + 5) % n][0] }).collect()).collect();
            out.push(case(format!("alpha_from_rgb_max triples {ch}ch scale 0x{sb:08x}"), (n, 1), 4, vec![Image { n, planes }, scale.clone(), recorded_out.clone()]));
            // adversarial values on the small frames
            for (di, &(w, h)) in DIMS_SMALL.iter().enumerate() {
                let m = w * h;
                let planes: Vec<Vec<f32>> = (0..ch).map(|p| (0..m).map(|i| vals[(i * 7 + p * 13 + si * 31 + di * 3) % vals.len()]).collect()).collect();
                let third = if di % 2 == 0 { recorded_out.clone() } else { Image { n: m, planes: vec![vec![0.0; m]] } };
                out.push(case(format!("alpha_from_rgb_max {w}x{h} {ch}ch scale 0x{sb:08x}"), (w, h), 4, vec![Image { n: m, planes }, scale.clone(), third]));
            }
            // a constant source, its lanes cycling through the edge values
            for r in (0..e.len()).step_by(3) {
                out.push(case(
                    format!("alpha_from_rgb_max const {ch} lanes round {r} scale 0x{sb:08x}"),
                    (7, 1),
                    4,
                    vec![const_image(ch, lanes_from(&e, r)), scale.clone(), recorded_out.clone()],
                ));
            }
        }
    }
    out
}

// ---- table entries ----------------------------------------------------------------------------------------------

pub const SCREEN: DeviceOp = DeviceOp { class: "screen", accepts: screen_accepts, run: screen_run, corpus: screen_corpus };
pub const OVERLAY_SUPER: DeviceOp = DeviceOp { class: "overlay_super", accepts: overlay_accepts, run: overlay_run, corpus: overlay_corpus };
pub const ALPHA_FROM_RGB_MAX: DeviceOp = DeviceOp { class: "alpha_from_rgb_max", accepts: afrm_accepts, run: afrm_run, corpus: afrm_corpus };

/// The ops of this module (the registration lines of `device_ops.rs` name them one by one).
pub fn ops() -> Vec<&'static DeviceOp> {
    vec![&SCREEN, &OVERLAY_SUPER, &ALPHA_FROM_RGB_MAX]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::KParams;

    fn call<'a>(class: &'static str, tag: i32, kp: &'a KParams) -> NodeCall<'a> {
        NodeCall { idx: 0, class, tag, n: 16, dims: (16, 1), kp }
    }

    #[test]
    fn the_recorded_glow_calls_are_accepted() {
        let kp = KParams::standin();
        let img = |c| ArgShape::Img { channels: c };
        let k = |n| ArgShape::Const { n };
        assert_eq!(screen_accepts(&call("screen", 1, &kp), &[img(1), img(1)]), Ok(()));
        assert_eq!(overlay_accepts(&call("overlay_super", 4, &kp), &[img(4), img(4)]), Ok(()));
        assert_eq!(afrm_accepts(&call("alpha_from_rgb_max", 4, &kp), &[img(4), k(1), k(4)]), Ok(()));
        assert!(afrm_accepts(&call("alpha_from_rgb_max", 3, &kp), &[img(4), k(1), k(4)]).is_err());
        assert!(afrm_accepts(&call("alpha_from_rgb_max", 4, &kp), &[img(4), img(1), k(4)]).is_err());
        assert!(screen_accepts(&call("screen", 4, &kp), &[k(4), k(4)]).is_err());
    }

    #[test]
    fn overlay_ignores_the_vector_constant_and_screen_needs_one() {
        let mut kp = KParams::standin();
        kp.vec_one = 0.75;
        let img = |c| ArgShape::Img { channels: c };
        assert_eq!(overlay_accepts(&call("overlay_super", 4, &kp), &[img(3), img(4)]), Ok(()));
        assert!(screen_accepts(&call("screen", 4, &kp), &[img(3), img(4)]).is_err());
    }

    #[test]
    fn the_corpora_hold_the_fused_cases_and_every_shape() {
        assert_eq!(fused_pairs(64, 0x5c4e, 0x3f00_0000, screen_fused).len(), 64);
        assert_eq!(fused_pairs(64, 0x0e1a, 0x3f00_0000, overlay_fused).len(), 64);
        for (class, cases) in [("screen", screen_corpus()), ("overlay_super", overlay_corpus()), ("alpha_from_rgb_max", afrm_corpus())] {
            assert!(cases.len() > 100, "{class}: {}", cases.len());
        }
    }
}
