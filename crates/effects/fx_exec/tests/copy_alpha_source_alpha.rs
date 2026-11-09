//! copy_alpha through the executor dispatch with a 1 channel colour operand and a 4 channel second operand: the 2 channel output
//! takes its alpha plane from channel 3 of the second operand, not from channel 1.
use fx_exec::kernels;
use fx_exec::types::{Arg, Image, KParams};

#[test]
fn two_channel_output_takes_alpha_from_channel_three() {
    let colour = Image { n: 3, planes: vec![vec![0.25, 0.5, 0.75]] };
    let src = Image { n: 3, planes: vec![vec![0.1, 0.2, 0.3], vec![0.4, 0.5, 0.6], vec![0.7, 0.8, 0.9], vec![0.9, 0.8, 0.7]] };
    let args = [Arg::Img(&colour), Arg::Img(&src)];
    let out = kernels::run_any("copy_alpha", 2, &args, 3, (3, 1), &KParams::standin()).unwrap().into_image().unwrap();
    assert_eq!(out.planes.len(), 2);
    assert_eq!(out.planes[0], vec![0.25, 0.5, 0.75]);
    assert_eq!(out.planes[1], vec![0.9, 0.8, 0.7]);
}

#[test]
fn matching_channel_counts_keep_the_last_channel_route() {
    let a = Image { n: 2, planes: vec![vec![1.0, 2.0], vec![3.0, 4.0]] };
    let b = Image { n: 2, planes: vec![vec![5.0, 6.0], vec![7.0, 8.0]] };
    let out = kernels::run_any("copy_alpha", 2, &[Arg::Img(&a), Arg::Img(&b)], 2, (2, 1), &KParams::standin()).unwrap().into_image().unwrap();
    assert_eq!(out.planes[0], vec![1.0, 2.0]);
    assert_eq!(out.planes[1], vec![7.0, 8.0]);
}
