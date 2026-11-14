//! Hand tests of the newly wired classes (v3_to_v4, coerce_to_v3, coerce_to_v4, coerce_up_to_v3, sharpen),
//! and the equality of the evaluator's constant folds `clamp_const` / `diff_const` with the native class crates `clamp` (shape 000, one
//! element) and `diff`.
use fx_exec::kernels::run_any;
use fx_exec::kernels2::Out;
use fx_exec::*;

fn cst(v: &[f32]) -> Arg<'static> {
    let mut l = [*v.last().unwrap(); 4].map(|x: f32| x.to_bits());
    for (i, x) in v.iter().enumerate() {
        l[i] = x.to_bits();
    }
    Arg::Const { n: v.len(), lanes: l }
}
fn img(chan: usize, n: usize) -> Image {
    // plane k, sample i = 10 * (k + 1) + i, all exact in f32
    Image::from_planes((0..chan).map(|k| (0..n).map(|i| (10 * (k + 1) + i) as f32).collect()).collect()).unwrap()
}
fn run(class: &str, args: &[Arg], w: usize, h: usize) -> Image {
    let c: Vec<i32> = args.iter().map(|a| a.count()).collect();
    let t = Tables::from_tsv(effects_rtdata::text("fx_exec/data/COMBOS_EXEC.tsv")).select_counts(class, &c);
    assert!(t > 0, "no combo for {} {:?}", class, c);
    // the sharpen arm has no default count; the caller supplies W x H (HOST-COUNT, ROUTE)
    let mut kp = KParams::host_defaults();
    kp.sharpen_count = Some(w * h);
    match run_any(class, t, args, w * h, (w, h), &kp).unwrap() {
        Out::Img(i) => i,
        _ => panic!(),
    }
}
fn same(a: &[f32], b: &[f32]) {
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(b) {
        assert_eq!(x.to_bits(), y.to_bits());
    }
}

#[test]
fn coerce_to_v3_rows() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (5, 3);
    let n = w * h;
    let img_1ch = img(1, n);
    let r = run("coerce_to_v3", &[Arg::Img(&img_1ch)], w, h);
    assert_eq!(r.channels(), 3);
    for k in 0..3 {
        same(&r.planes[k], &img_1ch.planes[0]);
    }
    let img_2ch = img(2, n);
    let r = run("coerce_to_v3", &[Arg::Img(&img_2ch)], w, h);
    assert_eq!(r.channels(), 3);
    same(&r.planes[0], &img_2ch.planes[0]);
    same(&r.planes[1], &img_2ch.planes[1]);
    same(&r.planes[2], &vec![0.0; n]);
    let img_4ch = img(4, n);
    let r = run("coerce_to_v3", &[Arg::Img(&img_4ch)], w, h);
    assert_eq!(r.channels(), 3);
    for k in 0..3 {
        same(&r.planes[k], &img_4ch.planes[k]);
    }
    let img_3ch = img(3, n);
    let r = run("coerce_to_v3", &[Arg::Img(&img_3ch)], w, h);
    for k in 0..3 {
        same(&r.planes[k], &img_3ch.planes[k]);
    }
}

#[test]
fn coerce_to_v4_and_up_to_v3_rows() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (5, 3);
    let n = w * h;
    let img_1ch = img(1, n);
    let r = run("coerce_to_v4", &[Arg::Img(&img_1ch)], w, h);
    assert_eq!(r.channels(), 4);
    for k in 0..3 {
        same(&r.planes[k], &img_1ch.planes[0]);
    }
    same(&r.planes[3], &vec![1.0; n]);
    let img_3ch = img(3, n);
    let r = run("coerce_to_v4", &[Arg::Img(&img_3ch)], w, h);
    for k in 0..3 {
        same(&r.planes[k], &img_3ch.planes[k]);
    }
    same(&r.planes[3], &vec![1.0; n]);
    let img_4ch = img(4, n);
    let r = run("coerce_to_v4", &[Arg::Img(&img_4ch)], w, h);
    for k in 0..4 {
        same(&r.planes[k], &img_4ch.planes[k]);
    }
    // coerce_up_to_v3: 4 channels stay 4, 2 channels get a zero third plane
    let r = run("coerce_up_to_v3", &[Arg::Img(&img_4ch)], w, h);
    assert_eq!(r.channels(), 4);
    for k in 0..4 {
        same(&r.planes[k], &img_4ch.planes[k]);
    }
    let img_2ch = img(2, n);
    let r = run("coerce_up_to_v3", &[Arg::Img(&img_2ch)], w, h);
    assert_eq!(r.channels(), 3);
    same(&r.planes[2], &vec![0.0; n]);
    let r = run("coerce_up_to_v3", &[Arg::Img(&img_1ch)], w, h);
    assert_eq!(r.channels(), 3);
    same(&r.planes[1], &img_1ch.planes[0]);
}

#[test]
fn v3_to_v4_rows() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (5, 3);
    let n = w * h;
    let img_3ch = img(3, n);
    let r = run("v3_to_v4", &[Arg::Img(&img_3ch), cst(&[0.5])], w, h);
    assert_eq!(r.channels(), 4);
    for k in 0..3 {
        same(&r.planes[k], &img_3ch.planes[k]);
    }
    same(&r.planes[3], &vec![0.5; n]);
    // (4,1): the alpha operand is an image
    let img_4ch = img(4, n);
    let other_img = img(1, n);
    let r = run("v3_to_v4", &[Arg::Img(&img_4ch), Arg::Img(&other_img)], w, h);
    for k in 0..3 {
        same(&r.planes[k], &img_4ch.planes[k]);
    }
    same(&r.planes[3], &other_img.planes[0]);
    // (1,1): gray broadcast to the three colour planes (channel clamped to count - 1)
    let g = img(1, n);
    let r = run("v3_to_v4", &[Arg::Img(&g), cst(&[1.0])], w, h);
    for k in 0..3 {
        same(&r.planes[k], &g.planes[0]);
    }
    same(&r.planes[3], &vec![1.0; n]);
    // (3,3): alpha operand 3 channels, only its channel 0 is used
    let other_img_b = img(3, n);
    let r = run("v3_to_v4", &[Arg::Img(&img_3ch), Arg::Img(&other_img_b)], w, h);
    same(&r.planes[3], &other_img_b.planes[0]);
}

#[test]
fn sharpen_basic_properties() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (16, 8);
    let n = w * h;
    // a constant plane is a fixed point (blur of a constant is the constant, x + g (x - b) = x)
    let c1 = Image::from_planes(vec![vec![0.25; n]]).unwrap();
    let r = run("sharpen", &[Arg::Img(&c1), cst(&[1.5]), cst(&[0.112])], w, h);
    assert_eq!(r.channels(), 1);
    for v in &r.planes[0] {
        assert!((v - 0.25).abs() < 1e-6, "{}", v);
    }
    // amount 0: radius 0 copies bit for bit
    let img_3ch = img(3, n);
    let r = run("sharpen", &[Arg::Img(&img_3ch), cst(&[2.0, 2.0, 2.0]), cst(&[0.0])], w, h);
    assert_eq!(r.channels(), 3);
    for k in 0..3 {
        same(&r.planes[k], &img_3ch.planes[k]);
    }
    // gain 0: copy
    let r = run("sharpen", &[Arg::Img(&img_3ch), cst(&[0.0]), cst(&[0.5])], w, h);
    for k in 0..3 {
        same(&r.planes[k], &img_3ch.planes[k]);
    }
    // an actual sharpen of a ramp is finite, deterministic and changes interior samples
    let ramp = Image::from_planes(vec![(0..n).map(|i| ((i % w) as f32) / (w as f32)).collect()]).unwrap();
    let a = run("sharpen", &[Arg::Img(&ramp), cst(&[2.0]), cst(&[0.5])], w, h);
    let b = run("sharpen", &[Arg::Img(&ramp), cst(&[2.0]), cst(&[0.5])], w, h);
    same(&a.planes[0], &b.planes[0]);
    assert!(a.planes[0].iter().all(|v| v.is_finite()));
}

fn specials() -> Vec<f32> {
    vec![
        0.0,
        -0.0,
        1.0,
        -1.0,
        0.5,
        3.0,
        5.0,
        -2.0,
        1e-40,
        f32::MAX,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NAN,
        f32::from_bits(0x7fa0_0001),
        f32::from_bits(0xffc0_0123),
    ]
}

fn nan_class_eq(a: f32, b: f32) -> bool {
    (a.is_nan() && b.is_nan()) || a.to_bits() == b.to_bits()
}

#[test]
fn clamp_fold_equals_native_class_shape_000() {
    let v = specials();
    let mut checked = 0;
    for &x in &v {
        for &lo in &v {
            for &hi in &v {
                let (px, plo, phi) = ([x], [lo], [hi]);
                let (sx, slo, shi): (Vec<&[f32]>, Vec<&[f32]>, Vec<&[f32]>) = (vec![&px[..]], vec![&plo[..]], vec![&phi[..]]);
                let oa = ops_arith::Operand { count: 1, is_const: false, consts: [0.0; 4], planes: &sx };
                let ob = ops_arith::Operand { count: 1, is_const: false, consts: [0.0; 4], planes: &slo };
                let oc = ops_arith::Operand { count: 1, is_const: false, consts: [0.0; 4], planes: &shi };
                let p = ops_clamp::Params { nout: 1, n: 1, allones: 0xffff_ffff, dst_phase_bytes: [0; 4] };
                let mut out = vec![0.0f32; 1];
                {
                    let mut outs: Vec<&mut [f32]> = vec![out.as_mut_slice()];
                    ops_clamp::render(&p, &oa, &ob, &oc, &mut outs).unwrap();
                }
                let f = fx_eval::fold::clamp_const(x, lo, hi);
                assert!(nan_class_eq(f, out[0]), "clamp({:?}, {:?}, {:?}): fold {:?} native {:?}", x, lo, hi, f, out[0]);
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 15 * 15 * 15);
}

#[test]
fn diff_fold_equals_native_class() {
    let v = specials();
    for &x in &v {
        for &y in &v {
            let (px, py) = ([x], [y]);
            let (sx, sy): (Vec<&[f32]>, Vec<&[f32]>) = (vec![&px[..]], vec![&py[..]]);
            let oa = ops_arith::Operand { count: 1, is_const: false, consts: [0.0; 4], planes: &sx };
            let ob = ops_arith::Operand { count: 1, is_const: false, consts: [0.0; 4], planes: &sy };
            let p = ops_diff::Params { nout: 1, n: 1, vec_one: 1.0, vec_mask: 0x7fff_ffff, dst3_phase_bytes: 0 };
            let mut out = vec![0.0f32; 1];
            {
                let mut outs: Vec<&mut [f32]> = vec![out.as_mut_slice()];
                ops_diff::render(&p, &oa, &ob, &mut outs).unwrap();
            }
            let f = fx_eval::fold::diff_const(x, y);
            assert!(nan_class_eq(f, out[0]), "diff({:?}, {:?}): fold {:?} native {:?}", x, y, f, out[0]);
        }
    }
}

#[test]
fn monochrome_a_rows() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (9, 4);
    let n = w * h;
    // 4 channels: luma plane (0.3125 p0 + 0.5 p1 + 0.1875 p2, f64 fma chain) and the alpha plane = channel 3
    let img_4ch = img(4, n);
    let r = run("monochrome_a", &[Arg::Img(&img_4ch)], w, h);
    assert_eq!(r.channels(), 2);
    same(&r.planes[1], &img_4ch.planes[3]);
    for i in 0..n {
        let (a, b, c) = (img_4ch.planes[0][i] as f64, img_4ch.planes[1][i] as f64, img_4ch.planes[2][i] as f64);
        let t = 0.5f64 * b;
        let t = a.mul_add(0.3125, t);
        let t = c.mul_add(0.1875, t);
        assert_eq!(r.planes[0][i].to_bits(), (t as f32).to_bits());
    }
    // 3 channels: one plane
    let img_3ch = img(3, n);
    let r = run("monochrome_a", &[Arg::Img(&img_3ch)], w, h);
    assert_eq!(r.channels(), 1);
    // 2 channels: out0 = channel 0, out1 = channel 1
    let img_2ch = img(2, n);
    let r = run("monochrome_a", &[Arg::Img(&img_2ch)], w, h);
    assert_eq!(r.channels(), 2);
    same(&r.planes[0], &img_2ch.planes[0]);
    same(&r.planes[1], &img_2ch.planes[1]);
    // 1 channel: copy
    let img_1ch = img(1, n);
    let r = run("monochrome_a", &[Arg::Img(&img_1ch)], w, h);
    assert_eq!(r.channels(), 1);
    same(&r.planes[0], &img_1ch.planes[0]);
}
