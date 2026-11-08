use overlay_super::*;

const S: u32 = 0x7fc0ffee;
fn outs(k: usize, n: usize) -> Vec<Vec<f32>> {
    (0..k).map(|_| vec![f32::from_bits(S); n]).collect()
}
fn go(p: &Params, a: &Operand, b: &Operand, o: &mut Vec<Vec<f32>>) -> Result<(), OverlayError> {
    let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
    render(p, a, b, &mut m)
}
fn op<'a>(count: i32, planes: &'a [&'a [f32]]) -> Operand<'a> {
    Operand { count, is_const: false, consts: [0.0; 4], planes }
}
fn cop(count: i32, c: [f32; 4]) -> Operand<'static> {
    Operand { count, is_const: true, consts: c, planes: &[] }
}

#[test]
fn hand_values() {
    // b <= 0.5: 2ab. a = 0.5, b = 0.25 -> 0.25. b = 0.5 exactly takes the low branch.
    // b in (0.5, 1], a <= 1: ((b + (b-1)) + t) + t with t = a - a*b. a = 0.5, b = 1: t = 0, out = 1.
    // a = 0.5, b = 0.75: t = 0.125, d = -0.25, s = 0.5, out = (0.5+0.125)+0.125 = 0.75
    // a > 1 or b > 1: max(a,b)
    let a0 = [0.5f32, 0.5, 0.5, 2.0, 0.5, 0.25];
    let b0 = [0.25f32, 0.5, 0.75, 0.9, 3.0, 1.0];
    let (pa, pb) = ([&a0[..]], [&b0[..]]);
    let mut o = outs(1, 8);
    go(&Params { nout: 1, n: 6 }, &op(1, &pa), &op(1, &pb), &mut o).unwrap();
    assert_eq!(&o[0][..6], &[0.25, 0.5, 0.75, 2.0, 3.0, 1.0]);
    assert_eq!(o[0][6].to_bits(), S);
}

#[test]
fn nan_and_zero_rules() {
    let a0 = [1.0f32, f32::NAN, -0.0, f32::INFINITY];
    let b0 = [f32::NAN, 0.75, 0.5, 0.75];
    let (pa, pb) = ([&a0[..]], [&b0[..]]);
    let mut o = outs(1, 4);
    go(&Params { nout: 1, n: 4 }, &op(1, &pa), &op(1, &pb), &mut o).unwrap();
    assert!(o[0][0].is_nan());
    assert!(o[0][1].is_nan());
    assert_eq!(o[0][2].to_bits(), 0x80000000);
    assert_eq!(o[0][3], f32::INFINITY);
}

#[test]
fn fused_not_unfused() {
    let mut seen = false;
    for i in 1..4000u32 {
        let (a, b) = (0.1 + i as f32 * 0.00021, 0.51 + i as f32 * 0.00011);
        if b > 1.0 || a > 1.0 {
            continue;
        }
        let fu = (-a).mul_add(b, a);
        let un = a - a * b;
        if fu.to_bits() != un.to_bits() {
            seen = true;
            let (x, y) = ([a], [b]);
            let (pa, pb) = ([&x[..]], [&y[..]]);
            let mut o = outs(1, 2);
            go(&Params { nout: 1, n: 1 }, &op(1, &pa), &op(1, &pb), &mut o).unwrap();
            let d = b - 1.0;
            assert_eq!(o[0][0].to_bits(), (((b + d) + fu) + fu).to_bits());
            break;
        }
    }
    assert!(seen);
}

#[test]
fn constants_and_extension() {
    // (3,4) with a constant A: channel 3 comes from B channel 3
    let b: Vec<Vec<f32>> = (0..4).map(|c| vec![0.25 + c as f32 * 0.25; 3]).collect();
    let br: Vec<&[f32]> = b.iter().map(|v| v.as_slice()).collect();
    let mut o = outs(4, 3);
    go(&Params { nout: 4, n: 3 }, &cop(3, [0.5, 0.5, 0.5, 0.0]), &op(4, &br), &mut o).unwrap();
    // ch0: a = 0.5, b = 0.25 -> 0.25; ch3: b3 = 1.0 -> not low, not > 1: d = 0, out = 1
    assert_eq!(o[0][0], 0.25);
    assert_eq!(o[3][0], 1.0);
    // (4,3): channel 3 = max(a3, 1)
    let a: Vec<Vec<f32>> = (0..4).map(|c| vec![if c == 3 { 0.2 } else { 0.5 }; 2]).collect();
    let ar: Vec<&[f32]> = a.iter().map(|v| v.as_slice()).collect();
    let b3: Vec<Vec<f32>> = (0..3).map(|_| vec![0.5; 2]).collect();
    let b3r: Vec<&[f32]> = b3.iter().map(|v| v.as_slice()).collect();
    let mut o = outs(4, 2);
    go(&Params { nout: 4, n: 2 }, &op(4, &ar), &op(3, &b3r), &mut o).unwrap();
    assert_eq!(o[3][0], 1.0);
}

#[test]
fn width_one_zero_and_errors() {
    let (x, y) = ([0.75f32], [0.75f32]);
    let (pa, pb) = ([&x[..]], [&y[..]]);
    let mut o = outs(1, 1);
    go(&Params { nout: 1, n: 1 }, &op(1, &pa), &op(1, &pb), &mut o).unwrap();
    assert!(o[0][0].is_finite());
    let mut o = outs(1, 1);
    go(&Params { nout: 1, n: 0 }, &op(1, &pa), &op(1, &pb), &mut o).unwrap();
    assert_eq!(o[0][0].to_bits(), S);
    let mut o = outs(1, 1);
    assert_eq!(go(&Params { nout: 1, n: 5 }, &op(1, &pa), &op(1, &pb), &mut o), Err(OverlayError::MissingOut));
    let empty: [&[f32]; 0] = [];
    let mut o = outs(1, 4);
    assert_eq!(go(&Params { nout: 1, n: 2 }, &op(1, &empty), &op(1, &pb), &mut o), Err(OverlayError::MissingPlane));
}

#[test]
fn deterministic() {
    let a0: Vec<f32> = (0..33).map(|i| i as f32 * 0.07 - 0.3).collect();
    let b0: Vec<f32> = (0..33).map(|i| i as f32 * 0.05).collect();
    let (pa, pb) = ([&a0[..]], [&b0[..]]);
    let mut o1 = outs(1, 33);
    let mut o2 = outs(1, 33);
    go(&Params { nout: 1, n: 33 }, &op(1, &pa), &op(1, &pb), &mut o1).unwrap();
    go(&Params { nout: 1, n: 33 }, &op(1, &pa), &op(1, &pb), &mut o2).unwrap();
    assert_eq!(o1, o2);
    assert!(!assumptions().is_empty());
}

#[test]
fn fused_negation_keeps_nan_sign() {
    // a NaN input in the fused branch is returned quieted with its own sign (the negation applies to the product)
    let a0 = [f32::from_bits(0x7fc00000), f32::from_bits(0xffc00123), f32::from_bits(0x7f800001)];
    let b0 = [0.75f32, 0.75, 0.75];
    let (pa, pb) = ([&a0[..]], [&b0[..]]);
    let mut o = outs(1, 3);
    go(&Params { nout: 1, n: 3 }, &op(1, &pa), &op(1, &pb), &mut o).unwrap();
    assert_eq!(o[0][0].to_bits(), 0x7fc00000);
    assert_eq!(o[0][1].to_bits(), 0xffc00123);
    assert_eq!(o[0][2].to_bits(), 0x7fc00001);
}
