//! ALS1: INPUT-DEFAULT-ALIAS (W353 (e), W414 (d)). An unbound input whose text default names another input binds that input's planes;
//! an unbound input without such a default is still refused. Runs the fx_run binary on Distort_RGB (Lens defaults to Source).
use std::process::Command;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

fn run(mask: &str, with_src: bool, lens_raw: bool, tag: &str) -> (i32, String, Option<Vec<u8>>) {
    let dir = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("als1_t26_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).unwrap();
    let (w, h) = (16usize, 8usize);
    let n = w * h;
    let mut raw = Vec::new();
    for c in 0..4 {
        for k in 0..n {
            let v = ((k * 7 + c * 13) % 31) as f32 / 31.0;
            raw.extend_from_slice(&v.to_le_bytes());
        }
    }
    let src = dir.join("src.f32");
    std::fs::write(&src, &raw).unwrap();
    let out = dir.join("o");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
    cmd.args([pt(), "Distort_RGB", "--size", "16x8", "--out", out.to_str().unwrap()]);
    if with_src {
        cmd.args(["--in", &format!("Source={}", src.display())]);
    }
    if lens_raw {
        cmd.args(["--in", &format!("Lens={}", src.display())]);
    }
    cmd.args(["--convert", "3", "--ctx-time", "0", "--ctx-word2", "0", "--ctx-1ae", "0", "--ctx-has-data", "0", "--ranges", "REC-HARNESS", "--bound-mask", mask]);
    let o = cmd.output().expect("fx_run");
    let dest = std::fs::read(format!("{}.dest.f32", out.display())).ok();
    (o.status.code().unwrap_or(-1), String::from_utf8_lossy(&o.stderr).to_string(), dest)
}

#[test]
fn unbound_lens_takes_the_source_planes() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let (rc, err, dest) = run("1", true, false, "a");
    assert_eq!(rc, 0, "{}", err);
    assert!(dest.is_some());
}

#[test]
fn bound_raw_lens_is_not_the_alias() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let (rc, _, alias) = run("1", true, false, "b");
    let (rc2, err2, raw) = run("3", true, true, "c");
    assert_eq!((rc, rc2), (0, 0), "{}", err2);
    assert_ne!(alias.unwrap(), raw.unwrap(), "the raw (unconverted) Lens control must differ from the converted alias");
}

#[test]
fn unbound_input_without_a_default_is_still_refused() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    // Source has no text default: mask 0 and no image leaves it unbound and the plan that reads it is refused
    let (rc, err, dest) = run("0", false, false, "d");
    assert_ne!(rc, 0);
    assert!(dest.is_none());
    assert!(err.contains("not given") || err.contains("INPUT-DEFAULT-ALIAS"), "{}", err);
}
