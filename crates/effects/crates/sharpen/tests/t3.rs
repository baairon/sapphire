use sharpen::combine::combine;
use sharpen::plan::*;
use fx_common::plane::Rng;

fn fnv(h: &mut u64, v: u32) { for b in v.to_le_bytes() { *h ^= b as u64; *h = h.wrapping_mul(0x100000001b3); } }

#[test]
fn determinism_and_finite() {
    let mut r = Rng::new(42);
    let n = 257;
    let x: Vec<f32> = (0..n).map(|_| r.next_f32() * 10.0 - 5.0).collect();
    let b: Vec<f32> = (0..n).map(|_| r.next_f32() * 10.0 - 5.0).collect();
    let mut hs = [0u64; 2];
    for pass in 0..2 {
        let mut h = 0xcbf29ce484222325u64;
        for &g in &[0.0f32, 0.5, 1.0, 2.0, -1.0, 100.0] {
            let mut o = vec![0.0f32; n];
            combine(&x, &b, g, &mut o, n).unwrap();
            assert!(o.iter().all(|v| v.is_finite()), "finite in gives finite out, g={}", g);
            for v in &o { fnv(&mut h, v.to_bits()); }
        }
        hs[pass] = h;
    }
    assert_eq!(hs[0], hs[1]);
    // plan grid determinism
    let mut h = 0u64;
    let mut cnt = 0;
    for &a in &[-1.0f32, 0.0, 1e-3, 0.3, 1.0, 5.0, f32::NAN] {
        for &w in &[0i32, 1, 7, 640, 1920] {
            for &g in &[-1.0f32, 0.0, 0.5, 1.0, 2.0, f32::NAN] {
                let p = plane_plan(radius(a, w), g);
                cnt += 1;
                fnv(&mut h, match p { PlanePlan::Copy => 0, PlanePlan::Run { radius, gain } => radius.to_bits() ^ gain.to_bits().rotate_left(7) | 1 });
            }
        }
    }
    assert_eq!(cnt, 210);
    println!("plan grid fnv {:016x}", h);
}

#[test]
fn radius_le_zero_is_copy() {
    for &a in &[0.0f32, -0.0, -3.0, -1e-30] {
        for &w in &[1i32, 640, 1920] {
            let r = radius(a, w);
            assert_eq!(plane_plan(r, 1.0), PlanePlan::Copy, "a={} w={}", a, w);
        }
    }
    assert_eq!(plane_plan(radius(1.0, 0), 1.0), PlanePlan::Copy); // width 0 gives radius 0
}
