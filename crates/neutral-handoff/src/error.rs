//! [`Error`]: how a run of the pipeline or of a stage fails, as one value with a machine readable code.
//!
//! The same object is printed in the three places a failure shows:
//!
//! - text mode: one line `error: <code>: <message>` on standard error;
//! - `--json` mode: `{"contract": "frames/2", "ok": false, "error": {"code", "stage", "exit_code", "message", "stderr_tail"}}`
//!   (`contracts/error.schema.json`), where `exit_code` is the exit code of the child process the
//!   error is about (`null` when it did not end by itself or no child was involved; under frames/1
//!   `stage apply` reports its own), `message` is one line and `stderr_tail` at most 5 lines;
//! - the process exit code of this tool: under frames/2 (the default) the table below, under
//!   `--contract frames/1` 1 for every failure and 2 for a command line the parser rejects
//!   ([`Contract::exit_for`]).
//!
//! | Code | frames/2 exit | Meaning |
//! |---|---|---|
//! | `usage` | 2 | a flag or argument the tool rejects |
//! | `unsupported` | 3 | something this tool or a stage refuses to do: unknown effect or stage, a bad parameter value, HDR input, `--lossless` at depth 32, an odd side on the lossy path, a stage that reports another contract version, or a stage that exits 3 |
//! | `bad_input` | 4 | the input is not usable: no such file, no video stream, a layer that does not decode, an output path that exists, a stage that exits 4 |
//! | `stage_failed` | 1 | a stage exited with another non-zero code, returned more frames than it was given, or the finished output did not verify |
//! | `stage_timeout` | 5 | a stage produced and consumed nothing for `--stage-timeout` seconds and was killed |
//! | `broken_pipe` | 4 | a stage closed its input or its output before the frames were through |
//! | `partial_frame` | 4 | a stream ended in the middle of a frame |
//! | `tool_missing` | 6 | ffmpeg, ffprobe, `vfx` or a stage program was not found or would not start |
//!
//! Exit 130 (interrupted) is not produced: catching Ctrl-C needs unsafe code or a dependency,
//! and the crate allows neither.

use std::fmt;

use crate::pyjson::Out;

/// Exit code of a successful run.
pub const EXIT_OK: i32 = 0;

/// Lines of a child's standard error an error document carries at most (`contracts/error.schema.json`).
pub const TAIL_MAX: usize = 5;

/// The contract a run speaks (`--contract frames/1|2`, `docs/handoff.md#contract-versions`).
///
/// frames/2, the default of this tool, exits with the normative codes of the table above and
/// carries `role`, `index` and `geometry` in its reports. frames/1 keeps the old wrapper exits:
/// 0 done, 1 any refusal or failure, 2 a command line the argument parser rejects.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Contract {
    /// `frames/1`.
    V1,
    /// `frames/2`.
    #[default]
    V2,
}

impl Contract {
    /// Both contracts, oldest first.
    pub const ALL: [Contract; 2] = [Contract::V1, Contract::V2];

    /// `frames/1` or `frames/2`.
    pub fn as_str(self) -> &'static str {
        match self {
            Contract::V1 => "frames/1",
            Contract::V2 => "frames/2",
        }
    }

    /// The value of `--contract`: `frames/1` or `frames/2`, nothing else.
    pub fn parse(text: &str) -> Option<Contract> {
        Contract::ALL.into_iter().find(|c| c.as_str() == text.trim())
    }

    /// The process exit code for `error` under this contract.
    pub fn exit_for(self, error: &Error) -> i32 {
        match self {
            Contract::V2 => error.code.exit_code(),
            Contract::V1 if error.code == Code::Usage && error.stage.as_deref() == Some("args") => 2,
            Contract::V1 => 1,
        }
    }

    /// The value of `--contract` among `args` (`--contract X` or `--contract=X`), read before the
    /// command line is parsed so that even a parse failure is reported under it. A missing or
    /// unknown value gives the default.
    pub fn scan(args: &[String]) -> Contract {
        let mut it = args.iter();
        while let Some(a) = it.next() {
            if a == "--" {
                break;
            }
            let value = match a.strip_prefix("--contract") {
                Some("") => it.next().map(|s| s.as_str()),
                Some(rest) => rest.strip_prefix('='),
                None => None,
            };
            if let Some(found) = value.and_then(Contract::parse) {
                return found;
            }
        }
        Contract::default()
    }
}

impl fmt::Display for Contract {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Why a run failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Code {
    /// A flag or argument the tool rejects.
    Usage,
    /// Something the tool or a stage refuses to do.
    Unsupported,
    /// The input is not usable.
    BadInput,
    /// A stage failed in a way that has no more specific code.
    StageFailed,
    /// A stage went silent for longer than `--stage-timeout`.
    StageTimeout,
    /// A stage closed a pipe before the frames were through.
    BrokenPipe,
    /// A stream ended in the middle of a frame.
    PartialFrame,
    /// A program was not found or would not start.
    ToolMissing,
}

impl Code {
    /// Every code, in the order of the table above.
    pub const ALL: [Code; 8] = [
        Code::Usage,
        Code::Unsupported,
        Code::BadInput,
        Code::StageFailed,
        Code::StageTimeout,
        Code::BrokenPipe,
        Code::PartialFrame,
        Code::ToolMissing,
    ];

    /// The name used in `--json` and in the text line.
    pub fn as_str(self) -> &'static str {
        match self {
            Code::Usage => "usage",
            Code::Unsupported => "unsupported",
            Code::BadInput => "bad_input",
            Code::StageFailed => "stage_failed",
            Code::StageTimeout => "stage_timeout",
            Code::BrokenPipe => "broken_pipe",
            Code::PartialFrame => "partial_frame",
            Code::ToolMissing => "tool_missing",
        }
    }

    /// The process exit code.
    pub fn exit_code(self) -> i32 {
        match self {
            Code::Usage => 2,
            Code::Unsupported => 3,
            Code::BadInput | Code::BrokenPipe | Code::PartialFrame => 4,
            Code::StageFailed => 1,
            Code::StageTimeout => 5,
            Code::ToolMissing => 6,
        }
    }

    /// The code for a stage process that exited with `exit`: the vfx codes 2, 3 and 4 keep their meaning.
    pub fn from_stage_exit(exit: i32) -> Code {
        match exit {
            2 => Code::Usage,
            3 => Code::Unsupported,
            4 => Code::BadInput,
            _ => Code::StageFailed,
        }
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A failure of the pipeline, of a stage, or of a check before either ran.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    /// What kind of failure.
    pub code: Code,
    /// Where it happened: `args`, `probe`, `plan`, `decode`, `vfx`, `stage1`, `stage2`, ..., `encode`, `verify`.
    pub stage: Option<String>,
    /// One line saying what happened.
    pub message: String,
    /// The last lines the failing process printed on standard error.
    pub stderr_tail: Vec<String>,
    /// The exit code of the process the error is about, when it ended by itself.
    pub process_exit: Option<i32>,
}

impl Error {
    /// An error of kind `code`.
    pub fn new(code: Code, message: impl Into<String>) -> Error {
        Error { code, stage: None, message: message.into(), stderr_tail: Vec::new(), process_exit: None }
    }

    /// A `usage` error.
    pub fn usage(message: impl Into<String>) -> Error {
        Error::new(Code::Usage, message).at("args")
    }

    /// An `unsupported` error.
    pub fn unsupported(message: impl Into<String>) -> Error {
        Error::new(Code::Unsupported, message)
    }

    /// A `bad_input` error.
    pub fn bad_input(message: impl Into<String>) -> Error {
        Error::new(Code::BadInput, message)
    }

    /// A `tool_missing` error.
    pub fn tool_missing(message: impl Into<String>) -> Error {
        Error::new(Code::ToolMissing, message)
    }

    /// A `stage_failed` error.
    pub fn stage_failed(message: impl Into<String>) -> Error {
        Error::new(Code::StageFailed, message)
    }

    /// The error for stage process `stage` that exited with `exit` after printing `tail`.
    pub fn stage_exit(stage: &str, exit: Option<i32>, tail: Vec<String>) -> Error {
        match exit {
            Some(n) => Error::new(Code::from_stage_exit(n), format!("{} exited with code {}", stage, n)),
            None => Error::new(Code::StageFailed, format!("{} was ended by a signal", stage)),
        }
        .at(stage)
        .with_exit(exit)
        .with_tail(tail)
    }

    /// Record the exit code of the process the error is about.
    pub fn with_exit(mut self, exit: Option<i32>) -> Error {
        self.process_exit = exit;
        self
    }

    /// Name the stage.
    pub fn at(mut self, stage: impl Into<String>) -> Error {
        self.stage = Some(stage.into());
        self
    }

    /// Attach the last lines of the failing process's standard error.
    pub fn with_tail(mut self, tail: Vec<String>) -> Error {
        self.stderr_tail = tail;
        self
    }

    /// The process exit code for this error.
    pub fn exit_code(&self) -> i32 {
        self.code.exit_code()
    }

    /// The text mode line: `error: <code>: <message>`, with the last line the process printed
    /// appended when the message does not already contain it.
    pub fn text_line(&self) -> String {
        let mut line = format!("error: {}: {}", self.code, self.message);
        if let Some(last) = self.stderr_tail.iter().rev().find(|l| !l.trim().is_empty()) {
            let shortened = last.trim().replace(": error: ", ": ");
            if !self.message.contains(last.trim()) && !self.message.contains(&shortened) {
                line.push_str(": ");
                line.push_str(last.trim());
            }
        }
        line
    }

    /// The error object of `--json` mode. `exit_code` is the exit code of the process the error
    /// is about: the failing stage's when it ended by itself, else `own_exit` (a stage reports its
    /// own exit code there; the wrapper passes `None`).
    pub fn to_object(&self, own_exit: Option<i32>) -> Out {
        let message: String = self.message.split(['\r', '\n']).map(str::trim).filter(|l| !l.is_empty()).collect::<Vec<_>>().join(" ");
        let message = if message.is_empty() { self.code.as_str().to_string() } else { message };
        let skip = self.stderr_tail.len().saturating_sub(TAIL_MAX);
        Out::Map(vec![
            ("code".into(), Out::Str(self.code.as_str().into())),
            ("stage".into(), self.stage.clone().filter(|s| !s.is_empty()).map(Out::Str).unwrap_or(Out::Null)),
            ("exit_code".into(), Out::opt_int(self.process_exit.or(own_exit).map(|c| c as i64))),
            ("message".into(), Out::Str(message)),
            ("stderr_tail".into(), Out::List(self.stderr_tail[skip..].iter().cloned().map(Out::Str).collect())),
        ])
    }

    /// The whole `--json` failure document under frames/1 (see [`Error::document`]).
    pub fn to_document(&self, own_exit: Option<i32>) -> Out {
        self.document(Contract::V1, own_exit)
    }

    /// The whole `--json` failure document (`contracts/error.schema.json`). Under frames/1
    /// `own_exit` (the exit code of a `stage apply` that fails by itself) fills `exit_code`; under
    /// frames/2 `exit_code` belongs to a failing child process only, so it is null otherwise.
    pub fn document(&self, contract: Contract, own_exit: Option<i32>) -> Out {
        let own = if contract == Contract::V1 { own_exit } else { None };
        Out::Map(vec![
            ("contract".into(), Out::Str(contract.as_str().into())),
            ("ok".into(), Out::Bool(false)),
            ("error".into(), self.to_object(own)),
        ])
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {}

/// Result alias of this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// The last `count` lines of `text`, trailing blanks removed from each (what `apply.py` shows of a log).
pub fn tail_lines(text: &str, count: usize) -> Vec<String> {
    let lines: Vec<&str> = text.trim().lines().collect();
    let start = lines.len().saturating_sub(count);
    lines[start..].iter().map(|l| l.trim_end().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exit_codes_follow_the_vfx_codes() {
        assert_eq!(Code::Usage.exit_code(), 2);
        assert_eq!(Code::Unsupported.exit_code(), 3);
        assert_eq!(Code::BadInput.exit_code(), 4);
        assert_eq!(Code::PartialFrame.exit_code(), 4);
        assert_eq!(Code::BrokenPipe.exit_code(), 4);
        assert_eq!(Code::StageFailed.exit_code(), 1);
        assert_eq!(Code::StageTimeout.exit_code(), 5);
        assert_eq!(Code::ToolMissing.exit_code(), 6);
    }

    #[test]
    fn code_names_are_the_contract_names() {
        let names: Vec<&str> = Code::ALL.iter().map(|c| c.as_str()).collect();
        assert_eq!(
            names,
            ["usage", "unsupported", "bad_input", "stage_failed", "stage_timeout", "broken_pipe", "partial_frame", "tool_missing"]
        );
    }

    #[test]
    fn a_stage_exit_keeps_its_meaning() {
        assert_eq!(Code::from_stage_exit(2), Code::Usage);
        assert_eq!(Code::from_stage_exit(3), Code::Unsupported);
        assert_eq!(Code::from_stage_exit(4), Code::BadInput);
        assert_eq!(Code::from_stage_exit(1), Code::StageFailed);
        assert_eq!(Code::from_stage_exit(101), Code::StageFailed);
        let e = Error::stage_exit("vfx", Some(3), vec!["vfx: error: frame 0: unsupported".into()]);
        assert_eq!((e.code, e.exit_code(), e.stage.as_deref()), (Code::Unsupported, 3, Some("vfx")));
        assert_eq!(e.message, "vfx exited with code 3");
    }

    #[test]
    fn text_line_is_one_line_with_the_last_stderr_line() {
        let e = Error::stage_exit("stage1", Some(1), vec!["first".into(), "boom".into()]);
        assert_eq!(e.text_line(), "error: stage_failed: stage1 exited with code 1: boom");
        assert!(!e.text_line().contains('\n'));
        let plain = Error::usage("expected K=V");
        assert_eq!(plain.text_line(), "error: usage: expected K=V");
    }

    #[test]
    fn json_object_has_the_five_fields_in_order() {
        let e = Error::bad_input("no such file: x.mp4").at("probe");
        assert_eq!(
            e.to_object(None).dumps(),
            r#"{"code": "bad_input", "stage": "probe", "exit_code": null, "message": "no such file: x.mp4", "stderr_tail": []}"#
        );
        assert_eq!(
            e.to_document(Some(4)).dumps(),
            r#"{"contract": "frames/1", "ok": false, "error": {"code": "bad_input", "stage": "probe", "exit_code": 4, "message": "no such file: x.mp4", "stderr_tail": []}}"#
        );
        let none = Error::new(Code::StageFailed, "x");
        assert!(none.to_object(None).dumps().contains(r#""stage": null"#));
        let stage = Error::stage_exit("vfx", Some(3), vec![]);
        assert!(stage.to_object(None).dumps().contains(r#""exit_code": 3"#), "the stage's own code, whatever the tool exits with");
    }

    #[test]
    fn tail_keeps_the_last_lines() {
        assert_eq!(tail_lines("a\nb\nc\nd  \n\n", 3), vec!["b", "c", "d"]);
        assert_eq!(tail_lines("", 3), Vec::<String>::new());
        assert_eq!(tail_lines("only", 3), vec!["only"]);
    }

    #[test]
    fn frames_2_exits_follow_the_code_table_and_frames_1_keeps_0_1_2() {
        let parse = Error::usage("unrecognized arguments: --nope");
        let stage_usage = Error::stage_exit("pre1", Some(2), vec![]);
        for (e, v2, v1) in [
            (&parse, 2, 2),
            (&stage_usage, 2, 1),
            (&Error::unsupported("x"), 3, 1),
            (&Error::bad_input("x"), 4, 1),
            (&Error::new(Code::BrokenPipe, "x"), 4, 1),
            (&Error::new(Code::PartialFrame, "x"), 4, 1),
            (&Error::new(Code::StageTimeout, "x"), 5, 1),
            (&Error::tool_missing("x"), 6, 1),
            (&Error::stage_failed("x"), 1, 1),
        ] {
            assert_eq!(Contract::V2.exit_for(e), v2, "{}", e.code);
            assert_eq!(Contract::V1.exit_for(e), v1, "{}", e.code);
        }
        assert_eq!(Contract::default(), Contract::V2);
    }

    #[test]
    fn the_contract_is_scanned_before_parsing() {
        let a = |list: &[&str]| list.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert_eq!(Contract::scan(&a(&["x", "--contract", "frames/1"])), Contract::V1);
        assert_eq!(Contract::scan(&a(&["x", "--contract=frames/1"])), Contract::V1);
        assert_eq!(Contract::scan(&a(&["x", "--contract", "frames/3"])), Contract::V2);
        assert_eq!(Contract::scan(&a(&["x"])), Contract::V2);
        assert_eq!(Contract::parse("frames/2"), Some(Contract::V2));
        assert_eq!(Contract::parse("2"), None);
    }

    #[test]
    fn a_frames_2_document_is_one_line_with_at_most_five_tail_lines_and_no_own_exit() {
        let tail: Vec<String> = (0..8).map(|i| format!("line {}", i)).collect();
        let e = Error::new(Code::BadInput, "first\nsecond").at("pre1").with_tail(tail);
        let doc = e.document(Contract::V2, Some(4)).dumps();
        assert!(doc.starts_with(r#"{"contract": "frames/2", "ok": false"#), "{}", doc);
        assert!(doc.contains(r#""message": "first second""#), "{}", doc);
        assert!(doc.contains(r#""exit_code": null"#), "{}", doc);
        assert!(doc.contains(r#""stderr_tail": ["line 3", "line 4", "line 5", "line 6", "line 7"]"#), "{}", doc);
        assert!(e.document(Contract::V1, Some(4)).dumps().contains(r#""exit_code": 4"#));
    }

    #[test]
    fn no_text_has_a_dash_other_than_a_hyphen() {
        for code in Code::ALL {
            let text = Error::new(code, "x").text_line();
            assert!(!text.contains('\u{2014}') && !text.contains('\u{2013}'));
        }
    }
}
