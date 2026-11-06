//! bit exact port of the original compiler's `sin` / `cos` constant fold kernel.
//!
//! The fold of the primitive classes `sin` and `cos` is NOT libm: the class kernel (vtable slot 53, for sin and
//!  for cos) multiplies the argument by TWO_PI (f32 0x40c90fdb), broadcasts it into an xmm register and
//! calls the Intel SVML 4 lane float routines through lazily bound stubs: `sin`stub -> variant (dispatch
//! table indices 6 and 7), `cos`stub -> variant (indices 6 and 7). Those SSE variants are ported here. All
//! four lanes hold the same value in the fold, so the lane mixing branches of the routine (eax masks other than 0 and 0xf) are not
//! reachable and are not ported. Listing offsets below (VA = offset).
//!
//! Regimes (xa = |x| as integer bits, compared as signed 32 bit integers, as pcmpgtd does):
//!   fast    xa <= 0x461c4000 (10000.0)   single precision Cody-Waite by pi, degree 9 odd polynomial
//!   medium  10000 < |x| <= 2^30          double precision reduction by pi/2 (5 part constant), even/odd n, same polynomial
//!   huge xa > 0x4e800000 (2^30), inf, NaN scalar routine (table driven, 64 bit reduction)
//! Data: the 24 vector constants are CAPTURED. The scalar routine tables (2304 B each) are computed at first use (pair region, 1/pi chunks and
//! three constants, see `libm_sincos::exact::svml_a`); no file is read.

const TWO_PI: f32 = f32::from_bits(0x40c90fdb); // 

// shared f32 constants (sin:, cos:; identical values)
const POLY_A: f32 = f32::from_bits(0x362f0519); // 
const POLY_B: f32 = f32::from_bits(0xb94fbaf1); // 
const POLY_C: f32 = f32::from_bits(0x3c088773); // 
const POLY_D: f32 = f32::from_bits(0xbe2aaaa5); // 
const PI_1: f32 = f32::from_bits(0x40490000); // 
const PI_2: f32 = f32::from_bits(0x3a7da000); // 
const PI_3: f32 = f32::from_bits(0x34222000); // 
const PI_4: f32 = f32::from_bits(0x2cb4611a); // 
const INV_PI: f32 = f32::from_bits(0x3ea2f983); // 
const HALF_PI_F: f32 = f32::from_bits(0x3fc90fdb); // cos only, 
const HALF: f32 = f32::from_bits(0x3f000000); // cos only, 
const T1_BITS: i32 = 0x461c4000; // 10000.0, 
const T2_BITS: i32 = 0x4e800000; // 2^30, 

// shared f64 constants 
const TWO_OVER_PI: f64 = f64::from_bits(0x3fe45f306dc9c883);
const C1: f64 = f64::from_bits(0x3ff921fa00000000);
const C2: f64 = f64::from_bits(0x3eb5444300000000);
const C3: f64 = f64::from_bits(0xbd473dca00000000);
const C4: f64 = f64::from_bits(0xbc03b39a00000000);
const C5: f64 = f64::from_bits(0x3a945c06e0e68948);
const MAGIC_52: f64 = f64::from_bits(0x4338000000000000); // 1.5 * 2^52

/// The scalar routine's table, computed at first use (module `libm_sincos::exact`, `svml_a`): the pair region (bytes 0..0x410,
/// formula pairs of sin(j pi/128) and cos(j pi/128) scaled by (pi/128)^2/2 and (pi/128)^3/6), the 15 chunk pairs of 1/pi
/// (0x410..0x500) and the three constants read at 0x540, 0x548 and 0x550. These are all the bytes any reader of the 2304 byte
/// table of the original ever touches; the other bytes of that table are never read and are not reproduced. MEASURED against the stored copies
/// (tests/t10_sin1.rs with the copies under VEC_DIR): every word of the computed regions equals the copies.
struct Tab {
    is_cos: bool,
}

impl Tab {
    fn rd(&self, off: usize) -> u64 {
        let a = libm_sincos::exact::svml_a();
        let w = off / 8;
        if w < 130 {
            let pairs = if self.is_cos { &a.cos_pairs } else { &a.sin_pairs };
            return pairs[w / 2][w % 2];
        }
        if off >= 0x410 && off < 0x500 {
            return a.chunks[(off - 0x410) / 16][(off / 8) % 2];
        }
        match off {
            0x540 => a.c540,
            0x548 => a.c548,
            0x550 => 0xffff_ffff_ff00_0000,
            _ => panic!("svml_trig: table byte {:#x} is not part of the computed regions", off),
        }
    }
}

/// polynomial tail shared by the three regimes: r2 = r * r (single precision), r1 = the reduced argument; returns sin(r1) before the sign
#[inline]
fn poly(r2: f32, r1: f32) -> f32 {
    let mut p = POLY_A * r2;
    p += POLY_B;
    p *= r2;
    p += POLY_C;
    p *= r2;
    p += POLY_D;
    p *= r2;
    p *= r1;
    p + r1
}

/// cvtps2dq with the default MXCSR (round to nearest even); inputs here are small
#[inline]
fn cvtps2dq(v: f32) -> i32 {
    let r = v.round_ties_even();
    if r >= 2147483648.0 || r < -2147483648.0 || r.is_nan() { i32::MIN } else { r as i32 }
}

/// fast regime reduction and polynomial: `a` is the value reduced by n * pi (sin: x, cos: |x| with the +pi/2 and -0.5 offsets)
#[inline]
fn fast_tail(a: f32, nf: f32, n: i32) -> f32 {
    let mut r = a - PI_1 * nf;
    r -= PI_2 * nf;
    r -= PI_3 * nf;
    let r1 = r - PI_4 * nf;
    let r2 = r * r;
    let p = poly(r2, r1);
    f32::from_bits(p.to_bits() ^ ((n as u32) << 31))
}

/// double precision pi/2 reduction (the 5 part Cody-Waite sequence; one double lane of each pair): returns r
#[inline]
fn reduce_pio2(a: f64, nd: f64) -> f64 {
    let c1n = C1 * nd;
    let c2n = C2 * nd;
    let t1 = a - c1n;
    let c3n = C3 * nd;
    let t2 = t1 - c2n;
    let mut e = t1 - t2;
    e -= c2n;
    let t3 = t2 - c3n;
    let u = (t2 - t3) - c3n;
    let c4n = C4 * nd;
    e += u;
    let t4 = t3 + e;
    let v = t3 - t4;
    let t5 = t4 - c4n;
    e += v;
    let w = (t4 - t5) - c4n;
    let c5n = C5 * nd;
    e += w;
    e -= c5n;
    e + t5
}

#[inline]
fn quiet_nan_sub(x: f32) -> f32 {
    // subss xmm0,xmm0 on inf or NaN: NaN -> the quieted NaN, inf -> the default NaN (negative sign, quiet bit)
    if x.is_nan() { f32::from_bits(x.to_bits() | 0x0040_0000) } else { f32::from_bits(0xffc0_0000) }
}

/// scalar routine (sin table) and (cos table): same code, different tables
fn scalar_fallback(x: f32, tab: &Tab, is_cos: bool) -> f32 {
    let bits = x.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 {
        return quiet_nan_sub(x);
    }
    let xd = x as f64;
    let ecx = (((bits & 0x7fff_ffff) >> 23).wrapping_sub(0x8e) & 0xfff8) as usize;
    let t3_bits = tab.rd(ecx * 2 + 0x410);
    let t5 = f64::from_bits(tab.rd(ecx * 2 + 0x418));
    let mask = tab.rd(0x550);
    let mut x2 = f64::from_bits(mask & t3_bits);
    let mut x3 = f64::from_bits(t3_bits << 40);
    let mut x1 = xd;
    x2 *= xd;
    x3 *= xd;
    x1 *= t5;
    let x0 = x2;
    x2 += x3;
    let x5 = x2;
    let d = x0 - x2;
    x2 += MAGIC_52;
    x3 += d;
    let ecx = x2.to_bits() as u32;
    x2 -= MAGIC_52;
    x1 += x3;
    let c540 = f64::from_bits(tab.rd(0x540));
    let x5 = x5 - x2;
    let ecx2 = ecx.wrapping_add(ecx);
    let sgn = (((ecx2 << 24) as i32) >> 31) as u32;
    let eax = 0x180u32 & ecx2;
    let edx = (ecx2.wrapping_add(sgn)) ^ sgn;
    x1 += x5;
    let idx = (edx & 0xfe) as usize;
    let c548 = f64::from_bits(tab.rd(0x548));
    let r = x1;
    let r2 = x1 * x1;
    let a3 = f64::from_bits(tab.rd(idx * 8));
    let a5 = f64::from_bits(tab.rd(idx * 8 + 8));
    // sin : cs from (eax), es from eax; cos has the two roles swapped
    let cs = if is_cos { (eax & 0x100) << 23 } else { ((eax.wrapping_add(0x80)) & 0x100) << 23 };
    let mut y3 = a3 * r;
    let mut y0 = c548 - r2;
    let es = if is_cos { ((eax.wrapping_add(0x80)) & 0x100) << 23 } else { (eax & 0x100) << 23 };
    let mut y4 = c540 - r2;
    y4 = f64::from_bits(y4.to_bits() ^ ((cs as u64) << 32));
    y0 *= a5;
    y3 *= y4;
    y0 = f64::from_bits(y0.to_bits() ^ ((es as u64) << 32));
    y0 += y3;
    y0 as f32
}

/// medium regime for sin (broadcast input): xa in (10000, 2^30]
fn sin_medium(x: f32) -> f32 {
    let a = f32::from_bits(x.to_bits() & 0x7fff_ffff) as f64;
    let q = (a * TWO_OVER_PI) as i32; // cvttpd2dq, truncation
    let n = q.wrapping_add(1) & !1;
    let nd = n as f64;
    let r = reduce_pio2(a, nd) as f32;
    let p = poly(r * r, r);
    let s = ((n as u32 >> 1) & 1) << 31; // psllq 62 selects bit 1 of n as the sign
    f32::from_bits(p.to_bits() ^ s ^ (x.to_bits() & 0x8000_0000))
}

/// the original `sin` library routine (SVML sinf4 SSE variant, all four lanes equal to `x`)
pub fn sinf_svml(x: f32) -> f32 {
    let xa = (x.to_bits() & 0x7fff_ffff) as i32;
    if xa <= T1_BITS {
        let nf_i = cvtps2dq(x * INV_PI);
        let nf = nf_i as f32;
        return fast_tail(x, nf, nf_i);
    }
    if xa > T2_BITS {
        return scalar_fallback(x, &Tab { is_cos: false }, false);
    }
    sin_medium(x)
}

/// the original `cos` library routine (SVML cosf4 SSE variant, all four lanes equal to `x`)
pub fn cosf_svml(x: f32) -> f32 {
    let xa_bits = x.to_bits() & 0x7fff_ffff;
    let xa = f32::from_bits(xa_bits);
    if (xa_bits as i32) <= T1_BITS {
        let n = cvtps2dq((xa + HALF_PI_F) * INV_PI);
        let nf = (n as f32) - HALF;
        return fast_tail(xa, nf, n);
    }
    if (xa_bits as i32) > T2_BITS {
        return scalar_fallback(xa, &Tab { is_cos: true }, true);
    }
    let a = xa as f64;
    let q = (a * TWO_OVER_PI) as i32;
    let n = q | 1;
    let r = reduce_pio2(a, n as f64) as f32;
    let p = poly(r * r, r);
    let s = (((n as u32 >> 1) & 1) ^ 1) << 31;
    f32::from_bits(p.to_bits() ^ s)
}

/// the compiler fold of the class `sin`: argument in revolutions
pub fn sin_rev(x: f32) -> f32 {
    sinf_svml(TWO_PI * x)
}

/// the compiler fold of the class `cos`: argument in revolutions
pub fn cos_rev(x: f32) -> f32 {
    cosf_svml(TWO_PI * x)
}
