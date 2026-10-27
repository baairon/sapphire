//! The pipeline against twixtor's `tools/apply.py`: for the same input, effect and flags, the same
//! plan, the same command lines, the same refusals and the same frames.
//!
//! - `frames`: both tools run the real `vfx` with `--lossless` (ffv1, so that the output is exact)
//!   on a 160x96, 12 frame clip at depth 8 and 16, for the frame effect `selftest-invert` and for
//!   `retime` with `speed_percent=50`; the two outputs are decoded to raw frames and compared byte
//!   for byte. The lossy path is compared by stream facts and a tolerance, because libx264 output
//!   is not reproducible (two runs of apply.py alone differ).
//! - `dry-run`: `--dry-run` text and `--json` of both tools, line by line.
//! - `refusals`: the message of every refusal that comes before a frame moves, against apply.py
//!   with the fake `vfx` (needs no real `vfx`).
//!
//! Every comparison is a row of the table written to `$HANDOFF_PARITY_TSV` (default: a file in the
//! target folder): `check case effect params depth mode apply_py_command neutral_handoff_command
//! apply_py_exit neutral_handoff_exit frames_apply_py frames_neutral_handoff frame_bytes equal
//! first_difference fnv1a64_apply_py fnv1a64_neutral_handoff note`.
//!
//! Skips with a printed reason when ffmpeg, `vfx`, Python or `apply.py` is missing.

mod common;

use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::Duration;

use common::*;
use neutral_ffmpeg::json::Json;
use neutral_handoff::media::probe;

static ROWS: Mutex<Vec<Vec<String>>> = Mutex::new(Vec::new());
static FAILURES: Mutex<Vec<String>> = Mutex::new(Vec::new());

const HEADER: [&str; 18] = [
    "check",
    "case",
    "effect",
    "params",
    "depth",
    "mode",
    "apply_py_command",
    "neutral_handoff_command",
    "apply_py_exit",
    "neutral_handoff_exit",
    "frames_apply_py",
    "frames_neutral_handoff",
    "frame_bytes",
    "equal",
    "first_difference",
    "fnv1a64_apply_py",
    "fnv1a64_neutral_handoff",
    "note",
];

fn tsv_path() -> PathBuf {
    std::env::var_os("HANDOFF_PARITY_TSV").map(PathBuf::from).unwrap_or_else(|| Path::new(env!("CARGO_TARGET_TMPDIR")).join("parity.tsv"))
}

fn work_dir(name: &str) -> PathBuf {
    let base = std::env::var_os("HANDOFF_PARITY_DIR").map(PathBuf::from).unwrap_or_else(|| Path::new(env!("CARGO_TARGET_TMPDIR")).join("parity"));
    let dir = base.join(name);
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_table() {
    let rows = ROWS.lock().unwrap();
    let mut text = HEADER.join("\t");
    text.push('\n');
    for row in rows.iter() {
        assert_eq!(row.len(), HEADER.len(), "{:?}", row);
        text.push_str(&row.iter().map(|c| c.replace(['\t', '\n', '\r'], " ")).collect::<Vec<_>>().join("\t"));
        text.push('\n');
    }
    if let Some(parent) = tsv_path().parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(tsv_path(), text).expect("write the parity table");
}

fn fail(message: String) {
    FAILURES.lock().unwrap().push(message);
}

struct Tools {
    ffmpeg: PathBuf,
    ffprobe: PathBuf,
    python: String,
    apply_py: PathBuf,
}

impl Tools {
    fn find(test: &str, need_vfx: bool) -> Option<(Tools, Option<PathBuf>)> {
        let (ffmpeg, ffprobe) = require_media(test)?;
        let (python, apply_py) = require_apply_py(test)?;
        let vfx = if need_vfx { Some(require_vfx(test)?) } else { None };
        Some((Tools { ffmpeg, ffprobe, python, apply_py }, vfx))
    }

    /// apply.py with `args`, naming the same ffmpeg and ffprobe as the pipeline does.
    fn apply(&self, args: &[String], vfx: &Path) -> Ran {
        let mut a = vec![self.apply_py.to_string_lossy().into_owned()];
        a.extend(args.iter().cloned());
        a.extend(self.paths(vfx));
        run_env(Path::new(&self.python), &a, &[], None, Duration::from_secs(300))
    }

    fn ours(&self, args: &[String], vfx: &Path) -> Ran {
        let mut a = args.to_vec();
        a.extend(self.paths(vfx));
        run_env(&handoff_exe(), &a, &[], None, Duration::from_secs(300))
    }

    fn paths(&self, vfx: &Path) -> Vec<String> {
        vec![
            "--ffmpeg-path".into(),
            self.ffmpeg.to_string_lossy().into_owned(),
            "--ffprobe-path".into(),
            self.ffprobe.to_string_lossy().into_owned(),
            "--vfx".into(),
            vfx.to_string_lossy().into_owned(),
        ]
    }
}

fn shorten(text: &str, dir: &Path) -> String {
    text.replace(&dir.to_string_lossy().into_owned(), "<work>")
}

fn strings(list: &[&str]) -> Vec<String> {
    list.iter().map(|s| s.to_string()).collect()
}

fn first_difference(a: &[u8], b: &[u8], frame_bytes: usize) -> String {
    match a.iter().zip(b).position(|(x, y)| x != y) {
        Some(i) => format!("frame {} byte {}", i / frame_bytes, i % frame_bytes),
        None if a.len() != b.len() => format!("lengths {} and {}", a.len(), b.len()),
        None => "none".to_string(),
    }
}

/// The facts of an output that must be the same whichever tool wrote it.
fn facts(ffprobe: &Path, path: &Path) -> String {
    let info = probe(ffprobe, &path.to_string_lossy()).expect("probe the output");
    format!(
        "{:?} {:?} {}x{} {:?} frames={:?} audio={:?}/{:?} dur={:.2} tags={:?}/{:?}/{:?}",
        info.vcodec,
        info.pix_fmt,
        info.width.unwrap(),
        info.height.unwrap(),
        info.rate.map(|r| r.text()),
        info.nb_frames,
        info.acodec,
        info.channels,
        info.duration.unwrap_or(0.0),
        info.color_space,
        info.color_primaries,
        info.color_transfer
    )
}

#[test]
fn parity_with_apply_py() {
    let Some((tools, _)) = Tools::find("parity_with_apply_py", false) else { return };
    if let Some(vfx) = require_vfx("parity_with_apply_py (frames and dry runs)") {
        frame_and_plan_rows(&tools, &vfx);
    }
    refusal_rows(&tools);
    write_table();
    let failures = FAILURES.lock().unwrap();
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

fn frame_and_plan_rows(tools: &Tools, vfx: &Path) {
    let vfx = vfx.to_path_buf();
    let dir = work_dir("main");
    let clip = make_clip(&dir, "clip.mp4", 160, 96, 12, 24, 0.6);
    let silent = make_clip(&dir, "silent.mp4", 160, 96, 12, 24, 0.0);
    let layer_clip = make_clip(&dir, "track.mp4", 80, 48, 6, 12, 0.0);
    let odd = {
        let path = dir.join("odd.mkv");
        let a: Vec<std::ffi::OsString> = ["-v", "error", "-nostdin", "-y", "-f", "lavfi", "-i", "testsrc2=size=33x17:rate=24:duration=0.5", "-frames:v", "12", "-c:v", "ffv1", "-pix_fmt", "yuv444p"]
            .iter()
            .map(std::ffi::OsString::from)
            .chain([path.clone().into_os_string()])
            .collect();
        let r = neutral_ffmpeg::exec::run_capture(&tools.ffmpeg, &a, Duration::from_secs(60), "ffmpeg", "odd clip").unwrap();
        assert!(r.status.success(), "{}", r.stderr_text());
        path
    };

    // ---- lossless: decoded frames equal, byte for byte
    struct Case {
        name: &'static str,
        input: PathBuf,
        effect: &'static str,
        params: Vec<&'static str>,
        extra: Vec<String>,
        depth: u32,
        bytes_per_pixel: usize,
        pix_fmt: &'static str,
    }
    let mut cases: Vec<Case> = Vec::new();
    for (effect, params) in [("selftest-invert", vec![]), ("retime", vec!["speed_percent=50"])] {
        for (depth, bpp, fmt) in [(8, 4, "rgba"), (16, 8, "rgba64le")] {
            cases.push(Case { name: "clip", input: clip.clone(), effect, params: params.clone(), extra: vec![], depth, bytes_per_pixel: bpp, pix_fmt: fmt });
        }
    }
    cases.push(Case { name: "clip-frames-6", input: clip.clone(), effect: "retime", params: vec!["speed_percent=50"], extra: vec!["--frames".to_string(), "6".to_string()], depth: 16, bytes_per_pixel: 8, pix_fmt: "rgba64le" });
    cases.push(Case { name: "silent", input: silent.clone(), effect: "selftest-invert", params: vec![], extra: vec![], depth: 8, bytes_per_pixel: 4, pix_fmt: "rgba" });
    cases.push(Case { name: "odd-33x17", input: odd.clone(), effect: "selftest-invert", params: vec![], extra: vec![], depth: 16, bytes_per_pixel: 8, pix_fmt: "rgba64le" });
    // an untagged HD source (decoded with the bt709 matrix by the size rule) and a source with a display rotation
    let hd = make_clip(&dir, "hd.mp4", 1920, 1080, 30, 30, 0.0);
    cases.push(Case { name: "hd-untagged", input: hd.clone(), effect: "selftest-invert", params: vec![], extra: vec![], depth: 8, bytes_per_pixel: 4, pix_fmt: "rgba" });
    let rotated = {
        let path = dir.join("rotated.mp4");
        let a: Vec<std::ffi::OsString> = ["-v", "error", "-nostdin", "-y", "-display_rotation", "90", "-i"]
            .iter()
            .map(std::ffi::OsString::from)
            .chain([clip.clone().into_os_string()])
            .chain(["-c", "copy"].iter().map(std::ffi::OsString::from))
            .chain([path.clone().into_os_string()])
            .collect();
        let r = neutral_ffmpeg::exec::run_capture(&tools.ffmpeg, &a, Duration::from_secs(60), "ffmpeg", "rotated clip").unwrap();
        assert!(r.status.success(), "{}", r.stderr_text());
        path
    };
    cases.push(Case { name: "rotated-90", input: rotated, effect: "selftest-invert", params: vec![], extra: vec![], depth: 8, bytes_per_pixel: 4, pix_fmt: "rgba" });
    // a temporal effect with an extra layer of another size and rate (decoded to a raw clip, scaled and resampled)
    cases.push(Case {
        name: "layer-80x48-12fps",
        input: clip.clone(),
        effect: "retime-fgbg",
        params: vec!["speed_percent=50", "warping=3"],
        extra: vec!["--layer".to_string(), format!("alt_source_b={}", layer_clip.display())],
        depth: 8,
        bytes_per_pixel: 4,
        pix_fmt: "rgba",
    });
    cases.push(Case { name: "clip-invert-alpha", input: clip.clone(), effect: "selftest-invert", params: vec!["invert_alpha=1"], extra: vec![], depth: 8, bytes_per_pixel: 4, pix_fmt: "rgba" });

    for (n, case) in cases.iter().enumerate() {
        let py_out = dir.join(format!("py_{}.mkv", n));
        let rs_out = dir.join(format!("rs_{}.mkv", n));
        let build = |out: &Path| -> Vec<String> {
            let mut a = vec![case.input.to_string_lossy().into_owned(), case.effect.to_string()];
            for p in &case.params {
                a.push("--param".into());
                a.push(p.to_string());
            }
            a.extend(case.extra.iter().cloned());
            a.extend(strings(&["--depth", &case.depth.to_string(), "--lossless", "-o", &out.to_string_lossy(), "--force"]));
            a
        };
        let py = tools.apply(&build(&py_out), &vfx);
        let rs = tools.ours(&build(&rs_out), &vfx);
        let (w, h) = match case.name {
            "odd-33x17" => (33usize, 17usize),
            "hd-untagged" => (1920, 1080),
            "rotated-90" => (96, 160),
            _ => (160, 96),
        };
        let frame_bytes = w * h * case.bytes_per_pixel;
        let (mut a, mut b) = (Vec::new(), Vec::new());
        if py.code == 0 && rs.code == 0 {
            a = decode_raw(&py_out, case.pix_fmt);
            b = decode_raw(&rs_out, case.pix_fmt);
        }
        let same_facts = py.code == 0 && rs.code == 0 && facts(&tools.ffprobe, &py_out) == facts(&tools.ffprobe, &rs_out);
        let equal = py.code == 0 && rs.code == 0 && a == b && !a.is_empty() && same_facts;
        let mut cmd_py = vec!["python".to_string(), "tools/apply.py".to_string()];
        cmd_py.extend(build(&py_out));
        let mut cmd_rs = vec!["sapphire".to_string()];
        cmd_rs.extend(build(&rs_out));
        ROWS.lock().unwrap().push(vec![
            "frames".into(),
            case.name.into(),
            case.effect.into(),
            case.params.join(";"),
            case.depth.to_string(),
            "lossless".into(),
            shorten(&cmd_py.join(" "), &dir),
            shorten(&cmd_rs.join(" "), &dir),
            py.code.to_string(),
            rs.code.to_string(),
            (a.len() / frame_bytes).to_string(),
            (b.len() / frame_bytes).to_string(),
            frame_bytes.to_string(),
            equal.to_string(),
            first_difference(&a, &b, frame_bytes),
            format!("{:016x}", fnv1a64(&a)),
            format!("{:016x}", fnv1a64(&b)),
            format!("stream facts equal: {}", same_facts),
        ]);
        if !equal {
            fail(format!("frames {} {} d{}: apply.py exit {}, pipeline exit {}: {} {}", case.name, case.effect, case.depth, py.code, rs.code, py.stderr, rs.stderr));
        }
    }

    // ---- lossy: stream facts equal, decoded frames within the noise of x264 itself
    for (effect, params, depth) in [("selftest-invert", vec![], 16u32), ("selftest-invert", vec![], 32), ("retime", vec!["speed_percent=50"], 8)] {
        let build = |out: &Path| -> Vec<String> {
            let mut a = vec![clip.to_string_lossy().into_owned(), effect.to_string()];
            for p in &params {
                a.push("--param".into());
                a.push(p.to_string());
            }
            a.extend(strings(&["--depth", &depth.to_string(), "-o", &out.to_string_lossy(), "--force"]));
            a
        };
        let (p1, p2, r1) = (dir.join(format!("lossy_py1_{}_{}.mp4", effect, depth)), dir.join(format!("lossy_py2_{}_{}.mp4", effect, depth)), dir.join(format!("lossy_rs_{}_{}.mp4", effect, depth)));
        let a1 = tools.apply(&build(&p1), &vfx);
        let a2 = tools.apply(&build(&p2), &vfx);
        let rs = tools.ours(&build(&r1), &vfx);
        assert_eq!((a1.code, a2.code, rs.code), (0, 0, 0), "{} {} {}", a1.stderr, a2.stderr, rs.stderr);
        let (d1, d2, dr) = (decode_raw(&p1, "rgb24"), decode_raw(&p2, "rgb24"), decode_raw(&r1, "rgb24"));
        let stat = |x: &[u8], y: &[u8]| -> (u32, f64) {
            let mut max = 0u32;
            let mut sum = 0u64;
            for (p, q) in x.iter().zip(y) {
                let d = (*p as i32 - *q as i32).unsigned_abs();
                max = max.max(d);
                sum += d as u64;
            }
            (max, sum as f64 / x.len().max(1) as f64)
        };
        let (max_noise, mean_noise) = stat(&d1, &d2);
        let (max_diff, mean_diff) = stat(&d1, &dr);
        let facts_equal = facts(&tools.ffprobe, &p1) == facts(&tools.ffprobe, &r1);
        let same_len = d1.len() == dr.len() && d1.len() == d2.len();
        let ok = facts_equal && same_len && mean_diff <= 0.5;
        let mut cmd_py = vec!["python".to_string(), "tools/apply.py".to_string()];
        cmd_py.extend(build(&p1));
        let mut cmd_rs = vec!["sapphire".to_string()];
        cmd_rs.extend(build(&r1));
        let frame_bytes = 160 * 96 * 3;
        ROWS.lock().unwrap().push(vec![
            "frames".into(),
            "clip".into(),
            effect.into(),
            params.join(";"),
            depth.to_string(),
            "lossy".into(),
            shorten(&cmd_py.join(" "), &dir),
            shorten(&cmd_rs.join(" "), &dir),
            a1.code.to_string(),
            rs.code.to_string(),
            (d1.len() / frame_bytes).to_string(),
            (dr.len() / frame_bytes).to_string(),
            frame_bytes.to_string(),
            ok.to_string(),
            format!("pipeline vs apply.py: max {} mean {:.3}; apply.py vs itself: max {} mean {:.3}", max_diff, mean_diff, max_noise, mean_noise),
            format!("{:016x}", fnv1a64(&d1)),
            format!("{:016x}", fnv1a64(&dr)),
            format!("decoded rgb24; libx264 is not reproducible, so equal means: stream facts equal ({}) and mean absolute difference at most 0.5", facts_equal),
        ]);
        if !ok {
            fail(format!("lossy {} d{}: facts equal {}, max {} mean {:.3}", effect, depth, facts_equal, max_diff, mean_diff));
        }
    }

    // ---- dry runs: the same text, the same JSON
    let clip_s = clip.to_string_lossy().into_owned();
    let dry_cases: Vec<(&str, Vec<String>)> = vec![
        ("invert", strings(&[&clip_s, "selftest-invert"])),
        ("retime-d8", strings(&[&clip_s, "retime", "--param", "speed_percent=50", "--depth", "8"])),
        ("retime-keys", strings(&[&clip_s, "retime", "--param", "speed_percent=100@0;50@24", "--frames", "90"])),
        ("retime-frames-6-lossless", strings(&[&clip_s, "retime", "--frames", "6", "--lossless", "--depth", "8", "-o", &dir.join("x.mkv").to_string_lossy()])),
        ("retime-fgbg-layer", strings(&[&clip_s, "retime-fgbg", "--param", "warping=3", "--layer", &format!("alt_source_b={}", layer_clip.display())])),
        ("odd-lossless", strings(&[&odd.to_string_lossy(), "selftest-invert", "--lossless"])),
        ("silent", strings(&[&silent.to_string_lossy(), "selftest-invert", "--depth", "32"])),
    ];
    for (name, base) in dry_cases {
        let mut args = base.clone();
        args.push("--dry-run".into());
        let py = tools.apply(&args, &vfx);
        let rs = tools.ours(&args, &vfx);
        let norm = |t: &str| -> String { normalise_pid(&t.replace("\r\n", "\n")) };
        let (py_text, rs_text) = (norm(&py.out_text()), norm(&rs.out_text()));
        let equal = py.code == 0 && rs.code == 0 && py_text == rs_text;
        let mut cmd_py = vec!["python".to_string(), "tools/apply.py".to_string()];
        cmd_py.extend(args.clone());
        let mut cmd_rs = vec!["sapphire".to_string()];
        cmd_rs.extend(args.clone());
        ROWS.lock().unwrap().push(vec![
            "dry-run".into(),
            name.into(),
            args[1].clone(),
            String::new(),
            String::new(),
            "text".into(),
            shorten(&cmd_py.join(" "), &dir),
            shorten(&cmd_rs.join(" "), &dir),
            py.code.to_string(),
            rs.code.to_string(),
            py_text.lines().count().to_string(),
            rs_text.lines().count().to_string(),
            String::new(),
            equal.to_string(),
            first_line_difference(&py_text, &rs_text),
            format!("{:016x}", fnv1a64(py_text.as_bytes())),
            format!("{:016x}", fnv1a64(rs_text.as_bytes())),
            "[plan], [temp], [cmd] lines and the last line, partial-<pid> normalised".into(),
        ]);
        if !equal {
            fail(format!("dry-run {}: {}\n--- apply.py\n{}\n--- pipeline\n{}\n{}", name, first_line_difference(&py_text, &rs_text), py_text, rs_text, rs.stderr));
        }

        // JSON: every key of apply.py's document with the same value
        let mut jargs = args.clone();
        jargs.push("--json".into());
        let (pj, rj) = (tools.apply(&jargs, &vfx), tools.ours(&jargs, &vfx));
        let (pd, rd) = (Json::parse(&norm(&pj.out_text())), Json::parse(&norm(&rj.out_text())));
        let (mut json_equal, mut extra_keys, mut diff) = (false, Vec::new(), String::from("not JSON"));
        if let (Ok(pd), Ok(rd)) = (&pd, &rd) {
            json_equal = true;
            diff = "none".into();
            for (key, value) in pd.as_object().unwrap() {
                if rd.get(key) != Some(value) {
                    json_equal = false;
                    diff = format!("key {}", key);
                    break;
                }
            }
            extra_keys = rd.as_object().unwrap().iter().map(|(k, _)| k.clone()).filter(|k| pd.get(k).is_none()).collect();
        }
        let json_ok = pj.code == 0 && rj.code == 0 && json_equal;
        ROWS.lock().unwrap().push(vec![
            "dry-run".into(),
            name.into(),
            args[1].clone(),
            String::new(),
            String::new(),
            "json".into(),
            shorten(&format!("python tools/apply.py {} ", jargs.join(" ")), &dir),
            shorten(&format!("sapphire {}", jargs.join(" ")), &dir),
            pj.code.to_string(),
            rj.code.to_string(),
            String::new(),
            String::new(),
            String::new(),
            json_ok.to_string(),
            diff.clone(),
            String::new(),
            String::new(),
            format!("every key of apply.py's document has the same value; keys only the pipeline adds: {}", extra_keys.join(",")),
        ]);
        if !json_ok {
            fail(format!("dry-run json {}: {}", name, diff));
        }
    }

}

fn normalise_pid(text: &str) -> String {
    let mut out = String::new();
    let mut rest = text;
    while let Some(at) = rest.find(".partial-") {
        out.push_str(&rest[..at + ".partial-".len()]);
        rest = &rest[at + ".partial-".len()..];
        let digits = rest.chars().take_while(|c| c.is_ascii_digit()).count();
        out.push_str("PID");
        rest = &rest[digits..];
    }
    out.push_str(rest);
    out
}

fn first_line_difference(a: &str, b: &str) -> String {
    for (i, (x, y)) in a.lines().zip(b.lines()).enumerate() {
        if x != y {
            return format!("line {}", i + 1);
        }
    }
    if a.lines().count() != b.lines().count() {
        return format!("line counts {} and {}", a.lines().count(), b.lines().count());
    }
    "none".to_string()
}

/// The message of every refusal that comes before a frame moves, against apply.py (with the fake vfx).
fn refusal_rows(tools: &Tools) {
    let dir = work_dir("refusals");
    let fake = fake_vfx();
    let clip = make_clip(&dir, "clip.mp4", 160, 96, 12, 24, 0.6);
    let odd = {
        let path = dir.join("odd.mkv");
        let a: Vec<std::ffi::OsString> = ["-v", "error", "-nostdin", "-y", "-f", "lavfi", "-i", "color=c=red:size=33x17:rate=10:duration=0.5", "-c:v", "ffv1", "-pix_fmt", "yuv444p"]
            .iter()
            .map(std::ffi::OsString::from)
            .chain([path.clone().into_os_string()])
            .collect();
        neutral_ffmpeg::exec::run_capture(&tools.ffmpeg, &a, Duration::from_secs(60), "ffmpeg", "odd clip").unwrap();
        path
    };
    let junk = dir.join("junk.mp4");
    std::fs::write(&junk, b"junk").unwrap();
    let exists = dir.join("exists.mkv");
    std::fs::write(&exists, b"keep").unwrap();
    let missing = dir.join("missing.mp4");
    let (clip_s, odd_s, junk_s, exists_s, missing_s) = (
        clip.to_string_lossy().into_owned(),
        odd.to_string_lossy().into_owned(),
        junk.to_string_lossy().into_owned(),
        exists.to_string_lossy().into_owned(),
        missing.to_string_lossy().into_owned(),
    );
    let table: Vec<(&str, Vec<String>)> = vec![
        ("no-such-file", strings(&[&missing_s, "fake-copy"])),
        ("not-media", strings(&[&junk_s, "fake-copy"])),
        ("unknown-effect", strings(&[&clip_s, "fake-nope"])),
        ("bad-param-value", strings(&[&clip_s, "fake-copy", "--param", "level=500"])),
        ("unknown-param", strings(&[&clip_s, "fake-copy", "--param", "nope=1"])),
        ("frames-on-frame-effect", strings(&[&clip_s, "fake-copy", "--frames", "5"])),
        ("layer-on-frame-effect", strings(&[&clip_s, "fake-copy", "--layer", "a=b.mp4"])),
        ("layer-missing", strings(&[&clip_s, "fake-layer"])),
        ("layer-unknown", strings(&[&clip_s, "fake-layer", "--layer", "nope=x.mp4"])),
        ("layer-not-a-file", strings(&[&clip_s, "fake-layer", "--layer", &format!("overlay={}", missing_s)])),
        ("lossless-depth-32", strings(&[&clip_s, "fake-copy", "--lossless", "--depth", "32"])),
        ("odd-sides", strings(&[&odd_s, "fake-copy"])),
        ("output-exists", strings(&[&clip_s, "fake-copy", "-o", &exists_s])),
        ("output-is-input", strings(&[&clip_s, "fake-copy", "-o", &clip_s])),
        ("vfx-missing", strings(&[&clip_s, "fake-copy", "--vfx", "no/such/vfx"])),
    ];
    let mut compared = 0;
    for (name, mut args) in table {
        let mut py_args = args.clone();
        let mut rs_args = args.clone();
        // the vfx flag is given by the table in one case; otherwise the fake one
        let has_vfx = args.iter().any(|a| a == "--vfx");
        if has_vfx {
            args.retain(|a| a != "--vfx");
        }
        let (py, rs) = if has_vfx {
            py_args.extend(strings(&["--ffmpeg-path", &tools.ffmpeg.to_string_lossy(), "--ffprobe-path", &tools.ffprobe.to_string_lossy()]));
            rs_args.extend(strings(&["--ffmpeg-path", &tools.ffmpeg.to_string_lossy(), "--ffprobe-path", &tools.ffprobe.to_string_lossy()]));
            let mut a = vec![tools.apply_py.to_string_lossy().into_owned()];
            a.extend(py_args.clone());
            (run_env(Path::new(&tools.python), &a, &[], None, Duration::from_secs(120)), run_env(&handoff_exe(), &rs_args, &[], None, Duration::from_secs(120)))
        } else {
            (tools.apply(&py_args, &fake), tools.ours(&rs_args, &fake))
        };
        let py_msg = py.stderr.split("[error] ").nth(1).map(|m| m.trim().to_string()).unwrap_or_default();
        let rs_line = rs.stderr.lines().next().unwrap_or("").to_string();
        let rs_msg = rs_line.strip_prefix("error: ").and_then(|l| l.split_once(": ")).map(|(_, m)| m.to_string()).unwrap_or_default();
        let equal = !py_msg.is_empty() && py_msg == rs_msg && py.code == 1 && rs.code != 0;
        let mut cmd_py = vec!["python".to_string(), "tools/apply.py".to_string()];
        cmd_py.extend(py_args.clone());
        let mut cmd_rs = vec!["sapphire".to_string()];
        cmd_rs.extend(rs_args.clone());
        ROWS.lock().unwrap().push(vec![
            "refusal".into(),
            name.into(),
            args[1].clone(),
            String::new(),
            String::new(),
            "message".into(),
            shorten(&cmd_py.join(" "), &dir),
            shorten(&cmd_rs.join(" "), &dir),
            py.code.to_string(),
            rs.code.to_string(),
            String::new(),
            String::new(),
            String::new(),
            equal.to_string(),
            if equal { "none".into() } else { format!("{:?} against {:?}", py_msg, rs_msg) },
            format!("{:016x}", fnv1a64(py_msg.as_bytes())),
            format!("{:016x}", fnv1a64(rs_msg.as_bytes())),
            format!("apply.py exits 1 for every refusal; the pipeline exits {} with `{}`", rs.code, rs_line.split(": ").nth(1).unwrap_or("")),
        ]);
        if !equal {
            fail(format!("refusal {}: apply.py {:?} (exit {}), pipeline {:?} (exit {})", name, py_msg, py.code, rs_msg, rs.code));
        }
        compared += 1;
        assert!(!partial_files(&dir).iter().any(|n| n.contains("partial")), "{}: left a partial file", name);
    }
    assert_eq!(compared, 15);
    assert_eq!(std::fs::read(&exists).unwrap(), b"keep");
}
