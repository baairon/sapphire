use readers14::*;

fn f(x: f32) -> u32 {
    x.to_bits()
}

fn px(a: f32, r: f32, g: f32, b: f32) -> [u32; 4] {
    [f(a), f(r), f(g), f(b)]
}

fn run4(src: &[u32], n: usize, s: i32, e: i32, block: bool) -> ([Vec<u32>; 4], Result<(), ReadErr>) {
    let mut p: [Vec<u32>; 4] = [vec![0xdead_beef; n], vec![0xdead_beef; n], vec![0xdead_beef; n], vec![0xdead_beef; n]];
    let r = {
        let [a, b, c, d] = &mut p;
        read4(src, &mut [&mut a[..], &mut b[..], &mut c[..], &mut d[..]], s, e, block)
    };
    (p, r)
}

#[test]
fn r4_weights_colour_by_alpha_and_stores_alpha() {
    let src: Vec<u32> = [px(0.5, 1.0, 2.0, 4.0), px(2.0, 3.0, -1.0, 0.25)].concat();
    for block in [false, true] {
        let (p, r) = run4(&src, 2, 0, 2, block);
        assert_eq!(r, Ok(()));
        assert_eq!(p[0], vec![f(0.5), f(6.0)]);
        assert_eq!(p[1], vec![f(1.0), f(-2.0)]);
        assert_eq!(p[2], vec![f(2.0), f(0.5)]);
        assert_eq!(p[3], vec![f(0.5), f(2.0)]);
    }
}

#[test]
fn r4_non_finite_becomes_plus_zero_before_the_multiply() {
    // alpha NaN -> 0; colour inf -> 0; 0 * x = +0 (or -0 for a negative colour times +0 alpha: fin(-1) * +0 = -0)
    let src: Vec<u32> = [px(f32::NAN, 1.0, 1.0, 1.0), px(1.0, f32::INFINITY, f32::NEG_INFINITY, f32::NAN), px(0.0, -1.0, 2.0, 3.0)].concat();
    let (p, r) = run4(&src, 3, 0, 3, false);
    assert_eq!(r, Ok(()));
    assert_eq!(p[0], vec![f(0.0), f(0.0), f(-0.0)]);
    assert_eq!(p[1], vec![f(0.0), f(0.0), f(0.0)]);
    assert_eq!(p[3], vec![f(0.0), f(1.0), f(0.0)]);
}

#[test]
fn counter_semantics_start_end_and_source_not_offset() {
    let src: Vec<u32> = [px(1.0, 1.0, 1.0, 1.0), px(1.0, 2.0, 2.0, 2.0), px(1.0, 3.0, 3.0, 3.0)].concat();
    // start 5, end 7: two pixels, taken from the FIRST two source pixels, written at plane index 0 and 1
    let (p, r) = run4(&src, 2, 5, 7, false);
    assert_eq!(r, Ok(()));
    assert_eq!(p[0], vec![f(1.0), f(2.0)]);
    // empty and reversed ranges write nothing
    for (s, e) in [(3, 3), (4, 3), (0, -1), (i32::MAX, 0)] {
        let (p, r) = run4(&[], 0, s, e, true);
        assert_eq!(r, Ok(()));
        assert!(p[0].is_empty());
    }
}

#[test]
fn block_loop_counter_overflow_is_refused_plain_loop_completes() {
    let src: Vec<u32> = (0..15).flat_map(|_| px(1.0, 1.0, 1.0, 1.0)).collect();
    let (_, r) = run4(&src, 15, i32::MAX - 15, i32::MAX, true);
    assert_eq!(r, Err(ReadErr::BlockCounterOverflow));
    let (p, r) = run4(&src, 15, i32::MAX - 15, i32::MAX, false);
    assert_eq!(r, Ok(()));
    assert_eq!(p[3], vec![f(1.0); 15]);
}

#[test]
fn short_buffers_are_refused_before_any_write() {
    let src: Vec<u32> = px(1.0, 1.0, 1.0, 1.0).to_vec();
    let (p, r) = run4(&src, 1, 0, 2, false);
    assert!(matches!(r, Err(ReadErr::SrcShort { .. })));
    assert_eq!(p[0], vec![0xdead_beef]);
}

#[test]
fn r3_stores_three_planes_and_never_alpha() {
    let src: Vec<u32> = [px(0.5, 1.0, 2.0, 4.0)].concat();
    let mut p = [vec![0xdead_beefu32; 1], vec![0xdead_beef; 1], vec![0xdead_beef; 1]];
    let [a, b, c] = &mut p;
    read3(&src, &mut [&mut a[..], &mut b[..], &mut c[..]], 0, 1, true).unwrap();
    assert_eq!((p[0][0], p[1][0], p[2][0]), (f(0.5), f(1.0), f(2.0)));
}

#[test]
fn r1_copies_the_first_word_only() {
    let src: Vec<u32> = [px(f32::NAN, 9.0, 9.0, 9.0), px(-3.5, 9.0, 9.0, 9.0), px(f32::INFINITY, 1.0, 1.0, 1.0)].concat();
    for block in [false, true] {
        let mut p = [vec![0xdead_beefu32; 3]];
        let [a] = &mut p;
        read1(&src, &mut [&mut a[..]], 0, 3, block).unwrap();
        assert_eq!(p[0], vec![0, f(-3.5), 0]);
    }
}
