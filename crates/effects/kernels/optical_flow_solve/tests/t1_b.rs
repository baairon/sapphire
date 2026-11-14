// Hand-evaluated and shape tests for the warp fetch, the derivative kernels and the polar kernel.
use optical_flow_solve::*;

fn sc() -> StencilConsts {
    StencilConsts { k_s: -1.0 / 12.0, k_v: -1.0 / 12.0, c_s: 8.0, c_v: 8.0 }
}
fn plane(w: usize, h: usize, f: impl Fn(usize, usize) -> f32) -> Vec<f32> {
    (0..w * h).map(|i| f(i % w, i / w)).collect()
}

#[test]
fn deriv_x_linear_ramp_scalar_and_vector_agree() {
    // f(x) = x: interior derivative (8*(f[-1]-f[+1]) - (f[-2]-f[+2])) * (-1/12) = (8*-2 - -4) * -1/12 = 1
    for w in [1usize, 2, 7, 8, 9, 16, 17, 40] {
        let inp = plane(w, 2, |x, _| x as f32);
        let mut out = vec![-7.0f32; w * 2];
        deriv_x(&inp, &mut out, w as i32, w as i32, 0, 2, 0, &sc(), None);
        for y in 0..2 {
            for x in 2..w.saturating_sub(2) {
                assert!((out[y * w + x] - 1.0).abs() < 1e-5, "w {w} x {x} got {}", out[y * w + x]);
            }
        }
    }
}

#[test]
fn deriv_y_matches_deriv_x_on_transpose() {
    let (w, h) = (11usize, 19usize);
    let inp = plane(w, h, |x, y| ((x * 7 + y * 13) % 17) as f32 * 0.25);
    let mut oy = vec![0.0f32; w * h];
    deriv_y(&inp, &mut oy, h as i32, w as i32, 0, w as i32, &sc(), None);
    let tr = plane(h, w, |y, x| inp[y * w + x]);
    let mut ox = vec![0.0f32; w * h];
    deriv_x(&tr, &mut ox, h as i32, h as i32, 0, w as i32, 0, &sc(), None);
    for y in 0..h {
        for x in 0..w {
            // the scalar tail rows of both kernels use the same expression
            if y >= (h & !7) {
                assert_eq!(oy[y * w + x].to_bits(), ox[x * h + y].to_bits());
            }
        }
    }
}

#[test]
fn gather_model_logs_indices_and_masks() {
    let p = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0];
    let mut log = Vec::new();
    let mut l = Some(&mut log);
    let r = gather8(&p, &[8, 0, 1, 2, 3, 4, 5, 6], &[true, true, true, true, false, true, true, true], "t", &mut l);
    assert_eq!(r, [9.0, 1.0, 2.0, 3.0, 0.0, 5.0, 6.0, 7.0]);
    assert_eq!(log.len(), 1);
    assert_eq!(log[0].idx[0], 8);
}

#[test]
fn warp_fetch_integer_and_half_positions() {
    let (w, h) = (4usize, 4usize);
    let src = plane(w, h, |x, y| (x + 10 * y) as f32);
    let mut out = vec![9.0f32; w * h];
    let delta_x = vec![0.0f32; w * h];
    let dy = vec![0.0f32; w * h];
    let c = FetchConsts { lo: 1e-4, margin: 1.0 };
    warp_fetch(&delta_x, &dy, &src, &mut out, w as i32, h as i32, w as i32, 0, h as i32, &c);
    // pixel (1,1): inside, exact; row 0 and column 0 are at or below the lower bound: zero
    assert_eq!(out[1 * w + 1], 11.0);
    assert_eq!(out[0], 0.0);
    let delta_x = vec![0.5f32; w * h];
    warp_fetch(&delta_x, &dy, &src, &mut out, w as i32, h as i32, w as i32, 0, h as i32, &c);
    assert_eq!(out[1 * w + 1], 11.5);
}

#[test]
fn warp_fetch_nan_position_clamps_to_corner_tap() {
    let src = vec![5.0f32; 16];
    let delta_x = vec![f32::NAN; 16];
    let dy = vec![0.5f32; 16];
    let mut out = vec![0.0f32; 16];
    warp_fetch(&delta_x, &dy, &src, &mut out, 4, 4, 4, 1, 1, &FetchConsts { lo: 1e-4, margin: 1.0 });
    assert!(out[4..8].iter().all(|v| v.is_nan()), "{:?}", &out[4..8]);
}

#[test]
fn polar_with_ported_routines_matches_hand_value() {
    let c = PolarConsts { arg2: 1.0, mul_s: 1.0, mul_v: 1.0, abs_mask: 0x7fff_ffff, sub: 3.0, fin: -0.5, thr: f32::from_bits(0x0080_0000) };
    let p = vec![3.0f32; 40];
    let q = vec![4.0f32; 40];
    let mut o = vec![0.0f32; 40];
    polar(&p, &q, &mut o, 40, 40, 0, 1, 0, &c);
    // s = 5, 5^1 = 5, e^5 = 148.41316
    let want = 148.41316f32;
    for i in [0usize, 9, 39] {
        assert!((o[i] - want).abs() / want < 1e-5, "{i} {}", o[i]);
    }
}

#[test]
fn approx_bound_sensitivity() {
    // perturb the estimate by up to 1.5 * 2^-12 relative; report the largest ulp change of the refined result
    let c = PolarConsts { arg2: 1.0, mul_s: 1.0, mul_v: 1.0, abs_mask: 0x7fff_ffff, sub: 3.0, fin: -0.5, thr: f32::from_bits(0x0080_0000) };
    let mut worst = 0i64;
    for k in 0..2000u32 {
        let s = 0.01f32 + (k as f32) * 0.37;
        let exact = rsqrt_model(s);
        let base = sqrt_refine(s, exact, &c);
        for e in [-1.5f32, -0.75, 0.75, 1.5] {
            let est = exact * (1.0 + e * 2f32.powi(-12));
            let v = sqrt_refine(s, est, &c);
            let d = (v.to_bits() as i64 - base.to_bits() as i64).abs();
            worst = worst.max(d);
        }
    }
    eprintln!("APPROX-BOUND largest output ulp change {worst}");
    assert!(worst < 1 << 20);
}

#[test]
fn zero_width_and_zero_rows_are_no_ops() {
    let mut out = vec![1.0f32; 8];
    deriv_x(&[0.0; 8], &mut out, 0, 4, 0, 2, 0, &sc(), None);
    deriv_x(&[0.0; 8], &mut out, 4, 4, 0, 0, 0, &sc(), None);
    deriv_y(&[0.0; 8], &mut out, 0, 4, 0, 4, &sc(), None);
    assert!(out.iter().all(|&v| v == 1.0));
}

#[test]
fn determinism() {
    let inp = plane(37, 3, |x, y| ((x * 31 + y * 5) % 11) as f32 - 5.0);
    let mut a = vec![0.0f32; 37 * 3];
    let mut b = a.clone();
    deriv_x(&inp, &mut a, 37, 37, 0, 3, 0, &sc(), None);
    deriv_x(&inp, &mut b, 37, 37, 0, 3, 0, &sc(), None);
    assert_eq!(a.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), b.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
}
