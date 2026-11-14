//! libm_fmod: double precision fmod (x, y), the remainder of x divided by y with the sign of x, for the AVX2 route of the libm routine used by blur_warp_bidirectional.
//! The dispatcher sees the low byte 0xff of the CPU feature word 0x9d97ff and takes this route (legacy SSE2 encoding only, no x87, no approximate instruction).
//! Fast path: one quotient estimate with a truncated quotient and a split divisor. Slow path: repeated reduction by the estimate. Special operands
//! (zero, infinite, NaN, power of two, subnormal or extreme divisor) take the helper routes below. No fused operations, no library fmod.

const ABS: u64 = 0x7fff_ffff_ffff_ffff;
const SIGN: u64 = 0x8000_0000_0000_0000;
const K_MASK: u64 = 0xffff_ffff_f800_0000; // 
const K_2P64: u64 = 0x43f0_0000_0000_0000; // 
const K_2M64: u64 = 0x3bf0_0000_0000_0000; // 
const K_HALF: u64 = 0x3fe0_0000_0000_0000; // 
const K_DBLMIN: u64 = 0x0010_0000_0000_0000; // 
const K_2P1023: u64 = 0x7fe0_0000_0000_0000; // 

use fx_common::x86::{nx_add64, nx_div64, nx_mul64, nx_sub64};

fn f(b: u64) -> f64 {
    f64::from_bits(b)
}
fn bt(x: f64) -> u64 {
    x.to_bits()
}
// double precision operations with the NaN result of x86 on every processor
fn mul(a: u64, b: u64) -> u64 {
    bt(nx_mul64(f(a), f(b)))
}
fn sub(a: u64, b: u64) -> u64 {
    bt(nx_sub64(f(a), f(b)))
}
fn add(a: u64, b: u64) -> u64 {
    bt(nx_add64(f(a), f(b)))
}
fn div(a: u64, b: u64) -> u64 {
    bt(nx_div64(f(a), f(b)))
}
/// Logical left shift of 64 bit lanes: a count above 63 gives zero.
fn shl_saturating(x: u64, count: u64) -> u64 {
    if count > 63 { 0 } else { x << count }
}
/// Logical right shift of 64 bit lanes: a count above 63 gives zero.
fn shr_saturating(x: u64, count: u64) -> u64 {
    if count > 63 { 0 } else { x >> count }
}
/// All ones when !(0 <= x), NaN included; zero otherwise.
fn negative_or_nan_mask(x: u64) -> u64 {
    if 0.0 <= f(x) { 0 } else { u64::MAX }
}
/// The top 16 bits of a word: sign, exponent and the first four mantissa bits.
fn top16(b: u64) -> u64 {
    (b >> 48) & 0xffff
}

pub fn assumptions() -> Vec<&'static str> {
    vec![
        "A1 feature word (low byte 0xff), so the dispatcher takes (INFERRED from the p1 precedent)", // origin: 0x9d97ff
        "A2 the error handler (called only for y == 0, via) is UNREAD; modelled as leaving the NaN result unchanged",
        "A3 the SSE route is not ported (not taken on the AVX2 route)",
        "A4 NaN payloads follow the host FPU and are compared as a class",
        "A5 MXCSR default (round to nearest, no FTZ or DAZ)",
    ]
}

fn handler_nan(y_bits: u64) -> u64 {
    div(y_bits, y_bits)
}

/// Result when x is infinite or NaN (or the invalid case of an infinite divisor path).
fn inf_nan_x(x_bits: u64, y_bits: u64, y_exp_m1: u64, y_mant_gate: u64) -> u64 {
    // 
    if (x_bits << 12) != 0 {
        return sub(x_bits, x_bits);
    }
    let y_exp_m1_signed = (y_exp_m1 as u32 as i32) as i64 as u64;
    if ((y_mant_gate & y_exp_m1_signed) as i64) < 0 {
        return handler_nan(y_bits);
    }
    if (y_bits << 1) > 0xffe0_0000_0000_0000 {
        return add(x_bits, y_bits);
    }
    sub(x_bits, x_bits)
}

/// Result when the divisor is a power of two: x with the bits below the divisor's unit cleared away.
fn y_pow2(x_bits: u64, exp_gap: u64) -> u64 {
    // 
    let keep_bits = 0x34u64.wrapping_sub(exp_gap);
    if (keep_bits as i64) <= 0 {
        // 
        if (top16(x_bits) & 0x7fff) == 0x7ff0 {
            return inf_nan_x(x_bits, 0, 0, 0);
        }
        return 0;
    }
    let high_part = shl_saturating(shr_saturating(x_bits, keep_bits), keep_bits);
    sub(x_bits, high_part) | (x_bits & SIGN)
}

/// Result when the exponent gap is large: reduce in two scaled steps.
fn big_exp(x_bits: u64, y_bits: u64, shift_room: u64) -> u64 {
    // 
    let gap = 0u64.wrapping_sub(shift_room);
    let x_reduced = if gap >= 0x3e7 { bt(fmod_x(f(x_bits), f(mul(y_bits, K_2P1023)))) } else { x_bits };
    mul(bt(fmod_x(f(mul(x_reduced, K_2P64)), f(mul(y_bits, K_2P64)))), K_2M64)
}

/// Result for a divisor that is zero, subnormal, infinite, NaN or of extreme exponent.
fn y_special(x_bits: u64, y_bits: u64, y_exp_m1: u64, y_mant_gate: u64, shift_room: u64) -> u64 {
    // 
    if (top16(x_bits) & 0x7ff0) == 0x7ff0 {
        return inf_nan_x(x_bits, y_bits, y_exp_m1, y_mant_gate);
    }
    if (y_exp_m1 as u32 as i32) < 0 {
        // 
        if (y_mant_gate as i64) < 0 {
            return handler_nan(y_bits);
        }
        if ((top16(x_bits) & 0x7ff0) as i32) > 1 {
            return big_exp(x_bits, y_bits, shift_room);
        }
        let scaled_x = mul(x_bits, K_2P64);
        let scaled_y = mul(y_bits, K_2P64);
        let estimate = div(scaled_x, scaled_y);
        let estimate_exp = (((top16(estimate) & 0x7ff0) >> 4) as i32).wrapping_sub(0x3ff);
        if estimate_exp < 0 {
            return x_bits;
        }
        let drop_bits = 0x34u32.wrapping_sub(estimate_exp as u32) as u64;
        let truncated = shl_saturating(shr_saturating(estimate, drop_bits), drop_bits);
        return mul(sub(scaled_x, mul(truncated, scaled_y)), K_2M64);
    }
    if (y_mant_gate as i64) < 0 {
        return x_bits;
    }
    add(x_bits, y_bits)
}

/// Double precision fmod as computed on the AVX2 route (target).
pub fn fmod_x(x: f64, y: f64) -> f64 {
    f(fmod_bits(bt(x), bt(y)))
}

fn fmod_bits(x_bits: u64, y_bits: u64) -> u64 {
    let abs_x = ABS & x_bits;
    let abs_y = ABS & y_bits;
    let y_mant_shifted = y_bits << 12;
    let x_over_max = 0x7fef_ffff_ffff_ffffu64.wrapping_sub(abs_x);
    // exponent gap of the magnitudes, and the shift room left for the quotient mask
    let exp_gap = ((abs_x.wrapping_sub(abs_y) as i64) >> 52) as u64;
    let shift_room = 0x19u64.wrapping_sub(exp_gap);
    // negative when the divisor mantissa is zero (a power of two)
    let y_mant_gate = ((y_mant_shifted >> 1).wrapping_sub(1)) as u64;
    let y_hi = y_bits & K_MASK;
    let y_exp_m1 = (abs_y >> 52).wrapping_sub(1);
    let y_exp_room = 0x7fdu64.wrapping_sub(y_exp_m1);
    let range_flags = x_over_max | exp_gap | shift_room | y_mant_gate | y_exp_m1 | y_exp_room;
    if (range_flags as i64) >= 0 {
        // 
        let quotient = div(x_bits, y_bits);
        let sign_x = x_bits & SIGN;
        let quot_mask = shl_saturating(K_MASK, shift_room);
        let y_lo = sub(y_bits, y_hi);
        let y_hi_signed = y_hi ^ sign_x;
        let quot_trunc = quotient & quot_mask;
        let y_lo_signed = y_lo ^ sign_x;
        let prod_hi = mul(y_hi_signed, quot_trunc);
        let prod_lo = mul(y_lo_signed, quot_trunc);
        let remainder = sub(x_bits & ABS, prod_hi);
        let remainder = sub(remainder, prod_lo);
        let fix = negative_or_nan_mask(remainder) & (y_bits & ABS);
        let remainder = add(remainder, fix);
        return remainder | sign_x;
    }
    // 
    let y_range = y_exp_room | y_exp_m1;
    if (y_range as i64) < 0 {
        return y_special(x_bits, y_bits, y_exp_m1, y_mant_gate, shift_room);
    }
    if (exp_gap as i64) < 0 {
        return x_bits;
    }
    if (top16(x_bits) & 0x7ff0) == 0x7ff0 {
        return inf_nan_x(x_bits, y_bits, y_exp_m1, y_mant_gate);
    }
    if (y_mant_gate as i64) < 0 {
        return y_pow2(x_bits, exp_gap);
    }
    // 
    let exp_limit_wide = ((y_exp_m1 as u32).wrapping_add(0x1a) as u64) << 4;
    let exp_limit = exp_limit_wide as u32;
    let mut divisor = (y_bits << 1) >> 1;
    let mut div_hi = (y_hi << 1) >> 1;
    let mut div_lo = sub(divisor, div_hi);
    let mut estimate = (x_bits << 1) >> 1;
    let sign_x = x_bits & SIGN;
    let mut remainder = estimate;
    let gap = 0u64.wrapping_sub(shift_room);
    let quot_mask = K_MASK;
    if gap >= 0x3e7 {
        // 
        divisor = mul(divisor, K_2P1023);
        let big_limit = exp_limit.wrapping_add(0x3e60);
        div_hi = mul(div_hi, K_2P1023);
        div_lo = mul(div_lo, K_2P1023);
        loop {
            estimate = div(estimate, divisor);
            estimate &= quot_mask;
            let prod_hi = mul(div_hi, estimate);
            let mut scale_exp = top16(estimate);
            let prod_lo = mul(estimate, div_lo);
            remainder = sub(remainder, prod_hi);
            remainder = sub(remainder, prod_lo);
            scale_exp &= 0x7ff0;
            scale_exp = scale_exp.wrapping_sub(0x190);
            let fix_mask = negative_or_nan_mask(remainder);
            let scaled_divisor = mul((scale_exp & 0xffff) << 48, divisor);
            remainder = add(remainder, fix_mask & scaled_divisor);
            let remainder_exp = (top16(remainder) & 0x7ff0) as u32;
            estimate = remainder;
            if remainder_exp <= big_limit {
                break;
            }
        }
        divisor = mul(mul(divisor, K_DBLMIN), K_HALF);
        div_hi = mul(mul(div_hi, K_DBLMIN), K_HALF);
        div_lo = mul(mul(div_lo, K_DBLMIN), K_HALF);
    }
    // 
    loop {
        estimate = div(estimate, divisor);
        estimate &= quot_mask;
        let prod_hi = mul(div_hi, estimate);
        let mut scale_exp = top16(estimate);
        let prod_lo = mul(estimate, div_lo);
        remainder = sub(remainder, prod_hi);
        remainder = sub(remainder, prod_lo);
        scale_exp &= 0xfff0;
        scale_exp = scale_exp.wrapping_sub(0x190);
        let fix_mask = negative_or_nan_mask(remainder);
        let scaled_divisor = mul((scale_exp & 0xffff) << 48, divisor);
        remainder = add(remainder, fix_mask & scaled_divisor);
        let remainder_exp = (top16(remainder) & 0x7ff0) as u32;
        estimate = remainder;
        if remainder_exp <= exp_limit {
            break;
        }
    }
    // 
    estimate = div(estimate, divisor);
    let estimate_exp = top16(estimate) & 0x7ff0;
    if estimate_exp < 0x3ff0 {
        return remainder | sign_x;
    }
    let final_shift = 0x418u64.wrapping_sub(estimate_exp >> 4);
    let final_mask = shl_saturating(quot_mask, final_shift);
    estimate &= final_mask;
    let prod_hi = mul(div_hi, estimate);
    let divisor_abs = (divisor << 1) >> 1;
    let prod_lo = mul(estimate, div_lo);
    remainder = sub(remainder, prod_hi);
    remainder = sub(remainder, prod_lo);
    let fix = negative_or_nan_mask(remainder) & divisor_abs;
    remainder = add(remainder, fix);
    remainder | sign_x
}
