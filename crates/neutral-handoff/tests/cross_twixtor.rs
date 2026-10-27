//! Either tool as a stage of the other, in one pipe: twixtor's `vfx` run by this pipeline as an
//! ordinary `--stage` command, and this crate's `stage apply` run by twixtor's `tools/apply.py` as a
//! `--pre-stage` and `--post-stage`. Inverting twice, through every order of the two inverters,
//! must give back the input frames (compared as decoded raw frames of lossless outputs).
//!
//! Skips with a printed reason when ffmpeg, `vfx`, Python or `apply.py` is missing.

mod common;

use std::path::{Path, PathBuf};
use std::time::Duration;

use common::*;
use neutral_ffmpeg::json::Json;

fn quoted(path: &Path) -> String {
    format!("\"{}\"", path.to_string_lossy().replace('\\', "/"))
}

fn vfx_stage(vfx: &Path) -> String {
    format!("{} apply selftest-invert --width {{width}} --height {{height}} --depth {{depth}} --fps {{fps}}", quoted(vfx))
}

fn our_stage() -> String {
    format!("{} stage apply selftest-invert --width {{width}} --height {{height}} --depth {{depth}} --fps {{fps}}", quoted(&handoff_exe()))
}

/// The colour samples of rgba or rgba64le frames inverted, alpha left alone.
fn invert_colour(data: &[u8], depth: u32) -> Vec<u8> {
    let mut out = data.to_vec();
    if depth == 8 {
        for px in out.chunks_exact_mut(4) {
            for c in &mut px[..3] {
                *c = 255 - *c;
            }
        }
    } else {
        for px in out.chunks_exact_mut(8) {
            for c in px[..6].chunks_exact_mut(2) {
                let v = u16::from_le_bytes([c[0], c[1]]);
                c.copy_from_slice(&(65535 - v).to_le_bytes());
            }
        }
    }
    out
}

fn expected(clip: &Path, depth: u32) -> (Vec<u8>, &'static str) {
    let fmt = if depth == 8 { "rgba" } else { "rgba64le" };
    (decode_with_matrix(clip, "smpte170m", fmt), fmt)
}

#[test]
fn this_pipeline_runs_vfx_as_a_plain_stage_in_either_order() {
    let name = "this_pipeline_runs_vfx_as_a_plain_stage_in_either_order";
    if require_media(name).is_none() {
        return;
    }
    let Some(vfx) = require_vfx(name) else { return };
    let dir = scratch("cross_our_wrapper");
    let clip = make_clip(&dir, "clip.mp4", 160, 96, 12, 24, 0.6);
    for depth in [8u32, 16] {
        let (want, fmt) = expected(&clip, depth);
        let orders: [(&str, Vec<String>); 4] = [
            ("vfx then builtin", vec![vfx_stage(&vfx), "@selftest-invert".into()]),
            ("builtin then vfx", vec!["@selftest-invert".into(), vfx_stage(&vfx)]),
            ("vfx then our process", vec![vfx_stage(&vfx), our_stage()]),
            ("our process then vfx", vec![our_stage(), vfx_stage(&vfx)]),
        ];
        for (label, stages) in orders {
            let out = dir.join(format!("{}_{}.mkv", label.replace(' ', "_"), depth));
            let mut a = vec![clip.to_string_lossy().into_owned()];
            for s in &stages {
                a.push("--stage".into());
                a.push(s.clone());
            }
            a.extend(args(&["--depth", &depth.to_string(), "--lossless", "-o", &out.to_string_lossy(), "--json"]));
            let ran = run(&handoff_exe(), &a);
            assert_eq!(ran.code, 0, "{} d{}: {}", label, depth, ran.stderr);
            let doc = Json::parse(&ran.out_text()).unwrap();
            assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some("frames/2"));
            assert_eq!(doc.get("stage_reports").and_then(|v| v.as_array()).map(|a| a.len()), Some(2));
            assert!(decode_raw(&out, fmt) == want, "{} at depth {}: not the input", label, depth);
        }
    }
}

#[test]
fn the_effect_marker_puts_vfx_before_or_after_our_stage() {
    let name = "the_effect_marker_puts_vfx_before_or_after_our_stage";
    if require_media(name).is_none() {
        return;
    }
    let Some(vfx) = require_vfx(name) else { return };
    let dir = scratch("cross_marker");
    let clip = make_clip(&dir, "clip.mp4", 160, 96, 12, 24, 0.0);
    for depth in [8u32, 16] {
        let (want, fmt) = expected(&clip, depth);
        for (n, markers) in [vec!["vfx".to_string(), our_stage()], vec![our_stage(), "vfx".to_string()]].into_iter().enumerate() {
            let out = dir.join(format!("m_{}_{}.mkv", n, depth));
            let mut a = vec![clip.to_string_lossy().into_owned(), "selftest-invert".into(), "--vfx".into(), vfx.to_string_lossy().into_owned()];
            for s in &markers {
                a.push("--stage".into());
                a.push(s.clone());
            }
            a.extend(args(&["--depth", &depth.to_string(), "--lossless", "-o", &out.to_string_lossy()]));
            let ran = run(&handoff_exe(), &a);
            assert_eq!(ran.code, 0, "{}", ran.stderr);
            assert!(decode_raw(&out, fmt) == want, "depth {}", depth);
        }
    }
}

#[test]
fn twixtor_s_wrapper_runs_our_stage_before_and_after_vfx() {
    let name = "twixtor_s_wrapper_runs_our_stage_before_and_after_vfx";
    let Some((ffmpeg, _)) = require_media(name) else { return };
    let Some(vfx) = require_vfx(name) else { return };
    let Some((python, apply_py)) = require_apply_py(name) else { return };
    // an apply.py from before the stage slot has no --pre-stage: say so and skip
    let help = run_env(Path::new(&python), &[apply_py.to_string_lossy().into_owned(), "--help".into()], &[], None, Duration::from_secs(60));
    if !help.out_text().contains("--pre-stage") {
        eprintln!("SKIPPED {}: this tools/apply.py has no --pre-stage", name);
        return;
    }
    let dir = scratch("cross_apply_py");
    let clip = make_clip(&dir, "clip.mp4", 160, 96, 12, 24, 0.6);
    let run_apply = |extra: &[String], out: &PathBuf, depth: u32| -> Ran {
        let mut a = vec![apply_py.to_string_lossy().into_owned(), clip.to_string_lossy().into_owned(), "selftest-invert".into()];
        a.extend(extra.iter().cloned());
        a.extend(args(&["--depth", &depth.to_string(), "--lossless", "-o", &out.to_string_lossy(), "--force", "--json"]));
        a.extend(args(&["--vfx", &vfx.to_string_lossy(), "--ffmpeg-path", &ffmpeg.to_string_lossy()]));
        run_env(Path::new(&python), &a, &[], None, Duration::from_secs(300))
    };
    for depth in [8u32, 16] {
        let (want, fmt) = expected(&clip, depth);
        let cases: [(&str, Vec<String>); 3] = [
            ("pre", vec!["--pre-stage".into(), our_stage()]),
            ("post", vec!["--post-stage".into(), our_stage()]),
            ("pre and post", vec!["--pre-stage".into(), our_stage(), "--post-stage".into(), our_stage()]),
        ];
        for (label, extra) in cases {
            let out = dir.join(format!("{}_{}.mkv", label.replace(' ', "_"), depth));
            let ran = run_apply(&extra, &out, depth);
            assert_eq!(ran.code, 0, "{} d{}: {}", label, depth, ran.stderr);
            let got = decode_raw(&out, fmt);
            if label == "pre and post" {
                // three inversions (our stage, vfx, our stage): the colour of the input inverted
                assert!(got == invert_colour(&want, depth), "{} at depth {}", label, depth);
            } else {
                // two inversions (one of our stage, one of vfx): the input again
                assert!(got == want, "{} at depth {}", label, depth);
            }
        }
    }
}

#[test]
fn twixtor_s_wrapper_reports_a_failure_of_our_stage_with_its_exit_code() {
    let name = "twixtor_s_wrapper_reports_a_failure_of_our_stage_with_its_exit_code";
    let Some((ffmpeg, _)) = require_media(name) else { return };
    let Some(vfx) = require_vfx(name) else { return };
    let Some((python, apply_py)) = require_apply_py(name) else { return };
    let help = run_env(Path::new(&python), &[apply_py.to_string_lossy().into_owned(), "--help".into()], &[], None, Duration::from_secs(60));
    if !help.out_text().contains("--pre-stage") {
        eprintln!("SKIPPED {}: this tools/apply.py has no --pre-stage", name);
        return;
    }
    let dir = scratch("cross_apply_py_failure");
    let clip = make_clip(&dir, "clip.mp4", 160, 96, 12, 24, 0.0);
    let out = dir.join("never.mkv");
    // our stage is told to expect 99 frames: it exits 4 after writing the 12 it was given
    let wrong_count = format!("{} --frames 99", our_stage());
    let mut a = vec![apply_py.to_string_lossy().into_owned(), clip.to_string_lossy().into_owned(), "selftest-invert".into(), "--pre-stage".into(), wrong_count];
    a.extend(args(&["--depth", "8", "--lossless", "-o", &out.to_string_lossy(), "--json", "--vfx", &vfx.to_string_lossy(), "--ffmpeg-path", &ffmpeg.to_string_lossy()]));
    let ran = run_env(Path::new(&python), &a, &[], None, Duration::from_secs(300));
    assert_ne!(ran.code, 0);
    assert!(!out.exists());
    let doc = Json::parse(ran.out_text().trim()).unwrap_or_else(|e| panic!("{}: {}", e, ran.out_text()));
    assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some("frames/1"));
    let error = doc.get("error").expect("error object");
    assert_eq!(error.get("stage").and_then(|v| v.as_str()), Some("pre1"));
    assert_eq!(error.get("exit_code").and_then(|v| v.as_i64()), Some(4));
    let tail: Vec<String> = error.get("stderr_tail").and_then(|v| v.as_array()).unwrap().iter().map(|l| l.as_str().unwrap().to_string()).collect();
    assert!(tail.iter().any(|l| l.starts_with("error: bad_input: the input holds 12 frames, but --frames says 99")), "{:?}", tail);
}
