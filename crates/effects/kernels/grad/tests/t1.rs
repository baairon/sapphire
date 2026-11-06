// T1/T3: hand-evaluated examples from the listing (gradient callback 0x341c80, slot 52 arithmetic 0x53b467..0x53b48d), edge cases, determinism.
use grad::*;

fn base() -> GradParams {
    GradParams { amount: 0.0, gain_param: 1.0, subpixel: 0, edge_mode: 0, render_scale: 20, aspect: None, info_aspect_set: false, flag_1: 1, past_end: 0.0 }
}

fn run(p: &GradParams, src: &[f32], w: usize, h: usize) -> (Vec<f32>, Vec<f32>, Vec<f32>) {
    let n = w * h;
    let (mut aux, mut dx, mut dy) = (vec![f32::NAN; n], vec![f32::NAN; n], vec![f32::NAN; n]);
    render(p, src, w, h, &mut aux, &mut dx, &mut dy).unwrap();
    (aux, dx, dy)
}

const M3: [f32; 9] = [1.0, 2.0, 4.0, 8.0, 16.0, 32.0, 64.0, 128.0, 256.0];

#[test]
fn sigma_gain_values() {
    let mut p = base();
    p.amount = 8.0;
    p.render_scale = 2;
    p.gain_param = 1.0;
    let (s, g) = sigma_gain(&p);
    assert_eq!(s, 2.0);
    assert_eq!(g.to_bits(), (2.0f32 * 0.05f32).to_bits());
}

#[test]
fn copy_path_is_identity_at_gain_one() {
    // amount 0 gives sigma 0 (blur copy path); k = 20 and gain_param 1 give gain (20 * 0.05f) = 1.0f exactly
    let (aux, _, _) = run(&base(), &M3, 3, 3);
    assert_eq!(aux, M3.to_vec());
}

#[test]
fn hand_gradient_3x3_scale2() {
    // info_aspect clear: xmm5 = 2.0, xmm1 = 4.0. Middle row factor 0.5 * 2.0 = 1.0, first and last row factor 0.5 * 4.0 = 2.0.
    let (_, dx, dy) = run(&base(), &M3, 3, 3);
    assert_eq!(&dx[0..3], &[2.0, 3.0, 4.0 * 0.0 + 4.0]); // row 0: (2-1)*2, 4-1, (4-2)*2
    assert_eq!(&dx[3..6], &[16.0, 24.0, 32.0]);
    assert_eq!(&dx[6..9], &[128.0, 192.0, 256.0]);
    // row 0: up = own row, down = row 1: (1-8, 2-16, 4-32) = (-7, -14, -28); columns 0 and 2: ((v * 0.5) * 4.0); column 1: v * (0.5 * 4.0)
    assert_eq!(&dy[0..3], &[-14.0, -28.0, -56.0]);
    // row 1: up row 0 minus down row 2 = (-63, -126, -252), factor 1.0 (columns 0, 2: (v*0.5)*2.0)
    assert_eq!(&dy[3..6], &[-63.0, -126.0, -252.0]);
    // row 2: up = row 1, down = own row: (8-64, 16-128, 32-256) = (-56, -112, -224), x4 factor 2.0 in the middle and (v*0.5)*4.0 at the edges
    assert_eq!(&dy[6..9], &[-112.0, -224.0, -448.0]);
}

#[test]
fn hand_gradient_3x3_scale1() {
    // info_aspect set and cell 0x1addc04 nonzero: xmm5 = 1.0, xmm1 = 2.0: middle row factor 0.5, first and last row factor 1.0
    let mut p = base();
    p.info_aspect_set = true;
    let (_, dx, dy) = run(&p, &M3, 3, 3);
    assert_eq!(&dx[3..6], &[16.0, 24.0, 32.0]); // dx does not depend on xmm5
    assert_eq!(&dy[3..6], &[-31.5, -63.0, -126.0]);
    assert_eq!(&dy[0..3], &[-7.0, -14.0, -28.0]);
    // cell cleared: xmm5 = 2.0 whatever info_aspect says
    p.flag_1 = 0;
    let (_, _, dy0) = run(&p, &M3, 3, 3);
    assert_eq!(&dy0[3..6], &[-63.0, -126.0, -252.0]);
}

#[test]
fn width_two_and_one() {
    let p = base();
    let (_, dx, _) = run(&p, &[1.0, 5.0, 2.0, 11.0], 2, 2);
    assert_eq!(dx, vec![8.0, 8.0, 18.0, 18.0]); // (p1 - p0) * 2.0 in both columns
    // width 1: p[1] is the next row, the last row reads past_end
    let mut q = base();
    q.past_end = 100.0;
    let (_, dx1, _) = run(&q, &[1.0, 3.0, 7.0], 1, 3);
    assert_eq!(dx1, vec![4.0, 8.0, 186.0]);
}

#[test]
fn single_row_has_zero_vertical_gradient() {
    let (_, _, dy) = run(&base(), &[1.0, 2.0, 3.0, 4.0, 5.0], 5, 1);
    assert!(dy.iter().all(|v| *v == 0.0 && v.is_sign_positive()));
}

#[test]
fn odd_and_even_widths_agree_with_the_definition() {
    for w in 2..=12usize {
        let h = 4usize;
        let src: Vec<f32> = (0..w * h).map(|i| ((i * 7 + 3) % 13) as f32).collect();
        let (_, dx, _) = run(&base(), &src, w, h);
        for y in 0..h {
            let r = &src[y * w..(y + 1) * w];
            for x in 0..w {
                let want = if x == 0 { (r[1] - r[0]) * 2.0 } else if x == w - 1 { (r[w - 1] - r[w - 2]) * 2.0 } else { r[x + 1] - r[x - 1] };
                assert_eq!(dx[y * w + x], want, "w {w} y {y} x {x}");
            }
        }
    }
}

#[test]
fn denormal_order_is_half_then_scale_at_the_edge_columns() {
    // column 0 does (v * 0.5) * x3, interior does v * (0.5 * x3): an odd denormal difference separates them
    let mut src = vec![0.0f32; 9];
    src[0] = f32::from_bits(3); // up row (row 0) value at column 0, down row (row 2) is 0
    let (_, _, dy) = run(&base(), &src, 3, 3);
    // row 1: v = 3 denorm units, (3 * 0.5) rounds to even 2, times 2.0 = 4; the single-multiply form would give 3
    assert_eq!(dy[3].to_bits(), 4);
}

#[test]
fn determinism_and_nan_propagation() {
    let mut src = M3.to_vec();
    src[4] = f32::NAN;
    let a = run(&base(), &src, 3, 3);
    let b = run(&base(), &src, 3, 3);
    assert_eq!(a.1.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), b.1.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    assert!(a.1[3].is_nan() && a.1[5].is_nan() && !a.1[4 - 3].is_nan()); // NaN centre: dx of the middle row's edges and itself
}

#[test]
fn errors_and_empty() {
    let mut aux = vec![0.0f32; 4];
    let (mut dx, mut dy) = (vec![0.0f32; 4], vec![0.0f32; 4]);
    let mut p = base();
    assert!(render(&p, &[0.0; 3], 2, 2, &mut aux, &mut dx, &mut dy).is_err());
    p.edge_mode = 5;
    p.amount = 1.0;
    assert!(render(&p, &[0.0; 4], 2, 2, &mut aux, &mut dx, &mut dy).is_err());
    assert!(render(&base(), &[], 0, 5, &mut [], &mut [], &mut []).is_ok());
}

#[test]
fn blur_half_is_class_blur() {
    // amount > 0: aux is exactly blur_core with sx = sy = sigma, gain, 3 passes
    let mut p = base();
    p.amount = 2.0;
    p.render_scale = 2;
    p.edge_mode = 1;
    let src: Vec<f32> = (0..30).map(|i| ((i * 5) % 11) as f32 / 7.0).collect();
    let (aux, _, _) = run(&p, &src, 6, 5);
    let (s, g) = sigma_gain(&p);
    let mut want = vec![0.0f32; 30];
    blur::blur_core(&src, &mut want, 6, 5, s, s, g, blur::Edge::Mirror, 3, false, None).unwrap();
    assert_eq!(aux, want);
}

#[test]
fn assumptions_listed() {
    let a = assumptions();
    assert!(a.iter().any(|x| x.id == "GR-A1" && x.tier == "PARAM-UNCAPTURED"));
    assert!(a.len() >= 8);
}
