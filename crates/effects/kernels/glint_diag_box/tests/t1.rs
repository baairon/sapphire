use glint_diag_box::*;

fn f(v: &str) -> f32 {
    v.parse::<f32>().unwrap()
}

fn go(p: &Params, band: Band, src: &[f32], w: usize, h: usize) -> Vec<f32> {
    let mut dst = vec![-1.0f32; w * h + 8];
    run(p, &RowPhases([0, 0, 0]), band, src, &mut dst, w, h).unwrap();
    dst.truncate(w * h);
    dst
}

#[test]
fn normaliser_values() {
    assert_eq!(normaliser([0, 0, 0]), 1.0);
    assert_eq!(normaliser([1, 0, 0]), (1.0f64 / 3.0) as f32);
    assert_eq!(normaliser([1, 2, 3]), (1.0f64 / 105.0) as f32);
    // negative radii still count: 2r+1 = -1 for r = -1
    assert_eq!(normaliser([-1, 0, 0]), -1.0);
}

#[test]
fn vector_route_predicate() {
    assert!(!vector_route(63, 0));
    assert!(vector_route(64, 0));
    assert!(!vector_route(64, 1));
    assert!(!vector_route(100, 2));
    assert!(!vector_route(70, 4)); // p = 7, needs n >= 71
    assert!(vector_route(71, 4));
    assert!(!vector_route(64, 28)); // p = 1
    assert!(vector_route(65, 28));
}

#[test]
fn shift_stage_branches() {
    let x = [1.0f32, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mut o = [9.0f32; 6];
    shift_blend(&x, &mut o, 6, 2); // L > 2D: left, overlap, right
    assert_eq!(o, [1.5, 2.0, 3.0, 4.0, 1.5, 2.0]);
    shift_blend(&x, &mut o, 6, 3); // L == 2D: no overlap
    assert_eq!(o, [2.0, 2.5, 3.0, 0.5, 1.0, 1.5]);
    shift_blend(&x, &mut o, 6, 4); // D < L < 2D: zero gap
    assert_eq!(o, [2.5, 3.0, 0.0, 0.0, 0.5, 1.0]);
    shift_blend(&x, &mut o, 6, 6); // L <= D: all zero
    assert_eq!(o, [0.0; 6]);
    assert!(o.iter().all(|v| v.to_bits() == 0));
}

#[test]
fn box_pass_radius_one_by_hand() {
    let x = [1.0f32, 2.0, 3.0, 4.0, 5.0];
    let mut out = [0.0f32; 5];
    pass_body_for_test(&x, &mut out, 1);
    assert_eq!(out, [3.0, 6.0, 9.0, 12.0, 9.0]);
    let mut out_one = [0.0f32; 1];
    pass_body_for_test(&[7.5], &mut out_one, 0);
    assert_eq!(out_one, [7.5]);
}

fn pass_body_for_test(x: &[f32], out: &mut [f32], r: usize) {
    // the public route: window sum entry point, then the pass through run's helpers via a one-row call is not possible, so use the sum + public pass
    let s = window_sum(x, r as i32 + 1, 0, 0).unwrap();
    glint_diag_box::box_pass(x, out, x.len(), r, s);
}

#[test]
fn middle_forms_differ_and_are_kept() {
    // r = 1, l = 6: middle steps k = 0 (form A), 1 (form B), 2 (odd remainder, form B). All-form-B would give 0.20000001788 at out[2].
    let x = [f("2.999999892949745e-08"), f("-0.30000001192092896"), 2.0, -1.5, f("5.000000058430487e-08"), f("-2.999999892949745e-08")];
    let mut out = [0.0f32; 6];
    pass_body_for_test(&x, &mut out, 1);
    assert_eq!(out[0].to_bits(), f("-0.29999998211860657").to_bits());
    assert_eq!(out[1].to_bits(), f("1.7000000476837158").to_bits());
    assert_eq!(out[2].to_bits(), f("0.20000004768371582").to_bits());
    assert_ne!(out[2].to_bits(), f("0.20000001788139343").to_bits());
    assert_eq!(out[3].to_bits(), f("0.5000001192092896").to_bits());
}

#[test]
fn whole_call_by_hand_w4_h2() {
    let src: Vec<f32> = (1..=8).map(|v| v as f32).collect();
    let band = Band { start: 0, count: 1 };
    // radii 0, d = 0: A = [1,4,7,0], B = [0,2,5,8], f = 1
    let p = Params { radii: [0, 0, 0], d: 0 };
    assert_eq!(go(&p, band, &src, 4, 2), vec![1.0, 2.0, 0.0, 0.0, 0.0, 0.0, 7.0, 8.0]);
    // radius 1 once: A' = [5,12,11,0] (bottom edge zero), B' = [0,7,15,13] (first word zero), f = 1/3
    let p = Params { radii: [1, 0, 0], d: 0 };
    let fm = (1.0f64 / 3.0) as f32;
    let want = vec![fm * 5.0, fm * 7.0, 0.0, 0.0, 0.0, 0.0, fm * 11.0, fm * 13.0];
    assert_eq!(go(&p, band, &src, 4, 2), want);
}

#[test]
fn lengths_and_widths_smoke() {
    for (w, h) in [(1usize, 1usize), (2, 1), (2, 2), (3, 2), (4, 2), (5, 3), (6, 3), (7, 4)] {
        let src: Vec<f32> = (0..w * h).map(|i| i as f32 + 1.0).collect();
        let n = ((w as i32 - 1) >> 1) + h as i32 - 2;
        let p = Params { radii: [1, 2, 0], d: 1 };
        let a = go(&p, Band { start: 0, count: n }, &src, w, h);
        let b = go(&p, Band { start: 0, count: n }, &src, w, h);
        assert_eq!(a, b);
    }
}

#[test]
fn nan_propagates_and_is_quiet() {
    let src = vec![f32::from_bits(0x7fa0_0001); 4 * 3];
    let p = Params { radii: [1, 0, 0], d: 0 };
    let out = go(&p, Band { start: 0, count: 2 }, &src, 4, 3);
    assert!(out.iter().any(|v| v.is_nan() && v.to_bits() & (1u32 << 22) != 0));
}

#[test]
fn error_paths() {
    let src = vec![1.0f32; 9];
    let mut dst = vec![0.0f32; 9];
    let rp = RowPhases([0, 0, 0]);
    let p = Params { radii: [1, 0, 0], d: 0 };
    assert_eq!(run(&p, &rp, Band { start: 0, count: 1 }, &src, &mut dst, 0, 3), Err(Error::Size));
    assert_eq!(run(&p, &rp, Band { start: 0, count: 1 }, &src[..4], &mut dst, 3, 3), Err(Error::Size));
    let pm = Params { radii: [1, 0, 0], d: i32::MIN };
    assert_eq!(run(&pm, &rp, Band { start: 0, count: 1 }, &src, &mut dst, 3, 3), Err(Error::ShiftMin));
    // W = 1: the epilogue writes past the plane
    let mut tight = vec![0.0f32; 1];
    assert!(matches!(run(&p, &rp, Band { start: 0, count: 0 }, &[1.0], &mut tight, 1, 1), Err(Error::WriteRange(_))));
    // vector route: radius 63 on a slice of length >= 127 with row phase 0 runs (no error)
    let (w, h) = (130usize, 70usize);
    let big: Vec<f32> = (0..w * h).map(|i| (i % 7) as f32).collect();
    let mut dist_c = vec![0.0f32; w * h + 8];
    let pv = Params { radii: [63, 0, 0], d: 0 };
    let n = ((w as i32 - 1) >> 1) + h as i32 - 2;
    run(&pv, &rp, Band { start: 0, count: n }, &big, &mut dist_c, w, h).unwrap();
    // the same sizes with an unaligned phase stay on the scalar route
    let rp1 = RowPhases([1, 1, 1]);
    run(&pv, &rp1, Band { start: 0, count: n }, &big, &mut dist_c, w, h).unwrap();
}

#[test]
fn assumptions_listed() {
    let a = assumptions();
    assert!(a.iter().any(|s| s.starts_with("SCRATCH-ALIGNMENT-AS-ARGUMENT")));
    assert!(a.iter().any(|s| s.starts_with("BAND-AS-ARGUMENT")));
}

fn thresholds() -> Vec<(usize, u32)> {
    let mut v = Vec::new();
    for ph in [0u32, 4, 8, 16, 28] {
        let p = if ph == 0 { 0 } else { ((32 - ph) >> 2) as usize };
        for rem in [0usize, 1, 7, 8, 9, 15, 16, 63] {
            v.push((p + 64 + rem, ph));
        }
        v.push((p + 64, ph));
        v.push((p + 65, ph));
    }
    v.push((64, 0));
    v.push((127, 0));
    v
}

#[test]
fn vector_sum_counts_every_element_once_at_each_threshold() {
    // integer-valued inputs: every order gives the exact sum, so a wrong index range shows
    for (n, ph) in thresholds() {
        assert!(vector_route(n as i32, ph), "n {n} ph {ph}");
        let x: Vec<f32> = (0..n + 8).map(|i| (i as f32) + 1.0).collect();
        let want: f32 = (1..=n).map(|i| i as f32).sum();
        assert_eq!(window_sum_vector(&x, n, ph), want, "n {n} ph {ph}");
        // elements past n must not be read
        let mut y = x.clone();
        for v in y[n..].iter_mut() {
            *v = f32::NAN;
        }
        assert_eq!(window_sum_vector(&y, n, ph), want, "n {n} ph {ph} tail");
    }
}

#[test]
fn vector_order_differs_from_scalar_order() {
    // x0 = 1, every other element 2^-24: the scalar running sum stays 1.0 (1 + 2^-24 rounds to 1);
    // the vector route sums the small elements in lane partials first, then adds them to the lane that holds 1.
    let tiny = f32::from_bits(0x33800000);
    for (n, ph) in [(64usize, 0u32), (127, 0), (72, 28), (65 + 8, 4), (64 + 9, 0)] {
        let mut x = vec![tiny; n + 8];
        x[0] = 1.0;
        let s = window_sum_scalar(&x, n);
        let v = window_sum_vector(&x, n, ph);
        assert_eq!(s, 1.0, "scalar n {n}");
        assert!(v > 1.0, "vector n {n} ph {ph}: {v}");
    }
}

#[test]
fn vector_peel_tail_and_signed_zero() {
    // an all -0 window gives +0 on the vector route (accumulators and T start at +0)
    let x = vec![-0.0f32; 80];
    assert_eq!(window_sum_vector(&x, 72, 28).to_bits(), 0);
    assert_eq!(window_sum_vector(&x, 64, 0).to_bits(), 0);
    // the peel and the scalar tail are added to T in index order, then S = (g0 + g1) + T:
    // ph = 28 gives p = 1; x0 = 1 is the peel, x[1..65) = 2^-24 each form the single main trip, rem = 8 is a wide trip
    let tiny = f32::from_bits(0x33800000);
    let mut x = vec![tiny; 80];
    x[0] = 1.0;
    let v = window_sum_vector(&x, 73, 28);
    // 64 + 8 tiny elements are summed lane-wise first (exact, multiples of 2^-24), then added to T = 1.0
    let small = 72.0f32 * tiny;
    assert_eq!(v, 1.0f32 + small);
}

#[test]
fn body_b_is_the_vertical_mirror_of_body_a() {
    // B on a plane equals A on the row-reversed plane, row-reversed back; the epilogue corners of B sit at rows 0 and H - 1
    let (w, h) = (6usize, 5usize);
    let src: Vec<f32> = (0..w * h).map(|i| (i as f32) * 0.37 - 3.0).collect();
    let p = Params { radii: [1, 1, 0], d: 1 };
    let ph = RowPhases([0, 0, 0]);
    let band = Band { start: 0, count: 4 };
    let mut b = vec![9.0f32; w * h];
    run_body(true, &p, &ph, band, &src, &mut b, w, h).unwrap();
    let mut fs = vec![0.0f32; w * h];
    for r in 0..h {
        fs[r * w..(r + 1) * w].copy_from_slice(&src[(h - 1 - r) * w..(h - r) * w]);
    }
    let mut a = vec![9.0f32; w * h];
    for r in 0..h {
        a[r * w..(r + 1) * w].fill(9.0);
    }
    run_body(false, &p, &ph, band, &fs, &mut a, w, h).unwrap();
    for r in 0..h {
        for c in 0..w {
            assert_eq!(b[r * w + c].to_bits(), a[(h - 1 - r) * w + c].to_bits());
        }
    }
    assert_eq!(b[0].to_bits(), 0);
    assert_eq!(b[w * h - 1].to_bits(), 0);
}
