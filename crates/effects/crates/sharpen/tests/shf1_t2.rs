//! T2: the sharpen crate against stored emulator runs of the original (harness v2, vex engine, "VEX follows the SDM, not silicon").
//! Layers: combine job 0x175e30 alone; wrapper 0x175c80 (blur dispatcher recorded, copy helpers 0x672c60 and 0x670340 real); slot function 0x530530
//! (flag 0; alias and not alias; wrapper real, blur recorded); slot function with the real blur dispatcher. Planes are compared bit for bit
//! (NaN as a class, counted); call records are compared field by field. Directory: env SHF1_T2 (default below).
use fx_common::oracle_io::{parse_json, Json};
use fx_common::plane::{Plane, PlaneMut};
use sharpen::combine::combine;
use sharpen::plan::*;
use sharpen::{render_with, sharpen_plane_with, Params};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn base() -> PathBuf {
    PathBuf::from(std::env::var("SHF1_T2").unwrap_or_else(|_| effects_vecdir::path_str("wave_s9/shf1_sharpen_floor/t2").into()))
}
fn rd(p: &Path) -> Vec<f32> {
    let b = std::fs::read(p).unwrap_or_else(|e| panic!("{}: {}", p.display(), e));
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}
fn hexu(j: &Json) -> u64 {
    u64::from_str_radix(j.as_str().unwrap().trim_start_matches("0x"), 16).unwrap()
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
fn pi(r: &Row, k: &str) -> i64 {
    r[k].parse().unwrap()
}
fn manifest(d: &Path) -> Json {
    let text = std::fs::read_to_string(d.join("manifest.json")).unwrap().replace("-Infinity", "null").replace("Infinity", "null").replace("NaN", "null");
    parse_json(&text).unwrap()
}
fn sets(layout: &str) -> Vec<PathBuf> {
    let root = base().join("out").join(layout);
    assert!(root.exists(), "run tools/run_t2.sh first: {}", root.display());
    let mut v: Vec<PathBuf> = std::fs::read_dir(&root).unwrap().map(|e| e.unwrap().path()).filter(|p| p.join("manifest.json").exists()).collect();
    v.sort();
    v
}
fn plane(d: &Path, c: &Json, io: &str, n: &str) -> Vec<f32> {
    rd(&d.join(c.get(io).unwrap().get(n).unwrap().get("file").unwrap().as_str().unwrap()))
}
fn records(d: &Path, cname: &str) -> Vec<Json> {
    let p = d.join(format!("{}.record_args.json", cname));
    if !p.exists() {
        return vec![];
    }
    let text = std::fs::read_to_string(p).unwrap();
    parse_json(&text).unwrap().as_arr().unwrap().to_vec()
}

#[derive(Default, Debug)]
struct Tot {
    sets: usize,
    cases: usize,
    floats: usize,
    diff: usize,
    nan_class: usize,
    copies: usize,
    blurs: usize,
    fields: usize,
    first: Option<String>,
}
impl Tot {
    fn cmp(&mut self, got: &[f32], want: &[f32], ctx: &str) {
        assert_eq!(got.len(), want.len(), "{} length", ctx);
        for i in 0..got.len() {
            if got[i].to_bits() == want[i].to_bits() {
            } else if got[i].is_nan() && want[i].is_nan() {
                self.nan_class += 1;
            } else {
                self.diff += 1;
                if self.first.is_none() {
                    self.first = Some(format!("{} [{}] got {:#010x} want {:#010x}", ctx, i, got[i].to_bits(), want[i].to_bits()));
                }
            }
        }
        self.floats += got.len();
    }
    fn done(&self, name: &str) {
        println!("SHF1 {}: {:?}", name, self);
        assert_eq!(self.diff, 0, "first differing: {:?}", self.first);
    }
}

fn pd(p: &Path) -> (usize, usize) {
    (0, 0 * p.as_os_str().len())
}

// ------------------------------------------------------------------------------------------------------------------ combine
fn run_comb(layout: &str) -> Tot {
    let pr = tsv(layout);
    let mut t = Tot::default();
    let _ = pd(Path::new("."));
    for d in sets(layout) {
        let name = d.file_name().unwrap().to_str().unwrap().to_string();
        let p = &pr[&name];
        let g = pf(p, "gainF");
        t.sets += 1;
        for c in manifest(&d).get("cases").unwrap().as_arr().unwrap() {
            let cname = c.get("name").unwrap().as_str().unwrap();
            assert_eq!(c.get("status").unwrap().as_str().unwrap(), "ok", "{} {}", name, cname);
            let (x, b, want) = (plane(&d, c, "inputs", "src"), plane(&d, c, "inputs", "blur"), plane(&d, c, "outputs", "out"));
            let count = if p.contains_key("count") { pi(p, "count") as usize } else { want.len() };
            let mut got = vec![f32::from_bits(0x7fc0ffee); want.len()];
            combine(&x, &b, g, &mut got, count).unwrap();
            t.cmp(&got, &want, &format!("{}/{}", name, cname));
            t.cases += 1;
        }
    }
    t
}
#[test]
fn comb_full() {
    if !effects_vecdir::have_all(&["wave_s9/shf1_sharpen_floor/t2"]) {
        return;
    }
    let t = run_comb("comb");
    t.done("comb");
    assert!(t.cases >= 200);
}
#[test]
fn comb_count() {
    if !effects_vecdir::have_all(&["wave_s9/shf1_sharpen_floor/t2"]) {
        return;
    }
    run_comb("comb_cnt").done("comb_cnt");
}

// ------------------------------------------------------------------------------------------------------------------ wrapper
fn check_blur_rec(e: &Json, w: usize, h: usize, rr: f32, gain: f32, aflag: i64, aspect: f32, dst: &str, inp: &str, ctxs: &str, t: &mut Tot) {
    let regs = e.get("regs").unwrap();
    let pt = e.get("points_to").unwrap();
    assert_eq!(pt.get("rsi").unwrap().as_str(), Some(dst), "dst");
    assert_eq!(pt.get("rdx").unwrap().as_str(), Some(inp), "in");
    assert_eq!(pt.get("rdi").unwrap().as_str(), Some(ctxs), "ctx");
    assert_eq!(hexu(regs.get("rcx").unwrap()), BLUR_EDGE_MIRROR as u64, "edge");
    assert_eq!(hexu(regs.get("r8").unwrap()), BLUR_PASSES as u64, "passes");
    assert_eq!(hexu(regs.get("r9").unwrap()) & 0xffffffff, BLUR_SUBPIXEL as u64, "subpixel");
    let xb = |i: usize| hexu(e.get("xmm").unwrap().get(&format!("xmm{}", i)).unwrap().get("f32_bits").unwrap()) as u32;
    assert_eq!(xb(0), rr.to_bits(), "sigma x");
    assert_eq!(xb(1), rr.to_bits(), "sigma y");
    assert_eq!(xb(2), BLUR_GAIN.to_bits(), "blur gain");
    assert_eq!(xb(3), gain.to_bits(), "sharpen gain kept in xmm3");
    let roi = [(w as u64) | ((h as u64) << 32), w as u64, 4u64];
    let st = e.get("stack").unwrap().as_arr().unwrap();
    assert_eq!(hexu(&st[0]) & 0xffffffff, BLUR_HALF as u64, "half");
    for k in 0..3 {
        assert_eq!(hexu(&st[1 + k]), roi[k], "roi word {}", k);
    }
    assert_eq!(hexu(&st[4]) & 0xffffffff, 0, "flag");
    assert_eq!(hexu(&st[5]), 0x10000000 + 0x1534d0, "pass X");
    assert_eq!(hexu(&st[6]), 0x10000000 + 0x153620, "pass Y");
    assert_eq!(hexu(&st[7]), 0, "ctx2");
    let hex = e.get("ptrs").unwrap().as_arr().unwrap()[0].get("hex").unwrap().as_str().unwrap().to_string();
    let byte = |o: usize| u8::from_str_radix(&hex[2 * o..2 * o + 2], 16).unwrap();
    let u32at = |o: usize| u32::from_le_bytes([byte(o), byte(o + 1), byte(o + 2), byte(o + 3)]);
    assert_eq!(u32at(0x30) as i32 as i64, aflag, "aspect flag");
    assert_eq!(u32at(0x38), aspect.to_bits(), "aspect");
    t.blurs += 1;
    t.fields += 17;
}
fn is_copy_entry(e: &Json) -> bool {
    e.get("hook").unwrap().as_str() == Some("REAL_CODE_ENTRY") && e.get("fn").unwrap().as_str() == Some("0x672c60")
}
fn is_blur(e: &Json) -> bool {
    e.get("hook").unwrap().as_str() == Some("blur")
}
/// bytes moved by the libc memcpy model during the copy: merged chunk calls start at plane offset 0 of the destination and cover n bytes
fn check_copy_bytes(calls: &[Json], dst_prefix: &str, want_bytes: usize, t: &mut Tot) {
    let mc: Vec<&Json> = calls.iter().filter(|e| e.get("hook").unwrap().as_str() == Some("LIBC-MODEL memcpy")).collect();
    let tot: usize = mc.iter().map(|e| e.get("n").unwrap().as_usize().unwrap()).sum();
    assert_eq!(tot, want_bytes, "memcpy bytes");
    assert!(mc[0].get("dst").unwrap().as_str().unwrap().starts_with(dst_prefix), "memcpy dst");
    t.fields += 2;
}

fn run_wrap(layout: &str) -> Tot {
    let pr = tsv(layout);
    let mut t = Tot::default();
    for d in sets(layout) {
        let name = d.file_name().unwrap().to_str().unwrap().to_string();
        let p = &pr[&name];
        let (radius, gain, aflag, aspect) = (pf(p, "radius"), pf(p, "gain"), pi(p, "aflag"), pf(p, "aspect"));
        t.sets += 1;
        for c in manifest(&d).get("cases").unwrap().as_arr().unwrap() {
            let cname = c.get("name").unwrap().as_str().unwrap();
            assert_eq!(c.get("status").unwrap().as_str().unwrap(), "ok", "{} {}", name, cname);
            let (x, b, out) = (plane(&d, c, "inputs", "src"), plane(&d, c, "inputs", "out"), plane(&d, c, "outputs", "out"));
            let (w, h) = (c.get("width").unwrap().as_usize().unwrap(), c.get("height").unwrap().as_usize().unwrap());
            let npix = w * h;
            let cnt = pi(p, "count");
            let count = if cnt < 0 { npix } else { cnt as usize };
            let calls = records(&d, cname);
            let want = sharpen_plane_with(&x, w, h, radius, gain, count, |_, _| Ok(b.clone())).unwrap();
            let ctx = format!("{}/{}", name, cname);
            t.cmp(&want, &out, &ctx);
            let blur: Vec<&Json> = calls.iter().filter(|e| is_blur(e)).collect();
            let copy: Vec<&Json> = calls.iter().filter(|e| is_copy_entry(e)).collect();
            match plane_plan(radius, gain) {
                PlanePlan::Copy => {
                    assert_eq!((blur.len(), copy.len()), (0, 1), "{}", ctx);
                    let e = copy[0];
                    assert!(e.get("rdi").unwrap().as_str().unwrap().starts_with("plane:out+0x0"), "copy dst");
                    assert_eq!(e.get("rdx").unwrap().as_usize().unwrap() & 0xffffffff, 0, "copy flag");
                    check_copy_bytes(&calls, "plane:out+0x0", npix * 4, &mut t);
                    t.copies += 1;
                    t.fields += 2;
                }
                PlanePlan::Run { radius: rr, gain: g } => {
                    assert_eq!((blur.len(), copy.len()), (1, 0), "{}", ctx);
                    check_blur_rec(blur[0], w, h, rr, g, aflag, aspect, "plane:out", "plane:src", "region:ctx", &mut t);
                }
            }
            t.cases += 1;
        }
    }
    t
}
#[test]
fn wrap_full() {
    if !effects_vecdir::have_all(&["wave_s9/shf1_sharpen_floor/t2"]) {
        return;
    }
    let t = run_wrap("wrap");
    t.done("wrap");
    assert!(t.blurs > 0 && t.copies > 0);
}
#[test]
fn wrap_count() {
    if !effects_vecdir::have_all(&["wave_s9/shf1_sharpen_floor/t2"]) {
        return;
    }
    run_wrap("wrap_cnt").done("wrap_cnt");
}
#[test]
fn wrap_big() {
    if !effects_vecdir::have_all(&["wave_s9/shf1_sharpen_floor/t2"]) {
        return;
    }
    let t = run_wrap("wrap_big");
    t.done("wrap_big");
    assert!(t.copies > 0);
}

// ------------------------------------------------------------------------------------------------------------------ slot function
struct DrvP {
    alias: bool,
    n: i64,
    in_count: usize,
    amount: f32,
    cw: i32,
    gains: Vec<f32>,
    aflag: i64,
    aspect: f32,
}
fn drv_params(p: &Row) -> DrvP {
    DrvP {
        alias: pi(p, "alias") != 0,
        n: pi(p, "n"),
        in_count: pi(p, "in_count") as usize,
        amount: pf(p, "amount"),
        cw: pi(p, "cw") as i32,
        gains: (0..5).map(|k| pf(p, &format!("g{}", k))).collect(),
        aflag: pi(p, "aflag"),
        aspect: pf(p, "aspect"),
    }
}
fn mk_params(q: &DrvP, npix: usize) -> Params {
    Params { amount: q.amount, gains: q.gains.clone(), in_count: q.in_count, ctx_width: q.cw, aspect: if q.aflag != 0 { Some(q.aspect) } else { None }, count: Some(npix) }
}

fn run_drv(layout: &str) -> Tot {
    let pr = tsv(layout);
    let mut t = Tot::default();
    for d in sets(layout) {
        let name = d.file_name().unwrap().to_str().unwrap().to_string();
        let q = drv_params(&pr[&name]);
        t.sets += 1;
        for c in manifest(&d).get("cases").unwrap().as_arr().unwrap() {
            let cname = c.get("name").unwrap().as_str().unwrap();
            assert_eq!(c.get("status").unwrap().as_str().unwrap(), "ok", "{} {}", name, cname);
            let (w, h) = (c.get("width").unwrap().as_usize().unwrap(), c.get("height").unwrap().as_usize().unwrap());
            let npix = w * h;
            let calls = records(&d, cname);
            let ctx = format!("{}/{}", name, cname);
            let nn = if q.n > 0 { (q.n as usize).min(4) } else { 0 };
            // inputs and blurred-plane stand-ins
            let (ins, bs, outs_init): (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<Vec<f32>>) = if q.alias {
                let p: Vec<Vec<f32>> = (0..4).map(|k| plane(&d, c, "inputs", &format!("p{}", k))).collect();
                let scr = plane(&d, c, "inputs", "scr");
                (p.clone(), (0..4).map(|_| scr.clone()).collect(), p)
            } else {
                ((0..4).map(|k| plane(&d, c, "inputs", &format!("i{}", k))).collect(), (0..4).map(|k| plane(&d, c, "inputs", &format!("o{}", k))).collect(),
                 (0..4).map(|k| plane(&d, c, "inputs", &format!("o{}", k))).collect())
            };
            let r = sharpen::plan::radius(q.amount, q.cw);
            let mut order: Vec<bool> = vec![]; // true = blur call, false = copy entry
            for ch in 0..nn {
                let g = q.gains[sharpen::plan::gain_index(ch, q.in_count)];
                order.push(matches!(plane_plan(r, g), PlanePlan::Run { .. }));
            }
            let mut run_ch: Vec<usize> = (0..nn).filter(|&ch| order[ch]).collect();
            run_ch.reverse();
            // model: the slot function through the crate's render_with, blur supplied by the recorded plane
            let params = mk_params(&q, npix);
            let srcp: Vec<Plane> = (0..nn).map(|k| Plane::new(&ins[k], w, h, w).unwrap()).collect();
            let mut obuf: Vec<Vec<f32>> = (0..nn).map(|_| vec![f32::from_bits(0x7fc0ffee); npix]).collect();
            {
                let mut dstp: Vec<PlaneMut> = obuf.iter_mut().map(|v| PlaneMut::new(v, w, h, w).unwrap()).collect();
                let bsr = &bs;
                render_with(&params, &srcp, &mut dstp, |_, _, _, _| {
                    let chan = run_ch.pop().expect("blur call without a Run plan");
                    Ok(bsr[chan].clone())
                })
                .unwrap();
            }
            // outputs
            for k in 0..4 {
                let nm = if q.alias { format!("p{}", k) } else { format!("o{}", k) };
                let got = plane(&d, c, "outputs", &nm);
                let want: Vec<f32> = if k < nn { obuf[k].clone() } else { outs_init[k].clone() };
                t.cmp(&want, &got, &format!("{} {}", ctx, nm));
            }
            if q.alias {
                let scr = plane(&d, c, "outputs", "scr");
                t.cmp(&bs[0], &scr, &format!("{} scr untouched", ctx));
            }
            // call record: sequence of blur calls and copy entries, host scratch call
            let seq: Vec<&Json> = calls.iter().filter(|e| is_blur(e) || is_copy_entry(e)).collect();
            assert_eq!(seq.len(), nn, "{}: wrapper calls", ctx);
            let host = calls.iter().filter(|e| e.get("hook").unwrap().as_str() == Some("host_tmp")).count();
            assert_eq!(host, if q.alias { 1 } else { 0 }, "{}: host scratch call (made before the plane-count test)", ctx);
            for (ch, e) in seq.iter().enumerate() {
                let g = q.gains[sharpen::plan::gain_index(ch, q.in_count)];
                if order[ch] {
                    assert!(is_blur(e), "{} ch {}: expected blur", ctx, ch);
                    let (dst, inp) = if q.alias { ("plane:scr".to_string(), format!("plane:p{}", ch)) } else { (format!("plane:o{}", ch), format!("plane:i{}", ch)) };
                    check_blur_rec(e, w, h, r, g, q.aflag, q.aspect, &dst, &inp, "region:ctx", &mut t);
                } else {
                    assert!(is_copy_entry(e), "{} ch {}: expected copy", ctx, ch);
                    let o = if q.alias { format!("plane:p{}+0x0", ch) } else { format!("plane:o{}+0x0", ch) };
                    assert!(e.get("rdi").unwrap().as_str().unwrap().starts_with(&o), "{} copy dst", ctx);
                    t.copies += 1;
                    t.fields += 1;
                }
            }
            t.cases += 1;
        }
    }
    t
}
#[test]
fn drv_plain() {
    if !effects_vecdir::have_all(&["wave_s9/shf1_sharpen_floor/t2"]) {
        return;
    }
    let t = run_drv("drv");
    t.done("drv");
    assert!(t.blurs > 0 && t.copies > 0);
}
#[test]
fn drv_alias() {
    if !effects_vecdir::have_all(&["wave_s9/shf1_sharpen_floor/t2"]) {
        return;
    }
    let t = run_drv("drv_alias");
    t.done("drv_alias");
}

// ------------------------------------------------------------------------------------------------------------------ slot function, real blur
#[cfg(feature = "blur")]
fn run_e2e(layout: &str) -> Tot {
    let pr = tsv(layout);
    let mut t = Tot::default();
    for d in sets(layout) {
        let name = d.file_name().unwrap().to_str().unwrap().to_string();
        let q = drv_params(&pr[&name]);
        t.sets += 1;
        for c in manifest(&d).get("cases").unwrap().as_arr().unwrap() {
            let cname = c.get("name").unwrap().as_str().unwrap();
            assert_eq!(c.get("status").unwrap().as_str().unwrap(), "ok", "{} {}", name, cname);
            let (w, h) = (c.get("width").unwrap().as_usize().unwrap(), c.get("height").unwrap().as_usize().unwrap());
            let npix = w * h;
            let nn = if q.n > 0 { (q.n as usize).min(4) } else { 0 };
            let (ins, outs_init): (Vec<Vec<f32>>, Vec<Vec<f32>>) = if q.alias {
                let p: Vec<Vec<f32>> = (0..4).map(|k| plane(&d, c, "inputs", &format!("p{}", k))).collect();
                (p.clone(), p)
            } else {
                ((0..4).map(|k| plane(&d, c, "inputs", &format!("i{}", k))).collect(), (0..4).map(|k| plane(&d, c, "inputs", &format!("o{}", k))).collect())
            };
            let params = mk_params(&q, npix);
            let aspect = params.aspect;
            let srcp: Vec<Plane> = (0..nn).map(|k| Plane::new(&ins[k], w, h, w).unwrap()).collect();
            let mut obuf: Vec<Vec<f32>> = (0..nn).map(|_| vec![f32::from_bits(0x7fc0ffee); npix]).collect();
            {
                let mut dstp: Vec<PlaneMut> = obuf.iter_mut().map(|v| PlaneMut::new(v, w, h, w).unwrap()).collect();
                render_with(&params, &srcp, &mut dstp, |xs, ww, hh, rr| {
                    let mut b = vec![0.0f32; xs.len()];
                    blur::blur_core_phase(xs, &mut b, ww, hh, rr, rr, BLUR_GAIN, blur::Edge::Mirror, BLUR_PASSES, false, aspect, sharpen::BASE_PHASE)?;
                    Ok(b)
                })
                .unwrap();
            }
            for k in 0..4 {
                let nm = if q.alias { format!("p{}", k) } else { format!("o{}", k) };
                let got = plane(&d, c, "outputs", &nm);
                let want: Vec<f32> = if k < nn { obuf[k].clone() } else { outs_init[k].clone() };
                t.cmp(&want, &got, &format!("{}/{} {}", name, cname, nm));
            }
            t.cases += 1;
        }
    }
    t
}
#[cfg(feature = "blur")]
#[test]
fn e2e_plain() {
    if !effects_vecdir::have_all(&["wave_s9/shf1_sharpen_floor/t2"]) {
        return;
    }
    run_e2e("drv_e2e0").done("drv_e2e0");
}
#[cfg(feature = "blur")]
#[test]
fn e2e_alias() {
    if !effects_vecdir::have_all(&["wave_s9/shf1_sharpen_floor/t2"]) {
        return;
    }
    run_e2e("drv_e2e1").done("drv_e2e1");
}
