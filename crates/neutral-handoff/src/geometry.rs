//! Declared geometry (`docs/handoff.md#geometry`, `contracts/describe.schema.json`).
//!
//! A stage manifest gives every stage a `geometry` value. `preserve`: the output frames have the
//! width, height, rate and format of the input. `declared`: the stage answers
//! `apply STAGE --describe --width W --height H --depth D --fps N/D [--frames N] ...` without
//! reading standard input, with one JSON document that names its output width, height and frame
//! count. Depth and rate never change: an answer with another rate is `unsupported`. Any other
//! value is refused (`unsupported`).
//!
//! The wrapper asks every declared stage in chain order, feeds each answer into the next
//! question and hands the last one to encode and verify ([`crate::plan`]).

use std::path::Path;
use std::time::Duration;

use neutral_ffmpeg::json::Json;

use crate::error::{tail_lines, Code, Error, Result};
use crate::format::Depth;
use crate::pyjson::Out;
use crate::rate::Rate;
use crate::sys::run_short;

/// The geometry values this tool knows.
pub const KNOWN: [&str; 2] = ["preserve", "declared"];

/// How long a `list --json` or `--describe` question may take.
pub const ASK_TIMEOUT: Duration = Duration::from_secs(30);

/// One `--describe` answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Answer {
    /// The stage id that answered.
    pub stage: String,
    /// Output width in pixels.
    pub width: u32,
    /// Output height in pixels.
    pub height: u32,
    /// Bits per channel, equal to the request.
    pub depth: Depth,
    /// The rate, equal to the request.
    pub fps: Rate,
    /// Output frame count, `None` when the stage cannot tell.
    pub frames: Option<u64>,
}

impl Answer {
    /// The answer as the JSON document of `contracts/describe.schema.json`.
    pub fn to_json(&self) -> Out {
        Out::map(vec![
            ("ok", Out::Bool(true)),
            ("contract", Out::text("frames/2")),
            ("stage", Out::text(self.stage.clone())),
            ("width", Out::Int(self.width as i64)),
            ("height", Out::Int(self.height as i64)),
            ("depth", Out::Int(self.depth.bits() as i64)),
            ("fps", Out::text(self.fps.text())),
            ("frames", Out::opt_int(self.frames.map(|n| n as i64))),
        ])
    }
}

/// Where the stage id sits in a stage command of the form `<program> [words] apply ID ...`:
/// the words between the program and `apply` (for example `stage`) and the id.
pub fn apply_form(argv: &[String]) -> Option<(Vec<String>, String)> {
    let at = argv.iter().take(3).skip(1).position(|a| a == "apply")? + 1;
    let id = argv.get(at + 1).filter(|id| !id.starts_with('-'))?;
    Some((argv[1..at].to_vec(), id.clone()))
}

/// The `geometry` of the stage a command runs, read from `<program> [words] list --json` (which
/// is also checked for a contract this tool knows, see [`crate::vfx::check_contract_value`]).
/// `None` when the command has no `apply ID` form, the program cannot list, the listing is a bare
/// array (vfx), or the stage has no `geometry` key: the stage is then taken as `preserve`.
pub fn listed_geometry(argv: &[String], stage: &str) -> Result<Option<String>> {
    let Some((words, id)) = apply_form(argv) else { return Ok(None) };
    let mut args = words;
    args.extend(["list".to_string(), "--json".to_string()]);
    let ran = run_short(Path::new(&argv[0]), &args, ASK_TIMEOUT);
    if ran.code != 0 {
        return Ok(None);
    }
    let doc = match Json::parse(&ran.stdout) {
        Ok(doc) if doc.as_object().is_some() => doc,
        _ => return Ok(None),
    };
    crate::vfx::check_contract_value(&doc, stage).map_err(|e| e.at(stage))?;
    Ok(geometry_of(&doc, &id))
}

/// The `geometry` of stage `id` in a stage manifest.
pub fn geometry_of(doc: &Json, id: &str) -> Option<String> {
    let stages = doc.get("stages").and_then(|s| s.as_array())?;
    let entry = stages.iter().find(|s| s.get("id").and_then(|v| v.as_str()) == Some(id))?;
    entry.get("geometry").and_then(|g| g.as_str()).map(|g| g.to_string())
}

/// Ask a declared stage for its output geometry: `argv --describe [--frames N]`, no standard
/// input. A refusal keeps the stage exit codes (2 usage, 3 unsupported, 4 bad input).
pub fn ask_describe(argv: &[String], frames: Option<u64>, stage: &str, depth: Depth, rate: &Rate) -> Result<Answer> {
    let mut args: Vec<String> = argv[1..].to_vec();
    args.push("--describe".into());
    if let Some(n) = frames {
        if !args.iter().any(|a| a == "--frames" || a.starts_with("--frames=")) {
            args.extend(["--frames".to_string(), n.to_string()]);
        }
    }
    let ran = run_short(Path::new(&argv[0]), &args, ASK_TIMEOUT);
    if ran.code == 124 {
        return Err(Error::new(Code::StageTimeout, format!("{} did not answer --describe within {} seconds", stage, ASK_TIMEOUT.as_secs())).at(stage));
    }
    if ran.code == 127 {
        return Err(Error::tool_missing(format!("cannot start `{}`", argv[0])).at(stage));
    }
    if ran.code != 0 {
        return Err(Error::stage_exit(stage, Some(ran.code), tail_lines(&ran.stderr, 3)));
    }
    parse_answer(&ran.stdout, stage, depth, rate)
}

/// Read and check a `--describe` answer against the request.
pub fn parse_answer(text: &str, stage: &str, depth: Depth, rate: &Rate) -> Result<Answer> {
    let bad = |what: &str| Error::stage_failed(format!("{} answered --describe with {}", stage, what)).at(stage);
    let doc = Json::parse(text.trim()).map_err(|_| bad("something that is not JSON"))?;
    if doc.get("ok").and_then(|v| v.as_bool()) != Some(true) || doc.get("contract").and_then(|v| v.as_str()) != Some("frames/2") {
        return Err(bad("a document that is not a frames/2 answer"));
    }
    let side = |key: &str| doc.get(key).and_then(|v| v.as_u64()).filter(|n| *n >= 1 && *n <= u32::MAX as u64).map(|n| n as u32);
    let (width, height) = match (side("width"), side("height")) {
        (Some(w), Some(h)) => (w, h),
        _ => return Err(bad("no positive width and height")),
    };
    let answered_depth = doc.get("depth").and_then(|v| v.as_u64()).and_then(|d| Depth::from_bits(d as u32));
    if answered_depth != Some(depth) {
        return Err(Error::unsupported(format!("{} would change the depth; only width, height and the frame count may change", stage)).at(stage));
    }
    let fps = doc.get("fps").and_then(|v| v.as_str()).and_then(Rate::parse).ok_or_else(|| bad("no exact fps"))?;
    if fps.num() * rate.den() != rate.num() * fps.den() {
        return Err(Error::unsupported(format!(
            "{} would change the frame rate from {} to {}; a rate change is not supported",
            stage,
            rate.text(),
            fps.text()
        ))
        .at(stage));
    }
    let frames = match doc.get("frames") {
        None => return Err(bad("no frames key")),
        Some(v) if v.is_null() => None,
        Some(v) => Some(v.as_u64().ok_or_else(|| bad("a frame count that is not a whole number"))?),
    };
    let id = doc.get("stage").and_then(|v| v.as_str()).unwrap_or(stage).to_string();
    Ok(Answer { stage: id, width, height, depth, fps: *rate, frames })
}

/// Whether this tool knows the geometry `value`.
pub fn is_known(value: &str) -> bool {
    KNOWN.contains(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rate() -> Rate {
        Rate::parse("30000/1001").unwrap()
    }

    fn a(list: &[&str]) -> Vec<String> {
        list.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_apply_form_finds_the_stage_id() {
        assert_eq!(apply_form(&a(&["vfx", "apply", "retime", "--width", "2"])), Some((vec![], "retime".to_string())));
        assert_eq!(apply_form(&a(&["nh", "stage", "apply", "ffmpeg-filter"])), Some((vec!["stage".to_string()], "ffmpeg-filter".to_string())));
        assert_eq!(apply_form(&a(&["tool", "run", "x"])), None);
        assert_eq!(apply_form(&a(&["tool", "apply", "--width"])), None);
    }

    #[test]
    fn an_answer_is_checked_against_the_request() {
        let ok = r#"{"ok": true, "contract": "frames/2", "stage": "ffmpeg-filter", "width": 80, "height": 48, "depth": 8, "fps": "30000/1001", "frames": 12}"#;
        let answer = parse_answer(ok, "post1", Depth::U8, &rate()).unwrap();
        assert_eq!((answer.width, answer.height, answer.frames), (80, 48, Some(12)));
        assert_eq!(answer.to_json().dumps(), ok);
        let unknown = ok.replace("12}", "null}");
        assert_eq!(parse_answer(&unknown, "post1", Depth::U8, &rate()).unwrap().frames, None);
        let other_rate = ok.replace("30000/1001", "60/1");
        let e = parse_answer(&other_rate, "post1", Depth::U8, &rate()).unwrap_err();
        assert_eq!((e.code, e.stage.as_deref()), (Code::Unsupported, Some("post1")));
        assert_eq!(parse_answer(ok, "post1", Depth::U16, &rate()).unwrap_err().code, Code::Unsupported);
        assert_eq!(parse_answer("nope", "post1", Depth::U8, &rate()).unwrap_err().code, Code::StageFailed);
        assert_eq!(parse_answer(&ok.replace("\"width\": 80", "\"width\": 0"), "post1", Depth::U8, &rate()).unwrap_err().code, Code::StageFailed);
    }

    #[test]
    fn the_geometry_of_a_manifest_stage_is_read() {
        let doc = Json::parse(r#"{"contract": "frames/2", "stages": [{"id": "a", "geometry": "declared"}, {"id": "b"}]}"#).unwrap();
        assert_eq!(geometry_of(&doc, "a").as_deref(), Some("declared"));
        assert_eq!(geometry_of(&doc, "b"), None);
        assert!(is_known("preserve") && is_known("declared") && !is_known("warp"));
    }

    #[test]
    fn a_program_that_cannot_list_is_taken_as_preserve() {
        assert_eq!(listed_geometry(&a(&["no-such-program-xyz", "apply", "x"]), "pre1").unwrap(), None);
        assert_eq!(listed_geometry(&a(&["no-such-program-xyz", "run"]), "pre1").unwrap(), None);
    }
}
