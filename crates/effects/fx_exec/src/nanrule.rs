//! The NaN of an invalid float operation on every processor.
//!
//! An invalid operation (`inf - inf`, `0 * inf`, `0 / 0`, the square root of a negative number) gives the default NaN of
//! the processor: `0xffc00000` on x86, the processor the references were rendered on, and `0x7fc00000` on ARM. A NaN
//! operand is propagated (quieted) by both. So a kernel output that holds `0x7fc00000` while none of its operands holds
//! that pattern comes from an invalid operation on ARM; it is set to the x86 default NaN, which makes the bytes the same
//! on every processor. On x86 the pass is compiled out, so x86 output is unchanged bit for bit.

/// The x86 default NaN in single precision.
pub const DEFAULT_NAN: u32 = 0xffc0_0000;
/// The ARM default NaN in single precision.
pub const ARM_DEFAULT_NAN: u32 = 0x7fc0_0000;

/// Whether this processor's default NaN differs from x86's.
pub const ACTIVE: bool = !cfg!(any(target_arch = "x86", target_arch = "x86_64"));

/// Kernels whose output the rewrite must not touch. `abs`, `negate` (the operand with its sign bit cleared or flipped) and
/// `diff` (the magnitude of a difference) give the same NaN on x86 and ARM, because the sign bit is forced or the operand NaN
/// is propagated: `abs`/`negate` of the x86 default NaN legitimately give `0x7fc00000`. `sharpen` combines its blurred plane
/// inside the kernel through `fx_common::nanop`, so its output NaN words already hold the same bits on every processor.
///
/// `hue_shift`, `hue_shift_preserve_luma`, `sat_scale`, `comp_premult`, `rotate`, `perspective` and `perspective_alpha` negate a
/// possibly fresh NaN inside the kernel. They do their arithmetic through `fx_common::x86::nx_*`, which gives the x86 NaN on every
/// processor, so a `0x7fc00000` in their output is the x86 result (a negated default NaN) and must not be rewritten.
///
/// `edge_aware_sharpen` computes each operation whose operands can both be NaN through `fx_common::nanop` and each other operation that can make a fresh NaN through `fx_common::x86::nx_*`, so a `0x7fc00000` in its output is the x86 result and must not be rewritten.
pub const NO_OUTPUT_REWRITE: [&str; 12] = [
    "abs", "negate", "diff", "sharpen", "hue_shift", "hue_shift_preserve_luma", "sat_scale", "comp_premult", "rotate", "perspective", "perspective_alpha",
    "edge_aware_sharpen",
];

/// `r` with the x86 default NaN for an invalid operation; `operand_has_arm_nan` tells whether an operand holds the ARM
/// default NaN pattern (then a NaN result may be that operand propagated and is left alone).
#[inline(always)]
pub fn canon(r: f32, operand_has_arm_nan: bool) -> f32 {
    if ACTIVE && !operand_has_arm_nan && r.to_bits() == ARM_DEFAULT_NAN {
        f32::from_bits(DEFAULT_NAN)
    } else {
        r
    }
}

/// The same rewrite for planes held as bit patterns (the ripmap of make_mipmap).
pub fn canon_bit_planes(planes: &mut [Vec<u32>], operand_has_arm_nan: bool) {
    if !ACTIVE || operand_has_arm_nan {
        return;
    }
    for p in planes.iter_mut() {
        for x in p.iter_mut() {
            if *x == ARM_DEFAULT_NAN {
                *x = DEFAULT_NAN;
            }
        }
    }
}

/// Whether any of the words holds the ARM default NaN pattern.
pub fn has_arm_nan(words: &[f32]) -> bool {
    words.iter().any(|x| x.to_bits() == ARM_DEFAULT_NAN)
}

/// Apply [`canon`] to every output plane of a kernel, given whether any operand holds the ARM default NaN pattern.
pub fn canon_planes(planes: &mut [Vec<f32>], operand_has_arm_nan: bool) {
    if !ACTIVE || operand_has_arm_nan {
        return;
    }
    for p in planes.iter_mut() {
        for x in p.iter_mut() {
            *x = canon(*x, false);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn op(f: fn(f32, f32) -> f32, a: f32, b: f32) -> u32 {
        canon(std::hint::black_box(f)(std::hint::black_box(a), std::hint::black_box(b)), false).to_bits()
    }

    #[test]
    fn every_invalid_operation_gives_the_x86_default_nan() {
        let inf = f32::INFINITY;
        assert_eq!(op(|a, b| a + b, inf, -inf), DEFAULT_NAN, "inf + -inf");
        assert_eq!(op(|a, b| a - b, inf, inf), DEFAULT_NAN, "inf - inf");
        assert_eq!(op(|a, b| a * b, 0.0, inf), DEFAULT_NAN, "0 * inf");
        assert_eq!(op(|a, b| a / b, 0.0, 0.0), DEFAULT_NAN, "0 / 0");
        assert_eq!(op(|a, b| a / b, inf, inf), DEFAULT_NAN, "inf / inf");
        assert_eq!(op(|a, _| a.sqrt(), -1.0, 0.0), DEFAULT_NAN, "sqrt(-1)");
        assert_eq!(op(|a, b| a.mul_add(b, 1.0), 0.0, inf), DEFAULT_NAN, "fma(0, inf, 1)");
        // a NaN operand is propagated, not replaced
        let q = f32::from_bits(0x7fc0_1234);
        assert_eq!(op(|a, b| a + b, q, 1.0), 0x7fc0_1234);
        assert_eq!(canon(f32::from_bits(ARM_DEFAULT_NAN), true).to_bits(), ARM_DEFAULT_NAN);
        assert_eq!(canon(1.5, false), 1.5);
    }

    #[test]
    fn sign_bit_kernels_are_exempt() {
        for c in ["abs", "negate", "diff", "sharpen", "hue_shift", "hue_shift_preserve_luma", "sat_scale", "comp_premult", "rotate", "perspective", "perspective_alpha", "edge_aware_sharpen"] {
            assert!(NO_OUTPUT_REWRITE.contains(&c));
        }
        assert!(!NO_OUTPUT_REWRITE.contains(&"+"));
    }
}
