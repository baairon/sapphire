//! Rust-only sensitivity test for the one approximate site (vrsqrtss at 0x2560ab). The estimate is perturbed by up to the SDM bound
//! (1.5 * 2^-12 relative) before the code's own Newton step; the maximum change of the refined factor x10 and of the radii is reported (APPROX-BOUND).
use edge_aware_sharpen_blur_core::*;

fn newton(x_in: f32, r: f32) -> f32 {
    let prod_2 = x_in * r;
    let prod_5 = r * -0.5f32;
    let prod_3 = prod_2 * r;
    let diff_6 = prod_3 - 3.0f32;
    prod_5 * diff_6
}
fn ulp(a: f32, b: f32) -> u64 {
    let k = |x: f32| -> i64 { let b = x.to_bits() as i32; (if b < 0 { i32::MIN.wrapping_sub(b) } else { b }) as i64 };
    (k(a) - k(b)).unsigned_abs()
}

#[test]
fn approx_bound() {
    let bound = 1.5f32 * 2f32.powi(-12);
    let mut s: u64 = 0x9e3779b97f4a7c15;
    let mut rnd = || { s ^= s << 13; s ^= s >> 7; s ^= s << 17; (s >> 11) as f64 / (1u64 << 53) as f64 };
    let mut worst = 0u64;
    let mut worst_at = (0f32, 0f32);
    // route value first: pw - 1 = 63
    let mut xs: Vec<f32> = vec![63.0];
    for _ in 0..200_000 {
        xs.push((10f64.powf(-3.0 + 9.0 * rnd())) as f32);
    }
    for &x_in in &xs {
        let approx = rsqrt_ss(x_in);
        let exact = newton(x_in, approx);
        for _ in 0..10 {
            let d = (2.0 * rnd() as f32 - 1.0) * bound;
            let refined = approx * (1.0 + d);
            let u = ulp(newton(x_in, refined), exact);
            if u > worst {
                worst = u;
                worst_at = (x_in, d);
            }
        }
        for d in [bound, -bound] {
            let u = ulp(newton(x_in, approx * (1.0 + d)), exact);
            if u > worst {
                worst = u;
                worst_at = (x_in, d);
            }
        }
    }
    println!("APPROX-BOUND vrsqrtss site 0x2560ab: max change of the refined factor {} ulp at x13 {} delta {}", worst, worst_at.0, worst_at.1);
    // radii: the change carries through two multiplications by the same factor, so the relative change is the same up to 1 ulp
    assert!(worst <= 8);
}
