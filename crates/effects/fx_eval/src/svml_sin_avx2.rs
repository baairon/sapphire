//! The compile-time `sin` fold on the AVX2/FMA entry of the original library's 8 lane single precision sine.
//!
//! The class kernel of `sin` multiplies the argument by TWO_PI (f32 0x40c90fdb), broadcasts the product into all eight lanes
//! of a ymm register and calls the dispatching stub. On a machine with the AVX2 and FMA feature bits the stub binds the FMA
//! variant (listing 0x136dfc0, dispatch index 9); this module is that variant, lane by lane, in the listing's operation
//! order with `f32::mul_add` exactly at the fused instructions.
//!
//! ROUTE AVX2-ONLY: the native computes the fused variant on every CPU, so its output does not depend on the machine. A CPU
//! without AVX2 and FMA takes the SSE entry (`svml_trig::sinf_svml`), which differs from this route on 10.7 percent of
//! argument bit patterns (at most 2 ulp, including the argument -0.0). The SSE entry stays in `svml_trig` for the cosine fold
//! and as the comparison; it is not used for `sin`.
//!
//! Structure of the routine (all constants are 8 equal lanes in the original):
//! * Fast route, taken when no lane has |x| > 10000.0 (a NaN lane compares false): a one step Cody-Waite reduction by pi with
//!   a three part pi, a degree 11 odd polynomial, and the sign flip for odd quadrants.
//! * Slow route, taken when any lane has |x| > 10000.0 or is infinite: all eight lanes are reduced with a 24 x 96 bit integer
//!   product against a window of 1/(2 pi) selected by the biased exponent, then a 256 entry sine/cosine table lookup and a
//!   short corrected polynomial. Lanes with biased exponent 255 are replaced by the scalar routine's result.
//!
//! The two tables (3072 bytes of windows of 1/(2 pi), 4096 bytes of sine/cosine entries) are computed from the definitions
//! with the exact number type of `libm_sincos`; no table is stored.

use libm_sincos::exact::{pi, sincos, Fx};
use std::sync::OnceLock;

const TWO_PI: f32 = f32::from_bits(0x40c9_0fdb);

// vector constant cells of the routine (each cell holds one value in all eight lanes)
const ABS_MASK: u32 = 0x7fff_ffff;
const LIMIT_BITS: u32 = 0x461c_4000; // 10000.0
const EXP_MASK: u32 = 0x7f80_0000;
const PI_HI: f32 = f32::from_bits(0x4049_0fdb);
const PI_MID: f32 = f32::from_bits(0xb3bb_bd2e);
const PI_LOW: f32 = f32::from_bits(0xa777_2ced);
const C2: f32 = f32::from_bits(0xbe2a_aaa6);
const C3: f32 = f32::from_bits(0x3c08_876a);
const C4: f32 = f32::from_bits(0xb94f_b7ff);
const C5: f32 = f32::from_bits(0x362e_def8);
const INV_PI: f32 = f32::from_bits(0x3ea2_f983);
const ROUND_MAGIC: f32 = f32::from_bits(0x4b40_0000); // 12582912.0
const S0: f32 = f32::from_bits(0xbe2a_aaab); // slow route series coefficients
const S1: f32 = f32::from_bits(0x3c08_885c);
const S2: f32 = f32::from_bits(0xbf00_0000);
const S3: f32 = f32::from_bits(0x3d2a_aa7c);
// immediates of the slow route
const INDEX_MAGIC: f32 = f32::from_bits(0x4740_0000); // 49152.0, rounds the reduced fraction to 1/256
const SPLIT: f32 = f32::from_bits(0xb43b_bd2e);
const SMALL_BITS: u32 = 0x3580_0000; // 2^-20: below it the lookup term is skipped

/// The two tables of the slow route.
pub struct SineTables {
    /// 256 entries of three words: the 96 bits of floor(2^(e-54) / (2 pi)) (mod 2^96), most significant word first
    pub window: Vec<[u32; 3]>,
    /// 256 entries of four words (c0, c1, c2, c3) for the angle 2 pi k / 256: c3 is cos rounded to a power of two (ties up in
    /// the logarithm), c0 = cos - c3, c1 = sin rounded to f32, c2 = sin - c1 rounded to f32
    pub lookup: Vec<[u32; 4]>,
}

fn take32(t: &Fx) -> (u32, Fx) {
    let y = t.mul_u64(1u64 << 32);
    let f = y.frac();
    let ip = y.sub(&f).to_f64() as u32;
    (ip, f)
}

fn window(e: i32) -> [u32; 3] {
    let inv = pi().recip().shr(1); // 1 / (2 pi)
    let s = e - 54;
    // floor(2^s / (2 pi)) mod 2^96 is the first 96 fraction bits of inv * 2^(s - 96)
    let t = if s >= 96 { inv.frac_shl((s - 96) as u32) } else { inv.shr((96 - s) as u32) };
    let (win_first, t) = take32(&t);
    let (diff_1, t) = take32(&t);
    let (win_last, _) = take32(&t);
    [win_first, diff_1, win_last]
}

fn lookup_entry(k: usize) -> [u32; 4] {
    let (q, r) = (k / 64, k % 64);
    let (s, c) = if r == 0 {
        [(0, 1), (1, 0), (0, -1), (-1, 0)][q].pipe(|(a, b)| (Fx::from_int(a), Fx::from_int(b)))
    } else {
        let (s, c) = sincos(&pi().mul_u64(r as u64).div_u64(128));
        match q {
            0 => (s, c),
            1 => (c, s.neg()),
            2 => (s.neg(), c.neg()),
            _ => (c.neg(), s),
        }
    };
    // c3: the cosine as a power of two, rounded in the logarithm, a tie going up
    let cd = c.to_f64();
    let c_rounded: f32 = if cd == 0.0 {
        0.0
    } else {
        let b = cd.to_bits();
        let mut e = ((b >> 52) & 0x7ff) as i32;
        // 0x6a09e667f3bcd is the mantissa field of the double nearest to 2^-1/2
        if (b & ((1u64 << 52) - 1)) >= 0x6_a09e_667f_3bcd {
            e += 1;
        }
        f32::from_bits((((e - 1023 + 127) as u32) << 23) | if cd < 0.0 { 0x8000_0000 } else { 0 })
    };
    let c_residual = c.sub(&Fx::from_f64(c_rounded as f64)).to_f32_scaled(0);
    let c1 = s.to_f32_scaled(0);
    let c2 = s.sub(&Fx::from_f64(c1 as f64)).to_f32_scaled(0);
    [c_residual.to_bits(), c1.to_bits(), c2.to_bits(), c_rounded.to_bits()]
}

trait Pipe: Sized {
    fn pipe<R>(self, f: impl FnOnce(Self) -> R) -> R {
        f(self)
    }
}
impl<T> Pipe for T {}

/// The computed tables (built once).
pub fn tables() -> &'static SineTables {
    static T: OnceLock<SineTables> = OnceLock::new();
    T.get_or_init(|| SineTables {
        window: (0..256).map(window).collect(),
        lookup: (0..256).map(lookup_entry).collect(),
    })
}

/// Little endian bytes of the window table (12 bytes per biased exponent: the three words, least significant word of
/// the 96 bit value last in memory order as the routine reads them, i.e. word 0 first).
pub fn window_bytes() -> Vec<u8> {
    tables().window.iter().flat_map(|w| w.iter().flat_map(|x| x.to_le_bytes())).collect()
}

/// Little endian bytes of the lookup table (16 bytes per entry).
pub fn lookup_bytes() -> Vec<u8> {
    tables().lookup.iter().flat_map(|w| w.iter().flat_map(|x| x.to_le_bytes())).collect()
}

#[inline]
fn f(u: u32) -> f32 {
    f32::from_bits(u)
}

/// One lane of the fast route (the fma sequence at 0x136e058 to 0x136e0b1). A NaN lane keeps its payload, quieted, and its sign.
fn fast_lane(x: u32) -> u32 {
    let a = x & ABS_MASK;
    let s = x & 0x8000_0000;
    if a > EXP_MASK {
        return x | 0x0040_0000;
    }
    let af = f(a);
    let t = af.mul_add(INV_PI, ROUND_MAGIC); // 0x136e058
    let n = t - ROUND_MAGIC;
    let odd = t.to_bits() << 31;
    let mut r = (-n).mul_add(PI_HI, af); // 0x136e066
    r = (-n).mul_add(PI_MID, r); // 0x136e06b
    r = (-n).mul_add(PI_LOW, r); // 0x136e074
    let q = r * r;
    let mut p = C5.mul_add(q, C4); // 0x136e089
    p = p.mul_add(q, C3); // 0x136e092
    p = p.mul_add(q, C2); // 0x136e09b
    let p = p * q;
    let u = f(r.to_bits() ^ odd);
    let v = p.mul_add(u, u); // 0x136e0ac
    v.to_bits() ^ s
}

/// One lane of the slow route (0x136e156 to 0x136e6f5), before the replacement of exponent 255 lanes.
fn slow_lane(x: u32, tab: &SineTables) -> u32 {
    let sg = x & 0x8000_0000;
    let e = ((x & EXP_MASK) >> 23) as usize;
    let [win_first, diff_1, win_last] = tab.window[e];
    let m = (x & 0x007f_ffff).wrapping_add(0x0080_0000);
    let (mlo, mhi) = (m & 0xffff, m >> 16);
    let (w0lo, w0hi) = (win_first & 0xffff, win_first >> 16);
    let (w1lo, w1hi) = (diff_1 & 0xffff, diff_1 >> 16);
    let (w2lo, w2hi) = (win_last & 0xffff, win_last >> 16);
    const M: u32 = 0xffff;
    // 24 x 96 bit product in 16 bit limbs (0x136e2ef to 0x136e3f8), 32 bit wrapping multiplies
    let cross_hi = mhi.wrapping_mul(w2lo) >> 16;
    let mut prod_11 = mhi.wrapping_mul(w2hi);
    let cross_lo = mlo.wrapping_mul(w2hi) >> 16;
    let fp33_sub_1 = mlo.wrapping_mul(w1lo);
    let mut low_acc = fp33_sub_1 & M;
    prod_11 = prod_11.wrapping_add(cross_lo);
    let d1h = fp33_sub_1 >> 16;
    low_acc = low_acc.wrapping_add(prod_11);
    let prod_3 = mlo.wrapping_mul(w1hi);
    let prod_2 = mhi.wrapping_mul(w1lo);
    let sum_11 = cross_hi.wrapping_add(low_acc);
    let mut low_acc_b = prod_3 & M;
    let mid_sum = prod_2.wrapping_add(d1h);
    let carry_mid = sum_11 >> 16;
    let bits_11 = sum_11 & M;
    low_acc_b = low_acc_b.wrapping_add(mid_sum);
    low_acc_b = carry_mid.wrapping_add(low_acc_b);
    let prod_15 = mhi.wrapping_mul(w1hi);
    let carry_lo = prod_3 >> 16;
    let flag_0 = low_acc_b >> 16;
    let fused_5 = low_acc_b << 16;
    let carry_sum = prod_15.wrapping_add(carry_lo);
    let route_int_7 = mlo.wrapping_mul(w0lo);
    let masked_route = route_int_7 & M;
    let carry_total = masked_route.wrapping_add(carry_sum);
    let flag_3 = route_int_7 >> 16;
    let top_sum = flag_0.wrapping_add(carry_total);
    let prod_w0 = mlo.wrapping_mul(w0hi);
    let prod_7 = mhi.wrapping_mul(w0lo);
    let bits_10 = prod_w0 & M;
    let sum_tail = prod_7.wrapping_add(flag_3);
    let flag_15 = top_sum >> 16;
    let bits_1 = top_sum & M;
    let sum_3 = fused_5.wrapping_add(bits_11);
    let sum_8 = bits_10.wrapping_add(sum_tail);
    let sum_15 = flag_15.wrapping_add(sum_8);
    let bits_4 = sg ^ 0x3f80_0000;
    let ag6 = sum_15 << 16;
    let ah8 = ag6.wrapping_add(bits_1);
    let ai1 = sg ^ 0x2880_0000;
    let ak5 = (ah8 >> 9) | bits_4;
    let ao6 = ((sum_3 & 0x3_ffff) << 5) | ai1;
    // the reduced fraction as a float pair
    let sum_9 = INDEX_MAGIC + f(ak5);
    let table_val_0 = f(ao6) - f(ai1);
    let diff_11 = sum_9 - INDEX_MAGIC;
    let idx = (sum_9.to_bits() & 0xff) as usize;
    let table_val_7 = f(ak5) - diff_11;
    let bits_5 = sg ^ 0x3400_0000;
    let flag_4 = (((ah8 & 0x1ff) << 14) | (sum_3 >> 18)) | bits_5;
    let table_diff = f(flag_4) - f(bits_5);
    let table_sum = table_val_7 + table_diff;
    let diff_7_b = table_val_7 - table_sum;
    let bits_10 = table_diff + diff_7_b;
    let mut sum_1 = bits_10 + table_val_0;
    let hi = table_sum * TWO_PI;
    let err = table_sum.mul_add(TWO_PI, -hi); // 0x136e4a2
    let mid = table_sum.mul_add(SPLIT, err); // 0x136e4ad
    sum_1 = sum_1.mul_add(TWO_PI, mid); // 0x136e4b2
    // small arguments keep x itself; the others the (hi, lo) pair
    let a = f(x & ABS_MASK);
    let gt = a > f(SMALL_BITS);
    let le = a <= f(SMALL_BITS);
    let lo = if gt { sum_1 } else { 0.0 };
    let r = f((if le { x } else { 0 }) | (if gt { hi.to_bits() } else { 0 }));
    let r2 = r * r;
    let t = tab.lookup[idx];
    let (tab_a, tab_b, sum_2, table_val_3) = (f(t[0]), f(t[1]), f(t[2]), f(t[3]));
    let poly_lo = S1.mul_add(r2, S0); // 0x136e614
    let p1r = poly_lo * r2;
    let fused_4 = table_val_3.mul_add(r, tab_b); // 0x136e646
    let sum_03 = tab_a + table_val_3;
    let diff_5 = tab_b - fused_4;
    let fused_7 = r.mul_add(tab_a, fused_4); // 0x136e658
    let diff_5 = table_val_3.mul_add(r, diff_5); // 0x136e65d
    let diff_8 = fused_4 - fused_7;
    let diff_8 = r.mul_add(tab_a, diff_8); // 0x136e676
    let diff_8 = diff_8 + diff_5;
    let p2 = S3.mul_add(r2, S2); // 0x136e68e
    let prod_5_b = r * p1r;
    let poly_c = (-tab_b).mul_add(r, sum_03); // 0x136e69c
    let prod_13 = r2 * p2;
    let fused_5_c = poly_c.mul_add(prod_5_b, diff_8); // 0x136e6d8
    let fused_10 = poly_c.mul_add(lo, sum_2); // 0x136e6e3
    let prod_13 = tab_b.mul_add(prod_13, fused_10); // 0x136e6e8
    let sum_3_b = prod_13 + fused_5_c;
    (fused_7 + sum_3_b).to_bits()
}

/// The scalar routine 0x135ce60 for a lane with biased exponent 255: infinity times zero (the default NaN), a NaN squared.
fn scalar_lane(x: u32) -> u32 {
    if x & ABS_MASK == EXP_MASK {
        0xffc0_0000
    } else {
        x | 0x0040_0000
    }
}

/// The 8 lane routine: eight argument bit patterns in, eight results out.
pub fn sin8(x: [u32; 8]) -> [u32; 8] {
    // vcmpgt_oqps against 10000.0: false for NaN lanes
    let slow = x.iter().any(|&u| {
        let a = u & ABS_MASK;
        a > LIMIT_BITS && a <= EXP_MASK
    });
    if !slow {
        return x.map(fast_lane);
    }
    let tab = tables();
    x.map(|u| if u & EXP_MASK == EXP_MASK { scalar_lane(u) } else { slow_lane(u, tab) })
}

/// The fold of `sin(x)` for a constant argument in revolutions: TWO_PI * x broadcast to eight lanes, lane 0.
pub fn sin_rev(x: f32) -> f32 {
    let v = (TWO_PI * x).to_bits();
    f(sin8([v; 8])[0])
}

/// Routine on a broadcast argument that is already multiplied by TWO_PI.
pub fn sinf_avx2(x: f32) -> f32 {
    f(sin8([x.to_bits(); 8])[0])
}

/// The vector constant cells of the routine as (byte offset in the 64 KiB table image, value bits); every cell holds the value in
/// all eight lanes (32 bytes).
pub fn constant_cells() -> Vec<(usize, u32)> {
    vec![
        (0x1000, ABS_MASK),
        (0x1040, LIMIT_BITS),
        (0x1080, EXP_MASK),
        (0x10c0, S0.to_bits()),
        (0x1100, S1.to_bits()),
        (0x1140, S2.to_bits()),
        (0x1180, S3.to_bits()),
        (0x12c0, PI_HI.to_bits()),
        (0x1300, PI_MID.to_bits()),
        (0x1340, PI_LOW.to_bits()),
        (0x1380, C2.to_bits()),
        (0x13c0, C3.to_bits()),
        (0x1400, C4.to_bits()),
        (0x1440, C5.to_bits()),
        (0x1480, INV_PI.to_bits()),
        (0x14c0, ROUND_MAGIC.to_bits()),
    ]
}

/// The stand-in labels of this module.
pub fn assumptions() -> &'static [&'static str] {
    &[
        "ROUTE AVX2-ONLY: the fused variant on every CPU; the SSE entry differs on 10.7 percent of arguments (not ported for sin)",
        "SINE-NAN-LANE: a NaN lane of the fast route is the quieted input (all operands carry the same payload; sign from the input)",
        "SINE-TABLES-COMPUTED: window and lookup tables are computed; bytes of the 64 KiB image beyond them are never read by the route",
    ]
}
