//! INT17 step 5: STYLE-SELECT on fx_run. --effect-answer N --effect-count K are judged before anything else
//! (fx_exec::style, BCJ3's render-path rule); a selected program other than the one run is refused by name.
use std::process::Command;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

fn run(prog: &str, extra: &[&str]) -> (i32, String) {
    let o = Command::new(env!("CARGO_BIN_EXE_fx_run")).args([pt(), prog]).args(extra).output().unwrap();
    (o.status.code().unwrap_or(-1), String::from_utf8_lossy(&o.stderr).to_string())
}

#[test]
fn mono_label_with_answer_1_is_refused() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (rc, e) = run("Blur_Mono", &["--effect-answer", "1", "--effect-count", "2"]);
    assert_eq!(rc, 2);
    assert!(e.contains("STYLE-SELECT-MISMATCH"), "{}", e);
}

#[test]
fn half_declaration_and_bad_count_are_refused() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let (rc, e) = run("Blur_Color", &["--effect-answer", "1"]);
    assert_eq!(rc, 2);
    assert!(e.contains("given together or not at all"), "{}", e);
    let (rc, e) = run("Blur_Color", &["--effect-answer", "1", "--effect-count", "3"]);
    assert_eq!(rc, 2);
    assert!(e.contains("STYLE-SELECT-COUNT-MISMATCH"), "{}", e);
    let (rc, e) = run("Diffuse_Autogen", &["--effect-answer", "1", "--effect-count", "1"]);
    assert_eq!(rc, 2);
    assert!(e.contains("STYLE-SELECT-NOT-TABLED"), "{}", e);
}

#[test]
fn matching_declaration_passes_the_style_gate() {
    // no --size: the run stops later for that reason, never for STYLE-SELECT
    let (_, e) = run("Blur_Mono_Autogen", &["--effect-answer", "2", "--effect-count", "2"]);
    assert!(!e.contains("STYLE-SELECT"), "{}", e);
    let (_, e) = run("Blur_Color", &["--effect-answer", "0", "--effect-count", "2"]);
    assert!(!e.contains("STYLE-SELECT"), "{}", e);
}
