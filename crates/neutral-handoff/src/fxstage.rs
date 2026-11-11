//! The effects stage provider: one frames/2 stage per registered style of the landed effects.
//!
//! Every stage renders in process through the `fx_exec::fxrun` library on the recorded host route (the 'ae32'
//! route that the stored predictions used), the same code the `fx_run` executable runs, so the proven
//! route carries the parity and this module touches no effect crate. The data is in two tables:
//!
//! - `data/fx_stages.tsv`: per style the stage id, program, family, kind, spool, layers, the name of
//!   the source input, the fixed options, the canonical host tuple, the buffer placement per
//!   measured size, the notes and `host_variants` (`-`, or entries `PARAM_ID=VALUE:--opt v|--opt v` that replace or add host
//!   options when the frame's value of that one offered parameter is VALUE).
//! - `data/fx_params.tsv`: per offered parameter its id, the `fx_run` option form, type, lanes,
//!   registered default and range.
//!
//! The stage holds no plugin table. The type, lanes, range and registered default of a parameter are
//! asked from `fx_run --describe-params`, which prints them from the native definitions, and an
//! unbound normalize_scale parameter is left to `fx_run --ns-default-unbound`.
//!
//! Per frame the stage hands the input to the runner as 'ae32' words (A, R, G, B words per pixel) in memory and takes the
//! `--ae32-out` words back in memory: no scratch files and no debug side files are written, and no process is started. At
//! depth 32 the relayout is a pure byte permutation (frames/2 `gbrapf32le` is planar G, B, R, A, 'ae32' is A, R, G, B per pixel,
//! both f32 and straight alpha). Depths 8 and 16 convert through f32 (`v/255`, `v/65535`, back with round half to even and a clamp)
//! and are outside the parity claim. The Trails history route chains its renders through a scratch directory.
//!
//! One runner thread (with the stack the effect code needs) serves a clip. `--device` picks the backend that thread installs
//! (`fx_exec::fxrun::select_and_install`).

use std::io::{Read, Write};
use std::path::PathBuf;
use fx_exec::fxrun::{Runner, StageIo};
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
    /// The type `fx_run --describe-params` prints (scalar, boolean, popup, int, color, vector or angle), or empty.
    pub ptype: String,
    /// The registered default, as text.
    pub default: String,
    /// For a `--cset` parameter: the host answer the host registers as its default (empty for any other parameter).
    pub answer_default: String,
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
    /// The host options that follow the value of one offered parameter (empty for `-`).
    pub variants: Vec<HostVariant>,
    /// Why the `host_variants` column does not parse, if it does not.
    pub variants_error: Option<String>,
}

/// One host arrangement selected by the value of an offered parameter.
#[derive(Clone, Debug, PartialEq)]
pub struct HostVariant {
    /// The selecting parameter id.
    pub param: String,
    /// The value that selects this entry.
    pub value: String,
    /// The options that replace or add to the host column.
    pub opts: Vec<(String, String)>,
}

/// Parse the `host_variants` column: `-`, or `PARAM_ID=VALUE:--opt v|--opt v` entries separated by `;`.
pub fn parse_variants(text: &str) -> std::result::Result<Vec<HostVariant>, String> {
    let text = text.trim();
    if text == "-" || text.is_empty() {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    for entry in text.split(';') {
        let (key, opts) = entry.split_once(':').ok_or_else(|| format!("HOST-VARIANT-MALFORMED: `{}` has no `:` after PARAM_ID=VALUE", entry))?;
        let (param, value) = key.split_once('=').ok_or_else(|| format!("HOST-VARIANT-MALFORMED: `{}` is not PARAM_ID=VALUE", key))?;
        if param.is_empty() || value.is_empty() {
            return Err(format!("HOST-VARIANT-MALFORMED: `{}` has an empty PARAM_ID or VALUE", key));
        }
        let opts = pairs(opts);
        if opts.is_empty() || opts.iter().any(|(k, _)| !k.starts_with("--")) {
            return Err(format!("HOST-VARIANT-MALFORMED: `{}` holds no options, or an item that is not `--opt value`", key));
        }
        out.push(HostVariant { param: param.to_string(), value: value.to_string(), opts });
    }
    Ok(out)
}

/// Options an entry never carries: derived by the run (`--do-postdiv`, `--owner-opacity`, `--size`, `--out`, `--ae32-src`,
/// `--ae32-prior`, `--ae32-out`, `--geom`, `--host-frame`, `--ctx-time`, `--host-times`, `--host-value`, `--ns-default-unbound`,
/// `--frame-words`), given by a parameter (`--cset`, `--set`, `--host-param`, `--host-curve-values`) or fixed per style
/// (`--prior-fill`); an option of the row's fixed column is refused as well.
const VARIANT_FORBIDDEN: [&str; 19] = [
    "--do-postdiv", "--owner-opacity", "--cset", "--set", "--host-param", "--host-curve-values", "--prior-fill", "--size", "--out", "--ae32-src",
    "--ae32-prior", "--ae32-out", "--geom", "--host-frame", "--ctx-time", "--host-times", "--host-value", "--ns-default-unbound", "--frame-words",
];

fn same_number(a: &str, b: &str) -> bool {
    match (a.trim().parse::<f64>(), b.trim().parse::<f64>()) {
        (Ok(x), Ok(y)) => x == y,
        _ => a.trim() == b.trim(),
    }
}

/// Check a profile's variants against its parameters, placement and defaults. Refusals are named and exit 3.
pub fn check_variants(p: &Profile) -> Result<()> {
    let bad = |m: String| Err(Error::unsupported(format!("{}: {}", p.id, m)));
    if let Some(e) = &p.variants_error {
        return bad(e.clone());
    }
    let mut selector: Option<&str> = None;
    for (i, v) in p.variants.iter().enumerate() {
        match selector {
            None => selector = Some(&v.param),
            Some(s) if s != v.param => return bad(format!("HOST-VARIANT-PARAMS: entries select on both `{}` and `{}`; one selecting parameter per row", s, v.param)),
            _ => {}
        }
        let q = match p.params.iter().find(|q| q.id == v.param) {
            Some(q) => q,
            None => return bad(format!("HOST-VARIANT-PARAM-UNKNOWN: `{}` is not an offered parameter", v.param)),
        };
        if p.variants[..i].iter().any(|u| same_number(&u.value, &v.value)) {
            return bad(format!("HOST-VARIANT-DUPLICATE: value `{}` of `{}` has two entries", v.value, v.param));
        }
        let dflt = if q.form == "--cset" { &q.answer_default } else { &q.default };
        if !dflt.is_empty() && same_number(dflt, &v.value) {
            return bad(format!("HOST-VARIANT-AT-DEFAULT: value `{}` of `{}` is the registered default; the host column serves it", v.value, v.param));
        }
        for (k, val) in &v.opts {
            if val.is_empty() {
                return bad(format!("HOST-VARIANT-MALFORMED: option `{}` of `{}={}` has no value", k, v.param, v.value));
            }
            if VARIANT_FORBIDDEN.contains(&k.as_str()) || p.fixed.iter().any(|(n, _)| n == k) {
                return bad(format!("HOST-VARIANT-OPTION: `{}` is derived, given by a parameter or fixed, never part of an entry", k));
            }
            if p.placement.iter().any(|(_, o)| o.iter().any(|(n, _)| n == k)) {
                return bad(format!("HOST-VARIANT-PLACEMENT: `{}` is a placement option, which stays per size", k));
            }
        }
    }
    Ok(())
}

/// The host options of one frame: the host column, with the options of the entry the frame's value of the selecting parameter selects
/// replacing the option of the same name or added after it. A value with no entry, or an unbound parameter, gives the host column.
pub fn host_at(p: &Profile, values: &[(String, String)]) -> Vec<(String, String)> {
    let mut host = p.host.clone();
    let chosen = p.variants.iter().find(|v| values.iter().rev().find(|(id, _)| *id == v.param).map_or(false, |(_, x)| same_number(x, &v.value)));
    if let Some(v) = chosen {
        for (k, val) in &v.opts {
            if let Some(at) = host.iter().position(|(n, _)| n == k) {
                host[at].1 = val.clone();
                let mut seen = false;
                host.retain(|(n, _)| {
                    if n != k {
                        return true;
                    }
                    let keep = !seen;
                    seen = true;
                    keep
                });
            } else {
                host.push((k.clone(), val.clone()));
            }
        }
    }
    host
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
            variants: Vec::new(),
            variants_error: None,
        });
        let last = out.last_mut().expect("pushed");
        match parse_variants(c.get(12).copied().unwrap_or("-")) {
            Ok(v) => last.variants = v,
            Err(e) => last.variants_error = Some(e),
        }
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
                ptype: String::new(),
                default: String::new(),
                answer_default: String::new(),
                min: if c[2] == "--cset" { Some(1.0) } else { None },
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
    let names: Vec<&str> = ps.iter().map(|p| p.program.as_str()).collect();
    parse_described(&fx_exec::fxrun::describe_params_json(&names))
}

/// Fill the type, lanes, range and default of each parameter from the described definitions.
pub fn enrich(ps: &mut [Profile], described: &[(String, Vec<Described>)]) {
    for p in ps.iter_mut() {
        let list = match described.iter().find(|(n, _)| *n == p.program) {
            Some((_, l)) => l,
            None => continue,
        };
        for q in p.params.iter_mut() {
            if q.form == "--cset" {
                q.answer_default = fx_exec::fxrun::registered_answer_default(&p.program, &q.name).map(|v| v.to_string()).unwrap_or_default();
                q.max = fx_exec::fxrun::registered_answer_count(&p.program, &q.name).map(|n| n as f64);
            }
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
                q.ptype = d.ptype.clone();
                q.default = d.default.split_whitespace().collect::<Vec<_>>().join(",");
                if d.ptype == "popup" && d.lanes == 1 {
                    // an unbound popup renders its registered default, which the manifest offers as its default
                    if let Some(x) = fx_exec::fxrun::popup_manifest_default(&p.program, &q.name) {
                        q.default = x;
                    }
                }
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

/// What is missing for the effect stages to run, as one line, or `None`. The effect runner is linked in, so nothing is.
pub fn missing() -> Option<String> {
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
                            default: if q.form == "--cset" { q.answer_default.trim().parse::<i64>().unwrap_or(0) } else { q.default.trim().parse::<f64>().unwrap_or(0.0) as i64 },
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
        // fx_run reads numeric lanes only: a checkbox word becomes 1 or 0
        let value = match (q.kind.as_str(), crate::stage::parse_bool(value)) {
            ("checkbox", Some(b)) => if b { "1" } else { "0" },
            _ => value,
        };
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
        if let Some((name, v)) = identity_value(k.index) {
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
fn identity_value(index: i32) -> Option<(&'static str, i64)> {
    match index {
        23 => Some(("rotate", 0)),
        24 | 25 => Some(("shift", 0)),
        22 => Some(("z_dist", 65536)),
        6 => Some(("wipe_amt", 0)),
        _ => None,
    }
}

/// The parameter name and identity value of a curve parameter index that the stage's own program carries: index 6 (wipe_amt) for
/// SwishPan, indices 22 to 25 for the two curve styles. `None` for any other program or index.
pub fn identity_sample(program: &str, index: i32) -> Option<(&'static str, i64)> {
    let curve_style = matches!(program, "Transform_and_Blur_Autogen" | "Blur_Motion_Curves_No_Transform_Autogen");
    match index {
        6 if program == "SwishPan" => identity_value(index),
        22..=25 if curve_style => identity_value(index),
        _ => None,
    }
}

/// The parameter the stage row offers for a curve index, when the identity curve would stand in for a value the user can set.
fn offered_sample<'a>(p: &Profile, index: i32) -> Option<&'a str> {
    identity_sample(&p.program, index).map(|x| x.0).filter(|n| p.params.iter().any(|q| q.id == *n))
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

/// The default-xy writer count named by `XY-BASE-LIST-MISMATCH (the placement model has N default-xy writers, --xy-base-list gives M)`.
pub fn parse_xy_writers(stderr: &str) -> Option<usize> {
    let line = stderr.lines().find(|l| l.contains("XY-BASE-LIST-MISMATCH (the placement model has "))?;
    let rest = line.split("XY-BASE-LIST-MISMATCH (the placement model has ").nth(1)?;
    let (n, tail) = rest.split_once(' ')?;
    if !tail.starts_with("default-xy writers, --xy-base-list gives ") {
        return None;
    }
    n.parse().ok().filter(|&n: &usize| (1..=64).contains(&n))
}

/// Give `--xy-base-list` `n` entries. The measured list's entries must share one base phase, which every writer then takes
/// (the measured lists hold one phase per size); anything else is refused by name.
pub fn fit_xy_list(args: &mut [String], n: usize, index: u64) -> Result<()> {
    let at = match args.iter().position(|a| a == "--xy-base-list") {
        Some(at) if at + 1 < args.len() => at + 1,
        _ => return Err(Error::unsupported(format!("frame {}: XY-BASE-LIST-UNSERVED: fx_run asks for a writer list the stage does not pass", index))),
    };
    let entries: Vec<&str> = args[at].split(',').collect();
    if entries.iter().any(|e| e.ends_with("r1")) || entries.windows(2).any(|w| w[0] != w[1]) {
        return Err(Error::unsupported(format!("frame {}: XY-BASE-LIST-UNSERVED: the measured list {} has no single base phase to give {} writers", index, args[at], n)));
    }
    args[at] = vec![entries[0]; n].join(",");
    Ok(())
}

/// What a clip learns from fx_run's own refusals and then reuses for every frame.
#[derive(Default)]
struct Learned {
    /// the derived `do_postdiv`
    postdiv: std::cell::Cell<u8>,
    /// the default-xy writer count of the placement model, when fx_run named one different from the measured `--xy-base-list`
    xy_writers: std::cell::Cell<Option<usize>>,
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
    /// `--host-value NAME=VALUE` entries (get_percent_time, get_total_frames), values as f32 bits.
    host_values: Vec<(String, f32)>,
    /// The clip length in frames: the `N` field of a `--tw-route` value stands for it.
    clip_len: Option<u64>,
}

/// A fixed stage value with its size placeholders filled in: `W` of `--palpha-cells` is the frame width, and the first `N` of
/// `--tw-route` is the clip length (left as it is when the length is unknown).
fn fill_fixed(key: &str, value: &str, width: u64, clip_len: Option<u64>) -> String {
    match (key, clip_len) {
        ("--palpha-cells", _) => value.replacen('W', &width.to_string(), 1),
        ("--tw-route", Some(n)) => value.replacen('N', &n.to_string(), 1),
        _ => value.to_string(),
    }
}

/// The in-memory files of one run: names are the virtual paths on the command line.
/// The lines of a run's log that start with `device: `, one to a line.
pub(crate) fn device_lines_of(log: &str) -> String {
    log.lines().filter(|l| l.starts_with("device: ")).map(|l| format!("{}\n", l)).collect()
}

/// Writes the collected `device: ` lines to standard error when dropped.
struct ForwardDeviceLines<'a>(&'a std::cell::RefCell<String>);

impl Drop for ForwardDeviceLines<'_> {
    fn drop(&mut self) {
        let text = self.0.borrow();
        if !text.is_empty() {
            use std::io::Write;
            let _ = std::io::stderr().write_all(text.as_bytes());
        }
    }
}

#[cfg(test)]
thread_local! {
    /// The arguments and the named word files of the last `run_fx` call on this thread (tests read it).
    static LAST_RUN: std::cell::RefCell<Option<(Vec<String>, Vec<(String, std::sync::Arc<Vec<u32>>)>)>> = std::cell::RefCell::new(None);
}

struct RunFiles {
    words: Vec<(String, std::sync::Arc<Vec<u32>>)>,
    curve: Option<(String, String)>,
}

fn run_fx(
    runner: &Runner,
    p: &Profile,
    spec: FrameSpec,
    index: u64,
    src: Vec<u32>,
    layers: Vec<(String, Vec<u32>)>,
    params: &[String],
    host: &[(String, String)],
    extra: Extra,
    learned: &Learned,
) -> Result<Vec<Vec<u32>>> {
    let (w, h) = (spec.width, spec.height);
    let hist = extra.history.is_some();
    // The history route (Trails) chains renders through `prev.f32` and the meta file, which need a real directory.
    let scratch = if hist { Some(Scratch::new()?) } else { None };
    let mut files = RunFiles { words: Vec::new(), curve: None };
    let (src_path, out_path) = if hist { ("src%R.u32".to_string(), "out.r%R.u32".to_string()) } else { ("src.u32".to_string(), "out.u32".to_string()) };
    let count = extra.history.as_ref().map(|f| f.len()).unwrap_or(1);
    if let Some(frames) = extra.history {
        for (k, f) in frames.into_iter().enumerate() {
            files.words.push((format!("src{}.u32", k + 1), std::sync::Arc::new(f)));
        }
    } else {
        files.words.push((src_path.clone(), std::sync::Arc::new(src)));
    }
    files.words.push(("prior.u32".to_string(), std::sync::Arc::new(vec![p.prior_fill; w as usize * h as usize * 4])));
    let out_prefix = match &scratch {
        Some(s) => s.0.join("o").to_string_lossy().into_owned(),
        None => "o".to_string(),
    };
    let mut args: Vec<String> = vec![p.program.clone(), "--size".into(), format!("{}x{}", w, h), "--out".into(), out_prefix, "--ae32-src".into(), format!("{}={}", p.src_name, src_path)];
    // the sources a transition stage's plan may leave unread: its second clip, and its first one once the transition has completed
    let mut droppable: Vec<String> = layers.iter().map(|l| l.0.clone()).collect();
    if p.is_transition() {
        droppable.push(p.src_name.clone());
    }
    for (name, words) in layers {
        let lp = format!("layer_{}.u32", name);
        files.words.push((lp.clone(), std::sync::Arc::new(words)));
        args.push("--ae32-src".into());
        args.push(format!("{}={}", name, lp));
    }
    args.extend(["--ae32-prior".to_string(), "prior.u32".to_string(), "--ae32-out".to_string(), out_path.clone()]);
    let tick = if hist { "%R".to_string() } else { index.to_string() };
    args.extend(["--geom".to_string(), geom(w, h), "--host-frame".to_string(), tick.clone(), "--ctx-time".to_string(), tick]);
    if hist {
        let times: Vec<String> = (1..=count).map(|k| k.to_string()).collect();
        args.extend(["--host-times".to_string(), times.join(",")]);
    }
    for (k, v) in host.iter().chain(placement_for(p, w, h).iter()).chain(p.fixed.iter()) {
        if v == "<path>" {
            continue;
        }
        args.push(k.clone());
        // `W` in a size-dependent value stands for the frame width, `N` of `--tw-route` for the clip length
        args.push(fill_fixed(k, v, w as u64, extra.clip_len));
    }
    args.extend(params.iter().cloned());
    for (name, v) in &extra.host_values {
        args.push("--host-value".into());
        args.push(format!("{}=0x{:08x}", name, v.to_bits()));
    }
    // an unbound normalize_scale parameter takes its registered default from the native definitions
    args.push("--ns-default-unbound".into());
    let own_curve = p.params.iter().any(|q| q.id == "host_curve_values") && !args.iter().any(|a| a == "--host-curve-values");
    let curve_name = "curve.txt".to_string();
    if own_curve {
        files.curve = Some((curve_name.clone(), constant_curve(w, h, index, &learned.curve_keys(index))));
        args.push("--host-curve-values".into());
        args.push(curve_name.clone());
    }
    args.push("--do-postdiv".into());
    args.push(learned.postdiv.get().to_string());
    for (i, (key, words)) in extra.frame_words.into_iter().enumerate() {
        let name = format!("fw{}.u32", i);
        files.words.push((name.clone(), std::sync::Arc::new(words)));
        args.push("--frame-words".into());
        args.push(format!("{}={}", key, name));
    }
    // the `device: ` lines of the last run go to stderr when this frame is done, on success and on failure
    let device_lines = std::cell::RefCell::new(String::new());
    let _forward = ForwardDeviceLines(&device_lines);
    let exec = |args: &[String], files: &RunFiles| {
        let mut io = StageIo::default();
        io.keep_chain_files = hist;
        for (k, v) in &files.words {
            io.words.insert(k.clone(), v.clone());
        }
        if let Some((k, t)) = &files.curve {
            io.texts.insert(k.clone(), t.clone());
        }
        let (code, io) = runner.run(args.to_vec(), io);
        *device_lines.borrow_mut() = device_lines_of(&io.log);
        (code, io)
    };
    if let Some(n) = learned.xy_writers.get() {
        fit_xy_list(&mut args, n, index)?;
    }
    #[cfg(test)]
    LAST_RUN.with(|l| *l.borrow_mut() = Some((args.clone(), files.words.iter().map(|(k, v)| (k.clone(), v.clone())).collect())));
    let (mut code, mut io) = exec(&args, &files);
    // The owner-arm `do_postdiv` is a required cross-check that the run reveals only through its refusal; the library hands
    // over the value the owner arm derives, it is learned once per clip and kept. The samples the route reads from the stage's
    // own constant curve are learned the same way: each `HOST-CURVE-VALUE-MISSING` (in a plan error or a refusal) names one
    // sample, which is added with its identity value.
    for _ in 0..8 + CURVE_RERUNS {
        let text = io.log.clone();
        if code == 1 && text.contains("XY-BASE-LIST-MISMATCH") && learned.xy_writers.get().is_none() {
            // the measured placement lists fewer or more default-xy writers than this plan has: the run names the count
            let line = text.lines().find(|l| l.contains("XY-BASE-LIST-MISMATCH")).unwrap_or("").to_string();
            let n = parse_xy_writers(&text).ok_or_else(|| Error::unsupported(format!("frame {}: XY-BASE-LIST-UNSERVED: cannot read the writer count from fx_run: {}", index, line)))?;
            learned.xy_writers.set(Some(n));
            fit_xy_list(&mut args, n, index)?;
            (code, io) = exec(&args, &files);
            continue;
        }
        if own_curve && matches!(code, 1 | 2) && text.contains("HOST-CURVE-VALUE-MISSING (") {
            let line = text.lines().find(|l| l.contains("HOST-CURVE-VALUE-MISSING")).unwrap_or("").to_string();
            let key = match parse_curve_missing(&text) {
                Some(k) if offered_sample(p, k.index).is_some() => {
                    return Err(Error::unsupported(format!("frame {}: HOST-CURVE-SAMPLE-UNSERVED: {} is an offered parameter and the constant identity curve does not serve it: {}", index, offered_sample(p, k.index).unwrap_or(""), line)).with_tail(tail_lines(&text, 3)))
                }
                Some(k) if identity_sample(&p.program, k.index).is_some() => k,
                _ => return Err(Error::unsupported(format!("frame {}: HOST-CURVE-SAMPLE-UNSERVED: the constant identity curve cannot serve this sample; pass host_curve_values: {}", index, line)).with_tail(tail_lines(&text, 3))),
            };
            let entry = (key.index, key.tick - index as i64, key.step, key.scale);
            if learned.curve.borrow().contains(&entry) {
                return Err(Error::unsupported(format!("frame {}: HOST-CURVE-SAMPLE-UNSERVED: the sample is in the curve and still refused: {}", index, line)).with_tail(tail_lines(&text, 3)));
            }
            learned.curve.borrow_mut().push(entry);
            files.curve = Some((curve_name.clone(), constant_curve(w, h, index, &learned.curve_keys(index))));
            (code, io) = exec(&args, &files);
            continue;
        }
        // A clip the compiled plan does not read (Dissolve_Shake below its dissolve threshold drops To, and after the dissolve it drops
        // From) is not an input of the plan: the run names it, and the frame is rendered again without that source.
        if code == 1 {
            if let Some(name) = unread_source(&droppable, &text) {
                drop_source(&mut args, name);
                (code, io) = exec(&args, &files);
                continue;
            }
        }
        if code != 2 {
            break;
        }
        if text.contains("POSTDIV-MISMATCH") {
            match io.postdiv {
                Some(n) => {
                    learned.postdiv.set(n);
                    let at = args.iter().rposition(|a| a == "--do-postdiv").expect("do-postdiv");
                    args[at + 1] = n.to_string();
                    (code, io) = exec(&args, &files);
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
    let stderr = io.log.clone();
    let tail = tail_lines(&stderr, 3);
    let first = stderr.lines().rev().find(|l| !l.trim().is_empty()).unwrap_or("").to_string();
    match code {
        0 => {}
        1 if stderr.contains("XY-BASE-LIST-MISMATCH") => {
            return Err(Error::unsupported(format!("frame {}: XY-BASE-LIST-UNSERVED: the placement fx_run asks for cannot be served: {}", index, first)).with_tail(tail))
        }
        1 if stderr.contains("HOST-CURVE-VALUE-MISSING") => {
            return Err(Error::unsupported(format!("frame {}: HOST-CURVE-SAMPLE-UNSERVED: the host curve does not hold a sample the route reads: {}", index, first)).with_tail(tail))
        }
        2 => return Err(Error::unsupported(format!("frame {}: fx_run refused: {}", index, first)).with_tail(tail)),
        code => return Err(Error::stage_failed(format!("frame {}: fx_run failed with {:?}", index, Some(code))).with_tail(tail)),
    }
    let mut outs = Vec::with_capacity(count);
    for k in 1..=count {
        let name = if hist { format!("out.r{}.u32", k) } else { out_path.clone() };
        let words = io.outs.remove(&name).ok_or_else(|| Error::stage_failed(format!("fx_run wrote no {}", name)))?;
        if words.len() != w as usize * h as usize * 4 {
            return Err(Error::stage_failed(format!("frame {}: fx_run wrote {} words for a {}x{} frame", index + k as u64 - 1, words.len(), w, h)).with_tail(tail));
        }
        outs.push(words);
    }
    Ok(outs)
}

/// The first of `candidates` that the log `text` of a refused run names as not an input of the compiled plan. The matched text is built by
/// `fx_exec::srcindep::not_an_input`, the function that writes the refusal, so the retry follows the refusal.
fn unread_source<'a>(candidates: &'a [String], text: &str) -> Option<&'a String> {
    candidates.iter().find(|n| text.contains(&fx_exec::srcindep::not_an_input(n)))
}

/// Remove the `--ae32-src NAME=...` pair of a source the plan does not read. A recorded class-3 allocate command counts the sourced
/// class-3 inputs, so its image count drops by one with the source.
fn drop_source(args: &mut Vec<String>, name: &str) {
    let key = format!("{}=", name);
    if let Some(at) = args.windows(2).position(|w| w[0] == "--ae32-src" && w[1].starts_with(&key)) {
        args.drain(at..at + 2);
        if let Some(c) = args.iter().position(|a| a == "--class3-record") {
            let mut f: Vec<String> = args[c + 1].split(',').map(|t| t.to_string()).collect();
            if let Some(n) = f.get(1).and_then(|t| t.trim().parse::<u32>().ok()) {
                f[1] = n.saturating_sub(1).to_string();
                args[c + 1] = f.join(",");
            }
        }
    }
}

/// A layer file: one frame (held in memory) or one frame per input frame (read from the file when its frame is asked for, so a
/// long clip is never held whole).
struct Layer {
    id: String,
    spec: FrameSpec,
    frames: u64,
    held: Option<Vec<u32>>,
    file: Option<std::cell::RefCell<std::fs::File>>,
}

impl Layer {
    fn load(id: &str, path: &str, spec: FrameSpec) -> Result<Layer> {
        use std::io::{Read, Seek, SeekFrom};
        let open_err = |e: std::io::Error| Error::bad_input(format!("cannot open layer {} at {}: {}", id, path, e));
        let mut file = std::fs::File::open(path).map_err(open_err)?;
        let len = file.metadata().map_err(open_err)?.len();
        let fb = spec.frame_bytes();
        if len == 0 || len % fb != 0 {
            return Err(Error::bad_input(format!("layer {} holds {} bytes, which is not a whole number of {}x{} frames", id, len, spec.width, spec.height)));
        }
        let frames = len / fb;
        if frames == 1 {
            let mut buf = vec![0u8; fb as usize];
            file.seek(SeekFrom::Start(0)).and_then(|_| file.read_exact(&mut buf)).map_err(open_err)?;
            return Ok(Layer { id: id.to_string(), spec, frames, held: Some(frame_to_words(spec, &buf)), file: None });
        }
        Ok(Layer { id: id.to_string(), spec, frames, held: None, file: Some(std::cell::RefCell::new(file)) })
    }

    /// A held layer of one frame that is not read from a file.
    fn held(id: &str, spec: FrameSpec, words: Vec<u32>) -> Layer {
        Layer { id: id.to_string(), spec, frames: 1, held: Some(words), file: None }
    }

    fn at(&self, index: u64) -> Result<(String, Vec<u32>)> {
        use std::io::{Read, Seek, SeekFrom};
        if let Some(f) = &self.held {
            return Ok((self.id.clone(), f.clone()));
        }
        if index >= self.frames {
            return Err(Error::bad_input(format!("layer {} holds {} frames but the input has more", self.id, self.frames)));
        }
        let fb = self.spec.frame_bytes();
        let mut buf = vec![0u8; fb as usize];
        let mut file = self.file.as_ref().expect("a layer of several frames has a file").borrow_mut();
        file.seek(SeekFrom::Start(index * fb))
            .and_then(|_| file.read_exact(&mut buf))
            .map_err(|e| Error::bad_input(format!("cannot read frame {} of layer {}: {}", index, self.id, e)))?;
        Ok((self.id.clone(), frame_to_words(self.spec, &buf)))
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

/// The `--param` values of a run, with the keyed ones read per frame.
struct Keyed {
    plain: Vec<(String, String)>,
    tracks: Vec<(String, crate::keys::Track, bool)>,
}

impl Keyed {
    /// Split `values` into plain and keyed ones. A keyed value needs the rate (its time base) and a stage that runs
    /// fx_run per frame.
    fn new(p: &Profile, values: &[(String, String)], rate: Option<&crate::rate::Rate>) -> Result<Keyed> {
        use crate::keys::{time_base, Shape, Track};
        let mut plain = Vec::new();
        let mut tracks = Vec::new();
        for (id, value) in values {
            if !crate::keys::is_keyed(value) {
                plain.push((id.clone(), value.clone()));
                continue;
            }
            if p.spool {
                return Err(Error::unsupported(format!(
                    "bad value for `{}`: {} renders the whole clip in one fx_run run, so its parameters cannot change from frame to frame",
                    id, p.id
                )));
            }
            let r = rate.ok_or_else(|| Error::usage(format!("`{}` is keyed: keyframes need --fps N[/D], the time base of the keys", id)))?;
            let (per_frame, per_second) = time_base(r.num(), r.den()).map_err(|e| Error::unsupported(format!("bad value for `{}`: {}", id, e)))?;
            let q = p.params.iter().find(|q| q.id == *id).ok_or_else(|| Error::unsupported(format!("unknown param `{}`", id)))?;
            let shape = match (q.form.as_str(), q.ptype.as_str(), q.lanes) {
                ("--cset", _, _) => Shape::Hold,
                (_, "scalar", 1) => Shape::Float { min: q.min.unwrap_or(f64::MIN), max: q.max.unwrap_or(f64::MAX) },
                (_, "angle", 1) => Shape::Angle,
                (_, "vector", 2) => Shape::Point,
                _ => Shape::Hold,
            };
            let t = Track::parse(value, shape, per_frame, per_second).map_err(|e| Error::unsupported(format!("bad value for `{}`: {}", id, e)))?;
            tracks.push((id.clone(), t, q.form == "--host-param"));
        }
        Ok(Keyed { plain, tracks })
    }

    /// The values of frame `f`. An interpolated host value is put on the 1/65536 grid of a host slider (round half away from zero).
    fn values_at(&self, f: u64) -> Vec<(String, String)> {
        let mut out = self.plain.clone();
        for (id, t, host) in &self.tracks {
            let text = match t.lanes_at((f as i64).saturating_mul(t.ticks_per_frame)) {
                Ok(i) => t.keys[i].text.clone(),
                Err(lanes) => lanes
                    .iter()
                    .map(|x| if *host { (x * 65536.0).round() / 65536.0 } else { *x })
                    .map(|x| format!("{}", x))
                    .collect::<Vec<_>>()
                    .join(","),
            };
            out.push((id.clone(), text));
        }
        out
    }

    /// Every combination a held parameter can take, for bounds: the plain values plus each key value of each track.
    fn variants(&self) -> Vec<Vec<(String, String)>> {
        let mut out = vec![self.plain.clone()];
        for (id, t, _) in &self.tracks {
            for text in t.key_texts() {
                let mut v = self.plain.clone();
                v.push((id.clone(), text));
                out.push(v);
            }
        }
        out
    }
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

impl Profile {
    /// A transition stage: it declares a second layer (the To clip).
    pub fn is_transition(&self) -> bool {
        !self.layers.is_empty() && fx_exec::fxrun::program_has_flag(&self.program, "transition")
    }

    /// Whether the bound parameters keep `get_percent_time` live, so the stage must be told the clip length.
    /// The transition stages map the clip onto the layer and the composition alike.
    pub fn time_live(&self) -> bool {
        self.is_transition()
    }
}

/// `get_percent_time` of frame `f` in a clip of `n` frames: f32(f) / f32(n - 1).
pub fn percent_time(f: u64, n: u64) -> f32 {
    f as f32 / (n - 1) as f32
}

/// The refusal of a rate or frame whose time fields do not fit the host's 32-bit fields.
fn time_range_error(num: u128, den: u128, f: u64) -> Error {
    Error::unsupported(format!("HOST-TIME-RANGE (get_time: --fps {}/{} at frame {} needs a host time field outside 32 bits)", num, den, f))
}

/// The largest numerator (time scale, unsigned 32-bit) and denominator (local time step, signed 32-bit) the host fields hold.
const HOST_SCALE_MAX: u128 = u32::MAX as u128;
const HOST_STEP_MAX: u128 = i32::MAX as u128;

/// Refuse a rate whose time scale or local time step does not fit the host's 32-bit time fields, before any frame is read.
fn check_time_rate(rate: Option<&crate::rate::Rate>) -> Result<()> {
    match rate {
        Some(r) if r.num() > HOST_SCALE_MAX || r.den() > HOST_STEP_MAX => Err(time_range_error(r.num(), r.den(), 0)),
        _ => Ok(()),
    }
}

/// `get_time` of frame `f` at the frame rate `num/den`, in the order the host computes it for a time scale `num` and a local time step
/// `den`: with tick = f * den, F = f32(tick) / f32(den) and rate = f32(num) / f32(den), the value is F / d, where d is 30.0 when
/// 1.0e-4 is above the rate and the rate otherwise. Every conversion is the correctly rounded f32 of the integer and every division
/// is an f32 division. A tick above 2^31 - 1, a time scale above 2^32 - 1 or a local time step above 2^31 - 1 is refused.
pub fn host_time(f: u64, num: u64, den: u64) -> Result<f32> {
    let tick = f as u128 * den as u128;
    if num as u128 > HOST_SCALE_MAX || den as u128 > HOST_STEP_MAX || tick > HOST_STEP_MAX {
        return Err(time_range_error(num as u128, den as u128, f));
    }
    let elapsed = tick as f32 / den as f32;
    let d = fx_exec::stored_frame_rate(host_rate(num, den));
    Ok(elapsed / d)
}

/// `get_frames_per_second` at the frame rate `num/den`: the host model has time scale `num` and local time step `den`,
/// so the value handed to the fold is f32(num) / f32(den); the fold applies the context replacement of the getter.
pub fn host_rate(num: u64, den: u64) -> f32 {
    num as f32 / den as f32
}

/// `get_aspect_ratio` of a frame: f32(height) / f32(width), the host fields +0x54 (the ratio) times +0x34 (the unit factor, 1.0).
pub fn aspect_ratio(width: u32, height: u32) -> f32 {
    height as f32 / width as f32
}

/// The host values of frame `f`: `get_time` and `get_frames_per_second` when the rate is known, and percent time and total frames when the
/// clip length is known (a transition stage also gets the frame's aspect ratio, `spec` being its size).
fn time_values(total: Option<u64>, rate: Option<&crate::rate::Rate>, f: u64, spec: FrameSpec) -> Result<Vec<(String, f32)>> {
    let mut out = Vec::new();
    if let Some(r) = rate {
        if r.num() > HOST_SCALE_MAX || r.den() > HOST_STEP_MAX {
            return Err(time_range_error(r.num(), r.den(), f));
        }
        out.push(("get_time".to_string(), host_time(f, r.num() as u64, r.den() as u64)?));
        out.push(("get_frames_per_second".to_string(), host_rate(r.num() as u64, r.den() as u64)));
    }
    if let Some(n) = total {
        out.push(("get_percent_time".to_string(), percent_time(f, n)));
        out.push(("get_total_frames".to_string(), n as f32));
        out.push(("get_aspect_ratio".to_string(), aspect_ratio(spec.width, spec.height)));
    }
    Ok(out)
}

/// The clip length a time-live stage needs: `--frames N` with N of 2 or more, refused by name otherwise.
fn check_time(p: &Profile, frames: Option<u64>) -> Result<Option<u64>> {
    if !p.time_live() {
        return Ok(None);
    }
    match frames {
        None => Err(Error::unsupported(format!("FRAMES-REQUIRED: {} keeps get_percent_time live and needs `--frames N` (the clip length)", p.id))),
        Some(n) if n < 2 => Err(Error::unsupported(format!("FRAMES-TOO-SMALL: {} needs `--frames` of 2 or more, got {}", p.id, n))),
        Some(n) => Ok(Some(n)),
    }
}

/// Load the `--layer ID=PATH` inputs of a transition stage. `To` names the stage's second layer; the declared layer id
/// is accepted as well. A missing file, a size that is not whole frames of the main size, an unknown id and a layer
/// given twice are refused by name (exit 3), as is a required layer that is absent.
fn load_layers(p: &Profile, layers: &[String], spec: FrameSpec) -> Result<Vec<Layer>> {
    let mut out: Vec<Layer> = Vec::new();
    for l in layers {
        let (id, path) = l.split_once('=').filter(|(i, v)| !i.is_empty() && !v.is_empty()).ok_or_else(|| Error::usage(format!("`--layer {}`: expected ID=PATH", l)))?;
        let declared = p
            .layers
            .iter()
            .find(|(d, _)| d == id)
            .or_else(|| if id == "To" { p.layers.first() } else { None })
            .ok_or_else(|| Error::unsupported(format!("LAYER-ID-UNKNOWN: {} has no layer `{}`", p.id, id)))?;
        if out.iter().any(|x| x.id == declared.0) {
            return Err(Error::unsupported(format!("LAYER-DUPLICATE: layer `{}` of {} is given twice", declared.0, p.id)));
        }
        let meta = std::fs::metadata(path).map_err(|e| Error::unsupported(format!("LAYER-FILE-MISSING: cannot open layer {} at {}: {}", id, path, e)))?;
        let fb = spec.frame_bytes();
        if meta.len() == 0 || meta.len() % fb != 0 {
            return Err(Error::unsupported(format!(
                "LAYER-SIZE-MISMATCH: layer {} holds {} bytes, which is not a whole number of {}x{} frames ({} bytes each)",
                id,
                meta.len(),
                spec.width,
                spec.height,
                fb
            )));
        }
        out.push(Layer::load(&declared.0, path, spec)?);
    }
    for (d, optional) in &p.layers {
        if out.iter().any(|x| &x.id == d) {
            continue;
        }
        if !optional {
            return Err(Error::unsupported(format!("LAYER-INPUT-MISSING: {} needs `--layer To=FILE` (the second clip)", p.id)));
        }
        // an optional layer that is not given is a layer at None: the host answers it with NULL pixels, and the plan's
        // convert of such a layer reads all-zero planes (one held frame of zero words, alpha included)
        let words = spec.width as usize * spec.height as usize * 4;
        out.push(Layer::held(d, spec, vec![0u32; words]));
    }
    Ok(out)
}

/// Run the effect stage over `input`.
pub fn apply(
    id: &str,
    spec: FrameSpec,
    rate: Option<&crate::rate::Rate>,
    values: &[(String, String)],
    layers: &[String],
    frames: Option<u64>,
    device: fx_exec::fxrun::Device,
    input: &mut dyn Read,
    output: &mut dyn Write,
) -> Result<()> {
    let p = profile(id).ok_or_else(|| Error::unsupported(format!("unknown stage `{}`", id)))?;
    if !layers.is_empty() && !p.is_transition() {
        return Err(Error::unsupported(format!("LAYER-INPUT-UNSUPPORTED: {} takes no `--layer` input; only transition stages take a second layer", p.id)));
    }
    let total = check_time(p, frames)?;
    check_variants(p)?;
    if let Some(m) = missing() {
        return Err(Error::unsupported(format!("the effect stages need fx_run: {}", m)));
    }
    // the runner thread selects and installs the backend (`fx_exec::fxrun::select_and_install`)
    let runner = Runner::new(device);
    if spec.width < 2 {
        return Err(Error::unsupported(format!("{} needs a width of 2 or more (the host grid needs two columns)", p.id)));
    }
    let keyed = Keyed::new(p, values, rate)?;
    check_time_rate(rate)?;
    // the arguments of frame `f`: a keyed parameter takes its value at that frame
    let args_at = |f: u64| param_args(p, &keyed.values_at(f));
    let ls: Vec<Layer> = load_layers(p, layers, spec)?;
    let fb = spec.frame_bytes() as usize;
    let mut buf = vec![0u8; fb];
    let mut out = vec![0u8; fb];
    let mut count = 0u64;
    let learned = Learned::default();
    let layer_words = |index: u64| -> Result<Vec<(String, Vec<u32>)>> { ls.iter().map(|l| l.at(index)).collect() };
    if p.id == "time_warp_rgb_autogen" {
        // the clip is read whole: its length is the duration of the time warp route
        let mut clip: Vec<Vec<u32>> = Vec::new();
        while read_frame(input, &mut buf, clip.len() as u64)? {
            clip.push(frame_to_words(spec, &buf));
        }
        let last = clip.len() as u64;
        let mut f = 0u64;
        loop {
            if f >= last {
                break;
            }
            let get = |k: u64| -> &Vec<u32> { &clip[k.min(last - 1) as usize] };
            let mut extra = Extra { host_values: time_values(total, rate, f, spec)?, clip_len: Some(last), ..Extra::default() };
            let vf = keyed.values_at(f);
            for s in tw_shifts(p, &vf) {
                // the route asks at most the last frame of the clip
                let k = ((f as i64 + s as i64).max(0) as u64).min(last - 1);
                extra.frame_words.push((format!("{},{}", k, k), get(k).clone()));
            }
            let words = run_fx(&runner, p, spec, f, get(f).clone(), layer_words(f)?, &param_args(p, &vf)?, &host_at(p, &vf), extra, &learned)?.remove(0);
            words_to_frame(spec, &words, &mut out);
            write_out(output, &out, f)?;
            count += 1;
            f += 1;
        }
    } else if p.spool {
        // Trails: buffer the clip, run the measured history route once (render k is frame k-1 at time k)
        let mut clip: Vec<Vec<u32>> = Vec::new();
        while read_frame(input, &mut buf, clip.len() as u64)? {
            clip.push(frame_to_words(spec, &buf));
        }
        if !clip.is_empty() {
            let extra = Extra { frame_words: Vec::new(), history: Some(clip), host_values: Vec::new(), clip_len: None };
            let outs = run_fx(&runner, p, spec, 0, Vec::new(), Vec::new(), &args_at(0)?, &host_at(p, &keyed.values_at(0)), extra, &learned)?;
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
            if let Some(n) = total {
                if count >= n {
                    return Err(Error::bad_input(format!("the input holds more than {} frames, but --frames says {}", n, n)));
                }
            }
            let src = frame_to_words(spec, &buf);
            let words = run_fx(&runner, p, spec, count, src, layer_words(count)?, &args_at(count)?, &host_at(p, &keyed.values_at(count)), Extra { host_values: time_values(total, rate, count, spec)?, ..Extra::default() }, &learned)?.remove(0);
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
    /// The keyed-value shape a stage parameter takes, and the value halfway between two keys.
    fn keyed_at_5(stage: &str, id: &str, keys: &str) -> (crate::keys::Shape, String, String) {
        let p = profile(stage).unwrap_or_else(|| panic!("stage {}", stage));
        let rate = crate::rate::Rate::parse("24").expect("rate");
        let q = p.params.iter().find(|q| q.id == id).unwrap_or_else(|| panic!("param {}", id));
        let k = Keyed::new(p, &[(id.to_string(), keys.to_string())], Some(&rate)).expect("keyed");
        let shape = k.tracks[0].1.shape.clone();
        let v = k.values_at(5).into_iter().find(|(i, _)| i == id).unwrap().1;
        (shape, q.ptype.clone(), v)
    }

    #[test]
    fn keyed_boolean_holds_between_keys() {
        let (shape, ptype, v) = keyed_at_5("blur_color_autogen", "subpixel", "0@0;1@10");
        assert_eq!((shape, ptype.as_str(), v.as_str()), (crate::keys::Shape::Hold, "boolean", "0"));
    }

    fn forwarded(id: &str, value: &str) -> Vec<String> {
        let p = profile("blur_color_autogen").expect("stage");
        super::param_args(p, &[(id.to_string(), value.to_string())]).expect("args")
    }

    #[test]
    fn checkbox_words_are_forwarded_as_numbers() {
        assert_eq!(forwarded("subpixel", "off"), vec!["--set", "subpixel=0"]);
        assert_eq!(forwarded("subpixel", "on"), vec!["--set", "subpixel=1"]);
        assert_eq!(forwarded("subpixel", "false"), vec!["--set", "subpixel=0"]);
        assert_eq!(forwarded("subpixel", "true"), vec!["--set", "subpixel=1"]);
        assert_eq!(forwarded("blur_amount", "2.5"), vec!["--host-param", "blur_amount=2.5"]);
    }

    #[test]
    fn keyed_checkbox_words_are_forwarded_as_numbers() {
        let p = profile("blur_color_autogen").expect("stage");
        let rate = crate::rate::Rate::parse("24").expect("rate");
        let k = Keyed::new(p, &[("subpixel".to_string(), "off@0;on@10".to_string())], Some(&rate)).expect("keyed");
        let at = |f: u64| super::param_args(p, &k.values_at(f)).expect("args");
        assert_eq!(at(5), vec!["--set", "subpixel=0"]);
        assert_eq!(at(10), vec!["--set", "subpixel=1"]);
    }

    #[test]
    fn keyed_popup_holds_between_keys() {
        let (shape, ptype, v) = keyed_at_5("blur_color_autogen", "combine_masks", "0@0;2@10");
        assert_eq!((shape, ptype.as_str(), v.as_str()), (crate::keys::Shape::Hold, "popup", "0"));
    }

    #[test]
    fn keyed_two_lane_popup_holds_between_keys() {
        let (shape, ptype, v) = keyed_at_5("warp_perspective_autogen", "wrap", "0,0@0;2,2@10");
        assert_eq!((shape, ptype.as_str(), v.as_str()), (crate::keys::Shape::Hold, "popup", "0,0"));
    }

    #[test]
    fn keyed_angle_takes_the_angle_shape() {
        for (stage, id) in [("warp_perspective_autogen", "swing"), ("warp_perspective_autogen", "roll"), ("distort_rgb_autogen", "rotate_warp_dir")] {
            let (shape, ptype, v) = keyed_at_5(stage, id, "0@0;40@10");
            assert_eq!((shape, ptype.as_str()), (crate::keys::Shape::Angle, "angle"), "{} {}", stage, id);
            assert_eq!(v, "20", "{} {}", stage, id);
        }
    }

    #[test]
    fn only_device_lines_are_forwarded() {
        let log = "fx_exec: note: x\ndevice: gpu 0: report\n  device: indented\ndevice: rendering on the CPU\nboom\n";
        assert_eq!(super::device_lines_of(log), "device: gpu 0: report\ndevice: rendering on the CPU\n");
        assert_eq!(super::device_lines_of(""), "");
    }

    use super::*;

    #[test]
    fn tables_parse_for_twenty_stages() {
        let ps = profiles();
        assert_eq!(ps.len(), 20);
        let hsb = profile("hue_sat_bright_autogen").expect("hsb");
        assert_eq!(hsb.family, "S_HueSatBright");
        assert_eq!(hsb.kind, "temporal");
        assert!(hsb.host.iter().any(|(k, v)| k == "--src-planes" && v == "3"));
        assert!(hsb.params.iter().any(|q| q.id == "hue_shift"));
        assert_eq!(profile("trails_autogen").unwrap().spool, true);
        assert_eq!(profile("time_warp_rgb_autogen").unwrap().kind, "temporal");
        // only the transition stages declare a layer: their second clip, optional
        assert!(ps.iter().filter(|q| q.id != "swishpan" && q.id != "dissolve_shake").all(|q| q.layers.is_empty()));
        assert_eq!(profile("swishpan").unwrap().layers, vec![("To".to_string(), true)]);
        assert_eq!(profile("dissolve_shake").unwrap().layers, vec![("To".to_string(), true)]);
        // the three Shake styles fix the mask parameters of their unbound Mask input
        for id in ["shake_normal_autogen", "shake_twitchy_autogen", "shake_jumpy_autogen"] {
            let p = profile(id).unwrap();
            assert_eq!(p.family, "S_Shake");
            assert_eq!(p.fixed, vec![("--set".to_string(), "mask_use=0".to_string()), ("--set".to_string(), "blur_mask=0.05".to_string()), ("--set".to_string(), "invert_mask=0".to_string())]);
        }
        assert_eq!(profile("dissolve_shake").unwrap().family, "S_DissolveShake");
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
    fn xy_writer_count_is_read_and_served() {
        let e = "bad plan: XY-BASE-LIST-MISMATCH (the placement model has 3 default-xy writers, --xy-base-list gives 2)";
        assert_eq!(parse_xy_writers(e), Some(3));
        assert_eq!(parse_xy_writers("XY-BASE-LIST-MISMATCH (other)"), None);
        let mut a: Vec<String> = ["--xy-base-list", "0,0"].iter().map(|s| s.to_string()).collect();
        fit_xy_list(&mut a, 3, 0).unwrap();
        assert_eq!(a[1], "0,0,0");
        let mut b: Vec<String> = ["--xy-base-list", "0,16"].iter().map(|s| s.to_string()).collect();
        assert!(fit_xy_list(&mut b, 3, 0).is_err());
    }

    #[test]
    fn curve_refusal_names_one_sample() {
        let e = "bad plan: compile: HOST-CURVE-VALUE-MISSING (Transform_and_Blur_Autogen: index 23, tick 1, step 1, scale 24)";
        let k = parse_curve_missing(e).unwrap();
        assert_eq!(k, CurveKey { index: 23, tick: 1, step: 1, scale: 24 });
        assert!(parse_curve_missing("HOST-CURVE-VALUE-MISSING (P: time argument not a constant)").is_none());
        assert!(identity_sample("Transform_and_Blur_Autogen", 99).is_none());
        assert_eq!(identity_sample("SwishPan", 6), Some(("wipe_amt", 0)));
        assert!(identity_sample("Blur_Motion_Curves_No_Transform_Autogen", 6).is_none());
        assert!(identity_sample("SwishPan", 23).is_none());
        assert_eq!(identity_sample("Transform_and_Blur_Autogen", 22), Some(("z_dist", 65536)));
        assert!(identity_sample("Blur_Color_Autogen", 22).is_none());
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

    /// `run_fx` of the real SwishPan row for frame 0 at 64x32, with the time values the row's route reads.
    fn run_swishpan(p: &Profile) -> Result<Vec<Vec<u32>>> {
        let spec = FrameSpec { width: 64, height: 32, depth: Depth::F32 };
        let extra = Extra { host_values: time_values(Some(3), None, 0, spec).unwrap(), ..Extra::default() };
        let runner = Runner::new(fx_exec::fxrun::Device::Cpu);
        let layers: Vec<(String, Vec<u32>)> = load_layers(p, &[], spec).unwrap().iter().map(|l| l.at(0).unwrap()).collect();
        run_fx(&runner, p, spec, 0, vec![0u32; 64 * 32 * 4], layers, &[], &host_at(p, &[]), extra, &Learned::default())
    }

    #[test]
    fn run_fx_refuses_an_offered_curve_parameter_and_serves_a_hidden_one() {
        let row = profile("swishpan").unwrap().clone();
        assert!(offered_sample(&row, 6).is_none());
        // the row does not offer wipe_amt: the constant identity curve serves the sample fx_run asks for, and the frame renders
        let served = run_swishpan(&row).unwrap_or_else(|e| panic!("{} {:?}", e.message, e.stderr_tail));
        assert_eq!(served.len(), 1);
        // a row that offers wipe_amt is refused by name when fx_run asks for that sample
        let mut offered = row.clone();
        offered.params.push(Param { id: "wipe_amt".into(), form: "--set".into(), name: "wipe_amt".into(), kind: "float".into(), lanes: 1, ptype: String::new(), default: String::new(), answer_default: String::new(), min: None, max: None });
        assert_eq!(offered_sample(&offered, 6), Some("wipe_amt"));
        let e = run_swishpan(&offered).unwrap_err();
        assert_eq!(e.exit_code(), 3, "{}", e.message);
        assert!(e.message.contains("HOST-CURVE-SAMPLE-UNSERVED") && e.message.contains("wipe_amt is an offered parameter"), "{}", e.message);
    }

    fn transition() -> Profile {
        let mut p = profile("distort_rgb_autogen").unwrap().clone();
        p.program = "SwishPan".to_string();
        p.layers = vec![("Background".to_string(), false)];
        p
    }

    fn temp(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("sapphire-fx-{}-{}", tag, std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn percent_time_formula() {
        let n = 25u64;
        assert_eq!(percent_time(0, n), 0.0);
        assert_eq!(percent_time(12, n).to_bits(), (12f32 / 24f32).to_bits());
        assert_eq!(percent_time(n - 1, n), 1.0);
        let sp = FrameSpec { width: 50, height: 7, depth: Depth::F32 };
        let v = time_values(Some(n), None, 12, sp).unwrap();
        assert_eq!(v[2], ("get_aspect_ratio".to_string(), 7f32 / 50f32));
        assert_eq!(aspect_ratio(64, 32), 0.5);
        assert_eq!(v[0], ("get_percent_time".to_string(), 0.5));
        assert_eq!(v[1], ("get_total_frames".to_string(), 25.0));
        assert!(time_values(None, None, 3, sp).unwrap().is_empty());
        let r = crate::rate::Rate::parse("24/1").unwrap();
        for f in [0u64, 7, 17] {
            assert_eq!(time_values(None, Some(&r), f, sp).unwrap()[0].1.to_bits(), (f as f32 / 24f32).to_bits());
        }
        let rv = time_values(None, Some(&r), 0, sp).unwrap();
        assert_eq!(rv[1].0, "get_frames_per_second");
        assert_eq!(rv[1].1.to_bits(), 24f32.to_bits());
        let r2 = crate::rate::Rate::parse("30000/1001").unwrap();
        assert_eq!(time_values(None, Some(&r2), 0, sp).unwrap()[1].1.to_bits(), (30000f32 / 1001f32).to_bits());
        assert_eq!(host_time(7, 24, 1).unwrap(), 7f32 / 24f32);
    }

    #[test]
    fn host_time_equals_the_getter_read_order_and_differs_from_one_division() {
        // frames 0..=1000: get_time against the getter's read order F / rate with F = f32(f) and rate = f32(num) / f32(den), and against
        // the single division f32(f * den) / f32(num). The read order gives no difference at any rate; the single division differs in
        // 312, 69, 69 and 277 frames at the fractional rates and in none at a whole rate.
        for (num, den, single_differs) in [(24u64, 1u64, 0usize), (30, 1, 0), (60, 1, 0), (2997, 1, 0), (24000, 1001, 312), (30000, 1001, 69), (60000, 1001, 69), (2997, 100, 277)] {
            let rate = host_rate(num, den);
            let (mut other_order, mut other_single) = (0usize, 0usize);
            for f in 0..=1000u64 {
                let got = host_time(f, num, den).unwrap().to_bits();
                if got != (f as f32 / rate).to_bits() {
                    other_order += 1;
                }
                if got != ((f * den) as f32 / num as f32).to_bits() {
                    other_single += 1;
                }
            }
            assert_eq!(other_order, 0, "{}/{} against F / rate", num, den);
            assert_eq!(other_single, single_differs, "{}/{} against the single division", num, den);
        }
    }

    #[test]
    fn get_time_divides_by_the_stored_frame_rate() {
        // the divisor of get_time is the stored frame rate of the rate getter's first step: a rate below 1.0e-4 is replaced by 30.0
        for (num, den) in [(24u64, 1u64), (30000, 1001), (1, 100000), (1, 10000), (1, 9999)] {
            let rate = host_rate(num, den);
            let want = (((7 * den) as f32) / den as f32) / fx_exec::stored_frame_rate(rate);
            assert_eq!(host_time(7, num, den).unwrap().to_bits(), want.to_bits(), "{}/{}", num, den);
        }
        assert_eq!(fx_exec::stored_frame_rate(host_rate(1, 100000)), 30.0);
        assert_eq!(fx_exec::stored_frame_rate(host_rate(24, 1)), 24.0);
    }

    #[test]
    fn an_offered_parameter_is_never_served_by_the_identity_curve() {
        let mut p = profile("distort_rgb_autogen").unwrap().clone();
        p.program = "SwishPan".to_string();
        assert!(offered_sample(&p, 6).is_none());
        p.params.push(Param { id: "wipe_amt".into(), form: "--set".into(), name: "wipe_amt".into(), kind: "float".into(), lanes: 1, ptype: String::new(), default: String::new(), answer_default: String::new(), min: None, max: None });
        assert_eq!(offered_sample(&p, 6), Some("wipe_amt"));
        assert!(offered_sample(&p, 23).is_none());
    }

    #[test]
    fn a_declared_layer_alone_does_not_make_a_stage_a_transition() {
        assert!(transition().is_transition());
        let mut p = profile("distort_rgb_autogen").unwrap().clone();
        p.layers = vec![("Background".to_string(), false)];
        assert!(!p.is_transition() && !p.time_live());
    }

    /// The three f32 divisions of the host's time computation, written out independently of `host_time`.
    fn reference_time(f: u64, num: u64, den: u64) -> f32 {
        let tick: f32 = (f * den) as f32;
        let step: f32 = den as f32;
        let scale: f32 = num as f32;
        let elapsed: f32 = tick / step;
        let rate: f32 = scale / step;
        let divisor: f32 = if rate < 1.0e-4 { 30.0 } else { rate };
        elapsed / divisor
    }

    #[test]
    fn get_time_at_whole_rates_is_the_plain_f32_quotient() {
        for (num, den) in [(24u64, 1u64), (30, 1), (60, 1), (2997, 1)] {
            for f in 0..=1000u64 {
                let want = (f as f32) / (num as f32);
                assert_eq!(host_time(f, num, den).unwrap().to_bits(), want.to_bits(), "{}/{} frame {}", num, den, f);
            }
        }
    }

    #[test]
    fn get_time_follows_the_read_order_and_differs_from_the_single_division_where_expected() {
        for (num, den, expect) in [(24000u64, 1001u64, 312usize), (30000, 1001, 69), (60000, 1001, 69), (2997, 100, 277)] {
            let mut differ = 0usize;
            for f in 0..=1000u64 {
                let got = host_time(f, num, den).unwrap();
                assert_eq!(got.to_bits(), reference_time(f, num, den).to_bits(), "{}/{} frame {}", num, den, f);
                let single = ((f * den) as f32) / (num as f32);
                if got.to_bits() != single.to_bits() {
                    differ += 1;
                }
            }
            assert_eq!(differ, expect, "{}/{}", num, den);
        }
    }

    #[test]
    fn a_rate_below_the_floor_divides_by_thirty() {
        // rate = 1/100000 is below 1.0e-4, so the divisor is 30.0
        let got = host_time(7, 1, 100000).unwrap();
        assert_eq!(got.to_bits(), ((7u64 * 100000) as f32 / 100000f32 / 30.0f32).to_bits());
    }

    #[test]
    fn get_time_refuses_values_outside_the_host_time_fields() {
        // the last frame at 30000/1001 whose tick fits 2^31 - 1 is served, the next one is refused
        assert!(host_time(2145338, 30000, 1001).is_ok());
        let e = host_time(2145339, 30000, 1001).unwrap_err();
        assert!(e.to_string().starts_with("HOST-TIME-RANGE (get_time: --fps 30000/1001 at frame 2145339 needs a host time field outside 32 bits)"), "{}", e);
        assert!(host_time(0, 4294967296, 1).is_err());
        assert!(host_time(0, 4294967295, 1).is_ok());
        assert!(host_time(0, 24, 2147483648).is_err());
        assert!(host_time(0, 24, 2147483647).is_ok());
        // the rate check runs before any frame is read
        let big = crate::rate::Rate::new(4294967296, 1).unwrap();
        assert!(check_time_rate(Some(&big)).is_err());
        assert!(check_time_rate(Some(&crate::rate::Rate::new(30000, 1001).unwrap())).is_ok());
        assert!(check_time_rate(None).is_ok());
        let sp = FrameSpec { width: 2, height: 2, depth: Depth::F32 };
        assert!(time_values(None, Some(&big), 0, sp).is_err());
        let r = crate::rate::Rate::new(30000, 1001).unwrap();
        assert!(time_values(None, Some(&r), 2145338, sp).is_ok());
        assert!(time_values(None, Some(&r), 2145339, sp).is_err());
    }

    #[test]
    fn layer_frames_are_read_when_asked_for() {
        let spec = FrameSpec { width: 2, height: 1, depth: Depth::U8 };
        let dir = temp("layerstream");
        let f = dir.join("four.raw");
        let bytes: Vec<u8> = (0u8..32).collect();
        std::fs::write(&f, &bytes).unwrap();
        let l = Layer::load("Lens", f.to_str().unwrap(), spec).unwrap();
        assert!(l.held.is_none() && l.frames == 4);
        for k in [3u64, 0, 2, 2, 1] {
            let (_, w) = l.at(k).unwrap();
            assert_eq!(w, frame_to_words(spec, &bytes[8 * k as usize..8 * k as usize + 8]));
        }
        assert_eq!(l.at(4).unwrap_err().exit_code(), 4);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn frames_are_required_by_name_for_a_time_live_stage() {
        let p = transition();
        for (frames, name) in [(None, "FRAMES-REQUIRED"), (Some(1), "FRAMES-TOO-SMALL")] {
            let e = check_time(&p, frames).unwrap_err();
            assert_eq!(e.exit_code(), 3);
            assert!(e.message.starts_with(name), "{}", e.message);
        }
        assert_eq!(check_time(&p, Some(2)).unwrap(), Some(2));
        let plain = profile("distort_rgb_autogen").unwrap();
        assert!(!plain.time_live());
        assert_eq!(check_time(plain, None).unwrap(), None);
    }

    #[test]
    fn to_layer_is_loaded_and_refused_by_name() {
        let spec = FrameSpec { width: 2, height: 1, depth: Depth::U8 };
        let p = transition();
        let dir = temp("tolayer");
        let good = dir.join("to.raw");
        let bytes: Vec<u8> = (0u8..16).collect();
        std::fs::write(&good, &bytes).unwrap();
        let ok = load_layers(&p, &[format!("To={}", good.display())], spec).unwrap();
        assert_eq!(ok.len(), 1);
        let (name, words) = ok[0].at(1).unwrap();
        assert_eq!(name, "Background");
        assert_eq!(words, frame_to_words(spec, &bytes[8..16]));
        let bad = dir.join("bad.raw");
        std::fs::write(&bad, vec![0u8; 5]).unwrap();
        let name_of = |l: &[String]| {
            let e = load_layers(&p, l, spec).err().unwrap();
            assert_eq!(e.exit_code(), 3, "{}", e.message);
            e.message
        };
        assert!(name_of(&[format!("To={}", bad.display())]).starts_with("LAYER-SIZE-MISMATCH"));
        assert!(name_of(&["To=/nonexistent/to.raw".to_string()]).starts_with("LAYER-FILE-MISSING"));
        assert!(name_of(&[format!("Other={}", good.display())]).starts_with("LAYER-ID-UNKNOWN"));
        assert!(name_of(&[]).starts_with("LAYER-INPUT-MISSING"));
        let twice = format!("To={}", good.display());
        assert!(name_of(&[twice.clone(), twice]).starts_with("LAYER-DUPLICATE"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn absent_optional_layer_is_a_zero_frame() {
        let spec = FrameSpec { width: 2, height: 1, depth: Depth::U8 };
        let mut p = transition();
        p.layers = vec![("To".to_string(), true)];
        let ls = load_layers(&p, &[], spec).unwrap();
        assert_eq!(ls.len(), 1);
        let (name, words) = ls[0].at(7).unwrap();
        assert_eq!(name, "To");
        assert_eq!(words, vec![0u32; 8]);
    }

    #[test]
    fn to_layer_and_host_values_reach_fx_run() {
        let spec = FrameSpec { width: 64, height: 32, depth: Depth::U8 };
        let p = transition();
        let dir = temp("reach");
        let to = dir.join("to.raw");
        let bytes: Vec<u8> = (0..spec.frame_bytes() as usize).map(|i| (i * 7 % 251) as u8).collect();
        std::fs::write(&to, &bytes).unwrap();
        let layers = load_layers(&p, &[format!("To={}", to.display())], spec).unwrap();
        let runner = Runner::new(fx_exec::fxrun::Device::Cpu);
        let src = vec![0u32; 64 * 32 * 4];
        let extra = Extra { host_values: time_values(Some(25), None, 12, FrameSpec { width: 64, height: 32, depth: Depth::F32 }).unwrap(), ..Extra::default() };
        // the stand-in program has no To parameter, so the run itself may refuse; the command line is what is checked
        let _ = run_fx(&runner, &p, spec, 12, src, vec![layers[0].at(12).unwrap()], &[], &p.host, extra, &Learned::default());
        let (args, files) = LAST_RUN.with(|l| l.borrow().clone()).unwrap();
        assert!(args.windows(2).any(|w| w[0] == "--ae32-src" && w[1] == "Background=layer_Background.u32"), "{:?}", args);
        let half = format!("get_percent_time=0x{:08x}", 0.5f32.to_bits());
        assert!(args.windows(2).any(|w| w[0] == "--host-value" && w[1] == half), "{:?}", args);
        let total = format!("get_total_frames=0x{:08x}", 25f32.to_bits());
        assert!(args.windows(2).any(|w| w[0] == "--host-value" && w[1] == total));
        let (_, words) = files.iter().find(|(k, _)| k == "layer_Background.u32").unwrap();
        assert_eq!(**words, frame_to_words(spec, &bytes));
        let _ = std::fs::remove_dir_all(&dir);
    }

    fn with_variants(text: &str) -> Profile {
        let mut p = profile("blur_color_autogen").unwrap().clone();
        p.variants = parse_variants(text).unwrap();
        p
    }

    const VARIANT_TEXT: &str = "cset_opacity=1:--src-planes 3|--src-head 0x7e91c0|--extra-opt 9;cset_opacity=3:--src-head 0x7ea840";

    fn val(v: &str) -> Vec<(String, String)> {
        vec![("cset_opacity".to_string(), v.to_string())]
    }

    #[test]
    fn host_variants_column_parses() {
        assert!(parse_variants("-").unwrap().is_empty());
        let v = parse_variants(VARIANT_TEXT).unwrap();
        assert_eq!(v.len(), 2);
        assert_eq!((v[0].param.as_str(), v[0].value.as_str()), ("cset_opacity", "1"));
        assert_eq!(v[0].opts, vec![("--src-planes".to_string(), "3".to_string()), ("--src-head".to_string(), "0x7e91c0".to_string()), ("--extra-opt".to_string(), "9".to_string())]);
        for bad in ["cset_opacity=1", "cset_opacity:--a 1", "=1:--a 1", "cset_opacity=1:", "cset_opacity=1:src 3"] {
            assert!(parse_variants(bad).unwrap_err().starts_with("HOST-VARIANT-MALFORMED"), "{}", bad);
        }
    }

    #[test]
    fn every_landed_row_parses_its_variants_and_the_table_checks() {
        for p in profiles() {
            assert!(p.variants_error.is_none(), "{}", p.id);
            check_variants(p).unwrap();
            // only the stages whose recorded runs differ by opacity answer carry entries, keyed on cset_opacity at the answers 1 and 3
            let keyed: Vec<(&str, &str)> = p.variants.iter().map(|v| (v.param.as_str(), v.value.as_str())).collect();
            if ["glow", "swishpan", "shake_normal_autogen", "shake_twitchy_autogen", "shake_jumpy_autogen", "dissolve_shake"].contains(&p.id.as_str()) {
                assert_eq!(keyed, [("cset_opacity", "1"), ("cset_opacity", "3")], "{}", p.id);
            } else {
                assert!(keyed.is_empty(), "{}", p.id);
            }
        }
    }

    #[test]
    fn a_layer_the_plan_does_not_read_is_dropped_with_its_class_3_image() {
        let mut a: Vec<String> = ["P", "--ae32-src", "From=a.u32", "--ae32-src", "To=b.u32", "--class3-record", "0,2,3,02", "--x", "1"].iter().map(|s| s.to_string()).collect();
        drop_source(&mut a, "To");
        assert_eq!(a.join(" "), "P --ae32-src From=a.u32 --class3-record 0,1,3,02 --x 1");
        // a source that is not there changes nothing
        let before = a.clone();
        drop_source(&mut a, "To");
        assert_eq!(a, before);
        // without a recorded allocate command only the source goes
        let mut b: Vec<String> = ["P", "--ae32-src", "To=b.u32", "--y", "2"].iter().map(|s| s.to_string()).collect();
        drop_source(&mut b, "To");
        assert_eq!(b.join(" "), "P --y 2");
    }

    /// The text the retry looks for is the one `fx_exec::srcindep::not_an_input` writes, whole: a longer name does not match its prefix, a candidate
    /// the log does not name does not match, and another refusal matches nothing.
    #[test]
    fn the_retry_matches_the_refusal_text_of_srcindep() {
        let names: Vec<String> = ["To", "From"].iter().map(|s| s.to_string()).collect();
        assert_eq!(fx_exec::srcindep::not_an_input("To"), "'ae32' source To is not an input of the compiled plan");
        assert_eq!(unread_source(&names, &fx_exec::srcindep::not_an_input("To")), Some(&names[0]));
        assert_eq!(unread_source(&names, &fx_exec::srcindep::not_an_input("From")), Some(&names[1]));
        let line = format!("ERR frame 3: {} (Dissolve_Shake)", fx_exec::srcindep::not_an_input("From"));
        assert_eq!(unread_source(&names, &line), Some(&names[1]));
        assert_eq!(unread_source(&names, &fx_exec::srcindep::not_an_input("Top")), None);
        assert_eq!(unread_source(&names, "INPUT-CLASS-ALLOC-MISMATCH (Dissolve_Shake: planned 2 image(s), elnum 3; record 1 image(s), elnum 3)"), None);
        assert_eq!(unread_source(&[], &fx_exec::srcindep::not_an_input("To")), None);
    }

    /// A clip the compiled plan does not read: fx_run refuses it with the text of `srcindep::not_an_input`, and the stage renders the frame again
    /// without the clip, to the bytes of the frame rendered without it. The program is a one-source style that has no `To` input.
    #[test]
    fn an_unread_clip_is_refused_by_fx_run_and_dropped_by_the_stage() {
        let spec = FrameSpec { width: 16, height: 8, depth: Depth::F32 };
        let p = profile("blur_color_autogen").unwrap().clone();
        let n = 16 * 8 * 4;
        let src: Vec<u32> = (0..n).map(|i| (0.05f32 + (i % 31) as f32 / 40.0).to_bits()).collect();
        let runner = Runner::new(fx_exec::fxrun::Device::Cpu);
        // one Learned for both calls: the first one learns the derived do_postdiv, so the second one's first command is refused for the clip alone
        let learned = Learned::default();
        let plain = run_fx(&runner, &p, spec, 0, src.clone(), Vec::new(), &[], &p.host, Extra::default(), &learned).expect("the frame without the clip");
        let with = run_fx(&runner, &p, spec, 0, src.clone(), vec![("To".to_string(), vec![0u32; n])], &[], &p.host, Extra::default(), &learned).expect("the clip is dropped");
        assert_eq!(plain, with, "the frame rendered after the drop equals the frame rendered without the clip");
        // the command the second call ran first still carries the clip: fx_run refuses it with the shared text
        let (args, files) = LAST_RUN.with(|l| l.borrow().clone()).unwrap();
        assert!(args.windows(2).any(|w| w[0] == "--ae32-src" && w[1] == "To=layer_To.u32"), "{:?}", args);
        let mut io = StageIo::default();
        for (k, v) in &files {
            io.words.insert(k.clone(), v.clone());
        }
        let (code, io) = runner.run(args, io);
        assert_eq!(code, 1, "{}", io.log);
        assert!(io.log.contains(&fx_exec::srcindep::not_an_input("To")), "{}", io.log);
        let names = vec!["To".to_string()];
        assert_eq!(unread_source(&names, &io.log), Some(&names[0]));
    }

    /// Dissolve_Shake through the stage's own row on a 3 frame clip: every frame is rendered. The command run first hands the plan both clips: the
    /// first frame's plan does not read the second clip (fx_run refuses it with the text for `To`), the middle frame's plan reads both, and the last
    /// frame's plan, the transition being over, does not read the first clip (the text for `From`).
    #[test]
    fn a_transition_frame_is_rendered_without_the_clip_its_plan_does_not_read() {
        let spec = FrameSpec { width: 16, height: 8, depth: Depth::F32 };
        let p = profile("dissolve_shake").unwrap().clone();
        let n = 16 * 8 * 4;
        let clip = |seed: usize| -> Vec<u32> { (0..n).map(|i| (0.05f32 + ((i * 7 + seed) % 31) as f32 / 40.0).to_bits()).collect() };
        let runner = Runner::new(fx_exec::fxrun::Device::Cpu);
        let learned = Learned::default();
        let rate = crate::rate::Rate::parse("24").unwrap();
        let mut seen: Vec<(u64, i32, String)> = Vec::new();
        // frame 0 twice: the first run learns the derived do_postdiv, so that the command of the second one is refused for a clip alone
        for f in [0u64, 0, 1, 2] {
            let extra = Extra { host_values: time_values(Some(3), Some(&rate), f, spec).unwrap(), ..Extra::default() };
            let out = run_fx(&runner, &p, spec, f, clip(1), vec![("To".to_string(), clip(2))], &[], &host_at(&p, &[]), extra, &learned);
            assert!(out.is_ok(), "frame {}: {:?}", f, out.err().map(|e| e.message));
            let (args, files) = LAST_RUN.with(|l| l.borrow().clone()).unwrap();
            let mut io = StageIo::default();
            for (k, v) in &files {
                io.words.insert(k.clone(), v.clone());
            }
            let (code, io) = runner.run(args, io);
            let names: Vec<&str> = ["From", "To"].into_iter().filter(|nm| io.log.contains(&fx_exec::srcindep::not_an_input(nm))).collect();
            seen.push((f, code, names.join("+")));
        }
        let want = vec![(0, 2, String::new()), (0, 1, "To".to_string()), (1, 0, String::new()), (2, 1, "From".to_string())];
        assert_eq!(seen, want);
    }

    #[test]
    fn variant_selection_replaces_adds_and_falls_back() {
        let p = with_variants(VARIANT_TEXT);
        let base = p.host.clone();
        // unbound, an unlisted value and the default value run the host column
        assert_eq!(host_at(&p, &[]), base);
        assert_eq!(host_at(&p, &val("2")), base);
        assert_eq!(host_at(&p, &val("4")), base);
        let one = host_at(&p, &val("1"));
        let planes = base.iter().position(|(k, _)| k == "--src-planes").unwrap();
        assert_eq!(one[planes], ("--src-planes".to_string(), "3".to_string()));
        assert_eq!(one.iter().find(|(k, _)| k == "--src-head").unwrap().1, "0x7e91c0");
        assert_eq!(one.last().unwrap(), &("--extra-opt".to_string(), "9".to_string()));
        assert_eq!(one.len(), base.len() + 1);
        // an option the entry does not name keeps the host column's value
        assert_eq!(one.iter().find(|(k, _)| k == "--ae32-dest").unwrap(), base.iter().find(|(k, _)| k == "--ae32-dest").unwrap());
        let three = host_at(&p, &val("3"));
        assert_eq!(three.iter().find(|(k, _)| k == "--src-head").unwrap().1, "0x7ea840");
        assert_eq!(three.len(), base.len());
        assert_eq!(three.iter().find(|(k, _)| k == "--src-planes"), base.iter().find(|(k, _)| k == "--src-planes"));
        // numeric spelling does not matter
        assert_eq!(host_at(&p, &val("1.0")), one);
    }

    #[test]
    fn a_keyed_selecting_parameter_gives_the_options_of_each_frame_value() {
        let p = with_variants(VARIANT_TEXT);
        let rate = crate::rate::Rate::parse("24").unwrap();
        let keyed = Keyed::new(&p, &val("1@0;3@2;2@4"), Some(&rate)).unwrap();
        for (f, plain) in [(0u64, "1"), (1, "1"), (2, "3"), (3, "3"), (4, "2"), (5, "2")] {
            assert_eq!(host_at(&p, &keyed.values_at(f)), host_at(&p, &val(plain)), "frame {}", f);
        }
    }

    #[test]
    fn invalid_variant_tables_are_refused_by_name() {
        let named = |p: &Profile, name: &str| {
            let e = check_variants(p).unwrap_err();
            assert_eq!(e.exit_code(), 3, "{}", e.message);
            assert!(e.message.contains(name), "{} in {}", name, e.message);
        };
        check_variants(&with_variants(VARIANT_TEXT)).unwrap();
        named(&with_variants("no_such=1:--a 1"), "HOST-VARIANT-PARAM-UNKNOWN");
        named(&with_variants("cset_opacity=1:--a 1;cset_opacity=1.0:--a 2"), "HOST-VARIANT-DUPLICATE");
        for opt in ["--do-postdiv 1", "--owner-opacity 1", "--cset Opacity=2", "--set a=1", "--host-param a=1", "--prior-fill 0"] {
            named(&with_variants(&format!("cset_opacity=1:{}", opt)), "HOST-VARIANT-OPTION");
        }
        for opt in ["--host-frame 3", "--ctx-time 3", "--geom 1", "--host-value get_time=0", "--ns-default-unbound x", "--size 4x4", "--ae32-src a=b", "--ae32-out o", "--host-times 1"] {
            named(&with_variants(&format!("cset_opacity=1:{}", opt)), "HOST-VARIANT-OPTION");
        }
        let mut fixed = with_variants("cset_opacity=1:--tw-route 1,2");
        fixed.fixed.push(("--tw-route".to_string(), "9".to_string()));
        named(&fixed, "HOST-VARIANT-OPTION");
        let mut empty = with_variants("cset_opacity=1:--a 1");
        empty.variants[0].opts.push(("--src-planes".to_string(), String::new()));
        named(&empty, "HOST-VARIANT-MALFORMED");
        let mut p = with_variants("cset_opacity=1:--xy-base 4");
        p.placement = vec![("64x32".to_string(), vec![("--xy-base".to_string(), "0".to_string())])];
        named(&p, "HOST-VARIANT-PLACEMENT");
        let mut p = with_variants("cset_opacity=1:--a 1;hue=2:--a 2");
        p.params.push(Param { id: "hue".into(), form: "--set".into(), name: "hue".into(), kind: "float".into(), lanes: 1, ptype: String::new(), default: String::new(), answer_default: String::new(), min: None, max: None });
        named(&p, "HOST-VARIANT-PARAMS");
        let mut p = with_variants("cset_opacity=2:--a 1");
        p.params.iter_mut().find(|q| q.id == "cset_opacity").unwrap().answer_default = "2".to_string();
        named(&p, "HOST-VARIANT-AT-DEFAULT");
        let mut p = profile("blur_color_autogen").unwrap().clone();
        p.variants_error = Some("HOST-VARIANT-MALFORMED: x".to_string());
        named(&p, "HOST-VARIANT-MALFORMED");
    }

    #[test]
    fn cset_selector_gets_its_registered_answer_default() {
        let row = profile("blur_color_autogen").expect("row");
        let q = row.params.iter().find(|q| q.id == "cset_opacity").expect("cset_opacity");
        assert_eq!(q.answer_default, "2");
        // the parameter definition the stage offers is unchanged
        assert!(q.default.is_empty());
        let keyed_at_default = |ans: &str| {
            let mut p = row.clone();
            p.variants = parse_variants(&format!("cset_opacity={}:--src-planes 3", ans)).unwrap();
            check_variants(&p)
        };
        let e = keyed_at_default("2").unwrap_err();
        assert_eq!(e.exit_code(), 3, "{}", e.message);
        assert!(e.message.contains("HOST-VARIANT-AT-DEFAULT"), "{}", e.message);
        let e = keyed_at_default("2.0").unwrap_err();
        assert!(e.message.contains("HOST-VARIANT-AT-DEFAULT"), "{}", e.message);
        keyed_at_default("1").unwrap();
        keyed_at_default("3").unwrap();
        // every offered --cset selector of every row carries a registered answer
        for p in profiles() {
            for q in p.params.iter().filter(|q| q.form == "--cset") {
                assert!(!q.answer_default.is_empty(), "{} {}", p.id, q.id);
            }
        }
    }

    #[test]
    fn listed_cset_default_is_the_registered_answer() {
        let defs = definitions();
        let mut seen = 0;
        for p in profiles() {
            let def = defs.iter().find(|d| d.id == p.id).expect("def");
            for q in p.params.iter().filter(|q| q.form == "--cset") {
                let listed = def.params.iter().find(|d| d.id == q.id).expect("param");
                match &listed.kind {
                    ParamKind::Int { default, .. } => assert_eq!(default.to_string(), q.answer_default, "{} {}", p.id, q.id),
                    _ => panic!("{} {} is not an int", p.id, q.id),
                }
                seen += 1;
            }
        }
        assert!(seen > 0);
        let def = defs.iter().find(|d| d.id == "time_warp_rgb_autogen").expect("def");
        let out = def.params.iter().find(|d| d.id == "cset_output_opacity").expect("param");
        assert!(matches!(out.kind, ParamKind::Int { default: 1, .. }));
    }

    #[test]
    fn listed_cset_range_is_one_to_the_registered_item_count() {
        let defs = definitions();
        let mut seen = 0;
        for p in profiles() {
            let def = defs.iter().find(|d| d.id == p.id).expect("def");
            for q in p.params.iter().filter(|q| q.form == "--cset") {
                // the popup item counts of the registrations, fixed here: the Opacity and Input Opacity popups hold 3 items, the Output Opacity popup 2
                let n: i64 = match q.id.as_str() {
                    "cset_opacity" | "cset_input_opacity" => 3,
                    "cset_output_opacity" => 2,
                    other => panic!("{} {}: no expected item count", p.id, other),
                };
                let listed = def.params.iter().find(|d| d.id == q.id).expect("param");
                match &listed.kind {
                    ParamKind::Int { default, min, max } => {
                        assert_eq!((*min, *max), (1, n), "{} {}", p.id, q.id);
                        assert!(*default >= *min && *default <= *max, "{} {}", p.id, q.id);
                    }
                    _ => panic!("{} {} is not an int", p.id, q.id),
                }
                // the runner's own count agrees with the fixed one
                assert_eq!(fx_exec::fxrun::registered_answer_count(&p.program, &q.name), Some(n as i32), "{} {}", p.id, q.id);
                seen += 1;
            }
        }
        assert_eq!(seen, 18);
    }

    #[test]
    fn variant_options_reach_fx_run() {
        let spec = FrameSpec { width: 64, height: 32, depth: Depth::U8 };
        let p = with_variants(VARIANT_TEXT);
        let runner = Runner::new(fx_exec::fxrun::Device::Cpu);
        let _ = run_fx(&runner, &p, spec, 0, vec![0u32; 64 * 32 * 4], Vec::new(), &[], &host_at(&p, &val("1")), Extra::default(), &Learned::default());
        let (args, _) = LAST_RUN.with(|l| l.borrow().clone()).unwrap();
        assert!(args.windows(2).any(|w| w[0] == "--src-head" && w[1] == "0x7e91c0"), "{:?}", args);
        assert_eq!(args.iter().filter(|a| *a == "--src-head").count(), 1);
        assert!(args.windows(2).any(|w| w[0] == "--extra-opt" && w[1] == "9"));
    }
}

#[cfg(test)]
mod popup_default_tests {
    use super::*;

    fn test_profile(program: &str, name: &str) -> Profile {
        Profile {
            id: "t".into(),
            program: program.into(),
            family: "t".into(),
            kind: "frame".into(),
            spool: false,
            layers: vec![],
            src_name: "From".into(),
            fixed: vec![],
            host: vec![],
            prior_fill: 0,
            placement: vec![],
            params: vec![Param {
                id: name.into(),
                form: "--set".into(),
                name: name.into(),
                kind: String::new(),
                lanes: 1,
                ptype: String::new(),
                default: String::new(),
                answer_default: String::new(),
                min: None,
                max: None,
            }],
            variants: vec![],
            variants_error: None,
        }
    }

    #[test]
    fn manifest_default_of_an_unbound_popup_is_the_registered_default() {
        let mut ps = vec![test_profile("SwishPan", "auto_trans"), test_profile("SwishPan", "wipe_amt")];
        let described = describe_programs(&ps).expect("describe");
        enrich(&mut ps, &described);
        assert_eq!(ps[0].params[0].ptype, "popup");
        assert_eq!(ps[0].params[0].default, "1");
        assert_eq!(fx_exec::fxrun::popup_manifest_default("SwishPan", "auto_trans").as_deref(), Some("1"));
        // a scalar keeps the range column's default
        assert_eq!(ps[1].params[0].default, "0");
        // a popup whose registered default equals its text default keeps it
        let mut qs = vec![test_profile("SwishPan", "transition_dir")];
        let d2 = describe_programs(&qs).unwrap();
        enrich(&mut qs, &d2);
        assert_eq!(qs[0].params[0].default, "0");
    }

    #[test]
    fn describe_params_keeps_the_range_column_default() {
        let doc = fx_exec::fxrun::describe_params_json(&["SwishPan"]);
        assert!(doc.contains("\"name\":\"auto_trans\",\"type\":\"popup\",\"lanes\":1,\"min\":0,\"max\":1,\"default\":\"0\""), "{}", doc);
    }

    #[test]
    fn time_warp_route_duration_is_the_clip_length() {
        assert_eq!(fill_fixed("--tw-route", "N,0,1,1,0,0,1", 64, Some(250)), "250,0,1,1,0,0,1");
        assert_eq!(fill_fixed("--tw-route", "N,0,1,1,0,0,1", 64, Some(12)), "12,0,1,1,0,0,1");
        assert_eq!(fill_fixed("--tw-route", "N,0,1,1,0,0,1", 64, None), "N,0,1,1,0,0,1");
        assert_eq!(fill_fixed("--palpha-cells", "0,W", 64, Some(250)), "0,64");
        assert_eq!(fill_fixed("--set", "N", 64, Some(250)), "N");
        let p = profile("time_warp_rgb_autogen").expect("stage");
        let route = p.fixed.iter().find(|(k, _)| k == "--tw-route").expect("route");
        assert!(route.1.starts_with("N,"), "{}", route.1);
    }
}
