//! T2: crate vs stored emulator runs of pool job 0x245d20 (harness v2, vex engine; "VEX follows the SDM, not silicon").
//! Directory: env ESH2_T2 (default t2 of this folder). Layouts: B1 B2 B3 BP (guide-plane mode), A1 AP (constant mode).
use edge_aware_sharpen_weights::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn base() -> PathBuf {
    PathBuf::from(std::env::var("ESH2_T2").unwrap_or_else(|_| effects_vecdir::path_str("wave_s9/esh2_job/t2").into()))
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
fn pf(r: &Row, k: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(r[k].trim_start_matches("0x"), 16).unwrap())
}
fn pi(r: &Row, k: &str) -> i32 {
    r[k].parse().unwrap()
}

#[derive(Default, Debug)]
struct Tally {
    runs: usize,
    cases: usize,
    floats: usize,
    diffs: usize,
    nan_class: usize,
    nan_payload_diff: usize,
    first: Option<String>,
}

const SENT: u32 = 0x7fc0ffee;

fn cmp(tag: &str, got: &[f32], want: &[f32], t: &mut Tally) {
    assert_eq!(got.len(), want.len(), "{} length", tag);
    for (i, (g, w)) in got.iter().zip(want.iter()).enumerate() {
        t.floats += 1;
        if g.is_nan() && w.is_nan() {
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

fn run_layout(layout: &str) -> Tally {
    let params = tsv(layout);
    let root = base().join("out").join(layout);
    assert!(root.exists(), "run the emulator first: {}", root.display());
    let mut t = Tally::default();
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&root).unwrap().map(|e| e.unwrap().path()).filter(|p| p.join("manifest.json").exists()).collect();
    dirs.sort();
    assert!(!dirs.is_empty());
    for d in dirs {
        let pname = d.file_name().unwrap().to_str().unwrap().to_string();
        let pr = &params[&pname];
        t.runs += 1;
        let mut names: Vec<String> = std::fs::read_dir(&d)
            .unwrap()
            .filter_map(|e| e.unwrap().file_name().to_str().map(|s| s.to_string()))
            .filter(|n| n.ends_with(".out_a.f32"))
            .map(|n| n.trim_end_matches(".out_a.f32").to_string())
            .collect();
        names.sort();
        for c in names {
            let parts: Vec<&str> = c.split('_').collect();
            let wh: Vec<i32> = parts[1].split('x').map(|v| v.parse().unwrap()).collect();
            let (w, s) = (wh[0], wh[1]);
            let mis: u64 = parts[4][1..].parse().unwrap();
            let p0 = rd(&d.join(format!("{}.in_p0.f32", c)));
            let p1 = rd(&d.join(format!("{}.in_p1.f32", c)));
            let cur = rd(&d.join(format!("{}.in_cur.f32", c)));
            let wa = rd(&d.join(format!("{}.out_a.f32", c)));
            let wt = rd(&d.join(format!("{}.out_t.f32", c)));
            let ctx = Ctx { w, s, flag_cell_b: pi(pr, "flag98"), f0: pf(pr, "f0"), f4: pf(pr, "f4"), f8: pf(pr, "f8"), fc: pf(pr, "fc"), esh2_job_ctx_a: pf(pr, "f100"), k: pf(pr, "k") };
            let n = (w * s) as usize;
            let mut a = vec![f32::from_bits(SENT); n];
            let mut tt = vec![f32::from_bits(SENT); n];
            let start = pi(pr, "pstart");
            let mut count = pi(pr, "pcount");
            if count < 0 {
                count = s - start;
            }
            // cur plane address: plane base is 0x50000000 + k * 0x1000000 (16 MiB aligned) + mis; only mod 64 matters
            let cur_addr = 0x5000_0000u64 + 2 * 0x100_0000 + mis;
            let mut b = Bufs { p0: &p0, p1: &p1, cur: &cur, a: &mut a, t: &mut tt };
            job_1(&ctx, start, count, cur_addr, &mut b).unwrap();
            let tag = format!("{}/{}/{}", layout, pname, c);
            cmp(&format!("{} a", tag), &a, &wa, &mut t);
            cmp(&format!("{} t", tag), &tt, &wt, &mut t);
            t.cases += 1;
        }
    }
    t
}

fn report(layout: &str) {
    let t = run_layout(layout);
    println!("T2 {}: runs {} cases {} floats {} diffs {} nan_class {} nan_payload_diff {} first {:?}", layout, t.runs, t.cases, t.floats, t.diffs, t.nan_class, t.nan_payload_diff, t.first);
    assert_eq!(t.diffs, 0, "{:?}", t.first);
    assert!(t.cases > 0);
}

#[test]
fn t2_b1() {
    if !effects_vecdir::have_all(&["wave_s9/esh2_job/t2"]) {
        return;
    }
    report("B1");
}
#[test]
fn t2_b2() {
    if !effects_vecdir::have_all(&["wave_s9/esh2_job/t2"]) {
        return;
    }
    report("B2");
}
#[test]
fn t2_b3() {
    if !effects_vecdir::have_all(&["wave_s9/esh2_job/t2"]) {
        return;
    }
    report("B3");
}
#[test]
fn t2_bp() {
    if !effects_vecdir::have_all(&["wave_s9/esh2_job/t2"]) {
        return;
    }
    report("BP");
}
#[test]
fn t2_a1() {
    if !effects_vecdir::have_all(&["wave_s9/esh2_job/t2"]) {
        return;
    }
    report("A1");
}
#[test]
fn t2_ap() {
    if !effects_vecdir::have_all(&["wave_s9/esh2_job/t2"]) {
        return;
    }
    report("AP");
}
#[test]
fn t2_a2() {
    if !effects_vecdir::have_all(&["wave_s9/esh2_job/t2"]) {
        return;
    }
    report("A2");
}
#[test]
fn t2_b4() {
    if !effects_vecdir::have_all(&["wave_s9/esh2_job/t2"]) {
        return;
    }
    report("B4");
}
#[test]
fn t2_b5() {
    if !effects_vecdir::have_all(&["wave_s9/esh2_job/t2"]) {
        return;
    }
    report("B5");
}
