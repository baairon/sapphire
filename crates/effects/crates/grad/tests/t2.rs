// T2: compare grad::render with the emulated original slot 52 (0x53b460) run by t2/gr1_oracle.py (VEX engine follows the SDM, not silicon).
// Reads every <T2_DIR>/**/manifest.json (default ./t2_out). All three output planes (aux = blurred plane, dx, dy) are compared; NaN compares as a class.
// Without run folders the test passes vacuously and prints T2 PENDING. Env T2_TSV writes one row per case.
use fx_common::cmp::PlaneDiff;
use fx_common::oracle_io::Manifest;
use grad::*;
use std::io::Write;
use std::path::{Path, PathBuf};

fn find_manifests(dir: &Path, out: &mut Vec<PathBuf>) {
    if dir.join("manifest.json").is_file() {
        out.push(dir.to_path_buf());
    }
    if let Ok(rd) = std::fs::read_dir(dir) {
        let mut subs: Vec<PathBuf> = rd.filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.is_dir()).collect();
        subs.sort();
        for s in subs {
            find_manifests(&s, out);
        }
    }
}

#[test]
fn t2_against_emulated_original() {
    let root = std::env::var("T2_DIR").unwrap_or_else(|_| effects_vecdir::path_str("crates/grad/t2_out").to_string());
    let mut dirs = Vec::new();
    find_manifests(Path::new(&root), &mut dirs);
    if dirs.is_empty() {
        eprintln!("T2 PENDING: no run folder under {root}");
        return;
    }
    let mut tsv = std::env::var("T2_TSV").ok().map(|p| std::fs::File::create(p).unwrap());
    let (mut runs, mut floats, mut bad_bytes, mut max_ulp, mut faults, mut secs) = (0usize, 0usize, 0usize, 0u32, 0usize, 0f64);
    let mut failures: Vec<String> = Vec::new();
    for d in &dirs {
        let m = Manifest::load(d).unwrap();
        for c in &m.cases {
            if !c.ok() {
                faults += 1;
                failures.push(format!("{} {}: status {}", d.display(), c.name, c.status));
                continue;
            }
            runs += 1;
            secs += c.seconds;
            let (w, h) = (c.width, c.height);
            assert_eq!(c.stride, w, "dense planes only");
            let pf = |n: &str| c.param_f64(n).unwrap();
            let p = GradParams {
                amount: pf("amount") as f32,
                gain_param: pf("gain_param") as f32,
                subpixel: pf("subpixel") as i32,
                edge_mode: pf("edge") as i32,
                render_scale: pf("k") as i32,
                aspect: if pf("aspect_on") != 0.0 { Some(pf("aspect") as f32) } else { None },
                info_aspect_set: pf("info3c") != 0.0,
                flag_1: pf("flag") as i32,
                past_end: 0.0, // GR-A3: the harness plane is followed by zero pad inside its mapped page
            };
            let src = c.input("src").unwrap();
            let (wa, wx, wy) = (c.output("aux").unwrap(), c.output("dx").unwrap(), c.output("dy").unwrap());
            let sent = f32::from_bits(0x7fc0ffee);
            let (mut aux, mut dx, mut dy) = (vec![sent; w * h], vec![sent; w * h], vec![sent; w * h]);
            render(&p, &src, w, h, &mut aux, &mut dx, &mut dy).unwrap();
            let mut line = format!("{}\t{}\t{}x{}", m.param_set, c.name, w, h);
            for (nm, got, want) in [("aux", &aux, &wa), ("dx", &dx, &wx), ("dy", &dy, &wy)] {
                let diff = PlaneDiff::compare(got, w, want, w, w, h);
                floats += diff.samples;
                bad_bytes += diff.differing_bytes;
                if diff.differing_samples > 0 {
                    max_ulp = max_ulp.max(diff.max_ulp);
                }
                line.push_str(&format!("\t{}:{}", nm, diff.differing_bytes));
                if !diff.is_parity() {
                    failures.push(format!("{} {} {} {}: {}", d.display(), m.param_set, c.name, nm, diff.summary()));
                }
            }
            if let Some(f) = tsv.as_mut() {
                writeln!(f, "{line}").unwrap();
            }
        }
    }
    eprintln!("T2 TOTAL runs {runs} floats {floats} differing bytes {bad_bytes} max ulp {max_ulp} faults {faults} emulator seconds {secs:.1}");
    assert!(failures.is_empty(), "{} failures, first: {:?}", failures.len(), failures.iter().take(5).collect::<Vec<_>>());
}
