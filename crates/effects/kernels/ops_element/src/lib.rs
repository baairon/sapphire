//! ops_element: native std-only ports of the builtin classes element_of, max_element, min_element (AVX2 route, flag 0).
pub mod common;
pub mod element_of;
pub mod max_element;
pub mod min_element;
pub mod reduce;

pub use common::{assumptions, ElError, Svc, Trace};
pub use element_of::{EMutant, EParams};
pub use reduce::{Kind, RMutant, RParams};

#[cfg(test)]
mod tests {
    use super::common::*;
    use super::element_of::{render as eo, EParams};
    use super::max_element;
    use super::min_element;
    use super::reduce::RParams;

    fn svc() -> Svc {
        Svc { in_pool: false, is_int: false, values: [0; 4], enable: false, img_valid: true }
    }
    fn pl(v: &[f32]) -> Vec<f32> {
        v.to_vec()
    }

    #[test]
    fn t1_max_hand_example() {
        // c0 = [1, 7, 2], c1 = [5, 3, 2], c2 = [3, 4, 9] -> [5, 7, 9]
        let (a, b, c) = (pl(&[1.0, 7.0, 2.0]), pl(&[5.0, 3.0, 2.0]), pl(&[3.0, 4.0, 9.0]));
        let mut d = vec![0.0; 3];
        let t = max_element::render(&RParams { count: 3, n: 3 }, &[&a, &b, &c], &svc(), &mut d).unwrap();
        assert_eq!(d, vec![5.0, 7.0, 9.0]);
        assert_eq!(t.service_calls, 0);
        let mut e = vec![0.0; 3];
        min_element::render(&RParams { count: 3, n: 3 }, &[&a, &b, &c], &svc(), &mut e).unwrap();
        assert_eq!(e, vec![1.0, 3.0, 2.0]);
    }

    #[test]
    fn t1_nan_and_zero_rule() {
        // max(a, b) = a > b ? a : b : NaN in either operand and a zero pair return b (second source)
        let nan = f32::NAN;
        let (a, b) = (pl(&[nan, 1.0, 0.0, -0.0]), pl(&[1.0, nan, -0.0, 0.0]));
        let mut d = vec![9.0; 4];
        max_element::render(&RParams { count: 2, n: 4 }, &[&a, &b], &svc(), &mut d).unwrap();
        assert_eq!(d[0], 1.0);
        assert!(d[1].is_nan());
        assert_eq!(d[2].to_bits(), (-0.0f32).to_bits());
        assert_eq!(d[3].to_bits(), 0.0f32.to_bits());
        let mut e = vec![9.0; 4];
        min_element::render(&RParams { count: 2, n: 4 }, &[&a, &b], &svc(), &mut e).unwrap();
        assert_eq!(e[0], 1.0);
        assert!(e[1].is_nan());
        assert_eq!(e[2].to_bits(), (-0.0f32).to_bits());
        assert_eq!(e[3].to_bits(), 0.0f32.to_bits());
    }

    #[test]
    fn t1_count1_goes_to_service_and_degenerate() {
        let a = pl(&[3.0, 4.0, 5.0, 6.0]);
        let mut d = vec![0.0; 4];
        let t = max_element::render(&RParams { count: 1, n: 4 }, &[&a], &svc(), &mut d).unwrap();
        assert_eq!(d, a);
        assert_eq!(t.service_calls, 1);
        let mut z = vec![7.0; 4];
        max_element::render(&RParams { count: 2, n: 0 }, &[&a, &a], &svc(), &mut z).unwrap();
        assert_eq!(z, vec![7.0; 4]);
        assert_eq!(max_element::render(&RParams { count: 5, n: 4 }, &[&a], &svc(), &mut z), Err(ElError::BadCount(5)));
        assert_eq!(max_element::render(&RParams { count: 0, n: 4 }, &[], &svc(), &mut z), Err(ElError::BadCount(0)));
    }

    #[test]
    fn t1_rem3_and_fold() {
        // signed remainder via the 0x55555556 multiply, negative fixed by +3 in the caller
        assert_eq!(rem3_raw(0), 0);
        assert_eq!(rem3_raw(4), 1);
        assert_eq!(rem3_raw(-1), -1);
        assert_eq!(rem3_raw(-3), 0);
        assert_eq!(rem3_raw(i32::MIN), -2);
        assert_eq!(cvtt(f32::NAN), i32::MIN);
        assert_eq!(cvtt(2.9), 2);
        assert_eq!(cvtt(-0.5), 0);
        assert_eq!(cvtt(1e10), i32::MIN);
    }

    fn run(count: i32, idx: [f32; 4], nout: i32, ac: Option<[f32; 4]>) -> Vec<Vec<f32>> {
        let mut planes: Vec<Vec<f32>> = (0..count).map(|c| (0..5).map(|i| (c * 10 + i) as f32).collect()).collect();
        let mut outs: Vec<Vec<f32>> = (0..4).map(|_| vec![-1.0; 5]).collect();
        let p = EParams { count, nout, n: 5, a_const: ac.is_some(), a_consts: ac.unwrap_or([0.0; 4]), idx };
        eo(&p, &mut planes, &svc(), &mut outs, [-1; 4]).unwrap();
        outs
    }

    #[test]
    fn t1_element_of_gather() {
        // count 3 image, idx (0, 1, 2): plane copies
        let o = run(3, [0.0, 1.0, 2.0, 0.0], 3, None);
        assert_eq!(o[0], vec![0.0, 1.0, 2.0, 3.0, 4.0]);
        assert_eq!(o[1], vec![10.0, 11.0, 12.0, 13.0, 14.0]);
        assert_eq!(o[2], vec![20.0, 21.0, 22.0, 23.0, 24.0]);
        assert_eq!(o[3], vec![-1.0; 5]);
        // count 3 wraps: idx 4 -> 1, idx -1 -> 2, idx NaN -> INT_MIN -> 1, idx 2.7 -> 2
        let o = run(3, [4.0, -1.0, f32::NAN, 2.7], 4, None);
        assert_eq!(o[0][0], 10.0);
        assert_eq!(o[1][0], 20.0);
        assert_eq!(o[2][0], 10.0);
        assert_eq!(o[3][0], 20.0);
        // count 4 and 2 mask: idx 5 -> 1, idx -1 -> 3 (count 4), NaN -> 0
        let o = run(4, [5.0, -1.0, f32::NAN, 3.0], 4, None);
        assert_eq!((o[0][0], o[1][0], o[2][0], o[3][0]), (10.0, 30.0, 0.0, 30.0));
        let o = run(2, [3.0, -2.0, 1.0, 0.0], 3, None);
        assert_eq!((o[0][0], o[1][0], o[2][0]), (10.0, 0.0, 10.0));
        // constant A: fills with the selected constant
        let o = run(3, [2.0, 0.0, 1.0, 0.0], 3, Some([7.0, 8.0, 9.0, 0.0]));
        assert_eq!((o[0][4], o[1][4], o[2][4]), (9.0, 7.0, 8.0));
    }

    #[test]
    fn t1_alias_skips_service() {
        let mut planes: Vec<Vec<f32>> = vec![vec![1.0; 4], vec![2.0; 4], vec![3.0; 4]];
        let mut outs: Vec<Vec<f32>> = (0..4).map(|_| vec![-1.0; 4]).collect();
        let p = EParams { count: 3, nout: 2, n: 4, a_const: false, a_consts: [0.0; 4], idx: [1.0, 0.0, 0.0, 0.0] };
        // out0 is plane 1 (idx 1: skipped), out1 is plane 2 (idx 0: copy plane 0 into plane 2)
        let t = eo(&p, &mut planes, &svc(), &mut outs, [1, 2, -1, -1]).unwrap();
        assert_eq!(t.service_calls, 1);
        assert_eq!(planes[1], vec![2.0; 4]);
        assert_eq!(planes[2], vec![1.0; 4]);
        assert_eq!(outs[0], vec![-1.0; 4]);
    }

    #[test]
    fn t3_determinism_and_errors() {
        let a = run(4, [0.5, 1.5, 2.5, 3.5], 4, None);
        let b = run(4, [0.5, 1.5, 2.5, 3.5], 4, None);
        assert_eq!(a, b);
        let mut planes = vec![vec![0.0; 3]; 3];
        let mut outs = vec![vec![0.0; 3]; 4];
        let p = EParams { count: 3, nout: 5, n: 3, a_const: false, a_consts: [0.0; 4], idx: [0.0; 4] };
        assert_eq!(eo(&p, &mut planes, &svc(), &mut outs, [-1; 4]), Err(ElError::BadNout(5)));
        let p = EParams { count: 9, nout: 1, n: 3, a_const: false, a_consts: [0.0; 4], idx: [0.0; 4] };
        assert_eq!(eo(&p, &mut planes, &svc(), &mut outs, [-1; 4]), Err(ElError::BadCount(9)));
        assert!(assumptions().len() >= 8);
    }
}
