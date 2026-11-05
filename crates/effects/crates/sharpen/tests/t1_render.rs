use fx_common::plane::{gen_checker, gen_flat, gen_grad, gen_noise, FxError, Plane, PlaneMut};
use fx_common::png::{gray_from_f32, write_gray8};
use sharpen::combine::combine;
use sharpen::plan::radius;
use sharpen::{render, sharpen_plane, Params, BASE_PHASE};
use std::path::Path;

fn params(amount: f32, gain: f32, w: usize) -> Params {
    Params { amount, gains: vec![gain], in_count: 1, ctx_width: w as i32, aspect: None, count: None }
}
fn run(p: &Params, x: &[f32], w: usize, h: usize) -> Vec<f32> {
    let mut out = vec![f32::from_bits(0x7fc0ffee); w * h];
    render(p, &[Plane::new(x, w, h, w).unwrap()], &mut [PlaneMut::new(&mut out, w, h, w).unwrap()]).unwrap();
    out
}
fn fnv(v: &[f32]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for x in v {
        for b in x.to_bits().to_le_bytes() {
            h ^= b as u64;
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

#[test]
fn radius_zero_or_gain_zero_is_copy() {
    let (w, h) = (37, 19);
    let x = gen_noise(w, h, 1000);
    for p in [params(0.0, 1.0, w), params(-1.0, 1.0, w), params(0.5, 0.0, w)] {
        assert_eq!(run(&p, &x, w, h).iter().map(|v| v.to_bits()).collect::<Vec<_>>(), x.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    }
}

#[test]
fn render_equals_manual_pipeline() {
    let (w, h) = (64, 64);
    let x = gen_noise(w, h, 1001);
    let p = params(0.3, 1.5, w);
    let got = run(&p, &x, w, h);
    let r = radius(0.3, 64);
    let mut b = vec![0.0f32; w * h];
    blur::blur_core(&x, &mut b, w, h, r, r, 1.0, blur::Edge::Mirror, 3, false, None).unwrap();
    let mut want = b.clone();
    combine(&x, &b, 1.5, &mut want, w * h).unwrap();
    assert_eq!(got.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), want.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    assert_eq!(BASE_PHASE, 0);
}

#[test]
fn constant_plane_stays_constant() {
    let (w, h) = (16, 16);
    let x = gen_flat(w, h, 1.0);
    let got = run(&params(0.4, 1.0, w), &x, w, h);
    for v in &got {
        assert!((v - 1.0).abs() <= 4.0 * f32::EPSILON, "{}", v);
    }
}

#[test]
fn finite_in_finite_out_and_deterministic() {
    let (w, h) = (48, 20);
    let x = gen_noise(w, h, 5);
    let a = run(&params(0.25, 2.0, w), &x, w, h);
    let b = run(&params(0.25, 2.0, w), &x, w, h);
    assert!(a.iter().all(|v| v.is_finite()));
    assert_eq!(fnv(&a), fnv(&b));
}

#[test]
fn errors_and_planes() {
    let x = vec![0.5f32; 16];
    let mut o = vec![0.0f32; 16];
    let p = params(0.5, 1.0, 4);
    assert_eq!(render(&Params { in_count: 0, ..p.clone() }, &[], &mut []), Ok(()));
    let mut two = Params { in_count: 2, ..p.clone() };
    two.gains = vec![1.0, 1.0];
    assert_eq!(render(&two, &[Plane::new(&x, 4, 4, 4).unwrap()], &mut [PlaneMut::new(&mut o, 4, 4, 4).unwrap()]), Err(FxError::PlaneCount));
    let short = Params { gains: vec![], ..p.clone() };
    assert_eq!(render(&short, &[Plane::new(&x, 4, 4, 4).unwrap()], &mut [PlaneMut::new(&mut o, 4, 4, 4).unwrap()]), Err(FxError::PlaneCount));
    let big = Params { count: Some(17), ..p.clone() };
    assert_eq!(render(&big, &[Plane::new(&x, 4, 4, 4).unwrap()], &mut [PlaneMut::new(&mut o, 4, 4, 4).unwrap()]), Err(FxError::Geometry));
    // two planes use per-plane gains
    let (w, h) = (16, 8);
    let x0 = gen_noise(w, h, 1);
    let x1 = gen_noise(w, h, 2);
    let (mut o0, mut o1) = (vec![0.0f32; w * h], vec![0.0f32; w * h]);
    let pp = Params { amount: 0.5, gains: vec![0.5, 2.0], in_count: 2, ctx_width: w as i32, aspect: None, count: None };
    render(&pp, &[Plane::new(&x0, w, h, w).unwrap(), Plane::new(&x1, w, h, w).unwrap()], &mut [PlaneMut::new(&mut o0, w, h, w).unwrap(), PlaneMut::new(&mut o1, w, h, w).unwrap()]).unwrap();
    let r = radius(0.5, w as i32);
    assert_eq!(o0, sharpen_plane(&x0, w, h, r, 0.5, None, w * h, 0).unwrap());
    assert_eq!(o1, sharpen_plane(&x1, w, h, r, 2.0, None, w * h, 0).unwrap());
}

#[test]
fn strided_equals_dense_and_padding_untouched() {
    let (w, h, s) = (13, 7, 16);
    let xd = gen_noise(w, h, 3);
    let mut xs = vec![9.0f32; s * h];
    for y in 0..h {
        xs[y * s..y * s + w].copy_from_slice(&xd[y * w..(y + 1) * w]);
    }
    let dense = run(&params(0.4, 1.3, w), &xd, w, h);
    let mut os = vec![-7.0f32; s * h];
    render(&params(0.4, 1.3, w), &[Plane::new(&xs, w, h, s).unwrap()], &mut [PlaneMut::new(&mut os, w, h, s).unwrap()]).unwrap();
    for y in 0..h {
        assert_eq!(&os[y * s..y * s + w], &dense[y * w..(y + 1) * w]);
        assert!(os[y * s + w..(y + 1) * s].iter().all(|v| *v == -7.0));
    }
}

/// previews and the T3 hash of the previews (input panel left, sharpened panel right)
#[test]
fn previews() {
    let out = effects_vecdir::out_dir();
    let dir = out.as_path();
    std::fs::create_dir_all(dir).unwrap();
    let mut all = Vec::new();
    for (name, w, h, x) in [
        ("grad_64x64", 64usize, 64usize, gen_grad(64, 64)),
        ("noise_37x19", 37, 19, gen_noise(37, 19, 1000)),
        ("checker_64x64", 64, 64, gen_checker(64, 64, 8)),
    ] {
        let y = run(&params(0.3, 1.5, w), &x, w, h);
        let mut pair = vec![0.0f32; 2 * w * h];
        for r in 0..h {
            pair[r * 2 * w..r * 2 * w + w].copy_from_slice(&x[r * w..(r + 1) * w]);
            pair[r * 2 * w + w..(r + 1) * 2 * w].copy_from_slice(&y[r * w..(r + 1) * w]);
        }
        let px = gray_from_f32(&pair, 2 * w, h, 2 * w, -0.25, 1.25);
        write_gray8(&dir.join(format!("{}_sharpen_amount0p3_gain1p5.png", name)), (2 * w) as u32, h as u32, &px).unwrap();
        all.extend_from_slice(&y);
    }
    println!("T3 preview outputs FNV64 {:016x}", fnv(&all));
}
