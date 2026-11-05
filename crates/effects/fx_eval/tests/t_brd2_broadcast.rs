//! BRD2: v3_to_v4_if4 broadcasts a scalar first argument only for exactly 3 output channels; 2 channels keep the concatenation [a0 a1].
use fx_eval::compile::ArgView2;
use fx_eval::fold::fold_lanes;

fn c(l: [f32; 4], n: usize, el: i32) -> ArgView2 {
    ArgView2 { is_const: true, lanes: l, n, elnum: Some(el) }
}
fn s(x: f32) -> ArgView2 {
    c([x, x, x, x], 1, 1)
}

#[test]
fn scalar_offset_reaches_all_three_planes() {
    // SCALE_LIGHTS_OFFSET_DARKS on the 3 channel class-3 layout; 4 channel form is [x x x 0] (PLAN_ORIG_Hue_Sat_Bright_od01 node 16)
    let q = 0.19999695f32;
    let r = fold_lanes("v3_to_v4_if4", &[s(q), s(0.0)], 3, None).expect("folded");
    assert_eq!(&r[0..3], &[q.to_bits(); 3], "all three lanes carry the offset");
}

#[test]
fn two_channel_keeps_concat_of_distinct_scalars() {
    // BCJ3 Blur_Mono originals (8 rows): [0.7416992 0.7319031] stays [a0 a1]
    let (a, b) = (0.7416992f32, 0.7319031f32);
    let r = fold_lanes("v3_to_v4_if4", &[s(a), s(b)], 2, None).expect("folded");
    assert_eq!(&r[0..2], &[a.to_bits(), b.to_bits()], "n = 2 is the concatenation, not the broadcast");
}

#[test]
fn four_channel_form_unchanged() {
    let r = fold_lanes("v3_to_v4_if4", &[s(0.1), s(0.0)], 4, None).expect("folded");
    assert_eq!(r, [0.1f32.to_bits(), 0.1f32.to_bits(), 0.1f32.to_bits(), 0]);
}

#[test]
fn blur_mono_one_one_same() {
    let r = fold_lanes("v3_to_v4_if4", &[s(1.0), s(1.0)], 2, None).expect("folded");
    assert_eq!(&r[0..2], &[1.0f32.to_bits(); 2]);
}

#[test]
fn vector_first_argument_keeps_concat_at_three() {
    let r = fold_lanes("v3_to_v4_if4", &[c([1.0, 2.0, 0.0, 0.0], 2, 2), s(7.0)], 3, None).expect("folded");
    assert_eq!(&r[0..3], &[1.0f32.to_bits(), 2.0f32.to_bits(), 7.0f32.to_bits()]);
}
