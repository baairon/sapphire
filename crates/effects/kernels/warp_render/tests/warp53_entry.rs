//! slot-function T2 for class warp slot 53 (0x56fac0): the Rust glue (`slot53_warp` over the warp_driver and warp_sampler crates) against the
//! emulated original (harness v2 vex engine, SDM not silicon; real code of the wrapper, 0x5ba5c0, 0xc6930, the driver dispatcher and AVX2 body,
//! the accessors, F1 and fmod; memcpy/memset models, round_q12 model, recorded 0x66a800, 0x5b9bd0, 0x6758e0, 0x66ad00). Cases: t2/cases_warp53.tsv.
//! Adapted from the test of warp_rel. Written: t2/WARP53_RESULT.tsv.
use std::io::Write;
use warp_render::*;

fn f(h: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(h, 16).unwrap())
}
fn fl(s: &str) -> Vec<f32> {
    if s.is_empty() { vec![] } else { s.split(',').map(f).collect() }
}
fn il(s: &str) -> Vec<i32> {
    if s.is_empty() { vec![] } else { s.split(',').map(|x| x.parse().unwrap()).collect() }
}
fn class_eq(a: u32, b: u32) -> bool {
    let na = (a >> 23) & 0xff == 0xff && a & 0x7fffff != 0;
    let nb = (b >> 23) & 0xff == 0xff && b & 0x7fffff != 0;
    (na && nb) || a == b
}
fn ord(b: u32) -> i64 {
    if b & 0x8000_0000 != 0 { -((b & 0x7fff_ffff) as i64) } else { b as i64 }
}

#[derive(Clone)]
struct Case {
    name: String,
    n: i32,
    cnt: i32,
    w: i32,
    h: i32,
    d_cell_d: i32,
    d_cell_e: i32,
    wt0: i32,
    ht0: i32,
    tstep: f32,
    fill: Vec<f32>,
    p: WarpPrim,
    rec: WarpRec,
    geom: Option<ImageGeom>,
    src: Vec<f32>,
}

fn parse(c: &[&str]) -> Case {
    let pi = |i: usize| c[i].parse::<i32>().unwrap();
    Case {
        name: c[0].to_string(),
        n: pi(1),
        cnt: pi(2),
        w: pi(3),
        h: pi(4),
        d_cell_d: pi(5),
        d_cell_e: pi(6),
        wt0: pi(21),
        ht0: pi(22),
        tstep: f(c[20]),
        fill: fl(c[28]),
        p: WarpPrim { const_coords: pi(10) != 0, const_a1: f(c[34]), const_a2: f(c[35]), a1_arr: fl(c[29]), a2_arr: fl(c[30]), c1_arr: fl(c[31]), c2_arr: fl(c[32]), flag: pi(9) as u8, mx: pi(7), my: pi(8) },
        rec: WarpRec { count: pi(2), aspect_block_d: f(c[17]), assumption_e: f(c[42]), idx: pi(33), assumption_c: pi(11), assumption_d: pi(12), assumption_f: f(c[18]), assumption_g: f(c[19]), assumption_h: f(c[43]) },
        geom: if pi(44) != 0 { Some(ImageGeom { left: f(c[45]), right: f(c[46]), assumption_h: f(c[47]), aspect: f(c[48]), assumption_c: pi(49), assumption_d: pi(50) }) } else { None },
        src: if c.len() > 40 { fl(c[40]) } else { vec![] },
    }
}

fn image(c: &Case) -> DriverImage {
    DriverImage {
        f1: F1Params { n: c.n, width_tab: vec![c.wt0; 8], height_tab: vec![c.ht0; 8], t: (0..16).map(|k| 1.0 + k as f32 * c.tstep).collect(), flag_x: 0, flag_y: 0 },
        w: c.w,
        h: c.h,
        d_cell_d: c.d_cell_d,
        d_cell_e: c.d_cell_e,
    }
}

fn tokens(c: &Case, recs: &[DriverRec]) -> String {
    let mut v = Vec::new();
    if ctx_logs_error(c.geom.as_ref()) {
        v.push("G".to_string());
    }
    for r in recs {
        match r {
            DriverRec::AssertCellA => v.push("A".to_string()),
            DriverRec::Fill { k } => {
                v.push("D:ok".to_string());
                v.push(format!("L:{}:{:08x}", k, c.fill.get(*k as usize).copied().unwrap_or(0.0).to_bits()));
            }
            _ => {}
        }
    }
    v.join(" ")
}

fn token1(c: &Case) -> String {
    let w = wrapper_args(&c.p, c.rec.idx).unwrap();
    format!("{}W:ok:r9={}:a1={:08x}:a2={:08x}:c1={:08x}:c2={:08x}:fl={}:mx={}:my={}", if ctx_logs_error(c.geom.as_ref()) { "G " } else { "" }, w.const_coords as i32, w.a1[0].to_bits(), w.a2[0].to_bits(), w.c1[0].to_bits(), w.c2[0].to_bits(), w.flag, w.mx, w.my)
}

type Res = Result<(Vec<Vec<f32>>, Vec<DriverRec>), WarpError>;

fn exec(c: &Case, src: &[f32]) -> Res {
    let mut img = image(c);
    let lat = Lattice { bufs: vec![vec![src.to_vec(); 8]; 8] };
    let mut out = vec![vec![f32::from_bits(0x7fc0ffee); c.cnt.max(0) as usize]; c.n.max(0) as usize];
    let r = slot53_warp(&c.p, &c.rec, c.geom.as_ref(), &mut img, &lat, &mut out)?;
    Ok((out, r))
}

#[test]
fn warp53_against_emulated_original() {
    if !effects_vecdir::have_all(&["crates/wpg1_glue/t2/cases_warp53.tsv"]) {
        return;
    }
    let txt = std::fs::read_to_string(effects_vecdir::path("crates/wpg1_glue/t2/cases_warp53.tsv")).unwrap();
    let mut lines = Vec::new();
    let (mut cases, mut compared, mut floats, mut diffb, mut maxulp) = (0, 0, 0usize, 0usize, 0i64);
    let (mut rec_diff, mut rec_tokens, mut st1_cmp, mut geo_cmp, mut flagpath_cmp) = (0, 0usize, 0, 0, 0);
    let (mut not_cmp, mut both_fault, mut limits, mut limit_emu_ok) = (0, 0, 0, 0);
    let mut by_n = std::collections::BTreeMap::new();
    let mut by_mode = std::collections::BTreeMap::new();
    let mut kept: Vec<Case> = Vec::new();
    for ln in txt.lines() {
        let c: Vec<&str> = ln.split('\t').collect();
        cases += 1;
        let cs = parse(&c);
        // the harness labels 0x5b9bd0's rsi 'badrsi' because here rsi is the 0x7d0 byte stack context built by 0x5ba5c0 (not a layout region): same call, normalised
        let rec2_norm = c[39].replace("D:badrsi", "D:ok");
        let (st1, rec1, st2, rec2) = (c[36], c[37], c[38], rec2_norm.as_str());
        let src = if cs.src.is_empty() { vec![0.0f32; 4096] } else { cs.src.clone() };
        if st1 == "ok" {
            st1_cmp += 1;
            rec_tokens += 1;
            let mine = token1(&cs);
            if mine != rec1 {
                rec_diff += 1;
                lines.push(format!("{}\tRECORD DIFF stage1\n  port {}\n  emu  {}", cs.name, mine, rec1));
            }
        } else {
            lines.push(format!("{}\tSTAGE1 EMULATOR FAULT '{}'", cs.name, st1));
        }
        let res = exec(&cs, &src);
        if let Err(WarpError::ContractLimit { n }) = &res {
            limits += 1;
            if st2 == "ok" {
                limit_emu_ok += 1;
            }
            lines.push(format!("{}\tCONTRACT-LIMIT n={} flag={} emulator '{}' | port ContractLimit (not compared)", cs.name, n, cs.p.flag, st2));
            continue;
        }
        match (st2, &res) {
            ("ok", Ok((out, recs))) => {
                let mine2 = tokens(&cs, recs);
                rec_tokens += 1;
                if mine2 != rec2 {
                    rec_diff += 1;
                    lines.push(format!("{}\tRECORD DIFF stage2\n  port {}\n  emu  {}", cs.name, mine2, rec2));
                }
                let emu: Vec<Vec<u32>> = c[41].split(';').map(|p| if p.is_empty() { vec![] } else { p.split(',').map(|h| u32::from_str_radix(h, 16).unwrap()).collect() }).collect();
                compared += 1;
                if cs.geom.is_some() { geo_cmp += 1; }
                if cs.p.flag != 0 { flagpath_cmp += 1; }
                *by_n.entry(cs.n).or_insert(0) += 1;
                *by_mode.entry(format!("m{}{}{}{}", cs.p.mx, cs.p.my, if cs.p.flag != 0 { "F" } else { "B" }, if cs.geom.is_some() { "G" } else { "" })).or_insert(0) += 1;
                let mut bad = 0;
                for k in 0..(cs.n.max(0) as usize).min(emu.len()) {
                    for p in 0..(cs.cnt.max(0) as usize).min(emu[k].len()) {
                        floats += 1;
                        let (g, e) = (out[k][p].to_bits(), emu[k][p]);
                        if !class_eq(g, e) {
                            diffb += 1;
                            bad += 1;
                            maxulp = maxulp.max((ord(g) - ord(e)).abs());
                            if bad < 4 {
                                lines.push(format!("{}\tDIFF plane {} col {} port {:08x} emu {:08x}", cs.name, k, p, g, e));
                            }
                        }
                    }
                }
                kept.push(Case { src: src.clone(), ..cs.clone() });
            }
            ("ok", Err(e)) => {
                not_cmp += 1;
                diffb += 1;
                lines.push(format!("{}\tDIFF: emulator ok, port error {:?}", cs.name, e));
            }
            (s, Err(e)) => {
                both_fault += 1;
                lines.push(format!("{}\tBOTH FAULT (outside contract, matched): emu '{}' | port {:?}", cs.name, s, e));
            }
            (s, Ok(_)) => {
                not_cmp += 1;
                lines.push(format!("{}\tNOT COMPARED: emulator fault '{}' (port ran ok)", cs.name, s));
            }
        }
    }
    type Mut = (&'static str, fn(&mut Case));
    let muts: Vec<Mut> = vec![
        ("idx + 1", |c| c.rec.idx += 1),
        ("const_coords toggled", |c| c.p.const_coords = !c.p.const_coords),
        ("a1_arr[idx] + 0.37", |c| { let i = c.rec.idx as usize; c.p.a1_arr[i] += 0.37 }),
        ("a2_arr[idx] + 0.37", |c| { let i = c.rec.idx as usize; c.p.a2_arr[i] += 0.37 }),
        ("c1 and c2 swapped", |c| std::mem::swap(&mut c.p.c1_arr, &mut c.p.c2_arr)),
        ("flag toggled", |c| c.p.flag = if c.p.flag == 0 { 1 } else { 0 }),
        ("mx + 1 mod 3", |c| c.p.mx = (c.p.mx + 1) % 3),
        ("my + 1 mod 3", |c| c.p.my = (c.p.my + 1) % 3),
        ("w + 1", |c| c.w += 1),
        ("h + 1", |c| c.h += 1),
        ("assumption_e + 0.5", |c| c.rec.assumption_e += 0.5),
        ("assumption_h + 0.5", |c| c.rec.assumption_h += 0.5),
        ("aspect_block_d + 0.5", |c| c.rec.aspect_block_d += 0.5),
        ("assumption_c + 1", |c| c.rec.assumption_c += 1),
        ("assumption_d + 1", |c| c.rec.assumption_d += 1),
        ("geometry block toggled", |c| c.geom = if c.geom.is_some() { None } else { Some(ImageGeom { left: -1.0, right: 9.0, assumption_h: 0.25, aspect: 1.5, assumption_c: 1, assumption_d: 2 }) }),
        ("geometry aspect * 2", |c| if let Some(g) = c.geom.as_mut() { g.aspect *= 2.0 }),
    ];
    for (nm, m) in &muts {
        let (mut changed, mut total) = (0, 0);
        for c in &kept {
            let base = exec(c, &c.src);
            if base.is_err() || (c.rec.idx as usize) + 1 >= c.p.a1_arr.len() {
                continue;
            }
            let mut c2 = c.clone();
            m(&mut c2);
            let r2 = exec(&c2, &c2.src);
            total += 1;
            if differs_bits(&base, &r2) {
                changed += 1;
            }
        }
        lines.push(format!("SENSITIVITY {}\t{}/{}", nm, changed, total));
    }
    lines.push(format!(
        "TOTAL cases={} compared={} floats={} diff_floats={} max_ulp={} not_compared={} both_fault={} stage1_cases={} record_diffs={} record_tokens={} contract_limit_cases={} contract_limit_emulator_ok={} geometry_cases_compared={} flag_cases_compared={} by_n={:?} by_mode={:?}",
        cases, compared, floats, diffb, maxulp, not_cmp, both_fault, st1_cmp, rec_diff, rec_tokens, limits, limit_emu_ok, geo_cmp, flagpath_cmp, by_n, by_mode
    ));
    std::fs::File::create(effects_vecdir::out_dir().join("WARP53_RESULT.tsv")).unwrap().write_all(lines.join("\n").as_bytes()).unwrap();
    assert_eq!((diffb, rec_diff), (0, 0), "see WARP53_RESULT.tsv in the scratch folder (effects_vecdir::out_dir)");
}

/// true when the two results differ in any output bit class or record (used by the sensitivity count)
fn differs_bits(a: &Res, b: &Res) -> bool {
    match (a, b) {
        (Ok((o1, r1)), Ok((o2, r2))) => r1 != r2 || o1.iter().flatten().zip(o2.iter().flatten()).any(|(x, y)| !class_eq(x.to_bits(), y.to_bits())),
        (Err(e1), Err(e2)) => e1 != e2,
        _ => true,
    }
}
