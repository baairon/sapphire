//! Small process helpers: a `Command` that opens no console window, and short runs with a deadline.

use std::ffi::OsString;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

use neutral_ffmpeg::exec::run_capture;
use neutral_ffmpeg::Error as BridgeError;

/// A command for `program`. On Windows the child gets no console window, so a host without a
/// console does not flash one for every stage.
pub fn command(program: impl AsRef<std::ffi::OsStr>) -> Command {
    let mut cmd = Command::new(program);
    configure(&mut cmd);
    cmd
}

/// Per-platform spawn settings (see [`command`]).
#[cfg(windows)]
pub fn configure(cmd: &mut Command) {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    cmd.creation_flags(CREATE_NO_WINDOW);
}

/// Per-platform spawn settings (see [`command`]). On POSIX the child leads a process group of its
/// own, so [`kill_tree`] ends it together with what it started. Unproven until a CI run on POSIX.
#[cfg(unix)]
pub fn configure(cmd: &mut Command) {
    use std::os::unix::process::CommandExt;
    cmd.process_group(0);
}

/// Per-platform spawn settings (see [`command`]).
#[cfg(not(any(windows, unix)))]
pub fn configure(_cmd: &mut Command) {}

/// End the process `pid` and every process it started, without waiting for them.
///
/// Windows: `taskkill /T /F /PID pid` (the tree). POSIX: `kill -KILL -- -pid` (the process group
/// that [`configure`] made the child lead), then `kill -KILL pid` in case it left the group;
/// unproven until a CI run on POSIX. Returns whether the kill command reported success. The
/// caller still kills and reaps its own `Child`, so a missing `taskkill` or `kill` only loses
/// the grandchildren.
pub fn kill_tree(pid: u32) -> bool {
    if pid == 0 {
        return false;
    }
    let quiet = |mut cmd: Command| -> bool {
        configure_quiet(&mut cmd);
        cmd.stdin(std::process::Stdio::null()).stdout(std::process::Stdio::null()).stderr(std::process::Stdio::null());
        cmd.status().map(|s| s.success()).unwrap_or(false)
    };
    if cfg!(windows) {
        let mut cmd = Command::new("taskkill");
        cmd.args(["/T", "/F", "/PID", &pid.to_string()]);
        quiet(cmd)
    } else {
        let mut group = Command::new("kill");
        group.args(["-KILL", "--", &format!("-{}", pid)]);
        let ended = quiet(group);
        let mut single = Command::new("kill");
        single.args(["-KILL", &pid.to_string()]);
        quiet(single) || ended
    }
}

/// The spawn settings of a helper such as `taskkill`: no console window, no process group.
#[cfg(windows)]
fn configure_quiet(cmd: &mut Command) {
    configure(cmd);
}

/// The spawn settings of a helper such as `kill`.
#[cfg(not(windows))]
fn configure_quiet(_cmd: &mut Command) {}

/// What a short run printed and how it ended.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ran {
    /// The exit code; 127 when the program would not start, 124 when it timed out, -1 without a code.
    pub code: i32,
    /// Standard output as text.
    pub stdout: String,
    /// Standard error as text.
    pub stderr: String,
}

/// Run `program args` with standard input closed and wait at most `timeout`.
pub fn run_short(program: &Path, args: &[String], timeout: Duration) -> Ran {
    let args: Vec<OsString> = args.iter().map(OsString::from).collect();
    match run_capture(program, &args, timeout, "program", &format!("running {}", program.display())) {
        Ok(out) => Ran {
            code: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: out.stderr_text(),
        },
        Err(BridgeError::Timeout { .. }) => Ran { code: 124, stdout: String::new(), stderr: "process timed out".into() },
        Err(_) => Ran { code: 127, stdout: String::new(), stderr: "executable unavailable".into() },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kill_tree_ends_a_process_and_its_children() {
        // a shell that waits on a child of its own: the tree kill must end it, within a deadline
        let mut cmd = if cfg!(windows) {
            let mut c = command("cmd");
            c.args(["/C", "ping -n 30 127.0.0.1 >NUL"]);
            c
        } else {
            let mut c = command("sh");
            c.args(["-c", "sleep 30; true"]);
            c
        };
        let mut child = cmd
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("spawn");
        std::thread::sleep(Duration::from_millis(300));
        let started = std::time::Instant::now();
        let reported = kill_tree(child.id());
        let deadline = started + Duration::from_secs(20);
        loop {
            if child.try_wait().expect("wait").is_some() {
                break;
            }
            if std::time::Instant::now() >= deadline {
                let _ = child.kill();
                panic!("the process outlived the tree kill (kill command reported {})", reported);
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        assert!(started.elapsed() < Duration::from_secs(20));
    }

    #[test]
    fn kill_tree_of_no_process_reports_failure() {
        assert!(!kill_tree(0));
    }

    #[test]
    fn a_missing_program_is_127() {
        let ran = run_short(Path::new("definitely-not-a-program-xyz"), &[], Duration::from_secs(5));
        assert_eq!(ran.code, 127);
    }
}
