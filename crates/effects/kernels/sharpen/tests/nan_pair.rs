//! Two-NaN operand rule of the sharpen combine job: general path `vsubss` then `vfmadd231ss` (priority x, d, g), unit path
//! `vfmsub213ss` (priority x, 2.0, then the subtracted b, returned quieted and not negated).
use sharpen::combine::{combine, combine_elem};

fn b(x: u32) -> f32 {
    f32::from_bits(x)
}

#[test]
fn general_path_picks_x_then_the_difference_then_the_gain() {
    let g = 0.5;
    // x and b both NaN: x
    assert_eq!(combine_elem(b(0x7fc0_0001), b(0xffc0_0002), g).to_bits(), 0x7fc0_0001);
    assert_eq!(combine_elem(b(0xffc0_0002), b(0x7fc0_0001), g).to_bits(), 0xffc0_0002);
    assert_eq!(combine_elem(b(0x7f80_0005), b(0xffc0_0002), g).to_bits(), 0x7fc0_0005);
    // x finite, b NaN: b (through the difference, quieted)
    assert_eq!(combine_elem(1.0, b(0x7f80_0006), g).to_bits(), 0x7fc0_0006);
    // x finite, b NaN, gain NaN: the difference before the gain
    assert_eq!(combine_elem(1.0, b(0xffc0_0006), b(0x7fc0_0009)).to_bits(), 0xffc0_0006);
    // x NaN and a NaN gain: x
    assert_eq!(combine_elem(b(0x7fc0_0001), 1.0, b(0x7fc0_0009)).to_bits(), 0x7fc0_0001);
    // only the gain is NaN
    assert_eq!(combine_elem(1.0, 2.0, b(0xff80_0009)).to_bits(), 0xffc0_0009);
    // infinity minus infinity is the default NaN, before a NaN gain
    assert_eq!(combine_elem(f32::INFINITY, f32::INFINITY, b(0x7fc0_0009)).to_bits(), 0xffc0_0000);
    assert_eq!(combine_elem(f32::INFINITY, f32::INFINITY, 0.5).to_bits(), 0xffc0_0000);
    // finite
    assert_eq!(combine_elem(1.0, 0.5, 2.0).to_bits(), (0.5f32 * 2.0 + 1.0).to_bits());
}

#[test]
fn unit_path_picks_x_first_and_returns_a_lone_nan_b_with_its_own_sign() {
    assert_eq!(combine_elem(b(0x7fc0_0001), b(0xffc0_0002), 1.0).to_bits(), 0x7fc0_0001);
    assert_eq!(combine_elem(b(0xffc0_0002), b(0x7fc0_0001), 1.0).to_bits(), 0xffc0_0002);
    assert_eq!(combine_elem(b(0x7f80_0005), b(0xffc0_0002), 1.0).to_bits(), 0x7fc0_0005);
    // x NaN alone
    assert_eq!(combine_elem(b(0x7f80_0007), 0.5, 1.0).to_bits(), 0x7fc0_0007);
    // b NaN alone: its own sign and payload, quieted
    assert_eq!(combine_elem(0.5, b(0x7fc0_0003), 1.0).to_bits(), 0x7fc0_0003);
    assert_eq!(combine_elem(0.5, b(0xff80_0003), 1.0).to_bits(), 0xffc0_0003);
    assert_eq!(combine_elem(b(0xbf62_85b9), b(0xffc0_0002), 1.0).to_bits(), 0xffc0_0002);
    assert_eq!(combine_elem(f32::INFINITY, f32::INFINITY, 1.0).to_bits(), 0xffc0_0000);
    assert_eq!(combine_elem(1.5, 0.5, 1.0).to_bits(), 2.5f32.to_bits());
}

#[test]
fn the_subtraction_pick_never_reaches_the_output() {
    // PAIR-INERT probe of `x - b`: whatever the difference carries, a NaN x wins at the fused step, and with a finite x there is one NaN at most
    for (x, bb) in [(0x7fc0_0001u32, 0xffc0_0002u32), (0xffc0_0002, 0x7fc0_0001), (0x7f80_0001, 0x7fc0_0002), (0x7fc0_0002, 0x7f80_0001)] {
        for g in [0.5f32, -2.0, b(0x7fc0_0009)] {
            assert_eq!(combine_elem(b(x), b(bb), g).to_bits(), x | 0x0040_0000);
        }
    }
}

#[test]
fn the_slice_job_equals_the_element_rule_in_both_forms() {
    let n = 19;
    let xs: Vec<f32> = (0..n).map(|i| if i % 4 == 0 { b(0x7f80_0000 + i as u32 + 1) } else { i as f32 * 0.5 }).collect();
    let bs: Vec<f32> = (0..n).map(|i| if i % 3 == 0 { b(0xffc0_0000 + i as u32 + 1) } else { 1.0 - i as f32 }).collect();
    for g in [1.0f32, 0.5, b(0xffc0_1111)] {
        let mut o = vec![0f32; n];
        combine(&xs, &bs, g, &mut o, n).unwrap();
        for i in 0..n {
            assert_eq!(o[i].to_bits(), combine_elem(xs[i], bs[i], g).to_bits());
        }
    }
    let clean_x = [0.5f32, f32::INFINITY, 2.0];
    let clean_b = [0.25f32, f32::INFINITY, -1.0];
    for g in [1.0f32, 0.5] {
        let mut o = [0f32; 3];
        combine(&clean_x, &clean_b, g, &mut o, 3).unwrap();
        assert_eq!(o[1].to_bits(), 0xffc0_0000);
        for i in 0..3 {
            assert_eq!(o[i].to_bits(), combine_elem(clean_x[i], clean_b[i], g).to_bits());
        }
    }
}

#[test]
fn a_nan_in_the_blurred_plane_keeps_its_bits_through_the_unit_and_general_paths() {
    // a plane holding 0x7fc00000 words and a source with a NaN only where the blur has none
    let xs = [0.25f32, 0.5, b(0x7fc0_0000), 1.0];
    let bs = [b(0x7fc0_0000), b(0xffc0_0000), 0.5, b(0x7f80_0001)];
    for (g, expect) in [(1.0f32, [0x7fc0_0000u32, 0xffc0_0000, 0x7fc0_0000, 0x7fc0_0001]), (0.5, [0x7fc0_0000, 0xffc0_0000, 0x7fc0_0000, 0x7fc0_0001])] {
        let mut o = [0f32; 4];
        combine(&xs, &bs, g, &mut o, 4).unwrap();
        for i in 0..4 {
            assert_eq!(o[i].to_bits(), expect[i], "g {g} element {i}");
        }
    }
}
