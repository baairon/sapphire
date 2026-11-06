use convert_pair::*;

fn p4() -> [Vec<f32>; 4] {
    [Vec::new(), Vec::new(), Vec::new(), Vec::new()]
}

fn read3(mem: &[f32], n: usize) -> [Vec<f32>; 4] {
    let mut p = p4();
    convert_from_source_line_3(mem, &mut p, n);
    p
}

fn bitsv(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

#[test]
fn reader3_premultiplies_reorders_and_leaves_no_alpha_plane() {
    // memory B,G,R,A = 0.5, 0.25, 1.0, 0.5 -> planes R,G,B = 0.5, 0.125, 0.25; no fourth plane
    let p = read3(&[0.5, 0.25, 1.0, 0.5], 1);
    assert_eq!((p[0][0], p[1][0], p[2][0]), (0.5, 0.125, 0.25));
    assert!(p[3].is_empty());
}

#[test]
fn reader3_alpha_classes() {
    // alpha above 1 scales colour up, negative alpha flips the sign, zero alpha gives signed zeros, NaN and inf alpha act as +0.0
    let c = |a: f32| read3(&[1.0, 2.0, 3.0, a], 1);
    let p = c(1.5);
    assert_eq!((p[0][0], p[1][0], p[2][0]), (4.5, 3.0, 1.5));
    let p = c(-0.5);
    assert_eq!((p[0][0], p[1][0], p[2][0]), (-1.5, -1.0, -0.5));
    let p = c(0.0);
    assert_eq!(bitsv(&[p[0][0], p[1][0], p[2][0]]), vec![0, 0, 0]);
    let p = c(-0.0);
    assert_eq!(bitsv(&[p[0][0], p[1][0], p[2][0]]), vec![0x8000_0000; 3]);
    for a in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::from_bits(0x7fa0_0000)] {
        let p = c(a);
        assert_eq!(bitsv(&[p[0][0], p[1][0], p[2][0]]), vec![0, 0, 0]);
    }
}

#[test]
fn reader3_non_finite_colour_becomes_zero_before_the_multiply() {
    let p = read3(&[f32::NAN, f32::INFINITY, f32::NEG_INFINITY, 2.0], 1);
    assert_eq!(bitsv(&[p[0][0], p[1][0], p[2][0]]), vec![0, 0, 0]);
    // negative alpha with a zeroed sample gives -0.0
    let p = read3(&[f32::NAN, 1.0, 1.0, -1.0], 1);
    assert_eq!(p[2][0].to_bits(), 0x8000_0000);
}

#[test]
fn reader3_overflow_gives_infinity() {
    let p = read3(&[0.0, 0.0, 3.0e38, 4.0], 1);
    assert_eq!(p[0][0], f32::INFINITY);
}

#[test]
fn reader3_equals_reader4_on_the_three_colour_planes() {
    // Relation read from the listings (0x7e6e60 is 0x7e8570 without the alpha store): sweep a grid of bit patterns, widths 1..40.
    let pats: [u32; 18] = [
        0, 0x8000_0000, 1, 0x8000_0001, 0x3f80_0000, 0xbf80_0000, 0x3f00_0000, 0x4000_0000, 0x7f7f_ffff, 0xff7f_ffff, 0x7f80_0000, 0xff80_0000, 0x7fc0_0000,
        0xffc0_0000, 0x7fa0_0000, 0x3e80_0000, 0x0080_0000, 0x4b00_0000,
    ];
    for n in 1..40usize {
        let mut mem = Vec::new();
        for i in 0..n {
            for k in 0..4 {
                mem.push(f32::from_bits(pats[(i * 5 + k * 7 + n) % pats.len()]));
            }
        }
        let a = read3(&mem, n);
        let mut b = p4();
        convert_from_source_line(&mem, &mut b, n);
        for k in 0..3 {
            assert_eq!(bitsv(&a[k]), bitsv(&b[k]));
        }
        assert!(a[3].is_empty());
    }
}

#[test]
fn reader3_n0_reads_and_writes_nothing() {
    let p = read3(&[], 0);
    assert!(p.iter().all(|v| v.is_empty()));
}

#[test]
fn writer3_modes() {
    let planes: [Vec<f32>; 4] = [vec![0.1], vec![0.2], vec![0.3], vec![]];
    let pre = f32::from_bits(0xaaaa_aaaa);
    let run = |m: AlphaMode| {
        let mut d = vec![pre; 4];
        let o = convert_to_dest_line_3(&planes, ToDestParams { mode: m }, &mut d, 1);
        (o, d)
    };
    let (o, d) = run(AlphaMode::One);
    assert_eq!(o, DestOutcome::Written);
    assert_eq!(d, vec![0.3, 0.2, 0.1, 1.0]);
    let (o, d) = run(AlphaMode::ClampKeep);
    assert_eq!(o, DestOutcome::Written);
    assert_eq!(d, vec![0.3, 0.2, 0.1, 1.0]);
    let (o, d) = run(AlphaMode::ZeroAlpha);
    assert_eq!(o, DestOutcome::Written);
    assert_eq!(bitsv(&d), bitsv(&[0.3, 0.2, 0.1, 0.0]));
    let (o, d) = run(AlphaMode::NoWrite);
    assert_eq!(o, DestOutcome::NotWritten);
    assert!(d.iter().all(|x| x.to_bits() == 0xaaaa_aaaa));
}

#[test]
fn writer3_equals_writer4_modes_2_and_3() {
    // The 4-plane image reaches the same 0x829df0 body in modes 2 and 3: same bytes for the same three colour planes.
    let n = 19;
    let mut planes: [Vec<f32>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    for k in 0..3 {
        planes[k] = (0..n).map(|i| f32::from_bits(0x3e00_0000u32.wrapping_mul(i as u32 + 1 + k as u32) ^ (k as u32 * 0x1234_5))).collect();
    }
    let mut p3 = planes.clone();
    p3[3] = Vec::new();
    planes[3] = (0..n).map(|i| i as f32 * 0.37 - 2.0).collect();
    for m in [AlphaMode::One, AlphaMode::ZeroAlpha] {
        let mut d3 = vec![f32::from_bits(0xaaaa_aaaa); 4 * n];
        let mut d4 = d3.clone();
        convert_to_dest_line_3(&p3, ToDestParams { mode: m }, &mut d3, n);
        convert_to_dest_line(&planes, ToDestParams { mode: m }, &mut d4, n);
        assert_eq!(bitsv(&d3), bitsv(&d4));
    }
}

#[test]
fn writer3_n0_writes_nothing_for_every_mode() {
    let planes = p4();
    for m in [AlphaMode::NoWrite, AlphaMode::ClampKeep, AlphaMode::ZeroAlpha, AlphaMode::One] {
        let mut d = vec![f32::from_bits(0xaaaa_aaaa); 16];
        convert_to_dest_line_3(&planes, ToDestParams { mode: m }, &mut d, 0);
        assert!(d.iter().all(|x| x.to_bits() == 0xaaaa_aaaa));
    }
}

#[test]
fn frame3_whole_pair() {
    // 2x2 frame, mode 3, cell 1: dest = (B*a, G*a, R*a, 1.0) per pixel
    let src: Vec<f32> = vec![0.5, 0.25, 1.0, 0.5, 1.0, 1.0, 1.0, 1.0, 2.0, 2.0, 2.0, 0.0, 0.1, 0.2, 0.3, 2.0];
    let mut d = vec![f32::from_bits(0xaaaa_aaaa); 16];
    let calls = convert_pair_frame_3(&src, 2, 2, 1, ToDestParams { mode: AlphaMode::One }, &mut d);
    assert_eq!(calls, 2);
    assert_eq!(&d[0..4], &[0.25, 0.125, 0.5, 1.0]);
    assert_eq!(&d[4..8], &[1.0, 1.0, 1.0, 1.0]);
    assert_eq!(bitsv(&d[8..11]), vec![0, 0, 0]);
    assert_eq!(d[11], 1.0);
    assert_eq!(&d[12..16], &[0.2, 0.4, 0.6, 1.0]);
}
