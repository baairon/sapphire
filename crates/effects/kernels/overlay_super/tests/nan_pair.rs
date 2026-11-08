//! Two-NaN operands in the low branch product: a times b on the plane routes, b times a when the A operand is the constant.
use overlay_super::{render, Operand, Params};

fn run(a_const: bool, b_const: bool, a: f32, b: f32) -> u32 {
    let pa = [vec![a; 4]];
    let pb = [vec![b; 4]];
    let pa_s: Vec<&[f32]> = pa.iter().map(|v| &v[..]).collect();
    let pb_s: Vec<&[f32]> = pb.iter().map(|v| &v[..]).collect();
    let oa = Operand { count: 1, is_const: a_const, consts: [a; 4], planes: &pa_s };
    let ob = Operand { count: 1, is_const: b_const, consts: [b; 4], planes: &pb_s };
    let mut d = vec![0f32; 4];
    let mut o: Vec<&mut [f32]> = vec![&mut d[..]];
    render(&Params { nout: 1, n: 4 }, &oa, &ob, &mut o).unwrap();
    assert!(d.iter().all(|x| x.to_bits() == d[0].to_bits()));
    d[0].to_bits()
}

#[test]
fn product_operand_order_follows_the_route() {
    let (x, y) = (f32::from_bits(0x7f80_0001), f32::from_bits(0xffc0_0002));
    // plane and plane, plane and constant B: a first
    assert_eq!(run(false, false, x, y), 0x7fc0_0001);
    assert_eq!(run(false, true, x, y), 0x7fc0_0001);
    assert_eq!(run(false, false, y, x), 0xffc0_0002);
    // constant A: b first
    assert_eq!(run(true, false, x, y), 0xffc0_0002);
    assert_eq!(run(true, false, y, x), 0x7fc0_0001);
}

#[test]
fn single_nan_and_fresh_nan() {
    assert_eq!(run(false, false, f32::from_bits(0x7f80_0005), 0.25), 0x7fc0_0005);
    assert_eq!(run(false, false, f32::INFINITY, 0.0), 0xffc0_0000);
}

/// -inf with b in (0.5, 1] reaches the fused -(a * b) + a, which is inf - inf: a NaN no operand supplies, the default NaN on every processor.
#[test]
fn negative_infinity_a_gives_the_default_nan() {
    for b in [0.75f32, 1.0, 0.5000001] {
        for (ac, bc) in [(false, false), (true, false), (false, true), (true, true)] {
            assert_eq!(run(ac, bc, f32::NEG_INFINITY, b), 0xffc0_0000, "b {b} routes {ac} {bc}");
        }
    }
}
