//! Sine tables, computed from their definitions with the fixed-point number of `exact`.
//!
//! main[j] = (A_j, B_j) for j = 0..=64 with h = pi/128: A_j = cos(j h) h^3 / 6 and B_j = sin(j h) h^2 / 2 (A_64 = 0 exactly), as f64.
//! big[g] for g = 0..=14: the chunk of 128/pi equal to (128/pi) mod 2^(16 - 8 g) (whole value for g = 0, 1), cut into its leading 29
//! significant bits (hi), the next 13 (mid) and the rest (lo, rounded to f64); word 0 packs the top 40 bits of the f64 `hi` with the
//! top 24 bits of the f64 `mid` in its low 24 bits, word 1 is `lo`.
//! tiny_scale = 1 - 2^-30.

use crate::exact::{pi, sincos, Fx};
use std::sync::OnceLock;

#[derive(Clone, Debug, PartialEq)]
pub struct SinTables {
    pub big: [[u64; 2]; 15],
    pub main: [[u64; 2]; 65],
    pub tiny_scale: f64,
}

fn big_chunk(g: u32) -> [u64; 2] {
    let inv_pi = pi().recip();
    let chunk = if g < 2 {
        inv_pi.mul_u64(128)
    } else {
        let p = 7 + 8 * (g - 2);
        let mut f = inv_pi;
        for _ in 0..p {
            f = f.mul_u64(2).frac();
        }
        f.shr(p - 7)
    };
    let chunk_hi = chunk.trunc_sig(29);
    let chunk_mid = chunk.trunc_sig(42);
    let chunk_mid_part = chunk_mid.sub(&chunk_hi);
    let bits_5 = chunk.sub(&chunk_mid);
    let hi_bits = chunk_hi.to_f64().to_bits();
    let mid_bits = chunk_mid_part.to_f64().to_bits();
    assert!(hi_bits & 0xff_ffff == 0 && mid_bits & 0xff_ffff_ffff == 0);
    [(hi_bits & 0xffff_ffff_ff00_0000) | (mid_bits >> 40), bits_5.to_f64().to_bits()]
}

fn compute() -> SinTables {
    let h = pi().div_u64(128);
    let h_sq = h.mul(&h);
    let bits_3 = h_sq.mul(&h);
    let mut main = [[0u64; 2]; 65];
    for (j, slot) in main.iter_mut().enumerate() {
        let (a, b): (Fx, Fx) = if j == 0 {
            (bits_3.div_u64(6), Fx::zero())
        } else if j == 64 {
            (Fx::zero(), h_sq.div_u64(2))
        } else {
            let th = pi().mul_u64(j as u64).div_u64(128);
            let (s, c) = sincos(&th);
            (c.mul(&bits_3).div_u64(6), s.mul(&h_sq).div_u64(2))
        };
        *slot = [a.to_f64().to_bits(), b.to_f64().to_bits()];
    }
    let mut big = [[0u64; 2]; 15];
    for (g, slot) in big.iter_mut().enumerate() {
        *slot = big_chunk(g as u32);
    }
    SinTables { big, main, tiny_scale: 1.0 - f64::from_bits(0x3e10_0000_0000_0000) }
}

impl SinTables {
    /// The computed tables (built once).
    pub fn computed() -> &'static SinTables {
        static T: OnceLock<SinTables> = OnceLock::new();
        T.get_or_init(compute)
    }

    /// Big table as 240 little-endian bytes (cell order of the image).
    pub fn big_bytes(&self) -> Vec<u8> {
        self.big.iter().flat_map(|e| e.iter().flat_map(|w| w.to_le_bytes())).collect()
    }
    /// Main table as 1,040 little-endian bytes.
    pub fn main_bytes(&self) -> Vec<u8> {
        self.main.iter().flat_map(|e| e.iter().flat_map(|w| w.to_le_bytes())).collect()
    }
}
