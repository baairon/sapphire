//! A class-3 bound input that is not an 'ae32' source (Dissolve_Shake's To at Opacity answer 1, where the plan drops the dissolve):
//! the class rule, the post-compile check, and the command line behaviour of the native program (no stored data needed).
use fx_exec::input_class::*;
use std::path::PathBuf;
use std::process::Command;

// ---- the class rule for the two declared inputs at Opacity answer 1 and at the default answer ----

/// Answer 1 cells: [owner+0x1354] 0, [owner+0x1358] 3, [prog+0x590] 1, [prog+0x594] 0, [prog+0x59f] 0, byte at the class-1 cell 1.
fn answer1() -> ClassCells {
    ClassCells { ae32_ready_c: 0, ae32_ready_d: 3, prog_mode_word: 1, prog_class_byte: Some(0), prog_flag_byte: Some(0), flag_2: Some(1) }
}
/// Answer 3 cells: [owner+0x1354] 2, [owner+0x1358] 0.
fn answer3() -> ClassCells {
    ClassCells { ae32_ready_c: 2, ae32_ready_d: 0, ..answer1() }
}
const R0: RowCells = RowCells { fmt_cell_a: 0, b_cell_o: 0, word_cell_a: 0 };

#[test]
fn from_and_to_derive_class_3_when_owner_1354_is_zero() {
    assert_eq!(derive_class(0, "From", &answer1(), &R0).unwrap(), 3);
    assert_eq!(derive_class(1, "To", &answer1(), &R0).unwrap(), 3);
}

#[test]
fn from_and_to_derive_class_4_when_owner_1354_is_nonzero() {
    assert_eq!(derive_class(0, "From", &answer3(), &R0).unwrap(), 4);
    assert_eq!(derive_class(1, "To", &answer3(), &R0).unwrap(), 4);
}

#[test]
fn the_name_arm_does_not_depend_on_the_context_row_or_the_class_byte() {
    // at [owner+0x1354] = 0 the exact names From and To return 0 before the row-derived value is used
    for rows in [R0, RowCells { fmt_cell_a: 1, b_cell_o: 0, word_cell_a: 0 }, RowCells { fmt_cell_a: 2, b_cell_o: 1, word_cell_a: 0 }] {
        for cb in [0u8, 1, 2, 0xff] {
            let c = ClassCells { prog_class_byte: Some(cb), ..answer1() };
            assert_eq!(derive_class(0, "From", &c, &rows).unwrap(), 3);
            assert_eq!(derive_class(1, "To", &c, &rows).unwrap(), 3);
        }
    }
}

#[test]
fn the_class_byte_cell_is_still_read_and_refused_when_unrecorded() {
    // [prog+0x594] is read when [owner+0x1358] is 3 (not 1); an unrecorded cell is refused by name
    let c = ClassCells { prog_class_byte: None, ..answer1() };
    assert_eq!(derive_class(1, "To", &c, &R0).unwrap_err(), ClassErr::Unrecorded("[prog]"));
}

#[test]
fn an_unlisted_name_at_owner_1354_zero_is_still_refused_by_name() {
    let e = derive_class(1, "Bumps", &answer1(), &R0).unwrap_err();
    assert!(matches!(e, ClassErr::ArmNotPorted(_)), "{:?}", e);
}

// ---- the post-compile check ----

#[test]
fn unread_input_absent_from_the_plan_passes() {
    assert!(check_unread_inputs("Dissolve_Shake", &["To".to_string()], &["From"]).is_ok());
    assert!(check_unread_inputs("Dissolve_Shake", &[], &["From", "To"]).is_ok());
}

#[test]
fn unread_input_present_in_the_plan_is_refused_by_name() {
    let e = check_unread_inputs("Dissolve_Shake", &["To".to_string()], &["From", "To"]).unwrap_err();
    assert_eq!(e, "INPUT-CLASS-NOT-PORTED (Dissolve_Shake input To: class 3 for an input that is not an 'ae32' source)");
    // only the exact name matches
    assert!(check_unread_inputs("S", &["To".to_string()], &["Toe", "From"]).is_ok());
}

#[test]
fn the_label_names_the_input_and_the_elnum() {
    let l = unread_label("Dissolve_Shake", "To");
    assert!(l.starts_with("label INPUT-CLASS-UNREAD-INPUT (Dissolve_Shake input To:"), "{}", l);
    assert!(l.contains("elnum 3"), "{}", l);
}

// ---- the native program on the command line ----

const W: usize = 64;
const H: usize = 32;

fn words(seed: u32) -> Vec<u8> {
    // 4 words per pixel, finite values in [0, 1]
    let mut v = Vec::new();
    for k in 0..(4 * W * H) as u32 {
        let x = ((k.wrapping_mul(2654435761).wrapping_add(seed.wrapping_mul(40503))) >> 8 & 0xffff) as f32 / 65535.0;
        v.extend_from_slice(&x.to_le_bytes());
    }
    v
}

struct Out {
    rc: Option<i32>,
    err: String,
    meta: String,
    plan: String,
    words: Vec<u8>,
}

/// The Opacity answer 1 line of Dissolve_Shake at 64x32; `sets` replaces the default parameter sets, `extra` is appended.
fn run(tag: &str, sets: &[&str], to_source: bool, extra: &[&str]) -> Out {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("unread_c3_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&d).unwrap();
    let src = d.join("src.u32");
    std::fs::write(&src, words(1)).unwrap();
    let to = d.join("to.u32");
    std::fs::write(&to, words(2)).unwrap();
    let prior = d.join("prior.u32");
    std::fs::write(&prior, vec![0u8; 16 * W * H]).unwrap();
    let os = d.join("o").to_str().unwrap().to_string();
    let plan = d.join("plan.txt");
    let out = d.join("out.u32");
    let _ = std::fs::remove_file(&out);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
    cmd.args(["Dissolve_Shake", "--size", "64x32", "--out", &os, "--ae32-src", &format!("From={}", src.display())]);
    if to_source {
        cmd.args(["--ae32-src", &format!("To={}", to.display())]);
    }
    cmd.args(["--ae32-prior", prior.to_str().unwrap(), "--ae32-out", out.to_str().unwrap()]);
    cmd.args(["--geom", "-0.5,63.5,1,0,-16.5,0,0", "--host-frame", "0", "--ctx-time", "0", "--src-planes", "3", "--owner-arm", "0,0,0"]);
    cmd.args(["--src-row58", "0", "--src-head", "0x7e91c0", "--aspect-triple", "0,3f800000,3f800000", "--ae32-dest", "3,3", "--ae32-copier", "none"]);
    cmd.args(["--dest-stack-word", "0", "--units-ratio", "1,1", "--bound-mask", "3", "--ctx-word2", "0", "--ctx-1ae", "3", "--ctx-has-data", "f8"]);
    cmd.args(["--ranges", "REAL", "--xy-base", "0", "--cset", "Opacity=1"]);
    for s in sets {
        cmd.args(["--set", s]);
    }
    cmd.args(["--host-value", "get_time=0x00000000", "--host-value", "get_percent_time=0x00000000", "--host-value", "get_total_frames=0x43700000"]);
    cmd.args(["--host-value", "get_frames_per_second=0x41c00000", "--host-value", "get_aspect_ratio=0x3f000000", "--ns-default-unbound", "--do-postdiv", "0"]);
    cmd.args(["--plan-text", plan.to_str().unwrap()]);
    cmd.args(extra);
    let p = cmd.output().expect("fx_run");
    Out {
        rc: p.status.code(),
        err: String::from_utf8_lossy(&p.stderr).to_string(),
        meta: std::fs::read_to_string(format!("{}.dest.meta.txt", os)).unwrap_or_default(),
        plan: std::fs::read_to_string(&plan).unwrap_or_default(),
        words: std::fs::read(&out).unwrap_or_default(),
    }
}

const CELLS: [&str; 4] = ["--class-cells", "00,00,01", "--class-record", "From=3,To=3"];
const SETS_ELIDED: [&str; 2] = ["transition_dir=0.0", "auto_trans=1.0"];
const SETS_DISSOLVE: [&str; 3] = ["transition_dir=0.0", "auto_trans=0.0", "dissolve_amt=0.45"];

fn extra<'a>(rec: &'a [&'a str]) -> Vec<&'a str> {
    let mut v: Vec<&str> = CELLS.to_vec();
    v.extend_from_slice(rec);
    v
}

#[test]
fn dissolve_dropped_to_unread_renders_one_image() {
    let o = run("ok", &SETS_ELIDED, false, &extra(&["--class3-record", "0,1,3,02"]));
    assert_eq!(o.rc, Some(0), "{}", o.err);
    assert_eq!(o.words.len(), 16 * W * H);
    assert!(o.meta.contains("label INPUT-CLASS-UNREAD-INPUT (Dissolve_Shake input To:"), "{}", o.meta);
    assert!(o.meta.contains("label ALLOCATE-PLANNED (Dissolve_Shake: 1 image(s), elnum 3;"), "{}", o.meta);
    assert!(o.meta.contains("derive classes [3, 3]"), "{}", o.meta);
    assert!(o.meta.contains("per-run record check 2 of 2 equal"), "{}", o.meta);
    assert!(o.plan.contains("NODE\t0\tFrom\tkind=input"), "{}", o.plan);
    assert!(!o.plan.contains("\tTo\tkind=input"), "{}", o.plan);
    assert_eq!(o.plan.matches("convert-from-source").count(), 1);
    // determinism
    let o2 = run("ok2", &SETS_ELIDED, false, &extra(&["--class3-record", "0,1,3,02"]));
    assert_eq!(o.words, o2.words);
}

#[test]
fn allocate_count_of_two_is_refused_while_to_is_unread() {
    let o = run("n2", &SETS_ELIDED, false, &extra(&["--class3-record", "0,2,3,02"]));
    assert_eq!(o.rc, Some(2));
    assert!(o.err.contains("INPUT-CLASS-ALLOC-MISMATCH (Dissolve_Shake: planned allocate command 1 image(s), elnum 3; record 2 image(s), elnum 3)"), "{}", o.err);
    assert!(o.words.is_empty());
}

#[test]
fn allocate_elnum_of_four_is_refused() {
    let o = run("e4", &SETS_ELIDED, false, &extra(&["--class3-record", "0,1,4,02"]));
    assert_eq!(o.rc, Some(2));
    assert!(o.err.contains("INPUT-CLASS-ALLOC-MISMATCH (Dissolve_Shake: planned allocate command 1 image(s), elnum 3; record 1 image(s), elnum 4)"), "{}", o.err);
}

#[test]
fn builder_hits_are_still_refused() {
    let o = run("bh", &SETS_ELIDED, false, &extra(&["--class3-record", "1,1,3,02"]));
    assert_eq!(o.rc, Some(2));
    assert!(o.err.contains("CLASS3-BUILDER-ON-ROUTE"), "{}", o.err);
}

#[test]
fn missing_class_cells_are_still_refused_by_name() {
    let o = run("nc", &SETS_ELIDED, false, &["--class3-record", "0,1,3,02"]);
    assert_eq!(o.rc, Some(2));
    assert!(o.err.contains("INPUT-CLASS-CELL-UNRECORDED (Dissolve_Shake: [prog])"), "{}", o.err);
}

#[test]
fn missing_class3_record_is_refused_by_name() {
    let o = run("nr", &SETS_ELIDED, false, &extra(&[]));
    assert_eq!(o.rc, Some(2));
    assert!(o.err.contains("INPUT-CLASS-CELL-UNRECORDED (Dissolve_Shake: the class-3 record"), "{}", o.err);
}

#[test]
fn a_plan_that_reads_to_without_a_source_is_refused_after_compile() {
    let o = run("rd", &SETS_DISSOLVE, false, &extra(&["--class3-record", "0,1,3,02"]));
    assert_eq!(o.rc, Some(2), "{}", o.err);
    assert!(o.err.contains("INPUT-CLASS-NOT-PORTED (Dissolve_Shake input To: class 3 for an input that is not an 'ae32' source)"), "{}", o.err);
    assert!(o.words.is_empty());
}

#[test]
fn a_plan_that_reads_to_with_a_source_takes_two_images() {
    let o = run("t2", &SETS_DISSOLVE, true, &extra(&["--class3-record", "0,2,3,02"]));
    assert_eq!(o.rc, Some(0), "{}", o.err);
    assert!(o.meta.contains("label ALLOCATE-PLANNED (Dissolve_Shake: 2 image(s), elnum 3;"), "{}", o.meta);
    assert!(!o.meta.contains("INPUT-CLASS-UNREAD-INPUT"), "{}", o.meta);
    assert!(o.plan.contains("\tTo\tkind=input"), "{}", o.plan);
    assert_eq!(o.plan.matches("convert-from-source").count(), 2);
    // one image given with the count 1 is a mismatch
    let o1 = run("t1", &SETS_DISSOLVE, true, &extra(&["--class3-record", "0,1,3,02"]));
    assert_eq!(o1.rc, Some(2));
    assert!(o1.err.contains("INPUT-CLASS-ALLOC-MISMATCH"), "{}", o1.err);
}

#[test]
fn a_given_to_source_that_the_plan_drops_is_refused() {
    let o = run("ts", &SETS_ELIDED, true, &extra(&["--class3-record", "0,2,3,02"]));
    assert_ne!(o.rc, Some(0));
    assert!(o.err.contains("'ae32' source To is not an input of the compiled plan"), "{}", o.err);
}
