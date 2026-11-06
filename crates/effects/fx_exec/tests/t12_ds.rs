//! unit tests of the exec-downsampled kernels (std only): shape rules, exact cases, value classes, and the V/S class table.
use fx_exec::downsampled::*;

#[test]
fn dims_and_levels() {
    assert_eq!(small_dims(64, 32, 1), (32, 16));
    assert_eq!(small_dims(64, 32, 2), (16, 8));
    assert_eq!(small_dims(50, 7, 1), (25, 4));
    assert_eq!(small_dims(50, 7, 2), (13, 2));
    assert_eq!(levels_of_factor(1), 0);
    assert_eq!(levels_of_factor(2), 1);
    assert_eq!(levels_of_factor(4), 2);
}

#[test]
fn constant_plane_down_is_c_plus_bias_term() {
    // a constant plane: out = c + BIAS / 16 (the bias is added once inside the sum, before the 1/16 factor)
    let c = 0.5f32;
    let big = vec![c; 20 * 6];
    let o = down_plane(&big, 20, 6, 0);
    assert_eq!(o.len(), 10 * 3);
    for v in o {
        assert!((v - (c + 0.0625 * BIAS)).abs() < 1e-7, "{}", v);
    }
}

#[test]
fn up_copies_even_samples_and_averages_odd() {
    let small: Vec<f32> = (0..12).map(|i| i as f32).collect(); // 4 x 3
    let big = up_plane(&small, 4, 3, 7, 5);
    assert_eq!(big.len(), 35);
    assert_eq!(big[0], 0.0);
    assert_eq!(big[2], 1.0);
    assert_eq!(big[1], 0.5);
    assert_eq!(big[6], 3.0); // tail repeats the last sample
    assert_eq!(big[14], 4.0); // row 2 = small row 1: its first sample is 4
    assert_eq!(big[7], (big[0] + big[14]) * 0.5); // row 1 = average of rows 0 and 2
}

#[test]
fn up_even_height_repeats_last_row() {
    let small = vec![1.0f32, 2.0, 3.0, 4.0]; // 2 x 2
    let big = up_plane(&small, 2, 2, 4, 4);
    assert_eq!(&big[12..16], &big[8..12]); // row 3 = row 2 (caller row copy)
}

#[test]
fn v_classes_table() {
    assert!(v_classes(11, 4).iter().all(|&v| !v)); // n < 16: all scalar
    let c = v_classes(23, 4); // 16 <= n < 29: no peel, one 16 wide block, rest scalar (7 < 8)
    assert_eq!(c.iter().filter(|&&v| v).count(), 16);
    let c = v_classes(30, 4 + 128 * 0); // n >= 29, row address + 4 = 4 mod 32: head 7, one 16 block, remainder 7 scalar
    assert_eq!(c.iter().filter(|&&v| v).count(), 16);
    assert!(c[..7].iter().all(|&v| !v));
    let c = v_classes(28, 4);
    assert_eq!(c.iter().filter(|&&v| v).count(), 24); // 16 wide block + one 8 wide block, 4 scalar
}

#[test]
fn level_roundtrip_shapes() {
    let big: Vec<f32> = (0..50 * 7).map(|i| (i % 13) as f32 * 0.1).collect();
    let (s, w2, h2) = downsample_plane(&big, 50, 7, 2, 0);
    assert_eq!((w2, h2), (13, 2));
    assert_eq!(s.len(), 26);
    let u = upsample_plane(&s, 50, 7, 2);
    assert_eq!(u.len(), 350);
}
