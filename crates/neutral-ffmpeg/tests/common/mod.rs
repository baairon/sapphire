//! Helpers shared by the integration tests.
//!
//! * `require` finds ffmpeg and ffprobe (explicit option, environment, PATH) and
//!   tells the test to skip, with a printed reason, when they are not there.
//! * `scratch` hands out directories under `CARGO_TARGET_TMPDIR` only.
#![allow(dead_code)]

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Duration;

use neutral_ffmpeg::exec::run_capture;
use neutral_ffmpeg::Binaries;

/// Both tools, resolved once per test binary.
pub fn binaries() -> Option<Binaries> {
    static CACHE: OnceLock<Option<Binaries>> = OnceLock::new();
    CACHE
        .get_or_init(|| {
            let base = Binaries::new();
            let ffmpeg = base.ffmpeg_path().ok()?;
            let ffprobe = base.ffprobe_path().ok()?;
            Some(Binaries::new().with_ffmpeg(ffmpeg).with_ffprobe(ffprobe))
        })
        .clone()
}

/// Binaries for a test, or `None` after printing why the test is skipped.
pub fn require(test: &str) -> Option<Binaries> {
    let found = binaries();
    if found.is_none() {
        eprintln!(
            "SKIPPED {}: ffmpeg and ffprobe were not found (set FFMPEG and FFPROBE, or put them on PATH)",
            test
        );
    }
    found
}

/// A fresh, empty directory under `CARGO_TARGET_TMPDIR`.
pub fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("neutral-ffmpeg").join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create scratch directory");
    dir
}

/// Result of a direct ffmpeg run made by the test harness itself.
pub struct Run {
    pub ok: bool,
    pub code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: String,
}

/// Longest any single ffmpeg run of the tests may take before it is killed.
pub const RUN_DEADLINE: Duration = Duration::from_secs(120);

/// Refuse to start ffmpeg on a generated source that has no end. A lavfi source
/// (`-f lavfi`) must carry a duration (`d=` or `duration=` in the source, or
/// `-t`/`-to` among the options); a frame count alone is not enough, because a
/// buffering filter would still see an endless input.
pub fn assert_bounded(args: &[&str]) {
    let lavfi = args.windows(2).any(|w| w[0] == "-f" && w[1] == "lavfi");
    if !lavfi {
        return;
    }
    let option_bound = args.iter().any(|a| matches!(*a, "-t" | "-to"));
    let source_bound = args.iter().any(|a| a.contains(":d=") || a.contains("duration="));
    assert!(
        option_bound || source_bound,
        "refusing to run ffmpeg on an unbounded lavfi source: {:?}",
        args
    );
}

/// Run ffmpeg directly (not through the crate) with the given arguments. The run
/// has a hard deadline of `RUN_DEADLINE`; the arguments must not name an endless source.
pub fn ffmpeg(bins: &Binaries, args: &[&str]) -> Run {
    assert_bounded(args);
    let exe = bins.ffmpeg_path().expect("ffmpeg path");
    let mut full: Vec<OsString> = ["-hide_banner", "-loglevel", "error", "-nostdin"].iter().map(OsString::from).collect();
    full.extend(args.iter().map(OsString::from));
    let out = run_capture(&exe, &full, RUN_DEADLINE, "ffmpeg", "a test run").expect("run ffmpeg");
    Run {
        ok: out.status.success(),
        code: out.status.code(),
        stdout: out.stdout,
        stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
    }
}

/// The reference command for frame sizes:
/// `ffmpeg -f lavfi -i color=c=black:s=WxH:d=1 -frames:v 1 -pix_fmt FMT -f rawvideo -`
/// (the one second duration only makes the source finite; the first frame is the same).
pub fn reference_frame(bins: &Binaries, pix_fmt: &str, w: u32, h: u32) -> Run {
    let input = format!("color=c=black:s={}x{}:d=1", w, h);
    ffmpeg(
        bins,
        &["-f", "lavfi", "-i", &input, "-frames:v", "1", "-pix_fmt", pix_fmt, "-f", "rawvideo", "-"],
    )
}

/// Deterministic bytes for building synthetic frames.
pub fn pattern(len: usize, seed: u32) -> Vec<u8> {
    let mut x = seed.wrapping_mul(2654435761).wrapping_add(12345);
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            (x >> 8) as u8
        })
        .collect()
}
