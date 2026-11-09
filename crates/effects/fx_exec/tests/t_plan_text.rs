//! `fx_run --plan-text PATH`: writes the plan text and NODE rows for the plan the run executes, changes no output, refuses an unwritable path before any frame.
use std::path::PathBuf;
use std::process::Command;

const W: usize = 16;
const H: usize = 8;

/// host curve state of a default-bound run (host fields, unit shifts, point and curve lists), size lines filled in
const CURVES: &str = "host effect_ref 1592590336\n\
host time_mult 1\n\
host time_scale 24\n\
host double_flag 0\n\
host global 1\n\
host selector 0\n\
host size_x {W}\n\
host size_y {H}\n\
unit shift rec6c 3f800000\n\
unit shift x3 3f800000\n\
unit shift d4 3ff0000000000000\n\
unit shift d5 3ff0000000000000\n\
point center list none\n\
point center gid 1\n\
point center selector 0\n\
point center rtype 1\n\
point center size_x 64\n\
point center size_y 32\n\
point center s11c 1\n\
point center s120 1\n\
point center s124 1\n\
point center s128 1\n\
point center off_a 0\n\
point center off_b 0\n\
point center scaled 1\n\
point center cur_time 0\n\
point center time_step 1\n\
point center time_scale 24\n\
point center px 3276800\n\
point center py 3276800\n\
point center d4 3ff0000000000000\n\
point center d5 3ff0000000000000\n\
point center rec6c 3f800000\n\
point center x3 3f800000\n\
point center prelude_status 0\n\
point center prelude_type 6\n\
value rotate 23 0 1 24 0 0\n\
value shift 24 0 1 24 0 0\n\
value shift 25 0 1 24 0 0\n\
value z_dist 22 0 1 24 0 65536\n\
value wipe_amt 6 0 1 24 0 0\n\
";

fn floats(n: usize, seed: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for k in 0..n {
        let x = ((k * 7 + seed * 13) % 31) as f32 / 31.0;
        v.extend_from_slice(&x.to_le_bytes());
    }
    v
}

fn run(test: &str, tag: &str, plan: Option<&str>) -> (Option<i32>, String, Vec<u8>, Vec<u8>) {
    run_with(test, tag, plan, None, &[])
}

/// `run` with the host value named `omit` left out and `add` appended to the command line. Every test passes its own name, so no
/// two tests of one process share a folder.
fn run_with(test: &str, tag: &str, plan: Option<&str>, omit: Option<&str>, add: &[&str]) -> (Option<i32>, String, Vec<u8>, Vec<u8>) {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("plantext_{}_{}_{}", std::process::id(), test, tag));
    std::fs::create_dir_all(&d).unwrap();
    let from = d.join("from.bin");
    let to = d.join("to.bin");
    std::fs::write(&from, floats(4 * W * H, 1)).unwrap();
    std::fs::write(&to, floats(4 * W * H, 2)).unwrap();
    let prior = d.join("prior.u32");
    std::fs::write(&prior, vec![0xaau8; 16 * W * H]).unwrap();
    let os = d.join("o").to_str().unwrap().to_string();
    let geom = format!("-0.5,{},1,0,{},0,0", W as f32 - 0.5, (H as f32 - W as f32) / 2.0 - 0.5);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
    cmd.args(["SwishPan", "--size", &format!("{}x{}", W, H), "--out", &os]);
    cmd.args(["--ae32-src", &format!("From={}", from.display()), "--ae32-src", &format!("To={}", to.display())]);
    cmd.args(["--ae32-prior", prior.to_str().unwrap(), "--ae32-out", &format!("{}.ae32.u32", os), "--geom", &geom, "--host-frame", "0", "--ctx-time", "0"]);
    cmd.args(["--src-planes", "4", "--owner-arm", "0,0,0", "--src-row58", "0", "--src-head", "0x7eb3d0", "--aspect-triple", "0,3f800000,3f800000"]);
    cmd.args(["--ae32-dest", "4,1", "--ae32-copier", "none", "--dest-stack-word", "0", "--units-ratio", "1,1", "--bound-mask", "3", "--ctx-word2", "0"]);
    cmd.args(["--ctx-1ae", "4", "--ctx-has-data", "f8", "--ranges", "REAL", "--xy-base", "0"]);
    cmd.args(["--set", "transition_dir=0", "--set", "auto_trans=1", "--set", "direction=0"]);
    let cv = d.join("curves.txt");
    std::fs::write(&cv, CURVES.replace("{W}", &W.to_string()).replace("{H}", &H.to_string())).unwrap();
    cmd.args(["--host-param", "blur_amount=60.0", "--set", "overlap=0.0", "--set", "slow_in=0.5", "--set", "slow_out=0.5", "--set", "opacity=1", "--set", "swish_auto_trans=0.0"]);
    for (n, v) in [("get_time", "0x00000000"), ("get_percent_time", "0x00000000"), ("get_total_frames", "0x43700000"), ("get_aspect_ratio", "0x3f000000")] {
        if omit != Some(n) {
            cmd.args(["--host-value", &format!("{}={}", n, v)]);
        }
    }
    cmd.args(add);
    cmd.args(["--ns-default-unbound", "--host-curve-values", cv.to_str().unwrap(), "--do-postdiv", "1"]);
    if let Some(p) = plan {
        cmd.args(["--plan-text", p]);
    }
    let p = cmd.output().expect("fx_run");
    let words = std::fs::read(format!("{}.ae32.u32", os)).unwrap_or_default();
    let dest = std::fs::read(format!("{}.dest.f32", os)).unwrap_or_default();
    (p.status.code(), String::from_utf8_lossy(&p.stderr).to_string(), words, dest)
}

#[test]
fn plan_text_is_written_and_changes_no_output() {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("plantext_file_{}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    let pf = d.join("plan.txt");
    let (rc0, e0, w0, d0) = run("written", "a", None);
    assert_eq!(rc0, Some(0), "{}", e0);
    assert!(!pf.exists());
    let (rc1, e1, w1, d1) = run("written", "b", Some(pf.to_str().unwrap()));
    assert_eq!(rc1, Some(0), "{}", e1);
    assert!(!w0.is_empty() && !d0.is_empty());
    assert_eq!(w0, w1);
    assert_eq!(d0, d1);
    let t = std::fs::read_to_string(&pf).unwrap();
    assert!(t.lines().any(|l| l.starts_with("CMD") && l.contains("allocate-images")), "{}", t);
    assert!(t.lines().any(|l| l.starts_with("CMD") && l.contains("run-primitive") && l.contains("make_mipmap")), "{}", t);
    assert!(t.lines().any(|l| l.starts_with("NODE\tN0\t") && l.contains("lanes=")), "{}", t);
}

#[test]
fn plan_text_unwritable_path_is_refused_by_name() {
    let (rc, e, w, _) = run("unwritable", "c", Some("/nonexistent_dir_plan_text/plan.txt"));
    assert_eq!(rc, Some(2), "{}", e);
    assert!(e.contains("--plan-text: refused") && e.contains("/nonexistent_dir_plan_text/plan.txt"), "{}", e);
    assert!(w.is_empty());
}

const GETTERS: [&str; 4] = ["get_time", "get_percent_time", "get_total_frames", "get_aspect_ratio"];

/// The getters whose absence this program refuses (the others are not read on its executed route). `who` names the calling test, so
/// that tests running at the same time use folders of their own.
fn refused_getters(who: &str) -> Vec<&'static str> {
    GETTERS
        .iter()
        .copied()
        .filter(|g| {
            let (rc, e, w, d) = run_with(who, &format!("miss_{}", g), None, Some(g), &[]);
            if rc == Some(2) {
                assert!(e.contains("HOST-VALUE-MISSING") && e.contains(g), "{}: {}", g, e);
                assert!(w.is_empty() && d.is_empty());
                true
            } else {
                assert_eq!(rc, Some(0), "{}: {}", g, e);
                false
            }
        })
        .collect()
}

#[test]
fn a_missing_host_getter_value_is_refused_by_name() {
    assert!(!refused_getters("names").is_empty(), "no getter of the program is refused when its value is missing");
}

#[test]
fn plan_text_is_not_written_or_truncated_when_the_run_is_refused() {
    let g = refused_getters("keeps")[0];
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("plantext_keep_{}", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    let pf = d.join("plan.txt");
    std::fs::write(&pf, "previous plan\n").unwrap();
    let (rc, e, _, _) = run_with("keeps", "d", Some(pf.to_str().unwrap()), Some(g), &[]);
    assert_eq!(rc, Some(2), "{}", e);
    assert_eq!(std::fs::read_to_string(&pf).unwrap(), "previous plan\n");
    let fresh = d.join("fresh.txt");
    let (rc, _, _, _) = run_with("keeps", "e", Some(fresh.to_str().unwrap()), Some(g), &[]);
    assert_eq!(rc, Some(2));
    assert!(!fresh.exists());
    let (rc, e, _, _) = run_with("keeps", "f", Some(pf.to_str().unwrap()), None, &[]);
    assert_eq!(rc, Some(0), "{}", e);
    assert!(std::fs::read_to_string(&pf).unwrap().contains("NODE\tN0\t"));
    let leftovers: Vec<_> = std::fs::read_dir(&d).unwrap().map(|x| x.unwrap().file_name().to_string_lossy().into_owned()).filter(|n| n.contains(".tmp") || n.contains(".probe")).collect();
    assert!(leftovers.is_empty(), "{:?}", leftovers);
}

#[test]
fn a_repeated_host_value_is_refused_by_name() {
    let (rc, e, w, _) = run_with("repeated", "i", None, None, &["--host-value", "get_time=0x3f800000"]);
    assert_eq!(rc, Some(2), "{}", e);
    assert!(e.contains("--host-value: refused") && e.contains("get_time is given twice"), "{}", e);
    assert!(w.is_empty());
}

#[test]
fn a_non_finite_host_value_is_refused_by_name() {
    for v in ["nan", "NaN", "inf", "-inf", "infinity", "1e40", "0x7fc00000", "0xffc00000", "0x7f800000", "0xff800000"] {
        let (rc, e, w, d) = run_with("nonfinite", "nf", None, Some("get_time"), &["--host-value", &format!("get_time={}", v)]);
        assert_eq!(rc, Some(2), "{}: {}", v, e);
        assert!(e.contains("--host-value: refused") && e.contains("get_time is not finite"), "{}: {}", v, e);
        assert!(w.is_empty() && d.is_empty(), "{}", v);
    }
    // a finite value, also the largest f32 and a subnormal, is accepted
    for v in ["0x3f800000", "3.4028235e38", "1e-45", "-0.0", "0x00000001"] {
        let (rc, e, _, _) = run_with("nonfinite", "fin", None, Some("get_time"), &["--host-value", &format!("get_time={}", v)]);
        assert_eq!(rc, Some(0), "{}: {}", v, e);
    }
}
