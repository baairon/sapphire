//! unit test: blur_channels over a source with fewer planes than the combo tag. Planes at or beyond the source plane
//! count start as copies of the plane before them, so a one-plane source at tag 3 equals the same plane given three times.
use fx_exec::kernels2::{run_wired, Out};
use fx_exec::*;

fn lanes(v: [f32; 4]) -> [u32; 4] {
    [v[0].to_bits(), v[1].to_bits(), v[2].to_bits(), v[3].to_bits()]
}
fn plane(w: usize, h: usize) -> Vec<f32> {
    let mut s = 777u64;
    (0..w * h)
        .map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (s >> 33) as u32 as f32 / 2147483648.0
        })
        .collect()
}
fn run(src: &Image, tag: i32, w: usize, h: usize) -> Result<Vec<Vec<u32>>, ExecError> {
    let args = [
        Arg::Img(src),
        Arg::Const { n: 2, lanes: lanes([6.72, 6.72, 0.0, 0.0]) },
        Arg::Const { n: 3, lanes: lanes([1.0, 1.2, 1.39999, 0.0]) },
        Arg::Const { n: 1, lanes: lanes([3.0; 4]) },
        Arg::Const { n: 3, lanes: lanes([2.0; 4]) },
        Arg::Const { n: 1, lanes: lanes([1.0; 4]) },
    ];
    match run_wired("blur_channels", tag, &args, w * h, (w, h), &KParams::standin())? {
        Out::Img(i) => Ok(i.planes.iter().map(|p| p.iter().map(|v| v.to_bits()).collect()).collect()),
        _ => panic!("blur_channels must give an image"),
    }
}

#[test]
fn one_plane_source_at_tag_3_equals_the_plane_repeated() {
    let (w, h) = (24usize, 10usize);
    let p = plane(w, h);
    let one = Image::from_planes(vec![p.clone()]).unwrap();
    let three = Image::from_planes(vec![p.clone(), p.clone(), p]).unwrap();
    let a = run(&one, 3, w, h).unwrap();
    assert_eq!(a.len(), 3);
    assert_eq!(a, run(&three, 3, w, h).unwrap());
}

#[test]
fn two_plane_source_at_tag_4_repeats_the_last_plane() {
    let (w, h) = (24usize, 10usize);
    let (p, q) = (plane(w, h), plane(w, h).iter().map(|v| 1.0 - v).collect::<Vec<f32>>());
    let two = Image::from_planes(vec![p.clone(), q.clone()]).unwrap();
    let four = Image::from_planes(vec![p, q.clone(), q.clone(), q]).unwrap();
    assert_eq!(run(&two, 4, w, h).unwrap(), run(&four, 4, w, h).unwrap());
}
