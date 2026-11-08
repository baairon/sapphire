// Zero-sized and tiny planes: the pass is a successful no-op and leaves the destination untouched.
use glow_layer::*;

fn args() -> PassArgs {
    PassArgs { thresholds: [0.5; 3], sigma_x: [1.0; 3], sigma_y: [1.0; 3], brightness: [1.0; 3], subpixel: 0, edge_mode: 0, show_threshold: 0, aspect: None }
}

#[test]
fn zero_sizes_are_noops() {
    for (w, h) in [(0usize, 0usize), (0, 5), (5, 0)] {
        for n in [1usize, 3] {
            let src: Vec<Vec<f32>> = (0..n).map(|_| vec![0.9f32; 25]).collect();
            let mut dst: Vec<Vec<f32>> = (0..n).map(|_| vec![f32::from_bits(0x7fc0ffee); 25]).collect();
            let pl: Vec<Plane> = src.iter().map(|s| Plane { data: s, width: w, height: h, stride: 5 }).collect();
            let mut it = dst.iter_mut();
            let mut dm: Vec<PlaneMut> = (0..n).map(|_| PlaneMut { data: it.next().unwrap(), width: w, height: h, stride: 5 }).collect();
            assert_eq!(glow_pass(&pl, &mut dm, &args()), Ok(()), "w {w} h {h} n {n}");
            drop(dm);
            assert!(dst.iter().all(|d| d.iter().all(|v| v.to_bits() == 0x7fc0ffee)));
            let p = GlowParams { thresholds: [0.5; 3], width_xy: [1.0; 2], width: [3.0; 3], brightness: [1.0; 3], subpixel: 0, edge_mode: 0, show_threshold: 0, plane_count: n, render_scale: 8, aspect: None };
            let mut it = dst.iter_mut();
            let mut dm: Vec<PlaneMut> = (0..n).map(|_| PlaneMut { data: it.next().unwrap(), width: w, height: h, stride: 5 }).collect();
            assert_eq!(render_full(&p, &pl, &mut dm), Ok(()));
            let bp = BrightParams { thresholds: [0.5; 3] };
            assert_eq!(bright_pass(&bp, &pl, &mut dm), Ok(()));
        }
    }
}

#[test]
fn zero_length_copy_is_ok() {
    let roi = RoiRecord::form1(0, 0);
    let mut d = [1.0f32; 4];
    let r = copy_plane(&mut d, &[2.0f32; 4], &roi).unwrap();
    assert_eq!(r.elements, 0);
    assert_eq!(d, [1.0; 4]);
}

#[test]
fn degenerate_geometry_runs() {
    // width 1 and height 1 planes go through the pass and the blur without error
    for (w, h) in [(1usize, 1usize), (1, 6), (6, 1)] {
        let src = vec![0.9f32; w * h];
        let mut dst = vec![0.0f32; w * h];
        let pl = [Plane { data: &src, width: w, height: h, stride: w }];
        let mut dm = [PlaneMut { data: &mut dst, width: w, height: h, stride: w }];
        assert_eq!(glow_pass(&pl, &mut dm, &args()), Ok(()), "{w}x{h}");
    }
}
