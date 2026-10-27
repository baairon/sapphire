//! Helpers shared by the integration tests.
//!
//! * `ffmpeg` and `real_vfx` find the tools and tell the test to skip, with a printed reason,
//!   when they are not there. `apply_py` finds twixtor's wrapper and a Python to run it.
//! * `scratch` hands out directories under `CARGO_TARGET_TMPDIR` only.
//! * `run` starts a program with a deadline, so that a hang in the code under test fails the test.
#![allow(dead_code)]

use std::ffi::OsString;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::OnceLock;
use std::thread;
use std::time::{Duration, Instant};

use neutral_ffmpeg::exec::run_capture;
use neutral_handoff::tools::{find_vfx_default, is_twixtor_checkout, process_env, twixtor_checkouts};

/// The sapphire binary built for these tests.
pub fn handoff_exe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_sapphire"))
}

/// The fake vfx built for these tests.
pub fn fake_vfx() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_fake-vfx"))
}

/// ffmpeg and ffprobe, found the way the binary under test finds them (flags aside), once per test binary.
pub fn media() -> Option<(PathBuf, PathBuf)> {
    static CACHE: OnceLock<Option<(PathBuf, PathBuf)>> = OnceLock::new();
    CACHE.get_or_init(|| neutral_handoff::tools::require_media(None, None, &process_env).ok().map(|m| (m.ffmpeg, m.ffprobe))).clone()
}

/// ffmpeg and ffprobe for a test, or `None` after printing why the test is skipped.
pub fn require_media(test: &str) -> Option<(PathBuf, PathBuf)> {
    let found = media();
    if found.is_none() {
        eprintln!("SKIPPED {}: ffmpeg and ffprobe were not found (set FFMPEG and FFPROBE, or put them on PATH)", test);
    }
    found
}

/// twixtor's `vfx` by the binary's own lookup (which honours `HANDOFF_TWIXTOR_DIR`), or `None`
/// after printing why the test is skipped (a failure under `HANDOFF_REQUIRE_PEERS=1`).
pub fn require_vfx(test: &str) -> Option<PathBuf> {
    match find_vfx_default(None) {
        Ok(p) => Some(p),
        Err(e) => {
            missing_peer(test, &format!("vfx was not found ({}); set VFX_PATH or build twixtor's `vfx`", e.message));
            None
        }
    }
}

/// Whether a missing peer fails the test (`HANDOFF_REQUIRE_PEERS=1`) instead of skipping it.
pub fn peers_required() -> bool {
    std::env::var("HANDOFF_REQUIRE_PEERS").map(|v| v.trim() == "1").unwrap_or(false)
}

/// Report a missing peer: a panic under `HANDOFF_REQUIRE_PEERS=1`, else a printed skip.
fn missing_peer(test: &str, why: &str) {
    if peers_required() {
        panic!("{}: {} (HANDOFF_REQUIRE_PEERS=1 makes a missing peer an error)", test, why);
    }
    eprintln!("SKIPPED {}: {}", test, why);
}

/// The twixtor checkout: `HANDOFF_TWIXTOR_DIR` when it is set, else the first sibling of this
/// repository found by its markers (`neutral_handoff::tools::twixtor_checkouts`, never by its
/// folder name) that holds `tools/apply.py`. `Err` says why none was found.
pub fn twixtor_dir() -> Result<PathBuf, String> {
    let explicit = std::env::var_os("HANDOFF_TWIXTOR_DIR").is_some_and(|v| !v.is_empty());
    let found = twixtor_checkouts(&process_env).into_iter().find(|d| d.join("tools").join("apply.py").is_file() && is_twixtor_checkout(d));
    found.ok_or_else(|| {
        if explicit {
            "HANDOFF_TWIXTOR_DIR has no tools/apply.py beside contracts/".to_string()
        } else {
            "no sibling of this repository has twixtor's markers and tools/apply.py; set HANDOFF_TWIXTOR_DIR".to_string()
        }
    })
}

/// twixtor's `tools/apply.py` and a Python to run it, or `None` after printing why the test is
/// skipped. The script is `HANDOFF_TWIXTOR_DIR`'s, else `APPLY_PY`, else a sibling's found by
/// [`twixtor_dir`]; under `HANDOFF_REQUIRE_PEERS=1` a missing script or Python fails the test.
pub fn require_apply_py(test: &str) -> Option<(String, PathBuf)> {
    let explicit_dir = std::env::var_os("HANDOFF_TWIXTOR_DIR").is_some_and(|v| !v.is_empty());
    let script = match std::env::var_os("APPLY_PY").filter(|v| !v.is_empty() && !explicit_dir) {
        Some(p) if Path::new(&p).is_file() => Ok(PathBuf::from(p)),
        Some(p) => Err(format!("APPLY_PY={} is not a file", PathBuf::from(p).display())),
        None => twixtor_dir().map(|d| d.join("tools").join("apply.py")),
    };
    let script = match script {
        Ok(s) => s,
        Err(why) => {
            missing_peer(test, &format!("twixtor's tools/apply.py was not found: {}", why));
            return None;
        }
    };
    for name in ["python", "python3", "py"] {
        let ok = Command::new(name).arg("-c").arg("print(1)").stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).status().map(|s| s.success()).unwrap_or(false);
        if ok {
            return Some((name.to_string(), script));
        }
    }
    missing_peer(test, "no Python was found to run apply.py");
    None
}

/// A fresh, empty directory under `CARGO_TARGET_TMPDIR`.
pub fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("sapphire").join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("create scratch directory");
    dir
}

/// What a finished program printed.
pub struct Ran {
    pub code: i32,
    pub stdout: Vec<u8>,
    pub stderr: String,
    pub seconds: f64,
}

impl Ran {
    pub fn out_text(&self) -> String {
        String::from_utf8_lossy(&self.stdout).into_owned()
    }
}

/// Run `program args` with extra environment variables, stdin closed (or fed `input`), and kill it
/// after `deadline`, failing the test.
pub fn run_env(program: &Path, args: &[String], env: &[(&str, &str)], input: Option<Vec<u8>>, deadline: Duration) -> Ran {
    let started = Instant::now();
    let mut cmd = Command::new(program);
    cmd.args(args).stdin(if input.is_some() { Stdio::piped() } else { Stdio::null() }).stdout(Stdio::piped()).stderr(Stdio::piped());
    for (k, v) in env {
        cmd.env(k, v);
    }
    let mut child = cmd.spawn().unwrap_or_else(|e| panic!("cannot start {}: {}", program.display(), e));
    if let Some(bytes) = input {
        let mut stdin = child.stdin.take().expect("piped");
        thread::spawn(move || {
            use std::io::Write;
            let _ = stdin.write_all(&bytes);
        });
    }
    let mut out = child.stdout.take().expect("piped");
    let mut err = child.stderr.take().expect("piped");
    let out_t = thread::spawn(move || {
        let mut v = Vec::new();
        let _ = out.read_to_end(&mut v);
        v
    });
    let err_t = thread::spawn(move || {
        let mut v = Vec::new();
        let _ = err.read_to_end(&mut v);
        v
    });
    let status = loop {
        match child.try_wait().expect("wait") {
            Some(s) => break s,
            None if started.elapsed() > deadline => {
                let _ = child.kill();
                let _ = child.wait();
                panic!("{} {:?} did not finish within {:?}", program.display(), args, deadline);
            }
            None => thread::sleep(Duration::from_millis(10)),
        }
    };
    Ran {
        code: status.code().unwrap_or(-1),
        stdout: out_t.join().unwrap(),
        stderr: String::from_utf8_lossy(&err_t.join().unwrap()).replace('\r', ""),
        seconds: started.elapsed().as_secs_f64(),
    }
}

/// [`run_env`] with the usual two minute deadline and no extra environment.
pub fn run(program: &Path, args: &[String]) -> Ran {
    run_env(program, args, &[], None, Duration::from_secs(120))
}

/// `&str` arguments as owned strings.
pub fn args(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

/// Make a test clip with lavfi: `frames` frames at `rate`, `w` x `h`, H.264 yuv420p, with a sine
/// audio track of `audio_seconds` when it is non-zero. Returns the path.
pub fn make_clip(dir: &Path, name: &str, w: u32, h: u32, frames: u32, rate: u32, audio_seconds: f64) -> PathBuf {
    let (ffmpeg, _) = media().expect("ffmpeg");
    let path = dir.join(name);
    let duration = frames as f64 / rate as f64;
    let mut a: Vec<OsString> = ["-v", "error", "-nostdin", "-y", "-f", "lavfi", "-i"].iter().map(OsString::from).collect();
    a.push(format!("testsrc2=size={}x{}:rate={}:duration={}", w, h, rate, duration).into());
    if audio_seconds > 0.0 {
        a.extend(["-f", "lavfi", "-i"].iter().map(OsString::from));
        a.push(format!("sine=frequency=440:duration={}", audio_seconds).into());
    }
    a.extend(["-frames:v", &frames.to_string(), "-c:v", "libx264", "-crf", "12", "-g", "5", "-pix_fmt", "yuv420p"].iter().map(OsString::from));
    if audio_seconds > 0.0 {
        a.extend(["-c:a", "aac", "-ac", "2"].iter().map(OsString::from));
    }
    a.push(path.clone().into());
    let out = run_capture(&ffmpeg, &a, Duration::from_secs(120), "ffmpeg", "making a test clip").expect("run ffmpeg");
    assert!(out.status.success(), "ffmpeg could not make the clip: {}", out.stderr_text());
    path
}

/// Decode `path` to raw frames of `pix_fmt` with ffmpeg's defaults (no matrix named).
pub fn decode_raw(path: &Path, pix_fmt: &str) -> Vec<u8> {
    let (ffmpeg, _) = media().expect("ffmpeg");
    let a: Vec<OsString> = ["-v", "error", "-nostdin", "-i"].iter().map(OsString::from).chain([path.as_os_str().to_os_string()]).chain(
        ["-an", "-f", "rawvideo", "-pix_fmt", pix_fmt, "-"].iter().map(OsString::from),
    ).collect();
    let out = run_capture(&ffmpeg, &a, Duration::from_secs(120), "ffmpeg", "decoding for comparison").expect("run ffmpeg");
    assert!(out.status.success(), "ffmpeg could not decode {}: {}", path.display(), out.stderr_text());
    out.stdout
}

/// The `[cmd]`-style decode of the wrapper for a stored clip: `-vf scale=in_color_matrix=M`.
pub fn decode_with_matrix(path: &Path, matrix: &str, pix_fmt: &str) -> Vec<u8> {
    let (ffmpeg, _) = media().expect("ffmpeg");
    let a: Vec<OsString> = ["-v", "error", "-nostdin", "-i"]
        .iter()
        .map(OsString::from)
        .chain([path.as_os_str().to_os_string()])
        .chain(["-an", "-vf"].iter().map(OsString::from))
        .chain([OsString::from(format!("scale=in_color_matrix={}", matrix))])
        .chain(["-pix_fmt", pix_fmt, "-f", "rawvideo", "-"].iter().map(OsString::from))
        .collect();
    let out = run_capture(&ffmpeg, &a, Duration::from_secs(120), "ffmpeg", "decoding for comparison").expect("run ffmpeg");
    assert!(out.status.success(), "ffmpeg could not decode {}: {}", path.display(), out.stderr_text());
    out.stdout
}

/// FNV-1a, 64 bits: a short fingerprint for the parity table.
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// Names of the files in `dir` that match the in-progress pattern of an output.
pub fn partial_files(dir: &Path) -> Vec<String> {
    std::fs::read_dir(dir)
        .map(|rd| rd.filter_map(|e| e.ok()).map(|e| e.file_name().to_string_lossy().into_owned()).filter(|n| n.contains(".partial-")).collect())
        .unwrap_or_default()
}
