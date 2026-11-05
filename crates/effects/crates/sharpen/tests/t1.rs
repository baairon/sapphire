use sharpen::combine::{combine, combine_elem};
use sharpen::plan::*;
use std::path::Path;

fn img(name: &str, off: usize) -> [u8; 4] {
    let p = Path::new(effects_vecdir::path_str("wave_shake/lifter/images/image_unmapped/bytes")).join(name);
    let b = std::fs::read(&p).unwrap_or_else(|e| panic!("{}: {}", p.display(), e));
    [b[off], b[off + 1], b[off + 2], b[off + 3]]
}

#[test]
fn constants_match_captured_image() {
    if !effects_vecdir::have_all(&["wave_shake/lifter/images/image_unmapped/bytes"]) {
        return;
    }
    // VA 0x15ea70c = 0x15ea680 + 0x8c ; VA 0x15ea710 = + 0x90 ; VA 0x15eaf94 start of its own region
    assert_eq!(f32::from_le_bytes(img("r_15ea680.bin", 0x8c)), 1.0);
    assert_eq!(f32::from_le_bytes(img("r_15ea680.bin", 0x90)), 2.0);
    assert_eq!(f32::from_le_bytes(img("r_15eaf94.bin", 0)), RADIUS_SCALE);
    assert_eq!(combine_const(), (1.0, 2.0));
}
fn combine_const() -> (f32, f32) { (sharpen::combine::ONE, sharpen::combine::TWO) }

#[test]
fn combine_hand_examples() {
    assert_eq!(combine_elem(3.0, 1.0, 0.5), 4.0); // 3 + 0.5*2
    assert_eq!(combine_elem(3.0, 1.0, 1.0), 5.0); // unit path 2*3-1
    assert_eq!(combine_elem(3.0, 1.0, 2.0), 7.0);
    assert_eq!(combine_elem(3.0, 1.0, -1.0), 1.0);
    assert_eq!(combine_elem(3.0, 1.0, 0.0), 3.0); // job itself still runs for gain 0 (the wrapper copies instead)
    assert!(combine_elem(3.0, 1.0, f32::NAN).is_nan()); // NaN gain goes general
    assert!(combine_elem(f32::NAN, 1.0, 1.0).is_nan());
    assert!(combine_elem(f32::INFINITY, f32::INFINITY, 0.5).is_nan());
    assert_eq!(combine_elem(3e38, -3e38, 1.0), f32::INFINITY); // no clamp
}

#[test]
fn combine_is_fused_not_two_rounded() {
    // find inputs where fma(d,g,x) differs from x + round(d*g): proves the pairing is fused (vfmadd231ss)
    let mut r = fx_common::plane::Rng::new(7);
    let mut gen_diff = 0;
    let mut unit_diff = 0;
    for _ in 0..20000 {
        let x = r.next_f32();
        let b = r.next_f32();
        let g = r.next_f32() * 3.0;
        let fused = combine_elem(x, b, g);
        let two = x + (x - b) * g;
        if fused.to_bits() != two.to_bits() { gen_diff += 1; }
        let u = combine_elem(x, b, 1.0);
        let u2 = (x * 2.0) - b; // 2x is exact, so unit path fused equals unfused
        if u.to_bits() != u2.to_bits() { unit_diff += 1; }
    }
    assert!(gen_diff > 100, "fused vs two rounding differ on {}", gen_diff);
    assert_eq!(unit_diff, 0);
}

#[test]
fn combine_count_and_errors() {
    let x = [1.0f32, 2.0, 3.0, 4.0];
    let b = [0.0f32; 4];
    let mut o = [9.0f32; 4];
    combine(&x, &b, 0.5, &mut o, 0).unwrap();
    assert_eq!(o, [9.0; 4]);
    combine(&x, &b, 1.0, &mut o, 3).unwrap();
    assert_eq!(o, [2.0, 4.0, 6.0, 9.0]);
    assert!(combine(&x, &b, 1.0, &mut o, 5).is_err());
}

#[test]
fn radius_order_and_rounding() {
    assert_eq!(radius(1.0, 1920), 240.0);
    for &(a, w) in &[(0.3f32, 1001i32), (0.7, 16777217), (1e-3, 1920), (2.5, 3), (0.0, 640), (-1.0, 640)] {
        let want = (((RADIUS_SCALE * a) as f64) * ((w as f32) as f64)) as f32; // product exact in f64, one rounding
        assert_eq!(radius(a, w).to_bits(), want.to_bits(), "a={} w={}", a, w);
    }
    assert_eq!(16777217i32 as f32, 16777216.0); // cvtsi2ss rounds to nearest even
    assert!(radius(f32::NAN, 100).is_nan());
    assert_eq!(radius(f32::INFINITY, 100), f32::INFINITY);
    assert!(radius(f32::INFINITY, 0).is_nan()); // inf * 0
}

#[test]
fn gain_index_rules() {
    assert_eq!(gain_index(0, 3), 0);
    assert_eq!(gain_index(2, 3), 2);
    assert_eq!(gain_index(3, 3), 3);
    assert_eq!(gain_index(5, 3), 3);
    assert_eq!(gain_index(0, 0), 0);
}

#[test]
fn copy_rules() {
    use PlanePlan::*;
    assert_eq!(plane_plan(0.0, 1.0), Copy);
    assert_eq!(plane_plan(-2.0, 1.0), Copy);
    assert_eq!(plane_plan(f32::NEG_INFINITY, 1.0), Copy);
    assert_eq!(plane_plan(2.0, 0.0), Copy);
    assert_eq!(plane_plan(2.0, -0.0), Copy);
    assert!(matches!(plane_plan(f32::NAN, 1.0), Run { .. })); // NaN radius is not taken (comiss CF=1)
    assert!(matches!(plane_plan(2.0, f32::NAN), Run { .. })); // NaN gain continues (jp)
    assert_eq!(plane_plan(0.5, 0.25), Run { radius: 0.5, gain: 0.25 });
    assert_eq!(plane_plan(f32::NAN, 0.0), Copy); // gain == 0 still copies
}

#[test]
fn blur_step_constants() {
    assert_eq!((BLUR_EDGE_MIRROR, BLUR_PASSES, BLUR_SUBPIXEL, BLUR_HALF, BLUR_GAIN), (1, 3, 0, 0, 1.0));
}
