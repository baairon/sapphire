//! ops_arith: native port of the arithmetic operator classes `+` (vtable, slot 53, AVX2 body),
//! `-*` . Listing offsets, VA = offset.
//! std only, no unsafe, explicit parameters (no Default), S3-style `render` per class module. See README.md and SPEC.md.
pub mod add;
pub mod common;
pub mod div;
pub mod mul;
pub mod sub;
pub use common::{assumptions, Kind, Mutant, OpError, Operand, Params, DIV_K, ONE};

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
    fn run(k: Kind, p: &Params, a: &Operand, b: &Operand, o: &mut Vec<Vec<f32>>) -> Result<(), OpError> {
        let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
        common::run(k, p, a, b, &mut m, Mutant::None)
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
    fn plane_plane_all_ops() {
        let a0 = pl(40, |i| i as f32 + 1.0);
        let b0 = pl(40, |i| 2.0 * i as f32 + 0.5);
        let (pa, pb) = ([a0.as_slice()], [b0.as_slice()]);
        for (k, f) in [(Kind::Add, (|x: f32, y: f32| x + y) as fn(f32, f32) -> f32), (Kind::Sub, |x, y| x - y), (Kind::Mul, |x, y| x * y), (Kind::Div, |x, y| x / y)] {
            let mut o = outs(1, 64);
            run(k, &p(1, 40), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
            for i in 0..40 {
                assert_eq!(o[0][i].to_bits(), f(a0[i], b0[i]).to_bits(), "{:?} {}", k, i);
            }
            assert_eq!(o[0][40].to_bits(), S);
        }
        // hand values: 3 + 2.5, 3 - 2.5, 3 * 2.5, 3 / 2.5 (i = 2: a = 3, b = 4.5 -> 7.5, -1.5, 13.5, 0.6666667)
        let mut o = outs(1, 8);
        run(Kind::Add, &p(1, 3), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
        assert_eq!(o[0][2], 7.5);
    }

    #[test]
    fn channel_clamp_and_scalar_broadcast() {
        // A has 1 channel, B has 3: out[c] = A0 op B[c]; nout = 3 takes the fixed 3 channel loop
        let a0 = [10.0f32, 20.0];
        let b: Vec<Vec<f32>> = (0..3).map(|c| vec![c as f32, c as f32 + 0.5]).collect();
        let (pa, pb): ([&[f32]; 1], Vec<&[f32]>) = ([&a0], b.iter().map(|v| v.as_slice()).collect());
        let mut o = outs(3, 4);
        run(Kind::Sub, &p(3, 2), &op(1, &pa), &op(3, &pb), &mut o).unwrap();
        assert_eq!(o[2][0], 8.0);
        assert_eq!(o[2][1], 17.5);
        assert_eq!(o[0][1], 19.5);
    }

    #[test]
    fn constants_and_operand_order() {
        let x = [7.0f32, 8.0];
        let (pa, pb) = ([&x[..]], [&x[..]]);
        let mut o = outs(1, 2);
        // A constant: out = Aconst - B ; B constant: out = A - Bconst
        run(Kind::Sub, &p(1, 2), &cop(1, [100.0, 0.0, 0.0, 0.0]), &op(1, &pb), &mut o).unwrap();
        assert_eq!((o[0][0], o[0][1]), (93.0, 92.0));
        run(Kind::Sub, &p(1, 2), &op(1, &pa), &cop(1, [3.0, 0.0, 0.0, 0.0]), &mut o).unwrap();
        assert_eq!((o[0][0], o[0][1]), (4.0, 5.0));
        run(Kind::Div, &p(1, 2), &cop(1, [1.0, 0.0, 0.0, 0.0]), &op(1, &pb), &mut o).unwrap();
        assert_eq!(o[0][1], 0.125);
    }

    #[test]
    fn division_zero_rule() {
        let a = [3.0f32, 3.0, 3.0, 3.0, 3.0];
        let b = [0.0f32, -0.0, f32::NAN, f32::INFINITY, 2.0];
        let (pa, pb) = ([&a[..]], [&b[..]]);
        let mut o = outs(1, 5);
        run(Kind::Div, &p(1, 5), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
        assert_eq!(o[0][0], 3_000_000.0); // K * a
        assert_eq!(o[0][1], 3_000_000.0); // -0 also takes the zero branch
        assert!(o[0][2].is_nan());
        assert_eq!(o[0][3], 0.0);
        assert_eq!(o[0][4], 1.5);
        // B constant zero: K * A
        run(Kind::Div, &p(1, 2), &op(1, &pa), &cop(1, [0.0; 4]), &mut o).unwrap();
        assert_eq!(o[0][0], 3_000_000.0);
    }

    fn ext_case(k: Kind, a_count: i32, b_count: i32, n: usize, vec_one: f32, phase: u32) -> Vec<Vec<f32>> {
        let mk = |c: usize| pl(n, move |i| (i as f32) * 0.25 + c as f32 + 1.0);
        let ap: Vec<Vec<f32>> = (0..a_count as usize).map(mk).collect();
        let bp: Vec<Vec<f32>> = (0..b_count as usize).map(|c| mk(c + 10)).collect();
        let (ar, br): (Vec<&[f32]>, Vec<&[f32]>) = (ap.iter().map(|v| v.as_slice()).collect(), bp.iter().map(|v| v.as_slice()).collect());
        let mut o = outs(4, 300);
        run(k, &Params { nout: 4, n: n as i32, vec_one, dst3_phase_bytes: phase }, &op(a_count, &ar), &op(b_count, &br), &mut o).unwrap();
        o
    }

    #[test]
    fn extension_channel_three() {
        // A 3 channels, B 4 channels (r14): out[3] = ONE op B[3]. n = 40, phase 0: peel 0, vector end 32 (lanes use vec_one), tail 32..40 uses ONE
        let o = ext_case(Kind::Add, 3, 4, 40, 0.75, 0);
        let b3 = |i: usize| (i as f32) * 0.25 + 14.0;
        assert_eq!(o[3][0], 0.75 + b3(0));
        assert_eq!(o[3][31], 0.75 + b3(31));
        assert_eq!(o[3][32], 1.0 + b3(32));
        assert_eq!(o[3][39], 1.0 + b3(39));
        assert_eq!(o[3][40].to_bits(), S);
        let o = ext_case(Kind::Sub, 3, 4, 40, 1.0, 0);
        assert_eq!(o[3][5], 1.0 - b3(5));
        // r12 (A 4, B 3): A3 - ONE for sub, copy for mul/div
        let a3 = |i: usize| (i as f32) * 0.25 + 4.0;
        let o = ext_case(Kind::Sub, 4, 3, 40, 1.0, 0);
        assert_eq!(o[3][7], a3(7) - 1.0);
        let o = ext_case(Kind::Mul, 4, 3, 40, 1.0, 0);
        assert_eq!(o[3][7], a3(7));
        let o = ext_case(Kind::Div, 4, 3, 40, 1.0, 0);
        assert_eq!(o[3][39], a3(39));
        // div B side: 1 / x with the 1e6 immediate for zero
        let o = ext_case(Kind::Div, 3, 4, 40, 1.0, 0);
        assert_eq!(o[3][2], 1.0 / b3(2));
        // peel by phase: phase 8 bytes -> peel 6 floats, head with ONE, core with vec_one from 6 .. end
        let o = ext_case(Kind::Add, 3, 4, 70, 0.75, 8);
        // n = 70, peel = (32 - 8) >> 2 = 6, vector end = 70 - ((70 - 6) & 15) = 70
        assert_eq!(o[3][5], 1.0 + (5.0 * 0.25 + 14.0));
        assert_eq!(o[3][6], 0.75 + (6.0 * 0.25 + 14.0));
        assert_eq!(o[3][69], 0.75 + (69.0 * 0.25 + 14.0));
        // phase not a multiple of 4 -> all scalar with ONE
        let o = ext_case(Kind::Add, 3, 4, 70, 0.75, 2);
        assert_eq!(o[3][30], 1.0 + (30.0 * 0.25 + 14.0));
    }

    #[test]
    fn extension_constant_fill() {
        let b: Vec<Vec<f32>> = (0..3).map(|_| vec![1.0f32; 9]).collect();
        let br: Vec<&[f32]> = b.iter().map(|v| v.as_slice()).collect();
        let bc = Operand { count: 4, is_const: true, consts: [0.0, 0.0, 0.0, 4.0], planes: &[] };
        let _ = br;
        let ap: Vec<Vec<f32>> = (0..3).map(|_| vec![2.0f32; 9]).collect();
        let ar: Vec<&[f32]> = ap.iter().map(|v| v.as_slice()).collect();
        let mut o = outs(4, 16);
        // B constant with 4 channels: channels 0..2 take Bconst[c] (0.0): A * 0 ; channel 3 = ONE / 4
        run(Kind::Div, &p(4, 9), &op(3, &ar), &bc, &mut o).unwrap();
        assert_eq!(o[3][8], 0.25);
        assert_eq!(o[0][0], 2_000_000.0); // Bconst[0] == 0 -> K * A
    }

    #[test]
    fn degenerate_and_errors() {
        let a = [1.0f32; 4];
        let pa = [&a[..]];
        let mut o = outs(1, 4);
        run(Kind::Add, &p(0, 4), &op(1, &pa), &op(1, &pa), &mut o).unwrap();
        assert_eq!(o[0][0].to_bits(), S);
        run(Kind::Add, &p(-1, 4), &op(1, &pa), &op(1, &pa), &mut o).unwrap();
        assert_eq!(o[0][0].to_bits(), S);
        run(Kind::Add, &p(1, 0), &op(1, &pa), &op(1, &pa), &mut o).unwrap();
        run(Kind::Add, &p(1, -3), &op(1, &pa), &op(1, &pa), &mut o).unwrap();
        assert_eq!(o[0][0].to_bits(), S);
        assert_eq!(run(Kind::Add, &p(1, 4), &op(1, &[]), &op(1, &pa), &mut o), Err(OpError::MissingPlane));
        assert_eq!(run(Kind::Add, &p(1, 9), &op(1, &pa), &op(1, &pa), &mut o), Err(OpError::MissingOut));
        assert_eq!(run(Kind::Add, &p(2, 4), &op(1, &pa), &op(1, &pa), &mut o), Err(OpError::MissingOut));
        // nout = 3 loops three channels even with two operand channels (clamped)
        let mut o3 = outs(3, 4);
        run(Kind::Mul, &p(3, 4), &op(1, &pa), &op(1, &pa), &mut o3).unwrap();
        assert_eq!(o3[2][3], 1.0);
    }

    #[test]
    fn determinism_and_finite() {
        let a = pl(77, |i| (i as f32 - 30.0) * 1.37);
        let b = pl(77, |i| (i as f32 + 3.0) * 0.5);
        let (pa, pb) = ([a.as_slice()], [b.as_slice()]);
        for k in [Kind::Add, Kind::Sub, Kind::Mul, Kind::Div] {
            let mut o1 = outs(1, 80);
            let mut o2 = outs(1, 80);
            run(k, &p(1, 77), &op(1, &pa), &op(1, &pb), &mut o1).unwrap();
            run(k, &p(1, 77), &op(1, &pa), &op(1, &pb), &mut o2).unwrap();
            let bits = |o: &Vec<Vec<f32>>| o.iter().map(|v| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>()).collect::<Vec<_>>();
            assert_eq!(bits(&o1), bits(&o2));
            assert!(o1[0][..77].iter().all(|v| v.is_finite()));
        }
    }

    #[test]
    fn class_modules_match_core() {
        let a = [1.5f32, 2.5];
        let b = [4.0f32, 0.5];
        let (pa, pb) = ([&a[..]], [&b[..]]);
        let mut o = outs(1, 2);
        {
            let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
            div::render(&p(1, 2), &op(1, &pa), &op(1, &pb), &mut m).unwrap();
        }
        assert_eq!((o[0][0], o[0][1]), (0.375, 5.0));
        assert_eq!(div::KIND, Kind::Div);
        assert_eq!(add::KIND, Kind::Add);
        assert_eq!(sub::KIND, Kind::Sub);
        assert_eq!(mul::KIND, Kind::Mul);
        assert!(assumptions().len() >= 8);
    }
}
