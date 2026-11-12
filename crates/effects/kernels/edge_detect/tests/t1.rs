// Hand-evaluated cases of the body and of slot 52 / slot 57 arithmetic, edge shapes, NaN propagation, determinism, error paths.
use edge_detect::*;

fn p0() -> EdgeParams {
    EdgeParams { amount: 0.0, gain_param: 1.0, subpixel: 0, edge_mode: 0, render_scale: 20, aspect: None, info3c: false, flag_cell: 1, past_end: 0.0 }
}
fn body(c: f32) -> Body {
    Body { c, k: 2.0, past_end: 0.0 }
}
fn run_body(bd: &Body, b: &[f32], w: usize, h: usize) -> Vec<f32> {
    let mut out = vec![f32::from_bits(0x7fc0ffee); w * h];
    edge_rows(bd, b, w, h, 0, h, &mut out);
    out
}

#[test]
fn flat_plane_is_zero() {
    let o = run_body(&body(1.0), &[3.0; 12], 4, 3);
    assert!(o.iter().all(|v| *v == 0.0));
}

#[test]
fn horizontal_ramp_hand_values() {
    // rows are 0,1,2,3: interior gx = 2, ends k * 1 = 2, gy = 0 -> 2 everywhere
    let b: Vec<f32> = (0..3).flat_map(|_| [0.0f32, 1.0, 2.0, 3.0]).collect();
    let o = run_body(&body(1.0), &b, 4, 3);
    assert!(o.iter().all(|v| *v == 2.0), "{:?}", o);
}

#[test]
fn vertical_ramp_row_factor() {
    // columns constant, rows 0,1,2: gx = 0; middle row gy = (0 - 2) * c = -2 -> 2; first row (0 - 1) * 2c = -2 -> 2; last row (1 - 2) * 2c -> 2
    let b = [0.0f32, 0.0, 1.0, 1.0, 2.0, 2.0];
    let o = run_body(&body(1.0), &b, 2, 3);
    assert_eq!(o, vec![2.0; 6]);
    let o = run_body(&body(0.5), &b, 2, 3);
    assert_eq!(o, vec![1.0; 6]); // c = 0.5: middle row 1.0, ends 2c = 1.0
}

#[test]
fn width_two_uses_first_pixel_gx_twice() {
    let o = run_body(&body(1.0), &[1.0, 4.0], 2, 1);
    // gx = 2 * 3 = 6, gy = 0 (up = down = own row)
    assert_eq!(o, vec![6.0, 6.0]);
}

#[test]
fn width_one_reads_past_end() {
    let mut bd = body(1.0);
    bd.past_end = 5.0;
    let o = run_body(&bd, &[1.0], 1, 1);
    assert_eq!(o, vec![8.0]); // gx = 2 * (5 - 1)
    bd.past_end = 1.0;
    assert_eq!(run_body(&bd, &[1.0], 1, 1), vec![0.0]);
    // two rows: the first row's p1 is the second row's pixel
    let o = run_body(&body(1.0), &[1.0, 3.0], 1, 2);
    // row 0: gx = 2 * (3 - 1) = 4, gy = (1 - 3) * 2 = -4 -> sqrt(32)
    assert_eq!(o[0], 32.0f32.sqrt());
}

#[test]
fn height_one_has_zero_gy() {
    let o = run_body(&body(1.0), &[0.0, 1.0, 3.0, 7.0], 4, 1);
    assert_eq!(o, vec![2.0, 3.0, 6.0, 8.0]);
}

#[test]
fn partition_rows_only() {
    let b: Vec<f32> = (0..12).map(|i| (i * i) as f32).collect();
    let all = run_body(&body(1.0), &b, 4, 3);
    let mut out = vec![f32::from_bits(0x7fc0ffee); 12];
    edge_rows(&body(1.0), &b, 4, 3, 1, 1, &mut out);
    assert_eq!(&out[4..8], &all[4..8]);
    assert_eq!(out[0].to_bits(), 0x7fc0ffee);
    assert_eq!(out[8].to_bits(), 0x7fc0ffee);
}

#[test]
fn nan_propagates_and_fresh_nan_is_canonical() {
    let b = [f32::NAN, 1.0, 2.0, 3.0];
    let o = run_body(&body(1.0), &b, 4, 1);
    assert!(o[0].is_nan() && o[1].is_nan());
    assert_eq!(o[3], 2.0);
    // inf - inf is a fresh NaN with the default sign
    let b = [f32::INFINITY, f32::INFINITY, 0.0, 0.0];
    let o = run_body(&body(1.0), &b, 2, 2);
    assert_eq!(o[0].to_bits(), 0xffc0_0000);
}

#[test]
fn deterministic() {
    let b: Vec<f32> = (0..64).map(|i| ((i * 7919) % 101) as f32 * 0.37).collect();
    assert_eq!(run_body(&body(1.0), &b, 8, 8), run_body(&body(1.0), &b, 8, 8));
}

#[test]
fn flag_selects_half_factor() {
    let mut p = p0();
    assert_eq!(body_for(&p).c, 1.0);
    p.info3c = true;
    assert_eq!(body_for(&p).c, 0.5);
    p.flag_cell = 0;
    assert_eq!(body_for(&p).c, 1.0);
}

#[test]
fn sigma_gain_values() {
    let mut p = p0();
    p.amount = 8.0;
    p.render_scale = 2;
    let (s, g) = sigma_gain(&p);
    assert_eq!(s, 2.0);
    assert_eq!(g.to_bits(), (0.05f32 * 2.0).to_bits());
}

#[test]
fn render_copy_path_and_errors() {
    let p = p0(); // amount 0: the blur is a copy at gain 20 * 0.05 = 1
    let src: Vec<f32> = (0..12).map(|i| i as f32).collect();
    let (mut aux, mut dst) = (vec![0.0; 12], vec![0.0; 12]);
    render(&p, &src, 4, 3, &mut aux, &mut dst).unwrap();
    assert_eq!(aux, src);
    let mut q = p0();
    q.edge_mode = 2;
    assert!(render(&q, &src, 4, 3, &mut aux, &mut dst).is_err());
    assert!(render(&p, &src[..5], 4, 3, &mut aux, &mut dst).is_err());
    assert!(render_rows(&p, &src, 4, 3, &mut aux, &mut dst, 2, 2).is_err());
    assert!(render(&p, &[], 0, 0, &mut [], &mut []).is_ok());
}

#[test]
fn margins_values() {
    let mut p = p0();
    p.amount = 4.0;
    p.render_scale = 2; // v = 1.0
    assert_eq!(margins(&p), (2, 2));
    p.amount = 4.4;
    assert_eq!(margins(&p), (3, 3)); // ceil(1.1) + 1
    p.aspect = Some(3.0);
    assert_eq!(margins(&p), (3, 5)); // second: ceil(3.3) + 1
    p.amount = f32::NAN;
    assert_eq!(margins(&p), (i32::MIN.wrapping_add(1), i32::MIN.wrapping_add(1)));
}

#[test]
fn assumptions_listed() {
    assert!(assumptions().len() >= 5);
}
