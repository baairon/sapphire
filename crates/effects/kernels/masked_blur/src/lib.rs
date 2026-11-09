//! Class `masked_blur` (slot 52): a normalised, mask weighted blur beside the `blur` class.
//! Sources of truth: neutral_1 listings (render glue), (wrapper, float path = render flag 0), (clamp),
//!  (premultiply), job (AVX2 body, combine) and the finished blur crates (dispatcher).
//! Route: AVX2 (feature word 0x9d97ff), render flag 0, single worker, polls return 0, half = 0. Std only, no unsafe.
//! Dataflow (MEASURED): M = mask clamped in place to [0,1]; W = blur(M); per plane c: T = src[c] * clamp01(M);
//! T = blur(T) (in place); dst[c] = combine(M, T, W, src[c]).
use blur::{blur_core, pass_count, Edge};
pub use fx_common::plane::{FxError, Plane, PlaneMut};
use fx_common::nanop::{canon, fma231, has_nan, NanOps};

/// min with the x86 operand rule of minss/minps: returns the second operand when unordered or equal.
#[inline]
fn min_x86(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

///  element rule (AVX2 vector and scalar twins agree): `lo > x` stores lo, otherwise min(hi, x). NaN stays NaN.
#[inline]
pub fn clamp_px(x: f32, lo: f32, hi: f32) -> f32 {
    if lo > x {
        lo
    } else {
        min_x86(hi, x)
    }
}

///  (render flag 0): clamp a dense buffer in place to [lo, hi]. The wrapper calls it with lo = 0.0 and hi = 1.0 on the mask.
pub fn clamp_in_place(buf: &mut [f32], lo: f32, hi: f32) {
    for v in buf.iter_mut() {
        *v = clamp_px(*v, lo, hi);
    }
}

///  element rule (flag 0): c = (0 > m) ? 0 : min(1, m), then dst = c * dst. The 1.0 of the vector body is the splat at
///  (NOT captured, ASSUMED 1.0 in every lane, MB-A3); the scalar twin uses = 1.0.
#[inline]
pub fn premultiply_px(m: f32, d: f32) -> f32 {
    let c = if 0.0f32 > m { 0.0 } else { min_x86(1.0, m) };
    // vmulss c, c, d: with two NaN operands the first source (the clamped mask value) is the result
    c.fmul(d)
}

/// dst[i] = clamp01(mask[i]) * dst[i]. When no operand is a NaN a product can only be a fresh NaN (zero times infinity), so a plain
/// loop with a canonicalizing select gives the same bits; otherwise each element follows [`premultiply_px`].
pub fn premultiply_in_place(dst: &mut [f32], mask: &[f32]) {
    if !has_nan(mask) && !has_nan(dst) {
        for (d, m) in dst.iter_mut().zip(mask.iter()) {
            let c = if 0.0f32 > *m { 0.0 } else { min_x86(1.0, *m) };
            *d = canon(c * *d);
        }
    } else {
        for (d, m) in dst.iter_mut().zip(mask.iter()) {
            *d = premultiply_px(*m, *d);
        }
    }
}

/// The combine job's threshold, an f32 constant: MEASURED-DYLIB = 0x39000000 = 2^-13 = 1.2207031e-4
/// (read from the approved plugin file, `__TEXT,__const`; the earlier port default 0.0 was refuted).
pub const MASK_THRESHOLD: f32 = f32::from_bits(0x3900_0000);

/// Job, AVX2 body (float path): inputs mask m, blurred premultiplied plane b, blurred mask w, source s.
/// m <= thr or unordered gives s. Otherwise b is divided by min(w, 1) when w > 0, and the result is fma(b - s, min(m, 1), s).
/// `thr` is the constant (MB-A1): MEASURED-DYLIB, [`MASK_THRESHOLD`] = 2^-13. The caller still passes it explicitly.
#[inline]
pub fn combine_px(m: f32, b: f32, w: f32, s: f32, thr: f32) -> f32 {
    if !(m > thr) {
        return s;
    }
    let mut b = b;
    if w > 0.0 {
        b /= min_x86(w, 1.0);
    }
    let m1 = min_x86(m, 1.0);
    // vsubss d = b - s; vfmadd231ss s = d * m1 + s: NaN priority s, then d (m1 is never a NaN here)
    let d = b.fsub(s);
    fma231(s, d, m1)
}

/// Job over a whole dense plane.
pub fn combine(out: &mut [f32], m: &[f32], b: &[f32], w: &[f32], s: &[f32], thr: f32) {
    for i in 0..out.len() {
        out[i] = combine_px(m[i], b[i], w[i], s[i], thr);
    }
}

/// Parameters of the `masked_blur` effect and the host values the route reads. Object offsets are `this+` unless stated.
#[derive(Clone, Debug)]
pub struct MaskedBlurParams {
    /// this, this: amount, horizontal and vertical.
    pub amount: [f32; 2],
    /// this: filter_shape; passes = 1 if negative, else min(shape + 1, 3) (same rule as `blur`).
    pub filter_shape: i32,
    /// this: passed as the dispatcher's sixth argument, which the blur crates call `subpixel` (MEASURED neighbours: it reaches r9d of). Nonzero = fractional radii.
    pub subpixel: i32,
    /// Result of this->vfunc[31]: colour planes, 0 to 4.
    pub plane_count: usize,
    /// src as an integer; sigma = (amount * k) * 0.125 (two mulss in this order).
    pub render_scale: i32,
    /// src != 0 gives Some(f32 at src) (B-A5 of blur).
    pub aspect: Option<f32>,
    /// Constant used by the combine job (MB-A1). MEASURED-DYLIB: the value the original uses is [`MASK_THRESHOLD`]
    /// (2^-13, bytes 00 00 00 39 in the approved plugin file). The field stays an explicit parameter with no default and no
    /// constructor or `Default` impl fills it; callers that mean the original pass `MASK_THRESHOLD`. Live range: the compared mask
    /// is clamped to [0,1], so pixels with m <= thr (or NaN) keep the source; any thr below 1 changes outputs, thr >= 1 keeps every pixel.
    pub mask_threshold: f32,
}

/// Sigma of one lane : (amount * (float)render_scale) * 0.125.
pub fn lane_sigma(amount: f32, render_scale: i32) -> f32 {
    (amount * (render_scale as f32)) * 0.125f32
}

fn dense(p: &Plane) -> Vec<f32> {
    let mut v = Vec::with_capacity(p.width * p.height);
    for y in 0..p.height {
        v.extend_from_slice(p.row(y));
    }
    v
}

/// render (flag 0). `mask` is the host mask layer (this object, buffer at); it is required
/// (MB-A2: the original dereferences it unconditionally, None gives FxError::Geometry). The original clamps the mask buffer in
/// place to [0,1]; this port works on a clamped copy and leaves the input untouched (documented difference, not a pixel one).
/// Planes may be strided; they are repacked into dense scratch (PARITY is claimed on dense planes, as in blur).
pub fn render(p: &MaskedBlurParams, src: &[Plane], mask: Option<&Plane>, dst: &mut [PlaneMut]) -> Result<(), FxError> {
    render_debug(p, src, mask, dst).map(|_| ())
}

/// Scratch planes of the route, for T2 (the original leaves them in memory): the clamped mask, W = blur(mask), and T = the
/// blurred premultiplied buffer after the last plane.
#[derive(Clone, Debug, Default)]
pub struct Intermediates {
    pub mask: Vec<f32>,
    pub w: Vec<f32>,
    pub t: Vec<f32>,
}

/// Same as `render`, returning the intermediate buffers.
pub fn render_debug(p: &MaskedBlurParams, src: &[Plane], mask: Option<&Plane>, dst: &mut [PlaneMut]) -> Result<Intermediates, FxError> {
    let n = p.plane_count;
    if n > 4 || src.len() < n || dst.len() < n {
        return Err(FxError::PlaneCount);
    }
    let mask = mask.ok_or(FxError::Geometry)?;
    if n == 0 {
        return Ok(Intermediates::default());
    }
    let (w, h) = (mask.width, mask.height);
    if src[..n].iter().any(|s| s.width != w || s.height != h) || dst[..n].iter().any(|d| d.width != w || d.height != h) {
        return Err(FxError::Geometry);
    }
    let edge = Edge::Zero; // MEASURED: the wrapper passes ecx = 0 (xor ecx,ecx), the cell is not consulted
    let passes = pass_count(p.filter_shape);
    let sx = lane_sigma(p.amount[0], p.render_scale);
    let sy = lane_sigma(p.amount[1], p.render_scale);
    let sub = p.subpixel != 0;
    let np = w * h;

    let mut m = dense(mask);
    clamp_in_place(&mut m, 0.0, 1.0);
    let mut wbuf = vec![0.0f32; np];
    blur_core(&m, &mut wbuf, w, h, sx, sy, 1.0, edge, passes, sub, p.aspect)?;

    let mut last_t = Vec::new();
    for c in 0..n {
        let s = dense(&src[c]);
        let mut t = s.clone();
        premultiply_in_place(&mut t, &m);
        let tin = t.clone();
        blur_core(&tin, &mut t, w, h, sx, sy, 1.0, edge, passes, sub, p.aspect)?;
        let mut out = vec![0.0f32; np];
        combine(&mut out, &m, &t, &wbuf, &s, p.mask_threshold);
        let d = &mut dst[c];
        for y in 0..h {
            d.row_mut(y).copy_from_slice(&out[y * w..(y + 1) * w]);
        }
        last_t = t;
    }
    Ok(Intermediates { mask: m, w: wbuf, t: last_t })
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
        Assumption { id: "MB-A1", what: "combine job threshold: MEASURED-DYLIB, 0x39000000 = 2^-13 read from the approved plugin file (this refuted the old default 0.0); MASK_THRESHOLD holds it; mask_threshold stays an explicit parameter with no default", label: "MEASURED-DYLIB" },
        Assumption { id: "MB-A2", what: "mask is required (the original dereferences this unconditionally); None gives FxError::Geometry; the original clamps the host mask in place, the port clamps a copy", label: "ASSUMED" },
        Assumption { id: "MB-A3", what: "premultiply vector splat, 1.0f in every lane: MEASURED-DYLIB (CONFIRMED all 8 lanes T2 emulator reads the file bytes)", label: "MEASURED-DYLIB" },
        Assumption { id: "MB-A4", what: "render flag != 0 (16-bit path: uncovered) is not ported; flag 0 only", label: "ASSUMED (scope)" },
        Assumption { id: "MB-A5", what: "in-place blur of the premultiplied plane (src == dst in the dispatcher) is modelled as blur from a copy into the buffer; T2 route tests it against the aliased original", label: "ASSUMED (verified by T2)" },
        Assumption { id: "MB-A6", what: "cancel polls, roi record {w,h,c=w,d=0,e=4}, scratch phase 0, single worker: inherited from blur (B-A1, B-A2, B-A4, B-A6); edge mode is fixed to zero by the wrapper (MEASURED), so U03 does not apply", label: "ASSUMED" },
        Assumption { id: "MB-A7", what: "half != 0 (K2) is unreachable: the wrapper passes the dispatcher half = 0 (ecx xor)", label: "MEASURED listing" },
        Assumption { id: "MB-A8", what: "plane_count above 4 gives FxError::PlaneCount", label: "INFERRED" },
    ]
}
