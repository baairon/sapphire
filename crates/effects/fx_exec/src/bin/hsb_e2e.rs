//! hsb_e2e (CV3): the whole Hue_Sat_Bright effect on HOST MEMORY, native (convert pair wired) against the ORIGINAL run through the E1 route.
//! usage: hsb_e2e <programs.tsv> <host.src> <DB.bin> <W> <H> "<name=v1,v2;name=v>" <mode 0..3> [--pln PLN.bin] [--prefill-aa] [--tsv]
//! host.src = H*W*4 f32 (B,G,R,A per pixel), DB.bin = the original's host destination bytes after the run, PLN.bin = the original's planes at the writer
//! entry (layout H,4,W). The native side: native convert-from-source per row, the native kernels (NV-5/NV-6 route), native convert-to-dest per row.
use convert_pair::AlphaMode;
use fx_eval::{build_effect, Corpus};
use fx_exec::*;
use std::collections::BTreeMap;

fn rd(path: &str) -> Vec<f32> {
    let b = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {}: {}", path, e));
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 8 {
        eprintln!("usage: hsb_e2e <programs.tsv> <host.src> <DB.bin> <W> <H> \"<sets>\" <mode> [--pln PLN.bin] [--prefill-aa]");
        std::process::exit(2);
    }
    let h = std::thread::Builder::new().stack_size(256 << 20).spawn(move || run(a)).unwrap();
    std::process::exit(h.join().unwrap());
}

struct Tally {
    total: usize,
    eq: usize,
    nan_payload: usize,
    diff: usize,
    nan_vs_num: usize,
    first: Vec<String>,
    per_ch: [usize; 4],
}

fn tally(native: &[f32], orig: &[f32], w: usize, src: &[f32], label: &str, ch_of: impl Fn(usize) -> usize) -> Tally {
    let mut t = Tally { total: 0, eq: 0, nan_payload: 0, diff: 0, nan_vs_num: 0, first: vec![], per_ch: [0; 4] };
    for (i, (x, y)) in native.iter().zip(orig.iter()).enumerate() {
        t.total += 1;
        if x.to_bits() == y.to_bits() {
            t.eq += 1;
        } else if x.is_nan() && y.is_nan() {
            t.nan_payload += 1;
        } else {
            t.diff += 1;
            if x.is_nan() != y.is_nan() {
                t.nan_vs_num += 1;
            }
            t.per_ch[ch_of(i) & 3] += 1;
            if t.first.len() < 8 {
                let p = i / 4;
                t.first.push(format!(
                    "{} idx{} (x{} y{} c{}) native {:08x} ({:e}) original {:08x} ({:e}) src=[{:08x} {:08x} {:08x} {:08x}] ({:e} {:e} {:e} {:e})",
                    label, i, p % w, p / w, ch_of(i), x.to_bits(), x, y.to_bits(), y, src[4 * p].to_bits(), src[4 * p + 1].to_bits(), src[4 * p + 2].to_bits(), src[4 * p + 3].to_bits(),
                    src[4 * p], src[4 * p + 1], src[4 * p + 2], src[4 * p + 3]
                ));
            }
        }
    }
    t
}

fn run(a: Vec<String>) -> i32 {
    let t = std::fs::read_to_string(&a[1]).unwrap();
    let c = Corpus::from_programs_tsv(&t);
    let g = build_effect(&c, c.find_effect("Hue_Sat_Bright", None).unwrap()).unwrap();
    let src = rd(&a[2]);
    let orig = rd(&a[3]);
    let (w, hh): (usize, usize) = (a[4].parse().unwrap(), a[5].parse().unwrap());
    let n = w * hh;
    assert_eq!(src.len(), 4 * n);
    assert_eq!(orig.len(), 4 * n);
    let mode_byte: i8 = a[7].parse().unwrap();
    let mode = AlphaMode::from_byte(mode_byte).expect("alpha mode");
    // INT8: strict, or the named option --state-a-defaults
    let mut b = match fx_exec::exec::strict_binding("hsb_e2e", &a) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    let mut sets_txt = String::new();
    for kv in a[6].split(';').filter(|s| !s.is_empty()) {
        let (k, v) = kv.split_once('=').unwrap();
        let lanes: Vec<f32> = v.split(',').map(|x| x.trim().parse::<f64>().unwrap() as f32).collect();
        b.overrides.insert(k.to_string(), lanes);
        sets_txt.push_str(kv);
        sets_txt.push(' ');
    }
    b.const_inputs.insert("Mask".into(), vec![1.0]);
    if let Ok(v) = std::env::var("CV_MUTANT") {
        convert_pair::set_mutant(v.parse().unwrap());
    }
    let mut pln: Option<Vec<f32>> = None;
    let mut plan_json: Option<String> = None;
    let mut aa = false;
    let mut i = 8;
    while i < a.len() {
        match a[i].as_str() {
            "--pln" => {
                pln = Some(rd(&a[i + 1]));
                i += 1
            }
            "--prefill-aa" => aa = true,
            "--plan" => {
                plan_json = Some(a[i + 1].clone());
                i += 1
            }
            _ => {}
        }
        i += 1;
    }
    let prefill = vec![f32::from_bits(0xaaaa_aaaa); 4 * n];
    let _ = aa;
    let hc = HostConvert { src: vec![src.clone()], mode, dest_prefill: prefill, cell: 1, margin: 0, ae32: None };
    let hf = HostFrames::default();
    let o = match execute_host(&g, &b, &hc, "Source", &BTreeMap::new(), w, hh, &hf) {
        Ok(o) => o,
        Err(e) => {
            println!("{}\t{}\tmode{}\tEXEC-ERROR {}", a[2], sets_txt, mode_byte, e);
            return 1;
        }
    };
    let ta = tally(&o.host_dest, &orig, w, &src, "host", |i| i % 4);
    let verdict = if ta.diff == 0 { if ta.nan_payload == 0 { "BIT-EQUAL" } else { "CLASS-EQUAL(nan payload only)" } } else { "DIFF" };
    let mut plane_txt = String::from("planes=n/a");
    let mut plane_first: Vec<String> = vec![];
    if let Some(p) = &pln {
        // the original's planes at the writer entry: layout (H, 4, W); the native root planes
        let mut nat = Vec::with_capacity(4 * n);
        let mut org = Vec::with_capacity(4 * n);
        for y in 0..hh {
            for k in 0..4 {
                for x in 0..w {
                    nat.push(o.exec.dest.planes[k][y * w + x]);
                    org.push(p[(y * 4 + k) * w + x]);
                }
            }
        }
        let tp = tally(&nat, &org, w, &src, "plane", |i| (i / w) % 4);
        plane_txt = format!("planes: total={} eq={} nan_payload={} diff={} nan_vs_num={} per_ch={:?}", tp.total, tp.eq, tp.nan_payload, tp.diff, tp.nan_vs_num, tp.per_ch);
        plane_first = tp.first;
    }
    println!(
        "{}\t{}\tmode{}\thost: total={} eq={} nan_payload={} diff={} nan_vs_num={} per_ch(B,G,R,A)={:?}\t{}\t{}\twrote={}",
        a[2], sets_txt, mode_byte, ta.total, ta.eq, ta.nan_payload, ta.diff, ta.nan_vs_num, ta.per_ch, plane_txt, verdict, o.wrote
    );
    for d in ta.first.iter().chain(plane_first.iter()) {
        println!("  DIFF {}", d);
    }
    if let Some(p) = plan_json {
        // the ORIGINAL plan text (printed constants) of the same E1 run through the command-level path with the convert pair wired
        let txt = std::fs::read_to_string(&p).unwrap();
        let cmds: Vec<String> = cmds::commands_of_dump(&txt).into_iter().filter(|c| c.starts_with("(")).collect();
        match cmds::execute_commands_hc(&cmds, &hc, w, hh, &hf, &b.kp) {
            Ok(co) => {
                let hd = co.host_dest.unwrap_or_default();
                let tc = tally(&hd, &orig, w, &src, "cmdpath", |i| i % 4);
                println!("  CMDPATH cmds={} total={} eq={} nan_payload={} diff={} nan_vs_num={}", cmds.len(), tc.total, tc.eq, tc.nan_payload, tc.diff, tc.nan_vs_num);
                for d in tc.first.iter().take(3) {
                    println!("  DIFF {}", d);
                }
            }
            Err(e) => println!("  CMDPATH error {}", e),
        }
    }
    0
}
