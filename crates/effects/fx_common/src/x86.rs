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

/// `sqrtss`: correctly rounded square root (NaN for negative inputs).
#[inline]
pub fn sqrtss(x: f32) -> f32 {
    x.sqrt()
}

/// Fused multiply add `a * b + c` with one rounding. Every vfmadd/vfmsub/vfnmadd form is built from this by the caller,
/// with the operand pairing read from the listing.
#[inline]
pub fn fma(a: f32, b: f32, c: f32) -> f32 {
    a.mul_add(b, c)
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
