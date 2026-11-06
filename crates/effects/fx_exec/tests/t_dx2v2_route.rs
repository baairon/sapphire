//! Step DX2 v2: DX2's bound-count rule for v3_to_v4_if4 runs only when CompileOptions::v3_bound_count is set (fx_run sets it on
//! the 'ae32' route only). On State A the node keeps its own count and no V3TOV4IF4-COUNT-UNBOUND refusal is raised.
use fx_exec::exec::combo_table;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

/// plan_dump's State A options (Mask := 1, no compile context), with the DX2 v2 flag as given.
fn state_a(prog: &str, flag: bool) -> (Result<(), String>, Vec<(usize, usize)>) {
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = fx_eval::Corpus::from_programs_tsv(&tsv);
    let g = fx_eval::build_effect(&c, c.find_effect(prog, None).unwrap()).unwrap();
    let mut o = fx_eval::CompileOptions::default();
    o.const_inputs.insert("Mask".into(), vec![1.0]);
    o.v3_bound_count = flag;
    let _ = fx_eval::fold::v3_take();
    let r = fx_eval::compile(&g, &combo_table(), &o).map(|_| ()).map_err(|e| e.msg);
    let v = fx_eval::fold::v3_take().iter().map(|x| (x.0, x.1)).collect();
    (r, v)
}

fn plan_dump(prog: &str) -> (i32, String, String) {
    let o = std::process::Command::new(env!("CARGO_BIN_EXE_plan_dump")).args([pt(), prog, "--state-a-defaults", "--cpu", "--gid"]).output().expect("plan_dump");
    (o.status.code().unwrap_or(-1), String::from_utf8_lossy(&o.stdout).to_string(), String::from_utf8_lossy(&o.stderr).to_string())
}

#[test]
fn dx2v2_state_a_drop_shadow_node_folds_at_its_own_count() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let (rc, out, err) = plan_dump("Drop_Shadow");
    assert_eq!(rc, 0, "{}", err);
    let f: Vec<&str> = out.lines().filter(|l| l.starts_with("V3FOLD\t")).collect();
    assert_eq!(f.len(), 1, "{:?}", f);
    assert!(f[0].contains("\tn 3\t"), "{}", f[0]);
    // the library at State A agrees: own count 3
    let (r, v) = state_a("Drop_Shadow", false);
    assert!(r.is_ok(), "{:?}", r);
    assert_eq!(v.iter().map(|x| x.1).collect::<Vec<_>>(), vec![3]);
}

#[test]
fn dx2v2_state_a_original_compiles() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let (rc, _, err) = plan_dump("Original");
    assert_eq!(rc, 0, "{}", err);
    assert!(!err.contains("V3TOV4IF4-COUNT-UNBOUND"), "{}", err);
    assert!(state_a("Original", false).0.is_ok());
}

#[test]
fn dx2v2_flag_with_unbound_count_refuses_with_the_exact_text() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let e = state_a("Original", true).0.unwrap_err();
    assert!(
        e.contains("V3TOV4IF4-COUNT-UNBOUND (Original: node 156: the element count of the bound image argument is not known at compile; the original stores it from the bound source; DCP1, W603)"),
        "{}",
        e
    );
}
