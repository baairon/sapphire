//! Checking a finished output against the input, the checks of `tools/apply.py` (`verify`).
//!
//! A problem is a sentence; an empty list means the output checks out:
//!
//! | Check | Rule |
//! |---|---|
//! | Video | the output has a readable video stream |
//! | Size | the input's displayed size |
//! | Rate | within 0.01 fps of the exact rate |
//! | Length, no `--frames` (or `--frames` equal to the input's own frame count) | the input's duration within one frame |
//! | Length, other `--frames N` | `N / rate` within max(one frame, 0.1 s), and exactly N frames where the container says |
//! | Audio | still there with the same codec and channel count; none added when the input had none |

use crate::media::Info;
use crate::rate::Rate;

/// An output cut at the end of the video keeps the audio up to the packet that crosses the end.
pub const TRIM_TOLERANCE: f64 = 0.1;

fn show_float(value: Option<f64>) -> String {
    match value {
        Some(v) => format!("{:?}", v),
        None => "None".to_string(),
    }
}

fn show_count(value: Option<u32>) -> String {
    value.map(|v| v.to_string()).unwrap_or_else(|| "None".to_string())
}

/// Problems found comparing `out` with `src`; empty means the output verifies.
///
/// `frames` is the `--frames` value and `source_frames` the frames the input decodes to when the
/// container says so; a `--frames` equal to the input's count compares durations as usual.
pub fn verify(src: &Info, out: Option<&Info>, rate: &Rate, frames: Option<u64>, source_frames: Option<u64>) -> Vec<String> {
    verify_with(src, out, rate, frames, source_frames, None, true)
}

/// [`verify`] after stages with declared geometry: `size` is the size the last stage declared
/// (instead of the source's), and `know_length` false skips the duration check when a declared
/// stage changed the frame count without saying by how much.
pub fn verify_with(
    src: &Info,
    out: Option<&Info>,
    rate: &Rate,
    frames: Option<u64>,
    source_frames: Option<u64>,
    size: Option<(u32, u32)>,
    know_length: bool,
) -> Vec<String> {
    let out = match out {
        Some(o) if o.has_video => o,
        _ => return vec!["the output has no readable video stream".to_string()],
    };
    let mut problems = Vec::new();
    let want = size.unwrap_or_else(|| src.display_size());
    if (out.width, out.height) != (Some(want.0), Some(want.1)) {
        problems.push(format!("size is {}x{}, expected {}x{}", show_count(out.width), show_count(out.height), want.0, want.1));
    }
    let fps = rate.as_f64();
    match out.fps {
        Some(f) if (f - fps).abs() <= 0.01 => {}
        other => problems.push(format!("frame rate is {}, expected {:.3}", show_float(other), fps)),
    }
    match frames {
        _ if !know_length => {}
        Some(n) if Some(n) != source_frames => {
            let expected = n as f64 / fps;
            let window = (1.0 / fps).max(TRIM_TOLERANCE) + 1e-3;
            match out.duration {
                None => problems.push("the output has no readable duration".to_string()),
                Some(d) if (d - expected).abs() > window => problems.push(format!(
                    "duration is {:.3}s, expected {:.3}s ({} frames at {:.3} fps) within {:.3}s",
                    d, expected, n, fps, window
                )),
                _ => {}
            }
        }
        _ => {
            if let Some(src_duration) = src.duration {
                match out.duration {
                    None => problems.push("the output has no readable duration".to_string()),
                    Some(d) if (d - src_duration).abs() > 1.0 / fps + 1e-3 => problems.push(format!(
                        "duration is {:.3}s, expected {:.3}s within one frame ({:.3}s)",
                        d,
                        src_duration,
                        1.0 / fps
                    )),
                    _ => {}
                }
            }
        }
    }
    if let (Some(n), Some(got)) = (frames, out.nb_frames) {
        if got != n as i64 {
            problems.push(format!("the output has {} frames, expected {}", got, n));
        }
    }
    if src.has_audio {
        if !out.has_audio {
            problems.push("the audio stream was lost".to_string());
        } else if (&out.acodec, out.channels) != (&src.acodec, src.channels) {
            problems.push(format!(
                "audio is {} {}ch, expected {} {}ch",
                out.acodec.as_deref().unwrap_or("None"),
                out.channels.map(|c| c.to_string()).unwrap_or_else(|| "None".into()),
                src.acodec.as_deref().unwrap_or("None"),
                src.channels.map(|c| c.to_string()).unwrap_or_else(|| "None".into())
            ));
        }
    } else if out.has_audio {
        problems.push("the output has audio the input did not".to_string());
    }
    problems
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn info(json: &str) -> Info {
        Info::from_json(json, Path::new("x.mp4")).unwrap()
    }

    fn clip(duration: &str, audio: bool) -> Info {
        let audio_stream = if audio {
            r#",{"index":1,"codec_type":"audio","codec_name":"aac","channels":2,"duration":"1.0"}"#
        } else {
            ""
        };
        info(&format!(
            r#"{{"streams":[{{"index":0,"codec_type":"video","codec_name":"h264","width":160,"height":96,"r_frame_rate":"24/1","avg_frame_rate":"24/1","nb_frames":"24"}}{}],"format":{{"duration":"{}"}}}}"#,
            audio_stream, duration
        ))
    }

    fn rate() -> Rate {
        Rate::new(24, 1).unwrap()
    }

    #[test]
    fn a_matching_output_has_no_problems() {
        let src = clip("1.000", true);
        assert!(verify(&src, Some(&clip("1.020", true)), &rate(), None, Some(24)).is_empty());
    }

    #[test]
    fn missing_video_is_the_only_problem_reported() {
        let src = clip("1.0", false);
        assert_eq!(verify(&src, None, &rate(), None, None), ["the output has no readable video stream"]);
        let audio_only = info(r#"{"streams":[{"index":0,"codec_type":"audio","codec_name":"aac","channels":2}]}"#);
        assert_eq!(verify(&src, Some(&audio_only), &rate(), None, None), ["the output has no readable video stream"]);
    }

    #[test]
    fn size_and_rate_are_compared() {
        let src = clip("1.0", false);
        let other = info(
            r#"{"streams":[{"index":0,"codec_type":"video","width":80,"height":96,"r_frame_rate":"25/1","avg_frame_rate":"25/1"}],"format":{"duration":"1.0"}}"#,
        );
        let problems = verify(&src, Some(&other), &rate(), None, None);
        assert_eq!(problems.len(), 2, "{:?}", problems);
        assert_eq!(problems[0], "size is 80x96, expected 160x96");
        assert_eq!(problems[1], "frame rate is 25.0, expected 24.000");
    }

    #[test]
    fn duration_has_a_one_frame_window() {
        let src = clip("1.000", false);
        assert!(verify(&src, Some(&clip("1.040", false)), &rate(), None, None).is_empty());
        let late = verify(&src, Some(&clip("1.100", false)), &rate(), None, None);
        assert_eq!(late, ["duration is 1.100s, expected 1.000s within one frame (0.042s)"]);
    }

    #[test]
    fn frames_set_the_expected_length_and_count() {
        let src = clip("1.000", false);
        let mut out = clip("0.500", false);
        out.nb_frames = Some(12);
        assert!(verify(&src, Some(&out), &rate(), Some(12), Some(24)).is_empty());
        out.nb_frames = Some(11);
        assert_eq!(verify(&src, Some(&out), &rate(), Some(12), Some(24)), ["the output has 11 frames, expected 12"]);
        let mut long = clip("0.900", false);
        long.nb_frames = None;
        let problems = verify(&src, Some(&long), &rate(), Some(12), Some(24));
        assert_eq!(problems.len(), 1);
        assert!(problems[0].starts_with("duration is 0.900s, expected 0.500s (12 frames at 24.000 fps) within 0.101s"), "{}", problems[0]);
    }

    #[test]
    fn frames_equal_to_the_source_compare_as_usual() {
        let src = clip("1.000", false);
        let out = clip("1.000", false);
        assert!(verify(&src, Some(&out), &rate(), Some(24), Some(24)).is_empty());
    }

    #[test]
    fn a_declared_size_and_an_unknown_length_are_honoured() {
        let src = clip("0.5", false);
        let mut out = clip("0.25", false);
        out.width = Some(80);
        out.height = Some(48);
        assert_eq!(verify_with(&src, Some(&out), &rate(), None, Some(12), Some((80, 48)), false), Vec::<String>::new());
        let problems = verify_with(&src, Some(&out), &rate(), None, Some(12), None, true);
        assert_eq!(problems.len(), 2, "{:?}", problems);
        assert!(problems[0].starts_with("size is 80x48, expected 160x96"), "{:?}", problems);
    }

    #[test]
    fn audio_must_survive_unchanged() {
        let src = clip("1.0", true);
        let lost = verify(&src, Some(&clip("1.0", false)), &rate(), None, None);
        assert_eq!(lost, ["the audio stream was lost"]);
        let mut other = clip("1.0", true);
        other.channels = Some(1);
        assert_eq!(verify(&src, Some(&other), &rate(), None, None), ["audio is aac 1ch, expected aac 2ch"]);
        let silent = clip("1.0", false);
        assert_eq!(verify(&silent, Some(&clip("1.0", true)), &rate(), None, None), ["the output has audio the input did not"]);
    }

    #[test]
    fn a_rotated_source_is_compared_at_its_displayed_size() {
        let src = info(
            r#"{"streams":[{"index":0,"codec_type":"video","width":160,"height":96,"r_frame_rate":"24/1","side_data_list":[{"rotation":90}]}],"format":{"duration":"1.0"}}"#,
        );
        let out = info(
            r#"{"streams":[{"index":0,"codec_type":"video","width":96,"height":160,"r_frame_rate":"24/1","avg_frame_rate":"24/1"}],"format":{"duration":"1.0"}}"#,
        );
        assert!(verify(&src, Some(&out), &rate(), None, None).is_empty());
    }
}
