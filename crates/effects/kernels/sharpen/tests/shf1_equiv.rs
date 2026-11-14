//! exhaustive equivalence proofs for the two surviving mutants (lens: every gain bit pattern, every small index pair).
use std::hint::black_box;
#[test]
fn m21_unit_test_bitwise_equals_float_equality_for_all_2_32_gains() {
    let one = black_box(1.0f32);
    let mut n_one = 0u64;
    for b in 0..=u32::MAX {
        let g = f32::from_bits(black_box(b));
        let a = g == one;
        let c = g.to_bits() == one.to_bits();
        assert_eq!(a, c, "bits {:#010x}", b);
        n_one += a as u64;
    }
    assert_eq!(n_one, 1, "exactly one bit pattern equals 1.0 (NaN, +-0, denormals included in the sweep)");
}
#[test]
fn m22_gain_index_ge_equals_gt_for_all_small_pairs() {
    let f_ge = |chan: usize, n: usize| if chan >= n { n } else { chan };
    let f_gt = |chan: usize, n: usize| if chan > n { n } else { chan };
    for chan in 0..4096usize {
        for n in 0..4096usize {
            assert_eq!(f_ge(chan, n), f_gt(chan, n));
            assert_eq!(sharpen::plan::gain_index(chan, n), f_ge(chan, n));
        }
    }
}
