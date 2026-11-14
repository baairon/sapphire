use xy_to_polar::*;

fn run(x: &[f32], y: &[f32], dir: f32, phase: usize) -> (Vec<f32>, Vec<f32>) {
    let mut a = vec![0.0; x.len()];
    let mut m = vec![0.0; x.len()];
    xy_to_polar(x, y, dir, &mut a, &mut m, phase).unwrap();
    (a, m)
}

#[test]
fn axes_and_quadrants() {
    let x = [1.0, 0.0, -1.0, 0.0, 2.0];
    let y = [0.0, 1.0, 0.0, -1.0, 2.0];
    let (a, m) = run(&x, &y, 0.0, 0);
    assert_eq!(a[0], 0.0);
    assert!((a[1] - 0.25).abs() < 1e-6);
    assert!((a[2] - 0.5).abs() < 1e-6);
    assert!((a[3] - 0.75).abs() < 1e-6);
    assert!((a[4] - 0.125).abs() < 1e-6);
    assert_eq!(m[0], 1.0);
    assert!((m[4] - 2.828427).abs() < 1e-5);
}

#[test]
fn width_one_and_odd_sizes() {
    for n in [1usize, 2, 7, 8, 9, 15, 1199, 1200, 1201, 2000] {
        let x: Vec<f32> = (0..n).map(|i| 1.0 + i as f32 * 0.25).collect();
        let y: Vec<f32> = (0..n).map(|i| 0.5 - i as f32 * 0.1).collect();
        let (a, m) = run(&x, &y, 0.1, 0);
        assert_eq!(a.len(), n);
        for i in 0..n {
            assert!(a[i] >= 0.0 && a[i] < 1.0, "{} {}", n, a[i]);
            let e = (x[i] * x[i] + y[i] * y[i]).sqrt();
            assert!((m[i] - e).abs() <= e * 1e-5);
        }
    }
}

#[test]
fn nan_and_inf_propagate() {
    let x = [f32::NAN, 1.0, f32::INFINITY, 0.0];
    let y = [1.0, f32::NAN, f32::INFINITY, 0.0];
    let (a, m) = run(&x, &y, 0.0, 0);
    assert!(a[0].is_nan() && a[1].is_nan() && m[0].is_nan() && m[1].is_nan());
    assert!((a[2] - 0.125).abs() < 1e-6);
    assert!(m[2].is_infinite());
    assert_eq!(a[3], 0.0);
    assert_eq!(m[3], 0.0);
}

#[test]
fn deterministic_and_partition() {
    assert_eq!(vector_body(7, 0), (0, 0));
    assert_eq!(vector_body(100, 0), (0, 96));
    assert_eq!(vector_body(1200, 0), (0, 1200));
    assert_eq!(vector_body(1200, 4), (7, 1199));
    assert_eq!(vector_body(1200, 2), (0, 0));
    let x: Vec<f32> = (0..64).map(|i| i as f32 - 20.0).collect();
    let y: Vec<f32> = (0..64).map(|i| 11.0 - i as f32 * 0.5).collect();
    assert_eq!(run(&x, &y, 0.3, 0), run(&x, &y, 0.3, 0));
}

#[test]
fn error_path_is_named() {
    let r = xy_to_polar(&[1e-40], &[2e-40], 0.0, &mut [0.0], &mut [0.0], 0);
    assert_eq!(r, Err(Refusal::Atan2General { index: 0 }));
}
