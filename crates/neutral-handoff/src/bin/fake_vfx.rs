//! `fake-vfx`: a stand-in for twixtor's `vfx` that the tests of this crate run.
//!
//! It speaks the command line the wrapper uses (`list --json [--all]` and
//! `apply EFFECT --width W --height H --depth D --fps R [--frames N] [--tmp-dir DIR]
//! [--layer ID=FILE] [--param K=V] [--check]`, raw frames on stdin and stdout, errors as
//! `vfx: error: <message>` with the exit codes 2, 3 and 4), and has effects that misbehave on
//! purpose, so that the pipeline's error paths run against a real process:
//!
//! | Effect | Kind | Behaviour |
//! |---|---|---|
//! | `fake-copy` | frame | copies every frame; parameter `level` (0 to 100) is checked |
//! | `fake-invert` | frame | `selftest-invert` of this crate |
//! | `fake-spool` | temporal | reads all of its input, then writes the frames in reverse order |
//! | `fake-layer` | temporal | copies; needs the layer `overlay` (and accepts `extra`) |
//! | `fake-number` | frame | copies, writing the frame number (modulo 256) into the first byte of each frame |
//! | `fake-slow` | frame | copies, sleeping `delay_ms` (parameter) before each frame |
//! | `fake-fail` | frame | copies two frames, then exits 3 with `frame 2: unsupported: injected failure` |
//! | `fake-exit1` | frame | exits 1 at once with a line on standard error |
//! | `fake-truncate` | frame | writes one frame and half of the next, then exits 0 |
//! | `fake-truncate-fail` | frame | writes one frame and half of the next, then exits 4 |
//! | `fake-close-early` | frame | writes one frame, then exits 0 without reading the rest |
//! | `fake-short` | frame | copies all frames but the last |
//! | `fake-extra` | frame | copies all frames and writes one more |
//! | `fake-hang` | frame | neither reads nor writes, never exits |
//! | `fake-hang-mid` | frame | copies one frame, then never reads or writes again |
//! | `fake-hang-at-end` | frame | copies everything, then never exits |
//! | `fake-flood` | frame | writes a lot to standard error while copying |
//!
//! The environment variable `FAKE_VFX_CONTRACT` makes `list --json` print the object form
//! `{"contract": VALUE, "stages": [...]}` instead of a bare array.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::io::{self, Read, Write};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

use neutral_handoff::format::{Depth, FrameSpec};
use neutral_handoff::pyjson::Out;
use neutral_handoff::stage::{FrameStage, Invert};

struct Effect {
    id: &'static str,
    temporal: bool,
    layers: &'static [(&'static str, bool)],
}

const EFFECTS: &[Effect] = &[
    Effect { id: "fake-copy", temporal: false, layers: &[] },
    Effect { id: "fake-invert", temporal: false, layers: &[] },
    Effect { id: "fake-spool", temporal: true, layers: &[] },
    Effect { id: "fake-layer", temporal: true, layers: &[("overlay", false), ("extra", true)] },
    Effect { id: "fake-number", temporal: false, layers: &[] },
    Effect { id: "fake-slow", temporal: false, layers: &[] },
    Effect { id: "fake-fail", temporal: false, layers: &[] },
    Effect { id: "fake-exit1", temporal: false, layers: &[] },
    Effect { id: "fake-truncate", temporal: false, layers: &[] },
    Effect { id: "fake-truncate-fail", temporal: false, layers: &[] },
    Effect { id: "fake-close-early", temporal: false, layers: &[] },
    Effect { id: "fake-short", temporal: false, layers: &[] },
    Effect { id: "fake-extra", temporal: false, layers: &[] },
    Effect { id: "fake-hang", temporal: false, layers: &[] },
    Effect { id: "fake-hang-mid", temporal: false, layers: &[] },
    Effect { id: "fake-hang-at-end", temporal: false, layers: &[] },
    Effect { id: "fake-flood", temporal: false, layers: &[] },
];

fn fail(code: i32, message: &str) -> ! {
    eprintln!("vfx: error: {}", message);
    exit(code)
}

fn listing() -> Out {
    let entries: Vec<Out> = EFFECTS
        .iter()
        .map(|e| {
            Out::map(vec![
                ("id", Out::text(e.id)),
                ("name", Out::text(e.id)),
                ("family", Out::text("fake")),
                ("kind", Out::text(if e.temporal { "temporal" } else { "frame" })),
                ("hidden", Out::Bool(false)),
                ("params", Out::List(Vec::new())),
                (
                    "layers",
                    Out::List(
                        e.layers
                            .iter()
                            .map(|(id, optional)| Out::map(vec![("id", Out::text(*id)), ("name", Out::text(*id)), ("optional", Out::Bool(*optional))]))
                            .collect(),
                    ),
                ),
            ])
        })
        .collect();
    match std::env::var("FAKE_VFX_CONTRACT") {
        Ok(contract) => Out::map(vec![("contract", Out::text(contract)), ("stages", Out::List(entries))]),
        Err(_) => Out::List(entries),
    }
}

#[derive(Default)]
struct Apply {
    effect: String,
    width: Option<String>,
    height: Option<String>,
    depth: Option<String>,
    fps: Option<String>,
    frames: Option<String>,
    tmp_dir: Option<String>,
    params: Vec<String>,
    layers: Vec<String>,
    check: bool,
}

fn parse_apply(args: &[String]) -> Apply {
    let mut a = Apply::default();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        let mut take = |name: &str| -> String {
            i += 1;
            match args.get(i) {
                Some(v) => v.clone(),
                None => fail(2, &format!("`{}` expects a value", name)),
            }
        };
        match arg.as_str() {
            "--width" => a.width = Some(take("--width")),
            "--height" => a.height = Some(take("--height")),
            "--depth" => a.depth = Some(take("--depth")),
            "--fps" => a.fps = Some(take("--fps")),
            "--frames" => a.frames = Some(take("--frames")),
            "--tmp-dir" => a.tmp_dir = Some(take("--tmp-dir")),
            "--param" => a.params.push(take("--param")),
            "--layer" => a.layers.push(take("--layer")),
            "--check" => a.check = true,
            other if !other.starts_with("--") && a.effect.is_empty() => a.effect = other.to_string(),
            other => fail(2, &format!("unknown option `{}`", other)),
        }
        i += 1;
    }
    a
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("list") => {
            println!("{}", listing().dumps_indent(2));
        }
        Some("apply") => apply(&args[1..]),
        _ => fail(2, "usage: fake-vfx list [--json] | apply EFFECT ..."),
    }
}

fn apply(args: &[String]) {
    let a = parse_apply(args);
    let effect = match EFFECTS.iter().find(|e| e.id == a.effect) {
        Some(e) => e,
        None => fail(3, &format!("unknown effect `{}`, `vfx list` shows the registered ones", a.effect)),
    };
    if !effect.temporal && (a.frames.is_some() || !a.layers.is_empty()) {
        let used = if a.frames.is_some() && !a.layers.is_empty() { "--frames and --layer" } else if a.frames.is_some() { "--frames" } else { "--layer" };
        fail(2, &format!("{} is a frame effect, {} only apply to temporal effects", effect.id, used));
    }
    let number = |name: &str, text: &Option<String>| -> u32 {
        match text {
            None => fail(2, &format!("missing {}", name)),
            Some(t) => match t.parse::<u32>() {
                Ok(n) if n >= 1 => n,
                _ => fail(2, &format!("`{}` expects a positive integer, got `{}`", name, t)),
            },
        }
    };
    let width = number("--width", &a.width);
    let height = number("--height", &a.height);
    let depth = match a.depth.as_deref().and_then(|d| d.parse::<u32>().ok()).and_then(Depth::from_bits) {
        Some(d) => d,
        None => fail(2, &format!("`--depth` expects 8, 16 or 32, got `{}`", a.depth.clone().unwrap_or_default())),
    };
    if effect.temporal && a.fps.is_none() {
        fail(2, &format!("{} is a temporal effect and needs --fps N[/D], for example 24 or 30000/1001", effect.id));
    }
    for p in &a.params {
        let (id, value) = p.split_once('=').unwrap_or((p.as_str(), ""));
        match (effect.id, id) {
            ("fake-copy", "level") => match value.parse::<i32>() {
                Ok(0..=100) => {}
                Ok(n) => fail(3, &format!("bad value for `level`: {} outside 0..=100", n)),
                Err(_) => fail(3, &format!("bad value for `level`: cannot parse `{}`", value)),
            },
            ("fake-slow", "delay_ms") => {
                if value.parse::<u64>().is_err() {
                    fail(3, &format!("bad value for `delay_ms`: cannot parse `{}`", value));
                }
            }
            _ => fail(3, &format!("unknown param `{}`", id)),
        }
    }
    for (id, optional) in effect.layers {
        let given = a.layers.iter().any(|l| l.starts_with(&format!("{}=", id)));
        if !given && !optional {
            fail(3, &format!("{} needs the layer `{}`", effect.id, id));
        }
    }
    for l in &a.layers {
        let id = l.split('=').next().unwrap_or("");
        if !effect.layers.iter().any(|(known, _)| *known == id) {
            fail(3, &format!("unknown layer `{}`", id));
        }
    }
    if a.check {
        return;
    }
    let spec = FrameSpec { width, height, depth };
    let bytes = spec.frame_bytes() as usize;
    if effect.id == "fake-layer" {
        for l in &a.layers {
            if let Some((_, path)) = l.split_once('=') {
                match std::fs::metadata(path) {
                    Ok(m) if m.len() > 0 && m.len() % bytes as u64 == 0 => {}
                    Ok(m) => fail(4, &format!("layer {} holds {} bytes, which is not a whole number of frames", path, m.len())),
                    Err(e) => fail(4, &format!("cannot open layer {}: {}", path, e)),
                }
            }
        }
        let _ = &a.tmp_dir;
    }
    let stdin = io::stdin();
    let mut input = stdin.lock();
    let stdout = io::stdout();
    let mut output = stdout.lock();
    run_effect(effect.id, &a, spec, bytes, &mut input, &mut output);
    let _ = output.flush();
}

fn read_frame(input: &mut impl Read, buf: &mut [u8], whole: u64) -> bool {
    let mut got = 0;
    while got < buf.len() {
        match input.read(&mut buf[got..]) {
            Ok(0) => break,
            Ok(n) => got += n,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => fail(4, &format!("cannot read the input: {}", e)),
        }
    }
    if got == 0 {
        return false;
    }
    if got < buf.len() {
        fail(4, &format!("input ends with a partial frame ({} of {} bytes) after {} whole frames", got, buf.len(), whole));
    }
    true
}

fn put(output: &mut impl Write, bytes: &[u8]) {
    if let Err(e) = output.write_all(bytes).and_then(|_| output.flush()) {
        fail(4, &format!("cannot write the output: {}", e));
    }
}

fn hang() -> ! {
    loop {
        sleep(Duration::from_secs(1));
    }
}

fn run_effect(id: &str, a: &Apply, spec: FrameSpec, bytes: usize, input: &mut impl Read, output: &mut impl Write) {
    let mut buf = vec![0u8; bytes];
    let mut n: u64 = 0;
    match id {
        "fake-hang" => hang(),
        "fake-exit1" => {
            eprintln!("boom: the fake effect fell over");
            exit(1);
        }
        "fake-spool" => {
            let mut all: Vec<Vec<u8>> = Vec::new();
            while read_frame(input, &mut buf, n) {
                all.push(buf.clone());
                n += 1;
            }
            let wanted = a.frames.as_deref().and_then(|f| f.parse::<usize>().ok()).unwrap_or(all.len());
            for k in 0..wanted {
                match all.get(all.len().wrapping_sub(1).wrapping_sub(k)) {
                    Some(f) if k < all.len() => put(output, f),
                    _ => put(output, &vec![0u8; bytes]),
                }
            }
        }
        "fake-short" => {
            let mut held: Option<Vec<u8>> = None;
            while read_frame(input, &mut buf, n) {
                if let Some(prev) = held.replace(buf.clone()) {
                    put(output, &prev);
                }
                n += 1;
            }
        }
        "fake-extra" => {
            while read_frame(input, &mut buf, n) {
                put(output, &buf);
                n += 1;
            }
            put(output, &vec![7u8; bytes]);
        }
        "fake-flood" => {
            let line = "x".repeat(200);
            while read_frame(input, &mut buf, n) {
                for _ in 0..200 {
                    eprintln!("{}", line);
                }
                put(output, &buf);
                n += 1;
            }
            eprintln!("flood done");
        }
        _ => {
            let delay = a.params.iter().find_map(|p| p.strip_prefix("delay_ms=")).and_then(|v| v.parse::<u64>().ok()).unwrap_or(0);
            let mut invert = Invert::new(spec, false);
            while read_frame(input, &mut buf, n) {
                if delay > 0 && id == "fake-slow" {
                    sleep(Duration::from_millis(delay));
                }
                match id {
                    "fake-fail" if n == 2 => fail(3, "frame 2: unsupported: injected failure"),
                    "fake-number" => {
                        buf[0] = n as u8;
                        put(output, &buf);
                    }
                    "fake-invert" => {
                        invert.process(n, &mut buf).expect("a whole frame");
                        put(output, &buf);
                    }
                    "fake-truncate" | "fake-truncate-fail" if n == 1 => {
                        put(output, &buf[..bytes / 2]);
                        let _ = output.flush();
                        if id == "fake-truncate-fail" {
                            fail(4, "output stopped inside a frame");
                        }
                        return;
                    }
                    "fake-close-early" if n == 1 => {
                        let _ = output.flush();
                        return;
                    }
                    "fake-hang-mid" if n == 1 => hang(),
                    _ => put(output, &buf),
                }
                n += 1;
            }
            if id == "fake-hang-at-end" {
                let _ = output.flush();
                hang();
            }
        }
    }
}
