use optical_flow_entry::*;

fn planes(w: i32, h: i32) -> (Vec<f32>, usize) {
    (vec![0.0; (w * h) as usize], (w * h) as usize)
}

struct Z {
    w: i32,
    h: i32,
    target_plane: Vec<f32>,
    img: Vec<f32>,
    u: Vec<f32>,
    v: Vec<f32>,
    g: [Vec<f32>; 4],
}
impl Z {
    fn new(w: i32, h: i32) -> Z {
        let (z, _) = planes(w, h);
        Z { w, h, target_plane: z.clone(), img: z.clone(), u: z.clone(), v: z.clone(), g: [z.clone(), z.clone(), z.clone(), z] }
    }
    fn run(&self, scale: f32) -> Vec<f32> {
        let p = WarpPlanes { width: self.w, height: self.h, stride: self.w, target_plane: &self.target_plane, img: &self.img, u: &self.u, v: &self.v,
            grad_a_x: &self.g[0], grad_a_y: &self.g[1], grad_b_x: &self.g[2], grad_b_y: &self.g[3], scale };
        let mut out = vec![0.0f32; (self.w * self.h) as usize];
        final_warp_rows(&p, &mut out, 0, self.h);
        out
    }
}

#[test]
fn zero_flow_interior_is_one_over_one_plus_diff() {
    // 4x4, zero flow: sample at (x, y) with x, y >= 1 is img[y][x]; target_plane - img = 0.5 -> 1 / (0.5*2 + 1) = 0.5
    let mut z = Z::new(4, 4);
    z.img = vec![1.0; 16];
    z.target_plane = vec![1.5; 16];
    let o = z.run(2.0);
    // interior pixel (1,1): inside the open window (1e-4, w - 1.0001)
    assert_eq!(o[5].to_bits(), 0.5f32.to_bits());
    // border pixel (0, 0): sx = 0 <= eps, outside: sample 0 -> diff 1.5 -> 1 / (1.5*2+1) = 0.25
    assert_eq!(o[0].to_bits(), 0.25f32.to_bits());
}

#[test]
fn gradient_terms_add_sqrt() {
    let mut z = Z::new(3, 3);
    z.g[0] = vec![3.0; 9];
    z.g[1] = vec![4.0; 9];
    z.g[2] = vec![0.0; 9];
    z.g[3] = vec![0.0; 9];
    // outside everywhere (3x3 window is (1e-4, 1.9999)): pixel (1,1) is inside though; use scale 0 and equal planes
    let o = z.run(0.0);
    // mag_a = 5, mag_b = 0, abs term = 1: 1 / (1 + 5) 
    assert_eq!(o[4].to_bits(), (1.0f32 / 6.0).to_bits());
}

#[test]
fn half_pixel_bilinear() {
    let mut z = Z::new(4, 4);
    for y in 0..4 { for x in 0..4 { z.img[y * 4 + x] = (x + 10 * y) as f32; } }
    z.u = vec![0.5; 16];
    z.v = vec![0.5; 16];
    // pixel (1,1): sample (1.5, 1.5): img = 1.5 + 15 = 16.5; target_plane = 16.5 -> diff 0 -> out = 1 / (0*s + 1) = 1
    z.target_plane = vec![16.5; 16];
    let o = z.run(3.0);
    assert_eq!(o[5].to_bits(), 1.0f32.to_bits());
}

#[test]
fn width_one_and_height_one() {
    let z = Z::new(1, 1);
    let o = z.run(1.0);
    // sx = 0 <= eps: outside, sample 0, diff 0 -> 1 / (0 + 1) = 1
    assert_eq!(o, vec![1.0]);
    let mut z = Z::new(1, 5);
    z.target_plane = vec![2.0; 5];
    assert_eq!(z.run(1.0)[3].to_bits(), (1.0f32 / 3.0).to_bits());
    let z = Z::new(5, 1);
    assert_eq!(z.run(0.0), vec![1.0; 5]);
}

#[test]
fn nan_propagates_and_is_deterministic() {
    let mut z = Z::new(4, 4);
    z.img[5] = f32::NAN;
    z.target_plane = vec![1.0; 16];
    let a = z.run(1.0);
    let b = z.run(1.0);
    assert_eq!(a.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), b.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
    assert!(a[5].is_nan());
    // a negative radicand never occurs (squares), but a fresh NaN is the default NaN
    let mut z = Z::new(2, 2);
    z.g[0] = vec![f32::INFINITY; 4];
    z.g[1] = vec![f32::NEG_INFINITY; 4];
    let o = z.run(1.0);
    assert_eq!(o[0].to_bits(), 0.0f32.to_bits()); // 1 / inf
    let mut z = Z::new(2, 2);
    z.u = vec![f32::NAN; 4];
    z.target_plane = vec![0.0; 4];
    // NaN position: the window test is false for every NaN comparison except the last (jb), which is taken when y is finite and in range
    let o = z.run(1.0);
    assert!(o.iter().all(|x| x.is_nan() || x.to_bits() == 0x3f80_0000));
}

#[test]
fn row_range_writes_only_its_rows() {
    let z = Z::new(3, 4);
    let p = WarpPlanes { width: 3, height: 4, stride: 3, target_plane: &z.target_plane, img: &z.img, u: &z.u, v: &z.v, grad_a_x: &z.g[0], grad_a_y: &z.g[1], grad_b_x: &z.g[2], grad_b_y: &z.g[3], scale: 1.0 };
    let mut out = vec![-7.0f32; 12];
    final_warp_rows(&p, &mut out, 1, 2);
    assert!(out[..3].iter().all(|&x| x == -7.0));
    assert!(out[9..].iter().all(|&x| x == -7.0));
    assert!(out[3..9].iter().all(|&x| x == 1.0));
}

#[test]
fn lane_params_fields() {
    let o = ObjFields { max_warps: 1, max_outer: 2, max_inner: 3, use_median: 4, lambda_bits: 5, texture_scale: 0.5, std_smooth_bits: 9 };
    let p = lane_params(&o, 0.0);
    assert_eq!((p.max_warps, p.max_outer, p.max_inner, p.use_median, p.lambda_bits, p.std_smooth_bits), (1, 2, 3, 4, 5, 9));
    assert_eq!(p.scale_gt, 1);
    assert_eq!(p.scale_bits, 0.5f32.to_bits());
    assert_eq!(p.smooth_weight.to_bits(), 0x3e66_6666);
    assert_eq!(lane_params(&ObjFields { texture_scale: f32::NAN, ..o }, 0.0).scale_gt, 0);
    assert_eq!(lane_params(&ObjFields { texture_scale: 0.0, ..o }, 0.0).scale_gt, 0);
}

#[derive(Default)]
struct Rec { log: Vec<String>, a_ret: i32, abort: bool }
impl EntryHost for Rec {
    fn pyramid_body(&mut self, _: &LaneParams) -> i32 { self.log.push("A".into()); self.a_ret }
    fn solve_body(&mut self, _: &LaneParams) { self.log.push("B".into()); }
    fn poll_abort(&mut self) -> bool { self.log.push("poll".into()); self.abort }
    fn launch(&mut self, l: Launch) { self.log.push(format!("{:?}", l)); }
    fn copy_plane(&mut self, s: EntryPlane, o: OutPlane) { self.log.push(format!("copy{:?}{:?}", s, o)); }
    fn scale_copy(&mut self, s: EntryPlane, o: OutPlane, k: f32) { self.log.push(format!("sc{:?}{:?}{}", s, o, k)); }
}

#[test]
fn entry_control_flow() {
    let o = ObjFields::default();
    let mut r = Rec::default();
    assert_eq!(run_entry(&mut r, &o, 0.0, false), EntryPath::InLine);
    assert_eq!(r.log.len(), 3 + 5 + 3);
    assert_eq!(&r.log[..3], &["A", "B", "poll"]);
    let mut r = Rec { a_ret: 1, ..Default::default() };
    assert_eq!(run_entry(&mut r, &o, 0.0, false), EntryPath::PyramidStopped);
    assert_eq!(r.log, vec!["A"]);
    let mut r = Rec { abort: true, ..Default::default() };
    assert_eq!(run_entry(&mut r, &o, 0.0, true), EntryPath::Aborted);
    let mut r = Rec::default();
    assert_eq!(run_entry(&mut r, &o, 0.0, true), EntryPath::Deferred);
    assert_eq!(r.log.len(), 3);
}

#[test]
fn scale_copy_cases() {
    let s = [1.0f32, -2.0, f32::NAN];
    let mut d = [0.0f32; 3];
    scale_copy_model(&mut d, &s, 1.0);
    assert_eq!(d[1], -2.0);
    scale_copy_model(&mut d, &s, -1.0);
    assert_eq!((d[0], d[1]), (-1.0, 2.0));
    assert!(d[2].is_nan());
}

#[test]
fn scale_copy_zero_scale_fills_positive_zero() {
    let s = [-3.5f32, 2.0, f32::NAN, -0.0];
    for k in [0.0f32, -0.0f32] {
        let mut d = [1.0f32; 4];
        scale_copy_model(&mut d, &s, k);
        for v in d {
            assert_eq!(v.to_bits(), 0);
        }
    }
    let mut d = [1.0f32; 4];
    scale_copy_model(&mut d, &s, f32::NAN);
    assert!(d.iter().all(|v| v.is_nan()));
}

#[test]
fn threshold_is_the_measured_cell_word() {
    assert_eq!(SCALE_THRESHOLD.to_bits(), 0x3c23_d70a);
    let o = ObjFields { texture_scale: 0.009, ..Default::default() };
    assert_eq!(lane_params(&o, SCALE_THRESHOLD).scale_gt, 0);
    let o = ObjFields { texture_scale: 0.011, ..Default::default() };
    assert_eq!(lane_params(&o, SCALE_THRESHOLD).scale_gt, 1);
}

#[test]
fn fix_sites_params_launches_and_outputs() {
    use optical_flow_entry::{EntryPlane, Launch, LAUNCHES};
    let p = lane_params(&ObjFields { texture_scale: 0.5, ..Default::default() }, SCALE_THRESHOLD);
    assert_eq!(p.small_weight.to_bits(), 0x3ca3_d70a);
    assert_eq!(p.one, 1);
    assert_eq!(LAUNCHES[3], Launch::DerivY { src: EntryPlane::FlowV, dst: EntryPlane::Nest2T });
}
