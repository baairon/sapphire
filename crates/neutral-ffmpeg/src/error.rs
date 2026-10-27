//! Error type shared by every module of the crate.

use std::fmt;
use std::io;
use std::path::PathBuf;

/// Result alias used across the crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Everything that can go wrong while bridging to ffmpeg.
#[derive(Debug)]
pub enum Error {
    /// The ffmpeg or ffprobe executable could not be located.
    ToolNotFound {
        /// `"ffmpeg"` or `"ffprobe"`.
        tool: &'static str,
        /// One line per place that was searched, in search order.
        tried: Vec<String>,
    },
    /// The executable exists but the operating system refused to start it.
    Spawn {
        /// Path that was launched.
        program: PathBuf,
        /// The underlying OS error.
        source: io::Error,
    },
    /// A pipe or file operation failed.
    Io {
        /// What the crate was doing.
        context: &'static str,
        /// The underlying OS error.
        source: io::Error,
    },
    /// ffmpeg or ffprobe ran and reported a failure. `stderr` carries its own
    /// words (the first part and the last part of what it printed).
    Ffmpeg {
        /// `"ffmpeg"` or `"ffprobe"`.
        tool: &'static str,
        /// Short description of the job ("decode 'a.mp4'", "encode to 'b.mkv'").
        context: String,
        /// Process exit code when the platform reports one.
        code: Option<i32>,
        /// Captured standard error text.
        stderr: String,
    },
    /// A child process did not finish before its deadline and was killed.
    Timeout {
        /// `"ffmpeg"`, `"ffprobe"` or another program the crate ran.
        tool: &'static str,
        /// Short description of the job.
        context: String,
        /// The deadline that was exceeded, in seconds.
        seconds: f64,
        /// What the process had printed on standard error.
        stderr: String,
    },
    /// The output size of a filter graph or decode could not be measured from the
    /// short dry run that the crate makes. Give the size explicitly.
    SizeUnknown {
        /// What was being measured.
        context: String,
    },
    /// A pixel format name that is not in the table.
    UnknownPixFmt(String),
    /// A pixel format that ffmpeg cannot emit or read as rawvideo.
    UnsupportedPixFmt {
        /// Canonical format name.
        name: &'static str,
        /// Why the table marks it unsupported.
        reason: &'static str,
    },
    /// Width or height that ffmpeg would refuse, or a frame too large to address.
    InvalidSize {
        /// Requested width.
        width: u32,
        /// Requested height.
        height: u32,
        /// What is wrong with it.
        reason: &'static str,
    },
    /// A byte buffer whose length does not match the frame it should hold.
    BufferSize {
        /// Length the frame layout requires.
        expected: usize,
        /// Length that was supplied.
        actual: usize,
    },
    /// A frame whose format or size differs from what the pipeline was set up for.
    FrameMismatch(String),
    /// The pipe ended in the middle of a frame.
    TruncatedFrame {
        /// Bytes of the partial frame that arrived.
        got: usize,
        /// Bytes a whole frame needs.
        expected: usize,
    },
    /// Malformed JSON.
    Json {
        /// Byte offset of the problem.
        offset: usize,
        /// What was expected or found.
        message: String,
    },
    /// ffprobe output that parsed as JSON but did not have the expected shape.
    Probe(String),
    /// A builder was given an unusable option.
    InvalidArgument(String),
}

impl Error {
    /// The text ffmpeg or ffprobe printed, when this error carries it.
    pub fn stderr(&self) -> Option<&str> {
        match self {
            Error::Ffmpeg { stderr, .. } | Error::Timeout { stderr, .. } => Some(stderr),
            _ => None,
        }
    }

    pub(crate) fn io(context: &'static str, source: io::Error) -> Error {
        Error::Io { context, source }
    }
}

/// Keep at most the last `max_lines` non-empty lines of `text`.
fn last_lines(text: &str, max_lines: usize) -> String {
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    if lines.len() <= max_lines {
        return lines.join("\n");
    }
    let skipped = lines.len() - max_lines;
    format!("[{} earlier lines omitted]\n{}", skipped, lines[skipped..].join("\n"))
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ToolNotFound { tool, tried } => {
                write!(f, "{} not found", tool)?;
                for line in tried {
                    write!(f, "\n  - {}", line)?;
                }
                Ok(())
            }
            Error::Spawn { program, source } => {
                write!(f, "cannot start '{}': {}", program.display(), source)
            }
            Error::Io { context, source } => write!(f, "i/o error while {}: {}", context, source),
            Error::Ffmpeg { tool, context, code, stderr } => {
                match code {
                    Some(c) => write!(f, "{} failed (exit code {}) while {}", tool, c, context)?,
                    None => write!(f, "{} failed while {}", tool, context)?,
                }
                let tail = last_lines(stderr, 12);
                if !tail.is_empty() {
                    write!(f, ":\n{}", tail)?;
                }
                Ok(())
            }
            Error::Timeout { tool, context, seconds, stderr } => {
                write!(f, "{} did not finish within {} s while {} and was killed", tool, seconds, context)?;
                let tail = last_lines(stderr, 12);
                if !tail.is_empty() {
                    write!(f, ":\n{}", tail)?;
                }
                Ok(())
            }
            Error::SizeUnknown { context } => write!(
                f,
                "could not measure the output size of {}: a short test run (about 2 seconds of input) produced no \
                 frame; give the output size explicitly (FilterBuilder::output_size or DecoderBuilder::size)",
                context
            ),
            Error::UnknownPixFmt(name) => write!(f, "unknown pixel format '{}'", name),
            Error::UnsupportedPixFmt { name, reason } => {
                write!(f, "pixel format '{}' is not supported as rawvideo: {}", name, reason)
            }
            Error::InvalidSize { width, height, reason } => {
                write!(f, "invalid frame size {}x{}: {}", width, height, reason)
            }
            Error::BufferSize { expected, actual } => {
                write!(f, "buffer has {} bytes but the frame needs {}", actual, expected)
            }
            Error::FrameMismatch(msg) => write!(f, "frame does not match the pipeline: {}", msg),
            Error::TruncatedFrame { got, expected } => {
                write!(f, "stream ended inside a frame ({} of {} bytes)", got, expected)
            }
            Error::Json { offset, message } => write!(f, "invalid JSON at byte {}: {}", offset, message),
            Error::Probe(msg) => write!(f, "unexpected ffprobe output: {}", msg),
            Error::InvalidArgument(msg) => write!(f, "invalid argument: {}", msg),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Spawn { source, .. } | Error::Io { source, .. } => Some(source),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(source: io::Error) -> Error {
        Error::Io { context: "talking to the child process", source }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ffmpeg_error_shows_tail_of_stderr() {
        let mut text = String::new();
        for i in 0..40 {
            text.push_str(&format!("line {}\n", i));
        }
        let e = Error::Ffmpeg {
            tool: "ffmpeg",
            context: "decode 'x'".to_string(),
            code: Some(1),
            stderr: text,
        };
        let shown = e.to_string();
        assert!(shown.contains("line 39"));
        assert!(!shown.contains("line 3\n"));
        assert!(shown.contains("earlier lines omitted"));
        assert_eq!(e.stderr().unwrap().lines().count(), 40);
    }

    #[test]
    fn display_has_no_dashes_other_than_hyphens() {
        let e = Error::TruncatedFrame { got: 3, expected: 9 };
        assert!(!e.to_string().contains('\u{2014}'));
        assert!(!e.to_string().contains('\u{2013}'));
    }
}
