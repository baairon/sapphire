//! Fresh NaN words of perspective are the x86 words on every processor: an infinite v makes `-m4 * v - m7` and the output chain invalid.
use perspective::{run, Obj};

const EXPECT: &[u32] = &[
    0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7fc00000, 0x7f800000, 0x3e2aaaaa,
    0xffc00000, 0xffc00000, 0x7f800000, 0xffc00000, 0xffc00000, 0xffc00000,
];

fn go(c: f32, a: f32, d: f32, r: (f32, f32), u: f32, v: f32) -> [u32; 2] {
    let o = Obj { port_obj_c: c, gpu_call_a: a, port_obj_d: d };
    let (mut o1, mut o2) = ([0.0f32], [0.0f32]);
    run(&o, true, false, 1, r, 0, &[u], &[v], &mut o1, &mut o2).unwrap();
    [o1[0].to_bits(), o2[0].to_bits()]
}

#[test]
fn fresh_nan_words_are_the_x86_words() {
    let inf = f32::INFINITY;
    let mut got: Vec<u32> = Vec::new();
    got.extend(go(1.0, 0.0, 1.0, (0.0, 1.0), 0.5, inf));
    got.extend(go(1.0, 0.0, 1.0, (0.0, 1.0), 0.5, -inf));
    got.extend(go(0.5, 0.0, 2.0, (0.5, 0.5), inf, 0.25));
    got.extend(go(0.0, 0.0, 0.0, (0.0, 0.0), 0.5, inf));
    got.extend(go(inf, 0.0, 1.0, (0.0, 1.0), 0.5, 0.25));
    got.extend(go(1.0, 0.0, inf, (0.5, 0.5), 0.5, 0.25));
    assert_eq!(format!("{got:08x?}"), format!("{EXPECT:08x?}"));
}
