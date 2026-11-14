//! Recorded cases of the shake_shift body: VEC_DIR/shake_shift_cases.tsv, compared bit for bit with the port.
//!
//! Records (tab separated, hex words):
//! - `T <table>`: the 1,024 entry table (3 words per entry, little endian bytes) that every guard-0 case builds.
//! - `C <idx> <guard> <p0..p11> <out> <guard out> <tail> <frame>` (the frame column is part of the stored format and is not read here): the twelve parameter words in `Params` order, the guard word the
//!   case starts with (0 builds the table, 1 runs on the built table), the output float, the guard after the call, the 12 bytes after
//!   the table and the 0xb10 byte frame of the original run.
//!
//! Compared per case: the output float, the guard, the whole table of a guard-0 case, the entry past the table. A NaN that differs from the recorded one counts as the engine's
//! two-NaN operand choice only when the port saw both operands NaN in that call. Both route selectors (parameter 9 zero and non-zero)
//! and both guard states must be present. Prints NOT RUN when the file is absent.
use shake_shift::body::{shake_shift_body, Config, Globals, Params, NAN_PAIRS, TAB_LEN};
use shake_shift::weights;
use shake_shift::RefServices;

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()).collect()
}

fn words(b: &[u8]) -> Vec<u32> {
    b.chunks(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

#[test]
fn recorded_cases() {
    let dir = match std::env::var("VEC_DIR") {
        Ok(d) => d,
        Err(_) => {
            println!("NOT RUN: VEC_DIR is not set");
            return;
        }
    };
    let path = std::path::Path::new(&dir).join("shake_shift_cases.tsv");
    let text = match std::fs::read_to_string(&path) {
        Ok(t) => t,
        Err(_) => {
            println!("NOT RUN: {} is absent", path.display());
            return;
        }
    };
    let cfg = Config::default();
    let mut sv = RefServices::new();
    let wt = weights::build_avx();
    let scale = weights::scale_from_cutoff(9074);
    let mut table: Option<Vec<[u32; 3]>> = None;
    let (mut n, mut route_a, mut route_b, mut stage1, mut stage2, mut nan_class) = (0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
    let mut bad: Vec<String> = Vec::new();
    for line in text.lines().filter(|l| !l.is_empty()) {
        let f: Vec<&str> = line.split('\t').collect();
        if f[0] == "T" {
            let w = words(&unhex(f[1]));
            assert_eq!(w.len(), 3 * TAB_LEN);
            table = Some(w.chunks(3).map(|c| [c[0], c[1], c[2]]).collect());
            continue;
        }
        assert_eq!(f[0], "C");
        let idx: usize = f[1].parse().unwrap();
        let guard: u32 = f[2].parse().unwrap();
        let pb: Vec<u32> = f[3..15].iter().map(|s| u32::from_str_radix(s, 16).unwrap()).collect();
        let rec_out = u32::from_str_radix(f[15], 16).unwrap();
        let rec_guard: u32 = f[16].parse().unwrap();
        let rec_tail = words(&unhex(f[17]));
        let g_in = |i: usize| f32::from_bits(pb[i]);
        let p = Params { seed_x: g_in(0), time: g_in(1), amp_x: g_in(2), rate_x: g_in(3), sine_amp: g_in(4), sine_freq: g_in(5), amp_y: g_in(6), rate_y: g_in(7), seed_y: g_in(8), route: g_in(9), limit_gap: g_in(10), gain: g_in(11) };
        let mut g = Globals { guard, tab: vec![[0; 3]; TAB_LEN], tail: [0; 3], weights: wt.float.clone(), scale };
        if guard != 0 {
            g.tab = table.clone().expect("a table record precedes the cases");
        }
        let before = NAN_PAIRS.load(std::sync::atomic::Ordering::Relaxed);
        let out = shake_shift_body(&p, &cfg, &mut g, &mut sv).to_bits();
        let pairs = NAN_PAIRS.load(std::sync::atomic::Ordering::Relaxed) - before;
        let mut why = String::new();
        if out != rec_out {
            if f32::from_bits(out).is_nan() && f32::from_bits(rec_out).is_nan() && pairs > 0 {
                nan_class += 1;
            } else {
                why += &format!("out recorded={:08x} port={:08x}; ", rec_out, out);
            }
        }
        if g.guard != rec_guard {
            why += &format!("guard recorded={} port={}; ", rec_guard, g.guard);
        }
        if guard == 0 && g.tab.as_slice() != table.as_ref().expect("table record").as_slice() {
            why += "table; ";
        }
        if g.tail[..] != rec_tail[..] {
            why += "tail; ";
        }
        if p.route == 0.0 {
            route_a += 1;
        } else {
            route_b += 1;
        }
        if guard == 0 {
            stage1 += 1;
        } else {
            stage2 += 1;
        }
        n += 1;
        if !why.is_empty() {
            bad.push(format!("case {}: {}", idx, why));
        }
    }
    println!(
        "{} recorded cases ({} on route A, {} on route B, {} building the table, {} on the built table), {} differ, {} two-NaN outputs",
        n,
        route_a,
        route_b,
        stage1,
        stage2,
        bad.len(),
        nan_class
    );
    assert!(n >= 3000 && route_a > 0 && route_b > 0 && stage1 > 0 && stage2 > 0, "the case file does not cover both routes and both guard states");
    assert!(bad.is_empty(), "{} cases differ, first: {}", bad.len(), bad[0]);
}
