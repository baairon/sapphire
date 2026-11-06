//! T2 driver-level: wrapper 0x175c80 (flag 0) emulated in harness v2 (vex engine, "VEX follows the SDM, not silicon"), blur dispatcher
//! 0x1b32b0 recorded (not run), pass-through copy 0x672c60 recorded and modelled as memcpy, pool 0x66f080 as serial_for running the
//! combine body 0x175e30 on the globals the wrapper published. Compares the recorded blur-step arguments and the output plane.
use fx_common::cmp::PlaneDiff;
use fx_common::oracle_io::{parse_json, Json};
use sharpen::combine::combine;
use sharpen::plan::*;
use std::path::Path;

fn rd(p: &Path) -> Vec<f32> {
    let b = std::fs::read(p).unwrap_or_else(|e| panic!("{}: {}", p.display(), e));
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}
fn radius_tok(t: &str) -> f32 {
    match t {
        "r09" => 0.9,
        "r25" => 2.5,
        "r70" => 7.0,
        "r0" => 0.0,
        "rm1" => -1.0,
        "rnan" => f32::NAN,
        "rinf" => f32::INFINITY,
        _ => panic!("{}", t),
    }
}
fn gain_tok(t: &str) -> f32 {
    match t {
        "g0" => 0.0,
        "g05" => 0.5,
        "g1" => 1.0,
        "g2" => 2.0,
        "gnan" => f32::NAN,
        "gm1" => -1.0,
        _ => panic!("{}", t),
    }
}
fn hexu(j: &Json) -> u64 {
    u64::from_str_radix(j.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
}

#[derive(Default, Debug)]
struct Tot {
    sets: usize,
    cases: usize,
    blur_calls: usize,
    copy_calls: usize,
    floats: usize,
    bytes: usize,
    maxulp: u32,
    fields: usize,
}

fn run(root: &str, part: bool) -> Tot {
    let root = Path::new(root);
    assert!(root.exists(), "run t2/run_wrapper.sh first");
    let mut t = Tot::default();
    let mut dirs: Vec<_> = std::fs::read_dir(root).unwrap().map(|e| e.unwrap().path()).filter(|p| p.join("manifest.json").exists()).collect();
    dirs.sort();
    for d in dirs {
        t.sets += 1;
        let name = d.file_name().unwrap().to_str().unwrap().to_string();
        let tok: Vec<&str> = name.split('_').collect();
        let (rr, gg, aflag, aspect, pcount): (f32, f32, i32, f32, Option<usize>) = if part {
            (radius_tok(tok[1]), gain_tok(tok[2]), 0, 1.0, Some(tok[0][1..].parse().unwrap()))
        } else {
            let a1 = tok[0] == "a1";
            (radius_tok(tok[1]), gain_tok(tok[2]), a1 as i32, if a1 { 1.5 } else { 1.0 }, None)
        };
        let text = std::fs::read_to_string(d.join("manifest.json")).unwrap().replace("NaN", "null").replace("Infinity", "null");
        let j = parse_json(&text).unwrap();
        for c in j.get("cases").unwrap().as_arr().unwrap() {
            let cname = c.get("name").unwrap().as_str().unwrap();
            assert_eq!(c.get("status").unwrap().as_str().unwrap(), "ok", "{} {}", name, cname);
            let f = |io: &str, n: &str| rd(&d.join(c.get(io).unwrap().get(n).unwrap().get("file").unwrap().as_str().unwrap()));
            let (x, b, out, bout) = (f("inputs", "src"), f("inputs", "blur"), f("outputs", "out"), f("outputs", "blur"));
            let (w, h) = (c.get("width").unwrap().as_usize().unwrap(), c.get("height").unwrap().as_usize().unwrap());
            let npix = w * h;
            assert_eq!(out.len(), npix);
            let count = pcount.unwrap_or(npix);
            let rec = parse_json(&std::fs::read_to_string(d.join(format!("{}.record_args.json", cname))).unwrap()).unwrap();
            let calls = rec.as_arr().unwrap();
            let blur: Vec<&Json> = calls.iter().filter(|e| e.get("hook").unwrap().as_str() == Some("blur")).collect();
            let copy: Vec<&Json> = calls.iter().filter(|e| e.get("hook").unwrap().as_str() == Some("copy")).collect();
            assert_eq!(blur.len() + copy.len(), calls.len());
            assert_eq!(bout.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), b.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), "blur plane untouched");
            let roi = [(w as u64) | ((h as u64) << 32), w as u64, 4u64];
            let mut want = vec![f32::from_bits(0x7fc0ffee); npix];
            match plane_plan(rr, gg) {
                PlanePlan::Copy => {
                    assert_eq!((blur.len(), copy.len()), (0, 1), "{} {}", name, cname);
                    let e = copy[0];
                    let pt = e.get("points_to").unwrap();
                    assert_eq!(pt.get("rdi").unwrap().as_str(), Some("plane:out"));
                    assert_eq!(pt.get("rsi").unwrap().as_str(), Some("plane:src"));
                    assert_eq!(hexu(e.get("regs").unwrap().get("rdx").unwrap()) & 0xffffffff, 0, "flag");
                    let st = e.get("stack").unwrap().as_arr().unwrap();
                    for k in 0..3 {
                        assert_eq!(hexu(&st[k]), roi[k], "copy roi word {}", k);
                    }
                    t.copy_calls += 1;
                    t.fields += 6;
                    want.copy_from_slice(&x);
                }
                PlanePlan::Run { radius: pr, gain } => {
                    assert_eq!((blur.len(), copy.len()), (1, 0), "{} {}", name, cname);
                    let e = blur[0];
                    let regs = e.get("regs").unwrap();
                    let pt = e.get("points_to").unwrap();
                    assert_eq!(pt.get("rsi").unwrap().as_str(), Some("plane:blur"), "dst");
                    assert_eq!(pt.get("rdx").unwrap().as_str(), Some("plane:src"), "in");
                    assert_eq!(hexu(regs.get("rcx").unwrap()), BLUR_EDGE_MIRROR as u64, "edge");
                    assert_eq!(hexu(regs.get("r8").unwrap()), BLUR_PASSES as u64, "passes");
                    assert_eq!(hexu(regs.get("r9").unwrap()) & 0xffffffff, BLUR_SUBPIXEL as u64, "subpixel");
                    let xb = |i: usize| hexu(e.get("xmm").unwrap().get(&format!("xmm{}", i)).unwrap().get("f32_bits").unwrap()) as u32;
                    assert_eq!(xb(0), pr.to_bits(), "sigma x");
                    assert_eq!(xb(1), pr.to_bits(), "sigma y");
                    assert_eq!(xb(2), BLUR_GAIN.to_bits(), "blur gain");
                    assert_eq!(xb(3), gain.to_bits(), "sharpen gain kept in xmm3");
                    let st = e.get("stack").unwrap().as_arr().unwrap();
                    assert_eq!(hexu(&st[0]) & 0xffffffff, BLUR_HALF as u64, "half");
                    for k in 0..3 {
                        assert_eq!(hexu(&st[1 + k]), roi[k], "roi word {}", k);
                    }
                    assert_eq!(hexu(&st[4]) & 0xffffffff, 0, "flag");
                    assert_eq!(hexu(&st[5]), 0x10000000 + 0x1534d0, "pass X");
                    assert_eq!(hexu(&st[6]), 0x10000000 + 0x153620, "pass Y");
                    assert_eq!(hexu(&st[7]), 0, "ctx2");
                    let hex = e.get("ptrs").unwrap().as_arr().unwrap()[0].get("hex").unwrap().as_str().unwrap().to_string();
                    let byte = |o: usize| u8::from_str_radix(&hex[2 * o..2 * o + 2], 16).unwrap();
                    let u32at = |o: usize| u32::from_le_bytes([byte(o), byte(o + 1), byte(o + 2), byte(o + 3)]);
                    assert_eq!(u32at(0x30) as i32, aflag, "aspect flag");
                    assert_eq!(u32at(0x38), aspect.to_bits(), "aspect");
                    t.blur_calls += 1;
                    t.fields += 16;
                    combine(&x, &b, gain, &mut want, count).unwrap();
                }
            }
            let df = PlaneDiff::compare(&out, npix, &want, npix, npix, 1);
            assert!(df.is_parity(), "{} {}: {}", name, cname, df.summary());
            t.cases += 1;
            t.floats += npix;
            t.bytes += df.differing_bytes;
            t.maxulp = t.maxulp.max(df.max_ulp);
        }
    }
    t
}

#[test]
fn wrapper_full_grid() {
    if !effects_vecdir::have_all(&["crates/sharpen/t2/out/wrapper_full", "crates/sharpen/t2/out/wrapper_part"]) {
        return;
    }
    let t = run(effects_vecdir::path_str("crates/sharpen/t2/out/wrapper_full"), false);
    println!("T2 wrapper full: {:?}", t);
    assert_eq!(t.bytes, 0);
    assert!(t.blur_calls > 0 && t.copy_calls > 0);
}

#[test]
fn wrapper_partial_count() {
    if !effects_vecdir::have_all(&["crates/sharpen/t2/out/wrapper_full", "crates/sharpen/t2/out/wrapper_part"]) {
        return;
    }
    let t = run(effects_vecdir::path_str("crates/sharpen/t2/out/wrapper_part"), true);
    println!("T2 wrapper part: {:?}", t);
    assert_eq!(t.bytes, 0);
}
