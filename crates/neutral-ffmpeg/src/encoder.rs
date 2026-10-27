//! [`Encoder`]: raw frames in, an encoded file out. The crate writes rawvideo to
//! ffmpeg's standard input and ffmpeg does the encoding and muxing.
//!
//! ```no_run
//! use neutral_ffmpeg::{Encoder, FrameBuf, PixFmt};
//!
//! let mut enc = Encoder::builder("out.mkv", PixFmt::RGB24, 640, 360)
//!     .fps(30, 1)
//!     .codec("ffv1")
//!     .spawn()?;
//! enc.write_frame(&FrameBuf::new(PixFmt::RGB24, 640, 360)?)?;
//! let stats = enc.finish()?;
//! assert_eq!(stats.frames, 1);
//! # Ok::<(), neutral_ffmpeg::Error>(())
//! ```

use std::ffi::{OsStr, OsString};
use std::io::Write;
use std::process::ChildStdin;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::framebuf::FrameBuf;
use crate::locate::Binaries;
use crate::pixfmt::PixFmt;
use crate::process::{os, path_arg, spawn, Proc};

/// Options for an [`Encoder`]. Create it with [`Encoder::builder`].
#[derive(Clone, Debug)]
pub struct EncoderBuilder {
    output: OsString,
    bins: Binaries,
    fmt: PixFmt,
    width: u32,
    height: u32,
    fps: (u32, u32),
    codec: Option<String>,
    format: Option<String>,
    out_pix_fmt: Option<PixFmt>,
    overwrite: bool,
    input_args: Vec<OsString>,
    output_args: Vec<OsString>,
    loglevel: String,
    finish_timeout: Duration,
}

/// How long [`Encoder::finish`] waits for ffmpeg to flush and close the file by default.
pub const DEFAULT_FINISH_TIMEOUT: Duration = Duration::from_secs(600);

impl EncoderBuilder {
    /// Longest [`Encoder::finish`] waits for ffmpeg after the input is closed
    /// (default ten minutes). ffmpeg is killed and [`Error::Timeout`] returned beyond it.
    pub fn finish_timeout(mut self, timeout: Duration) -> Self {
        self.finish_timeout = timeout;
        self
    }

    /// Where to find ffmpeg (default: environment, then `PATH`).
    pub fn binaries(mut self, bins: Binaries) -> Self {
        self.bins = bins;
        self
    }

    /// Frame rate of the raw input, as a fraction (default 25/1).
    pub fn fps(mut self, num: u32, den: u32) -> Self {
        self.fps = (num, den);
        self
    }

    /// Video encoder (`-c:v`), for example `ffv1`, `libx264`, `rawvideo`.
    pub fn codec(mut self, name: impl Into<String>) -> Self {
        self.codec = Some(name.into());
        self
    }

    /// Container format (`-f`), for example `matroska`. Otherwise ffmpeg picks it from the extension.
    pub fn format(mut self, name: impl Into<String>) -> Self {
        self.format = Some(name.into());
        self
    }

    /// Pixel format handed to the encoder (`-pix_fmt` on the output). Otherwise ffmpeg chooses.
    pub fn out_pix_fmt(mut self, fmt: PixFmt) -> Self {
        self.out_pix_fmt = Some(fmt);
        self
    }

    /// Replace an existing output file (`-y`, the default) or fail (`-n`).
    pub fn overwrite(mut self, on: bool) -> Self {
        self.overwrite = on;
        self
    }

    /// Options placed before the raw input, for example `["-color_range", "pc"]`.
    pub fn input_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.input_args.extend(args.into_iter().map(os));
        self
    }

    /// Output options passed through untouched, for example `["-crf", "18", "-preset", "slow"]`.
    pub fn output_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.output_args.extend(args.into_iter().map(os));
        self
    }

    /// ffmpeg `-loglevel` (default `error`). Whatever it prints is captured.
    pub fn loglevel(mut self, level: impl Into<String>) -> Self {
        self.loglevel = level.into();
        self
    }

    /// The ffmpeg arguments this configuration runs with (without the program name).
    pub fn command_line(&self) -> Vec<OsString> {
        let mut a: Vec<OsString> = vec![os("-hide_banner"), os("-loglevel"), os(&self.loglevel), os("-nostdin")];
        a.push(os(if self.overwrite { "-y" } else { "-n" }));
        a.extend([
            os("-f"),
            os("rawvideo"),
            os("-pix_fmt"),
            os(self.fmt.name()),
            os("-video_size"),
            os(format!("{}x{}", self.width, self.height)),
            os("-framerate"),
            os(format!("{}/{}", self.fps.0, self.fps.1)),
        ]);
        a.extend(self.input_args.iter().cloned());
        a.push(os("-i"));
        a.push(os("pipe:0"));
        if let Some(codec) = &self.codec {
            a.push(os("-c:v"));
            a.push(os(codec));
        }
        if let Some(fmt) = self.out_pix_fmt {
            a.push(os("-pix_fmt"));
            a.push(os(fmt.name()));
        }
        if let Some(format) = &self.format {
            a.push(os("-f"));
            a.push(os(format));
        }
        a.extend(self.output_args.iter().cloned());
        a.push(path_arg(&self.output));
        a
    }

    /// Start ffmpeg.
    pub fn spawn(self) -> Result<Encoder> {
        if self.fps.0 == 0 || self.fps.1 == 0 {
            return Err(Error::InvalidArgument("frame rate must be a positive fraction".into()));
        }
        let frame_bytes = self.fmt.frame_size(self.width, self.height)?;
        if let Some(out) = self.out_pix_fmt {
            if out.is_hwaccel() {
                return Err(Error::UnsupportedPixFmt {
                    name: out.name(),
                    reason: out.unsupported_reason().unwrap_or("unsupported"),
                });
            }
        }
        let ffmpeg = self.bins.ffmpeg_path()?;
        let args = self.command_line();
        let context = format!("encode to '{}'", self.output.to_string_lossy());
        let piped = spawn("ffmpeg", &ffmpeg, &args, true, false, context)?;
        Ok(Encoder {
            proc: piped.proc,
            stdin: piped.stdin,
            fmt: self.fmt,
            width: self.width,
            height: self.height,
            frame_bytes,
            frames: 0,
            args,
            finish_timeout: self.finish_timeout,
        })
    }
}

/// What an [`Encoder`] did, returned by [`Encoder::finish`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodeStats {
    /// Frames written to ffmpeg.
    pub frames: u64,
    /// Raw bytes written to ffmpeg.
    pub bytes: u64,
    /// Whatever ffmpeg printed at the chosen log level (warnings, for example).
    pub stderr: String,
}

/// A running ffmpeg that reads raw frames from its standard input.
///
/// Call [`Encoder::finish`] to close the input and wait for ffmpeg to finish
/// the file. Dropping the encoder without finishing stops ffmpeg, which leaves
/// an incomplete output file.
pub struct Encoder {
    proc: Proc,
    stdin: Option<ChildStdin>,
    fmt: PixFmt,
    width: u32,
    height: u32,
    frame_bytes: usize,
    frames: u64,
    args: Vec<OsString>,
    finish_timeout: Duration,
}

impl Encoder {
    /// Start configuring an encoder that writes `output` from raw frames of
    /// format `fmt` and size `width` x `height`.
    pub fn builder(output: impl AsRef<OsStr>, fmt: PixFmt, width: u32, height: u32) -> EncoderBuilder {
        EncoderBuilder {
            output: output.as_ref().to_os_string(),
            bins: Binaries::new(),
            fmt,
            width,
            height,
            fps: (25, 1),
            codec: None,
            format: None,
            out_pix_fmt: None,
            overwrite: true,
            input_args: Vec::new(),
            output_args: Vec::new(),
            loglevel: "error".to_string(),
            finish_timeout: DEFAULT_FINISH_TIMEOUT,
        }
    }

    /// Input pixel format.
    pub fn pix_fmt(&self) -> PixFmt {
        self.fmt
    }

    /// Input width in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Input height in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Bytes per input frame.
    pub fn frame_bytes(&self) -> usize {
        self.frame_bytes
    }

    /// Frames written so far.
    pub fn frames_written(&self) -> u64 {
        self.frames
    }

    /// What ffmpeg has printed on standard error so far.
    pub fn stderr(&self) -> String {
        self.proc.stderr_text()
    }

    /// The arguments ffmpeg was started with.
    pub fn command_line(&self) -> &[OsString] {
        &self.args
    }

    /// Write one frame. It must have the format and size the encoder was set up with.
    pub fn write_frame(&mut self, frame: &FrameBuf) -> Result<()> {
        if frame.fmt() != self.fmt || frame.width() != self.width || frame.height() != self.height {
            return Err(Error::FrameMismatch(format!(
                "encoder expects {} {}x{}, frame is {} {}x{}",
                self.fmt,
                self.width,
                self.height,
                frame.fmt(),
                frame.width(),
                frame.height()
            )));
        }
        self.write_bytes(frame.as_bytes())
    }

    /// Write one frame given as bytes in the tightly packed layout.
    pub fn write_raw_frame(&mut self, bytes: &[u8]) -> Result<()> {
        if bytes.len() != self.frame_bytes {
            return Err(Error::BufferSize { expected: self.frame_bytes, actual: bytes.len() });
        }
        self.write_bytes(bytes)
    }

    fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| Error::InvalidArgument("the encoder input is already closed".into()))?;
        match stdin.write_all(bytes) {
            Ok(()) => {
                self.frames += 1;
                Ok(())
            }
            Err(e) => {
                self.stdin = None;
                Err(self.proc.failure(e, "writing frames to ffmpeg"))
            }
        }
    }

    /// Close the input, let ffmpeg finish the file and report how it went.
    pub fn finish(mut self) -> Result<EncodeStats> {
        drop(self.stdin.take());
        self.proc.finish_within(self.finish_timeout)?;
        Ok(EncodeStats {
            frames: self.frames,
            bytes: self.frames * self.frame_bytes as u64,
            stderr: self.proc.stderr_text(),
        })
    }
}

impl Encoder {
    /// The process id of the running ffmpeg. A watchdog that owns no handle to this value can
    /// end a stalled ffmpeg by this id (the reading or writing thread then sees the pipe close).
    pub fn id(&self) -> u32 {
        self.proc.id()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(args: &[OsString]) -> Vec<String> {
        args.iter().map(|a| a.to_string_lossy().into_owned()).collect()
    }

    #[test]
    fn default_command_line() {
        let b = Encoder::builder("out.mkv", PixFmt::RGB24, 64, 48);
        assert_eq!(
            text(&b.command_line()).join(" "),
            "-hide_banner -loglevel error -nostdin -y -f rawvideo -pix_fmt rgb24 -video_size 64x48 \
             -framerate 25/1 -i pipe:0 out.mkv"
        );
    }

    #[test]
    fn codec_format_and_passthrough_options() {
        let b = Encoder::builder("out.bin", PixFmt::YUV420P, 32, 16)
            .fps(30000, 1001)
            .codec("ffv1")
            .format("matroska")
            .out_pix_fmt(PixFmt::YUV444P)
            .overwrite(false)
            .input_args(["-color_range", "pc"])
            .output_args(["-level", "3", "-g", "1"])
            .loglevel("warning");
        let args = text(&b.command_line());
        let at = |s: &str| args.iter().position(|a| a == s).unwrap_or_else(|| panic!("{} missing", s));
        assert!(args.contains(&"-n".to_string()));
        assert!(!args.contains(&"-y".to_string()));
        assert_eq!(args[at("-framerate") + 1], "30000/1001");
        assert!(at("-color_range") < at("-i"));
        assert_eq!(args[at("-c:v") + 1], "ffv1");
        assert!(at("-i") < at("-c:v"));
        let last_pix_fmt = args.iter().rposition(|a| a == "-pix_fmt").unwrap();
        assert_eq!(args[last_pix_fmt + 1], "yuv444p");
        assert!(last_pix_fmt > at("-i"), "output pix_fmt follows the input");
        let last_f = args.iter().rposition(|a| a == "-f").unwrap();
        assert_eq!(args[last_f + 1], "matroska");
        assert!(at("-level") > last_f);
        assert_eq!(args.last().unwrap(), "out.bin");
    }

    #[test]
    fn invalid_configuration_is_refused_before_starting() {
        let err = Encoder::builder("o.mkv", PixFmt::RGB24, 0, 4).spawn().err().unwrap();
        assert!(matches!(err, Error::InvalidSize { .. }), "{:?}", err);
        let err = Encoder::builder("o.mkv", PixFmt::VAAPI, 4, 4).spawn().err().unwrap();
        assert!(matches!(err, Error::UnsupportedPixFmt { .. }), "{:?}", err);
        let err = Encoder::builder("o.mkv", PixFmt::RGB24, 4, 4).fps(0, 1).spawn().err().unwrap();
        assert!(matches!(err, Error::InvalidArgument(_)), "{:?}", err);
    }
}
