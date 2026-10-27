//! [`Filter`]: raw frames in, raw frames out, through an ffmpeg `-vf` filter graph.
//!
//! The crate writes rawvideo to ffmpeg's standard input and reads the filtered
//! frames from its standard output. A thread always drains the output, so the
//! two pipes cannot deadlock each other even when the graph holds frames back
//! (deinterlacers, frame rate converters) or produces several frames per input.
//!
//! ```no_run
//! use neutral_ffmpeg::{Filter, FrameBuf, PixFmt};
//!
//! let mut filter = Filter::builder(PixFmt::RGB24, 640, 360)
//!     .graph("scale=320:180,format=gray")
//!     .out_pix_fmt(PixFmt::GRAY)
//!     .spawn()?;
//! filter.push(&FrameBuf::new(PixFmt::RGB24, 640, 360)?)?;
//! let frames = filter.finish()?;
//! assert_eq!(frames[0].width(), 320);
//! # Ok::<(), neutral_ffmpeg::Error>(())
//! ```

use std::ffi::{OsStr, OsString};
use std::io::{Read, Write};
use std::process::{ChildStdin, ChildStdout};
use std::sync::mpsc::{channel, Receiver, RecvTimeoutError, Sender, TryRecvError};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use crate::error::{Error, Result};
use crate::framebuf::FrameBuf;
use crate::locate::Binaries;
use crate::pixfmt::PixFmt;
use crate::process::{os, spawn, Proc, EXIT_WAIT};
use crate::sizeprobe::detect_size;

/// How the size of the filtered frames is determined.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SizeMode {
    /// Measure it with a one frame dry run of the graph (default).
    Auto,
    /// Append `scale=W:H` so the graph's output is exactly this size.
    Fixed(u32, u32),
    /// Trust that the graph keeps the input size and do not measure.
    Unchanged,
}

/// Options for a [`Filter`]. Create it with [`Filter::builder`].
#[derive(Clone, Debug)]
pub struct FilterBuilder {
    bins: Binaries,
    in_fmt: PixFmt,
    in_w: u32,
    in_h: u32,
    fps: (u32, u32),
    graph: String,
    out_fmt: Option<PixFmt>,
    size_mode: SizeMode,
    input_args: Vec<OsString>,
    output_args: Vec<OsString>,
    loglevel: String,
    timing_passthrough: bool,
}

impl FilterBuilder {
    /// Where to find ffmpeg (default: environment, then `PATH`).
    pub fn binaries(mut self, bins: Binaries) -> Self {
        self.bins = bins;
        self
    }

    /// The `-vf` filter graph (empty means no filtering, only conversion).
    pub fn graph(mut self, graph: impl Into<String>) -> Self {
        self.graph = graph.into();
        self
    }

    /// Pixel format of the frames that come back (default: the input format).
    pub fn out_pix_fmt(mut self, fmt: PixFmt) -> Self {
        self.out_fmt = Some(fmt);
        self
    }

    /// Frame rate attached to the raw input, as a fraction (default 25/1).
    /// Filters that look at time (`fps`, `setpts`) see this rate.
    pub fn fps(mut self, num: u32, den: u32) -> Self {
        self.fps = (num, den);
        self
    }

    /// Force the output size: `scale=w:h` is appended to the graph. Skips the dry run.
    pub fn output_size(mut self, width: u32, height: u32) -> Self {
        self.size_mode = SizeMode::Fixed(width, height);
        self
    }

    /// Declare that the graph keeps the input size. Skips the dry run; if the
    /// graph does change the size the frames cannot be told apart correctly.
    pub fn size_unchanged(mut self) -> Self {
        self.size_mode = SizeMode::Unchanged;
        self
    }

    /// Options placed before the raw input.
    pub fn input_args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.input_args.extend(args.into_iter().map(os));
        self
    }

    /// Output options passed through untouched.
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

    /// Keep one output frame per filtered frame (`-fps_mode passthrough`, the
    /// default). Turn it off for ffmpeg older than 5.1.
    pub fn timing_passthrough(mut self, on: bool) -> Self {
        self.timing_passthrough = on;
        self
    }

    fn chain(&self, with_scale: bool) -> Option<String> {
        let mut parts: Vec<String> = Vec::new();
        if !self.graph.trim().is_empty() {
            parts.push(self.graph.trim().to_string());
        }
        if with_scale {
            if let SizeMode::Fixed(w, h) = self.size_mode {
                parts.push(format!("scale={}:{}", w, h));
            }
        }
        if parts.is_empty() {
            None
        } else {
            Some(parts.join(","))
        }
    }

    fn rate_text(&self) -> String {
        format!("{}/{}", self.fps.0, self.fps.1)
    }

    /// The ffmpeg arguments this configuration runs with (without the program name).
    pub fn command_line(&self) -> Vec<OsString> {
        let mut a: Vec<OsString> = vec![os("-hide_banner"), os("-loglevel"), os(&self.loglevel), os("-nostdin")];
        a.extend([
            os("-f"),
            os("rawvideo"),
            os("-pix_fmt"),
            os(self.in_fmt.name()),
            os("-video_size"),
            os(format!("{}x{}", self.in_w, self.in_h)),
            os("-framerate"),
            os(self.rate_text()),
        ]);
        a.extend(self.input_args.iter().cloned());
        a.push(os("-i"));
        a.push(os("pipe:0"));
        for flag in ["-an", "-sn", "-dn"] {
            a.push(os(flag));
        }
        if let Some(chain) = self.chain(true) {
            a.push(os("-vf"));
            a.push(os(chain));
        }
        if self.timing_passthrough {
            a.push(os("-fps_mode"));
            a.push(os("passthrough"));
        }
        a.push(os("-pix_fmt"));
        a.push(os(self.out_fmt.unwrap_or(self.in_fmt).name()));
        a.extend(self.output_args.iter().cloned());
        a.push(os("-f"));
        a.push(os("rawvideo"));
        a.push(os("pipe:1"));
        a
    }

    /// Start ffmpeg. The output size is measured first unless it was given.
    pub fn spawn(self) -> Result<Filter> {
        if self.fps.0 == 0 || self.fps.1 == 0 {
            return Err(Error::InvalidArgument("frame rate must be a positive fraction".into()));
        }
        let out_fmt = self.out_fmt.unwrap_or(self.in_fmt);
        let in_bytes = self.in_fmt.frame_size(self.in_w, self.in_h)?;
        if !out_fmt.is_supported() {
            return Err(Error::UnsupportedPixFmt {
                name: out_fmt.name(),
                reason: out_fmt.unsupported_reason().unwrap_or("unsupported"),
            });
        }
        let ffmpeg = self.bins.ffmpeg_path()?;
        let (out_w, out_h) = match self.size_mode {
            SizeMode::Fixed(w, h) => (w, h),
            SizeMode::Unchanged => (self.in_w, self.in_h),
            SizeMode::Auto => {
                // The dry-run source is FINITE: a graph with a buffering filter
                // (reverse, tpad, loop ...) cannot emit its first frame before the input
                // ends, and on an endless source ffmpeg would buffer without bound.
                // About two seconds, but never more than 60 frames.
                let fps = self.fps.0 as f64 / self.fps.1 as f64;
                let seconds = (60.0 / fps).clamp(0.05, 2.0);
                let source = format!(
                    "color=c=black:s={}x{}:r={}:d={:.3},format={}",
                    self.in_w,
                    self.in_h,
                    self.rate_text(),
                    seconds,
                    self.in_fmt.name()
                );
                let chain = match self.chain(false) {
                    Some(c) => format!("{},showinfo", c),
                    None => "showinfo".to_string(),
                };
                let dry = vec![os("-f"), os("lavfi"), os("-i"), os(source), os("-vf"), os(chain)];
                detect_size(&ffmpeg, &dry, &format!("the filter graph '{}'", self.graph))?
            }
        };
        let out_bytes = out_fmt.frame_size(out_w, out_h)?;
        let args = self.command_line();
        let context = format!("filter with '{}'", self.graph);
        let piped = spawn("ffmpeg", &ffmpeg, &args, true, true, context)?;
        let stdout = piped.stdout.expect("standard output was piped");
        let (tx, rx) = channel();
        let reader = thread::Builder::new()
            .name("neutral-ffmpeg-frames".into())
            .spawn(move || read_frames(stdout, out_fmt, out_w, out_h, out_bytes, tx))
            .map_err(|e| Error::io("starting the frame reader", e))?;
        Ok(Filter {
            proc: piped.proc,
            stdin: piped.stdin,
            rx,
            reader: Some(reader),
            in_fmt: self.in_fmt,
            in_w: self.in_w,
            in_h: self.in_h,
            in_bytes,
            out_fmt,
            out_w,
            out_h,
            frames_in: 0,
            frames_out: 0,
            finished: false,
            args,
        })
    }
}

enum Message {
    Frame(FrameBuf),
    Eof,
    Failed(Error),
}

/// Reads whole frames from ffmpeg's standard output until it ends.
fn read_frames(mut stdout: ChildStdout, fmt: PixFmt, w: u32, h: u32, frame_bytes: usize, tx: Sender<Message>) {
    loop {
        let mut buf = vec![0u8; frame_bytes];
        let mut got = 0usize;
        while got < frame_bytes {
            match stdout.read(&mut buf[got..]) {
                Ok(0) => break,
                Ok(n) => got += n,
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => {
                    let _ = tx.send(Message::Failed(Error::io("reading filtered frames", e)));
                    return;
                }
            }
        }
        if got == frame_bytes {
            match FrameBuf::from_vec(fmt, w, h, buf) {
                Ok(frame) => {
                    if tx.send(Message::Frame(frame)).is_err() {
                        return;
                    }
                }
                Err(e) => {
                    let _ = tx.send(Message::Failed(e));
                    return;
                }
            }
        } else if got == 0 {
            let _ = tx.send(Message::Eof);
            return;
        } else {
            let _ = tx.send(Message::Failed(Error::TruncatedFrame { got, expected: frame_bytes }));
            return;
        }
    }
}

/// A running ffmpeg that filters raw frames.
///
/// Typical use is [`Filter::push`] for every input frame, [`Filter::try_pull`]
/// in between to collect what is ready, then [`Filter::finish`]. Output frames
/// wait in an unbounded queue until they are pulled. Dropping the filter stops ffmpeg.
pub struct Filter {
    proc: Proc,
    stdin: Option<ChildStdin>,
    rx: Receiver<Message>,
    reader: Option<JoinHandle<()>>,
    in_fmt: PixFmt,
    in_w: u32,
    in_h: u32,
    in_bytes: usize,
    out_fmt: PixFmt,
    out_w: u32,
    out_h: u32,
    frames_in: u64,
    frames_out: u64,
    finished: bool,
    args: Vec<OsString>,
}

impl Filter {
    /// Start configuring a filter for input frames of format `in_fmt` and size `in_w` x `in_h`.
    pub fn builder(in_fmt: PixFmt, in_w: u32, in_h: u32) -> FilterBuilder {
        FilterBuilder {
            bins: Binaries::new(),
            in_fmt,
            in_w,
            in_h,
            fps: (25, 1),
            graph: String::new(),
            out_fmt: None,
            size_mode: SizeMode::Auto,
            input_args: Vec::new(),
            output_args: Vec::new(),
            loglevel: "error".to_string(),
            timing_passthrough: true,
        }
    }

    /// Format of the frames that come back.
    pub fn out_pix_fmt(&self) -> PixFmt {
        self.out_fmt
    }

    /// Size of the frames that come back.
    pub fn out_size(&self) -> (u32, u32) {
        (self.out_w, self.out_h)
    }

    /// Frames pushed so far.
    pub fn frames_in(&self) -> u64 {
        self.frames_in
    }

    /// Frames pulled so far.
    pub fn frames_out(&self) -> u64 {
        self.frames_out
    }

    /// What ffmpeg has printed on standard error so far.
    pub fn stderr(&self) -> String {
        self.proc.stderr_text()
    }

    /// The arguments ffmpeg was started with.
    pub fn command_line(&self) -> &[OsString] {
        &self.args
    }

    /// Send one input frame.
    pub fn push(&mut self, frame: &FrameBuf) -> Result<()> {
        if frame.fmt() != self.in_fmt || frame.width() != self.in_w || frame.height() != self.in_h {
            return Err(Error::FrameMismatch(format!(
                "filter expects {} {}x{}, frame is {} {}x{}",
                self.in_fmt,
                self.in_w,
                self.in_h,
                frame.fmt(),
                frame.width(),
                frame.height()
            )));
        }
        debug_assert_eq!(frame.len(), self.in_bytes);
        let stdin = self
            .stdin
            .as_mut()
            .ok_or_else(|| Error::InvalidArgument("the filter input is already closed".into()))?;
        match stdin.write_all(frame.as_bytes()) {
            Ok(()) => {
                self.frames_in += 1;
                Ok(())
            }
            Err(e) => {
                self.stdin = None;
                Err(self.proc.failure(e, "writing frames to ffmpeg"))
            }
        }
    }

    /// Tell ffmpeg there are no more input frames. Frames still inside the graph
    /// come out through [`Filter::pull`].
    pub fn close_input(&mut self) {
        drop(self.stdin.take());
    }

    /// Take a filtered frame if one is ready, without waiting.
    pub fn try_pull(&mut self) -> Result<Option<FrameBuf>> {
        if self.finished {
            return Ok(None);
        }
        match self.rx.try_recv() {
            Ok(msg) => self.handle(msg),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => self.handle(Message::Eof),
        }
    }

    /// Wait for the next filtered frame. Returns `None` once the input is closed
    /// and every frame has come out. Blocks while ffmpeg still waits for input,
    /// so call [`Filter::close_input`] first when no more frames will be pushed.
    pub fn pull(&mut self) -> Result<Option<FrameBuf>> {
        if self.finished {
            return Ok(None);
        }
        match self.rx.recv() {
            Ok(msg) => self.handle(msg),
            Err(_) => self.handle(Message::Eof),
        }
    }

    /// Like [`Filter::pull`] but gives up after `timeout` and returns `None`
    /// (check [`Filter::is_finished`] to tell a timeout from the end of the stream).
    pub fn pull_timeout(&mut self, timeout: Duration) -> Result<Option<FrameBuf>> {
        if self.finished {
            return Ok(None);
        }
        match self.rx.recv_timeout(timeout) {
            Ok(msg) => self.handle(msg),
            Err(RecvTimeoutError::Timeout) => Ok(None),
            Err(RecvTimeoutError::Disconnected) => self.handle(Message::Eof),
        }
    }

    /// True after the end of the output stream was reached.
    pub fn is_finished(&self) -> bool {
        self.finished
    }

    fn handle(&mut self, msg: Message) -> Result<Option<FrameBuf>> {
        match msg {
            Message::Frame(frame) => {
                self.frames_out += 1;
                Ok(Some(frame))
            }
            Message::Eof => {
                self.finished = true;
                self.join_reader();
                self.proc.finish()?;
                Ok(None)
            }
            Message::Failed(err) => {
                self.finished = true;
                self.join_reader();
                // ffmpeg's own error text explains a broken stream better than the read error.
                match self.proc.wait_within(EXIT_WAIT) {
                    Ok(status) if !status.success() => Err(self.proc.error_for(status)),
                    _ => Err(err),
                }
            }
        }
    }

    fn join_reader(&mut self) {
        if let Some(h) = self.reader.take() {
            let _ = h.join();
        }
    }

    /// Close the input, collect every remaining frame and check that ffmpeg succeeded.
    pub fn finish(mut self) -> Result<Vec<FrameBuf>> {
        self.close_input();
        let mut frames = Vec::new();
        while let Some(frame) = self.pull()? {
            frames.push(frame);
        }
        Ok(frames)
    }

    /// Push every frame of `frames`, collecting output as it appears, then finish.
    pub fn run_all<I>(mut self, frames: I) -> Result<Vec<FrameBuf>>
    where
        I: IntoIterator<Item = FrameBuf>,
    {
        let mut out = Vec::new();
        for frame in frames {
            self.push(&frame)?;
            while let Some(done) = self.try_pull()? {
                out.push(done);
            }
        }
        self.close_input();
        while let Some(done) = self.pull()? {
            out.push(done);
        }
        Ok(out)
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
        let b = Filter::builder(PixFmt::RGB24, 64, 48).graph("hflip").out_pix_fmt(PixFmt::GRAY);
        assert_eq!(
            text(&b.command_line()).join(" "),
            "-hide_banner -loglevel error -nostdin -f rawvideo -pix_fmt rgb24 -video_size 64x48 \
             -framerate 25/1 -i pipe:0 -an -sn -dn -vf hflip -fps_mode passthrough -pix_fmt gray \
             -f rawvideo pipe:1"
        );
    }

    #[test]
    fn empty_graph_has_no_vf_and_fixed_size_appends_scale() {
        let b = Filter::builder(PixFmt::YUV420P, 64, 48);
        assert!(!text(&b.command_line()).contains(&"-vf".to_string()));
        let b = b.output_size(32, 24);
        let args = text(&b.command_line());
        let i = args.iter().position(|a| a == "-vf").unwrap();
        assert_eq!(args[i + 1], "scale=32:24");
        let b = Filter::builder(PixFmt::YUV420P, 64, 48).graph("hflip").output_size(32, 24);
        let args = text(&b.command_line());
        let i = args.iter().position(|a| a == "-vf").unwrap();
        assert_eq!(args[i + 1], "hflip,scale=32:24");
    }

    #[test]
    fn rate_and_passthrough_options() {
        let b = Filter::builder(PixFmt::GRAY, 8, 8)
            .fps(24000, 1001)
            .timing_passthrough(false)
            .input_args(["-thread_queue_size", "64"])
            .output_args(["-threads", "1"]);
        let args = text(&b.command_line());
        let i = args.iter().position(|a| a == "-framerate").unwrap();
        assert_eq!(args[i + 1], "24000/1001");
        assert!(!args.contains(&"-fps_mode".to_string()));
        let q = args.iter().position(|a| a == "-thread_queue_size").unwrap();
        assert!(q > i && q < args.iter().position(|a| a == "-i").unwrap());
        let t = args.iter().position(|a| a == "-threads").unwrap();
        assert!(t > args.iter().rposition(|a| a == "-pix_fmt").unwrap());
    }

    #[test]
    fn invalid_configuration_is_refused_before_starting() {
        let err = Filter::builder(PixFmt::RGB24, 0, 4).spawn().err().unwrap();
        assert!(matches!(err, Error::InvalidSize { .. }), "{:?}", err);
        let err = Filter::builder(PixFmt::VAAPI, 4, 4).spawn().err().unwrap();
        assert!(matches!(err, Error::UnsupportedPixFmt { .. }), "{:?}", err);
        let err = Filter::builder(PixFmt::RGB24, 4, 4).out_pix_fmt(PixFmt::VAAPI).spawn().err().unwrap();
        assert!(matches!(err, Error::UnsupportedPixFmt { .. }), "{:?}", err);
        let err = Filter::builder(PixFmt::RGB24, 4, 4).fps(1, 0).spawn().err().unwrap();
        assert!(matches!(err, Error::InvalidArgument(_)), "{:?}", err);
    }
}
