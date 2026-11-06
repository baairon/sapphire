//! Shared helpers of the classes `max`, `min` (binary, shape of the bodies) and `abs`, `negate` (unary, one record).
//! Listing offsets in comments: max body, min body (identical instruction for instruction after vmax <-> vmin, MEASURED
//! by a normalised diff of the two 423 line bodies), abs body, negate body . VA = listing offset.
//! The vector split used by the extension kernels and by the unary bodies is one and the same shape (threshold 16 and 0x1d, peel from the
//! destination phase, 16 floats per step, scalar head and tail): `split()` below.
pub use ops_arith::{OpError, Operand, Params, ONE};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BKind {
    Max,
    Min,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UKind {
    Abs,
    Negate,
}

/// Mutants of the binary body (T2 kill test) and of the unary bodies.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mutant {
    None,
    // binary
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
    ExtOrder,
    OtherOp,
    // shared vector split
    Cut28,
    NoPeel,
    PeelPlus1,
    VecOneIgnored,
    // unary
    VecMaskIgnored,
    CountMinus1,
    PhaseZero,
    ScalarMaskLsb,
    TailShort,
    HeadSkipped,
    Cut15,
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
            "ext_order" => ExtOrder,
            "other_op" => OtherOp,
            "cut28" => Cut28,
            "no_peel" => NoPeel,
            "peel_plus1" => PeelPlus1,
            "vec_one_ignored" => VecOneIgnored,
            "vec_mask_ignored" => VecMaskIgnored,
            "count_minus1" => CountMinus1,
            "phase_zero" => PhaseZero,
            "scalar_mask_lsb" => ScalarMaskLsb,
            "tail_short" => TailShort,
            "head_skipped" => HeadSkipped,
            "cut15" => Cut15,
            _ => return Option::None,
        })
    }
}

/// x86 MAXSS / MAXPS (SDM): dst = (a > b) ? a : b with a = first source, b = second source. A NaN in either operand, or a pair of zeros of
/// either sign, returns the SECOND source. Operand order is therefore part of the semantics (silicon NaN rule).
#[inline]
pub fn x86_max(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

/// x86 MINSS / MINPS: dst = (a < b) ? a : b, the second source on NaN or on a pair of zeros.
#[inline]
pub fn x86_min(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

impl BKind {
    pub fn f(self, x: f32, y: f32, m: Mutant) -> f32 {
        let k = if m == Mutant::OtherOp {
            match self {
                BKind::Max => BKind::Min,
                BKind::Min => BKind::Max,
            }
        } else {
            self
        };
        match k {
            BKind::Max => x86_max(x, y),
            BKind::Min => x86_min(x, y),
        }
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

/// The whole slot-53 AVX2 body of `maxmin` . `p.dst3_phase_bytes` = address of output plane 3 & 0x1f.
pub fn run_bin(k: BKind, p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    // r14 = (countA == 3 [edx is countA]), see : r14 = (A3 && B4), r12 = (A4 && B3)
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
    // (r14|r12) != 0 or nout == 3 take the fixed 3 channel loop; else nout channels (jle skips)
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
            // xmm0 = Aconst[ia]; vmaxss xmm0,xmm0,[B plane] -> f(Aconst, B)
            let ca = a.consts[cidx(ia)?];
            let pb = plane(b, ib)?;
            let d = out.get_mut(dst_i).ok_or(OpError::MissingOut)?;
            for i in 0..n {
                let y = *pb.get(i).ok_or(OpError::ShortPlane)?;
                d[i] = k.f(ca, y, m);
            }
        } else if b_const {
            // xmm0 = A plane; vmaxss xmm0,xmm0,[Bconst[ib]] -> f(A, Bconst) (no operand reload, unlike + and *)
            let cb = b.consts[cidx(ib)?];
            let pa = plane(a, ia)?;
            let d = out.get_mut(dst_i).ok_or(OpError::MissingOut)?;
            for i in 0..n {
                let x = *pa.get(i).ok_or(OpError::ShortPlane)?;
                d[i] = if m == Mutant::SwapConstB { k.f(cb, x, m) } else { k.f(x, cb, m) };
            }
        } else {
            let (pa, pb) = (plane(a, ia)?, plane(b, ib)?);
            let d = out.get_mut(dst_i).ok_or(OpError::MissingOut)?;
            for i in 0..n {
                let (x, y) = (*pa.get(i).ok_or(OpError::ShortPlane)?, *pb.get(i).ok_or(OpError::ShortPlane)?);
                d[i] = if m == Mutant::SwapPlane { k.f(y, x, m) } else { k.f(x, y, m) };
            }
        }
    }
    if n > 0 {
        if r14 {
            ext(k, true, p, a, b, out, m)?;
        }
        if r12 {
            ext(k, false, p, a, b, out, m)?;
        }
    }
    Ok(())
}

///  (r14, B side: dst[3] = f(ONE, B3)) and (r12, A side: dst[3] = f(A3, ONE)); vector core with the `vec_one` vector.
fn ext(k: BKind, b_side: bool, p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    let n = p.n as usize;
    let o = if b_side { b } else { a };
    let ci = if m == Mutant::ConstIdx { 2 } else { 3 }; // this (B) / this (A)
    let d = out.get_mut(3).ok_or(OpError::MissingOut)?;
    let flip = m == Mutant::ExtOrder;
    // one element: x from the operand (plane or constant), `one` = ONE (scalar code) or vec_one (vector core)
    let el = |x: f32, kvec: bool| -> f32 {
        let one = if kvec && m != Mutant::VecOneIgnored { p.vec_one } else { ONE };
        if b_side != flip {
            k.f(one, x, m) // vmaxss xmm1,ONE,[B3] and vmaxps ymm1,ymm0,[B3] 
        } else {
            k.f(x, one, m) // vmaxss xmm1,A3,ONE and vmaxps ymm2,ymm1,ymm0 
        }
    };
    if o.is_const {
        let v = el(o.consts[ci], false);
        for i in 0..n {
            d[i] = v;
        }
        return Ok(());
    }
    let src = plane(o, 3)?;
    if src.len() < n {
        return Err(OpError::ShortPlane);
    }
    split(d, src, n, p.dst3_phase_bytes, m, |x, kv| el(x, kv));
    Ok(())
}

/// The vector split of every body in this crate (max/min extension, abs, negate):
/// n < 16: all scalar; n < 0x1d: no peel, vector end n & ~15; else peel from the destination phase (0: none; not a multiple of 4: all
/// scalar; else (32 - ph) >> 2), all scalar when n < peel + 16, vector end n - ((n - peel) & 15). Scalar head [0, peel), core [peel, e),
/// scalar tail [e, n). `f(x, true)` is the vector lane, `f(x, false)` the scalar lane.
pub fn split(d: &mut [f32], src: &[f32], n: usize, phase: u32, m: Mutant, f: impl Fn(f32, bool) -> f32) {
    let (w, small) = (16usize, if m == Mutant::Cut15 { 15usize } else { 16 });
    let thr = if m == Mutant::Cut28 { 0x1c } else { 0x1d };
    let (peel, e) = if n < small {
        (0, 0)
    } else if n < thr {
        (0, n & !(w - 1))
    } else {
        let ph = (phase & 0x1f) as usize;
        let ph = if m == Mutant::PhaseZero { 0 } else { ph };
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
    let n_end = if m == Mutant::TailShort && n > 0 { n - 1 } else { n };
    if e == 0 {
        for i in 0..n_end {
            d[i] = f(src[i], false);
        }
        return;
    }
    for i in 0..peel {
        if m == Mutant::HeadSkipped && i == 0 {
            continue;
        }
        d[i] = f(src[i], false);
    }
    for i in peel..e {
        d[i] = f(src[i], true);
    }
    for i in e..n_end {
        d[i] = f(src[i], false);
    }
}

/// Explicit parameters of the unary classes. `n` = [info]. `vec_mask` = the 32 byte ymm constant (abs: OP2-A3,
/// negate: OP2-A4), 8 equal lanes, UNCOVERED, SEMANTIC, no default. `dst_phase_bytes[c]` = (address of output plane c) & 0x1f.
pub struct UParams {
    pub n: i32,
    pub vec_mask: u32,
    pub dst_phase_bytes: [u32; 4],
}

/// CAPTURED low lanes of the 16 byte scalar masks (0x7fffffff, -> 0x80000000, VA).
pub const ABS_SCALAR_MASK: u32 = 0x7fff_ffff;
pub const NEG_SCALAR_MASK: u32 = 0x8000_0000;

/// The whole slot-53 AVX2 body of `absnegate` : `count = [this]` channels, input plane c at
/// `[this] (one pointer per channel c)`output plane c at `outs[c]n` floats each; abs ANDs the bits with a mask, negate XORs the sign bit.
/// `planes` are the input planes of the single operand record.
pub fn run_un(k: UKind, p: &UParams, count: i32, planes: &[&[f32]], out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    let nch = if m == Mutant::CountMinus1 { count - 1 } else { count };
    let n = p.n;
    let sm0 = match k {
        UKind::Abs => ABS_SCALAR_MASK,
        UKind::Negate => NEG_SCALAR_MASK,
    };
    let sm = if m == Mutant::ScalarMaskLsb { sm0 ^ 1 } else { sm0 };
    let vm = if m == Mutant::VecMaskIgnored { sm0 } else { p.vec_mask };
    for ch in 0..nch.max(0) {
        if n <= 0 {
            continue; // test rdi; jle next channel
        }
        let nn = n as usize;
        let src = planes.get(ch as usize).copied().ok_or(OpError::MissingPlane)?;
        if src.len() < nn {
            return Err(OpError::ShortPlane);
        }
        let d = out.get_mut(ch as usize).ok_or(OpError::MissingOut)?;
        if d.len() < nn {
            return Err(OpError::MissingOut);
        }
        let phase = p.dst_phase_bytes[(ch as usize).min(3)];
        let ap = |x: f32, vector: bool| -> f32 {
            let mask = if vector { vm } else { sm };
            let bits = x.to_bits();
            f32::from_bits(match k {
                UKind::Abs => bits & mask,
                UKind::Negate => bits ^ mask,
            })
        };
        split(d, src, nn, phase, m, ap);
    }
    Ok(())
}

pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("OP2-A1", "CAPTURED", "ONE 1.0f for the max/min scalar extension; the scalar masks of abs (low lane 0x7fffffff) and negate (low lane 0x80000000): bytes read by lookup.py (gate/lookup_ops.tsv). Only the low lane is used (vmovss zeroes the rest and only the low lane is stored)"),
        ("OP2-A2", "SEMANTIC (explicit parameter vec_one)", "vector constant (uncovered) used by the max and min extension kernels as the first (B side) or second (A side) source; basis: the scalar head and tail of the same loop use captured ONE over the same array; T2 runs 1.0 and 0.75; sensitivity: paired runs"),
        ("OP2-A3", "SEMANTIC (new cell, explicit parameter vec_mask)", "abs vector mask ymm (uncovered): vandps with the data; basis: the scalar head and tail of the same loop AND the same array with the captured 0x7fffffff, so the core must be 0x7fffffff x8 for a loop that computes |x| element-wise; T2 runs 0x7fffffff and a second value"),
        ("OP2-A4", "SEMANTIC (explicit parameter vec_mask)", "negate vector mask ymm (uncovered, assumed 0x80000000 x8); negate vxorps is its consumer; basis as OP2-A3 with the captured scalar mask 0x80000000"),
        ("OP2-A5", "ROUTE", "AVX2 route only: feature word 0x9d97ff selects max, min, abs, negate; AVX tiers and SSE tiers are not ported"),
        ("OP2-A6", "HOST-FP-ASSUMED", "MXCSR round to nearest, no FTZ, no DAZ (max/min compare, abs/negate are bit operations); the emulator follows the SDM, not silicon"),
        ("OP2-A7", "ALIGN-DEPENDENT", "dst3_phase_bytes (max/min extension) and dst_phase_bytes[c] (abs/negate) = (address of the output plane) & 0x1f are explicit inputs; they matter only when the vector constant differs from the scalar one (max/min: vec_one != 1.0; abs/negate: vec_mask != scalar mask). The harness puts every output plane at the same phase, so T2 covers equal phases for all channels"),
        ("OP2-A8", "CALLEE-HOSTED (count getter), max and min only", "nout = result of virtual slot 31 is an explicit parameter, hooked in R-PARITY; abs and negate do not call it: their channel count is the operand count [this] and no virtual call exists in the body (MEASURED listing)"),
        ("OP2-A9", "INFERRED", "object layout: counts at, flags, constants../.., records with plane c at the record's per-channel pointer; unary classes read only and (the constant flag is ignored); host fill not run"),
        ("OP2-A10", "CONTRACT", "both operands flagged constant still reads B's plane; counts outside 1..4 with n > 0, missing or short planes return an error; for abs and negate a count above the number of planes returns MissingPlane"),
    ]
}
