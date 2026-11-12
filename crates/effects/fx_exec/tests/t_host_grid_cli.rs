//! The built fx_run refuses an off-grid --host-param value and an off-grid --set lane of a host type 2 slider on the 'ae32' route
//! (HOST-VALUE-OFF-GRID) and never snaps one. Needs the program text under VEC_DIR; prints NOT RUN without it.
use std::path::PathBuf;
use std::process::Command;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
const PROG: &str = "Time_Warp_RGB_Autogen";
const W: usize = 16;
const H: usize = 8;

fn dir(tag: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("hg_cli_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn floats(n: usize, seed: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for k in 0..n {
        let x = ((k * 7 + seed * 13) % 31) as f32 / 31.0;
        v.extend_from_slice(&x.to_le_bytes());
    }
    v
}


fn run_with(tag: &str, params: &[&str]) -> (Option<i32>, String) {
    let d = dir(tag);
    let src = d.join("src.bin");
    std::fs::write(&src, floats(4 * W * H, 1)).unwrap();
    let prior = d.join("prior.u32");
    std::fs::write(&prior, vec![0xaau8; 16 * W * H]).unwrap();
    let o = d.join("o");
    let os = o.to_str().unwrap().to_string();
    let head = format!("{:#x}", fx_exec::hostconv14::R3_HEAD);
    let geom = format!("-0.5,{},1,0,{},0,0", W as f32 - 0.5, (H as f32 - W as f32) / 2.0 - 0.5);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
    cmd.args([pt(), PROG, "--size", &format!("{}x{}", W, H), "--out", &os]);
    cmd.args(["--ae32-src", &format!("Source={}", src.display()), "--src-planes", "3", "--owner-arm", "0,0,0", "--owner-opacity", "1"]);
    cmd.args(["--src-row58", "0", "--src-head", &head, "--aspect-triple", "0,3f800000,3f800000", "--geom", &geom]);
    cmd.args(["--ae32-dest", "3,3", "--ae32-prior", prior.to_str().unwrap(), "--ae32-copier", "none", "--host-inplace", "1", "--dest-stack-word", "0"]);
    cmd.args(["--ae32-out", &format!("{}.ae32.u32", os), "--units-ratio", "1,1", "--do-postdiv", "0", "--bound-mask", "1", "--host-frame", "2"]);
    cmd.args(["--ctx-time", "2", "--ctx-word2", "0", "--ctx-1ae", "4", "--ctx-has-data", "7f8", "--ranges", "REAL", "--prerender-late", "0,0,0"]);
    cmd.args(["--class-cells", "02,80,-", "--class-record", "Source=3", "--class3-record", "0,1,3,02", "--tw-route", "240,0,1,1,0,0,1"]);
    cmd.args(params);
    let p = cmd.output().expect("fx_run");
    (p.status.code(), String::from_utf8_lossy(&p.stderr).to_string())
}

const BASE_SET: [&str; 6] = ["--set", "invert_mask=0", "--set", "mask_use=0", "--set", "shift_mocha_mask=0,-0"];

#[test]
fn hg_cli_off_grid_host_param_is_refused_and_not_snapped() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        println!("NOT RUN: the program text is absent");
        return;
    }
    let mut a = vec!["--host-param", "blur_mocha_mask=0.1", "--set", "blur_mask=0.05"];
    a.extend_from_slice(&BASE_SET);
    let (rc, err) = run_with("hp", &a);
    assert_eq!(rc, Some(2), "{}", err);
    assert!(err.contains(&format!("HOST-VALUE-OFF-GRID ({} blur_mocha_mask: --host-param 0.1 is ", PROG)), "{}", err);
}

#[test]
fn hg_cli_on_grid_values_pass_the_grid_checks() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        println!("NOT RUN: the program text is absent");
        return;
    }
    // a host value on the grid (0.5 -> lane 0.5 x 0.5 = 0.25 at width 16) and a text default lane pass; the run goes on to its next check
    let mut a = vec!["--host-param", "blur_mocha_mask=0.5", "--set", "blur_mask=0.05"];
    a.extend_from_slice(&BASE_SET);
    let (_, err) = run_with("ok", &a);
    assert!(!err.contains("OFF-GRID") && !err.contains("HOST-ROW-UNRESOLVED"), "{}", err);
}
