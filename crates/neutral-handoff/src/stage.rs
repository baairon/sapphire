//! Frame stages of this repository: the registry, the `selftest-invert` conformance stage and
//! the loop that runs a stage over raw frames.
//!
//! A stage is a raw frame filter. It reads whole frames of the `--depth` format on a stream,
//! writes the same number of frames of the same format and size on another, and its failures
//! use the vfx exit codes (2 usage, 3 unsupported, 4 bad input) with one line
//! `error: <code>: frame N: ...` on standard error. The binary exposes the registry as
//! `sapphire stage list --json` and a stage as `sapphire stage apply STAGE ...`;
//! the pipeline can also run a stage in-process (`--stage @NAME`).
//!
//! No effect of the name bank is mapped to behaviour, so the registry holds the conformance stage
//! `selftest-invert` and the ffmpeg filter stage `ffmpeg-filter` ([`crate::ffstage`]) only.
//! `stage list --json` prints them as a stage manifest (`contracts/stage-manifest.schema.json`):
//! the provider (`id`, `version`, `command`) and per stage `id`, `kind`, `spool`, `geometry`,
//! `depths`, `params` and `layers`.
//!
//! `selftest-invert` has the definition of `vfx apply selftest-invert`
//! (`crates/fx-cli/src/selftest.rs` of twixtor, `Invert::render`): the three colour samples of
//! every pixel become `max - v` (255, 65535, or `1.0 - v` for the float depth), and alpha is
//! left alone unless the checkbox parameter `invert_alpha` is on. At depth 32 the colour samples
//! are the planes G, B and R of `gbrapf32le`, and alpha is the fourth plane. Applying it twice
//! returns the input exactly at depths 8 and 16; at depth 32 `1.0 - (1.0 - v)` returns `v` for
//! every value that `1.0 - v` represents exactly (for instance multiples of 1/256 in 0 to 1),
//! and may differ by the last bit for other values, as it does in `vfx`.

use std::io::{self, Read, Write};

use crate::error::{Error, Result};
use crate::format::{Depth, FrameSpec};
use crate::pyjson::Out;

/// A frame filter that works on one frame at a time, in place.
pub trait FrameStage: Send {
    /// Transform `frame` (exactly one raw frame; `index` counts from 0). An error text ends the run.
    fn process(&mut self, index: u64, frame: &mut [u8]) -> std::result::Result<(), String>;
}

/// How a parameter is typed.
#[derive(Clone, Debug, PartialEq)]
pub enum ParamKind {
    /// A checkbox; the value is `1`, `0`, `true`, `false`, `on` or `off`.
    Checkbox {
        /// The value when the parameter is not given.
        default: bool,
    },
    /// Free text (an ffmpeg filter graph, for example); any value is accepted.
    Text {
        /// The value when the parameter is not given.
        default: &'static str,
    },
    /// One of a fixed list of words (`popup` in the manifest).
    Choice {
        /// The value when the parameter is not given.
        default: &'static str,
        /// The accepted values.
        choices: &'static [&'static str],
    },
}

/// One parameter of a stage.
#[derive(Clone, Debug, PartialEq)]
pub struct ParamDef {
    /// The id used in `--param ID=VALUE`.
    pub id: &'static str,
    /// The display name.
    pub name: &'static str,
    /// The type and default.
    pub kind: ParamKind,
}

/// What `stage list` says about a stage.
#[derive(Clone, Debug, PartialEq)]
pub struct StageDef {
    /// The id used in `stage apply ID`.
    pub id: &'static str,
    /// The display name.
    pub name: &'static str,
    /// The group the stage belongs to.
    pub family: &'static str,
    /// `frame` here: frame N of the output depends on frame N of the input only.
    pub kind: &'static str,
    /// The stage reads its whole input before it writes (false for every stage here).
    pub spool: bool,
    /// `preserve` or `declared` (`docs/handoff.md#geometry`).
    pub geometry: &'static str,
    /// The depths the stage accepts.
    pub depths: Vec<u32>,
    /// The parameters.
    pub params: Vec<ParamDef>,
}

/// Every stage this tool offers.
pub fn definitions() -> Vec<StageDef> {
    vec![
        StageDef {
            id: "selftest-invert",
            name: "Selftest Invert",
            family: "selftest",
            kind: "frame",
            spool: false,
            geometry: "preserve",
            depths: Depth::ALL.iter().map(|d| d.bits()).collect(),
            params: vec![ParamDef { id: "invert_alpha", name: "Invert Alpha", kind: ParamKind::Checkbox { default: false } }],
        },
        StageDef {
            id: crate::ffstage::ID,
            name: "ffmpeg Filter",
            family: "ffmpeg",
            kind: "frame",
            spool: false,
            geometry: "declared",
            depths: Depth::ALL.iter().map(|d| d.bits()).collect(),
            params: vec![
                ParamDef { id: "graph", name: "Graph", kind: ParamKind::Text { default: "null" } },
                ParamDef { id: "expect", name: "Expect", kind: ParamKind::Choice { default: "preserve", choices: &["preserve", "declared"] } },
                ParamDef { id: "unsafe", name: "Unsafe", kind: ParamKind::Checkbox { default: false } },
            ],
        },
    ]
}

/// The definition of the stage `id`.
pub fn find(id: &str) -> Option<StageDef> {
    definitions().into_iter().find(|d| d.id == id)
}

/// The program that printed the manifest, for the provider `command`.
pub fn program() -> String {
    std::env::current_exe().map(|p| p.to_string_lossy().into_owned()).unwrap_or_else(|_| "sapphire".into())
}

/// The `stage list --json` document under frames/2 (see [`list_manifest`]).
pub fn list_document() -> Out {
    list_manifest(crate::Contract::V2)
}

/// The `stage list --json` document: a stage manifest (`contracts/stage-manifest.schema.json`)
/// with the provider, the stages and, as extra keys, the tool, its version, the depths and their
/// pixel formats.
pub fn list_manifest(contract: crate::Contract) -> Out {
    let depths: Vec<Out> = Depth::ALL.iter().map(|d| Out::Int(d.bits() as i64)).collect();
    let pix_fmts = Out::Map(Depth::ALL.iter().map(|d| (d.bits().to_string(), Out::text(d.pix_fmt_name()))).collect());
    let stages: Vec<Out> = definitions()
        .iter()
        .map(|def| {
            let params: Vec<Out> = def
                .params
                .iter()
                .map(|p| match &p.kind {
                    ParamKind::Checkbox { default } => Out::map(vec![
                        ("id", Out::text(p.id)),
                        ("name", Out::text(p.name)),
                        ("kind", Out::text("checkbox")),
                        ("default", Out::Bool(*default)),
                    ]),
                    ParamKind::Text { default } => Out::map(vec![
                        ("id", Out::text(p.id)),
                        ("name", Out::text(p.name)),
                        ("kind", Out::text("text")),
                        ("default", Out::text(*default)),
                    ]),
                    ParamKind::Choice { default, choices } => Out::map(vec![
                        ("id", Out::text(p.id)),
                        ("name", Out::text(p.name)),
                        ("kind", Out::text("popup")),
                        ("default", Out::text(*default)),
                        ("choices", Out::List(choices.iter().map(|c| Out::text(*c)).collect())),
                    ]),
                })
                .collect();
            Out::map(vec![
                ("id", Out::text(def.id)),
                ("name", Out::text(def.name)),
                ("family", Out::text(def.family)),
                ("kind", Out::text(def.kind)),
                ("spool", Out::Bool(def.spool)),
                ("geometry", Out::text(def.geometry)),
                ("hidden", Out::Bool(false)),
                ("depths", Out::List(def.depths.iter().map(|d| Out::Int(*d as i64)).collect())),
                ("params", Out::List(params)),
                ("layers", Out::List(Vec::new())),
            ])
        })
        .collect();
    let provider = Out::map(vec![
        ("id", Out::text("sapphire")),
        ("version", Out::text(format!("sapphire {}", env!("CARGO_PKG_VERSION")))),
        ("command", Out::List(vec![Out::text(program()), Out::text("stage")])),
    ]);
    Out::map(vec![
        ("contract", Out::text(contract.as_str())),
        ("provider", provider),
        ("tool", Out::text("sapphire")),
        ("version", Out::text(env!("CARGO_PKG_VERSION"))),
        ("depths", Out::List(depths)),
        ("pix_fmts", pix_fmts),
        ("stages", Out::List(stages)),
    ])
}

/// The text table of `stage list`.
pub fn list_text() -> String {
    let mut out = String::from("id               name             family    kind   depths    params  layers\n");
    for def in definitions() {
        let depths: Vec<String> = def.depths.iter().map(|d| d.to_string()).collect();
        out.push_str(&format!(
            "{:<16} {:<16} {:<9} {:<6} {:<9} {:<7} {}\n",
            def.id,
            def.name,
            def.family,
            def.kind,
            depths.join(","),
            def.params.len(),
            0
        ));
    }
    out
}

/// A checkbox value: `1`, `true`, `on` or `0`, `false`, `off`.
pub fn parse_bool(text: &str) -> Option<bool> {
    match text {
        "1" | "true" | "on" => Some(true),
        "0" | "false" | "off" => Some(false),
        _ => None,
    }
}

/// `--param ID=VALUE` pairs checked against a stage: unknown ids, bad values and keyframes are
/// `unsupported` (vfx exit 3), an id given twice is `usage` (exit 2).
pub fn check_params(def: &StageDef, params: &[String]) -> Result<Vec<(String, String)>> {
    let mut seen: Vec<(String, String)> = Vec::new();
    for raw in params {
        let (id, value) = match raw.split_once('=') {
            Some((id, value)) if !id.is_empty() => (id, value),
            _ => return Err(Error::usage(format!("`--param` expects ID=VALUE, got `{}`", raw))),
        };
        if seen.iter().any(|(seen_id, _)| seen_id == id) {
            return Err(Error::usage(format!("param `{}` given more than once", id)));
        }
        let param = match def.params.iter().find(|p| p.id == id) {
            Some(p) => p,
            None => return Err(Error::unsupported(format!("unknown param `{}`", id))),
        };
        if value.contains('@') && !matches!(param.kind, ParamKind::Text { .. }) {
            return Err(Error::unsupported(format!(
                "bad value for `{}`: keyframes need a temporal effect, and {} is a frame effect",
                param.id, def.id
            )));
        }
        match &param.kind {
            ParamKind::Checkbox { .. } => {
                if parse_bool(value).is_none() {
                    return Err(Error::unsupported(format!("bad value for `{}`: cannot parse `{}`", param.id, value)));
                }
            }
            ParamKind::Text { .. } => {}
            ParamKind::Choice { choices, .. } => {
                if !choices.contains(&value) {
                    return Err(Error::unsupported(format!("bad value for `{}`: `{}` is not one of {}", param.id, value, choices.join(", "))));
                }
            }
        }
        seen.push((id.to_string(), value.to_string()));
    }
    Ok(seen)
}

/// The conformance stage.
#[derive(Clone, Debug)]
pub struct Invert {
    spec: FrameSpec,
    channels: usize,
}

impl Invert {
    /// An inverter for frames of `spec`; `invert_alpha` also inverts the alpha samples.
    pub fn new(spec: FrameSpec, invert_alpha: bool) -> Invert {
        Invert { spec, channels: if invert_alpha { 4 } else { 3 } }
    }
}

impl FrameStage for Invert {
    fn process(&mut self, _index: u64, frame: &mut [u8]) -> std::result::Result<(), String> {
        let want = self.spec.frame_bytes();
        if frame.len() as u64 != want {
            return Err(format!("a frame of {} bytes for a {}x{} frame of {} bytes", frame.len(), self.spec.width, self.spec.height, want));
        }
        match self.spec.depth {
            Depth::U8 => {
                for pixel in frame.chunks_exact_mut(4) {
                    for c in &mut pixel[..self.channels] {
                        *c = 255 - *c;
                    }
                }
            }
            Depth::U16 => {
                for pixel in frame.chunks_exact_mut(8) {
                    for c in pixel[..self.channels * 2].chunks_exact_mut(2) {
                        let v = u16::from_le_bytes([c[0], c[1]]);
                        c.copy_from_slice(&(65535 - v).to_le_bytes());
                    }
                }
            }
            Depth::F32 => {
                let plane = self.spec.width as usize * self.spec.height as usize * 4;
                for p in 0..self.channels {
                    for c in frame[p * plane..(p + 1) * plane].chunks_exact_mut(4) {
                        let v = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
                        c.copy_from_slice(&(1.0f32 - v).to_le_bytes());
                    }
                }
            }
        }
        Ok(())
    }
}

/// Build the stage `id` for frames of `spec` with `params`.
pub fn build(id: &str, spec: FrameSpec, params: &[String]) -> Result<Box<dyn FrameStage>> {
    let def = match find(id) {
        Some(d) => d,
        None => return Err(Error::unsupported(format!("unknown stage `{}`, `sapphire stage list` shows the registered ones", id))),
    };
    let values = check_params(&def, params)?;
    match def.id {
        "selftest-invert" => {
            let alpha = values.iter().find(|(k, _)| k == "invert_alpha").map(|(_, v)| parse_bool(v) == Some(true)).unwrap_or(false);
            Ok(Box::new(Invert::new(spec, alpha)))
        }
        other => Err(Error::unsupported(format!("stage `{}` runs as a process (`sapphire stage apply {}`), not in-process", other, other))),
    }
}

/// The largest frame a stage handles, as in vfx.
pub const MAX_FRAME_BYTES: u64 = 1 << 32;

/// Read whole frames from `input`, run `stage` on each and write the result to `output`.
///
/// Returns the number of frames written. Frames finished before an error stay written. A stream
/// that ends inside a frame is `partial_frame` (exit 4) naming the frame, a write that fails is
/// `broken_pipe` (exit 4) or `bad_input`, and `expect_frames` that does not match is `bad_input`.
pub fn run_frames<R: Read, W: Write>(
    stage: &mut dyn FrameStage,
    spec: FrameSpec,
    input: &mut R,
    output: &mut W,
    expect_frames: Option<u64>,
) -> Result<u64> {
    let bytes = spec.frame_bytes();
    if bytes > MAX_FRAME_BYTES {
        return Err(Error::usage(format!(
            "a {}x{} frame at depth {} is {} bytes; one frame may be at most 4 GiB",
            spec.width,
            spec.height,
            spec.depth.bits(),
            bytes
        )));
    }
    let mut buf = vec![0u8; bytes as usize];
    let mut count: u64 = 0;
    let result = (|| -> Result<u64> {
        loop {
            let mut got = 0usize;
            while got < buf.len() {
                match input.read(&mut buf[got..]) {
                    Ok(0) => break,
                    Ok(n) => got += n,
                    Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                    Err(e) => return Err(Error::bad_input(format!("frame {}: cannot read the input: {}", count, e))),
                }
            }
            if got == 0 {
                return Ok(count);
            }
            if got < buf.len() {
                return Err(Error::new(
                    crate::error::Code::PartialFrame,
                    format!("frame {}: input ends with a partial frame ({} of {} bytes) after {} whole frames", count, got, buf.len(), count),
                ));
            }
            stage.process(count, &mut buf).map_err(|e| Error::unsupported(format!("frame {}: {}", count, e)))?;
            if let Err(e) = output.write_all(&buf) {
                let code = if e.kind() == io::ErrorKind::BrokenPipe { crate::error::Code::BrokenPipe } else { crate::error::Code::BadInput };
                return Err(Error::new(code, format!("frame {}: cannot write the output: {}", count, e)));
            }
            count += 1;
        }
    })();
    let flushed = output.flush();
    let frames = result?;
    if let Err(e) = flushed {
        let code = if e.kind() == io::ErrorKind::BrokenPipe { crate::error::Code::BrokenPipe } else { crate::error::Code::BadInput };
        return Err(Error::new(code, format!("frame {}: cannot write the output: {}", frames, e)));
    }
    if let Some(want) = expect_frames {
        if want != frames {
            return Err(Error::bad_input(format!("the input holds {} frames, but --frames says {}", frames, want)));
        }
    }
    Ok(frames)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Code;

    fn spec(depth: Depth) -> FrameSpec {
        FrameSpec { width: 5, height: 3, depth }
    }

    /// Deterministic bytes that cover the whole range of every sample.
    fn pattern(depth: Depth, seed: u8) -> Vec<u8> {
        let n = spec(depth).frame_bytes() as usize;
        match depth {
            Depth::F32 => {
                // multiples of 1/256, so that 1.0 - v is exact and a double inversion is the identity
                let mut out = Vec::with_capacity(n);
                for i in 0..n / 4 {
                    let v = ((i as u32 * 37 + seed as u32 * 11) % 257) as f32 / 256.0;
                    out.extend_from_slice(&v.to_le_bytes());
                }
                out
            }
            _ => (0..n).map(|i| (i as u32).wrapping_mul(131).wrapping_add(seed as u32 * 17) as u8).collect(),
        }
    }

    #[test]
    fn the_registry_holds_the_conformance_and_ffmpeg_stages_only() {
        let defs = definitions();
        let ids: Vec<&str> = defs.iter().map(|d| d.id).collect();
        assert_eq!(ids, ["selftest-invert", "ffmpeg-filter"]);
        assert_eq!(defs[0].depths, vec![8, 16, 32]);
        assert_eq!((defs[0].geometry, defs[0].spool), ("preserve", false));
        assert_eq!((defs[1].geometry, defs[1].spool), ("declared", false));
        assert!(find("selftest-invert").is_some());
        assert!(find("retime").is_none(), "twixtor's effects are not stages of this tool");
    }

    #[test]
    fn text_and_choice_params_are_checked() {
        let def = find("ffmpeg-filter").unwrap();
        let ok = check_params(&def, &["graph=drawbox@a=x=1".into(), "expect=declared".into()]).unwrap();
        assert_eq!(ok.len(), 2, "an @ in a graph is an ffmpeg instance name, not a keyframe");
        let bad = check_params(&def, &["expect=sometimes".into()]).unwrap_err();
        assert_eq!(bad.code, crate::error::Code::Unsupported);
        assert!(bad.message.contains("is not one of preserve, declared"), "{}", bad.message);
    }

    #[test]
    fn the_manifest_names_the_provider_and_every_stage_key() {
        let doc = list_manifest(crate::Contract::V2);
        assert_eq!(doc.get("contract"), Some(&Out::text("frames/2")));
        let text = doc.dumps();
        assert!(text.contains(r#""provider": {"id": "sapphire", "version": "sapphire "#), "{}", text);
        assert!(text.contains(r#""stage"]}"#), "the command ends with the fixed word stage: {}", text);
        for key in ["\"spool\": false", "\"geometry\": \"preserve\"", "\"geometry\": \"declared\"", "\"layers\": []", "\"kind\": \"popup\"", "\"choices\": [\"preserve\", \"declared\"]"] {
            assert!(text.contains(key), "{} in {}", key, text);
        }
        assert_eq!(list_manifest(crate::Contract::V1).get("contract"), Some(&Out::text("frames/1")));
    }

    #[test]
    fn invert_twice_is_the_identity_at_every_depth() {
        for depth in Depth::ALL {
            for alpha in [false, true] {
                let original = pattern(depth, 3);
                let mut frame = original.clone();
                let mut stage = Invert::new(spec(depth), alpha);
                stage.process(0, &mut frame).unwrap();
                assert_ne!(frame, original, "{:?} alpha={}", depth, alpha);
                stage.process(1, &mut frame).unwrap();
                assert_eq!(frame, original, "{:?} alpha={}", depth, alpha);
            }
        }
    }

    #[test]
    fn depth_8_inverts_rgb_and_keeps_alpha() {
        let mut frame = vec![0u8; spec(Depth::U8).frame_bytes() as usize];
        frame[..4].copy_from_slice(&[10, 20, 30, 40]);
        frame[4..8].copy_from_slice(&[0, 255, 128, 255]);
        let mut stage = Invert::new(spec(Depth::U8), false);
        stage.process(0, &mut frame).unwrap();
        assert_eq!(&frame[..8], &[245, 235, 225, 40, 255, 0, 127, 255]);
        let mut with_alpha = Invert::new(spec(Depth::U8), true);
        with_alpha.process(0, &mut frame).unwrap();
        assert_eq!(&frame[..8], &[10, 20, 30, 215, 0, 255, 128, 0]);
    }

    #[test]
    fn depth_16_works_on_little_endian_samples() {
        let mut frame = vec![0u8; spec(Depth::U16).frame_bytes() as usize];
        let samples: [u16; 4] = [0x0102, 0xfffe, 0x8000, 0x1234];
        for (i, s) in samples.iter().enumerate() {
            frame[i * 2..i * 2 + 2].copy_from_slice(&s.to_le_bytes());
        }
        Invert::new(spec(Depth::U16), false).process(0, &mut frame).unwrap();
        let read = |i: usize| u16::from_le_bytes([frame[i * 2], frame[i * 2 + 1]]);
        assert_eq!((read(0), read(1), read(2), read(3)), (65535 - 0x0102, 1, 65535 - 0x8000, 0x1234));
    }

    #[test]
    fn depth_32_inverts_the_g_b_r_planes_and_keeps_the_alpha_plane() {
        let s = spec(Depth::F32);
        let plane = (s.width * s.height) as usize * 4;
        let mut frame = vec![0u8; s.frame_bytes() as usize];
        for p in 0..4usize {
            for i in 0..(plane / 4) {
                let v = (p as f32 + 1.0) / 8.0;
                frame[p * plane + i * 4..p * plane + i * 4 + 4].copy_from_slice(&v.to_le_bytes());
            }
        }
        Invert::new(s, false).process(0, &mut frame).unwrap();
        let sample = |p: usize| f32::from_le_bytes(frame[p * plane..p * plane + 4].try_into().unwrap());
        assert_eq!((sample(0), sample(1), sample(2), sample(3)), (1.0 - 0.125, 1.0 - 0.25, 1.0 - 0.375, 0.5));
        Invert::new(s, true).process(0, &mut frame).unwrap();
        let sample = |p: usize| f32::from_le_bytes(frame[p * plane..p * plane + 4].try_into().unwrap());
        assert_eq!((sample(0), sample(1), sample(2), sample(3)), (0.125, 0.25, 0.375, 0.5), "alpha flipped once, colour back");
    }

    #[test]
    fn depth_32_double_inversion_is_not_exact_for_every_float() {
        // The same arithmetic as vfx: 1.0 - (1.0 - 0.1) is not 0.1 in single precision.
        let s = FrameSpec { width: 1, height: 1, depth: Depth::F32 };
        let mut frame = Vec::new();
        for v in [0.1f32, 0.1, 0.1, 0.1] {
            frame.extend_from_slice(&v.to_le_bytes());
        }
        let original = frame.clone();
        let mut stage = Invert::new(s, false);
        stage.process(0, &mut frame).unwrap();
        stage.process(1, &mut frame).unwrap();
        let back = f32::from_le_bytes(frame[..4].try_into().unwrap());
        assert_eq!(back, 1.0f32 - (1.0f32 - 0.1f32));
        assert!((back - 0.1).abs() < 1e-7);
        assert_eq!(frame[12..], original[12..], "alpha is untouched");
    }

    #[test]
    fn a_frame_of_the_wrong_length_is_refused() {
        let mut stage = Invert::new(spec(Depth::U8), false);
        let err = stage.process(0, &mut [0u8; 7]).unwrap_err();
        assert!(err.contains("7 bytes"), "{}", err);
    }

    #[test]
    fn params_follow_vfx() {
        let def = find("selftest-invert").unwrap();
        assert!(check_params(&def, &[]).unwrap().is_empty());
        for ok in ["1", "0", "true", "false", "on", "off"] {
            assert!(check_params(&def, &[format!("invert_alpha={}", ok)]).is_ok(), "{}", ok);
        }
        let unknown = check_params(&def, &["nope=1".into()]).unwrap_err();
        assert_eq!((unknown.code, unknown.message.as_str()), (Code::Unsupported, "unknown param `nope`"));
        let bad = check_params(&def, &["invert_alpha=maybe".into()]).unwrap_err();
        assert_eq!(bad.message, "bad value for `invert_alpha`: cannot parse `maybe`");
        assert_eq!(bad.exit_code(), 3);
        let upper = check_params(&def, &["invert_alpha=TRUE".into()]).unwrap_err();
        assert_eq!(upper.code, Code::Unsupported);
        let twice = check_params(&def, &["invert_alpha=1".into(), "invert_alpha=0".into()]).unwrap_err();
        assert_eq!((twice.code, twice.message.as_str()), (Code::Usage, "param `invert_alpha` given more than once"));
        let keys = check_params(&def, &["invert_alpha=1@0".into()]).unwrap_err();
        assert_eq!(
            keys.message,
            "bad value for `invert_alpha`: keyframes need a temporal effect, and selftest-invert is a frame effect"
        );
        let shape = check_params(&def, &["invert_alpha".into()]).unwrap_err();
        assert_eq!(shape.code, Code::Usage);
    }

    #[test]
    fn build_refuses_an_unknown_stage() {
        let err = build("retime", spec(Depth::U8), &[]).err().unwrap();
        assert_eq!(err.code, Code::Unsupported);
        assert!(err.message.starts_with("unknown stage `retime`"));
        assert!(build("selftest-invert", spec(Depth::U8), &["invert_alpha=on".into()]).is_ok());
    }

    #[test]
    fn list_document_reports_what_exists() {
        let doc = list_document();
        assert_eq!(doc.get("contract"), Some(&Out::text("frames/2")));
        let text = doc.dumps();
        assert!(text.contains(r#""pix_fmts": {"8": "rgba", "16": "rgba64le", "32": "gbrapf32le"}"#), "{}", text);
        assert!(text.contains(r#""depths": [8, 16, 32]"#));
        assert!(text.contains(r#""id": "selftest-invert""#));
        assert!(text.contains(r#""kind": "frame""#));
        assert!(list_text().lines().nth(1).unwrap().starts_with("selftest-invert"));
    }

    #[test]
    fn run_frames_streams_whole_frames() {
        let s = spec(Depth::U8);
        let mut input = Vec::new();
        let frames: Vec<Vec<u8>> = (0..4).map(|i| pattern(Depth::U8, i)).collect();
        for f in &frames {
            input.extend_from_slice(f);
        }
        let mut stage = Invert::new(s, false);
        let mut out = Vec::new();
        let n = run_frames(&mut stage, s, &mut input.as_slice(), &mut out, Some(4)).unwrap();
        assert_eq!(n, 4);
        let mut back = out.clone();
        let mut second = Vec::new();
        run_frames(&mut Invert::new(s, false), s, &mut back.as_slice(), &mut second, None).unwrap();
        assert_eq!(second, input);
        back.clear();
    }

    #[test]
    fn a_partial_frame_is_exit_4_and_names_the_frame() {
        let s = spec(Depth::U8);
        let bytes = s.frame_bytes() as usize;
        let mut input = vec![9u8; bytes * 2 + 5];
        input[0] = 1;
        let mut out = Vec::new();
        let err = run_frames(&mut Invert::new(s, false), s, &mut input.as_slice(), &mut out, None).unwrap_err();
        assert_eq!(err.code, Code::PartialFrame);
        assert_eq!(err.exit_code(), 4);
        assert_eq!(err.message, format!("frame 2: input ends with a partial frame (5 of {} bytes) after 2 whole frames", bytes));
        assert_eq!(out.len(), bytes * 2, "frames finished before the error stay written");
    }

    #[test]
    fn empty_input_is_zero_frames() {
        let s = spec(Depth::U16);
        let mut out = Vec::new();
        let n = run_frames(&mut Invert::new(s, false), s, &mut io::empty(), &mut out, None).unwrap();
        assert_eq!((n, out.len()), (0, 0));
    }

    #[test]
    fn a_frame_count_that_disagrees_is_bad_input() {
        let s = spec(Depth::U8);
        let input = vec![0u8; s.frame_bytes() as usize * 3];
        let err = run_frames(&mut Invert::new(s, false), s, &mut input.as_slice(), &mut Vec::new(), Some(5)).unwrap_err();
        assert_eq!(err.code, Code::BadInput);
        assert_eq!(err.message, "the input holds 3 frames, but --frames says 5");
    }

    struct Failing;
    impl FrameStage for Failing {
        fn process(&mut self, index: u64, _frame: &mut [u8]) -> std::result::Result<(), String> {
            if index == 1 {
                Err("unsupported: nothing to do".into())
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn a_stage_error_names_its_frame() {
        let s = spec(Depth::U8);
        let input = vec![0u8; s.frame_bytes() as usize * 3];
        let mut out = Vec::new();
        let err = run_frames(&mut Failing, s, &mut input.as_slice(), &mut out, None).unwrap_err();
        assert_eq!((err.code, err.message.as_str()), (Code::Unsupported, "frame 1: unsupported: nothing to do"));
        assert_eq!(out.len(), s.frame_bytes() as usize);
    }

    struct Closed;
    impl Write for Closed {
        fn write(&mut self, _: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "pipe closed"))
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn a_closed_output_is_broken_pipe() {
        let s = spec(Depth::U8);
        let input = vec![0u8; s.frame_bytes() as usize];
        let err = run_frames(&mut Invert::new(s, false), s, &mut input.as_slice(), &mut Closed, None).unwrap_err();
        assert_eq!((err.code, err.exit_code()), (Code::BrokenPipe, 4));
        assert!(err.message.starts_with("frame 0: cannot write the output"));
    }

    #[test]
    fn a_frame_over_4_gib_is_a_usage_error() {
        let s = FrameSpec { width: 40000, height: 40000, depth: Depth::F32 };
        let err = run_frames(&mut Failing, s, &mut io::empty(), &mut Vec::new(), None).unwrap_err();
        assert_eq!(err.code, Code::Usage);
    }
}
