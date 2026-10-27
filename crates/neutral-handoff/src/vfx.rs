//! Talking to `vfx` before a frame moves: what an effect is, whether its parameters are valid,
//! and which contract version a stage speaks.
//!
//! - [`effect_info`] runs `vfx list --json --all` (falling back to `vfx list --json`) and reads
//!   the kind and the layers of one effect, as `tools/apply.py` does. The listing may be a bare
//!   array (today's vfx) or an object `{"contract": "frames/1", "stages" or "effects": [...]}`;
//!   an object that names another major contract version is refused.
//! - [`preflight`] runs `vfx apply ... --check`, which validates the effect id, every parameter
//!   (keyframes too), the layer ids and the rate without reading or writing a frame.
//! - [`check_stage_contract`] asks a `<program> apply ...` stage for `<program> list --json` and
//!   refuses one that reports another major version; a stage that lists nothing, or a bare
//!   array, is accepted.

use std::path::Path;
use std::time::Duration;

use neutral_ffmpeg::json::Json;

use crate::error::{Code, Error, Result};
use crate::sys::run_short;
use crate::CONTRACT;

/// How long `vfx list` and `vfx apply --check` may take, as in `apply.py` (`PROBE_TIMEOUT`).
pub const LIST_TIMEOUT: Duration = Duration::from_secs(30);

/// Whether an effect works frame by frame or needs random access to the clip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// One frame in, one frame out.
    Frame,
    /// Reads other frames of the clip, extra layers and parameters sampled over time.
    Temporal,
}

impl Kind {
    /// `frame` or `temporal`.
    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Frame => "frame",
            Kind::Temporal => "temporal",
        }
    }
}

/// An extra input a temporal effect lists.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LayerSpec {
    /// The id used in `--layer ID=VIDEO`.
    pub id: String,
    /// The effect runs without it.
    pub optional: bool,
}

/// What `vfx list` says about one effect.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EffectInfo {
    /// The effect id.
    pub id: String,
    /// Frame or temporal.
    pub kind: Kind,
    /// The layers, in listing order.
    pub layers: Vec<LayerSpec>,
}

/// Check the `contract` member of a listing or result document. A document without the member
/// is accepted; one that names another scheme or major version is `unsupported`.
pub fn check_contract_value(doc: &Json, who: &str) -> Result<()> {
    let value = match doc.get("contract") {
        None => return Ok(()),
        Some(v) => v,
    };
    let text = value.as_str().unwrap_or("");
    let major = text.strip_prefix("frames/").and_then(|rest| rest.split('.').next()).and_then(|m| m.parse::<u64>().ok());
    if major == Some(1) || major == Some(2) {
        Ok(())
    } else {
        let shown = match value {
            Json::Str(s) => s.clone(),
            other => format!("{:?}", other),
        };
        Err(Error::unsupported(format!("{} reports the contract `{}`, but this tool knows `frames/1` and `{}`", who, shown, CONTRACT)))
    }
}

/// The effect entries of a parsed listing.
fn entries<'a>(doc: &'a Json, who: &str) -> Result<&'a [Json]> {
    if let Some(items) = doc.as_array() {
        return Ok(items);
    }
    if doc.as_object().is_some() {
        check_contract_value(doc, who)?;
        for key in ["stages", "effects"] {
            if let Some(items) = doc.get(key).and_then(|v| v.as_array()) {
                return Ok(items);
            }
        }
    }
    Err(Error::stage_failed(format!("`{} list --json` did not print a list of effects", who)).at("vfx"))
}

/// Find `effect_id` in the text `vfx list --json` printed.
pub fn parse_listing(text: &str, effect_id: &str) -> Result<EffectInfo> {
    let doc = Json::parse(text).map_err(|_| Error::stage_failed("`vfx list --json` did not print JSON").at("vfx"))?;
    for entry in entries(&doc, "vfx")? {
        if entry.get("id").and_then(|v| v.as_str()) == Some(effect_id) {
            let kind = match entry.get("kind").and_then(|v| v.as_str()) {
                Some("temporal") => Kind::Temporal,
                _ => Kind::Frame,
            };
            let layers = entry
                .get("layers")
                .and_then(|v| v.as_array())
                .unwrap_or(&[])
                .iter()
                .filter_map(|l| {
                    Some(LayerSpec {
                        id: l.get("id")?.as_str()?.to_string(),
                        optional: l.get("optional").and_then(|v| v.as_bool()).unwrap_or(false),
                    })
                })
                .collect();
            return Ok(EffectInfo { id: effect_id.to_string(), kind, layers });
        }
    }
    Err(Error::unsupported(format!("unknown effect `{}`, `vfx list` shows the registered ones", effect_id)).at("vfx"))
}

/// Ask `vfx` about `effect_id`.
pub fn effect_info(vfx: &Path, effect_id: &str) -> Result<EffectInfo> {
    let mut last = None;
    for extra in [Some("--all"), None] {
        let mut args = vec!["list".to_string(), "--json".to_string()];
        args.extend(extra.map(|s| s.to_string()));
        let ran = run_short(vfx, &args, LIST_TIMEOUT);
        if ran.code == 127 {
            return Err(Error::tool_missing(format!("cannot run vfx at {}", vfx.display())).at("vfx"));
        }
        if ran.code == 0 {
            return parse_listing(&ran.stdout, effect_id);
        }
        last = Some(ran);
    }
    let ran = last.expect("the loop ran at least once");
    let message = match ran.stderr.trim().lines().last() {
        Some(line) => line.to_string(),
        None => format!("exit code {}", ran.code),
    };
    Err(Error::stage_failed(format!("`vfx list --json` failed: {}", message)).at("vfx"))
}

/// `vfx apply EFFECT ... --check` with the real size, depth, rate, parameters and layers.
///
/// `layers` are `(id, video path)` pairs, passed only for temporal effects.
#[allow(clippy::too_many_arguments)]
pub fn preflight(
    vfx: &Path,
    effect: &str,
    width: u32,
    height: u32,
    depth_bits: u32,
    rate_text: &str,
    layers: &[(String, String)],
    params: &[String],
) -> Result<()> {
    let mut args: Vec<String> = ["apply", effect, "--width", &width.to_string(), "--height", &height.to_string(), "--depth", &depth_bits.to_string(), "--fps", rate_text, "--check"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    for (id, video) in layers {
        args.push("--layer".into());
        args.push(format!("{}={}", id, video));
    }
    for p in params {
        args.push("--param".into());
        args.push(p.clone());
    }
    let ran = run_short(vfx, &args, LIST_TIMEOUT);
    if ran.code == 127 {
        return Err(Error::tool_missing(format!("cannot run vfx at {}", vfx.display())).at("vfx"));
    }
    if ran.code == 0 {
        return Ok(());
    }
    let tail = crate::error::tail_lines(&ran.stderr, 3);
    let message = match ran.stderr.trim().lines().last() {
        Some(line) => line.replace("vfx: error: ", "vfx: "),
        None => format!("vfx exited with code {}", ran.code),
    };
    let code = if ran.code == 124 { Code::StageTimeout } else { Code::from_stage_exit(ran.code) };
    Err(Error::new(code, message).at("vfx").with_tail(tail).with_exit(Some(ran.code)))
}

/// For a stage command of the form `<program> apply ...`, run `<program> list --json` and refuse
/// a contract version other than 1. Anything else (a program that cannot list, a bare array, no
/// `contract` member) is accepted: the check never makes a working stage fail.
pub fn check_stage_contract(argv: &[String], stage: &str) -> Result<()> {
    if argv.len() < 2 || argv[1] != "apply" {
        return Ok(());
    }
    let ran = run_short(Path::new(&argv[0]), &["list".to_string(), "--json".to_string()], Duration::from_secs(10));
    if ran.code != 0 {
        return Ok(());
    }
    match Json::parse(&ran.stdout) {
        Ok(doc) if doc.as_object().is_some() => check_contract_value(&doc, stage).map_err(|e| e.at(stage)),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LISTING: &str = r#"[
        {"id": "retime", "kind": "temporal", "hidden": false, "layers": [
            {"id": "alt_source_a", "name": "A", "optional": true}, {"id": "alt_source_b", "name": "B", "optional": false}]},
        {"id": "selftest-invert", "kind": "frame", "hidden": true, "layers": []},
        {"id": "old", "layers": []}
    ]"#;

    #[test]
    fn listing_gives_kind_and_layers() {
        let retime = parse_listing(LISTING, "retime").unwrap();
        assert_eq!(retime.kind, Kind::Temporal);
        assert_eq!(
            retime.layers,
            vec![LayerSpec { id: "alt_source_a".into(), optional: true }, LayerSpec { id: "alt_source_b".into(), optional: false }]
        );
        assert_eq!(parse_listing(LISTING, "selftest-invert").unwrap().kind, Kind::Frame);
        assert_eq!(parse_listing(LISTING, "old").unwrap().kind, Kind::Frame, "a vfx that predates kinds lists frame effects");
    }

    #[test]
    fn an_unknown_effect_is_unsupported() {
        let err = parse_listing(LISTING, "nope").unwrap_err();
        assert_eq!(err.code, Code::Unsupported);
        assert_eq!(err.message, "unknown effect `nope`, `vfx list` shows the registered ones");
        assert_eq!(err.exit_code(), 3);
    }

    #[test]
    fn junk_is_not_json() {
        let err = parse_listing("not json", "retime").unwrap_err();
        assert_eq!(err.message, "`vfx list --json` did not print JSON");
        let err = parse_listing("3", "retime").unwrap_err();
        assert!(err.message.contains("did not print a list of effects"), "{}", err.message);
    }

    #[test]
    fn an_object_listing_with_a_contract_is_read() {
        let text = r#"{"contract": "frames/1", "stages": [{"id": "a", "kind": "frame", "layers": []}]}"#;
        assert_eq!(parse_listing(text, "a").unwrap().kind, Kind::Frame);
        let minor = r#"{"contract": "frames/1.4", "effects": [{"id": "a", "kind": "temporal", "layers": []}]}"#;
        assert_eq!(parse_listing(minor, "a").unwrap().kind, Kind::Temporal);
    }

    #[test]
    fn another_major_version_is_refused() {
        for bad in ["frames/3", "frames/0", "frames/", "frames", "other/1", ""] {
            let text = format!(r#"{{"contract": "{}", "stages": [{{"id": "a"}}]}}"#, bad);
            let err = parse_listing(&text, "a").unwrap_err();
            assert_eq!(err.code, Code::Unsupported, "{}", bad);
            assert!(err.message.contains(&format!("`{}`", bad)), "{}", err.message);
            assert!(err.message.contains("this tool knows `frames/1` and `frames/2`"));
        }
        let two = r#"{"contract": "frames/2", "stages": [{"id": "a", "kind": "frame", "layers": []}]}"#;
        assert_eq!(parse_listing(two, "a").unwrap().kind, Kind::Frame, "a frames/2 manifest is read");
        let numeric = parse_listing(r#"{"contract": 1, "stages": []}"#, "a").unwrap_err();
        assert_eq!(numeric.code, Code::Unsupported);
    }

    #[test]
    fn a_stage_without_apply_is_not_asked() {
        assert!(check_stage_contract(&["no-such-program".into(), "run".into()], "stage1").is_ok());
        assert!(check_stage_contract(&["no-such-program".into()], "stage1").is_ok());
        assert!(check_stage_contract(&["no-such-program".into(), "apply".into()], "stage1").is_ok(), "a program that cannot list is accepted");
    }
}
