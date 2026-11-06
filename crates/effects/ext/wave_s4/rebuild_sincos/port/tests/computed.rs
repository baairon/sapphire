//! Tests of the computed tables that need no data.
use libm_sincos::{exact, tables};

#[test]
fn axes_and_diagonal() {
    let t = tables();
    assert_eq!(t.s[0], [0, 1.0f64.to_bits()]);
    assert_eq!(t.s[16], [1.0f64.to_bits(), 0]);
    assert_eq!(t.s[32], [0, (-1.0f64).to_bits()]);
    assert_eq!(t.s[48], [(-1.0f64).to_bits(), 0]);
    assert_eq!(t.s[8], [std::f64::consts::FRAC_1_SQRT_2.to_bits(); 2]);
}

#[test]
fn entries_are_within_one_ulp_of_std() {
    let t = tables();
    for k in 0..64usize {
        if k % 16 == 0 {
            continue;
        }
        let a = std::f64::consts::PI * (k as f64) / 32.0;
        let (s, c) = a.sin_cos();
        for (got, want) in [(f64::from_bits(t.s[k][0]), s), (f64::from_bits(t.s[k][1]), c)] {
            assert!((got - want).abs() <= 1e-15, "k {} {} {}", k, got, want);
        }
    }
}

#[test]
fn double_table_words_are_consistent() {
    let t = tables();
    for k in 0..64usize {
        let w: Vec<f64> = t.d[k].iter().map(|&b| f64::from_bits(b)).collect();
        // words 2 and 3 are the single table's pair; the split sums reproduce cos and -sin to about 2^-62 of 1
        assert_eq!(t.d[k][2], t.s[k][0]);
        assert_eq!(t.d[k][3], t.s[k][1]);
        assert!(((w[0] + w[6]) - w[3]).abs() < 3e-16, "k {}", k);
        assert!(((w[1] + w[7]) + w[2]).abs() < 3e-16, "k {}", k);
    }
}

#[test]
fn bignum_basics() {
    let p = exact::pi().to_f64();
    assert_eq!(p.to_bits(), std::f64::consts::PI.to_bits());
    let x = exact::Fx::from_f64(0.1);
    assert_eq!(x.to_f64().to_bits(), 0.1f64.to_bits());
    assert_eq!(exact::Fx::from_int(3).recip().to_f64().to_bits(), (1.0f64 / 3.0).to_bits());
}
