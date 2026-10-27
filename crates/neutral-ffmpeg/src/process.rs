//! Child process plumbing shared by the decoder, encoder, filter and probe:
//! spawn with piped standard streams, drain standard error on a thread so that
//! no pipe can fill up and stall ffmpeg, and kill the child when the owner is
//! dropped.

use std::collections::VecDeque;
use std::ffi::{OsStr, OsString};
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use crate::error::{Error, Result};

/// Bytes kept from the start of standard error.
const HEAD_CAP: usize = 8 * 1024;
/// Bytes kept from the end of standard error.
const TAIL_CAP: usize = 56 * 1024;
/// How long a failed pipe operation waits for ffmpeg to finish its own diagnosis.
const GRACE: Duration = Duration::from_secs(5);
/// How long a process whose pipes are closed may take to exit before it is killed.
pub(crate) const EXIT_WAIT: Duration = Duration::from_secs(60);

/// Shorthand for building argument lists.
pub(crate) fn os<S: AsRef<OsStr>>(s: S) -> OsString {
    s.as_ref().to_os_string()
}

/// A path as an ffmpeg argument. A leading '-' would be read as an option, so
/// such relative paths get a `./` prefix.
pub(crate) fn path_arg(path: &OsStr) -> OsString {
    if path.to_string_lossy().starts_with('-') {
        let mut v = OsString::from("./");
        v.push(path);
        v
    } else {
        path.to_os_string()
    }
}

/// Per-platform spawn settings: on Windows the child gets no console window, so a
/// host application without a console does not flash one for every ffmpeg run.
#[cfg(windows)]
pub(crate) fn configure(cmd: &mut Command) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    cmd.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
pub(crate) fn configure(_cmd: &mut Command) {}

/// First and last part of what a child printed on standard error.
#[derive(Default)]
struct Capture {
    head: Vec<u8>,
    tail: VecDeque<u8>,
    omitted: u64,
}

impl Capture {
    fn push(&mut self, mut chunk: &[u8]) {
        if self.head.len() < HEAD_CAP {
            let take = (HEAD_CAP - self.head.len()).min(chunk.len());
            self.head.extend_from_slice(&chunk[..take]);
            chunk = &chunk[take..];
        }
        if chunk.is_empty() {
            return;
        }
        if chunk.len() >= TAIL_CAP {
            self.omitted += (self.tail.len() + chunk.len() - TAIL_CAP) as u64;
            self.tail.clear();
            self.tail.extend(&chunk[chunk.len() - TAIL_CAP..]);
        } else {
            let overflow = (self.tail.len() + chunk.len()).saturating_sub(TAIL_CAP);
            if overflow > 0 {
                self.tail.drain(..overflow);
                self.omitted += overflow as u64;
            }
            self.tail.extend(chunk);
        }
    }

    fn text(&self) -> String {
        let mut bytes = self.head.clone();
        if self.omitted > 0 {
            bytes.extend_from_slice(format!("\n[... {} bytes omitted ...]\n", self.omitted).as_bytes());
        }
        bytes.extend(self.tail.iter());
        String::from_utf8_lossy(&bytes).replace('\r', "")
    }
}

/// Reads standard error until the child closes it.
struct StderrDrain {
    shared: Arc<Mutex<Capture>>,
    handle: Option<JoinHandle<()>>,
}

impl StderrDrain {
    fn start(stderr: ChildStderr) -> io::Result<StderrDrain> {
        let shared = Arc::new(Mutex::new(Capture::default()));
        let sink = Arc::clone(&shared);
        let handle = thread::Builder::new().name("neutral-ffmpeg-stderr".into()).spawn(move || {
            let mut stderr = stderr;
            let mut buf = [0u8; 4096];
            loop {
                match stderr.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => sink.lock().unwrap_or_else(|e| e.into_inner()).push(&buf[..n]),
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
                    Err(_) => break,
                }
            }
        })?;
        Ok(StderrDrain { shared, handle: Some(handle) })
    }

    fn snapshot(&self) -> String {
        self.shared.lock().unwrap_or_else(|e| e.into_inner()).text()
    }

    fn join(&mut self) {
        if let Some(h) = self.handle.take() {
            let _ = h.join();
        }
    }
}

/// A running ffmpeg or ffprobe. Dropping it kills the process if it is still running.
pub(crate) struct Proc {
    tool: &'static str,
    context: String,
    child: Child,
    drain: Option<StderrDrain>,
    stderr_final: Option<String>,
    status: Option<ExitStatus>,
}

/// A started process together with the pipe ends that were requested.
pub(crate) struct Piped {
    pub proc: Proc,
    pub stdin: Option<ChildStdin>,
    pub stdout: Option<ChildStdout>,
}

/// Start `program` with piped standard error and optionally piped standard in and out.
/// Streams that are not piped are connected to the null device.
pub(crate) fn spawn(
    tool: &'static str,
    program: &Path,
    args: &[OsString],
    pipe_stdin: bool,
    pipe_stdout: bool,
    context: String,
) -> Result<Piped> {
    let mut cmd = Command::new(program);
    cmd.args(args);
    cmd.stdin(if pipe_stdin { Stdio::piped() } else { Stdio::null() });
    cmd.stdout(if pipe_stdout { Stdio::piped() } else { Stdio::null() });
    cmd.stderr(Stdio::piped());
    configure(&mut cmd);
    let mut child = cmd
        .spawn()
        .map_err(|source| Error::Spawn { program: PathBuf::from(program), source })?;
    let stdin = child.stdin.take();
    let stdout = child.stdout.take();
    let stderr = child.stderr.take().expect("standard error was piped");
    let drain = match StderrDrain::start(stderr) {
        Ok(d) => d,
        Err(e) => {
            let _ = child.kill();
            let _ = child.wait();
            return Err(Error::io("starting the standard error reader", e));
        }
    };
    Ok(Piped {
        proc: Proc { tool, context, child, drain: Some(drain), stderr_final: None, status: None },
        stdin,
        stdout,
    })
}

impl Proc {
    /// Everything captured from standard error so far.
    pub(crate) fn stderr_text(&self) -> String {
        if let Some(text) = &self.stderr_final {
            return text.clone();
        }
        self.drain.as_ref().map(|d| d.snapshot()).unwrap_or_default()
    }

    fn join_drain(&mut self) {
        if let Some(mut d) = self.drain.take() {
            d.join();
            self.stderr_final = Some(d.snapshot());
        }
    }

    fn reap(&mut self, status: ExitStatus) {
        self.status = Some(status);
        self.join_drain();
    }

    /// Error carrying ffmpeg's own words for an unsuccessful exit.
    pub(crate) fn error_for(&self, status: ExitStatus) -> Error {
        Error::Ffmpeg {
            tool: self.tool,
            context: self.context.clone(),
            code: status.code(),
            stderr: self.stderr_text(),
        }
    }

    /// Wait for the process to exit, at most `timeout`. On timeout the process is
    /// killed and [`Error::Timeout`] is returned with what it had printed.
    pub(crate) fn wait_within(&mut self, timeout: Duration) -> Result<ExitStatus> {
        if let Some(status) = self.status {
            return Ok(status);
        }
        let deadline = Instant::now() + timeout;
        let mut nap = Duration::from_millis(1);
        loop {
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    self.reap(status);
                    return Ok(status);
                }
                Ok(None) => {}
                Err(e) => return Err(Error::io("waiting for the child process", e)),
            }
            if Instant::now() >= deadline {
                self.kill();
                self.join_drain();
                return Err(Error::Timeout {
                    tool: self.tool,
                    context: self.context.clone(),
                    seconds: timeout.as_secs_f64(),
                    stderr: self.stderr_text(),
                });
            }
            thread::sleep(nap);
            nap = (nap * 2).min(Duration::from_millis(20));
        }
    }

    /// Wait for the exit (within `timeout`) and turn a failure status into an error.
    pub(crate) fn finish_within(&mut self, timeout: Duration) -> Result<()> {
        let status = self.wait_within(timeout)?;
        if status.success() {
            Ok(())
        } else {
            Err(self.error_for(status))
        }
    }

    /// [`Proc::finish_within`] with the usual grace period for a process whose
    /// pipes are already closed.
    pub(crate) fn finish(&mut self) -> Result<()> {
        self.finish_within(EXIT_WAIT)
    }

    /// Explain a failed pipe operation. ffmpeg usually closed the pipe because it
    /// hit an error and printed why, so wait for it to exit and report that text;
    /// fall back to the I/O error when the process ended cleanly or hangs.
    pub(crate) fn failure(&mut self, cause: io::Error, what: &'static str) -> Error {
        let deadline = Instant::now() + GRACE;
        loop {
            match self.child.try_wait() {
                Ok(Some(status)) => {
                    self.reap(status);
                    break;
                }
                Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
                Ok(None) => {
                    self.kill();
                    self.join_drain();
                    break;
                }
                Err(_) => break,
            }
        }
        match self.status {
            Some(status) if !status.success() => self.error_for(status),
            _ => Error::io(what, cause),
        }
    }

    /// The operating system's id of the process, so that a watchdog on another thread can end it.
    pub(crate) fn id(&self) -> u32 {
        self.child.id()
    }

    /// Terminate the process if it is still running and reap it.
    pub(crate) fn kill(&mut self) {
        if self.status.is_some() {
            return;
        }
        let _ = self.child.kill();
        if let Ok(status) = self.child.wait() {
            self.status = Some(status);
        }
    }
}

impl Drop for Proc {
    fn drop(&mut self) {
        self.kill();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_process_reports_the_id_of_its_child() {
        // the test binary itself is a program that is always there: `--list` prints and exits
        let me = std::env::current_exe().expect("test binary");
        let mut piped = spawn("test", &me, &[OsString::from("--list")], false, true, "listing".into()).expect("spawn");
        let id = piped.proc.id();
        assert!(id > 0);
        let mut sink = Vec::new();
        let _ = piped.stdout.take().expect("stdout").read_to_end(&mut sink);
        piped.proc.finish().expect("exit 0");
        assert_eq!(piped.proc.id(), id, "the id stays readable after the exit");
    }

    #[test]
    fn capture_keeps_head_and_tail() {
        let mut c = Capture::default();
        let big = vec![b'a'; HEAD_CAP];
        c.push(&big);
        c.push(&vec![b'b'; TAIL_CAP * 3]);
        c.push(b"final line\n");
        let text = c.text();
        assert!(text.starts_with("aaaa"));
        assert!(text.ends_with("final line\n"));
        assert!(text.contains("bytes omitted"));
        assert!(text.len() < HEAD_CAP + TAIL_CAP + 100);
    }

    #[test]
    fn capture_small_input_is_verbatim() {
        let mut c = Capture::default();
        c.push(b"Error opening input\r\n");
        c.push(b"No such file\n");
        assert_eq!(c.text(), "Error opening input\nNo such file\n");
    }

    #[test]
    fn leading_dash_paths_are_protected() {
        assert_eq!(path_arg(OsStr::new("-weird.mp4")), OsString::from("./-weird.mp4"));
        assert_eq!(path_arg(OsStr::new("normal.mp4")), OsString::from("normal.mp4"));
    }
}
