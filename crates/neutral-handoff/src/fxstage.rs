//! The effects stage provider: one frames/2 stage per registered style of the landed effects.
//!
//! Every stage runs `fx_run` as a child process on the recorded host route (the 'ae32' route that
//! the frozen predictions used), so the proven binary carries the parity and this module touches
//! no effect crate. The data is in two tables:
//!
//! - `data/fx_stages.tsv`: per style the stage id, program, family, kind, spool, layers, the name of
//!   the source input, the fixed options, the canonical host tuple and the buffer placement per
//!   measured size.
//! - `data/fx_params.tsv`: per offered parameter its id, the `fx_run` option form, type, lanes,
//!   registered default and range.
//!
//! The stage holds no plugin table. The type, lanes, range and registered default of a parameter are
//! asked from `fx_run --describe-params`, which prints them from the native definitions, and an
//! unbound normalize_scale parameter is left to `fx_run --ns-default-unbound`.
//!
//! Per frame the stage writes the input as an 'ae32' word file (A, R, G, B words per pixel), runs
//! `fx_run` and relays the `--ae32-out` words back. At depth 32 the relayout is a pure byte
//! permutation (frames/2 `gbrapf32le` is planar G, B, R, A, 'ae32' is A, R, G, B per pixel, both
//! f32 and straight alpha). Depths 8 and 16 convert through f32 (`v/255`, `v/65535`, back with
//! round half to even and a clamp) and are outside the parity claim.
//!
//! `fx_run` is found beside the `sapphire` executable or through `SAPPHIRE_FX_RUN`; it needs no
//! external data. Without it the stages are listed hidden and `apply` exits 3 naming what is missing.

use std::collections::VecDeque;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use crate::error::{tail_lines, Code, Error, Result};
use crate::format::{Depth, FrameSpec};
use crate::stage::{LayerDef, ParamDef, ParamKind, StageDef};

const STAGES_TSV: &str = include_str!("../data/fx_stages.tsv");
const PARAMS_TSV: &str = include_str!("../data/fx_params.tsv");

/// The environment variable that names the `fx_run` executable.
pub const FX_RUN_ENV: &str = "SAPPHIRE_FX_RUN";

/// One offered parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct Param {
    /// The id of `--param ID=VALUE`.
    pub id: String,
    /// `--set`, `--host-param`, `--cset` or a plain option such as `--host-curve-values`.
    pub form: String,
    /// The name after the form (`NAME=VALUE`).
    pub name: String,
    /// `float`, `int`, `checkbox` or `text`.
    pub kind: String,
    /// The number of lanes of a vector value.
    pub lanes: usize,
    /// The registered default, as text.
    pub default: String,
    /// The smallest value.
    pub min: Option<f64>,
    /// The largest value.
    pub max: Option<f64>,
}

/// One style.
#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    /// The stage id.
    pub id: String,
    /// The registered program name.
    pub program: String,
    /// The bank effect string.
    pub family: String,
    /// `frame` or `temporal`.
    pub kind: String,
    /// The stage buffers its whole input.
    pub spool: bool,
    /// Layer ids with the optional flag.
    pub layers: Vec<(String, bool)>,
    /// The name of the source input.
    pub src_name: String,
    /// The fixed options (`--set a=1` as `("--set", "a=1")` pairs of option and value).
    pub fixed: Vec<(String, String)>,
    /// The canonical host tuple.
    pub host: Vec<(String, String)>,
    /// The prior fill word of the destination buffer.
    pub prior_fill: u32,
    /// The buffer placement per measured size (`WxH`).
    pub placement: Vec<(String, Vec<(String, String)>)>,
    /// The offered parameters.
    pub params: Vec<Param>,
}

fn pairs(text: &str) -> Vec<(String, String)> {
    text.split('|')
        .filter(|t| !t.is_empty())
        .map(|t| match t.split_once(' ') {
            Some((a, b)) => (a.to_string(), b.to_string()),
            None => (t.to_string(), String::new()),
        })
        .collect()
}

/// Parse the two tables.
pub fn parse_tables(stages: &str, params: &str) -> Vec<Profile> {
    let mut out: Vec<Profile> = Vec::new();
    for line in stages.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 11 {
            continue;
        }
        let mut host = pairs(c[9]);
        let mut prior_fill = 0u32;
        host.retain(|(k, v)| {
            if k == "--prior-fill" {
                prior_fill = u32::from_str_radix(v, 16).unwrap_or(0);
                false
            } else {
                true
            }
        });
        let placement = c[10]
            .split('/')
            .filter(|t| !t.is_empty())
            .filter_map(|t| t.split_once('=').map(|(s, o)| (s.to_string(), pairs(o))))
            .collect();
        out.push(Profile {
            id: c[0].to_string(),
            program: c[1].to_string(),
            family: c[2].to_string(),
            kind: c[3].to_string(),
            spool: c[4] == "true",
            layers: c[5]
                .split(',')
                .filter(|t| *t != "-" && !t.is_empty())
                .map(|t| (t.trim_end_matches('?').to_string(), t.ends_with('?')))
                .collect(),
            src_name: c[6].to_string(),
            fixed: pairs(c[8]),
            host,
            prior_fill,
            placement,
            params: Vec::new(),
        });
    }
    for line in params.lines().skip(1).filter(|l| !l.trim().is_empty()) {
        let c: Vec<&str> = line.split('\t').collect();
        if c.len() < 4 {
            continue;
        }
        if let Some(p) = out.iter_mut().find(|p| p.id == c[0]) {
            // type, lanes, range and default are filled in from `fx_run --describe-params` (see `enrich`)
            let curve = c[2] == "--host-curve-values";
            p.params.push(Param {
                id: c[1].to_string(),
                form: c[2].to_string(),
                name: c[3].to_string(),
                kind: if curve { "text".into() } else if c[2] == "--cset" { "int".into() } else { "float".into() },
                lanes: 1,
                default: String::new(),
                min: if c[2] == "--cset" { Some(0.0) } else { None },
                max: None,
            });
        }
    }
    out
}

/// The profiles of the built-in tables.
pub fn profiles() -> &'static [Profile] {
    static P: OnceLock<Vec<Profile>> = OnceLock::new();
    P.get_or_init(|| {
        let mut ps = parse_tables(STAGES_TSV, PARAMS_TSV);
        if let Some(described) = describe_programs(&ps) {
            enrich(&mut ps, &described);
        }
        ps
    })
}

/// One parameter as `fx_run --describe-params` prints it.
#[derive(Clone, Debug, PartialEq)]
pub struct Described {
    /// the program parameter name
    pub name: String,
    /// scalar, boolean, popup, int, color, vector or angle
    pub ptype: String,
    /// the number of lanes
    pub lanes: usize,
    /// the smallest value
    pub min: f64,
    /// the largest value
    pub max: f64,
    /// the default text, one value per lane
    pub default: String,
}

/// Parse the document of `fx_run --describe-params`: `{"PROGRAM":[{"name":..,"type":..,"lanes":..,"min":..,"max":..,"default":..}]}`.
pub fn parse_described(text: &str) -> Option<Vec<(String, Vec<Described>)>> {
    let doc = neutral_ffmpeg::json::Json::parse(text.trim()).ok()?;
    let members = doc.as_object()?;
    let mut out = Vec::new();
    for (prog, list) in members {
        let mut v = Vec::new();
        for it in list.as_array()? {
            v.push(Described {
                name: it.get("name")?.as_str()?.to_string(),
                ptype: it.get("type")?.as_str()?.to_string(),
                lanes: it.get("lanes")?.as_i64()? as usize,
                min: it.get("min")?.as_f64()?,
                max: it.get("max")?.as_f64()?,
                default: it.get("default")?.as_str()?.to_string(),
            });
        }
        out.push((prog.clone(), v));
    }
    Some(out)
}

fn describe_programs(ps: &[Profile]) -> Option<Vec<(String, Vec<Described>)>> {
    let fx = fx_run_path()?;
    let mut cmd = Command::new(fx);
    cmd.arg("--describe-params");
    for p in ps {
        cmd.arg(&p.program);
    }
    let ran = cmd.output().ok()?;
    if ran.status.code() != Some(0) {
        return None;
    }
    parse_described(&String::from_utf8_lossy(&ran.stdout))
}

/// Fill the type, lanes, range and default of each parameter from the described definitions.
pub fn enrich(ps: &mut [Profile], described: &[(String, Vec<Described>)]) {
    for p in ps.iter_mut() {
        let list = match described.iter().find(|(n, _)| *n == p.program) {
            Some((_, l)) => l,
            None => continue,
        };
        for q in p.params.iter_mut() {
            if q.form == "--cset" || q.form == "--host-curve-values" {
                continue;
            }
            if let Some(d) = list.iter().find(|d| d.name == q.name) {
                q.kind = match d.ptype.as_str() {
                    "boolean" if d.lanes == 1 => "checkbox",
                    "popup" | "int" if d.lanes == 1 => "int",
                    "scalar" | "angle" if d.lanes == 1 => "float",
                    _ => "text",
                }
                .to_string();
                q.lanes = d.lanes;
                q.default = d.default.split_whitespace().collect::<Vec<_>>().join(",");
                q.min = Some(d.min);
                q.max = Some(d.max);
            }
        }
    }
}

/// The profile of the stage `id`.
pub fn profile(id: &str) -> Option<&'static Profile> {
    profiles().iter().find(|p| p.id == id)
}

fn leak(s: &str) -> &'static str {
    Box::leak(s.to_string().into_boxed_str())
}

/// Where `fx_run` is.
pub fn fx_run_path() -> Option<PathBuf> {
    if let Some(p) = std::env::var_os(FX_RUN_ENV) {
        let p = PathBuf::from(p);
        return if p.is_file() { Some(p) } else { None };
    }
    let exe = std::env::current_exe().ok()?;
    let name = if cfg!(windows) { "fx_run.exe" } else { "fx_run" };
    let p = exe.parent()?.join(name);
    if p.is_file() {
        Some(p)
    } else {
        None
    }
}

/// What is missing for the effect stages to run, as one line, or `None`.
pub fn missing() -> Option<String> {
    if fx_run_path().is_none() {
        return Some(format!("fx_run is not beside the sapphire executable and {} does not name it", FX_RUN_ENV));
    }
    None
}

/// The stage definitions (hidden while the bundle is absent).
pub fn definitions() -> Vec<StageDef> {
    let hidden = missing().is_some();
    profiles()
        .iter()
        .map(|p| StageDef {
            id: leak(&p.id),
            name: leak(&p.program),
            family: leak(&p.family),
            kind: leak(&p.kind),
            spool: p.spool,
            geometry: "preserve",
            depths: Depth::ALL.iter().map(|d| d.bits()).collect(),
            params: p
                .params
                .iter()
                .map(|q| ParamDef {
                    id: leak(&q.id),
                    name: leak(&q.name),
                    kind: match q.kind.as_str() {
                        "checkbox" => ParamKind::Checkbox { default: q.default.trim() == "1" },
                        "float" => ParamKind::Float {
                            default: q.default.trim().parse().unwrap_or(0.0),
                            min: q.min.unwrap_or(f64::MIN),
                            max: q.max.unwrap_or(f64::MAX),
                        },
                        "int" => ParamKind::Int {
                            default: q.default.trim().parse::<f64>().unwrap_or(0.0) as i64,
                            min: q.min.unwrap_or(i64::MIN as f64) as i64,
                            max: q.max.unwrap_or(i64::MAX as f64) as i64,
                        },
                        _ => ParamKind::Text { default: leak(&q.default) },
                    },
                })
                .collect(),
            layers: p.layers.iter().map(|(id, optional)| LayerDef { id: leak(id), optional: *optional }).collect(),
            hidden,
        })
        .collect()
}

/// Whether `id` is an effect stage.
pub fn is_fx(id: &str) -> bool {
    profile(id).is_some()
}

// ---------------------------------------------------------------------------------------------
// relayout

/// Frame bytes to 'ae32' words (A, R, G, B per pixel, f32 bits).
pub fn frame_to_words(spec: FrameSpec, frame: &[u8]) -> Vec<u32> {
    let n = spec.width as usize * spec.height as usize;
    let mut w = vec![0u32; n * 4];
    match spec.depth {
        Depth::F32 => {
            let rd = |plane: usize, i: usize| -> u32 {
                let o = (plane * n + i) * 4;
                u32::from_le_bytes([frame[o], frame[o + 1], frame[o + 2], frame[o + 3]])
            };
            for i in 0..n {
                w[i * 4] = rd(3, i);
                w[i * 4 + 1] = rd(2, i);
                w[i * 4 + 2] = rd(0, i);
                w[i * 4 + 3] = rd(1, i);
            }
        }
        Depth::U8 => {
            for i in 0..n {
                let p = &frame[i * 4..i * 4 + 4];
                let f = |v: u8| (v as f32 / 255.0f32).to_bits();
                w[i * 4] = f(p[3]);
                w[i * 4 + 1] = f(p[0]);
                w[i * 4 + 2] = f(p[1]);
                w[i * 4 + 3] = f(p[2]);
            }
        }
        Depth::U16 => {
            for i in 0..n {
                let p = &frame[i * 8..i * 8 + 8];
                let f = |o: usize| (u16::from_le_bytes([p[o], p[o + 1]]) as f32 / 65535.0f32).to_bits();
                w[i * 4] = f(6);
                w[i * 4 + 1] = f(0);
                w[i * 4 + 2] = f(2);
                w[i * 4 + 3] = f(4);
            }
        }
    }
    w
}

/// 'ae32' words back to frame bytes.
pub fn words_to_frame(spec: FrameSpec, words: &[u32], frame: &mut [u8]) {
    let n = spec.width as usize * spec.height as usize;
    match spec.depth {
        Depth::F32 => {
            let mut wr = |plane: usize, i: usize, v: u32| {
                let o = (plane * n + i) * 4;
                frame[o..o + 4].copy_from_slice(&v.to_le_bytes());
            };
            for i in 0..n {
                wr(3, i, words[i * 4]);
                wr(2, i, words[i * 4 + 1]);
                wr(0, i, words[i * 4 + 2]);
                wr(1, i, words[i * 4 + 3]);
            }
        }
        Depth::U8 => {
            let q = |bits: u32| -> u8 {
                let x = f32::from_bits(bits);
                if x.is_nan() {
                    0
                } else {
                    (x * 255.0f32).round_ties_even().clamp(0.0, 255.0) as u8
                }
            };
            for i in 0..n {
                frame[i * 4] = q(words[i * 4 + 1]);
                frame[i * 4 + 1] = q(words[i * 4 + 2]);
                frame[i * 4 + 2] = q(words[i * 4 + 3]);
                frame[i * 4 + 3] = q(words[i * 4]);
            }
        }
        Depth::U16 => {
            let q = |bits: u32| -> u16 {
                let x = f32::from_bits(bits);
                if x.is_nan() {
                    0
                } else {
                    (x * 65535.0f32).round_ties_even().clamp(0.0, 65535.0) as u16
                }
            };
            for i in 0..n {
                let mut put = |o: usize, v: u16| frame[i * 8 + o..i * 8 + o + 2].copy_from_slice(&v.to_le_bytes());
                put(0, q(words[i * 4 + 1]));
                put(2, q(words[i * 4 + 2]));
                put(4, q(words[i * 4 + 3]));
                put(6, q(words[i * 4]));
            }
        }
    }
}

fn write_words(path: &Path, words: &[u32]) -> Result<()> {
    let mut bytes = Vec::with_capacity(words.len() * 4);
    for w in words {
        bytes.extend_from_slice(&w.to_le_bytes());
    }
    std::fs::write(path, bytes).map_err(|e| Error::stage_failed(format!("cannot write {}: {}", path.display(), e)))
}

fn read_words(path: &Path) -> Result<Vec<u32>> {
    let bytes = std::fs::read(path).map_err(|e| Error::stage_failed(format!("fx_run wrote no {}: {}", path.display(), e)))?;
    Ok(bytes.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect())
}

// ---------------------------------------------------------------------------------------------
// the run

/// A scratch folder, removed on every exit path.
struct Scratch(PathBuf);

impl Scratch {
    fn new() -> Result<Scratch> {
        static N: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = N.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!("sapphire-fx-{}-{}", std::process::id(), n));
        std::fs::create_dir_all(&dir).map_err(|e| Error::stage_failed(format!("cannot create {}: {}", dir.display(), e)))?;
        Ok(Scratch(dir))
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The `--geom` of the host for a frame of `w` by `h`.
pub fn geom(w: u32, h: u32) -> String {
    let f = |x: f64| format!("{}", x);
    format!("{},{},1,0,{},0,0", f(-0.5), f(w as f64 - 0.5), f((h as f64 - 1.0 - w as f64) / 2.0))
}

/// The placement options for a frame size: the measured size, else the 64x32 entry (INFERRED).
fn placement_for<'a>(p: &'a Profile, w: u32, h: u32) -> &'a [(String, String)] {
    let key = format!("{}x{}", w, h);
    p.placement
        .iter()
        .find(|(s, _)| *s == key)
        .or_else(|| p.placement.iter().find(|(s, _)| s == "64x32"))
        .or_else(|| p.placement.last())
        .map(|(_, o)| o.as_slice())
        .unwrap_or(&[])
}

/// A checked `--param` list as `fx_run` arguments.
fn param_args(p: &Profile, values: &[(String, String)]) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for (id, value) in values {
        let q = match p.params.iter().find(|q| q.id == *id) {
            Some(q) => q,
            None => continue,
        };
        let value = value.trim();
        if q.kind == "text" && q.lanes > 1 {
            let parts: Vec<&str> = value.split(',').collect();
            if parts.len() != q.lanes || parts.iter().any(|t| t.trim().parse::<f32>().is_err()) {
                return Err(Error::unsupported(format!("bad value for `{}`: expects {} numbers separated by commas", q.id, q.lanes)));
            }
        }
        match q.form.as_str() {
            "--set" | "--host-param" | "--cset" => {
                out.push(q.form.clone());
                out.push(format!("{}={}", q.name, value));
            }
            other => {
                out.push(other.to_string());
                out.push(value.to_string());
            }
        }
    }
    Ok(out)
}


/// The derived `do_postdiv` named by fx_run's refusal. The exact line is
/// `POSTDIV-MISMATCH: the owner arm <state> derives ctx = <0|1>, the recorded cell is <0|1>`.
pub fn parse_postdiv(stderr: &str) -> Option<u8> {
    let line = stderr.lines().find(|l| l.starts_with("POSTDIV-MISMATCH: the owner arm "))?;
    let rest = line.split(" derives ctx = ").nth(1)?;
    let (num, tail) = rest.split_once(", the recorded cell is ")?;
    if !tail.trim().chars().all(|c| c == '0' || c == '1') || tail.trim().len() != 1 {
        return None;
    }
    match num {
        "0" => Some(0),
        "1" => Some(1),
        _ => None,
    }
}

/// The constant host curve file for the styles that read one (Transform_and_Blur, Blur_Motion_Curves_No_Transform):
/// the identity transform (rotate 0, shift 0, z_dist 1, centre 50 by 50) at one time sample, in the recorded format.
/// MEASURED: 2 of the 13 in-profile members of each style have a recorded file equal to this text byte for byte.
/// `extra` adds the samples `fx_run` asked for beyond those (a shutter interval that reaches other times); each carries the
/// identity value of its parameter.
pub fn constant_curve(w: u32, h: u32, tick: u64, extra: &[CurveKey]) -> String {
    // frame 0 carries one sample (the measured files); a later frame needs its neighbours (fx_run asks for tick-1 as well)
    let ticks: Vec<i64> = if tick == 0 { vec![0] } else { vec![tick as i64 - 1, tick as i64, tick as i64 + 1] };
    let mut values = String::new();
    for (name, id, v) in [("rotate", 23, 0), ("shift", 24, 0), ("shift", 25, 0), ("z_dist", 22, 65536)] {
        for t in &ticks {
            values.push_str(&format!("value {} {} {} 1 24 0 {}\n", name, id, t, v));
        }
    }
    for k in extra {
        if let Some((name, v)) = identity_sample(k.index) {
            let line = format!("value {} {} {} {} {} 0 {}\n", name, k.index, k.tick, k.step, k.scale, v);
            if !values.contains(&line) {
                values.push_str(&line);
            }
        }
    }
    format!(
        "host effect_ref 1592590336\nhost time_mult 1\nhost time_scale 24\nhost double_flag 0\nhost global 1\nhost selector 0\nhost size_x {w}\nhost size_y {h}\n\
unit shift rec6c 3f800000\nunit shift x3 3f800000\nunit shift d4 3ff0000000000000\nunit shift d5 3ff0000000000000\n\
point center list none\npoint center gid 1\npoint center selector 0\npoint center rtype 1\npoint center size_x {w}\npoint center size_y {h}\n\
point center s11c 1\npoint center s120 1\npoint center s124 1\npoint center s128 1\npoint center off_a 0\npoint center off_b 0\n\
point center scaled 1\npoint center cur_time {t}\npoint center time_step 1\npoint center time_scale 24\npoint center px 3276800\npoint center py 3276800\n\
point center d4 3ff0000000000000\npoint center d5 3ff0000000000000\npoint center rec6c 3f800000\npoint center x3 3f800000\n\
point center prelude_status 0\npoint center prelude_type 6\n\
{values}",
        w = w,
        h = h,
        t = tick,
        values = values
    )
}

/// One host curve sample as `fx_run` keys it: parameter index, time tick, time step and time scale.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CurveKey {
    pub index: i32,
    pub tick: i64,
    pub step: i32,
    pub scale: i32,
}

/// The parameter name and identity value of a curve parameter index of the two curve styles, or `None` for an index
/// the constant identity curve does not carry.
pub fn identity_sample(index: i32) -> Option<(&'static str, i64)> {
    match index {
        23 => Some(("rotate", 0)),
        24 | 25 => Some(("shift", 0)),
        22 => Some(("z_dist", 65536)),
        _ => None,
    }
}

/// The sample named by `fx_run`'s refusal `HOST-CURVE-VALUE-MISSING (PROGRAM: index I, tick T, step S, scale C)`.
pub fn parse_curve_missing(stderr: &str) -> Option<CurveKey> {
    let line = stderr.lines().find(|l| l.contains("HOST-CURVE-VALUE-MISSING ("))?;
    let rest = line.split("HOST-CURVE-VALUE-MISSING (").nth(1)?;
    let inner = rest.split_once(')')?.0;
    let fields = inner.split_once(": ")?.1;
    let mut v = [None::<i64>; 4];
    for (slot, (part, key)) in fields.split(", ").zip(["index", "tick", "step", "scale"]).enumerate() {
        let (k, n) = part.split_once(' ')?;
        if k != key {
            return None;
        }
        v[slot] = Some(n.parse().ok()?);
    }
    Some(CurveKey { index: i32::try_from(v[0]?).ok()?, tick: v[1]?, step: i32::try_from(v[2]?).ok()?, scale: i32::try_from(v[3]?).ok()? })
}

/// How many times one frame may be repeated to learn host curve samples before the stage refuses it.
const CURVE_RERUNS: usize = 64;

/// What a clip learns from fx_run's own refusals and then reuses for every frame.
#[derive(Default)]
struct Learned {
    /// the derived `do_postdiv`
    postdiv: std::cell::Cell<u8>,
    /// host curve samples asked for beyond the frame's own time, as (index, tick minus frame index, step, scale)
    curve: std::cell::RefCell<Vec<(i32, i64, i32, i32)>>,
}

impl Learned {
    fn curve_keys(&self, index: u64) -> Vec<CurveKey> {
        self.curve.borrow().iter().map(|&(i, off, step, scale)| CurveKey { index: i, tick: index as i64 + off, step, scale }).collect()
    }
}


/// What a frame needs beyond its own pixels.
#[derive(Default)]
struct Extra {
    /// `--frame-words` entries (`KEY=path` written by the caller).
    frame_words: Vec<(String, Vec<u32>)>,
    /// The whole clip for the history route (Trails): render k is clip frame k-1 at time k.
    history: Option<Vec<Vec<u32>>>,
}

fn run_fx(p: &Profile, spec: FrameSpec, index: u64, src: &[u32], layers: &[(String, Vec<u32>)], params: &[String], extra: &Extra, learned: &Learned) -> Result<Vec<Vec<u32>>> {
    if let Some(m) = missing() {
        return Err(Error::unsupported(format!("the effect stages need fx_run: {}", m)));
    }
    let fx_run = fx_run_path().expect("checked");
    let scratch = Scratch::new()?;
    let dir = &scratch.0;
    let (w, h) = (spec.width, spec.height);
    let hist = extra.history.as_deref();
    let (src_path, out_path) = match hist {
        // the history route substitutes %R (1 based render) in these two paths
        Some(frames) => {
            for (k, f) in frames.iter().enumerate() {
                write_words(&dir.join(format!("src{}.u32", k + 1)), f)?;
            }
            (dir.join("src%R.u32"), dir.join("out.r%R.u32"))
        }
        None => (dir.join("src.u32"), dir.join("out.u32")),
    };
    let prior_path = dir.join("prior.u32");
    if hist.is_none() {
        write_words(&src_path, src)?;
    }
    write_words(&prior_path, &vec![p.prior_fill; w as usize * h as usize * 4])?;
    let mut args: Vec<String> = vec![
        p.program.clone(),
        "--size".into(),
        format!("{}x{}", w, h),
        "--out".into(),
        dir.join("o").to_string_lossy().into_owned(),
        "--ae32-src".into(),
        format!("{}={}", p.src_name, src_path.display()),
    ];
    for (name, words) in layers {
        let lp = dir.join(format!("layer_{}.u32", name));
        write_words(&lp, words)?;
        args.push("--ae32-src".into());
        args.push(format!("{}={}", name, lp.display()));
    }
    args.extend(["--ae32-prior".to_string(), prior_path.to_string_lossy().into_owned(), "--ae32-out".to_string(), out_path.to_string_lossy().into_owned()]);
    let tick = if hist.is_some() { "%R".to_string() } else { index.to_string() };
    args.extend(["--geom".to_string(), geom(w, h), "--host-frame".to_string(), tick.clone(), "--ctx-time".to_string(), tick]);
    if let Some(frames) = hist {
        let times: Vec<String> = (1..=frames.len()).map(|k| k.to_string()).collect();
        args.extend(["--host-times".to_string(), times.join(",")]);
    }
    for (k, v) in p.host.iter().chain(placement_for(p, w, h).iter()).chain(p.fixed.iter()) {
        if v == "<path>" {
            continue;
        }
        args.push(k.clone());
        // `W` in a size-dependent value stands for the frame width
        args.push(if k == "--palpha-cells" { v.replacen('W', &w.to_string(), 1) } else { v.clone() });
    }
    args.extend(params.iter().cloned());
    // an unbound normalize_scale parameter takes its registered default inside fx_run
    args.push("--ns-default-unbound".into());
    let curve_path = dir.join("curve.txt");
    let write_curve = || std::fs::write(&curve_path, constant_curve(w, h, index, &learned.curve_keys(index))).map_err(|e| Error::stage_failed(format!("cannot write {}: {}", curve_path.display(), e)));
    let own_curve = p.params.iter().any(|q| q.id == "host_curve_values") && !args.iter().any(|a| a == "--host-curve-values");
    if own_curve {
        write_curve()?;
        args.push("--host-curve-values".into());
        args.push(curve_path.to_string_lossy().into_owned());
    }
    args.push("--do-postdiv".into());
    args.push(learned.postdiv.get().to_string());
    for (i, (key, words)) in extra.frame_words.iter().enumerate() {
        let fp = dir.join(format!("fw{}.u32", i));
        write_words(&fp, words)?;
        args.push("--frame-words".into());
        args.push(format!("{}={}", key, fp.display()));
    }
    let exec = |args: &[String]| {
        Command::new(&fx_run)
            .args(args)
            .current_dir(dir)
            .output()
            .map_err(|e| Error::tool_missing(format!("cannot run {}: {}", fx_run.display(), e)))
    };
    let mut ran = exec(&args)?;
    // The owner-arm `do_postdiv` is a required cross-check on the 'ae32' route that fx_run only reveals through
    // its refusal; it is learned once per clip and kept. The samples the route reads from the stage's own constant curve
    // are learned the same way: each `HOST-CURVE-VALUE-MISSING` (in a plan error or a refusal) names one sample, which is added
    // with its identity value.
    for _ in 0..8 + CURVE_RERUNS {
        let text = String::from_utf8_lossy(&ran.stderr).into_owned();
        if own_curve && matches!(ran.status.code(), Some(1) | Some(2)) && text.contains("HOST-CURVE-VALUE-MISSING (") {
            let line = text.lines().find(|l| l.contains("HOST-CURVE-VALUE-MISSING")).unwrap_or("").to_string();
            let key = match parse_curve_missing(&text) {
                Some(k) if identity_sample(k.index).is_some() => k,
                _ => return Err(Error::unsupported(format!("frame {}: HOST-CURVE-SAMPLE-UNSERVED: the constant identity curve cannot serve this sample; pass host_curve_values: {}", index, line)).with_tail(tail_lines(&text, 3))),
            };
            let entry = (key.index, key.tick - index as i64, key.step, key.scale);
            if learned.curve.borrow().contains(&entry) {
                return Err(Error::unsupported(format!("frame {}: HOST-CURVE-SAMPLE-UNSERVED: the sample is in the curve and still refused: {}", index, line)).with_tail(tail_lines(&text, 3)));
            }
            learned.curve.borrow_mut().push(entry);
            write_curve()?;
            ran = exec(&args)?;
            continue;
        }
        if ran.status.code() != Some(2) {
            break;
        }
        if text.contains("POSTDIV-MISMATCH") {
            match parse_postdiv(&text) {
                Some(n) => {
                    learned.postdiv.set(n);
                    { let at = args.iter().rposition(|a| a == "--do-postdiv").expect("do-postdiv"); args[at + 1] = n.to_string(); }
                    ran = exec(&args)?;
                    continue;
                }
                None => {
                    let line = text.lines().find(|l| l.contains("POSTDIV-MISMATCH")).unwrap_or("").to_string();
                    return Err(Error::stage_failed(format!("frame {}: cannot read the derived do_postdiv from fx_run: {}", index, line)).with_tail(tail_lines(&text, 3)));
                }
            }
        }
        break;
    }
    let stderr = String::from_utf8_lossy(&ran.stderr).into_owned();
    let tail = tail_lines(&stderr, 3);
    let first = stderr.lines().rev().find(|l| !l.trim().is_empty()).unwrap_or("").to_string();
    match ran.status.code() {
        Some(0) => {}
        Some(1) if stderr.contains("HOST-CURVE-VALUE-MISSING") => {
            return Err(Error::unsupported(format!("frame {}: HOST-CURVE-SAMPLE-UNSERVED: the host curve does not hold a sample the route reads: {}", index, first)).with_tail(tail))
        }
        Some(2) => return Err(Error::unsupported(format!("frame {}: fx_run refused: {}", index, first)).with_tail(tail)),
        code => return Err(Error::stage_failed(format!("frame {}: fx_run failed with {:?}", index, code)).with_tail(tail)),
    }
    let count = hist.map(|f| f.len()).unwrap_or(1);
    let mut outs = Vec::with_capacity(count);
    for k in 1..=count {
        let path = if hist.is_some() { dir.join(format!("out.r{}.u32", k)) } else { out_path.clone() };
        let words = read_words(&path)?;
        if words.len() != w as usize * h as usize * 4 {
            return Err(Error::stage_failed(format!("frame {}: fx_run wrote {} words for a {}x{} frame", index + k as u64 - 1, words.len(), w, h)).with_tail(tail));
        }
        outs.push(words);
    }
    Ok(outs)
}

/// A layer file: one frame (held) or one frame per input frame.
struct Layer {
    id: String,
    frames: Vec<Vec<u32>>,
}

impl Layer {
    fn load(id: &str, path: &str, spec: FrameSpec) -> Result<Layer> {
        let bytes = std::fs::read(path).map_err(|e| Error::bad_input(format!("cannot open layer {} at {}: {}", id, path, e)))?;
        let fb = spec.frame_bytes() as usize;
        if bytes.is_empty() || bytes.len() % fb != 0 {
            return Err(Error::bad_input(format!("layer {} holds {} bytes, which is not a whole number of {}x{} frames", id, bytes.len(), spec.width, spec.height)));
        }
        Ok(Layer { id: id.to_string(), frames: bytes.chunks(fb).map(|c| frame_to_words(spec, c)).collect() })
    }

    fn at(&self, index: u64) -> Result<(String, Vec<u32>)> {
        let n = self.frames.len();
        let k = if n == 1 { 0 } else { index as usize };
        match self.frames.get(k) {
            Some(f) => Ok((self.id.clone(), f.clone())),
            None => Err(Error::bad_input(format!("layer {} holds {} frames but the input has more", self.id, n))),
        }
    }
}

fn read_frame(input: &mut dyn Read, buf: &mut [u8], index: u64) -> Result<bool> {
    let mut got = 0usize;
    while got < buf.len() {
        match input.read(&mut buf[got..]) {
            Ok(0) => break,
            Ok(n) => got += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
            Err(e) => return Err(Error::bad_input(format!("frame {}: cannot read the input: {}", index, e))),
        }
    }
    if got == 0 {
        return Ok(false);
    }
    if got < buf.len() {
        return Err(Error::new(Code::PartialFrame, format!("frame {}: input ends with a partial frame ({} of {} bytes)", index, got, buf.len())));
    }
    Ok(true)
}

fn write_out(output: &mut dyn Write, bytes: &[u8], index: u64) -> Result<()> {
    output.write_all(bytes).map_err(|e| {
        let code = if e.kind() == std::io::ErrorKind::BrokenPipe { Code::BrokenPipe } else { Code::BadInput };
        Error::new(code, format!("frame {}: cannot write the output: {}", index, e))
    })
}

/// The shift (in frames) of Time_Warp's three channels: the given parameter or the registered default.
fn tw_shifts(p: &Profile, values: &[(String, String)]) -> [i32; 3] {
    let mut s = [0i32; 3];
    for (i, id) in ["shift_frames_red", "shift_frames_green", "shift_frames_blue"].iter().enumerate() {
        let given = values.iter().find(|(k, _)| k == id).map(|(_, v)| v.trim().to_string());
        let dflt = p.params.iter().find(|q| q.id == *id).map(|q| q.default.clone()).unwrap_or_default();
        s[i] = given.unwrap_or(dflt).parse::<f64>().unwrap_or(0.0) as i32;
    }
    s
}

/// The command line that runs the stage `id` as a process in a pipeline.
pub fn command_line(id: &str, spec: FrameSpec, rate: &crate::rate::Rate, params: &[String]) -> Vec<String> {
    let exe = crate::ffstage::handoff_exe();
    let mut argv = vec![
        exe,
        "stage".into(),
        "apply".into(),
        id.into(),
        "--width".into(),
        spec.width.to_string(),
        "--height".into(),
        spec.height.to_string(),
        "--depth".into(),
        spec.depth.bits().to_string(),
        "--fps".into(),
        rate.text(),
    ];
    for p in params {
        argv.extend(["--param".to_string(), p.clone()]);
    }
    argv
}

/// Run the effect stage over `input`.
pub fn apply(
    id: &str,
    spec: FrameSpec,
    values: &[(String, String)],
    layers: &[String],
    frames: Option<u64>,
    input: &mut dyn Read,
    output: &mut dyn Write,
) -> Result<()> {
    let p = profile(id).ok_or_else(|| Error::unsupported(format!("unknown stage `{}`", id)))?;
    if let Some(m) = missing() {
        return Err(Error::unsupported(format!("the effect stages need fx_run: {}", m)));
    }
    if spec.width < 2 {
        return Err(Error::unsupported(format!("{} needs a width of 2 or more (the host grid needs two columns)", p.id)));
    }
    let args = param_args(p, values)?;
    let mut ls: Vec<Layer> = Vec::new();
    for raw in layers {
        let (lid, path) = raw.split_once('=').ok_or_else(|| Error::usage(format!("`--layer` expects ID=PATH, got `{}`", raw)))?;
        if !p.layers.iter().any(|(n, _)| n == lid) {
            return Err(Error::unsupported(format!("{} has no layer `{}`", p.id, lid)));
        }
        ls.push(Layer::load(lid, path, spec)?);
    }
    let fb = spec.frame_bytes() as usize;
    let mut buf = vec![0u8; fb];
    let mut out = vec![0u8; fb];
    let mut count = 0u64;
    let learned = Learned::default();
    let layer_words = |index: u64| -> Result<Vec<(String, Vec<u32>)>> { ls.iter().map(|l| l.at(index)).collect() };
    if p.id == "time_warp_rgb_autogen" {
        let shifts = tw_shifts(p, values);
        let maxpos = shifts.iter().copied().max().unwrap_or(0).max(0) as u64;
        let maxneg = (-shifts.iter().copied().min().unwrap_or(0)).max(0) as u64;
        // window of input frames: `base` is the index of window[0]
        let mut window: VecDeque<Vec<u32>> = VecDeque::new();
        let mut base = 0u64;
        let mut ended = false;
        let mut f = 0u64;
        loop {
            while !ended && base + (window.len() as u64) <= f + maxpos {
                let idx = base + window.len() as u64;
                if read_frame(input, &mut buf, idx)? {
                    window.push_back(frame_to_words(spec, &buf));
                } else {
                    ended = true;
                }
            }
            let last = base + window.len() as u64;
            if f >= last {
                break;
            }
            let get = |k: u64| -> &Vec<u32> {
                let k = k.min(last - 1);
                &window[(k - base) as usize]
            };
            let mut extra = Extra::default();
            for s in shifts {
                let k = (f as i64 + s as i64).max(0) as u64;
                extra.frame_words.push((format!("{},{}", k, k), get(k).clone()));
            }
            let words = run_fx(p, spec, f, get(f), &layer_words(f)?, &args, &extra, &learned)?.remove(0);
            words_to_frame(spec, &words, &mut out);
            write_out(output, &out, f)?;
            count += 1;
            f += 1;
            while base + maxneg < f && window.len() > 1 {
                window.pop_front();
                base += 1;
            }
        }
    } else if p.spool {
        // Trails: buffer the clip, run the measured history route once (render k is frame k-1 at time k)
        let mut clip: Vec<Vec<u32>> = Vec::new();
        while read_frame(input, &mut buf, clip.len() as u64)? {
            clip.push(frame_to_words(spec, &buf));
        }
        if !clip.is_empty() {
            let extra = Extra { frame_words: Vec::new(), history: Some(clip) };
            let outs = run_fx(p, spec, 0, &[], &[], &args, &extra, &learned)?;
            for (i, words) in outs.iter().enumerate() {
                words_to_frame(spec, words, &mut out);
                write_out(output, &out, i as u64)?;
                count += 1;
            }
        }
    } else {
        loop {
            if !read_frame(input, &mut buf, count)? {
                break;
            }
            let src = frame_to_words(spec, &buf);
            let words = run_fx(p, spec, count, &src, &layer_words(count)?, &args, &Extra::default(), &learned)?.remove(0);
            words_to_frame(spec, &words, &mut out);
            write_out(output, &out, count)?;
            count += 1;
        }
    }
    output.flush().map_err(|e| Error::new(Code::BadInput, format!("cannot write the output: {}", e)))?;
    if let Some(want) = frames {
        if want != count {
            return Err(Error::bad_input(format!("the input holds {} frames, but --frames says {}", count, want)));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tables_parse_for_thirteen_stages() {
        let ps = profiles();
        assert_eq!(ps.len(), 13);
        let hsb = profile("hue_sat_bright_autogen").expect("hsb");
        assert_eq!(hsb.family, "S_HueSatBright");
        assert_eq!(hsb.kind, "frame");
        assert!(hsb.host.iter().any(|(k, v)| k == "--src-planes" && v == "3"));
        assert!(hsb.params.iter().any(|q| q.id == "hue_shift"));
        assert_eq!(profile("trails_autogen").unwrap().spool, true);
        assert_eq!(profile("time_warp_rgb_autogen").unwrap().kind, "temporal");
        assert_eq!(profile("drop_shadow").unwrap().layers.len(), 2);
    }

    #[test]
    fn postdiv_probe_pattern() {
        let line = "POSTDIV-MISMATCH: the owner arm OwnerArm { owner_arm_nonzero: false, owner_arm_bit2: false, mode_byte_exit: false, opacity_bits: Some(1065353216), output_opacity_bits: None } derives ctx = 1, the recorded cell is 0";
        assert_eq!(parse_postdiv(line), Some(1));
        assert_eq!(parse_postdiv(&format!("noise\n{}\n", line.replace("= 1,", "= 0,"))), Some(0));
        assert_eq!(parse_postdiv(&line.replace("= 1,", "= 7,")), None);
        assert_eq!(parse_postdiv("POSTDIV-MISMATCH: something else"), None);
        assert_eq!(parse_postdiv(&line.replace("cell is 0", "cell is 12")), None);
    }

    #[test]
    fn described_parameters_fill_the_profile() {
        let text = r#"{"Hue_Sat_Bright_Autogen":[{"name":"hue_shift","type":"scalar","lanes":1,"min":-1,"max":1,"default":"0"},{"name":"scale_colors","type":"color","lanes":3,"min":-1000000,"max":1000000,"default":"1 1 1"}]}"#;
        let d = parse_described(text).expect("parses");
        let mut ps = parse_tables(STAGES_TSV, PARAMS_TSV);
        enrich(&mut ps, &d);
        let hsb = ps.iter().find(|p| p.id == "hue_sat_bright_autogen").unwrap();
        let hue = hsb.params.iter().find(|q| q.id == "hue_shift").unwrap();
        assert_eq!((hue.kind.as_str(), hue.min, hue.max, hue.default.as_str()), ("float", Some(-1.0), Some(1.0), "0"));
        let col = hsb.params.iter().find(|q| q.id == "scale_colors").unwrap();
        assert_eq!((col.kind.as_str(), col.lanes, col.default.as_str()), ("text", 3, "1,1,1"));
        let w = parse_described(r#"{"Diffuse_Autogen":[{"name":"wrap","type":"popup","lanes":2,"min":0,"max":2,"default":"2 2"}]}"#).unwrap();
        enrich(&mut ps, &w);
        let diffuse = ps.iter().find(|p| p.id == "diffuse_autogen").unwrap();
        let wrap = diffuse.params.iter().find(|q| q.id == "wrap").unwrap();
        assert_eq!((wrap.kind.as_str(), wrap.lanes), ("text", 2));
        assert!(parse_described("not json").is_none());
    }

    #[test]
    fn constant_curve_has_the_recorded_shape() {
        let t = constant_curve(64, 32, 0, &[]);
        assert!(t.starts_with("host effect_ref 1592590336\n"));
        assert!(t.contains("host size_x 64\nhost size_y 32\n"));
        assert!(t.ends_with("value z_dist 22 0 1 24 0 65536\n"));
    }

    #[test]
    fn curve_refusal_names_one_sample() {
        let e = "bad plan: compile: HOST-CURVE-VALUE-MISSING (Transform_and_Blur_Autogen: index 23, tick 1, step 1, scale 24)";
        let k = parse_curve_missing(e).unwrap();
        assert_eq!(k, CurveKey { index: 23, tick: 1, step: 1, scale: 24 });
        assert!(parse_curve_missing("HOST-CURVE-VALUE-MISSING (P: time argument not a constant)").is_none());
        assert!(identity_sample(99).is_none());
        let t = constant_curve(64, 32, 0, &[k, CurveKey { index: 22, tick: -1, step: 1, scale: 24 }]);
        assert!(t.ends_with("value z_dist 22 0 1 24 0 65536\nvalue rotate 23 1 1 24 0 0\nvalue z_dist 22 -1 1 24 0 65536\n"));
        assert_eq!(constant_curve(64, 32, 0, &[CurveKey { index: 23, tick: 0, step: 1, scale: 24 }]), constant_curve(64, 32, 0, &[]));
    }

    #[test]
    fn geom_formula_matches_measured_sizes() {
        assert_eq!(geom(64, 32), "-0.5,63.5,1,0,-16.5,0,0");
        assert_eq!(geom(50, 7), "-0.5,49.5,1,0,-22,0,0");
        assert_eq!(geom(49, 9), "-0.5,48.5,1,0,-20.5,0,0");
    }

    fn pattern(depth: Depth, spec: FrameSpec) -> Vec<u8> {
        let n = spec.frame_bytes() as usize;
        match depth {
            Depth::F32 => (0..n / 4).flat_map(|i| (i as f32 / 17.0 - 0.3).to_le_bytes()).collect(),
            _ => (0..n).map(|i| (i * 37 % 256) as u8).collect(),
        }
    }

    #[test]
    fn depth32_relayout_is_a_byte_permutation() {
        let spec = FrameSpec { width: 5, height: 3, depth: Depth::F32 };
        let frame = pattern(Depth::F32, spec);
        let words = frame_to_words(spec, &frame);
        // first pixel: A, R, G, B words come from planes 3, 2, 0, 1
        let n = 15;
        let rd = |plane: usize, i: usize| u32::from_le_bytes(frame[(plane * n + i) * 4..(plane * n + i) * 4 + 4].try_into().unwrap());
        assert_eq!(words[0], rd(3, 0));
        assert_eq!(words[1], rd(2, 0));
        assert_eq!(words[2], rd(0, 0));
        assert_eq!(words[3], rd(1, 0));
        let mut back = vec![0u8; frame.len()];
        words_to_frame(spec, &words, &mut back);
        assert_eq!(back, frame);
    }

    #[test]
    fn depth8_and_16_round_trip() {
        for depth in [Depth::U8, Depth::U16] {
            let spec = FrameSpec { width: 7, height: 3, depth };
            let frame = pattern(depth, spec);
            let words = frame_to_words(spec, &frame);
            let mut back = vec![0u8; frame.len()];
            words_to_frame(spec, &words, &mut back);
            assert_eq!(back, frame, "depth {}", depth.bits());
        }
    }

    #[test]
    fn placement_falls_back_to_64x32() {
        let p = profile("blur_motion_curves_no_transform_autogen").unwrap();
        let a = placement_for(p, 64, 32).to_vec();
        let b = placement_for(p, 20, 20).to_vec();
        assert_eq!(a, b);
        assert_ne!(placement_for(p, 50, 7), a.as_slice());
    }

    #[test]
    fn layer_lengths() {
        let spec = FrameSpec { width: 2, height: 1, depth: Depth::U8 };
        let dir = std::env::temp_dir().join(format!("sapphire-fx-layer-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let held = dir.join("held.raw");
        std::fs::write(&held, vec![9u8; 8]).unwrap();
        let two = dir.join("two.raw");
        std::fs::write(&two, vec![9u8; 16]).unwrap();
        let bad = dir.join("bad.raw");
        std::fs::write(&bad, vec![9u8; 5]).unwrap();
        let l = Layer::load("Lens", held.to_str().unwrap(), spec).unwrap();
        assert!(l.at(7).is_ok());
        let l = Layer::load("Lens", two.to_str().unwrap(), spec).unwrap();
        assert!(l.at(1).is_ok());
        assert_eq!(l.at(2).unwrap_err().exit_code(), 4);
        assert_eq!(Layer::load("Lens", bad.to_str().unwrap(), spec).err().unwrap().exit_code(), 4);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
