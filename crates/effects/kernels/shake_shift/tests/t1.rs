use shake_shift::dispatch::*;
use shake_shift::hash::*;
use shake_shift::sin::*;
use shake_shift::weights::*;

#[test]
fn lcg_equals_modulo_for_all_states() {
    for x in 0..714025u32 {
        assert_eq!(lcg_step(x) as u64, (4096u64 * x as u64 + 150889) % 714025);
    }
}

#[test]
fn hash_builder_and_hashes() {
    let h = HashTable::build();
    assert_eq!(h, HashTable::build());
    assert_eq!(h.to_bytes().len(), 1024);
    // hand evaluation of entry 0: y1 = 150889, y2 = (4096*150889+150889) % 714025, ...
    let y1 = 150889u64;
    let y2 = (4096 * y1 + 150889) % 714025;
    let y3 = (4096 * y2 + 150889) % 714025;
    assert_eq!(h.t[0], ((y1 >> 2) as u32).wrapping_add((y2 as u32) << 8).wrapping_add((y3 as u32) << 18));
    // hash1 is four rounds; hash2 with y = 0 differs from hash1 only by four more rounds
    let mut v = 12345u32;
    for _ in 0..4 {
        v = (v >> 8).wrapping_add(h.t[(v & 0xff) as usize]);
    }
    assert_eq!(h.hash1(12345), v);
    assert_ne!(h.hash2(1, 2), h.hash2(2, 1));
}

#[test]
fn dispatch_targets() {
    assert_eq!(select_body(0x9d97ff), Target::Avx);
    assert_eq!(select_body(0x1d97ff), Target::Sse);
    assert_eq!(select_body(0x2), Target::CpuInit);
    assert_eq!(select_sine(0xff), Target::Avx);
    assert_eq!(select_sine(0xfe), Target::CpuInit);
    assert_eq!(select_sine(0x3), Target::Sse);
}

#[test]
fn sine_values_and_specials() {
    assert_eq!(sinf_computed(0.0).to_bits(), 0);
    assert_eq!(sinf_computed(-0.0).to_bits(), 0x8000_0000, "negative zero");
    let one = sinf_computed(1.0);
    assert!((one - 0.841_470_95).abs() < 1e-7);
    assert!((sinf_computed(100.0) - (-0.506_365_64)).abs() < 1e-6);
    assert!((sinf_computed(1.0e6) - (-0.349_994_0)).abs() < 1e-5);
    assert_eq!(sinf_computed(f32::INFINITY).to_bits(), 0xffc0_0000);
    assert_eq!(sinf_computed(f32::NEG_INFINITY).to_bits(), 0xffc0_0000);
    assert_eq!(sinf_computed(f32::from_bits(0x7f80_0001)).to_bits(), 0x7fc0_0001);
    assert_eq!(sinf_computed(f32::from_bits(0xffc0_1234)).to_bits(), 0xffc0_1234);
    assert!(sinf_computed(f32::MAX).abs() <= 1.0);
    assert_eq!(sinf_computed(f32::from_bits(1)).to_bits(), 1, "smallest denormal");
}

#[test]
fn weights_basic() {
    let a = build_avx();
    let s = build_sse();
    assert_eq!(a.int.len(), WEIGHT_LEN);
    assert_eq!(a.int[0], 4096);
    assert_eq!(a.cutoff, a.int.iter().position(|&v| v == 0).map(|i| 2 * i as i32));
    assert_eq!(scale_from_cutoff(4096), 1.0);
    assert!(a.int.iter().zip(&s.int).all(|(x, y)| (x - y).abs() <= 1));
    assert_eq!(round_q12(0.5), 2048);
    assert_eq!(round_q12(-0.5), -2048);
    assert_eq!(round_q12(f32::NAN), i32::MIN);
    assert_eq!(round_q12(1e30), i32::MIN);
}

#[test]
fn assumptions_listed() {
    assert!(!shake_shift::assumptions().is_empty());
}

#[test]
fn fill_matches_memset_for_all_phases() {
    use shake_shift::fill::{fill, FillEnv};
    for sse2 in [true, false] {
        for nt in [i64::MAX, 0x100] {
            let env = FillEnv { sse2, nt_threshold: nt };
            for a in 0..16u64 {
                for cnt in 0..700i64 {
                    let mut x = vec![0xa5u8; 800];
                    let mut y = x.clone();
                    let r = fill(&mut x[..cnt as usize], a, 0x1ff, cnt, &env);
                    assert_eq!(r, a);
                    for b in &mut y[..cnt as usize] {
                        *b = 0xff;
                    }
                    assert_eq!(x, y, "sse2 {} nt {} align {} count {}", sse2, nt, a, cnt);
                }
            }
        }
    }
}
