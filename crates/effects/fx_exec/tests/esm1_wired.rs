//! stored cases (t2/export, emulator outputs) through the WIRED class in fx_exec (kernels::run_any, class edge_aware_sharpen).
//! Expected values are the original's stored output planes. Refusals are counted by name. The export is an in-tree fixture under fixtures/, checked by fixtures/MANIFEST.sha256 (its EXPORT.sha256 row) and sha256sum -c before it is read; env ESH6_T2 overrides the directory.
use fx_exec::kernels;
use fx_exec::types::{Arg, ExecError, Image, KParams};
use std::collections::HashMap;
use std::fs;

fn rd_f32(p: &str) -> Vec<f32> {
    fs::read(p).unwrap().chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}
fn dec(k: &str) -> bool {
    matches!(k, "w" | "h" | "n" | "nth")
}
fn cst(v: &[f32]) -> Arg<'static> {
    let mut l = [0u32; 4];
    for (i, x) in v.iter().enumerate().take(4) {
        l[i] = x.to_bits();
    }
    Arg::Const { n: v.len(), lanes: l }
}

/// the in-tree fixture, its file list checked against fixtures/MANIFEST.sha256, then every file by sha256sum -c.
fn export_dir() -> String {
    if let Ok(d) = std::env::var("ESH6_T2") {
        return d;
    }
    let root = effects_vecdir::path("fixtures");
    let man = fs::read_to_string(root.join("MANIFEST.sha256")).expect("fixtures/MANIFEST.sha256");
    let want = man.lines().find(|l| l.ends_with("  esh6_t2/EXPORT.sha256")).expect("fixture not in the manifest").split_whitespace().next().unwrap().to_string();
    let out = std::process::Command::new("sha256sum").arg(root.join("esh6_t2/EXPORT.sha256")).output().expect("sha256sum");
    let got = String::from_utf8_lossy(&out.stdout).split_whitespace().next().unwrap_or("").to_string();
    assert_eq!(got, want, "fixture SHA mismatch for the export checksum file");
    let exp = root.join("esh6_t2").join("export");
    let st = std::process::Command::new("sha256sum").arg("-c").arg("--quiet").arg("../EXPORT.sha256").current_dir(&exp).status().expect("sha256sum -c");
    assert!(st.success(), "the export files differ from EXPORT.sha256");
    exp.to_string_lossy().to_string()
}

#[test]
fn wired_class_equals_stored_emulator_cases() {
    if !effects_vecdir::have_all(&["fixtures"]) {
        return;
    }
    let dir = export_dir();
    let idx = fs::read_to_string(format!("{}/index.txt", dir)).unwrap();
    let (mut cases, mut admitted, mut planes, mut floats, mut bad, mut nanpay) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut refused: HashMap<String, u64> = HashMap::new();
    let mut stale_differs = 0u64;
    let mut kcell_not_one = 0u64;
    let mut first: Vec<String> = vec![];
    for cid in idx.lines() {
        let par = fs::read_to_string(format!("{}/{}.par", dir, cid)).unwrap();
        let mut m: HashMap<String, u32> = HashMap::new();
        for l in par.lines() {
            let mut it = l.split_whitespace();
            let k = it.next().unwrap().to_string();
            let v = it.next().unwrap();
            m.insert(k.clone(), u32::from_str_radix(v, if dec(&k) { 10 } else { 16 }).unwrap());
        }
        cases += 1;
        let f = |k: &str| f32::from_bits(m[k]);
        let (n, w, h) = (m["n"] as usize, m["w"] as usize, m["h"] as usize);
        let wh = w * h;
        let nt = 3 * n + 5;
        let mem0 = rd_f32(&format!("{}/{}.in", dir, cid));
        let exp = rd_f32(&format!("{}/{}.out", dir, cid));
        let plane = |mem: &Vec<f32>, k: usize| mem[k * wh..(k + 1) * wh].to_vec();
        let sig_byte = m["sig_byte"] != 0;
        if f("a4").to_bits() != 3 || f("a5").to_bits() != 1 || f("a6").to_bits() != 0 {
            stale_differs += 1;
        }
        if f("kcell") != 1.0 {
            kcell_not_one += 1;
        }
        let nsrc = n.min(3);
        let src = Image { n: wh, planes: (0..nsrc).map(|k| plane(&mem0, k)).collect() };
        let mask_img = Image { n: wh, planes: vec![plane(&mem0, 4 + 3 * n + 4)] };
        let src_arg = if n == 4 { Arg::Img(&src) } else { Arg::Img(&src) };
        let args: Vec<Arg> = vec![
            src_arg,
            if sig_byte { cst(&[f("sigma0")]) } else { Arg::Img(&mask_img) },
            cst(&[f("mode")]),
            cst(&[f("variant"), f("bcflag")]),
            cst(&[f("iters")]),
            cst(&[f("scale")]),
            cst(&[f("f10c")]),
            cst(&[f("a0"), f("a1"), f("a2"), f("a3")]),
            cst(&[f("g0"), f("g1"), f("g2")]),
            cst(&[1.0]),
        ];
        let mut kp = KParams::standin();
        kp.blur_render_scale = m["cw"] as i32;
        kp.host_aspect = f("kcell");
        if n == 4 {
            // outside the claim: the wired class must refuse (NCH-RANGE); the source cannot be built with 4 planes here, so only the refusal is asserted
            let src4 = Image { n: wh, planes: (0..4).map(|k| plane(&mem0, k)).collect() };
            let mut a4 = args.clone();
            a4[0] = Arg::Img(&src4);
            match kernels::run_any("edge_aware_sharpen", 4, &a4, wh, (w, h), &kp) {
                Err(ExecError::Unsupported { reason, .. }) if reason.contains("NCH-RANGE") => *refused.entry("NCH-RANGE".into()).or_insert(0) += 1,
                other => first.push(format!("{} n=4 not refused by name: {:?}", cid, other.err())),
            }
            continue;
        }
        match kernels::run_any("edge_aware_sharpen", n as i32, &args, wh, (w, h), &kp) {
            Err(ExecError::Unsupported { reason, .. }) if reason.contains("PORT-LIMIT VARIANT-NCH1") => {
                *refused.entry("PORT-LIMIT VARIANT-NCH1".into()).or_insert(0) += 1;
            }
            Err(e) => first.push(format!("{} error {:?}", cid, e)),
            Ok(o) => {
                let img = match o {
                    fx_exec::kernels2::Out::Img(i) => i,
                    _ => panic!("not an image"),
                };
                admitted += 1;
                assert_eq!(img.planes.len(), n);
                for k in 0..n {
                    planes += 1;
                    let e = plane(&exp, 4 + nt + k);
                    for i in 0..wh {
                        floats += 1;
                        let (a, b) = (img.planes[k][i], e[i]);
                        if a.to_bits() != b.to_bits() {
                            if a.is_nan() && b.is_nan() {
                                nanpay += 1;
                            } else {
                                bad += 1;
                                if first.len() < 12 {
                                    first.push(format!("{} plane {} elem {} native {:08x} emu {:08x}", cid, k, i, a.to_bits(), b.to_bits()));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    println!("ESM1_WIRED cases {} admitted {} planes {} floats {} differing {} nan_payload_only {} refused {:?} stale_cells_differ_from_fixed {} kcell_not_1 {}", cases, admitted, planes, floats, bad, nanpay, refused, stale_differs, kcell_not_one);
    for l in &first {
        println!("FIRST {}", l);
    }
    assert!(first.is_empty());
    assert_eq!(bad, 0);
    assert_eq!(admitted, 897);
}

/// The arm's own refusals reachable through the operands: levels above 7 (ITERS-ABOVE-7), levels 7 accepted by the class gate, a combo/source channel mismatch.
#[test]
fn wired_arm_refusals_by_name() {
    let (w, h) = (8usize, 4usize);
    let wh = w * h;
    let src = Image { n: wh, planes: (0..3).map(|k| (0..wh).map(|i| ((i * 7 + k * 13) % 17) as f32 / 17.0).collect()).collect() };
    let run = |levels: f32, tag: i32| {
        let args: Vec<Arg> = vec![Arg::Img(&src), cst(&[1.5]), cst(&[4.0]), cst(&[0.0, 0.0]), cst(&[levels]), cst(&[0.005]), cst(&[0.15]), cst(&[3.0, 1.5, 1.0, 2.0]), cst(&[1.0, 1.0, 1.0]), cst(&[1.0])];
        let mut kp = KParams::standin();
        kp.blur_render_scale = 1;
        kernels::run_any("edge_aware_sharpen", tag, &args, wh, (w, h), &kp)
    };
    for lv in [8.0f32, 9.0, 100.0] {
        match run(lv, 3) {
            Err(ExecError::Unsupported { reason, .. }) => assert!(reason.contains("ITERS-ABOVE-7"), "{}", reason),
            other => panic!("levels {} not refused: {:?}", lv, other.is_ok()),
        }
    }
    assert!(run(4.0, 3).is_ok());
    assert!(matches!(run(4.0, 2), Err(ExecError::Shape(_))));
}
