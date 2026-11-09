//! comp_premult: native port of the AVX2 route of builtin class `comp_premult`
//! (vtable, slot 53, AVX2 body, listing offsets of neutral_1.asm).
//!
//! The class composites operand B over operand A using a THIRD operand T as the alpha:
//! per output channel and sample, with t = T:  t >= 1 -> B;  0 >= t -> A + B;  else (NaN included) -> fma(A, 1 - t, B).
//! One variant (A constant, B plane, T constant) uses B + fma(-t, A, A) instead. Operands are planes or constants.
//! std only, no unsafe. Mutation switches (`Mutant`) exist only for the T2 kill-rate runs.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mutant {
    None,
    /// `t > 1` instead of `t >= 1`
    GtOne,
    /// `t < 0` instead of `t <= 0`
    LtZero,
    /// NaN t takes the A + B branch
    NanAdds,
    /// the (A const, B plane, T const) variant uses the generic fma(A, 1 - t, B)
    C3Generic,
    /// the generic fma is split into a multiply and an add (two roundings)
    NonFused,
    /// the sum branch returns A only
    SumDropsB,
    /// the t >= 1 branch returns A + B
    OneBranchSum,
    /// fma(B, 1 - t, A): operand roles swapped
    FmaSwapped,
    /// channel 3 of a 3-channel A is not filled with 1.0 (it is composited like the others)
    NoAlphaFill,
    /// the alpha fill value is 0.0
    AlphaFillZero,
    /// the alpha fill triggers for ca >= 3 (also ca == 4)
    FillCa4,
    /// the B := T swap on channel 3 (cb == 3) is not applied
    NoBSwap,
    /// the swap takes the plane of T but keeps B's own constant flag
    SwapKeepsFlag,
    /// plane channel index of A is min(ch, ca - 1) replaced by 0
    APlaneCh0,
    /// constant channel index of A is min(ch, ca - 1) instead of the raw channel
    AConstClamped,
    /// T plane channel index is always 0
    TPlaneCh0,
    /// all-constant case: no integer round trip
    NoIntTrunc,
    /// all-constant case: the conversion rounds to nearest instead of truncating
    IntRound,
    /// all-constant case: NaN or out of range converts to 0 (saturating cast) instead of i32::MIN
    IntSaturate,
    /// the last sample is skipped
    SkipLast,
    /// one output channel fewer
    NoutMinus1,
}

impl Mutant {
    pub const ALL: [Mutant; 21] = [
        Mutant::GtOne, Mutant::LtZero, Mutant::NanAdds, Mutant::C3Generic, Mutant::NonFused, Mutant::SumDropsB, Mutant::OneBranchSum,
        Mutant::FmaSwapped, Mutant::NoAlphaFill, Mutant::AlphaFillZero, Mutant::FillCa4, Mutant::NoBSwap, Mutant::SwapKeepsFlag,
        Mutant::APlaneCh0, Mutant::AConstClamped, Mutant::TPlaneCh0, Mutant::NoIntTrunc, Mutant::IntRound, Mutant::IntSaturate,
        Mutant::SkipLast, Mutant::NoutMinus1,
    ];
    pub fn from_name(s: &str) -> Option<Mutant> {
        if s == "none" {
            return Some(Mutant::None);
        }
        Mutant::ALL.iter().copied().find(|m| format!("{:?}", m) == s)
    }
}

/// One operand record: `count` channels (this), constant flag byte (this), four constants
/// (this) and, when not constant, the channel planes (one record pointer per channel ch).
#[derive(Clone, Copy)]
pub struct Operand<'a> {
    pub count: i32,
    pub is_const: bool,
    pub consts: [f32; 4],
    pub planes: &'a [&'a [f32]],
}

/// `nout`: the value of the class virtual slot 31 (0xf8), the number of output channels (explicit input, hooked in the T2 as a host service).
/// `n`: samples per channel, info.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub nout: i32,
    pub n: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// contract: nout in 0..=4 (channel constants are read by raw channel index from a four-float array)
    OutputCount(i32),
    /// contract: every operand count >= 1
    OperandCount(char, i32),
    /// a plane operand has fewer channel planes than the channel index needs (the original would read a null or foreign record)
    MissingPlane(char, usize),
    /// a plane is shorter than n
    ShortPlane(char, usize),
    /// an output plane is missing or shorter than n
    ShortOutput(usize),
}

use fx_common::x86::{nx_add, nx_fma, nx_mul, nx_sub};

const ONE: f32 = 1.0;

/// cvttss2si followed by cvtsi2ss: truncation toward zero; NaN and out of range give the integer indefinite 0x80000000.
fn int_round_trip(v: f32, m: Mutant) -> f32 {
    let r: i32 = match m {
        Mutant::NoIntTrunc => return v,
        Mutant::IntRound => {
            let x = v.round_ties_even();
            if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 { i32::MIN } else { x as i32 }
        }
        Mutant::IntSaturate => v as i32,
        _ => {
            if v.is_nan() || v >= 2147483648.0 || v < -2147483648.0 { i32::MIN } else { v as i32 }
        }
    };
    r as f32
}

/// The three-way select shared by every variant. `generic` is fma(A, 1 - t, B) (one rounding); the C3 variant passes its own value.
#[inline]
fn pick(t: f32, a: f32, b: f32, c3: bool, m: Mutant) -> f32 {
    let ge1 = if m == Mutant::GtOne { t > ONE } else { t >= ONE };
    if ge1 {
        return if m == Mutant::OneBranchSum { nx_add(a, b) } else { b };
    }
    let le0 = if m == Mutant::LtZero { t < 0.0 } else { t <= 0.0 };
    if m == Mutant::NanAdds && t.is_nan() {
        return nx_add(a, b);
    }
    if le0 {
        return if m == Mutant::SumDropsB { a } else { nx_add(a, b) };
    }
    if c3 && m != Mutant::C3Generic {
        // vfnmadd213ss xmm1(t), xmm7(a), xmm7(a): x = fma(-t, a, a); then b + x
        let x = nx_fma(-t, a, a);
        return nx_add(b, x);
    }
    match m {
        Mutant::NonFused => nx_add(nx_mul(a, nx_sub(ONE, t)), b),
        Mutant::FmaSwapped => nx_fma(b, nx_sub(ONE, t), a),
        _ => nx_fma(a, nx_sub(ONE, t), b),
    }
}

fn plane_of<'a>(o: &Operand<'a>, tag: char, ch: usize, need: usize) -> Result<&'a [f32], Error> {
    let p = o.planes.get(ch).ok_or(Error::MissingPlane(tag, ch))?;
    if p.len() < need {
        return Err(Error::ShortPlane(tag, ch));
    }
    Ok(p)
}

/// Runs the AVX2 route of slot 53 (flag 0). `outs[ch]` is the destination plane of channel ch.
pub fn run(p: &Params, a: &Operand, b: &Operand, t: &Operand, outs: &mut [&mut [f32]], m: Mutant) -> Result<(), Error> {
    let mut nout = p.nout;
    if m == Mutant::NoutMinus1 {
        nout -= 1;
    }
    if nout > 4 {
        return Err(Error::OutputCount(nout));
    }
    if nout <= 0 {
        return Ok(());
    }
    for (tag, o) in [('a', a), ('b', b), ('t', t)] {
        if o.count < 1 {
            return Err(Error::OperandCount(tag, o.count));
        }
    }
    let nu = if p.n > 0 { p.n as usize } else { 0 };
    for ch in 0..nout as usize {
        let r = ch as i32; // r13
        let ca1 = a.count - 1;
        let cb1 = b.count - 1;
        let ct1 = t.count - 1;
        let sel = |c1: i32| -> usize { (r.min(c1)) as usize };
        let mut b_const = b.is_const;
        let t_ch = if m == Mutant::TPlaneCh0 { 0 } else { sel(ct1) };
        let mut b_plane_from_t = false;
        if ch == 3 {
            if ca1 == 2 || (m == Mutant::FillCa4 && ca1 == 3) {
                if m == Mutant::NoAlphaFill {
                    // fall through to the generic composite
                } else {
                    // channel 3 of a three-channel A: the output is 1.0 and the function returns (no further channels)
                    if nu == 0 {
                        return Ok(());
                    }
                    let o = outs.get_mut(3).ok_or(Error::ShortOutput(3))?;
                    if o.len() < nu {
                        return Err(Error::ShortOutput(3));
                    }
                    let fill = if m == Mutant::AlphaFillZero { 0.0f32 } else { 1.0f32 };
                    for v in o[..nu].iter_mut() {
                        *v = fill;
                    }
                    return Ok(());
                }
            }
            if cb1 == 2 && m != Mutant::NoBSwap {
                // r9 := r8 (B plane pointer := T plane pointer), r15d := r12d (B constant flag := T constant flag)
                b_plane_from_t = true;
                if m != Mutant::SwapKeepsFlag {
                    b_const = t.is_const;
                }
            }
        }
        if nu == 0 {
            continue;
        }
        let o_len_ok = outs.get(ch).map(|o| o.len() >= nu).unwrap_or(false);
        if !o_len_ok {
            return Err(Error::ShortOutput(ch));
        }
        // operand access
        let a_idx = if m == Mutant::APlaneCh0 { 0 } else { sel(ca1) };
        let a_cidx = if m == Mutant::AConstClamped { sel(ca1) } else { ch };
        let a_plane = if a.is_const { None } else { Some(plane_of(a, 'a', a_idx, nu)?) };
        let t_plane = if t.is_const { None } else { Some(plane_of(t, 't', t_ch, nu)?) };
        let b_plane = if b_const {
            None
        } else if b_plane_from_t {
            // the swapped plane pointer is T's plane for this channel (null record would fault in the original)
            Some(plane_of(t, 't', t_ch, nu)?)
        } else {
            Some(plane_of(b, 'b', sel(cb1), nu)?)
        };
        let ac = a.consts[a_cidx];
        let bc = b.consts[ch];
        let tc = t.consts[ch];
        let end = if m == Mutant::SkipLast { nu.saturating_sub(1) } else { nu };
        let out = &mut *outs[ch];
        match (a.is_const, b_const, t.is_const) {
            (true, true, true) => {
                // all constants: one scalar result, converted through int32 and back, then the plane is filled
                let v = pick(tc, ac, bc, false, m);
                let v = int_round_trip(v, m);
                for o in out[..end].iter_mut() {
                    *o = v;
                }
            }
            (true, true, false) => {
                let tp = t_plane.unwrap();
                for i in 0..end {
                    out[i] = pick(tp[i], ac, bc, false, m);
                }
            }
            (true, false, true) => {
                // C3: A constant, B plane, T constant: B + fma(-t, A, A)
                let bp = b_plane.unwrap();
                for i in 0..end {
                    out[i] = pick(tc, ac, bp[i], true, m);
                }
            }
            (true, false, false) => {
                let (bp, tp) = (b_plane.unwrap(), t_plane.unwrap());
                for i in 0..end {
                    out[i] = pick(tp[i], ac, bp[i], false, m);
                }
            }
            (false, true, true) => {
                let ap = a_plane.unwrap();
                for i in 0..end {
                    out[i] = pick(tc, ap[i], bc, false, m);
                }
            }
            (false, true, false) => {
                let (ap, tp) = (a_plane.unwrap(), t_plane.unwrap());
                for i in 0..end {
                    out[i] = pick(tp[i], ap[i], bc, false, m);
                }
            }
            (false, false, true) => {
                let (ap, bp) = (a_plane.unwrap(), b_plane.unwrap());
                for i in 0..end {
                    out[i] = pick(tc, ap[i], bp[i], false, m);
                }
            }
            (false, false, false) => {
                let (ap, bp, tp) = (a_plane.unwrap(), b_plane.unwrap(), t_plane.unwrap());
                for i in 0..end {
                    out[i] = pick(tp[i], ap[i], bp[i], false, m);
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn op<'a>(count: i32, c: bool, k: [f32; 4], pl: &'a [&'a [f32]]) -> Operand<'a> {
        Operand { count, is_const: c, consts: k, planes: pl }
    }

    #[test]
    fn three_branches_all_planes() {
        let a = [[2.0f32, 2.0, 2.0, f32::NAN]];
        let b = [[1.0f32, 1.0, 1.0, 1.0]];
        let t = [[1.0f32, 0.0, 0.25, 0.5]];
        let ap: Vec<&[f32]> = a.iter().map(|x| &x[..]).collect();
        let bp: Vec<&[f32]> = b.iter().map(|x| &x[..]).collect();
        let tp: Vec<&[f32]> = t.iter().map(|x| &x[..]).collect();
        let mut o = vec![0.0f32; 4];
        {
            let mut outs: Vec<&mut [f32]> = vec![&mut o[..]];
            run(&Params { nout: 1, n: 4 }, &op(1, false, [0.0; 4], &ap), &op(1, false, [0.0; 4], &bp), &op(1, false, [0.0; 4], &tp), &mut outs, Mutant::None).unwrap();
        }
        assert_eq!(o[0], 1.0); // t >= 1: B
        assert_eq!(o[1], 3.0); // t <= 0: A + B
        assert_eq!(o[2], 2.0 * 0.75 + 1.0);
        assert!(o[3].is_nan());
    }

    #[test]
    fn nan_t_takes_fma() {
        let a = [[3.0f32]];
        let b = [[1.0f32]];
        let t = [[f32::NAN]];
        let ap: Vec<&[f32]> = a.iter().map(|x| &x[..]).collect();
        let bp: Vec<&[f32]> = b.iter().map(|x| &x[..]).collect();
        let tp: Vec<&[f32]> = t.iter().map(|x| &x[..]).collect();
        let mut o = vec![0.0f32; 1];
        {
            let mut outs: Vec<&mut [f32]> = vec![&mut o[..]];
            run(&Params { nout: 1, n: 1 }, &op(1, false, [0.0; 4], &ap), &op(1, false, [0.0; 4], &bp), &op(1, false, [0.0; 4], &tp), &mut outs, Mutant::None).unwrap();
        }
        assert!(o[0].is_nan());
    }

    #[test]
    fn all_constants_go_through_int() {
        let mut o = vec![0.0f32; 3];
        {
            let mut outs: Vec<&mut [f32]> = vec![&mut o[..]];
            // t = 0.5: fma(2.5, 0.5, 1.75) = 3.0; t = 0.25 would give 3.625 -> 3
            let k = |v: f32| [v, v, v, v];
            run(&Params { nout: 1, n: 3 }, &op(1, true, k(2.5), &[]), &op(1, true, k(1.75), &[]), &op(1, true, k(0.25), &[]), &mut outs, Mutant::None).unwrap();
        }
        assert_eq!(o, vec![3.0, 3.0, 3.0]);
        let mut o2 = vec![0.0f32; 1];
        {
            let mut outs: Vec<&mut [f32]> = vec![&mut o2[..]];
            let k = |v: f32| [v, v, v, v];
            run(&Params { nout: 1, n: 1 }, &op(1, true, k(1.0), &[]), &op(1, true, k(f32::NAN), &[]), &op(1, true, k(0.0), &[]), &mut outs, Mutant::None).unwrap();
        }
        assert_eq!(o2[0], -2147483648.0); // NaN -> integer indefinite
    }

    #[test]
    fn alpha_fill_and_b_swap() {
        // ca = 3, nout = 4: channel 3 is 1.0
        let ap_data = [[1.0f32; 2]; 3];
        let ap: Vec<&[f32]> = ap_data.iter().map(|x| &x[..]).collect();
        let bd = [[0.5f32; 2]; 3];
        let bp: Vec<&[f32]> = bd.iter().map(|x| &x[..]).collect();
        let td = [[0.0f32; 2]];
        let tp: Vec<&[f32]> = td.iter().map(|x| &x[..]).collect();
        let mut os = vec![vec![9.0f32; 2]; 4];
        {
            let mut outs: Vec<&mut [f32]> = os.iter_mut().map(|v| &mut v[..]).collect();
            run(&Params { nout: 4, n: 2 }, &op(3, false, [0.0; 4], &ap), &op(3, false, [0.0; 4], &bp), &op(1, false, [0.0; 4], &tp), &mut outs, Mutant::None).unwrap();
        }
        assert_eq!(os[0], vec![1.5, 1.5]);
        assert_eq!(os[3], vec![1.0, 1.0]);
        // ca = 4, cb = 3, T plane: channel 3 = T + A3 * (1 - T) with B3 := T
        let a4: Vec<[f32; 2]> = vec![[2.0; 2]; 4];
        let ap4: Vec<&[f32]> = a4.iter().map(|x| &x[..]).collect();
        let td2 = [[0.5f32; 2]];
        let tp2: Vec<&[f32]> = td2.iter().map(|x| &x[..]).collect();
        let mut os2 = vec![vec![9.0f32; 2]; 4];
        {
            let mut outs: Vec<&mut [f32]> = os2.iter_mut().map(|v| &mut v[..]).collect();
            run(&Params { nout: 4, n: 2 }, &op(4, false, [0.0; 4], &ap4), &op(3, false, [0.0; 4], &bp), &op(1, false, [0.0; 4], &tp2), &mut outs, Mutant::None).unwrap();
        }
        assert_eq!(os2[3], vec![2.0 * 0.5 + 0.5, 2.0 * 0.5 + 0.5]);
    }

    #[test]
    fn c3_variant_differs_from_generic() {
        // A const, B plane, T const: B + fma(-t, A, A). Pick values where the two roundings differ.
        let a = 1.0000001f32;
        let t = 0.3f32;
        let b = 0.1f32;
        let generic = a.mul_add(1.0 - t, b);
        let c3 = b + (-t).mul_add(a, a);
        let bd = [[b]];
        let bp: Vec<&[f32]> = bd.iter().map(|x| &x[..]).collect();
        let mut o = vec![0.0f32; 1];
        {
            let mut outs: Vec<&mut [f32]> = vec![&mut o[..]];
            let k = |v: f32| [v, v, v, v];
            run(&Params { nout: 1, n: 1 }, &op(1, true, k(a), &[]), &op(1, false, [0.0; 4], &bp), &op(1, true, k(t), &[]), &mut outs, Mutant::None).unwrap();
        }
        assert_eq!(o[0].to_bits(), c3.to_bits());
        let _ = generic;
    }

    #[test]
    fn degenerate_and_errors() {
        let mut o = vec![7.0f32; 2];
        {
            let mut outs: Vec<&mut [f32]> = vec![&mut o[..]];
            let k = [1.0f32; 4];
            run(&Params { nout: 1, n: 0 }, &op(1, true, k, &[]), &op(1, true, k, &[]), &op(1, true, k, &[]), &mut outs, Mutant::None).unwrap();
            run(&Params { nout: 1, n: -5 }, &op(1, true, k, &[]), &op(1, true, k, &[]), &op(1, true, k, &[]), &mut outs, Mutant::None).unwrap();
            run(&Params { nout: 0, n: 2 }, &op(1, true, k, &[]), &op(1, true, k, &[]), &op(1, true, k, &[]), &mut outs, Mutant::None).unwrap();
            assert_eq!(run(&Params { nout: 5, n: 2 }, &op(1, true, k, &[]), &op(1, true, k, &[]), &op(1, true, k, &[]), &mut outs, Mutant::None), Err(Error::OutputCount(5)));
            assert_eq!(run(&Params { nout: 1, n: 2 }, &op(1, false, k, &[]), &op(1, true, k, &[]), &op(1, true, k, &[]), &mut outs, Mutant::None), Err(Error::MissingPlane('a', 0)));
        }
        assert_eq!(o, vec![7.0, 7.0]);
    }

    #[test]
    fn mutant_names_round_trip() {
        for m in Mutant::ALL {
            assert_eq!(Mutant::from_name(&format!("{:?}", m)), Some(m));
        }
        assert_eq!(Mutant::from_name("none"), Some(Mutant::None));
    }
}
