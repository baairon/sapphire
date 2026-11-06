//! Driver arithmetic and the wrapper decision (flag 0 route).

/// 0.125f, in the captured data image (MEASURED, bytes 0000003e; re-read by tests/t1.rs).
pub const RADIUS_SCALE: f32 = 0.125;

/// Radius exactly as : `cvtsi2ss width` (round to nearest even), `movss C; mulss amount; mulss width`.
/// Two multiplications, no FMA.
#[inline]
pub fn radius(amount: f32, width: i32) -> f32 {
    let w = width as f32; // cvtsi2ss: i32 -> f32 round to nearest even
    (RADIUS_SCALE * amount) * w
}

/// Gain index: `idx = ch >= in_count ? in_count : ch` (cmovge); may equal in_count.
#[inline]
pub fn gain_index(ch: usize, in_count: usize) -> usize {
    if ch >= in_count { in_count } else { ch }
}

/// What the wrapper does for one plane.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlanePlan {
    /// comiss 0, radius; jae (taken when 0 >= radius, NaN radius not taken) or ucomiss gain, 0; je (gain == 0 taken, NaN not taken):
    /// the plane is copied through .
    Copy,
    /// flag 0 else path: blur_core(sx = sy = radius, gain 1.0, mirror, 3 passes, subpixel 0, half 0) then combine with `gain`.
    Run { radius: f32, gain: f32 },
}

/// Wrapper decision for flag 0.
#[inline]
pub fn plane_plan(radius: f32, gain: f32) -> PlanePlan {
    if 0.0 >= radius {
        return PlanePlan::Copy;
    }
    if gain == 0.0 {
        return PlanePlan::Copy;
    }
    PlanePlan::Run { radius, gain }
}

/// Blur-step arguments the wrapper hands to (MEASURED).
pub const BLUR_EDGE_MIRROR: i32 = 1;
pub const BLUR_PASSES: i32 = 3;
pub const BLUR_SUBPIXEL: i32 = 0;
pub const BLUR_HALF: i32 = 0;
/// Gain handed to the blur dispatcher: 1.0f from .
pub const BLUR_GAIN: f32 = 1.0;
