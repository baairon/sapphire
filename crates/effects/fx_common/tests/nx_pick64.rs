//! The double precision `nx_*` functions pick the NaN of x86 on every processor: the first operand of an addition, subtraction,
//! multiplication or division quieted, the second when the first is not a NaN, the default NaN when neither is.
use fx_common::x86::{nx_add64, nx_div64, nx_fma64, nx_mul64, nx_sub64};
use std::hint::black_box as bb;

fn nb(x: u64) -> f64 {
    f64::from_bits(x)
}

#[test]
fn double_precision_first_operand_nan_wins() {
    let (qa, qb, sa, sb) = (nb(0x7ff8_0000_0000_0001), nb(0x7ff8_0000_0000_0002), nb(0x7ff0_0000_0000_0001), nb(0x7ff0_0000_0000_0002));
    for f in [nx_add64, nx_sub64, nx_mul64, nx_div64] {
        assert_eq!(f(bb(qa), bb(qb)).to_bits(), 0x7ff8_0000_0000_0001);
        assert_eq!(f(bb(qb), bb(qa)).to_bits(), 0x7ff8_0000_0000_0002);
        assert_eq!(f(bb(qa), bb(sb)).to_bits(), 0x7ff8_0000_0000_0001);
        assert_eq!(f(bb(sa), bb(qb)).to_bits(), 0x7ff8_0000_0000_0001);
        assert_eq!(f(bb(sa), bb(1.0)).to_bits(), 0x7ff8_0000_0000_0001);
        assert_eq!(f(bb(1.0), bb(sb)).to_bits(), 0x7ff8_0000_0000_0002);
    }
    let inf = f64::INFINITY;
    assert_eq!(nx_add64(bb(inf), bb(-inf)).to_bits(), 0xfff8_0000_0000_0000);
    assert_eq!(nx_sub64(bb(inf), bb(inf)).to_bits(), 0xfff8_0000_0000_0000);
    assert_eq!(nx_mul64(bb(0.0), bb(inf)).to_bits(), 0xfff8_0000_0000_0000);
    assert_eq!(nx_div64(bb(0.0), bb(0.0)).to_bits(), 0xfff8_0000_0000_0000);
}

#[test]
fn double_precision_fma_priority_is_second_multiplicand_first_multiplicand_addend() {
    let (qa, qb, qc) = (nb(0x7ff8_0000_0000_0001), nb(0x7ff8_0000_0000_0002), nb(0x7ff8_0000_0000_0003));
    assert_eq!(nx_fma64(bb(qa), bb(qb), bb(qc)).to_bits(), 0x7ff8_0000_0000_0002);
    assert_eq!(nx_fma64(bb(qa), bb(1.0), bb(qc)).to_bits(), 0x7ff8_0000_0000_0001);
    assert_eq!(nx_fma64(bb(1.0), bb(qb), bb(qc)).to_bits(), 0x7ff8_0000_0000_0002);
    assert_eq!(nx_fma64(bb(1.0), bb(1.0), bb(qc)).to_bits(), 0x7ff8_0000_0000_0003);
    assert_eq!(nx_fma64(bb(nb(0x7ff0_0000_0000_0001)), bb(1.0), bb(qc)).to_bits(), 0x7ff8_0000_0000_0001);
}
