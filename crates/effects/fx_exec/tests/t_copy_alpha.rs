//! Hand tests of the wired class copy_alpha (crate copy_alpha). Expected values are written out here, not computed by the crate.
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
fn planes(ch: usize, n: usize, base: f32) -> Image {
    Image::from_planes((0..ch).map(|k| (0..n).map(|i| base + 10.0 * k as f32 + i as f32).collect()).collect()).unwrap()
}
fn run(tag: i32, args: &[Arg], w: usize, h: usize, kp: &KParams) -> Image {
    match run_any("copy_alpha", tag, args, w * h, (w, h), kp).unwrap() {
        Out::Img(i) => i,
        _ => panic!(),
    }
}
fn bits(p: &[f32]) -> Vec<u32> {
    p.iter().map(|x| x.to_bits()).collect()
}

#[test]
fn colour_from_the_first_operand_alpha_from_the_second() {
    let (w, h) = (3, 2);
    let n = w * h;
    let a = planes(4, n, 0.0);
    let b = planes(4, n, 100.0);
    let r = run(4, &[Arg::Img(&a), Arg::Img(&b)], w, h, &KParams::standin());
    assert_eq!(r.channels(), 4);
    for k in 0..3 {
        assert_eq!(bits(&r.planes[k]), bits(&a.planes[k]));
    }
    assert_eq!(bits(&r.planes[3]), bits(&b.planes[3]));
    // two channels: channel 1 of the second operand is the alpha
    let a2 = planes(2, n, 0.0);
    let b2 = planes(2, n, 100.0);
    let r = run(2, &[Arg::Img(&a2), Arg::Img(&b2)], w, h, &KParams::standin());
    assert_eq!(bits(&r.planes[0]), bits(&a2.planes[0]));
    assert_eq!(bits(&r.planes[1]), bits(&b2.planes[1]));
    // three channels: no alpha path, all channels from the first operand
    let a3 = planes(3, n, 0.0);
    let r = run(3, &[Arg::Img(&a3), Arg::Img(&b)], w, h, &KParams::standin());
    for k in 0..3 {
        assert_eq!(bits(&r.planes[k]), bits(&a3.planes[k]));
    }
}

#[test]
fn constants_fill_and_the_channel_is_clamped() {
    let (w, h) = (2, 2);
    let a = cst(&[0.5, 0.25]);
    let b = cst(&[8.0]);
    let r = run(4, &[a, b], w, h, &KParams::standin());
    assert_eq!(bits(&r.planes[0]), vec![0.5f32.to_bits(); 4]);
    assert_eq!(bits(&r.planes[1]), vec![0.25f32.to_bits(); 4]);
    assert_eq!(bits(&r.planes[2]), vec![0.25f32.to_bits(); 4]);
    assert_eq!(bits(&r.planes[3]), vec![8.0f32.to_bits(); 4]);
}

#[test]
fn large_constant_fills_clear_negative_zero_only_outside_a_pool_worker() {
    let (w, h) = (32, 32);
    let n = w * h;
    let a = cst(&[-0.0]);
    let b = cst(&[-0.0]);
    let r = run(4, &[a, b], w, h, &KParams::standin());
    assert!(r.planes.iter().all(|p| p.iter().all(|x| x.to_bits() == 0)));
    let mut kp = KParams::standin();
    kp.in_pool = true;
    let r = run(4, &[cst(&[-0.0]), cst(&[-0.0])], w, h, &kp);
    assert!(r.planes.iter().all(|p| p.iter().all(|x| x.to_bits() == 0x8000_0000)));
    // below the pool threshold the sign is kept
    let r = run(4, &[cst(&[-0.0]), cst(&[-0.0])], 22, 22, &KParams::standin());
    assert!(r.planes.iter().all(|p| p.iter().all(|x| x.to_bits() == 0x8000_0000)));
    assert_eq!(n, 1024);
}

#[test]
fn wrong_argument_count_is_refused() {
    assert!(run_any("copy_alpha", 4, &[cst(&[1.0])], 4, (2, 2), &KParams::standin()).is_err());
}
