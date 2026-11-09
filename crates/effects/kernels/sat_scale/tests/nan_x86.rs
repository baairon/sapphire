//! Fresh NaN words of sat_scale are the x86 words on every processor: the luma of opposite infinities is an invalid operation
//! in double precision, and the negated luma feeds the fused multiply add.
use sat_scale::{pixels, Inputs, Params};

const EXPECT: &[u32] = &[
    0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000,
    0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000,
    0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000,
    0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000,
];

fn run(r: f32, g: f32, b: f32, scale: f32, masked: bool) -> [u32; 3] {
    let (rp, gp, bp, mp) = ([r], [g], [b], [scale]);
    let p = Params { const_colour: false, scalar_scale: !masked, colour: [0.0; 3], scale, flag: false };
    let inp = Inputs { src: Some([&rp, &gp, &bp]), mask: if masked { Some(&mp) } else { None } };
    let (mut o0, mut o1, mut o2) = ([0.0f32], [0.0f32], [0.0f32]);
    pixels(&p, 1, &inp, &mut [&mut o0, &mut o1, &mut o2]).unwrap();
    [o0[0].to_bits(), o1[0].to_bits(), o2[0].to_bits()]
}

#[test]
fn fresh_nan_words_are_the_x86_words() {
    let inf = f32::INFINITY;
    let mut got: Vec<u32> = Vec::new();
    for masked in [false, true] {
        // opposite infinities: the luma is an invalid operation
        got.extend(run(inf, 0.5, -inf, 0.5, masked));
        got.extend(run(inf, -inf, 1.0, 2.0, masked));
        // an infinite scale
        got.extend(run(0.25, 0.5, 0.75, inf, masked));
        got.extend(run(0.0, 0.5, inf, -inf, masked));
    }
    assert_eq!(format!("{got:08x?}"), format!("{EXPECT:08x?}"));
}
