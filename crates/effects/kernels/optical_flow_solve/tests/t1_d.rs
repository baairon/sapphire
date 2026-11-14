use optical_flow_solve::relax::*;

fn ctx<'a>(w: i32, h: i32, st: i32, scale: f32, v: Option<&'a [f32]>, a: &'a [Vec<f32>; 6], ph: u32) -> Ctx<'a> {
    Ctx {
        width: w,
        height: h,
        stride: st,
        scale,
        v,
        nests: [NestIn { t: &a[0], u: &a[1], acc: &a[2] }, NestIn { t: &a[3], u: &a[4], acc: &a[5] }],
        phase_bytes: ph,
    }
}

fn planes(n: usize, f: impl Fn(usize, usize) -> f32) -> [Vec<f32>; 6] {
    [0, 1, 2, 3, 4, 5].map(|k| (0..n).map(|i| f(k, i)).collect())
}

/// Reference with small integers (every operation exact): out = scale*v*(dx + dy) + acc.
fn reference(w: usize, h: usize, st: usize, scale: f32, v: &[f32], t: &[f32], u: &[f32], acc: &[f32], y: usize) -> Vec<f32> {
    let mut o = vec![0.0; w];
    for x in 0..w {
        let i = y * st + x;
        let dy = if y == 0 { u[i] } else if y == h - 1 { -u[i - st] } else { u[i] - u[i - st] };
        let dx = if x == 0 { t[i] } else if x == w - 1 { -t[i - 1] } else { t[i] - t[i - 1] };
        // column 0 adds t[0] itself; later columns j add the difference between t[j] and t[j-1]
        o[x] = scale * v[i] * (dx + dy) + acc[i];
    }
    o
}

#[test]
fn hand_cases_small_integers() {
    for (w, h) in [(1usize, 1usize), (1, 3), (2, 2), (3, 3), (4, 3), (5, 4), (17, 3), (18, 2), (40, 3)] {
        let st = w + 2;
        let n = h * st;
        let a = planes(n, |k, i| ((i * 7 + k * 3) % 11) as f32 - 5.0);
        let v: Vec<f32> = (0..n).map(|i| (i % 3) as f32 + 1.0).collect();
        for ph in [0u32, 4, 8, 28, 60] {
            let c = ctx(w as i32, h as i32, st as i32, 2.0, Some(&v), &a, ph);
            let (mut vo_1, mut vo_2) = (vec![-99.0f32; n], vec![-99.0f32; n]);
            relax_callback(&c, [&mut vo_1, &mut vo_2], 0, h as i32);
            for y in 0..h {
                let vr_1 = reference(w, h, st, 2.0, &v, &a[0], &a[1], &a[2], y);
                let vr_2 = reference(w, h, st, 2.0, &v, &a[3], &a[4], &a[5], y);
                assert_eq!(&vo_1[y * st..y * st + w], &vr_1[..], "nest1 w{w} h{h} y{y} ph{ph}");
                assert_eq!(&vo_2[y * st..y * st + w], &vr_2[..], "nest2 w{w} h{h} y{y} ph{ph}");
                // padding stays untouched
                for p in w..st {
                    assert_eq!(vo_1[y * st + p], -99.0);
                }
            }
        }
    }
}

#[test]
fn split_follows_listing_thresholds() {
    assert_eq!(split_cols(15, 0), (0, 0, true));
    assert_eq!(split_cols(16, 0), (0, 16, false));
    assert_eq!(split_cols(1199, 12), (0, 1184, false));
    assert_eq!(split_cols(1200, 0), (0, 1200, false));
    assert_eq!(split_cols(1200, 2), (0, 0, true));
    assert_eq!(split_cols(1200, 4), (15, 1200 - ((1200 - 15) & 15), false));
}

#[test]
fn zero_width_and_empty_ranges() {
    let a = planes(8, |_, i| i as f32);
    let v = vec![1.0; 8];
    let c = ctx(0, 2, 4, 1.0, Some(&v), &a, 0);
    let (mut vo_1, mut vo_2) = (vec![5.0; 8], vec![5.0; 8]);
    relax_callback(&c, [&mut vo_1, &mut vo_2], 0, 2);
    assert!(vo_1.iter().all(|&x| x == 5.0));
    let c = ctx(3, 2, 4, 1.0, Some(&v), &a, 0);
    relax_callback(&c, [&mut vo_1, &mut vo_2], 1, 0);
    assert!(vo_1.iter().all(|&x| x == 5.0));
}

#[test]
fn scale_plane_absent_hand_cases() {
    for (w, h) in [(1usize, 1usize), (1, 3), (2, 2), (3, 3), (4, 3), (5, 4), (17, 3), (18, 2), (40, 3), (41, 5)] {
        let st = w + 2;
        let n = h * st;
        let a = planes(n, |k, i| ((i * 5 + k * 3) % 13) as f32 - 6.0);
        let ones = vec![1.0f32; n];
        for ph in [0u32, 4, 8, 28, 60] {
            let c = ctx(w as i32, h as i32, st as i32, 2.0, None, &a, ph);
            let (mut vo_1, mut vo_2) = (vec![0.0; n], vec![0.0; n]);
            relax_callback(&c, [&mut vo_1, &mut vo_2], 0, h as i32);
            for y in 0..h {
                let vr_1 = reference(w, h, st, 2.0, &ones, &a[0], &a[1], &a[2], y);
                let vr_2 = reference(w, h, st, 2.0, &ones, &a[3], &a[4], &a[5], y);
                assert_eq!(&vo_1[y * st..y * st + w], &vr_1[..], "w{w} h{h} y{y} ph{ph}");
                assert_eq!(&vo_2[y * st..y * st + w], &vr_2[..], "w{w} h{h} y{y} ph{ph}");
            }
        }
    }
}

#[test]
fn scale_plane_absent_empty_range_and_scale_is_used_directly() {
    let a = planes(8, |_, i| i as f32);
    let c = ctx(3, 2, 4, 1.0, None, &a, 0);
    let (mut vo_1, mut vo_2) = (vec![5.0; 8], vec![5.0; 8]);
    relax_callback(&c, [&mut vo_1, &mut vo_2], 0, 0);
    assert!(vo_1.iter().chain(vo_2.iter()).all(|&x| x == 5.0));
    // scale 0: out = acc exactly
    let c = ctx(3, 2, 4, 0.0, None, &a, 0);
    relax_callback(&c, [&mut vo_1, &mut vo_2], 0, 2);
    assert_eq!(vo_1[0], a[2][0]);
    assert_eq!(vo_2[5], a[5][5]);
}

#[test]
fn nan_propagates_and_is_deterministic() {
    let mut a = planes(12, |_, i| i as f32);
    a[0][1] = f32::from_bits(0xffc12345);
    let v = vec![1.0; 12];
    let run = || {
        let c = ctx(4, 3, 4, 1.0, Some(&v), &a, 0);
        let (mut vo_1, mut vo_2) = (vec![0.0; 12], vec![0.0; 12]);
        relax_callback(&c, [&mut vo_1, &mut vo_2], 0, 3);
        (vo_1, vo_2)
    };
    let (vr_1, vr_2) = (run(), run());
    assert_eq!(vr_1.0.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), vr_2.0.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
    // t[1] feeds columns 0 and 1 of nest 1 (difference with its neighbours), quiet payload kept
    assert_eq!(vr_1.0[1].to_bits(), 0xffc12345);
    assert!(vr_1.0[2].is_nan());
    assert!(vr_1.1.iter().all(|x| !x.is_nan()));
}

#[test]
fn assumptions_are_listed() {
    let ids: Vec<_> = assumptions().iter().map(|a| a.id).collect();
    assert!(ids.contains(&"SIGN-MASK-256") && ids.contains(&"LANE-INDEX-RAMP"));
}

// Pins of the first-operand rule at the two-NaN sites of the first row (silicon rule, source order).
#[test]
fn two_nan_pins_first_operand() {
    let nan_a = f32::from_bits(0x7fc0_0a0a);
    let nan_b = f32::from_bits(0x7fc0_0b0b);
    // column 0, first row: s = t + u; product = scale * v; result = fma(product, s, acc)
    let mut a = planes(8, |_, _| 1.0);
    let v = vec![1.0; 8];
    a[0][0] = nan_a; // t
    a[1][0] = nan_b; // u
    let c = ctx(2, 2, 4, 1.0, Some(&v), &a, 0);
    let (mut vo_1, mut vo_2) = (vec![0.0; 8], vec![0.0; 8]);
    relax_callback(&c, [&mut vo_1, &mut vo_2], 0, 1);
    assert_eq!(vo_1[0].to_bits(), 0x7fc0_0a0a, "t + u keeps t");
    // scale * v keeps scale
    let mut vv = vec![1.0; 8];
    vv[0] = nan_b;
    let a = planes(8, |_, _| 1.0);
    let c = ctx(2, 2, 4, nan_a, Some(&vv), &a, 0);
    relax_callback(&c, [&mut vo_1, &mut vo_2], 0, 1);
    assert_eq!(vo_1[0].to_bits(), 0x7fc0_0a0a, "scale * v keeps scale");
    // fma priority: product, sum, acc
    let mut a = planes(8, |_, _| 1.0);
    a[0][0] = nan_a; // sum NaN
    a[2][0] = nan_b; // acc NaN
    let v = vec![1.0; 8];
    let c = ctx(2, 2, 4, 1.0, Some(&v), &a, 0);
    relax_callback(&c, [&mut vo_1, &mut vo_2], 0, 1);
    assert_eq!(vo_1[0].to_bits(), 0x7fc0_0a0a, "sum before acc");
    // row difference of the middle row: u_cur - u_abv keeps u_cur (3 rows, row 1)
    let mut a = planes(12, |_, _| 1.0);
    a[1][4] = nan_a; // u_cur at row 1 col 0
    a[1][0] = nan_b; // u_abv
    let v = vec![1.0; 12];
    let c = ctx(2, 3, 4, 1.0, Some(&v), &a, 0);
    let (mut vo_1, mut vo_2) = (vec![0.0; 12], vec![0.0; 12]);
    relax_callback(&c, [&mut vo_1, &mut vo_2], 1, 1);
    assert_eq!(vo_1[4].to_bits(), 0x7fc0_0a0a, "u_cur - u_abv keeps u_cur");
}
