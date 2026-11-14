//! Tables of the vector power routine and of its scalar special-lane routine, computed from their definitions with the exact
//! number type of `exact` (a copy of the shared sine crate's). Nothing is stored.
use crate::exact::{ln2, log2_ratio, Fx};
use std::sync::OnceLock;

fn to_bits_f32(v: &Fx) -> u32 {
    (v.to_f64() as f32).to_bits()
}

/// The multiple of 2^-s nearest to v (ties do not occur for the irrational values used).
fn near_grid(v: &Fx, s: u32) -> Fx {
    let sc = v.abs().mul_u64(1u64 << s);
    let fr = sc.frac();
    let mut fl = sc.sub(&fr);
    if fr.gt(&Fx::from_int(1).shr(1)) {
        fl = fl.add(&Fx::from_int(1));
    }
    let r = fl.shr(s);
    if v.is_neg() { r.neg() } else { r }
}

/// v truncated toward zero to a multiple of 2^-s.
fn trunc_grid(v: &Fx, s: u32) -> Fx {
    let sc = v.abs().mul_u64(1u64 << s);
    let r = sc.sub(&sc.frac()).shr(s);
    if v.is_neg() { r.neg() } else { r }
}

/// 2^(k / d) by the exponential series of k ln 2 / d.
fn exp2_ratio(k: u64, d: u64) -> Fx {
    let y = ln2().mul_u64(k).div_u64(d);
    let mut term = Fx::from_int(1);
    let mut sum = term;
    let mut i = 1u64;
    loop {
        term = term.mul(&y).div_u64(i);
        if term.is_zero() {
            break;
        }
        sum = sum.add(&term);
        i += 1;
    }
    sum
}

/// A value split into a head on a coarse grid and a tail rounded to f32: (head bits, tail bits).
fn split_trunc17(v: &Fx) -> (u32, u32) {
    let hi = trunc_grid(v, 17);
    (to_bits_f32(&hi), to_bits_f32(&v.sub(&hi)))
}

/// Tables of the vector power routine: 127 - log2(1 + k / 8) as head and tail, and 2^(k / 8).
pub struct ATables {
    pub log_head: [u32; 8],
    pub log_tail: [u32; 8],
    pub exp_table: [u32; 8],
}

pub fn atables() -> &'static ATables {
    static V: OnceLock<ATables> = OnceLock::new();
    V.get_or_init(|| {
        let mut t = ATables { log_head: [0; 8], log_tail: [0; 8], exp_table: [0; 8] };
        for k in 0..8u64 {
            let v = Fx::from_int(127).sub(&log2_ratio(8 + k, 8));
            // head: the nearest f32 with its lowest mantissa bit cleared
            let hi = to_bits_f32(&v) & !1;
            t.log_head[k as usize] = hi;
            t.log_tail[k as usize] = ((v.to_f64() - f32::from_bits(hi) as f64) as f32).to_bits();
            t.exp_table[k as usize] = to_bits_f32(&exp2_ratio(4 * k, 32));
        }
        t
    })
}

fn rnd_div(num: u64, den: u64) -> u64 {
    (2 * num + den) / (2 * den)
}

/// Number of words of the computed part of the scalar routine's table (byte offset 0x5a4).
pub const SDATA_WORDS: usize = 0x5a4 / 4;

/// The computed part of the scalar power routine's table, word by word from its base: three reciprocal tables, three pair tables
/// (log2 of the reciprocal split into a head truncated to 2^-17 and an f32 tail) and 128 pairs of 2^(k / 128).
pub fn sdata() -> &'static [u32; SDATA_WORDS] {
    static V: OnceLock<[u32; SDATA_WORDS]> = OnceLock::new();
    V.get_or_init(|| {
        let mut w = [0u32; SDATA_WORDS];
        let fl = |n: u64, d: u64| (n as f32 / d as f32).to_bits();
        // level 1: reciprocal of 1 + i / 8 to 1/128; pairs: log2(1 / r), halved above 1.5
        for i in 0..9u64 {
            let n = rnd_div(1024, 8 + i);
            w[i as usize] = fl(n, 128);
            let v = if 2 * n * 3 > 2 * 256 { log2_ratio(128, n) } else { log2_ratio(64, n) };
            let (h, l) = split_trunc17(&v);
            w[(0x24 / 4 + 2 * i) as usize] = h;
            w[(0x28 / 4 + 2 * i) as usize] = l;
        }
        // level 2: reciprocals 1 - 2i/64 for i = 1, 2 and 1 + (8 - i)/64 above, as n / 64
        for i in 0..9u64 {
            let n: u64 = match i {
                0 | 8 => 64,
                1 | 2 => 64 - 2 * i,
                _ => 64 + (8 - i),
            };
            w[(0x6c / 4 + i) as usize] = fl(n, 64);
            let (h, l) = split_trunc17(&log2_ratio(64, n));
            w[(0x90 / 4 + 2 * i) as usize] = h;
            w[(0x94 / 4 + 2 * i) as usize] = l;
        }
        // level 3: multipliers 1.44140625 / (1 + u) to 1/512, u = i/256 up to i = 4 and (i - 16)/512 above
        for i in 0..17u64 {
            let n = if i <= 4 { rnd_div(738 * 256, 256 + i) } else { rnd_div(738 * 512, 512 + i - 16) };
            w[(0xd8 / 4 + i) as usize] = fl(n, 512);
            let (h, l) = split_trunc17(&log2_ratio(738, n));
            w[(0x11c / 4 + 2 * i) as usize] = h;
            w[(0x120 / 4 + 2 * i) as usize] = l;
        }
        // 2^(k / 128): head on the 2^-22 grid, tail rounded to f32
        for k in 0..128u64 {
            let v = exp2_ratio(k, 128);
            let hi = near_grid(&v, 22);
            w[(0x1a4 / 4 + 2 * k) as usize] = to_bits_f32(&hi);
            w[(0x1a8 / 4 + 2 * k) as usize] = to_bits_f32(&v.sub(&hi));
        }
        w
    })
}

/// The scalar power routine's tables as typed arrays (float bit patterns), cut from the computed block of `sdata` by level:
/// reciprocals of the three reduction levels with their (head, tail) logarithm pairs, and the 128 (head, tail) pairs of 2^(k / 128).
pub struct ScalarTables {
    pub rcp1: [u32; 9],
    pub pair1: [u32; 18],
    pub rcp2: [u32; 9],
    pub pair2: [u32; 18],
    pub rcp3: [u32; 17],
    pub pair3: [u32; 34],
    pub exp2: [u32; 256],
}

fn cut<const N: usize>(w: &[u32], first_word: usize) -> [u32; N] {
    let mut a = [0u32; N];
    a.copy_from_slice(&w[first_word..first_word + N]);
    a
}

pub fn stables() -> &'static ScalarTables {
    static V: OnceLock<ScalarTables> = OnceLock::new();
    V.get_or_init(|| {
        let w = &sdata()[..];
        ScalarTables {
            rcp1: cut(w, 0),
            pair1: cut(w, 0x24 / 4),
            rcp2: cut(w, 0x6c / 4),
            pair2: cut(w, 0x90 / 4),
            rcp3: cut(w, 0xd8 / 4),
            pair3: cut(w, 0x11c / 4),
            exp2: cut(w, 0x1a4 / 4),
        }
    })
}
