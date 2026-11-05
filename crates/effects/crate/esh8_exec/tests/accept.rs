//! ESH8 acceptance: the executor class against ESH6's stored emulator cases (t2/export) and against the slot52 port, plus refusal and tripwire tests.
use esh6_driver as d;
use esh8_exec::*;
use std::collections::HashMap;
use std::fs;

fn rd_f32(p: &str) -> Vec<f32> {
    fs::read(p).unwrap().chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

fn dec(k: &str) -> bool {
    matches!(k, "w" | "h" | "n" | "nth")
}

#[derive(Default)]
struct Tally {
    cases: u64,
    refused: HashMap<String, u64>,
    floats_emu: u64,
    bad_emu: u64,
    nanpay_emu: u64,
    bad_ref: u64,
    nanpay_ref: u64,
    bad_poison: u64,
    nanpay_poison: u64,
    trip: u64,
    first: Vec<String>,
    ref_n4_cases: u64,
    ref_n4_bad: u64,
}

fn cmp(a: &[f32], b: &[f32], bad: &mut u64, nanpay: &mut u64) -> Option<(usize, u32, u32)> {
    let mut first = None;
    for i in 0..a.len() {
        if a[i].to_bits() != b[i].to_bits() {
            if a[i].is_nan() && b[i].is_nan() {
                *nanpay += 1;
            } else {
                *bad += 1;
                if first.is_none() {
                    first = Some((i, a[i].to_bits(), b[i].to_bits()));
                }
            }
        }
    }
    first
}

#[test]
fn accept_stored_cases() {
    if !effects_vecdir::have_all(&["wave_s9/esh6_driver/t2/export"]) {
        return;
    }
    let dir = std::env::var("ESH6_T2").unwrap_or_else(|_| effects_vecdir::path_str("wave_s9/esh6_driver/t2/export").to_string());
    let idx = fs::read_to_string(format!("{}/index.txt", dir)).unwrap();
    let mut t = Tally::default();
    let mut compared_floats_emu = 0u64;
    for cid in idx.lines() {
        let par = fs::read_to_string(format!("{}/{}.par", dir, cid)).unwrap();
        let mut m: HashMap<String, u32> = HashMap::new();
        for l in par.lines() {
            let mut it = l.split_whitespace();
            let k = it.next().unwrap().to_string();
            let v = it.next().unwrap();
            let r = if dec(&k) { 10 } else { 16 };
            m.insert(k, u32::from_str_radix(v, r).unwrap());
        }
        t.cases += 1;
        let f = |k: &str| f32::from_bits(m[k]);
        let n = m["n"] as i32;
        let (w, h) = (m["w"] as i32, m["h"] as i32);
        let wh = (w * h) as usize;
        let nu = n as usize;
        let nt = 3 * nu + 5;
        let mem0 = rd_f32(&format!("{}/{}.in", dir, cid));
        let exp = rd_f32(&format!("{}/{}.out", dir, cid));
        let plane = |mem: &Vec<f32>, k: usize| mem[k * wh..(k + 1) * wh].to_vec();
        let mut sg = [0f32; 7];
        for k in 0..7 {
            sg[k] = f(&format!("a{}", k));
        }
        let sig_byte = m["sig_byte"] != 0;
        let ops = Operands {
            mask: if sig_byte { MaskKind::Scalar(f("sigma0")) } else { MaskKind::Plane },
            blur_type: f("mode"),
            deriv_io: [f("variant"), f("bcflag")],
            levels: f("iters"),
            tiny_detail_size: f("scale"),
            edge_threshold: f("f10c"),
            sigmoid_scale: sg,
            rel_chan: [f("g0"), f("g1"), f("g2")],
        };
        // the ESH6 reference run (the slot52 port on the full arena) for every case, including the cases the executor refuses
        let mut arr = [0f32; 11];
        for k in 0..7 {
            arr[k] = sg[k];
        }
        for k in 0..4 {
            arr[7 + k] = f(&format!("g{}", k));
        }
        let th = d::This { n, sig_byte, sigma0: f("sigma0"), mode: f("mode"), variant: f("variant"), bcflag: f("bcflag"), iters: f("iters"), scale: f("scale"), output_a: f("f10c"), arr };
        let cxd = d::Cx { cw: m["cw"] as i32, w, h, kcell: f("kcell") };
        let ids = d::Ids { inp: [0, wh, 2 * wh, 3 * wh], t: (0..nt).map(|k| (4 + k) * wh).collect(), out: (0..nu).map(|k| (4 + nt + k) * wh).collect() };
        let mut refmem = mem0.clone();
        let refrun = d::slot52(&th, &cxd, &ids, &mut refmem, 0);
        assert!(refrun.is_ok(), "{} reference error {:?}", cid, refrun.err());
        let ref_out: Vec<Vec<f32>> = (0..nu).map(|k| plane(&refmem, 4 + nt + k)).collect();
        let exp_out: Vec<Vec<f32>> = (0..nu).map(|k| plane(&exp, 4 + nt + k)).collect();
        let cc = CompileCtx { tag: 0, flag: 0, nch: n, cw: m["cw"] as i32, w, h, kcell: f("kcell") };
        let srcs: Vec<Vec<f32>> = (0..nu.min(3)).map(|k| plane(&mem0, k)).collect();
        let srefs: Vec<&[f32]> = srcs.iter().map(|v| v.as_slice()).collect();
        let curve = plane(&mem0, 4 + 3 * nu + 4);
        let inp = Inputs { source: &srefs, mask: if sig_byte { None } else { Some(&curve) } };
        if n == 4 {
            // outside the executor's nch range: refused; the port itself is judged on these by ESH6 (reference vs emulator here)
            assert_eq!(run(&cc, &ops, &inp, Scratch::Zero).err(), Some(ExecErr::NchRange(4)));
            *t.refused.entry("NchRange(4)".into()).or_insert(0) += 1;
            t.ref_n4_cases += 1;
            for k in 0..nu {
                cmp(&ref_out[k], &exp_out[k], &mut t.ref_n4_bad, &mut 0);
            }
            continue;
        }
        let r0 = run(&cc, &ops, &inp, Scratch::Zero);
        let o0 = match r0 {
            Err(ExecErr::VariantNch1) => {
                *t.refused.entry("VariantNch1".into()).or_insert(0) += 1;
                continue;
            }
            Err(e) => {
                if matches!(e, ExecErr::Tripwire(_)) {
                    t.trip += 1;
                }
                t.first.push(format!("{} executor error {:?}", cid, e));
                continue;
            }
            Ok(o) => o,
        };
        assert_eq!(o0.plane3_accesses, 0);
        let o1 = run(&cc, &ops, &inp, Scratch::Poison(0x7fc0_dead)).unwrap();
        for k in 0..nu {
            t.floats_emu += wh as u64;
            compared_floats_emu += wh as u64;
            if let Some((i, a, b)) = cmp(&o0.planes[k], &exp_out[k], &mut t.bad_emu, &mut t.nanpay_emu) {
                if t.first.len() < 12 {
                    t.first.push(format!("{} vs emulator plane {} elem {} native {:08x} emu {:08x}", cid, k, i, a, b));
                }
            }
            cmp(&o0.planes[k], &ref_out[k], &mut t.bad_ref, &mut t.nanpay_ref);
            cmp(&o1.planes[k], &o0.planes[k], &mut t.bad_poison, &mut t.nanpay_poison);
        }
        assert_eq!(o0.calls.pools, refrun.as_ref().unwrap().pools, "{} pool list", cid);
    }
    let adm = t.cases - t.refused.values().sum::<u64>() - t.trip;
    println!("RESULT cases {} refused {:?} admitted {} compared_floats {} vs_emulator bad {} nanpayload_only {} vs_slot52_port bad {} nanpayload_only {} poison_vs_zero bad {} nanpayload_only {} tripwire_fired {} errors {:?}", t.cases, t.refused, adm, compared_floats_emu, t.bad_emu, t.nanpay_emu, t.bad_ref, t.nanpay_ref, t.bad_poison, t.nanpay_poison, t.trip, t.first);
    println!("RESULT n4 (refused by the executor) port-vs-emulator differing floats {} in {} cases", t.ref_n4_bad, t.ref_n4_cases);
    assert_eq!(t.bad_emu + t.bad_ref + t.bad_poison, 0);
    assert!(t.first.is_empty());
}

fn base_ops() -> Operands {
    Operands { mask: MaskKind::Scalar(1.5), blur_type: 4.0, deriv_io: [0.0, 0.0], levels: 4.0, tiny_detail_size: 0.005, edge_threshold: 0.15, sigmoid_scale: [3.0, 1.5, 1.0, 2.0, 4.2e-45, 1.4e-45, 0.0], rel_chan: [1.0, 1.0, 1.0] }
}
fn base_ctx(nch: i32) -> CompileCtx {
    CompileCtx { tag: 3, flag: 0, nch, cw: 1, w: 8, h: 4, kcell: 1.0 }
}
fn img(n: usize) -> Vec<Vec<f32>> {
    (0..n).map(|k| (0..32).map(|i| ((i * 7 + k * 13) % 17) as f32 / 17.0).collect()).collect()
}

#[test]
fn refusals_and_mask_rule() {
    let im = img(3);
    let s: Vec<&[f32]> = im.iter().map(|v| v.as_slice()).collect();
    let inp = Inputs { source: &s, mask: None };
    let ops = base_ops();
    assert!(run(&base_ctx(3), &ops, &inp, Scratch::Zero).is_ok());
    let mut c = base_ctx(3);
    c.flag = 1;
    assert_eq!(run(&c, &ops, &inp, Scratch::Zero).err(), Some(ExecErr::FlagNonzero(1)));
    let mut c = base_ctx(3);
    c.tag = -1;
    assert_eq!(run(&c, &ops, &inp, Scratch::Zero).err(), Some(ExecErr::TagNegative(-1)));
    for n in [-1, 0, 4, 5] {
        assert_eq!(run(&base_ctx(n), &ops, &inp, Scratch::Zero).err(), Some(ExecErr::NchRange(n)));
    }
    let mut o = base_ops();
    o.levels = 8.0;
    assert_eq!(run(&base_ctx(3), &o, &inp, Scratch::Zero).err(), Some(ExecErr::ItersAbove7(8)));
    o.levels = 7.0;
    assert!(run(&base_ctx(3), &o, &inp, Scratch::Zero).is_ok());
    // mask plane kind with input 1 unbound
    let mut o = base_ops();
    o.mask = MaskKind::Plane;
    assert_eq!(run(&base_ctx(3), &o, &inp, Scratch::Zero).err(), Some(ExecErr::MaskUnbound));
    let m = vec![0.5f32; 32];
    let inp2 = Inputs { source: &s, mask: Some(&m) };
    assert!(run(&base_ctx(3), &o, &inp2, Scratch::Zero).is_ok());
    // variant != 0 at nch 1
    let mut o = base_ops();
    o.deriv_io = [1.0, 0.0];
    let s1 = &s[..1];
    let inp3 = Inputs { source: s1, mask: None };
    assert_eq!(run(&base_ctx(1), &o, &inp3, Scratch::Zero).err(), Some(ExecErr::VariantNch1));
    // wrong source count
    assert!(matches!(run(&base_ctx(2), &base_ops(), &inp, Scratch::Zero).err(), Some(ExecErr::Shape(_))));
}

/// Tripwire demonstration: handing the slot52 port an absent plane that the route reads must fail, never read a stand-in.
#[test]
fn tripwire_fires_on_absent_plane() {
    let wh = 32usize;
    let nu = 3usize;
    let nt = 3 * nu + 5;
    let mut mem = vec![0.25f32; (4 + nt + nu) * wh];
    let th = d::This { n: 3, sig_byte: true, sigma0: 1.5, mode: 4.0, variant: 0.0, bcflag: 0.0, iters: 4.0, scale: 0.005, output_a: 0.15, arr: [3.0, 1.5, 1.0, 2.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0] };
    let cx = d::Cx { cw: 1, w: 8, h: 4, kcell: 1.0 };
    let mk = |inp: [usize; 4]| d::Ids { inp, t: (0..nt).map(|k| (4 + k) * wh).collect(), out: (0..nu).map(|k| (4 + nt + k) * wh).collect() };
    // absent plane 3 at n = 3: silent (never read)
    assert!(d::slot52(&th, &cx, &mk([0, wh, 2 * wh, d::NO_PLANE]), &mut mem.clone(), 0).is_ok());
    // absent plane 1 at n = 3: fires
    assert_eq!(d::slot52(&th, &cx, &mk([0, d::NO_PLANE, 2 * wh, d::NO_PLANE]), &mut mem, 0).err(), Some(d::DErr::Tripwire(1)));
    // n = 4 with plane 3 absent: the weight job's own length guard stops it (no stand-in plane is read)
    let th4 = d::This { n: 4, ..th.clone() };
    let nt4 = 3 * 4 + 5;
    let mut mem4 = vec![0.25f32; (4 + nt4 + 4) * wh];
    let ids4 = d::Ids { inp: [0, wh, 2 * wh, d::NO_PLANE], t: (0..nt4).map(|k| (4 + k) * wh).collect(), out: (0..4).map(|k| (4 + nt4 + k) * wh).collect() };
    assert!(d::slot52(&th4, &cx, &ids4, &mut mem4, 0).is_err());
}
