//! tilt_alpha through the executor dispatch: the combo table picks tag 1 for (2 channel image, pair, scalar), and the run equals the
//! kernel crate called directly on the same planes; missing host fields, the host branch and a wrong operand shape are refused.
use fx_exec::kernels;
use fx_exec::selector::Tables;
use fx_exec::types::{Arg, Image, KParams};

fn cst(v: &[f32]) -> Arg<'static> {
    let mut l = [0u32; 4];
    for (i, x) in v.iter().enumerate().take(4) {
        l[i] = x.to_bits();
    }
    Arg::Const { n: v.len(), lanes: l }
}

fn planes(n: usize) -> Image {
    let u: Vec<f32> = (0..n).map(|i| (i as f32) * 0.25 - 1.0).collect();
    let v: Vec<f32> = (0..n).map(|i| 1.5 - (i as f32) * 0.125).collect();
    Image { n, planes: vec![u, v] }
}

fn cells() -> KParams {
    let mut kp = KParams::standin();
    kp.palpha_cells = Some((640, 480, 2.0));
    kp
}

#[test]
fn combo_tag_and_run_equal_the_kernel() {
    let img = planes(12);
    let args = [Arg::Img(&img), cst(&[20.0, 35.0]), cst(&[1.5])];
    let counts: Vec<i32> = args.iter().map(|a| a.count()).collect();
    assert_eq!(counts, vec![2, 2, 1]);
    let tag = Tables::embedded().select_counts("tilt_alpha", &counts);
    assert_eq!(tag, 1);
    let out = kernels::run_any("tilt_alpha", tag, &args, 12, (4, 3), &cells()).unwrap().into_image().unwrap();
    assert_eq!(out.planes.len(), 1);
    let st = tilt_alpha::State { swivel: 20.0, tilt: 35.0, amount: 1.5 };
    let host = tilt_alpha::Host { w: 640, d: 2.0, h: 480 };
    let mut want = vec![0.0f32; 12];
    for y in 0..3 {
        let mut row = vec![0.0f32; 4];
        tilt_alpha::tilt_alpha(&st, &host, false, 4, &img.planes[0][y * 4..y * 4 + 4], &img.planes[1][y * 4..y * 4 + 4], &mut row, (y * 16) & 0x1f).unwrap();
        want[y * 4..y * 4 + 4].copy_from_slice(&row);
    }
    let same = out.planes[0].iter().zip(&want).all(|(a, b)| a.to_bits() == b.to_bits());
    assert!(same, "{:?} vs {:?}", out.planes[0], want);
}

#[test]
fn refusals() {
    let img = planes(4);
    let args = [Arg::Img(&img), cst(&[20.0, 35.0]), cst(&[1.5])];
    // no host record fields
    assert!(kernels::run_any("tilt_alpha", 1, &args, 4, (4, 1), &KParams::standin()).is_err());
    // host branch: |cos(angle) / amount| < 1e-4
    let big = [Arg::Img(&img), cst(&[20.0, 35.0]), cst(&[1.0e6])];
    assert!(kernels::run_any("tilt_alpha", 1, &big, 4, (4, 1), &cells()).is_err());
    // wrong operand shape: one channel image
    let one = Image { n: 4, planes: vec![vec![0.0; 4]] };
    let bad = [Arg::Img(&one), cst(&[20.0, 35.0]), cst(&[1.5])];
    assert!(kernels::run_any("tilt_alpha", 1, &bad, 4, (4, 1), &cells()).is_err());
}
