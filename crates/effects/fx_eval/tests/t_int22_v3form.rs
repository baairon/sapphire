//! the v3_to_v4_if4 form classifier behind the claim-route refusal V3TOV4IF4-NARROW-UNMEASURED.
//! One test per refused form, and the measured forms (M4, M3Z, M3V, M2S) not refused. Unit tests on constructed operands; no original is judged here.
use fx_eval::compile::ArgView2;
use fx_eval::fold::{fold_lanes, v3_form, v3_note, v3_take};

fn c(l: [f32; 4], n: usize) -> ArgView2 {
    ArgView2 { is_const: true, lanes: l, n, elnum: Some(n as i32) }
}
fn s(x: f32) -> ArgView2 {
    c([x, x, x, x], 1)
}

// FLD3-FORM: the two earlier refusals of n = 3 with a scalar argument 0 and a scalar argument 1 (0.5, -0.0) are measured now: the form is
// argument 0 broadcast to lanes 0 to 2 and argument 1 in lane 3 (records). They stay refused when argument 1 is not one lane.
#[test]
fn n3_scalar_arg0_scalar_arg1_is_m3b_not_refused() {
    assert_eq!(v3_form(&[s(0.2), s(0.5)], 3), "M3B");
    assert_eq!(v3_form(&[s(0.2), s(-0.0)], 3), "M3B");
}

#[test]
fn refused_n3_scalar_arg0_vector_arg1() {
    assert_eq!(v3_form(&[s(0.2), c([1.0, 0.5, 0.0, 0.0], 2)], 3), "UNMEASURED-FORM");
}

// the test refused_n3_vector_arg0 is replaced by the four tests below and m3v_not_refused.
// distinct lanes at argument 1 = 1.0 are M3V after the widening; the first assert of the recorded test moved to t_int24_m3v.rs
// (as M3V); the argument 1 = 0 assert stays refused here.
#[test]
fn refused_n3_vector_arg0_distinct_lanes() {
    assert_eq!(v3_form(&[c([0.1, 0.2, 0.3, 0.0], 3), s(0.0)], 3), "UNMEASURED-FORM");
}

#[test]
fn refused_n3_two_lane_arg0() {
    assert_eq!(v3_form(&[c([0.75, 0.75, 0.0, 0.0], 2), s(1.0)], 3), "UNMEASURED-FORM");
}

#[test]
fn refused_n3_four_lane_arg0() {
    assert_eq!(v3_form(&[c([0.75, 0.75, 0.75, 0.75], 4), s(1.0)], 3), "UNMEASURED-FORM");
}

#[test]
fn refused_n3_equal_lane_arg0_arg1_half() {
    assert_eq!(v3_form(&[c([0.75, 0.75, 0.75, 0.0], 3), s(0.5)], 3), "UNMEASURED-FORM");
    // a vector argument 1 with differing lanes is refused even when lane 0 is 1.0
    assert_eq!(v3_form(&[c([0.75, 0.75, 0.75, 0.0], 3), c([1.0, 0.5, 0.0, 0.0], 2)], 3), "UNMEASURED-FORM");
}

#[test]
fn m3v_not_refused() {
    // HSB node 67 (hsBR, hsR0): argument 0 lanes 0.75 or 0.7, argument 1 = 1.0; result the three lanes of argument 0, argument 1 not placed
    for v in [0.75f32, 0.7, 1.0] {
        let a = [c([v, v, v, 0.0], 3), s(1.0)];
        assert_eq!(v3_form(&a, 3), "M3V");
        let r = fold_lanes("v3_to_v4_if4", &a, 3, None).expect("folded");
        assert_eq!(&r[..3], &[v.to_bits(); 3]);
    }
    assert_eq!(v3_form(&[c([0.75, 0.75, 0.75, 0.0], 3), c([1.0, 1.0, 0.0, 0.0], 2)], 3), "M3V");
}

#[test]
fn refused_n2_vector_arg0() {
    assert_eq!(v3_form(&[c([0.1, 0.2, 0.0, 0.0], 2), s(0.5)], 2), "UNMEASURED-FORM");
}

#[test]
fn refused_n2_vector_arg1_differing_lanes() {
    assert_eq!(v3_form(&[s(0.1), c([0.2, 0.3, 0.0, 0.0], 2)], 2), "UNMEASURED-FORM");
}

#[test]
fn refused_n1() {
    // FLD3M: n = 1 with one-lane arguments is the measured pair form (M1P) now; a vector argument is still refused
    assert_eq!(v3_form(&[s(0.1), s(0.0)], 1), "M1P");
    assert_eq!(v3_form(&[c([0.1, 0.2, 0.3, 0.0], 3), s(0.0)], 1), "UNMEASURED-FORM");
}

#[test]
fn measured_forms_not_refused() {
    assert_eq!(v3_form(&[s(0.2), s(0.0)], 3), "M3Z");
    assert_eq!(v3_form(&[s(0.7416992), s(0.7319031)], 2), "M2S");
    assert_eq!(v3_form(&[s(1.0), c([1.0, 1.0, 0.0, 0.0], 2)], 2), "M2S");
    assert_eq!(v3_form(&[s(0.1), s(0.5)], 4), "M4");
    assert_eq!(v3_form(&[c([0.1, 0.2, 0.3, 0.4], 4), s(0.5)], 4), "M4");
}

#[test]
fn note_and_take_record_the_form() {
    // FLD3-FORM: the argument pair (0.2, 0.5) is M3B now; a vector argument 1 is the unmeasured shape this test uses
    let a = [s(0.2), c([1.0, 0.5, 0.0, 0.0], 2)];
    let r = fold_lanes("v3_to_v4_if4", &a, 3, None).expect("folded");
    let _ = v3_take();
    v3_note(7, &a, 3, &r);
    let t = v3_take();
    assert_eq!(t.len(), 1);
    assert_eq!((t[0].0, t[0].1, t[0].5), (7, 3, "UNMEASURED-FORM"));
    assert!(v3_take().is_empty());
}
