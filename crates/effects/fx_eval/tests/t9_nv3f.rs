//! NV-3f tests: constant paths and collapses for the non-Milestone programs (values MEASURED in the EP-3c / EP-3d original dumps).
use fx_eval::compile::{ArgView2, ComboTable};
use fx_eval::fold::{collapse_arm, fold_kind, fold_lanes};
use fx_eval::plan::fmt_g6;

fn c(l: [f32; 4], n: usize, el: i32) -> ArgView2 {
    ArgView2 { is_const: true, lanes: l, n, elnum: Some(el) }
}
fn s(x: f32) -> ArgView2 {
    c([x, x, x, x], 1, 1)
}
fn img(el: i32) -> ArgView2 {
    ArgView2 { is_const: false, lanes: [0.0; 4], n: el as usize, elnum: Some(el) }
}
fn bits(r: Option<[u32; 4]>) -> [u32; 4] {
    r.expect("folded")
}

#[test]
fn channels_equal_constant() {
    // Rays a: channels_equal [0 0 0] folds to the lane 1.0 (0x3f800000)
    assert_eq!(bits(fold_lanes("channels_equal", &[c([0.0; 4], 3, 3)], 1, None))[0], 0x3f800000);
    assert_eq!(bits(fold_lanes("channels_equal", &[c([0.25, 0.25, 0.25, 0.0], 3, 3)], 1, None))[0], 0x3f800000);
    assert_eq!(bits(fold_lanes("channels_equal", &[c([0.25, 0.5, 0.25, 0.0], 3, 3)], 1, None))[0], 0);
}

#[test]
fn element_of_constant() {
    // EdgeRays: element_of [0.25 0.25 0.25] 1 = 0.25 (0x3e800000)
    assert_eq!(bits(fold_lanes("element_of", &[c([0.25, 0.25, 0.25, 0.0], 3, 3), s(1.0)], 1, None))[0], 0x3e800000);
    assert_eq!(bits(fold_lanes("element_of", &[c([1.0, 2.0, 3.0, 0.0], 3, 3), s(2.0)], 1, None))[0], 3.0f32.to_bits());
    // CMP3 MEASURED (original compile of constructed constants, work/eo, 48 of 48 rows): the index is truncated and taken modulo the lane count
    // (floor-modulo, so -1 is the last lane); index 9 of a 3-lane vector is lane 0 (this assertion was an INFERRED clamp to the last lane before)
    assert_eq!(bits(fold_lanes("element_of", &[c([1.0, 2.0, 3.0, 0.0], 3, 3), s(9.0)], 1, None))[0], 1.0f32.to_bits());
    assert_eq!(bits(fold_lanes("element_of", &[c([1.0, 2.0, 3.0, 0.0], 3, 3), s(-1.0)], 1, None))[0], 3.0f32.to_bits());
    assert_eq!(bits(fold_lanes("element_of", &[c([1.0, 2.0, 3.0, 4.0], 4, 4), s(9.0)], 1, None))[0], 2.0f32.to_bits());
}

#[test]
fn division_by_zero_and_fps() {
    // Rays a: 1 / [0 0 0] is 1e6 per lane (0x49742400)
    let r = bits(fold_lanes("/", &[s(1.0), c([0.0; 4], 3, 3)], 3, None));
    assert_eq!(&r[0..3], &[0x49742400, 0x49742400, 0x49742400]);
    // mag of three 1e6 lanes (Rays a: 0x49d36e97)
    let m = bits(fold_lanes("mag", &[c([1.0e6, 1.0e6, 1.0e6, 0.0], 3, 3)], 1, None));
    assert_eq!(m[0], 0x49d36e97);
    // ordinary division is untouched
    assert_eq!(bits(fold_lanes("/", &[s(1.0), s(4.0)], 1, None))[0], 0.25f32.to_bits());
    // get_frames_per_second is 30 (0x41f00000) in the minimal context
    assert_eq!(bits(fold_lanes("get_frames_per_second", &[], 1, None))[0], 0x41f00000);
    assert!(fold_kind("get_frames_per_second").is_some());
}

#[test]
fn shake_family() {
    let r = bits(fold_lanes("shake_shift", &[s(0.0), s(0.0), s(0.0)], 1, None));
    assert_eq!(r[0], 0xffc00000);
    assert_eq!(bits(fold_lanes("random_sawtooth", &[s(3.0), s(0.0)], 1, None))[0], 0);
    assert!(fold_lanes("random_sawtooth", &[s(3.0), s(0.5)], 1, None).is_none());
    assert_eq!(fmt_g6(f32::from_bits(0xffc00000) as f64), "nan");
    assert_eq!(fmt_g6(f64::INFINITY), "inf");
}

#[test]
fn v3_to_v4_if4_not_four_channels_and_scalar_broadcast() {
    // Blur_Mono: v3_to_v4_if4 1 1 <2-channel donor> is [1 1] with 2 channels
    let r = bits(fold_lanes("v3_to_v4_if4", &[s(1.0), s(1.0), img(2)], 2, None));
    assert_eq!((r[0], r[1]), (0x3f800000, 0x3f800000));
    // Blur_Color: scalar first argument, 4 channels: [1 1 1 1]
    let r = bits(fold_lanes("v3_to_v4_if4", &[s(1.0), s(1.0), img(4)], 4, None));
    assert_eq!(r, [0x3f800000; 4]);
    // a genuine v3 first argument keeps its lanes
    let r = bits(fold_lanes("v3_to_v4_if4", &[c([0.1, 0.2, 0.3, 0.0], 3, 3), s(0.5), img(4)], 4, None));
    assert_eq!(r[3], 0.5f32.to_bits());
    assert_eq!(r[1], 0.2f32.to_bits());
}

#[test]
fn big_mult_collapses_like_star() {
    // big_mult slot 38 0x4d0d60: the zero constant absorbs the image
    let a = [img(1), s(0.0)];
    assert_eq!(collapse_arm("big_mult", &a, Some(1)), Some(1));
    let a = [s(0.0), img(1)];
    assert_eq!(collapse_arm("big_mult", &a, Some(1)), Some(0));
    let a = [img(1), s(1.0)];
    assert_eq!(collapse_arm("big_mult", &a, Some(1)), Some(0));
}

#[test]
fn noise_combo_rows() {
    if !effects_vecdir::have_all(&["fx_eval/data/COMBOS.tsv"]) {
        return;
    }
    // noise slot 60 0x51ef20 (static read): (1,1,2,1) and (2,1,2,1) give 1 channel
    let t = ComboTable::from_tsv(&std::fs::read_to_string(effects_vecdir::path("fx_eval/data/COMBOS.tsv")).unwrap_or_default());
    assert_eq!(t.select("noise", &[1, 1, 2, 1]), Some(1));
    assert_eq!(t.select("noise", &[2, 1, 2, 1]), Some(1));
    assert_eq!(t.select("noise3", &[1, 1, 2, 1]), Some(3));
}
