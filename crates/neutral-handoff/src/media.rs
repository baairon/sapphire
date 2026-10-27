//! [`Info`]: what `ffprobe` says a file is, read the way `tools/apply.py` reads it.
//!
//! `neutral_ffmpeg::probe` reads the fields a media plugin needs; the wrapper also needs the
//! colour tags, the display rotation, the cover art flag and the container's frame count, so this
//! module runs the same `ffprobe -show_streams -show_format` command as `apply.py` and reads the
//! JSON with the `neutral_ffmpeg::json` parser. Every read is guarded because ffprobe omits what
//! it cannot find.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Duration;

use neutral_ffmpeg::exec::run_capture;
use neutral_ffmpeg::json::Json;

use crate::pyjson::Out;
use crate::pytext::format_g;
use crate::rate::Rate;

/// Longest a probe may take, as in `apply.py` (`PROBE_TIMEOUT`).
pub const PROBE_TIMEOUT: Duration = Duration::from_secs(30);

/// The facts about one file.
#[derive(Clone, Debug, PartialEq)]
pub struct Info {
    /// The absolute path that was probed.
    pub path: PathBuf,
    /// A video stream that is not cover art exists.
    pub has_video: bool,
    /// An audio stream exists.
    pub has_audio: bool,
    /// Codec of the chosen video stream.
    pub vcodec: Option<String>,
    /// Codec of the first audio stream.
    pub acodec: Option<String>,
    /// Pixel format of the chosen video stream.
    pub pix_fmt: Option<String>,
    /// Container index of the chosen video stream (`-map 0:N`).
    pub video_index: Option<u32>,
    /// Position of the chosen stream among the video streams (`-map 0:v:N`).
    pub video_ordinal: Option<u32>,
    /// Coded width.
    pub width: Option<u32>,
    /// Coded height.
    pub height: Option<u32>,
    /// Display rotation in degrees: 0, 90, 180 or 270.
    pub rotation: i32,
    /// The rate rounded to three places, for display and the 0.01 fps check.
    pub fps: Option<f64>,
    /// The exact rate the pipeline runs at.
    pub rate: Option<Rate>,
    /// `r_frame_rate` and `avg_frame_rate` disagree by more than a thousandth.
    pub vfr: bool,
    /// Container duration in seconds.
    pub duration: Option<f64>,
    /// Channels of the first audio stream.
    pub channels: Option<i64>,
    /// The transfer characteristic is PQ or HLG.
    pub hdr: bool,
    /// Container size in bytes.
    pub size: Option<i64>,
    /// Container format name.
    pub container: Option<String>,
    /// Frames the container states for the video stream.
    pub nb_frames: Option<i64>,
    /// Duration of the video stream.
    pub video_duration: Option<f64>,
    /// Duration of the audio stream.
    pub audio_duration: Option<f64>,
    /// `color_space` tag, `None` when unknown or unspecified.
    pub color_space: Option<String>,
    /// `color_primaries` tag, `None` when unknown or unspecified.
    pub color_primaries: Option<String>,
    /// `color_transfer` tag, `None` when unknown or unspecified.
    pub color_transfer: Option<String>,
}

fn py_float(v: &Json) -> Option<f64> {
    match v {
        Json::Int(i) => Some(*i as f64),
        Json::Float(f) => Some(*f),
        Json::Str(s) => s.trim().parse::<f64>().ok(),
        _ => None,
    }
}

fn py_str(v: &Json) -> Option<String> {
    match v {
        Json::Str(s) => Some(s.clone()),
        Json::Int(i) => Some(i.to_string()),
        _ => None,
    }
}

fn count(v: Option<&Json>) -> Option<i64> {
    let n = match v? {
        Json::Int(i) => *i,
        Json::Str(s) => s.trim().parse::<i64>().ok()?,
        _ => return None,
    };
    (n > 0).then_some(n)
}

fn seconds(v: Option<&Json>) -> Option<f64> {
    let t = py_float(v?)?;
    (t >= 0.0).then_some(t)
}

fn truthy(v: Option<&Json>) -> bool {
    match v {
        Some(Json::Bool(b)) => *b,
        Some(Json::Int(i)) => *i != 0,
        Some(Json::Float(f)) => *f != 0.0,
        Some(Json::Str(s)) => !s.is_empty(),
        Some(Json::Array(a)) => !a.is_empty(),
        Some(Json::Object(o)) => !o.is_empty(),
        _ => false,
    }
}

fn rate_of(v: Option<&Json>) -> Option<Rate> {
    Rate::parse(&py_str(v?)?)
}

/// The display rotation of a stream in degrees: side data first, the old `rotate` tag next.
fn rotation(stream: &Json) -> i32 {
    let mut raw: Option<&Json> = None;
    if let Some(list) = stream.get("side_data_list").and_then(|l| l.as_array()) {
        for side in list {
            if let Some(r) = side.get("rotation") {
                raw = Some(r);
            }
        }
    }
    if raw.is_none() {
        raw = stream.get("tags").and_then(|t| t.get("rotate"));
    }
    match raw.and_then(py_float) {
        Some(x) if x.is_finite() => (x.round_ties_even() as i64).rem_euclid(360) as i32,
        _ => 0,
    }
}

fn tag(stream: &Json, key: &str) -> Option<String> {
    let value = stream.get(key).and_then(py_str)?;
    if value == "unknown" || value == "unspecified" {
        None
    } else {
        Some(value)
    }
}

impl Info {
    fn empty(path: PathBuf) -> Info {
        Info {
            path,
            has_video: false,
            has_audio: false,
            vcodec: None,
            acodec: None,
            pix_fmt: None,
            video_index: None,
            video_ordinal: None,
            width: None,
            height: None,
            rotation: 0,
            fps: None,
            rate: None,
            vfr: false,
            duration: None,
            channels: None,
            hdr: false,
            size: None,
            container: None,
            nb_frames: None,
            video_duration: None,
            audio_duration: None,
            color_space: None,
            color_primaries: None,
            color_transfer: None,
        }
    }

    /// Read ffprobe's JSON text for the file at `path`. `None` when it is not a JSON object.
    pub fn from_json(text: &str, path: &Path) -> Option<Info> {
        let data = Json::parse(text).ok()?;
        data.as_object()?;
        let abs = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
        let mut info = Info::empty(abs);
        let mut video_streams_seen: u32 = 0;
        for st in data.get("streams").and_then(|s| s.as_array()).unwrap_or(&[]) {
            match st.get("codec_type").and_then(|c| c.as_str()) {
                Some("video") => {
                    let ordinal = video_streams_seen;
                    video_streams_seen += 1;
                    if info.has_video || truthy(st.get("disposition").and_then(|d| d.get("attached_pic"))) {
                        continue;
                    }
                    info.has_video = true;
                    info.video_index = st.get("index").and_then(|i| i.as_u64()).map(|i| i as u32);
                    info.video_ordinal = Some(ordinal);
                    info.vcodec = st.get("codec_name").and_then(py_str);
                    info.pix_fmt = st.get("pix_fmt").and_then(py_str);
                    info.width = st.get("width").and_then(|w| w.as_u64()).map(|w| w as u32);
                    info.height = st.get("height").and_then(|h| h.as_u64()).map(|h| h as u32);
                    info.rotation = rotation(st);
                    let avg = rate_of(st.get("avg_frame_rate"));
                    let base = rate_of(st.get("r_frame_rate"));
                    info.vfr = match (avg, base) {
                        (Some(a), Some(b)) => a.differs_from(&b),
                        _ => false,
                    };
                    info.rate = if info.vfr { avg } else { base.or(avg) };
                    info.fps = info.rate.map(|r| (r.as_f64() * 1000.0).round_ties_even() / 1000.0);
                    info.color_space = tag(st, "color_space");
                    info.color_primaries = tag(st, "color_primaries");
                    info.color_transfer = tag(st, "color_transfer");
                    let trc = info.color_transfer.clone().unwrap_or_default().to_lowercase();
                    info.hdr = trc == "smpte2084" || trc == "arib-std-b67";
                    info.nb_frames = count(st.get("nb_frames"));
                    info.video_duration = seconds(st.get("duration"));
                }
                Some("audio") if !info.has_audio => {
                    info.has_audio = true;
                    info.acodec = st.get("codec_name").and_then(py_str);
                    info.channels = st.get("channels").and_then(|c| c.as_i64());
                    info.audio_duration = seconds(st.get("duration"));
                }
                _ => {}
            }
        }
        if let Some(format) = data.get("format") {
            info.container = format.get("format_name").and_then(py_str);
            info.duration = format.get("duration").and_then(py_float);
            info.size = format.get("size").and_then(py_float).map(|v| v as i64);
        }
        Some(info)
    }

    /// The frame size ffmpeg decodes to: a 90 or 270 degree rotation swaps the sides.
    pub fn display_size(&self) -> (u32, u32) {
        let (w, h) = (self.width.unwrap_or(0), self.height.unwrap_or(0));
        if self.rotation == 90 || self.rotation == 270 {
            (h, w)
        } else {
            (w, h)
        }
    }

    /// The frames the input decodes to when the container says so for a constant frame rate.
    pub fn exact_frames(&self) -> Option<u64> {
        match (self.nb_frames, self.vfr) {
            (Some(n), false) => Some(n as u64),
            _ => None,
        }
    }

    /// `apply.py`'s `describe`: duration, size, rate and codecs on one line.
    pub fn describe(&self) -> String {
        let mut bits: Vec<String> = Vec::new();
        if let Some(d) = self.duration {
            bits.push(format!("{:.2}s", d));
        }
        if self.width.is_some() && self.height.is_some() {
            let (w, h) = self.display_size();
            bits.push(format!("{}x{}", w, h));
        }
        if let Some(fps) = self.fps {
            bits.push(format!("{} fps", format_g(fps)));
        }
        if let Some(v) = &self.vcodec {
            bits.push(v.clone());
        }
        if let Some(a) = &self.acodec {
            bits.push(match self.channels {
                Some(c) if c != 0 => format!("{} {}ch", a, c),
                _ => a.clone(),
            });
        }
        if bits.is_empty() {
            "no streams".to_string()
        } else {
            bits.join(", ")
        }
    }

    /// `apply.py`'s `public(info)`: the probe as JSON, the rate as `N/D` text.
    pub fn to_json(&self) -> Out {
        Out::map(vec![
            ("path", Out::text(self.path.to_string_lossy())),
            ("has_video", Out::Bool(self.has_video)),
            ("has_audio", Out::Bool(self.has_audio)),
            ("vcodec", Out::opt_text(self.vcodec.clone())),
            ("acodec", Out::opt_text(self.acodec.clone())),
            ("pix_fmt", Out::opt_text(self.pix_fmt.clone())),
            ("video_index", Out::opt_int(self.video_index.map(|v| v as i64))),
            ("width", Out::opt_int(self.width.map(|v| v as i64))),
            ("height", Out::opt_int(self.height.map(|v| v as i64))),
            ("rotation", Out::Int(self.rotation as i64)),
            ("fps", Out::opt_float(self.fps)),
            ("rate", Out::opt_text(self.rate.map(|r| r.text()))),
            ("vfr", Out::Bool(self.vfr)),
            ("duration", Out::opt_float(self.duration)),
            ("channels", Out::opt_int(self.channels)),
            ("hdr", Out::Bool(self.hdr)),
            ("size", Out::opt_int(self.size)),
            ("container", Out::opt_text(self.container.clone())),
            ("nb_frames", Out::opt_int(self.nb_frames)),
            ("video_duration", Out::opt_float(self.video_duration)),
            ("audio_duration", Out::opt_float(self.audio_duration)),
            ("color_space", Out::opt_text(self.color_space.clone())),
            ("color_primaries", Out::opt_text(self.color_primaries.clone())),
            ("color_transfer", Out::opt_text(self.color_transfer.clone())),
        ])
    }
}

/// The arguments of the probe: `ffprobe -v quiet -print_format json -show_streams -show_format PATH`.
pub fn probe_args(path: &str) -> Vec<String> {
    ["-v", "quiet", "-print_format", "json", "-show_streams", "-show_format", path].iter().map(|s| s.to_string()).collect()
}

/// Run ffprobe on `path`. `None` when it fails, prints nothing or prints something that is not JSON.
pub fn probe(ffprobe: &Path, path: &str) -> Option<Info> {
    let args: Vec<OsString> = probe_args(path).into_iter().map(OsString::from).collect();
    let out = run_capture(ffprobe, &args, PROBE_TIMEOUT, "ffprobe", &format!("probing '{}'", path)).ok()?;
    if !out.status.success() || out.stdout.iter().all(|b| b.is_ascii_whitespace()) {
        return None;
    }
    let text = String::from_utf8_lossy(&out.stdout);
    Info::from_json(&text, Path::new(path))
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLIP: &str = r#"{
        "streams": [
            {"index": 0, "codec_type": "video", "codec_name": "h264", "pix_fmt": "yuv420p",
             "width": 160, "height": 96, "r_frame_rate": "24/1", "avg_frame_rate": "24/1",
             "duration": "0.500000", "nb_frames": "12", "color_space": "bt709",
             "color_primaries": "unknown", "color_transfer": "bt709", "disposition": {"attached_pic": 0}},
            {"index": 1, "codec_type": "audio", "codec_name": "aac", "channels": 2, "duration": "0.52"}
        ],
        "format": {"format_name": "mov,mp4,m4a,3gp,3g2,mj2", "duration": "0.521000", "size": "12345"}
    }"#;

    fn parse(text: &str) -> Info {
        Info::from_json(text, Path::new("clip.mp4")).expect("parses")
    }

    #[test]
    fn reads_the_fields_apply_py_reads() {
        let i = parse(CLIP);
        assert!(i.has_video && i.has_audio);
        assert_eq!((i.vcodec.as_deref(), i.acodec.as_deref()), (Some("h264"), Some("aac")));
        assert_eq!((i.width, i.height, i.rotation), (Some(160), Some(96), 0));
        assert_eq!(i.rate.unwrap().text(), "24/1");
        assert_eq!(i.fps, Some(24.0));
        assert!(!i.vfr);
        assert_eq!(i.nb_frames, Some(12));
        assert_eq!(i.exact_frames(), Some(12));
        assert_eq!(i.color_space.as_deref(), Some("bt709"));
        assert_eq!(i.color_primaries, None, "unknown is no tag");
        assert_eq!(i.color_transfer.as_deref(), Some("bt709"));
        assert!(!i.hdr);
        assert_eq!(i.channels, Some(2));
        assert_eq!(i.audio_duration, Some(0.52));
        assert_eq!(i.duration, Some(0.521));
        assert_eq!(i.size, Some(12345));
        assert_eq!(i.container.as_deref(), Some("mov,mp4,m4a,3gp,3g2,mj2"));
        assert_eq!((i.video_index, i.video_ordinal), (Some(0), Some(0)));
        assert!(i.path.is_absolute());
    }

    #[test]
    fn cover_art_is_skipped_and_the_ordinal_counts_it() {
        let i = parse(
            r#"{"streams": [
                {"index": 0, "codec_type": "video", "codec_name": "mjpeg", "width": 8, "height": 8,
                 "disposition": {"attached_pic": 1}},
                {"index": 1, "codec_type": "audio", "codec_name": "mp3", "channels": 2},
                {"index": 2, "codec_type": "video", "codec_name": "h264", "width": 32, "height": 16,
                 "r_frame_rate": "30/1", "avg_frame_rate": "30/1"}
            ]}"#,
        );
        assert_eq!(i.vcodec.as_deref(), Some("h264"));
        assert_eq!((i.video_index, i.video_ordinal), (Some(2), Some(1)));
        assert_eq!((i.width, i.height), (Some(32), Some(16)));
    }

    #[test]
    fn variable_rate_uses_the_average() {
        let i = parse(
            r#"{"streams": [{"index": 0, "codec_type": "video", "width": 4, "height": 4,
                "r_frame_rate": "60/1", "avg_frame_rate": "7200/137"}]}"#,
        );
        assert!(i.vfr);
        assert_eq!(i.rate.unwrap().text(), "7200/137");
        assert_eq!(i.fps, Some(52.555));
        assert_eq!(i.exact_frames(), None);
    }

    #[test]
    fn rate_falls_back_when_one_side_is_unknown() {
        let only_avg = parse(
            r#"{"streams": [{"index": 0, "codec_type": "video", "width": 4, "height": 4,
                "r_frame_rate": "0/0", "avg_frame_rate": "25/1"}]}"#,
        );
        assert_eq!(only_avg.rate.unwrap().text(), "25/1");
        assert!(!only_avg.vfr);
        let none = parse(
            r#"{"streams": [{"index": 0, "codec_type": "video", "width": 4, "height": 4,
                "r_frame_rate": "0/0", "avg_frame_rate": "0/0"}]}"#,
        );
        assert_eq!(none.rate, None);
        assert_eq!(none.fps, None);
    }

    #[test]
    fn rotation_comes_from_side_data_or_the_tag() {
        let side = parse(
            r#"{"streams": [{"index": 0, "codec_type": "video", "width": 1920, "height": 1080,
                "r_frame_rate": "30/1", "side_data_list": [{"side_data_type": "Display Matrix", "rotation": -90}]}]}"#,
        );
        assert_eq!(side.rotation, 270);
        assert_eq!(side.display_size(), (1080, 1920));
        let tag = parse(
            r#"{"streams": [{"index": 0, "codec_type": "video", "width": 640, "height": 480,
                "r_frame_rate": "30/1", "tags": {"rotate": "90"}}]}"#,
        );
        assert_eq!(tag.rotation, 90);
        assert_eq!(tag.display_size(), (480, 640));
        let half = parse(
            r#"{"streams": [{"index": 0, "codec_type": "video", "width": 2, "height": 4,
                "r_frame_rate": "30/1", "side_data_list": [{"rotation": 180.4}]}]}"#,
        );
        assert_eq!((half.rotation, half.display_size()), (180, (2, 4)));
    }

    #[test]
    fn hdr_is_pq_or_hlg() {
        for (trc, hdr) in [("smpte2084", true), ("arib-std-b67", true), ("ARIB-STD-B67", true), ("bt709", false)] {
            let i = parse(&format!(
                r#"{{"streams": [{{"index": 0, "codec_type": "video", "width": 4, "height": 4,
                    "r_frame_rate": "30/1", "color_transfer": "{}"}}]}}"#,
                trc
            ));
            assert_eq!(i.hdr, hdr, "{}", trc);
        }
    }

    #[test]
    fn counts_and_durations_ignore_junk() {
        let i = parse(
            r#"{"streams": [{"index": 0, "codec_type": "video", "width": 4, "height": 4,
                "r_frame_rate": "30/1", "nb_frames": "N/A", "duration": "N/A"}]}"#,
        );
        assert_eq!((i.nb_frames, i.video_duration), (None, None));
        let zero = parse(
            r#"{"streams": [{"index": 0, "codec_type": "video", "width": 4, "height": 4,
                "r_frame_rate": "30/1", "nb_frames": "0", "duration": "-1"}]}"#,
        );
        assert_eq!((zero.nb_frames, zero.video_duration), (None, None));
    }

    #[test]
    fn not_json_is_none() {
        assert!(Info::from_json("not json", Path::new("x")).is_none());
        assert!(Info::from_json("[]", Path::new("x")).is_none());
        let empty = Info::from_json("{}", Path::new("x")).unwrap();
        assert!(!empty.has_video && !empty.has_audio);
        assert_eq!(empty.describe(), "no streams");
    }

    #[test]
    fn describe_matches_the_wrapper() {
        let i = parse(CLIP);
        assert_eq!(i.describe(), "0.52s, 160x96, 24 fps, h264, aac 2ch");
        assert_eq!(probe_args("a.mp4").join(" "), "-v quiet -print_format json -show_streams -show_format a.mp4");
    }

    #[test]
    fn json_form_prints_the_rate_as_text() {
        let text = parse(CLIP).to_json().dumps();
        assert!(text.contains(r#""rate": "24/1""#));
        assert!(text.contains(r#""fps": 24.0"#));
        assert!(text.contains(r#""color_primaries": null"#));
    }
}
