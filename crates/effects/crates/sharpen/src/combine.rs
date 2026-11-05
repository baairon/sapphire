//! Combine job (AVX2 body, SP-A2): out = x + g*(x - b) per element, count elements.
//! Listing order (neutral_1.asm): `vucomiss g, 1.0f` (cell); NaN (jp) or not equal
//! takes the general path: `vsubss d = x - b` (rounded), `vfmadd231ss x = d * g + x` (one rounding). Equal takes the unit path:
//! `vfmsub213ss x = x * 2.0f - b` (2.0f), one rounding. No clamp, min, max or abs.

/// 1.0f, in the captured data image (MEASURED, bytes 0000803f; re-read by tests/t1.rs).
pub const ONE: f32 = 1.0;
/// 2.0f, in the captured data image (MEASURED, bytes 00000040; re-read by tests/t1.rs).
pub const TWO: f32 = 2.0;

/// One element. `g == ONE` is the IEEE equality of vucomiss (+0/-0 irrelevant here), a NaN gain is unordered and goes general.
#[inline]
pub fn combine_elem(x: f32, b: f32, g: f32) -> f32 {
    if g == ONE {
        // vfmsub213ss xmm1 = xmm1 * xmm0 - [mem]: x * 2.0 - b with one rounding
        f32::mul_add(x, TWO, -b)
    } else {
        // vsubss d = x - b ; vfmadd231ss xmm1 = d * g + x
        let d = x - b;
        f32::mul_add(d, g, x)
    }
}

/// The job over `count` elements (single worker: start 0, count = n, B-A6). `count <= 0` writes nothing.
/// `dst` may be a different slice than `b` or `x`; the caller handles aliasing by value (the listing reads b[i], x[i] before writing o[i]).
pub fn combine(x: &[f32], b: &[f32], g: f32, dst: &mut [f32], count: usize) -> Result<(), fx_common::plane::FxError> {
    if x.len() < count || b.len() < count || dst.len() < count {
        return Err(fx_common::plane::FxError::Stride);
    }
    for i in 0..count {
        dst[i] = combine_elem(x[i], b[i], g);
    }
    Ok(())
}
