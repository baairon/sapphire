//! T2: crate vs stored emulator runs (harness v2, vex engine; the VEX engine follows the SDM, not silicon).
//! Layouts: XD XDP XDS XAD XAS XALL (pass job x), YD YDP YDS YAD YAS YALL (pass job y), C (callback 0x261780), F FDS (callback 0x24e700 standalone), E8 E9 (exp2).
//! Directory: env ESH5B_T2 (default t2 of this folder).
use esh5b_passjobs::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

const SENT: u32 = 0x7fc0ffee;

fn base() -> PathBuf {
    PathBuf::from(std::env::var("ESH5B_T2").unwrap_or_else(|_| effects_vecdir::path_str("wave_s9/esh5b_passjobs/t2").into()))
}
fn rd(p: &Path) -> Vec<f32> {
    let b = std::fs::read(p).unwrap_or_else(|e| panic!("{}: {}", p.display(), e));
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}
type Row = HashMap<String, String>;
fn tsv(layout: &str) -> HashMap<String, Row> {
    let text = std::fs::read_to_string(base().join(format!("layouts/params_{}.tsv", layout))).unwrap();
    let mut lines = text.lines();
    let hdr: Vec<&str> = lines.next().unwrap().split('\t').collect();
    let mut m = HashMap::new();
    for l in lines {
        let f: Vec<&str> = l.split('\t').collect();
        let row: Row = hdr.iter().zip(f.iter()).map(|(a, b)| (a.to_string(), b.to_string())).collect();
        m.insert(f[0].to_string(), row);
    }
    m
}
fn hexu(r: &Row, k: &str) -> u32 {
    u32::from_str_radix(r[k].trim_start_matches("0x"), 16).unwrap()
}
fn meta(layout: &str, pname: &str, case: &str) -> HashMap<String, String> {
    let t = std::fs::read_to_string(base().join(format!("meta/{}/{}/{}.meta", layout, pname, case))).unwrap_or_else(|e| panic!("meta {}/{}/{}: {}", layout, pname, case, e));
    t.lines().filter_map(|l| l.split_once('=')).map(|(a, b)| (a.to_string(), b.to_string())).collect()
}
fn mi(m: &HashMap<String, String>, k: &str) -> i64 {
    m[k].parse().unwrap()
}

#[derive(Default, Debug)]
struct Tally {
    runs: usize,
    cases: usize,
    floats: usize,
    diffs: usize,
    nan_class: usize,
    nan_payload_diff: usize,
    rec_diffs: usize,
    first: Option<String>,
}

fn cmp(tag: &str, got: &[f32], want: &[f32], t: &mut Tally) {
    assert_eq!(got.len(), want.len(), "{} length", tag);
    for (i, (g, w)) in got.iter().zip(want.iter()).enumerate() {
        t.floats += 1;
        if w.to_bits() == SENT || g.to_bits() == SENT {
            // a cell the original left untouched (harness sentinel) must be left untouched by the port, bit for bit
            if g.to_bits() != w.to_bits() {
                t.diffs += 1;
                t.first.get_or_insert(format!("{} idx {} sentinel got {:08x} want {:08x}", tag, i, g.to_bits(), w.to_bits()));
            }
        } else if g.is_nan() && w.is_nan() {
            t.nan_class += 1;
            if g.to_bits() != w.to_bits() {
                t.nan_payload_diff += 1;
            }
        } else if g.to_bits() != w.to_bits() {
            t.diffs += 1;
            if t.first.is_none() {
                t.first = Some(format!("{} idx {} got {:08x} want {:08x}", tag, i, g.to_bits(), w.to_bits()));
            }
        }
    }
}
fn recdiff(tag: &str, what: &str, got: i64, want: i64, t: &mut Tally) {
    if got != want {
        t.rec_diffs += 1;
        if t.first.is_none() {
            t.first = Some(format!("{} record {} got {} want {}", tag, what, got, want));
        }
    }
}

fn runs(layout: &str) -> Vec<(String, PathBuf, Vec<String>)> {
    let root = base().join("out").join(layout);
    assert!(root.exists(), "run the emulator first: {}", root.display());
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&root).unwrap().map(|e| e.unwrap().path()).filter(|p| p.join("manifest.json").exists()).collect();
    dirs.sort();
    assert!(!dirs.is_empty());
    dirs.into_iter()
        .map(|d| {
            let pname = d.file_name().unwrap().to_str().unwrap().to_string();
            let md = base().join("meta").join(layout).join(&pname);
            let mut names: Vec<String> = std::fs::read_dir(&md)
                .unwrap_or_else(|e| panic!("run export_meta.py first: {}: {}", md.display(), e))
                .filter_map(|e| e.unwrap().file_name().to_str().map(|s| s.to_string()))
                .filter(|n| n.ends_with(".meta"))
                .map(|n| n.trim_end_matches(".meta").to_string())
                .collect();
            names.sort();
            (pname, d, names)
        })
        .collect()
}

fn wh(case: &str) -> (i32, i32) {
    let p: Vec<&str> = case.split('_').collect();
    let k = if p[0] == "e5b" { 2 } else { 1 };
    let v: Vec<i32> = p[k].split('x').map(|v| v.parse().unwrap()).collect();
    (v[0], v[1])
}

fn run_job_layout(layout: &str) -> Tally {
    let job = layout.chars().next().unwrap();
    let alias = match &layout[1..] {
        "D" | "DP" => "D",
        a => a,
    };
    let params = tsv(layout);
    let mut t = Tally::default();
    for (pname, d, names) in runs(layout) {
        let pr = &params[&pname];
        t.runs += 1;
        for c in names {
            let (w, h) = wh(&c);
            let n = (w * h) as usize;
            let m = meta(layout, &pname, &c);
            let (start, count) = (mi(&m, "start") as i32, mi(&m, "count") as i32);
            let radius = f32::from_bits(hexu(pr, "rbits"));
            let f = |nm: &str| rd(&d.join(format!("{}.{}.f32", c, nm)));
            let sent = vec![f32::from_bits(SENT); n];
            let (mut mem, jb, outname, lo): (Vec<f32>, Job, &str, usize) = match alias {
                "D" => ([f("in_src"), f("in_aux"), sent].concat(), Job { w, h, radius, src: 0, aux: n, dst: 2 * n }, "out_dst", 2 * n),
                "DS" => ([f("in_src"), f("in_aux")].concat(), Job { w, h, radius, src: 0, aux: n, dst: 0 }, "out_src", 0),
                "AD" => ([f("in_src"), f("in_dst")].concat(), Job { w, h, radius, src: 0, aux: n, dst: n }, "out_dst", n),
                "AS" => ([f("in_src"), sent].concat(), Job { w, h, radius, src: 0, aux: 0, dst: n }, "out_dst", n),
                "ALL" => (f("in_src"), Job { w, h, radius, src: 0, aux: 0, dst: 0 }, "out_src", 0),
                _ => panic!("alias"),
            };
            let mut rec = Rec::default();
            if job == 'X' {
                job_x(&mut mem, &jb, start, count, &mut rec);
            } else {
                job_y(&mut mem, &jb, start, count, &mut rec);
            }
            let tag = format!("{}/{}/{}", layout, pname, c);
            cmp(&tag, &mem[lo..lo + n], &f(outname), &mut t);
            recdiff(&tag, "part_total", rec.part_total as i64, mi(&m, "total"), &mut t);
            recdiff(&tag, "nparts", 1, mi(&m, "nparts"), &mut t);
            recdiff(&tag, "asserts", rec.asserts as i64, mi(&m, "asserts"), &mut t);
            recdiff(&tag, "slice_calls", rec.slice_calls as i64, mi(&m, "nslice"), &mut t);
            if job == 'Y' {
                recdiff(&tag, "slice_seq", 1, mi(&m, "slice_ok"), &mut t);
            }
            t.cases += 1;
        }
    }
    t
}

fn pdec(r: &Row, k: &str) -> i32 {
    r[k].parse().unwrap()
}

/// Degenerate ctx dimensions: ctx+0x7c and +0x80 come from the parameters (cw, ch); the image is 8x4 and only its planes exist.
fn run_z(layout: &str) -> Tally {
    let params = tsv(layout);
    let mut t = Tally::default();
    for (pname, d, names) in runs(layout) {
        let pr = &params[&pname];
        t.runs += 1;
        for c in names {
            let n = 32usize;
            let m = meta(layout, &pname, &c);
            let (start, count) = (mi(&m, "start") as i32, mi(&m, "count") as i32);
            let f = |nm: &str| rd(&d.join(format!("{}.{}.f32", c, nm)));
            let mut mem = [f("in_src"), f("in_aux"), vec![f32::from_bits(SENT); n]].concat();
            let jb = Job { w: pdec(pr, "cw"), h: pdec(pr, "ch"), radius: f32::from_bits(hexu(pr, "rbits")), src: 0, aux: n, dst: 2 * n };
            let mut rec = Rec::default();
            if layout == "XZ" {
                job_x(&mut mem, &jb, start, count, &mut rec);
            } else {
                job_y(&mut mem, &jb, start, count, &mut rec);
            }
            let tag = format!("{}/{}/{}", layout, pname, c);
            cmp(&tag, &mem[2 * n..3 * n], &f("out_dst"), &mut t);
            recdiff(&tag, "part_total", rec.part_total as i64, mi(&m, "total"), &mut t);
            recdiff(&tag, "asserts", rec.asserts as i64, mi(&m, "asserts"), &mut t);
            recdiff(&tag, "slice_calls", rec.slice_calls as i64, mi(&m, "nslice"), &mut t);
            t.cases += 1;
        }
    }
    t
}

/// The standalone filter with n from a parameter (zero, negative, in range).
fn run_fn() -> Tally {
    let params = tsv("FN");
    let mut t = Tally::default();
    for (pname, d, names) in runs("FN") {
        let pr = &params[&pname];
        t.runs += 1;
        let p = [f32::from_bits(hexu(pr, "p0b")), f32::from_bits(hexu(pr, "p1b")), f32::from_bits(hexu(pr, "p2b"))];
        for c in names {
            let nn = pdec(pr, "nn") as i64;
            let f = |nm: &str| rd(&d.join(format!("{}.{}.f32", c, nm)));
            let mut mem = [f("in_src"), f("in_aux"), vec![f32::from_bits(SENT); 8]].concat();
            let mut rec = Rec::default();
            filter3(&mut mem, 0, 16, 8, nn, p, &mut rec);
            cmp(&format!("FN/{}/{}", pname, c), &mem[16..24], &f("out_dst"), &mut t);
            t.cases += 1;
        }
    }
    t
}

fn run_c() -> Tally {
    let params = tsv("C");
    let mut t = Tally::default();
    for (pname, d, names) in runs("C") {
        let pr = &params[&pname];
        t.runs += 1;
        for c in names {
            let got = cb_cell_a(f32::from_bits(hexu(pr, "rbits")));
            let want = rd(&d.join(format!("{}.out_o.f32", c)));
            let tag = format!("C/{}/{}", pname, c);
            cmp(&tag, &got, &want[..3], &mut t);
            recdiff(&tag, "untouched 4th float", want[3].to_bits() as i64, SENT as i64, &mut t);
            t.cases += 1;
        }
    }
    t
}

fn run_f(layout: &str) -> Tally {
    let params = tsv(layout);
    let mut t = Tally::default();
    for (pname, d, names) in runs(layout) {
        let pr = &params[&pname];
        t.runs += 1;
        let p = [f32::from_bits(hexu(pr, "p0b")), f32::from_bits(hexu(pr, "p1b")), f32::from_bits(hexu(pr, "p2b"))];
        for c in names {
            let (w, _) = wh(&c);
            let n = w as usize;
            let m = meta(layout, &pname, &c);
            let f = |nm: &str| rd(&d.join(format!("{}.{}.f32", c, nm)));
            let mut rec = Rec::default();
            let tag = format!("{}/{}/{}", layout, pname, c);
            if layout == "F" {
                let mut mem = [f("in_src"), f("in_aux"), vec![f32::from_bits(SENT); n]].concat();
                filter3(&mut mem, 0, 2 * n, n, n as i64, p, &mut rec);
                cmp(&tag, &mem[2 * n..3 * n], &f("out_dst"), &mut t);
            } else {
                let mut mem = [f("in_src"), f("in_aux")].concat();
                filter3(&mut mem, 0, 0, n, n as i64, p, &mut rec);
                cmp(&tag, &mem[..n], &f("out_src"), &mut t);
            }
            recdiff(&tag, "asserts", rec.asserts as i64, mi(&m, "asserts"), &mut t);
            t.cases += 1;
        }
    }
    t
}

fn run_e(layout: &str) -> Tally {
    let fma = layout == "E9";
    let params = tsv(layout);
    let mut t = Tally::default();
    for (pname, _d, names) in runs(layout) {
        let pr = &params[&pname];
        t.runs += 1;
        for c in names {
            let m = meta(layout, &pname, &c);
            let tag = format!("{}/{}/{}", layout, pname, c);
            let lanes: Vec<u32> = m["lanes"].split(',').map(|x| u32::from_str_radix(x, 16).unwrap()).collect();
            let slow: Vec<u32> = if m["slow"].is_empty() { vec![] } else { m["slow"].split(',').map(|x| u32::from_str_radix(x, 16).unwrap()).collect() };
            let mut want_slow = vec![];
            for k in 0..8 {
                let x = f32::from_bits(hexu(pr, &format!("l{}", k)));
                let got = match exp2_lane(x, fma) {
                    Ok(v) => v,
                    Err(b) => {
                        want_slow.push(b);
                        f32::from_bits(0x7fc0dead)
                    }
                };
                cmp(&format!("{} lane {}", tag, k), &[got], &[f32::from_bits(lanes[k])], &mut t);
            }
            if slow != want_slow {
                t.rec_diffs += 1;
                t.first.get_or_insert(format!("{} slow lanes {:?} want {:?}", tag, want_slow, slow));
            }
            t.cases += 1;
        }
    }
    t
}

fn report(name: &str, t: &Tally) {
    println!("T2 {}: runs {} cases {} floats {} diffs {} rec_diffs {} nan_class {} nan_payload_diff {} first {:?}", name, t.runs, t.cases, t.floats, t.diffs, t.rec_diffs, t.nan_class, t.nan_payload_diff, t.first);
    assert_eq!(t.diffs + t.rec_diffs, 0, "{} differs: {:?}", name, t.first);
    assert!(t.cases > 0);
}

macro_rules! jobtest {
    ($n:ident, $l:expr) => {
        #[test]
        fn $n() {
            if !effects_vecdir::have_all(&["wave_s9/esh5b_passjobs/t2"]) {
                return;
            }
            report($l, &run_job_layout($l));
        }
    };
}
jobtest!(t2_xd, "XD");
jobtest!(t2_xdp, "XDP");
jobtest!(t2_xds, "XDS");
jobtest!(t2_xad, "XAD");
jobtest!(t2_xas, "XAS");
jobtest!(t2_xall, "XALL");
jobtest!(t2_yd, "YD");
jobtest!(t2_ydp, "YDP");
jobtest!(t2_yds, "YDS");
jobtest!(t2_yad, "YAD");
jobtest!(t2_yas, "YAS");
jobtest!(t2_yall, "YALL");
#[test]
fn t2_xz() {
    if !effects_vecdir::have_all(&["wave_s9/esh5b_passjobs/t2"]) {
        return;
    }
    report("XZ", &run_z("XZ"));
}
#[test]
fn t2_yz() {
    if !effects_vecdir::have_all(&["wave_s9/esh5b_passjobs/t2"]) {
        return;
    }
    report("YZ", &run_z("YZ"));
}
#[test]
fn t2_fn() {
    if !effects_vecdir::have_all(&["wave_s9/esh5b_passjobs/t2"]) {
        return;
    }
    report("FN", &run_fn());
}
#[test]
fn t2_c() {
    if !effects_vecdir::have_all(&["wave_s9/esh5b_passjobs/t2"]) {
        return;
    }
    report("C", &run_c());
}
#[test]
fn t2_f() {
    if !effects_vecdir::have_all(&["wave_s9/esh5b_passjobs/t2"]) {
        return;
    }
    report("F", &run_f("F"));
}
#[test]
fn t2_fds() {
    if !effects_vecdir::have_all(&["wave_s9/esh5b_passjobs/t2"]) {
        return;
    }
    report("FDS", &run_f("FDS"));
}
#[test]
fn t2_e8() {
    if !effects_vecdir::have_all(&["wave_s9/esh5b_passjobs/t2"]) {
        return;
    }
    report("E8", &run_e("E8"));
}
#[test]
fn t2_e9() {
    if !effects_vecdir::have_all(&["wave_s9/esh5b_passjobs/t2"]) {
        return;
    }
    report("E9", &run_e("E9"));
}

/// The route domain of the exp2 helper: the core calls it with x = n - 1 - i, integers 0..=4 for n in 1..=5 (the only call site passes n = 3: x in {0, 1, 2}).
/// Both variants return exactly 2^x there.
#[test]
fn exp2_route_domain() {
    for x in 0..=4 {
        let want = (1u32 << x) as f32;
        for fma in [false, true] {
            assert_eq!(exp2_lane(x as f32, fma).unwrap().to_bits(), want.to_bits());
        }
    }
}
