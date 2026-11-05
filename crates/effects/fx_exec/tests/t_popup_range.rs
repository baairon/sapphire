//! POPUP-ANSWER-OUT-OF-RANGE (W630 (b)) at the command line (INT36).
use std::path::PathBuf;
use std::process::Command;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
const PROG: &str = "Time_Warp_RGB_Autogen";
const W: usize = 16;
const H: usize = 8;

fn floats(n: usize, seed: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for k in 0..n {
        let x = ((k * 7 + seed * 13) % 31) as f32 / 31.0;
        v.extend_from_slice(&x.to_le_bytes());
    }
    v
}

/// The TW line of t_envfence (default binding) plus EXTRA arguments; returns (rc, stderr, meta text, output words).
fn run(tag: &str, extra: &[&str]) -> (Option<i32>, String, String, Vec<u8>) {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("g2cli_{}_{}", std::process::id(), tag));
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
    cmd.args(extra);
    let p = cmd.output().expect("fx_run");
    let meta = std::fs::read_to_string(format!("{}.dest.meta.txt", os)).unwrap_or_default();
    let words = std::fs::read(format!("{}.ae32.u32", os)).unwrap_or_default();
    (p.status.code(), String::from_utf8_lossy(&p.stderr).to_string(), meta, words)
}

// W630 (b): the answer is judged before anything else of the line; out of range refuses with the exact text, in range does not.
#[test]
fn popup_answer_out_of_range_refuses_on_the_command_line() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    for (arg, v, n) in [("Output Opacity=3", 3, 2), ("Output Opacity=0", 0, 2), ("Input Opacity=4", 4, 3), ("Input Opacity=0", 0, 3)] {
        let name = arg.split('=').next().unwrap();
        let (rc, err, _, _) = run("oor", &["--cset", arg]);
        assert_eq!(rc, Some(2), "{}", err);
        let want = format!("POPUP-ANSWER-OUT-OF-RANGE ({}: '{}' answer {} outside 1..{}; W630)", PROG, name, v, n);
        assert!(err.contains(&want), "{}", err);
    }
}

#[test]
fn popup_answer_in_range_is_not_refused_for_range() {
    for arg in ["Output Opacity=1", "Output Opacity=2", "Input Opacity=1", "Input Opacity=3"] {
        let (_, err, _, _) = run("inr", &["--cset", arg]);
        assert!(!err.contains("POPUP-ANSWER-OUT-OF-RANGE"), "{}: {}", arg, err);
    }
}

