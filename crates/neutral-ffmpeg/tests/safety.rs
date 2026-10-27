//! Regression tests for runaway ffmpeg jobs.
//!
//! An earlier version measured the output size of a filter graph on an endless
//! source. With a buffering filter such as `reverse`, ffmpeg cannot emit its first
//! frame before the input ends, so it buffered until the machine ran out of memory.
//! These tests pin the fixes: finite dry-run inputs, hard deadlines with kill, and
//! a clear error when the size cannot be measured. Every source used here is finite.

mod common;

use std::ffi::OsString;
use std::time::{Duration, Instant};

use neutral_ffmpeg::exec::run_capture;
use neutral_ffmpeg::{Decoder, Encoder, Error, Filter, PixFmt};

fn args(list: &[&str]) -> Vec<OsString> {
    list.iter().map(OsString::from).collect()
}

#[test]
fn size_probe_for_reverse_terminates_quickly() {
    let Some(bins) = common::require("size_probe_for_reverse_terminates_quickly") else { return };
    let started = Instant::now();
    let filter = Filter::builder(PixFmt::RGB24, 64, 48)
        .binaries(bins.clone())
        .graph("reverse")
        .spawn()
        .expect("the size of a reversed graph is measurable");
    let took = started.elapsed();
    assert!(took < Duration::from_secs(10), "size probe took {:?}", took);
    assert_eq!(filter.out_size(), (64, 48));
}

#[test]
fn size_probe_for_other_buffering_filters_terminates() {
    let Some(bins) = common::require("size_probe_for_other_buffering_filters_terminates") else { return };
    for (graph, expect) in [
        ("tpad=stop_duration=1", (64u32, 48u32)),
        ("loop=loop=2:size=5", (64, 48)),
        ("tile=2x2", (128, 96)),
        ("framerate=fps=50", (64, 48)),
        ("scale=32:24,reverse", (32, 24)),
    ] {
        let started = Instant::now();
        let filter = Filter::builder(PixFmt::RGB24, 64, 48)
            .binaries(bins.clone())
            .graph(graph)
            .spawn()
            .unwrap_or_else(|e| panic!("{}: {}", graph, e));
        assert!(started.elapsed() < Duration::from_secs(10), "{} took {:?}", graph, started.elapsed());
        assert_eq!(filter.out_size(), expect, "{}", graph);
    }
}

#[test]
fn size_probe_that_gets_no_frame_says_to_give_the_size() {
    let Some(bins) = common::require("size_probe_that_gets_no_frame_says_to_give_the_size") else { return };
    // trim=start=5 drops everything in the two seconds of test input.
    let started = Instant::now();
    let err = Filter::builder(PixFmt::RGB24, 64, 48)
        .binaries(bins.clone())
        .graph("trim=start=5")
        .spawn()
        .err()
        .expect("no frame, no size");
    assert!(started.elapsed() < Duration::from_secs(10));
    assert!(matches!(err, Error::SizeUnknown { .. }), "{:?}", err);
    let shown = err.to_string();
    assert!(shown.contains("output size") && shown.contains("explicitly"), "{}", shown);

    // Giving the size explicitly skips the measurement and works.
    let filter = Filter::builder(PixFmt::RGB24, 64, 48)
        .binaries(bins.clone())
        .graph("trim=start=5")
        .output_size(64, 48)
        .spawn()
        .expect("explicit size");
    assert_eq!(filter.out_size(), (64, 48));
}

#[test]
fn decoder_size_probe_reads_only_a_bounded_slice_of_a_long_source() {
    let Some(bins) = common::require("decoder_size_probe_reads_only_a_bounded_slice_of_a_long_source") else {
        return;
    };
    // Ten minutes of tiny frames. The dry run limits its input to two seconds, so reverse
    // sees an end of input; without that bound it would buffer all 15000 frames first.
    let started = Instant::now();
    let dec = Decoder::builder("testsrc2=s=64x48:r=25:d=600")
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .filter("reverse")
        .pix_fmt(PixFmt::GRAY)
        .spawn()
        .expect("size of a reversed long source");
    assert!(started.elapsed() < Duration::from_secs(10), "took {:?}", started.elapsed());
    assert_eq!((dec.width(), dec.height()), (64, 48));
    dec.stop();
}

#[test]
fn run_capture_kills_a_command_that_overruns_its_deadline() {
    let Some(bins) = common::require("run_capture_kills_a_command_that_overruns_its_deadline") else { return };
    let ffmpeg = bins.ffmpeg_path().unwrap();
    // 30 seconds of source paced in real time (-re): it would run for 30 s.
    let slow = args(&[
        "-hide_banner", "-loglevel", "error", "-nostdin", "-re", "-f", "lavfi", "-i", "color=c=black:s=16x16:r=1:d=30",
        "-f", "null", "-",
    ]);
    let started = Instant::now();
    let err = run_capture(&ffmpeg, &slow, Duration::from_millis(800), "ffmpeg", "a slow test job").unwrap_err();
    assert!(started.elapsed() < Duration::from_secs(8), "took {:?}", started.elapsed());
    match err {
        Error::Timeout { tool, seconds, .. } => {
            assert_eq!(tool, "ffmpeg");
            assert!((seconds - 0.8).abs() < 1e-6);
        }
        other => panic!("expected a timeout, got {:?}", other),
    }
    // A job that finishes in time returns its output.
    let quick = run_capture(&ffmpeg, &args(&["-version"]), Duration::from_secs(30), "ffmpeg", "version").unwrap();
    assert!(quick.status.success());
    assert!(quick.stdout.starts_with(b"ffmpeg version"));
}

#[test]
fn encoder_finish_has_a_deadline() {
    let Some(bins) = common::require("encoder_finish_has_a_deadline") else { return };
    let dir = common::scratch("safety_encoder_deadline");
    // -re paces the raw input at its frame rate of 1 per second, so closing the input
    // after 4 frames leaves about 4 seconds of work; the deadline is well below that.
    let mut enc = Encoder::builder(dir.join("slow.nut"), PixFmt::GRAY, 16, 16)
        .binaries(bins.clone())
        .fps(1, 1)
        .input_args(["-re"])
        .codec("rawvideo")
        .format("nut")
        .finish_timeout(Duration::from_millis(700))
        .spawn()
        .unwrap();
    for _ in 0..4 {
        enc.write_raw_frame(&[3u8; 256]).unwrap();
    }
    let started = Instant::now();
    let err = enc.finish().unwrap_err();
    assert!(started.elapsed() < Duration::from_secs(8), "took {:?}", started.elapsed());
    assert!(matches!(err, Error::Timeout { .. }), "{:?}", err);
}

#[test]
fn the_test_harness_refuses_unbounded_sources() {
    common::assert_bounded(&["-f", "lavfi", "-i", "color=c=red:s=16x16:d=1"]);
    common::assert_bounded(&["-f", "lavfi", "-i", "sine=frequency=440:duration=1"]);
    common::assert_bounded(&["-f", "lavfi", "-t", "1", "-i", "color=c=red"]);
    common::assert_bounded(&["-i", "file.mkv"]);
    let unbounded = std::panic::catch_unwind(|| {
        common::assert_bounded(&["-f", "lavfi", "-i", "color=c=red:s=16x16", "-frames:v", "1"]);
    });
    assert!(unbounded.is_err(), "a lavfi source without a duration must be refused");
}
