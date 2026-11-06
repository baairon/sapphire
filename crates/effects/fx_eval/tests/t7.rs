//! T7 (FX-1): constant folds of the coerce family follow the member passes of ops_coerce.
use fx_eval::compile::ArgView2;
use fx_eval::fold::*;

fn cv(l: &[f32]) -> ArgView2 {
    // the compiler views replicate the last lane into the unused lanes (the bug of NV-3b to NV-3e depended on it)
    let mut lanes = [0f32; 4];
    for i in 0..4 {
        lanes[i] = l[i.min(l.len() - 1)];
    }
    ArgView2 { is_const: true, lanes, n: l.len(), elnum: Some(l.len() as i32) }
}

fn img(n: i32) -> ArgView2 {
    ArgView2 { is_const: false, lanes: [0.0; 4], n: n as usize, elnum: Some(n) }
}

fn bits(l: [f32; 4]) -> [u32; 4] {
    [l[0].to_bits(), l[1].to_bits(), l[2].to_bits(), l[3].to_bits()]
}

fn f(class: &str, x: &[f32], out: i32) -> [u32; 4] {
    fold_lanes(class, &[cv(x), img(out)], out, None).unwrap()
}

#[test]
fn coerce_up_three_to_four_alpha_is_one() {
    // MEASURED (PLAN_ORIG_TrailsCU_V3): (0.5, 0.25, 0.125) -> (0.5, 0.25, 0.125, 1.0); the FX-1 bug gave alpha 0.125
    assert_eq!(f("coerce_up", &[0.5, 0.25, 0.125], 4), bits([0.5, 0.25, 0.125, 1.0]));
    // the Trails fade_color fcc3 value times fade 0.8: the alpha lane of the constant is 1
    assert_eq!(f("coerce_up", &[1.0, 0.5, 0.25], 4), bits([1.0, 0.5, 0.25, 1.0]));
}

#[test]
fn coerce_up_scalar_and_two_lane() {
    // MEASURED (S1, V2)
    assert_eq!(f("coerce_up", &[0.5], 4), bits([0.5; 4]));
    assert_eq!(f("coerce_up", &[0.5, 0.25], 4), bits([0.5, 0.5, 0.5, 0.25]));
    // rows (1,2), (1,3), (2,3) of the combo table: D_CELL_A, D_CELL_A, DE_CELL_A
    assert_eq!(f("coerce_up", &[0.5], 2), bits([0.5, 0.5, 0.0, 0.0]));
    assert_eq!(f("coerce_up", &[0.5], 3), bits([0.5, 0.5, 0.5, 0.0]));
    assert_eq!(f("coerce_up", &[0.5, 0.25], 3), bits([0.5, 0.25, 0.0, 0.0]));
}

#[test]
fn coerce_down_and_same_count_copy_first_channels() {
    // C_CELL_A rows (4,3), (4,1), (3,1), (2,1), (4,4)
    assert_eq!(f("coerce_to", &[1.0, 2.0, 3.0, 4.0], 3), bits([1.0, 2.0, 3.0, 0.0]));
    assert_eq!(f("coerce_to", &[1.0, 2.0, 3.0, 4.0], 1), bits([1.0, 0.0, 0.0, 0.0]));
    assert_eq!(f("coerce_to", &[1.0, 2.0], 1), bits([1.0, 0.0, 0.0, 0.0]));
    assert_eq!(f("coerce_to", &[1.0, 2.0, 3.0, 4.0], 4), bits([1.0, 2.0, 3.0, 4.0]));
    // EF_CELL_A row (3,4) of coerce_to: alpha 1
    assert_eq!(f("coerce_to", &[1.0, 2.0, 3.0], 4), bits([1.0, 2.0, 3.0, 1.0]));
}

#[test]
fn coerce_to_v3_rows() {
    assert_eq!(f("coerce_to_v3", &[0.25], 3), bits([0.25, 0.25, 0.25, 0.0]));
    assert_eq!(f("coerce_to_v3", &[0.25, 0.5], 3), bits([0.25, 0.5, 0.0, 0.0]));
    assert_eq!(f("coerce_to_v3", &[1.0, 2.0, 3.0, 4.0], 3), bits([1.0, 2.0, 3.0, 0.0]));
}

#[test]
fn coerce_up_a1_and_v4_pad_with_one() {
    // E_CELL_A rows (1,2) and (1,4): channel 0 then 1.0; row (1,3) is D_CELL_A (splat)
    assert_eq!(f("coerce_up_a1", &[0.5], 4), bits([0.5, 0.5, 0.5, 1.0]));
    assert_eq!(f("coerce_up_a1", &[0.5], 2), bits([0.5, 1.0, 0.0, 0.0]));
    assert_eq!(f("coerce_up_a1", &[0.5], 3), bits([0.5, 0.5, 0.5, 0.0]));
    // the other rows equal coerce_up
    assert_eq!(f("coerce_up_a1", &[0.5, 0.25, 0.125], 4), bits([0.5, 0.25, 0.125, 1.0]));
    assert_eq!(f("coerce_up_a1", &[0.5, 0.25], 4), bits([0.5, 0.5, 0.5, 0.25]));
    assert_eq!(f("coerce_to_v4", &[0.5], 4), bits([0.5, 0.5, 0.5, 1.0]));
    assert_eq!(f("coerce_to_v4", &[0.5, 0.25, 0.125], 4), bits([0.5, 0.25, 0.125, 1.0]));
}

#[test]
fn coerce_to_alpha_takes_channel_zero_or_the_last() {
    // combos 1 and 3: C_CELL_A tag 1 (channel 0); combos 2 and 4: P_CELL_A (last channel)
    let r = |x: &[f32]| fold_lanes("coerce_to_alpha", &[cv(x)], 1, None).unwrap();
    assert_eq!(r(&[0.5]), bits([0.5, 0.0, 0.0, 0.0]));
    assert_eq!(r(&[0.5, 0.25]), bits([0.25, 0.0, 0.0, 0.0]));
    assert_eq!(r(&[0.5, 0.25, 0.125]), bits([0.5, 0.0, 0.0, 0.0]));
    assert_eq!(r(&[0.5, 0.25, 0.125, 0.0625]), bits([0.0625, 0.0, 0.0, 0.0]));
}

#[test]
fn old_behaviour_is_switchable() {
    // fold_kind lists the extra classes unless FX1_COERCE_FOLD_OLD=1 (not set in the test process)
    assert!(!coerce_old());
    for c in ["coerce_up_a1", "coerce_to_v4", "coerce_to_alpha", "coerce_up_to_v3"] {
        assert!(fold_kind(c).is_some(), "{}", c);
    }
    // an image first argument never folds
    assert!(!can_fold("coerce_up", &[img(3), img(4)]));
    assert!(!can_fold("coerce_up_a1", &[img(3), img(4)]));
}
