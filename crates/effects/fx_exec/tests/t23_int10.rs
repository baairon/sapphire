//! INT10 s1: the postdivide class through the executor dispatch, and the blur aspect host input's state A value.
use fx_exec::kernels::run_any;
use fx_exec::{Arg, Image, KParams};

fn cst1(v: f32) -> Arg<'static> {
    Arg::Const { n: 1, lanes: [v.to_bits(); 4] }
}

#[test]
fn t23_postdivide_dispatch_4_and_2() {
    let n = 4;
    let a = vec![1e-5f32, 0.5, 1.0, f32::NAN];
    let c = vec![0.25f32; n];
    let im4 = Image::from_planes(vec![c.clone(), c.clone(), c.clone(), a.clone()]).unwrap();
    let kp = KParams::standin();
    let o = run_any("postdivide", 4, &[Arg::Img(&im4), cst1(-1.0)], n, (n, 1), &kp).unwrap().into_image().unwrap();
    assert_eq!(o.planes[0], vec![0.0, 0.5, 0.25, 0.25]);
    assert_eq!(o.planes[3][0].to_bits(), 1e-5f32.to_bits());
    assert_eq!(o.planes[3][1], 0.5);
    assert_eq!(o.planes[3][2], 1.0);
    assert_eq!(o.planes[3][3], 1.0);
    let im2 = Image::from_planes(vec![c.clone(), a.clone()]).unwrap();
    let o2 = run_any("postdivide", 2, &[Arg::Img(&im2), cst1(-1.0)], n, (n, 1), &kp).unwrap().into_image().unwrap();
    assert_eq!(o2.planes[0], vec![0.0, 0.5, 0.25, 0.25]);
    // copy_alpha >= 0 is refused by name; elnum 3 is folded by the compiler and refused here
    let e = run_any("postdivide", 4, &[Arg::Img(&im4), cst1(0.0)], n, (n, 1), &kp).unwrap_err();
    assert!(format!("{:?}", e).contains("POSTDIV-COPY-ALPHA-SOURCE"));
    let im3 = Image::from_planes(vec![c.clone(), c.clone(), c.clone()]).unwrap();
    let e3 = run_any("postdivide", 3, &[Arg::Img(&im3), cst1(-1.0)], n, (n, 1), &kp).unwrap_err();
    assert!(format!("{:?}", e3).contains("POSTDIV-ELNUM-FOLDED"));
}

#[test]
fn t23_blur_aspect_state_a_is_none() {
    assert_eq!(KParams::standin().blur_aspect, None);
}
