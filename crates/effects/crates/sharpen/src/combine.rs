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

/// [`combine`] for a processor whose default NaN is `0x7fc00000` (`x86_nan`): a NaN result that no operand explains is the default NaN
/// of an invalid operation (`inf - inf`), which is set to the x86 default NaN `0xffc00000`. A NaN operand is propagated as it is
/// (`b` has been canonicalized by the caller, so the sign flip of `x * 2 - b` gives the x86 pattern).
pub fn combine_x86_nan(x: &[f32], b: &[f32], g: f32, dst: &mut [f32], count: usize) -> Result<(), fx_common::plane::FxError> {
    combine(x, b, g, dst, count)?;
    if !g.is_nan() {
        for i in 0..count {
            if dst[i].to_bits() == 0x7fc0_0000 && !x[i].is_nan() && !b[i].is_nan() {
                dst[i] = f32::from_bits(0xffc0_0000);
            }
        }
    }
    Ok(())
}
