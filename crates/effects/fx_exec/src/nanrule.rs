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
}
