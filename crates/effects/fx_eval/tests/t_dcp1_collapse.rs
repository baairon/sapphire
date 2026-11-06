//! DCP1: slot-38 collapse rules of v3_to_v4_if4 (low bit of the node's own element count) and postdivide
//! (collapse only when copy_alpha < 0 and the tag is 1 or 3), discriminating each choice.
use fx_eval::fold::{collapse_arm, copy_alpha_negative};
use fx_eval::ArgView2;

fn img(el: i32) -> ArgView2 {
    ArgView2 { is_const: false, lanes: [0.0; 4], n: el as usize, elnum: Some(el) }
}
fn cst(v: f32) -> ArgView2 {
    ArgView2 { is_const: true, lanes: [v; 4], n: 1, elnum: Some(1) }
}

#[test]
fn v3_to_v4_if4_collapses_on_odd_element_count() {
    let a = [img(3), img(1), img(3)];
    assert_eq!(collapse_arm("v3_to_v4_if4", &a, Some(3)), Some(0)); // donor 3 collapses
    assert_eq!(collapse_arm("v3_to_v4_if4", &a, Some(4)), None); // donor 4 keeps the node
    assert_eq!(collapse_arm("v3_to_v4_if4", &a, Some(2)), None); // even count (2 lanes) keeps the node
    assert_eq!(collapse_arm("v3_to_v4_if4", &a, Some(1)), Some(0));
    assert_eq!(collapse_arm("v3_to_v4_if4", &a, None), None);
}

#[test]
fn postdivide_collapses_only_for_negative_copy_alpha_and_tag_1_or_3() {
    for tag in [1, 3] {
        assert_eq!(collapse_arm("postdivide", &[img(tag), cst(-1.0)], None), Some(0), "tag {} copy_alpha -1", tag);
        assert_eq!(collapse_arm("postdivide", &[img(tag), cst(0.0)], None), None, "tag {} copy_alpha 0", tag);
        assert_eq!(collapse_arm("postdivide", &[img(tag), cst(1.0)], None), None, "tag {} copy_alpha 1", tag);
    }
    assert_eq!(collapse_arm("postdivide", &[img(4), cst(-1.0)], None), None); // tag 4 never collapses
    assert_eq!(collapse_arm("postdivide", &[img(4), cst(0.0)], None), None);
    assert_eq!(collapse_arm("postdivide", &[img(2), cst(-1.0)], None), None);
}

#[test]
fn copy_alpha_test_is_strictly_negative_and_constant() {
    assert!(copy_alpha_negative(&[img(3), cst(-1.0)]));
    assert!(copy_alpha_negative(&[img(3), cst(-0.5)]));
    assert!(!copy_alpha_negative(&[img(3), cst(0.0)]));
    assert!(!copy_alpha_negative(&[img(3), img(1)]));
    assert!(!copy_alpha_negative(&[img(3)]));
}

#[test]
fn v3_to_v4_if4_constant_arguments_stay_with_the_fold() {
    // Hue_Sat_Bright class 3: argument 0 a 3-lane constant, argument 1 the scalar 1, count 3: not collapsed (the fold applies)
    let hsb = [ArgView2 { is_const: true, lanes: [0.8125, 1.0625, 0.9375, 0.0], n: 3, elnum: Some(3) }, cst(1.0), img(3)];
    assert_eq!(collapse_arm("v3_to_v4_if4", &hsb, Some(3)), None);
    // Distort class 3: image arguments, count 3: collapsed
    let dist = [img(3), img(1), img(3)];
    assert_eq!(collapse_arm("v3_to_v4_if4", &dist, Some(3)), Some(0));
    // Blur_Color: count 4 never collapses
    assert_eq!(collapse_arm("v3_to_v4_if4", &dist, Some(4)), None);
}

#[test]
fn v3_to_v4_if4_constant_fold_follows_the_count() {
    use fx_eval::fold::fold_lanes;
    let z3 = ArgView2 { is_const: true, lanes: [0.0, 0.0, 0.0, 0.0], n: 3, elnum: Some(3) };
    let hsb = ArgView2 { is_const: true, lanes: [0.8125, 1.0625, 0.9375, 0.0], n: 3, elnum: Some(3) };
    let one = cst(1.0);
    // Drop_Shadow at count 4: argument 0 lanes (0,0,0) and argument 1 as the alpha lane: [0 0 0 1]
    let r4 = fold_lanes("v3_to_v4_if4", &[z3.clone(), one.clone(), img(4)], 4, None).unwrap();
    assert_eq!(r4.map(f32::from_bits), [0.0, 0.0, 0.0, 1.0]);
    // Hue_Sat_Bright at count 3: the 3 lanes of argument 0, nothing in lane 3
    let r3 = fold_lanes("v3_to_v4_if4", &[hsb.clone(), one.clone(), img(3)], 3, None).unwrap();
    assert_eq!(&r3.map(f32::from_bits)[..3], &[0.8125, 1.0625, 0.9375]);
    assert_eq!(r3[3], 0);
}

#[test]
fn v3_count_is_the_bound_image_arguments_element_count() {
    use fx_eval::fold::v3_count;
    // Blur_Mono: the node's own lanes are 2, the bound image argument has 4
    assert_eq!(v3_count(&[cst(0.7), cst(1.0), img(4)]), Some(4));
    assert_eq!(v3_count(&[img(3), cst(1.0), img(3)]), Some(3));
    // unknown count: None (the compile refuses with V3TOV4IF4-COUNT-UNBOUND)
    let unbound = ArgView2 { is_const: false, lanes: [0.0; 4], n: 1, elnum: None };
    assert_eq!(v3_count(&[cst(0.7), cst(1.0), unbound]), None);
}
