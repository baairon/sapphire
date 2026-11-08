//! screen: native port of the builtin class `screen` (vtable 0x1af9f10). Slot 53 dispatcher at 0x4e4450, 256-bit (feature word 0x9d97ff)
//! body at 0x4e44b0 to 0x4e4a2b. std only, no unsafe, explicit parameters (no Default).
//!
//! Per output channel ch and element i: a = A[min(ch, countA-1)][i], b = B[min(ch, countB-1)][i] (or constants)
//!   out = fma(1 - a, b, a)   (vsubss, then one fused multiply-add vfmadd231ss)
//! (3,4): channel 3 = 1.0 filled by a 16 wide kernel (the vector constant lanes in the ymm core); (4,3): channel 3 = (1.0 + a3) - a3.

use fx_common::nanop::{fma231, NanOps};

pub const ONE: f32 = 1.0;
pub const HALF: f32 = 0.5;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mutant {
    None,
    Unfused,
    OneMinusB,
    SwapAb,
    ClampA,
    ClampB,
    Fixed3Off,
    R14Swap,
    R12Off,
    Nch2,
    ConstIdx,
    NoConstA,
    NoConstB,
    Cut28,
    NoPeel,
    PeelPlus1,
    VecOneIgnored,
    ExtASimple,
    ExtAOrder,
    OrderMulAdd,
}

impl Mutant {
    pub fn from_name(s: &str) -> Option<Mutant> {
        use Mutant::*;
        Some(match s {
            "none" => None,
            "unfused" => Unfused,
            "one_minus_b" => OneMinusB,
            "swap_ab" => SwapAb,
            "clamp_a" => ClampA,
            "clamp_b" => ClampB,
            "fixed3_off" => Fixed3Off,
            "r14_swap" => R14Swap,
            "r12_off" => R12Off,
            "nch2" => Nch2,
            "const_idx" => ConstIdx,
            "no_const_a" => NoConstA,
            "no_const_b" => NoConstB,
            "cut28" => Cut28,
            "no_peel" => NoPeel,
            "peel_plus1" => PeelPlus1,
            "vec_one_ignored" => VecOneIgnored,
            "ext_a_simple" => ExtASimple,
            "ext_a_order" => ExtAOrder,
            "order_mul_add" => OrderMulAdd,
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

/// `vec_one` = the 32 byte vector constant (8 lanes of one value, 1.0f), `dst3_phase_bytes` = (address of output plane 3) & 0x1f.
/// `nout` is the result of the base virtual slot 31 (the output channel count of the chosen combo entry); `n` is the element count.
pub struct Params {
    pub nout: i32,
    pub n: i32,
    pub vec_one: f32,
    pub dst3_phase_bytes: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ScreenError {
    MissingPlane,
    ShortPlane,
    ConstIndex,
    MissingOut,
}

/// One element of the main loops.
fn screen(a: f32, b: f32, m: Mutant) -> f32 {
    let (a, b) = if m == Mutant::SwapAb { (b, a) } else { (a, b) };
    let om = if m == Mutant::OneMinusB { ONE - b } else { ONE - a };
    match m {
        Mutant::Unfused => om * b + a,
        Mutant::OneMinusB => om.mul_add(a, b),
        Mutant::OrderMulAdd => om.mul_add(a, b),
        // vfmadd231ss d=a, s2=(1 - a), s3=b: (1 - a) * b + a, one rounding; with NaN operands the first of a, (1 - a), b wins, quieted
        _ => fma231(a, om, b),
    }
}

fn plane<'a>(o: &Operand<'a>, idx: i32) -> Result<&'a [f32], ScreenError> {
    if idx < 0 {
        return Err(ScreenError::ConstIndex);
    }
    o.planes.get(idx as usize).copied().ok_or(ScreenError::MissingPlane)
}

fn cidx(idx: i32) -> Result<usize, ScreenError> {
    if (0..=3).contains(&idx) {
        Ok(idx as usize)
    } else {
        Err(ScreenError::ConstIndex)
    }
}

pub fn render(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]]) -> Result<(), ScreenError> {
    run(p, a, b, out, Mutant::None)
}

/// The whole 256-bit route of slot 53.
pub fn run(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), ScreenError> {
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
                return Err(ScreenError::MissingOut);
            }
        }
        if (r13 || rbx) && out.get(3).map_or(true, |d| d.len() < n as usize) {
            return Err(ScreenError::MissingOut);
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
            let d = out.get_mut(dst_i).ok_or(ScreenError::MissingOut)?;
            for i in 0..n {
                d[i] = screen(ca, *pb.get(i).ok_or(ScreenError::ShortPlane)?, m);
            }
        } else if b_const {
            let cb = b.consts[cidx(ib)?];
            let pa = plane(a, ia)?;
            let d = out.get_mut(dst_i).ok_or(ScreenError::MissingOut)?;
            for i in 0..n {
                d[i] = screen(*pa.get(i).ok_or(ScreenError::ShortPlane)?, cb, m);
            }
        } else {
            let (pa, pb) = (plane(a, ia)?, plane(b, ib)?);
            let d = out.get_mut(dst_i).ok_or(ScreenError::MissingOut)?;
            for i in 0..n {
                d[i] = screen(*pa.get(i).ok_or(ScreenError::ShortPlane)?, *pb.get(i).ok_or(ScreenError::ShortPlane)?, m);
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

/// Channel 3 when the counts are (3,4) (fill with one, 16 wide kernel) or (4,3) ((1 + a3) - a3 from A).
fn ext(b_side: bool, p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), ScreenError> {
    let n = p.n as usize;
    let ci = if m == Mutant::ConstIdx { 2 } else { 3 };
    let d = out.get_mut(3).ok_or(ScreenError::MissingOut)?;
    if b_side {
        // both the constant and the plane variant of B fill the plane with one; the core uses the vector constant
        let el = |_x: f32, kvec: bool| -> f32 {
            if kvec && m != Mutant::VecOneIgnored {
                p.vec_one
            } else {
                ONE
            }
        };
        let dummy = vec![0.0f32; n];
        vec_kernel(d, &dummy, n, p.dst3_phase_bytes, m, &el);
        return Ok(());
    }
    let el = |x: f32| -> f32 {
        match m {
            Mutant::ExtASimple => ONE,
            Mutant::ExtAOrder => (ONE - x) + x,
            _ => ONE.fadd(x).fsub(x),
        }
    };
    if a.is_const {
        let v = el(a.consts[ci]);
        for i in 0..n {
            d[i] = v;
        }
        return Ok(());
    }
    let src = plane(a, 3)?;
    if src.len() < n {
        return Err(ScreenError::ShortPlane);
    }
    for i in 0..n {
        d[i] = el(src[i]);
    }
    let _ = b;
    Ok(())
}

/// The 16 wide B-side kernel : scalar head with ONE, ymm core with the `vec_one` vector (two vmaxps ymm per step), scalar tail.
/// n < 16 all scalar; n < 0x1d no peel (core over n & ~15); else peel from the plane 3 phase (0: none; not a multiple of 4: all scalar;
/// else (32 - phase) / 4 elements), core end n - ((n - peel) & 15), all scalar if n < peel + 16.
fn vec_kernel(d: &mut [f32], src: &[f32], n: usize, phase: u32, m: Mutant, f: &dyn Fn(f32, bool) -> f32) {
    let (w, thr) = (16usize, if m == Mutant::Cut28 { 0x1c } else { 0x1d });
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

pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("SC-A1", "MEASURED-VALUE", "constant 1.0f (cell 0x15e850c) (value read from the reference run)"),
        ("SC-A7", "SEMANTIC (explicit parameter vec_one)", "32 byte vector constant (cell 0x1605560) used by the (3,4) fill kernel core; basis: the scalar head and tail of the same loop store the immediate 1.0f; the tests use 1.0 and 0.75"),
        ("SC-A8", "ALIGN-DEPENDENT", "dst3_phase_bytes = (address of output plane 3) & 0x1f is an explicit input (matters only when vec_one != 1.0)"),
        ("SC-A2", "ROUTE", "256-bit route only: feature word 0x9d97ff; the 128-bit route at 0x4e4aa0 and the SSE route at 0x4e5070 are not ported"),
        ("SC-A3", "HOST-FP-ASSUMED", "round to nearest, no flush-to-zero, no denormals-are-zero"),
        ("SC-A4", "CALLEE-HOSTED (count getter)", "nout = result of virtual slot 31 (output channel count of the chosen combo entry) is an explicit parameter; combo table from the shared operator block"),
        ("SC-A5", "INFERRED", "this+0x40/+0x70 are operand channel counts, +0x44/+0x74 the constant flags, +0x4c../+0x7c.. the constants, records at +0x60/+0x90 with plane c at rec+0x20+8c"),
        ("SC-A6", "CONTRACT", "both operands flagged constant still reads B's plane (A-constant branch first); counts outside 1..4 with n > 0 and short planes return an error"),
    ]
}
