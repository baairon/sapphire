//! Device ops: plane assembly (`copy_alpha`, `get_alpha`, `v3_to_v4_if4_a`, `append`). These CPU kernels only move planes
//! through the plane service: each output plane is a copy of one operand channel or a fill with one constant word. Each op
//! here resolves, on the host and exactly as its CPU wrapper does, which operand channel or constant feeds which output
//! plane, and runs that routing with `route_run` (the `copy_f32` and `pointwise_fill_f32` kernels). The pooled fill rule of the
//! plane service (a fill of 512 or more samples outside a pool worker stores +0.0 for either zero) is applied by `route_run`.
//!
//! Every CPU wrapper of these ops passes the flat element range record (`flat_roi`), so the ranged copy and ranged fill
//! routes of the plane service are never reached; the device route covers the flat record only, and a frame whose sample
//! count does not fit the record's 32 bit count is refused. Shapes the CPU wrapper refuses (operand count, combination row,
//! tag) are refused here too.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::device_ops_pointwise::{all_templates, const_image, gen_cases, route_check, route_run, shapes_of, Od, Src, DIMS_SMALL};
use crate::types::{Image, KParams};
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

const ONE_BITS: u32 = 0x3f80_0000;

type Plan = fn(&NodeCall, &[ArgShape]) -> Result<(Vec<(usize, Src)>, usize), &'static str>;

/// Channel counts of the operands (1..4 each); a ripmap or an out of range count is refused.
fn counts(args: &[ArgShape]) -> Result<Vec<i32>, &'static str> {
    args.iter()
        .map(|a| match a {
            ArgShape::Img { channels } if (1..=4).contains(channels) => Ok(*channels as i32),
            ArgShape::Const { n } if (1..=4).contains(n) => Ok(*n as i32),
            ArgShape::Mip => Err("a ripmap operand"),
            _ => Err("operand channel count outside 1..4"),
        })
        .collect()
}

fn flat_range(c: &NodeCall) -> Result<(), &'static str> {
    if c.n == 0 || c.n > i32::MAX as usize {
        return Err("sample count outside the flat element range record");
    }
    Ok(())
}

fn tag_1_to_4(c: &NodeCall) -> Result<usize, &'static str> {
    if (1..=4).contains(&c.tag) {
        Ok(c.tag as usize)
    } else {
        Err("combination tag outside 1..4")
    }
}

/// copy_alpha: output channels 0..cnt from operand 0 at channel min(last0, i); with mode 2 or 4 the last output channel
/// comes from operand 1 at channel min(last1, mode - 1). The mode word is `fx_eval::fold::copy_alpha_mode`, as in the CPU wrapper.
fn copy_alpha_plan(c: &NodeCall, args: &[ArgShape]) -> Result<(Vec<(usize, Src)>, usize), &'static str> {
    flat_range(c)?;
    if args.len() != 2 {
        return Err("two operands expected");
    }
    let tag = tag_1_to_4(c)?;
    let k = counts(args)?;
    let (last0, last1) = (k[0] - 1, k[1] - 1);
    let mode = fx_eval::fold::copy_alpha_mode(c.tag, k[1]);
    let flag = mode == 2 || mode == 4;
    let cnt = if flag { c.tag - 1 } else { c.tag };
    let mut r = Vec::new();
    for i in 0..cnt {
        let ch = if last0 < i { last0 } else { i };
        r.push((i as usize, Src::Chan { arg: 0, chan: ch }));
    }
    if flag {
        let want = mode - 1;
        let ch = if last1 < want { last1 } else { want };
        r.push((tag - 1, Src::Chan { arg: 1, chan: ch }));
    }
    Ok((r, tag))
}

/// get_alpha: channel 1 of a two channel operand, channel 3 of a four channel operand, else a fill of 1.0. One output plane.
fn get_alpha_plan(c: &NodeCall, args: &[ArgShape]) -> Result<(Vec<(usize, Src)>, usize), &'static str> {
    flat_range(c)?;
    if args.len() != 1 {
        return Err("one operand expected");
    }
    if c.tag != 1 {
        return Err("selector tag differs from the single result plane");
    }
    let k = counts(args)?;
    let src = match k[0] {
        2 => Src::Chan { arg: 0, chan: 1 },
        4 => Src::Chan { arg: 0, chan: 3 },
        _ => Src::Fill(ONE_BITS),
    };
    Ok((vec![(0, src)], 1))
}

/// v3_to_v4_if4_a: the pass of the selected combination row. Pass F's channel bound is the second operand's count, the
/// value the CPU wrapper passes as `this_nch`.
fn v3a_plan(c: &NodeCall, args: &[ArgShape]) -> Result<(Vec<(usize, Src)>, usize), &'static str> {
    use v3_to_v4_if4_a::Pass as P;
    flat_range(c)?;
    if args.len() != 3 {
        return Err("three operands expected");
    }
    let k = counts(args)?;
    let combo = v3_to_v4_if4_a::select_combo([k[0], k[1], k[2]]).ok_or("no combination row")?;
    let row = v3_to_v4_if4_a::COMBOS[combo];
    if row.out as i32 != c.tag {
        return Err("selector tag differs from the row result count");
    }
    let this_nch = k[1];
    let mut r = Vec::new();
    match row.pass {
        P::A => {
            let mx = k[0].wrapping_sub(1);
            for ch in 0..3i32 {
                let chan = if ch >= mx { mx } else { ch };
                r.push((ch as usize, Src::Chan { arg: 0, chan }));
            }
            r.push((3, Src::Chan { arg: 1, chan: 0 }));
        }
        P::B => {
            for ch in 0..row.out {
                r.push((ch, Src::Chan { arg: 0, chan: ch as i32 }));
            }
        }
        P::C => {
            for ch in 0..row.out {
                r.push((ch, Src::Chan { arg: 0, chan: 0 }));
            }
        }
        P::D => {
            r.push((0, Src::Chan { arg: 0, chan: 0 }));
            r.push((1, Src::Chan { arg: 1, chan: 0 }));
        }
        P::F => {
            let mx = this_nch.wrapping_sub(1);
            for ch in 0..row.out {
                let ci = ch as i32;
                let chan = if ci < mx { ci } else { mx };
                r.push((ch, Src::Chan { arg: 1, chan }));
            }
        }
    }
    Ok((r, row.out))
}

/// append: the channels of operand 0, then the channels of operand 1 until four planes are written; the result keeps the
/// first `tag` planes and a plane that no call writes is +0.0.
fn append_plan(c: &NodeCall, args: &[ArgShape]) -> Result<(Vec<(usize, Src)>, usize), &'static str> {
    flat_range(c)?;
    if args.len() != 2 {
        return Err("two operands expected");
    }
    let tag = tag_1_to_4(c)?;
    let k = counts(args)?;
    let (first, second) = (k[0] as usize, k[1] as usize);
    let mut r = Vec::new();
    for ch in 0..first {
        r.push((ch, Src::Chan { arg: 0, chan: ch as i32 }));
    }
    if first < 4 {
        let mut j = 0;
        while j < second {
            r.push((first + j, Src::Chan { arg: 1, chan: j as i32 }));
            j += 1;
            if first + j >= 4 {
                break;
            }
        }
    }
    Ok((r, tag))
}

fn plan_accepts(plan: Plan, c: &NodeCall, args: &[ArgShape]) -> Result<(), &'static str> {
    let (r, _) = plan(c, args)?;
    route_check(args, &r)
}

fn plan_run(plan: Plan, s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let (r, tag) = plan(c, &shapes_of(args)).map_err(|w| DevError::Unsupported(w.into()))?;
    route_run(s, c, args, &r, tag, false)
}

fn copy_alpha_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    plan_accepts(copy_alpha_plan, c, a)
}
fn get_alpha_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    plan_accepts(get_alpha_plan, c, a)
}
fn v3a_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    plan_accepts(v3a_plan, c, a)
}
fn append_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    plan_accepts(append_plan, c, a)
}

// ---- corpus -------------------------------------------------------------------------------------------------------

/// Constant lane words: -0.0, a quiet NaN with a payload, a small integer bit pattern (a subnormal as f32) and a negative
/// signalling NaN. Lane j of a case takes word (j + rot) mod 4, so every lane meets every word.
const LANE_WORDS: [u32; 4] = [0x8000_0000, 0x7fc0_0001, 0x0000_0007, 0xff80_0001];

/// Sample counts around the pooled fill threshold, as (n, in_pool, rotation). A template with a constant runs all four
/// rotations at 512 samples outside a pool worker (the route that clears a zero) and one rotation elsewhere.
const POOL_RUNS: [(usize, bool, usize); 9] = [(512, false, 0), (512, false, 1), (512, false, 2), (512, false, 3), (511, false, 0), (513, false, 1), (512, true, 2), (511, true, 3), (513, true, 0)];
const POOL_RUNS_IMG: [(usize, bool, usize); 2] = [(513, false, 0), (512, true, 1)];

fn pool_cases(class: &'static str, accepts: fn(&NodeCall, &[ArgShape]) -> Result<(), &'static str>, nargs: usize, tags: &[i32]) -> Vec<SelfCase> {
    let vals = adversarial_f32(1500, 91);
    let mut out = Vec::new();
    let mut salt = 0usize;
    for t in all_templates(nargs, true) {
        let shapes: Vec<ArgShape> = t.iter().map(|o| match *o { Od::Img(c) => ArgShape::Img { channels: c }, Od::Const(c) => ArgShape::Const { n: c } }).collect();
        let has_const = t.iter().any(|o| matches!(o, Od::Const(_)));
        for &tag in tags {
            let runs: &[(usize, bool, usize)] = if has_const { &POOL_RUNS } else { &POOL_RUNS_IMG };
            for &(n, pool, rot) in runs {
                let mut kp = KParams::standin();
                kp.in_pool = pool;
                let call = NodeCall { idx: 0, class, tag, n, dims: (n, 1), kp: &kp };
                if accepts(&call, &shapes).is_err() {
                    continue;
                }
                salt += 1;
                let args: Vec<Image> = t
                    .iter()
                    .enumerate()
                    .map(|(k, od)| match *od {
                        Od::Img(ch) => Image { n, planes: (0..ch).map(|p| (0..n).map(|i| vals[(i * 5 + k * 97 + p * 31 + salt * 7) % vals.len()]).collect()).collect() },
                        Od::Const(nl) => {
                            let mut l = [0u32; 4];
                            for (j, w) in l.iter_mut().enumerate() {
                                *w = LANE_WORDS[(j + rot + k) % 4];
                            }
                            const_image(nl, l)
                        }
                    })
                    .collect();
                out.push(SelfCase { name: format!("{class} n {n} pool {pool} rot {rot} tag {tag} {t:?}"), dims: (n, 1), tag, args, consts: vec![], kp: Some(kp) });
            }
        }
    }
    out
}

fn corpus_for(class: &'static str, accepts: fn(&NodeCall, &[ArgShape]) -> Result<(), &'static str>, nargs: usize, tags: &[i32], cap: usize) -> Vec<SelfCase> {
    let mut v = gen_cases(class, accepts, &all_templates(nargs, true), tags, DIMS_SMALL, cap);
    v.extend(pool_cases(class, accepts, nargs, tags));
    v
}

pub const COPY_ALPHA: DeviceOp = DeviceOp { class: "copy_alpha", accepts: copy_alpha_accepts, run: |s, c, a| plan_run(copy_alpha_plan, s, c, a), corpus: || corpus_for("copy_alpha", copy_alpha_accepts, 2, &[1, 2, 3, 4], 300) };
pub const GET_ALPHA: DeviceOp = DeviceOp { class: "get_alpha", accepts: get_alpha_accepts, run: |s, c, a| plan_run(get_alpha_plan, s, c, a), corpus: || corpus_for("get_alpha", get_alpha_accepts, 1, &[1], 120) };
pub const V3_TO_V4_IF4_A: DeviceOp = DeviceOp { class: "v3_to_v4_if4_a", accepts: v3a_accepts, run: |s, c, a| plan_run(v3a_plan, s, c, a), corpus: || corpus_for("v3_to_v4_if4_a", v3a_accepts, 3, &[1, 2, 3, 4], 240) };
pub const APPEND: DeviceOp = DeviceOp { class: "append", accepts: append_accepts, run: |s, c, a| plan_run(append_plan, s, c, a), corpus: || corpus_for("append", append_accepts, 2, &[1, 2, 3, 4], 300) };

/// The ops of this module (the registration lines of `device_ops.rs` name them one by one).
pub fn ops() -> Vec<&'static DeviceOp> {
    vec![&COPY_ALPHA, &GET_ALPHA, &V3_TO_V4_IF4_A, &APPEND]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device_ops::{selfcheck, ALLOWLIST, CPU_ONLY, DEVICE_OPS};
    use crate::types::Arg;
    use fx_device::selfcheck::NanPayloads;

    #[test]
    fn plane_ops_are_registered_and_allowlisted() {
        for op in ops() {
            assert!(DEVICE_OPS.iter().any(|d| d.class == op.class), "{} is not in DEVICE_OPS", op.class);
            assert!(ALLOWLIST.contains(&op.class), "{} is not allowlisted", op.class);
            assert!(!CPU_ONLY.iter().any(|(c, _)| *c == op.class), "{} still has a CPU_ONLY entry", op.class);
        }
    }

    /// accepts takes exactly the shapes on which the CPU wrapper runs: every operand count, kind and tag, compared with the CPU kernel.
    #[test]
    fn plane_ops_accept_exactly_what_the_cpu_wrapper_runs() {
        let kp = KParams::standin();
        let n = 6usize;
        for (op, nargs_list) in [(&COPY_ALPHA, vec![1usize, 2, 3]), (&GET_ALPHA, vec![1, 2]), (&V3_TO_V4_IF4_A, vec![2, 3]), (&APPEND, vec![1, 2, 3])] {
            let mut accepted = 0;
            for nargs in nargs_list {
                for t in all_templates(nargs, true) {
                    let imgs: Vec<Image> = t.iter().map(|o| match *o { Od::Img(c) => Image { n, planes: (0..c).map(|p| (0..n).map(|i| (i * 4 + p) as f32).collect()).collect() }, Od::Const(_) => Image { n, planes: vec![] } }).collect();
                    let args: Vec<Arg> = t.iter().zip(&imgs).map(|(o, i)| match *o { Od::Img(_) => Arg::Img(i), Od::Const(c) => Arg::Const { n: c, lanes: [1, 2, 3, 4] } }).collect();
                    let shapes: Vec<ArgShape> = t.iter().map(|o| match *o { Od::Img(c) => ArgShape::Img { channels: c }, Od::Const(c) => ArgShape::Const { n: c } }).collect();
                    for tag in -1..=5 {
                        let call = NodeCall { idx: 0, class: op.class, tag, n, dims: (n, 1), kp: &kp };
                        let dev = (op.accepts)(&call, &shapes);
                        let cpu = crate::kernels::run_any(op.class, tag, &args, n, (n, 1), &kp);
                        // tag 0 gives an empty image on the CPU for copy_alpha; the device refuses an empty output, which only costs a fallback
                        if cpu.is_ok() && !(op.class == "copy_alpha" && tag == 0) {
                            assert!(dev.is_ok(), "{} tag {tag} {t:?}: the CPU runs it, the device refuses: {:?}", op.class, dev);
                            accepted += 1;
                        }
                        if cpu.is_err() {
                            assert!(dev.is_err(), "{} tag {tag} {t:?}: the CPU refuses it, the device accepts", op.class);
                        }
                    }
                }
            }
            println!("{} accepted shapes {accepted}", op.class);
            assert!(accepted > 0);
        }
    }

    #[test]
    fn plane_corpora_cover_the_pool_threshold() {
        for op in ops() {
            let cases = (op.corpus)();
            for (n, pool) in [(511usize, false), (512, false), (513, false), (511, true), (512, true), (513, true)] {
                assert!(cases.iter().any(|c| c.dims.0 * c.dims.1 == n && c.kp.as_ref().map(|k| k.in_pool) == Some(pool)), "{}: no case with n {n} in_pool {pool}", op.class);
            }
            println!("{} corpus {}", op.class, cases.len());
        }
    }

    /// Every op of this module equals the CPU kernel bit for bit on the device, NaN payloads included (prints NOT RUN without a device).
    #[test]
    fn device_plane_ops_pass_the_self_check_bit_for_bit() {
        let ctx = match fx_device::select(fx_device::DeviceKind::Gpu) {
            fx_device::Backend::Device(d) => d,
            other => {
                if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
                    panic!("device required but unavailable: {}", other.describe());
                }
                println!("NOT RUN: no usable OpenCL device ({})", other.describe());
                return;
            }
        };
        for op in ops() {
            let ncases = (op.corpus)().len();
            let t = selfcheck(&ctx.session, op).unwrap_or_else(|e| panic!("{} refused: {e}", op.class));
            assert!(matches!(t.nan_payloads, NanPayloads::Same), "{}: NaN payloads differ: {:?}", op.class, t.nan_payloads);
            println!("PLANE-SELFCHECK {} cases {ncases} outputs {} nan_payloads Same", op.class, t.outputs);
        }
    }
}
