//! Sensitivity of the vector magnitude to the reciprocal square root estimate (APPROX-BOUND, Rust only, no hardware).
use xy_to_polar::*;

fn ulp(a: f32, b: f32) -> u64 {
    let k = |v: f32| {
        let b = v.to_bits() as i64;
        if b < 0 { (0x8000_0000i64 - (b & 0x7fff_ffff)) } else { b }
    };
    (k(a) - k(b)).unsigned_abs()
}

#[test]
fn approx_bound() {
    let rel = 1.5f32 * 2f32.powi(-12);
    let mut state = 0x1234_5678u64;
    let mut rand32 = || {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (state >> 33) as u32
    };
    let mut worst = 0u64;
    for _ in 0..200_000 {
        let e = (rand32() % 160) as i32 - 80;
        let x = (1.0 + (rand32() % (1u32 << 23)) as f32 / (1u32 << 23) as f32) * 2f32.powi(e) * if rand32() & 1 == 0 { 1.0 } else { -1.0 };
        let y = (1.0 + (rand32() % (1u32 << 23)) as f32 / (1u32 << 23) as f32) * 2f32.powi((rand32() % 160) as i32 - 80);
        let base = mag_vector_with(x, y, rsqrt_est);
        for k in [-1.0f32, -0.5, 0.5, 1.0] {
            let v = mag_vector_with(x, y, |s| rsqrt_est(s) * (1.0 + k * rel));
            if base.is_finite() && v.is_finite() {
                worst = worst.max(ulp(base, v));
            }
        }
    }
    println!("APPROX-BOUND (MEASURED on the port; no hardware): largest output change {} ulp over 200000 vectors and 4 perturbations of up to 1.5 * 2^-12", worst);
}
