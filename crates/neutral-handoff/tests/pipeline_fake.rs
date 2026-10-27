//! The pipeline against real ffmpeg and a fake `vfx`: ordering of the stages, the in-process
//! stage, the error object for every way a stage can misbehave (hang, close early, truncate,
//! exit non-zero, return the wrong number of frames), the stage timeout, the contract version,
//! the refusals that come before any frame moves, and the cleanup after a failure.
//!
//! Needs ffmpeg and ffprobe (skips with a printed reason without them). The fake `vfx` is built
//! with the crate and needs nothing.

mod common;

use std::path::{Path, PathBuf};
use std::time::Duration;

use common::*;
use neutral_ffmpeg::json::Json;

/// A temp folder for one test, passed as `TMPDIR`, so that the scratch folders of the run land
/// in a place the test can inspect.
struct Case {
    dir: PathBuf,
    tmp: PathBuf,
    clip: PathBuf,
}

impl Case {
    fn new(name: &str) -> Option<Case> {
        require_media(name)?;
        let dir = scratch(name);
        let tmp = dir.join("tmp");
        std::fs::create_dir_all(&tmp).unwrap();
        let clip = make_clip(&dir, "clip.mp4", 160, 96, 12, 24, 0.6);
        Some(Case { dir, tmp, clip })
    }

    fn out(&self, name: &str) -> String {
        self.dir.join(name).to_string_lossy().into_owned()
    }

    fn go(&self, extra: &[&str]) -> Ran {
        self.go_env(extra, &[])
    }

    fn go_env(&self, extra: &[&str], env: &[(&str, &str)]) -> Ran {
        let mut a = vec![self.clip.to_string_lossy().into_owned()];
        a.extend(extra.iter().map(|s| s.to_string()));
        let tmp = self.tmp.to_string_lossy().into_owned();
        let mut full_env: Vec<(&str, &str)> = vec![("TMPDIR", tmp.as_str()), ("TEMP", tmp.as_str()), ("TMP", tmp.as_str())];
        full_env.extend_from_slice(env);
        run_env(&handoff_exe(), &a, &full_env, None, Duration::from_secs(120))
    }

    fn leftovers(&self) -> Vec<String> {
        let mut found = partial_files(&self.dir);
        for entry in std::fs::read_dir(&self.tmp).unwrap().filter_map(|e| e.ok()) {
            found.push(format!("tmp/{}", entry.file_name().to_string_lossy()));
        }
        found
    }
}

fn fake() -> String {
    fake_vfx().to_string_lossy().replace('\\', "/")
}

/// A `--stage` command that runs `effect` of the fake vfx.
fn stage_cmd(effect: &str) -> String {
    format!("'{}' apply {} --width {{width}} --height {{height}} --depth {{depth}} --fps {{fps}}", fake(), effect)
}

fn json(ran: &Ran) -> Json {
    Json::parse(&ran.out_text()).unwrap_or_else(|e| panic!("stdout is not JSON ({}): {}", e, ran.out_text()))
}

fn error_object(ran: &Ran) -> (String, Option<String>, Option<i64>, String, Vec<String>) {
    let doc = json(ran);
    assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some("frames/2"), "{}", ran.out_text());
    assert_eq!(doc.get("ok").and_then(|v| v.as_bool()), Some(false));
    let e = doc.get("error").expect("an error object");
    let text = |k: &str| e.get(k).and_then(|v| v.as_str()).map(|s| s.to_string());
    (
        text("code").expect("code"),
        text("stage"),
        e.get("exit_code").and_then(|v| v.as_i64()),
        text("message").expect("message"),
        e.get("stderr_tail").and_then(|v| v.as_array()).expect("stderr_tail").iter().map(|l| l.as_str().unwrap().to_string()).collect(),
    )
}

fn rgba_frames(path: &str) -> Vec<Vec<u8>> {
    decode_raw(Path::new(path), "rgba").chunks(160 * 96 * 4).map(|c| c.to_vec()).collect()
}

#[test]
fn a_fake_effect_inverts_like_a_direct_inversion() {
    let Some(case) = Case::new("fake_invert") else { return };
    let out = case.out("inv.mkv");
    let ran = case.go(&["fake-invert", "--vfx", &fake(), "--depth", "8", "--lossless", "-o", &out]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    let mut expected = decode_with_matrix(&case.clip, "smpte170m", "rgba");
    for px in expected.chunks_exact_mut(4) {
        for c in &mut px[..3] {
            *c = 255 - *c;
        }
    }
    assert_eq!(decode_raw(Path::new(&out), "rgba"), expected);
    let last = ran.out_text().lines().last().unwrap().to_string();
    assert!(last.contains(": 0.5") && last.ends_with("s, 160x96, 24 fps, ffv1, aac 2ch"), "{}", last);
    assert!(case.leftovers().is_empty(), "{:?}", case.leftovers());
}

#[test]
fn stages_run_in_the_order_given_and_the_marker_moves_the_effect() {
    let Some(case) = Case::new("fake_order") else { return };
    // spool reverses the frame order, number writes the frame number it sees into the first byte.
    let tags = |path: &str| -> Vec<u8> { rgba_frames(path).iter().map(|f| f[0]).collect() };
    let base = ["--depth", "8", "--lossless"];

    // number after spool: frame i carries tag i
    let a = case.out("a.mkv");
    let ran = case.go(&[&["--stage", &stage_cmd("fake-spool"), "--stage", &stage_cmd("fake-number"), "-o", &a][..], &base[..]].concat());
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert_eq!(tags(&a), (0..12).collect::<Vec<u8>>());

    // number before spool: frame i carries tag 11 - i
    let b = case.out("b.mkv");
    let ran = case.go(&[&["--stage", &stage_cmd("fake-number"), "--stage", &stage_cmd("fake-spool"), "-o", &b][..], &base[..]].concat());
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert_eq!(tags(&b), (0..12).rev().collect::<Vec<u8>>());

    // the effect comes first by default ...
    let c = case.out("c.mkv");
    let ran = case.go(&[&["fake-spool", "--vfx", &fake(), "--stage", &stage_cmd("fake-number"), "-o", &c][..], &base[..]].concat());
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert_eq!(tags(&c), (0..12).collect::<Vec<u8>>());

    // ... and a `vfx` stage puts it where it is written
    let d = case.out("d.mkv");
    let ran = case.go(&[&["fake-spool", "--vfx", &fake(), "--stage", &stage_cmd("fake-number"), "--stage", "vfx", "-o", &d][..], &base[..]].concat());
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert_eq!(tags(&d), (0..12).rev().collect::<Vec<u8>>());
    assert!(case.leftovers().is_empty(), "{:?}", case.leftovers());
}

#[test]
fn an_in_process_stage_twice_returns_the_input() {
    let Some(case) = Case::new("fake_builtin") else { return };
    for (depth, fmt) in [("8", "rgba"), ("16", "rgba64le")] {
        let out = case.out(&format!("twice{}.mkv", depth));
        let ran = case.go(&["--stage", "@selftest-invert", "--stage", "@selftest-invert", "--depth", depth, "--lossless", "-o", &out]);
        assert_eq!(ran.code, 0, "{}", ran.stderr);
        assert_eq!(decode_raw(Path::new(&out), fmt), decode_with_matrix(&case.clip, "smpte170m", fmt), "depth {}", depth);
        let once = case.out(&format!("once{}.mkv", depth));
        let ran = case.go(&["--stage", "@selftest-invert", "--depth", depth, "--lossless", "-o", &once]);
        assert_eq!(ran.code, 0, "{}", ran.stderr);
        assert_ne!(decode_raw(Path::new(&once), fmt), decode_with_matrix(&case.clip, "smpte170m", fmt));
    }
}

#[test]
fn a_misbehaving_stage_ends_in_one_error_object() {
    let Some(case) = Case::new("fake_failures") else { return };
    // (effect, code, exit code of this tool, exit code of the stage, message fragment)
    let table: [(&str, &str, i64, Option<i64>, &str); 9] = [
        ("fake-fail", "unsupported", 3, Some(3), "pre1 exited with code 3"),
        ("fake-exit1", "stage_failed", 1, Some(1), "pre1 exited with code 1"),
        ("fake-truncate", "partial_frame", 4, Some(0), "partial frame (30720 of 61440 bytes) after 1 whole frames"),
        ("fake-truncate-fail", "bad_input", 4, Some(4), "pre1 exited with code 4"),
        ("fake-close-early", "broken_pipe", 4, Some(0), "pre1 closed its"),
        ("fake-short", "broken_pipe", 4, Some(0), "pre1 closed its output after 11 of 12 frames"),
        ("fake-extra", "stage_failed", 1, Some(0), "pre1 returned 13 frames for the 12 it was given"),
        ("fake-hang", "stage_timeout", 5, None, "pre1 moved no data for 1 seconds and was killed"),
        ("fake-hang-mid", "stage_timeout", 5, None, "pre1 moved no data"),
    ];
    for (effect, code, exit, stage_exit, fragment) in table {
        let out = case.out(&format!("{}.mkv", effect));
        let ran = case.go(&["--stage", &stage_cmd(effect), "--stage-timeout", "1", "--depth", "8", "--lossless", "-o", &out, "--json"]);
        assert_eq!(ran.code as i64, exit, "{}: {}", effect, ran.stderr);
        let (got_code, stage, got_exit, message, _) = error_object(&ran);
        assert_eq!((got_code.as_str(), got_exit), (code, stage_exit), "{}: {}", effect, message);
        assert_eq!(stage.as_deref(), Some("pre1"), "{}", effect);
        assert!(message.contains(fragment), "{}: {:?} lacks {:?}", effect, message, fragment);
        assert!(ran.stderr.starts_with(&format!("error: {}: ", code)), "{}: {}", effect, ran.stderr);
        assert_eq!(ran.stderr.trim().lines().count(), 1, "text mode is one line: {}", ran.stderr);
        assert!(!Path::new(&out).exists(), "{}: no output on failure", effect);
        if code == "stage_timeout" {
            assert!(ran.seconds < 30.0, "{}: took {} s", effect, ran.seconds);
        }
        assert!(case.leftovers().is_empty(), "{}: {:?}", effect, case.leftovers());
    }
}

#[test]
fn a_hang_after_the_last_frame_is_a_timeout_too() {
    let Some(case) = Case::new("fake_hang_end") else { return };
    let out = case.out("x.mkv");
    let ran = case.go(&["--stage", &stage_cmd("fake-hang-at-end"), "--stage-timeout", "1", "--depth", "8", "--lossless", "-o", &out, "--json"]);
    let (code, stage, exit, message, _) = error_object(&ran);
    assert_eq!((code.as_str(), stage.as_deref(), exit, ran.code), ("stage_timeout", Some("pre1"), None, 5), "{}", message);
    assert!(ran.seconds < 30.0);
    assert!(case.leftovers().is_empty(), "{:?}", case.leftovers());
}

#[test]
fn the_failing_stage_is_named_even_with_healthy_neighbours() {
    let Some(case) = Case::new("fake_neighbours") else { return };
    let out = case.out("n.mkv");
    for (first, second, culprit, code) in [
        ("fake-copy", "fake-fail", "pre2", "unsupported"),
        ("fake-fail", "fake-copy", "pre1", "unsupported"),
        ("fake-copy", "fake-truncate", "pre2", "partial_frame"),
        ("fake-truncate", "fake-copy", "pre1", "partial_frame"),
        ("fake-copy", "fake-short", "pre2", "broken_pipe"),
        ("fake-short", "fake-copy", "pre1", "broken_pipe"),
        ("fake-hang", "fake-copy", "pre1", "stage_timeout"),
        ("fake-copy", "fake-hang", "pre2", "stage_timeout"),
    ] {
        let ran = case.go(&["--stage", &stage_cmd(first), "--stage", &stage_cmd(second), "--stage-timeout", "1", "--depth", "8", "--lossless", "-o", &out, "--json"]);
        let (got_code, stage, _, message, _) = error_object(&ran);
        assert_eq!((got_code.as_str(), stage.as_deref()), (code, Some(culprit)), "{} then {}: {}", first, second, message);
        assert!(case.leftovers().is_empty(), "{} then {}: {:?}", first, second, case.leftovers());
    }
}

#[test]
fn the_effect_is_a_stage_like_any_other() {
    let Some(case) = Case::new("fake_effect_failures") else { return };
    let out = case.out("e.mkv");
    let ran = case.go(&["fake-fail", "--vfx", &fake(), "--depth", "8", "--lossless", "-o", &out, "--json"]);
    let (code, stage, exit, _, tail) = error_object(&ran);
    assert_eq!((code.as_str(), stage.as_deref(), exit, ran.code), ("unsupported", Some("vfx"), Some(3), 3));
    assert_eq!(tail.last().map(|s| s.as_str()), Some("vfx: error: frame 2: unsupported: injected failure"));
    // text mode appends the last line the stage printed
    let text = case.go(&["fake-fail", "--vfx", &fake(), "--depth", "8", "--lossless", "-o", &out]);
    assert_eq!(text.stderr.trim(), "error: unsupported: vfx exited with code 3: vfx: error: frame 2: unsupported: injected failure");
}

#[test]
fn a_stage_that_stderr_floods_cannot_stall_the_pipe() {
    let Some(case) = Case::new("fake_flood") else { return };
    let out = case.out("flood.mkv");
    let ran = case.go(&["--stage", &stage_cmd("fake-flood"), "--depth", "8", "--lossless", "-o", &out]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert!(Path::new(&out).is_file());
}

#[test]
fn a_stage_that_reads_everything_before_it_writes_cannot_deadlock_at_depth_32() {
    let Some(case) = Case::new("fake_spool32") else { return };
    // 12 frames of 160x96 at depth 32 are 2.9 MB each way, far more than any pipe holds
    let out = case.out("spool32.mp4");
    let ran = case.go(&["--stage", &stage_cmd("fake-spool"), "--stage", &stage_cmd("fake-spool"), "--depth", "32", "-o", &out, "--json"]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    let doc = json(&ran);
    let reports = doc.get("stage_reports").and_then(|v| v.as_array()).expect("stage_reports");
    assert_eq!(reports.len(), 2);
    for r in reports {
        assert_eq!((r.get("frames_in").and_then(|v| v.as_i64()), r.get("frames_out").and_then(|v| v.as_i64())), (Some(12), Some(12)));
        assert_eq!(r.get("exit_code").and_then(|v| v.as_i64()), Some(0));
    }
    assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some("frames/2"));
}

#[test]
fn a_slow_stage_that_keeps_moving_is_not_killed() {
    let Some(case) = Case::new("fake_slow") else { return };
    let out = case.out("slow.mkv");
    // 12 frames at 250 ms: three seconds in all, but never a second without data
    let ran = case.go(&["fake-slow", "--vfx", &fake(), "--param", "delay_ms=250", "--stage-timeout", "1", "--depth", "8", "--lossless", "-o", &out]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert!(ran.seconds >= 2.5, "took {} s", ran.seconds);
}

#[test]
fn silence_longer_than_the_timeout_kills_even_a_stage_that_works() {
    let Some(case) = Case::new("fake_silence") else { return };
    let out = case.out("slow.mkv");
    let slow = format!("{} --param delay_ms=1500", stage_cmd("fake-slow"));
    let ran = case.go(&["--stage", &slow, "--stage-timeout", "1", "--depth", "8", "--lossless", "-o", &out, "--json"]);
    let (code, stage, _, _, _) = error_object(&ran);
    assert_eq!((code.as_str(), stage.as_deref(), ran.code), ("stage_timeout", Some("pre1"), 5));
    assert!(ran.seconds < 30.0, "took {} s", ran.seconds);
    assert!(case.leftovers().is_empty(), "{:?}", case.leftovers());
}

#[test]
fn a_timeout_of_zero_never_kills() {
    let Some(case) = Case::new("fake_no_timeout") else { return };
    let out = case.out("slow.mkv");
    // 12 frames at 300 ms: longer than the one second a default of 1 would allow
    let ran = case.go(&["fake-slow", "--vfx", &fake(), "--param", "delay_ms=300", "--stage-timeout", "0", "--depth", "8", "--lossless", "-o", &out, "--force"]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    let ran = case.go(&["--stage", &stage_cmd("fake-copy"), "--stage-timeout", "0", "--depth", "8", "--lossless", "-o", &out, "--force"]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
}

#[test]
fn the_contract_version_of_a_stage_is_checked() {
    let Some(case) = Case::new("fake_contract") else { return };
    let out = case.out("c.mkv");
    for bad in ["frames/3", "frames/0", "other/1"] {
        let ran = case.go_env(&["--stage", &stage_cmd("fake-copy"), "--depth", "8", "--lossless", "-o", &out, "--json"], &[("FAKE_VFX_CONTRACT", bad)]);
        let (code, stage, _, message, _) = error_object(&ran);
        assert_eq!((code.as_str(), stage.as_deref(), ran.code), ("unsupported", Some("pre1"), 3), "{}", bad);
        assert!(message.contains(&format!("reports the contract `{}`", bad)), "{}", message);
        assert!(!Path::new(&out).exists());
        let ran = case.go_env(&["fake-copy", "--vfx", &fake(), "--depth", "8", "--lossless", "-o", &out, "--json"], &[("FAKE_VFX_CONTRACT", bad)]);
        let (code, _, _, message, _) = error_object(&ran);
        assert_eq!((code.as_str(), ran.code), ("unsupported", 3), "{}", bad);
        assert!(message.contains(bad), "{}", message);
    }
    for good in ["frames/1", "frames/1.3", "frames/2"] {
        let ran = case.go_env(&["--stage", &stage_cmd("fake-copy"), "--depth", "8", "--lossless", "-o", &out, "--force"], &[("FAKE_VFX_CONTRACT", good)]);
        assert_eq!(ran.code, 0, "{}: {}", good, ran.stderr);
        let ran = case.go_env(&["fake-copy", "--vfx", &fake(), "--depth", "8", "--lossless", "-o", &out, "--force"], &[("FAKE_VFX_CONTRACT", good)]);
        assert_eq!(ran.code, 0, "{}: {}", good, ran.stderr);
    }
}

#[test]
fn refusals_come_before_any_frame_moves() {
    let Some(case) = Case::new("fake_refusals") else { return };
    let out = case.out("r.mkv");
    let existing = case.out("exists.mkv");
    std::fs::write(&existing, b"keep").unwrap();
    let missing = case.out("missing.mp4");
    let odd = {
        let (ffmpeg, _) = media().unwrap();
        let path = case.dir.join("odd.mkv");
        let a: Vec<std::ffi::OsString> = ["-v", "error", "-nostdin", "-y", "-f", "lavfi", "-i", "color=c=red:size=33x17:rate=10:duration=0.5", "-c:v", "ffv1", "-pix_fmt", "yuv444p"]
            .iter()
            .map(std::ffi::OsString::from)
            .chain([path.clone().into_os_string()])
            .collect();
        let r = neutral_ffmpeg::exec::run_capture(&ffmpeg, &a, Duration::from_secs(60), "ffmpeg", "odd clip").unwrap();
        assert!(r.status.success(), "{}", r.stderr_text());
        path
    };
    let not_media = case.dir.join("notes.txt");
    std::fs::write(&not_media, b"not a video").unwrap();
    let v = fake();
    let clip = case.clip.to_string_lossy().into_owned();
    let odd_s = odd.to_string_lossy().into_owned();
    let not_media_s = not_media.to_string_lossy().into_owned();

    let refuse = |input: &str, rest: &[&str]| -> (String, String, i64, String) {
        let mut a = vec![input.to_string()];
        a.extend(rest.iter().map(|s| s.to_string()));
        a.push("--json".into());
        let ran = run(&handoff_exe(), &a);
        let (code, stage, _, message, _) = error_object(&ran);
        (code, stage.unwrap_or_default(), ran.code as i64, message)
    };
    let no_such = format!("no such file: {}", missing);
    // (what the run said, what it should say: code, stage, exit code, a piece of the message)
    let cases: Vec<((String, String, i64, String), (&str, &str, i64, &str))> = vec![
        (refuse(&missing, &["fake-copy", "--vfx", &v]), ("bad_input", "probe", 4, &no_such)),
        (refuse(&not_media_s, &["fake-copy", "--vfx", &v]), ("bad_input", "probe", 4, "is it a media file?")),
        (refuse(&clip, &["fake-nope", "--vfx", &v]), ("unsupported", "vfx", 3, "unknown effect `fake-nope`, `vfx list` shows the registered ones")),
        (refuse(&clip, &["fake-copy", "--vfx", &v, "--param", "level=500"]), ("unsupported", "vfx", 3, "vfx: bad value for `level`: 500 outside 0..=100")),
        (refuse(&clip, &["fake-copy", "--vfx", &v, "--param", "nope=1"]), ("unsupported", "vfx", 3, "vfx: unknown param `nope`")),
        (refuse(&clip, &["fake-copy", "--vfx", &v, "--frames", "5"]), ("unsupported", "plan", 3, "fake-copy is a frame effect, --frames only apply to temporal effects")),
        (refuse(&clip, &["fake-copy", "--vfx", &v, "--layer", "a=b.mp4"]), ("unsupported", "plan", 3, "fake-copy is a frame effect, --layer only apply to temporal effects")),
        (refuse(&clip, &["fake-layer", "--vfx", &v]), ("unsupported", "plan", 3, "fake-layer needs the layer `overlay`, pass --layer overlay=VIDEO")),
        (refuse(&clip, &["fake-layer", "--vfx", &v, "--layer", "nope=x.mp4"]), ("unsupported", "plan", 3, "unknown layer `nope`: fake-layer has the layers overlay, extra")),
        (refuse(&clip, &["fake-copy", "--vfx", &v, "--depth", "32", "--lossless"]), ("unsupported", "plan", 3, "--lossless needs --depth 8 or 16: ffv1 has no float pixel format")),
        (refuse(&odd_s, &["fake-copy", "--vfx", &v]), ("unsupported", "plan", 3, "the frame size 33x17 has an odd side, and yuv420p needs even sides")),
        (refuse(&clip, &["fake-copy", "--vfx", &v, "-o", &existing]), ("bad_input", "plan", 4, "output already exists; choose a new --output path or pass --force")),
        (refuse(&clip, &["fake-copy", "--vfx", &v, "-o", &clip]), ("bad_input", "plan", 4, "the output path is the input file; choose a different --output path")),
        (refuse(&clip, &["fake-copy", "--vfx", "no/such/vfx"]), ("tool_missing", "vfx", 6, "vfx not found at no/such/vfx")),
        (refuse(&clip, &["--stage", "no-such-stage-program apply"]), ("tool_missing", "pre1", 6, "pre1: the program `no-such-stage-program` was not found")),
        (refuse(&clip, &["--stage", "vfx"]), ("usage", "args", 2, "`--stage vfx` marks where the effect runs, and no EFFECT was given")),
        (refuse(&clip, &["--stage", "'unterminated"]), ("usage", "pre1", 2, "No closing quotation")),
        (refuse(&odd_s, &["--stage", "no-such apply --frames {frames}", "--lossless"]), ("usage", "pre1", 2, "--stage 1: {frames} is not known for this input")),
        (refuse(&clip, &["--stage", "@nope"]), ("unsupported", "pre1", 3, "unknown stage `@nope`")),
        (refuse(&clip, &["--stage", "@selftest-invert invert_alpha=maybe"]), ("unsupported", "pre1", 3, "bad value for `invert_alpha`: cannot parse `maybe`")),
        (refuse(&clip, &[]), ("usage", "args", 2, "give an EFFECT or at least one --stage")),
        (refuse(&clip, &["--stage", "@selftest-invert", "--param", "a=1"]), ("usage", "args", 2, "--param needs an EFFECT")),
        (refuse(&clip, &["--stage", "@selftest-invert", "--frames", "3"]), ("usage", "args", 2, "--layer and --frames only apply to a temporal EFFECT")),
    ];
    for (got, want) in cases {
        assert_eq!((got.0.as_str(), got.1.as_str(), got.2), (want.0, want.1, want.2), "{:?}", got.3);
        assert!(got.3.contains(want.3), "{:?} lacks {:?}", got.3, want.3);
    }
    assert_eq!(std::fs::read(&existing).unwrap(), b"keep", "an existing output is never touched by a refusal");
    assert!(!Path::new(&out).exists());
    assert!(case.leftovers().is_empty(), "{:?}", case.leftovers());
}

#[test]
fn ffmpeg_that_is_not_there_is_tool_missing() {
    let Some(case) = Case::new("fake_no_ffmpeg") else { return };
    let ran = case.go(&["fake-copy", "--vfx", &fake(), "--ffmpeg-path", "definitely-not-ffmpeg", "--json"]);
    let (code, _, _, message, _) = error_object(&ran);
    assert_eq!((code.as_str(), ran.code), ("tool_missing", 6));
    assert_eq!(message, "explicit ffmpeg executable not found: definitely-not-ffmpeg");
}

#[test]
fn a_dry_run_lists_every_stage_with_its_placeholders_expanded() {
    let Some(case) = Case::new("fake_dry_run") else { return };
    let cmd = format!("'{}' apply fake-copy --width {{width}} --height {{height}} --depth {{depth}} --fps {{fps}} --frames {{frames}} --param pix={{pix_fmt}}", fake());
    let ran = case.go(&["fake-copy", "--vfx", &fake(), "--stage", &cmd, "--stage", "@selftest-invert invert_alpha=1", "--dry-run", "--depth", "8"]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    let text = ran.out_text();
    let lines: Vec<&str> = text.lines().collect();
    assert!(lines[0].starts_with("[plan] fake-copy on clip.mp4: 160x96, 24.000 fps, 8 bpc (rgba), decode | vfx | post1 | post2 | encode -> "), "{}", lines[0]);
    let cmds: Vec<&&str> = lines.iter().filter(|l| l.starts_with("[cmd] ")).collect();
    assert_eq!(cmds.len(), 5, "decode, vfx, two stages, encode: {:?}", cmds);
    assert!(cmds[1].contains("fake-vfx") && cmds[1].ends_with("apply fake-copy --width 160 --height 96 --depth 8 --fps 24/1"), "{}", cmds[1]);
    assert!(cmds[2].contains("apply fake-copy --width 160 --height 96 --depth 8 --fps 24/1 --frames 12 --param pix=rgba"), "{}", cmds[2]);
    assert!(cmds[3].contains("stage apply selftest-invert --width 160 --height 96 --depth 8 --fps 24/1 --param invert_alpha=1"), "{}", cmds[3]);
    assert_eq!(*lines.last().unwrap(), "[dry-run] nothing was written");
    assert!(case.leftovers().is_empty());

    let doc_run = case.go(&["fake-copy", "--vfx", &fake(), "--stage", &cmd, "--dry-run", "--depth", "8", "--json"]);
    let doc = json(&doc_run);
    assert_eq!(doc.get("contract").and_then(|v| v.as_str()), Some("frames/2"));
    assert_eq!(doc.get("dry_run").and_then(|v| v.as_bool()), Some(true));
    let stages = doc.get("stages").and_then(|v| v.as_array()).unwrap();
    let kinds: Vec<&str> = stages.iter().filter_map(|s| s.get("kind").and_then(|k| k.as_str())).collect();
    assert_eq!(kinds, ["vfx", "external"]);
    let roles: Vec<&str> = stages.iter().map(|s| s.get("role").and_then(|k| k.as_str()).unwrap()).collect();
    assert_eq!(roles, ["decode", "effect", "post", "encode"]);
    assert!(stages.iter().all(|s| s.get("exit_code").map(|v| v.is_null()) == Some(true)), "a dry run has no exit codes");
    assert_eq!(doc.get("stage_timeout").and_then(|v| v.as_f64()), Some(120.0));
}

#[test]
fn a_run_reports_its_stages_in_json() {
    let Some(case) = Case::new("fake_report") else { return };
    let out = case.out("rep.mkv");
    let ran = case.go(&["fake-invert", "--vfx", &fake(), "--stage", &stage_cmd("fake-copy"), "--depth", "8", "--lossless", "-o", &out, "--json"]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    let doc = json(&ran);
    assert_eq!(doc.get("operation").and_then(|v| v.as_str()), Some("apply"));
    assert_eq!(doc.get("fps_exact").and_then(|v| v.as_str()), Some("24/1"));
    assert_eq!(doc.get("input_frames").and_then(|v| v.as_i64()), Some(12));
    let reports = doc.get("stage_reports").and_then(|v| v.as_array()).unwrap();
    let names: Vec<&str> = reports.iter().map(|r| r.get("name").and_then(|n| n.as_str()).unwrap()).collect();
    assert_eq!(names, ["vfx", "post1"]);
    let outputs = doc.get("outputs").and_then(|v| v.as_array()).unwrap();
    assert_eq!(outputs.len(), 1);
    let probe = outputs[0].get("probe").unwrap();
    assert_eq!(probe.get("vcodec").and_then(|v| v.as_str()), Some("ffv1"));
    assert_eq!(probe.get("acodec").and_then(|v| v.as_str()), Some("aac"));
    assert_eq!((probe.get("width").and_then(|v| v.as_i64()), probe.get("height").and_then(|v| v.as_i64())), (Some(160), Some(96)));
}

#[test]
fn a_temporal_effect_gets_its_layer_decoded_and_the_scratch_folder_is_removed() {
    let Some(case) = Case::new("fake_layers") else { return };
    let layer = make_clip(&case.dir, "layer.mp4", 80, 48, 6, 12, 0.0);
    let out = case.out("layer.mkv");
    let ran = case.go(&["fake-layer", "--vfx", &fake(), "--layer", &format!("overlay={}", layer.display()), "--depth", "8", "--lossless", "-o", &out, "--json"]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    let doc = json(&ran);
    let layers = doc.get("layers").and_then(|v| v.as_array()).unwrap();
    assert_eq!(layers.len(), 1);
    let temp = layers[0].get("temp").and_then(|v| v.as_str()).unwrap();
    assert!(temp.ends_with("layer_overlay.rgba"), "{}", temp);
    assert_eq!(doc.get("kind").and_then(|v| v.as_str()), Some("temporal"));
    assert!(case.leftovers().is_empty(), "scratch folder and partial file are removed: {:?}", case.leftovers());
    assert!(!Path::new(temp).exists());

    // a layer that is not media is refused before the run
    let junk = case.dir.join("junk.mp4");
    std::fs::write(&junk, b"junk").unwrap();
    let ran = case.go(&["fake-layer", "--vfx", &fake(), "--layer", &format!("overlay={}", junk.display()), "-o", &out, "--force", "--json"]);
    let (code, _, _, message, _) = error_object(&ran);
    assert_eq!((code.as_str(), ran.code), ("bad_input", 4));
    assert!(message.contains("is it a media file?"), "{}", message);
}

#[test]
fn an_existing_output_is_replaced_only_after_the_new_one_verifies() {
    let Some(case) = Case::new("fake_force") else { return };
    let out = case.out("keep.mkv");
    std::fs::write(&out, b"old output").unwrap();
    let ran = case.go(&["--stage", &stage_cmd("fake-fail"), "--depth", "8", "--lossless", "-o", &out, "--force"]);
    assert_eq!(ran.code, 3, "{}", ran.stderr);
    assert_eq!(std::fs::read(&out).unwrap(), b"old output", "a failed run leaves the old file alone");
    let ran = case.go(&["--stage", &stage_cmd("fake-copy"), "--depth", "8", "--lossless", "-o", &out, "--force"]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    assert_ne!(std::fs::read(&out).unwrap(), b"old output");
    assert!(case.leftovers().is_empty(), "{:?}", case.leftovers());
}

#[test]
fn the_frames_flag_sets_the_length_of_a_temporal_effect_and_the_audio_is_cut() {
    let Some(case) = Case::new("fake_frames") else { return };
    let out = case.out("short.mkv");
    // fake-spool renders N frames when asked: the output has N frames and the 0.6 s audio is cut at the video's end
    let ran = case.go(&["fake-spool", "--vfx", &fake(), "--frames", "6", "--depth", "8", "--lossless", "-o", &out, "--json"]);
    assert_eq!(ran.code, 0, "{}", ran.stderr);
    let doc = json(&ran);
    assert_eq!(doc.get("frames").and_then(|v| v.as_i64()), Some(6));
    assert_eq!(doc.get("audio").and_then(|a| a.get("plan")).and_then(|v| v.as_str()), Some("shortest"));
    assert_eq!(doc.get("audio").and_then(|a| a.get("shortest")).and_then(|v| v.as_bool()), Some(true));
    assert_eq!(rgba_frames(&out).len(), 6);
    let summary = doc.get("summary").and_then(|v| v.as_str()).unwrap();
    assert!(summary.contains("temporal, 6 frames") && summary.contains("audio cut at the end of the video (-shortest)"), "{}", summary);
}

#[test]
fn a_truncated_input_is_refused_and_not_quietly_shortened() {
    let Some(case) = Case::new("fake_truncated") else { return };
    // an mp4 with its index in front, cut off at 60 percent: ffprobe still reads it, ffmpeg decodes the
    // frames that are there and exits 0 (the input ends early), and the pipeline must not call that a clip
    let (ffmpeg, _) = media().unwrap();
    let whole = case.dir.join("whole.mp4");
    let a: Vec<std::ffi::OsString> = ["-v", "error", "-nostdin", "-y", "-f", "lavfi", "-i", "testsrc2=size=160x96:rate=24:duration=2", "-frames:v", "48", "-c:v", "libx264", "-crf", "12", "-g", "5", "-pix_fmt", "yuv420p", "-movflags", "+faststart"]
        .iter()
        .map(std::ffi::OsString::from)
        .chain([whole.clone().into_os_string()])
        .collect();
    let made = neutral_ffmpeg::exec::run_capture(&ffmpeg, &a, Duration::from_secs(60), "ffmpeg", "faststart clip").unwrap();
    assert!(made.status.success(), "{}", made.stderr_text());
    let bytes = std::fs::read(&whole).unwrap();
    let cut = case.dir.join("cut.mp4");
    std::fs::write(&cut, &bytes[..bytes.len() * 6 / 10]).unwrap();
    let out = case.out("cut_out.mkv");
    let ran = run_env(
        &handoff_exe(),
        &[cut.to_string_lossy().into_owned(), "--stage".into(), "@selftest-invert".into(), "--depth".into(), "8".into(), "--lossless".into(), "-o".into(), out.clone(), "--json".into()],
        &[],
        None,
        Duration::from_secs(60),
    );
    let (code, stage, _, message, _) = error_object(&ran);
    assert_eq!(code, "bad_input", "{}", message);
    assert!(matches!(stage.as_deref(), Some("probe") | Some("decode")), "{:?}: {}", stage, message);
    if stage.as_deref() == Some("decode") {
        assert!(message.contains("the input decoded to") && message.contains("its container states 48"), "{}", message);
    }
    assert!(!Path::new(&out).exists());
    assert!(partial_files(&case.dir).is_empty());
}

/// A stage written by a library user: it paints every pixel opaque black.
struct Black;

impl neutral_handoff::FrameStage for Black {
    fn process(&mut self, _index: u64, frame: &mut [u8]) -> Result<(), String> {
        for px in frame.chunks_exact_mut(4) {
            px.copy_from_slice(&[0, 0, 0, 255]);
        }
        Ok(())
    }
}

/// A stage that fails at its fourth frame.
struct FailsAtThree;

impl neutral_handoff::FrameStage for FailsAtThree {
    fn process(&mut self, index: u64, _frame: &mut [u8]) -> Result<(), String> {
        if index == 3 {
            Err("unsupported: this stage cannot do frame 3".into())
        } else {
            Ok(())
        }
    }
}

#[test]
fn a_library_user_plugs_in_its_own_frame_stage() {
    let Some(case) = Case::new("fake_hook") else { return };
    // the in-process slot is a step of the chain: `@selftest-invert` names the step `pre1`, and a hook of that name takes its place
    let out = case.out("hooked.mkv");
    let mut request = neutral_handoff::Request::new(case.clip.to_string_lossy(), None);
    request.stages = vec!["@selftest-invert".into()];
    request.depth = Some(neutral_handoff::Depth::U8);
    request.lossless = true;
    request.output = Some(out.clone());
    let prepared = neutral_handoff::prepare(&request).unwrap_or_else(|e| panic!("{}", e.text_line()));
    let done = neutral_handoff::execute(&prepared, vec![("pre1".to_string(), Box::new(Black))]).unwrap_or_else(|e| panic!("{}", e.text_line()));
    assert_eq!((done.run.decoded, done.run.encoded), (12, 12));
    assert_eq!(done.run.stages.len(), 1);
    let frames = decode_raw(Path::new(&out), "rgba");
    assert_eq!(frames.len(), 12 * 160 * 96 * 4);
    assert!(frames.chunks_exact(4).all(|px| px == [0, 0, 0, 255]), "every pixel is the one the stage wrote");

    // a stage that fails names its frame, and nothing is left behind
    let failed = case.out("failed.mkv");
    request.output = Some(failed.clone());
    let prepared = neutral_handoff::prepare(&request).unwrap();
    let err = neutral_handoff::execute(&prepared, vec![("pre1".to_string(), Box::new(FailsAtThree))]).err().expect("the run fails");
    assert_eq!((err.code, err.stage.as_deref()), (neutral_handoff::Code::Unsupported, Some("pre1")));
    assert_eq!(err.message, "frame 3: unsupported: this stage cannot do frame 3");
    assert_eq!(err.exit_code(), 3);
    assert!(!Path::new(&failed).exists());
    assert!(partial_files(&case.dir).is_empty());
}
