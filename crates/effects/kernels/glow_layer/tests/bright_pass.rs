use glow_layer::*;

fn lcg(seed: &mut u64) -> f32 {
    *seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    ((*seed >> 40) as f32 / 16777216.0) * 2.0 - 0.5
}
fn mk(w: usize, h: usize, seed: u64) -> Vec<f32> {
    let mut s = seed;
    (0..w * h).map(|_| lcg(&mut s)).collect()
}
fn run(p: &BrightParams, planes: &[Vec<f32>], w: usize, h: usize) -> Vec<Vec<f32>> {
    let src: Vec<Plane> = planes.iter().map(|d| Plane { data: d, width: w, height: h, stride: w }).collect();
    let mut out: Vec<Vec<f32>> = planes.iter().map(|_| vec![9.0; w * h]).collect();
    let mut dst: Vec<PlaneMut> = out.iter_mut().map(|d| PlaneMut { data: d, width: w, height: h, stride: w }).collect();
    bright_pass(p, &src, &mut dst).unwrap();
    out
}

#[test]
fn t1_known_one_plane() {
    // th 0.5: k = 1/(1-0.5) = 2.
    let p = BrightParams { thresholds: [0.5, 0.0, 0.0] };
    let o = run(&p, &[vec![0.75, 0.5, 0.6, -1.0, 1.0, 0.0, 0.5000001, 2.0, 3.0]], 9, 1);
    assert_eq!(o[0][0], 0.5); // d = 2*0.25 = 0.5 < 0.75
    assert_eq!(o[0][1], 0.0); // x == th
    assert_eq!(o[0][2].to_bits(), (2.0f32 * (0.6f32 - 0.5f32)).to_bits());
    assert_eq!(o[0][3], 0.0);
    assert_eq!(o[0][4], 1.0); // d = 1.0, x > d false -> x kept
    assert_eq!(o[0][5], 0.0);
    assert_eq!(o[0][7], 2.0); // d = 3.0 > x, kept
    assert_eq!(o[0][8], 3.0); // d = 5.0, kept
}
#[test]
fn t1_known_three_plane() {
    // th 0.5 each, k = 2.
    let p = BrightParams { thresholds: [0.5; 3] };
    let o = run(&p, &[vec![1.0, 2.0, 0.4], vec![1.0, 0.6, 0.4], vec![1.0, 0.1, 0.4]], 3, 1);
    assert_eq!((o[0][0], o[1][0], o[2][0]), (1.0, 1.0, 1.0)); // E = 1.5, S = 3, t = 3, kept
    assert_eq!((o[0][1], o[1][1], o[2][1]), (2.0, 0.6, 0.1)); // E = 1.6, S = 2.7, t = 3.2, kept
    assert_eq!((o[0][2], o[1][2], o[2][2]), (0.0, 0.0, 0.0)); // E = 0 -> zero
    // th = [0, 0, 0.9]: m = 0, k = 1. pixel (1, 1, 0.5): E = 2, S = 2.5, t = 2 -> scaled by 2/2.5
    let p2 = BrightParams { thresholds: [0.0, 0.0, 0.9] };
    let o2 = run(&p2, &[vec![1.0], vec![1.0], vec![0.5]], 1, 1);
    let f = 2.0f32 / 2.5f32;
    assert_eq!((o2[0][0], o2[1][0], o2[2][0]), (1.0 * f, f * 1.0, f * 0.5));
}
#[test]
fn t1_noop_when_thresholds_zero() {
    let src = mk(8, 3, 5);
    let o = run(&BrightParams { thresholds: [0.0, 5.0, 5.0] }, &[src.clone()], 8, 3);
    assert_eq!(o[0], src, "one plane reads only th0; th0 == 0 returns early");
    let o3 = run(&BrightParams { thresholds: [0.0; 3] }, &[src.clone(), src.clone(), src.clone()], 8, 3);
    assert!(o3.iter().all(|p| *p == src));
    let on = run(&BrightParams { thresholds: [-0.0, 0.0, 0.0] }, &[src.clone(), src.clone(), src.clone()], 8, 3);
    assert!(on.iter().all(|p| *p == src));
}
#[test]
fn t1_helpers_and_errors() {
    assert_eq!(x86::minss(f32::NAN, 1.0), 1.0);
    assert!(x86::minss(1.0, f32::NAN).is_nan());
    assert_eq!(x86::maxss(2.0, 2.0).to_bits(), 2.0f32.to_bits());
    let d = vec![0.0f32; 4];
    let s = [Plane { data: &d, width: 2, height: 2, stride: 2 }; 2];
    let mut o = vec![0.0f32; 4];
    let mut m = [PlaneMut { data: &mut o, width: 2, height: 2, stride: 2 }];
    assert_eq!(bright_pass(&BrightParams { thresholds: [0.5; 3] }, &s[..2], &mut m), Err(FxError::PlaneCount));
}
#[test]
fn t3_determinism_and_finite() {
    let (w, h) = (37, 19);
    let planes = vec![mk(w, h, 1), mk(w, h, 2), mk(w, h, 3)];
    let p = BrightParams { thresholds: [0.3, 0.2, 0.4] };
    let a = run(&p, &planes, w, h);
    assert_eq!(a, run(&p, &planes, w, h));
    assert!(a.iter().all(|pl| pl.iter().all(|v| v.is_finite())));
    let a1 = run(&p, &planes[..1], w, h);
    assert!(a1[0].iter().all(|v| v.is_finite()));
}
#[test]
fn t3_all_below_threshold() {
    let (w, h) = (16, 4);
    let low = vec![0.25f32; w * h];
    let o = run(&BrightParams { thresholds: [0.5; 3] }, &[low.clone()], w, h);
    assert!(o[0].iter().all(|v| v.to_bits() == 0));
    let o3 = run(&BrightParams { thresholds: [0.5; 3] }, &[low.clone(), low.clone(), low.clone()], w, h);
    assert!(o3.iter().all(|p| p.iter().all(|v| v.to_bits() == 0)));
}
#[test]
fn t3_strided_padding_untouched() {
    let (w, h, st) = (5, 3, 8);
    let src: Vec<f32> = (0..st * h).map(|i| i as f32 * 0.1).collect();
    let mut dst = vec![-7.0f32; st * h];
    let s = [Plane { data: &src, width: w, height: h, stride: st }];
    let mut d = [PlaneMut { data: &mut dst, width: w, height: h, stride: st }];
    bright_pass(&BrightParams { thresholds: [0.5; 3] }, &s, &mut d).unwrap();
    for y in 0..h {
        for x in w..st {
            assert_eq!(dst[y * st + x], -7.0);
        }
    }
}

/// A signalling or quiet NaN red sample on the three plane path: the clamp `maxss(r, 0.0)` returns its second operand, 0.0, on every
/// processor (a hardware maximum with number semantics would return a quiet NaN for a signalling first operand and zero the pixel).
#[test]
fn t1_three_plane_nan_red_sample() {
    let p = BrightParams { thresholds: [0.9, 0.0, 0.0] };
    for r in [0x7f80_0001u32, 0xff80_0005, 0x7fa0_0000, 0x7fc0_0000, 0xffc0_1234] {
        for b in [1.0f32, 0.5, f32::INFINITY] {
            let o = run(&p, &[vec![f32::from_bits(r)], vec![0.0], vec![b]], 1, 1);
            assert_eq!((o[0][0].to_bits(), o[1][0].to_bits(), o[2][0].to_bits()), (r, 0, b.to_bits()), "r {r:08x} b {b}");
        }
    }
}
/// The helpers with a signalling NaN first operand and a constant second operand give the second operand.
#[test]
fn t1_helpers_signalling_nan_first_operand() {
    for w in [0x7f80_0001u32, 0xff80_0005, 0x7fbf_ffff] {
        let s = std::hint::black_box(f32::from_bits(w));
        assert_eq!(x86::maxss(s, 0.0).to_bits(), 0.0f32.to_bits(), "maxss {w:08x}");
        assert_eq!(x86::maxss(s, 1.5).to_bits(), 1.5f32.to_bits(), "maxss {w:08x}");
        assert_eq!(x86::minss(s, 1.0).to_bits(), 1.0f32.to_bits(), "minss {w:08x}");
        assert_eq!(x86::minss(s, 0.0).to_bits(), 0.0f32.to_bits(), "minss {w:08x}");
    }
}
