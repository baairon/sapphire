//! The data plane: decode, a chain of stages, encode, all running at once on their own threads.
//!
//! ```text
//! ffmpeg decode --> [stage 1] --> [stage 2] --> ... --> ffmpeg encode
//! ```
//!
//! Every arrow is a small bounded queue of whole frames. A stage that is a child process (`vfx`
//! or a `--stage` command) has three threads of its own: one writes its standard input, one reads
//! its standard output, one keeps the tail of its standard error. An in-process stage has one.
//! Nothing waits for anything else to finish, so a stage that reads all of its input before it
//! writes (a temporal effect spools its input first) cannot deadlock the pipe, and a stage that
//! stops reading, stops writing or dies in the middle of a frame ends the run with a code.
//!
//! The main thread owns the children. It
//!
//! - kills a stage that moved no bytes for `stage_timeout` while it owed something (a stage that
//!   waits for its upstream is not counted), and reports `stage_timeout`;
//! - on the first failure of any thread, stops the others: it gives the stages a moment to exit by
//!   themselves, so that the real exit code is read, then kills what is left;
//! - after every thread has ended, decides the one root cause from the facts the threads left and
//!   the exit statuses of the children, so a stage that was only a victim of another never hides it.
//!
//! The root cause is picked in pipeline order: decode, then the stages in order, then encode. For
//! a stage: a timeout, else a non-zero exit (code 2, 3 and 4 keep their vfx meaning), else a
//! partial frame on its output, else a closed input, else the wrong number of frames (fewer than
//! it was given is `broken_pipe`, more is `stage_failed`).
//!
//! The ffmpeg stages are `neutral_ffmpeg::Decoder` and `Encoder`.

use std::collections::VecDeque;
use std::io::{Read, Write};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::{Child, ChildStderr, ChildStdin, ChildStdout, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::mpsc::{self, Receiver, SyncSender};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

use neutral_ffmpeg::{Binaries, Decoder, Encoder, Error as BridgeError};

use crate::error::{tail_lines, Code, Error, Result};
use crate::format::FrameSpec;
use crate::plan::{Step, StepKind};
use crate::pytext::format_command;
use crate::rate::Rate;
use crate::stage::{build as build_stage, FrameStage};
use crate::sys::command;
use crate::tools::Media;

/// How long the stages get to exit by themselves after one thread failed, before they are killed.
pub const GRACE: Duration = Duration::from_millis(1500);
/// How often the main thread looks at the children and the clock.
const TICK: Duration = Duration::from_millis(20);
/// Bytes written to a stage in one call, so that progress is visible during a large frame.
const WRITE_CHUNK: usize = 256 * 1024;
/// Bytes of standard error kept per stage.
const STDERR_KEEP: usize = 64 * 1024;
/// Bytes of frames a queue may hold.
const QUEUE_BYTES: usize = 48 * 1024 * 1024;

/// Everything the pipeline needs, decided by the plan.
pub struct PipelineConfig {
    /// ffmpeg and ffprobe.
    pub media: Media,
    /// The input file, as given.
    pub input: String,
    /// Position of the chosen stream among the video streams (`-map 0:v:N`).
    pub video_ordinal: u32,
    /// The `-vf` chain of the decode.
    pub decode_filter: String,
    /// The size and depth of the frames in every pipe.
    pub spec: FrameSpec,
    /// The exact frame rate.
    pub rate: Rate,
    /// The encoder's arguments after `-i pipe:0` and before the output path.
    pub encode_tail: Vec<String>,
    /// The file the encoder writes.
    pub partial: String,
    /// The stages between decode and encode, with their command lines.
    pub steps: Vec<Step>,
    /// How long a judged process (decode, a `--stage` command, encode) may be silent; `None`
    /// never kills. The effect (`vfx`) is never judged.
    pub stage_timeout: Option<Duration>,
}

impl PipelineConfig {
    /// The frames the encoder takes: the output of the last step, which differs from
    /// [`PipelineConfig::spec`] after a stage with declared geometry.
    pub fn out_spec(&self) -> FrameSpec {
        self.steps.last().map(|s| s.spec_out).unwrap_or(self.spec)
    }
}

/// What one stage did.
#[derive(Clone, Debug, PartialEq)]
pub struct StageReport {
    /// The step's label: `vfx`, `pre1`, `post1`, ... (see [`crate::plan::Step::name`]).
    pub name: String,
    /// The last lines the stage printed on standard error.
    pub stderr_tail: Vec<String>,
    /// The command that ran, as a `[cmd]` line.
    pub command: String,
    /// The exit code, for a process that ended by itself.
    pub exit_code: Option<i32>,
    /// Frames written to the stage.
    pub frames_in: u64,
    /// Frames read back from the stage.
    pub frames_out: u64,
    /// Seconds from start to exit.
    pub seconds: f64,
}

/// What a finished run did.
#[derive(Clone, Debug)]
pub struct RunReport {
    /// Frames the decoder delivered.
    pub decoded: u64,
    /// Frames the encoder was given.
    pub encoded: u64,
    /// One entry per stage, in order.
    pub stages: Vec<StageReport>,
    /// What the decoding ffmpeg printed (warnings about damaged data, for example).
    pub decode_stderr: String,
    /// What the encoding ffmpeg printed (warnings).
    pub encode_stderr: String,
}

// ----------------------------------------------------------------------------------------------
// facts the threads leave behind

#[derive(Debug)]
enum Failure {
    /// ffmpeg ended with a failure of its own.
    Ffmpeg(BridgeError),
    /// The stream ended inside a frame.
    Partial { got: usize, expected: usize, after: u64 },
    /// A write found the pipe closed.
    BrokenPipe(String),
    /// Another I/O failure.
    Io(String),
    /// An in-process stage reported an error at a frame.
    Stage { frame: u64, message: String },
    /// A thread panicked.
    Panic(String),
}

#[derive(Debug)]
enum Outcome {
    Ok,
    /// Stopped because the run was already failing or a neighbour went away.
    Aborted,
    Failed(Failure),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    Decode,
    Write(usize),
    Read(usize),
    Builtin(usize),
    Encode,
}

struct Done {
    role: Role,
    outcome: Outcome,
    frames: u64,
    text: String,
}

/// Progress of one process stage, read by the watchdog.
struct Meter {
    start: Instant,
    last_ms: AtomicU64,
    /// The stage is waiting for its upstream to hand it a frame.
    idle: AtomicBool,
    /// The stage's output is waiting for room in the queue to the next stage.
    blocked: AtomicBool,
}

impl Meter {
    fn new() -> Meter {
        Meter { start: Instant::now(), last_ms: AtomicU64::new(0), idle: AtomicBool::new(true), blocked: AtomicBool::new(false) }
    }

    fn touch(&self) {
        self.last_ms.store(self.start.elapsed().as_millis() as u64, Ordering::Relaxed);
    }

    /// How long nothing moved, counting only the time the stage was at fault: not while it waits
    /// for its upstream, and not while its output waits for the stage after it.
    fn quiet_for(&self) -> Duration {
        if self.idle.load(Ordering::Relaxed) || self.blocked.load(Ordering::Relaxed) {
            return Duration::ZERO;
        }
        let now = self.start.elapsed().as_millis() as u64;
        Duration::from_millis(now.saturating_sub(self.last_ms.load(Ordering::Relaxed)))
    }
}

/// A child process that is killed when it goes out of scope while still running.
struct Guard {
    child: Child,
    status: Option<ExitStatus>,
    ended: Option<Instant>,
}

impl Guard {
    fn poll(&mut self) -> bool {
        if self.status.is_some() {
            return true;
        }
        if let Ok(Some(status)) = self.child.try_wait() {
            self.status = Some(status);
            self.ended = Some(Instant::now());
            return true;
        }
        false
    }

    /// End the child and everything it started (`taskkill /T` on Windows, its process group on
    /// POSIX, see [`crate::sys::kill_tree`]), then reap it.
    fn kill(&mut self) {
        if !self.poll() {
            crate::sys::kill_tree(self.child.id());
            let _ = self.child.kill();
            if let Ok(status) = self.child.wait() {
                self.status = Some(status);
                self.ended = Some(Instant::now());
            }
        }
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        self.kill();
    }
}

/// Map an error of the ffmpeg bridge to the error object of a stage.
pub fn from_bridge(stage: &str, error: &BridgeError) -> Error {
    match error {
        BridgeError::Ffmpeg { code, stderr, .. } => Error::stage_exit(stage, *code, tail_lines(stderr, 3)),
        BridgeError::Spawn { .. } | BridgeError::ToolNotFound { .. } => Error::tool_missing(error.to_string()).at(stage),
        BridgeError::Timeout { stderr, .. } => Error::new(Code::StageTimeout, error.to_string()).at(stage).with_tail(tail_lines(stderr, 3)),
        BridgeError::TruncatedFrame { .. } => Error::new(Code::PartialFrame, error.to_string()).at(stage),
        other => Error::stage_failed(other.to_string()).at(stage),
    }
}

fn is_broken_pipe(e: &std::io::Error) -> bool {
    e.kind() == std::io::ErrorKind::BrokenPipe
}

// ----------------------------------------------------------------------------------------------
// threads

fn queue_capacity(frame_bytes: usize) -> usize {
    (QUEUE_BYTES / frame_bytes.max(1)).clamp(1, 4)
}

fn guarded(role: Role, f: impl FnOnce() -> (Outcome, u64, String)) -> Done {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok((outcome, frames, text)) => Done { role, outcome, frames, text },
        Err(panic) => {
            let message = panic.downcast_ref::<&str>().map(|s| s.to_string()).or_else(|| panic.downcast_ref::<String>().cloned()).unwrap_or_else(|| "unknown".into());
            Done { role, outcome: Outcome::Failed(Failure::Panic(message)), frames: 0, text: String::new() }
        }
    }
}

/// The decode joins the stall watch: it is at fault while ffmpeg owes the next frame, not while
/// the queue to the first stage is full.
fn decode_thread(mut decoder: Decoder, tx: SyncSender<Vec<u8>>, meter: &Meter, cancel: &AtomicBool) -> (Outcome, u64, String) {
    let mut frames = 0u64;
    meter.idle.store(false, Ordering::Relaxed);
    meter.touch();
    loop {
        if cancel.load(Ordering::Relaxed) {
            meter.idle.store(true, Ordering::Relaxed);
            return (Outcome::Aborted, frames, String::new());
        }
        match decoder.read_frame() {
            Ok(Some(frame)) => {
                meter.touch();
                meter.blocked.store(true, Ordering::Relaxed);
                let sent = tx.send(frame.into_vec());
                meter.blocked.store(false, Ordering::Relaxed);
                meter.touch();
                if sent.is_err() {
                    meter.idle.store(true, Ordering::Relaxed);
                    return (Outcome::Aborted, frames, String::new());
                }
                frames += 1;
            }
            Ok(None) => {
                meter.idle.store(true, Ordering::Relaxed);
                return (Outcome::Ok, frames, decoder.stderr());
            }
            Err(e) => {
                meter.idle.store(true, Ordering::Relaxed);
                cancel.store(true, Ordering::Relaxed);
                return (Outcome::Failed(Failure::Ffmpeg(e)), frames, String::new());
            }
        }
    }
}

fn write_thread(rx: Receiver<Vec<u8>>, mut stdin: ChildStdin, meter: &Meter, cancel: &AtomicBool) -> (Outcome, u64, String) {
    let mut frames = 0u64;
    meter.touch();
    loop {
        meter.idle.store(true, Ordering::Relaxed);
        let next = rx.recv();
        meter.idle.store(false, Ordering::Relaxed);
        meter.touch();
        let frame = match next {
            Ok(frame) => frame,
            Err(_) => {
                drop(stdin);
                meter.touch();
                let outcome = if cancel.load(Ordering::Relaxed) { Outcome::Aborted } else { Outcome::Ok };
                return (outcome, frames, String::new());
            }
        };
        for chunk in frame.chunks(WRITE_CHUNK) {
            if let Err(e) = stdin.write_all(chunk) {
                let outcome = if cancel.load(Ordering::Relaxed) {
                    Outcome::Aborted
                } else if is_broken_pipe(&e) {
                    Outcome::Failed(Failure::BrokenPipe(e.to_string()))
                } else {
                    Outcome::Failed(Failure::Io(format!("cannot write to the stage: {}", e)))
                };
                return (outcome, frames, String::new());
            }
            meter.touch();
        }
        frames += 1;
        if cancel.load(Ordering::Relaxed) {
            return (Outcome::Aborted, frames, String::new());
        }
    }
}

fn read_thread(mut stdout: ChildStdout, tx: SyncSender<Vec<u8>>, frame_bytes: usize, meter: &Meter, cancel: &AtomicBool) -> (Outcome, u64, String) {
    let mut frames = 0u64;
    loop {
        let mut buf = vec![0u8; frame_bytes];
        let mut got = 0usize;
        while got < frame_bytes {
            match stdout.read(&mut buf[got..]) {
                Ok(0) => break,
                Ok(n) => {
                    got += n;
                    meter.touch();
                }
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => {
                    let outcome = if cancel.load(Ordering::Relaxed) { Outcome::Aborted } else { Outcome::Failed(Failure::Io(format!("cannot read the stage output: {}", e))) };
                    return (outcome, frames, String::new());
                }
            }
        }
        meter.touch();
        if got == 0 {
            return (Outcome::Ok, frames, String::new());
        }
        if got < frame_bytes {
            // the stage closed its output inside a frame even when the run is being stopped (a
            // broken pipe on its input can raise the flag first); `analyse` sets it aside when
            // the stage was killed here
            return (Outcome::Failed(Failure::Partial { got, expected: frame_bytes, after: frames }), frames, String::new());
        }
        meter.blocked.store(true, Ordering::Relaxed);
        let sent = tx.send(buf);
        meter.blocked.store(false, Ordering::Relaxed);
        meter.touch();
        if sent.is_err() {
            drain_rest(&mut stdout);
            return (Outcome::Aborted, frames, String::new());
        }
        frames += 1;
        if cancel.load(Ordering::Relaxed) {
            drain_rest(&mut stdout);
            return (Outcome::Aborted, frames, String::new());
        }
    }
}

/// Read and discard what a stage still writes, so that it is not killed by a closed pipe while
/// the run is being stopped: a stage upstream of the failure must not look like the failure.
fn drain_rest(stdout: &mut ChildStdout) {
    let mut sink = [0u8; 64 * 1024];
    loop {
        match stdout.read(&mut sink) {
            Ok(0) | Err(_) => break,
            Ok(_) => {}
        }
    }
}

fn builtin_thread(rx: Receiver<Vec<u8>>, tx: SyncSender<Vec<u8>>, mut stage: Box<dyn FrameStage>, cancel: &AtomicBool) -> (Outcome, u64, String) {
    let mut frames = 0u64;
    while let Ok(mut frame) = rx.recv() {
        if cancel.load(Ordering::Relaxed) {
            return (Outcome::Aborted, frames, String::new());
        }
        if let Err(message) = stage.process(frames, &mut frame) {
            cancel.store(true, Ordering::Relaxed);
            return (Outcome::Failed(Failure::Stage { frame: frames, message }), frames, String::new());
        }
        if tx.send(frame).is_err() {
            return (Outcome::Aborted, frames, String::new());
        }
        frames += 1;
    }
    let outcome = if cancel.load(Ordering::Relaxed) { Outcome::Aborted } else { Outcome::Ok };
    (outcome, frames, String::new())
}

/// The encode joins the stall watch: it is at fault while ffmpeg refuses a frame, not while it
/// waits for the last stage or finishes its file.
fn encode_thread(rx: Receiver<Vec<u8>>, mut encoder: Encoder, meter: &Meter, cancel: &AtomicBool) -> (Outcome, u64, String) {
    let mut frames = 0u64;
    meter.touch();
    loop {
        meter.idle.store(true, Ordering::Relaxed);
        let next = rx.recv();
        meter.idle.store(false, Ordering::Relaxed);
        meter.touch();
        match next {
            Ok(buf) => {
                if cancel.load(Ordering::Relaxed) {
                    meter.idle.store(true, Ordering::Relaxed);
                    return (Outcome::Aborted, frames, String::new());
                }
                if let Err(e) = encoder.write_raw_frame(&buf) {
                    meter.idle.store(true, Ordering::Relaxed);
                    cancel.store(true, Ordering::Relaxed);
                    return (Outcome::Failed(Failure::Ffmpeg(e)), frames, String::new());
                }
                meter.touch();
                frames += 1;
            }
            Err(_) => {
                if cancel.load(Ordering::Relaxed) {
                    meter.idle.store(true, Ordering::Relaxed);
                    return (Outcome::Aborted, frames, String::new());
                }
                // finishing the file (flush, trailer) is not judged
                meter.idle.store(true, Ordering::Relaxed);
                let finished = encoder.finish();
                return match finished {
                    Ok(stats) => (Outcome::Ok, frames, stats.stderr),
                    Err(e) => {
                        cancel.store(true, Ordering::Relaxed);
                        (Outcome::Failed(Failure::Ffmpeg(e)), frames, String::new())
                    }
                };
            }
        }
    }
}

fn drain_stderr(mut stderr: ChildStderr) -> String {
    let mut kept: VecDeque<u8> = VecDeque::new();
    let mut buf = [0u8; 4096];
    loop {
        match stderr.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                kept.extend(&buf[..n]);
                if kept.len() > STDERR_KEEP * 2 {
                    let drop_n = kept.len() - STDERR_KEEP;
                    kept.drain(..drop_n);
                }
            }
        }
    }
    let bytes: Vec<u8> = kept.into_iter().collect();
    String::from_utf8_lossy(&bytes).replace('\r', "")
}

// ----------------------------------------------------------------------------------------------
// the run

struct Proc {
    step: usize,
    /// The watchdog may kill it. The effect (`vfx`) is never judged: a temporal effect is silent
    /// for as long as it renders.
    judged: bool,
    guard: Guard,
    meter: Arc<Meter>,
    started: Instant,
    timed_out: bool,
    killed: bool,
}

fn spawn_process(step: &Step) -> Result<(Child, ChildStdin, ChildStdout, ChildStderr)> {
    let argv = &step.argv;
    let mut cmd = command(&argv[0]);
    cmd.args(&argv[1..]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| Error::tool_missing(format!("cannot start `{}`: {}", argv[0], e)).at(&step.name))?;
    let stdin = child.stdin.take().expect("piped");
    let stdout = child.stdout.take().expect("piped");
    let stderr = child.stderr.take().expect("piped");
    Ok((child, stdin, stdout, stderr))
}

/// The decoder of a run: `ffmpeg -i INPUT -map 0:v:N -an -sn -dn -vf CHAIN -pix_fmt FMT -f rawvideo pipe:1`.
///
/// The chain is the one `tools/apply.py` builds; there is no `size()` (a second `scale` after
/// `scale=in_color_matrix=M` would change the colour result) and no `-fps_mode passthrough` (so
/// the arguments are apply.py's own). The crate measures the frame size with a one frame dry run.
pub fn decoder_builder(cfg: &PipelineConfig, bins: Binaries) -> neutral_ffmpeg::DecoderBuilder {
    Decoder::builder(&cfg.input)
        .binaries(bins)
        .pix_fmt(cfg.spec.depth.pix_fmt())
        .filter(cfg.decode_filter.clone())
        .video_stream(cfg.video_ordinal)
        .timing_passthrough(false)
}

/// The encoder of a run: raw frames on `pipe:0`, then the source as the second input for the
/// audio, the maps, the filter graph and the codec, all in `output_args` after `-i pipe:0`.
pub fn encoder_builder(cfg: &PipelineConfig, bins: Binaries) -> Result<neutral_ffmpeg::EncoderBuilder> {
    let (num, den) = cfg.rate.as_u32_pair().ok_or_else(|| Error::unsupported("the frame rate does not fit the encoder").at("encode"))?;
    let out = cfg.out_spec();
    Ok(Encoder::builder(&cfg.partial, out.depth.pix_fmt(), out.width, out.height)
        .binaries(bins)
        .fps(num, den)
        .overwrite(false)
        .output_args(cfg.encode_tail.clone())
        .finish_timeout(Duration::from_secs(3600)))
}

/// Run decode, the stages and encode. `hooks` replaces the in-process stage of a step by name.
pub fn run(cfg: &PipelineConfig, hooks: Vec<(String, Box<dyn FrameStage>)>) -> Result<RunReport> {
    let spec = cfg.spec;
    let frame_bytes = spec.frame_bytes() as usize;
    let bins = Binaries::new().with_ffmpeg(&cfg.media.ffmpeg).with_ffprobe(&cfg.media.ffprobe);

    // The decoder first: it measures the size, and a file that will not decode fails here.
    let decoder = decoder_builder(cfg, bins.clone()).spawn().map_err(|e| from_bridge("decode", &e))?;
    let decode_pid = decoder.id();
    let decode_meter = Meter::new();
    if (decoder.width(), decoder.height()) != (spec.width, spec.height) {
        return Err(Error::stage_failed(format!(
            "ffmpeg decodes {}x{}, but the probe says {}x{}",
            decoder.width(),
            decoder.height(),
            spec.width,
            spec.height
        ))
        .at("decode"));
    }

    // Stages: processes are started now so that a program that will not start fails before a frame moves.
    let mut hooks = hooks;
    let mut procs: Vec<Proc> = Vec::new();
    let mut ios: Vec<Option<(ChildStdin, ChildStdout, ChildStderr)>> = Vec::new();
    let mut builtins: Vec<Option<Box<dyn FrameStage>>> = Vec::new();
    for (i, step) in cfg.steps.iter().enumerate() {
        match &step.kind {
            StepKind::Builtin { id, params } => {
                let stage = match hooks.iter().position(|(n, _)| n == &step.name) {
                    Some(at) => hooks.remove(at).1,
                    None => build_stage(id, step.spec_in, params).map_err(|e| e.at(&step.name))?,
                };
                builtins.push(Some(stage));
                ios.push(None);
            }
            _ => {
                let (child, stdin, stdout, stderr) = spawn_process(step)?;
                procs.push(Proc {
                    step: i,
                    judged: !matches!(step.kind, StepKind::Vfx),
                    guard: Guard { child, status: None, ended: None },
                    meter: Arc::new(Meter::new()),
                    started: Instant::now(),
                    timed_out: false,
                    killed: false,
                });
                builtins.push(None);
                ios.push(Some((stdin, stdout, stderr)));
            }
        }
    }
    let encoder = encoder_builder(cfg, bins)?.spawn().map_err(|e| from_bridge("encode", &e))?;
    let encode_pid = encoder.id();
    let encode_meter = Meter::new();
    let mut decode_timed_out = false;
    let mut encode_timed_out = false;

    let n = cfg.steps.len();
    let capacity = queue_capacity(frame_bytes);
    let cancel = AtomicBool::new(false);
    let (events_tx, events_rx) = mpsc::channel::<Done>();

    // queue i carries the frames into step i; queue n carries them into the encoder
    let mut txs: Vec<Option<SyncSender<Vec<u8>>>> = Vec::new();
    let mut rxs: Vec<Option<Receiver<Vec<u8>>>> = Vec::new();
    for _ in 0..=n {
        let (tx, rx) = mpsc::sync_channel::<Vec<u8>>(capacity);
        txs.push(Some(tx));
        rxs.push(Some(rx));
    }

    let mut expected_events = 2usize; // decode and encode
    for step in &cfg.steps {
        expected_events += if matches!(step.kind, StepKind::Builtin { .. }) { 1 } else { 2 };
    }

    thread::scope(|scope| {
        let cancel = &cancel;
        let (decode_meter, encode_meter) = (&decode_meter, &encode_meter);
        {
            let tx = txs[0].take().expect("queue 0");
            let events = events_tx.clone();
            scope.spawn(move || {
                let _ = events.send(guarded(Role::Decode, || decode_thread(decoder, tx, decode_meter, cancel)));
            });
        }
        let mut drains: Vec<(usize, thread::ScopedJoinHandle<'_, String>)> = Vec::new();
        let mut proc_at = 0usize;
        for i in 0..n {
            let rx_in = rxs[i].take().expect("input queue");
            let tx_out = txs[i + 1].take().expect("output queue");
            if let Some(stage) = builtins[i].take() {
                let events = events_tx.clone();
                scope.spawn(move || {
                    let _ = events.send(guarded(Role::Builtin(i), || builtin_thread(rx_in, tx_out, stage, cancel)));
                });
                continue;
            }
            let (stdin, stdout, stderr) = ios[i].take().expect("process pipes");
            let out_bytes = cfg.steps[i].spec_out.frame_bytes() as usize;
            let meter = procs[proc_at].meter.clone();
            proc_at += 1;
            {
                let events = events_tx.clone();
                let meter = meter.clone();
                scope.spawn(move || {
                    let _ = events.send(guarded(Role::Write(i), || write_thread(rx_in, stdin, &meter, cancel)));
                });
            }
            {
                let events = events_tx.clone();
                scope.spawn(move || {
                    let _ = events.send(guarded(Role::Read(i), || read_thread(stdout, tx_out, out_bytes, &meter, cancel)));
                });
            }
            drains.push((i, scope.spawn(move || drain_stderr(stderr))));
        }
        {
            let rx = rxs[n].take().expect("last queue");
            let events = events_tx.clone();
            scope.spawn(move || {
                let _ = events.send(guarded(Role::Encode, || encode_thread(rx, encoder, encode_meter, cancel)));
            });
        }
        drop(events_tx);

        // ---- the main loop: events, children, watchdog
        let mut results: Vec<Done> = Vec::new();
        let mut failing = false;
        loop {
            match events_rx.recv_timeout(TICK) {
                Ok(done) => {
                    let failed = matches!(done.outcome, Outcome::Failed(_));
                    results.push(done);
                    if failed && !failing {
                        failing = true;
                        stop_everything(cancel, &mut procs, true);
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => thread::sleep(TICK),
            }
            for p in procs.iter_mut() {
                p.guard.poll();
            }
            if let Some(limit) = cfg.stage_timeout {
                let mut fired = false;
                for p in procs.iter_mut() {
                    if p.judged && p.guard.status.is_none() && !p.timed_out && p.meter.quiet_for() > limit {
                        p.timed_out = true;
                        fired = true;
                    }
                }
                // decode and encode are judged too; their ffmpeg is ended by its process id, and
                // the thread that reads or writes it then sees the pipe close
                if !decode_timed_out && find(&results, Role::Decode).is_none() && decode_meter.quiet_for() > limit {
                    decode_timed_out = true;
                    fired = true;
                    crate::sys::kill_tree(decode_pid);
                }
                if !encode_timed_out && find(&results, Role::Encode).is_none() && encode_meter.quiet_for() > limit {
                    encode_timed_out = true;
                    fired = true;
                    crate::sys::kill_tree(encode_pid);
                }
                if fired {
                    failing = true;
                    stop_everything(cancel, &mut procs, false);
                }
            }
            let threads_done = results.len() >= expected_events;
            let children_done = procs.iter().all(|p| p.guard.status.is_some());
            if threads_done && children_done {
                break;
            }
            if threads_done && failing {
                stop_everything(cancel, &mut procs, false);
            }
        }

        let mut drained: Vec<(usize, String)> = Vec::new();
        for (i, handle) in drains {
            drained.push((i, handle.join().unwrap_or_default()));
        }
        analyse(cfg, &results, &procs, &drained, (decode_timed_out, encode_timed_out))
    })
}

/// Stop the run: raise the flag, give the children `GRACE` to exit by themselves (when `grace`),
/// then kill the ones still running. A child that was killed here is marked, so its exit code is
/// not read as its own.
fn stop_everything(cancel: &AtomicBool, procs: &mut [Proc], grace: bool) {
    cancel.store(true, Ordering::Relaxed);
    if grace {
        let until = Instant::now() + GRACE;
        while Instant::now() < until {
            let mut all = true;
            for p in procs.iter_mut() {
                if !p.guard.poll() {
                    all = false;
                }
            }
            if all {
                return;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
    for p in procs.iter_mut() {
        if !p.guard.poll() {
            p.killed = true;
            p.guard.kill();
        }
    }
}

fn find(results: &[Done], role: Role) -> Option<&Done> {
    results.iter().find(|d| d.role == role)
}

/// Decide whether the run succeeded and, if not, which single failure explains it.
fn analyse(cfg: &PipelineConfig, results: &[Done], procs: &[Proc], drained: &[(usize, String)], ffmpeg_timeouts: (bool, bool)) -> Result<RunReport> {
    let timeout_text = cfg.stage_timeout.map(|d| d.as_secs_f64()).unwrap_or(0.0);
    for (who, fired) in [("decode", ffmpeg_timeouts.0), ("encode", ffmpeg_timeouts.1)] {
        if fired {
            return Err(Error::new(Code::StageTimeout, format!("{} moved no data for {} seconds and was killed", who, trim_seconds(timeout_text))).at(who));
        }
    }
    let decode = find(results, Role::Decode);
    if let Some(Done { outcome: Outcome::Failed(f), .. }) = decode {
        return Err(failure_error("decode", f, &[]));
    }
    let mut reports: Vec<StageReport> = Vec::new();
    for (i, step) in cfg.steps.iter().enumerate() {
        let name = step.name.as_str();
        let writer = find(results, Role::Write(i));
        let reader = find(results, Role::Read(i));
        let builtin = find(results, Role::Builtin(i));
        let frames_in = writer.map(|d| d.frames).or(builtin.map(|d| d.frames)).unwrap_or(0);
        let frames_out = reader.map(|d| d.frames).or(builtin.map(|d| d.frames)).unwrap_or(0);
        if let Some(Done { outcome: Outcome::Failed(f), .. }) = builtin {
            return Err(failure_error(name, f, &[]));
        }
        let proc = procs.iter().find(|p| p.step == i);
        let tail = drained.iter().find(|(at, _)| *at == i).map(|(_, t)| tail_lines(t, 3)).unwrap_or_default();
        let mut exit_code = None;
        if let Some(p) = proc {
            let status = p.guard.status;
            exit_code = status.and_then(|s| s.code());
            if p.timed_out {
                return Err(Error::new(
                    Code::StageTimeout,
                    format!("{} moved no data for {} seconds and was killed", name, trim_seconds(timeout_text)),
                )
                .at(name)
                .with_tail(tail));
            }
            if !p.killed {
                if let Some(status) = status {
                    if !status.success() {
                        return Err(Error::stage_exit(name, status.code(), tail));
                    }
                }
            }
        }
        let killed = proc.map(|p| p.killed).unwrap_or(false);
        match reader {
            // a stage killed here may stop inside a frame; that is not its failure
            Some(Done { outcome: Outcome::Failed(Failure::Partial { .. }), .. }) if killed => {}
            Some(Done { outcome: Outcome::Failed(Failure::Partial { got, expected, after }), .. }) => {
                return Err(Error::new(
                    Code::PartialFrame,
                    format!("frame {}: {} ended its output with a partial frame ({} of {} bytes) after {} whole frames", after, name, got, expected, after),
                )
                .at(name)
                .with_exit(exit_code).with_tail(tail));
            }
            Some(Done { outcome: Outcome::Failed(f), .. }) => return Err(failure_error(name, f, &tail)),
            _ => {}
        }
        if let Some(Done { outcome: Outcome::Failed(f), .. }) = writer {
            return Err(match f {
                Failure::BrokenPipe(_) => Error::new(
                    Code::BrokenPipe,
                    format!("frame {}: {} closed its input after {} frames while more were coming", frames_in, name, frames_in),
                )
                .at(name)
                .with_exit(exit_code).with_tail(tail),
                other => failure_error(name, other, &tail),
            });
        }
        let ok_run = matches!(writer.map(|d| &d.outcome), Some(Outcome::Ok)) && matches!(reader.map(|d| &d.outcome), Some(Outcome::Ok));
        let ok_builtin = matches!(builtin.map(|d| &d.outcome), Some(Outcome::Ok));
        if ok_run || ok_builtin {
            let expected = match (step.frames_in, step.frames_out) {
                (a, Some(b)) if a != Some(b) => b,
                _ => frames_in,
            };
            if frames_out < expected {
                return Err(Error::new(
                    Code::BrokenPipe,
                    format!("frame {}: {} closed its output after {} of {} frames", frames_out, name, frames_out, expected),
                )
                .at(name)
                .with_exit(exit_code).with_tail(tail));
            }
            if frames_out > expected {
                return Err(Error::stage_failed(format!("{} returned {} frames for the {} it was given", name, frames_out, expected)).at(name).with_exit(exit_code).with_tail(tail));
            }
        }
        let seconds = proc.map(|p| p.guard.ended.unwrap_or_else(Instant::now).duration_since(p.started).as_secs_f64()).unwrap_or(0.0);
        reports.push(StageReport { name: name.to_string(), stderr_tail: tail, command: format_command(&step.argv), exit_code, frames_in, frames_out, seconds });
    }
    let encode = find(results, Role::Encode);
    if let Some(Done { outcome: Outcome::Failed(f), .. }) = encode {
        return Err(failure_error("encode", f, &[]));
    }
    for done in results {
        if matches!(done.outcome, Outcome::Aborted) && !results.iter().any(|d| matches!(d.outcome, Outcome::Failed(_))) {
            // stopped without a failure anywhere: the run was cancelled from outside
            return Err(Error::stage_failed("the run was stopped before it finished"));
        }
    }
    match (decode, encode) {
        (Some(d), Some(e)) => {
            Ok(RunReport { decoded: d.frames, encoded: e.frames, stages: reports, decode_stderr: d.text.clone(), encode_stderr: e.text.clone() })
        }
        _ => Err(Error::stage_failed("the pipeline ended before decode and encode reported")),
    }
}

fn trim_seconds(s: f64) -> String {
    if s.fract() == 0.0 {
        format!("{}", s as u64)
    } else {
        format!("{}", s)
    }
}

fn failure_error(stage: &str, failure: &Failure, tail: &[String]) -> Error {
    match failure {
        Failure::Ffmpeg(e) => from_bridge(stage, e),
        Failure::Partial { got, expected, after } => Error::new(
            Code::PartialFrame,
            format!("frame {}: {} ended its output with a partial frame ({} of {} bytes) after {} whole frames", after, stage, got, expected, after),
        )
        .at(stage)
        .with_tail(tail.to_vec()),
        Failure::BrokenPipe(m) => Error::new(Code::BrokenPipe, format!("{}: {}", stage, m)).at(stage).with_tail(tail.to_vec()),
        Failure::Io(m) => Error::bad_input(format!("{}: {}", stage, m)).at(stage).with_tail(tail.to_vec()),
        Failure::Stage { frame, message } => Error::unsupported(format!("frame {}: {}", frame, message)).at(stage),
        Failure::Panic(m) => Error::stage_failed(format!("{} stopped on an internal error: {}", stage, m)).at(stage),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_holds_at_least_one_and_at_most_four_frames() {
        assert_eq!(queue_capacity(1), 4);
        assert_eq!(queue_capacity(160 * 96 * 8), 4);
        assert_eq!(queue_capacity(32 * 1024 * 1024), 1);
        assert_eq!(queue_capacity(16 * 1024 * 1024), 3);
        assert_eq!(queue_capacity(24 * 1024 * 1024), 2);
        assert_eq!(queue_capacity(usize::MAX / 2), 1);
    }

    #[test]
    fn a_meter_does_not_count_the_time_a_stage_waits_for_its_upstream() {
        let m = Meter::new();
        assert_eq!(m.quiet_for(), Duration::ZERO, "a stage that has been given nothing yet owes nothing");
        m.idle.store(false, Ordering::Relaxed);
        m.touch();
        thread::sleep(Duration::from_millis(30));
        assert!(m.quiet_for() >= Duration::from_millis(20));
        m.idle.store(true, Ordering::Relaxed);
        assert_eq!(m.quiet_for(), Duration::ZERO);
        m.idle.store(false, Ordering::Relaxed);
        m.touch();
        assert!(m.quiet_for() < Duration::from_millis(20));
    }

    #[test]
    fn bridge_errors_map_to_the_codes() {
        let e = BridgeError::Ffmpeg { tool: "ffmpeg", context: "encode".into(), code: Some(1), stderr: "a\nb\nc\nd\n".into() };
        let m = from_bridge("encode", &e);
        assert_eq!((m.code, m.stage.as_deref()), (Code::StageFailed, Some("encode")));
        assert_eq!(m.stderr_tail, ["b", "c", "d"]);
        let e = BridgeError::ToolNotFound { tool: "ffmpeg", tried: vec![] };
        assert_eq!(from_bridge("decode", &e).code, Code::ToolMissing);
        let e = BridgeError::TruncatedFrame { got: 1, expected: 4 };
        assert_eq!(from_bridge("decode", &e).code, Code::PartialFrame);
        let e = BridgeError::Timeout { tool: "ffmpeg", context: "x".into(), seconds: 1.0, stderr: String::new() };
        assert_eq!(from_bridge("decode", &e).code, Code::StageTimeout);
        let e = BridgeError::UnknownPixFmt("x".into());
        assert_eq!(from_bridge("decode", &e).code, Code::StageFailed);
    }

    #[test]
    fn a_stalled_decode_or_encode_is_the_root_cause() {
        let spec = FrameSpec { width: 2, height: 2, depth: crate::format::Depth::U8 };
        let cfg = PipelineConfig {
            media: Media { ffmpeg: "ffmpeg".into(), ffprobe: "ffprobe".into() },
            input: "in.mp4".into(),
            video_ordinal: 0,
            decode_filter: String::new(),
            spec,
            rate: Rate::parse("24").unwrap(),
            encode_tail: Vec::new(),
            partial: "out.partial.mp4".into(),
            steps: Vec::new(),
            stage_timeout: Some(Duration::from_secs(2)),
        };
        let d = analyse(&cfg, &[], &[], &[], (true, false)).unwrap_err();
        assert_eq!((d.code, d.stage.as_deref(), d.message.as_str()), (Code::StageTimeout, Some("decode"), "decode moved no data for 2 seconds and was killed"));
        let e = analyse(&cfg, &[], &[], &[], (false, true)).unwrap_err();
        assert_eq!((e.code, e.stage.as_deref()), (Code::StageTimeout, Some("encode")));
        assert_eq!(cfg.out_spec(), spec);
    }

    #[test]
    fn seconds_print_without_a_trailing_zero() {
        assert_eq!(trim_seconds(2.0), "2");
        assert_eq!(trim_seconds(0.5), "0.5");
        assert_eq!(trim_seconds(120.0), "120");
    }

    #[test]
    fn a_failure_is_described_with_its_stage() {
        let e = failure_error("stage2", &Failure::Partial { got: 5, expected: 10, after: 3 }, &["x".into()]);
        assert_eq!(e.code, Code::PartialFrame);
        assert_eq!(e.message, "frame 3: stage2 ended its output with a partial frame (5 of 10 bytes) after 3 whole frames");
        assert_eq!(e.stderr_tail, ["x"]);
        let e = failure_error("stage1", &Failure::Stage { frame: 4, message: "unsupported: x".into() }, &[]);
        assert_eq!((e.code, e.message.as_str()), (Code::Unsupported, "frame 4: unsupported: x"));
        let e = failure_error("vfx", &Failure::Io("boom".into()), &[]);
        assert_eq!(e.code, Code::BadInput);
    }
}
