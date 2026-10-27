//! Error paths: what the caller sees when ffmpeg or the input is wrong.

mod common;

use neutral_ffmpeg::{Binaries, Decoder, Encoder, Error, Filter, FrameBuf, PixFmt};

fn expect_ffmpeg_error(err: Error) -> (Option<i32>, String) {
    match err {
        Error::Ffmpeg { tool, code, stderr, .. } => {
            assert_eq!(tool, "ffmpeg");
            (code, stderr)
        }
        other => panic!("expected an ffmpeg error, got {:?}", other),
    }
}

#[test]
fn missing_input_file_surfaces_ffmpegs_message() {
    let Some(bins) = common::require("missing_input_file_surfaces_ffmpegs_message") else { return };
    let dir = common::scratch("err_missing");
    let missing = dir.join("nothing-here.mp4");

    // With an explicit size the failure arrives through the frame iterator.
    let dec = Decoder::builder(&missing).binaries(bins.clone()).size(16, 16).spawn().unwrap();
    let items: Vec<_> = dec.collect();
    assert_eq!(items.len(), 1, "one error, no frames");
    let (code, stderr) = expect_ffmpeg_error(items.into_iter().next().unwrap().unwrap_err());
    assert!(code.is_some() && code != Some(0));
    assert!(stderr.contains("nothing-here.mp4"), "stderr was {:?}", stderr);
    assert!(stderr.to_lowercase().contains("no such file"), "stderr was {:?}", stderr);

    // Without a size the dry run fails first, at spawn.
    let err = Decoder::builder(&missing).binaries(bins.clone()).spawn().err().expect("spawn must fail");
    let (_, stderr) = expect_ffmpeg_error(err);
    assert!(stderr.contains("nothing-here.mp4"), "stderr was {:?}", stderr);
}

#[test]
fn error_display_contains_ffmpegs_text() {
    let Some(bins) = common::require("error_display_contains_ffmpegs_text") else { return };
    let dir = common::scratch("err_display");
    let dec = Decoder::builder(dir.join("gone.mkv")).binaries(bins.clone()).size(8, 8).spawn().unwrap();
    let err = dec.map(|r| r.unwrap_err()).next().unwrap();
    let shown = err.to_string();
    assert!(shown.contains("exit code"), "{}", shown);
    assert!(shown.contains("gone.mkv"), "{}", shown);
    assert!(shown.to_lowercase().contains("no such file"), "{}", shown);
}

#[test]
fn input_that_is_not_media() {
    let Some(bins) = common::require("input_that_is_not_media") else { return };
    let dir = common::scratch("err_junk");
    let junk = dir.join("junk.mp4");
    std::fs::write(&junk, common::pattern(10_000, 3)).unwrap();
    let dec = Decoder::builder(&junk).binaries(bins.clone()).size(16, 16).spawn().unwrap();
    let err = dec.map(|r| r.unwrap_err()).next().unwrap();
    let (_, stderr) = expect_ffmpeg_error(err);
    assert!(stderr.contains("Invalid data") || stderr.contains("moov atom"), "stderr was {:?}", stderr);
}

#[test]
fn video_stream_that_does_not_exist() {
    let Some(bins) = common::require("video_stream_that_does_not_exist") else { return };
    let dec = Decoder::builder("testsrc2=s=32x32:r=5:d=1")
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .video_stream(3)
        .size(32, 32)
        .spawn()
        .unwrap();
    let err = dec.map(|r| r.unwrap_err()).next().unwrap();
    let (_, stderr) = expect_ffmpeg_error(err);
    assert!(stderr.contains("matches no streams"), "stderr was {:?}", stderr);
}

#[test]
fn bad_pixel_format_names() {
    // Our own table rejects unknown names.
    assert!(matches!(PixFmt::parse("nonsense"), Err(Error::UnknownPixFmt(n)) if n == "nonsense"));
    assert!(matches!(PixFmt::parse(""), Err(Error::UnknownPixFmt(_))));
    // A format ffmpeg cannot emit is refused before any process starts.
    for name in ["vaapi", "cuda", "bayer_rggb8"] {
        let fmt = PixFmt::parse(name).unwrap();
        let err = Decoder::builder("whatever.mp4").pix_fmt(fmt).size(8, 8).spawn().err().unwrap();
        match err {
            Error::UnsupportedPixFmt { name: n, reason } => {
                assert_eq!(n, name);
                assert!(!reason.is_empty());
            }
            other => panic!("{:?}", other),
        }
    }
}

#[test]
fn ffmpeg_rejects_a_pixel_format_passed_through_as_text() {
    let Some(bins) = common::require("ffmpeg_rejects_a_pixel_format_passed_through_as_text") else { return };
    let dir = common::scratch("err_bad_pix_fmt");
    let mut enc = Encoder::builder(dir.join("o.nut"), PixFmt::RGB24, 16, 16)
        .binaries(bins.clone())
        .codec("rawvideo")
        .output_args(["-pix_fmt", "definitely-not-a-format"])
        .spawn()
        .unwrap();
    let frame = FrameBuf::new(PixFmt::RGB24, 16, 16).unwrap();
    // ffmpeg exits right away; some write may still succeed before the pipe breaks.
    let mut failure: Option<Error> = None;
    for _ in 0..50 {
        if let Err(e) = enc.write_frame(&frame) {
            failure = Some(e);
            break;
        }
    }
    let err = match failure {
        Some(e) => e,
        None => enc.finish().unwrap_err(),
    };
    let (_, stderr) = expect_ffmpeg_error(err);
    assert!(stderr.contains("Unknown pixel format"), "stderr was {:?}", stderr);
    assert!(stderr.contains("definitely-not-a-format"), "stderr was {:?}", stderr);
}

#[test]
fn encoder_output_in_a_missing_directory() {
    let Some(bins) = common::require("encoder_output_in_a_missing_directory") else { return };
    let dir = common::scratch("err_enc_dir");
    let out = dir.join("no-such-subdir").join("o.mkv");
    let mut enc = Encoder::builder(&out, PixFmt::GRAY, 8, 8)
        .binaries(bins.clone())
        .codec("ffv1")
        .spawn()
        .unwrap();
    let mut failure: Option<Error> = None;
    for _ in 0..50 {
        if let Err(e) = enc.write_raw_frame(&[0u8; 64]) {
            failure = Some(e);
            break;
        }
    }
    let err = match failure {
        Some(e) => e,
        None => enc.finish().unwrap_err(),
    };
    let (_, stderr) = expect_ffmpeg_error(err);
    assert!(stderr.to_lowercase().contains("no such file"), "stderr was {:?}", stderr);
    assert!(stderr.contains("no-such-subdir"), "stderr was {:?}", stderr);
}

#[test]
fn encoder_that_never_gets_frames_still_reports_failure() {
    let Some(bins) = common::require("encoder_that_never_gets_frames_still_reports_failure") else { return };
    let dir = common::scratch("err_enc_nothing");
    let enc = Encoder::builder(dir.join("o.mkv"), PixFmt::GRAY, 8, 8)
        .binaries(bins.clone())
        .codec("this-codec-does-not-exist")
        .spawn()
        .unwrap();
    let err = enc.finish().unwrap_err();
    let (_, stderr) = expect_ffmpeg_error(err);
    assert!(stderr.contains("this-codec-does-not-exist"), "stderr was {:?}", stderr);
}

#[test]
fn invalid_filter_graphs_report_ffmpegs_complaint() {
    let Some(bins) = common::require("invalid_filter_graphs_report_ffmpegs_complaint") else { return };
    // Automatic size detection runs the graph once and fails at spawn.
    let err = Filter::builder(PixFmt::RGB24, 16, 16)
        .binaries(bins.clone())
        .graph("nosuchfilter=1")
        .spawn()
        .err()
        .expect("spawn must fail");
    let (_, stderr) = expect_ffmpeg_error(err);
    assert!(stderr.contains("nosuchfilter"), "stderr was {:?}", stderr);

    // Without the dry run the failure shows up when the pipe breaks or at finish.
    let mut filter = Filter::builder(PixFmt::RGB24, 16, 16)
        .binaries(bins.clone())
        .graph("nosuchfilter=1")
        .size_unchanged()
        .spawn()
        .unwrap();
    let frame = FrameBuf::new(PixFmt::RGB24, 16, 16).unwrap();
    let mut failure: Option<Error> = None;
    for _ in 0..50 {
        if let Err(e) = filter.push(&frame) {
            failure = Some(e);
            break;
        }
    }
    let err = match failure {
        Some(e) => e,
        None => filter.finish().unwrap_err(),
    };
    let (_, stderr) = expect_ffmpeg_error(err);
    assert!(stderr.contains("nosuchfilter"), "stderr was {:?}", stderr);
}

#[test]
fn graph_options_that_are_wrong_are_reported() {
    let Some(bins) = common::require("graph_options_that_are_wrong_are_reported") else { return };
    let err = Filter::builder(PixFmt::RGB24, 16, 16)
        .binaries(bins.clone())
        .graph("scale=notanumber:10")
        .spawn()
        .err()
        .expect("spawn must fail");
    let (_, stderr) = expect_ffmpeg_error(err);
    assert!(!stderr.trim().is_empty());
}

#[test]
fn missing_ffmpeg_is_a_clear_error() {
    let bins = Binaries::new().with_ffmpeg("definitely/not/ffmpeg");
    let err = Decoder::builder("x.mp4").binaries(bins.clone()).size(8, 8).spawn().err().unwrap();
    match &err {
        Error::ToolNotFound { tool, tried } => {
            assert_eq!(*tool, "ffmpeg");
            assert!(tried.iter().any(|l| l.contains("definitely/not/ffmpeg") || l.contains("definitely\\not\\ffmpeg")));
        }
        other => panic!("{:?}", other),
    }
    assert!(err.to_string().contains("ffmpeg not found"));
    assert!(matches!(
        Encoder::builder("o.mkv", PixFmt::GRAY, 8, 8).binaries(bins.clone()).spawn().err().unwrap(),
        Error::ToolNotFound { .. }
    ));
    assert!(matches!(
        Filter::builder(PixFmt::GRAY, 8, 8).binaries(bins).size_unchanged().spawn().err().unwrap(),
        Error::ToolNotFound { .. }
    ));
}

#[test]
fn a_file_that_cannot_be_started_is_a_spawn_error() {
    let dir = common::scratch("err_not_executable");
    let fake = dir.join("ffmpeg.exe");
    std::fs::write(&fake, b"this is not an executable").unwrap();
    #[cfg(windows)]
    {
        extern "system" {
            fn SetErrorMode(mode: u32) -> u32;
        }
        // Without this Windows opens an "Unsupported 16-Bit Application" box for the bogus image and the test waits for a click.
        // SEM_FAILCRITICALERRORS | SEM_NOOPENFILEERRORBOX
        unsafe {
            SetErrorMode(0x0001 | 0x8000);
        }
    }
    let bins = Binaries::new().with_ffmpeg(&fake);
    let err = Decoder::builder("x.mp4").binaries(bins).size(8, 8).spawn().err().unwrap();
    assert!(matches!(err, Error::Spawn { .. }), "{:?}", err);
}
