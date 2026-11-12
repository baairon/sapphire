use tilt_alpha::*;

fn st(sw: f32, tl: f32, pa: f32) -> State {
    State { swivel: sw, tilt: tl, amount: pa }
}
const H: Host = Host { w: 1000, d: 2.0, h: 500 };

fn run(s: &State, n: usize, plane_a: &[f32], plane_b: &[f32], flag: bool, ph: usize) -> (Vec<f32>, Outcome) {
    let mut d = vec![f32::from_bits(0xdeadbeef); n + 4];
    let o = tilt_alpha(s, &H, flag, n as i32, plane_a, plane_b, &mut d, ph).unwrap();
    (d, o)
}

#[test]
fn zero_tilt_zero_swivel_is_a_constant_ramp() {
    // angle 0: sin_ang = 0, cos_ang = 1; q = 1 / 1e-5 = 100000; knt = ksw = 0 (swivel 0, -0 tilt); t = fma(-0, 0, -1)... value 0 -> cos term
    let p = vec![0.5f32; 9];
    let (d, o) = run(&st(0.0, 0.0, 1.0), 9, &p, &p, false, 0);
    assert_eq!(o.route, Route::Ramp);
    assert!(d[..9].iter().all(|v| *v == d[0]));
    assert_eq!(d[9].to_bits(), 0xdeadbeef);
}

#[test]
fn width_one_and_empty() {
    let (d, o) = run(&st(10.0, 5.0, 1.0), 1, &[0.25], &[0.5], false, 0);
    assert_eq!(o.route, Route::Ramp);
    assert!(!d[0].is_nan());
    let mut e = vec![7.0f32; 2];
    let o = tilt_alpha(&st(1.0, 1.0, 1.0), &H, false, 0, &[], &[], &mut e, 0).unwrap();
    assert_eq!(o.route, Route::Empty);
    assert_eq!(e, vec![7.0, 7.0]);
}

#[test]
fn flag_and_host_routes_do_not_write() {
    let p = vec![1.0f32; 4];
    let (d, o) = run(&st(10.0, 5.0, 1.0), 4, &p, &p, true, 0);
    assert_eq!(o.route, Route::Flag);
    assert!(d.iter().all(|v| v.to_bits() == 0xdeadbeef));
    let (d, o) = run(&st(10.0, 5.0, 1e6), 4, &p, &p, false, 0);
    assert_eq!(o.route, Route::HostBranch);
    assert!(d.iter().all(|v| v.to_bits() == 0xdeadbeef));
}

#[test]
fn plan_sections() {
    assert_eq!(plan(0, 0), None);
    assert_eq!(plan(7, 0), Some(Plan { peel: 0, vend: 0 }));
    assert_eq!(plan(8, 12), Some(Plan { peel: 0, vend: 8 }));
    assert_eq!(plan(1199, 4), Some(Plan { peel: 0, vend: 1192 }));
    assert_eq!(plan(1200, 0), Some(Plan { peel: 0, vend: 1200 }));
    assert_eq!(plan(1203, 4), Some(Plan { peel: 7, vend: 1203 - ((1203 - 7) & 7) }));
    assert_eq!(plan(1300, 2), Some(Plan { peel: 0, vend: 0 }));
}

#[test]
fn nan_state_propagates_and_is_deterministic() {
    let p = vec![0.5f32; 16];
    let a = run(&st(f32::NAN, 1.0, 1.0), 16, &p, &p, false, 0).0;
    let b = run(&st(f32::NAN, 1.0, 1.0), 16, &p, &p, false, 0).0;
    assert_eq!(a.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), b.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    assert!(a[..16].iter().all(|v| v.is_nan()));
}

#[test]
fn error_path_short_buffer() {
    let mut d = vec![0.0f32; 2];
    let r = tilt_alpha(&st(1.0, 1.0, 1.0), &H, false, 5, &[0.0; 5], &[0.0; 5], &mut d, 0);
    assert_eq!(r, Err(Error::ShortBuffer));
}

#[test]
fn assumptions_listed() {
    assert!(assumptions().len() >= 5);
}
