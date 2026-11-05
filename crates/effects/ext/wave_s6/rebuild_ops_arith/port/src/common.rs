//! Shared helpers of the four operator classes `+ - * /` (OP1). All four slot-53 AVX2 bodies have one shape (MEASURED, listing
//!  diffed): a scalar main loop over up to 3 (or nout) channels with channel clamping and the two
//! constant-operand flags, then an optional extension of channel 3 when one operand has 3 channels and the other 4.
//! Listing offsets in comments are those of the `+` body unless a class is named.

/// CAPTURED : f32 1.0 (lookup_ops.tsv).
pub const ONE: f32 = 1.0;
/// CAPTURED : 1000000.0f, bytes 00247449 (lookup_ops.tsv). Also the immediate 0x49742400 of the extension.
pub const DIV_K: f32 = 1_000_000.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Add,
    Sub,
    Mul,
    Div,
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
    CopyShort,
    Cut28,
    NoPeel,
    PeelPlus1,
    VecOneIgnored,
    ExtOrder,
    DivZeroLe,
    DivK2,
    ExtZeroBits,
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
            "copy_short" => CopyShort,
            "cut28" => Cut28,
            "no_peel" => NoPeel,
            "peel_plus1" => PeelPlus1,
            "vec_one_ignored" => VecOneIgnored,
            "ext_order" => ExtOrder,
            "div_zero_le" => DivZeroLe,
            "div_k2" => DivK2,
            "ext_zero_bits" => ExtZeroBits,
            _ => return Option::None,
        })
    }
}

/// One operand: A = this (count), (constant flag), (constants), record at (planes at the record's per-channel pointer);
/// B = this, record at. A constant operand has no record (null) in a harness layout; its planes slice may be empty.
pub struct Operand<'a> {
    pub count: i32,
    pub is_const: bool,
    pub consts: [f32; 4],
    pub planes: &'a [&'a [f32]],
}

/// Explicit parameters. `nout` is the result of the base virtual slot 31 (: the word of the chosen combo entry, a count
/// getter hooked in R-PARITY, W69). `n` = [info]. `vec_one` = STD-002 (8 lanes of the same value, SEMANTIC 1.0f,
/// uncovered); no default. `dst3_phase_bytes` = (address of output plane 3) & 0x1f, ALIGN-DEPENDENT (only the extension kernels split on it).
pub struct Params {
    pub nout: i32,
    pub n: i32,
    pub vec_one: f32,
    pub dst3_phase_bytes: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum OpError {
    MissingPlane,
    ShortPlane,
    ConstIndex,
    MissingOut,
}

impl Kind {
    /// The scalar op of the main loop: f(x, y) with x from A and y from B (vaddss/vsubss/vmulss; division with the zero rule).
    pub fn f(self, x: f32, y: f32, m: Mutant) -> f32 {
        match self {
            Kind::Add => x + y,
            Kind::Sub => x - y,
            Kind::Mul => x * y,
            Kind::Div => {
                // vucomiss y,0; jne/jp -> divide; equal and ordered (+0 or -0) -> K * x 
                let zero = if m == Mutant::DivZeroLe { y <= 0.0 } else { y == 0.0 };
                if zero {
                    (if m == Mutant::DivK2 { 2.0 * DIV_K } else { DIV_K }) * x
                } else {
                    x / y
                }
            }
        }
    }
    /// add and mul reload the constant as the first operand in the B-constant branch (: xmm0 = Bconst, then op xmm0, [A]).
    pub fn swap_on_b_const(self) -> bool {
        matches!(self, Kind::Add | Kind::Mul)
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

/// The whole slot-53 AVX2 body of one class.
pub fn run(k: Kind, p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
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
    // (r14|r12) != 0 or nout == 3 take the fixed 3-channel loop (no nout <= 0 test); else loop nout channels (: jle skips)
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
            let ca = a.consts[cidx(ia)?];
            let pb = plane(b, ib)?; // r14 = B plane (needed by the A-constant branch even if B is flagged constant)
            let d = out.get_mut(dst_i).ok_or(OpError::MissingOut)?;
            for i in 0..n {
                let (x, y) = (ca, *pb.get(i).ok_or(OpError::ShortPlane)?);
                d[i] = k.f(x, y, m);
            }
        } else if b_const {
            let cb = b.consts[cidx(ib)?];
            let pa = plane(a, ia)?;
            let d = out.get_mut(dst_i).ok_or(OpError::MissingOut)?;
            for i in 0..n {
                let x = *pa.get(i).ok_or(OpError::ShortPlane)?;
                let sw = k.swap_on_b_const() != (m == Mutant::SwapConstB);
                d[i] = if sw { k.f(cb, x, m) } else { k.f(x, cb, m) };
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

///  (r14, B side: dst[3] from B) and (r12, A side: dst[3] from A).
fn ext(k: Kind, b_side: bool, p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), OpError> {
    let n = p.n as usize;
    let o = if b_side { b } else { a };
    let ci = if m == Mutant::ConstIdx { 2 } else { 3 }; // this (B) / this (A): constant index 3
    let d = out.get_mut(3).ok_or(OpError::MissingOut)?;
    let order_flip = m == Mutant::ExtOrder;
    // value form of one element given x (plane or constant)
    let el = |x: f32, kvec: bool| -> f32 {
        let one = if kvec && m != Mutant::VecOneIgnored { p.vec_one } else { ONE };
        match (k, b_side) {
            (Kind::Add, _) => one + x,
            (Kind::Sub, true) => {
                if order_flip {
                    x - one
                } else {
                    one - x
                }
            }
            (Kind::Sub, false) => {
                if order_flip {
                    one - x
                } else {
                    x - one
                }
            }
            (Kind::Mul, _) => x, // copy
            (Kind::Div, true) => {
                // ONE / x; zero -> immediate 0x49742400 
                if x == 0.0 {
                    if m == Mutant::ExtZeroBits {
                        2.0 * DIV_K
                    } else {
                        f32::from_bits(0x49742400)
                    }
                } else {
                    one / x
                }
            }
            (Kind::Div, false) => x, // copy
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
    match k {
        Kind::Add | Kind::Sub => vec_kernel(d, src, n, p.dst3_phase_bytes, 16, 0x1d, m, |x, kv| el(x, kv)),
        Kind::Mul => copy_kernel(d, src, n, m),
        Kind::Div => {
            if b_side {
                for i in 0..n {
                    d[i] = el(src[i], false);
                }
            } else {
                copy_kernel(d, src, n, m);
            }
        }
    }
    Ok(())
}

/// Pure copy (vmovups / mov): the 8-wide split (thresholds 8 and 0xdd, peel by dst phase) is output-neutral, so it is a plain copy.
fn copy_kernel(d: &mut [f32], src: &[f32], n: usize, m: Mutant) {
    let n = if m == Mutant::CopyShort && n > 0 { n - 1 } else { n };
    for i in 0..n {
        d[i] = src[i];
    }
}

/// The 16 wide add/sub kernel : scalar head with ONE, ymm core with the STD-002 vector, scalar tail with ONE.
/// `f(x, vector_lane)` gives one element; `thr` = 0x1d (peel threshold), width 16, small threshold 16.
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

pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("OP1-A1", "CAPTURED", "ONE 1.0f, DIV_K 1000000.0f : bytes read by lookup.py (gate/lookup_ops.tsv)"),
        ("OP1-A2", "SEMANTIC (STD-002, PARAM-UNCAPTURED-style explicit parameter vec_one)", "vector constant (32 B, uncovered) used by the + and - extension kernels; basis: the scalar head and tail of the same loop use captured ONE over the same array; T2 runs 1.0 and 0.75 on both sides"),
        ("OP1-A3", "ROUTE", "AVX2 route only: feature word 0x9d97ff selects; AVX tiers and SSE tiers not ported"),
        ("OP1-A4", "HOST-FP-ASSUMED", "MXCSR round to nearest, no FTZ, no DAZ; the emulator follows the SDM, not silicon"),
        ("OP1-A5", "ALIGN-DEPENDENT", "dst3_phase_bytes = (address of output plane 3) & 0x1f is an explicit input (only the + and - extension kernels split on it, and only matters when vec_one != 1.0); the main loops are scalar"),
        ("OP1-A6", "CALLEE-HOSTED (count getter)", "nout = result of virtual slot 31 (: [chosen combo] or -1) is an explicit parameter; the combo choice and the type tables of the combo builder are uncovered and not rebuilt (INFERRED: nout is the output channel count 1..4 of the chosen combo)"),
        ("OP1-A7", "INFERRED", "this are the operand channel counts, their 'is constant' flags,../.. the constants, records at with plane c at the record's per-channel pointer: measured from the body, host fill not run"),
        ("OP1-A8", "CONTRACT", "both operands flagged constant still reads B's plane (A-constant branch first); counts < 1 or > 4 with n > 0, and short planes, are outside the contract and return an error"),
    ]
}
