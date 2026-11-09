//! Fresh NaN words of the box filter are the x86 words on every processor: the fractional part of an infinite radius and the mean
//! of an empty window (0 / 0) are NaNs that no operand supplies, and they are 0xffc00000 on x86 and now on ARM.
use edge_aware_sharpen_box_filter::{cb_cell_a, filter3, Rec};
use std::hint::black_box as bb;

#[test]
fn infinite_radius_gives_the_x86_fresh_nan_in_the_third_parameter() {
    let p = cb_cell_a(bb(f32::INFINITY));
    assert_eq!(p[0].to_bits(), 0x7f80_0000);
    assert_eq!(p[2].to_bits(), 0xffc0_0000);
    let q = cb_cell_a(bb(f32::NEG_INFINITY));
    assert_eq!(q[0].to_bits(), 0xff80_0000);
    assert_eq!(q[2].to_bits(), 0xffc0_0000);
    // a finite radius is untouched
    let r = cb_cell_a(2.5);
    assert_eq!((r[0], r[2]), (3.0, 0.5));
}

/// The guide of NaN in the middle empties the window of the last element: its mean is 0 / 0.
fn line(guide: [f32; 3], r: f32) -> Vec<u32> {
    let n = 3usize;
    let mut mem = vec![0.0f32; 3 * n];
    mem[..n].copy_from_slice(&[1.0, 2.0, 3.0]);
    mem[n..2 * n].copy_from_slice(&guide);
    let mut rec = Rec::default();
    filter3(&mut mem, 0, 2 * n, n, n as i64, cb_cell_a(r), &mut rec);
    mem[2 * n..].iter().map(|x| x.to_bits()).collect()
}

#[test]
fn empty_window_mean_is_the_x86_fresh_nan() {
    let got = line([0.0, f32::NAN, 5.0], 2.0);
    assert_eq!(got[2], 0xffc0_0000, "{got:08x?}");
    // the other two elements are finite and the same on every processor
    assert!(f32::from_bits(got[0]).is_finite() && f32::from_bits(got[1]).is_finite(), "{got:08x?}");
}

#[test]
fn infinite_radius_line_matches_x86_words() {
    // an infinite radius makes p2 a fresh NaN, which the fused step then returns for every element
    let got = line([0.0, 1.0, 2.0], bb(f32::INFINITY));
    assert!(got.iter().all(|b| *b == 0xffc0_0000), "{got:08x?}");
}
