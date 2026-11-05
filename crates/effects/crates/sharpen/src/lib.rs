//! Class `sharpen` (S_Sharpen): unsharp mask of the Shake sharpen effect, AVX2 route (feature word 0x9d97ff), render flag 0.
//! Sources of truth: neutral_1 listings (slot 52 driver), (plane wrapper), (combine job), (roi).
//! Route: radius = (0.125 * amount) * (float)ctx[0x20]; per plane: copy when radius <= 0 or gain == 0, otherwise the blur class
//! (edge mirror, 3 passes, subpixel 0, half 0, gain 1.0) and then out = fma(x - b, g, x) (g == 1.0: fma(2.0, x, -b)).
//! Std only, no unsafe. VEX follows the SDM, not silicon.
pub mod assume;
pub mod combine;
pub mod plan;

#[allow(unused_imports)]
use fx_common::plane::{FxError, Plane, PlaneMut};
#[allow(unused_imports)]
use plan::{gain_index, plane_plan, radius, PlanePlan};

/// Explicit parameters (S3, no Default: the binaries contain no defaults).
#[derive(Clone, Debug)]
pub struct Params {
    /// this
    pub amount: f32,
    /// this, one gain per plane; indexed by min(ch, in_count), see `plan::gain_index` (SP-A5)
    pub gains: Vec<f32>,
    /// i32 [R], must equal the number of source planes here (SP-A5)
    pub in_count: usize,
    /// i32 [ctx], the width used for the radius (host field)
    pub ctx_width: i32,
    /// blur aspect: i32 [ctx] != 0 then f32 [ctx] (B-A5); None = off
    pub aspect: Option<f32>,
    /// i32 [ctx], the element count of the combine job; None = width * height (SP-A3)
    pub count: Option<usize>,
}

#[allow(dead_code)]
fn dense(p: &Plane) -> Vec<f32> {
    let mut v = Vec::with_capacity(p.width * p.height);
    for y in 0..p.height {
        v.extend_from_slice(&p.row(y)[..p.width]);
    }
    v
}

/// One plane of the wrapper (flag 0) on dense buffers with the blur step supplied by the caller (SHF1 refactor: the same code path
/// serves the native `sharpen_plane` and the T2 runs that record the blur dispatcher in the emulator and hand its plane in).
/// Copy shortcut, or `blur(x, radius)` followed by the combine job over `count` elements. Samples beyond `count`:
/// non-alias path (destination plane distinct from the source plane): the original blurs into the output plane, so the tail keeps the blurred value;
/// ALIAS path (destination plane is the source plane, SHE2 fix of W185 c): the blur goes into the host scratch plane and the combine job writes
/// into the source plane in place, so the tail keeps the INPUT value.
pub fn sharpen_plane_with<F>(x: &[f32], w: usize, h: usize, radius: f32, gain: f32, count: usize, blur: F) -> Result<Vec<f32>, FxError>
where
    F: FnOnce(&[f32], f32) -> Result<Vec<f32>, FxError>,
{
    sharpen_plane_alias(x, w, h, radius, gain, count, false, blur)
}

/// [`sharpen_plane_with`] with the alias flag: `alias == true` when the destination plane is the source plane.
pub fn sharpen_plane_alias<F>(x: &[f32], w: usize, h: usize, radius: f32, gain: f32, count: usize, alias: bool, blur: F) -> Result<Vec<f32>, FxError>
where
    F: FnOnce(&[f32], f32) -> Result<Vec<f32>, FxError>,
{
    let total = w.checked_mul(h).ok_or(FxError::Geometry)?;
    if x.len() < total || count > total {
        return Err(FxError::Geometry);
    }
    match plane_plan(radius, gain) {
        PlanePlan::Copy => Ok(x[..total].to_vec()),
        PlanePlan::Run { radius: rr, gain: g } => {
            let b = blur(&x[..total], rr)?;
            if b.len() < total {
                return Err(FxError::Geometry);
            }
            // tail source: blurred plane (non-alias) or the input plane (alias)
            let mut out = if alias { x[..total].to_vec() } else { b.clone() };
            combine::combine(&x[..total], &b, g, &mut out, count)?;
            Ok(out)
        }
    }
}

/// One plane of the wrapper with the native blur class. `base_phase` is the blur scratch alignment stand-in B-A4.
#[cfg(feature = "blur")]
pub fn sharpen_plane(x: &[f32], w: usize, h: usize, radius: f32, gain: f32, aspect: Option<f32>, count: usize, base_phase: usize) -> Result<Vec<f32>, FxError> {
    sharpen_plane_with(x, w, h, radius, gain, count, |xs, rr| {
        let mut b = vec![0.0f32; xs.len()];
        blur::blur_core_phase(xs, &mut b, w, h, rr, rr, plan::BLUR_GAIN, blur::Edge::Mirror, plan::BLUR_PASSES, false, aspect, base_phase)?;
        Ok(b)
    })
}

/// Scratch alignment stand-in used by `render` (B-A4): blur::DEFAULT_BASE_PHASE.
#[cfg(feature = "blur")]
pub const BASE_PHASE: usize = blur::DEFAULT_BASE_PHASE;

/// The slot function (flag 0) over dense planes with the blur step supplied by the caller. `src.len` is the plane count N (the vcall
/// [[this]]); `p.in_count` is [R] and is independent of N here (the original indexes the gains with min(ch, in_count)).
pub fn render_with<F>(p: &Params, src: &[Plane], dst: &mut [PlaneMut], blur: F) -> Result<(), FxError>
where
    F: FnMut(&[f32], usize, usize, f32) -> Result<Vec<f32>, FxError>,
{
    render_with_alias(p, src, dst, false, blur)
}

/// The slot function with the alias flag: `alias` is true when the destination planes are the source planes (the host hands the same node as input and
/// output: the split branch `sharpen :ref16.2 . :ref16.2`); the original then blurs into a host scratch plane (call [[ctx]]) and the
/// samples beyond `p.count` keep the input value (SHE2, W185 c).
pub fn render_with_alias<F>(p: &Params, src: &[Plane], dst: &mut [PlaneMut], alias: bool, mut blur: F) -> Result<(), FxError>
where
    F: FnMut(&[f32], usize, usize, f32) -> Result<Vec<f32>, FxError>,
{
    let n = src.len();
    if dst.len() != n {
        return Err(FxError::PlaneCount);
    }
    if n == 0 {
        return Ok(());
    }
    let (w, h) = (src[0].width, src[0].height);
    if src.iter().any(|s| s.width != w || s.height != h) || dst.iter().any(|d| d.width != w || d.height != h) {
        return Err(FxError::Geometry);
    }
    let total = w.checked_mul(h).ok_or(FxError::Geometry)?;
    let count = p.count.unwrap_or(total);
    if count > total {
        return Err(FxError::Geometry);
    }
    let r = radius(p.amount, p.ctx_width);
    for ch in 0..n {
        let idx = gain_index(ch, p.in_count);
        let g = *p.gains.get(idx).ok_or(FxError::PlaneCount)?;
        let x = dense(&src[ch]);
        let out = sharpen_plane_alias(&x, w, h, r, g, count, alias, |xs, rr| blur(xs, w, h, rr))?;
        let d = &mut dst[ch];
        for y in 0..h {
            d.row_mut(y)[..w].copy_from_slice(&out[y * w..(y + 1) * w]);
        }
    }
    Ok(())
}

/// render (flag 0 only; flag != 0 is a host serializer path and is not rebuilt, SP-A1). PARITY is claimed for stride == width;
/// strided planes are repacked to dense scratch (as the blur class does). SP-A5: the native API requires in_count == plane count.
#[cfg(feature = "blur")]
pub fn render(p: &Params, src: &[Plane], dst: &mut [PlaneMut]) -> Result<(), FxError> {
    if dst.len() != src.len() || p.in_count != src.len() {
        return Err(FxError::PlaneCount);
    }
    let aspect = p.aspect;
    render_with(p, src, dst, |xs, w, h, rr| {
        let mut b = vec![0.0f32; xs.len()];
        blur::blur_core_phase(xs, &mut b, w, h, rr, rr, plan::BLUR_GAIN, blur::Edge::Mirror, plan::BLUR_PASSES, false, aspect, BASE_PHASE)?;
        Ok(b)
    })
}

/// SHE3: the audited render with the alias flag and the explicit host count (Params.count). `alias` = the destination node is the source node in the plan.
#[cfg(feature = "blur")]
pub fn render_alias(p: &Params, src: &[Plane], dst: &mut [PlaneMut], alias: bool) -> Result<(), FxError> {
    if dst.len() != src.len() || p.in_count != src.len() {
        return Err(FxError::PlaneCount);
    }
    let aspect = p.aspect;
    render_with_alias(p, src, dst, alias, |xs, w, h, rr| {
        let mut b = vec![0.0f32; xs.len()];
        blur::blur_core_phase(xs, &mut b, w, h, rr, rr, plan::BLUR_GAIN, blur::Edge::Mirror, plan::BLUR_PASSES, false, aspect, BASE_PHASE)?;
        Ok(b)
    })
}
