use format14_read_b::*;

fn f(x: f32) -> u32 {
    x.to_bits()
}
const INF: u32 = 0x7f80_0000;

fn px(a: f32, r: f32, g: f32, b: f32) -> [u32; 4] {
    [f(a), f(r), f(g), f(b)]
}
fn flat(v: &[[u32; 4]]) -> Vec<u32> {
    v.iter().flat_map(|p| p.iter().copied()).collect()
}

#[test]
fn v4_stores_unweighted_planes() {
    let src = flat(&[px(0.5, 0.25, 0.75, 1.0), px(2.0, -1.0, 3.0, 4.0)]);
    for block in [false, true] {
        let mut p = [vec![9u32; 2], vec![9; 2], vec![9; 2], vec![9; 2]];
        let [a, b, c, d] = &mut p;
        read4b(&src, &mut [&mut a[..], &mut b[..], &mut c[..], &mut d[..]], 0, 2, block).unwrap();
        assert_eq!(p[0], vec![f(0.25), f(-1.0)]);
        assert_eq!(p[1], vec![f(0.75), f(3.0)]);
        assert_eq!(p[2], vec![f(1.0), f(4.0)]);
        assert_eq!(p[3], vec![f(0.5), f(2.0)]);
    }
}

#[test]
fn v4_non_finite_becomes_zero() {
    let src = flat(&[[INF, 0x7fc0_0000, 0xff80_0000, f(1.0)]]);
    let mut p = [vec![9u32; 1], vec![9; 1], vec![9; 1], vec![9; 1]];
    let [a, b, c, d] = &mut p;
    read4b(&src, &mut [&mut a[..], &mut b[..], &mut c[..], &mut d[..]], 0, 1, false).unwrap();
    assert_eq!((p[0][0], p[1][0], p[2][0], p[3][0]), (0, 0, f(1.0), 0));
}

#[test]
fn v3_ignores_alpha() {
    let src = flat(&[px(f32::NAN, 0.1, 0.2, 0.3)]);
    let mut p = [vec![9u32; 1], vec![9; 1], vec![9; 1]];
    let [a, b, c] = &mut p;
    read3b(&src, &mut [&mut a[..], &mut b[..], &mut c[..]], 0, 1, true).unwrap();
    assert_eq!((p[0][0], p[1][0], p[2][0]), (f(0.1), f(0.2), f(0.3)));
}

#[test]
fn v1_reads_word0_only() {
    let src = flat(&[px(0.5, f32::NAN, f32::NAN, f32::NAN), px(f32::INFINITY, 1.0, 1.0, 1.0)]);
    let mut p = [vec![9u32; 2]];
    let [a] = &mut p;
    read1b(&src, &mut [&mut a[..]], 0, 2, false).unwrap();
    assert_eq!(p[0], vec![f(0.5), 0]);
}

#[test]
fn v1_counts() {
    assert_eq!(range1(0, 0, true), 0);
    assert_eq!(range1(5, 3, false), 0);
    assert_eq!(range1(0, 33, true), 33);
    assert_eq!(range1(-5, 40, false), 45);
    assert_eq!(range1(-5, 40, true), 45);
    assert!(range1(i32::MAX - 20, i32::MAX, true) < 20);
}

#[test]
fn gate_truth_table() {
    assert!(!gate_is_b(0, 0, 0));
    assert!(!gate_is_b(0, 1, 0));
    assert!(gate_is_b(1, 0, 0));
    assert!(!gate_is_b(1, 1, 0));
    assert!(gate_is_b(0, 0, 1));
    assert!(gate_is_b(1, 1, 1));
}

#[test]
fn refuses_short_buffers() {
    let src = flat(&[px(1.0, 1.0, 1.0, 1.0)]);
    let mut p = [vec![0u32; 1], vec![0; 1], vec![0; 1]];
    let [a, b, c] = &mut p;
    assert!(matches!(read3b(&src, &mut [&mut a[..], &mut b[..], &mut c[..]], 0, 2, false), Err(ReadErr::SrcShort { .. })));
}
