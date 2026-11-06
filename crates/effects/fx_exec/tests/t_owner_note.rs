//! The OWNER-OPACITY-DERIVED note on an unbound row with a --set; the words do not change.
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
fn run_once(tag: &str, extra: &[&str], frames: &[(String, PathBuf)]) -> (Option<i32>, String, String, Vec<u8>) {
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
    for (k, p) in frames {
        cmd.args(["--frame-words", &format!("{}={}", k, p.display())]);
    }
    let p = cmd.output().expect("fx_run");
    let meta = std::fs::read_to_string(format!("{}.dest.meta.txt", os)).unwrap_or_default();
    let words = std::fs::read(format!("{}.ae32.u32", os)).unwrap_or_default();
    (p.status.code(), String::from_utf8_lossy(&p.stderr).to_string(), meta, words)
}

/// The (frame, stored) request named by fx_run's missing-host-frame message, if any.
fn missing_request(err: &str) -> Option<(i32, f32)> {
    let i = err.find("missing host frame get_frame request frame ")?;
    let t: Vec<&str> = err[i..].split_whitespace().collect();
    let fr = t.get(6)?.parse::<i32>().ok()?;
    let bits = t.get(10)?.trim_end_matches(')');
    Some((fr, f32::from_bits(u32::from_str_radix(bits, 16).ok()?)))
}

/// Runs the line, answering each missing host frame request with a frame given as words (step FH), until it finishes.
fn run(tag: &str, extra: &[&str]) -> (Option<i32>, String, String, Vec<u8>) {
    let mut frames: Vec<(String, PathBuf)> = Vec::new();
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("g2note_fr_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&d).unwrap();
    for _ in 0..8 {
        let r = run_once(tag, extra, &frames);
        match missing_request(&r.1) {
            Some((fr, st)) if r.0 != Some(0) => {
                let fp = d.join(format!("fr_{}_{:08x}.u32", fr, st.to_bits()));
                std::fs::write(&fp, floats(4 * W * H, 4 + frames.len())).unwrap();
                frames.push((format!("{},{}", fr, st), fp));
            }
            _ => return r,
        }
    }
    panic!("no finish after 8 host frame answers");
}

const NOTE_OUT: &str = "label OWNER-OPACITY-DERIVED (Time_Warp_RGB ";

// an unbound Time_Warp_RGB row with --set output_opacity (the registered default's loop value 0) prints the same note as the row without it,
// and the two runs give the same words.
#[test]
fn owner_opacity_note_prints_on_an_unbound_row_with_a_set() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let (rc0, err0, meta0, words0) = run("noset", &[]);
    assert_eq!(rc0, Some(0), "{}", err0);
    assert!(meta0.lines().any(|l| l.starts_with(NOTE_OUT) && l.contains("output_opacity")), "{}", meta0);
    let (rc1, err1, meta1, words1) = run("withset", &["--set", "output_opacity=0"]);
    assert_eq!(rc1, Some(0), "{}", err1);
    let n0: Vec<&str> = meta0.lines().filter(|l| l.starts_with(NOTE_OUT) && l.contains("output_opacity")).collect();
    let n1: Vec<&str> = meta1.lines().filter(|l| l.starts_with(NOTE_OUT) && l.contains("output_opacity")).collect();
    assert_eq!(n1.len(), 1, "{}", meta1);
    assert_eq!(n0, n1);
    assert!(!words0.is_empty() && words0 == words1, "no word change");
}
