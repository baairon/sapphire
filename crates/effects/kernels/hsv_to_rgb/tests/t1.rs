use hsv_to_rgb::*;

fn b(x: f32) -> u32 { x.to_bits() }

#[test]
fn zero_saturation_copies_v_bits() {
    let q = f32::from_bits(0x7f800001);
    assert_eq!(pixel(0.3, 0.0, q).map(b), [0x7f800001; 3]);
    assert_eq!(pixel_const(0.3, -0.0, 0.25).map(b), [b(0.25); 3]);
}

#[test]
fn sextants_plane_form() {
    // h = k/6 + 1/12 puts t at k + 0.5; v = 1, s = 1: p = 1, q = 0.
    let want = [
        [1.0, 0.5, 0.0], // 0
        [0.5, 1.0, 0.0], // 1
        [0.0, 1.0, 0.5], // 2
        [0.0, 0.5, 1.0], // 3
        [0.5, 0.0, 1.0], // 4
        [1.0, 0.0, 0.5], // 5
    ];
    for k in 0..6 {
        let h = (k as f32 + 0.5) / 6.0;
        let r = pixel(h, 1.0, 1.0);
        for c in 0..3 {
            assert!((r[c] - want[k][c]).abs() < 1e-5, "k={k} c={c} {:?}", r);
        }
        let r_second = pixel_const(h, 1.0, 1.0);
        for c in 0..3 {
            assert!((r_second[c] - want[k][c]).abs() < 1e-5);
        }
    }
}

#[test]
fn hue_zero_is_red_and_one_wraps() {
    assert_eq!(pixel(0.0, 1.0, 1.0), [1.0, 0.0, 0.0]);
    assert_eq!(pixel(1.0, 1.0, 1.0), [1.0, 0.0, 0.0]);
    assert_eq!(pixel(-1.0, 1.0, 1.0), [1.0, 0.0, 0.0]);
    assert_eq!(pixel(2.0, 0.5, 0.8).map(b), pixel(0.0, 0.5, 0.8).map(b));
}

#[test]
fn sextant_boundaries() {
    for k in 1..6 {
        let h = k as f32 / 6.0;
        for hh in [f32::from_bits(h.to_bits() - 1), h, f32::from_bits(h.to_bits() + 1)] {
            let r = pixel(hh, 0.7, 0.9);
            assert!(r.iter().all(|x| x.is_finite() && *x >= 0.0 && *x <= 0.9 + 1e-6));
        }
    }
}

#[test]
fn negative_saturation_shifts_hue_by_half() {
    // s < 0: |s| with hue + 0.5
    let a = pixel(0.1, -0.6, 0.8);
    let c = pixel(0.6, 0.6, 0.8);
    for i in 0..3 {
        assert!((a[i] - c[i]).abs() < 1e-5);
    }
    let a = pixel_const(0.1, -0.6, 0.8);
    for i in 0..3 {
        assert!((a[i] - c[i]).abs() < 1e-5);
    }
}

#[test]
fn nan_hue_goes_to_default_sextant() {
    // NaN hue: t = NaN, cvtt gives i32::MIN, case 2.
    let r = pixel(f32::NAN, 1.0, 1.0);
    assert_eq!(b(r[1]), b(1.0));
    assert!(r[0].is_finite() || r[0].is_nan());
    assert!(r[2].is_nan());
}

#[test]
fn nan_value_propagates() {
    let r = pixel(0.1, 0.5, f32::NAN);
    assert!(r.iter().all(|x| x.is_nan()));
}

#[test]
fn const_negative_nan_v_sign_differs_from_plane_form() {
    let n = f32::from_bits(0x7fc00001);
    let a = pixel(0.1, -0.5, n);
    let c = pixel_const(0.1, -0.5, n);
    assert!(a.iter().all(|x| x.is_nan()) && c.iter().all(|x| x.is_nan()));
}

#[test]
fn counts_and_lengths() {
    assert!(hsv_to_rgb_const(0.2, 0.5, 0.5, 0)[0].is_empty());
    assert!(hsv_to_rgb_const(0.2, 0.5, 0.5, -4)[1].is_empty());
    let o = hsv_to_rgb_const(0.2, 0.5, 0.5, 1);
    assert_eq!(o[0].len(), 1);
    assert_eq!(hsv_to_rgb(&[0.0], &[1.0, 1.0], &[1.0]), Err(HsvError::LengthMismatch));
    let o = hsv_to_rgb(&[0.0, 0.5], &[1.0, 1.0], &[1.0, 1.0]).unwrap();
    assert_eq!(o[0].len(), 2);
}

#[test]
fn determinism_and_assumptions() {
    let a = pixel(0.37, 0.61, 0.83);
    for _ in 0..4 {
        assert_eq!(pixel(0.37, 0.61, 0.83).map(b), a.map(b));
    }
}

#[test]
fn infinite_hue_is_nan_path() {
    let r = pixel(f32::INFINITY, 0.5, 0.5);
    assert!(r[2].is_nan());
}
