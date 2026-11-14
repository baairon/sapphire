//! libm_sincos: native port of the shared libm routine (INFERRED single precision sincosf), listing neutral_1.asm.
//!
//! Contract (MEASURED from the listing): argument f32 in xmm0, result r0 = sin in xmm0 and r1 = cos in xmm1 (lane 0 of the
//! table walk reads entry k, lane 1 reads entry k + 16, a quarter turn). Legacy SSE2 only: no VEX, no FMA, no x87 and no
//! approximate instruction on any route .
//! Routes: |x| < 8192 (normal) single f32 table walk; zero and denormal: sin = x * K, cos = 1 - x * x; inf and NaN: x * 0 in both
//! lanes; |x| >= 8192 finite: the f32 is widened and handed to the double precision sincos, whose result is narrowed with
//! cvtsd2ss. itself uses a table walk for |x| < 90112 and calls the argument reducer above that (Cody-Waite
//! below 2^30, a Payne-Hanek style limb product above).
//!
//! Tables: the sincosf table (64 entries of (sin, cos) of k pi/32), the double table (64 entries of 64 bytes), the Cody-Waite pieces of
//! pi/4 (`CW_S`, `CW_B`) and the 2/pi limbs of the large argument reducer (`PH_TAB`) are computed at first use from their definitions
//! (module `exact`: 320 bit fixed point, Machin's pi, Taylor series). No file and no captured value is read for them. The checks of
//! tests/vs_copy.rs (copies under VEC_DIR) compare the first two tables bit for bit with stored copies. The remaining constants (scalars and cells of at most 2 words) are literals.

pub mod exact;

use std::sync::{LazyLock, OnceLock};

fn f(b: u64) -> f64 {
    f64::from_bits(b)
}

// ---- captured constants (listing offsets in the comments, VA = offset)
const K1: f32 = f32::from_bits(0x4122f983); //  low dword, 32/pi (f32)
const SHIFTER32: f32 = f32::from_bits(0x4b400000); // 1.5 * 2^23
const A_HI: u64 = 0x77b9_21fb_5444_4000; // 
const B_LO: u64 = 0x7532_e7b9_6767_33af; // 
const SIGN32_MASK: u64 = 0x0000_0000_8000_0000; //  low qword
const ONE: f64 = 1.0; // 
const C_COS2: f64 = f64::from_bits(0xbfe0000000000000); // 
const C_SIN2: f64 = f64::from_bits(0xbfc5555555555555); // 
const C_COS4: f64 = f64::from_bits(0x3fa5555555555555); // 
const C_SIN4: f64 = f64::from_bits(0x3f81111111111111); // 
const TINY_K: f64 = f64::from_bits(0x3fefffffff800000); // 

//  (double sincos)
const KD: f64 = f64::from_bits(0x40245f306dc9c883); // 32/pi
const SHD: f64 = f64::from_bits(0x4338000000000000); // 1.5 * 2^52
const DP1: f64 = f64::from_bits(0x3fb921fb54400000); // pi/32 high
const DP2: f64 = f64::from_bits(0x3d90b4611a600000); // pi/32 middle
const DP3: f64 = f64::from_bits(0x3b63198a2e037073); // pi/32 low
const C_CELL_E: [u64; 2] = [0x3ec71de3a556c734, 0x3efa01a01a01a01a]; //  (1/9!, 1/8!)
const C_CELL_D: [u64; 2] = [0x3f81111111111111, 0x3fa5555555555555]; //  (1/5!, 1/4!)
const C_CELL_C: [u64; 2] = [0xbf2a01a01a01a01a, 0xbf56c16c16c16c17]; //  (-1/7!, -1/6!)
const C_CELL_B: [u64; 2] = [0xbfc5555555555555, 0xbfe0000000000000]; //  (-1/3!, -1/2!)

//  (argument reducer)
const FOUR_OVER_PI: f64 = f64::from_bits(0x3ff45f306dc9c883); // 
const SIGN_TAB: [u64; 2] = [0x3ff0000000000000, 0xbff0000000000000]; // 
//  CW_S and CW_B: pi/4 split into Cody-Waite pieces, computed from pi at 320 bits by exact::pi4_pieces_s and exact::pi4_pieces_b (widths and roundings are the method's parameters).
static CW_S: LazyLock<[u64; 4]> = LazyLock::new(exact::pi4_pieces_s);
static CW_B: LazyLock<[u64; 5]> = LazyLock::new(exact::pi4_pieces_b);
static PH_TAB: LazyLock<[u64; 14]> = LazyLock::new(exact::two_over_pi_limbs);
const PH_C_CELL_A: [u64; 2] = [0x0000000000000000, 0xbff0000000000000]; // 
const PH_C_CELL_B: u64 = 0x4198000000000000; // 
const PH_C_CELL_C: u64 = 0x3fe921fb40000000; // 
const PH_C_CELL_D: u64 = 0x3e64442d18469899; // 

/// Table storage (computed).
pub struct Tables {
    pub s: [[u64; 2]; 64],
    pub d: [[u64; 8]; 64],
}

fn build_tables() -> Tables {
    let mut t = Tables { s: [[0; 2]; 64], d: [[0; 8]; 64] };
    for k in 0..64 {
        t.s[k] = exact::stab_entry(k);
        t.d[k] = exact::dtab_entry(k);
    }
    t
}

/// The two tables, computed once.
pub fn tables() -> &'static Tables {
    static T: OnceLock<Tables> = OnceLock::new();
    T.get_or_init(build_tables)
}

/// A recorded assumption.
#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub text: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "SC1-A1", text: "sincosf table: entry k is the correctly rounded (sin, cos) of k pi/32, exact 0 and 1 at the axes, computed at first use (module exact). MEASURED on x86_64 Linux: all 64 entries equal the plugin file bit for bit (tests/vs_copy.rs, copies under VEC_DIR)." },
        Assumption { id: "SC1-A2", text: "double sincos table (64 B entries): computed at first use. Words 2 and 3 are the correctly rounded sin and cos of k pi/32, words 4 and 5 their rounding tails rounded to 24 bits, words 6 and 7 cos and -sin rounded to one bit, words 0 and 1 the exact remainders cos - w6 and -sin - w7 rounded to double. MEASURED on x86_64 Linux: all 64 entries equal the plugin file bit for bit. Only reached by finite |x| >= 8192." },
        Assumption { id: "SC1-A3", text: "MXCSR (round to nearest even, no DAZ or FTZ); Rust f32/f64 arithmetic is IEEE nearest even." },
        Assumption { id: "SC1-A4", text: "NaN results follow the silicon rule: x * 0 with x NaN returns x quieted (bit 22 set), payload and sign kept; x infinite returns the default NaN 0xffc00000 in both lanes. The emulator follows the SDM, not silicon." },
        Assumption { id: "SC1-A5", text: "Routine is called with the SSE2 baseline and no CPU feature dispatch (MEASURED: no feature word read in). Upper bits of xmm0/xmm1 are not modelled; only the low f32 of each result is returned. Callers read r0 and r1 as f32." },
        Assumption { id: "SC1-A6", text: "Class identity (sincosf) is INFERRED from the shape (quarter turn table walk, 32/pi reduction); the port is a body-level port, not a proof of the library function." },
    ]
}

/// Which route a call took (for coverage reports and mutation tests).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Route {
    Fast,
    Tiny,
    NonFinite,
    DoubleMid,
    DoubleCw,
    DoublePh,
}

#[derive(Clone, Debug)]
pub struct Trace {
    pub route: Route,
    /// sincosf table entries read (lane 0, lane 1)
    pub s_idx: Option<(usize, usize)>,
    /// double table entry read by 
    pub d_idx: Option<usize>,
    /// quadrant returned by the reducer
    pub quad: Option<u32>,
    /// limb table base index of the Payne-Hanek step
    pub reduce_step: Option<usize>,
}

fn tr(route: Route) -> Trace {
    Trace { route, s_idx: None, d_idx: None, quad: None, reduce_step: None }
}

/// cvtss2si (round to nearest even, 0x80000000 when out of range or NaN)
fn cvtss2si(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x.round_ties_even() as i32
    }
}

/// cvtsd2si 32 bit
fn cvtsd2si(x: f64) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x.round_ties_even() as i32
    }
}

fn nan_x0(x: f32) -> f32 {
    // mulss x, 0.0 on silicon: a NaN first operand comes back quieted, inf * 0 gives the default NaN
    if x.is_nan() {
        f32::from_bits(x.to_bits() | 0x0040_0000)
    } else {
        f32::from_bits(0xffc0_0000)
    }
}

/// (sin x, cos x) as the original computes them, plus the route taken.
pub fn sincosf_traced(x: f32) -> (f32, f32, Trace) {
    let bits = x.to_bits();
    let biased_top = (((bits >> 16) & 0x7fff) as u16).wrapping_sub(0x80);
    if biased_top <= 0x457f {
        return fast(x, bits);
    }
    if (biased_top as i16) <= 0x457f {
        // zero and denormal
        let xd = x as f64;
        let s = (xd * TINY_K) as f32;
        let c = (ONE - xd * xd) as f32;
        return (s, c, tr(Route::Tiny));
    }
    if (bits & 0x7f80_0000) == 0x7f80_0000 {
        let v = nan_x0(x);
        return (v, v, tr(Route::NonFinite));
    }
    let (s, c, t) = sincos_d(x as f64);
    (s as f32, c as f32, t)
}

pub fn sincosf_x(x: f32) -> (f32, f32) {
    let (s, c, _) = sincosf_traced(x);
    (s, c)
}

fn fast(x: f32, bits: u32) -> (f32, f32, Trace) {
    let tb = tables();
    let t = K1 * x;
    let copy_mode = cvtss2si(t);
    let rt = (t + SHIFTER32) - SHIFTER32;
    let xd = x as f64;
    let m = (bits as u64) & SIGN32_MASK;
    let sg = (m << 32) ^ (m << 29);
    let a = f(A_HI ^ sg);
    let b = f(B_LO ^ sg);
    let rd = f((rt.to_bits() as u64) << 29);
    let flag_0 = (((copy_mode as u32).wrapping_add(0x1c7600) << 4) as u64 & 0x3f0) as usize >> 4;
    let flag_1 = (((copy_mode as u32).wrapping_add(0x1c7610) << 4) as u64 & 0x3f0) as usize >> 4;
    let r = xd - a * rd;
    let w = b * rd + r;
    let r2 = r * r;
    let p = (C_COS2 * r2 + ONE) + C_COS4 * (r2 * r2);
    let q = (C_SIN2 * r2 + ONE) + C_SIN4 * (r2 * r2);
    let sin_row = tb.s[flag_0];
    let cos_row = tb.s[flag_1];
    // lane 0: table sin at k, table cos at k; lane 1: table sin at k + 16, table cos at k + 16
    let table_val_0 = f(sin_row[0]) * p + (w * f(sin_row[1])) * q;
    let table_val_1 = f(cos_row[0]) * p + (w * f(cos_row[1])) * q;
    let mut trc = tr(Route::Fast);
    trc.s_idx = Some((flag_0, flag_1));
    (table_val_0 as f32, table_val_1 as f32, trc)
}

fn mul2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] * b[0], a[1] * b[1]]
}
fn add2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] + b[0], a[1] + b[1]]
}
fn sub2(a: [f64; 2], b: [f64; 2]) -> [f64; 2] {
    [a[0] - b[0], a[1] - b[1]]
}
fn c2(c: [u64; 2]) -> [f64; 2] {
    [f(c[0]), f(c[1])]
}
fn split_pair(v: f64) -> [f64; 2] {
    [v, v]
}

///  for finite arguments inside the f32 range |x| >= 8192 (the tiny branch of the original is not reachable from an f32).
/// Returns (sin, cos) as doubles.
pub fn sincos_d(x: f64) -> (f64, f64, Trace) {
    let hi16 = ((x.to_bits() >> 48) & 0x7fff) as u16;
    let biased_top = hi16.wrapping_sub(0x3030);
    if biased_top <= 0x10c5 {
        let (s, c, idx) = core_d(x, None);
        let mut t = tr(Route::DoubleMid);
        t.d_idx = Some(idx);
        return (s, c, t);
    }
    // large (the tiny and inf/NaN branches cannot be reached from the f32 wrapper; inf/NaN would return x * 0)
    let e = ((x.to_bits() >> 48) & 0x7ff0) as u16;
    if e == 0x7ff0 {
        let v = x * 0.0;
        return (v, v, tr(Route::NonFinite));
    }
    let (yh, yl, q, small, step_out) = reduce_x(x);
    let (s, c, idx) = core_d(yh, Some((yl, q)));
    let mut t = tr(if small { Route::DoubleCw } else { Route::DoublePh });
    t.d_idx = Some(idx);
    t.quad = Some(q);
    t.reduce_step = step_out;
    (s, c, t)
}

/// The table walk shared by the mid and large routes of (and).
fn core_d(x: f64, red: Option<(f64, u32)>) -> (f64, f64, usize) {
    let tb = tables();
    let t = KD * x;
    let copy_mode = cvtsd2si(t);
    let n = (t + SHD) - SHD;
    let p1n = DP1 * n;
    let quadrant_bias = match red {
        Some((_, q)) => (q as u64) << 4,
        None => 0,
    };
    let idx = ((copy_mode as u32 as u64 + 0x1c7600 + quadrant_bias) & 0x3f) as usize;
    let e = &tb.d[idx];
    let p0 = [f(e[0]), f(e[1])];
    let e_pair = [f(e[2]), f(e[3])];
    let p2 = [f(e[4]), f(e[5])];
    let p3 = [f(e[6]), f(e[7])];
    let p2n = DP2 * n;
    let sum_1 = x - p1n;
    let p3n = n * DP3;
    let reduced = sum_1 - p2n;
    let mut poly_a = mul2(c2(C_CELL_E), split_pair(sum_1));
    let mut prod_hi7 = mul2(e_pair, split_pair(reduced));
    let mut diff_3 = sum_1 - reduced;
    poly_a = mul2(poly_a, split_pair(reduced));
    let r2 = reduced * reduced;
    diff_3 -= p2n;
    let mut bits_1 = p3n - diff_3;
    let aa = add2(p0, p3);
    prod_hi7 = sub2(prod_hi7, aa);
    if let Some((ylo, _)) = red {
        bits_1 -= ylo;
    }
    let m1v = mul2(split_pair(bits_1), prod_hi7);
    let hi7 = e_pair;
    let sq_prod = mul2(aa, split_pair(reduced));
    let mut poly_b = mul2(c2(C_CELL_D), split_pair(r2));
    let cubic_prod = mul2(p3, split_pair(reduced));
    let sq_prod = mul2(sq_prod, split_pair(r2));
    let hi7 = mul2(hi7, split_pair(r2));
    let prod_4 = r2 * r2;
    poly_a = add2(poly_a, c2(C_CELL_C));
    let const_prod = mul2(split_pair(reduced), p0);
    poly_b = add2(poly_b, c2(C_CELL_B));
    poly_a = mul2(poly_a, split_pair(prod_4));
    let cubic_prod_saved = cubic_prod;
    let sum_3_b = add2(cubic_prod, e_pair);
    poly_b = add2(poly_b, poly_a);
    let prod_6 = mul2(split_pair(poly_b[1]), hi7);
    let sum_2 = mul2(sq_prod, split_pair(poly_b[0]));
    let copy_7_b = const_prod;
    let sum_4_b = add2(const_prod, sum_3_b);
    let diff_5 = sub2(e_pair, sum_3_b);
    let diff_3_c = sub2(sum_3_b, sum_4_b);
    let mut m = add2(m1v, p2);
    let diff_5 = add2(diff_5, cubic_prod_saved);
    let sum_3_d = add2(diff_3_c, copy_7_b);
    m = add2(m, diff_5);
    m = add2(m, sum_3_d);
    m = add2(m, sum_2);
    m = add2(m, prod_6);
    m = add2(m, sum_4_b);
    (m[0], m[1], idx)
}

/// reduce |x| to (hi, lo) about a multiple of pi/4 with the sign applied, plus the quadrant.
/// Returns (hi, lo, quadrant, cody_waite_route, ph_table_base).
fn reduce_x(xin: f64) -> (f64, f64, u32, bool, Option<usize>) {
    let bits = xin.to_bits();
    let quadrant = (bits >> 63) as u32;
    let a_bits = bits & !(1u64 << 63);
    let a = f(a_bits);
    let copy_mode = ((a_bits >> 52) & 0x7ff) as u32;
    if copy_mode >= 0x41d {
        let (h, l, q, step_out) = reduce_ph(a_bits, copy_mode, quadrant);
        return (h, l, q, false, Some(step_out));
    }
    let tt = FOUR_OVER_PI * a;
    let tb = tt.to_bits();
    let flag = ((tb >> 52) & 0x7ff) as u32;
    let tap_b: u32 = if flag >= 0x3ff {
        let hi = (tb >> 32) as u32;
        let lo = tb as u32;
        let shift_count = 0x1eu32.wrapping_sub(flag) & 31;
        let s = ((hi | 0xfff0_0000) << 11) | (lo >> 21);
        s >> shift_count
    } else {
        0
    };
    let mi = tap_b.wrapping_add(tap_b & 1);
    let half = tap_b.wrapping_add(1) >> 1;
    let q = (1u32.wrapping_sub(2 * quadrant)).wrapping_mul(half) & 3;
    let m = (mi as i32) as f64;
    let (xh, xl);
    if copy_mode < 0x416 {
        let sum_1 = f(CW_S[0]) * m;
        let cw_hi = f(CW_S[1]) * m;
        let mut rem = a - sum_1;
        let mut rem_b = rem;
        let cw_lo = f(CW_S[2]) * m;
        rem_b -= cw_hi;
        rem -= rem_b;
        let prod_7 = m * f(CW_S[3]);
        rem -= cw_hi;
        let mut rem_c = rem_b;
        rem_c -= cw_lo;
        rem_b -= rem_c;
        rem_b -= cw_lo;
        rem += rem_b;
        rem -= prod_7;
        xh = rem_c;
        xl = rem;
    } else {
        let sum_1 = f(CW_B[0]) * m;
        let cw_hi = f(CW_B[1]) * m;
        let mut rem = a - sum_1;
        let mut rem_b = rem;
        let cw_lo = f(CW_B[2]) * m;
        rem_b -= cw_hi;
        rem -= rem_b;
        let mut prod_8 = rem_b;
        rem -= cw_hi;
        prod_8 -= cw_lo;
        let mut acc_a = prod_8;
        rem_b -= prod_8;
        let table_val_6 = f(CW_B[4]);
        rem_b -= cw_lo;
        rem += rem_b;
        acc_a += rem;
        let mut rem_c = acc_a;
        prod_8 -= acc_a;
        prod_8 += rem;
        let mut sum_0 = f(CW_B[3]);
        sum_0 *= m;
        let prod_7 = m * table_val_6;
        rem_c -= sum_0;
        acc_a -= rem_c;
        acc_a -= sum_0;
        prod_8 += acc_a;
        prod_8 -= prod_7;
        xh = rem_c;
        xl = prod_8;
    }
    let (h, l) = finish_sign(xh, xl, quadrant);
    (h, l, q, true, None)
}

fn finish_sign(hi: f64, lo: f64, quadrant: u32) -> (f64, f64) {
    let s = f(SIGN_TAB[quadrant as usize]);
    let prod_hs = hi * s;
    let prod_2 = lo * s;
    let o = prod_hs + prod_2;
    let lo_out = (prod_hs - o) + prod_2;
    (o, lo_out)
}

/// |x| >= 2^30, limb product against the 2/pi table .
fn reduce_ph(a_bits: u64, copy_mode: u32, quadrant: u32) -> (f64, f64, u32, usize) {
    let tab = |i: usize| f(PH_TAB[i]);
    let top_half_word = (a_bits >> 48) as u32;
    let mut exp_bits = top_half_word & 0xffff_800f;
    let flag_10 = (copy_mode.wrapping_sub(0xc8) & 0x7ff) << 4;
    exp_bits |= flag_10;
    let xp_bits = (a_bits & 0x0000_ffff_ffff_ffff) | (((exp_bits & 0xffff) as u64) << 48);
    let xp = f(xp_bits);
    let eax2 = copy_mode.wrapping_sub(0x41c) as i32;
    let quot_int = eax2.wrapping_mul(0x147ae) >> 21;
    let step_out = quot_int as i64 as usize;
    let tap_b = quot_int.wrapping_mul(-25);
    let ecx0 = (tap_b as u32).wrapping_add(copy_mode).wrapping_sub(0x41c);
    let flag = ecx0.wrapping_neg().wrapping_add(0x26);
    let mask = (!0u64) << (flag & 63);
    let h_bits = (xp_bits & 0xffff_ffff_0000_0000) | ((xp_bits as u32 & 0xf800_0000) as u64);
    let sum_1 = f(h_bits); // xmm1 = h
    let mut cw_hi = xp;
    let x0t = tab(step_out);
    let mut acc_a = tab(step_out + 1);
    let mut cw_lo = sum_1 * acc_a; // h * t1
    cw_hi -= sum_1; // l
    let mut rem = x0t * cw_hi; // t0 * l
    let mut acc_h = sum_1;
    acc_a *= cw_hi; // t1 * l
    let mut table_val_6 = rem;
    table_val_6 += cw_lo;
    let mut rem_b = table_val_6;
    rem -= table_val_6;
    let mut table_val_12 = tab(step_out + 2);
    rem += cw_lo;
    let mut prod_7 = sum_1 * table_val_12; // h * t2
    table_val_12 *= cw_hi; // l * t2
    rem_b += rem;
    let bits_val_10_b = rem_b.to_bits() & mask;
    cw_lo = tab(step_out + 3);
    acc_h *= cw_lo; // h * t3
    cw_lo *= cw_hi; // l * t3
    let mval = f(bits_val_10_b);
    table_val_6 -= mval;
    let mut prod_8 = table_val_6;
    prod_8 += acc_a;
    let mut acc_b = prod_8;
    table_val_6 -= prod_8;
    acc_b += prod_7;
    table_val_6 += acc_a;
    prod_8 -= acc_b;
    rem += table_val_6;
    prod_8 += prod_7;
    let mut acc_c = acc_b;
    acc_a = sum_1;
    prod_7 = tab(step_out + 4);
    rem += prod_8;
    acc_a *= prod_7; // h * t4
    prod_7 *= cw_hi; // l * t4
    acc_c += rem;
    let mut acc_d = rem;
    acc_c += table_val_12;
    acc_c += acc_h;
    // integer part of x9 from its raw bits
    let raw = acc_c.to_bits() as i64;
    let ecx9 = (((acc_c.to_bits() >> 52) & 0xfff) as u32).wrapping_neg().wrapping_add(0x33);
    let sh = ecx9 & 63;
    let sar = raw >> sh;
    let edx2 = sar as u32;
    let fbits = (sar << sh) as u64;
    acc_c = sum_1;
    let tap_c = edx2 & 1;
    let eq = (1u32.wrapping_sub(2 * quadrant)).wrapping_mul(edx2.wrapping_add(1) >> 1) & 3;
    let fval = f(fbits);
    acc_b -= fval;
    let table_val_11 = f(PH_C_CELL_A[tap_c as usize]);
    acc_d += acc_b;
    acc_b -= acc_d;
    acc_d += table_val_11;
    rem += acc_b;
    let mut acc_e = acc_d;
    let mut copy_11_b = sum_1;
    acc_e += table_val_12;
    rem_b = acc_e;
    acc_d -= acc_e;
    rem_b += acc_h;
    acc_d += table_val_12;
    acc_e -= rem_b;
    rem += acc_d;
    acc_e += acc_h;
    table_val_6 = rem_b;
    let mut copy_14_b = sum_1;
    table_val_12 = tab(step_out + 5);
    table_val_6 += cw_lo;
    acc_c *= table_val_12; // h * t5
    rem += acc_e;
    table_val_12 *= cw_hi; // l * t5
    rem_b -= table_val_6;
    prod_8 = table_val_6;
    rem_b += cw_lo;
    prod_8 += acc_a;
    rem += rem_b;
    table_val_6 -= prod_8;
    acc_b = prod_8;
    table_val_6 += acc_a;
    acc_b += prod_7;
    rem += table_val_6;
    prod_8 -= acc_b;
    cw_lo = tab(step_out + 6);
    acc_d = acc_b;
    copy_11_b *= cw_lo; // h * t6
    acc_d += acc_c;
    prod_8 += prod_7;
    cw_lo *= cw_hi; // l * t6
    table_val_12 += copy_11_b;
    acc_b -= acc_d;
    rem += prod_8;
    acc_b += acc_c;
    let table_val_15_b = tab(step_out + 7);
    rem_b = acc_d;
    copy_14_b *= table_val_15_b; // h * t7
    rem_b += table_val_12;
    rem += acc_b;
    cw_hi *= table_val_15_b; // l * t7
    cw_lo += copy_14_b;
    acc_d -= rem_b;
    acc_a = rem_b;
    acc_d += table_val_12;
    acc_a += cw_lo;
    rem += acc_d;
    rem_b -= acc_a;
    rem_b += cw_lo;
    cw_lo = tab(step_out + 8);
    let prod_1_b = sum_1 * cw_lo; // h * t8
    rem += rem_b;
    cw_hi += prod_1_b;
    cw_lo = cw_hi;
    let ph_coef_b = f(PH_C_CELL_B);
    cw_lo += acc_a;
    let mut prod_1 = ph_coef_b * cw_lo;
    cw_lo += prod_1;
    cw_lo -= prod_1;
    acc_a -= cw_lo;
    acc_a += cw_hi;
    let table_val_2_c = f(PH_C_CELL_C);
    prod_1 = table_val_2_c;
    rem += acc_a;
    prod_1 *= rem;
    rem += cw_lo;
    cw_lo *= table_val_2_c;
    rem *= f(PH_C_CELL_D);
    prod_1 += rem;
    rem = prod_1;
    rem += cw_lo;
    cw_lo -= rem;
    cw_lo += prod_1;
    let sgn = f(SIGN_TAB[quadrant as usize]);
    rem *= sgn;
    cw_lo *= sgn;
    let _ = (&mut acc_d, &mut acc_e, &mut copy_11_b, &mut copy_14_b);
    (rem, cw_lo, eq, step_out)
}
