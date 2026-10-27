//! Run a program to completion with a hard deadline.
//!
//! Every short ffmpeg or ffprobe job of this crate (size measurement, probing)
//! goes through [`run_capture`]: the child's output is drained on threads, the
//! process is polled, and when the deadline passes it is killed and an
//! [`Error::Timeout`] comes back. A graph that can never emit its first frame
//! therefore ends in an error after the deadline instead of running (and
//! allocating) forever.

use std::ffi::OsString;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::error::{Error, Result};
use crate::process::configure;

/// Deadline used for measuring sizes and probing files.
pub const DEFAULT_DEADLINE: Duration = Duration::from_secs(60);

/// Everything a finished program printed.
#[derive(Debug)]
pub struct Captured {
    /// Exit status.
    pub status: ExitStatus,
    /// Standard output bytes.
    pub stdout: Vec<u8>,
    /// Standard error bytes.
    pub stderr: Vec<u8>,
}

impl Captured {
    /// Standard error as text with carriage returns removed.
    pub fn stderr_text(&self) -> String {
        String::from_utf8_lossy(&self.stderr).replace('\r', "")
    }
}

fn drain<R: Read + Send + 'static>(mut pipe: R) -> JoinHandle<Vec<u8>> {
    thread::spawn(move || {
        let mut out = Vec::new();
        let mut buf = [0u8; 16 * 1024];
        loop {
            match pipe.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => out.extend_from_slice(&buf[..n]),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(_) => break,
            }
        }
        out
    })
}

/// Start `program` with `args` (standard input closed), collect its output and
/// wait at most `timeout`. On timeout the process is killed and
/// [`Error::Timeout`] carries what it had printed on standard error.
///
/// `tool` and `context` only label errors ("ffprobe", "probe 'a.mkv'").
pub fn run_capture(
    program: &Path,
    args: &[OsString],
    timeout: Duration,
    tool: &'static str,
    context: &str,
) -> Result<Captured> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.stdin(Stdio::null()).stdout(Stdio::piped()).stderr(Stdio::piped());
    configure(&mut cmd);
    let mut child = cmd
        .spawn()
        .map_err(|source| Error::Spawn { program: PathBuf::from(program), source })?;
    let out_reader = drain(child.stdout.take().expect("standard output was piped"));
    let err_reader = drain(child.stderr.take().expect("standard error was piped"));
    let deadline = Instant::now() + timeout;
    let mut nap = Duration::from_millis(1);
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Some(status),
            Ok(None) => {}
            Err(e) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(Error::io("waiting for the child process", e));
            }
        }
        if Instant::now() >= deadline {
            break None;
        }
        thread::sleep(nap);
        nap = (nap * 2).min(Duration::from_millis(20));
    };
    match status {
        Some(status) => {
            let stdout = out_reader.join().unwrap_or_default();
            let stderr = err_reader.join().unwrap_or_default();
            Ok(Captured { status, stdout, stderr })
        }
        None => {
            let _ = child.kill();
            let _ = child.wait();
            let _ = out_reader.join();
            let stderr = err_reader.join().unwrap_or_default();
            Err(Error::Timeout {
                tool,
                context: context.to_string(),
                seconds: timeout.as_secs_f64(),
                stderr: String::from_utf8_lossy(&stderr).replace('\r', ""),
            })
        }
    }
}
