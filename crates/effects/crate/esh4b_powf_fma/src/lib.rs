//! ESH4: std-only native of the original library routine `powf` (dispatcher, body, twin body).
//!
//! What it is (MEASURED by the listing and the emulator, see SPEC.md): single precision power, `powf(x, y)`, evaluated in double precision with a
//! 65 pair log2 table, a degree 5 style polynomial pair for log2, a 32 entry exp2 table and a short exp2 polynomial; the result is rounded once
//! by `cvtpd2ps`. The two bodies have the same instruction stream and byte equal data, so one port covers both
//! dispatcher arms (the third arm calls the CPU init and loops back).
//!
//! The port follows the instruction order of the listing (no fused multiply add anywhere: the body is legacy SSE), keeps every special case
//! path, and reports the error code that the original passes to its errno/matherr service (ecx at the call) in `Out::err`.
//! The service is modelled as leaving the result unchanged (labelled ASSUMPTION A1 in SPEC.md).
//!
//! HOST-FP-ASSUMED: round to nearest even, no flush to zero, no denormals are zero. The routine reads and writes MXCSR flags on two paths
//! (stmxcsr/ldmxcsr clearing the precision and underflow flags); flags are not modelled.
mod tables;
use tables::*;

/// Result of one call: the value bits returned in xmm0 and the error code handed to the host service, if the original calls it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Out {
    pub bits: u32,
    /// `ecx` at the call of (0x1e overflow, 0x1f underflow, 0x20 pow(0,0), 0x21 pole, 0x22 domain, 0x23 pow(NaN,0)).
    pub err: Option<u32>,
}

impl Out {
    pub fn value(&self) -> f32 {
        f32::from_bits(self.bits)
    }
}

fn d(b: u64) -> f64 {
    f64::from_bits(b)
}
fn ok(bits: u32) -> Out {
    Out { bits, err: None }
}
fn er(bits: u32, code: u32) -> Out {
    Out { bits, err: Some(code) }
}
/// `x + x` for a NaN operand (addss xmm0,xmm0): the quiet NaN with the same payload.
fn quiet(b: u32) -> u32 {
    b | 0x0040_0000
}

const ONE_F: u32 = 0x3f80_0000;
const INF_F: u32 = 0x7f80_0000;
/// 1.5 * 2^47 (immediate 0x42e8000000000000): rounding shifter for 1/32 steps.
const SHIFT_BITS: u64 = 0x42e8_0000_0000_0000;

/// Public entry: `powf(x, y)` as the original routine returns it.
pub fn powf(x: f32, y: f32) -> f32 {
    powf_route(x, y).value()
}

/// Full entry with the error code of the host service call.
pub fn powf_route(x: f32, y: f32) -> Out {
    let xb = x.to_bits();
    let yb = y.to_bits();
    let yd = y as f64; // cvtss2sd (exact; only the exponent word is read for non-finite y)
    // xmm5 = 1.0 | (xb << 29): the float mantissa becomes the double mantissa (the exponent and sign bits fall into bits that are already one)
    let m_bits = 1.0f64.to_bits() | ((xb as u64) << 29);
    if 0x7eff_ffffu32 < xb.wrapping_sub(0x0080_0000) {
        return special(xb, yb, yd);
    }
    main_path(xb, yb, yd, m_bits, (xb >> 16) as i32, 0)
}

/// table log2, exponent range test, then either the fast exp tail or the alternative path.
fn main_path(xb: u32, yb: u32, yd: f64, m_bits: u64, h: i32, ecx: u32) -> Out {
    let idx = (((h as u32 & 0x7f) + 1) & 0xfe) as usize;
    let c = d(TBL[idx]);
    let lg = d(TBL[idx + 1]);
    let x7 = d(((0x1ff80u32 + idx as u32) as u64) << 45);
    let k = ((h.wrapping_add(0x41)) >> 7) - 0x7f;
    let kd = k as f64;
    let r = d(m_bits) - x7;
    let rp = r * c;
    let z0 = (lg + rp) + kd;
    let p_lo = d(A_LO) * rp + d(B_LO);
    let p_hi = d(A_HI) * rp + d(B_HI);
    let q = rp * rp;
    // bsr of |k| (destination unchanged when the source is zero; eax held |k| = 0 then)
    let s = k >> 31;
    let a = (k.wrapping_add(s) ^ s) as u32;
    let bsr: u32 = if a == 0 { 0 } else { 31 - a.leading_zeros() };
    let ye = (((yd.to_bits() >> 48) as u32) & 0x7ff0) >> 4;
    let e = bsr.wrapping_sub(0x3e1).wrapping_add(ye);
    if e >= 0x23 {
        return alt(xb, yb, yd, e, ecx, z0, q, p_lo, p_hi);
    }
    // fast path 
    let zz = z0 * yd;
    let r4 = q * q;
    let pl = p_lo * r4;
    let ph = p_hi * q;
    let ex = yd * (pl + ph);
    let v = finish(zz, ex, ecx);
    ok(v)
}

///  onward (the exp2 reconstruction, shared by the fast path and): returns the float bits.
fn finish(zz: f64, ex: f64, ecx: u32) -> u32 {
    let sh = d(SHIFT_BITS);
    let s = zz + sh;
    let n = s.to_bits() as u32 as i32;
    let zr = s - sh;
    let f = (zz - zr) + ex;
    let t = d(TBL[T2_BASE + (n & 0x1f) as usize]);
    let edx = (n >> 1) & !0xf;
    let e2 = (ecx | 0x3ff0).wrapping_add(edx as u32);
    let scale = d(((e2 & 0xffff) as u64) << 48);
    let v_lo = d(C_LO) * f;
    let v_hi = d(C_HI) * f;
    let f2 = f * f;
    let ft = f * t;
    let w_lo = d(D_LO) + v_lo;
    let w_hi = d(D_HI) + v_hi;
    let ts = t * scale;
    let g = f2 * w_lo;
    let fts = ft * scale;
    let acc = (w_hi + g) * fts;
    let res = ts + acc;
    (res as f32).to_bits()
}

/// exponent product outside the plain window.
#[allow(clippy::too_many_arguments)]
fn alt(xb: u32, yb: u32, yd: f64, e: u32, ecx: u32, z0: f64, q: f64, p_lo: f64, p_hi: f64) -> Out {
    if xb == ONE_F {
        return ok(ONE_F); // also clears MXCSR flags 0x30
    }
    let ya = yb & 0x7fff_ffff;
    if ya >= INF_F {
        return ycase(xb, yb);
    }
    if ya == 0 {
        return yzero(xb);
    }
    if (e as i32) < 0 {
        return ok((ecx << 16) | ONE_F); // 
    }
    if e < 0x26 {
        return poly2(yd, ecx, z0, q, p_lo, p_hi);
    }
    // 
    let edx = (xb & 0x7f80_0000).wrapping_sub(0x3f00_0000);
    if edx <= 0x0080_0000 {
        // 
        let ax = xb & 0x7fff_ffff;
        let t = 1.0f32 - f32::from_bits(ax);
        if t.to_bits() & 0x7f80_0000 == 0 {
            return ok((ecx << 16) | ONE_F);
        }
        let prod = yd * z0;
        if ((prod.to_bits() >> 48) as u32) & 0x7ff0 < 0x4070 {
            return poly2(yd, ecx, z0, q, p_lo, p_hi);
        }
    }
    // 
    let sgn = ((xb & 0x7f80_0000).wrapping_sub(0x3f80_0000) ^ yb) & 0x8000_0000;
    if sgn == 0 {
        // overflow, +-inf
        let bits = ((ecx | 0x7f7f) & 0xffff) << 16;
        let v = f32::from_bits(bits);
        er((v + v).to_bits(), 0x1e)
    } else {
        // underflow, +-0
        er((ecx & 0xffff) << 16, 0x1f)
    }
}

/// second evaluation order, then the result class test .
fn poly2(yd: f64, ecx: u32, z0: f64, q: f64, p_lo: f64, p_hi: f64) -> Out {
    let r4 = q * q;
    let pl = p_lo * r4;
    let ph = p_hi * q;
    let zz = (z0 + ph) * yd;
    let ex = yd * pl;
    let bits = finish(zz, ex, ecx);
    let hi = (bits >> 16) & 0x7fff;
    if hi >= 0x7f80 {
        er(bits, 0x1e)
    } else if hi < 0x10 {
        er(bits, 0x1f)
    } else {
        ok(bits)
    }
}

/// x is not a positive normal float.
fn special(xb: u32, yb: u32, yd: f64) -> Out {
    let ybits = yd.to_bits();
    let yabs_hi = ((ybits & 0x7fff_ffff_ffff_ffff) >> 32) as u32;
    if yabs_hi >= 0x7ff0_0000 {
        // 
        if (xb & 0x7fff_ffff) > INF_F {
            return ok(quiet(xb));
        }
        return ycase(xb, yb);
    }
    if (ybits as u32 | yabs_hi) == 0 {
        return yzero(xb);
    }
    if xb <= 0x7f7f_ffff {
        return zero_or_denormal(xb, yb, 0);
    }
    let e = ((yb >> 23) & 0xff) as i32 - 0x7f;
    let cnt: u32 = if e >= 0 { (e + 8).min(32) as u32 } else { 0 };
    let mant = ((yb & 0x7f_ffff) | 0x80_0000) as u64;
    let eax = (mant << cnt) as u32;
    let ax = xb & 0x7fff_ffff;
    if ax >= INF_F {
        // +-inf or NaN
        let ypos = (ybits >> 48) & 0x8000 == 0;
        if xb == INF_F {
            return if ypos { ok(INF_F) } else { ok(0) };
        }
        if ax > INF_F {
            return ok(quiet(xb));
        }
        // -inf
        return if eax == 0x8000_0000 {
            if ypos { ok(0xff80_0000) } else { ok(0x8000_0000) }
        } else if ypos {
            ok(INF_F)
        } else {
            ok(0)
        };
    }
    if eax & 0x7fff_ffff != 0 {
        // negative base, non integer exponent
        if ax == 0 {
            if yb & 0x8000_0000 == 0 {
                return ok(0);
            }
            return er(INF_F, 0x21);
        }
        //  = 0x7f800001 (signalling NaN) squared: quiet NaN with payload 1
        return er(0x7fc0_0001, 0x22);
    }
    let ecx = eax >> 16;
    if ax < 0x80_0000 {
        return zero_or_denormal(xb, yb, ecx);
    }
    let h = ((xb >> 16) & 0x7fff) as i32;
    let m_bits = 1.0f64.to_bits() | ((xb as u64) << 29);
    main_path(xb, yb, yd, m_bits, h, ecx)
}

/// x is +-0 or +-denormal (ecx carries the odd integer exponent flag 0x8000 for negative x).
fn zero_or_denormal(xb: u32, yb: u32, ecx: u32) -> Out {
    let xs = f32::from_bits(xb) * f32::from_bits(0x5f80_0000);
    let xsb = xs.to_bits();
    if xsb & 0x7fff_ffff == 0 {
        return zero_case(xb, yb, ecx);
    }
    let hi = (xsb >> 16) & 0xffff;
    let m_bits = 1.0f64.to_bits() | ((xsb as u64) << 29);
    let h = (hi & 0x7fff) as i32 - 0x2000;
    let yd = f32::from_bits(yb) as f64;
    main_path(xb, yb, yd, m_bits, h, ecx)
}

/// x is +-0 (ecx<<16 selects the sign of the result).
fn zero_case(xb: u32, yb: u32, ecx: u32) -> Out {
    let m = ecx << 16;
    if yb & 0x8000_0000 != 0 {
        er((xb & m) | INF_F, 0x21)
    } else if xb & m != 0 {
        ok(0x8000_0000)
    } else {
        ok(0)
    }
}

/// y is +-inf or NaN, x is not NaN.
fn ycase(xb: u32, yb: u32) -> Out {
    if (yb & 0x7fff_ffff) > INF_F {
        return ok(quiet(yb));
    }
    let ecx = yb & 0x8000_0000;
    if xb & 0x7fff_ffff == 0 {
        return zero_case(xb, yb, ecx);
    }
    if xb ^ 0xbf80_0000 == 0 {
        return ok(ONE_F); // 
    }
    if ecx == 0 {
        // y = +inf
        if (xb >> 16) & 0x7f80 >= 0x3f80 {
            ok(INF_F)
        } else {
            ok(0)
        }
    } else if xb & 0x7fff_ffff >= ONE_F {
        ok(0)
    } else {
        ok(INF_F)
    }
}

/// y is +-0.
fn yzero(xb: u32) -> Out {
    if xb == 0 {
        return er(ONE_F, 0x20);
    }
    if xb & 0x7fff_ffff > INF_F {
        return er(ONE_F, 0x23);
    }
    ok(ONE_F)
}
