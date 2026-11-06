//! Two different stages run one after the other in one process give the bytes that separate processes give: the executor
//! keeps nothing from one run in the next.

use std::io::Write;
use std::process::{Command, Stdio};

use neutral_handoff::format::{Depth, FrameSpec};
use neutral_handoff::fxstage;

const W: u32 = 64;
const H: u32 = 32;
const FRAMES: usize = 3;

fn clip() -> Vec<u8> {
    let mut state = 20261106u64;
    let mut out = Vec::new();
    for _ in 0..FRAMES * 4 * (W * H) as usize {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let v = 0.05f32 + (state >> 40) as f32 / (1u64 << 24) as f32;
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn in_process(stage: &str, input: &[u8]) -> Vec<u8> {
    let spec = FrameSpec { width: W, height: H, depth: Depth::F32 };
    let rate = neutral_handoff::rate::Rate::parse("24").unwrap();
    let mut out = Vec::new();
    let mut src = input;
    fxstage::apply(stage, spec, Some(&rate), &[], &[], Some(FRAMES as u64), fx_exec::fxrun::Device::Cpu, &mut src, &mut out).unwrap();
    out
}

fn separate_process(stage: &str, input: &[u8]) -> Vec<u8> {
    let mut child = Command::new(env!("CARGO_BIN_EXE_sapphire"))
        .args(["stage", "apply", stage, "--width", &W.to_string(), "--height", &H.to_string(), "--depth", "32", "--fps", "24", "--frames", &FRAMES.to_string()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let data = input.to_vec();
    let writer = std::thread::spawn(move || {
        let _ = stdin.write_all(&data);
    });
    let out = child.wait_with_output().unwrap();
    writer.join().unwrap();
    assert!(out.status.success(), "{} failed", stage);
    out.stdout
}

#[test]
fn stages_in_sequence_equal_stages_in_separate_processes() {
    let input = clip();
    // a stage with a default-xy writer list, one without, the same two again in the other order
    let order = ["warp_perspective_autogen", "blur_color_autogen", "transform_and_blur_autogen", "diffuse_autogen", "drop_shadow", "warp_perspective_autogen"];
    for stage in order {
        let a = in_process(stage, &input);
        let b = separate_process(stage, &input);
        assert!(a == b, "{}: the in-process run after other stages differs from a fresh process", stage);
    }
}

fn refused(extra: &[&str]) -> (Option<i32>, String) {
    let mut args = vec!["stage", "apply", "hue_sat_bright_autogen", "--width", "16", "--height", "8", "--depth", "32", "--frames", "6"];
    args.extend_from_slice(extra);
    let mut child = Command::new(env!("CARGO_BIN_EXE_sapphire")).args(&args).stdin(Stdio::piped()).stdout(Stdio::null()).stderr(Stdio::piped()).spawn().unwrap();
    drop(child.stdin.take());
    let out = child.wait_with_output().unwrap();
    (out.status.code(), String::from_utf8_lossy(&out.stderr).into_owned())
}

#[test]
fn key_times_and_rates_beyond_2_53_exit_3() {
    let (code, err) = refused(&["--fps", "24", "--param", "saturation=0@-5e16;2@5e16"]);
    assert_eq!(code, Some(3), "{}", err);
    assert!(err.contains("out of range"), "{}", err);
    let (code, err) = refused(&["--fps", "1/9223372036854775813", "--param", "saturation=0@0;2@4"]);
    assert_eq!(code, Some(3), "{}", err);
    assert!(err.contains("out of range"), "{}", err);
}
