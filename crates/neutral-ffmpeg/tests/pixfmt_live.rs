//! The pixel format table against the live ffmpeg and ffprobe.
//!
//! Every test skips with a printed reason when the binaries are not found.

mod common;

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::Path;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use neutral_ffmpeg::exec::run_capture;

use neutral_ffmpeg::pixfmt::{
    Endian, F_ALPHA, F_BIG_ENDIAN, F_BITSTREAM, F_HWACCEL, F_INPUT, F_OUTPUT, F_PALETTED, F_PLANAR, F_RGB,
};
use neutral_ffmpeg::{Binaries, PixFmt, PlaneKind};

/// The five sizes of the brief: tiny, even, odd, and two larger ones.
const SIZES: [(u32, u32); 5] = [(1, 1), (2, 2), (33, 17), (64, 48), (175, 99)];

/// Concurrent ffmpeg processes inside one sweep (default 4, NEUTRAL_FFMPEG_JOBS to change).
fn workers() -> usize {
    std::env::var("NEUTRAL_FFMPEG_JOBS")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .map(|n| n.clamp(1, 16))
        .unwrap_or(4)
}

/// The slice of `items` to work on. By default everything. With
/// `NEUTRAL_FFMPEG_CHUNK=k/n` only the k-th of n contiguous chunks (k counts from 0),
/// so the whole sweep can be run as several small cargo invocations.
/// The flag tells whether the whole list is covered.
fn chunk_of<T: Clone>(items: &[T]) -> (Vec<T>, bool) {
    let spec = match std::env::var("NEUTRAL_FFMPEG_CHUNK") {
        Ok(s) => s,
        Err(_) => return (items.to_vec(), true),
    };
    let (k, n) = spec
        .split_once('/')
        .and_then(|(k, n)| Some((k.trim().parse::<usize>().ok()?, n.trim().parse::<usize>().ok()?)))
        .filter(|&(k, n)| n > 0 && k < n)
        .unwrap_or_else(|| panic!("NEUTRAL_FFMPEG_CHUNK must look like 2/6 (chunk 2 of 6), got {:?}", spec));
    let per = items.len().div_ceil(n);
    let start = (k * per).min(items.len());
    let end = ((k + 1) * per).min(items.len());
    (items[start..end].to_vec(), n == 1)
}

/// Run `job(i)` for i in 0..count on a small pool.
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

#[derive(Debug, PartialEq, Eq)]
struct Row {
    name: String,
    input: bool,
    output: bool,
    hwaccel: bool,
    paletted: bool,
    bitstream: bool,
    nb_components: u32,
    bits_per_pixel: u32,
    depths: Vec<u32>,
}

/// Parse `ffmpeg -hide_banner -pix_fmts`.
fn live_pix_fmts(bins: &Binaries) -> Vec<Row> {
    let run = common::ffmpeg(bins, &["-pix_fmts"]);
    assert!(run.ok, "ffmpeg -pix_fmts failed: {}", run.stderr);
    let text = String::from_utf8_lossy(&run.stdout).into_owned();
    let mut rows = Vec::new();
    let mut started = false;
    for line in text.lines() {
        if !started {
            started = line.trim() == "-----";
            continue;
        }
        let mut it = line.split_whitespace();
        let flags = it.next().expect("flags column");
        let name = it.next().expect("name column");
        let nb: u32 = it.next().expect("components").parse().unwrap();
        let bpp: u32 = it.next().expect("bits per pixel").parse().unwrap();
        let depths: Vec<u32> = it.next().expect("bit depths").split('-').map(|d| d.parse().unwrap()).collect();
        let f: Vec<char> = flags.chars().collect();
        assert_eq!(f.len(), 5, "flag column of {}", name);
        rows.push(Row {
            name: name.to_string(),
            input: f[0] == 'I',
            output: f[1] == 'O',
            hwaccel: f[2] == 'H',
            paletted: f[3] == 'P',
            bitstream: f[4] == 'B',
            nb_components: nb,
            bits_per_pixel: bpp,
            depths,
        });
    }
    rows
}

#[test]
fn table_matches_live_pix_fmts() {
    let Some(bins) = common::require("table_matches_live_pix_fmts") else { return };
    let live = live_pix_fmts(&bins);
    let table: Vec<Row> = PixFmt::all()
        .map(|f| {
            let i = f.info();
            Row {
                name: i.name.to_string(),
                input: i.has(F_INPUT),
                output: i.has(F_OUTPUT),
                hwaccel: i.has(F_HWACCEL),
                paletted: i.has(F_PALETTED),
                bitstream: i.has(F_BITSTREAM),
                nb_components: i.nb_components as u32,
                bits_per_pixel: i.bits_per_pixel as u32,
                depths: i.bit_depths[..i.nb_components.max(1) as usize].iter().map(|&d| d as u32).collect(),
            }
        })
        .collect();
    let live_names: Vec<&str> = live.iter().map(|r| r.name.as_str()).collect();
    let table_names: Vec<&str> = table.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(
        live_names, table_names,
        "format list differs from the live ffmpeg; regenerate with scripts/gen_pixfmt.py probe, then generate"
    );
    for (l, t) in live.iter().zip(table.iter()) {
        assert_eq!(l, t, "descriptor mismatch for {}", l.name);
    }
}

/// Parse `ffprobe -show_pixel_formats` into name -> key/value map.
fn live_descriptors(bins: &Binaries) -> BTreeMap<String, BTreeMap<String, String>> {
    let exe = bins.ffprobe_path().expect("ffprobe path");
    let args: Vec<OsString> = ["-hide_banner", "-show_pixel_formats"].iter().map(OsString::from).collect();
    let out = run_capture(&exe, &args, Duration::from_secs(60), "ffprobe", "listing pixel formats").expect("ffprobe");
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let mut map = BTreeMap::new();
    let mut cur: Option<BTreeMap<String, String>> = None;
    for line in text.lines() {
        let line = line.trim();
        if line == "[PIXEL_FORMAT]" {
            cur = Some(BTreeMap::new());
        } else if line == "[/PIXEL_FORMAT]" {
            let m = cur.take().unwrap();
            map.insert(m["name"].clone(), m);
        } else if let Some(m) = cur.as_mut() {
            if let Some((k, v)) = line.split_once('=') {
                m.entry(k.trim_start_matches("FLAGS:").to_string()).or_insert_with(|| v.to_string());
            }
        }
    }
    map
}

#[test]
fn ffprobe_descriptors_agree_with_the_table() {
    let Some(bins) = common::require("ffprobe_descriptors_agree_with_the_table") else { return };
    let live = live_descriptors(&bins);
    assert_eq!(live.len(), PixFmt::count());
    for fmt in PixFmt::all() {
        let info = fmt.info();
        let d = live.get(info.name).unwrap_or_else(|| panic!("ffprobe does not list {}", info.name));
        let flag = |k: &str| d.get(k).map(|v| v == "1").unwrap_or(false);
        assert_eq!(flag("big_endian"), info.has(F_BIG_ENDIAN), "{} big_endian", info.name);
        assert_eq!(flag("planar"), info.has(F_PLANAR), "{} planar", info.name);
        assert_eq!(flag("rgb"), info.has(F_RGB), "{} rgb", info.name);
        assert_eq!(flag("alpha"), info.has(F_ALPHA), "{} alpha", info.name);
        assert_eq!(flag("palette"), info.has(F_PALETTED), "{} palette", info.name);
        assert_eq!(flag("bitstream"), info.has(F_BITSTREAM), "{} bitstream", info.name);
        assert_eq!(flag("hwaccel"), info.has(F_HWACCEL), "{} hwaccel", info.name);
        if info.is_supported() {
            let shift = |k: &str| d.get(k).and_then(|v| v.parse::<u8>().ok()).unwrap_or(0);
            assert_eq!(
                (shift("log2_chroma_w"), shift("log2_chroma_h")),
                (info.chroma_shift_w, info.chroma_shift_h),
                "{} chroma subsampling",
                info.name
            );
            let expect_endian = if !info.name.ends_with("le") && !info.name.ends_with("be") {
                Endian::None
            } else if info.has(F_BIG_ENDIAN) {
                Endian::Big
            } else {
                Endian::Little
            };
            assert_eq!(info.endian, expect_endian, "{} byte order", info.name);
        }
    }
}

#[test]
fn generated_file_is_what_the_script_produces_now() {
    let Some(bins) = common::require("generated_file_is_what_the_script_produces_now") else { return };
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let script = manifest.join("..").join("scripts").join("gen_pixfmt.py");
    let probe = manifest.join("..").join("scripts").join("pixfmt_probe.json");
    if !script.is_file() || !probe.is_file() {
        eprintln!("SKIPPED generated_file_is_what_the_script_produces_now: generator or probe data not found");
        return;
    }
    let out_dir = common::scratch("regenerate_table");
    let out_file = out_dir.join("pixfmt_table.rs");
    let args: Vec<OsString> = vec![
        "-I".into(),
        script.clone().into_os_string(),
        "generate".into(),
        "--ffmpeg".into(),
        bins.ffmpeg_path().unwrap().into_os_string(),
        "--ffprobe".into(),
        bins.ffprobe_path().unwrap().into_os_string(),
        "--probe".into(),
        probe.clone().into_os_string(),
        "--out".into(),
        out_file.clone().into_os_string(),
    ];
    // The generator only reads ffmpeg -pix_fmts and ffprobe -show_pixel_formats and the stored
    // probe data; it starts no media job. Hard deadline anyway.
    let run = match run_capture(Path::new("python"), &args, Duration::from_secs(120), "python", "regenerating the table") {
        Ok(r) => r,
        Err(neutral_ffmpeg::Error::Spawn { source, .. }) => {
            eprintln!("SKIPPED generated_file_is_what_the_script_produces_now: python could not be started ({})", source);
            return;
        }
        Err(e) => panic!("{}", e),
    };
    assert!(
        run.status.success(),
        "generator failed:\n{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let fresh = std::fs::read(&out_file).expect("generated file");
    let checked_in = std::fs::read(manifest.join("src").join("pixfmt_table.rs")).expect("checked-in table");
    assert!(
        fresh == checked_in,
        "src/pixfmt_table.rs is stale: run `python -I scripts\\gen_pixfmt.py generate` and commit the result"
    );
}

#[test]
fn frame_size_matches_ffmpeg_for_every_supported_format_and_size() {
    let Some(bins) = common::require("frame_size_matches_ffmpeg_for_every_supported_format_and_size") else {
        return;
    };
    let all: Vec<PixFmt> = PixFmt::all().filter(|f| f.is_supported()).collect();
    let (formats, whole) = chunk_of(&all);
    let jobs: Vec<(PixFmt, u32, u32)> =
        formats.iter().flat_map(|&f| SIZES.iter().map(move |&(w, h)| (f, w, h))).collect();
    if whole {
        assert!(jobs.len() >= 205 * 5, "expected the full matrix, got {} jobs", jobs.len());
    }
    let failures = Mutex::new(Vec::new());
    parallel(jobs.len(), |i| {
        let (fmt, w, h) = jobs[i];
        let run = common::reference_frame(&bins, fmt.name(), w, h);
        let expected = fmt.frame_size(w, h).expect("supported format has a size");
        if !run.ok || run.stdout.len() != expected {
            failures.lock().unwrap().push(format!(
                "{} {}x{}: ffmpeg {} bytes (ok={}, {}), table {}",
                fmt,
                w,
                h,
                run.stdout.len(),
                run.ok,
                run.stderr.lines().next().unwrap_or(""),
                expected
            ));
        }
    });
    let failures = failures.into_inner().unwrap();
    assert!(failures.is_empty(), "{} mismatches:\n{}", failures.len(), failures.join("\n"));
}

/// More awkward sizes than the five of the brief: single rows and columns, primes, sizes
/// that are odd in one direction only. Each was also part of the layout fit, but the
/// Rust function is checked against fresh ffmpeg output here.
const MORE_SIZES: [(u32, u32); 8] = [(2, 1), (1, 2), (3, 3), (5, 7), (13, 6), (9, 9), (255, 1), (1, 255)];

#[test]
fn frame_size_matches_ffmpeg_at_more_odd_sizes() {
    let Some(bins) = common::require("frame_size_matches_ffmpeg_at_more_odd_sizes") else { return };
    let all: Vec<PixFmt> = PixFmt::all().filter(|f| f.is_supported()).collect();
    let (formats, _) = chunk_of(&all);
    let jobs: Vec<(PixFmt, u32, u32)> =
        formats.iter().flat_map(|&f| MORE_SIZES.iter().map(move |&(w, h)| (f, w, h))).collect();
    let failures = Mutex::new(Vec::new());
    parallel(jobs.len(), |i| {
        let (fmt, w, h) = jobs[i];
        let run = common::reference_frame(&bins, fmt.name(), w, h);
        let expected = fmt.frame_size(w, h).expect("supported format has a size");
        if !run.ok || run.stdout.len() != expected {
            failures.lock().unwrap().push(format!(
                "{} {}x{}: ffmpeg {} bytes (ok={}), table {}",
                fmt,
                w,
                h,
                run.stdout.len(),
                run.ok,
                expected
            ));
        }
    });
    let failures = failures.into_inner().unwrap();
    assert!(failures.is_empty(), "{} mismatches:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn unsupported_formats_really_cannot_be_emitted() {
    let Some(bins) = common::require("unsupported_formats_really_cannot_be_emitted") else { return };
    let list: Vec<PixFmt> = PixFmt::all().filter(|f| !f.is_supported()).collect();
    assert!(list.len() >= 62);
    let failures = Mutex::new(Vec::new());
    parallel(list.len(), |i| {
        let fmt = list[i];
        let run = common::reference_frame(&bins, fmt.name(), 64, 48);
        if run.ok && !run.stdout.is_empty() {
            failures.lock().unwrap().push(format!(
                "{} is marked unsupported ({}) but ffmpeg emitted {} bytes",
                fmt,
                fmt.unsupported_reason().unwrap_or(""),
                run.stdout.len()
            ));
        }
    });
    let failures = failures.into_inner().unwrap();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

/// Unit size of the sample `fmt` stores first in plane 0.
fn first_sample(data: &[u8], word: usize, big: bool, float: bool) -> f64 {
    let chunk = &data[..word];
    if float {
        match (word, big) {
            (4, false) => f32::from_le_bytes(chunk.try_into().unwrap()) as f64,
            (4, true) => f32::from_be_bytes(chunk.try_into().unwrap()) as f64,
            _ => f64::NAN,
        }
    } else {
        let mut v = 0u64;
        if big {
            for &b in chunk {
                v = (v << 8) | b as u64;
            }
        } else {
            for &b in chunk.iter().rev() {
                v = (v << 8) | b as u64;
            }
        }
        v as f64
    }
}

/// True when every pixel group of the plane matches the first one, sample by sample, within
/// 1/64 of the first group's value (at least one step). swscale rounds the last pixels of a row
/// differently on some CPUs (gbrp 9x4 on arm64), while a wrong plane boundary, stride or byte
/// order moves a sample by far more: the planes of the test color differ by 30% or more. When
/// `edge_exempt` is set the last group of each row is not compared: swscale leaves the trailing
/// chroma sample unwritten (zero) for some semi-planar formats when the width is odd.
fn plane_is_uniform(plane: &[u8], p: &neutral_ffmpeg::PlaneLayout, edge_exempt: bool, word: usize, big: bool, float: bool) -> Result<(), String> {
    let step = p.group_bytes;
    let word = word.clamp(1, step);
    let first: Vec<f64> = (0..step / word).map(|k| first_sample(&plane[k * word..], word, big, float)).collect();
    let groups = if edge_exempt && p.cols > 1 { p.cols - 1 } else { p.cols };
    for row in 0..p.rows {
        let line = &plane[row * p.stride..(row + 1) * p.stride];
        for g in 0..groups {
            for (k, want) in first.iter().enumerate() {
                let got = first_sample(&line[g * step + k * word..], word, big, float);
                let slack = if float { (want.abs() / 64.0).max(1e-6) } else { (want.abs() / 64.0).max(1.0) };
                if (got - want).abs() > slack {
                    return Err(format!("row {} group {} sample {} is {}, the first group has {}", row, g, k, got, want));
                }
            }
        }
    }
    Ok(())
}

#[test]
fn planar_formats_have_uniform_planes_and_the_right_byte_order() {
    let Some(bins) = common::require("planar_formats_have_uniform_planes_and_the_right_byte_order") else {
        return;
    };
    // A constant color frame: every plane of a planar format must be uniform, which
    // only holds if the plane boundaries of the table are where ffmpeg puts them.
    // None of these sizes is one the layout was fitted on (the fit used 6x4 frames).
    let sizes = [(8u32, 6u32), (7, 5), (9, 4), (10, 3)];
    let emit = |fmt: &str, w: u32, h: u32| {
        let input = format!("color=c=0x3A7BC9:s={}x{}:d=1", w, h);
        common::ffmpeg(
            &bins,
            &["-f", "lavfi", "-i", &input, "-frames:v", "1", "-pix_fmt", fmt, "-f", "rawvideo", "-"],
        )
    };
    let luma8 = {
        let run = emit("gray", 8, 6);
        assert!(run.ok);
        run.stdout[0] as f64
    };
    let green8 = {
        let run = emit("gbrp", 8, 6);
        assert!(run.ok);
        run.stdout[0] as f64
    };
    let all: Vec<PixFmt> = PixFmt::all().filter(|f| f.is_supported() && f.nb_planes() >= 2).collect();
    let (list, whole) = chunk_of(&all);
    if whole {
        assert!(all.len() > 100, "only {} planar formats", all.len());
    }
    let jobs: Vec<(PixFmt, u32, u32)> =
        list.iter().flat_map(|&f| sizes.iter().map(move |&(w, h)| (f, w, h))).collect();
    let failures = Mutex::new(Vec::new());
    parallel(jobs.len(), |i| {
        let (fmt, w, h) = jobs[i];
        let run = emit(fmt.name(), w, h);
        let layout = fmt.plane_layout(w, h).unwrap();
        if !run.ok || run.stdout.len() != layout.size() {
            failures
                .lock()
                .unwrap()
                .push(format!("{} {}x{}: emitted {} bytes, layout {}", fmt, w, h, run.stdout.len(), layout.size()));
            return;
        }
        let info = fmt.info();
        let (word, big, float) = (info.sample_bytes as usize, info.endian == Endian::Big, fmt.name().contains("f32"));
        for (pi, p) in layout.planes().iter().enumerate() {
            if p.kind != PlaneKind::Samples {
                continue;
            }
            let plane = &run.stdout[p.offset..p.offset + p.size];
            let exempt = w % p.group_width != 0;
            if let Err(why) = plane_is_uniform(plane, p, exempt, word, big, float) {
                failures.lock().unwrap().push(format!("{} {}x{} plane {}: {}", fmt, w, h, pi, why));
            }
        }
        // Byte order of the first sample of plane 0, for 2 and 4 byte samples.
        let info = fmt.info();
        let word = info.sample_bytes as usize;
        if word >= 2 {
            let big = info.endian == Endian::Big;
            let is_float = fmt.name().contains("f32");
            let reference = if fmt.is_rgb() { green8 } else { luma8 };
            let data = &run.stdout[layout.planes()[0].offset..];
            let value = first_sample(data, word, big, is_float);
            let depth = info.bit_depths[0] as u32;
            let ok = if is_float {
                (value * 255.0 - reference).abs() <= 3.0
            } else {
                let lsb_max = ((1u64 << depth) - 1) as f64;
                let msb_max = ((1u64 << (8 * word as u32)) - 1) as f64;
                (value / lsb_max * 255.0 - reference).abs() <= 3.0 || (value / msb_max * 255.0 - reference).abs() <= 3.0
            };
            if !ok {
                failures.lock().unwrap().push(format!(
                    "{} {}x{}: first sample {} does not decode to level {} as {:?}",
                    fmt, w, h, value, reference, info.endian
                ));
            }
        }
    });
    let failures = failures.into_inner().unwrap();
    assert!(failures.is_empty(), "{} problems:\n{}", failures.len(), failures.join("\n"));
}

#[test]
fn gray_byte_order_is_visible_in_the_data() {
    let Some(bins) = common::require("gray_byte_order_is_visible_in_the_data") else { return };
    let emit = |fmt: &str| {
        let run = common::ffmpeg(
            &bins,
            &["-f", "lavfi", "-i", "color=c=0x3A7BC9:s=2x2:d=1", "-frames:v", "1", "-pix_fmt", fmt, "-f", "rawvideo", "-"],
        );
        assert!(run.ok, "{}", run.stderr);
        run.stdout
    };
    let level = emit("gray")[0];
    let le = emit("gray16le");
    let be = emit("gray16be");
    assert_eq!(PixFmt::GRAY16LE.endian(), Endian::Little);
    assert_eq!(PixFmt::GRAY16BE.endian(), Endian::Big);
    assert_eq!(le[1], level, "little endian: most significant byte second");
    assert_eq!(be[0], level, "big endian: most significant byte first");
    assert_eq!(le[0], be[1]);
}

#[test]
fn pal8_palette_sits_after_the_pixels() {
    let Some(bins) = common::require("pal8_palette_sits_after_the_pixels") else { return };
    let run = common::reference_frame(&bins, "pal8", 33, 17);
    assert!(run.ok, "{}", run.stderr);
    let layout = PixFmt::PAL8.plane_layout(33, 17).unwrap();
    assert_eq!(run.stdout.len(), layout.size());
    let palette = layout.planes()[1];
    assert_eq!(palette.kind, PlaneKind::Palette);
    assert_eq!(palette.offset, 33 * 17, "no padding between pixels and palette");
    let entries = &run.stdout[palette.offset..palette.offset + palette.size];
    assert!(entries.chunks(4).all(|e| e[3] == 0xff), "default palette is opaque");
    assert!(entries.iter().any(|&b| b != 0));
}

#[test]
fn accepted_alias_spellings_emit_the_bytes_of_their_target() {
    let Some(bins) = common::require("accepted_alias_spellings_emit_the_bytes_of_their_target") else { return };
    // Every alias the table knows is either the little-endian spelling (name without
    // the "le" suffix) or one of the few fixed ones.
    let mut cases: Vec<(String, PixFmt)> = Vec::new();
    for fmt in PixFmt::all().filter(|f| f.is_supported()) {
        if let Some(stem) = fmt.name().strip_suffix("le") {
            if let Some(found) = PixFmt::from_name(stem) {
                assert_eq!(found, fmt, "{} should resolve to {}", stem, fmt);
                cases.push((stem.to_string(), fmt));
            }
        }
    }
    cases.push(("rgb32".to_string(), PixFmt::from_name("rgb32").expect("rgb32 alias")));
    cases.push(("bgr32".to_string(), PixFmt::from_name("bgr32").expect("bgr32 alias")));
    cases.push(("gray8".to_string(), PixFmt::from_name("gray8").expect("gray8 alias")));
    assert!(cases.len() > 50);
    let (cases, _) = chunk_of(&cases);
    let failures = Mutex::new(Vec::new());
    parallel(cases.len(), |i| {
        let (alias, target) = &cases[i];
        let a = common::reference_frame(&bins, alias, 6, 4);
        let t = common::reference_frame(&bins, target.name(), 6, 4);
        if !a.ok || !t.ok || a.stdout != t.stdout {
            failures.lock().unwrap().push(format!("alias {} differs from {}: {}", alias, target, a.stderr));
        }
    });
    let failures = failures.into_inner().unwrap();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
