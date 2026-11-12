//! Refusals by name that no shipped stage reaches: a frame the 32 bit kernels cannot count (FRAME-SIZE-OVER-I32, at the kernel entry
//! and through `fx_run`), the Glow atmosphere (SIMPLEX-GRADIENT-TABLE-ABSENT, through `fx_run`) and a shake_shift node that the fold leaves in the plan
//! (SHAKE-SHIFT-UNFOLDED, through `fx_run`).
use fx_exec::kernels::run_any;
use fx_exec::*;
use std::path::PathBuf;
use std::process::Command;

fn cst(v: f32) -> Arg<'static> {
    Arg::Const { n: 1, lanes: [v.to_bits(); 4] }
}

const OVER: usize = i32::MAX as usize + 1;

fn std_params() -> KParams {
    let mut kp = KParams::standin();
    kp.blur_render_scale = 0;
    kp
}

/// The refusal text of a kernel that gets a plane length or width above i32::MAX. The checks run before any buffer is allocated.
fn refused(class: &str, tag: i32, args: &[Arg], n: usize, dims: (usize, usize)) -> String {
    match run_any(class, tag, args, n, dims, &std_params()) {
        Err(ExecError::Shape(s)) => s,
        other => panic!("{}: expected a shape refusal, got {:?}", class, other.map(|_| ())),
    }
}

#[test]
fn overlay_super_refuses_a_plane_length_over_i32_max() {
    let s = refused("overlay_super", 3, &[cst(0.5), cst(0.25)], OVER, (OVER, 1));
    assert!(s.starts_with("FRAME-SIZE-OVER-I32 (overlay_super: the plane length is 2147483648"), "{}", s);
    let s = refused("overlay_super_if_needed", 3, &[cst(0.5), cst(0.25)], OVER + 7, (OVER + 7, 1));
    assert!(s.contains("FRAME-SIZE-OVER-I32") && s.contains("2147483655"), "{}", s);
}

#[test]
fn screen_refuses_a_plane_length_over_i32_max() {
    let s = refused("screen", 3, &[cst(0.5), cst(0.25)], OVER, (OVER, 1));
    assert!(s.starts_with("FRAME-SIZE-OVER-I32 (screen: the plane length is 2147483648"), "{}", s);
}

#[test]
fn glow_refuses_a_frame_width_over_i32_max() {
    let args: Vec<Arg> = (0..8).map(|_| cst(1.0)).collect();
    let s = refused("glow", 3, &args, OVER, (OVER, 1));
    assert!(s.starts_with("FRAME-SIZE-OVER-I32 (glow: the frame width is 2147483648"), "{}", s);
}

#[test]
fn counts_that_fit_are_not_refused() {
    let a = Image::from_planes(vec![vec![0.25; 6]; 3]).unwrap();
    let b = Image::from_planes(vec![vec![0.5; 6]; 3]).unwrap();
    for c in ["overlay_super", "screen"] {
        let r = run_any(c, 3, &[Arg::Img(&a), Arg::Img(&b)], 6, (3, 2), &std_params());
        assert!(r.is_ok(), "{}: {:?}", c, r.map(|_| ()));
    }
}

const W: usize = 16;
const H: usize = 8;

/// `fx_run Glow` on the verified route with the default parameters, plus `add`. Returns the exit code, stderr and the ae32 output words.
fn glow(tag: &str, size: &str, add: &[&str]) -> (Option<i32>, String, Vec<u8>) {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("refusals_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&d).unwrap();
    let src = d.join("src.u32");
    let words: Vec<u8> = (0..4 * W * H).flat_map(|k| (((k * 7 % 31) as f32) / 31.0).to_le_bytes()).collect();
    std::fs::write(&src, words).unwrap();
    let prior = d.join("prior.u32");
    std::fs::write(&prior, vec![0xaau8; 16 * W * H]).unwrap();
    let os = d.join("o").to_str().unwrap().to_string();
    let geom = format!("-0.5,{},1,0,{},0,0", W as f32 - 0.5, (H as f32 - W as f32) / 2.0 - 0.5);
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
    cmd.args(["Glow", "--size", size, "--out", &os, "--ae32-src", &format!("Source={}", src.display())]);
    cmd.args(["--ae32-prior", prior.to_str().unwrap(), "--ae32-out", &format!("{}.ae32.u32", os), "--geom", &geom, "--host-frame", "0", "--ctx-time", "0"]);
    cmd.args(["--src-planes", "4", "--owner-arm", "0,0,0", "--src-row58", "0", "--src-head", "0x7eb3d0", "--aspect-triple", "0,3f800000,3f800000"]);
    cmd.args(["--ae32-dest", "4,1", "--ae32-copier", "none", "--dest-stack-word", "0", "--units-ratio", "1,1", "--bound-mask", "1", "--ctx-word2", "0"]);
    cmd.args(["--ctx-1ae", "4", "--ctx-has-data", "f8", "--ranges", "REAL", "--xy-base", "0"]);
    cmd.args(add);
    cmd.args(["--ns-default-unbound", "--do-postdiv", "1"]);
    let p = cmd.output().expect("fx_run");
    let out = std::fs::read(format!("{}.ae32.u32", os)).unwrap_or_default();
    (p.status.code(), String::from_utf8_lossy(&p.stderr).to_string(), out)
}

#[test]
fn glow_default_run_renders() {
    let (rc, e, out) = glow("base", &format!("{}x{}", W, H), &[]);
    assert_eq!(rc, Some(0), "{}", e);
    assert_eq!(out.len(), 16 * W * H);
}

#[test]
fn a_glow_atmosphere_is_refused_by_name_and_renders_nothing() {
    // a nonzero atmosphere amplitude (a 16.16 host value: 39322 / 65536) keeps simplex_noise on the executed route; get_time is given so the missing-value refusal is not the one that fires
    let (rc, e, out) = glow("atm", &format!("{}x{}", W, H), &["--set", "atmosphere_amp=0.600006103515625", "--host-value", "get_time=0x00000000"]);
    assert_eq!(rc, Some(2), "{}", e);
    assert!(e.contains("SIMPLEX-GRADIENT-TABLE-ABSENT"), "{}", e);
    assert!(out.is_empty());
    // an amplitude of 0 folds the noise away and renders
    let (rc, e, out) = glow("atm0", &format!("{}x{}", W, H), &["--set", "atmosphere_amp=0", "--host-value", "get_time=0x00000000"]);
    assert_eq!(rc, Some(0), "{}", e);
    assert_eq!(out.len(), 16 * W * H);
}

#[test]
fn a_frame_over_32_bits_is_refused_before_any_input_is_read() {
    // the source file holds 16 x 8 pixels, so a run that read it first would name the size mismatch instead
    for size in ["2147483648x1", "65536x32768", "46341x46341"] {
        let (rc, e, out) = glow(&format!("big{}", size), size, &[]);
        assert_eq!(rc, Some(2), "{}: {}", size, e);
        assert!(e.contains("FRAME-SIZE-OVER-I32") && e.contains("the plane length W x H is"), "{}: {}", size, e);
        assert!(out.is_empty());
    }
}

#[test]
fn a_shake_shift_node_with_a_non_constant_argument_is_refused_by_name() {
    // twelve image arguments cannot fold to a constant, so the node stays in the plan and the renderer has no run-time kernel for it
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("refusals_{}_shake_shift", std::process::id()));
    std::fs::create_dir_all(&d).unwrap();
    let tsv = d.join("programs.tsv");
    std::fs::write(&tsv, "def_effect\tT\t0x0\t0\t(def_effect T ( S ) ( ) ( ) ( shake_shift S S S S S S S S S S S S ))\n").unwrap();
    let os = d.join("o").to_str().unwrap().to_string();
    let p = Command::new(env!("CARGO_BIN_EXE_fx_run"))
        .arg(&tsv)
        .args(["T", "--size", "16x8", "--out", &os, "--ctx-time", "0", "--ctx-word2", "0", "--ctx-1ae", "4", "--ctx-has-data", "f8", "--ranges", "REAL", "--bound-mask", "1"])
        .output()
        .expect("fx_run");
    let e = String::from_utf8_lossy(&p.stderr).to_string();
    assert_eq!(p.status.code(), Some(2), "{}", e);
    assert!(e.contains("SHAKE-SHIFT-UNFOLDED"), "{}", e);
}
