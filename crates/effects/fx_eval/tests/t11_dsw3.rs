//! T11: the unbound-input exception of the range pass (0x11b730, listing 0x11b86e..0x11b8e9) and the blur / comp_premult / mocha_render fold rules.
use fx_eval::fold::*;
use fx_eval::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn combos() -> ComboTable {
    let p = effects_vecdir::path("fx_eval/data").join("COMBOS.tsv");
    ComboTable::from_tsv(&std::fs::read_to_string(p).unwrap())
}

/// The Drop_Shadow test program: stored test data (`VEC_DIR/effects/fx_eval/data/TEST_DROP_SHADOW.txt`), not part of the repository.
fn ds() -> String {
    std::fs::read_to_string(effects_vecdir::path("fx_eval/data/TEST_DROP_SHADOW.txt")).expect("TEST_DROP_SHADOW.txt")
}

fn params(bound_matte: bool, sets: &[(&str, f32)]) -> BTreeMap<String, f32> {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", &ds()).unwrap();
    let ei = c.find_effect("Drop_Shadow", None).unwrap();
    let g = build_effect(&c, ei).unwrap();
    let mut ov = BTreeMap::new();
    for (k, v) in sets {
        ov.insert(k.to_string(), vec![*v]);
    }
    let mut bound_elnum = BTreeMap::new();
    if bound_matte {
        bound_elnum.insert("Matte".to_string(), 4);
    }
    let cp = compile(&g, &combos(), &CompileOptions { overrides: ov, no_fold: true, minimal_context: true, bound_elnum, ..Default::default() }).unwrap();
    cp.params.iter().map(|(n, l)| (n.clone(), f32::from_bits(l[0]))).collect()
}

#[test]
fn matte_parameters_are_unclamped_while_matte_is_unbound() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    let m = params(false, &[("matte_use", 2.7117), ("invert_matte", -3.5329), ("shadow_opacity", -0.4417)]);
    assert_eq!(m["matte_use"], 2.7117);
    assert_eq!(m["invert_matte"], -3.5329);
    // a parameter without an input index is clamped as before (record range [0, 1e6])
    assert_eq!(m["shadow_opacity"], 0.0);
}

#[test]
fn matte_parameters_are_clamped_once_matte_is_bound() {
    if !effects_vecdir::have_all(&["fx_eval/data", "fx_eval/data/TEST_DROP_SHADOW.txt"]) {
        return;
    }
    let m = params(true, &[("matte_use", 2.7117), ("invert_matte", -3.5329)]);
    assert_eq!(m["matte_use"], 1.0);
    assert_eq!(m["invert_matte"], 0.0);
}

fn cv(l: [f32; 4], n: usize) -> ArgView2 {
    ArgView2 { is_const: true, lanes: l, n, elnum: Some(n as i32) }
}
fn img(n: i32) -> ArgView2 {
    ArgView2 { is_const: false, lanes: [0.0; 4], n: n as usize, elnum: Some(n) }
}

#[test]
fn comp_premult_collapses_to_a_only_when_b_and_t_are_zero() {
    let z3 = cv([0.0; 4], 3);
    assert_eq!(collapse_arm("comp_premult", &[img(3), z3.clone(), cv([0.0; 4], 1)], Some(3)), Some(0));
    assert_eq!(collapse_arm("comp_premult", &[img(3), z3.clone(), cv([0.5; 4], 1)], Some(3)), None);
    assert_eq!(collapse_arm("comp_premult", &[img(3), img(3), cv([0.0; 4], 1)], Some(3)), None);
}

#[test]
fn mocha_render_value_is_zero_for_a_zero_scale_lane() {
    let a = |sx: f32, sy: f32| vec![cv([0.0; 4], 1), cv([sx, sy, 0.0, 0.0], 2), cv([0.0; 4], 2)];
    let f = |sx: f32, sy: f32| f32::from_bits(fold_lanes("mocha_render", &a(sx, sy), 1, None).unwrap()[0]);
    assert_eq!(f(1.0, 1.0), 1.0);
    assert_eq!(f(0.0, 4.0), 0.0);
    assert_eq!(f(2.0, 0.0), 0.0);
    assert_eq!(f(-0.0, 1.0), 0.0);
    assert_eq!(f(f32::NAN, 1.0), 1.0);
}
