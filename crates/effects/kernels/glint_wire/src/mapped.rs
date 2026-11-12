//! The three mapped services each a call into a ported crate or one fill.
//! Blur flag 0 shape of 0x1b32b0: Edge 1 (Mirror), passes 3, subpixel 1, gain 1.0, sigma_x = sigma_y = x, half 0, BLUR-ALIGN phase 3 default 0.
use blur::{blur_core_phase, Edge};
use fx_common::plane::FxError;
use glow_layer::{bright_pass_in_place, BrightParams, PlaneMut};

/// Scratch base phase of the blur passes (BLUR-ALIGN), a host value; 0 unless a counted render records another.
pub const BLUR_ALIGN_DEFAULT: usize = 0;

/// Call::Blur in the flag-0 shape. `aspect` is the value the driver reads from the record fields, None when the field is off.
pub fn blur_flag0(src: &[f32], dst: &mut [f32], w: usize, h: usize, x: f32, aspect: Option<f32>, phase: usize) -> Result<(), FxError> {
    blur_core_phase(src, dst, w, h, x, x, 1.0, Edge::Mirror, 3, true, aspect, phase)
}

/// Call::BrightPass: in place on three equal planes, thresholds = the float[3] at object + 0x6c.
pub fn bright_inplace(th: [f32; 3], planes: &mut [PlaneMut]) -> Result<(), String> {
    bright_pass_in_place(&BrightParams { thresholds: th }, planes).map_err(|e| format!("{e:?}"))
}

/// Call::Fill with value 0: zero the first `third * h` floats of the destination (0x673100 with esi 0; bytes = third * h * element size 4).
pub fn zero_fill(dst: &mut [f32], third: usize, h: usize) -> Result<(), FxError> {
    let n = third.checked_mul(h).ok_or(FxError::Geometry)?;
    if n > dst.len() {
        return Err(FxError::Stride);
    }
    for v in &mut dst[..n] {
        *v = 0.0;
    }
    Ok(())
}
