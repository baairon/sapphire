//! Single-precision sine computed in double precision: 0xb7cc60 (selected by the dispatcher 0xb7d170 when the low feature byte is 0xff;
//! 0xb7cee0 is the same arithmetic on a second copy of the tables). Rounding is to nearest throughout.

use crate::tables::SinTables;

/// 1.5 * 2^52 (immediate at 0xb7ccb3).
pub const MAGIC: f64 = f64::from_bits(0x4338_0000_0000_0000);
/// 128/pi rounded to 24 bits (immediate at 0xb7cc82).
pub const K_128_OVER_PI: f64 = f64::from_bits(0x4044_5f30_6e00_0000);
/// Low correction to 128/pi (immediate at 0xb7ccc6).
pub const C_LOW: f64 = f64::from_bits(0xbe5b_1bbe_ad60_3d8b);
/// 32768/pi^2 (immediate at 0xb7cd2d).
pub const K1: f64 = f64::from_bits(0x40a9_f02f_6222_c720);
/// 98304/pi^2 (immediate at 0xb7ccf6).
pub const K2: f64 = f64::from_bits(0x40c3_7423_899a_1558);
const MAIN_MIN_BITS: u32 = 0x3980_0000;
const MAIN_MAX_BITS: u32 = 0x49ff_ffff;
const TINY_LIMIT_BITS: u32 = 0x3e00_0000;

/// `x - x` of the listing for an infinite or NaN argument: a NaN input comes back quieted, an infinity gives the default NaN.
fn sub_self(x: f32) -> f32 {
    let b = x.to_bits();
    if b & 0x007f_ffff != 0 {
        f32::from_bits(b | 0x0040_0000)
    } else {
        f32::from_bits(0xffc0_0000)
    }
}

/// Sine of `x` with the tables `t`.
pub fn sinf(x: f32, t: &SinTables) -> f32 {
    let bits = x.to_bits();
    let abs = bits & 0x7fff_ffff;
    let r8 = MAIN_MAX_BITS.wrapping_sub(abs);
    let r10 = abs.wrapping_sub(MAIN_MIN_BITS);
    let xd = x as f64;
    let n: u32;
    let g: f64;
    if ((r8 | r10) as i32) < 0 {
        // 0xb7cdad: below 2^-12, above 2^21 or non-finite
        if abs <= TINY_LIMIT_BITS {
            return (xd * t.tiny_scale) as f32;
        }
        if (bits & 0x7f80_0000) == 0x7f80_0000 {
            return sub_self(x);
        }
        // 0xb7cdf5 to 0xb7ce70: exponent group lookup
        let grp = ((((r10 >> 23).wrapping_sub(0x1b)) & 0xfff8) >> 3) as usize;
        let e = t.big[grp];
        let hi = f64::from_bits(e[0] & 0xffff_ffff_ff00_0000);
        let mid = f64::from_bits(e[0] << 40);
        let lo = f64::from_bits(e[1]);
        let hx = hi * xd;
        let mx = mid * xd;
        let xlo = xd * lo;
        let s = hx + mx;
        let e1 = hx - s;
        let s2 = s + MAGIC;
        let err = mx + e1;
        n = (s2.to_bits() & 0xffff_ffff) as u32;
        let nd = s2 - MAGIC;
        let f = s - nd;
        g = (xlo + err) + f;
    } else {
        // 0xb7ccb3 to 0xb7cd1c: one multiply by the split constant
        let tt = K_128_OVER_PI * xd;
        let s2 = tt + MAGIC;
        n = (s2.to_bits() & 0xffff_ffff) as u32;
        let nd = s2 - MAGIC;
        let xc = xd * C_LOW;
        let f = tt - nd;
        g = xc + f;
    }
    // 0xb7cd09 to 0xb7cd8f: sector fold and polynomial
    let n2 = n.wrapping_add(n);
    let b = n2 as u8;
    let m = ((b as i8) >> 7) as u8;
    let babs = b.wrapping_add(m) ^ m;
    let idx = (babs & 0xfe) as usize;
    let eax = n2 & 0x180;
    let s_a = (((eax + 0x80) & 0x100) as u64) << 0x37;
    let s_b = ((eax & 0x100) as u64) << 0x37;
    let ent = t.main[idx >> 1];
    let a = f64::from_bits(ent[0]);
    let bb = f64::from_bits(ent[1]);
    let g2 = g * g;
    let p_cos = (K1 - g2) * bb;
    let poly = f64::from_bits((K2 - g2).to_bits() ^ s_a);
    let p_sin = (a * g) * poly;
    let r = f64::from_bits(p_cos.to_bits() ^ s_b) + p_sin;
    r as f32
}

/// Sine with the computed tables.
pub fn sinf_computed(x: f32) -> f32 {
    sinf(x, SinTables::computed())
}
