//! The powf table, the 2/pi limbs and the pi/4 pieces are computed from their definitions. The first three tests compare
//! them with the standard library's own functions within the tolerance stated at each check (relative 2 * 2^-52 for
//! log2(e)/x, absolute 1e-12 for log2, relative 2^-52 for 2^(j/32) and for the pi/4 sums, relative 1e-6 for the top 2/pi
//! limb) and with the structure the routines rely on. Those tolerances leave room for a wrong low bit, so the last test
//! pins every generated word through a 64-bit FNV-1a digest of each table.
use libm_sincos::exact::{pi4_pieces_b, pi4_pieces_s, powf_table, two_over_pi_limbs};

fn f(b: u64) -> f64 {
    f64::from_bits(b)
}

/// Relative closeness: `|a - b| <= k * 2^-52 * |b|`, which is between k and 2k units in the last place of b.
fn rel_close(a: f64, b: f64, k: f64) -> bool {
    (a - b).abs() <= k * f64::EPSILON * b.abs().max(f64::MIN_POSITIVE)
}

#[test]
fn powf_table_matches_std() {
    let t = powf_table();
    for i in 0..=64usize {
        let x = 1.0 + i as f64 / 64.0;
        // log2(e)/x: relative tolerance 2 * 2^-52
        assert!(rel_close(f(t[2 * i]), std::f64::consts::LOG2_E / x, 2.0), "c[{i}]");
        let want = if i < 32 { x.log2() } else { (x / 2.0).log2() };
        // log2: absolute tolerance 1e-12
        assert!((f(t[2 * i + 1]) - want).abs() < 1e-12, "log2[{i}]");
        let q = f(t[2 * i + 1]) * 2f64.powi(44);
        assert_eq!(q, q.round(), "log2[{i}] is a multiple of 2^-44");
    }
    assert_eq!(f(t[0]), std::f64::consts::LOG2_E);
    assert_eq!(f(t[128]), std::f64::consts::LOG2_E / 2.0);
    assert_eq!((t[130], t[131]), (0, 0));
    for j in 0..32usize {
        // 2^(j/32): relative tolerance 2^-52
        assert!(rel_close(f(t[132 + j]), (j as f64 / 32.0).exp2(), 1.0), "exp2[{j}]");
    }
    assert_eq!(f(t[132]), 1.0);
}

#[test]
fn two_over_pi_limbs_have_their_structure_and_top_limb() {
    let l = two_over_pi_limbs();
    assert_eq!(l[0], 0);
    // the first limb alone is 2/pi to 25 bits: relative tolerance 1e-6 (the remaining limbs are not summed here)
    let top = f(l[1]) * 2f64.powi(-201);
    assert!(rel_close(top, 2.0 / std::f64::consts::PI, 1e-6 / f64::EPSILON));
    for k in 1..14usize {
        // exact checks: each limb is an integer below 2^25 times 2^(201-25k)
        let m = f(l[k]) / 2f64.powi(201 - 25 * k as i32);
        assert_eq!(m, m.trunc(), "limb {k} is an integer times 2^(201-25k)");
        assert!(m < 2f64.powi(25));
    }
}

#[test]
fn pi4_pieces_sum_to_pi_over_4() {
    for p in [pi4_pieces_s().to_vec(), pi4_pieces_b().to_vec()] {
        // the pieces added smallest first in f64 equal pi/4 within a relative 2^-52
        let s: f64 = p.iter().rev().map(|&b| f(b)).sum();
        assert!(rel_close(s, std::f64::consts::FRAC_PI_4, 1.0));
        assert!(f(p[0]) > 0.78 && f(p[0]) < 0.79);
        // each piece is smaller than the one before by a factor of at least 1e5
        for w in p.windows(2) {
            assert!(f(w[1]).abs() < f(w[0]).abs() * 1e-5);
        }
    }
}

/// 64-bit FNV-1a over the words of a table, in index order, each as 8 little endian bytes.
fn fnv1a(words: &[u64]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for w in words {
        for b in w.to_le_bytes() {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x100000001b3);
        }
    }
    h
}

const TBL_DIGEST: u64 = 0xa10f45b74d62f5a0;
const PH_TAB_DIGEST: u64 = 0x40ba319ace59e307;
const CW_S_DIGEST: u64 = 0x33da3f2cd2129a0c;
const CW_B_DIGEST: u64 = 0x6fa74b2e1efd0bb9;

/// Exact check: every word of each generated table, through its digest. A single wrong bit in any word changes the digest.
#[test]
fn tables_match_their_digests() {
    let got = [
        ("TBL", fnv1a(&powf_table()), TBL_DIGEST),
        ("PH_TAB", fnv1a(&two_over_pi_limbs()), PH_TAB_DIGEST),
        ("CW_S", fnv1a(&pi4_pieces_s()), CW_S_DIGEST),
        ("CW_B", fnv1a(&pi4_pieces_b()), CW_B_DIGEST),
    ];
    for (name, have, want) in got {
        assert_eq!(have, want, "{name}: digest {have:016x}, expected {want:016x}");
    }
}
