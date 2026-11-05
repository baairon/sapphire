//! Plane comparison (S6): ulp distance, NaN compared as a class, and the `PlaneDiff` report.

/// Monotone integer key of an f32 (sign-magnitude folded), so that +0 and -0 share key 0 and the key is continuous across zero.
#[inline]
fn ord_key(v: f32) -> i64 {
    let b = v.to_bits() as i32 as i64;
    if b < 0 { i32::MIN as i64 - b } else { b }
}

/// Distance in representable f32 values between `a` and `b`. +0 and -0 are 0 apart; the distance is continuous across
/// zero; infinities count as ordinary ordered values (inf is 1 above f32::MAX). Both NaN gives 0 (class equality);
/// exactly one NaN gives u32::MAX.
pub fn ulp_diff(a: f32, b: f32) -> u32 {
    match (a.is_nan(), b.is_nan()) {
        (true, true) => 0,
        (true, false) | (false, true) => u32::MAX,
        _ => (ord_key(a) - ord_key(b)).unsigned_abs().min(u32::MAX as u64) as u32,
    }
}

/// Equal as bit patterns, or both NaN (any payload or sign).
#[inline]
pub fn eq_class(a: f32, b: f32) -> bool {
    a.to_bits() == b.to_bits() || (a.is_nan() && b.is_nan())
}

/// Result of comparing two planes of equal width and height.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlaneDiff {
    pub width: usize,
    pub height: usize,
    /// Samples compared (width * height).
    pub samples: usize,
    /// Samples that are not class-equal.
    pub differing_samples: usize,
    /// Bytes that differ, summed over the samples that are not class-equal (4 bytes per sample at most).
    pub differing_bytes: usize,
    /// Positions where both are NaN but the bit patterns differ (matches as a class, reported for information).
    pub nan_class_matches: usize,
    /// Positions where exactly one side is NaN.
    pub nan_mismatches: usize,
    /// Largest ulp distance among positions with at most one NaN side; u32::MAX when a NaN/non-NaN mismatch exists.
    pub max_ulp: u32,
    /// First differing position as (x, y, a_bits, b_bits).
    pub first: Option<(usize, usize, u32, u32)>,
}

impl PlaneDiff {
    /// Compare `a` and `b` over `width` x `height` samples; `stride_a` and `stride_b` are row pitches in f32 units.
    pub fn compare(a: &[f32], stride_a: usize, b: &[f32], stride_b: usize, width: usize, height: usize) -> PlaneDiff {
        let mut d = PlaneDiff { width, height, samples: width * height, ..PlaneDiff::default() };
        for y in 0..height {
            for x in 0..width {
                let (va, vb) = (a[y * stride_a + x], b[y * stride_b + x]);
                if va.to_bits() == vb.to_bits() {
                    continue;
                }
                if va.is_nan() && vb.is_nan() {
                    d.nan_class_matches += 1;
                    continue;
                }
                if va.is_nan() != vb.is_nan() {
                    d.nan_mismatches += 1;
                }
                d.differing_samples += 1;
                let x_or = va.to_bits() ^ vb.to_bits();
                d.differing_bytes += (0..4).filter(|k| (x_or >> (8 * k)) & 0xff != 0).count();
                d.max_ulp = d.max_ulp.max(ulp_diff(va, vb));
                if d.first.is_none() {
                    d.first = Some((x, y, va.to_bits(), vb.to_bits()));
                }
            }
        }
        d
    }

    /// PARITY bar of S8: no differing byte, NaN compared as a class.
    pub fn is_parity(&self) -> bool {
        self.differing_bytes == 0
    }

    /// One report line for logs and READMEs.
    pub fn summary(&self) -> String {
        format!(
            "{}x{}: differing bytes {} ({} samples), NaN-class matches {}, NaN mismatches {}, max ulp {}",
            self.width, self.height, self.differing_bytes, self.differing_samples, self.nan_class_matches, self.nan_mismatches, self.max_ulp
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ulp_across_zero() {
        let tiny = f32::from_bits(1);
        assert_eq!(ulp_diff(0.0, -0.0), 0);
        assert_eq!(ulp_diff(tiny, -tiny), 2);
        assert_eq!(ulp_diff(tiny, 0.0), 1);
        assert_eq!(ulp_diff(-tiny, -0.0), 1);
        assert_eq!(ulp_diff(1.0, f32::from_bits(1.0f32.to_bits() + 1)), 1);
        assert_eq!(ulp_diff(-1.0, f32::from_bits((-1.0f32).to_bits() + 1)), 1);
        assert_eq!(ulp_diff(f32::MAX, f32::INFINITY), 1);
    }

    #[test]
    fn ulp_nan_rules() {
        assert_eq!(ulp_diff(f32::NAN, f32::from_bits(0xffc0_0001)), 0);
        assert_eq!(ulp_diff(f32::NAN, 1.0), u32::MAX);
        assert_eq!(ulp_diff(1.0, f32::NAN), u32::MAX);
    }

    #[test]
    fn plane_diff_counts() {
        let a = [1.0f32, 2.0, f32::NAN, 0.0, 5.0, 6.0];
        let mut b = a;
        b[2] = f32::from_bits(0x7fc0_0001); // NaN class match
        b[3] = -0.0; // sign bit differs: one byte
        b[5] = f32::from_bits(6.0f32.to_bits() + 1); // low byte differs, 1 ulp
        let d = PlaneDiff::compare(&a, 3, &b, 3, 3, 2);
        assert_eq!(d.nan_class_matches, 1);
        assert_eq!(d.differing_samples, 2);
        assert_eq!(d.differing_bytes, 2);
        assert_eq!(d.max_ulp, 1);
        assert_eq!(d.first, Some((0, 1, 0.0f32.to_bits(), (-0.0f32).to_bits())));
        assert!(!d.is_parity());
        assert!(PlaneDiff::compare(&a, 3, &a, 3, 3, 2).is_parity());
    }

    #[test]
    fn plane_diff_with_different_strides_and_nan_mismatch() {
        let a = [1.0f32, 2.0, 99.0, 3.0, 4.0, 99.0];
        let b = [1.0f32, 2.0, 3.0, f32::NAN];
        let d = PlaneDiff::compare(&a, 3, &b, 2, 2, 2);
        assert_eq!(d.differing_samples, 1); // only (1,1): 4.0 vs NaN; the stride padding 99.0 is never compared
        assert_eq!(d.nan_mismatches, 1);
        assert_eq!(d.max_ulp, u32::MAX);
    }
}
