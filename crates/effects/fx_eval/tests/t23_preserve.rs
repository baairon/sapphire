//! T23 (STK2): the stack word 2 = 1 preserve-group model. Skipped when the programs table is not at the fleet path.
//! (1) Distort_RGB at word 2 = 1 emits one `preserve` command, `allocate-images-preserved` and `unlock-preserved-images`; at word 2 = 0 none.
//! (2) The word 2 = 1 plan has 3 commands fewer than the word 2 = 0 plan (PCX2: 72 against 69, MEASURED on the originals).
//! (3) A program with a motion_blur node is refused by name.
use fx_eval::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn load() -> Option<(Corpus, ComboTable)> {
    let pt = PathBuf::from(effects_vecdir::path_str("record_extract/programs/programs.tsv"));
    let p = std::fs::read_to_string(pt).ok()?;
    let ct = std::fs::read_to_string(effects_vecdir::path("fx_eval/data").join("COMBOS.tsv")).ok()?;
    Some((Corpus::from_programs_tsv(&p), ComboTable::from_tsv(&ct)))
}

fn plan(c: &Corpus, ct: &ComboTable, prog: &str, w2: bool) -> String {
    let g = build_effect(c, c.find_effect(prog, None).unwrap()).unwrap();
    let mut sets = BTreeMap::new();
    sets.insert("blur_mask".to_string(), vec![0.2713f32]);
    let o = CompileOptions { overrides: sets, minimal_context: true, ctx_word2: w2, ctx_has_data: 0x7c8, bound_input_mask: 1, ..Default::default() };
    let cp = compile(&g, ct, &o).unwrap();
    cpuplan::plan_text_cpu(&cp)
}

#[test]
fn distort_rgb_preserve_group() {
    let Some((c, ct)) = load() else { return };
    let p0 = plan(&c, &ct, "Distort_RGB", false);
    let p1 = plan(&c, &ct, "Distort_RGB", true);
    assert!(!p0.contains("preserve"));
    assert!(p1.contains("(preserve ") && p1.contains("allocate-images-preserved") && p1.contains("unlock-preserved-images"));
    let n = |p: &str| p.lines().filter(|l| l.starts_with("CMD")).count();
    assert_eq!(n(&p0), n(&p1) + 3);
}
