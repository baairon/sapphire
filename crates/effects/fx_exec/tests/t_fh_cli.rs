//! Step FH (W617 (b), W620 (b)): the built fx_run refuses raw host frame planes (--frame-req) on the 'ae32' route with
//! FRAME-CONTENT-UNCONVERTED, and a State A run of Time_Warp_RGB still takes them (each missing request answered until rc 0).
use std::path::PathBuf;
use std::process::Command;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
const PROG: &str = "Time_Warp_RGB_Autogen";
const W: usize = 16;
const H: usize = 8;

fn dir(tag: &str) -> PathBuf {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("fh_cli_{}_{}", std::process::id(), tag));
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

#[test]
fn fh_cli_raw_frame_planes_refused_on_ae32() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let d = dir("ae32");
    let src = d.join("src.bin");
    std::fs::write(&src, floats(4 * W * H, 1)).unwrap();
    let prior = d.join("prior.u32");
    std::fs::write(&prior, vec![0xaau8; 16 * W * H]).unwrap();
    let frame = d.join("frame.f32");
    std::fs::write(&frame, floats(4 * W * H, 2)).unwrap();
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
    cmd.args(["--host-param", "blur_mocha_mask=0", "--set", "blur_mask=0.05", "--set", "invert_mask=0", "--set", "mask_use=0", "--set", "shift_mocha_mask=0,-0"]);
    cmd.args(["--frame-req", &format!("1,1={}", frame.display())]);
    let p = cmd.output().expect("fx_run");
    let err = String::from_utf8_lossy(&p.stderr).to_string();
    assert_eq!(p.status.code(), Some(2), "{}", err);
    assert!(err.contains(&format!("FRAME-CONTENT-UNCONVERTED ({}: a host frame was given as raw planes on the 'ae32' route", PROG)), "{}", err);
    assert!(!std::path::Path::new(&format!("{}.ae32.u32", os)).exists());
}

/// The (frame, stored) request named by fx_run's missing-host-frame message, if any.
fn missing_request(err: &str) -> Option<(i32, f32)> {
    let i = err.find("missing host frame get_frame request frame ")?;
    let t: Vec<&str> = err[i..].split_whitespace().collect();
    // missing host frame get_frame request frame F stored S (bits B) ...
    let fr = t.get(6)?.parse::<i32>().ok()?;
    let bits = t.get(10)?.trim_end_matches(')');
    let st = f32::from_bits(u32::from_str_radix(bits, 16).ok()?);
    Some((fr, st))
}

#[test]
fn fh_cli_state_a_still_takes_raw_frame_planes() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let d = dir("sa");
    let src = d.join("src.f32");
    std::fs::write(&src, floats(4 * W * H, 3)).unwrap();
    let o = d.join("o");
    let mut reqs: Vec<(String, PathBuf)> = Vec::new();
    for _ in 0..8 {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
        cmd.args([pt(), PROG, "--size", &format!("{}x{}", W, H), "--out", o.to_str().unwrap(), "--in", &format!("Source={}", src.display())]);
        cmd.args(["--convert", "3", "--ctx-time", "2", "--ctx-word2", "0", "--ctx-1ae", "0", "--ctx-has-data", "0", "--ranges", "REC-HARNESS"]);
        cmd.args(["--bound-mask", "1", "--host-frame", "2", "--tw-route", "240,0,1,1,0,0,1"]);
        for (k, p) in &reqs {
            cmd.args(["--frame-req", &format!("{}={}", k, p.display())]);
        }
        let p = cmd.output().expect("fx_run");
        let err = String::from_utf8_lossy(&p.stderr).to_string();
        assert!(!err.contains("FRAME-CONTENT-UNCONVERTED"), "{}", err);
        if p.status.code() == Some(0) {
            assert!(!reqs.is_empty(), "the State A run must have read at least one host frame");
            assert!(std::fs::metadata(format!("{}.dest.f32", o.display())).is_ok());
            return;
        }
        let (fr, st) = missing_request(&err).unwrap_or_else(|| panic!("unexpected State A failure: {}", err));
        let fp = d.join(format!("fr_{}_{:08x}.f32", fr, st.to_bits()));
        std::fs::write(&fp, floats(4 * W * H, 4 + reqs.len())).unwrap();
        reqs.push((format!("{},{}", fr, st), fp));
    }
    panic!("State A run did not finish after 8 host frame answers");
}
