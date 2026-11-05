//! DRW5 T2 compare: the native caller port + set_aspect against the emulated original (t2/emu_caller.tsv, cases t2/cases.tsv).
use fx_exec::aspect::*;
use std::fs;

/// W237 (c): the fixture is inside the integrated tree and its SHA-256 is checked against fixtures/MANIFEST.sha256 before it is read.
fn fixture(rel: &str) -> String {
    let root = effects_vecdir::path("fixtures");
    let p = root.join(rel);
    let man = std::fs::read_to_string(root.join("MANIFEST.sha256")).expect("fixtures/MANIFEST.sha256");
    let want = man.lines().find(|l| l.ends_with(&format!("  {}", rel))).expect("fixture not in the manifest").split_whitespace().next().unwrap().to_string();
    let out = std::process::Command::new("sha256sum").arg(&p).output().expect("sha256sum");
    let got = String::from_utf8_lossy(&out.stdout).split_whitespace().next().unwrap_or("").to_string();
    assert_eq!(got, want, "fixture SHA mismatch for {}", rel);
    std::fs::read_to_string(&p).expect("fixture")
}

fn h32(s: &str) -> u32 { u32::from_str_radix(s, 16).unwrap() }
fn h64(s: &str) -> u64 { u64::from_str_radix(s, 16).unwrap() }

#[test]
fn caller_block_matches_emulated_original() {
    if !effects_vecdir::have_all(&["fixtures", "fixtures/drw5_t2"]) {
        return;
    }
    let dir = effects_vecdir::path("fixtures/drw5_t2").to_string_lossy().into_owned();
    let cases = fixture("drw5_t2/cases.tsv");
    let emu = fixture("drw5_t2/emu_caller.tsv");
    let (mut n, mut bad, mut dw) = (0usize, 0usize, 0usize);
    let mut routes = [0usize; 3];
    let mut out = String::new();
    for (c, e) in cases.lines().zip(emu.lines()) {
        let c: Vec<&str> = c.split('\t').collect();
        let e: Vec<&str> = e.split('\t').collect();
        assert_eq!(c[0], e[0]);
        assert_eq!(e[3], "1", "emulator case {} not done: {}", c[0], e.get(10).unwrap_or(&""));
        let inp = CallerInputs {
            override_cell: f32::from_bits(h32(c[1])), host_id: h32(c[2]),
            w_cell_a: h32(c[3]), w_cell_b: h32(c[4]), w_cell_c: h32(c[5]), w_cell_d: h32(c[6]), w_cell_e: h32(c[7]), w_cell_f: h32(c[8]),
            d_cell_b: f64::from_bits(h64(c[9])), d_cell_c: f64::from_bits(h64(c[10])),
        };
        let aspect_block_a = f32::from_bits(h32(c[11]));
        let (route, a) = caller_aspect(&inp);
        let rs = match route { Route::Override => "OVR", Route::Computed => "CMP", Route::Fallback => "FBK" };
        routes[route as usize] += 1;
        let b = set_aspect(a, aspect_block_a).expect("a != 0 on every route");
        let mine = [b.aspect_block_b, b.aspect_block_c, b.aspect_block_d, b.aspect_block_e, b.aspect_block_f as u32, b.aspect_block_g as u32];
        let mut diff = 0;
        if rs != e[1] || a.to_bits() != h32(e[2]) { diff += 1; }
        for k in 0..6 {
            n += 1;
            if mine[k] != h32(e[4 + k]) { dw += 1; diff += 1; }
        }
        if diff > 0 {
            bad += 1;
            out.push_str(&format!("DIFF {} native {} {:08x} emu {} {}\n", c[0], rs, a.to_bits(), e[1], e[2]));
        }
        let _ = n;
    }
    println!("{}", out);
    println!("cases {} differing_cases {} differing_dwords {} routes OVR/CMP/FBK(by enum order Override,Computed,Fallback) {:?}", cases.lines().count(), bad, dw, routes);
    fs::write(format!("{}/native_caller_summary.txt", dir), format!("cases {} differing_cases {} differing_dwords {} routes {:?}\n{}", cases.lines().count(), bad, dw, routes, out)).unwrap();
    assert_eq!(bad, 0);
}

/// Sensitivity control (a specification check, not a kill claim): variants of the port written beside it; each must differ from the
/// emulated original on at least one case of the T2, otherwise the case set could not tell the port from that variant.
fn variant(i: &CallerInputs, m: u32) -> (Route, f32) {
    let one = (Route::Fallback, 1.0f32);
    if m != 4 && (i.override_cell as f64) > 0.001 { return (Route::Override, i.override_cell); }
    if m == 4 && (i.override_cell as f64) >= 0.001 { return (Route::Override, i.override_cell); }
    if i.w_cell_e == 0 || i.w_cell_f == 0 { return one; }
    if m != 2 && (i.w_cell_a == 0 || i.w_cell_b == 0) { return one; }
    if m == 2 && i.w_cell_a == 0 { return one; }
    if i.w_cell_c == 0 || i.w_cell_d == 0 { return one; }
    let num = if m == 7 { (i.w_cell_f as i32) as f32 } else { i.w_cell_f as f32 };
    let den = if m == 6 { i.w_cell_e as f32 } else { (i.w_cell_e as i32) as f32 };
    let dd = i.d_cell_b / i.d_cell_c;
    let inv = if m == 1 { false } else { i.host_id == 6 };
    let q = if inv { den / num } else { num / den };
    let a = if m == 8 { ((q * (dd as f32)) as f32) } else { ((q as f64) * dd) as f32 };
    let lo_ok = if m == 3 { (a as f64) >= 1e-4 } else { (a as f64) > 1e-4 };
    if !lo_ok { return one; }
    let hi_ok = if m == 5 { a <= 10000.0 } else { a < 10000.0 };
    if !hi_ok { return one; }
    (Route::Computed, a)
}

#[test]
fn case_set_distinguishes_variants() {
    if !effects_vecdir::have_all(&["fixtures", "fixtures/drw5_t2"]) {
        return;
    }
    let dir = effects_vecdir::path("fixtures/drw5_t2").to_string_lossy().into_owned();
    let cases = fixture("drw5_t2/cases.tsv");
    let emu = fixture("drw5_t2/emu_caller.tsv");
    let names = ["0 identity (must be 0)", "1 host 6 inversion removed", "2 pair gate A needs only w11c", "3 lo compare >=", "4 override >= 0.001",
                 "5 hi compare <=", "6 w12c unsigned", "7 w130 signed", "8 product in f32"];
    let mut out = String::new();
    for m in [0u32, 1, 2, 3, 4, 5, 6, 8, 7] {
        let mut n = 0;
        for (c, e) in cases.lines().zip(emu.lines()) {
            let c: Vec<&str> = c.split('\t').collect();
            let e: Vec<&str> = e.split('\t').collect();
            let inp = CallerInputs {
                override_cell: f32::from_bits(h32(c[1])), host_id: h32(c[2]), w_cell_a: h32(c[3]), w_cell_b: h32(c[4]), w_cell_c: h32(c[5]), w_cell_d: h32(c[6]),
                w_cell_e: h32(c[7]), w_cell_f: h32(c[8]), d_cell_b: f64::from_bits(h64(c[9])), d_cell_c: f64::from_bits(h64(c[10])),
            };
            let (r, a) = variant(&inp, m);
            let rs = match r { Route::Override => "OVR", Route::Computed => "CMP", Route::Fallback => "FBK" };
            if rs != e[1] || a.to_bits() != h32(e[2]) { n += 1; }
        }
        out.push_str(&format!("variant {} differs on {} of {} cases\n", names[m as usize], n, cases.lines().count()));
        // variants 3 and 4 change only the equality case (double)x == 1e-4 or == 0.001, which no f32 value attains (the doubles are not f32 values): equivalent by representation
        if m == 0 || m == 3 || m == 4 { assert_eq!(n, 0); } else { assert!(n > 0, "variant {} not distinguished", m); }
    }
    println!("{}", out);
    fs::write(format!("{}/variant_sensitivity.txt", dir), out).unwrap();
}
