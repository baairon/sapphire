//! Exact elementary values, computed from the mathematical definitions with a small fixed-point big number.
//!
//! Value = sign * mag / 2^320 with six 64 bit limbs (five fraction limbs and one integer limb). pi comes from Machin's formula,
//! sin and cos from their Taylor series, both far beyond the 2^-110 needed to round a double and to split it into a tail.
//! No table, no constant of any other program and no crate is used. Every conversion to a double rounds to nearest even.

use std::sync::OnceLock;

const L: usize = 6;
const FRAC_LIMBS: usize = 5;
const FRAC_BITS: i32 = 320;

#[derive(Clone, Copy, Debug)]
pub struct Fx {
    neg: bool,
    m: [u64; L],
}

fn cmp_mag(a: &[u64; L], b: &[u64; L]) -> std::cmp::Ordering {
    for i in (0..L).rev() {
        if a[i] != b[i] {
            return a[i].cmp(&b[i]);
        }
    }
    std::cmp::Ordering::Equal
}

fn add_mag(a: &[u64; L], b: &[u64; L]) -> [u64; L] {
    let mut r = [0u64; L];
    let mut carry = 0u128;
    for i in 0..L {
        let s = a[i] as u128 + b[i] as u128 + carry;
        r[i] = s as u64;
        carry = s >> 64;
    }
    r
}

/// a - b with a >= b
fn sub_mag(a: &[u64; L], b: &[u64; L]) -> [u64; L] {
    let mut r = [0u64; L];
    let mut borrow = 0i128;
    for i in 0..L {
        let mut d = a[i] as i128 - b[i] as i128 - borrow;
        if d < 0 {
            d += 1i128 << 64;
            borrow = 1;
        } else {
            borrow = 0;
        }
        r[i] = d as u64;
    }
    r
}

impl Fx {
    pub fn zero() -> Fx {
        Fx { neg: false, m: [0; L] }
    }
    pub fn from_int(n: i64) -> Fx {
        let mut m = [0u64; L];
        m[FRAC_LIMBS] = n.unsigned_abs();
        Fx { neg: n < 0, m }
    }
    pub fn is_zero(&self) -> bool {
        self.m.iter().all(|&w| w == 0)
    }
    /// self / 2^n (n >= 0, bits shifted out below 2^-320 are dropped)
    pub fn shr(&self, n: u32) -> Fx {
        let mut r = *self;
        for _ in 0..n {
            let mut carry = 0u64;
            for i in (0..L).rev() {
                let v = r.m[i];
                r.m[i] = (v >> 1) | (carry << 63);
                carry = v & 1;
            }
        }
        if r.is_zero() { Fx::zero() } else { r }
    }
    /// The fractional part of a non-negative value.
    pub fn frac(&self) -> Fx {
        let mut r = *self;
        r.m[FRAC_LIMBS] = 0;
        if r.is_zero() { Fx::zero() } else { r }
    }
    /// Keep the leading `n` significant bits (truncation toward zero).
    pub fn trunc_sig(&self, n: i32) -> Fx {
        let mut r = *self;
        if let Some(hb) = self.top_bit() {
            for q in 0..(hb - (n - 1)).max(0) {
                r.m[(q / 64) as usize] &= !(1u64 << (q % 64));
            }
        }
        r
    }
    pub fn neg(mut self) -> Fx {
        if !self.is_zero() {
            self.neg = !self.neg;
        }
        self
    }
    pub fn add(&self, o: &Fx) -> Fx {
        if self.neg == o.neg {
            return Fx { neg: self.neg, m: add_mag(&self.m, &o.m) };
        }
        match cmp_mag(&self.m, &o.m) {
            std::cmp::Ordering::Equal => Fx::zero(),
            std::cmp::Ordering::Greater => Fx { neg: self.neg, m: sub_mag(&self.m, &o.m) },
            std::cmp::Ordering::Less => Fx { neg: o.neg, m: sub_mag(&o.m, &self.m) },
        }
    }
    pub fn sub(&self, o: &Fx) -> Fx {
        self.add(&o.neg())
    }
    pub fn mul(&self, o: &Fx) -> Fx {
        let mut p = [0u64; 2 * L];
        for i in 0..L {
            let mut carry = 0u128;
            for j in 0..L {
                let t = self.m[i] as u128 * o.m[j] as u128 + p[i + j] as u128 + carry;
                p[i + j] = t as u64;
                carry = t >> 64;
            }
            p[i + L] = carry as u64;
        }
        let mut m = [0u64; L];
        m.copy_from_slice(&p[FRAC_LIMBS..FRAC_LIMBS + L]);
        let r = Fx { neg: self.neg != o.neg, m };
        if r.is_zero() { Fx::zero() } else { r }
    }
    pub fn mul_u64(&self, n: u64) -> Fx {
        let mut m = [0u64; L];
        let mut carry = 0u128;
        for i in 0..L {
            let t = self.m[i] as u128 * n as u128 + carry;
            m[i] = t as u64;
            carry = t >> 64;
        }
        Fx { neg: self.neg, m }
    }
    pub fn div_u64(&self, d: u64) -> Fx {
        let mut m = [0u64; L];
        let mut rem = 0u128;
        for i in (0..L).rev() {
            let cur = (rem << 64) | self.m[i] as u128;
            m[i] = (cur / d as u128) as u64;
            rem = cur % d as u128;
        }
        let r = Fx { neg: self.neg, m };
        if r.is_zero() { Fx::zero() } else { r }
    }
    /// Exact value of a finite double (its magnitude must be at least 2^-250 or zero).
    pub fn from_f64(x: f64) -> Fx {
        if x == 0.0 {
            return Fx::zero();
        }
        let b = x.to_bits();
        let e = ((b >> 52) & 0x7ff) as i32;
        let (mant, ex) = if e == 0 { (b & ((1u64 << 52) - 1), -1074) } else { ((b & ((1u64 << 52) - 1)) | (1u64 << 52), e - 1075) };
        let shift = ex + FRAC_BITS;
        assert!(shift >= 0 && shift + 53 <= 64 * L as i32, "from_f64 out of range");
        let mut m = [0u64; L];
        let limb = (shift / 64) as usize;
        let bit = (shift % 64) as u32;
        let v = (mant as u128) << bit;
        m[limb] = v as u64;
        if limb + 1 < L {
            m[limb + 1] = (v >> 64) as u64;
        }
        Fx { neg: (b >> 63) != 0, m }
    }
    fn top_bit(&self) -> Option<i32> {
        for i in (0..L).rev() {
            if self.m[i] != 0 {
                return Some(i as i32 * 64 + 63 - self.m[i].leading_zeros() as i32);
            }
        }
        None
    }
    fn bit(&self, p: i32) -> bool {
        if p < 0 || p >= 64 * L as i32 {
            return false;
        }
        (self.m[(p / 64) as usize] >> (p % 64)) & 1 == 1
    }
    fn any_below(&self, p: i32) -> bool {
        // any bit strictly below position p
        for q in 0..p.max(0).min(64 * L as i32) {
            if self.bit(q) {
                return true;
            }
        }
        false
    }
    /// Correctly rounded (nearest even) double.
    pub fn to_f64(&self) -> f64 {
        let hb = match self.top_bit() {
            None => return 0.0,
            Some(h) => h,
        };
        // keep bits hb..hb-52
        let low = hb - 52; // position of the lowest kept bit
        let mut m53: u64 = 0;
        for i in 0..53 {
            if self.bit(low + i) {
                m53 |= 1u64 << i;
            }
        }
        let half = self.bit(low - 1);
        let sticky = self.any_below(low - 1);
        if half && (sticky || (m53 & 1) == 1) {
            m53 += 1;
        }
        let mut e = hb - FRAC_BITS; // unbiased exponent of the leading bit
        if m53 == 1u64 << 53 {
            m53 >>= 1;
            e += 1;
        }
        assert!(e > -1022 && e < 1023, "to_f64 out of range");
        let bits = (((e + 1023) as u64) << 52) | (m53 & ((1u64 << 52) - 1));
        let v = f64::from_bits(bits);
        if self.neg { -v } else { v }
    }
    /// self * 2^k rounded to nearest even single precision, with subnormal results and overflow to infinity.
    pub fn to_f32_scaled(&self, k: i32) -> f32 {
        let hb = match self.top_bit() {
            None => return if self.neg { -0.0 } else { 0.0 },
            Some(h) => h,
        };
        let e = hb - FRAC_BITS + k;
        if e > 127 {
            return if self.neg { f32::NEG_INFINITY } else { f32::INFINITY };
        }
        let prec = if e >= -126 { 24 } else { 24 - (-126 - e) };
        let low = hb - (prec - 1);
        let mut m: u64 = 0;
        for i in 0..prec.max(0) {
            if self.bit(low + i) {
                m |= 1u64 << i;
            }
        }
        let half = self.bit(low - 1);
        let sticky = self.any_below(low - 1);
        if half && (sticky || (m & 1) == 1) {
            m += 1;
        }
        // m * 2^(low - FRAC_BITS + k), exact in a double
        let ex = low - FRAC_BITS + k;
        let v = (m as f64) * f64::from_bits(((ex + 1023) as u64) << 52);
        let r = v as f32;
        if self.neg { -r } else { r }
    }
    /// The fractional part of self * 2^n for a non-negative self (integer bits shifted out are dropped).
    pub fn frac_shl(&self, n: u32) -> Fx {
        let mut r = self.frac();
        for _ in 0..n {
            let mut carry = 0u64;
            for i in 0..L {
                let v = r.m[i];
                r.m[i] = (v << 1) | carry;
                carry = v >> 63;
            }
            r.m[FRAC_LIMBS] = 0;
        }
        if r.is_zero() { Fx::zero() } else { r }
    }
    /// Whether the value is negative.
    pub fn is_neg(&self) -> bool {
        self.neg
    }
    /// The absolute value.
    pub fn abs(mut self) -> Fx {
        self.neg = false;
        self
    }
    /// Compare with another value.
    pub fn gt(&self, o: &Fx) -> bool {
        let d = self.sub(o);
        !d.is_zero() && !d.neg
    }
    /// 1 / self by Newton iteration (self must be positive and not tiny).
    pub fn recip(&self) -> Fx {
        let mut r = Fx::from_f64(1.0 / self.to_f64());
        let two = Fx::from_int(2);
        for _ in 0..6 {
            r = r.mul(&two.sub(&self.mul(&r)));
        }
        r
    }
}

fn atan_inv(n: u64) -> Fx {
    // atan(1/n) = sum (-1)^j / ((2j+1) n^(2j+1))
    let nn = n * n;
    let mut term = Fx::from_int(1).div_u64(n);
    let mut sum = term;
    let mut k = 1u64;
    loop {
        term = term.div_u64(nn);
        k += 2;
        if term.is_zero() {
            break;
        }
        let t = term.div_u64(k);
        sum = if (k / 2) % 2 == 1 { sum.sub(&t) } else { sum.add(&t) };
    }
    sum
}

/// pi to 320 bits (Machin: 16 atan(1/5) - 4 atan(1/239)).
pub fn pi() -> &'static Fx {
    static P: OnceLock<Fx> = OnceLock::new();
    P.get_or_init(|| atan_inv(5).mul_u64(16).sub(&atan_inv(239).mul_u64(4)))
}

/// (sin a, cos a) by Taylor series, for |a| up to about 3.2.
pub fn sincos(a: &Fx) -> (Fx, Fx) {
    let a2 = a.mul(a);
    let mut s = *a;
    let mut c = Fx::from_int(1);
    let mut ts = *a;
    let mut tc = Fx::from_int(1);
    for k in 1u64..400 {
        tc = tc.mul(&a2).div_u64((2 * k - 1) * (2 * k)).neg();
        ts = ts.mul(&a2).div_u64((2 * k) * (2 * k + 1)).neg();
        if tc.is_zero() && ts.is_zero() {
            break;
        }
        c = c.add(&tc);
        s = s.add(&ts);
    }
    (s, c)
}

/// (sin, cos) of k pi / 32 for any integer k, exact at the axes.
pub fn sincos_k32(k: i64) -> (Fx, Fx) {
    let km = k.rem_euclid(64);
    if km % 16 == 0 {
        let (s, c) = [(0, 1), (1, 0), (0, -1), (-1, 0)][(km / 16) as usize];
        return (Fx::from_int(s), Fx::from_int(c));
    }
    let kk = if km > 32 { km - 64 } else { km };
    let a = pi().mul_u64(kk.unsigned_abs()).div_u64(32);
    let (s, c) = sincos(&a);
    if kk < 0 { (s.neg(), c) } else { (s, c) }
}

/// Round a double to 24 significant bits (nearest even).
pub fn round24(x: f64) -> f64 {
    let b = x.to_bits();
    let sign = b & (1u64 << 63);
    let mag = b & !(1u64 << 63);
    if mag == 0 {
        return x;
    }
    let r = (mag + (1u64 << 28) - 1 + ((mag >> 29) & 1)) & !((1u64 << 29) - 1);
    f64::from_bits(sign | r)
}

/// The value with its significand rounded to one bit (a power of two), ties upward in magnitude.
pub fn coarse(x: f64) -> f64 {
    let b = x.to_bits();
    let sign = b & (1u64 << 63);
    let mag = b & !(1u64 << 63);
    if mag == 0 {
        return 0.0;
    }
    f64::from_bits(sign | ((mag + (1u64 << 51)) & 0xfff0_0000_0000_0000))
}

/// Entry k of the single precision table: bits of (sin, cos) of k pi / 32, correctly rounded.
pub fn stab_entry(k: usize) -> [u64; 2] {
    let (s, c) = sincos_k32(k as i64);
    [s.to_f64().to_bits(), c.to_f64().to_bits()]
}

/// Entry k of the double table, eight words: [w0, w1, sin, cos, w4, w5, w6, w7].
/// sin and cos are correctly rounded; w4 and w5 are the rounding tails (exact value minus the double) rounded to 24 bits;
/// w6 and w7 are cos and -sin rounded to one bit; w0 and w1 are cos - w6 and -sin - w7 rounded to double.
pub fn dtab_entry(k: usize) -> [u64; 8] {
    let (s, c) = sincos_k32(k as i64);
    let s2 = s.to_f64();
    let c2 = c.to_f64();
    let w4 = round24(s.sub(&Fx::from_f64(s2)).to_f64());
    let w5 = round24(c.sub(&Fx::from_f64(c2)).to_f64());
    let w6 = coarse(c2);
    let w7 = coarse(if s2 == 0.0 { 0.0 } else { -s2 });
    let w0 = c.sub(&Fx::from_f64(w6)).to_f64();
    let w1 = s.neg().sub(&Fx::from_f64(w7)).to_f64();
    [w0, w1, s2, c2, w4, w5, w6, w7].map(f64::to_bits)
}

/// Chunk m (0..15) of 1/pi for the 64 bit reduction, as the table's pair (t3, t5).
/// The value is 2^7/pi for m = 0 and 1, and for m >= 2 it is frac(2^p / pi) / 2^(p-7) with p = 7 + 8 (m - 2), the bits of 1/pi
/// from position p on, scaled by 2^7. It is cut in three: x2 = its leading 29 significant bits, x3 = the next bits up to 42
/// significant bits in all, t5 = the rest rounded to a double. t3 holds the top 40 bits of the double x2 and, in its low 24 bits,
/// the top 24 bits of the double x3 (the routine masks one and shifts the other).
pub fn svml_chunk(m: u32) -> [u64; 2] {
    let inv_pi = pi().recip();
    let chunk = if m < 2 {
        inv_pi.mul_u64(128)
    } else {
        let p = 7 + 8 * (m - 2);
        let mut f = inv_pi;
        for _ in 0..p {
            f = f.mul_u64(2).frac();
        }
        f.shr(p - 7)
    };
    let x2 = chunk.trunc_sig(29);
    let x23 = chunk.trunc_sig(42);
    let x3 = x23.sub(&x2);
    let t5 = chunk.sub(&x23);
    let b2 = x2.to_f64().to_bits();
    let b3 = x3.to_f64().to_bits();
    assert!(b2 & 0xff_ffff == 0 && b3 & 0xff_ffff_ffff == 0);
    [(b2 & 0xffff_ffff_ff00_0000) | (b3 >> 40), t5.to_f64().to_bits()]
}

/// The scalar sin/cos routine's table, regions that follow a formula: with h = pi / 128 and th = j h,
/// pair j (j = 0..=64) is (X, Y) with Y = sin(th) h^2 / 2 and X = cos(th) h^3 / 6 (the sin table), and the constants
/// c548 = 2 / h^2, c540 = 6 / h^2. The cos table is the same pairs read from the other end:
/// X_cos(j) = -X_sin(64 - j), Y_cos(j) = Y_sin(64 - j).
pub struct SvmlA {
    /// the 15 chunk pairs (t3, t5) at table bytes 0x410..0x500, see `svml_chunk`
    pub chunks: Vec<[u64; 2]>,
    pub sin_pairs: Vec<[u64; 2]>,
    pub cos_pairs: Vec<[u64; 2]>,
    pub c540: u64,
    pub c548: u64,
}

pub fn svml_a() -> &'static SvmlA {
    static T: OnceLock<SvmlA> = OnceLock::new();
    T.get_or_init(|| {
        let h = pi().div_u64(128);
        let h2 = h.mul(&h);
        let h3 = h2.mul(&h);
        let mut sp = Vec::new();
        for j in 0..=64i64 {
            let (x, y) = if j == 0 {
                (h3.div_u64(6), Fx::zero())
            } else if j == 64 {
                (Fx::zero(), h2.div_u64(2))
            } else {
                let th = pi().mul_u64(j as u64).div_u64(128);
                let (s, c) = sincos(&th);
                (c.mul(&h3).div_u64(6), s.mul(&h2).div_u64(2))
            };
            sp.push([x.to_f64().to_bits(), y.to_f64().to_bits()]);
        }
        let cp: Vec<[u64; 2]> = (0..=64usize)
            .map(|j| {
                let [x, y] = sp[64 - j];
                let xf = f64::from_bits(x);
                [if xf == 0.0 { 0 } else { (-xf).to_bits() }, y]
            })
            .collect();
        let c548 = h2.recip().mul_u64(2);
        let c540 = h2.recip().mul_u64(6);
        let chunks = (0..15).map(svml_chunk).collect();
        SvmlA { chunks, sin_pairs: sp, cos_pairs: cp, c540: c540.to_f64().to_bits(), c548: c548.to_f64().to_bits() }
    })
}
