//! T2: native combine vs the combine job body 0x175e30 run in harness v2 (vex engine, "VEX follows the SDM, not silicon").
use fx_common::cmp::PlaneDiff;
use fx_common::oracle_io::parse_json;
use sharpen::combine::combine;
use std::path::Path;

fn gain_of(name: &str) -> f32 {
    let g = name.rsplit('_').next().unwrap();
    match g { "g0" => 0.0, "g05" => 0.5, "g1" => 1.0, "g2" => 2.0, "gnan" => f32::NAN, "gm1" => -1.0, "g06" => 0.6, _ => panic!("gain {}", name) }
}

fn read_f32(p: &Path) -> Vec<f32> {
    let b = std::fs::read(p).unwrap_or_else(|e| panic!("{}: {}", p.display(), e));
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

fn run(root: &str, part: bool) -> (usize, usize, usize, usize, u32) {
    let root = Path::new(root);
    assert!(root.exists(), "run t2/run_combine.sh first: {}", root.display());
    let (mut cases, mut floats, mut bytes, mut sets, mut maxulp) = (0, 0, 0, 0, 0u32);
    let mut dirs: Vec<_> = std::fs::read_dir(root).unwrap().map(|e| e.unwrap().path()).filter(|p| p.join("manifest.json").exists()).collect();
    dirs.sort();
    for d in dirs {
        sets += 1;
        let name = d.file_name().unwrap().to_str().unwrap().to_string();
        let g = gain_of(&name);
        let text = std::fs::read_to_string(d.join("manifest.json")).unwrap().replace("NaN", "null");
        let j = parse_json(&text).unwrap();
        for c in j.get("cases").unwrap().as_arr().unwrap() {
            let cname = c.get("name").unwrap().as_str().unwrap();
            let status = c.get("status").unwrap().as_str().unwrap();
            assert_eq!(status, "ok", "{} {}", name, cname);
            let f = |io: &str, n: &str| read_f32(&d.join(c.get(io).unwrap().get(n).unwrap().get("file").unwrap().as_str().unwrap()));
            let (x, b, want) = (f("inputs", "src"), f("inputs", "blur"), f("outputs", "out"));
            let count = if part {
                let cnt: usize = name.split('_').next().unwrap()[1..].parse().unwrap();
                cnt
            } else { want.len() };
            let mut got = vec![f32::from_bits(0x7fc0ffee); want.len()];
            combine(&x, &b, g, &mut got, count).unwrap();
            let df = PlaneDiff::compare(&got, want.len(), &want, want.len(), want.len(), 1);
            assert!(df.is_parity(), "{} {}: {}", name, cname, df.summary());
            cases += 1; floats += want.len(); bytes += df.differing_bytes; maxulp = maxulp.max(df.max_ulp);
        }
    }
    (sets, cases, floats, bytes, maxulp)
}

#[test]
fn combine_full_vs_emulated() {
    if !effects_vecdir::have_all(&["crates/sharpen/t2/out/combine_full", "crates/sharpen/t2/out/combine_part"]) {
        return;
    }
    let r = run(effects_vecdir::path_str("crates/sharpen/t2/out/combine_full"), false);
    println!("T2 combine full: sets {} cases {} floats {} differing bytes {} max ulp {}", r.0, r.1, r.2, r.3, r.4);
    assert_eq!(r.3, 0);
    assert!(r.1 >= 7 * 13);
}

#[test]
fn combine_partial_count_vs_emulated() {
    if !effects_vecdir::have_all(&["crates/sharpen/t2/out/combine_full", "crates/sharpen/t2/out/combine_part"]) {
        return;
    }
    let r = run(effects_vecdir::path_str("crates/sharpen/t2/out/combine_part"), true);
    println!("T2 combine part: sets {} cases {} floats {} differing bytes {} max ulp {}", r.0, r.1, r.2, r.3, r.4);
    assert_eq!(r.3, 0);
}
