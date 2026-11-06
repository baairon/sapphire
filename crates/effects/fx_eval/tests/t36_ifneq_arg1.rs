//! INT21 step Q: a non-constant second argument of if_neq_const is noted for fx_run's claim-route refusal IFNEQ-CONST-ARG1-NONCONST.
//! EQC1's c7 probe text (if_neq_const 1 x ...) is noted; c6's (if_neq_const x 1 ...) is not. One test function: the note is a process-wide cell.
use fx_eval::{build_effect, compile, enumerate_branches_opt, ComboTable, CompileOptions, Corpus};
use std::collections::BTreeMap;

fn noted(case: &str) -> (usize, Vec<usize>) {
    let dir = effects_vecdir::path_str("fx_eval");
    let c = Corpus::from_programs_tsv(&std::fs::read_to_string(format!("{}/tests/eqc1_probe/progs_{}.tsv", dir, case)).unwrap());
    let ct = ComboTable::from_tsv(&std::fs::read_to_string(format!("{}/tests/eqc1_probe/PROGRAM_FORMS.tsv", dir)).unwrap());
    let g = build_effect(&c, c.find_effect("Blend", None).unwrap()).unwrap();
    let _ = fx_eval::compile::ifneq_arg1_nonconst_take();
    let mut n = 0;
    for s in enumerate_branches_opt(&g, &ct, &BTreeMap::new(), false) {
        let opt = CompileOptions { overrides: s.overrides.clone(), strict: false, paramset: s.paramset.clone(), ..Default::default() };
        compile(&g, &ct, &opt).unwrap();
        n += 1;
    }
    (n, fx_eval::compile::ifneq_arg1_nonconst_take())
}

#[test]
fn t36_c7_noted_c6_not() {
    if !effects_vecdir::have_all(&["fx_eval"]) {
        return;
    }
    let (n6, ids6) = noted("c6");
    assert!(n6 > 0);
    assert!(ids6.is_empty(), "c6 (argument 0 = x, argument 1 constant) must not be noted: {:?}", ids6);
    let (n7, ids7) = noted("c7");
    assert!(n7 > 0);
    assert!(!ids7.is_empty(), "c7 (argument 1 = x) must be noted for IFNEQ-CONST-ARG1-NONCONST");
    let (_, ids2) = noted("c2");
    assert!(ids2.is_empty(), "c2 (both constant) must not be noted: {:?}", ids2);
}
