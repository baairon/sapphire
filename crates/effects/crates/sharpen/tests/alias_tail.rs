//! SHF1 T2: the sharpen crate against stored emulator runs of the original (harness v2, vex engine, "VEX follows the SDM, not silicon").
//! Layers: combine job 0x175e30 alone; wrapper 0x175c80 (blur dispatcher recorded, copy helpers 0x672c60 and 0x670340 real); slot function 0x530530
//! (flag 0; alias and not alias; wrapper real, blur recorded); slot function with the real blur dispatcher. Planes are compared bit for bit
//! (NaN as a class, counted); call records are compared field by field. Directory: env SHE2_T2 (default below).
use fx_common::oracle_io::{parse_json, Json};
use fx_common::plane::{Plane, PlaneMut};
use sharpen::combine::combine;
use sharpen::plan::*;
use sharpen::{render_with_alias, Params};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn base() -> PathBuf {
    PathBuf::from(std::env::var("SHE2_T2").unwrap_or_else(|_| effects_vecdir::path_str("wave_s9/she2_convert3ch/sharpen/t2").into()))
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


// ---------------------------------------------------------------------------------------------------------------- SHE2 alias tail
struct DrvP {
    alias: bool,
    n: i64,
    in_count: usize,
    amount: f32,
    cw: i32,
    gains: Vec<f32>,
    aflag: i64,
    aspect: f32,
    count: usize,
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
        count: pi(p, "count") as usize,
    }
}

/// Counts the cases whose count is below the plane size and the cells whose tail (index >= count) was compared.
fn run_tail(layout: &str) -> (Tot, usize, usize) {
    let pr = tsv(layout);
    let mut t = Tot::default();
    let (mut below, mut tail_cells) = (0usize, 0usize);
    for d in sets(layout) {
        let name = d.file_name().unwrap().to_str().unwrap().to_string();
        let q = drv_params(&pr[&name]);
        t.sets += 1;
        for c in manifest(&d).get("cases").unwrap().as_arr().unwrap() {
            let cname = c.get("name").unwrap().as_str().unwrap();
            assert_eq!(c.get("status").unwrap().as_str().unwrap(), "ok", "{} {}", name, cname);
            let (w, h) = (c.get("width").unwrap().as_usize().unwrap(), c.get("height").unwrap().as_usize().unwrap());
            let npix = w * h;
            assert!(h >= 2 && q.count <= npix);
            let nn = if q.n > 0 { (q.n as usize).min(4) } else { 0 };
            let (ins, bs, outs_init): (Vec<Vec<f32>>, Vec<Vec<f32>>, Vec<Vec<f32>>) = if q.alias {
                let p: Vec<Vec<f32>> = (0..4).map(|k| plane(&d, c, "inputs", &format!("p{}", k))).collect();
                let scr = plane(&d, c, "inputs", "scr");
                (p.clone(), (0..4).map(|_| scr.clone()).collect(), p)
            } else {
                ((0..4).map(|k| plane(&d, c, "inputs", &format!("i{}", k))).collect(), (0..4).map(|k| plane(&d, c, "inputs", &format!("o{}", k))).collect(),
                 (0..4).map(|k| plane(&d, c, "inputs", &format!("o{}", k))).collect())
            };
            let r = sharpen::plan::radius(q.amount, q.cw);
            let mut run_ch: Vec<usize> = (0..nn)
                .filter(|&ch| matches!(plane_plan(r, q.gains[sharpen::plan::gain_index(ch, q.in_count)]), PlanePlan::Run { .. }))
                .collect();
            run_ch.reverse();
            let params = Params { amount: q.amount, gains: q.gains.clone(), in_count: q.in_count, ctx_width: q.cw, aspect: if q.aflag != 0 { Some(q.aspect) } else { None }, count: Some(q.count) };
            let srcp: Vec<Plane> = (0..nn).map(|k| Plane::new(&ins[k], w, h, w).unwrap()).collect();
            let mut obuf: Vec<Vec<f32>> = (0..nn).map(|_| vec![f32::from_bits(0x7fc0ffee); npix]).collect();
            {
                let mut dstp: Vec<PlaneMut> = obuf.iter_mut().map(|v| PlaneMut::new(v, w, h, w).unwrap()).collect();
                let bsr = &bs;
                render_with_alias(&params, &srcp, &mut dstp, q.alias, |_, _, _, _| {
                    let chan = run_ch.pop().expect("blur call without a Run plan");
                    Ok(bsr[chan].clone())
                })
                .unwrap();
            }
            for k in 0..4 {
                let nm = if q.alias { format!("p{}", k) } else { format!("o{}", k) };
                let got = plane(&d, c, "outputs", &nm);
                let want: Vec<f32> = if k < nn { obuf[k].clone() } else { outs_init[k].clone() };
                t.cmp(&want, &got, &format!("{}/{} {}", name, cname, nm));
                if k < nn {
                    tail_cells += npix - q.count;
                }
            }
            if q.count < npix {
                below += 1;
            }
            t.cases += 1;
        }
    }
    (t, below, tail_cells)
}

fn go(layout: &str) {
    let (t, below, tail) = run_tail(layout);
    println!("SHE2 {}: {:?} cases below w*h {} tail cells compared {}", layout, t, below, tail);
    assert_eq!(t.diff, 0, "first differing: {:?}", t.first);
    assert!(below > 0 && tail > 0);
}
#[test]
fn alias_cnt32() {
    if !effects_vecdir::have_all(&["wave_s9/she2_convert3ch/sharpen/t2"]) {
        return;
    }
    go("drv_alias_cnt32");
}
#[test]
fn alias_cnt15() {
    if !effects_vecdir::have_all(&["wave_s9/she2_convert3ch/sharpen/t2"]) {
        return;
    }
    go("drv_alias_cnt15");
}
#[test]
fn plain_cnt32() {
    if !effects_vecdir::have_all(&["wave_s9/she2_convert3ch/sharpen/t2"]) {
        return;
    }
    go("drv_plain_cnt32");
}
#[test]
fn plain_cnt15() {
    if !effects_vecdir::have_all(&["wave_s9/she2_convert3ch/sharpen/t2"]) {
        return;
    }
    go("drv_plain_cnt15");
}

// ---------------------------------------------------------------------------------------------------------------- pure model facts (no emulator)
#[test]
fn model_alias_tail_is_the_input_and_plain_tail_is_the_blur() {
    use sharpen::sharpen_plane_alias;
    let (w, h) = (4usize, 3usize);
    let x: Vec<f32> = (0..12).map(|i| i as f32 * 0.5 + 1.0).collect();
    let b: Vec<f32> = (0..12).map(|i| 100.0 + i as f32).collect();
    for count in [0usize, 1, 5, 11, 12] {
        let a = sharpen_plane_alias(&x, w, h, 2.0, 2.0, count, true, |_, _| Ok(b.clone())).unwrap();
        let p = sharpen_plane_alias(&x, w, h, 2.0, 2.0, count, false, |_, _| Ok(b.clone())).unwrap();
        for i in count..12 {
            assert_eq!(a[i].to_bits(), x[i].to_bits(), "alias tail i={} count={}", i, count);
            assert_eq!(p[i].to_bits(), b[i].to_bits(), "plain tail i={} count={}", i, count);
        }
        for i in 0..count {
            assert_eq!(a[i].to_bits(), p[i].to_bits());
        }
    }
}
