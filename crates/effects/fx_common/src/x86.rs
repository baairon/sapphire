//! x86 helpers where silicon differs from plain Rust (S5). Everything else is plain Rust arithmetic.
//! `minss`/`maxss` use Intel operand order (first source a; NaN or equal returns b). Conversions return the
//! "integer indefinite" i32::MIN for NaN and out of range inputs. MXCSR (round to nearest even) is assumed.

/// `minss a, b` (dest = a): `if a < b { a } else { b }`. NaN in either operand, or equal operands (+0/-0), return b.
#[inline]
pub fn minss(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// `maxss a, b` (dest = a): `if a > b { a } else { b }`. NaN in either operand, or equal operands (+0/-0), return b.
#[inline]
pub fn maxss(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

/// `cvttss2si r32, xmm`: truncate toward zero; NaN or out of range gives i32::MIN.
#[inline]
pub fn cvttss2si32(x: f32) -> i32 {
    let t = x.trunc();
    if t >= -2147483648.0 && t < 2147483648.0 { t as i32 } else { i32::MIN }
}

/// `cvttsd2si r32, xmm`: truncate toward zero; NaN or out of range gives i32::MIN.
#[inline]
pub fn cvttsd2si32(x: f64) -> i32 {
    let t = x.trunc();
    if t >= -2147483648.0 && t < 2147483648.0 { t as i32 } else { i32::MIN }
}

/// `cvtss2si r32, xmm` under round-to-nearest-even: half cases go to the even integer; NaN or out of range gives i32::MIN.
#[inline]
pub fn cvtss2si32_rne(x: f32) -> i32 {
    let r = x.round_ties_even();
    if r >= -2147483648.0 && r < 2147483648.0 { r as i32 } else { i32::MIN }
}

/// `sqrtss`: correctly rounded square root (the default NaN 0xffc00000 for negative inputs, on every processor).
#[inline]
pub fn sqrtss(x: f32) -> f32 {
    nx_sqrt(x)
}

/// Fused multiply add `a * b + c` with one rounding. Every vfmadd/vfmsub/vfnmadd form is built from this by the caller,
/// with the operand pairing read from the listing.
#[inline]
pub fn fma(a: f32, b: f32, c: f32) -> f32 {
    nx_fma(a, b, c)
}

/// `andps` with the abs mask 0x7fffffff on one lane.
#[inline]
pub fn fabs_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fff_ffff)
}

/// `xorps` with the sign mask 0x80000000 on one lane.
#[inline]
pub fn flip_sign(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

/// `cmpltss`/`vcmpltss` lane mask: all ones when `a < b`, else 0 (false for NaN).
#[inline]
pub fn cmp_lt_mask(a: f32, b: f32) -> u32 {
    if a < b { u32::MAX } else { 0 }
}

/// `a <= b` lane mask (false for NaN).
#[inline]
pub fn cmp_le_mask(a: f32, b: f32) -> u32 {
    if a <= b { u32::MAX } else { 0 }
}

/// `vcmpgtss` (predicate 0x0e) lane mask: all ones when `a > b` (a is the first source), else 0 (false for NaN).
#[inline]
pub fn cmp_gt_mask(a: f32, b: f32) -> u32 {
    if a > b { u32::MAX } else { 0 }
}

/// `cmpeqss` lane mask (false for NaN).
#[inline]
pub fn cmp_eq_mask(a: f32, b: f32) -> u32 {
    if a == b { u32::MAX } else { 0 }
}

/// Bits of the default NaN of x86 SSE and AVX hardware (the result of an invalid operation).
pub const DEFAULT_NAN_BITS: u32 = 0xffc0_0000;

/// Arithmetic with the NaN results of x86 hardware on every processor.
///
/// An invalid operation (`inf - inf`, `0 * inf`, `0 / 0`, `sqrt(-1)`) gives the default NaN 0xffc00000 on x86 and 0x7fc00000 on
/// AArch64, and a sign flip or an absolute value applied afterwards then differs too. With two NaN operands x86 returns the first
/// source operand quieted for add, sub, mul and div, and the second multiplicand, then the first, then the addend for the fused
/// multiply add of the system math library. AArch64 picks a signalling NaN first and puts the addend first. The `nx_` functions
/// below are plain arithmetic on x86 and apply the x86 rule on any other processor, so a kernel written with them has one result
/// on every target. A result that is not a NaN is never touched.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
mod nx_rule {
    #[inline(always)]
    pub fn r2(r: f32, _a: f32, _b: f32) -> f32 {
        r
    }
    #[inline(always)]
    pub fn r3(r: f32, _a: f32, _b: f32, _c: f32) -> f32 {
        r
    }
    #[inline(always)]
    pub fn d2(r: f64, _a: f64, _b: f64) -> f64 {
        r
    }
    #[inline(always)]
    pub fn d3(r: f64, _a: f64, _b: f64, _c: f64) -> f64 {
        r
    }
}

#[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
mod nx_rule {
    #[cold]
    #[inline(never)]
    fn pick2(a: f32, b: f32) -> f32 {
        if a.is_nan() {
            f32::from_bits(a.to_bits() | 0x0040_0000)
        } else if b.is_nan() {
            f32::from_bits(b.to_bits() | 0x0040_0000)
        } else {
            f32::from_bits(super::DEFAULT_NAN_BITS)
        }
    }
    #[cold]
    #[inline(never)]
    fn pick3(a: f32, b: f32, c: f32) -> f32 {
        // priority of the system library's fused multiply add on x86: second multiplicand, first multiplicand, addend
        for v in [b, a, c] {
            if v.is_nan() {
                return f32::from_bits(v.to_bits() | 0x0040_0000);
            }
        }
        f32::from_bits(super::DEFAULT_NAN_BITS)
    }
    #[cold]
    #[inline(never)]
    fn pick2d(a: f64, b: f64) -> f64 {
        if a.is_nan() {
            f64::from_bits(a.to_bits() | 0x0008_0000_0000_0000)
        } else if b.is_nan() {
            f64::from_bits(b.to_bits() | 0x0008_0000_0000_0000)
        } else {
            f64::from_bits(0xfff8_0000_0000_0000)
        }
    }
    #[cold]
    #[inline(never)]
    fn pick3d(a: f64, b: f64, c: f64) -> f64 {
        for v in [b, a, c] {
            if v.is_nan() {
                return f64::from_bits(v.to_bits() | 0x0008_0000_0000_0000);
            }
        }
        f64::from_bits(0xfff8_0000_0000_0000)
    }
    #[inline(always)]
    pub fn r2(r: f32, a: f32, b: f32) -> f32 {
        if r != r { pick2(a, b) } else { r }
    }
    #[inline(always)]
    pub fn r3(r: f32, a: f32, b: f32, c: f32) -> f32 {
        if r != r { pick3(a, b, c) } else { r }
    }
    #[inline(always)]
    pub fn d2(r: f64, a: f64, b: f64) -> f64 {
        if r != r { pick2d(a, b) } else { r }
    }
    #[inline(always)]
    pub fn d3(r: f64, a: f64, b: f64, c: f64) -> f64 {
        if r != r { pick3d(a, b, c) } else { r }
    }
}

/// `a + b` with the NaN result of x86.
#[inline(always)]
pub fn nx_add(a: f32, b: f32) -> f32 {
    nx_rule::r2(a + b, a, b)
}
/// `a - b` with the NaN result of x86.
#[inline(always)]
pub fn nx_sub(a: f32, b: f32) -> f32 {
    nx_rule::r2(a - b, a, b)
}
/// `a * b` with the NaN result of x86.
#[inline(always)]
pub fn nx_mul(a: f32, b: f32) -> f32 {
    nx_rule::r2(a * b, a, b)
}
/// `a / b` with the NaN result of x86.
#[inline(always)]
pub fn nx_div(a: f32, b: f32) -> f32 {
    nx_rule::r2(a / b, a, b)
}
/// `a.mul_add(b, c)` with the NaN result of x86 (system library fused multiply add).
#[inline(always)]
pub fn nx_fma(a: f32, b: f32, c: f32) -> f32 {
    nx_rule::r3(a.mul_add(b, c), a, b, c)
}
/// `a + b` in double precision with the NaN result of x86.
#[inline(always)]
pub fn nx_add64(a: f64, b: f64) -> f64 {
    nx_rule::d2(a + b, a, b)
}
/// `a - b` in double precision with the NaN result of x86.
#[inline(always)]
pub fn nx_sub64(a: f64, b: f64) -> f64 {
    nx_rule::d2(a - b, a, b)
}
/// `a * b` in double precision with the NaN result of x86.
#[inline(always)]
pub fn nx_mul64(a: f64, b: f64) -> f64 {
    nx_rule::d2(a * b, a, b)
}
/// `a / b` in double precision with the NaN result of x86.
#[inline(always)]
pub fn nx_div64(a: f64, b: f64) -> f64 {
    nx_rule::d2(a / b, a, b)
}
/// Square root with the NaN result of x86 (the square root of a negative number is the default NaN).
#[inline(always)]
pub fn nx_sqrt(a: f32) -> f32 {
    nx_rule::r2(a.sqrt(), a, a)
}
/// `a.mul_add(b, c)` in double precision with the NaN result of x86.
#[inline(always)]
pub fn nx_fma64(a: f64, b: f64, c: f64) -> f64 {
    nx_rule::d3(a.mul_add(b, c), a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAN: f32 = f32::NAN;

    #[test]
    fn minss_nan_either_operand_returns_second() {
        assert_eq!(minss(NAN, 1.0), 1.0);
        assert!(minss(1.0, NAN).is_nan());
        assert!(minss(NAN, NAN).is_nan());
    }

    #[test]
    fn maxss_nan_either_operand_returns_second() {
        assert_eq!(maxss(NAN, 1.0), 1.0);
        assert!(maxss(1.0, NAN).is_nan());
        assert!(maxss(NAN, NAN).is_nan());
    }

    #[test]
    fn minss_maxss_zero_pairs_return_second() {
        assert_eq!(minss(0.0, -0.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(minss(-0.0, 0.0).to_bits(), 0.0f32.to_bits());
        assert_eq!(maxss(0.0, -0.0).to_bits(), (-0.0f32).to_bits());
        assert_eq!(maxss(-0.0, 0.0).to_bits(), 0.0f32.to_bits());
    }

    #[test]
    fn minss_maxss_ordinary() {
        assert_eq!(minss(1.0, 2.0), 1.0);
        assert_eq!(minss(2.0, 1.0), 1.0);
        assert_eq!(maxss(1.0, 2.0), 2.0);
        assert_eq!(maxss(2.0, 1.0), 2.0);
        assert_eq!(minss(f32::NEG_INFINITY, 0.0), f32::NEG_INFINITY);
        assert_eq!(maxss(f32::INFINITY, 0.0), f32::INFINITY);
    }

    #[test]
    fn cvtt_ss_edge_cases() {
        assert_eq!(cvttss2si32(NAN), i32::MIN);
        assert_eq!(cvttss2si32(f32::INFINITY), i32::MIN);
        assert_eq!(cvttss2si32(f32::NEG_INFINITY), i32::MIN);
        assert_eq!(cvttss2si32(2147483648.0), i32::MIN); // 2^31
        assert_eq!(cvttss2si32(-2147483648.0), i32::MIN); // exactly i32::MIN
        assert_eq!(cvttss2si32(-2147483904.0), i32::MIN); // next f32 below -2^31
        assert_eq!(cvttss2si32(2147483520.0), 2147483520); // largest f32 below 2^31
        assert_eq!(cvttss2si32(-1.9), -1);
        assert_eq!(cvttss2si32(1.9), 1);
        assert_eq!(cvttss2si32(-0.5), 0);
    }

    #[test]
    fn cvtt_sd_edge_cases() {
        assert_eq!(cvttsd2si32(f64::NAN), i32::MIN);
        assert_eq!(cvttsd2si32(f64::INFINITY), i32::MIN);
        assert_eq!(cvttsd2si32(f64::NEG_INFINITY), i32::MIN);
        assert_eq!(cvttsd2si32(2147483648.0), i32::MIN);
        assert_eq!(cvttsd2si32(2147483647.9), 2147483647);
        assert_eq!(cvttsd2si32(-2147483648.0), i32::MIN);
        assert_eq!(cvttsd2si32(-2147483649.0), i32::MIN); // -2^31 - 1
        assert_eq!(cvttsd2si32(-2147483648.9), i32::MIN); // truncates to -2^31
        assert_eq!(cvttsd2si32(-2147483647.9), -2147483647);
    }

    #[test]
    fn cvtss2si_rne_half_cases() {
        assert_eq!(cvtss2si32_rne(0.5), 0);
        assert_eq!(cvtss2si32_rne(1.5), 2);
        assert_eq!(cvtss2si32_rne(2.5), 2);
        assert_eq!(cvtss2si32_rne(3.5), 4);
        assert_eq!(cvtss2si32_rne(-0.5), 0);
        assert_eq!(cvtss2si32_rne(-1.5), -2);
        assert_eq!(cvtss2si32_rne(-2.5), -2);
        assert_eq!(cvtss2si32_rne(2.4999998), 2);
        assert_eq!(cvtss2si32_rne(2.5000002), 3);
        assert_eq!(cvtss2si32_rne(NAN), i32::MIN);
        assert_eq!(cvtss2si32_rne(f32::INFINITY), i32::MIN);
        assert_eq!(cvtss2si32_rne(2147483648.0), i32::MIN);
    }

    #[test]
    fn masks_and_bit_helpers() {
        assert_eq!(fabs_bits(-3.0), 3.0);
        assert_eq!(fabs_bits(f32::from_bits(0xffc0_0000)).to_bits(), 0x7fc0_0000);
        assert_eq!(flip_sign(1.0), -1.0);
        assert_eq!(flip_sign(0.0).to_bits(), 0x8000_0000);
        assert_eq!(cmp_lt_mask(1.0, 2.0), u32::MAX);
        assert_eq!(cmp_lt_mask(NAN, 2.0), 0);
        assert_eq!(cmp_gt_mask(2.0, NAN), 0);
        assert_eq!(cmp_gt_mask(2.0, 1.0), u32::MAX);
        assert_eq!(cmp_le_mask(1.0, 1.0), u32::MAX);
        assert_eq!(cmp_eq_mask(0.0, -0.0), u32::MAX);
    }

    fn nb(x: u32) -> f32 {
        f32::from_bits(x)
    }

    /// Operands go through `black_box` so the compiler cannot fold a NaN operation or choose the operand order from constants.
    fn bb(x: f32) -> f32 {
        std::hint::black_box(x)
    }

    #[test]
    fn nx_invalid_operations_give_the_x86_default_nan() {
        let inf = f32::INFINITY;
        assert_eq!(nx_add(bb(inf), bb(-inf)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(nx_sub(bb(inf), bb(inf)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(nx_mul(bb(0.0), bb(inf)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(nx_div(bb(0.0), bb(0.0)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(nx_div(bb(inf), bb(inf)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(nx_fma(bb(0.0), bb(inf), bb(1.0)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(nx_fma(bb(1.0), bb(inf), bb(-inf)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(nx_mul64(bb64(0.0), bb64(f64::INFINITY)).to_bits(), 0xfff8_0000_0000_0000);
        assert_eq!(nx_fma64(bb64(0.0), bb64(f64::INFINITY), bb64(1.0)).to_bits(), 0xfff8_0000_0000_0000);
        // a sign flip of the default NaN is the positive quiet NaN on every processor
        assert_eq!((-nx_sub(bb(inf), bb(inf))).to_bits(), 0x7fc0_0000);
    }

    fn bb64(x: f64) -> f64 {
        std::hint::black_box(x)
    }

    #[test]
    fn nx_first_operand_nan_wins_for_add_sub_mul_div() {
        let (qa, qb, sa, sb) = (nb(0x7fc0_0001), nb(0x7fc0_0002), nb(0x7f80_0001), nb(0x7f80_0002));
        for f in [nx_add, nx_sub, nx_mul, nx_div] {
            assert_eq!(f(bb(qa), bb(qb)).to_bits(), 0x7fc0_0001);
            assert_eq!(f(bb(qb), bb(qa)).to_bits(), 0x7fc0_0002);
            assert_eq!(f(bb(qa), bb(sb)).to_bits(), 0x7fc0_0001);
            assert_eq!(f(bb(sa), bb(qb)).to_bits(), 0x7fc0_0001);
            assert_eq!(f(bb(sa), bb(1.0)).to_bits(), 0x7fc0_0001);
            assert_eq!(f(bb(1.0), bb(sb)).to_bits(), 0x7fc0_0002);
        }
    }

    #[test]
    fn nx_fma_priority_is_second_multiplicand_first_multiplicand_addend() {
        let (qa, qb, qc, sa) = (nb(0x7fc0_0001), nb(0x7fc0_0002), nb(0x7fc0_0003), nb(0x7f80_0001));
        assert_eq!(nx_fma(bb(qa), bb(qb), bb(qc)).to_bits(), 0x7fc0_0002);
        assert_eq!(nx_fma(bb(qa), bb(1.0), bb(qc)).to_bits(), 0x7fc0_0001);
        assert_eq!(nx_fma(bb(1.0), bb(qb), bb(qc)).to_bits(), 0x7fc0_0002);
        assert_eq!(nx_fma(bb(1.0), bb(1.0), bb(qc)).to_bits(), 0x7fc0_0003);
        assert_eq!(nx_fma(bb(sa), bb(qb), bb(qc)).to_bits(), 0x7fc0_0002);
        assert_eq!(nx_fma(bb(f32::INFINITY), bb(0.0), bb(qc)).to_bits(), 0x7fc0_0003);
        assert_eq!(nx_fma64(bb64(f64::from_bits(0x7ff8_0000_0000_0001)), bb64(f64::from_bits(0x7ff8_0000_0000_0002)), bb64(1.0)).to_bits(), 0x7ff8_0000_0000_0002);
    }

    #[test]
    fn nx_results_that_are_not_nan_are_plain() {
        assert_eq!(nx_add(bb(1.5), bb(2.25)).to_bits(), 3.75f32.to_bits());
        assert_eq!(nx_sub(bb(-0.0), bb(0.0)).to_bits(), (-0.0f32).to_bits());
        assert_eq!(nx_mul(bb(f32::INFINITY), bb(-2.0)).to_bits(), f32::NEG_INFINITY.to_bits());
        assert_eq!(nx_div(bb(1.0), bb(0.0)).to_bits(), f32::INFINITY.to_bits());
        let a = 1.0f32 + 1.0 / 4096.0;
        assert_eq!(nx_fma(a, a, -1.0 - 1.0 / 2048.0), 1.0 / 16777216.0);
    }

    #[test]
    fn sqrtss_and_fma_helpers_give_the_x86_nan_words() {
        assert_eq!(sqrtss(bb(-1.0)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(sqrtss(bb(f32::NEG_INFINITY)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(sqrtss(bb(nb(0xffc0_0042))).to_bits(), 0xffc0_0042);
        assert_eq!(fma(bb(0.0), bb(f32::INFINITY), bb(1.0)).to_bits(), DEFAULT_NAN_BITS);
        assert_eq!(fma(bb(nb(0x7fc0_0001)), bb(nb(0x7fc0_0002)), bb(1.0)).to_bits(), 0x7fc0_0002);
    }

    #[test]
    fn fma_is_single_rounding() {
        // (1 + 2^-12)^2 = 1 + 2^-11 + 2^-24: the product is not exact in f32, the fused result keeps the 2^-24 term.
        let a = 1.0f32 + 1.0 / 4096.0;
        let fused = fma(a, a, -1.0 - 1.0 / 2048.0);
        assert_eq!(fused, 1.0 / 16777216.0);
        assert_eq!(sqrtss(4.0), 2.0);
        assert!(sqrtss(-1.0).is_nan());
    }
}
