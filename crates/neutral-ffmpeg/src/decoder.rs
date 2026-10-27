//! [`Decoder`]: ffmpeg decodes a file, the crate reads raw frames from its standard output.
//!
//! ```no_run
//! use neutral_ffmpeg::{Decoder, PixFmt};
//!
//! let decoder = Decoder::builder("clip.mp4")
//!     .pix_fmt(PixFmt::BGRA)
//!     .size(1280, 720)
//!     .spawn()?;
//! for frame in decoder {
//!     let frame = frame?;
//!     assert_eq!(frame.width(), 1280);
//! }
//! # Ok::<(), neutral_ffmpeg::Error>(())
//! ```

use std::ffi::{OsStr, OsString};
use std::io::Read;
use std::process::ChildStdout;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::framebuf::FrameBuf;
use crate::locate::Binaries;
use crate::pixfmt::PixFmt;
use crate::process::{os, path_arg, spawn, Proc};
use crate::sizeprobe::detect_size;

/// Input read by the size-measuring dry run, in seconds.
const DRY_RUN_INPUT_SECONDS: u32 = 2;

/// Seconds with microsecond precision, the way ffmpeg options like `-ss` read them.
fn seconds(d: Duration) -> String {
    format!("{}.{:06}", d.as_secs(), d.subsec_micros())
}

/// Options for a [`Decoder`]. Create it with [`Decoder::builder`].
#[derive(Clone, Debug)]
pub struct DecoderBuilder {
    input: OsString,
    bins: Binaries,
    pix_fmt: PixFmt,
    size: Option<(u32, u32)>,
    scale_flags: Option<String>,
    fps: Option<(u32, u32)>,
    start: Option<Duration>,
    duration: Option<Duration>,
    max_frames: Option<u64>,
    video_stream: u32,
    filter: Option<String>,
    input_args: Vec<OsString>,
    output_args: Vec<OsString>,
    loglevel: String,
    timing_passthrough: bool,
}

impl DecoderBuilder {
    fn new(input: &OsStr) -> DecoderBuilder {
        DecoderBuilder {
            input: input.to_os_string(),
            bins: Binaries::new(),
            pix_fmt: PixFmt::RGB24,
            size: None,
            scale_flags: None,
            fps: None,
            start: None,
            duration: None,
            max_frames: None,
            video_stream: 0,
            filter: None,
            input_args: Vec::new(),
            output_args: Vec::new(),
            loglevel: "error".to_string(),
            timing_passthrough: true,
        }
    }

    /// Where to find ffmpeg (default: environment, then `PATH`).
    pub fn binaries(mut self, bins: Binaries) -> Self {
        self.bins = bins;
        self
    }

    /// Pixel format of the frames that are read (default `rgb24`).
    pub fn pix_fmt(mut self, fmt: PixFmt) -> Self {
        self.pix_fmt = fmt;
        self
    }

    /// Scale every frame to `width` x `height`. Without it the size of the filtered
    /// video is measured with a one frame dry run.
    pub fn size(mut self, width: u32, height: u32) -> Self {
        self.size = Some((width, height));
        self
    }

    /// Extra `scale` filter flags such as `lanczos` (only used with [`DecoderBuilder::size`]).
    pub fn scale_flags(mut self, flags: impl Into<String>) -> Self {
        self.scale_flags = Some(flags.into());
        self
    }

    /// Convert to a constant frame rate of `num/den` with the `fps` filter.
    pub fn fps(mut self, num: u32, den: u32) -> Self {
        self.fps = Some((num, den));
        self
    }

    /// Seek to this position before decoding (`-ss` as an input option).
    pub fn start(mut self, at: Duration) -> Self {
        self.start = Some(at);
        self
    }

    /// Stop after this much output time (`-t`).
    pub fn duration(mut self, length: Duration) -> Self {
        self.duration = Some(length);
        self
    }

    /// Stop after this many frames (`-frames:v`).
    pub fn max_frames(mut self, n: u64) -> Self {
        self.max_frames = Some(n);
        self
    }

    /// Pick the n-th video stream (default 0).
    pub fn video_stream(mut self, n: u32) -> Self {
        self.video_stream = n;
        self
    }

    /// A linear `-vf` chain applied before the optional `fps` and `scale` steps.
    pub fn filter(mut self, chain: impl Into<String>) -> Self {
        self.filter = Some(chain.into());
        self
    }

    /// Options placed before the input, for example `["-f", "lavfi"]` or `["-threads", "2"]`.
    pub fn input_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.input_args.extend(args.into_iter().map(os));
        self
    }

    /// Options placed after the input and before the output.
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

    /// Keep one output frame per decoded frame (`-fps_mode passthrough`, the
    /// default). Turn it off for ffmpeg older than 5.1, or to let ffmpeg
    /// duplicate and drop frames to a constant rate.
    pub fn timing_passthrough(mut self, on: bool) -> Self {
        self.timing_passthrough = on;
        self
    }

    fn chain(&self, with_scale: bool) -> Option<String> {
        let mut parts: Vec<String> = Vec::new();
        if let Some(f) = &self.filter {
            if !f.trim().is_empty() {
                parts.push(f.clone());
            }
        }
        if let Some((n, d)) = self.fps {
            parts.push(format!("fps={}/{}", n, d));
        }
        if with_scale {
            if let Some((w, h)) = self.size {
                match &self.scale_flags {
                    Some(flags) => parts.push(format!("scale={}:{}:flags={}", w, h, flags)),
                    None => parts.push(format!("scale={}:{}", w, h)),
                }
            }
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(","))
        }
    }

    /// Arguments common to the dry run and the real run, up to the filter chain.
    ///
    /// The dry run (`dry`) reads at most `DRY_RUN_INPUT_SECONDS` of input (`-t` as an
    /// input option), so that buffering filters such as `reverse` see an end of
    /// input even when the source is very long or endless.
    fn common_args(&self, dry: bool) -> Vec<OsString> {
        let mut a: Vec<OsString> = Vec::new();
        if let Some(start) = self.start {
            a.push(os("-ss"));
            a.push(os(seconds(start)));
        }
        a.extend(self.input_args.iter().cloned());
        if dry {
            a.push(os("-t"));
            a.push(os(DRY_RUN_INPUT_SECONDS.to_string()));
        }
        a.push(os("-i"));
        a.push(path_arg(&self.input));
        if let (false, Some(d)) = (dry, self.duration) {
            a.push(os("-t"));
            a.push(os(seconds(d)));
        }
        a.push(os("-map"));
        a.push(os(format!("0:v:{}", self.video_stream)));
        for flag in ["-an", "-sn", "-dn"] {
            a.push(os(flag));
        }
        a
    }

    /// The ffmpeg arguments this configuration runs with (without the program name).
    pub fn command_line(&self) -> Vec<OsString> {
        let mut a: Vec<OsString> = vec![os("-hide_banner"), os("-loglevel"), os(&self.loglevel), os("-nostdin")];
        a.extend(self.common_args(false));
        if let Some(chain) = self.chain(true) {
            a.push(os("-vf"));
            a.push(os(chain));
        }
        if let Some(n) = self.max_frames {
            a.push(os("-frames:v"));
            a.push(os(n.to_string()));
        }
        if self.timing_passthrough {
            a.push(os("-fps_mode"));
            a.push(os("passthrough"));
        }
        a.push(os("-pix_fmt"));
        a.push(os(self.pix_fmt.name()));
        a.extend(self.output_args.iter().cloned());
        a.push(os("-f"));
        a.push(os("rawvideo"));
        a.push(os("pipe:1"));
        a
    }

    /// Start ffmpeg. The frame size is measured first when [`DecoderBuilder::size`] was not set.
    pub fn spawn(self) -> Result<Decoder> {
        if !self.pix_fmt.is_supported() {
            return Err(Error::UnsupportedPixFmt {
                name: self.pix_fmt.name(),
                reason: self.pix_fmt.unsupported_reason().unwrap_or("unsupported"),
            });
        }
        let ffmpeg = self.bins.ffmpeg_path()?;
        let (width, height) = match self.size {
            Some(size) => size,
            None => {
                let mut dry: Vec<OsString> = self.common_args(true);
                let chain = match self.chain(false) {
                    Some(c) => format!("{},showinfo", c),
                    None => "showinfo".to_string(),
                };
                dry.push(os("-vf"));
                dry.push(os(chain));
                detect_size(&ffmpeg, &dry, &format!("'{}'", self.input.to_string_lossy()))?
            }
        };
        let frame_bytes = self.pix_fmt.frame_size(width, height)?;
        // A measured size needs no scale step; an explicit size adds one.
        let args = self.command_line();
        let context = format!("decode '{}'", self.input.to_string_lossy());
        let piped = spawn("ffmpeg", &ffmpeg, &args, false, true, context)?;
        Ok(Decoder {
            proc: piped.proc,
            stdout: piped.stdout,
            fmt: self.pix_fmt,
            width,
            height,
            frame_bytes,
            frames: 0,
            done: false,
            args,
        })
    }
}

/// A running ffmpeg that decodes one video stream to raw frames.
///
/// `Decoder` is an iterator over `Result<FrameBuf>`. A failure of ffmpeg itself
/// (missing file, unsupported codec, bad option) comes out as the last item,
/// carrying ffmpeg's error text. Dropping the decoder stops ffmpeg.
pub struct Decoder {
    proc: Proc,
    stdout: Option<ChildStdout>,
    fmt: PixFmt,
    width: u32,
    height: u32,
    frame_bytes: usize,
    frames: u64,
    done: bool,
    args: Vec<OsString>,
}

impl Decoder {
    /// Start configuring a decoder for `input` (a path, URL or lavfi source with
    /// matching [`DecoderBuilder::input_args`]).
    pub fn builder(input: impl AsRef<OsStr>) -> DecoderBuilder {
        DecoderBuilder::new(input.as_ref())
    }

    /// Shorthand: decode `input` to `fmt` frames of `width` x `height`.
    pub fn open(input: impl AsRef<OsStr>, fmt: PixFmt, width: u32, height: u32) -> Result<Decoder> {
        Decoder::builder(input).pix_fmt(fmt).size(width, height).spawn()
    }

    /// Pixel format of the frames.
    pub fn pix_fmt(&self) -> PixFmt {
        self.fmt
    }

    /// Frame width in pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Frame height in pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Bytes per frame.
    pub fn frame_bytes(&self) -> usize {
        self.frame_bytes
    }

    /// Frames delivered so far.
    pub fn frames_read(&self) -> u64 {
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

    /// Read the next frame, or `None` at the end of the stream.
    pub fn read_frame(&mut self) -> Result<Option<FrameBuf>> {
        let mut buf = vec![0u8; self.frame_bytes];
        if self.fill(&mut buf)? {
            Ok(Some(FrameBuf::from_vec(self.fmt, self.width, self.height, buf)?))
        } else {
            Ok(None)
        }
    }

    /// Read the next frame into an existing buffer. Returns `false` at the end of the stream.
    pub fn read_frame_into(&mut self, frame: &mut FrameBuf) -> Result<bool> {
        if frame.fmt() != self.fmt || frame.width() != self.width || frame.height() != self.height {
            return Err(Error::FrameMismatch(format!(
                "decoder produces {} {}x{}, buffer is {} {}x{}",
                self.fmt,
                self.width,
                self.height,
                frame.fmt(),
                frame.width(),
                frame.height()
            )));
        }
        self.fill(frame.as_bytes_mut())
    }

    /// Stop ffmpeg now and discard anything it has not delivered.
    pub fn stop(mut self) {
        self.done = true;
        self.proc.kill();
    }

    fn fill(&mut self, buf: &mut [u8]) -> Result<bool> {
        if self.done {
            return Ok(false);
        }
        let stdout = match self.stdout.as_mut() {
            Some(s) => s,
            None => return Ok(false),
        };
        let mut got = 0usize;
        while got < buf.len() {
            match stdout.read(&mut buf[got..]) {
                Ok(0) => break,
                Ok(n) => got += n,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => {
                    self.done = true;
                    return Err(self.proc.failure(e, "reading frames from ffmpeg"));
                }
            }
        }
        if got == buf.len() {
            self.frames += 1;
            return Ok(true);
        }
        // End of the stream. ffmpeg's own verdict is more useful than a short read.
        self.done = true;
        self.stdout = None;
        self.proc.finish()?;
        if got == 0 {
            Ok(false)
        } else {
            Err(Error::TruncatedFrame { got, expected: buf.len() })
        }
    }
}

impl Iterator for Decoder {
    type Item = Result<FrameBuf>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.read_frame() {
            Ok(Some(frame)) => Some(Ok(frame)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        }
    }
}

impl Decoder {
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
        let b = Decoder::builder("in.mp4").pix_fmt(PixFmt::BGRA).size(640, 360);
        let args = text(&b.command_line());
        let joined = args.join(" ");
        assert_eq!(
            joined,
            "-hide_banner -loglevel error -nostdin -i in.mp4 -map 0:v:0 -an -sn -dn -vf scale=640:360 \
             -fps_mode passthrough -pix_fmt bgra -f rawvideo pipe:1"
        );
    }

    #[test]
    fn options_land_in_the_right_places() {
        let b = Decoder::builder("in.mkv")
            .pix_fmt(PixFmt::GRAY)
            .size(32, 16)
            .scale_flags("neighbor")
            .fps(30000, 1001)
            .filter("hflip")
            .start(Duration::from_millis(1500))
            .duration(Duration::from_secs(2))
            .max_frames(9)
            .video_stream(1)
            .input_args(["-f", "matroska"])
            .output_args(["-threads", "1"])
            .loglevel("warning")
            .timing_passthrough(false);
        let args = text(&b.command_line());
        let at = |s: &str| args.iter().position(|a| a == s).unwrap_or_else(|| panic!("{} missing in {:?}", s, args));
        assert!(at("-ss") < at("-f"), "seek and input options come before -i");
        assert!(at("-f") < at("-i"));
        assert_eq!(args[at("-ss") + 1], "1.500000");
        assert_eq!(args[at("-t") + 1], "2.000000");
        assert!(at("-i") < at("-t"));
        assert_eq!(args[at("-map") + 1], "0:v:1");
        assert_eq!(args[at("-vf") + 1], "hflip,fps=30000/1001,scale=32:16:flags=neighbor");
        assert_eq!(args[at("-frames:v") + 1], "9");
        assert!(!args.iter().any(|a| a == "-fps_mode"));
        assert!(at("-threads") > at("-pix_fmt"));
        assert_eq!(args[at("-loglevel") + 1], "warning");
        assert_eq!(args.last().unwrap(), "pipe:1");
    }

    #[test]
    fn leading_dash_input_is_protected() {
        let b = Decoder::builder("-odd.mp4");
        let args = text(&b.command_line());
        let i = args.iter().position(|a| a == "-i").unwrap();
        assert_eq!(args[i + 1], "./-odd.mp4");
    }

    #[test]
    fn unsupported_format_is_refused_before_starting() {
        let err = Decoder::builder("in.mp4").pix_fmt(PixFmt::VAAPI).size(16, 16).spawn().err().unwrap();
        assert!(matches!(err, Error::UnsupportedPixFmt { .. }), "{:?}", err);
    }
}
