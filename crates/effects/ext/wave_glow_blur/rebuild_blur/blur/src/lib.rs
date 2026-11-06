//! Class `blur` (S_Blur: STRONG exact id, membership PROVEN): blur_core and render.
//! Sources of truth: neutral_1 dump listings. blur_core = core wrapper + dispatcher + driver body 
//! (AVX2, feature word 0x9d97ff) + pass wrappers + row kernels K1 and K3, glued from the
//! crates blur_coef (driver arithmetic), blur_rows (K1) and blur_rows_y (K3). render = .
//! Route: render flag 0, single worker, polls return 0, half = 0 (kernel K2 is out of scope). Std only, no unsafe.
use blur_coef::{drive, DriverParams, PassPlan};
use blur_rows::JobX;
use blur_rows_y::PassYJob;
pub use fx_common::plane::{FxError, Plane, PlaneMut};

/// Edge mode of the row kernels (U03 / B-U2): 0 zero outside, 1 mirror with duplication. The original asserts on anything else.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    Zero = 0,
    Mirror = 1,
}

impl Edge {
    pub fn from_i32(v: i32) -> Result<Edge, FxError> {
        match v {
            0 => Ok(Edge::Zero),
            1 => Ok(Edge::Mirror),
            _ => Err(FxError::Geometry), // BG-A2: fx_common's FxError has no edge variant and is not changed by this crate
        }
    }
}

/// Scratch buffer base alignment modulo 32 bytes (B-A4). Only windows of 64 or more samples notice it.
pub const DEFAULT_BASE_PHASE: usize = 0;

/// blur_core on dense w*h buffers. `half` is not a parameter: Blur and Glow pass 0.
/// Order as the driver: gain 0 clears dst; both sigmas not positive scales or copies src; otherwise the X pass (sigma_x > 0),
/// the Y pass (sigma_y > 0, reading dst in place after an X pass, else src), then the final scale gain / prod(2*B+1).
#[allow(clippy::too_many_arguments)]
pub fn blur_core(src: &[f32], dst: &mut [f32], w: usize, h: usize, sx: f32, sy: f32, gain: f32, edge: Edge, passes: i32, subpixel: bool, aspect: Option<f32>) -> Result<(), FxError> {
    blur_core_phase(src, dst, w, h, sx, sy, gain, edge, passes, subpixel, aspect, DEFAULT_BASE_PHASE)
}

/// Same with the scratch base phase (B-A4) as an explicit input, used by the tests that sweep it.
#[allow(clippy::too_many_arguments)]
pub fn blur_core_phase(src: &[f32], dst: &mut [f32], w: usize, h: usize, sx: f32, sy: f32, gain: f32, edge: Edge, passes: i32, subpixel: bool, aspect: Option<f32>, base_phase: usize) -> Result<(), FxError> {
    let n = w.checked_mul(h).ok_or(FxError::Geometry)?;
    if w > i32::MAX as usize || h > i32::MAX as usize || n > i32::MAX as usize {
        return Err(FxError::Geometry);
    }
    if src.len() < n || dst.len() < n {
        return Err(FxError::Stride);
    }
    let p = DriverParams { sigma_x: sx, sigma_y: sy, gain, passes, subpixel, half: false, aspect, width: w as i32, height: h as i32 };
    let err: std::cell::Cell<Option<FxError>> = std::cell::Cell::new(None);
    let e = edge as i32;
    let r = drive(
        &p,
        &src[..n],
        &mut dst[..n],
        |pl: &PassPlan, s: &[f32], d: &mut [f32]| {
            let job = JobX { edge: e, npasses: pl.npasses as usize, subpixel: subpixel as i32, frac: pl.frac, a: &pl.a, b: &pl.b, c: &pl.c, base_phase };
            if blur_rows::pass_x(s, d, w, h, &job).is_err() {
                err.set(Some(FxError::Geometry));
            }
        },
        |pl: &PassPlan, s: Option<&[f32]>, d: &mut [f32]| {
            let job = PassYJob { width: w, height: h, edge: e, passes: pl.npasses as usize, a: &pl.a, b: &pl.b, c: &pl.c, frac: pl.frac, subpixel, base_phase };
            let res = match s {
                None => blur_rows_y::pass_y_inplace(&job, d),
                Some(s) => blur_rows_y::pass_y(&job, s, d),
            };
            if res.is_err() {
                err.set(Some(FxError::Geometry));
            }
        },
    );
    if r.is_err() {
        return Err(FxError::Stride);
    }
    match err.get() {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

/// Entry for a caller that asks for the half-height vertical mode (kernel K2, not ported, decision): always an error.
#[allow(clippy::too_many_arguments)]
pub fn blur_core_half(src: &[f32], dst: &mut [f32], w: usize, h: usize, sx: f32, sy: f32, gain: f32, edge: Edge, passes: i32, subpixel: bool, aspect: Option<f32>, half: bool) -> Result<(), FxError> {
    if half {
        return Err(FxError::Geometry); // BG-A1: unsupported, no dedicated FxError variant
    }
    blur_core(src, dst, w, h, sx, sy, gain, edge, passes, subpixel, aspect)
}

/// Parameters of the `blur` effect and the host values render reads. Object offsets are `this+` unless stated.
#[derive(Clone, Debug)]
pub struct BlurParams {
    /// this, this: amount, horizontal and vertical lanes (lanes 2 and 3 are not read).
    pub amount: [f32; 2],
    /// this: filter_shape; passes = 1 if negative, else min(shape + 1, 3).
    pub filter_shape: i32,
    /// this: per plane output gain (four lanes).
    pub scale: [f32; 4],
    /// this: number of supplied lanes of `scale` (drives the gain overrides).
    pub scale_count: i32,
    /// this: subpixel_p, nonzero = fractional radii.
    pub subpixel: i32,
    /// `**(int**)` (cell in listing space), pointee uncaptured (U03): 0 zero, 1 mirror, else FxError.
    pub edge_mode: i32,
    /// Result of the virtual call this->vfunc[31]: colour planes, 1 to 4 (0 does nothing).
    pub plane_count: usize,
    /// src as an integer (render scale of the source image); k = (float)render_scale, sigma = amount * 0.125 * k.
    pub render_scale: i32,
    /// src != 0 gives Some(f32 at src) (B-A5): multiplies the vertical sigma after the positivity test.
    pub aspect: Option<f32>,
}

/// filter_shape to pass count .
pub fn pass_count(filter_shape: i32) -> i32 {
    if filter_shape < 0 {
        1
    } else {
        (filter_shape + 1).min(3)
    }
}

/// Gain of plane `c` : this, one float per channel c, forced to 1.0 for c == 1 when n == 2 and scale_count == 1,
/// and for c == 3 when scale_count < 4.
pub fn plane_gain(p: &BlurParams, c: usize) -> f32 {
    let mut g = p.scale[c];
    if p.plane_count == 2 && c == 1 && p.scale_count == 1 {
        g = 1.0;
    }
    if c == 3 && p.scale_count < 4 {
        g = 1.0;
    }
    g
}

/// Sigma of one lane : (amount * 0.125) * (float)render_scale, two mulss.
pub fn lane_sigma(amount: f32, render_scale: i32) -> f32 {
    (amount * 0.125f32) * (render_scale as f32)
}

/// render (flag 0): per plane c in 0..plane_count one blur_core call. src and dst planes must share one width and
/// height; strided planes are repacked into dense scratch (dst is copied in first, as the original works in place on dst) and back.
/// PARITY is claimed on dense planes.
pub fn render(p: &BlurParams, src: &[Plane], dst: &mut [PlaneMut]) -> Result<(), FxError> {
    let n = p.plane_count;
    if n > 4 || src.len() < n || dst.len() < n {
        return Err(FxError::PlaneCount);
    }
    if n == 0 {
        return Ok(());
    }
    let (w, h) = (src[0].width, src[0].height);
    if src[..n].iter().any(|s| s.width != w || s.height != h) || dst[..n].iter().any(|d| d.width != w || d.height != h) {
        return Err(FxError::Geometry);
    }
    let edge = Edge::from_i32(p.edge_mode)?;
    let passes = pass_count(p.filter_shape);
    let sx = lane_sigma(p.amount[0], p.render_scale);
    let sy = lane_sigma(p.amount[1], p.render_scale);
    for c in 0..n {
        let gain = plane_gain(p, c);
        let s = &src[c];
        let d = &mut dst[c];
        let sbuf: Vec<f32>;
        let sslice: &[f32] = if s.stride == w {
            &s.data[..w * h]
        } else {
            let mut v = Vec::with_capacity(w * h);
            for y in 0..h {
                v.extend_from_slice(s.row(y));
            }
            sbuf = v;
            &sbuf
        };
        if d.stride == w {
            blur_core(sslice, &mut d.data[..w * h], w, h, sx, sy, gain, edge, passes, p.subpixel != 0, p.aspect)?;
        } else {
            let mut dbuf = Vec::with_capacity(w * h);
            for y in 0..h {
                dbuf.extend_from_slice(&d.data[y * d.stride..y * d.stride + w]);
            }
            blur_core(sslice, &mut dbuf, w, h, sx, sy, gain, edge, passes, p.subpixel != 0, p.aspect)?;
            for y in 0..h {
                d.row_mut(y).copy_from_slice(&dbuf[y * w..(y + 1) * w]);
            }
        }
    }
    Ok(())
}

/// One stand-in or decision record (S7).
#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub what: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "B-A1", what: "cancel polls return 0 (never abort)", label: "ASSUMED" },
        Assumption { id: "B-A2", what: "roi record {w, h, c = w, d = 0, e = 4}; dense planes", label: "ASSUMED" },
        Assumption { id: "B-A4", what: "scratch base address mod 32 = base_phase (default 0), worker 0, stride (bytes+15)&!15", label: "ASSUMED" },
        Assumption { id: "B-A5", what: "aspect (src) is a caller field; None = off", label: "ASSUMED" },
        Assumption { id: "B-A6", what: "single worker, one whole tile", label: "ASSUMED" },
        Assumption { id: "BG-A1", what: "half != 0 returns FxError::Geometry (kernel K2 not ported, decision W3; FxError has no dedicated variant)", label: "ASSUMED" },
        Assumption { id: "BG-A2", what: "edge mode other than 0 or 1 returns FxError::Geometry before any work (the original asserts inside the kernel, only when a pass runs)", label: "ASSUMED" },
        Assumption { id: "BG-A3", what: "plane_count (virtual call result) is 0 to 4; more returns FxError::PlaneCount", label: "INFERRED" },
    ]
}
