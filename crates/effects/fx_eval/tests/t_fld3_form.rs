//! FLD3-FORM: the folded v3_to_v4_if4 node with n = 3, a one-lane argument 0 and a one-lane argument 1 is
//! [a0 a0 a0 a1]: argument 0 broadcast to lanes 0 to 2, argument 1 in lane 3. MEASURED by records (Blur_Color_Autogen node 67,
//! literal read from the original compiler's plan in the emulator): brightness 1.0 with scale_opacity 1.0 gives [1 1 1 1]; 1.21 (bits 3f9ae180)
//! with 1.0 gives [1.21 1.21 1.21 1]; 1.21 with 0.6875 gives [1.21 1.21 1.21 0.6875]. The wrong candidate forms (broadcast with lane 3 = 1,
//! product, own lane with zeros, concatenation) are mutants; record C separates broadcast and product from the measured form.
use fx_eval::compile::ArgView2;
use fx_eval::fold::{fold_lanes, v3_form};

fn c(l: [f32; 4], n: usize) -> ArgView2 {
    ArgView2 { is_const: true, lanes: l, n, elnum: Some(n as i32) }
}
fn s(x: f32) -> ArgView2 {
    c([x, x, x, x], 1)
}

#[test]
fn fld3_records_a_b_c() {
    for (a0, a1) in [(0x3f80_0000u32, 0x3f80_0000u32), (0x3f9a_e180, 0x3f80_0000), (0x3f9a_e180, 0x3f30_0000)] {
        let a = [s(f32::from_bits(a0)), s(f32::from_bits(a1))];
        assert_eq!(v3_form(&a, 3), "M3B");
        let r = fold_lanes("v3_to_v4_if4", &a, 3, None).expect("folded");
        assert_eq!(r, [a0, a0, a0, a1], "a0 {:08x} a1 {:08x}", a0, a1);
    }
    assert_eq!(f32::from_bits(0x3f30_0000), 0.6875);
}

#[test]
fn fld3_every_other_shape_stays_unmeasured() {
    // a vector argument 0 at n = 3 keeps its own forms (M3V at argument 1 = 1.0), a vector argument 1 is not the measured shape
    assert_eq!(v3_form(&[s(1.21), c([1.0, 0.5, 0.0, 0.0], 2)], 3), "UNMEASURED-FORM");
    assert_eq!(v3_form(&[c([1.0, 2.0, 0.0, 0.0], 2), s(1.0)], 3), "UNMEASURED-FORM");
    assert_eq!(v3_form(&[c([1.0, 1.0, 1.0, 1.0], 4), s(1.0)], 3), "UNMEASURED-FORM");
    assert_eq!(v3_form(&[c([0.1, 0.2, 0.3, 0.0], 3), s(0.5)], 3), "UNMEASURED-FORM");
    assert_eq!(v3_form(&[c([0.1, 0.2, 0.3, 0.0], 3), s(1.0)], 3), "M3V");
    // n = 2 keeps its own measured form, n = 4 is M4, n = 1 (Blur_Mono node 68) stays unmeasured
    assert_eq!(v3_form(&[s(1.21), s(1.0)], 2), "M2S");
    assert_eq!(v3_form(&[s(1.21), s(1.0)], 4), "M4");
    assert_eq!(v3_form(&[c([0.5, 0.5, 0.5, 0.5], 4), s(1.0)], 1), "UNMEASURED-FORM");
}

#[test]
fn fld3_earlier_measured_zero_argument_1_unchanged() {
    // M3Z (argument 1 = +0): lanes 0 to 2 are the broadcast and lane 3 stays 0, as before
    let r = fold_lanes("v3_to_v4_if4", &[s(0.7), s(0.0)], 3, None).expect("folded");
    assert_eq!(r, [0.7f32.to_bits(), 0.7f32.to_bits(), 0.7f32.to_bits(), 0]);
}

// FLD3M: Blur_Mono_Autogen node 68 (records gA to gD; plans read from the original compiler in the emulator): n = 1, one-lane arguments, the folded
// constant is the pair [a0 a1]: [1 1] at the defaults and with mix_with_source 0.4519, [1 0.6875] with scale_opacity 0.6875, [1.21 0.6875] with brightness 1.21
// as well. The candidate forms one lane (own, concat truncated), broadcast of argument 0, product and swapped lanes are mutants; the bindings gB and gD kill them.
#[test]
fn fld3m_records_ga_gb_gc_gd() {
    for (a0, a1) in [(0x3f80_0000u32, 0x3f80_0000u32), (0x3f80_0000, 0x3f30_0000), (0x3f9a_e180, 0x3f30_0000)] {
        let a = [s(f32::from_bits(a0)), s(f32::from_bits(a1))];
        assert_eq!(v3_form(&a, 1), "M1P");
        let r = fold_lanes("v3_to_v4_if4", &a, 1, None).expect("folded");
        assert_eq!(r, [a0, a1, 0, 0], "a0 {:08x} a1 {:08x}", a0, a1);
    }
}

#[test]
fn fld3m_other_shapes_stay_unmeasured() {
    assert_eq!(v3_form(&[s(1.0), c([1.0, 0.5, 0.0, 0.0], 2)], 1), "UNMEASURED-FORM");
    assert_eq!(v3_form(&[c([1.0, 1.0, 0.0, 0.0], 2), s(1.0)], 1), "UNMEASURED-FORM");
}
