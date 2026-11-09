//! Fresh NaN words of perspective_alpha are the x86 words on every processor: `-p * v - r1` with p = 0 and an infinite v is invalid.
use perspective_alpha::{perspective_alpha, Info, Params};

const EXPECT: &[u32] = &[
    0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000,
    0xffc00000, 0xffc00000, 0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000,
    0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000,
];

#[test]
fn fresh_nan_words_are_the_x86_words() {
    let info = Info { n: 4, roi_form: 0, port_blk_a: 4, port_blk_b: 1, port_blk_c: 4, s8: 8, port_info_b: 4, port_info_a: 1.0 };
    let mut got: Vec<u32> = Vec::new();
    for (par, v) in [
        (Params { params_b: 0.0, params_a: 1.0, params_c: 0.0 }, f32::INFINITY),
        (Params { params_b: 0.0, params_a: 1.0, params_c: 0.0 }, f32::NEG_INFINITY),
        (Params { params_b: 0.0, params_a: f32::INFINITY, params_c: 1.0 }, 0.5),
        (Params { params_b: 0.0, params_a: 1.0, params_c: f32::INFINITY }, 0.5),
    ] {
        let src = [v; 4];
        let mut dst = [7.0f32; 4];
        perspective_alpha(&par, &info, false, &src, &mut dst, 0).unwrap();
        got.extend(dst.map(f32::to_bits));
    }
    assert_eq!(format!("{got:08x?}"), format!("{EXPECT:08x?}"));
}
