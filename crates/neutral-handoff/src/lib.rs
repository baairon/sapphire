//! Pipeline from ffmpeg through twixtor's `vfx` and back to ffmpeg, plus the stage commands of
//! this repository. Standard library only, no unsafe code.
//!
//! ```text
//! input file --ffprobe--> plan (size, rate, colour matrix, stream)
//!    |
//!    ffmpeg decode   (neutral_ffmpeg::Decoder, raw frames of the --depth format)
//!    |
//!    stages, in order: vfx apply EFFECT, --stage commands, in-process stages (@NAME)
//!    |
//!    ffmpeg encode   (neutral_ffmpeg::Encoder, same format in, file out)
//!    |
//! output file, verified against the input, then moved into place
//! ```
//!
//! The plan, the ffmpeg and `vfx` command lines, the checks and the output naming are those of
//! `tools/apply.py` in the twixtor repository, so for the same input, effect and flags the output
//! frames are equal. The contract between the tools is twixtor's raw frame format: depth 8
//! `rgba`, 16 `rgba64le`, 32 `gbrapf32le`, whole frames back to back, straight alpha
//! ([`format`]). A stage is a raw frame filter with the same command line shape as `vfx apply`;
//! this crate runs stages ([`pipeline`]) and is one ([`stage`], `sapphire stage apply`).
//!
//! Modules:
//! - [`error`]: [`Error`], its codes (`usage`, `unsupported`, `bad_input`, `stage_failed`,
//!   `stage_timeout`, `broken_pipe`, `partial_frame`, `tool_missing`) and exit codes.
//! - [`format`]: [`Depth`] and [`FrameSpec`], the raw frame format.
//! - [`rate`]: [`Rate`], an exact frame rate.
//! - [`media`]: [`Info`], what `ffprobe` says a file is.
//! - [`tools`]: finding ffmpeg, ffprobe, `vfx` and stage programs.
//! - [`vfx`]: `vfx list`, `vfx apply --check` and the contract version check.
//! - [`plan`]: [`Request`], [`prepare`], the commands and the `--dry-run` text.
//! - [`stage`]: the registry, `selftest-invert`, the stage manifest and the frame loop of `stage apply`.
//! - [`ffstage`]: the `ffmpeg-filter` stage (`graph`, `expect`) on `neutral_ffmpeg::Filter`.
//! - [`geometry`]: declared geometry, `--describe` answers and their chain.
//! - [`pipeline`]: threads, queues, the watchdog and the root cause of a failure.
//! - [`verify`]: the checks of a finished output.
//! - [`run`]: scratch folder, layer clips, the run itself, the output text and JSON.
//! - [`cli`] and [`args`]: the command line.
//! - [`paths`], [`pytext`], [`pyjson`], [`sys`]: output naming and the Python compatible text,
//!   JSON and process helpers the above use.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod args;
pub mod cli;
pub mod error;
pub mod ffstage;
pub mod format;
pub mod geometry;
pub mod media;
pub mod paths;
pub mod pipeline;
pub mod plan;
pub mod pyjson;
pub mod pytext;
pub mod rate;
pub mod run;
pub mod stage;
pub mod sys;
pub mod tools;
pub mod verify;
pub mod vfx;

pub use error::{Code, Contract, Error, Result};
pub use format::{Depth, FrameSpec};
pub use media::Info;
pub use plan::{prepare, Prepared, Request};
pub use rate::Rate;
pub use run::{dry_run, execute};
pub use stage::FrameStage;

/// The contract this crate speaks by default (`--contract frames/1|2`, [`Contract`]). Every
/// `--json` output carries the contract of its run, and a stage listing that reports a major
/// version other than 1 or 2 is refused.
pub const CONTRACT: &str = "frames/2";
