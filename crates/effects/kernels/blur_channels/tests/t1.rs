use blur_channels::*;

fn base(n: usize, nin: usize) -> ChannelParams {
    ChannelParams { amount: [0.0, 0.0], widths: [1.0; 4], gains: [1.0; 4], filter_shape: 1, scale_count: 4, subpixel: 0, edge_mode: 1, plane_count: n, n_in: nin, render_scale: 1, aspect: None }
}

fn run(p: &ChannelParams, srcs: &[Vec<f32>], w: usize, h: usize) -> Result<Vec<Vec<f32>>, FxError> {
    let pl: Vec<Plane> = srcs.iter().map(|s| Plane::new(s, w, h, w).unwrap()).collect();
    let mut outs: Vec<Vec<f32>> = (0..p.plane_count).map(|_| vec![9.0; w * h]).collect();
    {
        let mut it = outs.iter_mut();
        let mut dm: Vec<PlaneMut> = (0..p.plane_count).map(|_| PlaneMut::new(it.next().unwrap(), w, h, w).unwrap()).collect();
        render(p, &pl, &mut dm)?;
    }
    Ok(outs)
}

#[test]
fn sigmas_are_hand_values() {
    let mut p = base(3, 3);
    p.amount = [8.0, 16.0];
    p.render_scale = 2;
    p.widths = [1.0, 0.5, -2.0, 1.0];
    assert_eq!(channel_sigmas(&p, 0), (2.0, 4.0));
    assert_eq!(channel_sigmas(&p, 1), (1.0, 2.0));
    assert_eq!(channel_sigmas(&p, 2), (4.0, 8.0));
}

#[test]
fn gain_overrides() {
    let mut p = base(2, 2);
    p.gains = [0.5, 0.25, 0.125, 0.0625];
    p.scale_count = 1;
    assert_eq!(channel_gain(&p, 1), 1.0);
    assert_eq!(channel_gain(&p, 0), 0.5);
    p.plane_count = 3;
    assert_eq!(channel_gain(&p, 1), 0.25);
    assert_eq!(channel_gain(&p, 3), 1.0);
    p.scale_count = 4;
    assert_eq!(channel_gain(&p, 3), 0.0625);
}

#[test]
fn zero_sigma_scales_by_gain() {
    let mut p = base(2, 2);
    p.gains = [0.5, 2.0, 1.0, 1.0];
    let o = run(&p, &[vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]], 3, 1).unwrap();
    assert_eq!(o[0], vec![0.5, 1.0, 1.5]);
    assert_eq!(o[1], vec![8.0, 10.0, 12.0]);
}

#[test]
fn missing_planes_copy_the_last_supplied() {
    let mut p = base(3, 1);
    p.gains = [1.0, 2.0, 3.0, 1.0];
    let o = run(&p, &[vec![1.0, 2.0]], 2, 1).unwrap();
    assert_eq!(o[0], vec![1.0, 2.0]);
    assert_eq!(o[1], vec![2.0, 4.0]);
    assert_eq!(o[2], vec![3.0, 6.0]);
}

#[test]
fn one_by_one_and_flat() {
    let mut p = base(1, 1);
    p.amount = [8.0, 8.0];
    let o = run(&p, &[vec![0.75]], 1, 1).unwrap();
    assert_eq!(o[0].len(), 1);
    let flat = run(&p, &[vec![1.0; 9]], 3, 3).unwrap();
    assert!(flat[0].iter().all(|v| (*v - 1.0).abs() < 1e-5));
}

#[test]
fn nan_propagates_and_is_deterministic() {
    let mut p = base(1, 1);
    p.amount = [8.0, 8.0];
    let src = vec![f32::NAN, 1.0, 1.0, 1.0];
    let a = run(&p, &[src.clone()], 4, 1).unwrap();
    let b = run(&p, &[src], 4, 1).unwrap();
    assert!(a[0].iter().any(|v| v.is_nan()));
    assert_eq!(a[0].iter().map(|v| v.to_bits()).collect::<Vec<_>>(), b[0].iter().map(|v| v.to_bits()).collect::<Vec<_>>());
}

#[test]
fn nan_amount_gives_canonical_nan_sigma() {
    let mut p = base(1, 1);
    p.amount = [f32::NAN, 1.0];
    p.widths[0] = f32::from_bits(0x7fc0_1234);
    let (sx, _) = channel_sigmas(&p, 0);
    assert!(sx.is_nan());
}

#[test]
fn error_paths() {
    let p = base(5, 1);
    let src = vec![0.0f32; 4];
    let pl = vec![Plane::new(&src, 2, 2, 2).unwrap()];
    let mut d = vec![0.0f32; 4];
    let mut dm = vec![PlaneMut::new(&mut d, 2, 2, 2).unwrap()];
    assert!(render(&p, &pl, &mut dm).is_err());
    let mut q = base(1, 0);
    q.n_in = 0;
    assert!(render(&q, &pl, &mut dm).is_err());
    let mut e = base(1, 1);
    e.edge_mode = 7;
    assert!(render(&e, &pl, &mut dm).is_err());
}

#[test]
fn slot38_matches_the_emulated_table() {
    // rows from the emulated entry 0x52bb70 with both predicates hooked: (a1, a2, a4, b4) -> eax (t2/layouts/slot38.json)
    let rows: [((bool, bool, bool, bool), i32); 16] = [
        ((false, false, false, false), -1), ((true, false, false, false), -1), ((false, true, false, false), -1), ((true, true, false, false), -1),
        ((false, false, true, false), 4), ((true, false, true, false), 4), ((false, true, true, false), 4), ((true, true, true, false), 4),
        ((false, false, false, true), -1), ((true, false, false, true), 0), ((false, true, false, true), 0), ((true, true, false, true), 0),
        ((false, false, true, true), 4), ((true, false, true, true), 4), ((false, true, true, true), 4), ((true, true, true, true), 4),
    ];
    for ((a_has_one, a_has_two, a_has_four, b_has_four), want) in rows {
        assert_eq!(binding_collapse(a_has_one, a_has_two, a_has_four, b_has_four), want);
    }
}
