//! Fresh NaN words of comp_premult are the x86 words on every processor: the fused `-t * a + a` of an infinite A is invalid.
use comp_premult::{run, Mutant, Operand, Params};

const EXPECT: &[u32] = &[
    0xffc00000, 0x3f000000, 0xffc00000, 0x7f800000, 0xff800000, 0x3f000000,
    0x3f800000,
];

fn op<'a>(count: i32, is_const: bool, k: f32, planes: &'a [&'a [f32]]) -> Operand<'a> {
    Operand { count, is_const, consts: [k; 4], planes }
}

fn one(a: Operand, b: Operand, t: Operand) -> u32 {
    let mut o = vec![0.0f32; 1];
    {
        let mut outs: Vec<&mut [f32]> = vec![&mut o[..]];
        run(&Params { nout: 1, n: 1 }, &a, &b, &t, &mut outs, Mutant::None).unwrap();
    }
    o[0].to_bits()
}

#[test]
fn fresh_nan_words_are_the_x86_words() {
    let inf = f32::INFINITY;
    let bp = [[0.5f32]];
    let bs: Vec<&[f32]> = bp.iter().map(|x| &x[..]).collect();
    let mut got: Vec<u32> = Vec::new();
    // A constant, B plane, T constant: B + fma(-t, A, A)
    for (a, t) in [(inf, 0.5), (-inf, 1.0), (inf, 0.25)] {
        got.push(one(op(1, true, a, &[]), op(1, false, 0.0, &bs), op(1, true, t, &[])));
    }
    // all planes
    for (a, t) in [(inf, 0.25f32), (-inf, 0.75), (0.5, inf), (0.5, -inf)] {
        let (ap, tp) = ([[a]], [[t]]);
        let (av, tv): (Vec<&[f32]>, Vec<&[f32]>) = (ap.iter().map(|x| &x[..]).collect(), tp.iter().map(|x| &x[..]).collect());
        got.push(one(op(1, false, 0.0, &av), op(1, false, 0.0, &bs), op(1, false, 0.0, &tv)));
    }
    assert_eq!(format!("{got:08x?}"), format!("{EXPECT:08x?}"));
}
