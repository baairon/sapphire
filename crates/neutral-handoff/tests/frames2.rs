//! frames/2 (`docs/handoff.md`): the stage manifest, `--describe`, the `ffmpeg-filter` stage,
//! geometry chained through the wrapper, `role` and `index` in reports, and the exits of both
//! contracts. Every process runs under a deadline; a run past it is killed with its tree.

mod common;

use std::path::{Path, PathBuf};
use std::time::Duration;

use common::*;
use neutral_ffmpeg::json::Json;

const DEADLINE: Duration = Duration::from_secs(120);

fn nh(list: &[&str], input: Option<Vec<u8>>) -> Ran {
    run_env(&handoff_exe(), &args(list), &[], input, DEADLINE)
}

fn ffmpeg_or_skip(test: &str) -> Option<PathBuf> {
    require_media(test).map(|(ffmpeg, _)| ffmpeg)
}

fn ff_apply(ffmpeg: &Path, w: u32, h: u32, depth: u32, params: &[&str], extra: &[&str], input: Vec<u8>) -> Ran {
    let (w, h, d) = (w.to_string(), h.to_string(), depth.to_string());
    let ff = ffmpeg.to_string_lossy().into_owned();
    let mut a = vec!["stage", "apply", "ffmpeg-filter", "--width", &w, "--height", &h, "--depth", &d, "--fps", "24/1", "--ffmpeg-path", &ff];
    for p in params {
        a.extend(["--param", p]);
    }
    a.extend(extra);
    nh(&a, Some(input))
}

/// The error object is the last line: an ffmpeg other than the measured version adds a
/// warning line before it on standard error.
fn error_of(text: &str) -> (String, Option<String>) {
    let line = text.trim().lines().last().unwrap_or("");
    let doc = Json::parse(line).unwrap_or_else(|e| panic!("{}: {}", e, text));
    let e = doc.get("error").expect("error object");
    (e.get("code").and_then(|v| v.as_str()).unwrap().to_string(), e.get("stage").and_then(|v| v.as_str()).map(|s| s.to_string()))
}

#[test]
fn the_stage_manifest_is_frames_2_with_provider_spool_and_geometry() {
    let ran = nh(&["stage", "list", "--json"], None);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    let doc = Json::parse(&ran.out_text()).expect("JSON");
    assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some("frames/2"));
    let provider = doc.get("provider").expect("provider");
    assert_eq!(provider.get("id").and_then(|v| v.as_str()), Some("sapphire"));
    assert!(provider.get("version").and_then(|v| v.as_str()).unwrap().starts_with("sapphire "));
    let command = provider.get("command").and_then(|v| v.as_array()).unwrap();
    assert_eq!((command.len(), command[1].as_str()), (2, Some("stage")));
    for stage in doc.get("stages").and_then(|v| v.as_array()).unwrap() {
        for key in ["id", "kind", "spool", "geometry", "depths", "params", "layers"] {
            assert!(stage.get(key).is_some(), "{} lacks {}", ran.out_text(), key);
        }
        assert_eq!(stage.get("spool").and_then(|v| v.as_bool()), Some(false));
    }
    let old = nh(&["stage", "list", "--json", "--contract", "frames/1"], None);
    assert_eq!(Json::parse(&old.out_text()).unwrap().get("contract").and_then(|v| v.as_str()), Some("frames/1"));
}

#[test]
fn describe_reads_no_input_and_is_frames_2_only() {
    let ran = nh(&["stage", "apply", "selftest-invert", "--width", "4", "--height", "2", "--depth", "8", "--fps", "24", "--frames", "3", "--describe"], None);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert_eq!(
        ran.out_text().trim(),
        r#"{"ok": true, "contract": "frames/2", "stage": "selftest-invert", "width": 4, "height": 2, "depth": 8, "fps": "24/1", "frames": 3}"#
    );
    let old = nh(&["stage", "apply", "selftest-invert", "--width", "4", "--height", "2", "--depth", "8", "--describe", "--contract", "frames/1"], None);
    assert_eq!(old.code, 2, "a usage error of the command line keeps exit 2 under frames/1: {}", old.stderr);
}

#[test]
fn ffmpeg_filter_declares_its_geometry_with_a_bounded_dry_run() {
    let Some(ffmpeg) = ffmpeg_or_skip("ffmpeg_filter_declares_its_geometry_with_a_bounded_dry_run") else { return };
    let ff = ffmpeg.to_string_lossy().into_owned();
    let base = ["stage", "apply", "ffmpeg-filter", "--width", "80", "--height", "48", "--depth", "8", "--fps", "30000/1001", "--ffmpeg-path", &ff, "--describe"];
    let mut a = base.to_vec();
    a.extend(["--frames", "12", "--param", "graph=scale=40:24", "--param", "expect=declared"]);
    let ran = nh(&a, None);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert_eq!(
        ran.out_text().trim(),
        r#"{"ok": true, "contract": "frames/2", "stage": "ffmpeg-filter", "width": 40, "height": 24, "depth": 8, "fps": "30000/1001", "frames": 12}"#
    );
    // refusals keep the stage exits: 3 unsupported
    for (graph, expect, fragment) in [
        ("fps=60", "declared", "changes the frame rate"),
        ("scale=40:24", "preserve", "expect=declared"),
        ("movie=clip.mp4", "declared", "unsafe=1"),
        ("lut3d=a.cube", "declared", "unsafe=1"),
        ("nosuchfilter", "preserve", "has no filter `nosuchfilter`"),
    ] {
        let mut a = base.to_vec();
        let g = format!("graph={}", graph);
        let e = format!("expect={}", expect);
        a.extend(["--param", &g, "--param", &e, "--json"]);
        let ran = nh(&a, None);
        assert_eq!(ran.code, 3, "{}: {}", graph, ran.stderr);
        let (code, _) = error_of(&ran.stderr);
        assert_eq!(code, "unsupported", "{}", graph);
        assert!(ran.stderr.contains(fragment), "{}: {}", graph, ran.stderr);
    }
}

#[test]
fn ffmpeg_filter_under_preserve_gives_back_the_bytes_it_was_given() {
    let Some(ffmpeg) = ffmpeg_or_skip("ffmpeg_filter_under_preserve_gives_back_the_bytes_it_was_given") else { return };
    for (depth, bpp) in [(8u32, 4usize), (16, 8), (32, 16)] {
        let (w, h, n) = (33u32, 17u32, 3usize);
        let mut data: Vec<u8> = (0..w as usize * h as usize * bpp * n).map(|i| (i * 7 % 251) as u8).collect();
        if depth == 32 {
            // floats that are multiples of 1/256 in 0 to 1
            data = (0..w as usize * h as usize * 4 * n).flat_map(|i| ((i % 257) as f32 / 256.0).to_le_bytes()).collect();
        }
        let ran = ff_apply(&ffmpeg, w, h, depth, &["graph=null"], &[], data.clone());
        assert_eq!(ran.code, 0, "depth {}: {}", depth, ran.stderr);
        assert_eq!(ran.stdout.len(), data.len(), "depth {}: bytes out equal bytes in", depth);
        assert!(ran.stdout == data, "depth {}: a null graph returns the input", depth);
    }
    let frame: Vec<u8> = (0..4 * 4 * 2 * 2).map(|i| (i * 13 % 256) as u8).collect();
    let once = ff_apply(&ffmpeg, 4, 2, 8, &["graph=negate"], &[], frame.clone());
    assert_eq!(once.code, 0, "{}", once.stderr);
    assert_ne!(once.stdout, frame);
    let twice = ff_apply(&ffmpeg, 4, 2, 8, &["graph=negate"], &[], once.stdout.clone());
    assert_eq!(twice.stdout, frame, "negate twice is the identity");
}

#[test]
fn ffmpeg_filter_counts_frames_because_ffmpeg_exits_0_on_truncation() {
    let Some(ffmpeg) = ffmpeg_or_skip("ffmpeg_filter_counts_frames_because_ffmpeg_exits_0_on_truncation") else { return };
    let frame = 4 * 2 * 4;
    let ran = ff_apply(&ffmpeg, 4, 2, 8, &["graph=null"], &["--json"], vec![9u8; frame + frame / 2]);
    assert_eq!(ran.code, 4, "{}", ran.stderr);
    assert_eq!(error_of(&ran.stderr).0, "partial_frame");
    let wrong = ff_apply(&ffmpeg, 4, 2, 8, &["graph=null"], &["--frames", "5"], vec![9u8; frame * 2]);
    assert_eq!(wrong.code, 4, "{}", wrong.stderr);
}

fn clip(test: &str) -> Option<(PathBuf, PathBuf)> {
    require_media(test)?;
    let dir = scratch(test);
    let clip = make_clip(&dir, "clip.mp4", 160, 96, 12, 24, 0.0);
    Some((dir, clip))
}

#[test]
fn the_wrapper_chains_declared_geometry_into_encode_and_verify() {
    let Some((dir, clip)) = clip("the_wrapper_chains_declared_geometry_into_encode_and_verify") else { return };
    let out = dir.join("small.mkv");
    let c = clip.to_string_lossy().into_owned();
    let o = out.to_string_lossy().into_owned();
    let ran = nh(&[&c, "--stage", "@ffmpeg-filter graph=scale=80:48 expect=declared", "--stage", "@selftest-invert", "--depth", "8", "--lossless", "-o", &o, "--json"], None);
    assert_eq!(ran.code, 0, "{}{}", ran.out_text(), ran.stderr);
    let doc = Json::parse(&ran.out_text()).unwrap();
    assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some("frames/2"));
    let g = doc.get("geometry").expect("geometry");
    let get = |k: &str| g.get(k).and_then(|v| v.as_i64());
    assert_eq!((get("width"), get("height"), get("depth"), get("frames")), (Some(80), Some(48), Some(8), Some(12)));
    let stages = doc.get("stages").and_then(|v| v.as_array()).unwrap();
    let roles: Vec<(&str, i64, &str)> = stages
        .iter()
        .map(|s| (s.get("role").and_then(|v| v.as_str()).unwrap(), s.get("index").and_then(|v| v.as_i64()).unwrap(), s.get("name").and_then(|v| v.as_str()).unwrap()))
        .collect();
    assert_eq!(roles, [("decode", 1, "decode"), ("pre", 1, "pre1"), ("pre", 2, "pre2"), ("encode", 1, "encode")]);
    for s in stages {
        assert!(s.get("command").and_then(|v| v.as_array()).is_some(), "command is an argv");
        assert!(s.get("stderr_tail").and_then(|v| v.as_array()).is_some());
    }
    assert_eq!(stages[1].get("exit_code").and_then(|v| v.as_i64()), Some(0));
    assert_eq!(decode_raw(&out, "rgba").len(), 12 * 80 * 48 * 4, "the output has the declared size");
}

#[test]
fn a_rate_change_and_an_unknown_contract_are_refused_by_the_wrapper() {
    let Some((dir, clip)) = clip("a_rate_change_and_an_unknown_contract_are_refused_by_the_wrapper") else { return };
    let c = clip.to_string_lossy().into_owned();
    let o = dir.join("x.mkv").to_string_lossy().into_owned();
    let ran = nh(&[&c, "--stage", "@ffmpeg-filter graph=fps=48 expect=declared", "--depth", "8", "--lossless", "-o", &o, "--json"], None);
    assert_eq!(ran.code, 3, "{}", ran.stderr);
    assert_eq!(error_of(&ran.out_text()), ("unsupported".to_string(), Some("pre1".to_string())));
    let old = nh(&[&c, "--stage", "@ffmpeg-filter graph=scale=80:48 expect=declared", "--contract", "frames/1", "--depth", "8", "--lossless", "-o", &o, "--json"], None);
    assert_eq!(old.code, 1, "frames/1 refuses declared geometry with exit 1: {}", old.stderr);
    let bad = nh(&[&c, "--stage", "@selftest-invert", "--contract", "frames/9"], None);
    assert_eq!(bad.code, 2, "{}", bad.stderr);
}

#[test]
fn frames_1_keeps_exit_1_where_frames_2_names_the_code() {
    let Some((dir, clip)) = clip("frames_1_keeps_exit_1_where_frames_2_names_the_code") else { return };
    let c = clip.to_string_lossy().into_owned();
    let o = dir.join("y.mkv").to_string_lossy().into_owned();
    for (contract, exit) in [("frames/2", 3), ("frames/1", 1)] {
        let ran = nh(&[&c, "--stage", "@nope", "--contract", contract, "--depth", "8", "--lossless", "-o", &o, "--json"], None);
        assert_eq!(ran.code, exit, "{}: {}", contract, ran.stderr);
        let doc = Json::parse(ran.out_text().trim()).unwrap();
        assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some(contract));
        assert_eq!(error_of(&ran.out_text()).0, "unsupported");
    }
    let parse = nh(&[&c, "--nope", "--contract", "frames/1"], None);
    assert_eq!(parse.code, 2, "an argument parser rejection is 2 in both contracts");
    let dry = nh(&[&c, "--stage", "@selftest-invert", "--contract", "frames/1", "--dry-run", "--json", "--depth", "8"], None);
    assert_eq!(dry.code, 0, "{}", dry.stderr);
    let doc = Json::parse(&dry.out_text()).unwrap();
    assert!(doc.get("geometry").is_none(), "frames/1 keeps the old report");
    let stages = doc.get("stages").and_then(|v| v.as_array()).unwrap();
    assert_eq!(stages.len(), 1);
    assert_eq!(stages[0].get("name").and_then(|v| v.as_str()), Some("pre1"));
    assert!(stages[0].get("command").and_then(|v| v.as_str()).is_some(), "frames/1 keeps the command as text");
}

#[test]
fn geometry_flags_and_describe_follow_apply_py() {
    let Some((_dir, clip)) = clip("geometry_flags_and_describe_follow_apply_py") else { return };
    let c = clip.to_string_lossy().into_owned();
    let ours = format!(
        "\"{}\" stage apply selftest-invert --width {{width}} --height {{height}} --depth {{depth}} --fps {{fps}}",
        handoff_exe().to_string_lossy().replace('\\', "/")
    );
    let ran = nh(&[&c, "--stage", &ours, "--geometry", "declared", "--stage", "@ffmpeg-filter graph=scale=80:48 expect=declared", "--depth", "8", "--describe"], None);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert_eq!(
        ran.out_text().trim(),
        r#"{"ok": true, "contract": "frames/2", "stage": "stages", "width": 80, "height": 48, "depth": 8, "fps": "24/1", "frames": 12}"#
    );
    let warp = nh(&[&c, "--stage", &ours, "--geometry", "warp", "--depth", "8", "--json"], None);
    assert_eq!(warp.code, 3, "an unknown geometry value is unsupported: {}", warp.stderr);
    assert_eq!(error_of(&warp.out_text()), ("unsupported".to_string(), Some("pre1".to_string())));
    let early = nh(&[&c, "--geometry", "declared", "--stage", &ours], None);
    assert_eq!(early.code, 2, "{}", early.stderr);
    let old = nh(&[&c, "--stage", &ours, "--describe", "--contract", "frames/1"], None);
    assert_eq!(old.code, 2, "{}", old.stderr);
}

/// A success report, from a dry run and from a real run, checked against every key and enum that
/// `contracts/run-report.schema.json` requires, read from the schema itself.
#[test]
fn a_success_report_carries_every_key_the_run_report_schema_requires() {
    let Some((dir, clip)) = clip("a_success_report_carries_every_key_the_run_report_schema_requires") else { return };
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..").join("contracts").join("run-report.schema.json");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {}", path.display(), e));
    let schema = Json::parse(&text).unwrap();
    let strings = |v: Option<&Json>| -> Vec<String> { v.and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|s| s.as_str().map(String::from)).collect()).unwrap_or_default() };
    let props = schema.get("properties").expect("properties");
    let item = props.get("stages").and_then(|s| s.get("items")).expect("stages items");
    let geo = props.get("geometry").expect("geometry");
    let top = strings(schema.get("required"));
    let step_keys = strings(item.get("required"));
    let geo_keys = strings(geo.get("required"));
    let geo_props: Vec<String> = geo.get("properties").and_then(|p| p.as_object()).unwrap().iter().map(|(k, _)| k.clone()).collect();
    let contracts = strings(props.get("contract").and_then(|c| c.get("enum")));
    let roles = strings(item.get("properties").and_then(|p| p.get("role")).and_then(|r| r.get("enum")));
    let depths: Vec<i64> = geo_props_enum(geo, "depth");
    assert!(top.iter().any(|k| k == "ok") && !step_keys.is_empty() && !geo_keys.is_empty(), "schema read: {:?}", top);
    assert!(!contracts.is_empty() && !roles.is_empty() && !depths.is_empty(), "schema enums read");
    let c = clip.to_string_lossy().into_owned();
    for dry in [true, false] {
        let o = dir.join(if dry { "dry.mkv" } else { "run.mkv" }).to_string_lossy().into_owned();
        let mut list = vec![c.as_str(), "--stage", "@selftest-invert", "--depth", "8", "--lossless", "-o", o.as_str(), "--json"];
        if dry {
            list.push("--dry-run");
        }
        let ran = nh(&list, None);
        let text = ran.out_text();
        assert_eq!(ran.code, 0, "{}{}", text, ran.stderr);
        let doc = Json::parse(&text).unwrap_or_else(|e| panic!("{}: {}", e, text));
        for key in &top {
            assert!(doc.get(key).is_some(), "dry {}: the report lacks {}: {}", dry, key, text);
        }
        assert_eq!(doc.get("ok").and_then(|v| v.as_bool()), Some(true), "ok is the constant true");
        let contract = doc.get("contract").and_then(|v| v.as_str()).unwrap();
        assert!(contracts.iter().any(|k| k == contract), "contract {}", contract);
        let stages = doc.get("stages").and_then(|v| v.as_array()).expect("stages is an array");
        assert!(stages.len() >= 3, "{}", text);
        for s in stages {
            for key in &step_keys {
                assert!(s.get(key).is_some(), "dry {}: a step lacks {}: {}", dry, key, text);
            }
            let role = s.get("role").and_then(|v| v.as_str()).unwrap();
            assert!(roles.iter().any(|r| r == role), "role {}", role);
            assert!(s.get("index").and_then(|v| v.as_i64()).unwrap() >= 1);
            assert!(!s.get("name").and_then(|v| v.as_str()).unwrap().is_empty());
            assert!(s.get("command").and_then(|v| v.as_array()).unwrap().iter().all(|a| a.as_str().is_some()), "command is an argv of strings");
            assert!(s.get("stderr_tail").and_then(|v| v.as_array()).unwrap().iter().all(|l| l.as_str().is_some()));
            let exit = s.get("exit_code").unwrap();
            if dry {
                assert!(exit.is_null(), "a dry run starts nothing: {}", text);
            } else {
                assert_eq!(exit.as_i64(), Some(0), "{}", text);
            }
        }
        let g = doc.get("geometry").expect("frames/2 geometry");
        for key in &geo_keys {
            assert!(g.get(key).is_some(), "dry {}: geometry lacks {}: {}", dry, key, text);
        }
        for (key, _) in g.as_object().unwrap() {
            assert!(geo_props.contains(key), "geometry allows no {}", key);
        }
        assert!(g.get("width").and_then(|v| v.as_i64()).unwrap() >= 1 && g.get("height").and_then(|v| v.as_i64()).unwrap() >= 1);
        assert!(depths.contains(&g.get("depth").and_then(|v| v.as_i64()).unwrap()));
        let fps = g.get("fps").and_then(|v| v.as_str()).unwrap();
        let part = |p: &str| !p.is_empty() && !p.starts_with('0') && p.bytes().all(|b| b.is_ascii_digit());
        assert!(fps.split_once('/').is_some_and(|(n, d)| part(n) && part(d)), "fps {}", fps);
        let frames = g.get("frames").unwrap();
        assert!(frames.is_null() || frames.as_i64().is_some_and(|n| n >= 0), "frames {}", text);
    }
}

fn geo_props_enum(geo: &Json, key: &str) -> Vec<i64> {
    let values = geo.get("properties").and_then(|p| p.get(key)).and_then(|d| d.get("enum")).and_then(|e| e.as_array());
    values.map(|a| a.iter().filter_map(|d| d.as_i64()).collect()).unwrap_or_default()
}
