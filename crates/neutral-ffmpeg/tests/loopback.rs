//! Decoder and Encoder against real ffmpeg: generate frames from lavfi sources,
//! encode them losslessly, decode them again and compare byte for byte.

mod common;

use std::time::{Duration, Instant};

use neutral_ffmpeg::{Binaries, Decoder, Encoder, FrameBuf, PixFmt};

/// Frames of a lavfi source (`testsrc2` and friends) in the requested format and size.
fn lavfi_frames(bins: &Binaries, spec: &str, fmt: PixFmt, w: u32, h: u32) -> Vec<FrameBuf> {
    Decoder::builder(spec)
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .pix_fmt(fmt)
        .size(w, h)
        .spawn()
        .expect("start decoder")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode lavfi source")
}

fn encode_to(
    bins: &Binaries,
    path: &std::path::Path,
    frames: &[FrameBuf],
    configure: impl FnOnce(neutral_ffmpeg::EncoderBuilder) -> neutral_ffmpeg::EncoderBuilder,
) -> neutral_ffmpeg::EncodeStats {
    let first = &frames[0];
    let builder = Encoder::builder(path, first.fmt(), first.width(), first.height())
        .binaries(bins.clone())
        .fps(10, 1);
    let mut enc = configure(builder).spawn().expect("start encoder");
    for f in frames {
        enc.write_frame(f).expect("write frame");
    }
    enc.finish().expect("finish encoding")
}

fn decode_file(bins: &Binaries, path: &std::path::Path, fmt: PixFmt, w: u32, h: u32) -> Vec<FrameBuf> {
    Decoder::builder(path)
        .binaries(bins.clone())
        .pix_fmt(fmt)
        .size(w, h)
        .spawn()
        .expect("start decoder")
        .collect::<Result<Vec<_>, _>>()
        .expect("decode file")
}

#[test]
fn lavfi_source_decodes_to_the_expected_frames() {
    let Some(bins) = common::require("lavfi_source_decodes_to_the_expected_frames") else { return };
    let frames = lavfi_frames(&bins, "testsrc2=s=64x48:r=10:d=1", PixFmt::RGB24, 64, 48);
    assert_eq!(frames.len(), 10);
    for f in &frames {
        assert_eq!((f.fmt(), f.width(), f.height(), f.len()), (PixFmt::RGB24, 64, 48, 9216));
    }
    assert_ne!(frames[0], frames[9], "testsrc2 is animated");
    assert!(frames[0].as_bytes().iter().any(|&b| b != 0));
}

#[test]
fn size_is_measured_when_not_given() {
    let Some(bins) = common::require("size_is_measured_when_not_given") else { return };
    let dec = Decoder::builder("testsrc2=s=96x64:r=5:d=1")
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .pix_fmt(PixFmt::YUV420P)
        .spawn()
        .expect("start");
    assert_eq!((dec.width(), dec.height()), (96, 64));
    assert_eq!(dec.frame_bytes(), 96 * 64 * 3 / 2);
    let frames: Vec<FrameBuf> = dec.collect::<Result<_, _>>().unwrap();
    assert_eq!(frames.len(), 5);

    // A filter that changes the size is taken into account.
    let dec = Decoder::builder("testsrc2=s=96x64:r=5:d=1")
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .filter("crop=50:30,scale=iw/2:ih/2")
        .pix_fmt(PixFmt::GRAY)
        .spawn()
        .expect("start");
    assert_eq!((dec.width(), dec.height()), (25, 15));
    assert_eq!(dec.collect::<Result<Vec<_>, _>>().unwrap().len(), 5);
}

#[test]
fn scaling_to_a_requested_size_and_options() {
    let Some(bins) = common::require("scaling_to_a_requested_size_and_options") else { return };
    let frames = Decoder::builder("testsrc2=s=64x48:r=10:d=1")
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .pix_fmt(PixFmt::BGRA)
        .size(33, 17)
        .scale_flags("neighbor")
        .max_frames(7)
        .spawn()
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(frames.len(), 7);
    assert!(frames.iter().all(|f| (f.width(), f.height(), f.len()) == (33, 17, 33 * 17 * 4)));

    let some = Decoder::builder("testsrc2=s=64x48:r=10:d=1")
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .pix_fmt(PixFmt::RGB24)
        .size(64, 48)
        .start(Duration::from_millis(500))
        .duration(Duration::from_millis(400))
        .spawn()
        .unwrap()
        .count();
    assert_eq!(some, 4, "0.4 s at 10 fps");
}

#[test]
fn read_frame_into_reuses_a_buffer() {
    let Some(bins) = common::require("read_frame_into_reuses_a_buffer") else { return };
    let all = lavfi_frames(&bins, "testsrc2=s=32x24:r=10:d=1", PixFmt::RGB24, 32, 24);
    let mut dec = Decoder::builder("testsrc2=s=32x24:r=10:d=1")
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .pix_fmt(PixFmt::RGB24)
        .size(32, 24)
        .spawn()
        .unwrap();
    let mut buf = FrameBuf::new(PixFmt::RGB24, 32, 24).unwrap();
    let mut n = 0;
    while dec.read_frame_into(&mut buf).unwrap() {
        assert_eq!(buf, all[n]);
        n += 1;
    }
    assert_eq!((n, dec.frames_read()), (10, 10));
    assert!(!dec.read_frame_into(&mut buf).unwrap(), "stays at the end");
    let mut wrong = FrameBuf::new(PixFmt::RGB24, 16, 16).unwrap();
    assert!(dec.read_frame_into(&mut wrong).is_err());
}

#[test]
fn byte_exact_loopback_rawvideo_in_nut() {
    let Some(bins) = common::require("byte_exact_loopback_rawvideo_in_nut") else { return };
    let dir = common::scratch("loop_raw");
    let frames = lavfi_frames(&bins, "testsrc2=s=64x48:r=10:d=1", PixFmt::RGB24, 64, 48);
    let out = dir.join("loop.nut");
    let stats = encode_to(&bins, &out, &frames, |b| b.codec("rawvideo").format("nut"));
    assert_eq!(stats.frames, 10);
    assert_eq!(stats.bytes, 10 * 9216);
    let back = decode_file(&bins, &out, PixFmt::RGB24, 64, 48);
    assert_eq!(back.len(), frames.len());
    assert!(back == frames, "decoded frames differ from the encoded ones");
}

#[test]
fn byte_exact_loopback_ffv1_in_matroska_rgb() {
    let Some(bins) = common::require("byte_exact_loopback_ffv1_in_matroska_rgb") else { return };
    let dir = common::scratch("loop_ffv1_rgb");
    let frames = lavfi_frames(&bins, "testsrc2=s=64x48:r=10:d=1", PixFmt::RGB24, 64, 48);
    let out = dir.join("loop.mkv");
    encode_to(&bins, &out, &frames, |b| b.codec("ffv1").output_args(["-level", "3", "-g", "1"]));
    let back = decode_file(&bins, &out, PixFmt::RGB24, 64, 48);
    assert_eq!(back.len(), frames.len());
    assert!(back == frames, "ffv1 is lossless, frames must match");
}

#[test]
fn byte_exact_loopback_ffv1_yuv420p_and_odd_sizes() {
    let Some(bins) = common::require("byte_exact_loopback_ffv1_yuv420p_and_odd_sizes") else { return };
    let dir = common::scratch("loop_ffv1_yuv");
    for (w, h) in [(64u32, 48u32), (33, 17), (7, 5)] {
        let spec = format!("testsrc2=s={}x{}:r=10:d=1", w.max(16), h.max(16));
        let frames = lavfi_frames(&bins, &spec, PixFmt::YUV420P, w, h);
        assert_eq!(frames.len(), 10);
        let out = dir.join(format!("loop_{}x{}.mkv", w, h));
        encode_to(&bins, &out, &frames, |b| b.codec("ffv1"));
        let back = decode_file(&bins, &out, PixFmt::YUV420P, w, h);
        assert!(back == frames, "{}x{} yuv420p differs after ffv1", w, h);
    }
}

#[test]
fn byte_exact_loopback_for_a_spread_of_formats() {
    let Some(bins) = common::require("byte_exact_loopback_for_a_spread_of_formats") else { return };
    let dir = common::scratch("loop_formats");
    // Synthetic frames, every byte pattern random, so nothing is hidden by smooth content.
    let formats = [
        PixFmt::GRAY,
        PixFmt::GRAY16LE,
        PixFmt::RGB24,
        PixFmt::BGRA,
        PixFmt::YUV422P,
        PixFmt::YUV444P10LE,
        PixFmt::NV12,
        PixFmt::YUYV422,
        PixFmt::RGB48LE,
        PixFmt::GBRP,
    ];
    for fmt in formats {
        let (w, h) = (21u32, 13u32);
        let size = fmt.frame_size(w, h).unwrap();
        let frames: Vec<FrameBuf> = (0..4)
            .map(|i| FrameBuf::from_vec(fmt, w, h, common::pattern(size, 100 + i)).unwrap())
            .collect();
        let out = dir.join(format!("{}.nut", fmt.name()));
        encode_to(&bins, &out, &frames, |b| b.codec("rawvideo").format("nut"));
        let back = decode_file(&bins, &out, fmt, w, h);
        assert_eq!(back.len(), 4, "{}", fmt);
        assert!(back == frames, "{}: rawvideo/nut loopback changed the bytes", fmt);
    }
}

#[test]
fn paths_with_spaces_work() {
    let Some(bins) = common::require("paths_with_spaces_work") else { return };
    let dir = common::scratch("loop_spaces").join("a folder with spaces");
    std::fs::create_dir_all(&dir).unwrap();
    let frames = lavfi_frames(&bins, "testsrc2=s=32x24:r=10:d=1", PixFmt::RGB24, 32, 24);
    let out = dir.join("my clip.mkv");
    encode_to(&bins, &out, &frames, |b| b.codec("ffv1"));
    assert_eq!(decode_file(&bins, &out, PixFmt::RGB24, 32, 24), frames);
}

#[test]
fn encoder_rejects_mismatched_frames() {
    let Some(bins) = common::require("encoder_rejects_mismatched_frames") else { return };
    let dir = common::scratch("enc_mismatch");
    let mut enc = Encoder::builder(dir.join("o.nut"), PixFmt::RGB24, 16, 16)
        .binaries(bins.clone())
        .codec("rawvideo")
        .spawn()
        .unwrap();
    let other_size = FrameBuf::new(PixFmt::RGB24, 16, 8).unwrap();
    let other_fmt = FrameBuf::new(PixFmt::BGR24, 16, 16).unwrap();
    assert!(matches!(enc.write_frame(&other_size), Err(neutral_ffmpeg::Error::FrameMismatch(_))));
    assert!(matches!(enc.write_frame(&other_fmt), Err(neutral_ffmpeg::Error::FrameMismatch(_))));
    assert!(matches!(enc.write_raw_frame(&[0u8; 5]), Err(neutral_ffmpeg::Error::BufferSize { .. })));
    enc.write_raw_frame(&vec![9u8; 16 * 16 * 3]).unwrap();
    assert_eq!(enc.frames_written(), 1);
    assert_eq!(enc.finish().unwrap().frames, 1);
}

#[test]
fn passthrough_options_reach_ffmpeg() {
    let Some(bins) = common::require("passthrough_options_reach_ffmpeg") else { return };
    let dir = common::scratch("enc_passthrough");
    let frames = lavfi_frames(&bins, "testsrc2=s=64x48:r=10:d=1", PixFmt::YUV420P, 64, 48);
    // An output option with a visible effect: -t 0.5 limits the file to 5 frames at 10 fps.
    let out = dir.join("short.mkv");
    encode_to(&bins, &out, &frames, |b| b.codec("ffv1").output_args(["-t", "0.5"]));
    assert_eq!(decode_file(&bins, &out, PixFmt::YUV420P, 64, 48).len(), 5);
    // The container is chosen by -f even when the extension says otherwise.
    let odd = dir.join("looks-like-mp4.mp4");
    encode_to(&bins, &odd, &frames, |b| b.codec("ffv1").format("matroska"));
    let p = neutral_ffmpeg::probe_file(&bins, &odd).unwrap();
    assert!(p.format.format_name.unwrap().contains("matroska"));
    // The output pixel format option is passed through.
    let gray = dir.join("gray.mkv");
    encode_to(&bins, &gray, &frames, |b| b.codec("ffv1").out_pix_fmt(PixFmt::GRAY));
    let p = neutral_ffmpeg::probe_file(&bins, &gray).unwrap();
    assert_eq!(p.first_video().unwrap().pix_fmt.as_deref(), Some("gray"));
}

#[test]
fn stopping_a_decoder_kills_ffmpeg_promptly() {
    let Some(bins) = common::require("stopping_a_decoder_kills_ffmpeg_promptly") else { return };
    // A real-time source of 60 seconds (-re paces it at 4 frames per second, 16x16 pixels):
    // ffmpeg would run for a minute if stop() and drop did not kill it.
    let mut dec = Decoder::builder("color=c=blue:s=16x16:r=4:d=60")
        .binaries(bins.clone())
        .input_args(["-re", "-f", "lavfi"])
        .pix_fmt(PixFmt::GRAY)
        .size(16, 16)
        .spawn()
        .unwrap();
    assert!(dec.read_frame().unwrap().is_some());
    let started = Instant::now();
    dec.stop();
    assert!(started.elapsed() < Duration::from_secs(5), "stop() must not wait for the endless source");

    let mut dec = Decoder::builder("color=c=blue:s=16x16:r=4:d=60")
        .binaries(bins.clone())
        .input_args(["-re", "-f", "lavfi"])
        .pix_fmt(PixFmt::GRAY)
        .size(16, 16)
        .spawn()
        .unwrap();
    assert!(dec.read_frame().unwrap().is_some());
    let started = Instant::now();
    drop(dec);
    assert!(started.elapsed() < Duration::from_secs(5), "drop must not wait for the endless source");
}

#[test]
fn dropping_an_encoder_does_not_hang() {
    let Some(bins) = common::require("dropping_an_encoder_does_not_hang") else { return };
    let dir = common::scratch("enc_drop");
    let mut enc = Encoder::builder(dir.join("o.nut"), PixFmt::GRAY, 16, 16)
        .binaries(bins.clone())
        .codec("rawvideo")
        .spawn()
        .unwrap();
    enc.write_raw_frame(&[1u8; 256]).unwrap();
    let started = Instant::now();
    drop(enc);
    assert!(started.elapsed() < Duration::from_secs(5));
}

#[test]
fn a_chatty_ffmpeg_cannot_stall_the_pipes() {
    let Some(bins) = common::require("a_chatty_ffmpeg_cannot_stall_the_pipes") else { return };
    // trace level prints far more than any pipe buffer holds. If standard error were
    // not drained on its own thread, ffmpeg would block on it and this would hang.
    let mut dec = Decoder::builder("testsrc2=s=64x48:r=25:d=0.4")
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .pix_fmt(PixFmt::RGB24)
        .size(64, 48)
        .loglevel("trace")
        .spawn()
        .unwrap();
    let mut n = 0;
    while let Some(_) = dec.read_frame().unwrap() {
        n += 1;
        if n == 5 {
            std::thread::sleep(Duration::from_millis(300));
        }
    }
    assert_eq!(n, 10);
    let text = dec.stderr();
    assert!(text.len() > 10_000, "expected plenty of trace output, got {} bytes", text.len());
    assert!(text.len() < 80_000, "captured text must stay bounded, got {} bytes", text.len());
}
