//! INT24 (W526 (a) 2): the claim-route tripwire OPS-ARITH-SCALAR-SCALAR-UNJUDGED and the OPS_SCALAR_PAIRS census rows.
//! Constructed rows for the tripwire text and its trigger set; one real compile (Hue_Sat_Bright_Autogen, state A defaults as plan_dump) for the rows.
use fx_exec::ops_census::{census_line, ops_rows, scalar_scalar_tripwire, OpsRow};

fn row(op: &str, plane: [bool; 2]) -> OpsRow {
    OpsRow { node: 12, gid: 30, op: op.into(), plane: plane.to_vec(), counts: vec![3, 1], nout: 3, path: "x: * a b".into() }
}

#[test]
fn tripwire_fires_on_both_non_plane() {
    for op in ["+", "*", "-", "/"] {
        let m = scalar_scalar_tripwire("P", &[row(op, [false, false])]).expect("tripwire");
        assert_eq!(
            m,
            format!("OPS-ARITH-SCALAR-SCALAR-UNJUDGED (P: node 12: {} with both operands non-plane, counts 3,1, nout 3; this operand pair is outside ops_arith's judged R cases; PLR4, W526)", op)
        );
    }
}

#[test]
fn tripwire_silent_on_judged_pairs_and_dissolve() {
    for p in [[true, true], [true, false], [false, true]] {
        assert!(scalar_scalar_tripwire("P", &[row("+", p), row("*", p)]).is_none());
    }
    // dissolve is a census column (W526 (b) 4), not a tripwire class
    assert!(scalar_scalar_tripwire("P", &[row("dissolve", [false, false])]).is_none());
}

#[test]
fn census_rows_of_a_real_plan() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = fx_eval::Corpus::from_programs_tsv(&tsv);
    let g = fx_eval::build_effect(&c, c.find_effect("Hue_Sat_Bright_Autogen", None).unwrap()).unwrap();
    let mut o = fx_eval::CompileOptions::default();
    o.const_inputs.insert("Mask".into(), vec![1.0]);
    // FLD2 binding A at Source 3 channels (the default binding is an identity plan with no run-primitive)
    o.input_elnum.insert("Source".into(), 3);
    o.overrides.insert("scale_colors".into(), vec![0.8125, 1.0625, 0.9375]);
    o.overrides.insert("brightness".into(), vec![1.0]);
    o.overrides.insert("offset_darks".into(), vec![0.0]);
    let cp = fx_eval::compile(&g, &fx_exec::exec::combo_table(), &o).expect("compile");
    let rows = ops_rows(&cp);
    assert_eq!(rows.len(), 1, "{:?}", rows);
    assert_eq!((rows[0].op.as_str(), rows[0].plane.clone(), rows[0].counts.clone(), rows[0].nout), ("*", vec![true, false], vec![3, 3], 3));
    assert!(scalar_scalar_tripwire("Hue_Sat_Bright_Autogen", &rows).is_none());
    for r in &rows {
        let l = census_line(r);
        assert!(l.starts_with("diagnostic OPS-ARITH-RUN\tnode "), "{}", l);
        assert!(l.contains("\tpath "), "{}", l);
        assert_eq!(r.plane.len(), r.counts.len());
    }
}
