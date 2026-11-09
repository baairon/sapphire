//! The constant fold of shake_shift against the crate's own body run at the same constant arguments.
use fx_eval::compile::ArgView2;
use fx_eval::fold::{fold_kind, fold_lanes};
use shake_shift::body::{shake_shift_body, Config, Frame, Globals, Params, TAB_LEN};

fn s(x: f32) -> ArgView2 {
    ArgView2 { is_const: true, lanes: [x; 4], n: 1, elnum: Some(1) }
}

/// The crate's result for twelve scalar arguments, with tables built by the crate.
fn crate_value(a: &[f32; 12]) -> f32 {
    let t = shake_shift::weights::build_avx();
    let scale = shake_shift::weights::scale_from_cutoff(t.cutoff.expect("the weight table ends in a zero entry"));
    let mut g = Globals { guard: 0, tab: vec![[0; 3]; TAB_LEN], tail: [0; 3], weights: t.float, scale };
    let p = Params { p4c: a[0], p7c: a[1], pac: a[2], pdc: a[3], p10c: a[4], p13c: a[5], p16c: a[6], p19c: a[7], p1cc: a[8], p1fc: a[9], p22c: a[10], p25c: a[11] };
    shake_shift_body(&p, &Config::default(), &mut g, &mut shake_shift::RefServices::new(), &mut Frame::new())
}

fn cases() -> Vec<[f32; 12]> {
    vec![
        // route selector 0 and 1, a typical Normal-style binding, the all-zero binding with unit factor
        [3.0, 0.5, 0.25, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0],
        [3.0, 0.5, 0.25, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 1.0],
        [0.0; 12],
        [7.0, 1.75, 0.1, 2.0, 0.05, 3.0, 0.02, 9.0, 4.0, 0.0, 0.3, 0.8],
        [7.0, 1.75, 0.1, 2.0, 0.05, 3.0, 0.02, 9.0, 4.0, 1.0, 0.3, 0.8],
        [-2.0, 12.5, 1.0, 0.5, 0.2, 0.1, 0.4, 2.0, -3.0, 1.0, 0.9, 0.5],
        [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, f32::NAN, 0.0, 1.0],
    ]
}

#[test]
fn fold_equals_the_crate_body_at_constant_arguments() {
    assert!(fold_kind("shake_shift").is_some());
    for c in cases() {
        let args: Vec<ArgView2> = c.iter().map(|&x| s(x)).collect();
        let want = crate_value(&c).to_bits();
        for n in 1..=3 {
            let r = fold_lanes("shake_shift", &args, n, None).expect("folds");
            for i in 0..n as usize {
                assert_eq!(r[i], want, "case {:?} lanes {} lane {}", c, n, i);
            }
        }
    }
}

#[test]
fn fold_is_deterministic_and_finite_on_ordinary_inputs() {
    let c = cases()[0];
    let args: Vec<ArgView2> = c.iter().map(|&x| s(x)).collect();
    let a = fold_lanes("shake_shift", &args, 1, None).unwrap();
    let b = fold_lanes("shake_shift", &args, 1, None).unwrap();
    assert_eq!(a, b);
    assert!(f32::from_bits(a[0]).is_finite());
}

#[test]
fn fold_refuses_missing_arguments() {
    let args: Vec<ArgView2> = (0..11).map(|_| s(0.0)).collect();
    assert!(fold_lanes("shake_shift", &args, 1, None).is_none());
}

#[test]
fn nan_argument_reaches_the_output() {
    let mut c = cases()[0];
    c[11] = f32::NAN;
    let args: Vec<ArgView2> = c.iter().map(|&x| s(x)).collect();
    let r = fold_lanes("shake_shift", &args, 1, None).unwrap();
    assert!(f32::from_bits(r[0]).is_nan());
}
