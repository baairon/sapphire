//! trails_cmp (after hsb_cmp): run the native executor on the INPUT planes and the explicit PREV-DEST frame stored in an
//! ORIG_PLANES file (layout IN(4) PREV_DEST_FRAME(4) OUT(4), f32 little endian, planes of H*W, stride W) and bit-compare with the stored ORIGINAL output planes.
//! usage: trails_cmp <programs.tsv> <file.bin> <W> <H> "<name=v1,v2;name=v>" [--plan RESULT.json] [--dump out.bin] [--consts] [--noprev] [--prog Name]
//! --zerosrc binds an all-zero Source (diagnostic for the stand-in wiring artefact of combine 0 and 6).
//! --noprev hands no prev_dest to the native side (negative control: get_prev_dest then stands in with its argument).
use fx_eval::{build_effect, Corpus};
use fx_exec::*;
use std::collections::BTreeMap;

fn rd(path: &str) -> Vec<f32> {
    let b = std::fs::read(path).unwrap_or_else(|e| panic!("cannot read {}: {}", path, e));
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

fn ord(b: u32) -> i64 {
    if b & 0x8000_0000 != 0 {
        -((b & 0x7fff_ffff) as i64)
    } else {
        b as i64
    }
}

struct Cmp {
    samples: usize,
    bit_equal: usize,
    nan_payload: usize,
    different: usize,
    nan_vs_num: usize,
    max_ulp: i64,
    first: Option<String>,
    per_ch: Vec<usize>,
    list: Vec<String>,
}

fn compare(native: &Image, orig: &[f32], n: usize, w: usize, inp: &[f32], prev: &[f32]) -> Cmp {
    let mut c = Cmp { samples: 0, bit_equal: 0, nan_payload: 0, different: 0, nan_vs_num: 0, max_ulp: 0, first: None, per_ch: vec![0; native.channels()], list: vec![] };
    for ch in 0..native.channels() {
        for i in 0..n {
            let a = native.planes[ch][i];
            let b = orig[ch * n + i];
            c.samples += 1;
            if a.to_bits() == b.to_bits() {
                c.bit_equal += 1;
            } else if a.is_nan() && b.is_nan() {
                c.nan_payload += 1;
            } else {
                c.different += 1;
                c.per_ch[ch] += 1;
                if a.is_nan() != b.is_nan() {
                    c.nan_vs_num += 1;
                } else if a.is_finite() && b.is_finite() || (a.is_infinite() && b.is_infinite()) {
                    c.max_ulp = c.max_ulp.max((ord(a.to_bits()) - ord(b.to_bits())).abs());
                }
                let d = format!(
                    "c{} i{} (x{} y{}) native {:08x} original {:08x} src=[{:08x} {:08x} {:08x} {:08x}] prev=[{:08x} {:08x} {:08x} {:08x}] (src {:e} prev {:e} -> native {:e} orig {:e})",
                    ch, i, i % w, i / w, a.to_bits(), b.to_bits(),
                    inp[i].to_bits(), inp[n + i].to_bits(), inp[2 * n + i].to_bits(), inp[3 * n + i].to_bits(),
                    prev[i].to_bits(), prev[n + i].to_bits(), prev[2 * n + i].to_bits(), prev[3 * n + i].to_bits(),
                    inp[ch * n + i], prev[ch * n + i], a, b
                );
                if c.first.is_none() {
                    c.first = Some(d.clone());
                }
                if c.list.len() < 6 {
                    c.list.push(d);
                }
            }
        }
    }
    c
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    if a.len() < 6 {
        eprintln!("usage: trails_cmp <programs.tsv> <file.bin> <W> <H> \"<sets>\" [--plan RESULT.json] [--dump out.bin] [--consts] [--noprev] [--prog Name]");
        std::process::exit(2);
    }
    let h = std::thread::Builder::new().stack_size(256 << 20).spawn(move || run(a)).unwrap();
    std::process::exit(h.join().unwrap());
}

fn run(a: Vec<String>) -> i32 {
    let mut prog = "Trails".to_string();
    let mut plan: Option<String> = None;
    let mut dump: Option<String> = None;
    let mut consts = false;
    let mut noprev = false;
    let mut zerosrc = false;
    let mut blur_edge: Option<i32> = None;
    let mut blur_scale: Option<i32> = None;
    let mut i = 6;
    while i < a.len() {
        match a[i].as_str() {
            "--plan" => {
                plan = Some(a[i + 1].clone());
                i += 1
            }
            "--dump" => {
                dump = Some(a[i + 1].clone());
                i += 1
            }
            "--prog" => {
                prog = a[i + 1].clone();
                i += 1
            }
            "--consts" => consts = true,
            "--noprev" => noprev = true,
            "--zerosrc" => zerosrc = true,
            "--blur-edge" => {
                blur_edge = Some(a[i + 1].parse().unwrap());
                i += 1
            }
            "--blur-scale" => {
                blur_scale = Some(a[i + 1].parse().unwrap());
                i += 1
            }
            _ => {}
        }
        i += 1;
    }
    let t = std::fs::read_to_string(&a[1]).unwrap();
    let c = Corpus::from_programs_tsv(&t);
    let g = build_effect(&c, c.find_effect(&prog, None).unwrap()).unwrap();
    let file = &a[2];
    let (w, hh): (usize, usize) = (a[3].parse().unwrap(), a[4].parse().unwrap());
    let n = w * hh;
    let all = rd(file);
    assert_eq!(all.len(), 12 * n, "file size does not match 12 planes of {}x{}", w, hh);
    let inp = &all[..4 * n];
    let prev = &all[4 * n..8 * n];
    let orig = &all[8 * n..];
    // strict, or the named option --state-a-defaults
    let mut b = match fx_exec::exec::strict_binding("trails_cmp", &a) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("{}", e);
            return 2;
        }
    };
    let mut sets_txt = String::new();
    for kv in a[5].split(';').filter(|s| !s.is_empty()) {
        let (k, v) = kv.split_once('=').unwrap();
        // same conversion as the original harness: decimal text -> double -> f32
        let lanes: Vec<f32> = v.split(',').map(|x| x.trim().parse::<f64>().unwrap() as f32).collect();
        b.overrides.insert(k.to_string(), lanes);
        sets_txt.push_str(kv);
        sets_txt.push(' ');
    }
    b.const_inputs.insert("Mask".into(), vec![1.0]);
    if let Some(e) = blur_edge {
        b.kp.blur_edge = e;
    }
    // the executor default is the frame width; --blur-scale overrides (1 = the old NV9-A6 value)
    if let Some(s) = blur_scale {
        b.kp.blur_render_scale = s;
    }
    let planes = |s: &[f32]| Image::from_planes((0..4).map(|ch| s[ch * n..(ch + 1) * n].to_vec()).collect()).unwrap();
    let src = if zerosrc { Image::from_planes((0..4).map(|_| vec![0.0f32; n]).collect()).unwrap() } else { planes(inp) };
    let prev_img = planes(prev);
    let mut images = BTreeMap::new();
    images.insert("Source".to_string(), src.clone());
    let inputs = Inputs { width: w, height: hh, images };
    let hf = HostFrames { prev_dest: if noprev { None } else { Some(prev_img.clone()) }, frames: BTreeMap::new(), frame_round_old: false, ..HostFrames::default() };
    let o = match execute(&g, &b, &inputs, &hf) {
        Ok(o) => o,
        Err(e) => {
            println!("{}\t{}\tEXEC-ERROR {}", file, sets_txt, e);
            return 1;
        }
    };
    let classes: Vec<String> = o.trace.iter().map(|r| format!("{}:{:?}->{}", r.class, r.operands, r.tag)).collect();
    let cm = compare(&o.dest, orig, n, w, inp, prev);
    let verdict = if cm.different == 0 { if cm.nan_payload == 0 { "BIT-EQUAL" } else { "CLASS-EQUAL(nan payload only)" } } else { "DIFF" };
    // what save_prev_dest stored must equal the destination (the original saves a copy of the result)
    let saved_eq = o.saved_prev_dest.as_ref().map(|s| s.planes.iter().zip(o.dest.planes.iter()).all(|(p, q)| p.iter().zip(q.iter()).all(|(x, y)| x.to_bits() == y.to_bits()))).unwrap_or(false);
    println!(
        "{}\t{}\tdest_ch={}\truns=[{}]\tsamples={}\tbit_equal={}\tnan_payload_only={}\tdifferent={}\tnan_vs_number={}\tmax_ulp_finite={}\tper_channel_diff={:?}\tsaved_eq_dest={}\t{}",
        file, sets_txt, o.dest.channels(), classes.join(" "), cm.samples, cm.bit_equal, cm.nan_payload, cm.different, cm.nan_vs_num, cm.max_ulp, cm.per_ch, saved_eq, verdict
    );
    for d in &cm.list {
        println!("  DIFF {}", d);
    }
    if consts {
        let cp = compile_for(&g, &b).unwrap();
        for nd in &cp.nodes {
            if nd.b160 || nd.folded || nd.kind == "func" || nd.kind == "auto" {
                println!(
                    "  NODE {} {} {} {}={} folded={} lanes=[{:?}]",
                    nd.idx,
                    nd.kind,
                    nd.prim,
                    fx_eval::judge_keys::PLAN_KEY_NODE_FLAG,
                    nd.b160,
                    nd.folded,
                    nd.lanes.iter().map(|l| format!("{:e}", f32::from_bits(*l))).collect::<Vec<_>>()
                );
            }
        }
    }
    if let Some(p) = dump {
        let mut bytes = Vec::new();
        for pl in src.planes.iter().chain(prev_img.planes.iter()).chain(o.dest.planes.iter()) {
            for x in pl {
                bytes.extend_from_slice(&x.to_le_bytes());
            }
        }
        std::fs::write(p, bytes).unwrap();
    }
    if let Some(p) = plan {
        // diagnostic: the ORIGINAL plan text commands (6 digit constants) run through the command level path with the same host frame
        let txt = std::fs::read_to_string(&p).unwrap();
        let cmds: Vec<String> = txt
            .lines()
            .map(|l| l.trim())
            .filter(|l| l.starts_with("\"(") && !l.contains("\"plan\""))
            .map(|l| l.trim_end_matches(',').trim_matches('"').to_string())
            .collect();
        match cmds::execute_commands(&cmds, &[src.clone()], w, hh, &hf, &b.kp) {
            Ok(co) => {
                let d = co.dest.unwrap();
                let cc = compare(&d, orig, n, w, inp, prev);
                println!(
                    "  PLANCMDS cmds={} bit_equal={} nan_payload_only={} different={} max_ulp_finite={}",
                    cmds.len(), cc.bit_equal, cc.nan_payload, cc.different, cc.max_ulp
                );
                if let Some(f) = cc.first {
                    println!("  PLANCMDS first diff {}", f);
                }
            }
            Err(e) => println!("  PLANCMDS error {}", e),
        }
    }
    0
}
