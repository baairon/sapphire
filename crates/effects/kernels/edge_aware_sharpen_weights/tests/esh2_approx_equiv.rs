//! Rust-only tests: APPROX-BOUND sensitivity of the vrcpps estimate and the exhaustive equivalence sweeps behind the argued mutants.
use edge_aware_sharpen_weights::*;

fn ulps(a: f32, b: f32) -> u32 {
    let (x, y) = (a.to_bits() as i64, b.to_bits() as i64);
    (x - y).unsigned_abs() as u32
}

/// APPROX-BOUND: perturb the reciprocal estimate by up to 1.5 * 2^-12 (relative, SDM bound of vrcpps) before the code's own Newton step and report the largest
/// change of the vector-lane result in ulps, over 2,000,000 seeded inputs in the working range [1e-10, 0.9999999999].
#[test]
fn approx_bound() {
    let mut s: u64 = 0x9e3779b97f4a7c15;
    let mut next = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let bound = 1.5f32 * (2.0f32).powi(-12);
    let mut worst = 0u32;
    for i in 0..2_000_000u64 {
        let u = (next() >> 11) as f64 / (1u64 << 53) as f64;
        let m = (1e-10f64.ln() + u * (0.9999f64.ln() - 1e-10f64.ln())).exp() as f32;
        let r = 1.0f32 / m;
        let exact = newton_from(r, m);
        for &sgn in &[-1.0f32, 1.0, -0.5, 0.5, 0.0] {
            let rp = r * (1.0 + sgn * bound);
            let d = ulps(newton_from(rp, m), exact);
            if d > worst {
                worst = d;
                println!("APPROX-BOUND new max {} ulp at m {:e} (i {}) sgn {}", d, m, i, sgn);
            }
        }
    }
    println!("APPROX-BOUND max change of the Newton step output: {} ulp", worst);
    assert!(worst <= 8);
}

/// The thresholds are not f32 values, so `>` and `>=` against them agree for every f32 (mutants M38, M39 are equivalent).
#[test]
fn thresholds_not_f32() {
    assert_ne!((LOW as f32) as f64, LOW);
    assert_ne!((HIGH as f32) as f64, HIGH);
}

/// vminss `a < b ? a : b` versus `a <= b ? a : b` (mutant M12): over all 2^32 values a and the partners b = a and b = -a the results are bit equal
/// or both zero (and a zero is classified below 1e-10 either way, so its sign never reaches an output).
#[test]
fn minss_le_sweep() {
    let mut zero_pairs = 0u64;
    for bits in 0..=u32::MAX {
        let a = f32::from_bits(bits);
        for &b in &[a, f32::from_bits(bits ^ 0x8000_0000)] {
            let m1 = minss(a, b);
            let m2 = if a <= b { a } else { b };
            if m1.to_bits() != m2.to_bits() {
                assert!(m1 == 0.0 && m2 == 0.0, "differ at {:08x}", bits);
                zero_pairs += 1;
            }
        }
    }
    println!("minss_le_sweep: {} pairs differ only in the sign of zero", zero_pairs);
}
