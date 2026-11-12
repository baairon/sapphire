//! Class `blur_channels` (vtable 0x1b04e50 at load, id getter 0x5921b0 returns "blur_channels"): render, slot 52 at 0x52cc40.
//! Per channel blur: each colour plane i has its own width factor and gain; the blur itself is the ported `blur` crate (driver 0x1537a0).
//! Source of truth: the neutral_1 dump listing 0x52cc40 to 0x52cf5a. Route: flag 0, single worker, dense planes, half 0. Std only, no unsafe.
use blur::{blur_core, pass_count, Edge};
use fx_common::nanop::NanOps;
pub use fx_common::plane::{FxError, Plane, PlaneMut};

/// Constant of the listing cell 0x15e8d94 (0.125f, read from the emulated image).
const EIGHTH: f32 = 0.125;

#[derive(Clone, Debug)]
pub struct ChannelParams {
    /// this+0x6c, this+0x70: amount lanes.
    pub amount: [f32; 2],
    /// this+0x88: per channel width factors.
    pub widths: [f32; 4],
    /// this+0xc0: per channel gains.
    pub gains: [f32; 4],
    /// this+0xa4: filter_shape (passes = 1 if negative, else min(shape + 1, 3)).
    pub filter_shape: i32,
    /// this+0xb4: number of supplied gain lanes.
    pub scale_count: i32,
    /// this+0xdc: subpixel.
    pub subpixel: i32,
    /// cell 0x1a98f18 pointee: 0 zero, 1 mirror.
    pub edge_mode: i32,
    /// this->vfunc[31]: colour plane count.
    pub plane_count: usize,
    /// this+0x40: number of supplied source planes (at least 1).
    pub n_in: usize,
    /// ctx+0x20: render scale.
    pub render_scale: i32,
    /// ctx+0x30 / +0x38: optional aspect.
    pub aspect: Option<f32>,
}

/// Gain of channel `c`: forced to 1.0 for c == 3 when scale_count < 4, and for c == 1 when two planes and scale_count == 1.
pub fn channel_gain(p: &ChannelParams, c: usize) -> f32 {
    if c == 3 {
        if p.scale_count < 4 {
            return 1.0;
        }
    } else if p.plane_count == 2 && c == 1 && p.scale_count == 1 {
        return 1.0;
    }
    p.gains[c]
}

fn abs_bits(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fff_ffff)
}

/// Sigmas of channel `c`: a = (amount0 * 0.125) * k, b = (0.125 * amount1) * k, sx = |a * w|, sy = |w * b| (mulss operand order of the listing).
pub fn channel_sigmas(p: &ChannelParams, c: usize) -> (f32, f32) {
    let k = p.render_scale as f32;
    let a = p.amount[0].fmul(EIGHTH).fmul(k);
    let b = EIGHTH.fmul(p.amount[1]).fmul(k);
    let w = p.widths[c];
    (abs_bits(a.fmul(w)), abs_bits(w.fmul(b)))
}

/// render: planes at or beyond n_in start as copies of the last supplied plane (copy chain 0x672c60), then every plane c in 0..plane_count is blurred into dst[c].
pub fn render(p: &ChannelParams, src: &[Plane], dst: &mut [PlaneMut]) -> Result<(), FxError> {
    let n = p.plane_count;
    if n == 0 {
        return Ok(());
    }
    if n > 4 || p.n_in == 0 || src.len() < p.n_in.min(n) || dst.len() < n {
        return Err(FxError::PlaneCount);
    }
    let (w, h) = (dst[0].width, dst[0].height);
    if dst[..n].iter().any(|d| d.width != w || d.height != h || d.stride != w) || src[..p.n_in.min(n)].iter().any(|s| s.width != w || s.height != h || s.stride != w) {
        return Err(FxError::Geometry);
    }
    let edge = Edge::from_i32(p.edge_mode)?;
    let passes = pass_count(p.filter_shape);
    // copy chain: local[j] for j >= n_in is the destination plane j holding a copy of local[j - 1]
    let last = p.n_in.min(n) - 1;
    let mut srcs: Vec<Vec<f32>> = Vec::with_capacity(n);
    for j in 0..n {
        if j < p.n_in {
            srcs.push(src[j].data[..w * h].to_vec());
        } else {
            srcs.push(src[last].data[..w * h].to_vec());
        }
    }
    for c in 0..n {
        let (sx, sy) = channel_sigmas(p, c);
        blur_core(&srcs[c], &mut dst[c].data[..w * h], w, h, sx, sy, channel_gain(p, c), edge, passes, p.subpixel != 0, p.aspect)?;
    }
    Ok(())
}


/// maxss: the second operand when either is NaN or the operands compare not greater.
fn maxss(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

/// cvttss2si: NaN and out of range give 0x80000000.
fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// Slot 57 (0x52cb30), region growth in pixels along x and y: m = max over the planes of the width factors (from 0),
/// a = max((amount0 * 0.125) * k, 0) * m, b = max((0.125 * amount1) * k, 0) * m, times the aspect when present; each rounded up and converted.
/// The rounding helper 0xb6f7b0 is modelled as ceilf (stand-in BC-A5).
pub fn roi_expand(p: &ChannelParams) -> (i32, i32) {
    let mut m = 0.0f32;
    for c in 0..p.plane_count {
        m = maxss(m, p.widths[c]);
    }
    let k = p.render_scale as f32;
    let a = maxss(p.amount[0].fmul(EIGHTH).fmul(k), 0.0);
    let b = maxss(EIGHTH.fmul(p.amount[1]).fmul(k), 0.0);
    let x = a.fmul(m);
    let mut y = m.fmul(b);
    if let Some(asp) = p.aspect {
        y = y.fmul(asp);
    }
    (cvtt(x.ceil()), cvtt(y.ceil()))
}

/// Slot 38 (0x52bb70) collapse test over the two binding predicates of the shared base class (0x4ca7b0 `a(i)`, 0x4ca650 `b(i)`,
/// taken as host services): 4 when a(4); 0 when (a(1) or a(2)) and b(4); otherwise -1. The rest of the route is the predicate bodies.
pub fn binding_collapse(a_has_one: bool, a_has_two: bool, a_has_four: bool, b_has_four: bool) -> i32 {
    if a_has_four {
        4
    } else if (a_has_one || a_has_two) && b_has_four {
        0
    } else {
        -1
    }
}

#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub what: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "BC-A1", what: "render flag (cl) is 0; the flag 1 copy and driver paths are not ported", label: "ASSUMED" },
        Assumption { id: "BC-A2", what: "dense planes, roi record {w, h, c = w, d = 0, e = 4}, single worker, polls return 0 (inherited from the blur crate)", label: "ASSUMED" },
        // Edge mode cell: 0x1a98f18.
        Assumption { id: "BC-A3", what: "edge mode cell points at an int; its value 1 is MEASURED-EMULATOR-READ, other values are labelled patches", label: "MEASURED value 1" },
        // Rounding helper: 0xb6f7b0.
        Assumption { id: "BC-A5", what: "rounding helper of slot 57 is ceilf (roundss imm 2); its fenv dispatch variants are not executed", label: "INFERRED" },
        Assumption { id: "BC-A4", what: "n_in at least 1 (this+0x40 <= 0 skips the copy and reads uninitialised stack in the original)", label: "ASSUMED" },
    ]
}
