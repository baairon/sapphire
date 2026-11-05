//! ENVFENCE (W618 (b)): the 'ae32' route refuses with ENV-SWITCH-REFUSED when any fenced environment name is set (WPG1_PBASE named by
//! the brief, plus every name added by this step), and the default line (no switch set) never reaches that refusal.
use std::path::PathBuf;
use std::process::Command;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
const PROG: &str = "Time_Warp_RGB_Autogen";
const W: usize = 16;
const H: usize = 8;

const ADDED: [&str; 17] = [
    "WPG1_PBASE", "WPG1_ABASE", "WPG1_S8", "WPG1_S48", "WPG1_S30", "WPG12_OLDFMT", "DSW3_UNBOUND_OFF", "FLD1_NO_COPIES", "FLD1_NO_INTTYPE",
    "NEG2_RANGES_OFF", "LYM2_REPORT", "NV3E_DONOR", "NV3E_DS", "NV3E_FLAG", "NV3E_INPLACE", "NV3E_LASTC", "NV3E_SLOT",
];

fn floats(n: usize, seed: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for k in 0..n {
        let x = ((k * 7 + seed * 13) % 31) as f32 / 31.0;
        v.extend_from_slice(&x.to_le_bytes());
    }
    v
}

fn run(tag: &str, env: Option<&str>) -> (Option<i32>, String) {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("envfence_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&d).unwrap();
    let src = d.join("src.bin");
    std::fs::write(&src, floats(4 * W * H, 1)).unwrap();
    let prior = d.join("prior.u32");
    std::fs::write(&prior, vec![0xaau8; 16 * W * H]).unwrap();
    let os = d.join("o").to_str().unwrap().to_string();
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
    cmd.args(["--host-param", "blur_mocha_mask=0", "--set", "blur_mask=0.05", "--set", "invert_mask=0", "--set", "mask_use=0", "--set", "shift_mocha_mask=0,-0"]);
    for n in ADDED {
        cmd.env_remove(n);
    }
    if let Some(n) = env {
        cmd.env(n, "1");
    }
    let p = cmd.output().expect("fx_run");
    (p.status.code(), String::from_utf8_lossy(&p.stderr).to_string())
}

#[test]
fn envfence_wpg1_pbase_refuses_with_the_existing_switch_refusal() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let (rc, err) = run("pbase", Some("WPG1_PBASE"));
    assert_eq!(rc, Some(2), "{}", err);
    assert!(err.contains("ENV-SWITCH-REFUSED: WPG1_PBASE is set; the 'ae32' route runs with no environment switch"), "{}", err);
}

#[test]
fn envfence_every_added_name_refuses() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    for (i, n) in ADDED.iter().enumerate() {
        let (_, err) = run(&format!("n{}", i), Some(n));
        assert!(err.contains(&format!("ENV-SWITCH-REFUSED: {} is set", n)), "{}: {}", n, err);
    }
}

#[test]
fn envfence_default_line_has_no_env_refusal() {
    let (_, err) = run("none", None);
    assert!(!err.contains("ENV-SWITCH-REFUSED"), "{}", err);
}
