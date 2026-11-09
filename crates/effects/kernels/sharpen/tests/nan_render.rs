//! Whole-render NaN behaviour of sharpen: the output bits are the same on every processor with no rewrite of the blurred plane or of
//! the combine result. The digest line is compared between x86_64 and aarch64 (qemu) by the gate logs.
use fx_common::plane::{Plane, PlaneMut};
use sharpen::{render_alias, Params};

fn lcg(s: &mut u64) -> u32 {
    *s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    (*s >> 33) as u32
}

/// family 0: finite plus infinities (fresh NaN in the blur); 1: 0x7fc00000 words in the source; 2: a NaN with other bits in a few
/// pixels (a NaN only in the blur neighbourhood of most outputs); 3: both signs, signalling and quiet payloads.
fn plane(fam: usize, n: usize, seed: u64) -> Vec<f32> {
    let mut s = seed;
    (0..n)
        .map(|_| {
            let r = lcg(&mut s);
            let v = (r % 2001) as f32 / 1000.0 - 0.5;
            match (fam, r % 23) {
                (0, 0) => f32::INFINITY,
                (0, 1) => f32::NEG_INFINITY,
                (1, 0) => f32::from_bits(0x7fc0_0000),
                (1, 1) => f32::INFINITY,
                (2, 0) => f32::from_bits(0xffc0_0000 | (r & 0x3fffff)),
                (3, 0) => f32::from_bits(0x7f80_0000 | (r & 0x7fffff) | 1),
                (3, 1) => f32::from_bits(0xff80_0000 | (r & 0x7fffff) | 1),
                (3, 2) => f32::from_bits(0x7fc0_0000 | (r & 0x3fffff)),
                _ => v,
            }
        })
        .collect()
}

fn fnv(h: &mut u64, v: u32) {
    for k in 0..4 {
        *h ^= ((v >> (8 * k)) & 0xff) as u64;
        *h = h.wrapping_mul(0x100000001b3);
    }
}

#[test]
fn the_render_digest_is_printed_for_every_family_gain_and_alias_mode() {
    let mut h = 0xcbf29ce484222325u64;
    let mut nan_words = 0usize;
    let mut fresh_ok = 0usize;
    for fam in 0..4 {
        for &(w, hh) in &[(17usize, 13usize), (48, 9)] {
            let n = w * hh;
            for gain in [1.0f32, 0.5, 2.0] {
                for alias in [false, true] {
                    let src = plane(fam, n, 17 + fam as u64 * 31 + w as u64);
                    let planes = vec![Plane::new(&src, w, hh, w).unwrap()];
                    let mut out = vec![0f32; n];
                    {
                        let mut dst = vec![PlaneMut::new(out.as_mut_slice(), w, hh, w).unwrap()];
                        let p = Params { amount: 0.6, gains: vec![gain], in_count: 1, ctx_width: 64, aspect: None, count: None };
                        render_alias(&p, &planes, &mut dst, alias).unwrap();
                    }
                    for (i, v) in out.iter().enumerate() {
                        fnv(&mut h, v.to_bits());
                        if v.is_nan() {
                            nan_words += 1;
                            if !src.iter().any(|x| x.is_nan()) {
                                assert_eq!(v.to_bits(), 0xffc0_0000, "fresh NaN at {i}");
                                fresh_ok += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    println!("SHARPEN_RENDER_DIGEST {h:016x} nan_words {nan_words} fresh_ok {fresh_ok}");
    assert!(nan_words > 0 && fresh_ok > 0);
}
