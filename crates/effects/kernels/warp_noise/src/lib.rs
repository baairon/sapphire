//! warp_noise (builtin class): native port of the one-channel evaluation reached by a constant node.
//!
//! The class evaluator loops over the elements of its coordinate input and calls the dispatcher, whose AVX2 body (selected when the
//! feature word has all bits of 0x9d97ff) computes, per element, a value noise with 3 x 3 cell taps per octave:
//! cells of size 1 / (2^octave * freq), a hash per cell (the two-argument hash of fx_common), a smooth weight table indexed by squared
//! distance, and a final remap `0.5 * v + 0.5`. The per-point and per-cell memo caches of the original are not modelled: a cold cache
//! is assumed (see `assumptions`).
//!
//! Arithmetic follows the listing: `mul_add` at the vfmadd/vfmsub sites, plain mul and add elsewhere, cvttss2si/cvtss2si semantics for
//! the integer conversions (NaN or out of range gives i32::MIN). The weight table, the weight limit and the hash table are computed
//! (fx_common builders), not copied.
//!
//! A NaN that no operand supplies (the amplitude division and the accumulating fused multiply-add of each octave, when the weight sum is
//! zero) is the x86 default NaN 0xffc00000 on every processor: those two sites go through `fx_common::nanop`, because the hardware default
//! NaN of other processors has the sign bit clear. The tap distances can also make a NaN from infinite coordinates, but such a distance
//! fails the weight-limit comparison, so its bits are never read.

use fx_common::nanop::{fma231, NanOps};
use fx_common::shake_natives::{build_weights_1, hash_2, HashTable};

/// Number of entries of the weight table.
pub const WEIGHT_LEN: usize = 4608;
/// 2^-12.
const Q12: f32 = f32::from_bits(0x3980_0000);
/// 4096.0.
const F4096: f32 = f32::from_bits(0x4580_0000);

/// Squared-distance limit read by both bodies (the scale cell, 9215 / 4096 = 2.2497559, bits 0x400ffc00).
pub const WEIGHT_LIMIT: f32 = f32::from_bits(0x400f_fc00);

/// Tables the evaluation reads: the hash table, the weight table and the weight limit.
pub struct Tables {
    pub hash: [u32; 256],
    pub weights: Vec<f32>,
    /// squared-distance limit (the taps with `limit > d2` count)
    pub limit: f32,
}

impl Tables {
    /// Built as the library initialisers build them.
    pub fn build() -> Tables {
        let h = HashTable::build();
        let w = build_weights_1();
        Tables { hash: h.t, weights: w.float, limit: WEIGHT_LIMIT }
    }
}

/// cvttss2si (truncate; NaN or out of range gives i32::MIN).
#[inline]
pub fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// cvtss2si (round to nearest even; NaN or out of range gives i32::MIN).
#[inline]
pub fn cvt_near(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x.round_ties_even() as i32
    }
}

/// Round half away from zero below 2^23 (add copysign(0.5), truncate), identity above and for NaN; then cvtss2si.
#[inline]
fn cell_of(t: f32) -> i32 {
    let a = f32::from_bits(t.to_bits() & 0x7fff_ffff);
    let r = if a < 8388608.0 {
        let half = f32::from_bits((t.to_bits() & 0x8000_0000) | 0x3f00_0000);
        (t + half).trunc()
    } else {
        t
    };
    cvt_near(r)
}

/// A weight-table index outside the table: the original reads memory outside the table there (a squared distance that is negative
/// or infinite from overflowing coordinates, magnitude above about 1.7e38 after scaling); the port reports it instead.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Oob {
    pub index: i32,
}

/// One element, one output channel, cold caches. `oct` octave count (the caller clamps), `seed` the integer seed, `x`,`y` the scaled
/// coordinates, `persist` the amplitude argument, `out_prev` the value already in the output cell.
pub fn noise1(t: &Tables, oct: i32, seed: i32, x: f32, y: f32, persist: f32, out_prev: f32) -> Result<f32, Oob> {
    let mut out = out_prev;
    let last = oct.wrapping_sub(1);
    if last >= 0 {
        for i in 0..=last {
            let s = ((1u32 << (i as u32)) as i32) as f32;
            let cx = cell_of(x * s);
            let cy = cell_of(y * s);
            let cxf = cx as f32;
            let cyf = cy as f32;
            let (dx, dy) = (s.mul_add(x, -cxf), y.mul_add(s, -cyf));
            let two_dy = dy + dy;
            let dy2 = dy * dy;
            let dx2 = dx * dx;
            let two_dx = dx + dx;
            let (p_dy, m_dy, p_dx, m_dx) = (dy.mul_add(dy, two_dy), dy.mul_add(dy, -two_dy), dx.mul_add(dx, two_dx), dx.mul_add(dx, -two_dx));
            let ax = [p_dx + 1.0, dx2, m_dx + 1.0];
            let ay = [p_dy + 1.0, dy2, m_dy + 1.0];
            let mut sumw = 0.0f32;
            let mut acc = 0.0f32;
            for jy in 0..3usize {
                for ix in 0..3usize {
                    let d2 = ay[jy] + ax[ix];
                    let in_range = t.limit > d2;
                    if !in_range {
                        continue;
                    }
                    let q = cvtt(d2 * F4096);
                    let wi = q >> 1;
                    let w = if wi >= 0 && (wi as usize) < t.weights.len() { t.weights[wi as usize] } else { return Err(Oob { index: wi }) };
                    let cell_x = cx.wrapping_sub(1).wrapping_add(ix as i32);
                    let cell_y = cy.wrapping_sub(1).wrapping_add(jy as i32);
                    let shift = 4;
                    let salt = i ^ seed;
                    let hv = hash_2(&t.hash, ((cell_y as u32) << shift) ^ (salt as u32), cell_x as u32);
                    let mask = 0x1fff;
                    let e = ((hv & mask) as i32).wrapping_add(-4096);
                    let v = (e as f32) * Q12;
                    let prod = w * v;
                    sumw += w;
                    acc = prod + acc;
                }
            }
            let base = if i > 0 {
                let b = if i == last { 1.0f32 } else { persist };
                b / s
            } else {
                persist
            };
            let amp = base.fdiv(sumw);
            out = fma231(out, acc, amp);
        }
    }
    Ok(((out as f64).mul_add(0.5, 0.5)) as f32)
}

/// Evaluator of one element (the class entry loop body): fields of the node object.
#[derive(Clone, Copy, Debug)]
pub struct Node {
    /// frequency lane 0 and 1
    pub freq: [f32; 2],
    /// element count of the frequency argument
    pub freq_count: i32,
    /// octave field (integer)
    pub octaves: i32,
    /// seed lane
    pub seed: f32,
}

/// The coordinate scales the entry derives from the frequency lanes.
pub fn scales(n: &Node) -> (f32, f32) {
    let sx = n.freq[0] * 2.0;
    let sy = if n.freq_count > 1 {
        2.0 * n.freq[1]
    } else {
        sx
    };
    (sx, sy)
}

/// Port of the class entry for one element (the output cell starts at zero).
pub fn eval(t: &Tables, n: &Node, x_in: f32, y_in: f32) -> Result<f32, Oob> {
    let (sx, sy) = scales(n);
    let seed_i = cvtt(F4096 * n.seed);
    let mut oct = if n.octaves >= 8 { 8 } else { n.octaves };
    if oct <= 0 {
        oct = 1;
    }
    let xa = x_in * sx;
    let ya = sy * y_in;
    noise1(t, oct, seed_i, xa, ya, 1.0, 0.0)
}

/// Labelled stand-ins of this port: (label, kind, text).
pub fn assumptions() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("WN-A1", "CACHE-COLD", "the per-point cache (keys and quantised result) and the per-octave cell caches are memos; the port assumes they never hit, so each call recomputes. A hit on the per-point cache returns the quantised value cvtt(4096*out)/4096 and would differ; the initial cache contents are not measured"),
        ("WN-A2", "TABLES-BY-BUILDER", "weight table and hash table come from the fx_common builders (the AVX twin kernel), the weight limit is the scale cell value 2.2497559 (bits 0x400ffc00), read by the body at every tap; weights are zero from index 4537, so the limit only matters at taps whose weight index is below that"),
        ("WN-A3", "FIELD-MAP", "mapping of the program arguments (xy, freq, octaves, seed) to the node fields (freq, count, octave int, seed lane) is read from the entry loop and the argument order; it is INFERRED, not observed on a built node"),
    ]
}

/// Three-channel evaluation (edi = 3 path of the body, one element, cold caches). Same octave loop as `noise1` with three differences:
/// the squared distances use the unfused forms `(1 + 2d) + d*d` and `(1 - 2d) + d*d`, each tap draws three values from one hash word,
/// and the three outputs accumulate together with the shared weight sum. `out_prev` are the values already in the three output cells.
pub fn noise3(t: &Tables, oct: i32, seed: i32, x: f32, y: f32, persist: f32, out_prev: [f32; 3]) -> Result<[f32; 3], Oob> {
    let mut out = out_prev;
    let last = oct.wrapping_sub(1);
    if last >= 0 {
        for i in 0..=last {
            let s = ((1u32 << (i as u32)) as i32) as f32;
            let cx = cell_of(x * s);
            let cy = cell_of(y * s);
            let dx = s.mul_add(x, -(cx as f32));
            let dy = y.mul_add(s, -(cy as f32));
            let dx2 = dx * dx;
            let dy2 = dy * dy;
            let tx = dx + dx;
            let ty = dy + dy;
            let xp = (1.0 + tx) + dx2;
            let xm = (1.0 - tx) + dx2;
            let yp = (1.0 + ty) + dy2;
            let ym = (1.0 - ty) + dy2;
            let ax = [xp, dx2, xm];
            let ay = [yp, dy2, ym];
            let mut sumw = 0.0f32;
            let mut acc = [0.0f32; 3];
            for jy in 0..3usize {
                for ix in 0..3usize {
                    let d2 = ax[ix] + ay[jy];
                    if !(t.limit > d2) {
                        continue;
                    }
                    let q = cvtt(d2 * F4096);
                    let wi = q >> 1;
                    let w = if wi >= 0 && (wi as usize) < t.weights.len() { t.weights[wi as usize] } else { return Err(Oob { index: wi }) };
                    let cell_x = cx.wrapping_sub(1).wrapping_add(ix as i32);
                    let cell_y = cy.wrapping_sub(1).wrapping_add(jy as i32);
                    let hv = hash_2(&t.hash, ((cell_y as u32) << 4) ^ ((i ^ seed) as u32), cell_x as u32);
                    let e0 = ((hv & 0x1fff) as i32).wrapping_add(-4096);
                    let e1 = (((hv >> 16) & 0x1fff) as i32).wrapping_add(-4096);
                    let h2 = hv.wrapping_add(t.hash[(hv & 0xff) as usize]);
                    let e2 = (((h2 >> 8) & 0x1fff) as i32).wrapping_add(-4096);
                    let v = [(e0 as f32) * Q12, (e1 as f32) * Q12, (e2 as f32) * Q12];
                    sumw += w;
                    for c in 0..3 {
                        acc[c] = w * v[c] + acc[c];
                    }
                }
            }
            let base = if i > 0 { (if i == last { 1.0f32 } else { persist }) / s } else { persist };
            let amp = base.fdiv(sumw);
            for c in 0..3 {
                out[c] = fma231(out[c], acc[c], amp);
            }
        }
    }
    Ok([0, 1, 2].map(|c| ((out[c] as f64).mul_add(0.5, 0.5)) as f32))
}

/// Port of the three-channel class entry for one element (outputs start at zero).
pub fn eval3(t: &Tables, n: &Node, x_in: f32, y_in: f32) -> Result<[f32; 3], Oob> {
    let (sx, sy) = scales(n);
    let seed_i = cvtt(F4096 * n.seed);
    let mut oct = if n.octaves >= 8 { 8 } else { n.octaves };
    if oct <= 0 {
        oct = 1;
    }
    noise3(t, oct, seed_i, x_in * sx, sy * y_in, 1.0, [0.0; 3])
}
