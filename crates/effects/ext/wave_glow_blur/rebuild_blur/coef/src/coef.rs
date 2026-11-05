//! Radii, coefficients and final scale of driver (SPEC section 3, steps 5 to 8). All arithmetic follows the
//! listing: f32 for sigma, frac and the ratio, f64 for the coefficient product and the accumulator.
use crate::x86::{cvtss2si32_rne, cvttsd2si32, cvttss2si32};

/// Per pass floor radii A, ceil radii B (entries at or beyond the pass count stay 0) and the fraction.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Radii {
    pub a: [i32; 3],
    pub b: [i32; 3],
    pub frac: f32,
}

/// Pass count of the driver: `arg > 3` gives 3, `arg <= 0` gives 1 .
pub fn clamp_passes(arg: i32) -> i32 {
    let n = if arg > 3 { 3 } else { arg };
    if n > 0 { n } else { 1 }
}

/// Spread `r` over `n` passes: q = (r - used) / (n - i) truncated toward zero, capped at `maxr` (signed cmovge).
fn distribute(r: i32, n: usize, maxr: i32) -> [i32; 3] {
    let mut out = [0i32; 3];
    let mut used = 0i32;
    for i in 0..n {
        let q = r.wrapping_sub(used) / (n - i) as i32;
        let q = if q >= maxr { maxr } else { q };
        out[i] = q;
        used = used.wrapping_add(q);
    }
    out
}

/// Radii for one axis (X section, Y section). Only valid for `sigma > 0` (the driver skips the
/// section for sigma <= 0 and NaN). `extent` is the width (X) or the height, halved by the caller when `half` (Y).
/// `passes` is the already clamped count n (1..=3).
pub fn radii(sigma: f32, extent: i32, passes: i32, subpixel: bool) -> Radii {
    let n = passes.clamp(1, 3) as usize;
    let (r0, r1, frac) = if subpixel {
        let t = cvttss2si32(sigma);
        let frac = sigma - (t as f32);
        let r1 = cvtss2si32_rne(sigma.ceil());
        (t, r1, frac)
    } else {
        let v = cvttsd2si32(sigma as f64 + 0.5);
        (v, v, 0.0f32)
    };
    if extent > 1 {
        let maxr = (extent >> 1) - 1;
        let r0 = if r0 <= 0 { 0 } else { r0 };
        let r1 = if r1 <= 0 { 0 } else { r1 };
        Radii { a: distribute(r0, n, maxr), b: distribute(r1, n, maxr), frac }
    } else {
        Radii { a: [0; 3], b: [0; 3], frac }
    }
}

/// Coefficients C[i] and the updated accumulator for `n` passes (scalar loop; the vector block is dead for
/// n <= 3). `acc_in` is the accumulator before this section (1.0 for the X section, the X result for the Y section).
/// C[i] = (float)((1.0 - (double)frac) * (double)(b_f / a_f)) with a_f = (float)(2A+1), b_f = (float)(2B+1) and the f32
/// division; acc = acc * (double)(2B+1). Entries beyond `n` are 0.
pub fn coeffs(a: &[i32; 3], b: &[i32; 3], frac: f32, n: usize, acc_in: f64) -> ([f32; 3], f64) {
    let one_minus = 1.0f64 - (frac as f64);
    let mut c = [0.0f32; 3];
    let mut acc = acc_in;
    for i in 0..n.min(3) {
        let ai = a[i].wrapping_mul(2).wrapping_add(1);
        let bi = b[i].wrapping_mul(2).wrapping_add(1);
        let q = (bi as f32) / (ai as f32);
        c[i] = (one_minus * (q as f64)) as f32;
        acc *= bi as f64;
    }
    (c, acc)
}

/// Final scale `(float)((double)gain / acc)` .
pub fn final_scale(gain: f32, acc: f64) -> f32 {
    ((gain as f64) / acc) as f32
}
