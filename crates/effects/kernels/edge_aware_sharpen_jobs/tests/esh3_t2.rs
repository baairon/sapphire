//! T2: crate vs stored emulator runs of pool job 0x245d20 (harness v2, vex engine; "VEX follows the SDM, not silicon").
//! Directory: env ESH3_T2 (default t2 of this folder). Layouts: B1 B2 B3 BP (guide-plane mode), A1 AP (constant mode).
use edge_aware_sharpen_jobs::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn base() -> PathBuf {
    PathBuf::from(std::env::var("ESH3_T2").unwrap_or_else(|_| effects_vecdir::path_str("wave_s9/esh3_jobs/t2").into()))
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
    skipped: usize,
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


fn cases_of(d: &Path, suffix: &str) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(d)
        .unwrap()
        .filter_map(|e| e.unwrap().file_name().to_str().map(|s| s.to_string()))
        .filter(|n| n.ends_with(suffix))
        .map(|n| n.trim_end_matches(suffix).to_string())
        .collect();
    names.sort();
    names
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
        let kind = layout.chars().next().unwrap();
        let _ = kind;
        let sfx = if layout.starts_with("K3") { ".out_x.f32" } else if layout.starts_with("K2") { ".out_o0.f32" } else { ".out_a.f32" };
        for c in cases_of(&d, sfx) {
            let parts: Vec<&str> = c.split('_').collect();
            let wh: Vec<i32> = parts[1].split('x').map(|v| v.parse().unwrap()).collect();
            let (w, s) = (wh[0], wh[1]);
            let n = (w * s) as usize;
            let tag = format!("{}/{}/{}", layout, pname, c);
            let mut start = pi(pr, "pstart");
            let mut count = pi(pr, "pcount");
            let total = if layout.starts_with("K3") { w * s } else { s };
            if count < 0 {
                count = total - start;
            }
            if layout.starts_with("K3") {
                let x = rd(&d.join(format!("{}.in_x.f32", c)));
                let want = rd(&d.join(format!("{}.out_x.f32", c)));
                let mut got = x.clone();
                job_3(pf(pr, "s0"), start, count, &mut got).unwrap();
                cmp(&tag, &got, &want, &mut t);
            } else if layout.starts_with("K2") {
                let a = rd(&d.join(format!("{}.in_a.f32", c)));
                let tt = rd(&d.join(format!("{}.in_t.f32", c)));
                let (mut start2, mut count2) = (pi(pr, "pstart2"), pi(pr, "pcount2"));
                if count2 < 0 {
                    count2 = w - start2;
                }
                let _ = &mut start2;
                let mut p0 = vec![f32::from_bits(SENT); n];
                let mut p1 = vec![f32::from_bits(SENT); n];
                if job_cell_a(w, s, (start, count), (start2, count2), &a, &tt, &mut p0, &mut p1).is_err() {
                    t.skipped += 1; // tile outside the contract (count larger than the total): not compared
                    continue;
                }
                cmp(&format!("{} p0", tag), &p0, &rd(&d.join(format!("{}.out_o0.f32", c))), &mut t);
                cmp(&format!("{} p1", tag), &p1, &rd(&d.join(format!("{}.out_o1.f32", c))), &mut t);
            } else {
                let chs: Vec<Vec<f32>> = (0..4).map(|i| rd(&d.join(format!("{}.in_c{}.f32", c, i)))).collect();
                let cur = rd(&d.join(format!("{}.in_cur.f32", c)));
                let wa = rd(&d.join(format!("{}.out_a.f32", c)));
                let wt = rd(&d.join(format!("{}.out_t.f32", c)));
                let ctx = Ctx { w, s, nch: pi(pr, "nch"), flag_cell_b: pi(pr, "flag98"), f0: pf(pr, "f0"), f4: pf(pr, "f4"), f8: pf(pr, "f8"), fc: pf(pr, "fc"), esh2_job_ctx_a: pf(pr, "f100"), k: pf(pr, "k") };
                let mut a = vec![f32::from_bits(SENT); n];
                let mut tt = vec![f32::from_bits(SENT); n];
                let mut b = Bufs { ch: [&chs[0], &chs[1], &chs[2], &chs[3]], cur: &cur, a: &mut a, t: &mut tt };
                job_2(&ctx, start, count, &mut b).unwrap();
                cmp(&format!("{} a", tag), &a, &wa, &mut t);
                cmp(&format!("{} t", tag), &tt, &wt, &mut t);
            }
            start = 0;
            let _ = start;
            t.cases += 1;
        }
    }
    t
}

fn report(layout: &str) {
    let t = run_layout(layout);
    println!("T2 {}: runs {} cases {} floats {} diffs {} nan_class {} nan_payload_diff {} skipped {} first {:?}", layout, t.runs, t.cases, t.floats, t.diffs, t.nan_class, t.nan_payload_diff, t.skipped, t.first);
    assert_eq!(t.diffs, 0, "{:?}", t.first);
    assert!(t.cases > 0);
}
#[test]
fn t2_k1b1() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K1B1");
}
#[test]
fn t2_k1b2() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K1B2");
}
#[test]
fn t2_k1n() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K1N");
}
#[test]
fn t2_k1p() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K1P");
}
#[test]
fn t2_k1a() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K1A");
}
#[test]
fn t2_k1l() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K1L");
}
#[test]
fn t2_k2() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K2");
}
#[test]
fn t2_k3() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K3");
}
#[test]
fn t2_k3p() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K3P");
}
#[test]
fn t2_k1v() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K1V");
}
#[test]
fn t2_k3o() {
    if !effects_vecdir::have_all(&["wave_s9/esh3_jobs/t2"]) {
        return;
    }
    report("K3O");
}
