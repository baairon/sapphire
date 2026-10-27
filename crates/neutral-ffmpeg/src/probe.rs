//! ffprobe wrapper: run it with JSON output and read the fields a media plugin needs.
//!
//! Only this subset is read, everything else in ffprobe's output is ignored:
//! per stream `codec_type`, `codec_name`, `width`, `height`, `pix_fmt`,
//! `r_frame_rate`, `avg_frame_rate`, `duration`, `nb_frames`, `sample_rate`,
//! `channels`; for the container `format_name` and `duration`. ffprobe prints
//! most numbers as strings ("duration": "5.000000"); both spellings are accepted.

use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::exec::{run_capture, DEFAULT_DEADLINE};
use crate::json::Json;
use crate::locate::Binaries;
use crate::process::{os, path_arg};

/// A fraction such as `30000/1001`. The denominator may be zero ("0/0" means unknown).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rational {
    /// Numerator.
    pub num: i64,
    /// Denominator.
    pub den: i64,
}

impl Rational {
    /// Parse `num/den` or a bare integer.
    pub fn parse(text: &str) -> Option<Rational> {
        let text = text.trim();
        match text.split_once('/') {
            Some((n, d)) => Some(Rational { num: n.trim().parse().ok()?, den: d.trim().parse().ok()? }),
            None => Some(Rational { num: text.parse().ok()?, den: 1 }),
        }
    }

    /// The value as a float, or `None` when the denominator is zero.
    pub fn to_f64(&self) -> Option<f64> {
        if self.den == 0 {
            None
        } else {
            Some(self.num as f64 / self.den as f64)
        }
    }

    /// True for `0/0`, `0/1` and other fractions that carry no rate.
    pub fn is_zero_or_unknown(&self) -> bool {
        self.num == 0 || self.den == 0
    }
}

/// Kind of a stream (`codec_type`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StreamKind {
    /// Video (including still images).
    Video,
    /// Audio.
    Audio,
    /// Subtitles.
    Subtitle,
    /// Data tracks.
    Data,
    /// Attachments such as fonts.
    Attachment,
    /// Any other value ffprobe printed.
    Other(String),
}

impl StreamKind {
    fn from_text(s: &str) -> StreamKind {
        match s {
            "video" => StreamKind::Video,
            "audio" => StreamKind::Audio,
            "subtitle" => StreamKind::Subtitle,
            "data" => StreamKind::Data,
            "attachment" => StreamKind::Attachment,
            other => StreamKind::Other(other.to_string()),
        }
    }
}

/// One stream of the file. Fields that ffprobe did not report are `None`.
#[derive(Clone, Debug, PartialEq)]
pub struct StreamInfo {
    /// Stream index inside the container.
    pub index: Option<u32>,
    /// Video, audio, ...
    pub codec_type: StreamKind,
    /// Codec short name such as `h264`.
    pub codec_name: Option<String>,
    /// Coded width in pixels.
    pub width: Option<u32>,
    /// Coded height in pixels.
    pub height: Option<u32>,
    /// Pixel format name as ffmpeg spells it.
    pub pix_fmt: Option<String>,
    /// Base frame rate.
    pub r_frame_rate: Option<Rational>,
    /// Average frame rate.
    pub avg_frame_rate: Option<Rational>,
    /// Duration in seconds.
    pub duration: Option<f64>,
    /// Number of frames when the container knows it.
    pub nb_frames: Option<u64>,
    /// Audio sample rate in Hz.
    pub sample_rate: Option<u32>,
    /// Audio channel count.
    pub channels: Option<u32>,
}

/// Container level information.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContainerInfo {
    /// Comma separated demuxer names, for example `matroska,webm`.
    pub format_name: Option<String>,
    /// Duration in seconds.
    pub duration: Option<f64>,
}

/// Everything read from one ffprobe run.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Probe {
    /// Container information.
    pub format: ContainerInfo,
    /// Streams in container order.
    pub streams: Vec<StreamInfo>,
}

impl Probe {
    /// Video streams in order.
    pub fn video_streams(&self) -> impl Iterator<Item = &StreamInfo> {
        self.streams.iter().filter(|s| s.codec_type == StreamKind::Video)
    }

    /// Audio streams in order.
    pub fn audio_streams(&self) -> impl Iterator<Item = &StreamInfo> {
        self.streams.iter().filter(|s| s.codec_type == StreamKind::Audio)
    }

    /// The first video stream.
    pub fn first_video(&self) -> Option<&StreamInfo> {
        self.video_streams().next()
    }

    /// The first audio stream.
    pub fn first_audio(&self) -> Option<&StreamInfo> {
        self.audio_streams().next()
    }

    /// Read the JSON text that `ffprobe -print_format json` produces.
    pub fn from_json_text(text: &str) -> Result<Probe> {
        parse_probe_json(text)
    }
}

fn number_f64(v: &Json) -> Option<f64> {
    match v {
        Json::Int(_) | Json::Float(_) => v.as_f64(),
        Json::Str(s) => s.trim().parse::<f64>().ok().filter(|x| x.is_finite()),
        _ => None,
    }
}

fn number_u64(v: &Json) -> Option<u64> {
    match v {
        Json::Int(_) | Json::Float(_) => v.as_u64(),
        Json::Str(s) => s.trim().parse::<u64>().ok(),
        _ => None,
    }
}

fn number_u32(v: &Json) -> Option<u32> {
    number_u64(v).and_then(|n| u32::try_from(n).ok())
}

fn text_of(v: &Json) -> Option<String> {
    v.as_str().filter(|s| *s != "N/A").map(|s| s.to_string())
}

fn rational_of(v: &Json) -> Option<Rational> {
    v.as_str().and_then(Rational::parse)
}

fn parse_stream(obj: &Json) -> Result<StreamInfo> {
    if obj.as_object().is_none() {
        return Err(Error::Probe("a stream entry is not an object".into()));
    }
    let field = |name: &str| obj.get(name);
    Ok(StreamInfo {
        index: field("index").and_then(number_u32),
        codec_type: field("codec_type")
            .and_then(|v| v.as_str())
            .map(StreamKind::from_text)
            .unwrap_or_else(|| StreamKind::Other(String::new())),
        codec_name: field("codec_name").and_then(text_of),
        width: field("width").and_then(number_u32),
        height: field("height").and_then(number_u32),
        pix_fmt: field("pix_fmt").and_then(text_of),
        r_frame_rate: field("r_frame_rate").and_then(rational_of),
        avg_frame_rate: field("avg_frame_rate").and_then(rational_of),
        duration: field("duration").and_then(number_f64),
        nb_frames: field("nb_frames").and_then(number_u64),
        sample_rate: field("sample_rate").and_then(number_u32),
        channels: field("channels").and_then(number_u32),
    })
}

/// Turn ffprobe's JSON text into a [`Probe`].
pub fn parse_probe_json(text: &str) -> Result<Probe> {
    let doc = Json::parse(text)?;
    if doc.as_object().is_none() {
        return Err(Error::Probe("the top level value is not an object".into()));
    }
    let mut probe = Probe::default();
    if let Some(format) = doc.get("format") {
        probe.format.format_name = format.get("format_name").and_then(text_of);
        probe.format.duration = format.get("duration").and_then(number_f64);
    }
    match doc.get("streams") {
        None | Some(Json::Null) => {}
        Some(Json::Array(items)) => {
            for item in items {
                probe.streams.push(parse_stream(item)?);
            }
        }
        Some(_) => return Err(Error::Probe("'streams' is not an array".into())),
    }
    Ok(probe)
}

/// Arguments that make ffprobe print just the fields this module reads, as JSON.
fn probe_args(input: &OsStr) -> Vec<OsString> {
    vec![
        os("-v"),
        os("error"),
        os("-hide_banner"),
        os("-print_format"),
        os("json"),
        os("-show_entries"),
        os(
            "format=format_name,duration:stream=index,codec_type,codec_name,width,height,pix_fmt,\
             r_frame_rate,avg_frame_rate,duration,nb_frames,sample_rate,channels",
        ),
        os("-i"),
        path_arg(input),
    ]
}

/// Run ffprobe on `path` (a file, URL or anything ffmpeg can open).
pub fn probe_file(bins: &Binaries, path: impl AsRef<Path>) -> Result<Probe> {
    probe_input(bins, path.as_ref().as_os_str(), &[])
}

/// Like [`probe_file`] with extra ffprobe input options placed before the input,
/// for example `["-f", "lavfi"]`.
pub fn probe_input(bins: &Binaries, input: &OsStr, input_args: &[OsString]) -> Result<Probe> {
    probe_input_within(bins, input, input_args, DEFAULT_DEADLINE)
}

/// Like [`probe_input`] with an explicit deadline; ffprobe is killed and
/// [`Error::Timeout`] returned when it takes longer (a stalled network input, say).
pub fn probe_input_within(
    bins: &Binaries,
    input: &OsStr,
    input_args: &[OsString],
    deadline: Duration,
) -> Result<Probe> {
    let ffprobe = bins.ffprobe_path()?;
    let mut args = probe_args(input);
    // Input options must precede "-i".
    let at = args.iter().position(|a| a == "-i").expect("probe_args contains -i");
    for (k, extra) in input_args.iter().enumerate() {
        args.insert(at + k, extra.clone());
    }
    let context = format!("probing '{}'", input.to_string_lossy());
    let out = run_capture(&ffprobe, &args, deadline, "ffprobe", &context)?;
    let stderr = out.stderr_text();
    if !out.status.success() {
        return Err(Error::Ffmpeg {
            tool: "ffprobe",
            context: format!("probe '{}'", input.to_string_lossy()),
            code: out.status.code(),
            stderr,
        });
    }
    let text = String::from_utf8_lossy(&out.stdout);
    parse_probe_json(&text)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
        "streams": [
            {"index": 0, "codec_name": "ffv1", "codec_type": "video", "width": 64, "height": 48,
             "pix_fmt": "yuv420p", "r_frame_rate": "10/1", "avg_frame_rate": "10/1",
             "duration": "1.000000", "nb_frames": "10"},
            {"index": 1, "codec_name": "pcm_s16le", "codec_type": "audio", "sample_rate": "44100",
             "channels": 1, "r_frame_rate": "0/0", "avg_frame_rate": "0/0", "duration": "1.000000"},
            {"index": 2, "codec_type": "subtitle", "codec_name": "ass", "duration": "N/A"}
        ],
        "format": {"format_name": "matroska,webm", "duration": "1.000000"}
    }"#;

    #[test]
    fn reads_the_documented_subset() {
        let p = Probe::from_json_text(SAMPLE).unwrap();
        assert_eq!(p.format.format_name.as_deref(), Some("matroska,webm"));
        assert_eq!(p.format.duration, Some(1.0));
        assert_eq!(p.streams.len(), 3);
        let v = p.first_video().unwrap();
        assert_eq!((v.index, v.width, v.height), (Some(0), Some(64), Some(48)));
        assert_eq!(v.codec_name.as_deref(), Some("ffv1"));
        assert_eq!(v.pix_fmt.as_deref(), Some("yuv420p"));
        assert_eq!(v.r_frame_rate, Some(Rational { num: 10, den: 1 }));
        assert_eq!(v.avg_frame_rate.unwrap().to_f64(), Some(10.0));
        assert_eq!(v.nb_frames, Some(10));
        assert_eq!(v.duration, Some(1.0));
        let a = p.first_audio().unwrap();
        assert_eq!((a.sample_rate, a.channels), (Some(44100), Some(1)));
        assert_eq!(a.r_frame_rate.unwrap().to_f64(), None, "0/0 has no value");
        assert!(a.r_frame_rate.unwrap().is_zero_or_unknown());
        assert_eq!(p.streams[2].codec_type, StreamKind::Subtitle);
        assert_eq!(p.streams[2].duration, None, "N/A is not a number");
        assert_eq!(p.video_streams().count(), 1);
    }

    #[test]
    fn numbers_may_be_json_numbers_or_strings() {
        let p = Probe::from_json_text(
            r#"{"streams":[{"codec_type":"video","width":"320","height":240.0,"duration":2.5,"nb_frames":7}]}"#,
        )
        .unwrap();
        let v = &p.streams[0];
        assert_eq!((v.width, v.height, v.duration, v.nb_frames), (Some(320), Some(240), Some(2.5), Some(7)));
    }

    #[test]
    fn missing_parts_are_fine() {
        let p = Probe::from_json_text("{}").unwrap();
        assert_eq!(p, Probe::default());
        let p = Probe::from_json_text(r#"{"streams": [], "format": {}}"#).unwrap();
        assert!(p.streams.is_empty());
        assert_eq!(p.format, ContainerInfo::default());
        let p = Probe::from_json_text(r#"{"streams":[{"codec_type":"data"}]}"#).unwrap();
        assert_eq!(p.streams[0].codec_type, StreamKind::Data);
        assert_eq!(p.streams[0].width, None);
    }

    #[test]
    fn malformed_documents_are_errors() {
        assert!(matches!(Probe::from_json_text("not json"), Err(Error::Json { .. })));
        assert!(matches!(Probe::from_json_text("[]"), Err(Error::Probe(_))));
        assert!(matches!(Probe::from_json_text(r#"{"streams": 3}"#), Err(Error::Probe(_))));
        assert!(matches!(Probe::from_json_text(r#"{"streams": [1]}"#), Err(Error::Probe(_))));
    }

    #[test]
    fn rational_parsing() {
        assert_eq!(Rational::parse("30000/1001"), Some(Rational { num: 30000, den: 1001 }));
        assert_eq!(Rational::parse(" 25 "), Some(Rational { num: 25, den: 1 }));
        assert_eq!(Rational::parse("0/0"), Some(Rational { num: 0, den: 0 }));
        assert_eq!(Rational::parse("a/b"), None);
        assert_eq!(Rational::parse(""), None);
        assert!((Rational::parse("30000/1001").unwrap().to_f64().unwrap() - 29.97002997).abs() < 1e-6);
    }

    #[test]
    fn input_options_go_before_the_input() {
        let mut args = probe_args(OsStr::new("x.mkv"));
        let at = args.iter().position(|a| a == "-i").unwrap();
        args.insert(at, os("-f"));
        args.insert(at + 1, os("lavfi"));
        let i = args.iter().position(|a| a == "-i").unwrap();
        assert_eq!(args[i - 2], os("-f"));
        assert_eq!(args[i + 1], os("x.mkv"));
    }
}
