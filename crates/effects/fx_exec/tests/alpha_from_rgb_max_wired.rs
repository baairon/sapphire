//! alpha_from_rgb_max through the executor dispatch: the combo table picks tag 4 for a 4 channel input, one amount lane and a
//! 4 lane output operand; the first three planes are copies and plane 3 is the per-pixel maximum of them (amount 1).
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

#[test]
fn four_channel_input_gives_copies_and_the_maximum() {
    let img = Image { n: 3, planes: vec![vec![0.25, 2.0, -1.0], vec![0.5, 1.0, -3.0], vec![0.125, 4.0, -2.0], vec![9.0, 9.0, 9.0]] };
    let amount = cst(&[1.0]);
    let outv = cst(&[-999.0, -999.0, -999.0, -999.0]);
    let args = [Arg::Img(&img), amount, outv];
    let counts: Vec<i32> = args.iter().map(|a| a.count()).collect();
    assert_eq!(counts, vec![4, 1, 4]);
    let tag = Tables::embedded().select_counts("alpha_from_rgb_max", &counts);
    assert_eq!(tag, 4);
    let out = kernels::run_any("alpha_from_rgb_max", tag, &args, 3, (3, 1), &KParams::standin()).unwrap().into_image().unwrap();
    assert_eq!(out.planes.len(), 4);
    assert_eq!(out.planes[0], vec![0.25, 2.0, -1.0]);
    assert_eq!(out.planes[1], vec![0.5, 1.0, -3.0]);
    assert_eq!(out.planes[2], vec![0.125, 4.0, -2.0]);
    assert_eq!(out.planes[3], vec![0.5, 4.0, -1.0]);
}

#[test]
fn amount_scales_the_maximum_and_zero_clears_it() {
    let img = Image { n: 2, planes: vec![vec![1.0, 2.0], vec![3.0, 1.0], vec![2.0, 0.5], vec![0.0, 0.0]] };
    let run = |s: f32| {
        let args = [Arg::Img(&img), cst(&[s]), cst(&[0.0, 0.0, 0.0, 0.0])];
        kernels::run_any("alpha_from_rgb_max", 4, &args, 2, (2, 1), &KParams::standin()).unwrap().into_image().unwrap().planes[3].clone()
    };
    assert_eq!(run(0.5), vec![1.5, 1.0]);
    assert_eq!(run(0.0), vec![0.0, 0.0]);
}

#[test]
fn other_output_counts_are_refused() {
    let img = Image { n: 1, planes: vec![vec![1.0], vec![1.0], vec![1.0], vec![1.0]] };
    let args = [Arg::Img(&img), cst(&[1.0]), cst(&[0.0, 0.0, 0.0])];
    assert!(kernels::run_any("alpha_from_rgb_max", 3, &args, 1, (1, 1), &KParams::standin()).is_err());
}
