//! A NaN that no operand supplies is the x86 default NaN 0xffc00000 on every processor, and the negations after it follow.
//! The expected words are what x86 produces (hardware default NaN, then the sign flips of the kernel).
use hue_shift::pixel;

const EXPECT: &[u32] = &[
    0x3e800000, 0x3f400000, 0x7fc00000, 0x3e800000, 0x3f400000, 0x7fc00000,
    0xffc00000, 0x7f800000, 0xffc00000, 0xffc00000, 0x7f800000, 0xffc00000,
    0xffc00000, 0xff800000, 0xffc00000, 0x7f800000, 0xffc00000, 0xffc00000,
];

#[test]
fn fresh_nan_words_are_the_x86_words() {
    let inf = f32::INFINITY;
    let cases: [(f32, f32, f32, f32); 6] = [
        // an infinite shift: `x - floor(x)` is infinity minus infinity, and the negated copies of the NaN reach the output
        (0.5, 0.25, 0.75, inf),
        (0.5, 0.25, 0.75, -inf),
        // an infinite channel against a negative one: the chroma `hi - lo` and the saturation `delta / hi` are invalid
        (inf, 0.25, -inf, 0.1),
        (inf, inf, 0.25, 0.1),
        (-inf, 0.5, 0.25, 0.3),
        (0.0, 0.0, inf, 0.2),
    ];
    let got: Vec<u32> = cases.iter().flat_map(|c| pixel(c.0, c.1, c.2, c.3).map(f32::to_bits)).collect();
    assert_eq!(format!("{got:08x?}"), format!("{EXPECT:08x?}"));
}
