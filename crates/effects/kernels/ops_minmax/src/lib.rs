//! ops_minmax: native port of the operator classes `max` (vtable, slot 53, AVX2 body), `min` (
//! ), `abs` and `negate` . Listing offsets, VA = offset.
//! std only, no unsafe, explicit parameters (no Default), S3-style `render` per class module.
pub mod abs;
pub mod common;
pub mod max;
pub mod min;
pub mod negate;
pub use common::{assumptions, BKind, Mutant, OpError, Operand, Params, UKind, UParams, ONE};

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
        Params { nout, n, vec_one: 1.0, dst3_phase_bytes: 0 }
    }
    fn bin(k: BKind, p: &Params, a: &Operand, b: &Operand, o: &mut Vec<Vec<f32>>) -> Result<(), OpError> {
        let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        common::run_bin(k, p, a, b, &mut m, Mutant::None)
    }
    fn un(k: UKind, p: &UParams, count: i32, pl: &[&[f32]], o: &mut Vec<Vec<f32>>) -> Result<(), OpError> {
        let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        common::run_un(k, p, count, pl, &mut m, Mutant::None)
    }

    #[test]
    fn hand_examples_max_min() {
        let a = [1.0f32, -2.0, 3.5, 0.0];
        let b = [2.0f32, -3.0, 3.5, -1.0];
        let (pa, pb) = ([&a[..]], [&b[..]]);
        let mut o = outs(1, 6);
        bin(BKind::Max, &p(1, 4), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
        assert_eq!(&o[0][..4], &[2.0, -2.0, 3.5, 0.0]);
        assert_eq!(o[0][4].to_bits(), S);
        let mut o = outs(1, 6);
        bin(BKind::Min, &p(1, 4), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
        assert_eq!(&o[0][..4], &[1.0, -3.0, 3.5, -1.0]);
    }

    #[test]
    fn nan_and_zero_follow_the_second_source() {
        // SDM: a NaN in either operand, and a pair of zeros, return the SECOND source
        let a = [f32::NAN, 1.0, 0.0, -0.0];
        let b = [1.0f32, f32::NAN, -0.0, 0.0];
        let (pa, pb) = ([&a[..]], [&b[..]]);
        for (k, tag) in [(BKind::Max, "max"), (BKind::Min, "min")] {
            let mut o = outs(1, 4);
            bin(k, &p(1, 4), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
            assert_eq!(o[0][0].to_bits(), 1.0f32.to_bits(), "{} nan,1", tag);
            assert!(o[0][1].is_nan(), "{} 1,nan", tag);
            assert_eq!(o[0][2].to_bits(), (-0.0f32).to_bits(), "{} +0,-0", tag);
            assert_eq!(o[0][3].to_bits(), 0.0f32.to_bits(), "{} -0,+0", tag);
        }
    }

    #[test]
    fn constants_and_order() {
        let a = [5.0f32, f32::NAN];
        let pa = [&a[..]];
        // flag B: f(A, Bconst); flag A: f(Aconst, B)
        let mut o = outs(1, 2);
        bin(BKind::Max, &p(1, 2), &op(1, &pa), &cop(1, [3.0, 0.0, 0.0, 0.0]), &mut o).unwrap();
        assert_eq!(o[0][0], 5.0);
        assert_eq!(o[0][1], 3.0); // NaN in A, second source = the constant
        let mut o = outs(1, 2);
        bin(BKind::Max, &p(1, 2), &cop(1, [3.0, 0.0, 0.0, 0.0]), &op(1, &pa), &mut o).unwrap();
        assert_eq!(o[0][0], 5.0);
        assert!(o[0][1].is_nan()); // NaN in B, second source = B
    }

    #[test]
    fn extension_channel_three() {
        // (3,4): out[3] = f(ONE, B3); (4,3): out[3] = f(A3, ONE)
        let n = 40usize;
        let c: Vec<Vec<f32>> = (0..4).map(|k| (0..n).map(|i| i as f32 * 0.1 - 1.0 + k as f32).collect()).collect();
        let pl: Vec<&[f32]> = c.iter().map(|v| v.as_slice()).collect();
        let three = [pl[0], pl[1], pl[2]];
        let mut o = outs(4, 64);
        bin(BKind::Max, &p(4, n as i32), &op(3, &three), &op(4, &pl), &mut o).unwrap();
        for i in 0..n {
            assert_eq!(o[3][i].to_bits(), common::x86_max(1.0, c[3][i]).to_bits());
        }
        let mut o = outs(4, 64);
        bin(BKind::Min, &p(4, n as i32), &op(4, &pl), &op(3, &three), &mut o).unwrap();
        for i in 0..n {
            assert_eq!(o[3][i].to_bits(), common::x86_min(c[3][i], 1.0).to_bits());
        }
        // NaN plane element: B side keeps the NaN (second source), A side takes ONE
        let mut c2 = c.clone();
        c2[3][5] = f32::NAN;
        let pl2: Vec<&[f32]> = c2.iter().map(|v| v.as_slice()).collect();
        let mut o = outs(4, 64);
        bin(BKind::Max, &p(4, n as i32), &op(3, &three), &op(4, &pl2), &mut o).unwrap();
        assert!(o[3][5].is_nan());
        let mut o = outs(4, 64);
        bin(BKind::Max, &p(4, n as i32), &op(4, &pl2), &op(3, &three), &mut o).unwrap();
        assert_eq!(o[3][5], 1.0);
        // constant channel 3 fills n elements
        let mut o = outs(4, 64);
        bin(BKind::Max, &p(4, 7), &op(3, &three), &cop(4, [0.0, 0.0, 0.0, -9.0]), &mut o).unwrap();
        assert!(o[3][..7].iter().all(|&x| x == 1.0));
        assert_eq!(o[3][7].to_bits(), S);
    }

    #[test]
    fn vec_one_only_reaches_the_vector_core() {
        // n = 40, phase 0: peel 0, vector end 32; elements 32..40 are scalar (ONE), 0..32 use vec_one
        let x = vec![0.0f32; 40];
        let a3: Vec<Vec<f32>> = (0..3).map(|_| vec![0.0; 40]).collect();
        let pl4: Vec<&[f32]> = vec![&a3[0], &a3[1], &a3[2], &x];
        let three: Vec<&[f32]> = vec![&a3[0], &a3[1], &a3[2]];
        let mut o = outs(4, 64);
        let pp = Params { nout: 4, n: 40, vec_one: 0.75, dst3_phase_bytes: 0 };
        bin(BKind::Max, &pp, &op(3, &three), &op(4, &pl4), &mut o).unwrap();
        assert_eq!(o[3][0], 0.75);
        assert_eq!(o[3][31], 0.75);
        assert_eq!(o[3][32], 1.0);
        assert_eq!(o[3][39], 1.0);
    }

    #[test]
    fn unary_hand_values_and_masks() {
        let x = [-1.5f32, 2.0, -0.0, f32::from_bits(0xffc00123)];
        let pl = [&x[..]];
        let up = UParams { n: 4, vec_mask: 0x7fff_ffff, dst_phase_bytes: [0; 4] };
        let mut o = outs(1, 4);
        un(UKind::Abs, &up, 1, &pl, &mut o).unwrap();
        assert_eq!(o[0][0], 1.5);
        assert_eq!(o[0][2].to_bits(), 0);
        assert_eq!(o[0][3].to_bits(), 0x7fc00123); // payload kept, sign cleared
        let up = UParams { n: 4, vec_mask: 0x8000_0000, dst_phase_bytes: [0; 4] };
        let mut o = outs(1, 4);
        un(UKind::Negate, &up, 1, &pl, &mut o).unwrap();
        assert_eq!(o[0][0], 1.5);
        assert_eq!(o[0][2].to_bits(), 0);
        assert_eq!(o[0][3].to_bits(), 0x7fc00123);
    }

    #[test]
    fn unary_vector_core_uses_vec_mask() {
        // n = 40, phase 0: vector end 32 (core uses vec_mask), scalar tail uses the captured mask
        let x: Vec<f32> = (0..40).map(|i| -(i as f32) - 1.0).collect();
        let pl = [&x[..]];
        let up = UParams { n: 40, vec_mask: 0x3fff_ffff, dst_phase_bytes: [0; 4] };
        let mut o = outs(1, 40);
        un(UKind::Abs, &up, 1, &pl, &mut o).unwrap();
        assert_eq!(o[0][39], 40.0);
        assert_eq!(o[0][0].to_bits(), x[0].to_bits() & 0x3fff_ffff);
        // phase 4: peel (32-4)>>2 = 7 scalar head elements
        let up = UParams { n: 40, vec_mask: 0x3fff_ffff, dst_phase_bytes: [4; 4] };
        let mut o = outs(1, 40);
        un(UKind::Abs, &up, 1, &pl, &mut o).unwrap();
        assert_eq!(o[0][6], 7.0);
        assert_eq!(o[0][7].to_bits(), x[7].to_bits() & 0x3fff_ffff);
        // phase 1 (not a multiple of 4): all scalar
        let up = UParams { n: 40, vec_mask: 0x3fff_ffff, dst_phase_bytes: [1; 4] };
        let mut o = outs(1, 40);
        un(UKind::Abs, &up, 1, &pl, &mut o).unwrap();
        assert!((0..40).all(|i| o[0][i] == (i as f32 + 1.0)));
    }

    #[test]
    fn channels_and_degenerate() {
        let c: Vec<Vec<f32>> = (0..4).map(|k| vec![-(k as f32) - 1.0; 5]).collect();
        let pl: Vec<&[f32]> = c.iter().map(|v| v.as_slice()).collect();
        let up = UParams { n: 5, vec_mask: 0x7fff_ffff, dst_phase_bytes: [0; 4] };
        let mut o = outs(4, 8);
        un(UKind::Abs, &up, 3, &pl, &mut o).unwrap();
        assert_eq!(o[2][0], 3.0);
        assert_eq!(o[3][0].to_bits(), S); // count 3 leaves channel 3 untouched
        // n 0 and negative: nothing written, no error; count 0: nothing
        for n in [0, -1, -100] {
            let up = UParams { n, vec_mask: 0x7fff_ffff, dst_phase_bytes: [0; 4] };
            let mut o = outs(4, 8);
            un(UKind::Negate, &up, 4, &pl, &mut o).unwrap();
            assert!(o.iter().all(|v| v.iter().all(|x| x.to_bits() == S)));
            bin(BKind::Max, &Params { nout: 1, n, vec_one: 1.0, dst3_phase_bytes: 0 }, &op(1, &[&c[0][..]]), &op(1, &[&c[1][..]]), &mut o).unwrap();
            assert!(o.iter().all(|v| v.iter().all(|x| x.to_bits() == S)));
        }
        let mut o = outs(4, 8);
        un(UKind::Abs, &up, 0, &pl, &mut o).unwrap();
        assert!(o.iter().all(|v| v.iter().all(|x| x.to_bits() == S)));
        // missing plane and short plane are errors
        assert_eq!(un(UKind::Abs, &up, 5, &pl, &mut outs(5, 8)), Err(OpError::MissingPlane));
        let short = [1.0f32; 3];
        assert_eq!(un(UKind::Abs, &up, 1, &[&short[..]], &mut outs(1, 8)), Err(OpError::ShortPlane));
    }

    #[test]
    fn identity_and_determinism() {
        // max(x, x) = x and min(x, x) = x for non-NaN; two runs give equal bits
        let a: Vec<f32> = (0..50).map(|i| (i as f32 - 20.0) * 0.37).collect();
        let pa = [&a[..]];
        let mut o1 = outs(1, 50);
        let mut o2 = outs(1, 50);
        bin(BKind::Max, &p(1, 50), &op(1, &pa), &op(1, &pa), &mut o1).unwrap();
        bin(BKind::Max, &p(1, 50), &op(1, &pa), &op(1, &pa), &mut o2).unwrap();
        assert_eq!(o1, o2);
        assert!((0..50).all(|i| o1[0][i].to_bits() == a[i].to_bits()));
        let mut o = outs(1, 50);
        bin(BKind::Min, &p(1, 50), &op(1, &pa), &op(1, &pa), &mut o).unwrap();
        assert!((0..50).all(|i| o[0][i].to_bits() == a[i].to_bits()));
        // abs twice = abs; negate twice = identity (bits)
        let up = UParams { n: 50, vec_mask: 0x8000_0000, dst_phase_bytes: [0; 4] };
        let mut o = outs(1, 50);
        un(UKind::Negate, &up, 1, &pa, &mut o).unwrap();
        let o0 = o[0].clone();
        let mut o2 = outs(1, 50);
        un(UKind::Negate, &up, 1, &[&o0[..]], &mut o2).unwrap();
        assert!((0..50).all(|i| o2[0][i].to_bits() == a[i].to_bits()));
    }

    #[test]
    fn module_entries_and_assumptions() {
        let a = [1.0f32, 4.0];
        let b = [3.0f32, 2.0];
        let (pa, pb) = ([&a[..]], [&b[..]]);
        let mut o = outs(1, 2);
        let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        max::render(&p(1, 2), &op(1, &pa), &op(1, &pb), &mut m).unwrap();
        drop(m);
        assert_eq!(o[0], vec![3.0, 4.0]);
        let mut o = outs(1, 2);
        let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        min::render(&p(1, 2), &op(1, &pa), &op(1, &pb), &mut m).unwrap();
        drop(m);
        assert_eq!(o[0], vec![1.0, 2.0]);
        assert_eq!(assumptions().len(), 10);
    }
}
