//! Fresh NaN words of rotate are the x86 words on every processor: `v - centre` of equal infinities is invalid and its negation
//! feeds the fused multiply add chain.
use rotate2d::{render, Angle, Vec2};

const EXPECT: &[u32] = &[
    0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000,
    0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000,
];

fn run(v: Vec2, a: Angle, c: Vec2) -> [u32; 2] {
    let (mut ox, mut oy) = ([0.0f32], [0.0f32]);
    render(v, a, c, 1, &mut ox, &mut oy).unwrap();
    [ox[0].to_bits(), oy[0].to_bits()]
}

#[test]
fn fresh_nan_words_are_the_x86_words() {
    let inf = f32::INFINITY;
    let (vx, vy) = ([inf], [inf]);
    let (cx, cy) = ([inf], [-inf]);
    let ang = [0.25f32];
    let mut got: Vec<u32> = Vec::new();
    // angle constant: planes against planes, planes against a constant centre
    got.extend(run(Vec2::Planes(&vx, &vy), Angle::Const(0.125), Vec2::Planes(&cx, &cy)));
    got.extend(run(Vec2::Planes(&vx, &vy), Angle::Const(0.125), Vec2::Const(inf, inf)));
    // angle plane: all four operand shapes with an infinite v
    got.extend(run(Vec2::Planes(&vx, &vy), Angle::Plane(&ang), Vec2::Planes(&cx, &cy)));
    got.extend(run(Vec2::Planes(&vx, &vy), Angle::Plane(&ang), Vec2::Const(inf, 0.0)));
    got.extend(run(Vec2::Const(inf, 1.0), Angle::Plane(&ang), Vec2::Planes(&cx, &cy)));
    got.extend(run(Vec2::Const(inf, 1.0), Angle::Plane(&ang), Vec2::Const(inf, 1.0)));
    assert_eq!(format!("{got:08x?}"), format!("{EXPECT:08x?}"));
}
