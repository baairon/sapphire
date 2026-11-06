//! INT24 widening step: M3V for every 3-lane argument 0 at argument 1 bit equal to 1.0 (own lanes, argument 1 not placed),
//! at FLD2's two argument-0 lane sets (wave_s9/fld2_m3v_record LANES.tsv: binding A [0.8125 1.0625 0.9375], binding B the `*` product
//! [1.0390625 0.6015625 0.875]); argument 1 other than 1.0 with a 3-lane argument 0 stays refused. Also refusal text.
use fx_eval::compile::ArgView2;
use fx_eval::fold::{fold_lanes, v3_form};
use fx_eval::layoutmerge::{find_layout_text, merge_layout, read_layout};
use fx_eval::{build_effect, Corpus};

fn v(bits: [u32; 3]) -> ArgView2 {
    let l = [f32::from_bits(bits[0]), f32::from_bits(bits[1]), f32::from_bits(bits[2]), 0.0];
    ArgView2 { is_const: true, lanes: l, n: 3, elnum: Some(3) }
}
fn s(x: f32) -> ArgView2 {
    ArgView2 { is_const: true, lanes: [x; 4], n: 1, elnum: Some(1) }
}

const FLD2_A: [u32; 3] = [0x3f50_0000, 0x3f88_0000, 0x3f70_0000];
const FLD2_B: [u32; 3] = [0x3f85_0000, 0x3f1a_0000, 0x3f60_0000];

#[test]
fn m3v_fld2_binding_a_own_lanes() {
    let a = [v(FLD2_A), s(1.0)];
    assert_eq!(v3_form(&a, 3), "M3V");
    let r = fold_lanes("v3_to_v4_if4", &a, 3, None).expect("folded");
    assert_eq!([r[0], r[1], r[2]], FLD2_A);
}

#[test]
fn m3v_fld2_binding_b_own_lanes() {
    let a = [v(FLD2_B), s(1.0)];
    assert_eq!(v3_form(&a, 3), "M3V");
    let r = fold_lanes("v3_to_v4_if4", &a, 3, None).expect("folded");
    assert_eq!([r[0], r[1], r[2]], FLD2_B);
}

#[test]
fn m3v_distinct_lanes_vector_arg1_equal_one() {
    // INT23's first assert of refused_n3_vector_arg0_distinct_lanes, now M3V; and a bit-equal 2-lane argument 1 of 1.0
    assert_eq!(v3_form(&[v([0.75f32.to_bits(), 0.7f32.to_bits(), 0.75f32.to_bits()]), s(1.0)], 3), "M3V");
    let a1 = ArgView2 { is_const: true, lanes: [1.0, 1.0, 0.0, 0.0], n: 2, elnum: Some(2) };
    assert_eq!(v3_form(&[v(FLD2_A), a1], 3), "M3V");
}

#[test]
fn refused_n3_three_lane_arg0_arg1_not_one() {
    // FLD2's brightness 0.875 (inside the `*` node, not a V3ToV4If4 argument) and other values stay refused
    for x in [0.875f32, 0.0, -1.0, 1.0000001, 2.0] {
        assert_eq!(v3_form(&[v(FLD2_A), s(x)], 3), "UNMEASURED-FORM", "argument 1 {}", x);
    }
    let a1 = ArgView2 { is_const: true, lanes: [1.0, 0.875, 0.0, 0.0], n: 2, elnum: Some(2) };
    assert_eq!(v3_form(&[v(FLD2_B), a1], 3), "UNMEASURED-FORM");
    // -0.0 lanes and NaN in argument 0 do not change the class (any lanes); argument 1 NaN is refused
    assert_eq!(v3_form(&[v([0x8000_0000, 0x7fc0_0000, 0x3f80_0000]), s(1.0)], 3), "M3V");
    assert_eq!(v3_form(&[v(FLD2_A), s(f32::NAN)], 3), "UNMEASURED-FORM");
}

#[test]
fn layout_modify_value_text_prints_name_and_value() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    // Texture_Chroma_Spiral_Autogen's row carries `:modify shift = 0`
    fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = Corpus::from_programs_tsv(&tsv);
    let prog = "Texture_Chroma_Spiral_Autogen";
    let rows = find_layout_text(c.layout_texts.iter().map(|x| x.1.as_str()), prog);
    let li = read_layout(rows[0]).expect("layout");
    let mut g = build_effect(&c, c.find_effect(prog, None).unwrap()).unwrap();
    let e = merge_layout(&mut g, &li).unwrap_err();
    assert!(e.starts_with("LAYOUT-MODIFY-VALUE-UNSUPPORTED (Texture_Chroma_Spiral_Autogen: :modify "), "{}", e);
    assert!(e.ends_with(" in the layout row; the original's handling of a :modify value is not ported; no claimed program carries one, MEASURED IFNEQ_AFTER_Q.tsv against STYLE_MAP.tsv; W518)"), "{}", e);
    assert!(!e.contains(": 0 carries a value"), "{}", e);
    assert_eq!(fx_eval::layoutmerge::modify_value_refusal("P", ":modify a = 3"), "LAYOUT-MODIFY-VALUE-UNSUPPORTED (P: :modify a = 3 in the layout row; the original's handling of a :modify value is not ported; no claimed program carries one, MEASURED IFNEQ_AFTER_Q.tsv against STYLE_MAP.tsv; W518)");
}
