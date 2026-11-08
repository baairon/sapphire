use screen::*;

const S: u32 = 0x7fc0ffee;
fn outs(k: usize, n: usize) -> Vec<Vec<f32>> {
    (0..k).map(|_| vec![f32::from_bits(S); n]).collect()
}
fn go(p: &Params, a: &Operand, b: &Operand, o: &mut Vec<Vec<f32>>) -> Result<(), ScreenError> {
    let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
    render(p, a, b, &mut m)
}
fn op<'a>(count: i32, planes: &'a [&'a [f32]]) -> Operand<'a> {
    Operand { count, is_const: false, consts: [0.0; 4], planes }
}
fn cop(count: i32, c: [f32; 4]) -> Operand<'static> {
    Operand { count, is_const: true, consts: c, planes: &[] }
}
fn p(nout: i32, n: i32) -> Params {
    Params { nout, n, vec_one: 1.0, dst3_phase_bytes: 0 }
}

#[test]
fn hand_values() {
    // screen = a + (1-a) b: (0.5,0.5) -> 0.75, (0.25,0.5) -> 0.625, (0,x) -> x, (1,b) -> 1, no clamping
    let a0 = [0.5f32, 0.25, 0.0, 1.0, 2.0];
    let b0 = [0.5f32, 0.5, 0.75, 0.3, 4.0];
    let (pa, pb) = ([&a0[..]], [&b0[..]]);
    let mut o = outs(1, 8);
    go(&p(1, 5), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
    assert_eq!(&o[0][..5], &[0.75, 0.625, 0.75, 1.0, -2.0]);
    assert_eq!(o[0][5].to_bits(), S);
}

#[test]
fn fused_not_unfused() {
    let mut seen = false;
    for i in 1..4000u32 {
        let (a, b) = (i as f32 * 0.00037, 0.7123f32 + i as f32 * 0.0001);
        let fu = (1.0 - a).mul_add(b, a);
        if fu.to_bits() != ((1.0 - a) * b + a).to_bits() {
            seen = true;
            let (x, y) = ([a], [b]);
            let (pa, pb) = ([&x[..]], [&y[..]]);
            let mut o = outs(1, 2);
            go(&p(1, 1), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
            assert_eq!(o[0][0].to_bits(), fu.to_bits());
            break;
        }
    }
    assert!(seen);
}

#[test]
fn nan_and_inf() {
    let a0 = [f32::NAN, 1.0, f32::INFINITY, -0.0];
    let b0 = [0.5f32, f32::NAN, 0.5, 0.0];
    let (pa, pb) = ([&a0[..]], [&b0[..]]);
    let mut o = outs(1, 4);
    go(&p(1, 4), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
    assert!(o[0][0].is_nan() && o[0][1].is_nan() && o[0][2].is_nan());
    assert_eq!(o[0][3].to_bits(), 0);
}

#[test]
fn extension_kernels() {
    let b: Vec<Vec<f32>> = (0..4).map(|_| vec![0.5; 40]).collect();
    let br: Vec<&[f32]> = b.iter().map(|v| v.as_slice()).collect();
    let mut o = outs(4, 40);
    go(&p(4, 40), &cop(3, [0.5, 0.5, 0.5, 0.0]), &op(4, &br), &mut o).unwrap();
    assert!(o[3][..40].iter().all(|x| *x == 1.0));
    // vec_one only changes the ymm core
    let mut o = outs(4, 40);
    let q = Params { nout: 4, n: 40, vec_one: 0.75, dst3_phase_bytes: 0 };
    go(&q, &cop(3, [0.5, 0.5, 0.5, 0.0]), &op(4, &br), &mut o).unwrap();
    assert!(o[3][..40].iter().any(|x| *x == 0.75) && o[3][0] == 0.75 && o[3][39] == 1.0);
    // (4,3): (1 + a3) - a3
    let a: Vec<Vec<f32>> = (0..4).map(|c| vec![if c == 3 { 1.0e8 } else { 0.5 }; 3]).collect();
    let ar: Vec<&[f32]> = a.iter().map(|v| v.as_slice()).collect();
    let b3: Vec<Vec<f32>> = (0..3).map(|_| vec![0.5; 3]).collect();
    let b3r: Vec<&[f32]> = b3.iter().map(|v| v.as_slice()).collect();
    let mut o = outs(4, 3);
    go(&p(4, 3), &op(4, &ar), &op(3, &b3r), &mut o).unwrap();
    assert_eq!(o[3][0], 0.0);
}

#[test]
fn degenerate_and_errors() {
    let (x, y) = ([0.5f32], [0.5f32]);
    let (pa, pb) = ([&x[..]], [&y[..]]);
    let mut o = outs(1, 1);
    go(&p(1, 0), &op(1, &pa), &op(1, &pb), &mut o).unwrap();
    assert_eq!(o[0][0].to_bits(), S);
    let mut o = outs(1, 1);
    assert_eq!(go(&p(1, 3), &op(1, &pa), &op(1, &pb), &mut o), Err(ScreenError::MissingOut));
    let mut o = outs(1, 4);
    let a = cop(1, [0.5; 4]);
    let bb = cop(1, [0.5; 4]);
    assert_eq!(go(&p(1, 2), &a, &bb, &mut o), Err(ScreenError::MissingPlane));
    assert!(!assumptions().is_empty());
}

#[test]
fn deterministic() {
    let a0: Vec<f32> = (0..33).map(|i| i as f32 * 0.07 - 0.3).collect();
    let b0: Vec<f32> = (0..33).map(|i| i as f32 * 0.05).collect();
    let (pa, pb) = ([&a0[..]], [&b0[..]]);
    let mut o1 = outs(1, 33);
    let mut o2 = outs(1, 33);
    go(&p(1, 33), &op(1, &pa), &op(1, &pb), &mut o1).unwrap();
    go(&p(1, 33), &op(1, &pa), &op(1, &pb), &mut o2).unwrap();
    assert_eq!(o1, o2);
}
