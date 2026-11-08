//! Full glow layer: ROI + copy + bright pass + blur core, as the pass wrappers 0x33bca0 (one plane) and 0x33bfb0
//! (three planes) compose them, and the sigma derivation of the render body 0x52b220 (AVX2 class).
//! Output is the glow layer in the destination planes; the add-back onto the picture is not part of this crate.
use crate::bright::{bright_pass_flat, BrightParams};
use fx_common::nanop::NanOps;
use crate::plane::{check, FxError, Plane, PlaneMut};

/// 0.125f (bits 0x3e000000), read at VA 0x15eaf94.
const EIGHTH: f32 = 0.125;
/// The pass count constant of both wrappers: `mov r9d,3` at 0x33bdef and 0x33c0e4. MEASURED.
pub const GLOW_PASSES: i32 = 3;

/// Parameters of the glow render body 0x52b220, with the object offsets read by the body (S_Glow STRUCTURE thunks).
#[derive(Clone, Copy, Debug)]
pub struct GlowParams {
    /// this+0x6c, +0x70, +0x74 (lane pointer passed to the bright pass). One plane reads lane 0 only.
    pub thresholds: [f32; 3],
    /// this+0x88, +0x8c: horizontal and vertical stretch.
    pub width_xy: [f32; 2],
    /// this+0xa4, +0xa8, +0xac: width per colour plane. One plane reads lane 0 only.
    pub width: [f32; 3],
    /// this+0xc0, +0xc4, +0xc8: per plane gain handed to the blur core.
    pub brightness: [f32; 3],
    /// this+0xdc.
    pub subpixel: i32,
    /// this+0xf8: edge mode of the blur kernels (0 zero, 1 mirror; the original asserts inside the kernel on others).
    pub edge_mode: i32,
    /// this+0x114: nonzero returns the bright-pass layer without blurring.
    pub show_threshold: i32,
    /// result of this->vfunc[31] (call [vtable+0xf8]); 1 selects 0x33bca0, anything else 0x33bfb0 in the original.
    pub plane_count: usize,
    /// src+0x20, the integer render scale (vcvtsi2ss from [r12+0x20]).
    pub render_scale: i32,
    /// src+0x30 / src+0x38 read inside the blur core (caller field).
    pub aspect: Option<f32>,
}

/// Arguments of the pass wrappers 0x33bca0 / 0x33bfb0 after the render body derived them.
#[derive(Clone, Copy, Debug)]
pub struct PassArgs {
    pub thresholds: [f32; 3],
    pub sigma_x: [f32; 3],
    pub sigma_y: [f32; 3],
    pub brightness: [f32; 3],
    pub subpixel: i32,
    pub edge_mode: i32,
    pub show_threshold: i32,
    pub aspect: Option<f32>,
}

/// Sigma pairs exactly as the two branches of 0x52b220 multiply them (the association differs, MEASURED listing).
/// One plane (52b36b..52b3a3): w = (0.125 * width0) * (float)scale; sx = w * width_xy0; sy = w * width_xy1.
/// Three planes (52b269..52b2da): k = 0.125 * (float)scale; w_c = k * width_c; sx_c = width_xy0 * w_c; sy_c = w_c * width_xy1.
pub fn glow_sigmas(p: &GlowParams) -> ([f32; 3], [f32; 3]) {
    let s = p.render_scale as f32; // vcvtsi2ss, round to nearest even
    if p.plane_count == 1 {
        let w = EIGHTH.fmul(p.width[0]).fmul(s);
        ([w.fmul(p.width_xy[0]), 0.0, 0.0], [w.fmul(p.width_xy[1]), 0.0, 0.0])
    } else {
        let k = EIGHTH * s;
        let mut sx = [0.0f32; 3];
        let mut sy = [0.0f32; 3];
        for c in 0..3 {
            let w = k.fmul(p.width[c]);
            sx[c] = p.width_xy[0].fmul(w);
            sy[c] = w.fmul(p.width_xy[1]);
        }
        (sx, sy)
    }
}

/// Pass arguments from the render parameters.
pub fn pass_args(p: &GlowParams) -> PassArgs {
    let (sigma_x, sigma_y) = glow_sigmas(p);
    PassArgs { thresholds: p.thresholds, sigma_x, sigma_y, brightness: p.brightness, subpixel: p.subpixel, edge_mode: p.edge_mode, show_threshold: p.show_threshold, aspect: p.aspect }
}

/// Render body 0x52b220 minus the host plumbing: derive the arguments, run the one or three plane pass. a plane count
/// other than 1 or 3 is `PlaneCount` (the original sends every n != 1 to 0x33bfb0).
pub fn render_full(p: &GlowParams, src: &[Plane], dst: &mut [PlaneMut]) -> Result<(), FxError> {
    if (p.plane_count != 1 && p.plane_count != 3) || dst.len() != p.plane_count {
        return Err(FxError::PlaneCount);
    }
    glow_pass(src, dst, &pass_args(p))
}

/// 0x33bca0 (dst.len() == 1) or 0x33bfb0 (dst.len() == 3). Input plane for output c is src[min(c, src.len() - 1)] (33c00c..33c019).
/// Steps: copy (0x672c60), bright pass (0x673fa0), return if show_threshold, then blur core 0x1537a0 per plane in place with
/// passes 3 and half 0. Planes may be strided: they are gathered into dense buffers and only visible samples are written back.
pub fn glow_pass(src: &[Plane], dst: &mut [PlaneMut], a: &PassArgs) -> Result<(), FxError> {
    let n = dst.len();
    if (n != 1 && n != 3) || src.is_empty() {
        return Err(FxError::PlaneCount);
    }
    let (w, h) = (dst[0].width, dst[0].height);
    for d in dst.iter() {
        if d.width != w || d.height != h {
            return Err(FxError::Geometry);
        }
        check(d.data.len(), d.width, d.height, d.stride)?;
    }
    for s in src.iter() {
        if s.width != w || s.height != h {
            return Err(FxError::Geometry);
        }
        check(s.data.len(), s.width, s.height, s.stride)?;
    }
    if w == 0 || h == 0 {
        return Ok(());
    }
    let total = w * h;
    // copy: dense planes, contiguous copy of the visible samples
    let mut bufs: Vec<Vec<f32>> = Vec::with_capacity(n);
    for c in 0..n {
        let s = &src[c.min(src.len() - 1)];
        let mut b = Vec::with_capacity(total);
        for y in 0..h {
            b.extend_from_slice(&s.data[y * s.stride..y * s.stride + w]);
        }
        bufs.push(b);
    }
    // bright pass (0x673fa0): one plane reads thresholds[0] only
    {
        let bp = BrightParams { thresholds: if n == 1 { [a.thresholds[0], 0.0, 0.0] } else { a.thresholds } };
        let mut views: Vec<&mut [f32]> = bufs.iter_mut().map(|b| b.as_mut_slice()).collect();
        bright_pass_flat(&bp, &mut views, total)?;
    }
    if a.show_threshold == 0 {
        // invalid edge modes are rejected here, before any blur work (the original asserts inside the kernel)
        let edge = blur::Edge::from_i32(a.edge_mode).map_err(|_| FxError::Geometry)?;
        for c in 0..n {
            let mut out = vec![0.0f32; total];
            blur::blur_core(&bufs[c], &mut out, w, h, a.sigma_x[c], a.sigma_y[c], a.brightness[c], edge, GLOW_PASSES, a.subpixel != 0, a.aspect)
                .map_err(|_| FxError::Geometry)?;
            bufs[c] = out;
        }
    }
    for c in 0..n {
        let d = &mut dst[c];
        for y in 0..h {
            d.data[y * d.stride..y * d.stride + w].copy_from_slice(&bufs[c][y * w..(y + 1) * w]);
        }
    }
    Ok(())
}

/// Assumptions of the full glow layer .
pub fn full_assumptions() -> Vec<crate::bright::Assumption> {
    use crate::bright::Assumption;
    vec![
        Assumption { id: "plane-count", va: "0x52b260", value: "a plane count other than 1 or 3 returns PlaneCount; the original sends every n != 1 to 0x33bfb0", basis: "cmp eax,1 at 52b260", label: "conservative choice" },
        Assumption { id: "in-place-blur", va: "0x1537a0", value: "the in-place blur (rsi == rdx) is modelled as blur_core(copy, out)", basis: "listing 33bdc2..33bdc9 and 33c124..33c128; equality checked against the original's in-place call", label: "checked" },
        Assumption { id: "roi-form", va: "0x5b9b10", value: "ROI form 1 (info+8 == 0), dense planes, d = 0 and element bytes = 4", basis: "ROI constants at 0x160f274 and 0x160f280", label: "assumed" },
        Assumption { id: "aspect", va: "0x1537a0", value: "aspect (info+0x30, +0x38) is a caller field, None by default", basis: "blur core argument record", label: "assumed" },
        Assumption { id: "edge-mode", va: "0x1537a0", value: "an edge mode other than 0 or 1 gives FxError::Geometry before blur work, only when show_threshold == 0", basis: "the original asserts inside the blur kernel", label: "assumed" },
        Assumption { id: "pool-route", va: "0x674000", value: "the bright pass runs its inline path; the pool route is equal to it", basis: "pool and inline layouts compared byte for byte on the bright pass", label: "assumed" },
    ]
}
