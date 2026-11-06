//! T2 (NV-2): compiler semantics. Hand-evaluated examples per rule, the milestone programs under
//! their branch sets, determinism, malformed input.
use fx_eval::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn big<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new().stack_size(128 << 20).spawn(f).unwrap().join().unwrap();
}

fn combos() -> ComboTable {
    let p = effects_vecdir::path("fx_eval/data").join("COMBOS.tsv");
    ComboTable::from_tsv(&std::fs::read_to_string(p).unwrap())
}

fn gen(text: &str, name: &str) -> Genotype {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", text).unwrap();
    let ei = c.find_effect(name, None).unwrap();
    build_effect(&c, ei).unwrap()
}

fn ov(k: &str, v: f32) -> CompileOptions {
    let mut m = BTreeMap::new();
    m.insert(k.to_string(), vec![v]);
    CompileOptions { overrides: m, ..Default::default() }
}

fn names(cp: &CompiledProgram) -> Vec<String> {
    cp.nodes.iter().map(|n| n.name.clone()).collect()
}

fn av(c: bool, l: f32, n: usize) -> ArgView2 {
    ArgView2 { is_const: c, lanes: [l; 4], n, elnum: Some(n as i32) }
}

#[test]
fn r3_select_first_match() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let t = combos();
    assert_eq!(t.select("*", &[1, 1]), Some(1));
    assert_eq!(t.select("*", &[3, 1]), Some(3));
    assert_eq!(t.select("*", &[1, 4]), Some(4));
    assert_eq!(t.select("*", &[2, 3]), None); // not in the table: the selector's -1
    assert_eq!(t.select("no_such_class", &[1]), None);
}

#[test]
fn r5_prune_arms_by_hand() {
    // if+ c a b: c > 0 -> 1, c <= 0 -> 2, non-constant -> none
    assert_eq!(prune_arm("if+", &[av(true, 1.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(1));
    assert_eq!(prune_arm("if+", &[av(true, 0.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(2));
    assert_eq!(prune_arm("if+", &[av(true, -3.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(2));
    assert_eq!(prune_arm("if+", &[av(false, 1.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), None);
    // vector condition with mixed lanes: not pruned
    let mixed = ArgView2 { is_const: true, lanes: [1.0, -1.0, 1.0, 1.0], n: 2, elnum: Some(2) };
    assert_eq!(prune_arm("if+", &[mixed, av(false, 0.0, 4), av(false, 0.0, 4)]), None);
    // if> a b x y
    assert_eq!(prune_arm("if>", &[av(true, 2.0, 1), av(true, 1.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(2));
    assert_eq!(prune_arm("if>", &[av(true, 1.0, 1), av(true, 1.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(3));
    assert_eq!(prune_arm("if>", &[av(false, 2.0, 1), av(true, 1.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), None);
    // if_eq a b x y
    assert_eq!(prune_arm("if_eq", &[av(true, 3.0, 1), av(true, 3.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(2));
    assert_eq!(prune_arm("if_eq", &[av(true, 3.0, 1), av(true, 4.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(3));
    // if_neq_const c k a b: equal -> 3 (b), else 2 (a); a non-constant condition silently takes 2
    assert_eq!(prune_arm("if_neq_const", &[av(true, 3.0, 1), av(true, 3.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(3));
    assert_eq!(prune_arm("if_neq_const", &[av(true, 2.0, 1), av(true, 3.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(2));
    assert_eq!(prune_arm("if_neq_const", &[av(false, 3.0, 1), av(true, 3.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(2));
    // switch sel k1 v1 k2 v2 default (7 args): hit -> 2 / 4, miss -> last (5)
    let sw = |s: f32| prune_arm("switch", &[av(true, s, 1), av(true, 1.0, 1), av(false, 0.0, 4), av(true, 2.0, 1), av(false, 0.0, 4), av(false, 0.0, 4)]);
    assert_eq!(sw(1.0), Some(2));
    assert_eq!(sw(2.0), Some(4));
    assert_eq!(sw(9.0), Some(5));
    // if_has_alpha img a b: 2 or 4 channels -> arm 1
    let a4 = ArgView2 { is_const: false, lanes: [0.0; 4], n: 4, elnum: Some(4) };
    let a3 = ArgView2 { is_const: false, lanes: [0.0; 4], n: 3, elnum: Some(3) };
    assert_eq!(prune_arm("if_has_alpha", &[a4.clone(), av(false, 0.0, 4), av(false, 0.0, 4)]), Some(1));
    assert_eq!(prune_arm("if_has_alpha", &[a3, av(false, 0.0, 4), av(false, 0.0, 4)]), Some(2));
    // a class outside the family never prunes
    assert_eq!(prune_arm("*", &[av(true, 1.0, 1), av(true, 1.0, 1)]), None);
}

#[test]
fn if_plus_on_a_parameter_both_arms() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let g = gen("(def_effect T ( S M ) ( ) ( c = 1 ) ( if+ c S M ))", "T");
    let ct = combos();
    let a = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert_eq!(names(&a), vec!["S"]); // c = 1: S kept, M dropped, if+ gone
    assert!(a.pruned.iter().filter(|p| p.class == "if+").count() >= 1);
    let b = compile(&g, &ct, &ov("c", 0.0)).unwrap();
    assert_eq!(names(&b), vec!["M"]);
    let c = compile(&g, &ct, &ov("c", -2.0)).unwrap();
    assert_eq!(names(&c), vec!["M"]);
    assert_eq!(a.nodes.last().unwrap().elnum, 4);
}

#[test]
fn if_gt_and_switch_by_hand() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let ct = combos();
    let g = gen("(def_effect T ( S M ) ( ) ( a = 2 ) ( if> a 1 S M ))", "T");
    assert_eq!(names(&compile(&g, &ct, &CompileOptions::default()).unwrap()), vec!["S"]);
    assert_eq!(names(&compile(&g, &ct, &ov("a", 1.0)).unwrap()), vec!["M"]);
    let g = gen("(def_effect T ( S M ) ( ) ( k = 2 ) ( switch k 1 S 2 M S ))", "T");
    assert_eq!(names(&compile(&g, &ct, &CompileOptions::default()).unwrap()), vec!["M"]);
    assert_eq!(names(&compile(&g, &ct, &ov("k", 1.0)).unwrap()), vec!["S"]);
    assert_eq!(names(&compile(&g, &ct, &ov("k", 7.0)).unwrap()), vec!["S"]);
}

#[test]
fn r2_elnum_from_combo_tag() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let ct = combos();
    // image(4) * scalar parameter(1) -> 4 ; scalar * scalar constant folding is NOT done: stays a node
    let g = gen("(def_effect T ( S ) ( ) ( a = 2 ) ( * S a ))", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    let m = cp.nodes.iter().find(|n| n.prim == "*").unwrap();
    assert_eq!(m.elnum, 4);
    let g = gen("(def_effect T ( S ) ( ) ( a = 2 ) ( * S ( * a 3 ) ))", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    let inner = cp.nodes.iter().find(|n| n.kind == "const" && n.folded).unwrap();
    assert_eq!(inner.elnum, 1); // NV-3: the constant subgraph folds (0x98d0)
    assert_eq!(cp.nodes.iter().filter(|n| n.prim == "*").count(), 1);
    // vec3 constant times image
    let g = gen("(def_effect T ( S ) ( ) ( ) ( * S [ 1 2 3 ] ))", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert_eq!(cp.nodes.iter().find(|n| n.prim == "*").unwrap().elnum, 4);
}

#[test]
fn invalid_combo_is_reported_and_strict_errors() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let ct = combos();
    let g = gen("(def_effect T ( S ) ( ) ( ) ( * [ 1 2 ] [ 1 2 3 ] ))", "T");
    let lenient = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert!(lenient.diagnostics.iter().any(|d| d.contains("invalid combo")));
    let strict = compile(&g, &ct, &CompileOptions { strict: true, ..Default::default() });
    assert!(strict.is_err());
}

#[test]
fn auto_nodes() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let ct = combos();
    let g = gen("(def_effect T ( S ) ( ) ( ) S)", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert_eq!(names(&cp), vec!["S"]);
    assert!(cp.pruned.iter().any(|p| p.class == "if+" && p.name.starts_with("if+")));
    assert!(cp.labels.contains(&"EVALUATOR-PLAN-CHECKED"));
    let g = gen("(def_effect T ( S ) ( dynamic ) ( ) S)", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert_eq!(names(&cp), vec!["S", "", "postdivide"]);
}

#[test]
fn malformed_genotypes_error_not_panic() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let ct = combos();
    let g = gen("(def_effect T ( S ) ( ) ( ) S)", "T");
    let mut a = g.clone();
    a.dest.clear();
    assert!(compile(&a, &ct, &CompileOptions::default()).is_err());
    let mut b = g.clone();
    b.dest = vec![999];
    assert!(compile(&b, &ct, &CompileOptions::default()).is_err());
    let g2 = gen("(def_effect T ( S ) ( ) ( ) ( * S 2 ))", "T");
    let mut c = g2.clone();
    let k = c.nodes.iter().position(|n| n.name == "*").unwrap();
    c.nodes[k].children = vec![0, 12345];
    assert!(compile(&c, &ct, &CompileOptions::default()).is_err());
    // empty combo table: every class unknown, still no panic
    let cp = compile(&g2, &ComboTable::default(), &CompileOptions::default()).unwrap();
    assert!(cp.diagnostics.iter().any(|d| d.contains("UNKNOWN-CLASS")));
    assert!(ComboTable::from_tsv("").classes.is_empty());
    assert!(ComboTable::from_tsv("garbage\nx\ty\n").classes.is_empty());
}

fn programs() -> Option<Corpus> {
    let base = effects_vecdir::root();
    let tsv = base.join("record_extract").join("programs").join("programs.tsv");
    let t = std::fs::read_to_string(tsv).ok()?;
    Some(Corpus::from_programs_tsv(&t))
}

fn sets(c: &Corpus, name: &str) -> (Genotype, Vec<BranchSet>) {
    let ct = combos();
    let ei = c.find_effect(name, None).unwrap();
    let g = build_effect(c, ei).unwrap();
    let s = enumerate_branches(&g, &ct, &BTreeMap::new());
    (g, s)
}


#[test]
fn determinism_and_all_targets_compile() {
    big(|| {
        let c = match programs() {
            Some(c) => c,
            None => return,
        };
        let ct = combos();
        let forms = std::fs::read_to_string(effects_vecdir::path("fx_eval/data").join("PROGRAM_FORMS.tsv")).unwrap();
        let mut n = 0;
        for (i, line) in forms.lines().enumerate() {
            let f: Vec<&str> = line.split('\t').collect();
            if i == 0 || f.len() < 3 || f[2] != "1" {
                continue;
            }
            let ei = c.find_effect(f[0], Some(f[1])).unwrap();
            let g = build_effect(&c, ei).unwrap();
            // HC-ZERO-LEGACY (named state A option)
            let s1 = enumerate_branches_opt(&g, &ct, &BTreeMap::new(), true);
            let s2 = enumerate_branches_opt(&g, &ct, &BTreeMap::new(), true);
            assert_eq!(s1.len(), s2.len());
            for (a, b) in s1.iter().zip(s2.iter()) {
                assert_eq!(a.signature, b.signature);
            }
            let p1 = compile(&g, &ct, &CompileOptions { hc_zero_legacy: true, ..Default::default() }).unwrap().plan_tsv();
            let p2 = compile(&g, &ct, &CompileOptions { hc_zero_legacy: true, ..Default::default() }).unwrap().plan_tsv();
            assert_eq!(p1, p2);
            n += 1;
        }
        assert_eq!(n, 38);
    });
}
