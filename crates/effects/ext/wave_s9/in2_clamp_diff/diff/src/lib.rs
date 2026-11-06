//! diff (IN2): native port of the operator class `diff` (vtable, first slot, getter/factory, slot 53 dispatcher,
//! AVX2 body, AVX tier, SSE tier, slot 54 GPU twin). Listing offsets; VA = offset.
//! The body is the OP1 `-` body shape (scalar main loop over up to 3 or nout channels, two constant-operand flags, extension of channel 3
//! for the (3,4) and (4,3) operand pairs) with an absolute value on every result: out = bits(a - b) AND mask.
//! std only, no unsafe, explicit parameters (no Default), S3-style `render`. Assumption prefix IN2D-. See README.md and SPEC.md.
pub use ops_arith::{OpError, Operand, ONE};

/// Scalar abs mask, CAPTURED : low lane 0x7fffffff (lookup_ops.tsv of OP2).
pub const ABS_MASK: u32 = 0x7fff_ffff;

/// Explicit parameters. `nout` = result of virtual slot 31 (hooked count getter). `n` = [info].
/// `vec_one` = STD-002 (8 lanes, MEASURED-DYLIB 1.0f), `vec_mask` = OP2-A3 (8 lanes,
/// MEASURED-DYLIB 0x7fffffff); both uncovered in the captured image, no default. `dst3_phase_bytes` = (address of output plane 3) & 0x1f (ALIGN-DEPENDENT).
pub struct Params {
    pub nout: i32,
    pub n: i32,
    pub vec_one: f32,
    pub vec_mask: u32,
    pub dst3_phase_bytes: u32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mutant {
    None,
    SwapPlane,
    SwapConstB,
    ClampA,
    ClampB,
    Fixed3Off,
    R14Swap,
    R12Off,
    Nch2,
    ConstIdx,
    NoConstA,
    NoConstB,
    ExtA3Only,
    Cut28,
    NoPeel,
    PeelPlus1,
    VecOneIgnored,
    ExtOrder,
    NoAbs,
    AbsExtOff,
    VecMaskIgnored,
}

impl Mutant {
    pub fn from_name(s: &str) -> Option<Mutant> {
        use Mutant::*;
        Some(match s {
            "none" => None,
            "swap_plane" => SwapPlane,
            "swap_const_b" => SwapConstB,
            "clamp_a" => ClampA,
            "clamp_b" => ClampB,
            "fixed3_off" => Fixed3Off,
            "r14_swap" => R14Swap,
            "r12_off" => R12Off,
            "nch2" => Nch2,
            "const_idx" => ConstIdx,
            "no_const_a" => NoConstA,
            "no_const_b" => NoConstB,
            "ext_a3_only" => ExtA3Only,
            "cut28" => Cut28,
            "no_peel" => NoPeel,
            "peel_plus1" => PeelPlus1,
            "vec_one_ignored" => VecOneIgnored,
            "ext_order" => ExtOrder,
            "no_abs" => NoAbs,
            "abs_ext_off" => AbsExtOff,
            "vec_mask_ignored" => VecMaskIgnored,
            _ => return Option::None,
        })
    }
}

fn plane<'a>(o: &Operand<'a>, idx: i32) -> Result<&'a [f32], OpError> {
    if idx < 0 {
        return Err(OpError::ConstIndex);
    }
    o.planes.get(idx as usize).copied().ok_or(OpError::MissingPlane)
}

fn cidx(idx: i32) -> Result<usize, OpError> {
    if (0..=3).contains(&idx) {
        Ok(idx as usize)
    } else {
        Err(OpError::ConstIndex)
    }
}

/// vsubss/vsubps then vandps with the mask: bit AND, so a NaN keeps its payload and loses the sign.
fn absd(x: f32, mask: u32, m: Mutant) -> f32 {
    if m == Mutant::NoAbs {
        x
    } else {
        f32::from_bits(x.to_bits() & mask)
    }
}

/// Class module entry: the whole slot-53 AVX2 body.
pub fn render(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]]) -> Result<(), OpError> {
    run(p, a, b, out, Mutant::None)
}

pub fn render_mutant(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    run(p, a, b, out, m)
}

pub fn run(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    // r14 = (countA == 3 && countB == 4), r12 = (countA == 4 && countB == 3)
    let (mut r14, mut r12) = (a.count == 3 && b.count == 4, a.count == 4 && b.count == 3);
    if m == Mutant::R14Swap {
        r14 = a.count == 4 && b.count == 3;
    }
    if m == Mutant::ExtA3Only {
        r14 = a.count == 3;
    }
    if m == Mutant::R12Off {
        r12 = false;
    }
    // (r14|r12) != 0 or nout == 3 take the fixed 3 channel loop (no nout <= 0 test); else nout channels (: jle skips)
    let fixed3 = if m == Mutant::Fixed3Off { r14 || r12 } else { r14 || r12 || p.nout == 3 };
    let mut nch: i32 = if fixed3 { 3 } else if p.nout > 0 { p.nout } else { 0 };
    if m == Mutant::Nch2 && nch > 2 {
        nch = 2;
    }
    let n = p.n;
    if n > 0 {
        for ch in 0..nch {
            if out.get(ch as usize).map_or(true, |d| d.len() < n as usize) {
                return Err(OpError::MissingOut);
            }
        }
        if (r14 || r12) && out.get(3).map_or(true, |d| d.len() < n as usize) {
            return Err(OpError::MissingOut);
        }
    }
    let (a_const, b_const) = (a.is_const && m != Mutant::NoConstA, b.is_const && m != Mutant::NoConstB);
    for ch in 0..nch {
        // cmp esi,eax ; cmovl edx,esi : idx = min(ch, count - 1), signed
        let mut ia = ch.min(a.count - 1);
        let mut ib = ch.min(b.count - 1);
        if m == Mutant::ClampA {
            ia = ch.min(a.count);
        }
        if m == Mutant::ClampB {
            ib = ch.min(b.count);
        }
        if n <= 0 {
            continue; // cmp [n],0; jle next channel
        }
        let n = n as usize;
        let dst_i = ch as usize;
        if a_const {
            let ca = a.consts[cidx(if m == Mutant::ConstIdx { ch.min(2) } else { ia })?];
            let pb = plane(b, ib)?; // the A-constant branch still holds B's plane in r14
            let d = out.get_mut(dst_i).ok_or(OpError::MissingOut)?;
            for i in 0..n {
                let y = *pb.get(i).ok_or(OpError::ShortPlane)?;
                d[i] = absd(ca - y, ABS_MASK, m);
            }
        } else if b_const {
            let cb = b.consts[cidx(ib)?];
            let pa = plane(a, ia)?;
            let d = out.get_mut(dst_i).ok_or(OpError::MissingOut)?;
            for i in 0..n {
                let x = *pa.get(i).ok_or(OpError::ShortPlane)?;
                let v = if m == Mutant::SwapConstB { cb - x } else { x - cb };
                d[i] = absd(v, ABS_MASK, m);
            }
        } else {
            let (pa, pb) = (plane(a, ia)?, plane(b, ib)?);
            let d = out.get_mut(dst_i).ok_or(OpError::MissingOut)?;
            for i in 0..n {
                let (x, y) = (*pa.get(i).ok_or(OpError::ShortPlane)?, *pb.get(i).ok_or(OpError::ShortPlane)?);
                let v = if m == Mutant::SwapPlane { y - x } else { x - y };
                d[i] = absd(v, ABS_MASK, m);
            }
        }
    }
    if n > 0 {
        if r14 {
            ext(true, p, a, b, out, m)?;
        }
        if r12 {
            ext(false, p, a, b, out, m)?;
        }
    }
    Ok(())
}

///  (r14, B side: dst[3] = |ONE - B3|) and (r12, A side: dst[3] = |A3 - ONE|).
fn ext(b_side: bool, p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    let n = p.n as usize;
    let o = if b_side { b } else { a };
    let ci = if m == Mutant::ConstIdx { 2 } else { 3 }; // this (B) / this (A)
    let d = out.get_mut(3).ok_or(OpError::MissingOut)?;
    let flip = m == Mutant::ExtOrder;
    let el = |x: f32, kvec: bool| -> f32 {
        let one = if kvec && m != Mutant::VecOneIgnored { p.vec_one } else { ONE };
        let mask = if kvec && m != Mutant::VecMaskIgnored { p.vec_mask } else { ABS_MASK };
        let v = if b_side != flip { one - x } else { x - one };
        if m == Mutant::AbsExtOff {
            v
        } else {
            absd(v, mask, m)
        }
    };
    if o.is_const {
        let c = o.consts[ci];
        let v = el(c, false);
        for i in 0..n {
            d[i] = v;
        }
        return Ok(());
    }
    let src = plane(o, 3)?;
    if src.len() < n {
        return Err(OpError::ShortPlane);
    }
    vec_kernel(d, src, n, p.dst3_phase_bytes, 16, 0x1d, m, el);
    Ok(())
}

/// 16 wide kernel (and): scalar head with ONE and the scalar mask, ymm core with the two uncovered vectors, scalar tail.
fn vec_kernel(d: &mut [f32], src: &[f32], n: usize, phase: u32, w: usize, thr: usize, m: Mutant, f: impl Fn(f32, bool) -> f32) {
    let thr = if m == Mutant::Cut28 { thr - 1 } else { thr };
    let (peel, e) = if n < w {
        (0, 0)
    } else if n < thr {
        (0, n & !(w - 1))
    } else {
        let ph = (phase & 0x1f) as usize;
        if ph != 0 && (ph & 3) != 0 {
            (0, 0)
        } else {
            let mut peel = if ph == 0 { 0 } else { (32 - ph) >> 2 };
            if m == Mutant::NoPeel {
                peel = 0;
            }
            if m == Mutant::PeelPlus1 {
                peel += 1;
            }
            if n < peel + w {
                (0, 0)
            } else {
                (peel, n - ((n - peel) & (w - 1)))
            }
        }
    };
    if e == 0 {
        for i in 0..n {
            d[i] = f(src[i], false);
        }
        return;
    }
    for i in 0..peel {
        d[i] = f(src[i], false);
    }
    for i in peel..e {
        d[i] = f(src[i], true);
    }
    for i in e..n {
        d[i] = f(src[i], false);
    }
}

/// Every stand-in of this port with its tier.
pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("IN2D-A1", "CAPTURED", "ONE 1.0f and the scalar abs mask 0x7fffffff (low lane of): bytes read by lookup.py (OP2 gate/lookup_ops.tsv)"),
        ("IN2D-A2", "MEASURED-DYLIB (STD-002)", "vector constant (32 B, uncovered in the image) = 1.0f x8, used by vsubps; explicit parameter vec_one; T2 runs 1.0 and 0.75"),
        ("IN2D-A3", "MEASURED-DYLIB (OP2-A3)", "vector mask (32 B, uncovered) = 0x7fffffff x8, used by vandps; explicit parameter vec_mask; T2 runs 0x7fffffff and 0x3fffffff"),
        ("IN2D-A4", "ROUTE", "AVX2 route only: feature word 0x9d97ff selects; AVX tier and SSE tier are not ported"),
        ("IN2D-A5", "HOST-FP-ASSUMED", "MXCSR round to nearest, no FTZ, no DAZ; the emulator follows the SDM, not silicon"),
        ("IN2D-A6", "ALIGN-DEPENDENT", "dst3_phase_bytes = (address of output plane 3) & 0x1f is an explicit input; only the extension kernels split on it, and it only matters when vec_one != 1.0 or vec_mask != 0x7fffffff"),
        ("IN2D-A7", "CALLEE-HOSTED (count getter)", "nout = result of virtual slot 31 is an explicit parameter (hooked ret_const in R-PARITY, W69); combo choice and type tables of the shared builder are uncovered and not rebuilt (INFERRED: nout is the output channel count of the chosen combo)"),
        ("IN2D-A8", "INFERRED", "this operand channel counts, constant flags,../.. constants, records at with plane c at the record's per-channel pointer: measured from the body, host fill not run"),
        ("IN2D-A9", "CONTRACT", "both operands flagged constant still reads B's plane (A-constant branch first); counts outside 1..4 with n > 0 and short planes return an error"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    const S: u32 = 0x7fc0ffee;
    fn outs(k: usize, n: usize) -> Vec<Vec<f32>> {
        (0..k).map(|_| vec![f32::from_bits(S); n]).collect()
    }
    fn op<'a>(count: i32, planes: &'a [&'a [f32]]) -> Operand<'a> {
        Operand { count, is_const: false, consts: [0.0; 4], planes }
    }
    fn cop(count: i32, c: [f32; 4]) -> Operand<'static> {
        Operand { count, is_const: true, consts: c, planes: &[] }
    }
    fn p(nout: i32, n: i32) -> Params {
        Params { nout, n, vec_one: 1.0, vec_mask: 0x7fff_ffff, dst3_phase_bytes: 0 }
    }
    fn run1(p: &Params, a: &Operand, b: &Operand, o: &mut Vec<Vec<f32>>) -> Result<(), OpError> {
        let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        render(p, a, b, &mut m)
    }

    #[test]
    fn hand_values_plane_plane() {
        // |3 - 4.5| = 1.5, |4.5 - 3| = 1.5, |-2 - 1| = 3
        let a = [3.0f32, 4.5, -2.0];
        let b = [4.5f32, 3.0, 1.0];
        let (pa, pb) = ([&a[..]], [&b[..]]);
        let mut o = outs(1, 4);
        run1(&p(1, 3), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
        assert_eq!((o[0][0], o[0][1], o[0][2]), (1.5, 1.5, 3.0));
        assert_eq!(o[0][3].to_bits(), S);
    }

    #[test]
    fn constants_and_order() {
        let x = [7.0f32, 8.0];
        let (pa, pb) = ([&x[..]], [&x[..]]);
        let mut o = outs(1, 2);
        run1(&p(1, 2), &cop(1, [100.0, 0.0, 0.0, 0.0]), &op(1, &pb), &mut o).unwrap();
        assert_eq!((o[0][0], o[0][1]), (93.0, 92.0));
        run1(&p(1, 2), &op(1, &pa), &cop(1, [10.0, 0.0, 0.0, 0.0]), &mut o).unwrap();
        assert_eq!((o[0][0], o[0][1]), (3.0, 2.0));
    }

    #[test]
    fn nan_keeps_payload_and_loses_sign() {
        let a = [f32::from_bits(0xffc0_1234)];
        let b = [1.0f32];
        let (pa, pb) = ([&a[..]], [&b[..]]);
        let mut o = outs(1, 1);
        run1(&p(1, 1), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
        assert_eq!(o[0][0].to_bits(), 0x7fc0_1234);
        // -0 - 0 = -0 -> +0 ; inf - inf = NaN
        let a = [-0.0f32, f32::INFINITY];
        let b = [0.0f32, f32::INFINITY];
        let (pa, pb) = ([&a[..]], [&b[..]]);
        let mut o = outs(1, 2);
        run1(&p(1, 2), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
        assert_eq!(o[0][0].to_bits(), 0);
        assert!(o[0][1].is_nan());
    }

    #[test]
    fn extension_channel_three() {
        let n = 40usize;
        let mk = |c: usize| (0..n).map(|i| i as f32 * 0.25 + c as f32 + 1.0).collect::<Vec<f32>>();
        let ap: Vec<Vec<f32>> = (0..3).map(mk).collect();
        let bp: Vec<Vec<f32>> = (0..4).map(|c| mk(c + 10)).collect();
        let (ar, br): (Vec<&[f32]>, Vec<&[f32]>) = (ap.iter().map(|v| v.as_slice()).collect(), bp.iter().map(|v| v.as_slice()).collect());
        let mut o = outs(4, 300);
        run1(&Params { nout: 4, n: n as i32, vec_one: 0.75, vec_mask: 0x7fff_ffff, dst3_phase_bytes: 0 }, &op(3, &ar), &op(4, &br), &mut o).unwrap();
        let b3 = |i: usize| i as f32 * 0.25 + 14.0;
        assert_eq!(o[3][0], (0.75 - b3(0)).abs()); // vector lane uses vec_one
        assert_eq!(o[3][31], (0.75 - b3(31)).abs());
        assert_eq!(o[3][32], (1.0 - b3(32)).abs()); // scalar tail uses ONE
        assert_eq!(o[3][n].to_bits(), S);
        // A side (4,3): |A3 - ONE|
        let ap: Vec<Vec<f32>> = (0..4).map(mk).collect();
        let bp: Vec<Vec<f32>> = (0..3).map(|c| mk(c + 10)).collect();
        let (ar, br): (Vec<&[f32]>, Vec<&[f32]>) = (ap.iter().map(|v| v.as_slice()).collect(), bp.iter().map(|v| v.as_slice()).collect());
        let mut o = outs(4, 300);
        run1(&p(4, n as i32), &op(4, &ar), &op(3, &br), &mut o).unwrap();
        assert_eq!(o[3][7], (7.0f32 * 0.25 + 4.0 - 1.0).abs());
    }

    #[test]
    fn vec_mask_acts_only_in_the_core() {
        let n = 40usize;
        let ap: Vec<Vec<f32>> = (0..3).map(|_| vec![1.0f32; n]).collect();
        let bp: Vec<Vec<f32>> = (0..4).map(|_| vec![3.0f32; n]).collect();
        let (ar, br): (Vec<&[f32]>, Vec<&[f32]>) = (ap.iter().map(|v| v.as_slice()).collect(), bp.iter().map(|v| v.as_slice()).collect());
        let mut o = outs(4, 300);
        // mask 0x3fffffff clears bit 30: |1 - 3| = 2 (0x40000000) -> 0 in the core (lanes 0..31), tail keeps 2
        run1(&Params { nout: 4, n: n as i32, vec_one: 1.0, vec_mask: 0x3fff_ffff, dst3_phase_bytes: 0 }, &op(3, &ar), &op(4, &br), &mut o).unwrap();
        assert_eq!(o[3][0], 0.0);
        assert_eq!(o[3][33], 2.0);
    }

    #[test]
    fn degenerate_and_errors() {
        let a = [1.0f32; 4];
        let pa = [&a[..]];
        let mut o = outs(1, 4);
        run1(&p(0, 4), &op(1, &pa), &op(1, &pa), &mut o).unwrap();
        assert_eq!(o[0][0].to_bits(), S);
        run1(&p(1, -3), &op(1, &pa), &op(1, &pa), &mut o).unwrap();
        assert_eq!(o[0][0].to_bits(), S);
        assert_eq!(run1(&p(1, 4), &op(1, &[]), &op(1, &pa), &mut o), Err(OpError::MissingPlane));
        assert_eq!(run1(&p(1, 9), &op(1, &pa), &op(1, &pa), &mut o), Err(OpError::MissingOut));
    }

    #[test]
    fn determinism_and_identity() {
        let a: Vec<f32> = (0..77).map(|i| (i as f32 - 30.0) * 1.37).collect();
        let (pa, pb) = ([a.as_slice()], [a.as_slice()]);
        let mut o1 = outs(1, 80);
        let mut o2 = outs(1, 80);
        run1(&p(1, 77), &op(1, &pa), &op(1, &pb), &mut o1).unwrap();
        run1(&p(1, 77), &op(1, &pa), &op(1, &pb), &mut o2).unwrap();
        assert_eq!(o1[0][..77].iter().map(|v| v.to_bits()).collect::<Vec<_>>(), o2[0][..77].iter().map(|v| v.to_bits()).collect::<Vec<_>>());
        assert!(o1[0][..77].iter().all(|v| *v == 0.0)); // diff of a plane with itself is +0
        assert_eq!(assumptions().len(), 9);
    }
}
