//! Binary float operations with a fixed NaN rule.
//!
//! A hardware add, sub or mul with two NaN operands returns one of them, and which one depends on the operand
//! order the compiler happens to emit (addition and multiplication are commutative, so the order may be swapped).
//! These helpers pin the rule of the original instructions (the first source operand, quieted, wins; a fresh NaN is
//! the default NaN 0xffc00000) so the result does not depend on code generation.

pub trait NanOps: Sized {
    fn fdiv(self, b: Self) -> Self;
    fn fadd(self, b: Self) -> Self;
    fn fsub(self, b: Self) -> Self;
    fn fmul(self, b: Self) -> Self;
}

#[cold]
#[inline(never)]
fn pick(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        f32::from_bits(a.to_bits() | 0x0040_0000)
    } else if b.is_nan() {
        f32::from_bits(b.to_bits() | 0x0040_0000)
    } else {
        f32::from_bits(0xffc0_0000)
    }
}

impl NanOps for f32 {
    #[inline(always)]
    fn fdiv(self, b: f32) -> f32 {
        let r = self / b;
        if r.is_nan() { pick(self, b) } else { r }
    }
    #[inline(always)]
    fn fadd(self, b: f32) -> f32 {
        let r = self + b;
        if r.is_nan() { pick(self, b) } else { r }
    }
    #[inline(always)]
    fn fsub(self, b: f32) -> f32 {
        let r = self - b;
        if r.is_nan() { pick(self, b) } else { r }
    }
    #[inline(always)]
    fn fmul(self, b: f32) -> f32 {
        let r = self * b;
        if r.is_nan() { pick(self, b) } else { r }
    }
}

/// First NaN among the operands in instruction order (quieted), else the default NaN.
#[cold]
#[inline(never)]
fn pick3(a: f32, b: f32, c: f32) -> f32 {
    for v in [a, b, c] {
        if v.is_nan() {
            return f32::from_bits(v.to_bits() | 0x0040_0000);
        }
    }
    f32::from_bits(0xffc0_0000)
}

/// `vfmsub213ss d, s2, s3`: d * s2 - s3 with one rounding. NaN priority d, s2, s3; the subtracted operand is not negated.
#[inline(always)]
pub fn fms213(d: f32, s2: f32, s3: f32) -> f32 {
    let r = d.mul_add(s2, -s3);
    if r.is_nan() { pick3(d, s2, s3) } else { r }
}

/// `vfmadd231ss d, s2, s3`: s2 * s3 + d with one rounding. NaN priority d, s2, s3.
#[inline(always)]
pub fn fma231(d: f32, s2: f32, s3: f32) -> f32 {
    let r = s2.mul_add(s3, d);
    if r.is_nan() { pick3(d, s2, s3) } else { r }
}

/// True when any element is a NaN. A branch free reduction the compiler can vectorize.
#[inline]
pub fn has_nan(s: &[f32]) -> bool {
    s.iter().fold(false, |a, &x| a | (x != x))
}

/// A NaN result that no operand supplied is the default NaN; every other value is unchanged.
#[inline(always)]
fn canon(r: f32) -> f32 {
    if r != r { f32::from_bits(0xffc0_0000) } else { r }
}

/// `out[i] = out[i] * k` for every element, with the rule of [`NanOps::fmul`]. When no operand is a NaN a result can only be
/// a fresh NaN (infinity times zero), so a plain loop plus a canonicalizing select gives the same bits and vectorizes.
pub fn scale_in_place(out: &mut [f32], k: f32) {
    if !k.is_nan() && !has_nan(out) {
        for v in out.iter_mut() {
            *v = canon(*v * k);
        }
    } else {
        for v in out.iter_mut() {
            *v = v.fmul(k);
        }
    }
}

/// `dst[i] = src[i] * k` for the first `dst.len()` elements, same rule as [`scale_in_place`].
pub fn scale_into(dst: &mut [f32], src: &[f32], k: f32) {
    let src = &src[..dst.len()];
    if !k.is_nan() && !has_nan(src) {
        for (d, &x) in dst.iter_mut().zip(src) {
            *d = canon(x * k);
        }
    } else {
        for (d, &x) in dst.iter_mut().zip(src) {
            *d = x.fmul(k);
        }
    }
}

/// `out[i] = fma231(frac * z[i], c, out[i])`: the subpixel blend of a window sum, element by element.
pub fn blend_in_place(out: &mut [f32], z: &[f32], c: f32, frac: f32) {
    let z = &z[..out.len()];
    if !c.is_nan() && !frac.is_nan() && !has_nan(out) && !has_nan(z) {
        for (o, &zv) in out.iter_mut().zip(z) {
            *o = canon(c.mul_add(*o, frac * zv));
        }
    } else {
        for (o, &zv) in out.iter_mut().zip(z) {
            *o = fma231(frac.fmul(zv), c, *o);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn b(x: u32) -> f32 {
        f32::from_bits(x)
    }

    #[test]
    fn first_nan_operand_wins_and_is_quieted() {
        let (s, q) = (b(0x7f80_0001), b(0xffc0_0000));
        assert_eq!(s.fadd(q).to_bits(), 0x7fc0_0001);
        assert_eq!(q.fadd(s).to_bits(), 0xffc0_0000);
        assert_eq!(s.fsub(q).to_bits(), 0x7fc0_0001);
        assert_eq!(q.fmul(s).to_bits(), 0xffc0_0000);
        assert_eq!(q.fdiv(s).to_bits(), 0xffc0_0000);
        assert_eq!(1.0f32.fadd(s).to_bits(), 0x7fc0_0001);
    }

    #[test]
    fn fresh_nan_is_default_nan() {
        let inf = f32::INFINITY;
        assert_eq!(inf.fsub(inf).to_bits(), 0xffc0_0000);
        assert_eq!(0.0f32.fmul(inf).to_bits(), 0xffc0_0000);
        assert_eq!(0.0f32.fdiv(0.0).to_bits(), 0xffc0_0000);
    }

    #[test]
    fn finite_results_are_plain() {
        assert_eq!(1.5f32.fadd(2.25).to_bits(), (1.5f32 + 2.25).to_bits());
        assert_eq!((-0.0f32).fadd(-0.0).to_bits(), 0x8000_0000);
    }

    #[test]
    fn fused_priority_is_destination_then_sources() {
        let (a, c) = (b(0x7fc0_0001), b(0x7fc0_0002));
        assert_eq!(fms213(c, a, 1.0).to_bits(), 0x7fc0_0002);
        assert_eq!(fma231(1.0, a, c).to_bits(), 0x7fc0_0001);
        assert_eq!(fma231(c, a, 1.0).to_bits(), 0x7fc0_0002);
        assert_eq!(fma231(1.0, f32::INFINITY, 0.0).to_bits(), 0xffc0_0000);
    }

    #[test]
    fn bulk_helpers_equal_the_per_element_rule() {
        let specials = [0x7f80_0001u32, 0xffc0_0000, 0x7fc0_0002, 0x7f80_0000, 0xff80_0000, 0, 0x8000_0000, 0x3f80_0000, 0x7f7f_ffff, 0x0000_0001, 0xc0a0_0000];
        let mut st = 0x1234_5678_9abc_def1u64;
        let mut next = || {
            st = st.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (st >> 33) as usize
        };
        for round in 0..4000 {
            let n = 1 + next() % 19;
            let nan_rate = [0usize, 3, 9, 1][round % 4];
            let mut gen = |r: &mut dyn FnMut() -> usize| -> f32 {
                let v = r();
                if nan_rate != 0 && v % nan_rate == 0 { b(specials[v % 3]) } else if v % 5 == 0 { b(specials[v % specials.len()]) } else { (v % 1000) as f32 / 37.0 - 9.0 }
            };
            let src: Vec<f32> = (0..n).map(|_| gen(&mut next)).collect();
            let z: Vec<f32> = (0..n).map(|_| gen(&mut next)).collect();
            let k = if round % 7 == 0 { b(0x7f80_0001) } else { gen(&mut next) };
            let c = gen(&mut next);
            let mut a = src.clone();
            scale_in_place(&mut a, k);
            let mut d = vec![0f32; n];
            scale_into(&mut d, &src, k);
            let mut o = src.clone();
            blend_in_place(&mut o, &z, c, k);
            for i in 0..n {
                assert_eq!(a[i].to_bits(), src[i].fmul(k).to_bits());
                assert_eq!(d[i].to_bits(), src[i].fmul(k).to_bits());
                assert_eq!(o[i].to_bits(), fma231(k.fmul(z[i]), c, src[i]).to_bits());
            }
        }
    }
}
