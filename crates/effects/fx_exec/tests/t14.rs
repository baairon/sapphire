//! Hand tests of the newly wired classes remove_alpha (pass C_CELL_A, rows 1 2 3 4 -> 1 1 3 3), round (scale_rgb crate,
//! AVX2 body 0x4dab50) and scale_rgb (slot 53 0x4d5050 with its host pad call through the plane service). Expected values are computed here
//! with independent expressions, not by calling the crates.
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
fn img(ch: usize, n: usize) -> Image {
    // plane k, sample i = 10 * (k + 1) + i, all exact in f32
    Image::from_planes((0..ch).map(|k| (0..n).map(|i| (10 * (k + 1) + i) as f32).collect()).collect()).unwrap()
}
fn run(class: &str, args: &[Arg], w: usize, h: usize) -> Image {
    let c: Vec<i32> = args.iter().map(|a| a.count()).collect();
    let t = Tables::from_tsv(effects_rtdata::text("fx_exec/data/COMBOS_EXEC.tsv")).select_counts(class, &c);
    assert!(t > 0, "no combo for {} {:?}", class, c);
    match run_any(class, t, args, w * h, (w, h), &KParams::standin()).unwrap() {
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
/// round(x, y) for x >= 0, y != 0: y * floor(x / y) in double, rounded to f32 (listing 0x4dac91, hand formula)
fn snap_pos(x: f32, y: f32) -> f32 {
    ((y as f64) * ((x / y).floor() as f64)) as f32
}

#[test]
fn remove_alpha_rows() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (5, 3);
    let n = w * h;
    for (c, want) in [(1usize, 1usize), (2, 1), (3, 3), (4, 3)] {
        let i = img(c, n);
        let r = run("remove_alpha", &[Arg::Img(&i)], w, h);
        assert_eq!(r.channels(), want, "{} channels", c);
        for k in 0..want {
            same(&r.planes[k], &i.planes[k]);
        }
    }
    // a constant operand: the lanes are filled (4 lanes -> 3 results)
    let r = run("remove_alpha", &[cst(&[0.5, 0.25, 0.125, 0.75])], w, h);
    assert_eq!(r.channels(), 3);
    same(&r.planes[0], &vec![0.5; n]);
    same(&r.planes[1], &vec![0.25; n]);
    same(&r.planes[2], &vec![0.125; n]);
}

#[test]
fn round_image_and_constant_shapes() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (5, 3);
    let n = w * h;
    // (1,1): image x = 10 + i, constant step 4
    let a = img(1, n);
    let r = run("round", &[Arg::Img(&a), cst(&[4.0])], w, h);
    assert_eq!(r.channels(), 1);
    let want: Vec<f32> = a.planes[0].iter().map(|&x| snap_pos(x, 4.0)).collect();
    same(&r.planes[0], &want);
    assert_eq!(want[0], 8.0);
    assert_eq!(want[2], 12.0);
    // (1,1) with the constant first: x = 25, step = image 10 + i
    let b = img(1, n);
    let r = run("round", &[cst(&[25.0]), Arg::Img(&b)], w, h);
    let want: Vec<f32> = b.planes[0].iter().map(|&y| snap_pos(25.0, y)).collect();
    same(&r.planes[0], &want);
    assert_eq!(want[0], 20.0);
    // y = 0 returns x unchanged
    let r = run("round", &[Arg::Img(&a), cst(&[0.0])], w, h);
    same(&r.planes[0], &a.planes[0]);
    // negative x: ceil(x / y) * y - |y|
    let neg = Image::from_planes(vec![(0..n).map(|i| -1.3 - i as f32).collect()]).unwrap();
    let r = run("round", &[Arg::Img(&neg), cst(&[0.5])], w, h);
    assert_eq!(r.planes[0][0], -1.5);
    assert_eq!(r.planes[0][1], -2.5);
}

#[test]
fn round_channel_shapes() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (5, 3);
    let n = w * h;
    // (3,3): per channel snap against the matching channel of the second image
    let a = img(3, n);
    let b = img(3, n);
    let r = run("round", &[Arg::Img(&a), Arg::Img(&b)], w, h);
    assert_eq!(r.channels(), 3);
    for k in 0..3 {
        let want: Vec<f32> = (0..n).map(|i| snap_pos(a.planes[k][i], b.planes[k][i])).collect();
        same(&r.planes[k], &want);
    }
    // (2,1): the second operand's channel 0 serves both channels (min(c, n_out - 1))
    let a2 = img(2, n);
    let r = run("round", &[Arg::Img(&a2), cst(&[3.0])], w, h);
    assert_eq!(r.channels(), 2);
    for k in 0..2 {
        let want: Vec<f32> = a2.planes[k].iter().map(|&x| snap_pos(x, 3.0)).collect();
        same(&r.planes[k], &want);
    }
    // (3,4): three snapped channels, then plane 3 = y * floor(1 / y) of the second operand's channel 3 (alpha synthesis)
    let b4 = Image::from_planes(vec![
        (0..n).map(|i| 1.0 + i as f32).collect(),
        (0..n).map(|i| 2.0 + i as f32).collect(),
        (0..n).map(|i| 3.0 + i as f32).collect(),
        vec![0.25; n],
    ])
    .unwrap();
    let r = run("round", &[Arg::Img(&a), Arg::Img(&b4)], w, h);
    assert_eq!(r.channels(), 4);
    for k in 0..3 {
        let want: Vec<f32> = (0..n).map(|i| snap_pos(a.planes[k][i], b4.planes[k][i])).collect();
        same(&r.planes[k], &want);
    }
    same(&r.planes[3], &vec![1.0; n]); // 0.25 * floor(4) = 1
    // (4,3): plane 3 = floor(x) of the first operand's channel 3 (alpha drop)
    let a4 = Image::from_planes(vec![vec![5.0; n], vec![6.0; n], vec![7.0; n], (0..n).map(|i| 2.5 + i as f32).collect()]).unwrap();
    let r = run("round", &[Arg::Img(&a4), Arg::Img(&b)], w, h);
    assert_eq!(r.channels(), 4);
    let want3: Vec<f32> = (0..n).map(|i| (2.5 + i as f32).floor()).collect();
    same(&r.planes[3], &want3);
}

#[test]
fn round_refuses_two_constants() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let args = [cst(&[1.5]), cst(&[0.5])];
    let t = Tables::from_tsv(effects_rtdata::text("fx_exec/data/COMBOS_EXEC.tsv")).select_counts("round", &[1, 1]);
    let e = run_any("round", t, &args, 15, (5, 3), &KParams::standin()).unwrap_err();
    assert!(matches!(e, ExecError::Unsupported { .. }));
}

#[test]
fn scale_rgb_shapes() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (5, 3);
    let n = w * h;
    // (3,3) both images: src * aux per channel
    let a = img(3, n);
    let b = img(3, n);
    let r = run("scale_rgb", &[Arg::Img(&a), Arg::Img(&b)], w, h);
    assert_eq!(r.channels(), 3);
    for k in 0..3 {
        let want: Vec<f32> = (0..n).map(|i| a.planes[k][i] * b.planes[k][i]).collect();
        same(&r.planes[k], &want);
    }
    // (3,3) with a constant gain (0.5, 2, 1) as the second operand
    let r = run("scale_rgb", &[Arg::Img(&a), cst(&[0.5, 2.0, 1.0])], w, h);
    let g = [0.5f32, 2.0, 1.0];
    for k in 0..3 {
        let want: Vec<f32> = a.planes[k].iter().map(|&x| g[k] * x).collect();
        same(&r.planes[k], &want);
    }
    // (4,3) tag 4: three products, plane 3 copied from the first operand's channel 3 by the host pad call (mode 0)
    let a4 = img(4, n);
    let r = run("scale_rgb", &[Arg::Img(&a4), Arg::Img(&b)], w, h);
    assert_eq!(r.channels(), 4);
    for k in 0..3 {
        let want: Vec<f32> = (0..n).map(|i| a4.planes[k][i] * b.planes[k][i]).collect();
        same(&r.planes[k], &want);
    }
    same(&r.planes[3], &a4.planes[3]);
    // (3,4) tag 4: plane 3 from the second operand's channel 3 (mode 1)
    let b4 = img(4, n);
    let r = run("scale_rgb", &[Arg::Img(&a), Arg::Img(&b4)], w, h);
    assert_eq!(r.channels(), 4);
    same(&r.planes[3], &b4.planes[3]);
}

/// IN7-A4: the evaluator's constant path of round (`snap_const`, `snap_one_over_const`, `snap_unit_const`) equals the crate's element kernels
/// over special values (NaN compared as a class, everything else bit for bit).
#[test]
fn round_fold_equals_the_crate_kernels() {
    let vals = [
        0.0f32, -0.0, 1.0, -1.0, 0.5, -0.5, 0.3, -0.3, 2.5, -2.5, 7.0, 1e-3, 1e30, -1e30, 1e-40, f32::INFINITY, f32::NEG_INFINITY, f32::NAN,
    ];
    let eq = |a: f32, b: f32| (a.is_nan() && b.is_nan()) || a.to_bits() == b.to_bits();
    let mut pairs = 0;
    for &x in &vals {
        for &y in &vals {
            assert!(eq(fx_eval::fold::snap_const(x, y), scale_rgb::snap(x, y)), "snap {:?} {:?}", x, y);
            pairs += 1;
        }
        assert!(eq(fx_eval::fold::snap_one_over_const(x), scale_rgb::snap_one_over(x)), "one_over {:?}", x);
        assert!(eq(fx_eval::fold::snap_unit_const(x), scale_rgb::snap_unit(x)), "unit {:?}", x);
    }
    assert_eq!(pairs, 324);
}
