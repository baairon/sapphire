// T1: hand-evaluated cases, edge shapes, NaN propagation, determinism and error paths for callback 0x46b9d0.
// Values are powers of two where a result is written out by hand (c = 4 gives d = 1 / (4 + 4) = 0.125 and c*d = 0.5 exactly).
use optical_flow_solve::smooth::*;

fn geo(w: i32, h: i32, st: i32, c: f32) -> Geo {
    Geo { width: w, height: h, stride: st, relax_scale: 4.0, smooth_weight: 1.0, c }
}

fn run(g: &Geo, s: Option<&[f32]>, a: &[f32], p: &mut [f32], q: &mut [f32], start: i32, count: u32) {
    let (d_src, mut p_second, mut q_second) = (a.to_vec(), p.to_vec(), q.to_vec());
    let mut one = Pass { src: a, p, q };
    let mut two = Pass { src: &d_src, p: &mut p_second, q: &mut q_second };
    smooth_normalise(g, s, &mut one, &mut two, start, count);
}

#[test]
fn damping_cases() {
    assert_eq!(damping(&geo(1, 1, 1, 4.0)), 0.125);
    assert_eq!(damping(&geo(1, 1, 1, 0.0)), 0.25); // f6c / f68
    assert_eq!(damping(&geo(1, 1, 1, -3.0)), 0.25);
    assert_eq!(damping(&geo(1, 1, 1, f32::NAN)), 0.25); // unordered goes to the else arm
    assert!(damping(&Geo { width: 1, height: 1, stride: 1, relax_scale: 0.0, smooth_weight: 0.0, c: 0.0 }).is_nan());
}

#[test]
fn one_by_one_unscaled_below_threshold() {
    // last row and last column: p' = p - p*cd, q' = q - q*cd with cd = 0.5; length^2 = 0.0625 + 0.25 <= 1
    let (a, mut p, mut q) = ([0.0f32], [0.5f32], [1.0f32]);
    run(&geo(1, 1, 1, 4.0), None, &a, &mut p, &mut q, 0, 1);
    assert_eq!((p[0], q[0]), (0.25, 0.5));
}

#[test]
fn one_by_one_normalised() {
    // (6, 8) halves to (3, 4), length 5 > 1, so the pair is divided by 5
    let (a, mut p, mut q) = ([0.0f32], [6.0f32], [8.0f32]);
    run(&geo(1, 1, 1, 4.0), None, &a, &mut p, &mut q, 0, 1);
    assert_eq!((p[0], q[0]), (3.0f32 / 5.0, 4.0f32 / 5.0));
}

#[test]
fn scaled_equals_unscaled_for_unit_scale_exact_values() {
    // 2x2 plane of small integers over powers of two: both fused orders are exact, so they agree
    let a = [1.0f32, 2.0, 3.0, 5.0];
    let s = [1.0f32; 4];
    let (mut p_first, mut q_first) = ([0.5f32, 0.25, 1.0, 0.125], [0.25f32, 0.5, 0.125, 1.0]);
    let (mut p_second, mut q_second) = (p_first, q_first);
    run(&geo(2, 2, 2, 4.0), Some(&s), &a, &mut p_first, &mut q_first, 0, 2);
    run(&geo(2, 2, 2, 4.0), None, &a, &mut p_second, &mut q_second, 0, 2);
    assert_eq!(p_first.map(f32::to_bits), p_second.map(f32::to_bits));
    assert_eq!(q_first.map(f32::to_bits), q_second.map(f32::to_bits));
}

#[test]
fn hand_two_by_one_row_first_column_scaled() {
    // width 2, height 2, row 0, column 0 with scale 1: dy = a[1][0]-a[0][0] = 1, dx = a[0][1]-a[0][0] = 2
    // q' = q + d*(s*dy - c*q) = 0.5 + 0.125*(1 - 4*0.5) = 0.375 ; p' = p + d*(s*dx - c*p) = 1 + 0.125*(2 - 4) = 0.75
    let a = [0.0f32, 2.0, 1.0, 0.0];
    let s = [1.0f32; 4];
    let (mut p, mut q) = ([1.0f32, 0.0, 0.0, 0.0], [0.5f32, 0.0, 0.0, 0.0]);
    run(&geo(2, 2, 2, 4.0), Some(&s), &a, &mut p, &mut q, 0, 1);
    assert_eq!((p[0], q[0]), (0.75, 0.375));
    assert_eq!(p[2].to_bits(), 0); // row 1 untouched
}

#[test]
fn empty_and_wrapping_ranges_do_nothing() {
    let a = [1.0f32; 4];
    for (start, count) in [(0, 0u32), (1, 0), (3, 0xffff_fffe)] {
        let (mut p, mut q) = ([0.5f32; 4], [0.25f32; 4]);
        run(&geo(2, 2, 2, 4.0), None, &a, &mut p, &mut q, start, count);
        assert_eq!((p, q), ([0.5f32; 4], [0.25f32; 4]), "{start} {count}");
    }
}

#[test]
fn zero_width_does_nothing() {
    let (a, mut p, mut q) = ([1.0f32; 4], [0.5f32; 4], [0.25f32; 4]);
    run(&geo(0, 2, 2, 4.0), None, &a, &mut p, &mut q, 0, 2);
    assert_eq!((p, q), ([0.5f32; 4], [0.25f32; 4]));
}

#[test]
fn nan_propagates_and_fresh_nan_is_canonical() {
    let (a, mut p, mut q) = ([0.0f32], [f32::NAN], [1.0f32]);
    run(&geo(1, 1, 1, 4.0), None, &a, &mut p, &mut q, 0, 1);
    assert!(p[0].is_nan());
    assert_eq!(q[0], 0.5); // NaN length fails the ordered test, q' is stored unnormalised
    // infinity in p: p - p*cd = inf - inf is a fresh NaN, 0xffc00000
    let (mut p, mut q) = ([f32::INFINITY], [1.0f32]);
    run(&geo(1, 1, 1, 4.0), None, &[0.0], &mut p, &mut q, 0, 1);
    assert_eq!(p[0].to_bits(), 0xffc0_0000);
    let g = Geo { width: 1, height: 1, stride: 1, relax_scale: 0.0, smooth_weight: 0.0, c: 0.0 };
    let (mut p, mut q) = ([1.0f32], [1.0f32]);
    run(&g, None, &[0.0], &mut p, &mut q, 0, 1);
    assert_eq!(p[0].to_bits(), 0xffc0_0000);
}

#[test]
fn deterministic() {
    let a: Vec<f32> = (0..35).map(|i| (i as f32) * 0.37 - 3.0).collect();
    let s: Vec<f32> = (0..35).map(|i| 1.0 + (i as f32) * 0.01).collect();
    let mk = || (a.clone(), (0..35).map(|i| i as f32 * 0.11).collect::<Vec<f32>>(), (0..35).map(|i| 2.0 - i as f32 * 0.07).collect::<Vec<f32>>());
    let (_, mut p_first, mut q_first) = mk();
    let (_, mut p_second, mut q_second) = mk();
    run(&geo(7, 5, 7, 1.3), Some(&s), &a, &mut p_first, &mut q_first, 0, 5);
    run(&geo(7, 5, 7, 1.3), Some(&s), &a, &mut p_second, &mut q_second, 0, 5);
    assert_eq!(p_first.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), p_second.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
    assert_eq!(q_first.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), q_second.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
}

#[test]
#[should_panic]
fn short_plane_panics() {
    let (a, mut p, mut q) = ([0.0f32; 2], [0.0f32; 2], [0.0f32; 2]);
    run(&geo(4, 2, 4, 4.0), None, &a, &mut p, &mut q, 0, 2);
}

#[test]
fn assumptions_listed() {
    assert!(assumptions().iter().any(|a| a.id == "CELL-ONE" && a.label == "MEASURED-EMULATOR-READ"));
}

#[test]
fn scale_plane_hand_cases() {
    // width 3, stride 4, rows 1..3 scaled by 0.5; padding column and row 0 untouched
    let mut x: Vec<f32> = (0..16).map(|i| i as f32).collect();
    scale_plane(&mut x, 3, 4, 0.5, 1, 2);
    let want: Vec<f32> = (0..16).map(|i| if (4..7).contains(&i) || (8..11).contains(&i) { i as f32 * 0.5 } else { i as f32 }).collect();
    assert_eq!(x, want);
}

#[test]
fn scale_plane_edges() {
    let mut x = vec![2.0f32];
    scale_plane(&mut x, 1, 1, 3.0, 0, 1);
    assert_eq!(x[0], 6.0);
    let mut y = vec![2.0f32, 4.0];
    scale_plane(&mut y, 2, 2, 3.0, 1, 0); // empty
    scale_plane(&mut y, 2, 2, 3.0, 1, 0xffff_ffff); // start + count wraps to 0: nothing
    scale_plane(&mut y, 0, 2, 3.0, 0, 1); // zero width
    assert_eq!(y, vec![2.0, 4.0]);
    let mut z = vec![f32::NAN, f32::INFINITY, 1.0];
    scale_plane(&mut z, 3, 3, 0.0, 0, 1);
    assert!(z[0].is_nan() && z[1].is_nan() && z[2] == 0.0);
    assert_eq!(z[1].to_bits(), 0xffc0_0000); // inf * 0 is a fresh NaN
}

#[test]
fn length_exactly_one_is_stored_unchanged() {
    // (2, 0) halves to (1, 0): n2 == 1.0 exactly, not above the threshold; dividing by sqrt(1) would give the same words,
    // so the strict and the non-strict test are indistinguishable (mutant m01 is equivalent)
    let (a, mut p, mut q) = ([0.0f32], [2.0f32], [0.0f32]);
    run(&geo(1, 1, 1, 4.0), None, &a, &mut p, &mut q, 0, 1);
    assert_eq!((p[0], q[0]), (1.0, 0.0));
}
