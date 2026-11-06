//! T2 of the native `set_aspect` port (src/aspect.rs) against STORED emulator runs of the original function 0xc6930
//! (t2/emu_sa.tsv, produced by t2/sa_drv.py through hole1_evg.py fix). Expected values are the original's, not a reading.
use fx_exec::aspect::*;
use std::io::Write;

/// the fixture is inside the integrated tree and its SHA-256 is checked against fixtures/MANIFEST.sha256 before it is read.
fn fixture(rel: &str) -> String {
    let root = effects_vecdir::path("fixtures");
    let p = root.join(rel);
    let man = std::fs::read_to_string(root.join("MANIFEST.sha256")).expect("fixtures/MANIFEST.sha256");
    let want = man.lines().find(|l| l.ends_with(&format!("  {}", rel))).expect("fixture not in the manifest").split_whitespace().next().unwrap().to_string();
    let out = std::process::Command::new("sha256sum").arg(&p).output().expect("sha256sum");
    let got = String::from_utf8_lossy(&out.stdout).split_whitespace().next().unwrap_or("").to_string();
    assert_eq!(got, want, "fixture SHA mismatch for {}", rel);
    std::fs::read_to_string(&p).expect("fixture")
}

fn h(s: &str) -> u32 {
    u32::from_str_radix(s, 16).unwrap()
}

#[test]
fn set_aspect_port_equals_emulated_original() {
    if !effects_vecdir::have_all(&["fixtures", "fixtures/drw4_t2"]) {
        return;
    }
    let dir = effects_vecdir::path("fixtures/drw4_t2");
    let txt = fixture("drw4_t2/emu_sa.tsv");
    let mut out = std::fs::File::create(effects_vecdir::out_dir().join("native_sa_rust.tsv")).unwrap();
    let (mut n, mut bad, mut errpath, mut notdone) = (0usize, 0usize, 0usize, 0usize);
    let mut first_bad: Option<String> = None;
    for line in txt.lines() {
        let f: Vec<&str> = line.split('\t').collect();
        let (a, aspect_block_a) = (f32::from_bits(h(f[0])), f32::from_bits(h(f[1])));
        if f[3] != "1" {
            notdone += 1;
            continue;
        }
        let emu: Vec<u32> = f[4..10].iter().map(|x| h(x)).collect();
        let nat = set_aspect(a, aspect_block_a);
        let nv: Vec<u32> = match nat {
            Some(b) => vec![b.aspect_block_b, b.aspect_block_c, b.aspect_block_d, b.aspect_block_e, b.aspect_block_f as u32, b.aspect_block_g as u32],
            None => {
                errpath += 1;
                vec![0xa5a5a5a5; 6]
            }
        };
        writeln!(out, "{}\t{}\t{}\t{}", f[0], f[1], f[2], nv.iter().map(|x| format!("{:08x}", x)).collect::<Vec<_>>().join("\t")).unwrap();
        n += 1;
        if nv != emu {
            bad += 1;
            if first_bad.is_none() {
                first_bad = Some(format!("a={} aspect_block_a={} native {:08x?} emulator {:08x?}", f[0], f[1], nv, emu));
            }
        }
    }
    eprintln!("set_aspect T2: {} cases compared, {} differing, {} a==0 error-path cases, {} not done", n, bad, errpath, notdone);
    assert!(n >= 10_000, "fewer than 10,000 cases");
    assert_eq!(notdone, 0);
    assert!(bad == 0, "first differing case: {}", first_bad.unwrap_or_default());
}

#[test]
fn caller_range_and_thresholds() {
    assert!(in_caller_range(1.0) && in_caller_range(0.5) && in_caller_range(2.0));
    assert!(!in_caller_range(0.0) && !in_caller_range(f32::NAN) && !in_caller_range(f32::INFINITY) && !in_caller_range(10000.0) && !in_caller_range(-1.0));
    assert!(in_caller_range(f32::from_bits(9999.999f32.to_bits())));
    // 1e-4 is a double bound: the f32 nearest 1e-4 is above the double 1e-4 only if its widening is larger
    let f = 1e-4f32;
    assert_eq!(in_caller_range(f), (f as f64) > 1e-4);
    let b = set_aspect(1.6f32, 0.0).unwrap();
    assert_eq!(b.aspect_block_e, 1);
    let b = set_aspect(f32::from_bits(1.6f32.to_bits() - 1), 0.0).unwrap();
    assert_eq!(b.aspect_block_e, 0);
}
