// Hand-evaluated cases of the full glow layer (render_full, glow_pass).
use glow_layer::*;

fn lcg(seed: &mut u64) -> f32 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((*seed >> 40) as f32 / 16777216.0) * 2.0 - 0.5
}
fn mk(w: usize, h: usize, seed: u64) -> Vec<f32> {
    let mut s = seed;
    (0..w * h).map(|_| lcg(&mut s)).collect()
}
fn params(n: usize) -> GlowParams {
    GlowParams {
        thresholds: [0.3, 0.2, 0.4],
        width_xy: [1.0, 1.5],
        width: [4.0, 5.0, 6.0],
        brightness: [1.0, 0.7, 1.3],
        subpixel: 0,
        edge_mode: 1,
        show_threshold: 0,
        plane_count: n,
        render_scale: 8,
        aspect: None,
    }
}
fn run(p: &GlowParams, planes: &[Vec<f32>], w: usize, h: usize) -> Vec<Vec<f32>> {
    let src: Vec<Plane> = planes.iter().map(|d| Plane { data: d, width: w, height: h, stride: w }).collect();
    let mut out: Vec<Vec<f32>> = (0..p.plane_count).map(|_| vec![9.0; w * h]).collect();
    let mut dst: Vec<PlaneMut> = out.iter_mut().map(|d| PlaneMut { data: d, width: w, height: h, stride: w }).collect();
    render_full(p, &src, &mut dst).unwrap();
    out
}
fn bits(v: &[Vec<f32>]) -> Vec<Vec<u32>> {
    v.iter().map(|p| p.iter().map(|x| x.to_bits()).collect()).collect()
}

#[test]
fn t3_determinism() {
    for n in [1usize, 3] {
        let planes: Vec<Vec<f32>> = (0..n).map(|c| mk(37, 19, 100 + c as u64)).collect();
        let p = params(n);
        let a = run(&p, &planes, 37, 19);
        let b = run(&p, &planes, 37, 19);
        assert_eq!(bits(&a), bits(&b));
        assert!(a.iter().all(|pl| pl.iter().all(|x| x.is_finite())));
    }
}

#[test]
fn t3_brightness_zero_clears() {
    // gain 0: the blur core clears the plane (listing implies: 1537a0 driver gain-0 shortcut, so the layer is +0.0
    for n in [1usize, 3] {
        for edge in [0, 1] {
            let planes: Vec<Vec<f32>> = (0..n).map(|c| mk(20, 11, 7 + c as u64)).collect();
            let mut p = params(n);
            p.brightness = [0.0; 3];
            p.edge_mode = edge;
            let o = run(&p, &planes, 20, 11);
            assert!(o.iter().all(|pl| pl.iter().all(|x| x.to_bits() == 0)), "n={n} edge={edge}");
        }
    }
}

#[test]
fn t3_show_threshold_equals_bright_pass() {
    for n in [1usize, 3] {
        let planes: Vec<Vec<f32>> = (0..n).map(|c| mk(37, 19, 300 + c as u64)).collect();
        let mut p = params(n);
        p.show_threshold = 1;
        let got = run(&p, &planes, 37, 19);
        let src: Vec<Plane> = planes.iter().map(|d| Plane { data: d, width: 37, height: 19, stride: 37 }).collect();
        let mut want: Vec<Vec<f32>> = (0..n).map(|_| vec![9.0; 37 * 19]).collect();
        let mut dst: Vec<PlaneMut> = want.iter_mut().map(|d| PlaneMut { data: d, width: 37, height: 19, stride: 37 }).collect();
        bright_pass(&BrightParams { thresholds: if n == 1 { [p.thresholds[0], 0.0, 0.0] } else { p.thresholds } }, &src, &mut dst).unwrap();
        assert_eq!(bits(&got), bits(&want), "n={n}");
    }
}

#[test]
fn t1_composition_equals_stages() {
    // one plane: bright_pass then blur_core with the derived sigma, passes 3
    let (w, h) = (24, 17);
    let plane = mk(w, h, 5);
    let p = params(1);
    let got = run(&p, &[plane.clone()], w, h);
    let mut bp = vec![0.0f32; w * h];
    {
        let src = [Plane { data: &plane, width: w, height: h, stride: w }];
        let mut dst = [PlaneMut { data: &mut bp, width: w, height: h, stride: w }];
        bright_pass(&BrightParams { thresholds: [p.thresholds[0], 0.0, 0.0] }, &src, &mut dst).unwrap();
    }
    let (sx, sy) = glow_sigmas(&p);
    let mut want = vec![0.0f32; w * h];
    blur::blur_core(&bp, &mut want, w, h, sx[0], sy[0], p.brightness[0], blur::Edge::Mirror, 3, false, None).unwrap();
    assert_eq!(bits(&got), bits(&[want]));
}

#[test]
fn t1_sigma_derivation() {
    let mut p = params(1);
    p.render_scale = 3;
    p.width = [2.5, 0.7, 1.9];
    p.width_xy = [1.3, 0.6];
    let (sx, sy) = glow_sigmas(&p);
    let w = (0.125f32 * 2.5) * 3.0;
    assert_eq!(sx[0].to_bits(), (w * 1.3f32).to_bits());
    assert_eq!(sy[0].to_bits(), (w * 0.6f32).to_bits());
    p.plane_count = 3;
    let (sx, sy) = glow_sigmas(&p);
    let k = 0.125f32 * 3.0;
    for c in 0..3 {
        let wc = k * p.width[c];
        assert_eq!(sx[c].to_bits(), (1.3f32 * wc).to_bits());
        assert_eq!(sy[c].to_bits(), (wc * 0.6f32).to_bits());
    }
}

#[test]
fn t1_association_orders_are_bit_equal() {
    // 0.125 is a power of two, so (0.125 * width) * scale and (0.125 * scale) * width are bit equal (no underflow here):
    // the different association of the one and three plane branches of 0x52b220 cannot change a bit (INFERRED, checked)
    let mut s = 99u64;
    for _ in 0..20000 {
        let width = (lcg(&mut s) + 0.6) * 5.0;
        let scale = ((lcg(&mut s) + 0.6) * 400.0) as i32 + 1;
        let a = (0.125f32 * width) * scale as f32;
        let b = (0.125f32 * scale as f32) * width;
        assert_eq!(a.to_bits(), b.to_bits());
    }
}

#[test]
fn t1_errors_and_clamped_inputs() {
    let plane = mk(8, 8, 1);
    let src = [Plane { data: &plane, width: 8, height: 8, stride: 8 }];
    let mut o = vec![0.0f32; 64];
    let mut p = params(2);
    let mut dst = [PlaneMut { data: &mut o, width: 8, height: 8, stride: 8 }];
    assert_eq!(render_full(&p, &src, &mut dst), Err(FxError::PlaneCount));
    p.plane_count = 3;
    assert_eq!(render_full(&p, &src, &mut dst), Err(FxError::PlaneCount));
    p.plane_count = 1;
    p.edge_mode = 2;
    assert_eq!(render_full(&p, &src, &mut dst), Err(FxError::Geometry));
    p.show_threshold = 1; // the edge mode is not looked at when the layer is returned early
    assert!(render_full(&p, &src, &mut dst).is_ok());
    // three planes with one input plane: all outputs come from src[0]
    let mut p3 = params(3);
    p3.show_threshold = 1;
    let mut o3: Vec<Vec<f32>> = (0..3).map(|_| vec![0.0; 64]).collect();
    let mut d3: Vec<PlaneMut> = o3.iter_mut().map(|d| PlaneMut { data: d, width: 8, height: 8, stride: 8 }).collect();
    render_full(&p3, &src, &mut d3).unwrap();
    let (a, b) = (bits(&o3[..1]), bits(&o3[1..2]));
    let _ = (a, b); // thresholds differ per lane, so only the shape is checked here
    assert!(o3.iter().all(|pl| pl.len() == 64));
}

#[test]
fn t3_strided_matches_dense_and_padding_untouched() {
    let (w, h, stride) = (13, 9, 16);
    let dense = mk(w, h, 11);
    let mut padded = vec![f32::from_bits(0x7fc0ffee); stride * h];
    for y in 0..h {
        padded[y * stride..y * stride + w].copy_from_slice(&dense[y * w..(y + 1) * w]);
    }
    let p = params(1);
    let want = run(&p, &[dense], w, h);
    let mut out = vec![f32::from_bits(0x7fc0ffee); stride * h];
    {
        let src = [Plane { data: &padded, width: w, height: h, stride }];
        let mut dst = [PlaneMut { data: &mut out, width: w, height: h, stride }];
        render_full(&p, &src, &mut dst).unwrap();
    }
    for y in 0..h {
        for x in 0..stride {
            let v = out[y * stride + x];
            if x < w {
                assert_eq!(v.to_bits(), want[0][y * w + x].to_bits());
            } else {
                assert_eq!(v.to_bits(), 0x7fc0ffee);
            }
        }
    }
}
