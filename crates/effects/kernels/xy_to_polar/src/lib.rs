//! Native port of the class `xy_to_polar`: for every pixel of two input planes (x and y) it writes the angle in turns
//! (`atan2(y, x) / 2pi - dir_offset`, reduced to its fractional part) and the magnitude `sqrt(x*x + y*y)`.
//!
//! ROUTE AVX2-ONLY: the 8 lane body is entered on machines with the AVX2 and FMA feature bits; the SSE and AVX twins are not
//! ported. Structure of the body (all arithmetic in the listing's order, `mul_add` exactly at the fused sites):
//! * the angle comes from the 8 lane single precision atan2 routine of the C library the plugin links (here `atan2_vec`
//!   for lanes whose two magnitudes lie in [2^-125, 2^122), and `atan2_scalar` for every other lane);
//! * the magnitude of a pixel in the vector body uses the reciprocal square root estimate with one Newton correction (APPROX-MODEL,
//!   `rsqrt_est`); a pixel in the scalar head or tail uses an exact square root.
//!
//! NOT REBUILT: the constant input route (flag byte set on the object) and the general table driven branch of the scalar atan2
//! helper (finite non zero lanes outside the vector range whose exponents differ by less than 54): they return a named refusal.

use fx_common::nanop::{self, NanOps};

/// 1 / (2 pi) as f32.
pub const INV_TWO_PI: f32 = f32::from_bits(0x3e22_f983);
const PI_HALF_F: f32 = f32::from_bits(0x3fc9_0fdb);
const PI_F: f32 = f32::from_bits(0x4049_0fdb);
// polynomial coefficients of the vector atan2 (each cell holds one value in all 8 lanes)
const P_A: f32 = f32::from_bits(0x3b32_2cc0);
const P_B: f32 = f32::from_bits(0xbc7f_2631);
const P_C: f32 = f32::from_bits(0x3d2b_c384);
const P_D: f32 = f32::from_bits(0xbd98_7629);
const P_E: f32 = f32::from_bits(0x3dd9_6474);
const P_F: f32 = f32::from_bits(0xbe11_61f8);
const P_G: f32 = f32::from_bits(0x3e4c_b79f);
const P_H: f32 = f32::from_bits(0xbeaa_aa49);
const P_I: f32 = 1.0;
/// range test constants of the vector routine: lane is special when (|v| bits + 0x7f000000) as i32 >= RANGE_LIMIT
const RANGE_BIAS: u32 = 0x7f00_0000;
const RANGE_LIMIT: i32 = 0xfc00_0000u32 as i32;
const FLT_MIN: f32 = f32::MIN_POSITIVE;
const NEG_HALF: f32 = -0.5;
const THREE: f32 = 3.0;

// scalar helper constants: each angle is a double pair (high part, low part), summed in double and narrowed to f32
const QUARTER_PI_HI: u64 = 0x3fe9_21fb_5444_2d18;
const QUARTER_PI_LO: u64 = 0x3c81_a626_3314_5c07;
const HALF_PI_HI: u64 = 0x3ff9_21fb_5444_2d18;
const HALF_PI_LO: u64 = 0x3c91_a626_3314_5c07;
const PI_HI: u64 = 0x4009_21fb_5444_2d18;
const PI_LO: u64 = 0x3ca1_a626_3314_5c07;
const THREE_QUARTER_PI_HI: u64 = 0x4002_d97c_7f33_21d2;
const THREE_QUARTER_PI_LO: u64 = 0x3c9a_7939_4c9e_8a0a;

/// A route the port does not rebuild.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// finite non zero lane outside the vector range whose exponents differ by less than 54 (table driven helper branch)
    Atan2General { index: usize },
}

pub fn assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        // 0x53a660: vrsqrtps in the eight-lane body (APPROX-MODEL site)
        ("APPROX-MODEL", "reciprocal square root estimate of the vector magnitude body modelled as 1/sqrt(x) with the emulator's two roundings (sqrt, then divide); sites: vrsqrtps in the 8 lane body"),
        ("ROUTE AVX2-ONLY", "8 lane body entered on the AVX2 and FMA feature bits; SSE and AVX twins not ported"),
        ("ATAN2-GENERAL-NOT-PORTED", "scalar helper branch for finite non zero lanes outside [2^-125, 2^122) with exponent difference below 54 is refused by name"),
        ("CONST-INPUT-NOT-PORTED", "route with the constant input flag set (scalar atan2 of two constants, integer outputs) is not rebuilt"),
        ("ALIGN-PHASE", "the vector body partition depends on the address of the magnitude output modulo 32; the caller passes it"),
    ]
}

#[inline]
fn abs_bits(v: f32) -> u32 {
    v.to_bits() & 0x7fff_ffff
}

#[inline]
fn quiet(v: f32) -> f32 {
    f32::from_bits(v.to_bits() | 0x0040_0000)
}

/// True when the lane stays on the vector polynomial (a NaN, infinity, zero, denormal, tiny or huge magnitude does not).
pub fn in_vector_range(v: f32) -> bool {
    let t = abs_bits(v).wrapping_add(RANGE_BIAS) as i32;
    t < RANGE_LIMIT
}

/// The 8 lane polynomial routine for one lane whose two arguments are in range.
pub fn atan2_vec(y: f32, x: f32) -> f32 {
    let abs_x = f32::from_bits(abs_bits(x));
    let ay = f32::from_bits(abs_bits(y));
    let lt = ay < abs_x;
    let num = if lt { ay } else { f32::from_bits(abs_x.to_bits() | 0x8000_0000) };
    let den = if lt { abs_x } else { ay };
    let le = x <= 0.0;
    let q = num / den;
    let sx = x.to_bits() ^ abs_x.to_bits();
    let sy = y.to_bits() ^ ay.to_bits();
    let z = q * q;
    let z_sq = z * z;
    let mut poly_hi = P_B;
    let mut poly_lo = P_A;
    poly_hi = poly_hi.mul_add(z_sq, P_D);
    poly_lo = poly_lo.mul_add(z_sq, P_C);
    poly_hi = poly_hi.mul_add(z_sq, P_F);
    poly_lo = poly_lo.mul_add(z_sq, P_E);
    poly_hi = poly_hi.mul_add(z_sq, P_H);
    poly_lo = poly_lo.mul_add(z_sq, P_G);
    poly_lo = poly_lo.mul_add(z_sq, P_I);
    poly_hi = poly_hi.mul_add(z, poly_lo);
    let quad_offset = if lt { 0.0 } else { PI_HALF_F };
    let r = poly_hi.mul_add(q, quad_offset);
    let pi_offset = if le { PI_F } else { 0.0 };
    let r_signed = f32::from_bits(r.to_bits() | sx);
    let r_adjusted = r_signed + pi_offset;
    f32::from_bits(r_adjusted.to_bits() | sy)
}

fn pair(hi: u64, lo: u64) -> f64 {
    f64::from_bits(hi) + f64::from_bits(lo)
}

fn with_sign(res: f64, neg: bool) -> f32 {
    let b = (res.to_bits() & 0x7fff_ffff_ffff_ffff) | ((neg as u64) << 63);
    f64::from_bits(b) as f32
}

fn dexp(v: f32) -> i32 {
    ((v as f64).to_bits() >> 52 & 0x7ff) as i32
}

/// The scalar helper for lanes outside the vector range (argument order: y, x as the helper takes them).
pub fn atan2_scalar(y: f32, x: f32, index: usize) -> Result<f32, Refusal> {
    // the helper multiplies both arguments by 1.0f (a quiet NaN stays a NaN, its payload quieted)
    let yp = y.fmul(1.0);
    let xp = x.fmul(1.0);
    let yneg = yp.to_bits() >> 31 != 0;
    let xneg = xp.to_bits() >> 31 != 0;
    let ye = (yp.to_bits() >> 23) & 0xff;
    let xe = (xp.to_bits() >> 23) & 0xff;
    if ye == 0xff || xe == 0xff {
        if yp.is_nan() || xp.is_nan() {
            return Ok(yp.fadd(xp));
        }
        if xe != 0xff {
            return Ok(with_sign(pair(HALF_PI_HI, HALF_PI_LO), yneg));
        }
        if ye == 0xff {
            let p = if !xneg { pair(QUARTER_PI_HI, QUARTER_PI_LO) } else { pair(THREE_QUARTER_PI_HI, THREE_QUARTER_PI_LO) };
            return Ok(with_sign(p, yneg));
        }
        return Ok(if !xneg { with_sign(0.0, yneg) } else { with_sign(pair(PI_HI, PI_LO), yneg) });
    }
    if yp == 0.0 {
        return Ok(if xneg { with_sign(pair(PI_HI, PI_LO), yneg) } else { with_sign(0.0, yneg) });
    }
    if xp == 0.0 {
        return Ok(with_sign(pair(HALF_PI_HI, HALF_PI_LO), yneg));
    }
    let d = dexp(yp) - dexp(xp);
    if d <= -54 {
        if xneg {
            return Ok(with_sign(pair(PI_HI, PI_LO), yneg));
        }
        let q = (yp.abs() as f64) / (xp.abs() as f64);
        let qf = q as f32;
        let res = if (qf.to_bits() >> 23) & 0xff != 0 {
            q * (1.0 + q)
        } else {
            let qf_sq = qf * qf;
            (qf_sq as f64) + q
        };
        return Ok(with_sign(res, yneg));
    }
    if d >= 74 {
        return Ok(with_sign(pair(HALF_PI_HI, HALF_PI_LO), yneg));
    }
    if d >= 54 {
        let xd = (xp as f64) / (yp.abs() as f64);
        let res = (f64::from_bits(HALF_PI_LO) - xd) + f64::from_bits(HALF_PI_HI);
        return Ok(with_sign(res, yneg));
    }
    Err(Refusal::Atan2General { index })
}

/// atan2(y, x) of one lane as the AVX2 routine returns it.
pub fn atan2_lane(y: f32, x: f32, index: usize) -> Result<f32, Refusal> {
    if in_vector_range(y) && in_vector_range(x) {
        Ok(atan2_vec(y, x))
    } else {
        atan2_scalar(y, x, index)
    }
}

/// Reciprocal square root estimate of the vector body, modelled as 1/sqrt(s) (APPROX-MODEL).
pub fn rsqrt_est(s: f32) -> f32 {
    1.0f32 / s.sqrt()
}

fn sqrt_ss(s: f32) -> f32 {
    if s.is_nan() { quiet(s) } else { s.sqrt() }
}

fn floor_q(v: f32) -> f32 {
    if v.is_nan() { quiet(v) } else { v.floor() }
}

/// Angle plane value of one pixel.
fn angle(a: f32, dir: f32) -> f32 {
    let v = nanop::fms213(a, INV_TWO_PI, dir);
    let fl = floor_q(v);
    v.fsub(fl)
}

/// Magnitude of a pixel in the scalar head or tail.
fn mag_scalar(x: f32, y: f32) -> f32 {
    let y_sq = y.fmul(y);
    sqrt_ss(nanop::fma213(x, x, y_sq))
}

/// Magnitude of a pixel in the 8 lane body.
/// Magnitude of a pixel in the 8 lane body.
pub fn vec_mag(x: f32, y: f32) -> f32 {
    mag_vector(x, y)
}

fn mag_vector(x: f32, y: f32) -> f32 {
    mag_vector_with(x, y, rsqrt_est)
}

/// The vector magnitude with a caller supplied reciprocal square root estimate (used by the sensitivity test).
pub fn mag_vector_with(x: f32, y: f32, est: impl Fn(f32) -> f32) -> f32 {
    let y_sq = y.fmul(y);
    let s = nanop::fma213(x, x, y_sq);
    let a = f32::from_bits(abs_bits(s));
    let m = a >= FLT_MIN;
    let r = if m { est(s) } else { 0.0 };
    let t = s.fmul(r);
    let u = t.fmul(r);
    let v = u.fsub(THREE);
    let w = t.fmul(v);
    w.fmul(NEG_HALF)
}

/// Vector body range [lo, hi) of a row of `n` pixels whose magnitude output starts `mag_phase` bytes past a 32 byte boundary.
pub fn vector_body(n: usize, mag_phase: usize) -> (usize, usize) {
    if n < 8 {
        return (0, 0);
    }
    if n < 0x4b0 {
        return (0, n & !7);
    }
    let ph = mag_phase & 0x1f;
    let head = if ph == 0 {
        0
    } else if ph & 3 != 0 {
        return (0, 0);
    } else {
        (0x20 - ph) >> 2
    };
    if n < head + 8 {
        return (0, 0);
    }
    (head, n - ((n - head) & 7))
}

/// Run over `n = x.len()` pixels. On a refusal the pixels before the refused one are written and the error is returned.
pub fn xy_to_polar(x: &[f32], y: &[f32], dir: f32, ang: &mut [f32], mag: &mut [f32], mag_phase: usize) -> Result<(), Refusal> {
    let n = x.len();
    let (lo, hi) = vector_body(n, mag_phase);
    for i in 0..n {
        let a = atan2_lane(y[i], x[i], i)?;
        ang[i] = angle(a, dir);
        mag[i] = if i >= lo && i < hi { mag_vector(x[i], y[i]) } else { mag_scalar(x[i], y[i]) };
    }
    Ok(())
}
