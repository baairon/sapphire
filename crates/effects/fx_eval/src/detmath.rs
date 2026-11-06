//! Deterministic elementary functions for the effect compiler.
//!
//! The constant folder and the program parser used the platform math library (`f32::ln`, `exp`, `tan`, `sin`, `cos`,
//! `atan`, `atan2`, `f64::powf`, `f32::powi`, `log10`), whose last bit differs between the C libraries of Linux, macOS and
//! Windows. Every function here is computed from its mathematical definition in the 320 bit fixed point of
//! `libm_sincos::exact` (or with exact big integers for powers of ten) and rounded once to nearest even, so the result is
//! the correctly rounded value on every platform. Square root and the basic operations are exact in IEEE 754 and stay.

use libm_sincos::exact::{pi, sincos, Fx};
use std::sync::OnceLock;

/// The quiet NaN the folder returns for an invalid argument (the default NaN of the x86 SSE units).
const QNAN: f32 = f32::from_bits(0xffc0_0000);

/// 2^n as a double (n in the normal range).
fn pow2(n: i32) -> f64 {
    f64::from_bits(((n + 1023) as u64) << 52)
}

/// (m, e) with |x| = m * 2^e, m in [1, 2), for a finite non-zero f32.
fn split(x: f32) -> (f64, i32) {
    let b = x.abs().to_bits();
    let (mant, e) = if b >> 23 == 0 { (b as u64, -149) } else { (((b & 0x7f_ffff) | 0x80_0000) as u64, (b >> 23) as i32 - 150) };
    let lz = 63 - mant.leading_zeros() as i32;
    let m = mant as f64 * pow2(-lz);
    (m, e + lz)
}

fn ln2() -> &'static Fx {
    static C: OnceLock<Fx> = OnceLock::new();
    C.get_or_init(|| atanh(&Fx::from_int(1).div_u64(3)).mul_u64(2))
}

fn two_pi() -> &'static Fx {
    static C: OnceLock<Fx> = OnceLock::new();
    C.get_or_init(|| pi().mul_u64(2))
}

fn inv_two_pi() -> &'static Fx {
    static C: OnceLock<Fx> = OnceLock::new();
    C.get_or_init(|| two_pi().recip())
}

/// a / b for positive b.
fn div(a: &Fx, b: &Fx) -> Fx {
    a.mul(&b.recip())
}

/// atanh(s) = s + s^3/3 + s^5/5 + ... for |s| <= 1/3.
fn atanh(s: &Fx) -> Fx {
    let s2 = s.mul(s);
    let mut p = *s;
    let mut sum = *s;
    let mut k = 1u64;
    loop {
        p = p.mul(&s2);
        k += 2;
        let t = p.div_u64(k);
        if t.is_zero() {
            break;
        }
        sum = sum.add(&t);
    }
    sum
}

/// atan(t) for 0 <= t <= 1 by Euler's series: sum of (2n)!! / (2n+1)!! * t / (1 + t^2) * (t^2 / (1 + t^2))^n.
fn atan_unit(t: &Fx) -> Fx {
    if t.is_zero() {
        return Fx::zero();
    }
    let t2 = t.mul(t);
    let inv = Fx::from_int(1).add(&t2).recip();
    let y = t2.mul(&inv);
    let mut term = t.mul(&inv);
    let mut sum = term;
    for n in 1u64..2000 {
        term = term.mul(&y).mul_u64(2 * n).div_u64(2 * n + 1);
        if term.is_zero() {
            break;
        }
        sum = sum.add(&term);
    }
    sum
}

/// exp(r) for |r| < 1 by its Taylor series.
fn exp_small(r: &Fx) -> Fx {
    let mut term = Fx::from_int(1);
    let mut sum = term;
    for k in 1u64..400 {
        term = term.mul(r).div_u64(k);
        if term.is_zero() {
            break;
        }
        sum = sum.add(&term);
    }
    sum
}

/// The angle x reduced into [-pi, pi], as a fixed point value.
fn reduce(x: f32) -> Fx {
    let (m, e) = split(x);
    // x = M * 2^E with M a 24 bit integer
    let mi = (m * pow2(23)) as u64;
    let ee = e - 23;
    let f = if ee >= 0 {
        inv_two_pi().frac_shl(ee as u32).mul_u64(mi).frac()
    } else {
        Fx::from_f64(x.abs() as f64).mul(inv_two_pi()).frac()
    };
    let mut a = f.mul(two_pi());
    if a.gt(pi()) {
        a = a.sub(two_pi());
    }
    if x < 0.0 { a.neg() } else { a }
}

/// Correctly rounded natural logarithm.
pub fn ln(x: f32) -> f32 {
    if x.is_nan() {
        return x + x;
    }
    if x < 0.0 {
        return QNAN;
    }
    if x == 0.0 {
        return f32::NEG_INFINITY;
    }
    if x.is_infinite() {
        return x;
    }
    let (m, e) = split(x);
    let mf = Fx::from_f64(m);
    let one = Fx::from_int(1);
    let s = div(&mf.sub(&one), &mf.add(&one));
    let lm = atanh(&s).mul_u64(2);
    let le = ln2().mul_u64(e.unsigned_abs() as u64);
    let v = if e < 0 { lm.sub(&le) } else { lm.add(&le) };
    v.to_f32_scaled(0)
}

/// Correctly rounded exponential.
pub fn exp(x: f32) -> f32 {
    if x.is_nan() {
        return x + x;
    }
    if x > 89.0 {
        return f32::INFINITY;
    }
    if x < -104.0 {
        return 0.0;
    }
    let xf = Fx::from_f64(x as f64);
    let q = div(&xf.abs(), ln2()).to_f64();
    let k = (q + 0.5).floor() as i64 * if x < 0.0 { -1 } else { 1 };
    let r = xf.sub(&ln2().mul_u64(k.unsigned_abs()).neg_if(k < 0));
    exp_small(&r).to_f32_scaled(k as i32)
}

trait NegIf {
    fn neg_if(self, c: bool) -> Self;
}
impl NegIf for Fx {
    fn neg_if(self, c: bool) -> Fx {
        if c { self.neg() } else { self }
    }
}

/// Correctly rounded (sin x, cos x).
pub fn sin_cos(x: f32) -> (f32, f32) {
    if !x.is_finite() {
        return (if x.is_nan() { x + x } else { QNAN }, if x.is_nan() { x + x } else { QNAN });
    }
    if x == 0.0 {
        return (x, 1.0);
    }
    let (s, c) = sincos(&reduce(x));
    (s.to_f32_scaled(0), c.to_f32_scaled(0))
}

/// Correctly rounded sine.
pub fn sin(x: f32) -> f32 {
    sin_cos(x).0
}

/// Correctly rounded cosine.
pub fn cos(x: f32) -> f32 {
    sin_cos(x).1
}

/// Correctly rounded tangent.
pub fn tan(x: f32) -> f32 {
    if !x.is_finite() {
        return if x.is_nan() { x + x } else { QNAN };
    }
    if x == 0.0 {
        return x;
    }
    let (s, c) = sincos(&reduce(x));
    let q = s.mul(&c.abs().recip());
    let q = if c.is_neg() { q.neg() } else { q };
    q.to_f32_scaled(0)
}

/// atan of a non-negative ratio num / den (den > 0), both given as f32 magnitudes, as a fixed point angle in [0, pi/2].
fn atan_ratio(num: f32, den: f32) -> Fx {
    let half_pi = pi().div_u64(2);
    let (lo, hi, swap) = if num <= den { (num, den, false) } else { (den, num, true) };
    let (_, eh) = split(hi);
    let ls = lo as f64 * pow2(-eh);
    let hs = hi as f64 * pow2(-eh);
    let t = if lo == 0.0 || ls < pow2(-240) { Fx::zero() } else { Fx::from_f64(ls).mul(&Fx::from_f64(hs).recip()) };
    let a = atan_unit(&t);
    if swap { half_pi.sub(&a) } else { a }
}

/// Correctly rounded arctangent.
pub fn atan(x: f32) -> f32 {
    if x.is_nan() {
        return x + x;
    }
    if x == 0.0 {
        return x;
    }
    let a = if x.is_infinite() { pi().div_u64(2) } else { atan_ratio(x.abs(), 1.0) };
    if x.abs() < 1.0 && x.abs() < pow2(-100) as f32 {
        // atan(x) = x - x^3/3: x itself is the correctly rounded value
        return x;
    }
    (if x < 0.0 { a.neg() } else { a }).to_f32_scaled(0)
}

/// Correctly rounded atan2(y, x), with the C99 special cases.
pub fn atan2(y: f32, x: f32) -> f32 {
    if x.is_nan() || y.is_nan() {
        return x + y;
    }
    let sign = |v: Fx| if y.is_sign_negative() { v.neg() } else { v };
    let p = *pi();
    let half = p.div_u64(2);
    if y == 0.0 {
        return if x.is_sign_negative() { sign(p).to_f32_scaled(0) } else { y };
    }
    if x == 0.0 {
        return sign(half).to_f32_scaled(0);
    }
    if y.is_infinite() {
        let a = if x.is_infinite() { if x > 0.0 { p.div_u64(4) } else { p.mul_u64(3).div_u64(4) } } else { half };
        return sign(a).to_f32_scaled(0);
    }
    if x.is_infinite() {
        return if x > 0.0 { if y < 0.0 { -0.0 } else { 0.0 } } else { sign(p).to_f32_scaled(0) };
    }
    let a = atan_ratio(y.abs(), x.abs());
    let a = if x < 0.0 { p.sub(&a) } else { a };
    if a.is_zero() {
        return if y < 0.0 { -0.0 } else { 0.0 };
    }
    // a tiny positive result below the fixed point range of the ratio: |y| / x rounds as itself
    sign(a).to_f32_scaled(0)
}

// ------------------------------------------------------------------------------------------------
// powers of ten with exact big integers

#[derive(Clone, Debug, PartialEq, Eq)]
struct Big(Vec<u32>);

impl Big {
    fn from_u64(v: u64) -> Big {
        let mut b = Big(vec![v as u32, (v >> 32) as u32]);
        b.trim();
        b
    }
    fn trim(&mut self) {
        while self.0.last() == Some(&0) {
            self.0.pop();
        }
    }
    fn mul_small(&mut self, k: u32) {
        let mut c = 0u64;
        for w in self.0.iter_mut() {
            let t = *w as u64 * k as u64 + c;
            *w = t as u32;
            c = t >> 32;
        }
        if c > 0 {
            self.0.push(c as u32);
        }
    }
    fn pow10(n: u32) -> Big {
        let mut b = Big::from_u64(1);
        for _ in 0..n {
            b.mul_small(10);
        }
        b
    }
    fn bits(&self) -> i64 {
        match self.0.last() {
            None => 0,
            Some(&t) => (self.0.len() as i64 - 1) * 32 + 32 - t.leading_zeros() as i64,
        }
    }
    fn shl(&self, n: u64) -> Big {
        let (w, b) = ((n / 32) as usize, (n % 32) as u32);
        let mut r = vec![0u32; w];
        let mut c = 0u32;
        for &x in &self.0 {
            r.push(if b == 0 { x } else { (x << b) | c });
            c = if b == 0 { 0 } else { x >> (32 - b) };
        }
        r.push(c);
        let mut r = Big(r);
        r.trim();
        r
    }
    fn cmp(&self, o: &Big) -> std::cmp::Ordering {
        if self.0.len() != o.0.len() {
            return self.0.len().cmp(&o.0.len());
        }
        for i in (0..self.0.len()).rev() {
            if self.0[i] != o.0[i] {
                return self.0[i].cmp(&o.0[i]);
            }
        }
        std::cmp::Ordering::Equal
    }
    fn sub(&mut self, o: &Big) {
        let mut br = 0i64;
        for i in 0..self.0.len() {
            let d = self.0[i] as i64 - o.0.get(i).copied().unwrap_or(0) as i64 - br;
            if d < 0 {
                self.0[i] = (d + (1i64 << 32)) as u32;
                br = 1;
            } else {
                self.0[i] = d as u32;
                br = 0;
            }
        }
        self.trim();
    }
    fn is_zero(&self) -> bool {
        self.0.is_empty()
    }
}

/// floor(num / den) as u64 (the quotient must fit) and whether a remainder is left.
fn divmod(num: &Big, den: &Big) -> (u64, bool) {
    let mut rem = num.clone();
    let mut q = 0u64;
    let nb = num.bits() - den.bits();
    for s in (0..=nb.max(0)).rev() {
        let d = den.shl(s as u64);
        q <<= 1;
        if rem.cmp(&d) != std::cmp::Ordering::Less {
            rem.sub(&d);
            q |= 1;
        }
    }
    (q, !rem.is_zero())
}

/// num / den (both positive) rounded to nearest even with `prec` significant bits and the given minimum normal exponent,
/// as an exact double (the caller converts it to its own format). None when the value overflows the exponent `emax`.
fn round_ratio(num: &Big, den: &Big, prec: i64, emin: i64, emax: i64) -> Option<f64> {
    // e: 2^e <= num/den < 2^(e+1)
    let mut e = num.bits() - den.bits();
    let ge = |e: i64| if e >= 0 { num.cmp(&den.shl(e as u64)) != std::cmp::Ordering::Less } else { num.shl((-e) as u64).cmp(den) != std::cmp::Ordering::Less };
    if !ge(e) {
        e -= 1;
    }
    if e > emax {
        return None;
    }
    let p = if e >= emin { prec } else { prec - (emin - e) };
    // q = floor(num * 2^(p - e) / den): p + 1 bits including the half bit (p may be 0 or less for deep underflow)
    let sh = p - e;
    let (n2, d2) = if sh >= 0 { (num.shl(sh as u64), den.clone()) } else { (num.clone(), den.shl((-sh) as u64)) };
    if p < -1 {
        return Some(0.0);
    }
    let (q, sticky) = divmod(&n2, &d2);
    let mut m = q >> 1;
    let half = q & 1 == 1;
    if half && (sticky || m & 1 == 1) {
        m += 1;
    }
    let k = (e - p + 1) as i32;
    // m * 2^k is representable after the rounding; two exact steps keep 2^k inside the normal range
    let v = if k < -1000 { m as f64 * pow2(k + 600) * pow2(-600) } else { m as f64 * pow2(k) };
    Some(v)
}

/// The correctly rounded double of 10^e.
pub fn pow10_f64(e: i32) -> f64 {
    let n = Big::pow10(e.unsigned_abs());
    let one = Big::from_u64(1);
    let r = if e >= 0 { round_ratio(&n, &one, 53, -1022, 1023) } else { round_ratio(&one, &n, 53, -1022, 1023) };
    r.unwrap_or(f64::INFINITY)
}

/// The correctly rounded single of 10^e.
pub fn pow10_f32(e: i32) -> f32 {
    let n = Big::pow10(e.unsigned_abs());
    let one = Big::from_u64(1);
    let r = if e >= 0 { round_ratio(&n, &one, 24, -126, 127) } else { round_ratio(&one, &n, 24, -126, 127) };
    r.map(|v| v as f32).unwrap_or(f32::INFINITY)
}

/// floor(log10(|x|)) for a finite non-zero f32, exact.
pub fn floor_log10(x: f32) -> i32 {
    let (m, e) = split(x);
    let mant = (m * pow2(52)) as u64;
    // |x| = mant * 2^(e - 52) = num / den
    let (num, den) = if e - 52 >= 0 { (Big::from_u64(mant).shl((e - 52) as u64), Big::from_u64(1)) } else { (Big::from_u64(mant), Big::from_u64(1).shl((52 - e) as u64)) };
    // 10^n <= num/den  <=>  num >= den * 10^n (n >= 0), num * 10^-n >= den (n < 0)
    let le = |n: i32| {
        if n >= 0 {
            let mut d = den.clone();
            for _ in 0..n {
                d.mul_small(10);
            }
            num.cmp(&d) != std::cmp::Ordering::Less
        } else {
            let mut q = num.clone();
            for _ in 0..(-n) {
                q.mul_small(10);
            }
            q.cmp(&den) != std::cmp::Ordering::Less
        }
    };
    let mut n = ((e as f64) * 0.30103).floor() as i32;
    while !le(n) {
        n -= 1;
    }
    while le(n + 1) {
        n += 1;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_cases() {
        assert_eq!(ln(1.0), 0.0);
        assert_eq!(exp(0.0), 1.0);
        assert_eq!(pow10_f64(22), 1e22);
        assert_eq!(pow10_f64(-5), 1e-5);
        assert_eq!(pow10_f32(-3), 1e-3f32);
        assert_eq!(floor_log10(999.0), 2);
        assert_eq!(floor_log10(1000.0), 3);
        assert_eq!(floor_log10(0.001), -3);
        assert_eq!(atan2(0.0, -1.0), std::f32::consts::PI);
        assert_eq!(atan(1.0), std::f32::consts::FRAC_PI_4);
    }
}
