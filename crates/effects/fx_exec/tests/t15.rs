//! Hand tests of the newly wired classes comp_premult (crate comp_premult) and copy (crate copy_planes over the
//! plane service). Expected values are computed here with independent expressions (dyadic values, so every operation is exact), not by calling the crates.
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
fn planes(ch: usize, n: usize, f: impl Fn(usize, usize) -> f32) -> Image {
    Image::from_planes((0..ch).map(|k| (0..n).map(|i| f(k, i)).collect()).collect()).unwrap()
}
fn run(class: &str, args: &[Arg], w: usize, h: usize) -> Image {
    let c: Vec<i32> = args.iter().map(|a| a.count()).collect();
    let t = Tables::embedded().select_counts(class, &c);
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
/// premultiplied B over A with alpha t: t >= 1 -> b; t <= 0 -> a + b; else a * (1 - t) + b (exact for the dyadic values used here)
fn over(a: f32, b: f32, t: f32) -> f32 {
    if t >= 1.0 {
        b
    } else if t <= 0.0 {
        a + b
    } else {
        a * (1.0 - t) + b
    }
}

const TS: [f32; 8] = [1.0, 0.0, 0.25, -0.5, 2.0, 0.5, 0.75, -0.0];

#[test]
fn comp_premult_rgba_over_rgba() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (4, 2);
    let n = w * h;
    let a = planes(4, n, |k, i| (k + 1) as f32 * 0.5 + i as f32 * 0.25);
    let b = planes(4, n, |k, i| 2.0 - (k as f32) * 0.25 + i as f32 * 0.5);
    let t = planes(1, n, |_, i| TS[i]);
    let r = run("comp_premult", &[Arg::Img(&a), Arg::Img(&b), Arg::Img(&t)], w, h);
    assert_eq!(r.channels(), 4);
    for k in 0..4 {
        let want: Vec<f32> = (0..n).map(|i| over(a.planes[k][i], b.planes[k][i], TS[i])).collect();
        same(&r.planes[k], &want);
    }
}

#[test]
fn comp_premult_rgb_rows_and_alpha_fill() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (4, 2);
    let n = w * h;
    let a3 = planes(3, n, |k, i| (k + 1) as f32 + i as f32);
    let b3 = planes(3, n, |k, i| 0.5 * (k + 1) as f32 - 0.25 * i as f32);
    let b4 = planes(4, n, |k, i| 0.5 * (k + 1) as f32 - 0.25 * i as f32);
    let t = planes(1, n, |_, i| TS[i]);
    // (3,3,1) -> 3 channels
    let r = run("comp_premult", &[Arg::Img(&a3), Arg::Img(&b3), Arg::Img(&t)], w, h);
    assert_eq!(r.channels(), 3);
    for k in 0..3 {
        let want: Vec<f32> = (0..n).map(|i| over(a3.planes[k][i], b3.planes[k][i], TS[i])).collect();
        same(&r.planes[k], &want);
    }
    // (3,4,1) -> 4 channels, A has 3 channels: output plane 3 is 1.0
    let r = run("comp_premult", &[Arg::Img(&a3), Arg::Img(&b4), Arg::Img(&t)], w, h);
    assert_eq!(r.channels(), 4);
    same(&r.planes[3], &vec![1.0; n]);
    for k in 0..3 {
        let want: Vec<f32> = (0..n).map(|i| over(a3.planes[k][i], b4.planes[k][i], TS[i])).collect();
        same(&r.planes[k], &want);
    }
}

#[test]
fn comp_premult_constants_and_int_round_trip() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (3, 2);
    let n = w * h;
    // all constants: fma(2.5, 1 - 0.25, 1.75) = 3.625, truncated through int32 -> 3
    let r = run("comp_premult", &[cst(&[2.5]), cst(&[1.75]), cst(&[0.25])], w, h);
    assert_eq!(r.channels(), 1);
    same(&r.planes[0], &vec![3.0; n]);
    // NaN through the int round trip is the integer indefinite
    let r = run("comp_premult", &[cst(&[1.0]), cst(&[f32::NAN]), cst(&[0.0])], w, h);
    same(&r.planes[0], &vec![-2147483648.0; n]);
}

#[test]
fn copy_copies_each_channel_and_fills_constants() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (w, h) = (4, 2);
    let n = w * h;
    for c in 1..=4usize {
        let src = planes(c, n, |k, i| 10.0 * (k + 1) as f32 + i as f32);
        let r = run("copy", &[Arg::Img(&src)], w, h);
        assert_eq!(r.channels(), c);
        for k in 0..c {
            same(&r.planes[k], &src.planes[k]);
        }
    }
    let r = run("copy", &[cst(&[0.5, 0.25, 0.125])], w, h);
    assert_eq!(r.channels(), 3);
    same(&r.planes[0], &vec![0.5; n]);
    same(&r.planes[1], &vec![0.25; n]);
    same(&r.planes[2], &vec![0.125; n]);
}
