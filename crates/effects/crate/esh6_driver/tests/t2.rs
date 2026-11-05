//! Judge: native slot52 against the emulator T2 export (tools/export_cases.py). Compares every plane (bitwise; NaN as a class) and the pool call list.
use esh6_driver::*;
use std::collections::HashMap;
use std::fs;

fn rd_f32(p: &str) -> Vec<f32> {
    let b = fs::read(p).unwrap();
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

#[test]
fn t2_all() {
    if !effects_vecdir::have_all(&["wave_s9/esh6_driver/t2/export"]) {
        return;
    }
    let dir = std::env::var("ESH6_T2").unwrap_or_else(|_| effects_vecdir::path_str("wave_s9/esh6_driver/t2/export").to_string());
    let only = std::env::var("ESH6_ONLY").ok();
    let idx = fs::read_to_string(format!("{}/index.txt", dir)).unwrap();
    let (mut cases, mut floats, mut bad, mut nanpay, mut poolbad, mut errs) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut first: Vec<String> = Vec::new();
    for cid in idx.lines() {
        if let Some(o) = &only {
            if !cid.contains(o.as_str()) {
                continue;
            }
        }
        let par = fs::read_to_string(format!("{}/{}.par", dir, cid)).unwrap();
        let mut m: HashMap<String, u32> = HashMap::new();
        for l in par.lines() {
            let mut it = l.split_whitespace();
            let k = it.next().unwrap().to_string();
            let v = it.next().unwrap();
            let r = if k_is_dec(&k) { 10 } else { 16 };
            m.insert(k, u32::from_str_radix(v, r).unwrap());
        }
        let f = |k: &str| f32::from_bits(m[k]);
        let n = m["n"] as i32;
        let mut arr = [0f32; 11];
        for k in 0..7 {
            arr[k] = f(&format!("a{}", k));
        }
        for k in 0..4 {
            arr[7 + k] = f(&format!("g{}", k));
        }
        let th = This { n, sig_byte: m["sig_byte"] != 0, sigma0: f("sigma0"), mode: f("mode"), variant: f("variant"), bcflag: f("bcflag"), iters: f("iters"), scale: f("scale"), output_a: f("f10c"), arr };
        let (w, h) = (m["w"] as i32, m["h"] as i32);
        let cx = Cx { cw: m["cw"] as i32, w, h, kcell: f("kcell") };
        let wh = (w * h) as usize;
        let nu = n as usize;
        let nt = 3 * nu + 5;
        let mut mem = rd_f32(&format!("{}/{}.in", dir, cid));
        let exp = rd_f32(&format!("{}/{}.out", dir, cid));
        assert_eq!(mem.len(), (4 + nt + nu) * wh, "{}", cid);
        let ids = Ids { inp: [0, wh, 2 * wh, 3 * wh], t: (0..nt).map(|k| (4 + k) * wh).collect(), out: (0..nu).map(|k| (4 + nt + k) * wh).collect() };
        cases += 1;
        let pools_exp: Vec<u32> = fs::read_to_string(format!("{}/{}.pools", dir, cid)).unwrap().split_whitespace().map(|s| u32::from_str_radix(s, 16).unwrap()).collect();
        match slot52(&th, &cx, &ids, &mut mem, 0) {
            Err(e) => {
                errs += 1;
                first.push(format!("{} native error {:?}", cid, e));
            }
            Ok(calls) => {
                if calls.pools != pools_exp {
                    poolbad += 1;
                    if first.len() < 12 {
                        first.push(format!("{} pools native {:x?} emu {:x?}", cid, calls.pools, pools_exp));
                    }
                }
                for i in 0..exp.len().min(mem.len()) {
                    floats += 1;
                    let (a, b) = (mem[i].to_bits(), exp[i].to_bits());
                    if a != b {
                        if mem[i].is_nan() && exp[i].is_nan() {
                            nanpay += 1;
                        } else {
                            bad += 1;
                            if first.len() < 12 {
                                first.push(format!("{} plane {} elem {} native {:08x} emu {:08x}", cid, i / wh, i % wh, a, b));
                            }
                        }
                    }
                }
            }
        }
    }
    println!("cases {} floats {} differing {} nan-payload-only {} pool-list mismatches {} native errors {}", cases, floats, bad, nanpay, poolbad, errs);
    for l in &first {
        println!("  {}", l);
    }
    assert!(cases > 0);
    assert_eq!(bad + poolbad + errs, 0);
}

fn k_is_dec(k: &str) -> bool {
    matches!(k, "w" | "h")
}
