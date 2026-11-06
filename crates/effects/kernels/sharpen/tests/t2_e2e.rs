//! T2 end-to-end: wrapper 0x175c80 (flag 0) with the real blur dispatcher, driver, pass wrappers, K1, K3, helpers and the combine body
//! 0x175e30, all emulated (harness v2, engine vex: "VEX follows the SDM, not silicon"), against sharpen::sharpen_plane.
//! Run folders come from t2/run_e2e.sh (kept on the server). The blur scratch alignment phase (B-A4) is a per-set parameter.
use fx_common::cmp::PlaneDiff;
use fx_common::oracle_io::parse_json;
use sharpen::sharpen_plane;
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
        "r40" => 40.0,
        "r90" => 90.0,
        "r150" => 150.0,
        "r240" => 240.0,
        _ => panic!("{}", t),
    }
}
fn gain_tok(t: &str) -> f32 {
    match t {
        "g0" => 0.0,
        "g05" => 0.5,
        "g07" => 0.7,
        "g1" => 1.0,
        "g2" => 2.0,
        "gnan" => f32::NAN,
        _ => panic!("{}", t),
    }
}

/// returns (sets, cases, floats, differing bytes, max ulp, copy cases, long window cases)
fn run(root: &str) -> (usize, usize, usize, usize, u32, usize, usize) {
    let root = Path::new(root);
    assert!(root.exists(), "run t2/run_e2e.sh first: {}", root.display());
    let (mut sets, mut cases, mut floats, mut bytes, mut maxulp, mut copies, mut longw) = (0, 0, 0, 0, 0u32, 0, 0);
    let mut dirs: Vec<_> = std::fs::read_dir(root).unwrap().map(|e| e.unwrap().path()).filter(|p| p.join("manifest.json").exists()).collect();
    dirs.sort();
    for d in dirs {
        sets += 1;
        let name = d.file_name().unwrap().to_str().unwrap().to_string();
        let tok: Vec<&str> = name.split('_').collect();
        let phase: usize = tok.last().unwrap()[1..].parse().unwrap();
        let (r, g, aspect): (f32, f32, Option<f32>) = match tok[0] {
            "asp" => (radius_tok(tok[1]), 0.7, Some(1.5)),
            "copy" => (if tok[1] == "r0" { 0.0 } else { 2.5 }, if tok[1] == "g0" { 0.0 } else { 1.0 }, None),
            "gnan" => (2.5, f32::NAN, None),
            _ => (radius_tok(tok[0]), gain_tok(tok[1]), None),
        };
        let text = std::fs::read_to_string(d.join("manifest.json")).unwrap().replace("NaN", "null").replace("Infinity", "null");
        let j = parse_json(&text).unwrap();
        for c in j.get("cases").unwrap().as_arr().unwrap() {
            let cname = c.get("name").unwrap().as_str().unwrap();
            assert_eq!(c.get("status").unwrap().as_str().unwrap(), "ok", "{} {}", name, cname);
            let f = |io: &str, n: &str| rd(&d.join(c.get(io).unwrap().get(n).unwrap().get("file").unwrap().as_str().unwrap()));
            let (x, out) = (f("inputs", "src"), f("outputs", "out"));
            let (w, h) = (c.get("width").unwrap().as_usize().unwrap(), c.get("height").unwrap().as_usize().unwrap());
            let want = sharpen_plane(&x, w, h, r, g, aspect, w * h, phase).unwrap();
            let df = PlaneDiff::compare(&out, w * h, &want, w * h, w * h, 1);
            println!("{:<22} {:<12} {}", name, cname, df.summary());
            if sharpen::plan::plane_plan(r, g) == sharpen::plan::PlanePlan::Copy {
                copies += 1;
            }
            if r >= 40.0 {
                longw += 1;
            }
            cases += 1;
            floats += w * h;
            bytes += df.differing_bytes;
            maxulp = maxulp.max(df.max_ulp);
        }
    }
    (sets, cases, floats, bytes, maxulp, copies, longw)
}

#[test]
fn e2e_small_radii() {
    if !effects_vecdir::have_all(&["crates/sharpen/t2/out/e2e_sq", "crates/sharpen/t2/out/e2e_strip"]) {
        return;
    }
    let r = run(effects_vecdir::path_str("crates/sharpen/t2/out/e2e_sq"));
    println!("T2 e2e small: sets {} cases {} floats {} differing bytes {} max ulp {} copy cases {} long-window cases {}", r.0, r.1, r.2, r.3, r.4, r.5, r.6);
    assert_eq!(r.3, 0);
}

#[test]
fn e2e_long_windows() {
    if !effects_vecdir::have_all(&["crates/sharpen/t2/out/e2e_sq", "crates/sharpen/t2/out/e2e_strip"]) {
        return;
    }
    let r = run(effects_vecdir::path_str("crates/sharpen/t2/out/e2e_strip"));
    println!("T2 e2e strips: sets {} cases {} floats {} differing bytes {} max ulp {} copy cases {} long-window cases {}", r.0, r.1, r.2, r.3, r.4, r.5, r.6);
    assert_eq!(r.3, 0);
}
