//! Filter graphs run through ffmpeg: identity, pixel format changes, size changes,
//! frame count changes and the no-deadlock guarantee.

mod common;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use neutral_ffmpeg::{Binaries, Decoder, Encoder, Filter, FrameBuf, PixFmt};

fn source_frames(bins: &Binaries, spec: &str, fmt: PixFmt, w: u32, h: u32) -> Vec<FrameBuf> {
    Decoder::builder(spec)
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .pix_fmt(fmt)
        .size(w, h)
        .spawn()
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[test]
fn null_filter_is_byte_exact() {
    let Some(bins) = common::require("null_filter_is_byte_exact") else { return };
    let frames = source_frames(&bins, "testsrc2=s=64x48:r=10:d=1", PixFmt::RGB24, 64, 48);
    let filter = Filter::builder(PixFmt::RGB24, 64, 48)
        .binaries(bins.clone())
        .fps(10, 1)
        .graph("null")
        .spawn()
        .unwrap();
    assert_eq!(filter.out_size(), (64, 48));
    assert_eq!(filter.out_pix_fmt(), PixFmt::RGB24);
    let out = filter.run_all(frames.clone()).unwrap();
    assert_eq!(out.len(), frames.len());
    assert!(out == frames, "the null filter must not change a single byte");
}

#[test]
fn empty_graph_is_byte_exact_for_many_formats() {
    let Some(bins) = common::require("empty_graph_is_byte_exact_for_many_formats") else { return };
    for fmt in [PixFmt::GRAY, PixFmt::YUV420P, PixFmt::YUV444P12LE, PixFmt::NV12, PixFmt::RGB48LE, PixFmt::BGRA, PixFmt::GBRP] {
        let (w, h) = (23u32, 11u32);
        let size = fmt.frame_size(w, h).unwrap();
        let frames: Vec<FrameBuf> =
            (0..3).map(|i| FrameBuf::from_vec(fmt, w, h, common::pattern(size, 40 + i)).unwrap()).collect();
        let out = Filter::builder(fmt, w, h)
            .binaries(bins.clone())
            .size_unchanged()
            .spawn()
            .unwrap()
            .run_all(frames.clone())
            .unwrap();
        assert!(out == frames, "{}: empty graph changed the bytes", fmt);
    }
}

#[test]
fn format_changing_filter_matches_ffmpegs_own_conversion() {
    let Some(bins) = common::require("format_changing_filter_matches_ffmpegs_own_conversion") else { return };
    let dir = common::scratch("filter_gray");
    let frames = source_frames(&bins, "testsrc2=s=64x48:r=10:d=1", PixFmt::RGB24, 64, 48);

    let out = Filter::builder(PixFmt::RGB24, 64, 48)
        .binaries(bins.clone())
        .graph("format=gray")
        .out_pix_fmt(PixFmt::GRAY)
        .spawn()
        .unwrap()
        .run_all(frames.clone())
        .unwrap();
    assert_eq!(out.len(), 10);
    for f in &out {
        assert_eq!((f.fmt(), f.width(), f.height(), f.len()), (PixFmt::GRAY, 64, 48, 3072));
    }

    // The same conversion done by the Encoder with a raw file as output.
    let raw = dir.join("converted.raw");
    let mut enc = Encoder::builder(&raw, PixFmt::RGB24, 64, 48)
        .binaries(bins.clone())
        .fps(10, 1)
        .codec("rawvideo")
        .format("rawvideo")
        .out_pix_fmt(PixFmt::GRAY)
        .spawn()
        .unwrap();
    for f in &frames {
        enc.write_frame(f).unwrap();
    }
    enc.finish().unwrap();
    let expected = std::fs::read(&raw).unwrap();
    let got: Vec<u8> = out.iter().flat_map(|f| f.as_bytes().iter().copied()).collect();
    assert_eq!(got.len(), expected.len());
    assert!(got == expected, "filter output differs from the encoder's conversion");
}

#[test]
fn solid_red_becomes_a_flat_gray() {
    let Some(bins) = common::require("solid_red_becomes_a_flat_gray") else { return };
    let mut red = FrameBuf::new(PixFmt::RGB24, 16, 8).unwrap();
    for px in red.as_bytes_mut().chunks_mut(3) {
        px.copy_from_slice(&[255, 0, 0]);
    }
    let out = Filter::builder(PixFmt::RGB24, 16, 8)
        .binaries(bins.clone())
        .graph("format=gray")
        .out_pix_fmt(PixFmt::GRAY)
        .spawn()
        .unwrap()
        .run_all(vec![red.clone(), red])
        .unwrap();
    assert_eq!(out.len(), 2);
    let level = out[0].as_bytes()[0];
    assert!((60..=100).contains(&level), "luma of pure red was {}", level);
    assert!(out.iter().all(|f| f.as_bytes().iter().all(|&b| b == level)));
}

#[test]
fn size_changing_graphs_are_measured_automatically() {
    let Some(bins) = common::require("size_changing_graphs_are_measured_automatically") else { return };
    let frames = source_frames(&bins, "testsrc2=s=64x48:r=10:d=1", PixFmt::RGB24, 64, 48);
    for (graph, expect) in [
        ("scale=32:24", (32u32, 24u32)),
        ("crop=40:20:4:4", (40, 20)),
        ("scale=iw*2:ih*2,pad=iw+10:ih+6", (138, 102)),
        ("transpose=1", (48, 64)),
    ] {
        let filter = Filter::builder(PixFmt::RGB24, 64, 48)
            .binaries(bins.clone())
            .fps(10, 1)
            .graph(graph)
            .spawn()
            .unwrap();
        assert_eq!(filter.out_size(), expect, "graph {}", graph);
        let out = filter.run_all(frames.clone()).unwrap();
        assert_eq!(out.len(), frames.len(), "graph {}", graph);
        assert!(out.iter().all(|f| (f.width(), f.height()) == expect), "graph {}", graph);
    }
}

#[test]
fn declared_output_size_is_enforced() {
    let Some(bins) = common::require("declared_output_size_is_enforced") else { return };
    let frames = source_frames(&bins, "testsrc2=s=64x48:r=10:d=1", PixFmt::YUV420P, 64, 48);
    let out = Filter::builder(PixFmt::YUV420P, 64, 48)
        .binaries(bins.clone())
        .fps(10, 1)
        .graph("hflip")
        .output_size(30, 20)
        .spawn()
        .unwrap()
        .run_all(frames)
        .unwrap();
    assert_eq!(out.len(), 10);
    assert!(out.iter().all(|f| (f.width(), f.height(), f.len()) == (30, 20, 30 * 20 + 2 * 15 * 10)));
}

#[test]
fn filters_that_drop_or_reorder_frames() {
    let Some(bins) = common::require("filters_that_drop_or_reorder_frames") else { return };
    let frames = source_frames(&bins, "testsrc2=s=32x24:r=10:d=1", PixFmt::RGB24, 32, 24);
    assert_eq!(frames.len(), 10);

    let every_second = Filter::builder(PixFmt::RGB24, 32, 24)
        .binaries(bins.clone())
        .fps(10, 1)
        .graph("framestep=2")
        .spawn()
        .unwrap()
        .run_all(frames.clone())
        .unwrap();
    assert_eq!(every_second.len(), 5);
    for (i, f) in every_second.iter().enumerate() {
        assert_eq!(f, &frames[2 * i], "framestep output {}", i);
    }

    // reverse holds every frame until the input ends, then emits them backwards.
    let mut filter = Filter::builder(PixFmt::RGB24, 32, 24)
        .binaries(bins.clone())
        .fps(10, 1)
        .graph("reverse")
        .spawn()
        .unwrap();
    for f in &frames {
        filter.push(f).unwrap();
        assert!(filter.try_pull().unwrap().is_none(), "nothing can come out before the input ends");
    }
    assert_eq!(filter.frames_in(), 10);
    let reversed = filter.finish().unwrap();
    let expect: Vec<FrameBuf> = frames.iter().rev().cloned().collect();
    assert!(reversed == expect, "reverse must emit the frames backwards");
}

#[test]
fn output_amplification_does_not_deadlock() {
    let Some(bins) = common::require("output_amplification_does_not_deadlock") else { return };
    // 10 input frames of 9 KB become about 100 output frames: ten times more data
    // comes out than goes in, a megabyte against pipe buffers of a few KB. A design
    // that only writes before it reads would stall here. The watchdog turns a hang
    // into a failure.
    let (tx, rx) = mpsc::channel();
    let bins2 = bins.clone();
    thread::spawn(move || {
        let frame = FrameBuf::from_vec(PixFmt::RGB24, 64, 48, common::pattern(9216, 5)).unwrap();
        let mut filter = Filter::builder(PixFmt::RGB24, 64, 48)
            .binaries(bins2)
            .fps(25, 1)
            .graph("fps=250")
            .spawn()
            .unwrap();
        for _ in 0..10 {
            filter.push(&frame).unwrap();
        }
        let out = filter.finish().unwrap();
        let _ = tx.send(out.len());
    });
    match rx.recv_timeout(Duration::from_secs(60)) {
        Ok(n) => assert!((90..=110).contains(&n), "expected about 100 frames, got {}", n),
        Err(_) => panic!("the filter pipeline hung"),
    }
}

#[test]
fn counters_and_state_are_reported() {
    let Some(bins) = common::require("counters_and_state_are_reported") else { return };
    let frame = FrameBuf::new(PixFmt::GRAY, 8, 8).unwrap();
    let mut filter = Filter::builder(PixFmt::GRAY, 8, 8).binaries(bins.clone()).size_unchanged().spawn().unwrap();
    filter.push(&frame).unwrap();
    filter.push(&frame).unwrap();
    filter.close_input();
    assert!(filter.push(&frame).is_err(), "input is closed");
    let mut got = 0;
    while filter.pull().unwrap().is_some() {
        got += 1;
    }
    assert_eq!(got, 2);
    assert!(filter.is_finished());
    assert_eq!((filter.frames_in(), filter.frames_out()), (2, 2));
    assert!(filter.pull().unwrap().is_none(), "stays finished");
    assert!(filter.try_pull().unwrap().is_none());
    assert!(filter.pull_timeout(Duration::from_millis(10)).unwrap().is_none());
}

#[test]
fn mismatched_input_frames_are_rejected() {
    let Some(bins) = common::require("mismatched_input_frames_are_rejected") else { return };
    let mut filter = Filter::builder(PixFmt::GRAY, 8, 8).binaries(bins.clone()).size_unchanged().spawn().unwrap();
    let wrong_size = FrameBuf::new(PixFmt::GRAY, 8, 4).unwrap();
    let wrong_fmt = FrameBuf::new(PixFmt::RGB24, 8, 8).unwrap();
    assert!(matches!(filter.push(&wrong_size), Err(neutral_ffmpeg::Error::FrameMismatch(_))));
    assert!(matches!(filter.push(&wrong_fmt), Err(neutral_ffmpeg::Error::FrameMismatch(_))));
    assert_eq!(filter.frames_in(), 0);
}
