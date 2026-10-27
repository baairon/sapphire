//! Bridge to ffmpeg that runs it as a separate process and moves raw frames
//! over pipes.
//!
//! Nothing here links against libav*. The crate starts `ffmpeg` and `ffprobe`
//! as child processes, feeds or reads `rawvideo` on their standard streams and
//! reports ffmpeg's own error text when something goes wrong. That keeps the
//! rewrite free of any ffmpeg code or license, and any ffmpeg build the user
//! installs (or the plugin ships next to itself) can be used.
//!
//! Modules:
//! - [`pixfmt`]: table of ffmpeg pixel formats with exact rawvideo frame
//!   geometry, generated from the live binary.
//! - [`framebuf`]: [`FrameBuf`], one tightly packed frame, plus stride helpers.
//! - [`probe`]: `ffprobe` JSON reader built on the small parser in [`json`].
//! - [`decoder`]: [`Decoder`], file to frames.
//! - [`encoder`]: [`Encoder`], frames to file.
//! - [`filter`]: [`Filter`], frames through an ffmpeg filter graph.
//! - [`locate`]: finding the executables.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod decoder;
pub mod encoder;
pub mod error;
pub mod exec;
pub mod filter;
pub mod framebuf;
pub mod json;
pub mod locate;
pub mod pixfmt;
mod pixfmt_table;
pub mod probe;
mod process;
mod sizeprobe;

pub use decoder::{Decoder, DecoderBuilder};
pub use encoder::{EncodeStats, Encoder, EncoderBuilder};
pub use error::{Error, Result};
pub use filter::{Filter, FilterBuilder};
pub use framebuf::FrameBuf;
pub use locate::Binaries;
pub use pixfmt::{
    Endian, FormatInfo, FrameLayout, PixFmt, PlaneKind, PlaneLayout, PlaneSpec, Support,
};
pub use pixfmt_table::GENERATED_FROM as PIXFMT_TABLE_SOURCE;
pub use probe::{probe_file, Probe, Rational, StreamInfo, StreamKind};
