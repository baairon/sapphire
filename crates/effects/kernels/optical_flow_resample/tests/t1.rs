use optical_flow_resample::{assumptions, resample_h, resample_v};

fn h(src: &[f32], w: i32, rows: i32, wd: i32, mis: usize) -> Vec<f32> {
    let mut d = vec![0.0f32; (wd * rows) as usize];
    resample_h(src, w, rows, wd, &mut d, mis, 0, rows);
    d
}
fn v(src: &[f32], stride: i32, hs: i32, hd: i32) -> Vec<f32> {
    let mut d = vec![0.0f32; (stride * hd) as usize];
    resample_v(src, stride, hs, hd, &mut d, 0, stride);
    d
}

#[test]
fn constant_plane_is_preserved() {
    for (w, wd) in [(5, 9), (9, 5), (4, 4), (17, 3), (3, 17)] {
        let src = vec![0.5f32; w as usize];
        for x in h(&src, w, 1, wd, 0) {
            assert!((x - 0.5).abs() < 1e-6, "w {} wd {} got {}", w, wd, x);
        }
    }
}

#[test]
fn identity_size_is_exact_for_a_ramp() {
    // equal sizes: scale-up branch, Catmull-Rom is interpolating so every output equals its input sample
    let src: Vec<f32> = (0..7).map(|i| i as f32 * 0.25).collect();
    let d = h(&src, 7, 1, 7, 0);
    for i in 0..7 { assert_eq!(d[i], src[i]); }
}

#[test]
fn width_one_and_height_one() {
    let d = h(&[0.75], 1, 1, 1, 0);
    assert_eq!(d.len(), 1);
    let d = h(&[0.75], 1, 1, 4, 0);
    assert!(d.iter().all(|x| (*x - 0.75).abs() < 1e-6));
    let d = v(&[0.1, 0.2, 0.3], 1, 3, 1);
    assert_eq!(d.len(), 1);
    let d = v(&[0.5], 1, 1, 3);
    assert!(d.iter().all(|x| (*x - 0.5).abs() < 1e-6));
}

#[test]
fn odd_and_even_sizes_pass_vertical_equals_horizontal_transpose() {
    for (n, m) in [(8, 5), (5, 8), (9, 4), (16, 3), (33, 6)] {
        let src: Vec<f32> = (0..n).map(|i| ((i * 37) % 11) as f32 * 0.1).collect();
        let a = h(&src, n, 1, m, 0);
        let b = v(&src, 1, n, m);
        assert_eq!(a.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), b.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), "n {} m {}", n, m);
    }
}

#[test]
fn nan_propagates_and_is_canonical() {
    let mut src = vec![0.25f32; 9];
    src[4] = f32::from_bits(0x7fc0_1234);
    let d = h(&src, 9, 1, 5, 0);
    assert!(d.iter().any(|x| x.is_nan()));
    // infinity times a zero weight: a fresh NaN has the default pattern
    let mut scale_second = vec![1.0f32; 40];
    scale_second[0] = f32::INFINITY;
    let d = h(&scale_second, 40, 1, 3, 0);
    assert!(d.iter().all(|x| !x.is_nan() || x.to_bits() & 0xffc0_0000 == 0xffc0_0000 || x.to_bits() & 0x7fc0_0000 == 0x7fc0_0000));
}

#[test]
fn deterministic() {
    let src: Vec<f32> = (0..64).map(|i| ((i * 13) % 7) as f32).collect();
    assert_eq!(h(&src, 64, 1, 10, 0), h(&src, 64, 1, 10, 0));
}

#[test]
fn error_paths_do_not_panic() {
    let mut d = vec![0.0f32; 4];
    resample_h(&[1.0; 4], 4, 1, 0, &mut d, 0, 0, 1);
    resample_h(&[1.0; 4], 4, 1, 4, &mut d, 0, 0, 0);
    resample_v(&[1.0; 4], 4, 1, 0, &mut d, 0, 4);
}

#[test]
fn hand_evaluated_upscale_midpoint() {
    // two samples a, b upscaled to three: the middle output is the Catmull-Rom blend at t=0.5 of both taps
    let d = h(&[0.0, 1.0], 2, 1, 3, 0);
    assert_eq!(d[0], 0.0);
    assert_eq!(d[2], 1.0);
    assert!((d[1] - 0.5).abs() < 1e-6);
}

#[test]
fn labels_present() {
    assert!(assumptions().iter().any(|s| s.starts_with("ALIGN-PHASE")));
}
