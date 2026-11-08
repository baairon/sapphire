//! overlay_super: native port of the builtin class `overlay_super` (the registry also holds it under `overlay_super_if_needed`).
//! Slot 53 dispatcher at 0x4e84d0, 256-bit (feature word 0x9d97ff) body at 0x4e8520 to 0x4e8c2f.
//! std only, no unsafe, explicit parameters (no Default). Operand layout and combo base are those of the operator classes `* + -`.
//!
//! Per output channel ch and element i: a = A[min(ch, countA-1)][i] (or the A constant), b = B[min(ch, countB-1)][i] (or the B constant).
//!   b <= 0.5 or b is NaN:  out = (a * b) + (a * b)
//!   else if a > 1.0 or b > 1.0:  out = maxs(a, b)
//!   else: t = fnmadd(a, b, a) = a - a * b (one rounding), d = b - 1.0, out = (((b + d) + t) + t)
//! (3,4): channel 3 from B channel 3; (4,3): channel 3 = a3 > 1.0 ? a3 : 1.0.

use fx_common::nanop::NanOps;

pub const ONE: f32 = 1.0;
pub const HALF: f32 = 0.5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mutant {
    None,
    Unfused,
    HalfGe,
    OneGeA,
    OneGeB,
    OnlyATest,
    OnlyBTest,
    MaxSwap,
    DiffSwap,
    NoDouble,
    SumOrder,
    TwoT,
    ClampA,
    ClampB,
    Fixed3Off,
    R14Swap,
    R12Off,
    Nch2,
    ConstIdx,
    NoConstA,
    NoConstB,
    ExtBNoNeg,
    ExtBHalfGe,
    ExtBMax,
    ExtASimple,
    ExtAGe,
    ExtBDiffSwap,
    ExtBLowPlain,
    ExtAMaxSwap,
    NanNeg,
}

impl Mutant {
    pub fn from_name(s: &str) -> Option<Mutant> {
        use Mutant::*;
        Some(match s {
            "none" => None,
            "unfused" => Unfused,
            "half_ge" => HalfGe,
            "one_ge_a" => OneGeA,
            "one_ge_b" => OneGeB,
            "only_a_test" => OnlyATest,
            "only_b_test" => OnlyBTest,
            "max_swap" => MaxSwap,
            "diff_swap" => DiffSwap,
            "no_double" => NoDouble,
            "sum_order" => SumOrder,
            "two_t" => TwoT,
            "clamp_a" => ClampA,
            "clamp_b" => ClampB,
            "fixed3_off" => Fixed3Off,
            "r14_swap" => R14Swap,
            "r12_off" => R12Off,
            "nch2" => Nch2,
            "const_idx" => ConstIdx,
            "no_const_a" => NoConstA,
            "no_const_b" => NoConstB,
            "ext_b_no_neg" => ExtBNoNeg,
            "ext_b_half_ge" => ExtBHalfGe,
            "ext_b_max" => ExtBMax,
            "ext_a_simple" => ExtASimple,
            "ext_a_ge" => ExtAGe,
            "ext_b_diff_swap" => ExtBDiffSwap,
            "ext_b_low_plain" => ExtBLowPlain,
            "ext_a_max_swap" => ExtAMaxSwap,
            "nan_neg" => NanNeg,
            _ => return Option::None,
        })
    }
}

/// One operand: count (channels), constant flag, constants, and the planes of the operand record.
pub struct Operand<'a> {
    pub count: i32,
    pub is_const: bool,
    pub consts: [f32; 4],
    pub planes: &'a [&'a [f32]],
}

/// `nout` is the result of the base virtual slot 31 (the output channel count of the chosen combo entry); `n` is the element count.
pub struct Params {
    pub nout: i32,
    pub n: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OverlayError {
    MissingPlane,
    ShortPlane,
    ConstIndex,
    MissingOut,
}

/// vmaxss d, d, s: d > s ? d : s (second operand on NaN or equal).
#[inline]
fn maxs(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}

/// One element of the main loops. `b_first` selects the operand order of the low-branch product: the route with the A operand constant
/// multiplies b by a, every other route multiplies a by b (the first operand's NaN wins when both are NaN).
fn overlay(a: f32, b: f32, b_first: bool, m: Mutant) -> f32 {
    let low = if m == Mutant::HalfGe { !(b >= HALF) } else { !(b > HALF) };
    if low {
        let p = if b_first { b.fmul(a) } else { a.fmul(b) };
        return if m == Mutant::NoDouble { p } else { p.fadd(p) };
    }
    let big = match m {
        Mutant::OneGeA => a >= ONE || b > ONE,
        Mutant::OneGeB => a > ONE || b >= ONE,
        Mutant::OnlyATest => a > ONE,
        Mutant::OnlyBTest => b > ONE,
        _ => a > ONE || b > ONE,
    };
    if big {
        return if m == Mutant::MaxSwap { maxs(b, a) } else { maxs(a, b) };
    }
    // vfnmadd231ss: -(a*b) + a, one rounding. The negation applies to the product only: a NaN input is returned quieted with its
    // own sign (the first NaN operand in the order dest(a), a, b); b cannot be NaN here (the low branch took it).
    let t = if m == Mutant::Unfused {
        a - a * b
    } else if a.is_nan() && m != Mutant::NanNeg {
        f32::from_bits(a.to_bits() | 0x0040_0000)
    } else {
        let r = (-a).mul_add(b, a);
        // a = -inf gives inf - inf, a NaN no operand supplies: the x86 default NaN on every processor
        if r.is_nan() && !a.is_nan() { f32::from_bits(0xffc0_0000) } else { r }
    };
    let d = if m == Mutant::DiffSwap { ONE - b } else { b - ONE };
    let s = b + d;
    if m == Mutant::TwoT {
        return (s + t) + (t + t);
    }
    if m == Mutant::SumOrder {
        return (t + t) + s;
    }
    (s + t) + t
}

fn plane<'a>(o: &Operand<'a>, idx: i32) -> Result<&'a [f32], OverlayError> {
    if idx < 0 {
        return Err(OverlayError::ConstIndex);
    }
    o.planes.get(idx as usize).copied().ok_or(OverlayError::MissingPlane)
}

fn cidx(idx: i32) -> Result<usize, OverlayError> {
    if (0..=3).contains(&idx) {
        Ok(idx as usize)
    } else {
        Err(OverlayError::ConstIndex)
    }
}

pub fn render(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]]) -> Result<(), OverlayError> {
    run(p, a, b, out, Mutant::None)
}

/// The whole 256-bit route of slot 53.
pub fn run(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OverlayError> {
    // r13 = (countA == 3 && countB == 4), rbx = (countA == 4 && countB == 3)
    let (mut r13, mut rbx) = (a.count == 3 && b.count == 4, a.count == 4 && b.count == 3);
    if m == Mutant::R14Swap {
        r13 = a.count == 4 && b.count == 3;
    }
    if m == Mutant::R12Off {
        rbx = false;
    }
    let fixed3 = if m == Mutant::Fixed3Off { r13 || rbx } else { r13 || rbx || p.nout == 3 };
    let mut nch: i32 = if fixed3 {
        3
    } else if p.nout > 0 {
        p.nout
    } else {
        0
    };
    if m == Mutant::Nch2 && nch > 2 {
        nch = 2;
    }
    let n = p.n;
    if n > 0 {
        for ch in 0..nch {
            if out.get(ch as usize).map_or(true, |d| d.len() < n as usize) {
                return Err(OverlayError::MissingOut);
            }
        }
        if (r13 || rbx) && out.get(3).map_or(true, |d| d.len() < n as usize) {
            return Err(OverlayError::MissingOut);
        }
    }
    let (a_const, b_const) = (a.is_const && m != Mutant::NoConstA, b.is_const && m != Mutant::NoConstB);
    for ch in 0..nch {
        let mut ia = ch.min(a.count - 1);
        let mut ib = ch.min(b.count - 1);
        if m == Mutant::ClampA {
            ia = ch.min(a.count);
        }
        if m == Mutant::ClampB {
            ib = ch.min(b.count);
        }
        if n <= 0 {
            continue;
        }
        let n = n as usize;
        let dst_i = ch as usize;
        if a_const {
            // the A constant flag is tested first; the B plane is read even if B is flagged constant
            let ca = a.consts[cidx(ia)?];
            let pb = plane(b, ib)?;
            let d = out.get_mut(dst_i).ok_or(OverlayError::MissingOut)?;
            for i in 0..n {
                d[i] = overlay(ca, *pb.get(i).ok_or(OverlayError::ShortPlane)?, true, m);
            }
        } else if b_const {
            let cb = b.consts[cidx(ib)?];
            let pa = plane(a, ia)?;
            let d = out.get_mut(dst_i).ok_or(OverlayError::MissingOut)?;
            for i in 0..n {
                d[i] = overlay(*pa.get(i).ok_or(OverlayError::ShortPlane)?, cb, false, m);
            }
        } else {
            let (pa, pb) = (plane(a, ia)?, plane(b, ib)?);
            let d = out.get_mut(dst_i).ok_or(OverlayError::MissingOut)?;
            for i in 0..n {
                d[i] = overlay(*pa.get(i).ok_or(OverlayError::ShortPlane)?, *pb.get(i).ok_or(OverlayError::ShortPlane)?, false, m);
            }
        }
    }
    if n > 0 {
        if r13 {
            ext(true, p, a, b, out, m)?;
        }
        if rbx {
            ext(false, p, a, b, out, m)?;
        }
    }
    Ok(())
}

/// Channel 3 when the counts are (3,4) (b_side, from B) or (4,3) (A side, from A).
fn ext(b_side: bool, p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OverlayError> {
    let n = p.n as usize;
    let o = if b_side { b } else { a };
    let ci = if m == Mutant::ConstIdx { 2 } else { 3 };
    let d = out.get_mut(3).ok_or(OverlayError::MissingOut)?;
    let el = |x: f32| -> f32 {
        if b_side {
            let low = if m == Mutant::ExtBHalfGe { !(x >= HALF) } else { !(x > HALF) };
            if low {
                return if m == Mutant::ExtBLowPlain { x } else { x + x };
            }
            if x > ONE {
                return if m == Mutant::ExtBMax { x } else { maxs(ONE, x) };
            }
            let dd = if m == Mutant::ExtBDiffSwap { ONE - x } else { x - ONE };
            let nd = if m == Mutant::ExtBNoNeg { dd } else { -dd };
            ((x + dd) + nd) + nd
        } else if m == Mutant::ExtASimple {
            x
        } else if m == Mutant::ExtAGe {
            if x >= ONE {
                x
            } else {
                ONE
            }
        } else if m == Mutant::ExtAMaxSwap {
            maxs(ONE, x)
        } else {
            maxs(x, ONE)
        }
    };
    if o.is_const {
        let v = el(o.consts[ci]);
        for i in 0..n {
            d[i] = v;
        }
        return Ok(());
    }
    let src = plane(o, 3)?;
    if src.len() < n {
        return Err(OverlayError::ShortPlane);
    }
    for i in 0..n {
        d[i] = el(src[i]);
    }
    Ok(())
}

pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("OV-A1", "MEASURED-VALUE", "constants 1.0f (cell 0x15e850c) and 0.5f (cell 0x15e8514) (value read from the reference run)"),
        ("OV-A2", "ROUTE", "256-bit route only: feature word 0x9d97ff; the SSE route at 0x4e8c30 (separate multiply and add, no fused site) is not ported"),
        ("OV-A3", "HOST-FP-ASSUMED", "round to nearest, no flush-to-zero, no denormals-are-zero"),
        ("OV-A4", "CALLEE-HOSTED (count getter)", "nout = result of virtual slot 31 (output channel count of the chosen combo entry) is an explicit parameter; combo table from the shared operator block"),
        ("OV-A5", "INFERRED", "this+0x40/+0x70 are operand channel counts, +0x44/+0x74 the constant flags, +0x4c../+0x7c.. the constants, records at +0x60/+0x90 with plane c at rec+0x20+8c"),
        ("OV-A6", "CONTRACT", "both operands flagged constant still reads B's plane (A-constant branch first); counts outside 1..4 with n > 0 and short planes return an error"),
    ]
}
