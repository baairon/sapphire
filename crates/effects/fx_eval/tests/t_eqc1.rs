use fx_eval::compile::{prune_arm, ArgView2};

fn av(c: bool, l: f32, n: usize) -> ArgView2 {
    ArgView2 { is_const: c, lanes: [l; 4], n, elnum: Some(n as i32) }
}
fn nc() -> ArgView2 {
    av(false, 0.0, 4)
}

// Listing 0x503a90: arm 3 only if [+0x44] != 0, [+0x40] == [+0x70] and 0x4cabb0 true; every other path returns 2.
#[test]
fn arm3_flag_set_counts_equal_compare_true() {
    assert_eq!(prune_arm("if_neq_const", &[av(true, 3.0, 1), av(true, 3.0, 1), nc(), nc()]), Some(3));
    assert_eq!(prune_arm("if_neq_const", &[av(true, 3.0, 3), av(true, 3.0, 3), nc(), nc()]), Some(3));
}
#[test]
fn arm2_compare_false() {
    assert_eq!(prune_arm("if_neq_const", &[av(true, 3.0, 1), av(true, 4.0, 1), nc(), nc()]), Some(2));
}
#[test]
fn arm2_differing_counts_even_if_lanes_equal() {
    // the lanes pad to the same values, so the native compare alone would say equal; the listing never compares
    assert_eq!(prune_arm("if_neq_const", &[av(true, 3.0, 1), av(true, 3.0, 2), nc(), nc()]), Some(2));
    assert_eq!(prune_arm("if_neq_const", &[av(true, 3.0, 4), av(true, 3.0, 3), nc(), nc()]), Some(2));
}
#[test]
fn arm2_flag_clear() {
    assert_eq!(prune_arm("if_neq_const", &[av(false, 3.0, 1), av(true, 3.0, 1), nc(), nc()]), Some(2));
    assert_eq!(prune_arm("if_neq_const", &[av(true, 3.0, 1), av(false, 3.0, 1), nc(), nc()]), Some(2));
}

// Probe cases (EQC1 resumed): whole-program compiles of the Blend probe texts; the original's arm per case is MEASURED in results/.
use fx_eval::{build_effect, compile, enumerate_branches_opt, ComboTable, CompileOptions, Corpus};
use std::collections::BTreeMap;

fn arms(case: &str) -> Result<Vec<String>, String> {
    arms_el(case, BTreeMap::new())
}
fn arms_el(case: &str, el: BTreeMap<String, i32>) -> Result<Vec<String>, String> {
    let dir = effects_vecdir::path_str("fx_eval");
    let c = Corpus::from_programs_tsv(&std::fs::read_to_string(format!("{}/tests/eqc1_probe/progs_{}.tsv", dir, case)).unwrap());
    let ct = ComboTable::from_tsv(&std::fs::read_to_string(&format!("{}/tests/eqc1_probe/PROGRAM_FORMS.tsv", dir)).unwrap());
    let g = build_effect(&c, c.find_effect("Blend", None).unwrap()).map_err(|e| e.to_string())?;
    let mut out = vec![];
    for s in enumerate_branches_opt(&g, &ct, &BTreeMap::new(), false) {
        let opt = CompileOptions { overrides: s.overrides.clone(), input_elnum: el.clone(), strict: false, paramset: s.paramset.clone(), hc_zero_legacy: false, ..Default::default() };
        match compile(&g, &ct, &opt) {
            Ok(cp) => out.push(cp.plan_tsv()),
            Err(e) => return Err(e.msg),
        }
    }
    Ok(out)
}

#[test]
fn probe_c1_counts_1_and_3_take_arm_2() {
    if !effects_vecdir::have_all(&["fx_eval"]) {
        return;
    }
    // original MEASURED arm 2 ([+0x40]=1, [+0x70]=3); the unpatched native took arm 3
    let p = arms("c1").unwrap().concat();
    assert!(p.contains("ifneqc\t2"), "{}", p);
}
#[test]
fn probe_c2_counts_3_and_3_take_arm_3() {
    if !effects_vecdir::have_all(&["fx_eval"]) {
        return;
    }
    let p = arms("c2").unwrap().concat();
    assert!(p.contains("ifneqc\t3"), "{}", p);
}
#[test]
fn probe_count_5_is_refused() {
    if !effects_vecdir::have_all(&["fx_eval"]) {
        return;
    }
    // c0 is the unchanged Blend node (if_neq_const Matte 1 ...); binding Matte to 5 channels gives an unclamped count of 5
    let mut el = BTreeMap::new();
    el.insert("Matte".to_string(), 5);
    let e = arms_el("c0", el).unwrap_err();
    assert!(e.starts_with("IFNEQ-CONST-COUNT-OUT-OF-RANGE (Blend: node "), "{}", e);
    assert!(e.ends_with(": 5, 1)"), "{}", e);
}
