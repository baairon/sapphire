//! S_WarpTransform (Warp_Transform_Autogen) at 64x32: the negative Scale fold and the collapsed warp at the Opacity heads.
//! (1) scale_safe = if+ (negate scale) (min -0.001 scale) (max 0.001 scale) folds lanewise when the condition lanes disagree in sign,
//!     so tilt receives the constant z_dist / scale_safe. (2) A collapsed warp leaves make_mipmap as the root at Opacity 1 and 3; the
//!     root takes the plain format and renders as the base image.
use std::path::PathBuf;
use std::process::Command;

const W: usize = 64;
const H: usize = 32;

fn run(tag: &str, variant: &[&str], pd_xy: (&str, &str), sets: &[&str]) -> (Option<i32>, String, String) {
    let d = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("wtfold_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&d).unwrap();
    let n = W * H;
    let mut src = Vec::new();
    for k in 0..n * 4 {
        let x = ((k * 7 + 3) % 31) as f32 / 31.0;
        src.extend_from_slice(&x.to_le_bytes());
    }
    let sp = d.join("src.u32");
    std::fs::write(&sp, src).unwrap();
    let pp = d.join("prior.u32");
    std::fs::write(&pp, vec![0u8; n * 16]).unwrap();
    let os = d.join("o").to_str().unwrap().to_string();
    let plan = d.join("plan.txt");
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_fx_run"));
    cmd.args(["-", "Warp_Transform_Autogen", "--size", &format!("{}x{}", W, H), "--out", &os, "--ctx-time", "0", "--ctx-word2", "0", "--ctx-has-data", "7f8"]);
    cmd.args(["--owner-arm", "0,0,0", "--src-row58", "0", "--aspect-triple", "0,3f800000,3f800000", "--ae32-copier", "none", "--dest-stack-word", "0"]);
    cmd.args(["--units-ratio", "1,1", "--bound-mask", "1", "--ranges", "REAL", "--do-postdiv", pd_xy.0]);
    if !pd_xy.1.is_empty() {
        cmd.args(["--xy-base-list", pd_xy.1]);
    }
    cmd.args(["--ae32-src", &format!("Source={}", sp.display()), "--ae32-prior", pp.to_str().unwrap(), "--ae32-out", &format!("{}.out.u32", os)]);
    cmd.args(["--geom", "-0.5,63.5,1.0,0.0,-16.5,0,0", "--host-frame", "0", "--host-param", "blur_mask=0", "--host-param", "blur_mocha_mask=0"]);
    cmd.args(["--host-param", "shift_mocha_mask=0,0", "--set", "invert_mask=0", "--set", "mask_use=0"]);
    // the variant options replace the defaults of the 4-plane arrangement
    if variant.is_empty() {
        cmd.args(["--ctx-1ae", "4", "--src-planes", "4", "--src-head", "0x7eb3d0", "--ae32-dest", "4,1"]);
    } else {
        cmd.args(variant);
    }
    for s in sets {
        cmd.args(["--set", s]);
    }
    cmd.args(["--plan-text", plan.to_str().unwrap()]);
    let p = cmd.output().expect("fx_run");
    let plan_text = std::fs::read_to_string(&plan).unwrap_or_default();
    (p.status.code(), String::from_utf8_lossy(&p.stderr).to_string(), plan_text)
}

fn tilt_line(plan: &str) -> String {
    plan.lines().find(|l| l.contains("run-primitive") && l.contains(" tilt ")).unwrap_or("").to_string()
}

#[test]
fn negative_scale_x_folds_the_scale_safe_constant() {
    for (tag, sc, want) in [("n15", "scale=-1.5,1.0", "-0.666667 1"), ("n1", "scale=-1.0,1.0", "-1 1"), ("n05", "scale=-0.5,1.0", "-2 1")] {
        let (rc, err, plan) = run(tag, &[], ("1", "0"), &[sc, "shift=0.5625,-1.0625"]);
        assert_eq!(rc, Some(0), "{} {}", tag, err);
        let t = tilt_line(&plan);
        assert!(t.contains(want), "{}: {}", tag, t);
    }
}

#[test]
fn negative_scale_y_folds_too() {
    let (rc, err, plan) = run("y15", &[], ("1", "0"), &["scale=1.0,-1.5", "shift=0.5625,-1.0625"]);
    assert_eq!(rc, Some(0), "{}", err);
    assert!(tilt_line(&plan).contains("1 -0.666667"), "{}", tilt_line(&plan));
}

#[test]
fn positive_scale_is_unchanged() {
    let (rc, _e, plan) = run("p15", &[], ("1", "0"), &["scale=1.5,1.0", "shift=0.5625,-1.0625"]);
    assert_eq!(rc, Some(0));
    assert!(tilt_line(&plan).contains("0.666667 1"), "{}", tilt_line(&plan));
}

const OP1: [&str; 14] = ["--ctx-1ae", "3", "--src-planes", "3", "--src-head", "0x7e91c0", "--ae32-dest", "3,3", "--class-cells", "01,00,01", "--class3-record", "0,1,3,04", "--cset", "Opacity=1"];
const OP3: [&str; 10] = ["--ctx-1ae", "4", "--src-planes", "4", "--src-head", "0x7ea840", "--ae32-dest", "4,1", "--cset", "Opacity=3"];

#[test]
fn collapsed_warp_renders_at_opacity_1() {
    // Shift XY at the centre: tilt and warp collapse, make_mipmap is the root
    let (rc, err, plan) = run("c1", &OP1, ("0", ""), &["shift=0,0"]);
    assert_eq!(rc, Some(0), "{}", err);
    assert!(plan.contains(" make_mipmap ") && !plan.contains(" tilt ") && !plan.contains(" warp "), "{}", plan);
}

#[test]
fn collapsed_warp_renders_at_opacity_3() {
    let (rc, err, plan) = run("c3", &OP3, ("0", ""), &["shift=0,0"]);
    assert_eq!(rc, Some(0), "{}", err);
    assert!(plan.contains(" make_mipmap ") && !plan.contains(" tilt ") && !plan.contains(" warp "), "{}", plan);
}
