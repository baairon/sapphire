//! Retime_Longer with the Result popup bound to a non-zero answer: the answer needs get_frame_width, which no fold resolves and no
//! kernel wrapper covers. fx_run refuses it by name (status 2, no output) instead of failing as a broken renderer.
use std::path::PathBuf;
use std::process::Command;

const W: usize = 50;
const H: usize = 7;

fn plane_words(seed: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for k in 0..4 * W * H {
        let x = if k < W * H { 1.0f32 } else { ((k * 7 + seed) % 31) as f32 / 31.0 };
        v.extend_from_slice(&x.to_le_bytes());
    }
    v
}

fn run(result: &str, tag: &str) -> (Option<i32>, String, PathBuf) {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("retime_result_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&d).unwrap();
    std::fs::write(d.join("src.u32"), plane_words(5)).unwrap();
    std::fs::write(d.join("src_k1.u32"), plane_words(9)).unwrap();
    std::fs::write(d.join("prior.u32"), plane_words(5)).unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_fx_run"))
        .current_dir(&d)
        .args(["-", "Retime_Longer", "--size", "50x7", "--out", "o", "--ctx-time", "0", "--ctx-word2", "0", "--ctx-1ae", "4", "--ctx-has-data", "7f8"])
        .args(["--src-planes", "4", "--owner-arm", "0,0,0", "--src-row58", "0", "--src-head", "0x7ea840", "--aspect-triple", "0,3f800000,3f800000"])
        .args(["--ae32-dest", "4,1", "--ae32-copier", "none", "--dest-stack-word", "0", "--units-ratio", "1,1", "--bound-mask", "1", "--ranges", "REAL"])
        .args(["--ae32-src", "Source=src.u32", "--ae32-prior", "prior.u32", "--ae32-out", "out.u32", "--do-postdiv", "0"])
        .args(["--geom", "-0.5,49.5,1.0,0.0,-22.0,0,0", "--host-frame", "0", "--host-value", "get_total_frames=240", "--tw-route", "240,0,1,1,0,0,1"])
        .args(["--frame-words", "0,0=src.u32", "--frame-words", "1,1=src_k1.u32", "--set", &format!("result={}", result)])
        .output()
        .unwrap();
    (o.status.code(), String::from_utf8_lossy(&o.stderr).to_string(), d)
}

#[test]
fn result_one_is_refused_by_name_with_no_output() {
    let (rc, err, d) = run("1", "one");
    assert_eq!(rc, Some(2), "{}", err);
    assert!(err.contains("RESULT-NOT-REBUILT"), "{}", err);
    assert!(!d.join("out.u32").exists());
    assert!(!d.join("o.dest.f32").exists());
}

#[test]
fn result_zero_still_renders() {
    let (rc, err, d) = run("0", "zero");
    assert_eq!(rc, Some(0), "{}", err);
    assert!(d.join("out.u32").exists());
}
