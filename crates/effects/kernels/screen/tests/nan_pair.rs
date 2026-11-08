//! Two-NaN operands: the first operand of the fused add (a, then 1 - a, then b) wins, quieted, on every route.
use screen::{render, Operand, Params};

fn run(a_const: bool, b_const: bool, a: f32, b: f32) -> u32 {
    let pa = [vec![a; 4]];
    let pb = [vec![b; 4]];
    let pa_s: Vec<&[f32]> = pa.iter().map(|v| &v[..]).collect();
    let pb_s: Vec<&[f32]> = pb.iter().map(|v| &v[..]).collect();
    let oa = Operand { count: 1, is_const: a_const, consts: [a; 4], planes: &pa_s };
    let ob = Operand { count: 1, is_const: b_const, consts: [b; 4], planes: &pb_s };
    let mut d = vec![0f32; 4];
    let mut o: Vec<&mut [f32]> = vec![&mut d[..]];
    render(&Params { nout: 1, n: 4, vec_one: 1.0, dst3_phase_bytes: 0 }, &oa, &ob, &mut o).unwrap();
    assert!(d.iter().all(|x| x.to_bits() == d[0].to_bits()));
    d[0].to_bits()
}

#[test]
fn first_operand_wins_with_two_nans() {
    let (sa, qb) = (f32::from_bits(0x7f80_0001), f32::from_bits(0xffc0_0002));
    for (ac, bc) in [(false, false), (true, false), (false, true)] {
        assert_eq!(run(ac, bc, sa, qb), 0x7fc0_0001, "a signalling, b quiet, routes {} {}", ac, bc);
        assert_eq!(run(ac, bc, qb, sa), 0xffc0_0002, "a quiet negative, b signalling, routes {} {}", ac, bc);
    }
}

#[test]
fn one_nan_and_fresh_nan_rules() {
    assert_eq!(run(false, false, 0.25, f32::from_bits(0x7f80_0003)), 0x7fc0_0003);
    assert_eq!(run(false, false, f32::from_bits(0xff80_0004), 0.5), 0xffc0_0004);
    // inf * 0 inside the fused add is the default NaN
    assert_eq!(run(false, false, f32::NEG_INFINITY, 0.0), 0xffc0_0000);
}
