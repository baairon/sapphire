//! The ffmpeg filter stage `ffmpeg-filter` (`docs/handoff.md#the-ffmpeg-filter-stage`): one ffmpeg
//! filter graph over raw frames, on [`neutral_ffmpeg::Filter`]. ffmpeg stays upstream and
//! unmodified and always runs as a child process.
//!
//! | Part | Rule |
//! |---|---|
//! | Parameters | `graph` (an ffmpeg filter graph, default `null`), `expect` (`preserve` or `declared`, default `preserve`), `unsafe` (checkbox) |
//! | `expect preserve` | bytes out equal bytes in: the same size, rate and frame count; a graph that changes one of them is `unsupported` (exit 3) |
//! | `expect declared` | `--describe` measures width, height and frame count with a bounded dry run ([`DRY_RUN_FRAMES`] frames, at most [`DRY_RUN_SECONDS`] seconds of input) |
//! | Rate | an exact fraction; a graph that changes it is `unsupported` under either value |
//! | Safe filters | without `--param unsafe=1` a graph may only use the frame filters of [`SAFE_FILTERS`], which read and write nothing but the frames; any other filter needs it |
//! | Probing | `ffmpeg -version`, `-pix_fmts` and `-filters` run before use; [`MEASURED_VERSION`] is the measured version, another one is reported as unverified on standard error |
//! | Truncation | ffmpeg exits 0 on a truncated stream, so the stage counts whole frames and bytes itself: input that ends inside a frame is `partial_frame` (exit 4) |
//! | Colour | the colour matrix rule of `tools/apply.py`: frames of 1280 wide or 720 high and up are tagged `bt709`, smaller ones `smpte170m` |
//!
//! In the wrapper, `--stage "@ffmpeg-filter graph=..."` runs `sapphire stage apply
//! ffmpeg-filter ...` as a child process (the program is `SAPPHIRE_PATH` or the running
//! executable), so the stall watch judges it like any stage process.

use std::io::{self, Read, Write};
use std::path::Path;
use std::time::Duration;

use neutral_ffmpeg::{Binaries, Error as BridgeError, Filter, FrameBuf};

use crate::error::{tail_lines, Code, Error, Result};
use crate::format::FrameSpec;
use crate::geometry::Answer;
use crate::rate::Rate;
use crate::sys::run_short;

/// The stage id.
pub const ID: &str = "ffmpeg-filter";
/// The ffmpeg version the conventions were measured with.
pub const MEASURED_VERSION: &str = "9.0.2";
/// Frames of the bounded dry run, at most.
pub const DRY_RUN_FRAMES: u64 = 60;
/// Seconds of input of the bounded dry run, at most.
pub const DRY_RUN_SECONDS: u64 = 2;
/// The frame filters a graph may use without `unsafe=1`: they read and write nothing but the
/// frames. Any other filter needs it (the same list as twixtor's `tools/ffstage.py`).
pub const SAFE_FILTERS: [&str; 44] = [
    "boxblur",
    "colorbalance",
    "colorchannelmixer",
    "colorlevels",
    "copy",
    "crop",
    "deflicker",
    "drawbox",
    "drawgrid",
    "edgedetect",
    "eq",
    "fade",
    "format",
    "fps",
    "framerate",
    "gblur",
    "geq",
    "hflip",
    "hue",
    "lut",
    "lutrgb",
    "lutyuv",
    "minterpolate",
    "negate",
    "noise",
    "null",
    "pad",
    "reverse",
    "rotate",
    "scale",
    "select",
    "setdar",
    "setparams",
    "setpts",
    "setsar",
    "smartblur",
    "sobel",
    "tblend",
    "tmix",
    "transpose",
    "trim",
    "unsharp",
    "vflip",
    "vignette",
];
/// How long one probe of ffmpeg or the dry run may take.
const PROBE_TIMEOUT: Duration = Duration::from_secs(60);

/// The value of the `expect` parameter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Expect {
    /// Same size, rate and frame count.
    Preserve,
    /// The size and frame count are measured by `--describe`.
    Declared,
}

/// The parameters of one run of the stage.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Options {
    /// The filter graph.
    pub graph: String,
    /// `preserve` or `declared`.
    pub expect: Expect,
    /// `unsafe=1`: filters outside [`SAFE_FILTERS`] are allowed.
    pub allow_unsafe: bool,
}

impl Options {
    /// The options from `--param` pairs already checked by [`crate::stage::check_params`].
    pub fn from_values(values: &[(String, String)]) -> Result<Options> {
        let get = |key: &str| values.iter().find(|(k, _)| k == key).map(|(_, v)| v.as_str());
        let graph = get("graph").unwrap_or("null").trim().to_string();
        if graph.is_empty() {
            return Err(Error::usage("`graph` is empty; give an ffmpeg filter graph, for example graph=negate"));
        }
        let expect = match get("expect").unwrap_or("preserve") {
            "declared" => Expect::Declared,
            _ => Expect::Preserve,
        };
        let allow_unsafe = get("unsafe").and_then(crate::stage::parse_bool).unwrap_or(false);
        Ok(Options { graph, expect, allow_unsafe })
    }
}

/// The program that runs the stage for the wrapper: `SAPPHIRE_PATH`, else the running
/// executable.
pub fn handoff_exe() -> String {
    match std::env::var("SAPPHIRE_PATH") {
        Ok(p) if !p.is_empty() => p,
        _ => crate::stage::program(),
    }
}

/// The command line the wrapper runs for `@ffmpeg-filter` with `params` (`K=V` words).
pub fn command_line(spec: FrameSpec, rate: &Rate, _frames: Option<u64>, params: &[String], ffmpeg: &Path) -> Vec<String> {
    let mut argv = vec![
        handoff_exe(),
        "stage".into(),
        "apply".into(),
        ID.into(),
        "--width".into(),
        spec.width.to_string(),
        "--height".into(),
        spec.height.to_string(),
        "--depth".into(),
        spec.depth.bits().to_string(),
        "--fps".into(),
        rate.text(),
        "--ffmpeg-path".into(),
        ffmpeg.to_string_lossy().into_owned(),
    ];
    for p in params {
        argv.extend(["--param".to_string(), p.clone()]);
    }
    argv
}

/// The colour matrix of `tools/apply.py` for an untagged source of this size.
pub fn colour_matrix(width: u32, height: u32) -> &'static str {
    if width >= 1280 || height >= 720 {
        "bt709"
    } else {
        "smpte170m"
    }
}

/// Split `text` at the characters of `seps` outside single quotes; a backslash keeps the next
/// character. Quotes and escapes stay in the pieces.
fn split_unquoted(text: &str, seps: &[char]) -> Vec<String> {
    let mut pieces: Vec<String> = vec![String::new()];
    let mut quoted = false;
    let mut chars = text.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                pieces.last_mut().expect("one").push(c);
                if let Some(next) = chars.next() {
                    pieces.last_mut().expect("one").push(next);
                }
            }
            '\'' => {
                quoted = !quoted;
                pieces.last_mut().expect("one").push(c);
            }
            c if !quoted && seps.contains(&c) => pieces.push(String::new()),
            _ => pieces.last_mut().expect("one").push(c),
        }
    }
    pieces
}

/// The byte index of the first character of `targets` outside single quotes and escapes.
fn find_unquoted(text: &str, targets: &[char]) -> Option<usize> {
    let (mut quoted, mut escaped) = (false, false);
    for (i, c) in text.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match c {
            '\\' => escaped = true,
            '\'' => quoted = !quoted,
            c if !quoted && targets.contains(&c) => return Some(i),
            _ => {}
        }
    }
    None
}

/// One filter of a graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GraphFilter {
    /// The filter name (labels and the `@` instance name removed).
    pub name: String,
    /// The keys of its `key=value` options, in order; a positional value has none. The text right
    /// after `name=` is the first option, so `drawtext=fontfile=f.ttf` has the key `fontfile`.
    pub keys: Vec<String>,
}

/// The filters of a graph, in order: pad labels, instance names, quoted and escaped text are
/// skipped; the options end at the first output label.
pub fn parse_graph(graph: &str) -> Vec<GraphFilter> {
    let mut filters = Vec::new();
    for seg in split_unquoted(graph, &[',', ';']) {
        let mut s = seg.trim();
        while let Some(rest) = s.strip_prefix('[') {
            s = rest.split_once(']').map(|(_, r)| r).unwrap_or("").trim_start();
        }
        let end = s.find(|c: char| c == '=' || c == '@' || c == '[' || c.is_whitespace()).unwrap_or(s.len());
        if end == 0 {
            continue;
        }
        let mut rest = &s[end..];
        if rest.starts_with('@') {
            rest = &rest[rest.find(|c: char| c == '=' || c == '[').unwrap_or(rest.len())..];
        }
        let mut keys = Vec::new();
        if let Some(args) = rest.trim_start().strip_prefix('=') {
            let args = &args[..find_unquoted(args, &['[', ']']).unwrap_or(args.len())];
            for item in split_unquoted(args, &[':']) {
                if let Some(eq) = find_unquoted(&item, &['=']) {
                    keys.push(item[..eq].trim().to_string());
                }
            }
        }
        filters.push(GraphFilter { name: s[..end].to_string(), keys });
    }
    filters
}

/// The filter names of a graph, in order (labels, instance names and options removed; quoted
/// and escaped text is skipped).
pub fn filter_names(graph: &str) -> Vec<String> {
    parse_graph(graph).into_iter().map(|f| f.name).collect()
}

/// The filters of a graph outside [`SAFE_FILTERS`], each once in the order they appear: what
/// needs `unsafe=1`. Empty for a graph of safe filters.
pub fn unsafe_filters(graph: &str) -> Vec<String> {
    let mut found: Vec<String> = Vec::new();
    for name in filter_names(graph) {
        if !SAFE_FILTERS.contains(&name.as_str()) && !found.contains(&name) {
            found.push(name);
        }
    }
    found
}

/// What a probe of ffmpeg found.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Caps {
    /// The version word of `ffmpeg -version`.
    pub version: String,
    /// Pixel formats ffmpeg can both read and write.
    pub pix_fmts: Vec<String>,
    /// Filter names.
    pub filters: Vec<String>,
}

impl Caps {
    /// Whether this is the measured version; a leading `n` (the git tag form of builds such as
    /// `n9.0.2-...`) is read past, as twixtor's `tools/ffstage.py` does.
    pub fn verified(&self) -> bool {
        let version = self.version.strip_prefix('n').unwrap_or(&self.version);
        version == MEASURED_VERSION || version.starts_with(&format!("{}-", MEASURED_VERSION))
    }

    /// Read the three probe outputs.
    pub fn parse(version: &str, pix_fmts: &str, filters: &str) -> Caps {
        let version = version.lines().next().and_then(|l| l.split_whitespace().nth(2)).unwrap_or("unknown").to_string();
        let pix_fmts = pix_fmts
            .lines()
            .filter_map(|l| {
                let mut it = l.split_whitespace();
                let flags = it.next()?;
                let name = it.next()?;
                (flags.len() == 5 && flags.starts_with("IO")).then(|| name.to_string())
            })
            .collect();
        let filters = filters
            .lines()
            .skip_while(|l| !l.trim_start().starts_with("------"))
            .skip(1)
            .filter_map(|l| l.split_whitespace().nth(1).map(|s| s.to_string()))
            .collect();
        Caps { version, pix_fmts, filters }
    }
}

/// Run `ffmpeg -version`, `-pix_fmts` and `-filters`.
pub fn probe(ffmpeg: &Path) -> Result<Caps> {
    let mut texts = Vec::new();
    for flag in ["-version", "-pix_fmts", "-filters"] {
        let ran = run_short(ffmpeg, &["-hide_banner".to_string(), flag.to_string()], PROBE_TIMEOUT);
        match ran.code {
            0 => texts.push(ran.stdout),
            127 => return Err(Error::tool_missing(format!("cannot start ffmpeg at {}", ffmpeg.display()))),
            _ => return Err(Error::stage_failed(format!("`ffmpeg {}` failed with code {}", flag, ran.code)).with_tail(tail_lines(&ran.stderr, 3))),
        }
    }
    Ok(Caps::parse(&texts[0], &texts[1], &texts[2]))
}

/// Refuse what this ffmpeg or this request cannot do: an unknown pixel format or filter, or a
/// graph with a filter outside [`SAFE_FILTERS`] without `unsafe=1`.
pub fn check(caps: &Caps, opts: &Options, spec: FrameSpec) -> Result<()> {
    let fmt = spec.depth.pix_fmt_name();
    if !caps.pix_fmts.iter().any(|p| p == fmt) {
        return Err(Error::unsupported(format!("ffmpeg {} cannot read and write {}", caps.version, fmt)));
    }
    for name in filter_names(&opts.graph) {
        if !caps.filters.iter().any(|f| *f == name) {
            return Err(Error::unsupported(format!("ffmpeg {} has no filter `{}`", caps.version, name)));
        }
    }
    let outside = unsafe_filters(&opts.graph);
    if !outside.is_empty() && !opts.allow_unsafe {
        return Err(Error::unsupported(format!(
            "the graph uses filters outside the safe list ({}); pass --param unsafe=1 to allow it",
            outside.join(", ")
        )));
    }
    Ok(())
}

/// Frames of the dry run for `rate`: two seconds of input, at most 60 frames, at least 1.
pub fn dry_run_frames(rate: &Rate) -> u64 {
    let two_seconds = (DRY_RUN_SECONDS as u128 * rate.num() / rate.den().max(1)) as u64;
    two_seconds.clamp(1, DRY_RUN_FRAMES)
}

/// What the dry run measured.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Measured {
    /// Output width and height.
    pub size: (u32, u32),
    /// Output frames for the dry run's input frames.
    pub frames: u64,
    /// Bytes of the first output frame.
    pub frame_bytes: Option<u64>,
    /// The output rate is the input rate.
    pub same_rate: bool,
}

/// Read the `framemd5` text of the dry run: `#tb`, `#dimensions` and one line per frame.
pub fn parse_framemd5(text: &str, rate: &Rate) -> Option<Measured> {
    let mut tb: Option<(u128, u128)> = None;
    let mut size: Option<(u32, u32)> = None;
    let mut rows: Vec<(i128, i128, u64)> = Vec::new();
    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("#tb 0:") {
            let (a, b) = rest.trim().split_once('/')?;
            tb = Some((a.trim().parse().ok()?, b.trim().parse().ok()?));
        } else if let Some(rest) = line.strip_prefix("#dimensions 0:") {
            let (w, h) = rest.trim().split_once('x')?;
            size = Some((w.trim().parse().ok()?, h.trim().parse().ok()?));
        } else if !line.starts_with('#') && !line.trim().is_empty() {
            let cols: Vec<&str> = line.split(',').map(|c| c.trim()).collect();
            if cols.len() >= 6 && cols[0] == "0" {
                rows.push((cols[2].parse().ok()?, cols[3].parse().ok()?, cols[4].parse().ok()?));
            }
        }
    }
    let (a, b) = tb?;
    // one output frame lasts `delta` ticks of a/b seconds; the input frame lasts den/num seconds
    let delta = match rows.as_slice() {
        [first, second, ..] => (second.0 - first.0).max(0) as u128,
        [only] => only.1.max(0) as u128,
        [] => 0,
    };
    let same_rate = delta == 0 || a * delta * rate.num() == b * rate.den();
    Some(Measured { size: size?, frames: rows.len() as u64, frame_bytes: rows.first().map(|r| r.2), same_rate })
}

/// The bounded dry run: [`dry_run_frames`] grey frames of `spec` through the graph, measured.
pub fn measure(spec: FrameSpec, rate: &Rate, opts: &Options, ffmpeg: &Path) -> Result<(Measured, u64)> {
    let k = dry_run_frames(rate);
    let fmt = spec.depth.pix_fmt_name();
    let source = format!("color=c=gray:s={}x{}:r={},format={},trim=end_frame={}", spec.width, spec.height, rate.text(), fmt, k);
    let args: Vec<String> = [
        "-hide_banner", "-nostdin", "-v", "error", "-f", "lavfi", "-i", &source, "-vf", &format!("{},format={}", opts.graph, fmt), "-f", "framemd5", "-",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let ran = run_short(ffmpeg, &args, PROBE_TIMEOUT);
    match ran.code {
        0 => {}
        127 => return Err(Error::tool_missing(format!("cannot start ffmpeg at {}", ffmpeg.display()))),
        124 => return Err(Error::new(Code::StageTimeout, format!("the dry run of the graph did not end within {} seconds", PROBE_TIMEOUT.as_secs()))),
        n => {
            return Err(Error::unsupported(format!("ffmpeg refused the graph `{}` (code {})", opts.graph, n)).with_tail(tail_lines(&ran.stderr, 3)));
        }
    }
    let measured = parse_framemd5(&ran.stdout, rate).ok_or_else(|| Error::stage_failed("the dry run of the graph printed no frame checksums"))?;
    Ok((measured, k))
}

/// The `--describe` answer: probe ffmpeg, check the request, run the bounded dry run and refuse
/// a rate change (and, under `expect preserve`, any change).
pub fn describe(spec: FrameSpec, rate: &Rate, frames: Option<u64>, opts: &Options, ffmpeg: &Path) -> Result<Answer> {
    describe_probed(spec, rate, frames, opts, ffmpeg).map(|(answer, _)| answer)
}

/// [`describe`], also returning what the probe found.
fn describe_probed(spec: FrameSpec, rate: &Rate, frames: Option<u64>, opts: &Options, ffmpeg: &Path) -> Result<(Answer, Caps)> {
    let caps = probe(ffmpeg)?;
    check(&caps, opts, spec)?;
    let (m, k) = measure(spec, rate, opts, ffmpeg)?;
    if !m.same_rate {
        return Err(Error::unsupported(format!("the graph `{}` changes the frame rate; a rate change is not supported", opts.graph)));
    }
    let (w, h) = m.size;
    if w == 0 || h == 0 {
        return Err(Error::unsupported(format!("the graph `{}` gives frames of {}x{}", opts.graph, w, h)));
    }
    let out_spec = FrameSpec { width: w, height: h, depth: spec.depth };
    if let Some(bytes) = m.frame_bytes {
        if bytes != out_spec.frame_bytes() {
            return Err(Error::unsupported(format!("the graph `{}` gives {} byte frames, not {}", opts.graph, bytes, spec.depth.pix_fmt_name())));
        }
    }
    let same_count = m.frames == k;
    if opts.expect == Expect::Preserve && ((w, h) != (spec.width, spec.height) || !same_count) {
        return Err(Error::unsupported(format!(
            "the graph `{}` changes the frames ({}x{} to {}x{}, {} to {} frames in the dry run); pass --param expect=declared",
            opts.graph, spec.width, spec.height, w, h, k, m.frames
        )));
    }
    let answer = Answer { stage: ID.to_string(), width: w, height: h, depth: spec.depth, fps: *rate, frames: if same_count { frames } else { None } };
    Ok((answer, caps))
}

fn bridge_error(e: &BridgeError) -> Error {
    match e {
        BridgeError::Spawn { .. } | BridgeError::ToolNotFound { .. } => Error::tool_missing(e.to_string()),
        BridgeError::TruncatedFrame { .. } => Error::new(Code::PartialFrame, format!("ffmpeg ended its output inside a frame: {}", e)),
        BridgeError::Ffmpeg { stderr, .. } => Error::stage_failed(format!("ffmpeg failed: {}", e)).with_tail(tail_lines(stderr, 3)),
        other => Error::stage_failed(other.to_string()),
    }
}

fn write_frame<W: Write>(output: &mut W, frame: &FrameBuf, index: u64) -> Result<()> {
    output.write_all(frame.as_bytes()).map_err(|e| {
        let code = if e.kind() == io::ErrorKind::BrokenPipe { Code::BrokenPipe } else { Code::BadInput };
        Error::new(code, format!("frame {}: cannot write the output: {}", index, e))
    })
}

/// Run the graph over whole frames from `input` to `output`; returns the frames written.
///
/// The input is counted frame by frame and byte by byte (ffmpeg exits 0 on a truncated
/// stream): a stream that ends inside a frame is `partial_frame`. Under `expect preserve` the
/// output must hold as many frames of the same size as the input. `expect_frames` checks the
/// input count, as `--frames` does for every stage.
pub fn apply<R: Read, W: Write>(
    spec: FrameSpec,
    rate: &Rate,
    expect_frames: Option<u64>,
    opts: &Options,
    ffmpeg: &Path,
    input: &mut R,
    output: &mut W,
) -> Result<u64> {
    let (answer, caps) = describe_probed(spec, rate, expect_frames, opts, ffmpeg)?;
    if !caps.verified() {
        eprintln!("warning: ffmpeg is not the measured version {}; this run is unverified", MEASURED_VERSION);
    }
    let (num, den) = rate.as_u32_pair().ok_or_else(|| Error::unsupported(format!("the rate {} does not fit ffmpeg", rate.text())))?;
    let fmt = spec.depth.pix_fmt();
    let mut builder = Filter::builder(fmt, spec.width, spec.height)
        .binaries(Binaries::new().with_ffmpeg(ffmpeg))
        .graph(opts.graph.clone())
        .out_pix_fmt(fmt)
        .fps(num, den)
        .input_args(["-colorspace", colour_matrix(spec.width, spec.height)]);
    if opts.expect == Expect::Preserve {
        builder = builder.size_unchanged();
    }
    let mut filter = builder.spawn().map_err(|e| bridge_error(&e))?;
    if filter.out_size() != (answer.width, answer.height) {
        return Err(Error::unsupported(format!("the graph gives {:?}, the dry run measured {}x{}", filter.out_size(), answer.width, answer.height)));
    }
    let bytes = spec.frame_bytes() as usize;
    let mut buf = vec![0u8; bytes];
    let (mut read, mut written) = (0u64, 0u64);
    loop {
        let mut got = 0usize;
        while got < bytes {
            match input.read(&mut buf[got..]) {
                Ok(0) => break,
                Ok(n) => got += n,
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                Err(e) => return Err(Error::bad_input(format!("frame {}: cannot read the input: {}", read, e))),
            }
        }
        if got == 0 {
            break;
        }
        if got < bytes {
            return Err(Error::new(
                Code::PartialFrame,
                format!("frame {}: input ends with a partial frame ({} of {} bytes) after {} whole frames", read, got, bytes, read),
            ));
        }
        let frame = FrameBuf::from_slice(fmt, spec.width, spec.height, &buf).map_err(|e| bridge_error(&e))?;
        filter.push(&frame).map_err(|e| bridge_error(&e))?;
        read += 1;
        while let Some(out) = filter.try_pull().map_err(|e| bridge_error(&e))? {
            write_frame(output, &out, written)?;
            written += 1;
        }
    }
    filter.close_input();
    while let Some(out) = filter.pull().map_err(|e| bridge_error(&e))? {
        write_frame(output, &out, written)?;
        written += 1;
    }
    output.flush().map_err(|e| Error::new(if e.kind() == io::ErrorKind::BrokenPipe { Code::BrokenPipe } else { Code::BadInput }, format!("cannot flush the output: {}", e)))?;
    if let Some(want) = expect_frames {
        if want != read {
            return Err(Error::bad_input(format!("the input holds {} frames, but --frames says {}", read, want)));
        }
    }
    if opts.expect == Expect::Preserve && written != read {
        return Err(Error::unsupported(format!("the graph `{}` gave {} frames for {}; pass --param expect=declared", opts.graph, written, read)));
    }
    Ok(written)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::format::Depth;

    #[test]
    fn filter_names_skip_labels_options_and_quotes() {
        assert_eq!(filter_names("negate"), ["negate"]);
        assert_eq!(filter_names("[in]scale=40:24[a];[a]hue@x=s=0,select='eq(n\\,0)+lt(n,3)'"), ["scale", "hue", "select"]);
        assert_eq!(filter_names(" format=rgba , null "), ["format", "null"]);
    }

    #[test]
    fn the_safe_list_finds_the_filters_outside_it() {
        let keys = |graph: &str| parse_graph(graph).into_iter().map(|f| f.keys).collect::<Vec<_>>();
        assert_eq!(keys("drawtext=fontfile=f.ttf:text='a=b':24"), [vec!["fontfile".to_string(), "text".to_string()]]);
        assert_eq!(keys("[in]scale=2:2[a];[a]hue@x=s=0[out]"), [vec![], vec!["s".to_string()]]);
        assert_eq!(unsafe_filters("movie=x.mp4[a];[in][a]overlay"), ["movie", "overlay"]);
        assert_eq!(unsafe_filters("hflip,lut3d=a.cube,psnr@p,lut3d"), ["lut3d", "psnr"]);
        assert_eq!(unsafe_filters("drawtext=text='a'"), ["drawtext"]);
        for graph in [
            "transpose=dir=clock",
            "hflip",
            "scale=2:2",
            "null",
            "scale=w=2:h=2:flags=neighbor",
            "hue@x=s=0",
            "[in]null[out]",
            "select='eq(n\\,0)+lt(n,3)'",
            "format=rgba,setparams=range=pc",
        ] {
            assert!(unsafe_filters(graph).is_empty(), "{}: {:?}", graph, unsafe_filters(graph));
        }
        for name in ["movie", "amovie", "sendcmd", "zmq", "lut3d", "subtitles", "drawtext", "curves", "overlay"] {
            assert!(!SAFE_FILTERS.contains(&name), "{}", name);
        }
    }

    #[test]
    fn filters_outside_the_safe_list_need_unsafe() {
        let names = ["movie", "drawtext", "negate", "sendcmd", "lut3d", "subtitles", "psnr", "hflip", "scale", "transpose"];
        let caps = Caps { version: "9.0.2".into(), pix_fmts: vec!["rgba".into()], filters: names.iter().map(|s| s.to_string()).collect() };
        let spec = FrameSpec { width: 2, height: 2, depth: Depth::U8 };
        let opts = |graph: &str, allow: bool| Options { graph: graph.into(), expect: Expect::Preserve, allow_unsafe: allow };
        for graph in ["movie=a.mp4", "sendcmd=f=cmds.txt,negate", "drawtext=text=a", "lut3d=look.cube", "subtitles=s.srt", "psnr"] {
            let e = check(&caps, &opts(graph, false), spec).unwrap_err();
            assert_eq!(e.code, Code::Unsupported, "{}", graph);
            assert!(e.message.contains("unsafe=1"), "{}", e.message);
            assert!(check(&caps, &opts(graph, true), spec).is_ok(), "{}", graph);
        }
        for graph in ["negate", "hflip", "scale=2:2", "scale=w=2:h=2,hflip", "transpose=dir=clock"] {
            assert!(check(&caps, &opts(graph, false), spec).is_ok(), "{}", graph);
        }
        assert!(check(&caps, &opts("nosuchfilter", false), spec).unwrap_err().message.contains("has no filter `nosuchfilter`"));
        let deep = FrameSpec { width: 2, height: 2, depth: Depth::U16 };
        assert!(check(&caps, &opts("negate", false), deep).unwrap_err().message.contains("rgba64le"));
    }

    #[test]
    fn probes_are_parsed() {
        let caps = Caps::parse(
            "ffmpeg version 9.0.2-full_build-www.gyan.dev Copyright (c) 2000-2026\n",
            "Pixel formats:\nI.... = Supported Input\n-----\nIO... rgba                   4             32      8-8-8-8\n..H.. vaapi 0 0 0\n",
            "Filters:\n  T.. = Timeline\n  ------\n TS aap  AA->A  x\n .. negate V->V Negate.\n",
        );
        assert_eq!(caps.version, "9.0.2-full_build-www.gyan.dev");
        assert!(caps.verified());
        assert_eq!(caps.pix_fmts, ["rgba"]);
        assert_eq!(caps.filters, ["aap", "negate"]);
        assert!(!Caps { version: "8.1".into(), pix_fmts: vec![], filters: vec![] }.verified());
        assert!(Caps { version: "n9.0.2-12-gabcdef0".into(), pix_fmts: vec![], filters: vec![] }.verified());
        assert!(!Caps { version: "n9.0.1-3-gabcdef0".into(), pix_fmts: vec![], filters: vec![] }.verified());
    }

    #[test]
    fn rates_are_exact_and_the_dry_run_is_bounded() {
        assert_eq!(dry_run_frames(&Rate::parse("30000/1001").unwrap()), 59);
        assert_eq!(dry_run_frames(&Rate::parse("24/1").unwrap()), 48);
        assert_eq!(dry_run_frames(&Rate::parse("120/1").unwrap()), 60);
        assert_eq!(dry_run_frames(&Rate::parse("1/10").unwrap()), 1);
        let text = "#tb 0: 1001/30000\n#dimensions 0: 40x24\n0, 0, 0, 1, 3840, x\n0, 1, 1, 1, 3840, y\n";
        let m = parse_framemd5(text, &Rate::parse("30000/1001").unwrap()).unwrap();
        assert_eq!((m.size, m.frames, m.frame_bytes, m.same_rate), ((40, 24), 2, Some(3840), true));
        let doubled = text.replace("1001/30000", "1/60");
        assert!(!parse_framemd5(&doubled, &Rate::parse("30000/1001").unwrap()).unwrap().same_rate);
        assert_eq!(colour_matrix(1280, 2), "bt709");
        assert_eq!(colour_matrix(2, 720), "bt709");
        assert_eq!(colour_matrix(1279, 719), "smpte170m");
    }

    #[test]
    fn options_come_from_checked_params() {
        let v = |pairs: &[(&str, &str)]| pairs.iter().map(|(k, x)| (k.to_string(), x.to_string())).collect::<Vec<_>>();
        let o = Options::from_values(&v(&[("graph", "negate"), ("expect", "declared"), ("unsafe", "on")])).unwrap();
        assert_eq!(o, Options { graph: "negate".into(), expect: Expect::Declared, allow_unsafe: true });
        assert_eq!(Options::from_values(&[]).unwrap().graph, "null");
        assert_eq!(Options::from_values(&v(&[("graph", "  ")])).unwrap_err().code, Code::Usage);
    }
}
