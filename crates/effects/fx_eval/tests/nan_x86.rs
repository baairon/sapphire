//! Fresh NaN words of the unit conversion and the constant folds are the x86 words on every processor: an invalid operation
//! (infinity times zero, infinity minus infinity, zero over zero) gives 0xffc00000 on x86 and 0x7fc00000 on AArch64 unless the
//! operation goes through `fx_common::x86::nx_*`.
use fx_eval::compile::ArgView2;
use fx_eval::fold::fold_lanes;
use fx_eval::units::{conv_record, normalize_scale_record, registered_ns_host, units_width, ConvIn, RatioArm};

use std::hint::black_box as bb;

const INF: f32 = f32::INFINITY;

fn av(lanes: &[f32]) -> ArgView2 {
    let mut l = [0.0f32; 4];
    l[..lanes.len()].copy_from_slice(lanes);
    ArgView2 { is_const: true, lanes: l, n: lanes.len(), elnum: Some(lanes.len() as i32) }
}

fn fold(class: &str, args: &[&[f32]], n: i32) -> [u32; 4] {
    let a: Vec<ArgView2> = args.iter().map(|x| av(bb(x))).collect();
    fold_lanes(class, &a, n, None).unwrap()
}

fn conv(arm: RatioArm, conv_in_b: f32, a: f64, b: f64, c: f64, ns: f32, lane: u32, host: f32) -> u32 {
    let ci = bb(ConvIn { w: 64, conv_in_b, arm, conv_in_a: a, d_cell_b: b, d_cell_c: c, ptype: 1, ns, kind: 5, flags: 0, lane, host });
    conv_record(&ci).unwrap().to_bits()
}

const EXPECT_CONV: &[u32] = &[0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000, 0xffc00000];
const EXPECT_FOLD: &[u32] = &[
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0xffc00000,
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
    0xffc00000, 0xffc00000, 0x00000000, 0x00000000,
    0xffc00000, 0xffc00000, 0x00000000, 0x00000000,
    0x7f800000, 0xffc00000, 0x00000000, 0x00000000,
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
    0xffc00000, 0x00000000, 0x00000000, 0x00000000,
];

#[test]
fn unit_conversion_fresh_nan_words_are_the_x86_words() {
    let got = vec![
        // infinite host value, zero scale: infinity times zero
        conv(RatioArm::Plain, 1.0, 1.0, 1.0, 1.0, 0.0, 0, INF),
        // lane 1: an infinite numerator over a negated infinite denominator
        conv(RatioArm::Ratio, INF, 1.0, f64::INFINITY, 1.0, 1.0, 1, 1.0),
        // the ratio of two infinite cells
        conv(RatioArm::Ratio, 1.0, f64::INFINITY, f64::INFINITY, 1.0, 1.0, 0, 1.0),
        units_width(bb(64), bb(f64::INFINITY), bb(f64::INFINITY)).to_bits(),
        normalize_scale_record(bb(INF), bb(1.0), bb(0.0), false).to_bits(),
        normalize_scale_record(bb(1.0), bb(0.0), bb(0.0), true).to_bits(),
        registered_ns_host(bb(0.0), bb(0.0)).to_bits(),
    ];
    assert_eq!(format!("{got:08x?}"), format!("{EXPECT_CONV:08x?}"));
}

#[test]
fn fold_fresh_nan_words_are_the_x86_words() {
    let z = 0.0f32;
    let mut got: Vec<u32> = Vec::new();
    got.extend(fold("*", &[&[INF], &[z]], 1));
    got.extend(fold("+", &[&[INF], &[-INF]], 1));
    got.extend(fold("-", &[&[INF], &[INF]], 1));
    got.extend(fold("/", &[&[INF], &[INF]], 1));
    got.extend(fold("mod", &[&[INF], &[2.0]], 1));
    got.extend(fold("screen", &[&[INF], &[z]], 1));
    got.extend(fold("round", &[&[INF], &[INF]], 1));
    got.extend(fold("round", &[&[1.0, 1.0, 1.0], &[2.0, 2.0, 2.0, INF]], 4));
    got.extend(fold("interp", &[&[z], &[INF], &[z]], 1));
    got.extend(fold("cmult", &[&[INF, z], &[z, z]], 2));
    got.extend(fold("cdiv", &[&[z, z], &[z, z]], 2));
    got.extend(fold("cexp", &[&[INF, z]], 2));
    got.extend(fold("dist", &[&[INF], &[INF]], 1));
    got.extend(fold("slow_in_out", &[&[0.5], &[INF], &[z], &[z]], 1));
    assert_eq!(format!("{got:08x?}"), format!("{EXPECT_FOLD:08x?}"));
}
