//! The scalar special-lane routines behind the vector exponential and power routines, as ordinary Rust over f32 and u32
//! values: e^x for arguments outside the vector routine's fast range, and x^y for the lanes its mask selects.
//! Float operations follow the instruction order of the routines, and every one goes through the first-operand NaN rule
//! (`NanOps`). Compares state the unordered case explicitly. The power routine's finite main path is the generated
//! `scalar_core`.
use crate::scalar_consts::*;
use crate::scalar_core::pow_core;
use crate::tables::stables;
use fx_common::nanop::NanOps;

fn f(b: u32) -> f32 {
    f32::from_bits(b)
}

/// The default quiet NaN an invalid operation produces.
const DEFAULT_NAN: u32 = 0xffc0_0000;

/// Scalar exponential e^x. Every path produces the result; `init` is returned only if none did (never).
pub fn expf_scalar_bits(xb: u32, init: u32) -> u32 {
    let x = f(xb);
    // exponent field all ones: infinity or NaN
    if (xb >> 23) & 0xff == 0xff {
        let negative = xb >> 31 == 1;
        if negative && xb & 0x7f_ffff == 0 {
            return 0; // e^-inf
        }
        return x.fmul(x).to_bits(); // +inf, or a NaN squared
    }
    // above the overflow bound: the largest float squared; ordered compare is false for NaN but x is finite here
    if f(EXP_X_MAX) < x {
        let m = f(F32_MAX);
        return m.fmul(m).to_bits();
    }
    // below the underflow bound: the smallest subnormal squared
    if x < f(EXP_X_MIN) {
        let m = f(F32_TINY);
        return m.fmul(m).to_bits();
    }
    // n = round(x log2 e) through the shifter; r = x - n ln 2 in two parts
    let t = f(LOG2_E).fmul(x);
    let a = t.fadd(f(ROUND_SHIFTER));
    let n_bits = a.to_bits();
    let k = a.fsub(f(ROUND_SHIFTER));
    let r = x.fsub(k.fmul(f(LN2_HI))).fsub(k.fmul(f(LN2_LO)));
    let one_p = f(ONE_PLUS_ULP);
    let mut p = f(EXP_C4).fmul(r);
    p = p.fadd(f(EXP_C3)).fmul(r);
    p = p.fadd(f(EXP_C2)).fmul(r);
    p = p.fadd(f(EXP_C1)).fmul(r);
    p = p.fadd(one_p).fmul(r);
    p = p.fadd(one_p);
    // n is the low 16 bits of the shifter sum, as a signed value
    let n = (n_bits as u16 as i16) as i32;
    if x < f(EXP_X_DENORM) {
        // subnormal range: scale in two steps, 2^(n - 69) then 2^-66
        let scale = (((n - 69) as u32) & 0xff) << 23;
        return p.fmul(f(scale)).fmul(f(TWO_POW_M66)).to_bits();
    }
    let biased = (n + 127) as u32;
    if biased <= 254 {
        let scale = (biased & 0xff) << 23;
        return p.fmul(f(scale)).to_bits();
    }
    // exponent out of single range: 2^(n - 1) then times two
    let scale = (((n + 126) as u32) & 0xff) << 23;
    let _ = init;
    p.fmul(f(scale)).fmul(f(TWO)).to_bits()
}

/// Integer class of y: 0 not an integer (or |y| < 1, or not finite), 1 odd integer, 2 even integer.
fn integer_class(yb: u32) -> u32 {
    let a = yb & 0x7fff_ffff;
    if a.wrapping_sub(0x3f80_0000) >= 0x4000_0000 {
        return 0;
    }
    if a >= 0x4b80_0000 {
        return 2; // 2^24 and above: even
    }
    let shift = (((a & 0xff80_0000).wrapping_add(0x80_0000)) >> 23) & 31;
    let moved = a << shift;
    if moved & 0x7f_ffff != 0 {
        return 0;
    }
    if moved & 0xff_ffff == 0x80_0000 {
        1
    } else {
        2
    }
}

/// Scalar power x^y. Every path produces the result; `init` is returned only if none did (never).
pub fn powf_scalar_bits(xb: u32, yb: u32, init: u32) -> u32 {
    let _ = init;
    let (x, y) = (f(xb), f(yb));
    let xsign = xb >> 31;
    let ysign = yb >> 31;
    let xexp = (xb >> 23) & 0xff;
    let yexp = (yb >> 23) & 0xff;
    let x_man_zero = xb & 0x7f_ffff == 0;
    let y_man_zero = yb & 0x7f_ffff == 0;
    let x_finite = xexp != 0xff;
    let y_finite = yexp != 0xff;
    let yint = integer_class(yb);

    // x == 1 (positive) or y == 0: the result is one; the sign of x + y is copied onto 1.0 and squared away
    let one_result = || {
        let s = x.fadd(y);
        let one = f(ONE | ((s.to_bits() >> 31) << 31));
        one.fmul(one).to_bits()
    };
    if xsign == 0 && xexp == 0x7f && x_man_zero {
        return one_result();
    }
    if yexp == 0 && y_man_zero {
        return one_result();
    }
    // a NaN in either operand propagates through x + y
    if !(x_man_zero || x_finite) || !(y_man_zero || y_finite) {
        return x.fadd(y).to_bits();
    }
    // x is +0 or -0 (ordered equal; NaN is excluded above)
    if x == 0.0 {
        let xx = x.fmul(x);
        let sgn = yint & xsign;
        if ysign == 0 {
            return f(SIGN_ONE[sgn as usize]).fmul(xx).to_bits();
        }
        return f(SIGN_ONE[sgn as usize]).fdiv(xx).to_bits();
    }
    // x == -1 with an integer or infinite y
    if x == f(MINUS_ONE) && (yint != 0 || !y_finite) {
        return SIGN_ONE[(yint & 1) as usize];
    }
    // infinite x or y
    if !x_finite || !y_finite {
        if xexp < 0x7f {
            // |x| < 1: y = +inf gives 0, y = -inf gives +inf
            return if ysign == 0 { 0 } else { y.fmul(y).to_bits() };
        }
        if ysign == 0 {
            let xx = x.fmul(x);
            let sgn = yint & xsign;
            return y.fmul(xx).fmul(f(SIGN_ONE[sgn as usize])).to_bits();
        }
        let sgn = yint & xsign;
        return SIGN_ZERO[sgn as usize];
    }
    // negative x needs an integer y
    if !(x > 0.0) && yint == 0 {
        return DEFAULT_NAN;
    }
    let sgn = yint & xsign;
    let sign_factor = f(SIGN_ONE[sgn as usize]);
    let abs = xb & 0x7fff_ffff;
    let (s_c, sel_a) = if xexp == 0 { (f(abs).fmul(f(SCALE_2P64)).to_bits(), 0xffff_ffc0u32) } else { (abs, 0u32) };
    pow_core(stables(), s_c, sel_a, yb, ysign, yexp, sign_factor)
}
