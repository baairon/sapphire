//! Byte-for-byte agreement between the crate and ffmpeg's own output.
//!
//! For every format the table can emit, the Decoder is run on a short generated clip
//! and its frames are compared with the bytes of the same ffmpeg command run directly
//! (`-f rawvideo -`). If the crate sliced the stream at the wrong frame size, got a
//! palette or a plane wrong, or lost a byte, the concatenated frames would differ.
//! Every source is finite and tiny (0.3 s of 64x48 scaled to 33x17, three frames).

mod common;

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use neutral_ffmpeg::{Binaries, Decoder, Encoder, Filter, FrameBuf, PixFmt};

fn workers() -> usize {
    std::env::var("NEUTRAL_FFMPEG_JOBS").ok().and_then(|v| v.parse().ok()).unwrap_or(4usize).clamp(1, 16)
}

fn parallel(count: usize, job: impl Fn(usize) + Sync) {
    let next = AtomicUsize::new(0);
    std::thread::scope(|s| {
        for _ in 0..workers() {
            s.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::SeqCst);
                if i >= count {
                    break;
                }
                job(i);
            });
        }
    });
}

const SOURCE: &str = "testsrc2=s=64x48:r=10:d=0.3";

/// ffmpeg's own rawvideo bytes for `SOURCE` scaled to `w`x`h` in `fmt`.
fn direct(bins: &Binaries, fmt: PixFmt, w: u32, h: u32) -> common::Run {
    let vf = format!("scale={}:{}", w, h);
    common::ffmpeg(
        bins,
        &[
            "-f", "lavfi", "-i", SOURCE, "-vf", &vf, "-fps_mode", "passthrough", "-pix_fmt", fmt.name(), "-f",
            "rawvideo", "-",
        ],
    )
}

#[test]
fn decoder_output_equals_ffmpegs_rawvideo_for_every_supported_format() {
    let Some(bins) = common::require("decoder_output_equals_ffmpegs_rawvideo_for_every_supported_format") else {
        return;
    };
    let list: Vec<PixFmt> = PixFmt::all().filter(|f| f.is_supported()).collect();
    let failures = Mutex::new(Vec::new());
    parallel(list.len(), |i| {
        let fmt = list[i];
        let (w, h) = (33u32, 17u32);
        let want = direct(&bins, fmt, w, h);
        if !want.ok {
            failures.lock().unwrap().push(format!("{}: ffmpeg itself failed: {}", fmt, want.stderr));
            return;
        }
        let dec = Decoder::builder(SOURCE)
            .binaries(bins.clone())
            .input_args(["-f", "lavfi"])
            .pix_fmt(fmt)
            .size(w, h)
            .spawn();
        let frames: Result<Vec<FrameBuf>, _> = match dec {
            Ok(d) => d.collect(),
            Err(e) => {
                failures.lock().unwrap().push(format!("{}: decoder did not start: {}", fmt, e));
                return;
            }
        };
        let frames = match frames {
            Ok(f) => f,
            Err(e) => {
                failures.lock().unwrap().push(format!("{}: decoder failed: {}", fmt, e));
                return;
            }
        };
        let got: Vec<u8> = frames.iter().flat_map(|f| f.as_bytes().iter().copied()).collect();
        let frame_size = fmt.frame_size(w, h).unwrap();
        if frames.len() != 3 || got.len() != 3 * frame_size {
            failures.lock().unwrap().push(format!("{}: {} frames, {} bytes, expected 3 x {}", fmt, frames.len(), got.len(), frame_size));
        } else if got != want.stdout {
            failures.lock().unwrap().push(format!("{}: decoder bytes differ from ffmpeg's rawvideo output", fmt));
        }
    });
    let failures = failures.into_inner().unwrap();
    assert!(failures.is_empty(), "{} formats differ:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn automatic_size_equals_the_size_ffmpeg_produces() {
    let Some(bins) = common::require("automatic_size_equals_the_size_ffmpeg_produces") else { return };
    // No size given: the crate measures it, then must slice ffmpeg's output exactly.
    for (chain, fmt) in [
        ("scale=40:30", PixFmt::RGB24),
        ("crop=33:17", PixFmt::YUV420P),
        ("pad=70:50:3:1", PixFmt::GRAY),
        ("transpose=1", PixFmt::BGRA),
    ] {
        let dec = Decoder::builder(SOURCE)
            .binaries(bins.clone())
            .input_args(["-f", "lavfi"])
            .filter(chain)
            .pix_fmt(fmt)
            .spawn()
            .unwrap_or_else(|e| panic!("{}: {}", chain, e));
        let (w, h) = (dec.width(), dec.height());
        let frames: Vec<FrameBuf> = dec.collect::<Result<_, _>>().unwrap();
        let want = common::ffmpeg(
            &bins,
            &[
                "-f", "lavfi", "-i", SOURCE, "-vf", chain, "-fps_mode", "passthrough", "-pix_fmt", fmt.name(), "-f",
                "rawvideo", "-",
            ],
        );
        assert!(want.ok, "{}", want.stderr);
        let got: Vec<u8> = frames.iter().flat_map(|f| f.as_bytes().iter().copied()).collect();
        assert_eq!(want.stdout.len(), 3 * fmt.frame_size(w, h).unwrap(), "{}: measured size {}x{}", chain, w, h);
        assert!(got == want.stdout, "{}: bytes differ", chain);
    }
}

#[test]
fn file_decode_equals_ffmpegs_rawvideo() {
    let Some(bins) = common::require("file_decode_equals_ffmpegs_rawvideo") else { return };
    let dir = common::scratch("parity_file");
    let clip = dir.join("clip.mkv");
    let made = common::ffmpeg(
        &bins,
        &[
            "-y", "-f", "lavfi", "-i", "testsrc2=s=64x48:r=10:d=1", "-c:v", "ffv1", "-pix_fmt", "yuv420p",
            clip.to_str().unwrap(),
        ],
    );
    assert!(made.ok, "{}", made.stderr);
    for fmt in [PixFmt::YUV420P, PixFmt::RGB24, PixFmt::GRAY16LE, PixFmt::NV12] {
        let want = common::ffmpeg(
            &bins,
            &["-i", clip.to_str().unwrap(), "-fps_mode", "passthrough", "-pix_fmt", fmt.name(), "-f", "rawvideo", "-"],
        );
        assert!(want.ok, "{}", want.stderr);
        let frames: Vec<FrameBuf> = Decoder::builder(&clip)
            .binaries(bins.clone())
            .pix_fmt(fmt)
            .spawn()
            .unwrap()
            .collect::<Result<_, _>>()
            .unwrap();
        assert_eq!(frames.len(), 10, "{}", fmt);
        let got: Vec<u8> = frames.iter().flat_map(|f| f.as_bytes().iter().copied()).collect();
        assert!(got == want.stdout, "{}: file decode differs from ffmpeg", fmt);
    }
}

#[test]
fn encoder_output_file_equals_ffmpegs_own_encode() {
    let Some(bins) = common::require("encoder_output_file_equals_ffmpegs_own_encode") else { return };
    // The Encoder pipes raw frames; ffmpeg's own command reads the same bytes from a raw
    // file. Both write a rawvideo-in-nut file, which must be identical byte for byte.
    let dir = common::scratch("parity_encoder");
    let frames: Vec<FrameBuf> = Decoder::builder(SOURCE)
        .binaries(bins.clone())
        .input_args(["-f", "lavfi"])
        .pix_fmt(PixFmt::YUV420P)
        .size(33, 17)
        .spawn()
        .unwrap()
        .collect::<Result<_, _>>()
        .unwrap();
    let raw: Vec<u8> = frames.iter().flat_map(|f| f.as_bytes().iter().copied()).collect();
    let raw_path = dir.join("in.yuv");
    std::fs::write(&raw_path, &raw).unwrap();

    let ours = dir.join("ours.nut");
    let mut enc = Encoder::builder(&ours, PixFmt::YUV420P, 33, 17)
        .binaries(bins.clone())
        .fps(10, 1)
        .codec("rawvideo")
        .format("nut")
        .output_args(["-fflags", "+bitexact", "-flags", "+bitexact"])
        .spawn()
        .unwrap();
    for f in &frames {
        enc.write_frame(f).unwrap();
    }
    enc.finish().unwrap();

    let theirs = dir.join("theirs.nut");
    let run = common::ffmpeg(
        &bins,
        &[
            "-y", "-f", "rawvideo", "-pix_fmt", "yuv420p", "-video_size", "33x17", "-framerate", "10/1", "-i",
            raw_path.to_str().unwrap(), "-c:v", "rawvideo", "-f", "nut", "-fflags", "+bitexact", "-flags", "+bitexact",
            theirs.to_str().unwrap(),
        ],
    );
    assert!(run.ok, "{}", run.stderr);
    let a = std::fs::read(&ours).unwrap();
    let b = std::fs::read(&theirs).unwrap();
    assert!(a == b, "Encoder file ({} bytes) differs from ffmpeg's own ({} bytes)", a.len(), b.len());
}

#[test]
fn null_filter_identity_is_byte_exact_for_every_supported_format() {
    let Some(bins) = common::require("null_filter_identity_is_byte_exact_for_every_supported_format") else {
        return;
    };
    let list: Vec<PixFmt> = PixFmt::all().filter(|f| f.is_supported()).collect();
    let failures = Mutex::new(Vec::new());
    parallel(list.len(), |i| {
        let fmt = list[i];
        let (w, h) = (17u32, 9u32);
        let size = fmt.frame_size(w, h).unwrap();
        // Random bytes are not valid for every format (a palette, say, is data like any other
        // byte in the stream, and ffmpeg passes it through untouched), so build frames from the
        // source ffmpeg itself produces in this format.
        let want = direct(&bins, fmt, w, h);
        if !want.ok || want.stdout.len() != 3 * size {
            failures.lock().unwrap().push(format!("{}: reference run gave {} bytes", fmt, want.stdout.len()));
            return;
        }
        let frames: Vec<FrameBuf> =
            want.stdout.chunks(size).map(|c| FrameBuf::from_slice(fmt, w, h, c).unwrap()).collect();
        let out = Filter::builder(fmt, w, h)
            .binaries(bins.clone())
            .fps(10, 1)
            .graph("null")
            .spawn()
            .and_then(|f| f.run_all(frames.clone()));
        match out {
            Ok(out) if out == frames => {}
            Ok(out) => failures.lock().unwrap().push(format!("{}: {} frames came back, bytes differ", fmt, out.len())),
            Err(e) => failures.lock().unwrap().push(format!("{}: {}", fmt, e)),
        }
    });
    let failures = failures.into_inner().unwrap();
    assert!(failures.is_empty(), "{} formats not identical:\n{}", failures.len(), failures.join("\n"));
}
