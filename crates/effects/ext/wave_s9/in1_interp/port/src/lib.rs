//! interp (IN1): native port of the pixel builtin `interp` (linear interpolation of two images by a third, out = a + (b - a) * t).
//! Class `interp`: vtable symbol, first slot, id getter, factory, object size 0xd0, slot 53 (CPU
//! entry, feature-word dispatcher), AVX2 body (2,832 B), AVX tier, SSE tier, GPU twin slot 54
//!  (NOT REBUILT). Listing offsets (neutral_1.asm); VA = offset. std only, no unsafe, explicit parameters (no Default).
//!
//! What the body computes (MEASURED from the listing, see SPEC in README.md): per output channel `ch` (0..nout, nout from virtual slot 31 =
//! hooked in R-PARITY), per element i < n: `out = fma(b - a, t, a)` with `b - a` an f32 subtraction and the multiply-add FUSED
//! (vfmadd*ss / vfmadd*ps, 29 sites, all the same function). Each of a, b, t is either a plane (channel index min(ch, count - 1)) or a
//! constant (this, one float per channel), selected by the three flag bytes : 8 variants, each with a scalar head,
//! a 16 float ymm core and a scalar tail that compute the same element function (so the output does not depend on the buffer address).
//! Two special rules: (1) at channel 3 an operand with fewer than 4 channels becomes the constant 1.0 (CAPTURED ONE; flags are forced on
//! for later channels too); (2) with a and b constants and t a plane, if a is within (-1e-10, 1e-10) and b within (1 - 1e-10, 1 + 1e-10)
//! (four f64 compares) the output is a straight copy of t's plane (memcpy / 8 wide copy loop, so bits and NaN payloads pass through).
pub mod interp;
pub use interp::{assumptions, render, run, InterpError, Mutant, Operand, Params, MUTANT_NAMES, ONE, RNG_A_HI, RNG_A_LO, RNG_B_HI, RNG_B_LO};

#[cfg(test)]
mod tests {
    use super::*;

    const S: u32 = 0x7fc0ffee;
    fn outs(k: usize, n: usize) -> Vec<Vec<f32>> {
        (0..k).map(|_| vec![f32::from_bits(S); n]).collect()
    }
    fn go(p: &Params, a: &Operand, b: &Operand, t: &Operand, o: &mut Vec<Vec<f32>>, m: Mutant) -> Result<(), InterpError> {
        let mut v: Vec<&mut [f32]> = o.iter_mut().map(|x| x.as_mut_slice()).collect();
        run(p, a, b, t, &mut v, m)
    }
    fn pop<'a>(count: i32, planes: &'a [&'a [f32]]) -> Operand<'a> {
        Operand { count, is_const: false, consts: [0.0; 4], planes }
    }
    fn cop(count: i32, c: [f32; 4]) -> Operand<'static> {
        Operand { count, is_const: true, consts: c, planes: &[] }
    }

    #[test]
    fn hand_plane_plane_const_t() {
        // a = [0, 10], b = [10, 20], t = 0.5 -> [5, 15]
        let (a0, b0) = ([0.0f32, 10.0], [10.0f32, 20.0]);
        let (pa, pb) = ([&a0[..]], [&b0[..]]);
        let mut o = outs(1, 4);
        go(&Params { nout: 1, n: 2 }, &pop(1, &pa), &pop(1, &pb), &cop(1, [0.5, 0.0, 0.0, 0.0]), &mut o, Mutant::None).unwrap();
        assert_eq!(o[0][0], 5.0);
        assert_eq!(o[0][1], 15.0);
        assert_eq!(o[0][2].to_bits(), S);
    }

    #[test]
    fn fused_not_double_rounded() {
        // a = -1, b = 2^-12, t = 1 + 2^-12: b - a = 1 + 2^-12 exactly; fused result 2^-11 + 2^-24, unfused 2^-11
        let a = -1.0f32;
        let b = 2.0f32.powi(-12);
        let t = 1.0 + 2.0f32.powi(-12);
        let mut o = outs(1, 1);
        go(&Params { nout: 1, n: 1 }, &cop(1, [a, 0.0, 0.0, 0.0]), &cop(1, [b, 0.0, 0.0, 0.0]), &cop(1, [t, 0.0, 0.0, 0.0]), &mut o, Mutant::None).unwrap();
        assert_eq!(o[0][0], 2.0f32.powi(-11) + 2.0f32.powi(-24));
        let mut o2 = outs(1, 1);
        go(&Params { nout: 1, n: 1 }, &cop(1, [a, 0.0, 0.0, 0.0]), &cop(1, [b, 0.0, 0.0, 0.0]), &cop(1, [t, 0.0, 0.0, 0.0]), &mut o2, Mutant::UnfusedCcc).unwrap();
        assert_eq!(o2[0][0], 2.0f32.powi(-11));
    }

    #[test]
    fn t_zero_one_identities_on_planes() {
        let (a0, b0) = ([1.5f32, -2.0, 7.0], [9.0f32, 8.0, -4.0]);
        let (t0, t1) = ([0.0f32; 3], [1.0f32; 3]);
        let (pa, pb, pt0, pt1) = ([&a0[..]], [&b0[..]], [&t0[..]], [&t1[..]]);
        let mut o = outs(1, 3);
        go(&Params { nout: 1, n: 3 }, &pop(1, &pa), &pop(1, &pb), &pop(1, &pt0), &mut o, Mutant::None).unwrap();
        assert_eq!(o[0], a0.to_vec());
        let mut o = outs(1, 3);
        go(&Params { nout: 1, n: 3 }, &pop(1, &pa), &pop(1, &pb), &pop(1, &pt1), &mut o, Mutant::None).unwrap();
        assert_eq!(o[0], b0.to_vec());
    }

    #[test]
    fn range_shortcut_copies_t_bits() {
        // a = 0, b = 1 constants, t plane: straight copy (t = -0 stays -0; the formula would give +0)
        let t0 = [-0.0f32, f32::from_bits(0x7fc12345), 0.25, 3.0];
        let pt = [&t0[..]];
        let mut o = outs(1, 4);
        go(&Params { nout: 1, n: 4 }, &cop(1, [0.0; 4]), &cop(1, [1.0, 0.0, 0.0, 0.0]), &pop(1, &pt), &mut o, Mutant::None).unwrap();
        for i in 0..4 {
            assert_eq!(o[0][i].to_bits(), t0[i].to_bits());
        }
        let mut o = outs(1, 4);
        go(&Params { nout: 1, n: 4 }, &cop(1, [0.0; 4]), &cop(1, [1.0, 0.0, 0.0, 0.0]), &pop(1, &pt), &mut o, Mutant::RangeOff).unwrap();
        assert_eq!(o[0][0].to_bits(), 0.0f32.to_bits());
        // a just outside the window: 2e-10 is not copied; b = 0.99999994 is outside
        let mut o = outs(1, 4);
        go(&Params { nout: 1, n: 4 }, &cop(1, [2e-10, 0.0, 0.0, 0.0]), &cop(1, [1.0, 0.0, 0.0, 0.0]), &pop(1, &pt), &mut o, Mutant::None).unwrap();
        assert_ne!(o[0][0].to_bits(), t0[0].to_bits());
        // window edge inside: 9e-11 and -9e-11 copy
        for av in [9e-11f32, -9e-11, 0.0, -0.0] {
            let mut o = outs(1, 4);
            go(&Params { nout: 1, n: 4 }, &cop(1, [av, 0.0, 0.0, 0.0]), &cop(1, [1.0, 0.0, 0.0, 0.0]), &pop(1, &pt), &mut o, Mutant::None).unwrap();
            assert_eq!(o[0][2].to_bits(), 0.25f32.to_bits());
            assert_eq!(o[0][0].to_bits(), (-0.0f32).to_bits());
        }
    }

    #[test]
    fn alpha_channel_rule() {
        // A has 3 channels, B has 4, T constant: channel 3 uses a = ONE (constant), b = B plane 3
        let a: Vec<Vec<f32>> = (0..3).map(|c| vec![c as f32; 5]).collect();
        let b: Vec<Vec<f32>> = (0..4).map(|c| vec![10.0 + c as f32; 5]).collect();
        let (ra, rb): (Vec<&[f32]>, Vec<&[f32]>) = (a.iter().map(|v| &v[..]).collect(), b.iter().map(|v| &v[..]).collect());
        let mut o = outs(4, 5);
        go(&Params { nout: 4, n: 5 }, &pop(3, &ra), &pop(4, &rb), &cop(1, [0.5; 4]), &mut o, Mutant::None).unwrap();
        // ch 3: 1 + (13 - 1) * 0.5 = 7
        assert_eq!(o[3][0], 7.0);
        // ch 0: 0 + (10 - 0) * 0.5 = 5
        assert_eq!(o[0][0], 5.0);
        // both 3 channel: ch 3 = ONE + (ONE - ONE) * t = 1
        let mut o = outs(4, 5);
        go(&Params { nout: 4, n: 5 }, &pop(3, &ra), &pop(3, &ra), &cop(1, [0.25; 4]), &mut o, Mutant::None).unwrap();
        assert_eq!(o[3][0], 1.0);
        assert_eq!(o[1][4], 1.0);
        let mut o = outs(4, 5);
        go(&Params { nout: 4, n: 5 }, &pop(3, &ra), &pop(4, &rb), &cop(1, [0.5; 4]), &mut o, Mutant::AlphaAOff).unwrap();
        assert_ne!(o[3][0], 7.0);
    }

    #[test]
    fn channel_clamp_and_t_plane_indexing() {
        // A, B have 1 channel (broadcast), T has 3: out[c] = a0 + (b0 - a0) * t[c]
        let (a0, b0) = ([2.0f32; 6], [4.0f32; 6]);
        let t: Vec<Vec<f32>> = (0..3).map(|c| vec![0.0, 0.5, 1.0, 0.25, 0.75, c as f32]).collect();
        let rt: Vec<&[f32]> = t.iter().map(|v| &v[..]).collect();
        let (pa, pb) = ([&a0[..]], [&b0[..]]);
        let mut o = outs(3, 6);
        go(&Params { nout: 3, n: 6 }, &pop(1, &pa), &pop(1, &pb), &pop(3, &rt), &mut o, Mutant::None).unwrap();
        for c in 0..3 {
            assert_eq!(o[c][1], 3.0);
            assert_eq!(o[c][5], 2.0 + 2.0 * c as f32);
        }
    }

    #[test]
    fn degenerate_and_determinism() {
        let a0 = [1.0f32; 8];
        let pa = [&a0[..]];
        for n in [0, -1, -100] {
            let mut o = outs(1, 8);
            go(&Params { nout: 1, n }, &pop(1, &pa), &pop(1, &pa), &pop(1, &pa), &mut o, Mutant::None).unwrap();
            assert!(o[0].iter().all(|x| x.to_bits() == S));
        }
        let mut o = outs(1, 8);
        go(&Params { nout: 0, n: 8 }, &pop(1, &pa), &pop(1, &pa), &pop(1, &pa), &mut o, Mutant::None).unwrap();
        assert!(o[0].iter().all(|x| x.to_bits() == S));
        // errors: plane missing, short plane, short out
        let mut o = outs(1, 8);
        assert_eq!(go(&Params { nout: 1, n: 8 }, &pop(1, &[]), &pop(1, &pa), &pop(1, &pa), &mut o, Mutant::None), Err(InterpError::MissingPlane));
        let mut o = outs(1, 4);
        assert_eq!(go(&Params { nout: 1, n: 8 }, &pop(1, &pa), &pop(1, &pa), &pop(1, &pa), &mut o, Mutant::None), Err(InterpError::ShortOut));
        // NaN/inf: finite in gives finite out for finite t; NaN in gives NaN class out; deterministic
        let (x, y) = ([f32::NAN, f32::INFINITY, 1.0, -f32::INFINITY], [1.0f32, 1.0, f32::NAN, 1.0]);
        let (px, py) = ([&x[..]], [&y[..]]);
        let mut o1 = outs(1, 4);
        let mut o2 = outs(1, 4);
        go(&Params { nout: 1, n: 4 }, &pop(1, &px), &pop(1, &py), &cop(1, [0.5, 0.0, 0.0, 0.0]), &mut o1, Mutant::None).unwrap();
        go(&Params { nout: 1, n: 4 }, &pop(1, &px), &pop(1, &py), &cop(1, [0.5, 0.0, 0.0, 0.0]), &mut o2, Mutant::None).unwrap();
        assert!(o1[0][0].is_nan() && o1[0][2].is_nan());
        assert_eq!(o1[0].iter().map(|v| v.to_bits()).collect::<Vec<_>>(), o2[0].iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    }

    #[test]
    fn all_eight_variants_agree_with_plane_formula() {
        let n = 37;
        let a: Vec<f32> = (0..n).map(|i| i as f32 * 0.37 - 3.0).collect();
        let b: Vec<f32> = (0..n).map(|i| 5.0 - i as f32 * 0.21).collect();
        let t: Vec<f32> = (0..n).map(|i| (i as f32 * 0.11) % 1.3).collect();
        let (pa, pb, pt) = ([&a[..]], [&b[..]], [&t[..]]);
        let (ca, cb, ct) = (0.7f32, -1.3f32, 0.9f32);
        for bits in 0..8u32 {
            let (fa, fb, ft) = (bits & 4 != 0, bits & 2 != 0, bits & 1 != 0);
            let oa = if fa { cop(1, [ca; 4]) } else { pop(1, &pa) };
            let ob = if fb { cop(1, [cb; 4]) } else { pop(1, &pb) };
            let ot = if ft { cop(1, [ct; 4]) } else { pop(1, &pt) };
            let mut o = outs(1, n);
            go(&Params { nout: 1, n: n as i32 }, &oa, &ob, &ot, &mut o, Mutant::None).unwrap();
            for i in 0..n {
                let (x, y, z) = (if fa { ca } else { a[i] }, if fb { cb } else { b[i] }, if ft { ct } else { t[i] });
                assert_eq!(o[0][i].to_bits(), (y - x).mul_add(z, x).to_bits(), "variant {} i {}", bits, i);
            }
        }
    }

    #[test]
    fn assumptions_listed() {
        assert!(assumptions().len() >= 6);
    }
}
