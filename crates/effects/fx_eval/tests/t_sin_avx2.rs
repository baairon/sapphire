//! The AVX2/FMA sine fold against the stored emulator data (VEC_DIR/effects/sine_avx2) and hand values.
use fx_eval::svml_sin_avx2::{constant_cells, lookup_bytes, sin8, sin_rev, sinf_avx2, tables, window_bytes};
use fx_eval::svml_trig::sinf_svml;

fn rd(name: &str) -> Option<Vec<u8>> {
    std::fs::read(effects_vecdir::path(&format!("sine_avx2/{}", name))).ok()
}
fn words(b: &[u8]) -> Vec<u32> {
    b.chunks_exact(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}
fn not_run(test: &str) {
    println!("NOT RUN {}: VEC_DIR/effects/sine_avx2 data is absent", test);
}
fn isnan(u: u32) -> bool {
    (u & 0x7fff_ffff) > 0x7f80_0000
}

#[test]
fn tables_are_computed_equal_to_the_image() {
    let (a, b) = match (rd("tab_a.bin"), rd("tab_b.bin")) {
        (Some(a), Some(b)) => (a, b),
        _ => return not_run("tables_are_computed_equal_to_the_image"),
    };
    assert_eq!(a.len(), 65536);
    assert_eq!(b.len(), 8192);
    let w = window_bytes();
    assert_eq!(w.len(), 3072);
    assert!(w[..] == b[..3072], "window table differs from the image");
    let l = lookup_bytes();
    assert_eq!(l.len(), 4096);
    assert!(l[..] == a[..4096], "lookup table differs from the image");
    for (off, v) in constant_cells() {
        for lane in 0..8 {
            let o = off + 4 * lane;
            assert_eq!(u32::from_le_bytes([a[o], a[o + 1], a[o + 2], a[o + 3]]), v, "constant cell {:#x} lane {}", off, lane);
        }
    }
    // the image continues past the lookup table with cells of other routines; the route reads only the regions above
    println!("tables equal: window 3072 bytes, lookup 4096 bytes, {} constant cells", constant_cells().len());
}

#[test]
fn recorded_judge_calls_equal() {
    let d = match rd("judge_calls.bin") {
        Some(d) => words(&d),
        None => return not_run("recorded_judge_calls_equal"),
    };
    assert_eq!(d.len() % 16, 0);
    let mut n = 0;
    for c in d.chunks(16) {
        let mut x = [0u32; 8];
        x.copy_from_slice(&c[..8]);
        assert_eq!(sin8(x).to_vec(), c[8..].to_vec(), "judge call {:x?}", &c[..8]);
        n += 1;
    }
    assert!(n >= 129);
    println!("recorded judge calls equal: {}", n);
}

/// Mixed lane form: the T2 driver groups the arguments (fast before slow, eight per call, the last padded with its last member).
fn mixed_groups(xs: &[u32], idx: &[u32]) -> Vec<Vec<usize>> {
    let slow = |u: u32| {
        let a = u & 0x7fff_ffff;
        a > 0x461c_4000 && a <= 0x7f80_0000
    };
    let mut out = Vec::new();
    for cls in [false, true] {
        let grp: Vec<usize> = idx.iter().map(|&i| i as usize).filter(|&i| slow(xs[i]) == cls).collect();
        for c in grp.chunks(8) {
            out.push(c.to_vec());
        }
    }
    out
}

#[test]
fn corpus_mixed_lane_equals_the_emulator() {
    let (inp, out, idx) = match (rd("in_main.bin"), rd("avx2_mixed.bin"), rd("avx2_mixed.bin.idx")) {
        (Some(a), Some(b), Some(c)) => (words(&a), words(&b), words(&c)),
        _ => return not_run("corpus_mixed_lane_equals_the_emulator"),
    };
    let pos: std::collections::HashMap<u32, usize> = idx.iter().enumerate().map(|(p, &i)| (i, p)).collect();
    let (mut n, mut diff, mut nanpair) = (0usize, 0usize, 0usize);
    let mut first = None;
    for g in mixed_groups(&inp, &idx) {
        let mut lanes: Vec<u32> = g.iter().map(|&i| inp[i]).collect();
        let last = *lanes.last().unwrap();
        lanes.resize(8, last);
        let mut v = [0u32; 8];
        v.copy_from_slice(&lanes);
        let r = sin8(v);
        for (j, &i) in g.iter().enumerate() {
            let want = out[pos[&(i as u32)]];
            n += 1;
            if r[j] != want {
                if isnan(r[j]) && isnan(want) {
                    nanpair += 1;
                }
                diff += 1;
                if first.is_none() {
                    first = Some((i, inp[i], r[j], want));
                }
            }
        }
    }
    println!("mixed compared {} differing {} (both NaN: {}) first {:x?}", n, diff, nanpair, first);
    assert_eq!(diff, 0, "first {:x?}", first);
    assert!(n >= 318_690);
}

#[test]
fn flicker_wave_class_broadcast_equals_the_emulator() {
    let (inp, out, idx) = match (rd("in_flicker.bin"), rd("avx2_flicker_bcast.bin"), rd("avx2_flicker_bcast.bin.idx")) {
        (Some(a), Some(b), Some(c)) => (words(&a), words(&b), words(&c)),
        _ => return not_run("flicker_wave_class_broadcast_equals_the_emulator"),
    };
    let mut diff = 0;
    for (p, &i) in idx.iter().enumerate() {
        let x = inp[i as usize];
        if sin8([x; 8])[0] != out[p] {
            diff += 1;
        }
    }
    println!("flicker broadcast compared {} differing {}", idx.len(), diff);
    assert_eq!(diff, 0);
}

#[test]
fn broadcast_corpus_equals_the_emulator_when_present() {
    let (inp, out, idx) = match (rd("in_main.bin"), rd("avx2_bcast.bin"), rd("avx2_bcast.bin.idx")) {
        (Some(a), Some(b), Some(c)) => (words(&a), words(&b), words(&c)),
        _ => return not_run("broadcast_corpus_equals_the_emulator_when_present"),
    };
    let (mut diff, mut nanpair) = (0, 0);
    let mut first = None;
    for (p, &i) in idx.iter().enumerate() {
        let x = inp[i as usize];
        let r = sin8([x; 8]);
        assert!(r.iter().all(|&q| q == r[0]), "lanes of a broadcast call differ for {:08x}", x);
        if r[0] != out[p] {
            if isnan(r[0]) && isnan(out[p]) {
                nanpair += 1;
            }
            diff += 1;
            first.get_or_insert((i, x, r[0], out[p]));
        }
    }
    println!("broadcast compared {} differing {} (both NaN {}) first {:x?}", idx.len(), diff, nanpair, first);
    assert_eq!(diff, 0, "first {:x?}", first);
}

#[test]
fn hand_values_need_no_data() {
    let b = |x: f32| x.to_bits();
    // the judge's answer for 2 pi * 0.25 in a Flicker compile
    assert_eq!(b(sinf_avx2(f32::from_bits(0x3fc9_0fdb))), 0x3f80_0000);
    assert_eq!(b(sin_rev(0.25)), 0x3f80_0000);
    assert_eq!(b(sinf_avx2(0.0)), 0);
    assert_eq!(b(sinf_avx2(-0.0)), 0x8000_0000);
    assert_eq!(b(sinf_avx2(f32::INFINITY)), 0xffc0_0000);
    assert_eq!(b(sinf_avx2(f32::NEG_INFINITY)), 0xffc0_0000);
    assert_eq!(b(sinf_avx2(f32::from_bits(0x7fc0_0001))), 0x7fc0_0001);
    assert_eq!(b(sinf_avx2(f32::from_bits(0xff80_0001))), 0xffc0_0001);
    // denormal keeps its value and sign
    assert_eq!(b(sinf_avx2(f32::from_bits(0x8000_0003))), 0x8000_0003);
    // the SSE entry differs here: -0.0 and the wave argument of phase -0.9
    assert_ne!(b(sinf_svml(-0.0)), b(sinf_avx2(-0.0)));
    let w = f32::from_bits(0x40e7_3852);
    assert_ne!(b(sinf_svml(w)), b(sinf_avx2(w)));
}

#[test]
fn deterministic_and_lane_independent_in_the_fast_route() {
    let a = [0x3f80_0000u32, 0x4000_0000, 0xbf80_0000, 0x4048_f5c3, 0x3e80_0000, 0x1, 0x8000_0000, 0x4640_e6b6 & 0x45ff_ffff];
    let r1 = sin8(a);
    let r2 = sin8(a);
    assert_eq!(r1, r2);
    for k in 0..8 {
        assert_eq!(sin8([a[k]; 8])[0], r1[k], "lane {}", k);
    }
}

#[test]
fn a_large_lane_moves_every_lane_to_the_slow_route() {
    // 0x461c4001 is just above 10000.0: the vector takes the slow route, a small lane is still a sine
    let mut v = [0x3e80_0000u32; 8];
    v[3] = 0x461c_4001;
    let r = sin8(v);
    assert_eq!(r[3], sin8([0x461c_4001; 8])[0]);
    assert_eq!(r[0], sin8([0x3e80_0000; 8])[0]); // lanes are independent in the slow route as well
    let _ = tables();
}
