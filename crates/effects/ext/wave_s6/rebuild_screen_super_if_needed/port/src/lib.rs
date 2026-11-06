//! screen_super (, launch target screen_super_if_needed): native port of the builtin class `screen_super` (vtable, first slot), which the
//! registry also holds under the literal name `screen_super_if_needed` (a second prototype instance of the SAME class, see README).
//! Slot 53 = (dispatcher), AVX2 body (1,808 B). Listing offsets, VA = offset.
//! std only, no unsafe, explicit parameters (no Default). Same operand layout and combo base as the operator classes `* + -`.
//!
//! Per output channel ch and element i: with a = A[min(ch, countA-1)][i] (or the A constant), b = B[min(ch, countB-1)][i] (or the B constant)
//!   t = fma(1 - a, b, a)            (vsubss, then ONE fused multiply-add: vfmadd213ss / vfmadd231ss)
//!   out = maxs(a, maxs(b, t))       (maxs(x, y) = x > y ? x : y, the vmaxss operand rule)
//! (3,4): channels 0..2 as above, channel 3 = maxs(1, maxs(b3, 1)); (4,3): channel 3 = maxs(a3, maxs(1, (1 + a3) - a3)).

/// CAPTURED : f32 1.0 (lookup_ss.tsv).
pub const ONE: f32 = 1.0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mutant {
    None,
    Unfused,
    InnerSwap,
    OuterSwap,
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
    ExtA3Only,
    Cut28,
    NoPeel,
    PeelPlus1,
    VecOneIgnored,
    ExtASimple,
    ExtBNoMax,
    ExtAOrder,
}

impl Mutant {
    pub fn from_name(s: &str) -> Option<Mutant> {
        use Mutant::*;
        Some(match s {
            "none" => None,
            "unfused" => Unfused,
            "inner_swap" => InnerSwap,
            "outer_swap" => OuterSwap,
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
            "ext_a3_only" => ExtA3Only,
            "cut28" => Cut28,
            "no_peel" => NoPeel,
            "peel_plus1" => PeelPlus1,
            "vec_one_ignored" => VecOneIgnored,
            "ext_a_simple" => ExtASimple,
            "ext_b_nomax" => ExtBNoMax,
            "ext_a_order" => ExtAOrder,
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
/// getter hooked in R-PARITY; MEASURED table in audit_evt\COMBOS.tsv). `n` = [info]. `vec_one` = STD-002 (8 lanes
/// of the same value, SEMANTIC 1.0f, uncovered); no default. `dst3_phase_bytes` = (address of output plane 3) & 0x1f, ALIGN-DEPENDENT.
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

/// vmaxss d, d, s : d > s ? d : s (second operand on NaN or equal).
#[inline]
fn maxs(x: f32, y: f32) -> f32 {
    if x > y {
        x
    } else {
        y
    }
}

/// One element of the main loop (and their twins in the fixed 3 channel loop).
fn screen(a: f32, b: f32, m: Mutant) -> f32 {
    let (a, b) = if m == Mutant::SwapAb { (b, a) } else { (a, b) };
    let om = if m == Mutant::OneMinusB { ONE - b } else { ONE - a }; // vsubss xmm, ONE, a
    let t = match m {
        Mutant::Unfused => om * b + a,
        Mutant::OneMinusB => om.mul_add(a, b),
        _ => om.mul_add(b, a), // vfmadd213ss / vfmadd231ss: (1 - a) * b + a, one rounding
    };
    let inner = if m == Mutant::InnerSwap { maxs(t, b) } else { maxs(b, t) };
    if m == Mutant::OuterSwap {
        maxs(inner, a)
    } else {
        maxs(a, inner)
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

/// S3-style entry: renders the output planes of the class's slot-53 AVX2 body from explicit parameters.
pub fn render(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]]) -> Result<(), ScreenError> {
    run(p, a, b, out, Mutant::None)
}

/// The whole slot-53 AVX2 body .
pub fn run(p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), ScreenError> {
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
        if (r14 || r12) && out.get(3).map_or(true, |d| d.len() < n as usize) {
            return Err(ScreenError::MissingOut);
        }
    }
    let (a_const, b_const) = (a.is_const && m != Mutant::NoConstA, b.is_const && m != Mutant::NoConstB);
    for ch in 0..nch {
        // cmp esi,eax ; cmovl : idx = min(ch, count - 1), signed
        let mut ia = ch.min(a.count - 1);
        let mut ib = ch.min(b.count - 1);
        if m == Mutant::ClampA {
            ia = ch.min(a.count);
        }
        if m == Mutant::ClampB {
            ib = ch.min(b.count);
        }
        if n <= 0 {
            continue; // test r10d; jle next channel
        }
        let n = n as usize;
        let dst_i = ch as usize;
        if a_const {
            // flag A first; B plane r14 is read even if B is flagged constant
            let ca = a.consts[cidx(ia)?];
            let pb = plane(b, ib)?;
            let d = out.get_mut(dst_i).ok_or(ScreenError::MissingOut)?;
            for i in 0..n {
                d[i] = screen(ca, *pb.get(i).ok_or(ScreenError::ShortPlane)?, m);
            }
        } else if b_const {
            // A plane with the B constant
            let cb = b.consts[cidx(ib)?];
            let pa = plane(a, ia)?;
            let d = out.get_mut(dst_i).ok_or(ScreenError::MissingOut)?;
            for i in 0..n {
                d[i] = screen(*pa.get(i).ok_or(ScreenError::ShortPlane)?, cb, m);
            }
        } else {
            // two planes
            let (pa, pb) = (plane(a, ia)?, plane(b, ib)?);
            let d = out.get_mut(dst_i).ok_or(ScreenError::MissingOut)?;
            for i in 0..n {
                d[i] = screen(*pa.get(i).ok_or(ScreenError::ShortPlane)?, *pb.get(i).ok_or(ScreenError::ShortPlane)?, m);
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

///  (r14, B side: dst[3] from B plane 3 / constant 3) and (r12, A side: dst[3] from A).
fn ext(b_side: bool, p: &Params, a: &Operand, b: &Operand, out: &mut [&mut [f32]], m: Mutant) -> Result<(), ScreenError> {
    let n = p.n as usize;
    let o = if b_side { b } else { a };
    let ci = if m == Mutant::ConstIdx { 2 } else { 3 }; // this (B) / this (A): constant index 3
    let d = out.get_mut(3).ok_or(ScreenError::MissingOut)?;
    // value of one element given x (plane or constant); kvec = element comes from the ymm core (STD-002 lanes)
    let el = |x: f32, kvec: bool| -> f32 {
        if b_side {
            let one = if kvec && m != Mutant::VecOneIgnored { p.vec_one } else { ONE };
            if m == Mutant::ExtBNoMax {
                x
            } else {
                maxs(one, maxs(x, one)) // vmaxss(x, ONE) then vmaxss(ONE, .) ; the ymm core: vmaxps with the STD-002 vector
            }
        } else if m == Mutant::ExtASimple {
            maxs(x, ONE)
        } else if m == Mutant::ExtAOrder {
            maxs(ONE, maxs(x, (ONE + x) - x))
        } else {
            maxs(x, maxs(ONE, (ONE + x) - x)) // vaddss, vsubss, vmaxss(ONE, .), vmaxss(x, .)
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
        return Err(ScreenError::ShortPlane);
    }
    if b_side {
        vec_kernel(d, src, n, p.dst3_phase_bytes, m, &el);
    } else {
        for i in 0..n {
            d[i] = el(src[i], false);
        }
    }
    Ok(())
}

/// The 16 wide B-side kernel : scalar head with ONE, ymm core with the STD-002 vector (two vmaxps ymm per step), scalar tail.
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
        ("SS1-A1", "CAPTURED", "ONE 1.0f : bytes 0000803f read by lookup.py (lookup_ss.tsv)"),
        ("SS1-A2", "SEMANTIC (STD-002, explicit parameter vec_one)", "vector constant (32 B, uncovered) used by the (3,4) B-side extension kernel; basis: the scalar head and tail of the same loop use captured ONE over the same array; T2 runs 1.0 and 0.75 on both sides"),
        ("SS1-A3", "ROUTE", "AVX2 route only: feature word 0x9d97ff selects; the AVX body and the SSE body are not ported"),
        ("SS1-A4", "HOST-FP-ASSUMED", "MXCSR round to nearest, no FTZ, no DAZ; the emulator follows the SDM, not silicon"),
        ("SS1-A5", "ALIGN-DEPENDENT", "dst3_phase_bytes = (address of output plane 3) & 0x1f is an explicit input (only the (3,4) B-side kernel splits on it, and only matters when vec_one != 1.0)"),
        ("SS1-A6", "CALLEE-HOSTED (count getter)", "nout = result of virtual slot 31 (: [chosen combo] or -1) is an explicit parameter; the combo table is MEASURED (audit_evt COMBOS.tsv), the choice and the builder are not rebuilt (COMBO-PARITY)"),
        ("SS1-A7", "INFERRED", "this are the operand channel counts, their 'is constant' flags,../.. the constants, records at with plane c at the record's per-channel pointer: measured from the body, host fill not run"),
        ("SS1-A8", "CONTRACT", "both operands flagged constant still reads B's plane (A-constant branch first); counts < 1 or > 4 with n > 0, and short planes, are outside the contract and return an error"),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    const S: u32 = 0x7fc0ffee;
    fn pl(n: usize, f: impl Fn(usize) -> f32) -> Vec<f32> {
        (0..n).map(f).collect()
    }
    fn outs(k: usize, n: usize) -> Vec<Vec<f32>> {
        (0..k).map(|_| vec![f32::from_bits(S); n]).collect()
    }
    fn go(p: &Params, a: &Operand, b: &Operand, o: &mut Vec<Vec<f32>>) -> Result<(), ScreenError> {
        let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        render(p, a, b, &mut m)
    }
    fn op<'a>(count: i32, planes: &'a [&'a [f32]]) -> Operand<'a> {
        Operand { count, is_const: false, consts: [0.0; 4], planes }
    }
    fn cop(count: i32, c: [f32; 4]) -> Operand<'static> {
        Operand { count, is_const: true, consts: c, planes: &[] }
    }
    fn p(nout: i32, n: i32) -> Params {
        Params { nout, n, vec_one: 1.0, dst3_phase_bytes: 0 }
    }

    #[test]
    fn hand_values() {
        // screen = 1 - (1-a)(1-b): a = 0.5, b = 0.5 -> 0.75; a = 0.25, b = 0.5 -> 0.625; a = 0, b = x -> x; a = 1 -> 1
        let a0 = [0.5f32, 0.25, 0.0, 1.0];
        let b0 = [0.5f32, 0.5, 0.75, 0.3];
        let (pa, pb) = ([&a0[..]], [&b0[..]]);
        let mut o = outs(1, 8);
        go(&p(1, 4), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
        assert_eq!(&o[0][..4], &[0.75, 0.625, 0.75, 1.0]);
        assert_eq!(o[0][4].to_bits(), S);
    }

    #[test]
    fn fused_not_unfused() {
        // a = 0.1, b = 0.7: fused and unfused t differ in the last bit for some inputs; scan for one and require the port to equal the fused value
        let mut seen = false;
        for i in 1..2000u32 {
            let (a, b) = (i as f32 * 0.00037, 0.7123f32 + i as f32 * 0.0001);
            let fu = (1.0 - a).mul_add(b, a);
            let un = (1.0 - a) * b + a;
            if fu.to_bits() != un.to_bits() {
                seen = true;
                let (x, y) = ([a], [b]);
                let (pa, pb) = ([&x[..]], [&y[..]]);
                let mut o = outs(1, 2);
                go(&p(1, 1), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
                assert_eq!(o[0][0].to_bits(), maxs(a, maxs(b, fu)).to_bits());
                break;
            }
        }
        assert!(seen);
    }

    #[test]
    fn clamp_const_and_extension() {
        // (3,4): channels 0..2 screen, channel 3 = max(1, max(b3, 1)); b3 = 0.2 -> 1.0, b3 = 3.0 -> 3.0, b3 = NaN -> 1.0
        let a: Vec<Vec<f32>> = (0..3).map(|_| vec![0.5f32; 4]).collect();
        let b: Vec<Vec<f32>> = (0..4).map(|c| if c == 3 { vec![0.2, 3.0, f32::NAN, 1.0] } else { vec![0.5; 4] }).collect();
        let (pa, pb): (Vec<&[f32]>, Vec<&[f32]>) = (a.iter().map(|v| &v[..]).collect(), b.iter().map(|v| &v[..]).collect());
        let mut o = outs(4, 8);
        go(&p(4, 4), &op(3, &pa), &op(4, &pb), &mut o).unwrap();
        assert_eq!(&o[3][..4], &[1.0, 3.0, 1.0, 1.0]);
        assert_eq!(&o[0][..4], &[0.75; 4]);
        // (4,3): channel 3 = max(a3, max(1, (1 + a3) - a3)): a3 = 0.25 -> 1.0, a3 = 4.0 -> 4.0
        let a4: Vec<Vec<f32>> = (0..4).map(|c| if c == 3 { vec![0.25, 4.0, 0.0, 1.0] } else { vec![0.5; 4] }).collect();
        let b3: Vec<Vec<f32>> = (0..3).map(|_| vec![0.5f32; 4]).collect();
        let (pa, pb): (Vec<&[f32]>, Vec<&[f32]>) = (a4.iter().map(|v| &v[..]).collect(), b3.iter().map(|v| &v[..]).collect());
        let mut o = outs(4, 8);
        go(&p(4, 4), &op(4, &pa), &op(3, &pb), &mut o).unwrap();
        assert_eq!(&o[3][..4], &[1.0, 4.0, 1.0, 1.0]);
        // constant B (3 channel) against a plane A (1 channel), nout 3: every channel b = const[c]
        let a1 = [0.5f32; 4];
        let pa = [&a1[..]];
        let mut o = outs(3, 8);
        go(&p(3, 4), &op(1, &pa), &cop(3, [0.5, 0.0, 1.0, 9.0]), &mut o).unwrap();
        assert_eq!(&o[0][..4], &[0.75; 4]);
        assert_eq!(&o[1][..4], &[0.5; 4]);
        assert_eq!(&o[2][..4], &[1.0; 4]);
    }

    #[test]
    fn degenerate_and_errors() {
        let a0 = pl(8, |i| i as f32 * 0.1);
        let pa = [&a0[..]];
        let mut o = outs(1, 8);
        go(&p(1, 0), &op(1, &pa), &op(1, &pa), &mut o).unwrap();
        assert_eq!(o[0][0].to_bits(), S);
        go(&p(0, 4), &op(1, &pa), &op(1, &pa), &mut o).unwrap();
        assert_eq!(o[0][0].to_bits(), S);
        assert_eq!(go(&p(1, 4), &op(1, &[]), &op(1, &pa), &mut o), Err(ScreenError::MissingPlane));
        let mut o2 = outs(1, 2);
        assert_eq!(go(&p(1, 4), &op(1, &pa), &op(1, &pa), &mut o2), Err(ScreenError::MissingOut));
    }

    #[test]
    fn vec_one_changes_only_the_core() {
        // (3,4), n = 40, phase 0: elements 0..32 come from the ymm core when vec_one is used (core end 32), the tail stays at ONE
        let a: Vec<Vec<f32>> = (0..3).map(|_| vec![0.5f32; 40]).collect();
        let b: Vec<Vec<f32>> = (0..4).map(|_| vec![0.25f32; 40]).collect();
        let (pa, pb): (Vec<&[f32]>, Vec<&[f32]>) = (a.iter().map(|v| &v[..]).collect(), b.iter().map(|v| &v[..]).collect());
        let mut o = outs(4, 64);
        let pp = Params { nout: 4, n: 40, vec_one: 0.75, dst3_phase_bytes: 0 };
        go(&pp, &op(3, &pa), &op(4, &pb), &mut o).unwrap();
        // core lanes: max(0.75, max(0.25, 0.75)) = 0.75; tail lanes: max(1, max(0.25, 1)) = 1
        assert_eq!(o[3][0], 0.75);
        assert_eq!(o[3][31], 0.75);
        assert_eq!(o[3][32], 1.0);
        assert_eq!(o[3][39], 1.0);
    }

    #[test]
    fn deterministic_and_nan() {
        let a0 = [f32::NAN, f32::INFINITY, -0.0, 0.0, f32::NEG_INFINITY, 1e30];
        let b0 = [0.5f32, f32::NAN, 0.0, -0.0, 1.0, -1e30];
        let (pa, pb) = ([&a0[..]], [&b0[..]]);
        let mut o1 = outs(1, 8);
        let mut o2 = outs(1, 8);
        go(&p(1, 6), &op(1, &pa), &op(1, &pb), &mut o1).unwrap();
        go(&p(1, 6), &op(1, &pa), &op(1, &pb), &mut o2).unwrap();
        assert!(o1[0].iter().zip(&o2[0]).all(|(x, y)| x.to_bits() == y.to_bits() || (x.is_nan() && y.is_nan())));
        // a NaN: t NaN, maxs(b, NaN) = NaN (second operand), maxs(NaN, NaN) = NaN
        assert!(o1[0][0].is_nan());
    }
}
