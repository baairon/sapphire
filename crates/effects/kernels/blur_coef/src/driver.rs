//! Driver body control flow (SPEC section 3): shortcut order, X and Y sections, final scale. The two pass
//! kernels (rows K1 and K3) are supplied by the caller as closures, so BG can glue this to `blur_rows` / `blur_rows_y`.
//! Cancel polls return 0 (stand-in B-A1), so no early exit exists.
use crate::coef::{clamp_passes, coeffs, final_scale, radii};
use crate::helpers::{clear, scale_or_copy, HelperError};

/// Explicit inputs of the driver (no Default). Maps to rdi/rsi/... of the original in the comments.
#[derive(Clone, Copy, Debug)]
pub struct DriverParams {
    pub sigma_x: f32,       // xmm0
    pub sigma_y: f32,       // xmm1
    pub gain: f32,          // xmm2
    pub passes: i32,        // r8d, clamped to 1..3 inside
    pub subpixel: bool,     // r9d != 0
    pub half: bool,         // [rbx] != 0 (Blur and Glow pass 0)
    pub aspect: Option<f32>, // src != 0 -> Some(f32 at src) (B-A5), None = off
    pub width: i32,         // roi.w
    pub height: i32,        // roi.h
}

/// What one pass wrapper receives (arrays A, B, C, frac, pass count n; edge mode and subpixel are the caller's).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PassPlan {
    pub a: [i32; 3],
    pub b: [i32; 3],
    pub c: [f32; 3],
    pub frac: f32,
    pub npasses: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Plan {
    /// gain == 0.0: clears dst.
    Clear,
    /// Both sigmas not "positive": (dst, src, gain).
    ScaleOrCopy { gain: f32 },
    /// X pass (if any), Y pass (if any), then (dst, dst, scale). With both passes absent (NaN sigma quirk,
    /// SPEC 3 step 3) dst is scaled as it is.
    Run { x: Option<PassPlan>, y: Option<PassPlan>, scale: f32 },
}

/// Compute the plan exactly as the listing branches (ucomiss / vcomiss with their NaN behaviour).
pub fn plan(p: &DriverParams) -> Plan {
    // ucomiss gain, 0: equal and ordered -> clear (NaN falls through).
    if p.gain == 0.0 {
        return Plan::Clear;
    }
    // vcomiss 0, sigma; jb taken when 0 < sigma or unordered.
    let proceed = |s: f32| 0.0f32 < s || s.is_nan();
    if !proceed(p.sigma_x) && !proceed(p.sigma_y) {
        return Plan::ScaleOrCopy { gain: p.gain };
    }
    let n = clamp_passes(p.passes);
    let mut acc = 1.0f64;
    // X section: vcomiss sx, 0; jbe skips for sx <= 0 and NaN.
    let x = if p.sigma_x > 0.0 {
        let r = radii(p.sigma_x, p.width, n, p.subpixel);
        let (c, a2) = coeffs(&r.a, &r.b, r.frac, n as usize, acc);
        acc = a2;
        Some(PassPlan { a: r.a, b: r.b, c, frac: r.frac, npasses: n })
    } else {
        None
    };
    // Y section: sigma halved first (vmulss 0.5), tested, then multiplied by the aspect value.
    let mut sy = p.sigma_y;
    if p.half {
        sy *= 0.5;
    }
    let y = if sy > 0.0 {
        if let Some(k) = p.aspect {
            sy *= k;
        }
        let extent = if p.half { p.height >> 1 } else { p.height };
        let r = radii(sy, extent, n, p.subpixel);
        let (c, a2) = coeffs(&r.a, &r.b, r.frac, n as usize, acc);
        acc = a2;
        Some(PassPlan { a: r.a, b: r.b, c, frac: r.frac, npasses: n })
    } else {
        None
    };
    Plan::Run { x, y, scale: final_scale(p.gain, acc) }
}

/// Run the driver on dense w*h planes. `pass_x(plan, src, dst)` is the horizontal kernel; `pass_y(plan, src, dst)` the
/// vertical one, where `src` is `None` when the original reads dst in place (after an X pass) and `Some(src)` when the X
/// section was skipped (r13 still points at the source plane, is the only place it changes).
pub fn drive<FX, FY>(
    p: &DriverParams,
    src: &[f32],
    dst: &mut [f32],
    mut pass_x: FX,
    mut pass_y: FY,
) -> Result<Plan, HelperError>
where
    FX: FnMut(&PassPlan, &[f32], &mut [f32]),
    FY: FnMut(&PassPlan, Option<&[f32]>, &mut [f32]),
{
    let pl = plan(p);
    match pl {
        Plan::Clear => clear(dst, p.width, p.height)?,
        Plan::ScaleOrCopy { gain } => scale_or_copy(dst, Some(src), gain, p.width, p.height)?,
        Plan::Run { x, y, scale } => {
            if let Some(xp) = &x {
                pass_x(xp, src, dst);
            }
            if let Some(yp) = &y {
                if x.is_some() {
                    pass_y(yp, None, dst);
                } else {
                    pass_y(yp, Some(src), dst);
                }
            }
            scale_or_copy(dst, None, scale, p.width, p.height)?;
        }
    }
    Ok(pl)
}
