//! DSZ1 unit test: blur on a make_mipmap operand equals blur on the mipmap's base image (Drop_Shadow at Shift 0);
//! a non-ripmap operand (image, constant) is unchanged. Std and crate APIs only.
use fx_exec::kernels2::{run_wired, Out};
use fx_exec::*;

fn cst(n: usize, v: f32) -> Arg<'static> {
    Arg::Const { n, lanes: [v.to_bits(); 4] }
}
fn img(w: usize, h: usize) -> Image {
    let mut s = 12345u64;
    let p: Vec<f32> = (0..w * h)
        .map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            0.05 + ((s >> 33) as u32 as f32 / 2147483648.0)
        })
        .collect();
    Image::from_planes(vec![p]).unwrap()
}
fn planes(o: Out) -> Vec<Vec<u32>> {
    match o {
        Out::Img(i) => i.planes.iter().map(|p| p.iter().map(|v| v.to_bits()).collect()).collect(),
        _ => panic!("blur must give an image"),
    }
}
fn blur(a0: Arg, w: usize, h: usize) -> Result<Out, ExecError> {
    let args = [a0, cst(2, 0.0302124), cst(1, 3.0), cst(1, 0.739105), cst(1, 1.0)];
    run_wired("blur", 1, &args, w * h, (w, h), &KParams::standin())
}

#[test]
fn blur_on_mip_operand_equals_blur_on_base_image() {
    for &(w, h) in &[(16usize, 8usize), (13, 5)] {
        let base = img(w, h);
        let mm = match run_wired("make_mipmap", 1, &[Arg::Img(&base), cst(2, 0.0), cst(1, 0.0)], w * h, (w, h), &KParams::standin()).unwrap() {
            Out::Mip(m) => m,
            _ => panic!("make_mipmap must give a ripmap"),
        };
        let via_mip = planes(blur(Arg::Mip(&mm), w, h).unwrap());
        let via_img = planes(blur(Arg::Img(&base), w, h).unwrap());
        assert_eq!(via_mip, via_img, "{}x{}", w, h);
    }
}

#[test]
fn blur_on_non_mip_operands_unchanged() {
    let (w, h) = (16usize, 8usize);
    let flat = Image::from_const(&[0.5f32.to_bits(); 4], 1, w * h);
    let a = planes(blur(Arg::Img(&flat), w, h).unwrap());
    let b = planes(blur(cst(1, 0.5), w, h).unwrap());
    assert_eq!(a, b);
    let base = img(w, h);
    let c = planes(blur(Arg::Img(&base), w, h).unwrap());
    assert_ne!(c, planes(blur(Arg::Img(&flat), w, h).unwrap()));
}
