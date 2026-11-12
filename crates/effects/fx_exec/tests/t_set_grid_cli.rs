//! The built fx_run refuses an off-grid --set lane of a host type 2 slider on the 'ae32' route before any output
//! (SET-VALUE-OFF-GRID for a slider without normalize_scale, SET-LANE-OFF-GRID for a normalize_scale slider) and accepts
//! a lane that is the record lane of a 16.16 host value at the frame width. Needs the program text under VEC_DIR; prints
//! NOT RUN without it.
use std::path::PathBuf;
use std::process::Command;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

fn dir(tag: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("sg_cli_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&d).unwrap();
    d
}

/// One fx_run line of PROG at W x H with the given option words after the common ones.
fn run(tag: &str, prog: &str, w: usize, h: usize, geom: &str, params: &[&str]) -> (Option<i32>, String, bool) {
    let d = dir(tag);
    let src = d.join("src.u32");
    std::fs::write(&src, vec![0u8; 4 * 4 * w * h]).unwrap();
    let prior = d.join("prior.u32");
    std::fs::write(&prior, vec![0u8; 4 * 4 * w * h]).unwrap();
    let o = d.join("o");
    let os = o.to_str().unwrap().to_string();
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
    cmd.args([pt(), prog, "--size", &format!("{}x{}", w, h), "--out", &os]);
    cmd.args(["--src-planes", "4", "--owner-arm", "0,0,0", "--src-row58", "0", "--src-head", "0x7eb3d0", "--aspect-triple", "0,3f800000,3f800000"]);
    cmd.args(["--ae32-dest", "4,1", "--ae32-copier", "none", "--dest-stack-word", "0", "--units-ratio", "1,1", "--do-postdiv", "1", "--bound-mask", "1"]);
    cmd.args(["--host-frame", "0", "--ctx-time", "0", "--ctx-word2", "0", "--ctx-1ae", "4", "--ctx-has-data", "f8", "--ranges", "REAL", "--geom", geom]);
    let src_arg = format!("{}={}", if prog == "Drop_Shadow" { "Foreground" } else { "Source" }, src.display());
    cmd.args(["--ae32-src", &src_arg, "--ae32-prior", prior.to_str().unwrap(), "--ae32-out", &format!("{}.ae32.u32", os)]);
    cmd.args(params);
    let p = cmd.output().expect("fx_run");
    let wrote = o.exists() || std::path::Path::new(&format!("{}.ae32.u32", os)).exists();
    (p.status.code(), String::from_utf8_lossy(&p.stderr).to_string(), wrote)
}

fn ds(tag: &str, w: usize, h: usize, shift: &str) -> (Option<i32>, String, bool) {
    let geom = format!("-0.5,{},1,0,{},0,0", w as f32 - 0.5, (h as f32 - w as f32) / 2.0 - 0.5);
    let shift = format!("shift={}", shift);
    run(tag, "Drop_Shadow", w, h, &geom, &[
        "--src-chan", "4", "--host-param", "shadow_blur=24", "--host-param", "blur_mocha_mask=0", "--set", "color=0,0,0", "--set", "shadow_opacity=1",
        "--set", &shift, "--set", "fg_opacity=1", "--set", "comp_premult=1", "--set", "matte_use=1", "--set", "invert_matte=0", "--set", "combine_masks=0",
        "--set", "mocha_mask_opacity=1", "--set", "show_mocha_mask_only=0", "--set", "resize_mocha_mask=1", "--set", "shift_mocha_mask=0,-0",
        "--set", "resize_mocha_rel_x=1", "--set", "resize_mocha_rel_y=1", "--set", "invert_mocha_mask=0", "--set", "bypass_mocha_mask=0",
    ])
}

fn have() -> bool {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        println!("NOT RUN: the program text is absent");
        return false;
    }
    true
}

#[test]
fn sg_cli_edges_threshold_off_grid_is_refused() {
    if !have() { return; }
    let (rc, err, wrote) = run("ge", "Glow_Edges", 64, 32, "-0.5,63.5,1,0,-16.5,0,0", &[
        "--host-param", "blur_mocha_mask=0", "--host-param", "shift_mocha_mask=0,0", "--set", "edges_smooth=0.375", "--set", "edges_brightness=1.5",
        "--set", "edges_threshold=0.3499999940395355", "--host-param", "width=53.75999450683594",
    ]);
    assert_eq!(rc, Some(2), "{}", err);
    assert!(err.contains("SET-VALUE-OFF-GRID (Glow_Edges edges_threshold: --set 0.35 is "), "{}", err);
    assert!(!wrote, "no output before the refusal");
}

#[test]
fn sg_cli_drop_shadow_shift_of_the_wrong_width_is_refused() {
    if !have() { return; }
    let (rc, err, wrote) = ds("ds50bad", 50, 7, "0.00299692154,-0.00117826462");
    assert_eq!(rc, Some(2), "{}", err);
    assert!(err.contains("SET-LANE-OFF-GRID (Drop_Shadow shift: --set 0.0029969215 is not the record lane of any 16.16 host value at width 50)"), "{}", err);
    assert!(!wrote, "no output before the refusal");
}

#[test]
fn sg_cli_drop_shadow_shift_on_the_lattice_passes_the_set_check() {
    if !have() { return; }
    let (_, err, _) = ds("ds50ok", 50, 7, "0.00383605948,-0.00150817866");
    assert!(!err.contains("OFF-GRID"), "{}", err);
    let (_, err, _) = ds("ds64ok", 64, 32, "0.00299692154,-0.00117826462");
    assert!(!err.contains("OFF-GRID"), "{}", err);
}
