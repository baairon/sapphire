//! DS1 R-PARITY judge: reads case files written by tools/mkcases.py (original kernel input and output captured from the emulator) and
//! compares the Rust port bit for bit. usage: ds_check CASE_DIR [mutant]
//! case file: u32 kind (0 down, 1 up), W, H, w2, h2, dst_phase, nin, nexp, then f32 in[nin], f32 exp[nexp]
//! down: in = big (W x H), exp = small (w2 x h2). up: in = small, exp = big (W x H), compared on rows < min(H, 2*h2 - 1) (the caller's row copy is separate).
use fx_exec::downsampled::{down_plane, up_plane};
use std::fs;

fn rd_u32(b: &[u8], i: usize) -> u32 {
    u32::from_le_bytes([b[4 * i], b[4 * i + 1], b[4 * i + 2], b[4 * i + 3]])
}

fn main() {
    let dir = std::env::args().nth(1).expect("case dir");
    let mut names: Vec<_> = fs::read_dir(&dir).unwrap().filter_map(|e| e.ok()).map(|e| e.path()).filter(|p| p.extension().map(|x| x == "case").unwrap_or(false)).collect();
    names.sort();
    let (mut cases, mut bad_cases, mut floats, mut bad_floats, mut nan_pl) = (0, 0, 0u64, 0u64, 0u64);
    for p in names {
        let b = fs::read(&p).unwrap();
        let (kind, w, h, w2, h2, phase, nin, nexp) = (rd_u32(&b, 0), rd_u32(&b, 1) as usize, rd_u32(&b, 2) as usize, rd_u32(&b, 3) as usize, rd_u32(&b, 4) as usize, rd_u32(&b, 5), rd_u32(&b, 6) as usize, rd_u32(&b, 7) as usize);
        let f = |off: usize, n: usize| -> Vec<f32> { (0..n).map(|i| f32::from_bits(rd_u32(&b, 8 + off + i))).collect() };
        let inp = f(0, nin);
        let exp = f(nin, nexp);
        let got = if kind == 0 { down_plane(&inp, w, h, phase) } else { up_plane(&inp, w2, h2, w, h) };
        let limit = if kind == 0 { nexp } else { w * h.min(2 * h2 - 1) };
        let (mut d, mut np) = (0u64, 0u64);
        for i in 0..limit {
            let (a, c) = (exp[i].to_bits(), got[i].to_bits());
            if a != c {
                if exp[i].is_nan() && got[i].is_nan() { np += 1; } else { d += 1; }
            }
        }
        cases += 1;
        floats += limit as u64;
        bad_floats += d;
        nan_pl += np;
        if d > 0 {
            bad_cases += 1;
            let first = (0..limit).find(|&i| exp[i].to_bits() != got[i].to_bits() && !(exp[i].is_nan() && got[i].is_nan())).unwrap();
            println!("DIFF {} kind {} {}x{} phase {} first at {} (x {} y {}) orig {:08x} native {:08x}", p.file_name().unwrap().to_string_lossy(), kind, w, h, phase, first, first % if kind == 0 { w2 } else { w }, first / if kind == 0 { w2 } else { w }, exp[first].to_bits(), got[first].to_bits());
        }
    }
    println!("ds_check cases {} different_cases {} floats {} different_floats {} nan_payload_only {}", cases, bad_cases, floats, bad_floats, nan_pl);
    std::process::exit(if bad_floats == 0 { 0 } else { 1 });
}
