//! Two-NaN operand rule of the masked_blur arithmetic sites: the first source of the listing's instruction, quieted.
use masked_blur::{combine, combine_px, premultiply_in_place, premultiply_px, MASK_THRESHOLD};

fn b(x: u32) -> f32 {
    f32::from_bits(x)
}
const THR: f32 = MASK_THRESHOLD;

#[test]
fn premultiply_returns_the_clamped_mask_value_when_both_operands_are_nan() {
    assert_eq!(premultiply_px(b(0x7fc0_0001), b(0xffc0_0002)).to_bits(), 0x7fc0_0001);
    assert_eq!(premultiply_px(b(0xffc0_0002), b(0x7fc0_0001)).to_bits(), 0xffc0_0002);
    // a signalling mask value is quieted
    assert_eq!(premultiply_px(b(0x7f80_0005), b(0xffc0_0002)).to_bits(), 0x7fc0_0005);
    // one NaN operand: that operand, quieted
    assert_eq!(premultiply_px(0.5, b(0x7f80_0007)).to_bits(), 0x7fc0_0007);
    assert_eq!(premultiply_px(b(0x7f80_0007), 0.5).to_bits(), 0x7fc0_0007);
    // a fresh NaN is the default NaN
    assert_eq!(premultiply_px(0.0, f32::INFINITY).to_bits(), 0xffc0_0000);
}

#[test]
fn premultiply_bulk_equals_the_element_rule() {
    let m = [b(0x7fc0_0001), 0.5, b(0x7f80_0009), -1.0, 2.0, f32::NAN, 0.25, 0.0];
    let d = [b(0xffc0_0002), b(0xff80_0003), 1.0, f32::INFINITY, b(0xffc0_0004), 0.5, f32::INFINITY, f32::INFINITY];
    let mut o = d;
    premultiply_in_place(&mut o, &m);
    for i in 0..m.len() {
        assert_eq!(o[i].to_bits(), premultiply_px(m[i], d[i]).to_bits(), "element {i}");
    }
    let mut o = [1.0f32, 2.0, f32::INFINITY, 0.5];
    premultiply_in_place(&mut o, &[0.5, 0.0, 0.0, 2.0]);
    assert_eq!([o[0].to_bits(), o[1].to_bits(), o[2].to_bits(), o[3].to_bits()], [0.5f32.to_bits(), 0.0f32.to_bits(), 0xffc0_0000, 0.5f32.to_bits()]);
}

#[test]
fn combine_picks_the_source_first_then_the_difference() {
    // b and s both NaN: the fused step returns s (operand 1)
    assert_eq!(combine_px(0.5, b(0x7fc0_0001), 1.0, b(0xffc0_0002), THR).to_bits(), 0xffc0_0002);
    assert_eq!(combine_px(0.5, b(0xffc0_0002), 1.0, b(0x7fc0_0001), THR).to_bits(), 0x7fc0_0001);
    // a signalling source is quieted
    assert_eq!(combine_px(0.5, b(0xffc0_0002), 1.0, b(0x7f80_0005), THR).to_bits(), 0x7fc0_0005);
    // s finite, b NaN: the quieted b (through the difference)
    assert_eq!(combine_px(0.5, b(0x7f80_0006), 1.0, 0.25, THR).to_bits(), 0x7fc0_0006);
    // s NaN, b finite
    assert_eq!(combine_px(0.5, 3.0, 1.0, b(0x7f80_0008), THR).to_bits(), 0x7fc0_0008);
    // m not above the threshold (or NaN) returns the source untouched, NaN payload included
    assert_eq!(combine_px(0.0, b(0x7fc0_0001), 1.0, b(0x7f80_0005), THR).to_bits(), 0x7f80_0005);
    assert_eq!(combine_px(f32::NAN, 1.0, 1.0, b(0x7f80_0005), THR).to_bits(), 0x7f80_0005);
    // infinity minus infinity is the default NaN
    assert_eq!(combine_px(0.5, f32::INFINITY, 1.0, f32::INFINITY, THR).to_bits(), 0xffc0_0000);
}

#[test]
fn combine_divisor_is_never_a_nan_and_a_nan_weight_skips_the_division() {
    // PAIR-INERT probe of the divide: w NaN leaves b as it is, w positive divides by min(w,1)
    assert_eq!(combine_px(0.5, b(0x7fc0_0003), f32::NAN, 1.0, THR).to_bits(), 0x7fc0_0003);
    assert_eq!(combine_px(0.5, b(0xffc0_0003), 0.5, 1.0, THR).to_bits(), 0xffc0_0003);
    assert_eq!(combine_px(0.5, b(0xffc0_0003), f32::INFINITY, 1.0, THR).to_bits(), 0xffc0_0003);
    assert_eq!(combine_px(0.5, f32::INFINITY, 1.0e-45, 1.0, THR).to_bits(), f32::INFINITY.to_bits());
    assert_eq!(combine_px(0.5, 1.0, 0.5, 0.0, THR).to_bits(), (2.0f32 * 0.5).to_bits());
}

#[test]
fn combine_plane_job_equals_the_element_rule() {
    let n = 12;
    let m: Vec<f32> = (0..n).map(|i| if i % 5 == 0 { 0.0 } else { 0.25 + i as f32 / 20.0 }).collect();
    let bb: Vec<f32> = (0..n).map(|i| if i % 2 == 0 { b(0x7fc0_0000 + i as u32 + 1) } else { i as f32 }).collect();
    let w: Vec<f32> = (0..n).map(|i| if i % 7 == 3 { f32::NAN } else { 0.5 }).collect();
    let s: Vec<f32> = (0..n).map(|i| if i % 3 == 0 { b(0xff80_0000 + i as u32 + 1) } else { 1.0 - i as f32 }).collect();
    let mut o = vec![0f32; n];
    combine(&mut o, &m, &bb, &w, &s, THR);
    for i in 0..n {
        assert_eq!(o[i].to_bits(), combine_px(m[i], bb[i], w[i], s[i], THR).to_bits());
    }
}
