//! T2: crate vs stored emulator runs of the shared blur core (entered at 0x255ff0, mode 4, flag 0). Reads t2/cases/<layout>.txt written by
//! tools/export_cases.py from the stored emulator records. Events (callee calls and their arguments, the radius arrays seen by the scratch service)
//! are compared in order; float fields compare bit exact, NaN as a class (payload-only differences are counted).
use esh5_core::*;
use std::collections::HashMap;

fn base() -> String {
    std::env::var("ESH5_T2").unwrap_or_else(|_| effects_vecdir::path_str("wave_s9/esh5_core/t2").into())
}

fn id(name: &str) -> u64 {
    match name {
        "pa" => 1,
        "pb" => 2,
        "pc" => 3,
        "pd" => 4,
        "pe" => 5,
        "pf" => 6,
        "pg" => 7,
        _ => panic!("ptr name {}", name),
    }
}
fn nm(v: u64) -> String {
    ["?", "pa", "pb", "pc", "pd", "pe", "pf", "pg"][v as usize].to_string()
}

struct Rec {
    ev: Vec<String>,
    powf_ret: u32,
}
fn join(v: &[f32]) -> String {
    if v.is_empty() {
        "-".into()
    } else {
        v.iter().map(|x| x.to_bits().to_string()).collect::<Vec<_>>().join(",")
    }
}
impl Host for Rec {
    fn powf(&mut self, x: f32, y: f32) -> f32 {
        self.ev.push(format!("powf x0={} x1={}", x.to_bits(), y.to_bits()));
        f32::from_bits(self.powf_ret)
    }
    fn scratch_init(&mut self, this: u64, bytes: i64, ecx: u32, r8d: u32, g0: u64, g1: u64, hx: &[f32], hy: &[f32]) -> u64 {
        self.ev.push(format!("scratch rsi={} rdx={} ecx={} r8d={} g0={} g1={} hx={} hy={}", nm(this), bytes, ecx, r8d, g0, g1, join(hx), join(hy)));
        HANDLE.with(|h| h.get())
    }
    fn pool_run(&mut self, this: u64, job: u32, radius: u32, src: u64, dst: u64, aux: u64) {
        self.ev.push(format!("pool rdi={} job=0x{:x} radius={} src={} dst={} aux={}", nm(this), job, radius, nm(src), nm(dst), nm(aux)));
    }
    fn copy(&mut self, dst: u64, src: u64, edx: u32, w0: u64, w1: u64, w2: u64) {
        self.ev.push(format!("copy rdi={} rsi={} edx={} w0={} w1={} w2={}", nm(dst), nm(src), edx, w0, w1, nm(w2)));
    }
    fn scratch_free(&mut self, handle: u64, g0: u64, g1: u64) {
        self.ev.push(format!("free rdi={} g0={} g1={}", handle, g0, g1));
    }
}
thread_local! { static HANDLE: std::cell::Cell<u64> = std::cell::Cell::new(0); }

/// NaN-canonical form of the float-bit fields of an event line; returns (canonical, number of NaN fields with a non-canonical payload).
fn canon(line: &str, nan_payload: &mut usize) -> String {
    let mut out = Vec::new();
    for tok in line.split(' ') {
        if let Some((k, v)) = tok.split_once('=') {
            if matches!(k, "x0" | "x1" | "radius" | "hx" | "hy") && v != "-" {
                let c: Vec<String> = v
                    .split(',')
                    .map(|b| {
                        let bits: u32 = b.parse().unwrap();
                        if f32::from_bits(bits).is_nan() {
                            if bits != 0x7fc00000 {
                                *nan_payload += 1;
                            }
                            "nan".to_string()
                        } else {
                            b.to_string()
                        }
                    })
                    .collect();
                out.push(format!("{}={}", k, c.join(",")));
                continue;
            }
        }
        out.push(tok.to_string());
    }
    out.join(" ")
}

fn floats_in(line: &str) -> usize {
    line.split(' ').filter_map(|t| t.split_once('=')).filter(|(k, v)| matches!(*k, "x0" | "x1" | "radius" | "hx" | "hy") && *v != "-").map(|(_, v)| v.split(',').count()).sum()
}

#[derive(Default, Debug)]
pub struct Tally {
    cases: usize,
    compared: usize,
    skipped_emu_fault: usize,
    refused: usize,
    floats: usize,
    events: usize,
    diffs: usize,
    nan_payload: usize,
    first: Option<String>,
}

pub fn run_layout(name: &str) -> Tally {
    let text = std::fs::read_to_string(format!("{}/cases/{}.txt", base(), name)).unwrap();
    let mut t = Tally::default();
    let mut ptrs: HashMap<String, String> = HashMap::new();
    let mut lines = text.lines().peekable();
    while let Some(l) = lines.next() {
        if let Some(rest) = l.strip_prefix("layout ") {
            for kv in rest.split(' ').skip(1) {
                let (k, v) = kv.split_once('=').unwrap();
                ptrs.insert(k.into(), v.into());
            }
            continue;
        }
        let case = l.strip_prefix("case ").unwrap().to_string();
        let pl = lines.next().unwrap().strip_prefix("param ").unwrap().to_string();
        let p: HashMap<&str, &str> = pl.split(' ').map(|kv| kv.split_once('=').unwrap()).collect();
        let st = lines.next().unwrap().strip_prefix("status ").unwrap().to_string();
        let mut want = Vec::new();
        loop {
            let e = lines.next().unwrap();
            if e == "end" {
                break;
            }
            want.push(e.strip_prefix("ev ").unwrap().to_string());
        }
        t.cases += 1;
        if st != "ok" {
            t.skipped_emu_fault += 1;
            continue;
        }
        let pb = |k: &str| -> u32 { let v = p[k]; if let Some(h) = v.strip_prefix("0x") { u32::from_str_radix(h, 16).unwrap() } else { v.parse().unwrap() } };
        let pi = |k: &str| -> i32 { p[k].parse().unwrap() };
        let a = Args {
            this: id("pg"),
            mode: pi("mode"),
            p_in: id(&ptrs["rdx"]),
            p_b: id(&ptrs["rcx"]),
            p_c: id(&ptrs["r8"]),
            p_d: id("pd"),
            p_e: id("pe"),
            roi: [pi("roi0"), pi("roi1"), pi("roi2"), pi("roi3")],
            p_f: id("pf"),
            n: pi("n"),
            flag: pi("flag") as u32,
            sx: f32::from_bits(pb("sx")),
            sy: f32::from_bits(pb("sy")),
        };
        HANDLE.with(|h| h.set(p["handle"].parse().unwrap()));
        let mut rec = Rec { ev: Vec::new(), powf_ret: p["powf_ret"].parse().unwrap() };
        match core_mode4(&a, &mut rec) {
            Err(e) => {
                t.refused += 1;
                if t.first.is_none() {
                    t.first = Some(format!("{}: port refuses: {:?}", case, e));
                }
                continue;
            }
            Ok(()) => {}
        }
        t.compared += 1;
        let mut np = 0usize;
        let g: Vec<String> = rec.ev.iter().map(|x| canon(x, &mut np)).collect();
        let mut np2 = 0usize;
        let w: Vec<String> = want.iter().map(|x| canon(x, &mut np2)).collect();
        t.events += w.len();
        t.floats += want.iter().map(|x| floats_in(x)).sum::<usize>();
        t.nan_payload += np.max(np2);
        if g != w {
            t.diffs += 1;
            if t.first.is_none() || t.first.as_ref().unwrap().contains("refuses") {
                let k = g.iter().zip(w.iter()).position(|(a, b)| a != b).unwrap_or(g.len().min(w.len()));
                t.first = Some(format!("{}: event {} port {:?} emu {:?}", case, k, g.get(k), w.get(k)));
            }
        }
    }
    t
}

#[test]
fn t2_all_layouts() {
    if !effects_vecdir::have_all(&["wave_s9/esh5_core/t2"]) {
        return;
    }
    let mut tot = Tally::default();
    for l in ["L0", "L1", "L2", "L3", "L4", "L5"] {
        let t = run_layout(l);
        println!("{} {:?}", l, t);
        tot.cases += t.cases;
        tot.compared += t.compared;
        tot.refused += t.refused;
        tot.skipped_emu_fault += t.skipped_emu_fault;
        tot.floats += t.floats;
        tot.events += t.events;
        tot.diffs += t.diffs;
        tot.nan_payload += t.nan_payload;
        if tot.first.is_none() {
            tot.first = t.first;
        }
    }
    println!("TOTAL {:?}", tot);
    assert_eq!(tot.diffs, 0, "{:?}", tot.first);
    assert_eq!(tot.skipped_emu_fault, 0);
    assert_eq!(tot.refused, 0, "{:?}", tot.first);
}
