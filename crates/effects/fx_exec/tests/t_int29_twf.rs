//! Tests of `twframe` (reads the stored sets in place, writes its reports to `effects_vecdir::out_dir()`), one sequential test because the mutation switch is global: (1) the port against the stored T2 set (5851 cases), (2) the mutant floor over the same set,
//! (3) the new limit-window arithmetic against the new emulator block L (15463 cases), (4) unit tests of the history, record and cache models.
use fx_exec::twframe::*;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::sync::atomic::Ordering;

fn hx(s: &str) -> u32 { u32::from_str_radix(s, 16).unwrap() }
fn fb(s: &str) -> f32 { f32::from_bits(hx(s)) }

/// Returns Some(ok) for a case, None when the emulator did not finish it.
fn eval(c: &[&str], e: &[String]) -> Option<bool> {
    if e[1] != "1" { return None; }
    let eh: Vec<u32> = e[2..6].iter().map(|s| hx(s)).collect();
    Some(match c[1] {
        "F" => {
            let scalar: i32 = c[2].parse().unwrap(); let r8: i32 = c[3].parse().unwrap();
            let w: i32 = c[4].parse().unwrap(); let h: i32 = c[5].parse().unwrap();
            let lanes: Vec<f32> = if c.len() > 7 && !c[7].is_empty() { c[7].split(',').map(fb).collect() } else { vec![] };
            let n = w.wrapping_mul(h);
            // the lane array in the stored set is exactly as long as the emulator read; a mutant that reads more would index past it: guard
            let (lo, hi) = if scalar != 0 { frame_range_scalar(lanes[0], r8 != 0) } else {
                let need = if n > 0 { n as usize } else { 0 };
                if need > lanes.len() { return Some(false); }
                frame_range_lanes(w, h, &lanes, r8 != 0) };
            lo as u32 == eh[0] && hi as u32 == eh[1]
        }
        "C" => {
            let i = CheckoutIn { f: c[2].parse().unwrap(), big_f: fb(c[3]), dur_frames: fb(c[4]), lim_global: c[5].parse().unwrap(),
                step: if c[9] == "1" { Some(c[6].parse().unwrap()) } else { None }, kind: c[7].parse().unwrap(), f4: c[8].parse().unwrap() };
            match checkout_request(i) { Ok(r) => r.stored.to_bits() == eh[0] && r.trunc_x as u32 == eh[1] && r.frame as u32 == eh[2], Err(_) => false }
        }
        "D" => {
            let st: i32 = c[2].parse().unwrap(); let s = fb(c[3]);
            // the D block's native is the request's host_time formula; route it through checkout_request's formula via a one-off request is impossible (inputs differ), so
            // the mutant switch 17 is applied here through the same expression
            let t = if MUT.load(Ordering::Relaxed) == 17 { let v = (st as f32) * s; let r = v.round_ties_even(); if r.is_nan() || r >= 2147483648.0 || r < -2147483648.0 { i32::MIN } else { r as i32 } } else { cvtt_ss((st as f32) * s) };
            t as u32 == eh[0]
        }
        "E" => {
            let s = fb(c[2]);
            let k = if MUT.load(Ordering::Relaxed) == 18 { cvtt_ss(s * 2.0 + 100.0) } else { cvtt_ss(s.mul_add(2.0, 100.0)) };
            k as u32 == eh[0]
        }
        "W" => {
            let p: Vec<i64> = c[2..9].iter().map(|s| s.parse().unwrap()).collect();
            let o = render_entry_f(p[0] as i32, p[1] as i32, p[2] as i32, p[3] as u32, p[4] as i32, p[5] as i32, p[6] as i32);
            o.f.to_bits() == eh[0] && o.dur_frames.to_bits() == eh[1] && o.ts_over_step.to_bits() == eh[2] && o.step as u32 == eh[3]
        }
        _ => panic!("block"),
    })
}

#[test]
fn twf3_all() {
    if !effects_vecdir::have_all(&["wave_s9/twf2_frame_request/t2", "wave_s9/twf3_frame_floor/t3"]) {
        return;
    }
    let d2 = effects_vecdir::dir_str("wave_s9/twf2_frame_request/t2");
    let d3 = effects_vecdir::dir_str("wave_s9/twf3_frame_floor/t3");
    let cases = std::fs::read_to_string(format!("{d2}cases.tsv")).unwrap() + &std::fs::read_to_string(format!("{d3}cases_W2.tsv")).unwrap();
    let emu = std::fs::read_to_string(format!("{d2}emu.tsv")).unwrap() + &std::fs::read_to_string(format!("{d3}emu_W2.tsv")).unwrap();
    let mut em: HashMap<String, Vec<String>> = HashMap::new();
    for l in emu.lines() { let v: Vec<String> = l.split('\t').map(|s| s.to_string()).collect(); em.insert(v[0].clone(), v); }
    let rows: Vec<Vec<&str>> = cases.lines().map(|l| l.split('\t').collect()).collect();
    let mut out = String::new();

    // (1) baseline
    MUT.store(0, Ordering::Relaxed);
    let mut bad = 0; let mut n = 0;
    for c in &rows { match eval(c, &em[c[0]]) { Some(true) => n += 1, _ => { bad += 1; } } }
    writeln!(out, "BASELINE (mutant 0, 5851 stored T2 cases plus 72 new W rows with time scale >= 2^31 and kinds): {n} of {} equal, {bad} differing", rows.len()).unwrap();

    // (2) mutants
    let names = std::fs::read_to_string(format!("{d3}MUTANTS.tsv")).unwrap();
    let mut mt = String::from("id\tkilled_cases\tof\tfirst_killer\tkillers_by_block\tdescription\n");
    let mut survivors = vec![];
    for l in names.lines().skip(1) {
        let f: Vec<&str> = l.split('\t').collect();
        let id: u32 = f[0].parse().unwrap();
        MUT.store(id, Ordering::Relaxed);
        let mut killed = 0; let mut first = String::new(); let mut by: HashMap<String, usize> = HashMap::new();
        for c in &rows {
            let ok = eval(c, &em[c[0]]);
            if ok != Some(true) { killed += 1; if first.is_empty() { first = c[0].to_string(); } *by.entry(c[1].to_string()).or_default() += 1; }
        }
        let mut bk: Vec<_> = by.iter().map(|(k, v)| format!("{k}:{v}")).collect(); bk.sort();
        writeln!(mt, "{id}\t{killed}\t{}\t{first}\t{}\t{}", rows.len(), bk.join(","), f[1]).unwrap();
        if killed == 0 { survivors.push(id); }
    }
    MUT.store(0, Ordering::Relaxed);
    std::fs::write(effects_vecdir::out_dir().join("mutants_result.tsv"), &mt).unwrap();
    writeln!(out, "MUTANTS: {} run, survivors {:?}", names.lines().count() - 1, survivors).unwrap();

    // (3) block L
    let lc = std::fs::read_to_string(format!("{d3}cases_L.tsv")).unwrap();
    let le = std::fs::read_to_string(format!("{d3}emu_L.tsv")).unwrap();
    let mut lem: HashMap<String, Vec<String>> = HashMap::new();
    for l in le.lines() { let v: Vec<String> = l.split('\t').map(|s| s.to_string()).collect(); lem.insert(v[0].clone(), v); }
    let (mut ln, mut lbad, mut win, mut fma_vs_mul, mut dec_diff) = (0usize, 0usize, 0usize, 0usize, 0usize);
    let mut dd_first = String::new();
    let mut first_bad = vec![];
    for l in lc.lines() {
        let c: Vec<&str> = l.split('\t').collect();
        let e = &lem[c[0]];
        assert_eq!(e[1], "1");
        let var: i32 = c[2].parse().unwrap(); let cur: i32 = c[4].parse().unwrap(); let st: i32 = c[5].parse().unwrap(); let last: i32 = c[6].parse().unwrap();
        let ar = if var == 0 { Arith::Fma } else { Arith::MulAdd };
        let (w, lim) = limit_window(cur, last, st, ar);
        let (w2, lim2) = limit_window(cur, last, st, if var == 0 { Arith::MulAdd } else { Arith::Fma });
        if w != w2 || lim.to_bits() != lim2.to_bits() { fma_vs_mul += 1; }
        if w != w2 { dec_diff += 1; if dd_first.is_empty() { dd_first = format!("{} cur={cur} step={st} last={last}", c[0]); } }
        let ok = (if w { 1u32 } else { 2 }) == hx(&e[2]) && lim.to_bits() == hx(&e[3]);
        ln += 1; if w { win += 1; }
        if !ok { lbad += 1; if first_bad.len() < 10 { first_bad.push(format!("{} nat win={w} lim={:08x} emu {} {}", c[0], lim.to_bits(), e[2], e[3])); } }
    }
    writeln!(out, "BLOCK L (limit window arithmetic, emulator AVX body and SSE twin): {} of {ln} equal, {lbad} differing; window cases {win}; cases where the FMA and mul+add forms differ (either field): {fma_vs_mul}; of those the window DECISION differs in {dec_diff} (first: {dd_first})", ln - lbad).unwrap();
    for f in &first_bad { writeln!(out, "DIFF {f}").unwrap(); }

    // (4) unit tests of the history, record and cache models
    let mk = |cur: i32, step: i32, host: i32| EntryIn { host_id: host, cur_time: cur, step, entry_in_a: 1, entry_in_b: 1, entry_in_c: 1, entry_in_d: 1, entry_in_e: 0, entry_in_f: 0, entry_in_g: 0, entry_in_h: 0, host_arg_value: 0,
        layer_empty: false, flag_cell_a_4: true, settings: (0, 0), arith: Arith::Fma };
    let mut h = OwnerHist::fresh();
    // fresh: last = -1e6, so the first render is not in the window of a contract step unless cur is within 1.9 * step of -1e6
    assert_eq!(render_entry_limit(&mut h, &mk(10, 1, 0)), Some(10));
    assert_eq!(render_entry_limit(&mut h, &mk(11, 1, 0)), Some(10));  // 11 > 10 and 11 < 10 + 1.9: run continues, anchor stays 10
    assert_eq!(render_entry_limit(&mut h, &mk(12, 1, 0)), Some(10));  // 12 < 11 + 1.9
    assert_eq!(render_entry_limit(&mut h, &mk(14, 1, 0)), Some(14));  // 14 >= 12 + 1.9: a jump restarts the run
    assert_eq!(render_entry_limit(&mut h, &mk(14, 1, 0)), Some(14));  // cur == last: restart (jle)
    assert_eq!(render_entry_limit(&mut h, &mk(13, 1, 0)), Some(13));  // backwards: restart
    let mut i2 = mk(15, 1, 0); i2.settings = (3, 0);
    assert_eq!(render_entry_limit(&mut h, &i2), Some(15));            // settings words differ (0x274600 returns 1): restart although in window
    let mut i3 = mk(16, 1, 0); i3.flag_cell_a_4 = false;
    assert_eq!(render_entry_limit(&mut h, &i3), Some(0));             // bit 2 of [s+0x59e] clear: the cell is zeroed
    let mut i4 = mk(17, 1, 0); i4.entry_in_a = 0; i4.entry_in_c = 0; i4.entry_in_b = 0; i4.entry_in_d = 0; i4.layer_empty = true;
    assert_eq!(render_entry_limit(&mut h, &i4), None);                // early-out path 0x70152f: no cell write
    // host id 2 and 9 select record 1 through bit 0
    let mut h2 = OwnerHist::fresh();
    let mut i5 = mk(100, 1, 2); i5.host_arg_value = 130;
    render_entry_limit(&mut h2, &i5); assert!(h2.bit0 && h2.cur == 1);
    i5.host_arg_value = 140; render_entry_limit(&mut h2, &i5); assert!(!h2.bit0 && h2.cur == 0);
    let mut i6 = mk(100, 1, 9); i6.entry_in_e = 1; i6.entry_in_f = 5; i6.entry_in_g = 1; i6.entry_in_h = 13;
    render_entry_limit(&mut h2, &i6); assert!(h2.bit0);
    // sequence writer: consecutive frames keep the anchor
    let mut h3 = OwnerHist::fresh();
    render_entry_seq(&mut h3, 5, (0, 0), true); assert_eq!(h3.rec[0].anchor, 5);
    render_entry_seq(&mut h3, 6, (0, 0), true); assert_eq!(h3.rec[0].anchor, 5);
    render_entry_seq(&mut h3, 8, (0, 0), true); assert_eq!(h3.rec[0].anchor, 8);
    // cache lifetime
    let mut c = FrameCache::default();
    assert_eq!(c.checkout(200, 7, true), None); assert_eq!(c.checkout(200, 9, true), Some(7)); // same key, other host value: stale hit inside a render
    c.end_of_render(true, true); assert_eq!(c.count, 0); assert_eq!(c.checkout(200, 9, true), None); // reset at the end of a render
    c.end_of_render(false, true); assert_eq!(c.checkout(200, 11, true), Some(9));                       // [ctx+0x788] != 0: persists into the next render
    let mut c2 = FrameCache::default();
    assert_eq!(c2.checkout(200, 7, false), None); assert_eq!(c2.checkout(200, 9, false), None); assert_eq!(c2.count, 0); // flag80 clear: inert
    let r = checkout_request(CheckoutIn { f: 3, big_f: 2.5, dur_frames: 100.0, lim_global: 0, step: Some(1), kind: 0, f4: 0 }).unwrap();
    assert_eq!(r.x, 3.5); assert_eq!(r.frame, 3);
    writeln!(out, "UNIT TESTS: history, record, sequence writer and cache models pass").unwrap();
    std::fs::write(effects_vecdir::out_dir().join("t3_summary.txt"), &out).unwrap();
    eprintln!("{out}");
    assert_eq!(bad, 0); assert_eq!(lbad, 0);
}
