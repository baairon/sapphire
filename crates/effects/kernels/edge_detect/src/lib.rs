//! Class `edge_detect`: render slot 52 (per element: blur of the source, then the gradient magnitude body) and slot 57 (margins).
//!
//! Slot 52: for each element i, s = (float) k, a = (amount * 0.125) * s, b = (gain * 0.05) * s; the source plane is blurred (class blur, sigma a in
//! both axes, gain b, three passes) into a scratch plane, and the body turns that plane into sqrt(gx^2 + gy^2) per pixel.
//! The body is the AVX2 scalar form: central differences inside the plane, one-sided (scaled by `K`) at the first and last column, vertical
//! differences scaled by the row factor (`C` inside, `2 C` on the first and last row). Fused multiply-add sites keep the listing's operand roles.
//! Std only, no unsafe, no platform libm, no environment access.
use blur::{blur_core, Edge};
use fx_common::nanop::{fma213, NanOps};
pub use fx_common::plane::{FxError, Plane, PlaneMut};

/// Scalar constants of the image: `ONE` 0x15e850c, `TWO` 0x15e8510, `HALF` 0x15e8514, `SIGMA_SCALE` 0x15e8d94, `GAIN_SCALE` 0x15ee094
/// (values read from the emulator's image world).
pub const ONE: f32 = 1.0;
pub const TWO: f32 = 2.0;
pub const HALF: f32 = 0.5;
pub const SIGMA_SCALE: f32 = 0.125;
pub const GAIN_SCALE: f32 = 0.05;

/// Fields the render reads.
#[derive(Clone, Debug)]
pub struct EdgeParams {
    /// object field 0x6c (blur amount)
    pub amount: f32,
    /// object field 0x88 (gain multiplier)
    pub gain_param: f32,
    /// object field 0xa4 (nonzero = fractional radii in the blur)
    pub subpixel: i32,
    /// edge mode cell: 0 zero, 1 mirror, else refused
    pub edge_mode: i32,
    /// image object field 0x20 (integer render scale)
    pub render_scale: i32,
    /// image object fields 0x30 / 0x38: Some(aspect) when the aspect flag is set
    pub aspect: Option<f32>,
    /// image object field 0x3c != 0
    pub info3c: bool,
    /// runtime flag cell 0x1addc04 != 0
    pub flag_cell: i32,
    /// value the body reads one float past the plane when the width is 1
    pub past_end: f32,
}

/// `vsqrtss`: exact; a NaN input is returned quieted, a negative input gives the default NaN.
#[inline(always)]
fn sqrt_ss(x: f32) -> f32 {
    let r = x.sqrt();
    if r.is_nan() {
        return if x.is_nan() { f32::from_bits(x.to_bits() | 0x0040_0000) } else { f32::from_bits(0xffc0_0000) };
    }
    r
}

/// Body parameters: `c` is the row factor cell (1.0, or 0.5 under the flag), `k` the one-sided scale (2.0).
#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub c: f32,
    pub k: f32,
    pub past_end: f32,
}

/// Row factor and body parameters from the flags (0x342180: cmp [0x1addc04], test job[0x3c]).
pub fn body_for(p: &EdgeParams) -> Body {
    let c = if p.flag_cell != 0 && p.info3c { HALF } else { ONE };
    Body { c, k: TWO, past_end: p.past_end }
}

/// The body over the rows `start .. start + count` of a `w x h` plane `b` (the whole plane is readable). Writes `out` rows only for those rows.
pub fn edge_rows(bd: &Body, b: &[f32], w: usize, h: usize, start: usize, count: usize, out: &mut [f32]) {
    if w == 0 || h == 0 {
        return;
    }
    let at = |i: usize| -> f32 { if i < b.len() { b[i] } else { bd.past_end } };
    let c = bd.c;
    let c_doubled = c + c;
    for y in start..start + count {
        let cur = y * w;
        let up = if y == 0 { cur } else { cur - w };
        let down = if y + 1 == h { cur } else { cur + w };
        // row factor: 2c on the first and last row, else c
        let v = if y == 0 || y + 1 == h { c_doubled } else { c };
        // first pixel: gx = k * (px_second - px_first), squared with a plain multiply; gy fused
        let px_first = b[cur];
        let px_second = at(cur + 1);
        let gx0 = bd.k.fmul(px_second.fsub(px_first));
        let gx0sq = gx0.fmul(gx0);
        let gy = b[up].fsub(b[down]).fmul(v);
        out[cur] = sqrt_ss(fma213(gy, gy, gx0sq));
        let mut gxl_sq = gx0sq;
        if w > 2 {
            // interior: gx = p[x+1] - p[x-1], gy squared with a plain multiply, gx fused
            for x in 1..w - 1 {
                let gyr = v.fmul(b[up + x].fsub(b[down + x]));
                let gysq = gyr.fmul(gyr);
                let gx = b[cur + x + 1].fsub(b[cur + x - 1]);
                out[cur + x] = sqrt_ss(fma213(gx, gx, gysq));
            }
            let last_diff = b[cur + w - 1].fsub(b[cur + w - 2]);
            let g = bd.k.fmul(last_diff);
            gxl_sq = g.fmul(g);
        }
        // last pixel (also the only pixel of a width-1 row: it rewrites column 0 with gx0 as computed above)
        let gyl = b[up + w - 1].fsub(b[down + w - 1]).fmul(v);
        out[cur + w - 1] = sqrt_ss(fma213(gyl, gyl, gxl_sq));
    }
}

/// Slot 52 scale arithmetic for one element: (sigma, gain) from `s` = (float) render scale.
pub fn sigma_gain(p: &EdgeParams) -> (f32, f32) {
    let s = p.render_scale as f32;
    let sigma = p.amount.fmul(SIGMA_SCALE).fmul(s);
    let gain = p.gain_param.fmul(GAIN_SCALE).fmul(s);
    (sigma, gain)
}

/// One element of slot 52 on dense planes: blur `src` into `aux`, then the body over the whole frame into `dst`.
pub fn render(p: &EdgeParams, src: &[f32], w: usize, h: usize, aux: &mut [f32], dst: &mut [f32]) -> Result<(), FxError> {
    render_rows(p, src, w, h, aux, dst, 0, h)
}

/// Same with the body restricted to the rows `start .. start + count` (one partition of the original's row split).
pub fn render_rows(p: &EdgeParams, src: &[f32], w: usize, h: usize, aux: &mut [f32], dst: &mut [f32], start: usize, count: usize) -> Result<(), FxError> {
    let n = w.checked_mul(h).ok_or(FxError::Geometry)?;
    if w > i32::MAX as usize || h > i32::MAX as usize || n > i32::MAX as usize || start.checked_add(count).map_or(true, |e| e > h) {
        return Err(FxError::Geometry);
    }
    if src.len() < n || aux.len() < n || dst.len() < n {
        return Err(FxError::Stride);
    }
    if n == 0 {
        return Ok(()); // the original loops over zero rows
    }
    let edge = Edge::from_i32(p.edge_mode)?;
    let (sigma, gain) = sigma_gain(p);
    blur_core(&src[..n], &mut aux[..n], w, h, sigma, sigma, gain, edge, 3, p.subpixel != 0, p.aspect)?;
    edge_rows(&body_for(p), &aux[..n], w, h, start, count, &mut dst[..n]);
    Ok(())
}

/// i32 conversion with truncation as `cvttss2si`: NaN or out of range gives i32::MIN.
pub fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// Slot 57: the two margins. v = ((float) k * amount) * 0.125; the second value is multiplied by the aspect when the flag is set (aspect * v);
/// each is ceil(v) truncated, plus one (wrapping).
pub fn margins(p: &EdgeParams) -> (i32, i32) {
    let k = p.render_scale as f32;
    let v = k.fmul(p.amount).fmul(SIGMA_SCALE);
    let v_aspect = match p.aspect {
        Some(a) => a.fmul(v),
        _ => v,
    };
    let m = |x: f32| cvtt(x.ceil()).wrapping_add(1);
    (m(v), m(v_aspect))
}

/// One stand-in or decision record.
#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub what: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        // Runtime flag cell: 0x1addc04.
        Assumption { id: "ED-A1", what: "runtime flag cell: value 1, read from the emulated image (MEASURED, equal to the T2 value), no store in the listing; with job field 0x3c nonzero the body factor is 0.5 instead of 1.0; the render flag is a parameter", label: "MEASURED-EMULATOR-READ (sensitivity tested)" },
        Assumption { id: "ED-A2", what: "width 1: the body reads one float past the row; the value is host memory, parameter past_end (zero pad in the emulator harness)", label: "PARAM-UNCAPTURED" },
        Assumption { id: "ED-A3", what: "row partition returns the rows given by the caller; parallel_for runs the body once", label: "HOST-HOOKED" },
        Assumption { id: "ED-A4", what: "device route (render flag != 0, 0x344c90) is not rebuilt; the CPU route only", label: "NOT REBUILT" },
        Assumption { id: "ED-A5", what: "feature word 0x9d97ff superset: the AVX2 body 0x342180 is the judged route; the SSE twin is not ported", label: "ROUTE AVX2-ONLY" },
        Assumption { id: "ED-A6", what: "blur ROI record with the image field 0 only; blur_core scratch phase 0 (inherited from crate blur)", label: "ASSUMED" },
        Assumption { id: "ED-A7", what: "ceilf of slot 57 is roundss ceil on the AVX2 word; modelled by f32::ceil", label: "INFERRED, tested in T2" },
    ]
}
