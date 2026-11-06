//! T3 (NV-3): fold and collapse rules by hand, plan emission, the compare tool, all 38 targets.
use fx_eval::fold::*;
use fx_eval::plan::*;
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

fn cv(l: [f32; 4], n: usize) -> ArgView2 {
    ArgView2 { is_const: true, lanes: l, n, elnum: Some(n as i32) }
}

fn img(n: i32) -> ArgView2 {
    ArgView2 { is_const: false, lanes: [0.0; 4], n: n as usize, elnum: Some(n) }
}

fn bits(l: [f32; 4]) -> [u32; 4] {
    [l[0].to_bits(), l[1].to_bits(), l[2].to_bits(), l[3].to_bits()]
}

#[test]
fn eps_boundaries_are_the_measured_doubles() {
    assert_eq!(ZERO_EPS, 1e-10);
    assert_eq!(ZERO_EPS_NEG, -1e-10);
    assert!((ONE_HI - 1.0 - 1e-10).abs() < 1e-15 && (1.0 - ONE_LO - 1e-10).abs() < 1e-15);
    assert!(is_zero(&cv([0.0, 0.0, 0.0, 0.0], 3)));
    assert!(is_zero(&cv([1e-11, 0.0, 0.0, 0.0], 1)));
    assert!(!is_zero(&cv([1e-9, 0.0, 0.0, 0.0], 1)));
    assert!(!is_zero(&img(1)));
    assert!(is_one(&cv([1.0, 1.0, 1.0, 5.0], 3))); // lane 3 is past elnum 3: not looked at
    assert!(!is_one(&cv([1.0, 1.0, 1.0, 5.0], 4)));
    assert!(!is_one(&cv([1.001, 1.0, 1.0, 1.0], 1)));
}

#[test]
fn collapse_rules_by_hand() {
    let one = cv([1.0; 4], 1);
    let zero = cv([0.0; 4], 1);
    assert_eq!(collapse_arm("*", &[img(4), one.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("*", &[one.clone(), img(4)], Some(4)), Some(1));
    assert_eq!(collapse_arm("*", &[img(4), zero.clone()], Some(4)), Some(1));
    assert_eq!(collapse_arm("*", &[zero.clone(), img(4)], Some(4)), Some(0));
    assert_eq!(collapse_arm("*", &[img(4), cv([2.0; 4], 1)], Some(4)), None);
    assert_eq!(collapse_arm("+", &[img(4), zero.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("+", &[zero.clone(), img(4)], Some(4)), Some(1));
    assert_eq!(collapse_arm("-", &[img(4), zero.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("-", &[zero.clone(), img(4)], Some(4)), None);
    assert_eq!(collapse_arm("/", &[img(4), one.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("/", &[zero.clone(), img(4)], Some(4)), Some(0));
    assert_eq!(collapse_arm("gamma", &[img(4), one.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("gamma", &[img(4), cv([2.0; 4], 1)], Some(4)), None);
    assert_eq!(collapse_arm("hue_shift", &[img(4), zero.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("hue_shift", &[img(4), cv([0.25; 4], 1)], Some(4)), None);
    assert_eq!(collapse_arm("sat_scale", &[img(4), one.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("sat_scale", &[img(1), cv([0.5; 4], 1)], Some(1)), Some(0)); // below 3 channels: identity
    assert_eq!(collapse_arm("sat_scale", &[img(4), cv([0.5; 4], 1)], Some(4)), None);
    assert_eq!(collapse_arm("monochrome", &[img(1)], Some(1)), Some(0));
    assert_eq!(collapse_arm("monochrome", &[img(4)], Some(1)), None);
    assert_eq!(collapse_arm("blur", &[img(4), cv([0.0; 4], 2), img(1), cv([1.0; 4], 1), zero.clone()], Some(4)), Some(0));
    assert_eq!(collapse_arm("unread_class", &[img(4), zero], Some(4)), None);
}

#[test]
fn fold_by_hand_f32() {
    // Hue_Sat_Bright: brightness 1.5 * scale_colors (1, .5, 2) = (1.5, .75, 3, 0): zero tail past elnum
    let r = fold_lanes("*", &[cv([1.5; 4], 1), cv([1.0, 0.5, 2.0, 2.0], 3)], 3, None).unwrap();
    assert_eq!(r, bits([1.5, 0.75, 3.0, 0.0]));
    // minus offset_darks 0.1 in f32
    let r = fold_lanes("-", &[cv([1.5, 0.75, 3.0, 0.0], 3), cv([0.1; 4], 1)], 3, None).unwrap();
    assert_eq!(r[0], (1.5f32 - 0.1f32).to_bits());
    assert_eq!(f32::from_bits(r[0]), 1.4f32);
    assert_eq!(f32::from_bits(r[1]), 0.65f32);
    assert_eq!(f32::from_bits(r[2]), 2.9f32);
    assert_eq!(r[3], 0);
    let r = fold_lanes("/", &[cv([1.5, 0.75, 3.0, 0.0], 3), cv([1.5, 0.75, 3.0, 0.0], 3)], 3, None).unwrap();
    assert_eq!(r, bits([1.0, 1.0, 1.0, 0.0]));
    assert_eq!(fold_lanes("abs", &[cv([-1.5, 2.0, -3.0, 0.0], 3)], 3, None).unwrap(), bits([1.5, 2.0, 3.0, 0.0]));
    assert_eq!(fold_lanes("max", &[cv([0.0; 4], 1), cv([0.2; 4], 1)], 1, None).unwrap(), bits([0.2, 0.0, 0.0, 0.0]));
    assert_eq!(fold_lanes("one_minus", &[cv([0.8; 4], 1)], 1, None).unwrap()[0], (1.0f32 - 0.8f32).to_bits());
    assert_eq!(fold_lanes("coerce_up", &[cv([1.0; 4], 1), img(4)], 4, None).unwrap(), bits([1.0; 4]));
    // v3_to_v4_if4 (v3, a, donor): (x, y, z, a)
    let r = fold_lanes("v3_to_v4_if4", &[cv([1.4, 0.65, 2.9, 0.0], 3), cv([1.0; 4], 1), img(4)], 4, None).unwrap();
    assert_eq!(r, bits([1.4, 0.65, 2.9, 1.0]));
    assert!(fold_lanes("noise", &[cv([1.0; 4], 1)], 1, None).is_none());
    assert!(!fold_arg_required("v3_to_v4_if4", 2) && fold_arg_required("v3_to_v4_if4", 0));
}

#[test]
fn compile_collapses_and_folds_a_tiny_effect() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let ct = combos();
    // (S * a) * 1 collapses the outer node
    let g = gen("(def_effect T ( S ) ( ) ( a = 2 ) ( * ( * S a ) 1 ))", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    let runs: Vec<&str> = cp.nodes.iter().filter(|n| n.kind == "func").map(|n| n.prim.as_str()).collect();
    assert_eq!(runs, vec!["*"]);
    // (a * b) folds to a constant that survives as a run argument
    let g = gen("(def_effect T ( S ) ( ) ( a = 2 b = 3 ) ( * S ( * a b ) ))", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert_eq!(cp.folds.len(), 1);
    let c = cp.nodes.iter().find(|n| n.kind == "const" && n.folded).unwrap();
    assert_eq!(f32::from_bits(c.lanes[0]), 6.0);
    // no_fold keeps the arithmetic node
    let cp = compile(&g, &ct, &CompileOptions { no_fold: true, ..Default::default() }).unwrap();
    assert_eq!(cp.nodes.iter().filter(|n| n.kind == "func").count(), 2);
    // the whole program is its input when everything collapses
    let g = gen("(def_effect T ( S ) ( ) ( a = 0 ) ( + S a ))", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert_eq!(cp.nodes.len(), 1);
    assert_eq!(canon_mine(&cp).dest, "IN:S");
}

#[test]
fn diff_tool_detects_and_ignores() {
    let a = Canon { runs: vec![("*".into(), vec!["IN:S".into(), "[2]".into()])], dest: "R0".into() };
    let mut b = a.clone();
    assert!(diff(&a, &b).is_empty());
    b.runs[0].1[1] = "[3]".into();
    assert_eq!(diff(&a, &b).len(), 1);
    b.runs[0].1[1] = "CLIP".into();
    assert!(diff(&a, &b).is_empty());
    let t = a.to_text();
    assert_eq!(Canon::from_text(&t), a);
    assert_eq!(fmt_g6(1.4f32 as f64), "1.4");
    assert_eq!(fmt_g6(0.65f32 as f64), "0.65");
    assert_eq!(fmt_g6(0.0), "0");
    // the EP-3 text form parses: a run with a clip argument
    let txt = "CMD\t0\tconvert-from-source\t(convert-from-source 0 :ref5.2g)\t-\nCMD\t1\trun-primitive\t(run-primitive 1 -1 :ref6.2g get_frame (clip 0 4) [ 1 ] [ 0 ] )\t-\nCMD\t2\tconvert-to-dest\t(convert-to-dest :ref6.2g)\t-\nEVALTEXT\tEval list:\\n Source(5)\\n _get_frame(6)\n";
    let c = canon_orig(txt);
    assert_eq!(c.runs.len(), 1);
    assert_eq!(c.runs[0].1, vec!["CLIP", "[1]", "[0]"]);
    assert_eq!(c.dest, "R0");
}

#[test]
fn all_38_targets_compile_deterministically_with_folding() {
    big(|| {
        let base = effects_vecdir::root();
        let Ok(t) = std::fs::read_to_string(base.join("record_extract").join("programs").join("programs.tsv")) else { return };
        let Ok(forms) = std::fs::read_to_string(effects_vecdir::path("fx_eval/data").join("PROGRAM_FORMS.tsv")) else { return };
        let c = Corpus::from_programs_tsv(&t);
        let ct = combos();
        let mut done = 0;
        for (i, line) in forms.lines().enumerate() {
            let f: Vec<&str> = line.split('\t').collect();
            if i == 0 || f.len() < 3 || f[2] != "1" {
                continue;
            }
            let ei = c.find_effect(f[0], Some(f[1])).unwrap();
            let g = build_effect(&c, ei).unwrap();
            // HC-ZERO-LEGACY (named state A option)
            let sets = enumerate_branches_opt(&g, &ct, &BTreeMap::new(), true);
            assert!(!sets.is_empty());
            for s in &sets {
                let o = CompileOptions { overrides: s.overrides.clone(), paramset: s.paramset.clone(), hc_zero_legacy: true, ..Default::default() };
                let a = compile(&g, &ct, &o).unwrap();
                let b = compile(&g, &ct, &o).unwrap();
                assert_eq!(plan_text(&a), plan_text(&b));
                assert_eq!(a.plan_tsv(), b.plan_tsv());
            }
            done += 1;
        }
        assert_eq!(done, 38);
    });
}

#[test]
fn min_one_div_zero_by_image_folds_to_zero_like_time_warp_clamp0() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    // Time_Warp_RGB clamp_chroma = 0: `min 1 (/ 0 c)` becomes the constant 0, here with an image-valued c.
    // 0x4d1ec0: arg0 zero -> collapse to arg 0; elnums differ (1 vs 4) -> alias cancelled, node becomes a
    // broadcast constant (R6); min then folds in f32.
    let ct = combos();
    let g = gen("(def_effect T ( S ) ( ) ( ) ( min 1 ( / 0 S ) ))", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    let root = &cp.nodes[cp.root];
    assert_eq!(root.kind, "const");
    assert_eq!(root.elnum, 4);
    assert!(root.lanes.iter().all(|&b| f32::from_bits(b) == 0.0));
    assert!(cp.nodes.iter().all(|n| n.kind != "func"));
    // with a non-zero numerator the division stays a run
    let g = gen("(def_effect T ( S ) ( ) ( ) ( min 1 ( / 2 S ) ))", "T");
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert!(cp.nodes.iter().any(|n| n.prim == "/") && cp.nodes.iter().any(|n| n.prim == "min"));
}
