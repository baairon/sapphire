//! bit exact port of the original compiler's `sin` / `cos` constant fold kernel.
//!
//! The fold of the primitive classes `sin` and `cos` is NOT libm: the class kernel (vtable slot 53, for sin and
//!  for cos) multiplies the argument by TWO_PI (f32 0x40c90fdb), broadcasts it into an xmm register and
//! calls the Intel SVML 4 lane float routines through lazily bound stubs: `sin`stub -> variant (dispatch
//! table indices 6 and 7), `cos`stub -> variant (indices 6 and 7). Those SSE variants are ported here. All
//! four lanes hold the same value in the fold, so the lane mixing branches of the routine (eax masks other than 0 and 0xf) are not
//! reachable and are not ported. 
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

/// The three single constants of the scalar routine's table.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Single {
    /// 6 / h^2 with h = pi / 128 (the cubic-term subtrahend)
    SixOverHSq,
    /// 2 / h^2 with h = pi / 128 (the linear-term subtrahend)
    TwoOverHSq,
    /// bit mask that keeps the high part of a chunk word
    HighMask,
}

/// Typed access to the scalar routine's table: the sine or cosine pair by index (0..=64), the 1/pi chunk by index (0..15)
/// and the three single constants.
pub(crate) trait SvmlTable {
    fn pair(&self, index: usize) -> [u64; 2];
    fn chunk(&self, index: usize) -> [u64; 2];
    fn single(&self, which: Single) -> u64;
}

/// The scalar routine's table, computed at first use (module `libm_sincos::exact`, `svml_a`): 65 formula pairs of
/// sin(j pi/128) and cos(j pi/128) scaled by (pi/128)^2/2 and (pi/128)^3/6, the 15 chunk pairs of 1/pi and the three
/// single constants. These are all the values the routine reads. MEASURED against the stored copies
/// (tests/t10_sin1.rs with the copies under VEC_DIR): every word of the computed regions equals the copies.
struct Tab {
    is_cos: bool,
}

impl SvmlTable for Tab {
    fn pair(&self, index: usize) -> [u64; 2] {
        let a = libm_sincos::exact::svml_a();
        if self.is_cos { a.cos_pairs[index] } else { a.sin_pairs[index] }
    }
    fn chunk(&self, index: usize) -> [u64; 2] {
        libm_sincos::exact::svml_a().chunks[index]
    }
    fn single(&self, which: Single) -> u64 {
        let a = libm_sincos::exact::svml_a();
        match which {
            Single::SixOverHSq => a.table_coef_a,
            Single::TwoOverHSq => a.table_coef_b,
            Single::HighMask => 0xffff_ffff_ff00_0000,
        }
    }
}

/// polynomial tail shared by the three regimes: r2 = r * r (single precision), r1 = the reduced argument; returns sin(r1) before the sign
#[inline]
fn poly(r2: f32, reduced: f32) -> f32 {
    let mut p = POLY_A * r2;
    p += POLY_B;
    p *= r2;
    p += POLY_C;
    p *= r2;
    p += POLY_D;
    p *= r2;
    p *= reduced;
    p + reduced
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
    let reduced = r - PI_4 * nf;
    let r2 = r * r;
    let p = poly(r2, reduced);
    f32::from_bits(p.to_bits() ^ ((n as u32) << 31))
}

/// double precision pi/2 reduction (the 5 part Cody-Waite sequence; one double lane of each pair): returns r
#[inline]
fn reduce_pio2(a: f64, nd: f64) -> f64 {
    let c1n = C1 * nd;
    let c2n = C2 * nd;
    let reduced_hi = a - c1n;
    let c3n = C3 * nd;
    let sum_2 = reduced_hi - c2n;
    let mut e = reduced_hi - sum_2;
    e -= c2n;
    let diff_3 = sum_2 - c3n;
    let u = (sum_2 - diff_3) - c3n;
    let c4n = C4 * nd;
    e += u;
    let sum_4 = diff_3 + e;
    let v = diff_3 - sum_4;
    let diff_5 = sum_4 - c4n;
    e += v;
    let w = (sum_4 - diff_5) - c4n;
    let c5n = C5 * nd;
    e += w;
    e -= c5n;
    e + diff_5
}

#[inline]
fn quiet_nan_sub(x: f32) -> f32 {
    // subss xmm0,xmm0 on inf or NaN: NaN -> the quieted NaN, inf -> the default NaN (negative sign, quiet bit)
    if x.is_nan() { f32::from_bits(x.to_bits() | 0x0040_0000) } else { f32::from_bits(0xffc0_0000) }
}

/// scalar routine (sin table) and (cos table): same code, different tables
fn scalar_fallback<T: SvmlTable>(x: f32, tab: &T, is_cos: bool) -> f32 {
    let bits = x.to_bits();
    if bits & 0x7f80_0000 == 0x7f80_0000 {
        return quiet_nan_sub(x);
    }
    let xd = x as f64;
    let flag = (((bits & 0x7fff_ffff) >> 23).wrapping_sub(0x8e) & 0xfff8) as usize;
    let [table_bits, chunk_low_word] = tab.chunk(flag >> 3);
    let diff_5 = f64::from_bits(chunk_low_word);
    let mask = tab.single(Single::HighMask);
    let mut tab_hi = f64::from_bits(mask & table_bits);
    let mut tab_lo = f64::from_bits(table_bits << 40);
    let mut sum_1 = xd;
    tab_hi *= xd;
    tab_lo *= xd;
    sum_1 *= diff_5;
    let tab_hi_saved = tab_hi;
    tab_hi += tab_lo;
    let head_sum = tab_hi;
    let d = tab_hi_saved - tab_hi;
    tab_hi += MAGIC_52;
    tab_lo += d;
    let flag = tab_hi.to_bits() as u32;
    tab_hi -= MAGIC_52;
    sum_1 += tab_lo;
    let table_coef_a = f64::from_bits(tab.single(Single::SixOverHSq));
    let head_sum = head_sum - tab_hi;
    let ecx2 = flag.wrapping_add(flag);
    let sgn = (((ecx2 << 24) as i32) >> 31) as u32;
    let quadrant = 0x180u32 & ecx2;
    let copy_mode = (ecx2.wrapping_add(sgn)) ^ sgn;
    sum_1 += head_sum;
    let idx = (copy_mode & 0xfe) as usize;
    let table_coef_b = f64::from_bits(tab.single(Single::TwoOverHSq));
    let r = sum_1;
    let r2 = sum_1 * sum_1;
    let [cubic_word, linear_word] = tab.pair(idx >> 1);
    let cubic_coef = f64::from_bits(cubic_word);
    let from_bits_5 = f64::from_bits(linear_word);
    // sin : cs from (eax), es from eax; cos has the two roles swapped
    let cs = if is_cos { (quadrant & 0x100) << 23 } else { ((quadrant.wrapping_add(0x80)) & 0x100) << 23 };
    let mut prod_3 = cubic_coef * r;
    let mut diff_0 = table_coef_b - r2;
    let es = if is_cos { ((quadrant.wrapping_add(0x80)) & 0x100) << 23 } else { (quadrant & 0x100) << 23 };
    let mut diff_4 = table_coef_a - r2;
    diff_4 = f64::from_bits(diff_4.to_bits() ^ ((cs as u64) << 32));
    diff_0 *= from_bits_5;
    prod_3 *= diff_4;
    diff_0 = f64::from_bits(diff_0.to_bits() ^ ((es as u64) << 32));
    diff_0 += prod_3;
    diff_0 as f32
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

#[cfg(test)]
mod tests {
    use super::*;

    /// The earlier byte-offset accessor, kept as the oracle. Offsets live only here.
    struct Oracle {
        is_cos: bool,
    }

    impl Oracle {
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
                0x540 => a.table_coef_a,
                0x548 => a.table_coef_b,
                0x550 => Tab { is_cos: false }.single(Single::HighMask),
                _ => panic!("table byte {:#x} not computed", off),
            }
        }
    }

    impl SvmlTable for Oracle {
        fn pair(&self, index: usize) -> [u64; 2] {
            [self.rd(index * 16), self.rd(index * 16 + 8)]
        }
        fn chunk(&self, index: usize) -> [u64; 2] {
            [self.rd(0x410 + index * 16), self.rd(0x418 + index * 16)]
        }
        fn single(&self, which: Single) -> u64 {
            self.rd(match which {
                Single::SixOverHSq => 0x540,
                Single::TwoOverHSq => 0x548,
                Single::HighMask => 0x550,
            })
        }
    }

    #[test]
    fn every_index_matches_oracle() {
        for is_cos in [false, true] {
            let (t, o) = (Tab { is_cos }, Oracle { is_cos });
            for i in 0..=64 {
                assert_eq!(t.pair(i), o.pair(i));
            }
            for i in 0..15 {
                assert_eq!(t.chunk(i), o.chunk(i));
            }
            for w in [Single::SixOverHSq, Single::TwoOverHSq, Single::HighMask] {
                assert_eq!(t.single(w), o.single(w));
            }
        }
    }

    #[test]
    fn sweep_bit_equal_to_oracle_path() {
        let mut inputs: Vec<u32> = Vec::new();
        let mut x: u32 = 0x1234_5679;
        for _ in 0..1_200_000 {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            inputs.push(x);
        }
        // region boundaries and their neighbours, both signs
        for b in [T1_BITS as u32, T2_BITS as u32, 0x7f80_0000, 0x7fc0_0000, 0, 1, 0x7f7f_ffff] {
            for d in 0..64u32 {
                for v in [b.wrapping_add(d), b.wrapping_sub(d)] {
                    inputs.push(v);
                    inputs.push(v | 0x8000_0000);
                }
            }
        }
        for bits in inputs {
            let f = f32::from_bits(bits);
            let xa = (bits & 0x7fff_ffff) as i32;
            if xa > T2_BITS {
                let s = scalar_fallback(f, &Oracle { is_cos: false }, false);
                assert_eq!(sinf_svml(f).to_bits(), s.to_bits(), "sin {:08x}", bits);
                let fa = f32::from_bits(bits & 0x7fff_ffff);
                let c = scalar_fallback(fa, &Oracle { is_cos: true }, true);
                assert_eq!(cosf_svml(f).to_bits(), c.to_bits(), "cos {:08x}", bits);
            } else {
                let _ = (sinf_svml(f), cosf_svml(f));
            }
        }
    }
}
