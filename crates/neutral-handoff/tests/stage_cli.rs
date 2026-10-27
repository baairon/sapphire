//! The `stage` subcommand as a process: `sapphire stage list --json` and
//! `sapphire stage apply selftest-invert ...`, raw frames on stdin and stdout.
//!
//! The tests that compare with twixtor's `vfx` (the same stage in both tools, and the two tools in
//! one pipe in either order) skip with a printed reason when `vfx` is not there. The others need
//! nothing but the binary built with the crate.

mod common;

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use common::*;
use neutral_ffmpeg::json::Json;

#[derive(Clone, Copy, Debug)]
enum Alpha {
    Opaque,
    Partial,
    Zero,
}

/// Deterministic raw frames: `frames` of `w` x `h` at `depth` bits with the given alpha.
/// At depth 32 colour samples are multiples of 1/256, so that a double inversion is exact.
fn frames_of(w: u32, h: u32, depth: u32, frames: u32, alpha: Alpha, seed: u32) -> Vec<u8> {
    let n = (w * h) as usize;
    let mut state = seed.wrapping_mul(2654435761).wrapping_add(12345);
    let mut next = move || {
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;
        state
    };
    let mut out = Vec::new();
    for _ in 0..frames {
        match depth {
            8 => {
                for _ in 0..n {
                    let a = match alpha {
                        Alpha::Opaque => 255,
                        Alpha::Partial => (next() % 254 + 1) as u8,
                        Alpha::Zero => 0,
                    };
                    out.extend_from_slice(&[next() as u8, next() as u8, next() as u8, a]);
                }
            }
            16 => {
                for _ in 0..n {
                    let a: u16 = match alpha {
                        Alpha::Opaque => 65535,
                        Alpha::Partial => (next() % 65534 + 1) as u16,
                        Alpha::Zero => 0,
                    };
                    for v in [next() as u16, next() as u16, next() as u16, a] {
                        out.extend_from_slice(&v.to_le_bytes());
                    }
                }
            }
            _ => {
                // planes G, B, R, A
                for plane in 0..4 {
                    for _ in 0..n {
                        let v: f32 = if plane == 3 {
                            match alpha {
                                Alpha::Opaque => 1.0,
                                Alpha::Partial => (next() % 255 + 1) as f32 / 256.0,
                                Alpha::Zero => 0.0,
                            }
                        } else {
                            (next() % 257) as f32 / 256.0
                        };
                        out.extend_from_slice(&v.to_le_bytes());
                    }
                }
            }
        }
    }
    out
}

/// Arbitrary finite and infinite floats (never NaN) in the gbrapf32le layout, to compare the
/// arithmetic of the two inverters bit for bit.
fn wild_float_frames(w: u32, h: u32, frames: u32, seed: u32) -> Vec<u8> {
    let mut state = seed.wrapping_add(99);
    let mut out = Vec::new();
    for _ in 0..(w * h * 4 * frames) {
        state = state.wrapping_mul(1664525).wrapping_add(1013904223);
        let mut v = f32::from_bits(state);
        if v.is_nan() {
            v = 0.5;
        }
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

fn stage_args(w: u32, h: u32, depth: u32, extra: &[&str]) -> Vec<String> {
    let mut a = args(&["stage", "apply", "selftest-invert", "--width", &w.to_string(), "--height", &h.to_string(), "--depth", &depth.to_string(), "--fps", "24"]);
    a.extend(extra.iter().map(|s| s.to_string()));
    a
}

fn invert(data: Vec<u8>, w: u32, h: u32, depth: u32, extra: &[&str]) -> Ran {
    run_env(&handoff_exe(), &stage_args(w, h, depth, extra), &[], Some(data), Duration::from_secs(60))
}

fn vfx_invert(vfx: &Path, data: Vec<u8>, w: u32, h: u32, depth: u32, extra: &[&str]) -> Ran {
    let mut a = args(&["apply", "selftest-invert", "--width", &w.to_string(), "--height", &h.to_string(), "--depth", &depth.to_string(), "--fps", "24"]);
    a.extend(extra.iter().map(|s| s.to_string()));
    run_env(vfx, &a, &[], Some(data), Duration::from_secs(60))
}

#[test]
fn stage_list_reports_the_contract_and_what_exists() {
    let ran = run(&handoff_exe(), &args(&["stage", "list", "--json"]));
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    let doc = Json::parse(&ran.out_text()).expect("JSON");
    assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some("frames/2"));
    assert_eq!(doc.get("tool").and_then(|v| v.as_str()), Some("sapphire"));
    let depths: Vec<i64> = doc.get("depths").and_then(|v| v.as_array()).unwrap().iter().map(|d| d.as_i64().unwrap()).collect();
    assert_eq!(depths, [8, 16, 32]);
    let fmts = doc.get("pix_fmts").unwrap();
    assert_eq!(
        (fmts.get("8").and_then(|v| v.as_str()), fmts.get("16").and_then(|v| v.as_str()), fmts.get("32").and_then(|v| v.as_str())),
        (Some("rgba"), Some("rgba64le"), Some("gbrapf32le"))
    );
    let stages = doc.get("stages").and_then(|v| v.as_array()).unwrap();
    let ids: Vec<&str> = stages.iter().map(|s| s.get("id").and_then(|v| v.as_str()).unwrap()).collect();
    assert_eq!(ids, ["selftest-invert", "ffmpeg-filter"], "the vocabulary is exactly what exists");
    let invert = &stages[0];
    assert_eq!(invert.get("kind").and_then(|v| v.as_str()), Some("frame"));
    let params = invert.get("params").and_then(|v| v.as_array()).unwrap();
    assert_eq!(params[0].get("id").and_then(|v| v.as_str()), Some("invert_alpha"));
    assert_eq!(params[0].get("default").and_then(|v| v.as_bool()), Some(false));
}

#[test]
fn the_version_and_help_exit_zero() {
    let ran = run(&handoff_exe(), &args(&["--version"]));
    assert_eq!(ran.code, 0);
    let text = ran.out_text();
    assert!(text.starts_with("sapphire 0."), "{}", text);
    assert_eq!(text.trim().lines().count(), 1);
    let ran = run(&handoff_exe(), &args(&["-V"]));
    assert_eq!((ran.code, ran.out_text().trim().starts_with("sapphire ")), (0, true));
    let ran = run(&handoff_exe(), &args(&["--help"]));
    assert_eq!(ran.code, 0);
    assert!(ran.out_text().contains("--stage-timeout"));
}

#[test]
fn invert_twice_returns_the_input_for_every_depth_size_and_alpha() {
    let mut runs = 0;
    for depth in [8, 16, 32] {
        for (w, h) in [(1, 1), (2, 2), (33, 17), (160, 96)] {
            for alpha in [Alpha::Opaque, Alpha::Partial, Alpha::Zero] {
                let frames = if (w, h) == (160, 96) { 2 } else { 3 };
                let input = frames_of(w, h, depth, frames, alpha, w * 31 + h);
                let once = invert(input.clone(), w, h, depth, &[]);
                assert_eq!(once.code, 0, "{}x{} d{} {:?}: {}", w, h, depth, alpha, once.stderr);
                assert_eq!(once.stdout.len(), input.len());
                assert_ne!(once.stdout, input, "{}x{} d{} {:?}", w, h, depth, alpha);
                let twice = invert(once.stdout.clone(), w, h, depth, &[]);
                assert_eq!(twice.code, 0, "{}", twice.stderr);
                assert_eq!(twice.stdout, input, "{}x{} d{} {:?}", w, h, depth, alpha);
                runs += 2;
            }
        }
    }
    assert_eq!(runs, 72);
}

/// The colour samples and the alpha samples of raw frames, each in order.
fn split_alpha(data: &[u8], w: u32, h: u32, depth: u32) -> (Vec<u8>, Vec<u8>) {
    let n = (w * h) as usize;
    let (mut colour, mut alpha) = (Vec::new(), Vec::new());
    match depth {
        8 => data.chunks_exact(4).for_each(|p| {
            colour.extend_from_slice(&p[..3]);
            alpha.push(p[3]);
        }),
        16 => data.chunks_exact(8).for_each(|p| {
            colour.extend_from_slice(&p[..6]);
            alpha.extend_from_slice(&p[6..]);
        }),
        _ => data.chunks_exact(n * 16).for_each(|f| {
            colour.extend_from_slice(&f[..n * 12]);
            alpha.extend_from_slice(&f[n * 12..]);
        }),
    }
    (colour, alpha)
}

#[test]
fn invert_alpha_inverts_the_alpha_samples_too() {
    for depth in [8, 16, 32] {
        let input = frames_of(5, 3, depth, 2, Alpha::Partial, 7);
        let (colour, alpha) = split_alpha(&input, 5, 3, depth);
        let everything = invert(input.clone(), 5, 3, depth, &["--param", "invert_alpha=1"]);
        assert_eq!(everything.code, 0, "{}", everything.stderr);
        let (everything_colour, everything_alpha) = split_alpha(&everything.stdout, 5, 3, depth);
        assert_ne!(everything_alpha, alpha, "depth {}: alpha was inverted", depth);
        assert_ne!(everything_colour, colour);
        // colour back, alpha still inverted: the plain stage leaves alpha alone
        let back = invert(everything.stdout.clone(), 5, 3, depth, &[]);
        assert_eq!(back.code, 0, "{}", back.stderr);
        let (back_colour, back_alpha) = split_alpha(&back.stdout, 5, 3, depth);
        assert_eq!(back_colour, colour, "depth {}", depth);
        assert_eq!(back_alpha, everything_alpha, "depth {}: the plain stage does not touch alpha", depth);
        // and the plain stage alone leaves alpha as it was
        let plain = invert(input.clone(), 5, 3, depth, &[]);
        assert_eq!(split_alpha(&plain.stdout, 5, 3, depth).1, alpha, "depth {}", depth);
        // the checkbox may be written the ways vfx accepts
        for spelling in ["1", "true", "on"] {
            let same = invert(input.clone(), 5, 3, depth, &["--param", &format!("invert_alpha={}", spelling)]);
            assert_eq!(same.stdout, everything.stdout, "{}", spelling);
        }
        for spelling in ["0", "false", "off"] {
            let same = invert(input.clone(), 5, 3, depth, &["--param", &format!("invert_alpha={}", spelling)]);
            assert_eq!(same.stdout, plain.stdout, "{}", spelling);
        }
    }
}

#[test]
fn a_partial_frame_is_exit_4_with_a_frame_message() {
    let frame = 2 * 2 * 4;
    let mut data = frames_of(2, 2, 8, 2, Alpha::Opaque, 1);
    data.extend_from_slice(&[9u8; 7]);
    let ran = invert(data.clone(), 2, 2, 8, &[]);
    assert_eq!(ran.code, 4, "{}", ran.stderr);
    assert_eq!(ran.stdout.len(), 2 * frame, "the whole frames before the partial one are written");
    assert_eq!(ran.stderr.trim(), format!("error: partial_frame: frame 2: input ends with a partial frame (7 of {} bytes) after 2 whole frames", frame));
    let short = invert(vec![1, 2, 3], 2, 2, 8, &["--json"]);
    assert_eq!(short.code, 4);
    assert!(short.stdout.is_empty());
    let doc = Json::parse(short.stderr.trim()).expect("one JSON line on stderr");
    assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some("frames/2"));
    let e = doc.get("error").unwrap();
    assert_eq!(e.get("code").and_then(|v| v.as_str()), Some("partial_frame"));
    assert!(e.get("exit_code").map(|v| v.is_null()).unwrap_or(false), "frames/2: no child failed");
    assert!(e.get("message").and_then(|v| v.as_str()).unwrap().starts_with("frame 0: input ends with a partial frame (3 of 16 bytes)"));
}

#[test]
fn empty_input_is_zero_frames_and_exit_zero() {
    let ran = invert(Vec::new(), 4, 4, 16, &[]);
    assert_eq!((ran.code, ran.stdout.len(), ran.stderr.as_str()), (0, 0, ""));
}

#[test]
fn the_usage_and_unsupported_exit_codes_are_vfx_s() {
    let cases: Vec<(Vec<String>, i32, &str)> = vec![
        (args(&["stage", "apply", "selftest-invert", "--height", "2", "--depth", "8"]), 2, "error: usage: missing --width"),
        (args(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "2", "--depth", "12"]), 2, "error: usage: `--depth` expects 8, 16 or 32, got `12`"),
        (args(&["stage", "apply", "selftest-invert", "--width", "0", "--height", "2", "--depth", "8"]), 2, "error: usage: `--width` expects a positive integer, got `0`"),
        (args(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "2", "--depth", "8", "--bogus"]), 2, "error: usage: unknown option `--bogus`"),
        (args(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "2", "--depth", "8", "--layer", "a=b"]), 2, "error: usage: selftest-invert is a frame effect, --layer only apply to temporal effects"),
        (args(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "2", "--depth", "8", "--param", "invert_alpha=1", "--param", "invert_alpha=0"]), 2, "error: usage: param `invert_alpha` given more than once"),
        (args(&["stage", "apply", "nope", "--width", "2", "--height", "2", "--depth", "8"]), 3, "error: unsupported: unknown stage `nope`"),
        (args(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "2", "--depth", "8", "--param", "nope=1"]), 3, "error: unsupported: unknown param `nope`"),
        (args(&["stage", "apply", "selftest-invert", "--width", "2", "--height", "2", "--depth", "8", "--param", "invert_alpha=maybe"]), 3, "error: unsupported: bad value for `invert_alpha`: cannot parse `maybe`"),
    ];
    for (a, code, line) in cases {
        let ran = run_env(&handoff_exe(), &a, &[], Some(Vec::new()), Duration::from_secs(30));
        assert_eq!(ran.code, code, "{:?}: {}", a, ran.stderr);
        assert!(ran.stderr.trim().starts_with(line), "{:?}: {}", a, ran.stderr);
        assert_eq!(ran.stderr.trim().lines().count(), 1);
    }
}

#[test]
fn check_validates_without_reading() {
    let ran = run_env(&handoff_exe(), &stage_args(2, 2, 8, &["--check"]), &[], Some(vec![1, 2, 3]), Duration::from_secs(30));
    assert_eq!((ran.code, ran.stdout.len()), (0, 0));
}

#[test]
fn the_frame_count_is_checked_when_asked() {
    let input = frames_of(2, 2, 8, 3, Alpha::Opaque, 2);
    let ok = invert(input.clone(), 2, 2, 8, &["--frames", "3"]);
    assert_eq!(ok.code, 0, "{}", ok.stderr);
    let bad = invert(input, 2, 2, 8, &["--frames", "4"]);
    assert_eq!(bad.code, 4);
    assert_eq!(bad.stderr.trim(), "error: bad_input: the input holds 3 frames, but --frames says 4");
}

#[test]
fn fps_may_be_spelled_the_way_vfx_spells_it() {
    for fps in ["24", "29.97", "30000/1001", "24/1"] {
        let a = args(&["stage", "apply", "selftest-invert", "--width", "1", "--height", "1", "--depth", "8", "--fps", fps]);
        let ran = run_env(&handoff_exe(), &a, &[], Some(vec![1, 2, 3, 4]), Duration::from_secs(30));
        assert_eq!(ran.code, 0, "{}: {}", fps, ran.stderr);
    }
    for fps in ["0", "1/0", "abc", "-5"] {
        let a = args(&["stage", "apply", "selftest-invert", "--width", "1", "--height", "1", "--depth", "8", "--fps", fps]);
        let ran = run_env(&handoff_exe(), &a, &[], Some(Vec::new()), Duration::from_secs(30));
        assert_eq!(ran.code, 2, "{}", fps);
    }
}

#[test]
fn files_may_stand_in_for_the_pipes() {
    let dir = scratch("stage_files");
    let input = frames_of(3, 3, 16, 2, Alpha::Partial, 4);
    std::fs::write(dir.join("in.raw"), &input).unwrap();
    let a = args(&["stage", "apply", "selftest-invert", "--width", "3", "--height", "3", "--depth", "16", "--in", &dir.join("in.raw").to_string_lossy(), "--out", &dir.join("out.raw").to_string_lossy()]);
    let ran = run(&handoff_exe(), &a);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert!(ran.stdout.is_empty());
    let out = std::fs::read(dir.join("out.raw")).unwrap();
    assert_eq!(out.len(), input.len());
    let missing = run(&handoff_exe(), &args(&["stage", "apply", "selftest-invert", "--width", "3", "--height", "3", "--depth", "16", "--in", &dir.join("nope.raw").to_string_lossy()]));
    assert_eq!(missing.code, 4);
    assert!(missing.stderr.starts_with("error: bad_input: cannot open input"), "{}", missing.stderr);
    let same = run(
        &handoff_exe(),
        &args(&["stage", "apply", "selftest-invert", "--width", "3", "--height", "3", "--depth", "16", "--in", &dir.join("in.raw").to_string_lossy(), "--out", &dir.join("in.raw").to_string_lossy()]),
    );
    assert_eq!(same.code, 2);
    assert_eq!(std::fs::read(dir.join("in.raw")).unwrap(), input, "the input is never overwritten");
}

#[test]
fn a_reader_that_goes_away_ends_the_stage_with_broken_pipe_and_no_hang() {
    let mut child = Command::new(handoff_exe())
        .args(stage_args(160, 96, 8, &[]))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("start");
    let mut stdin = child.stdin.take().unwrap();
    let writer = std::thread::spawn(move || {
        use std::io::Write;
        let frame = vec![5u8; 160 * 96 * 4];
        for _ in 0..200 {
            if stdin.write_all(&frame).is_err() {
                break;
            }
        }
    });
    let mut stdout = child.stdout.take().unwrap();
    let mut first = [0u8; 100];
    stdout.read_exact(&mut first).unwrap();
    drop(stdout);
    let started = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait().unwrap() {
            break s;
        }
        assert!(started.elapsed() < Duration::from_secs(30), "the stage did not notice that its reader was gone");
        std::thread::sleep(Duration::from_millis(20));
    };
    let mut err = String::new();
    child.stderr.take().unwrap().read_to_string(&mut err).unwrap();
    let _ = writer.join();
    assert_eq!(status.code(), Some(4), "{}", err);
    assert!(err.starts_with("error: broken_pipe: frame "), "{}", err);
}

#[test]
fn our_stage_equals_vfx_selftest_invert_bit_for_bit() {
    let Some(vfx) = require_vfx("our_stage_equals_vfx_selftest_invert_bit_for_bit") else { return };
    for depth in [8u32, 16, 32] {
        for (w, h) in [(1u32, 1u32), (2, 2), (33, 17), (160, 96)] {
            for alpha in [Alpha::Opaque, Alpha::Partial, Alpha::Zero] {
                let input = if depth == 32 && matches!(alpha, Alpha::Partial) { wild_float_frames(w, h, 2, w + h) } else { frames_of(w, h, depth, 2, alpha, w * 7 + h) };
                for extra in [&[][..], &["--param", "invert_alpha=1"][..]] {
                    let theirs = vfx_invert(&vfx, input.clone(), w, h, depth, extra);
                    let ours = invert(input.clone(), w, h, depth, extra);
                    assert_eq!((theirs.code, ours.code), (0, 0), "{}x{} d{} {:?} {:?}: {} / {}", w, h, depth, alpha, extra, theirs.stderr, ours.stderr);
                    assert_eq!(ours.stdout.len(), theirs.stdout.len());
                    assert!(ours.stdout == theirs.stdout, "{}x{} d{} {:?} {:?}: outputs differ", w, h, depth, alpha, extra);
                }
            }
        }
    }
}

#[test]
fn vfx_then_ours_and_ours_then_vfx_return_the_input() {
    let Some(vfx) = require_vfx("vfx_then_ours_and_ours_then_vfx_return_the_input") else { return };
    for depth in [8u32, 16, 32] {
        for (w, h, frames) in [(1u32, 1u32, 1u32), (2, 2, 2), (33, 17, 3), (160, 96, 12)] {
            for alpha in [Alpha::Opaque, Alpha::Partial, Alpha::Zero] {
                let input = frames_of(w, h, depth, frames, alpha, w + h + frames);
                let via_vfx_first = {
                    let first = vfx_invert(&vfx, input.clone(), w, h, depth, &[]);
                    assert_eq!(first.code, 0, "{}", first.stderr);
                    invert(first.stdout, w, h, depth, &[])
                };
                assert_eq!(via_vfx_first.code, 0, "{}", via_vfx_first.stderr);
                assert!(via_vfx_first.stdout == input, "vfx then ours: {}x{}x{} d{} {:?}", w, h, frames, depth, alpha);
                let via_ours_first = {
                    let first = invert(input.clone(), w, h, depth, &[]);
                    assert_eq!(first.code, 0, "{}", first.stderr);
                    vfx_invert(&vfx, first.stdout, w, h, depth, &[])
                };
                assert_eq!(via_ours_first.code, 0, "{}", via_ours_first.stderr);
                assert!(via_ours_first.stdout == input, "ours then vfx: {}x{}x{} d{} {:?}", w, h, frames, depth, alpha);
            }
        }
    }
}
