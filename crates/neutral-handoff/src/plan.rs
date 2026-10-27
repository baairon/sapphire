//! The plan: what a run does, decided before any frame moves, and the commands it runs.
//!
//! [`Request`] holds the arguments of a run (the flags of `tools/apply.py` plus `--stage` and
//! `--stage-timeout`). [`prepare`] makes every check `apply.py` makes, in its order, and returns
//! a [`Prepared`] run; [`Prepared::assemble`] names the scratch folder, the layer clips and the
//! command of every stage. The ffmpeg and `vfx` command lines are those of `apply.py`, word for
//! word, so `--dry-run` prints the same `[plan]`, `[temp]` and `[cmd]` lines.
//!
//! The pieces, as `apply.py` decides them:
//!
//! | Question | Rule |
//! |---|---|
//! | Frame size | the displayed size: a 90 or 270 degree rotation swaps the sides |
//! | Frame rate | `r_frame_rate`; when it differs from `avg_frame_rate` by more than a thousandth the input is variable rate, the rate is `avg_frame_rate` and the decode adds `fps=N/D`; the rate is always an exact fraction |
//! | Colour matrix | the source's own `color_space` when it is one of bt709, bt470bg, smpte170m, bt2020nc, smpte240m, fcc; else bt709 when the longer side is at least 1280 or the height at least 720, else smpte170m, and the tags are written to match |
//! | Refused | no video stream, an unreadable size or rate, HDR (`smpte2084`, `arib-std-b67`), `--lossless` at depth 32, an odd side on the lossy path |
//! | Encoder | libx264 CRF 18 yuv420p, tv range, with the matrix and tags; or ffv1 level 3 in RGB under `--lossless`; audio copied; `-movflags +faststart` for `.mp4`, `.m4v`, `.mov` |
//!
//! The chain of stages between decode and encode is the effect (`EFFECT`, run by `vfx apply`)
//! and the `--stage` commands. Without a marker the effect comes first; a stage that is exactly
//! `vfx` marks where it goes instead; a stage that starts with `@` is an in-process stage of
//! this crate (`@selftest-invert`, optionally followed by `K=V` parameters).

use std::path::PathBuf;

use crate::error::{Code, Contract, Error, Result};
use crate::format::{Depth, FrameSpec};
use crate::media::{probe, Info};
use crate::paths::{default_output, discard, lexists, partial_path, same_file};
use crate::pyjson::Out;
use crate::pytext::{basename, format_command, split_command};
use crate::rate::Rate;
use crate::tools::{find_vfx_default, process_env, require_media, require_program, EnvFn, Media};
use crate::vfx::{effect_info, preflight, EffectInfo, Kind};

/// The default for `--stage-timeout`, in seconds.
pub const DEFAULT_STAGE_TIMEOUT: f64 = 120.0;

/// The arguments of a run.
#[derive(Clone, Debug, Default)]
pub struct Request {
    /// The video file.
    pub input: String,
    /// The `vfx` effect id; `None` runs the `--stage` commands alone.
    pub effect: Option<String>,
    /// `--param K=V`, repeatable.
    pub params: Vec<String>,
    /// `--layer ID=VIDEO`, repeatable.
    pub layers: Vec<(String, String)>,
    /// `--frames N`.
    pub frames: Option<u64>,
    /// `--depth`, 16 when `None`.
    pub depth: Option<Depth>,
    /// `--lossless`.
    pub lossless: bool,
    /// `--vfx PATH`.
    pub vfx: Option<String>,
    /// `-o PATH`.
    pub output: Option<String>,
    /// `--dry-run`.
    pub dry_run: bool,
    /// `--json`.
    pub json: bool,
    /// `--force`.
    pub force: bool,
    /// `--ffmpeg-path`.
    pub ffmpeg_path: Option<String>,
    /// `--ffprobe-path`.
    pub ffprobe_path: Option<String>,
    /// `--stage CMD`, repeatable, in order.
    pub stages: Vec<String>,
    /// `--stage-timeout SECONDS`; `None` means the default, 0 disables.
    pub stage_timeout: Option<f64>,
    /// `--contract frames/1|2`; frames/2 when not given.
    pub contract: Contract,
    /// `--geometry VALUE` given after the `--stage` at the same position (frames/2), as in
    /// `tools/apply.py`; it overrides what the stage's listing says.
    pub stage_geometry: Vec<Option<String>>,
    /// `--describe` (frames/2): print the output geometry of the run and stop.
    pub describe: bool,
}

impl Request {
    /// A request for `input` and `effect` with everything else at its default.
    pub fn new(input: impl Into<String>, effect: Option<&str>) -> Request {
        Request { input: input.into(), effect: effect.map(|e| e.to_string()), ..Request::default() }
    }

    /// The depth in force.
    pub fn depth(&self) -> Depth {
        self.depth.unwrap_or(Depth::U16)
    }

    /// The stage timeout in force, in seconds; 0 means never.
    pub fn stage_timeout(&self) -> f64 {
        self.stage_timeout.unwrap_or(DEFAULT_STAGE_TIMEOUT)
    }
}

/// A `--layer` video that was probed.
#[derive(Clone, Debug)]
pub struct LayerProbe {
    /// The layer id.
    pub id: String,
    /// The absolute path of the video.
    pub video: String,
    /// What ffprobe says about it.
    pub info: Info,
}

/// What kind of step a stage of the chain is.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StepKind {
    /// `vfx apply EFFECT`.
    Vfx,
    /// An external command.
    External,
    /// An in-process stage of this crate: its id and its `K=V` parameters.
    Builtin {
        /// The stage id.
        id: String,
        /// The `K=V` parameters.
        params: Vec<String>,
    },
}

/// One stage of the chain with its command line.
#[derive(Clone, Debug)]
pub struct Step {
    /// A label, not an identifier: `vfx` for the effect, `pre1`, `pre2`, ... for the `--stage`
    /// commands before the `vfx` marker and `post1`, `post2`, ... for those after it, as
    /// `tools/apply.py` names its `--pre-stage` and `--post-stage` steps. Without a marker the
    /// effect runs first (every stage is `post`); without an effect every stage is `pre`.
    pub name: String,
    /// What runs.
    pub kind: StepKind,
    /// The program and its arguments, placeholders expanded.
    pub argv: Vec<String>,
    /// Frames this step is given, when known.
    pub frames_in: Option<u64>,
    /// Frames this step must return, when known.
    pub frames_out: Option<u64>,
    /// The frames this step reads.
    pub spec_in: FrameSpec,
    /// The frames this step writes: equal to `spec_in` unless the stage declares its geometry.
    pub spec_out: FrameSpec,
}

impl Step {
    /// The report role (`contracts/run-report.schema.json`): `effect`, `pre` or `post`.
    pub fn role(&self) -> &'static str {
        if self.kind == StepKind::Vfx {
            "effect"
        } else if self.name.starts_with("post") {
            "post"
        } else {
            "pre"
        }
    }
}

/// The report name of the `index`th stage (1 based) of `role` `pre` or `post`.
pub fn step_name(role: &str, index: usize) -> String {
    format!("{}{}", role, index)
}

/// The placeholders of a `--stage` command.
#[derive(Clone, Debug)]
pub struct StageVars {
    /// `{width}`
    pub width: u32,
    /// `{height}`
    pub height: u32,
    /// `{depth}`
    pub depth: u32,
    /// `{fps}`, as `N/D`
    pub fps: String,
    /// `{frames}`, when known
    pub frames: Option<u64>,
    /// `{pix_fmt}`
    pub pix_fmt: &'static str,
}

/// The six placeholders of a stage command.
pub const PLACEHOLDERS: [&str; 6] = ["width", "height", "depth", "fps", "frames", "pix_fmt"];

/// Expand `{width}`, `{height}`, `{depth}`, `{fps}`, `{frames}` and `{pix_fmt}` in one argument.
///
/// Anything else in braces is left alone, so that a command can carry code or JSON, as in
/// `tools/apply.py`. `{frames}` is an error when the frame count is not known.
pub fn expand_placeholders(token: &str, vars: &StageVars) -> std::result::Result<String, String> {
    let mut out = String::new();
    let mut rest = token;
    while let Some(open) = rest.find('{') {
        out.push_str(&rest[..open]);
        let after = &rest[open + 1..];
        let named = after.find('}').map(|close| (&after[..close], close)).filter(|(name, _)| PLACEHOLDERS.contains(name));
        match named {
            Some((name, close)) => {
                match name {
                    "width" => out.push_str(&vars.width.to_string()),
                    "height" => out.push_str(&vars.height.to_string()),
                    "depth" => out.push_str(&vars.depth.to_string()),
                    "fps" => out.push_str(&vars.fps),
                    "pix_fmt" => out.push_str(vars.pix_fmt),
                    _ => match vars.frames {
                        Some(n) => out.push_str(&n.to_string()),
                        None => {
                            return Err("{frames} is not known for this input: it needs a constant frame rate input whose container states the frame count (or, after the effect, --frames)".to_string())
                        }
                    },
                }
                rest = &after[close + 1..];
            }
            None => {
                out.push('{');
                rest = after;
            }
        }
    }
    out.push_str(rest);
    Ok(out)
}

/// The ffmpeg matrix name and the tags to write, from the probe (`color_settings` of `apply.py`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Colour {
    /// swscale's name for the matrix.
    pub matrix: &'static str,
    /// The `colorspace` tag.
    pub space: String,
    /// The `color_primaries` tag, when the source had one.
    pub primaries: Option<String>,
    /// The `color_trc` tag, when the source had one.
    pub trc: Option<String>,
}

fn matrix_for(space: &str) -> Option<&'static str> {
    match space {
        "bt709" => Some("bt709"),
        "bt470bg" => Some("bt601"),
        "smpte170m" => Some("smpte170m"),
        "bt2020nc" => Some("bt2020"),
        "smpte240m" => Some("smpte240m"),
        "fcc" => Some("fcc"),
        _ => None,
    }
}

/// A tagged source keeps its matrix; an untagged one follows the size convention of players.
pub fn colour_settings(info: &Info) -> Colour {
    if let Some(space) = info.color_space.as_deref() {
        if let Some(matrix) = matrix_for(space) {
            return Colour { matrix, space: space.to_string(), primaries: info.color_primaries.clone(), trc: info.color_transfer.clone() };
        }
    }
    let (w, h) = info.display_size();
    let space = if w.max(h) >= 1280 || h >= 720 { "bt709" } else { "smpte170m" };
    Colour { matrix: matrix_for(space).expect("a known name"), space: space.to_string(), primaries: Some(space.to_string()), trc: Some(space.to_string()) }
}

/// What the output does to the audio when it has `frames` frames, judged by lengths alone.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioPlan {
    /// The copy is cut at the end of the video (`-shortest`).
    Shortest,
    /// The audio ends before the video does.
    EndsEarly,
    /// The audio is kept as it is.
    Same,
}

impl AudioPlan {
    /// `shortest`, `ends-early` or `same`.
    pub fn as_str(self) -> &'static str {
        match self {
            AudioPlan::Shortest => "shortest",
            AudioPlan::EndsEarly => "ends-early",
            AudioPlan::Same => "same",
        }
    }
}

/// `audio_plan` of `apply.py`.
pub fn audio_plan(info: &Info, frames: u64, rate: &Rate) -> Option<AudioPlan> {
    if !info.has_audio {
        return None;
    }
    let audio = match info.audio_duration.filter(|d| *d != 0.0).or(info.duration) {
        Some(a) => a,
        None => return Some(AudioPlan::Same),
    };
    let fps = rate.as_f64();
    let (video, slack) = (frames as f64 / fps, 1.0 / fps);
    Some(if video + slack < audio {
        AudioPlan::Shortest
    } else if video > audio + slack {
        AudioPlan::EndsEarly
    } else {
        AudioPlan::Same
    })
}

/// A file the run creates in the scratch folder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TempFile {
    /// Its path.
    pub path: String,
    /// What it is for.
    pub what: String,
}

/// A layer as the plan carries it.
#[derive(Clone, Debug)]
pub struct LayerPlan {
    /// The layer id.
    pub id: String,
    /// The absolute path of the video.
    pub video: String,
    /// The raw clip it is decoded to.
    pub temp: String,
    /// The probe of the video.
    pub info: Info,
}

/// The decisions that depend on the scratch folder.
#[derive(Clone, Debug)]
pub struct Plan {
    /// Frame or temporal.
    pub kind: Kind,
    /// The scratch folder.
    pub scratch: String,
    /// Layer clips.
    pub layers: Vec<LayerPlan>,
    /// Frames of the output, when known.
    pub frames: Option<u64>,
    /// Frames the input decodes to, when the container says so.
    pub input_frames: Option<u64>,
    /// What happens to the audio when `--frames` changes the length.
    pub audio: Option<AudioPlan>,
    /// The audio copy is cut at the end of the video.
    pub shortest: bool,
    /// Files and folders the run creates.
    pub temp_files: Vec<TempFile>,
}

/// Every command of a run, as argument lists.
#[derive(Clone, Debug)]
pub struct Commands {
    /// One ffmpeg per layer.
    pub layers: Vec<Vec<String>>,
    /// The decode.
    pub decode: Vec<String>,
    /// The stages in order.
    pub steps: Vec<Step>,
    /// The encode, as `apply.py` writes it (the output is the partial path).
    pub encode: Vec<String>,
    /// The arguments the encode takes after `-i pipe:0`, without the output path.
    pub encode_tail: Vec<String>,
}

impl Commands {
    /// All commands as `[cmd]` lines: layer decodes, decode, the stages, encode.
    pub fn lines(&self) -> Vec<String> {
        let mut lines: Vec<String> = self.layers.iter().map(|c| format_command(c)).collect();
        lines.push(format_command(&self.decode));
        lines.extend(self.steps.iter().map(|s| format_command(&s.argv)));
        lines.push(format_command(&self.encode));
        lines
    }
}

/// A run that passed every check.
#[derive(Clone, Debug)]
pub struct Prepared {
    /// The arguments.
    pub request: Request,
    /// ffmpeg and ffprobe.
    pub media: Media,
    /// The input's probe.
    pub info: Info,
    /// `vfx`, when an effect is run.
    pub vfx: Option<PathBuf>,
    /// The effect's kind and layers.
    pub effect: Option<EffectInfo>,
    /// The probed layer videos.
    pub layers: Vec<LayerProbe>,
    /// Where the output goes.
    pub dst: String,
    /// The size and depth of the frames in every pipe.
    pub spec: FrameSpec,
    /// The exact frame rate.
    pub rate: Rate,
    /// The stages between decode and encode, with the placeholders expanded.
    pub steps: Vec<Step>,
}

/// A step before its position in the chain is known.
enum Raw {
    Vfx,
    /// `index` is the position among the `--stage` flags (for messages), `name` the report name.
    Stage { index: usize, name: String, text: String },
}

fn split_stage(index: usize, name: &str, text: &str) -> Result<Vec<String>> {
    let argv = split_command(text, cfg!(windows)).map_err(|e| Error::usage(format!("--stage {}: {}", index, e)).at(name))?;
    if argv.is_empty() {
        return Err(Error::usage(format!("--stage {} is empty", index)).at(name));
    }
    Ok(argv)
}

/// The order of the chain: where the effect goes among the `--stage` commands, and the name of
/// every stage relative to the `vfx` marker (`pre1`, ... before it, `post1`, ... after it).
fn order_of(request: &Request) -> Result<Vec<Raw>> {
    let mut chain: Vec<Raw> = Vec::new();
    let mut marker = false;
    for text in &request.stages {
        if text.trim() == "vfx" {
            if request.effect.is_none() {
                return Err(Error::usage("`--stage vfx` marks where the effect runs, and no EFFECT was given").at("args"));
            }
            if marker {
                return Err(Error::usage("`--stage vfx` is given more than once").at("args"));
            }
            marker = true;
        }
    }
    // the effect runs first when it is not marked, so every stage follows it
    let mut after = request.effect.is_some() && !marker;
    let (mut pre, mut post) = (0usize, 0usize);
    for (i, text) in request.stages.iter().enumerate() {
        if text.trim() == "vfx" {
            after = true;
            chain.push(Raw::Vfx);
            continue;
        }
        let name = if after {
            post += 1;
            step_name("post", post)
        } else {
            pre += 1;
            step_name("pre", pre)
        };
        chain.push(Raw::Stage { index: i + 1, name, text: text.clone() });
    }
    if request.effect.is_some() && !marker {
        chain.insert(0, Raw::Vfx);
    }
    if chain.is_empty() {
        return Err(Error::usage("give an EFFECT or at least one --stage").at("args"));
    }
    Ok(chain)
}

/// Make every check of `apply.py` in its order and resolve the chain.
pub fn prepare(request: &Request) -> Result<Prepared> {
    prepare_with(request, &process_env, find_vfx_default)
}

/// [`prepare`] with the environment and the `vfx` lookup supplied (tests use their own).
pub fn prepare_with(
    request: &Request,
    env: EnvFn,
    find_vfx: impl Fn(Option<&str>) -> Result<PathBuf>,
) -> Result<Prepared> {
    // configure(): the output path against the input and --force
    if let Some(output) = &request.output {
        if lexists(output) {
            if same_file(output, &request.input) {
                return Err(Error::bad_input("the output path is the input file; choose a different --output path").at("plan"));
            }
            if !request.force {
                return Err(Error::bad_input("output already exists; choose a new --output path or pass --force").at("plan"));
            }
        }
    }
    if request.stage_timeout.map(|t| t < 0.0 || !t.is_finite()).unwrap_or(false) {
        return Err(Error::usage("--stage-timeout expects a number of seconds, 0 or more").at("args"));
    }
    let chain = order_of(request)?;

    let media = require_media(request.ffmpeg_path.as_deref(), request.ffprobe_path.as_deref(), env)?;
    // require_input()
    if !std::path::Path::new(&request.input).is_file() {
        return Err(Error::bad_input(format!("no such file: {}", request.input)).at("probe"));
    }
    let info = probe(&media.ffprobe, &request.input)
        .ok_or_else(|| Error::bad_input(format!("cannot probe {}; is it a media file?", request.input)).at("probe"))?;
    if !info.has_video {
        return Err(Error::bad_input("input has no video stream").at("probe"));
    }
    let rate = match (info.rate, info.width, info.height) {
        (Some(rate), Some(w), Some(h)) if w > 0 && h > 0 => rate,
        _ => return Err(Error::bad_input("cannot read the input's frame size and frame rate").at("probe")),
    };
    if info.hdr {
        return Err(Error::unsupported("HDR input is not supported; tone mapping is not part of this tool").at("probe"));
    }
    let depth = request.depth();
    let (w, h) = info.display_size();
    if request.lossless && depth == Depth::F32 {
        return Err(Error::unsupported("--lossless needs --depth 8 or 16: ffv1 has no float pixel format").at("plan"));
    }
    if (w % 2 != 0 || h % 2 != 0) && !request.lossless {
        return Err(Error::unsupported(format!(
            "the frame size {}x{} has an odd side, and yuv420p needs even sides; resize or crop the input first (ffmpeg -vf crop=trunc(iw/2)*2:trunc(ih/2)*2), or pass --lossless",
            w, h
        ))
        .at("plan"));
    }
    let spec = FrameSpec { width: w, height: h, depth };
    if let Err(e) = depth.pix_fmt().frame_size(w, h) {
        return Err(Error::unsupported(format!(
            "the frame size {}x{} at depth {} is larger than this pipeline handles ({}); try a smaller --depth or scale the input first",
            w,
            h,
            depth.bits(),
            e
        ))
        .at("plan"));
    }
    if rate.as_u32_pair().is_none() {
        return Err(Error::unsupported(format!("the frame rate {} has a term above 4294967295", rate)).at("probe"));
    }

    // vfx and the effect
    let mut vfx_path: Option<PathBuf> = None;
    let mut einfo: Option<EffectInfo> = None;
    let mut layers: Vec<LayerProbe> = Vec::new();
    if let Some(effect) = &request.effect {
        let vfx = find_vfx(request.vfx.as_deref())?;
        let e = effect_info(&vfx, effect)?;
        check_layer_options(request, &e)?;
        if e.kind == Kind::Temporal {
            layers = probe_layers(request, &media)?;
        }
        let layer_args: Vec<(String, String)> = if e.kind == Kind::Temporal { request.layers.clone() } else { Vec::new() };
        preflight(&vfx, effect, w, h, depth.bits(), &rate.text(), &layer_args, &request.params)?;
        vfx_path = Some(vfx);
        einfo = Some(e);
    } else {
        if !request.params.is_empty() {
            return Err(Error::usage("--param needs an EFFECT; the --stage commands take their own arguments").at("args"));
        }
        if !request.layers.is_empty() || request.frames.is_some() {
            return Err(Error::usage("--layer and --frames only apply to a temporal EFFECT").at("args"));
        }
    }

    // the stages
    let input_frames = info.exact_frames();
    let out_frames = match (&einfo, request.frames) {
        (Some(e), Some(n)) if e.kind == Kind::Temporal => Some(n),
        _ => input_frames,
    };
    let mut steps: Vec<Step> = Vec::new();
    let mut count = input_frames;
    let mut cur = spec;
    for raw in chain {
        match raw {
            Raw::Vfx => {
                let e = einfo.as_ref().expect("a vfx step has an effect");
                if cur != spec {
                    return Err(Error::unsupported("the effect runs at the input size, so a stage with declared geometry can only follow it").at("vfx"));
                }
                let after = if e.kind == Kind::Temporal && request.frames.is_some() { out_frames } else { count };
                steps.push(Step { name: "vfx".into(), kind: StepKind::Vfx, argv: Vec::new(), frames_in: count, frames_out: after, spec_in: cur, spec_out: cur });
                count = after;
            }
            Raw::Stage { index, name, text } => {
                let template = split_stage(index, &name, &text)?;
                let vars = StageVars {
                    width: cur.width,
                    height: cur.height,
                    depth: depth.bits(),
                    fps: rate.text(),
                    frames: count,
                    pix_fmt: depth.pix_fmt_name(),
                };
                let mut argv = Vec::with_capacity(template.len());
                for token in &template {
                    argv.push(expand_placeholders(token, &vars).map_err(|m| Error::new(Code::Usage, format!("--stage {}: {}", index, m)).at(&name))?);
                }
                let mut kind = if let Some(id) = argv[0].strip_prefix('@') {
                    StepKind::Builtin { id: id.to_string(), params: argv[1..].to_vec() }
                } else {
                    StepKind::External
                };
                let mut geometry: Option<String> = None;
                let mut ff_opts: Option<crate::ffstage::Options> = None;
                match kind.clone() {
                    StepKind::Builtin { id, params } => {
                        let def = crate::stage::find(&id).ok_or_else(|| {
                            Error::unsupported(format!("unknown stage `@{}`, `sapphire stage list` shows the registered ones", id)).at(&name)
                        })?;
                        let values = crate::stage::check_params(&def, &params).map_err(|e| e.at(&name))?;
                        if id == crate::ffstage::ID {
                            // ffmpeg is a child process anyway: the stage runs as `sapphire
                            // stage apply ffmpeg-filter`, judged by the stall watch like any process
                            let opts = crate::ffstage::Options::from_values(&values).map_err(|e| e.at(&name))?;
                            if opts.expect == crate::ffstage::Expect::Declared {
                                geometry = Some("declared".into());
                            }
                            argv = crate::ffstage::command_line(cur, &rate, count, &params, &media.ffmpeg);
                            kind = StepKind::External;
                            ff_opts = Some(opts);
                        }
                    }
                    _ => {
                        require_program(&argv[0], env, &name)?;
                        geometry = crate::geometry::listed_geometry(&argv, &name)?;
                        if let Some(given) = request.stage_geometry.get(index - 1).cloned().flatten() {
                            geometry = Some(given);
                        }
                    }
                }
                let (spec_out, frames_out) = match geometry.as_deref() {
                    None | Some("preserve") => (cur, count),
                    Some("declared") => {
                        if request.contract == Contract::V1 {
                            return Err(Error::unsupported(format!("{} declares its geometry, which needs --contract frames/2", name)).at(&name));
                        }
                        let answer = if let Some(opts) = &ff_opts {
                            crate::ffstage::describe(cur, &rate, count, opts, &media.ffmpeg).map_err(|e| e.at(&name))?
                        } else {
                            crate::geometry::ask_describe(&argv, count, &name, cur.depth, &rate)?
                        };
                        (FrameSpec { width: answer.width, height: answer.height, depth: cur.depth }, answer.frames)
                    }
                    Some(other) => {
                        return Err(Error::unsupported(format!(
                            "{} declares the geometry `{}`, which this tool does not know (it knows preserve and declared)",
                            name, other
                        ))
                        .at(&name))
                    }
                };
                steps.push(Step { name, kind, argv, frames_in: count, frames_out, spec_in: cur, spec_out });
                cur = spec_out;
                count = frames_out;
            }
        }
    }
    if cur != spec && (cur.width % 2 != 0 || cur.height % 2 != 0) && !request.lossless {
        return Err(Error::unsupported(format!("the stages make {}x{} frames, and yuv420p needs even sides; pass --lossless", cur.width, cur.height)).at("plan"));
    }

    // the output path
    let dst = request.output.clone().unwrap_or_else(|| {
        let label = request.effect.as_deref().unwrap_or("stages");
        default_output(&request.input, label, if request.lossless { "mkv" } else { "mp4" })
    });
    if same_file(&dst, &request.input) {
        return Err(Error::bad_input("the output path is the input file; choose a different --output path").at("plan"));
    }
    for layer in &layers {
        if same_file(&dst, &layer.video) {
            return Err(Error::bad_input(format!("the output path is the video of layer {}; choose a different --output path", layer.id)).at("plan"));
        }
    }

    Ok(Prepared { request: request.clone(), media, info, vfx: vfx_path, effect: einfo, layers, dst, spec, rate, steps })
}

/// `--layer` and `--frames` against the kind of the effect.
fn check_layer_options(request: &Request, e: &EffectInfo) -> Result<()> {
    if e.kind != Kind::Temporal {
        let mut used: Vec<&str> = Vec::new();
        if !request.layers.is_empty() {
            used.push("--layer");
        }
        if request.frames.is_some() {
            used.push("--frames");
        }
        if !used.is_empty() {
            return Err(Error::unsupported(format!("{} is a frame effect, {} only apply to temporal effects", e.id, used.join(" and "))).at("plan"));
        }
        return Ok(());
    }
    let ids: Vec<&str> = e.layers.iter().map(|l| l.id.as_str()).collect();
    let mut seen: Vec<&str> = Vec::new();
    for (id, _) in &request.layers {
        if !ids.contains(&id.as_str()) {
            let have = if ids.is_empty() { "no layers".to_string() } else { format!("the layers {}", ids.join(", ")) };
            return Err(Error::unsupported(format!("unknown layer `{}`: {} has {}", id, e.id, have)).at("plan"));
        }
        if seen.contains(&id.as_str()) {
            return Err(Error::usage(format!("layer `{}` given more than once", id)).at("plan"));
        }
        seen.push(id);
    }
    for layer in &e.layers {
        if !layer.optional && !seen.contains(&layer.id.as_str()) {
            return Err(Error::unsupported(format!("{} needs the layer `{}`, pass --layer {}=VIDEO", e.id, layer.id, layer.id)).at("plan"));
        }
    }
    Ok(())
}

/// Probe every `--layer` video up front.
fn probe_layers(request: &Request, media: &Media) -> Result<Vec<LayerProbe>> {
    let mut layers = Vec::new();
    for (id, path) in &request.layers {
        if !std::path::Path::new(path).is_file() {
            return Err(Error::bad_input(format!("no such file: {}", path)).at("probe"));
        }
        let info = probe(&media.ffprobe, path)
            .ok_or_else(|| Error::bad_input(format!("cannot probe {}; is it a media file?", path)).at("probe"))?;
        if !info.has_video {
            return Err(Error::bad_input(format!("layer {}: {} has no video stream", id, path)).at("probe"));
        }
        if info.rate.is_none() || info.width.unwrap_or(0) == 0 || info.height.unwrap_or(0) == 0 {
            return Err(Error::bad_input(format!("layer {}: cannot read the frame size and frame rate of {}", id, path)).at("probe"));
        }
        if info.hdr {
            return Err(Error::unsupported(format!("layer {}: HDR video is not supported; tone mapping is not part of this tool", id)).at("probe"));
        }
        let video = std::path::absolute(path).unwrap_or_else(|_| PathBuf::from(path)).to_string_lossy().into_owned();
        layers.push(LayerProbe { id: id.clone(), video, info });
    }
    Ok(layers)
}

/// The result of [`Prepared::assemble`].
#[derive(Clone, Debug)]
pub struct Assembled {
    /// The plan.
    pub plan: Plan,
    /// The commands.
    pub commands: Commands,
}

impl Prepared {
    /// Whether the effect is temporal.
    pub fn kind(&self) -> Kind {
        self.effect.as_ref().map(|e| e.kind).unwrap_or(Kind::Frame)
    }

    fn effect_frames(&self) -> Option<u64> {
        match (self.kind(), self.request.frames) {
            (Kind::Temporal, Some(n)) => Some(n),
            _ => self.info.exact_frames(),
        }
    }

    /// The frames the encoder takes: the output of the last step (see [`Step::spec_out`]).
    pub fn out_spec(&self) -> FrameSpec {
        self.steps.last().map(|s| s.spec_out).unwrap_or(self.spec)
    }

    /// Whether a stage with declared geometry changed the size or the frame count.
    pub fn geometry_changed(&self) -> bool {
        self.steps.iter().any(|s| s.spec_out != s.spec_in || (s.kind != StepKind::Vfx && s.frames_out != s.frames_in))
    }

    /// Frames of the output, when known.
    pub fn output_frames(&self) -> Option<u64> {
        self.steps.last().and_then(|s| s.frames_out).or_else(|| self.effect_frames())
    }

    /// The plan and the commands for `scratch` (the folder is named here, not created).
    pub fn assemble(&self, scratch: &str) -> Assembled {
        let plan = self.make_plan(scratch);
        let layer_commands = self.layer_commands(&plan);
        let decode = self.decode_command();
        let (encode_tail, encode) = self.encode_command(&plan);
        let steps = self.steps.iter().map(|s| self.finish_step(s, &plan)).collect();
        Assembled { plan, commands: Commands { layers: layer_commands, decode, steps, encode, encode_tail } }
    }

    fn make_plan(&self, scratch: &str) -> Plan {
        let kind = self.kind();
        let mut plan = Plan {
            kind,
            scratch: scratch.to_string(),
            layers: Vec::new(),
            frames: None,
            input_frames: self.info.exact_frames(),
            audio: None,
            shortest: false,
            temp_files: Vec::new(),
        };
        if kind != Kind::Temporal {
            return plan;
        }
        let fmt = self.spec.depth.pix_fmt_name();
        let join = |name: &str| std::path::Path::new(scratch).join(name).to_string_lossy().into_owned();
        for layer in &self.layers {
            let temp = join(&format!("layer_{}.{}", layer.id, fmt));
            plan.temp_files.push(TempFile { path: temp.clone(), what: format!("layer {}, decoded to raw {}", layer.id, fmt) });
            plan.layers.push(LayerPlan { id: layer.id.clone(), video: layer.video.clone(), temp, info: layer.info.clone() });
        }
        plan.temp_files.push(TempFile {
            path: scratch.to_string(),
            what: "scratch directory: the layer clips, the logs and the spool file vfx makes of the piped input".to_string(),
        });
        plan.frames = self.request.frames.or(plan.input_frames);
        if let Some(n) = self.request.frames {
            if Some(n) != plan.input_frames {
                plan.audio = audio_plan(&self.info, n, &self.rate);
                plan.shortest = plan.audio == Some(AudioPlan::Shortest);
            }
        }
        plan
    }

    fn layer_commands(&self, plan: &Plan) -> Vec<Vec<String>> {
        let (w, h) = (self.spec.width, self.spec.height);
        let fmt = self.spec.depth.pix_fmt_name();
        plan.layers
            .iter()
            .map(|layer| {
                let li = &layer.info;
                let matrix = colour_settings(li).matrix;
                let mut graph: Vec<String> = Vec::new();
                if li.vfr || li.rate != Some(self.rate) {
                    graph.push(format!("fps={}", self.rate.text()));
                }
                if li.display_size() != (w, h) {
                    graph.push(format!("scale={}:{}:in_color_matrix={}", w, h, matrix));
                } else {
                    graph.push(format!("scale=in_color_matrix={}", matrix));
                }
                let mut cmd = self.ffmpeg_base();
                cmd.extend(["-y", "-i"].map(String::from));
                cmd.push(layer.video.clone());
                cmd.extend(["-map".to_string(), format!("0:{}", li.video_index.unwrap_or(0))]);
                cmd.extend(["-an", "-sn", "-dn", "-vf"].map(String::from));
                cmd.push(graph.join(","));
                cmd.extend(["-pix_fmt", fmt, "-f", "rawvideo"].map(String::from));
                cmd.push(layer.temp.clone());
                cmd
            })
            .collect()
    }

    fn ffmpeg_base(&self) -> Vec<String> {
        vec![self.media.ffmpeg.to_string_lossy().into_owned(), "-v".into(), "error".into(), "-nostdin".into()]
    }

    /// The `-vf` chain of the decode: `[fps=N/D,]scale=in_color_matrix=M`.
    pub fn decode_filter(&self) -> String {
        let mut graph: Vec<String> = Vec::new();
        if self.info.vfr {
            graph.push(format!("fps={}", self.rate.text()));
        }
        graph.push(format!("scale=in_color_matrix={}", colour_settings(&self.info).matrix));
        graph.join(",")
    }

    fn decode_command(&self) -> Vec<String> {
        let mut cmd = self.ffmpeg_base();
        cmd.extend(["-i".to_string(), self.request.input.clone()]);
        cmd.extend(["-map".to_string(), format!("0:{}", self.info.video_index.unwrap_or(0))]);
        cmd.extend(["-an", "-sn", "-dn", "-vf"].map(String::from));
        cmd.push(self.decode_filter());
        cmd.extend(["-pix_fmt".to_string(), self.spec.depth.pix_fmt_name().to_string()]);
        cmd.extend(["-f", "rawvideo", "-"].map(String::from));
        cmd
    }

    /// Returns the arguments after `-i pipe:0` (no output path) and the whole `apply.py` command.
    fn encode_command(&self, plan: &Plan) -> (Vec<String>, Vec<String>) {
        let out = self.out_spec();
        let (w, h) = (out.width, out.height);
        let colour = colour_settings(&self.info);
        let mut tail: Vec<String> = vec!["-i".into(), self.request.input.clone(), "-map".into(), "0:v:0".into(), "-map".into(), "1:a?".into()];
        if self.request.lossless {
            let fmt = self.spec.depth.lossless_pix_fmt().unwrap_or("rgba64le");
            tail.extend(["-c:v", "ffv1", "-level", "3", "-pix_fmt", fmt, "-c:a", "copy"].map(String::from));
        } else {
            let mut tags = vec![format!("colorspace={}", colour.space)];
            if let Some(p) = &colour.primaries {
                tags.push(format!("color_primaries={}", p));
            }
            if let Some(t) = &colour.trc {
                tags.push(format!("color_trc={}", t));
            }
            tags.push("range=tv".to_string());
            let graph = format!("scale=out_color_matrix={}:out_range=tv,setparams={}", colour.matrix, tags.join(":"));
            tail.extend(["-vf".to_string(), graph]);
            tail.extend(["-c:v", "libx264", "-crf", "18", "-pix_fmt", "yuv420p", "-c:a", "copy"].map(String::from));
        }
        if plan.shortest {
            tail.push("-shortest".into());
        }
        let ext = crate::pytext::splitext(&self.dst).1.to_lowercase();
        if matches!(ext.as_str(), ".mp4" | ".m4v" | ".mov") {
            tail.extend(["-movflags", "+faststart"].map(String::from));
        }
        let mut cmd = self.ffmpeg_base();
        cmd.extend(["-n", "-f", "rawvideo", "-pix_fmt"].map(String::from));
        cmd.push(self.spec.depth.pix_fmt_name().to_string());
        cmd.extend(["-video_size".to_string(), format!("{}x{}", w, h), "-framerate".to_string(), self.rate.text()]);
        cmd.extend(["-i", "-"].map(String::from));
        cmd.extend(tail.iter().cloned());
        cmd.push(partial_path(&self.dst));
        (tail, cmd)
    }

    fn finish_step(&self, step: &Step, plan: &Plan) -> Step {
        let mut done = step.clone();
        match &step.kind {
            StepKind::Vfx => {
                let vfx = self.vfx.as_ref().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();
                let mut argv = vec![
                    vfx,
                    "apply".to_string(),
                    self.request.effect.clone().unwrap_or_default(),
                    "--width".to_string(),
                    step.spec_in.width.to_string(),
                    "--height".to_string(),
                    step.spec_in.height.to_string(),
                    "--depth".to_string(),
                    step.spec_in.depth.bits().to_string(),
                    "--fps".to_string(),
                    self.rate.text(),
                ];
                if plan.kind == Kind::Temporal {
                    if let Some(n) = plan.frames {
                        argv.extend(["--frames".to_string(), n.to_string()]);
                    }
                    argv.extend(["--tmp-dir".to_string(), plan.scratch.clone()]);
                    for layer in &plan.layers {
                        argv.extend(["--layer".to_string(), format!("{}={}", layer.id, layer.temp)]);
                    }
                }
                for p in &self.request.params {
                    argv.extend(["--param".to_string(), p.clone()]);
                }
                done.argv = argv;
            }
            StepKind::Builtin { id, params } => {
                let exe = std::env::current_exe().map(|p| p.to_string_lossy().into_owned()).unwrap_or_else(|_| "sapphire".into());
                let mut argv = vec![
                    exe,
                    "stage".to_string(),
                    "apply".to_string(),
                    id.clone(),
                    "--width".to_string(),
                    step.spec_in.width.to_string(),
                    "--height".to_string(),
                    step.spec_in.height.to_string(),
                    "--depth".to_string(),
                    step.spec_in.depth.bits().to_string(),
                    "--fps".to_string(),
                    self.rate.text(),
                ];
                for p in params {
                    argv.extend(["--param".to_string(), p.clone()]);
                }
                done.argv = argv;
            }
            StepKind::External => {}
        }
        done
    }

    /// The `[plan]` summary line (without the prefix).
    pub fn summary(&self, plan: &Plan) -> String {
        let label = self.request.effect.as_deref().unwrap_or("stages");
        let (w, h) = (self.spec.width, self.spec.height);
        let mut parts = vec![format!(
            "{} on {}: {}x{}, {:.3} fps, {} bpc ({})",
            label,
            basename(&self.request.input),
            w,
            h,
            self.rate.as_f64(),
            self.spec.depth.bits(),
            self.spec.depth.pix_fmt_name()
        )];
        if plan.kind == Kind::Temporal {
            parts.push(match plan.frames {
                Some(n) => format!("temporal, {} frames", n),
                None => "temporal, as many frames as the input decodes to".to_string(),
            });
            if !plan.layers.is_empty() {
                parts.push(format!("layers {}", plan.layers.iter().map(|l| l.id.as_str()).collect::<Vec<_>>().join(", ")));
            }
            match plan.audio {
                Some(AudioPlan::Shortest) => parts.push("audio cut at the end of the video (-shortest)".to_string()),
                Some(AudioPlan::EndsEarly) => parts.push("the audio ends before the video does".to_string()),
                _ => {}
            }
        }
        let names: Vec<&str> = self.steps.iter().map(|s| s.name.as_str()).collect();
        parts.push(format!("decode | {} | encode -> {}", names.join(" | "), self.dst));
        parts.join(", ")
    }

    /// The report of a run, in the layout of `apply.py --json` plus `contract`, `stages` and
    /// `stage_timeout`. `outputs` and `seconds` are filled after a real run.
    pub fn result(&self, assembled: &Assembled, outputs: Vec<Out>, seconds: Option<f64>, stage_reports: Option<Vec<Out>>) -> Out {
        let plan = &assembled.plan;
        let (w, h) = (self.spec.width, self.spec.height);
        let mut members: Vec<(String, Out)> = vec![
            ("contract".into(), Out::text(self.request.contract.as_str())),
            ("operation".into(), Out::text("apply")),
            ("input".into(), Out::text(std::path::absolute(&self.request.input).unwrap_or_else(|_| PathBuf::from(&self.request.input)).to_string_lossy())),
            ("effect".into(), Out::opt_text(self.request.effect.clone())),
            ("kind".into(), Out::text(plan.kind.as_str())),
            ("params".into(), Out::List(self.request.params.iter().cloned().map(Out::Str).collect())),
            ("depth".into(), Out::Int(self.spec.depth.bits() as i64)),
            ("pix_fmt".into(), Out::text(self.spec.depth.pix_fmt_name())),
            ("size".into(), Out::text(format!("{}x{}", w, h))),
            ("fps".into(), Out::Float(self.rate.as_f64())),
            ("fps_exact".into(), Out::text(self.rate.text())),
            ("frames".into(), Out::opt_int(plan.frames.map(|n| n as i64))),
            ("input_frames".into(), Out::opt_int(plan.input_frames.map(|n| n as i64))),
            (
                "layers".into(),
                Out::List(
                    plan.layers
                        .iter()
                        .map(|l| Out::map(vec![("id", Out::text(l.id.clone())), ("video", Out::text(l.video.clone())), ("temp", Out::text(l.temp.clone()))]))
                        .collect(),
                ),
            ),
            (
                "audio".into(),
                Out::map(vec![("plan", Out::opt_text(plan.audio.map(|a| a.as_str()))), ("shortest", Out::Bool(plan.shortest))]),
            ),
            (
                "temp_files".into(),
                Out::List(plan.temp_files.iter().map(|t| Out::map(vec![("path", Out::text(t.path.clone())), ("what", Out::text(t.what.clone()))])).collect()),
            ),
            (
                "stages".into(),
                Out::List(
                    assembled
                        .commands
                        .steps
                        .iter()
                        .map(|s| {
                            Out::map(vec![
                                ("name", Out::text(s.name.clone())),
                                (
                                    "kind",
                                    Out::text(match s.kind {
                                        StepKind::Vfx => "vfx",
                                        StepKind::External => "external",
                                        StepKind::Builtin { .. } => "builtin",
                                    }),
                                ),
                                ("command", Out::text(format_command(&s.argv))),
                            ])
                        })
                        .collect(),
                ),
            ),
            ("stage_timeout".into(), Out::Float(self.request.stage_timeout())),
            ("vfx".into(), Out::opt_text(self.vfx.as_ref().map(|p| p.to_string_lossy().into_owned()))),
            ("summary".into(), Out::text(self.summary(plan))),
            ("dry_run".into(), Out::Bool(self.request.dry_run)),
            ("commands".into(), Out::List(assembled.commands.lines().into_iter().map(Out::Str).collect())),
            ("outputs".into(), Out::List(outputs)),
        ];
        if let Some(reports) = stage_reports {
            members.push(("stage_reports".into(), Out::List(reports)));
        }
        if let Some(s) = seconds {
            members.push(("seconds".into(), Out::Float(s)));
        }
        if self.request.contract == Contract::V2 {
            // contracts/run-report.schema.json requires `ok: true`; ok and contract lead, as in
            // tools/apply.py's framed_result. frames/1 keeps apply.py's frames/1 document.
            members.insert(0, ("ok".into(), Out::Bool(true)));
            for member in members.iter_mut() {
                if member.0 == "stages" {
                    member.1 = self.report_stages(assembled, None);
                }
            }
            members.push(("geometry".into(), self.geometry_json()));
        }
        Out::Map(members)
    }

    /// The frames/2 `geometry` of a result: the frames the encoder takes and the output length.
    pub fn geometry_json(&self) -> Out {
        let out = self.out_spec();
        Out::map(vec![
            ("width", Out::Int(out.width as i64)),
            ("height", Out::Int(out.height as i64)),
            ("depth", Out::Int(out.depth.bits() as i64)),
            ("fps", Out::text(self.rate.text())),
            ("frames", Out::opt_int(self.output_frames().map(|n| n as i64))),
        ])
    }

    /// The frames/2 `stages` of a result (`contracts/run-report.schema.json`): decode, every step,
    /// encode, in pipe order, each with `role`, `index` (1 based within its role), `name` (a
    /// label), `command` (the argv), `exit_code` (null in a dry run) and `stderr_tail`.
    pub fn report_stages(&self, assembled: &Assembled, run: Option<&crate::pipeline::RunReport>) -> Out {
        let tail = |text: &str| Out::List(crate::error::tail_lines(text, crate::error::TAIL_MAX).into_iter().map(Out::Str).collect());
        let entry = |role: &str, index: usize, name: &str, argv: &[String], exit: Option<i32>, stderr: Out, kind: Option<&str>| {
            let mut m = vec![
                ("role", Out::text(role)),
                ("index", Out::Int(index as i64)),
                ("name", Out::text(name)),
                ("command", Out::List(argv.iter().cloned().map(Out::Str).collect())),
                ("exit_code", Out::opt_int(exit.map(|c| c as i64))),
                ("stderr_tail", stderr),
            ];
            if let Some(k) = kind {
                m.push(("kind", Out::text(k)));
            }
            Out::map(m)
        };
        let ran = run.map(|_| 0);
        let mut list = vec![entry("decode", 1, "decode", &assembled.commands.decode, ran, run.map(|r| tail(&r.decode_stderr)).unwrap_or(Out::List(Vec::new())), None)];
        let mut counts: Vec<(&str, usize)> = Vec::new();
        for step in &assembled.commands.steps {
            let role = step.role();
            let index = match counts.iter_mut().find(|(r, _)| *r == role) {
                Some((_, n)) => {
                    *n += 1;
                    *n
                }
                None => {
                    counts.push((role, 1));
                    1
                }
            };
            let report = run.and_then(|r| r.stages.iter().find(|s| s.name == step.name));
            let kind = match step.kind {
                StepKind::Vfx => "vfx",
                StepKind::External => "external",
                StepKind::Builtin { .. } => "builtin",
            };
            let stderr = Out::List(report.map(|r| r.stderr_tail.iter().cloned().map(Out::Str).collect()).unwrap_or_default());
            // Only a finished run reaches this report, so a step with a report succeeded. A builtin
            // that ran in process has no child status; the schema keeps null for a step that did
            // not start, so it reports the 0 its `stage apply` argv would have exited with.
            list.push(entry(role, index, &step.name, &step.argv, report.map(|r| r.exit_code.unwrap_or(0)), stderr, Some(kind)));
        }
        list.push(entry("encode", 1, "encode", &assembled.commands.encode, ran, run.map(|r| tail(&r.encode_stderr)).unwrap_or(Out::List(Vec::new())), None));
        Out::List(list)
    }

    /// The partial output path of this run.
    pub fn partial(&self) -> String {
        partial_path(&self.dst)
    }

    /// Remove the partial output of this run.
    pub fn discard_partial(&self) {
        discard(self.partial());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn vars(frames: Option<u64>) -> StageVars {
        StageVars { width: 160, height: 96, depth: 16, fps: "24/1".into(), frames, pix_fmt: "rgba64le" }
    }

    #[test]
    fn placeholders_expand_in_one_token() {
        let v = vars(Some(12));
        assert_eq!(expand_placeholders("--width={width}x{height}", &v).unwrap(), "--width=160x96");
        assert_eq!(expand_placeholders("{depth}", &v).unwrap(), "16");
        assert_eq!(expand_placeholders("{fps}", &v).unwrap(), "24/1");
        assert_eq!(expand_placeholders("{pix_fmt}", &v).unwrap(), "rgba64le");
        assert_eq!(expand_placeholders("{frames}", &v).unwrap(), "12");
        assert_eq!(expand_placeholders("{width}{width}", &v).unwrap(), "160160");
        assert_eq!(expand_placeholders("plain", &v).unwrap(), "plain");
    }

    #[test]
    fn other_braces_are_left_alone_like_apply_py() {
        let v = vars(Some(12));
        assert_eq!(expand_placeholders("{nope}", &v).unwrap(), "{nope}");
        assert_eq!(expand_placeholders("{\"a\": {width}}", &v).unwrap(), "{\"a\": 160}");
        assert_eq!(expand_placeholders("{width", &v).unwrap(), "{width");
        assert_eq!(expand_placeholders("a}", &v).unwrap(), "a}");
        assert_eq!(expand_placeholders("{{width}}", &v).unwrap(), "{160}");
        assert_eq!(expand_placeholders("{ width }", &v).unwrap(), "{ width }");
    }

    #[test]
    fn frames_needs_a_known_count() {
        let v = vars(None);
        assert!(expand_placeholders("{frames}", &v).unwrap_err().contains("{frames} is not known"));
        assert_eq!(expand_placeholders("{width}", &v).unwrap(), "160", "the other placeholders still work");
    }

    #[test]
    fn colour_follows_the_tags_then_the_size() {
        let mut info = Info::from_json(
            r#"{"streams":[{"index":0,"codec_type":"video","width":160,"height":96,"r_frame_rate":"24/1"}]}"#,
            std::path::Path::new("a.mp4"),
        )
        .unwrap();
        let c = colour_settings(&info);
        assert_eq!((c.matrix, c.space.as_str(), c.primaries.as_deref(), c.trc.as_deref()), ("smpte170m", "smpte170m", Some("smpte170m"), Some("smpte170m")));
        info.width = Some(1280);
        info.height = Some(544);
        assert_eq!(colour_settings(&info).matrix, "bt709", "the longer side counts");
        info.width = Some(640);
        info.height = Some(720);
        assert_eq!(colour_settings(&info).space, "bt709", "so does a height of 720");
        info.color_space = Some("bt470bg".into());
        info.color_primaries = None;
        info.color_transfer = Some("bt709".into());
        let tagged = colour_settings(&info);
        assert_eq!((tagged.matrix, tagged.space.as_str(), tagged.primaries, tagged.trc.as_deref()), ("bt601", "bt470bg", None, Some("bt709")));
        info.color_space = Some("bt2020nc".into());
        assert_eq!(colour_settings(&info).matrix, "bt2020");
        info.color_space = Some("ycgco".into());
        assert_eq!(colour_settings(&info).space, "bt709", "an unlisted space falls back to the size rule");
    }

    #[test]
    fn audio_plan_judges_by_lengths() {
        let mut info = Info::from_json(
            r#"{"streams":[{"index":0,"codec_type":"video","width":4,"height":4,"r_frame_rate":"24/1"},{"index":1,"codec_type":"audio","codec_name":"aac","channels":2,"duration":"2.0"}],"format":{"duration":"2.0"}}"#,
            std::path::Path::new("a.mp4"),
        )
        .unwrap();
        let rate = Rate::new(24, 1).unwrap();
        assert_eq!(audio_plan(&info, 24, &rate), Some(AudioPlan::Shortest));
        assert_eq!(audio_plan(&info, 48, &rate), Some(AudioPlan::Same));
        assert_eq!(audio_plan(&info, 96, &rate), Some(AudioPlan::EndsEarly));
        info.audio_duration = None;
        info.duration = None;
        assert_eq!(audio_plan(&info, 24, &rate), Some(AudioPlan::Same));
        info.has_audio = false;
        assert_eq!(audio_plan(&info, 24, &rate), None);
    }

    fn request(stages: &[&str], effect: Option<&str>) -> Request {
        let mut r = Request::new("in.mp4", effect);
        r.stages = stages.iter().map(|s| s.to_string()).collect();
        r
    }

    fn names(chain: &[Raw]) -> Vec<String> {
        chain
            .iter()
            .map(|r| match r {
                Raw::Vfx => "vfx".to_string(),
                Raw::Stage { name, .. } => name.clone(),
            })
            .collect()
    }

    #[test]
    fn the_effect_comes_first_by_default() {
        let chain = order_of(&request(&["a", "b"], Some("retime"))).unwrap();
        assert_eq!(names(&chain), ["vfx", "post1", "post2"]);
    }

    #[test]
    fn a_vfx_stage_marks_the_position() {
        let chain = order_of(&request(&["a", "vfx", "b"], Some("retime"))).unwrap();
        assert_eq!(names(&chain), ["pre1", "vfx", "post1"]);
        let last = order_of(&request(&["a", " vfx "], Some("retime"))).unwrap();
        assert_eq!(names(&last), ["pre1", "vfx"]);
        let two = order_of(&request(&["a", "b", "vfx", "c", "d"], Some("retime"))).unwrap();
        assert_eq!(names(&two), ["pre1", "pre2", "vfx", "post1", "post2"]);
    }

    #[test]
    fn stages_alone_need_no_effect() {
        let chain = order_of(&request(&["a"], None)).unwrap();
        assert_eq!(names(&chain), ["pre1"]);
        let two = order_of(&request(&["a", "b"], None)).unwrap();
        assert_eq!(names(&two), ["pre1", "pre2"]);
    }

    #[test]
    fn chain_mistakes_are_usage_errors() {
        for (stages, effect) in [(vec!["vfx"], None), (vec!["vfx", "vfx"], Some("retime")), (vec![], None)] {
            let err = order_of(&request(&stages, effect)).err().expect("an error");
            assert_eq!(err.code, Code::Usage, "{:?}", stages);
            assert_eq!(err.exit_code(), 2);
        }
    }

    #[test]
    fn stage_text_is_split_like_a_shell() {
        assert_eq!(split_stage(1, "pre1", "tool apply 'a b' --x={width}").unwrap(), ["tool", "apply", "a b", "--x={width}"]);
        let err = split_stage(2, "pre2", "tool \"unterminated").unwrap_err();
        assert_eq!((err.code, err.stage.as_deref()), (Code::Usage, Some("pre2")));
        assert!(err.message.starts_with("--stage 2: No closing quotation"), "{}", err.message);
        assert_eq!(split_stage(3, "post1", "   ").unwrap_err().message, "--stage 3 is empty");
    }

    #[test]
    fn a_negative_timeout_is_refused_before_anything_runs() {
        let mut r = request(&["a"], None);
        r.stage_timeout = Some(-1.0);
        let none = |_: &str| None;
        let err = prepare_with(&r, &none, |_| unreachable!()).unwrap_err();
        assert_eq!(err.code, Code::Usage);
    }

    #[test]
    fn the_output_checks_come_before_the_tool_checks() {
        let dir = std::env::temp_dir().join(format!("sapphire-plan-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let existing = dir.join("out.mp4");
        std::fs::write(&existing, b"x").unwrap();
        let mut r = request(&["a"], None);
        r.output = Some(existing.to_string_lossy().into_owned());
        let none = |_: &str| None;
        let err = prepare_with(&r, &none, |_| unreachable!()).unwrap_err();
        assert_eq!(err.message, "output already exists; choose a new --output path or pass --force");
        r.input = existing.to_string_lossy().into_owned();
        let err = prepare_with(&r, &none, |_| unreachable!()).unwrap_err();
        assert_eq!(err.message, "the output path is the input file; choose a different --output path");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn without_ffmpeg_the_install_message_comes_first() {
        let r = request(&["a"], None);
        let none = |name: &str| if name == "PATH" { Some(String::new()) } else { None };
        let err = prepare_with(&r, &none, |_| unreachable!()).unwrap_err();
        assert_eq!(err.code, Code::ToolMissing);
        assert!(err.message.starts_with("this operation requires ffmpeg and ffprobe."));
    }

    // ---- the commands, against what tools/apply.py prints for the same plan

    use crate::pipeline::{decoder_builder, encoder_builder};
    use crate::run::pipeline_config;
    use neutral_ffmpeg::Binaries;

    const CLIP: &str = r#"{"streams":[{"index":0,"codec_type":"video","codec_name":"h264","width":160,"height":96,"r_frame_rate":"24/1","avg_frame_rate":"24/1","nb_frames":"12","duration":"0.5"},
        {"index":1,"codec_type":"audio","codec_name":"aac","channels":2,"duration":"2.0"}],"format":{"duration":"2.0"}}"#;

    fn prepared(json: &str, dst: &str, effect: &str, kind: Kind, configure: impl FnOnce(&mut Request)) -> Prepared {
        let info = Info::from_json(json, std::path::Path::new("clip.mp4")).unwrap();
        let mut request = Request::new("clip.mp4", Some(effect));
        configure(&mut request);
        let (w, h) = info.display_size();
        let rate = info.rate.unwrap();
        let spec = FrameSpec { width: w, height: h, depth: request.depth() };
        let frames_out = match (kind, request.frames) {
            (Kind::Temporal, Some(n)) => Some(n),
            _ => info.exact_frames(),
        };
        let steps = vec![Step { name: "vfx".into(), kind: StepKind::Vfx, argv: Vec::new(), frames_in: info.exact_frames(), frames_out, spec_in: spec, spec_out: spec }];
        Prepared {
            request,
            media: Media { ffmpeg: "ffmpeg".into(), ffprobe: "ffprobe".into() },
            info,
            vfx: Some("vfx".into()),
            effect: Some(EffectInfo { id: effect.into(), kind, layers: Vec::new() }),
            layers: Vec::new(),
            dst: dst.into(),
            spec,
            rate,
            steps,
        }
    }

    fn lines(p: &Prepared) -> Vec<String> {
        p.assemble("S").commands.lines()
    }

    #[test]
    fn the_lossy_commands_are_those_of_apply_py() {
        let p = prepared(CLIP, "out.mp4", "selftest-invert", Kind::Frame, |_| {});
        let got = lines(&p);
        let pid = std::process::id();
        assert_eq!(got.len(), 3);
        assert_eq!(got[0], "ffmpeg -v error -nostdin -i clip.mp4 -map 0:0 -an -sn -dn -vf scale=in_color_matrix=smpte170m -pix_fmt rgba64le -f rawvideo -");
        assert_eq!(got[1], "vfx apply selftest-invert --width 160 --height 96 --depth 16 --fps 24/1");
        assert_eq!(
            got[2],
            format!(
                "ffmpeg -v error -nostdin -n -f rawvideo -pix_fmt rgba64le -video_size 160x96 -framerate 24/1 -i - -i clip.mp4 -map 0:v:0 -map '1:a?' -vf scale=out_color_matrix=smpte170m:out_range=tv,setparams=colorspace=smpte170m:color_primaries=smpte170m:color_trc=smpte170m:range=tv -c:v libx264 -crf 18 -pix_fmt yuv420p -c:a copy -movflags +faststart out.partial-{}.mp4",
                pid
            )
        );
    }

    #[test]
    fn the_lossless_commands_are_those_of_apply_py() {
        for (depth, stored) in [(Depth::U8, "bgra"), (Depth::U16, "rgba64le")] {
            let p = prepared(CLIP, "out.mkv", "selftest-invert", Kind::Frame, |r| {
                r.lossless = true;
                r.depth = Some(depth);
            });
            let encode = lines(&p).pop().unwrap();
            assert_eq!(
                encode,
                format!(
                    "ffmpeg -v error -nostdin -n -f rawvideo -pix_fmt {} -video_size 160x96 -framerate 24/1 -i - -i clip.mp4 -map 0:v:0 -map '1:a?' -c:v ffv1 -level 3 -pix_fmt {} -c:a copy out.partial-{}.mkv",
                    depth.pix_fmt_name(),
                    stored,
                    std::process::id()
                )
            );
        }
    }

    #[test]
    fn a_variable_rate_input_decodes_with_the_fps_filter_first() {
        let vfr = r#"{"streams":[{"index":0,"codec_type":"video","width":160,"height":96,"r_frame_rate":"60/1","avg_frame_rate":"7200/137"}],"format":{"duration":"1.0"}}"#;
        let p = prepared(vfr, "out.mp4", "selftest-invert", Kind::Frame, |_| {});
        assert_eq!(p.decode_filter(), "fps=7200/137,scale=in_color_matrix=smpte170m");
        let got = lines(&p);
        assert!(got[0].contains("-vf fps=7200/137,scale=in_color_matrix=smpte170m"), "{}", got[0]);
        assert!(got[1].ends_with("--fps 7200/137"), "{}", got[1]);
        assert!(got[2].contains("-framerate 7200/137"), "{}", got[2]);
    }

    #[test]
    fn a_temporal_effect_gets_frames_scratch_layers_and_the_shortest_audio_cut() {
        let mut p = prepared(CLIP, "out.mp4", "retime", Kind::Temporal, |r| {
            r.frames = Some(6);
            r.depth = Some(Depth::U8);
            r.params = vec!["speed_percent=50".into()];
        });
        let layer_info = Info::from_json(
            r#"{"streams":[{"index":0,"codec_type":"video","width":80,"height":48,"r_frame_rate":"12/1","avg_frame_rate":"12/1"}]}"#,
            std::path::Path::new("track.mp4"),
        )
        .unwrap();
        p.layers = vec![crate::plan::LayerProbe { id: "alt".into(), video: "track.mp4".into(), info: layer_info }];
        let assembled = p.assemble("S");
        let temp = std::path::Path::new("S").join("layer_alt.rgba").to_string_lossy().into_owned();
        let got = assembled.commands.lines();
        assert_eq!(got.len(), 4, "the layer decode comes first");
        assert_eq!(
            got[0],
            format!("ffmpeg -v error -nostdin -y -i track.mp4 -map 0:0 -an -sn -dn -vf fps=24/1,scale=160:96:in_color_matrix=smpte170m -pix_fmt rgba -f rawvideo {}", crate::pytext::shlex_quote(&temp))
        );
        assert_eq!(
            got[2],
            format!(
                "vfx apply retime --width 160 --height 96 --depth 8 --fps 24/1 --frames 6 --tmp-dir S --layer {} --param speed_percent=50",
                crate::pytext::shlex_quote(&format!("alt={}", temp))
            )
        );
        assert!(got[3].contains("-c:a copy -shortest -movflags +faststart"), "{}", got[3]);
        assert_eq!(assembled.plan.audio, Some(AudioPlan::Shortest));
        assert_eq!(assembled.plan.temp_files.len(), 2);
        assert_eq!(assembled.plan.temp_files[0].what, "layer alt, decoded to raw rgba");
        let summary = p.summary(&assembled.plan);
        assert_eq!(
            summary,
            "retime on clip.mp4: 160x96, 24.000 fps, 8 bpc (rgba), temporal, 6 frames, layers alt, audio cut at the end of the video (-shortest), decode | vfx | encode -> out.mp4"
        );
    }

    #[test]
    fn a_temporal_effect_is_told_the_frames_the_container_states() {
        let p = prepared(CLIP, "out.mp4", "retime", Kind::Temporal, |_| {});
        let assembled = p.assemble("S");
        let vfx = &assembled.commands.lines()[1];
        assert_eq!(vfx, "vfx apply retime --width 160 --height 96 --depth 16 --fps 24/1 --frames 12 --tmp-dir S");
        assert_eq!(assembled.plan.frames, Some(12));
        assert_eq!(assembled.plan.audio, None);
        assert!(p.summary(&assembled.plan).contains("temporal, 12 frames"));
    }

    /// The decoder and encoder the pipeline starts run apply.py's arguments, apart from the
    /// spellings the crate fixes: no `-hide_banner`, `-loglevel error` for `-v error`, `pipe:0` and
    /// `pipe:1` for `-`, and `0:v:N` for `0:INDEX`.
    #[test]
    fn the_ffmpeg_builders_run_apply_py_s_arguments() {
        for (json, dst, configure) in [
            (CLIP, "out.mp4", (|_: &mut Request| {}) as fn(&mut Request)),
            (CLIP, "out.mkv", |r: &mut Request| {
                r.lossless = true;
                r.depth = Some(Depth::U8);
            }),
        ] {
            let p = prepared(json, dst, "selftest-invert", Kind::Frame, configure);
            let assembled = p.assemble("S");
            let cfg = pipeline_config(&p, &assembled);
            let bins = Binaries::new().with_ffmpeg("ffmpeg").with_ffprobe("ffprobe");

            let decode: Vec<String> = decoder_builder(&cfg, bins.clone()).command_line().iter().map(|a| a.to_string_lossy().into_owned()).collect();
            let mut fixed: Vec<String> = Vec::new();
            let mut it = decode.iter().peekable();
            while let Some(a) = it.next() {
                match a.as_str() {
                    "-hide_banner" => {}
                    "-loglevel" => {
                        fixed.push("-v".into());
                        fixed.push(it.next().unwrap().clone());
                    }
                    "-map" => {
                        it.next();
                        fixed.push("-map".into());
                        fixed.push(format!("0:{}", p.info.video_index.unwrap()));
                    }
                    "pipe:1" => fixed.push("-".into()),
                    other => fixed.push(other.to_string()),
                }
            }
            assert_eq!(fixed, assembled.commands.decode[1..], "decode");

            let encode: Vec<String> = encoder_builder(&cfg, bins).ok().unwrap().command_line().iter().map(|a| a.to_string_lossy().into_owned()).collect();
            let mut fixed: Vec<String> = Vec::new();
            let mut it = encode.iter();
            while let Some(a) = it.next() {
                match a.as_str() {
                    "-hide_banner" => {}
                    "-loglevel" => {
                        fixed.push("-v".into());
                        fixed.push(it.next().unwrap().clone());
                    }
                    "pipe:0" => fixed.push("-".into()),
                    other => fixed.push(other.to_string()),
                }
            }
            assert_eq!(fixed, assembled.commands.encode[1..], "encode");
        }
    }

    #[test]
    fn the_ordinal_of_the_video_stream_is_what_the_decoder_maps() {
        let json = r#"{"streams":[{"index":0,"codec_type":"video","codec_name":"mjpeg","width":8,"height":8,"disposition":{"attached_pic":1}},
            {"index":1,"codec_type":"audio","codec_name":"mp3","channels":2},
            {"index":2,"codec_type":"video","codec_name":"h264","width":160,"height":96,"r_frame_rate":"24/1","avg_frame_rate":"24/1"}]}"#;
        let p = prepared(json, "out.mp4", "selftest-invert", Kind::Frame, |_| {});
        let assembled = p.assemble("S");
        assert!(assembled.commands.decode.join(" ").contains("-map 0:2 "), "apply.py maps the container index");
        let cfg = pipeline_config(&p, &assembled);
        assert_eq!(cfg.video_ordinal, 1, "the decoder maps the position among the video streams");
        let args: Vec<String> = decoder_builder(&cfg, Binaries::new().with_ffmpeg("ffmpeg")).command_line().iter().map(|a| a.to_string_lossy().into_owned()).collect();
        let at = args.iter().position(|a| a == "-map").unwrap();
        assert_eq!(args[at + 1], "0:v:1");
    }

    #[test]
    fn a_frame_effect_has_no_frames_tmp_dir_or_temp_files() {
        let p = prepared(CLIP, "out.mp4", "selftest-invert", Kind::Frame, |_| {});
        let assembled = p.assemble("S");
        assert!(assembled.plan.temp_files.is_empty());
        assert_eq!(assembled.plan.frames, None);
        assert_eq!(assembled.plan.input_frames, Some(12));
    }

    #[test]
    fn the_result_document_names_the_contract_first_and_keeps_apply_py_s_keys() {
        let p = prepared(CLIP, "out.mp4", "selftest-invert", Kind::Frame, |_| {});
        let assembled = p.assemble("S");
        let doc = p.result(&assembled, Vec::new(), None, None);
        let keys: Vec<String> = match &doc {
            Out::Map(m) => m.iter().map(|(k, _)| k.clone()).collect(),
            _ => panic!("object"),
        };
        if p.request.contract == crate::error::Contract::V2 {
            assert_eq!((keys[0].as_str(), keys[1].as_str()), ("ok", "contract"));
            assert_eq!(doc.get("ok"), Some(&Out::Bool(true)));
        } else {
            assert_eq!(keys[0], "contract");
        }
        for key in [
            "operation", "input", "effect", "kind", "params", "depth", "pix_fmt", "size", "fps", "fps_exact", "frames", "input_frames", "layers", "audio", "temp_files", "vfx", "summary",
            "dry_run", "commands", "outputs",
        ] {
            assert!(keys.iter().any(|k| k == key), "{} missing from {:?}", key, keys);
        }
        assert_eq!(doc.get("fps_exact"), Some(&Out::text("24/1")));
        assert_eq!(doc.get("kind"), Some(&Out::text("frame")));
        assert!(!keys.contains(&"seconds".to_string()), "a dry run has no duration");
        let run = p.result(&assembled, Vec::new(), Some(1.5), Some(Vec::new()));
        assert_eq!(run.get("seconds"), Some(&Out::Float(1.5)));
        assert!(run.get("stage_reports").is_some());
    }
}
