use tilt::*;

fn run(st: &State, x: &[f32], y: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let n = x.len();
    let (mut a, mut b) = (vec![9.0; n], vec![9.0; n]);
    render(st, x, y, &mut a, &mut b, n as i32, false).unwrap();
    (a, b)
}

fn wrapped() -> State {
    State { wrap: [1, 1], ..State::default() }
}

#[test]
fn identity_passes_through_with_wrap() {
    let (a, b) = run(&wrapped(), &[1.5, -2.0, 100.0], &[0.25, 7.0, -300.0]);
    assert_eq!(a, vec![1.5, -2.0, 100.0]);
    assert_eq!(b, vec![0.25, 7.0, -300.0]);
}

#[test]
fn shift_then_scale_hand_evaluated() {
    let st = State { shift: [1.0, -2.0], scale: [2.0, 0.5], ..wrapped() };
    let (a, b) = run(&st, &[3.0], &[10.0]);
    assert_eq!((a[0], b[0]), (8.0, 4.0)); // (3+1)*2, (10-2)*0.5
}

#[test]
fn shear_hand_evaluated() {
    let st = State { shear: [0.5, 0.25], ..wrapped() };
    let (a, b) = run(&st, &[4.0], &[8.0]);
    assert_eq!((a[0], b[0]), (8.0, 9.0)); // 8*0.5+4 and 4*0.25+8
}

#[test]
fn clamp_to_63_9_when_wrap_is_zero() {
    let st = State::default();
    let (a, b) = run(&st, &[100.0, -100.0, 5.0], &[1.0, 2.0, 70.0]);
    assert_eq!(a[0], 63.9f64 as f32);
    assert_eq!(a[1], -63.9f64 as f32);
    assert_eq!(a[2], 5.0);
    assert_eq!(b[2], 63.9f64 as f32);
    assert_eq!(b[0], 1.0);
}

#[test]
fn clamp_keeps_zero_signs_and_nan() {
    assert_eq!(clamp(-0.0).to_bits(), 0x8000_0000);
    assert_eq!(clamp(0.0).to_bits(), 0);
    assert!(clamp(f32::from_bits(0x7fa0_0001)).is_nan());
    assert_eq!(clamp(f32::from_bits(0x7fa0_0001)).to_bits(), 0x7fe0_0001); // signalling NaN quieted by the conversion
    assert_eq!(clamp(f32::INFINITY), 63.9f64 as f32);
    assert_eq!(clamp(f32::NEG_INFINITY), -63.9f64 as f32);
}

#[test]
fn rotate_small_angle_matches_sincos_native() {
    let st = State { rotate: 90.0, ..wrapped() };
    let (s, c) = sincos_native(90.0f32 * C_DEG);
    let (a, b) = run(&st, &[1.0], &[0.0]);
    assert_eq!(a[0].to_bits(), (1.0f32 * c - 0.0 * s).to_bits());
    assert_eq!(b[0].to_bits(), (0.0f32 * c + 1.0 * s).to_bits());
}

#[test]
fn nan_propagates_and_fresh_nan_is_canonical() {
    let nan = f32::from_bits(0xffc1_2345);
    let (a, _) = run(&wrapped(), &[nan], &[0.0]);
    assert_eq!(a[0].to_bits(), 0xffc1_2345);
    let st = State { scale: [f32::INFINITY, 1.0], ..wrapped() };
    let (a, _) = run(&st, &[0.0], &[0.0]);
    assert_eq!(a[0].to_bits(), 0xffc0_0000); // inf * 0
}

#[test]
fn odd_even_and_single_widths_are_deterministic() {
    let st = State { rotate: 0.25, shear: [0.1, 0.2], ..wrapped() };
    for n in [1usize, 2, 3, 8, 9] {
        let x: Vec<f32> = (0..n).map(|i| i as f32 * 0.37 - 1.0).collect();
        let y: Vec<f32> = (0..n).map(|i| 2.0 - i as f32 * 0.11).collect();
        assert_eq!(run(&st, &x, &y), run(&st, &x, &y));
    }
}

#[test]
fn error_paths() {
    let mut a = [0.0f32; 2];
    let mut b = [0.0f32; 2];
    let x = [1.0f32, 2.0];
    assert_eq!(render(&State::default(), &x, &x, &mut a, &mut b, 2, true), Err(Path::Flag));
    let p = State { swivel: 0.5, ..State::default() };
    assert_eq!(render(&State::default(), &x, &x, &mut a, &mut b, 3, false), Err(Path::ShortBuffers));
    assert_eq!(render(&p, &x, &x, &mut a, &mut b, 0, false), Ok(()));
    assert_eq!(render(&State::default(), &x, &x, &mut a, &mut b, -4, false), Ok(()));
}

#[test]
fn setup_flags_and_assumptions() {
    let s = setup(&State { swivel: f32::NAN, ..State::default() });
    assert!(s.persp_flag);
    let s = setup(&State { tilt: -0.0, ..State::default() });
    assert!(!s.persp_flag && !s.rot_flag && !s.shear_flag);
    assert_eq!(setup(&State { shear: [0.0, f32::NAN], ..State::default() }).shear_flag, true);
    assert_eq!(assumptions().len(), 6);
}

#[test]
fn perspective_basic_properties() {
    let st = State { swivel: 0.5, tilt: 0.25, perspective: 1.0, wrap: [1, 1], ..State::default() };
    let x = [0.0f32, 0.3, -0.7];
    let y = [0.0f32, -0.2, 0.9];
    let (mut a, mut b) = ([9.0f32; 3], [9.0f32; 3]);
    assert_eq!(render(&st, &x, &y, &mut a, &mut b, 3, false), Ok(()));
    let (mut first_a, mut first_b) = ([9.0f32; 3], [9.0f32; 3]);
    assert_eq!(render(&st, &x, &y, &mut first_a, &mut first_b, 3, false), Ok(()));
    assert_eq!(a.map(f32::to_bits), first_a.map(f32::to_bits));
    assert_eq!(b.map(f32::to_bits), first_b.map(f32::to_bits));
    // the origin maps to the origin (all products of zero, finite denominator)
    assert_eq!(a[0], 0.0);
    assert_eq!(b[0], 0.0);
    assert!(a[1].is_finite() && b[2].is_finite());
}

#[test]
fn perspective_n_one_and_nan() {
    let st = State { swivel: f32::NAN, tilt: 0.0, wrap: [1, 1], ..State::default() };
    let (mut a, mut b) = ([9.0f32; 2], [9.0f32; 2]);
    assert_eq!(render(&st, &[0.5, 0.5], &[0.5, 0.5], &mut a, &mut b, 1, false), Ok(()));
    assert!(a[0].is_nan() && b[0].is_nan());
    assert_eq!(a[1], 9.0); // n of 1 writes one element
    let st = State { swivel: 0.5, tilt: 0.25, wrap: [1, 1], ..State::default() };
    let (mut a, mut b) = ([9.0f32; 2], [9.0f32; 2]);
    assert_eq!(render(&st, &[0.5, 0.5], &[0.5, 0.5], &mut a, &mut b, 1, false), Ok(()));
    assert!(a[0].is_finite() && a[1] == 9.0 && b[1] == 9.0);
}

#[test]
fn perspective_divisor_zero() {
    // perspective 0 gives inv_persp = +inf; the run must finish and be deterministic
    let st = State { swivel: 0.5, tilt: 0.25, perspective: 0.0, wrap: [1, 1], ..State::default() };
    let (mut a, mut b) = ([0.0f32; 2], [0.0f32; 2]);
    assert_eq!(render(&st, &[0.5, 0.1], &[0.5, 0.2], &mut a, &mut b, 2, false), Ok(()));
    let (mut first_a, mut first_b) = ([0.0f32; 2], [0.0f32; 2]);
    assert_eq!(render(&st, &[0.5, 0.1], &[0.5, 0.2], &mut first_a, &mut first_b, 2, false), Ok(()));
    assert_eq!(a.map(f32::to_bits), first_a.map(f32::to_bits));
}
