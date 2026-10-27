//! ffprobe wrapper against files generated with lavfi sources.

mod common;

use std::ffi::OsString;

use neutral_ffmpeg::probe::{probe_file, probe_input};
use neutral_ffmpeg::{Error, StreamKind};

fn make_clip(bins: &neutral_ffmpeg::Binaries, dir: &std::path::Path) -> std::path::PathBuf {
    let out = dir.join("clip.mkv");
    let run = common::ffmpeg(
        bins,
        &[
            "-y",
            "-f",
            "lavfi",
            "-i",
            "testsrc2=s=64x48:r=10:d=1",
            "-f",
            "lavfi",
            "-i",
            "sine=frequency=440:sample_rate=44100:duration=1",
            "-c:v",
            "ffv1",
            "-pix_fmt",
            "yuv420p",
            "-c:a",
            "pcm_s16le",
            "-ac",
            "1",
            out.to_str().unwrap(),
        ],
    );
    assert!(run.ok, "could not generate the clip: {}", run.stderr);
    out
}

#[test]
fn probes_a_generated_file() {
    let Some(bins) = common::require("probes_a_generated_file") else { return };
    let dir = common::scratch("probe_clip");
    let clip = make_clip(&bins, &dir);
    let p = probe_file(&bins, &clip).expect("probe");

    let format_name = p.format.format_name.as_deref().unwrap_or("");
    assert!(format_name.contains("matroska"), "format_name was {:?}", format_name);
    let duration = p.format.duration.expect("container duration");
    assert!((duration - 1.0).abs() < 0.1, "duration {}", duration);

    assert_eq!(p.streams.len(), 2);
    let v = p.first_video().expect("video stream");
    assert_eq!(v.codec_type, StreamKind::Video);
    assert_eq!(v.codec_name.as_deref(), Some("ffv1"));
    assert_eq!((v.width, v.height), (Some(64), Some(48)));
    assert_eq!(v.pix_fmt.as_deref(), Some("yuv420p"));
    assert_eq!(v.r_frame_rate.unwrap().to_f64(), Some(10.0));
    assert_eq!(v.avg_frame_rate.unwrap().to_f64(), Some(10.0));
    if let Some(n) = v.nb_frames {
        assert_eq!(n, 10);
    }
    assert_eq!(v.sample_rate, None);
    assert_eq!(v.channels, None);

    let a = p.first_audio().expect("audio stream");
    assert_eq!(a.codec_type, StreamKind::Audio);
    assert_eq!(a.codec_name.as_deref(), Some("pcm_s16le"));
    assert_eq!(a.sample_rate, Some(44100));
    assert_eq!(a.channels, Some(1));
    assert_eq!((a.width, a.height, a.pix_fmt.as_deref()), (None, None, None));
    assert!(a.index.unwrap() > v.index.unwrap());
}

#[test]
fn probes_a_lavfi_source_with_input_options() {
    let Some(bins) = common::require("probes_a_lavfi_source_with_input_options") else { return };
    let p = probe_input(
        &bins,
        std::ffi::OsStr::new("color=c=red:s=32x24:r=5:d=1"),
        &[OsString::from("-f"), OsString::from("lavfi")],
    )
    .expect("probe lavfi");
    let v = p.first_video().expect("video");
    assert_eq!((v.width, v.height), (Some(32), Some(24)));
    assert_eq!(v.r_frame_rate.unwrap().to_f64(), Some(5.0));
}

#[test]
fn missing_file_reports_ffprobes_own_words() {
    let Some(bins) = common::require("missing_file_reports_ffprobes_own_words") else { return };
    let dir = common::scratch("probe_missing");
    let err = probe_file(&bins, dir.join("does-not-exist.mkv")).unwrap_err();
    match &err {
        Error::Ffmpeg { tool, stderr, code, .. } => {
            assert_eq!(*tool, "ffprobe");
            assert!(code.is_some());
            assert!(
                stderr.contains("does-not-exist.mkv") && stderr.to_lowercase().contains("no such file"),
                "stderr was {:?}",
                stderr
            );
        }
        other => panic!("expected an ffprobe error, got {:?}", other),
    }
    assert!(err.to_string().contains("does-not-exist.mkv"));
}

#[test]
fn a_file_that_is_not_media_is_an_error() {
    let Some(bins) = common::require("a_file_that_is_not_media_is_an_error") else { return };
    let dir = common::scratch("probe_garbage");
    let junk = dir.join("junk.bin");
    std::fs::write(&junk, common::pattern(4096, 7)).unwrap();
    let err = probe_file(&bins, &junk).unwrap_err();
    assert!(matches!(err, Error::Ffmpeg { tool: "ffprobe", .. }), "{:?}", err);
    assert!(err.stderr().is_some_and(|s| !s.is_empty()));
}

#[test]
fn explicit_missing_ffprobe_is_reported_as_such() {
    let bins = neutral_ffmpeg::Binaries::new().with_ffprobe("no/such/ffprobe-binary");
    let err = probe_file(&bins, "whatever.mkv").unwrap_err();
    assert!(matches!(err, Error::ToolNotFound { tool: "ffprobe", .. }), "{:?}", err);
}

// ---------------------------------------------------------------------------------
// Cross-check against ffprobe's own `flat` writer, a different output format from the
// JSON the crate parses, over several generated files.

use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use neutral_ffmpeg::exec::run_capture;
use neutral_ffmpeg::{Binaries, Probe, StreamInfo};

/// `ffprobe -of flat` for the same entries, as key -> value with quotes removed.
fn flat(bins: &Binaries, file: &Path) -> BTreeMap<String, String> {
    let ffprobe = bins.ffprobe_path().unwrap();
    let args: Vec<OsString> = [
        "-v",
        "error",
        "-hide_banner",
        "-show_entries",
        "format=format_name,duration:stream=index,codec_type,codec_name,width,height,pix_fmt,r_frame_rate,avg_frame_rate,duration,nb_frames,sample_rate,channels",
        "-of",
        "flat",
    ]
    .iter()
    .map(OsString::from)
    .chain([file.as_os_str().to_os_string()])
    .collect();
    let out = run_capture(&ffprobe, &args, Duration::from_secs(60), "ffprobe", "flat probe").unwrap();
    assert!(out.status.success(), "{}", out.stderr_text());
    let mut map = BTreeMap::new();
    for line in String::from_utf8_lossy(&out.stdout).lines() {
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.to_string(), v.trim_matches('"').to_string());
        }
    }
    map
}

fn check_against_flat(probe: &Probe, flat: &BTreeMap<String, String>, label: &str) {
    let get = |k: &str| flat.get(k).filter(|v| v.as_str() != "N/A").cloned();
    assert_eq!(probe.format.format_name, get("format.format_name"), "{} format_name", label);
    let dur = |s: Option<String>| s.and_then(|v| v.parse::<f64>().ok());
    assert_eq!(probe.format.duration, dur(get("format.duration")), "{} format duration", label);
    let n_flat = flat.keys().filter(|k| k.starts_with("streams.stream.") && k.ends_with(".codec_type")).count();
    assert_eq!(probe.streams.len(), n_flat, "{} stream count", label);
    for (i, s) in probe.streams.iter().enumerate() {
        let key = |f: &str| format!("streams.stream.{}.{}", i, f);
        let kind = match s.codec_type {
            StreamKind::Video => "video",
            StreamKind::Audio => "audio",
            StreamKind::Subtitle => "subtitle",
            StreamKind::Data => "data",
            StreamKind::Attachment => "attachment",
            StreamKind::Other(ref o) => o.as_str(),
        };
        assert_eq!(Some(kind.to_string()), get(&key("codec_type")), "{} stream {} codec_type", label, i);
        assert_eq!(s.codec_name, get(&key("codec_name")), "{} stream {} codec_name", label, i);
        assert_eq!(s.pix_fmt, get(&key("pix_fmt")), "{} stream {} pix_fmt", label, i);
        let int = |f: &str| get(&key(f)).and_then(|v| v.parse::<u64>().ok());
        assert_eq!(s.index.map(|v| v as u64), int("index"), "{} stream {} index", label, i);
        assert_eq!(s.width.map(|v| v as u64), int("width"), "{} stream {} width", label, i);
        assert_eq!(s.height.map(|v| v as u64), int("height"), "{} stream {} height", label, i);
        assert_eq!(s.nb_frames, int("nb_frames"), "{} stream {} nb_frames", label, i);
        assert_eq!(s.sample_rate.map(|v| v as u64), int("sample_rate"), "{} stream {} sample_rate", label, i);
        assert_eq!(s.channels.map(|v| v as u64), int("channels"), "{} stream {} channels", label, i);
        let rate = |r: Option<neutral_ffmpeg::Rational>| r.map(|r| format!("{}/{}", r.num, r.den));
        assert_eq!(rate(s.r_frame_rate), get(&key("r_frame_rate")), "{} stream {} r_frame_rate", label, i);
        assert_eq!(rate(s.avg_frame_rate), get(&key("avg_frame_rate")), "{} stream {} avg_frame_rate", label, i);
        assert_eq!(s.duration, dur(get(&key("duration"))), "{} stream {} duration", label, i);
        let _: &StreamInfo = s;
    }
}

#[test]
fn parsed_probe_equals_ffprobes_flat_output_for_several_generated_files() {
    let Some(bins) = common::require("parsed_probe_equals_ffprobes_flat_output_for_several_generated_files") else {
        return;
    };
    let dir = common::scratch("probe_many");
    // (file name, ffmpeg arguments that create it from finite lavfi sources)
    let p = |n: &str| dir.join(n).to_str().unwrap().to_string();
    let recipes: Vec<(String, Vec<String>)> = vec![
        (
            p("a_ffv1_pcm.mkv"),
            ["-f", "lavfi", "-i", "testsrc2=s=64x48:r=10:d=1", "-f", "lavfi", "-i", "sine=frequency=440:sample_rate=44100:duration=1",
             "-c:v", "ffv1", "-pix_fmt", "yuv420p", "-c:a", "pcm_s16le", "-ac", "1"].iter().map(|s| s.to_string()).collect(),
        ),
        (
            p("b_gray_odd.nut"),
            ["-f", "lavfi", "-i", "color=c=gray:s=33x17:r=5:d=1", "-c:v", "rawvideo", "-pix_fmt", "gray"]
                .iter().map(|s| s.to_string()).collect(),
        ),
        (
            p("c_stereo.wav"),
            ["-f", "lavfi", "-i", "sine=frequency=220:sample_rate=48000:duration=1", "-ac", "2", "-c:a", "pcm_s16le"]
                .iter().map(|s| s.to_string()).collect(),
        ),
        (
            p("d_ntsc_rate.mkv"),
            ["-f", "lavfi", "-i", "testsrc2=s=48x32:r=30000/1001:d=1", "-c:v", "ffv1", "-pix_fmt", "yuv444p"]
                .iter().map(|s| s.to_string()).collect(),
        ),
        (
            p("e_rgb48.nut"),
            ["-f", "lavfi", "-i", "color=c=red:s=16x16:r=2:d=1", "-c:v", "rawvideo", "-pix_fmt", "rgb48le"]
                .iter().map(|s| s.to_string()).collect(),
        ),
        (
            p("f_h264.mp4"),
            ["-f", "lavfi", "-i", "testsrc2=s=64x48:r=25:d=1", "-f", "lavfi", "-i", "sine=frequency=330:sample_rate=32000:duration=1",
             "-c:v", "libx264", "-pix_fmt", "yuv420p", "-c:a", "aac", "-shortest"].iter().map(|s| s.to_string()).collect(),
        ),
    ];
    let mut checked = 0;
    for (path, recipe) in recipes {
        let mut args: Vec<&str> = vec!["-y"];
        args.extend(recipe.iter().map(|s| s.as_str()));
        args.push(&path);
        let made = common::ffmpeg(&bins, &args);
        if !made.ok {
            eprintln!("note: could not create {} ({}); this ffmpeg lacks a codec, skipping that file", path, made.stderr.lines().next().unwrap_or(""));
            continue;
        }
        let probe = probe_file(&bins, &path).unwrap_or_else(|e| panic!("{}: {}", path, e));
        check_against_flat(&probe, &flat(&bins, Path::new(&path)), &path);
        checked += 1;
    }
    assert!(checked >= 5, "only {} files were generated and checked", checked);
}
