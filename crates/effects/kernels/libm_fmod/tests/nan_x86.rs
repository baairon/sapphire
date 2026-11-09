//! The NaN results of fmod are the x86 words on every processor: zero divisors and infinite dividends are invalid operations.
use libm_fmod::fmod_x;

const EXPECT: &[u64] = &[
    0xfff8000000000000,
    0xfff8000000000000,
    0xfff8000000000000,
    0xfff8000000000000,
    0xfff8000000000000,
    0xfff8000000000000,
    0xfff8000000000000,
    0x7ff8000000000000,
    0x7ff8000000000001,
];

#[test]
fn invalid_operations_give_the_x86_default_nan() {
    let inf = f64::INFINITY;
    let cases = [(0.0, 0.0), (-0.0, 0.0), (1.0, 0.0), (-3.5, 0.0), (inf, 2.0), (-inf, 2.0), (inf, inf), (2.0, f64::NAN), (f64::from_bits(0x7ff0_0000_0000_0001), 2.0)];
    let got: Vec<u64> = cases.iter().map(|c| fmod_x(std::hint::black_box(c.0), std::hint::black_box(c.1)).to_bits()).collect();
    assert_eq!(format!("{got:016x?}"), format!("{EXPECT:016x?}"));
}
