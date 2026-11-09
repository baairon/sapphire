//! The identity collapses of copy_alpha and v3_to_v4_if4_a (vtable slot 38) at the operand counts the combo tables define.
//! No stored data.
use fx_eval::fold::{collapse_arm, copy_alpha_mode};
use fx_eval::ArgView2;

fn img(n: i32) -> ArgView2 {
    ArgView2 { is_const: false, lanes: [0.0; 4], n: n as usize, elnum: Some(n) }
}

fn cv(v: f32, n: usize) -> ArgView2 {
    ArgView2 { is_const: true, lanes: [v; 4], n, elnum: Some(n as i32) }
}

/// copy_alpha at operand counts [a, b] with the combo tag the table gives.
fn ca(a: i32, b: i32, tag: i32) -> Option<usize> {
    collapse_arm("copy_alpha", &[img(a), img(b)], Some(tag))
}

/// v3_to_v4_if4_a at operand counts [a, b, c] with the combo tag the table gives.
fn va(a: i32, b: i32, c: i32, tag: i32) -> Option<usize> {
    collapse_arm("v3_to_v4_if4_a", &[img(a), img(b), img(c)], Some(tag))
}

#[test]
fn the_mode_word_is_the_tag_except_for_two_channels_over_four() {
    assert_eq!(copy_alpha_mode(1, 1), 1);
    assert_eq!(copy_alpha_mode(1, 3), 1);
    assert_eq!(copy_alpha_mode(2, 2), 2);
    assert_eq!(copy_alpha_mode(2, 4), 4);
    assert_eq!(copy_alpha_mode(3, 4), 3);
    assert_eq!(copy_alpha_mode(4, 4), 4);
}

#[test]
fn copy_alpha_forwards_argument_zero_unless_its_mode_is_two_or_four() {
    // the combo tags of copy_alpha: [1,1] 1, [1,2] 2, [1,3] 1, [1,4] 2 (mode 4), [3,3] 3, [3,4] 4
    assert_eq!(ca(1, 1, 1), Some(0), "[1, 1]");
    assert_eq!(ca(1, 2, 2), None, "[1, 2]: mode 2");
    assert_eq!(ca(1, 3, 1), Some(0), "[1, 3]: the source without alpha");
    assert_eq!(ca(1, 4, 2), None, "[1, 4]: mode 4");
    assert_eq!(ca(3, 3, 3), Some(0), "[3, 3]");
    assert_eq!(ca(3, 4, 4), None, "[3, 4]: mode 4");
}

#[test]
fn copy_alpha_collapse_needs_the_counts_of_both_operands() {
    let unknown = ArgView2 { is_const: false, lanes: [0.0; 4], n: 1, elnum: None };
    assert_eq!(collapse_arm("copy_alpha", &[img(1), unknown.clone()], Some(1)), None);
    assert_eq!(collapse_arm("copy_alpha", &[img(1), img(3)], None), None);
    // a constant second operand is read by its lane count like an image by its channel count
    assert_eq!(collapse_arm("copy_alpha", &[img(1), cv(1.0, 3)], Some(1)), Some(0));
    assert_eq!(collapse_arm("copy_alpha", &[img(1), cv(1.0, 4)], Some(2)), None);
}

#[test]
fn v3_to_v4_if4_a_forwards_argument_zero_at_count_three_and_argument_one_at_tag_one_count_one() {
    assert_eq!(va(3, 1, 3, 3), Some(0), "[3, 1, 3]: the source without alpha");
    assert_eq!(va(3, 1, 4, 4), None, "[3, 1, 4]");
    assert_eq!(va(1, 1, 1, 1), Some(1), "[1, 1, 1]: tag 1 and count 1");
    assert_eq!(va(1, 1, 4, 4), None, "[1, 1, 4]");
    // the count is argument 2's, not the node's tag: count 1 with tag 3 and count 2 with tag 2 keep the node
    assert_eq!(va(3, 1, 1, 3), None, "[3, 1, 1]: tag 3, count 1");
    assert_eq!(va(1, 1, 2, 2), None, "[1, 1, 2]");
    // an unknown count never collapses
    let unknown = ArgView2 { is_const: false, lanes: [0.0; 4], n: 1, elnum: None };
    assert_eq!(collapse_arm("v3_to_v4_if4_a", &[img(3), img(1), unknown], Some(3)), None);
}
