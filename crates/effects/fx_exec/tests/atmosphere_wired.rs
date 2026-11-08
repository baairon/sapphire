//! Executor dispatch of the Glow atmosphere words: `append` through `run_any`.
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

fn img(planes: Vec<Vec<f32>>) -> Image {
    Image::from_planes(planes).unwrap()
}

fn out(o: Out) -> Image {
    o.into_image().unwrap()
}

#[test]
fn append_joins_planes_and_broadcasts_the_constant() {
    let (w, h) = (3usize, 2usize);
    let a = img(vec![(0..6).map(|i| i as f32).collect(), (0..6).map(|i| 10.0 + i as f32).collect()]);
    let r = out(run_any("append", 3, &[Arg::Img(&a), cst(&[0.1])], w * h, (w, h), &KParams::standin()).unwrap());
    assert_eq!(r.channels(), 3);
    assert_eq!(r.planes[0], a.planes[0]);
    assert_eq!(r.planes[1], a.planes[1]);
    assert_eq!(r.planes[2], vec![0.1f32; 6]);
}
