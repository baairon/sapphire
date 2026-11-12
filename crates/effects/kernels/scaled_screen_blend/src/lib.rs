//! Scaled screen blend: for every element of `src` that is strictly positive, the destination element is combined
//! with it. With `scale == 1.0` exactly the plain screen form is used, otherwise the source is multiplied by `scale` first.
//!
//! Per element, with s = src[i] and d = dst[i]:
//!   scale != 1.0 (also NaN scale):  if s > 0 { t = scale * s; dst = if 1.0 > t { fma213(1.0 - t, d, t) } else { 1.0 } }
//!   scale == 1.0:                   if s > 0 { dst = if 1.0 > s { max2(s, max2(d, fma213(1.0 - s, d, s))) } else { 1.0 } }
//! Elements with s <= 0 or NaN s are left untouched. `max2(a, b)` is the x86 maximum: `a` if a > b, else `b`.
//! Every fused step goes through `fx_common::nanop` (one rounding; NaN priority first operand, then second, then third,
//! the NaN returned quieted; a fresh NaN is 0xffc00000). Both operands of a two-NaN pick can never be NaN on this route:
//! the multiplied source is positive (not NaN) and `1.0 - t` is reached only for a non-NaN `t`.

use fx_common::nanop::{fma213, NanOps};

/// x86 MAXSS/MAXPS: returns `a` only when `a > b`; NaN or equal returns `b`.
#[inline(always)]
fn max2(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

/// One element, scale != 1.0 route (the listing's 132 scalar form and 213 vector forms differ only in NaN priority between
/// operands that are never both NaN here, so one form is used).
#[inline(always)]
fn scaled(d: f32, s: f32, scale: f32) -> Option<f32> {
    if !(s > 0.0) {
        return None;
    }
    let one = 1.0f32;
    let t = scale.fmul(s);
    Some(if one > t {
        let omt = one.fsub(t);
        fma213(omt, d, t)
    } else {
        one
    })
}

/// One element, scale == 1.0 route.
#[inline(always)]
fn plain(d: f32, s: f32) -> Option<f32> {
    if !(s > 0.0) {
        return None;
    }
    let one = 1.0f32;
    Some(if one > s {
        let omt = one.fsub(s);
        let f = fma213(omt, d, s);
        let m = max2(d, f);
        max2(s, m)
    } else {
        one
    })
}

/// Blends `src` into `dst` over `min(dst.len(), src.len())` elements.
pub fn blend(dst: &mut [f32], src: &[f32], scale: f32) {
    let n = dst.len().min(src.len());
    let one_route = scale == 1.0;
    for i in 0..n {
        let r = if one_route { plain(dst[i], src[i]) } else { scaled(dst[i], src[i], scale) };
        if let Some(v) = r {
            dst[i] = v;
        }
    }
}

/// Labelled stand-ins this port depends on.
pub fn assumptions() -> &'static [&'static str] {
    &[
        "THREADS-AS-ARGUMENT: the per-thread element range is the caller's slice; the whole array is one range",
        "NOTE: the two constant tables (eight 1.0f lanes, one scalar 1.0f splatted by code, and an all-ones lane mask) are fixed constants of the routine",
    ]
}
