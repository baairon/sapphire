//! Plane types (same shape as fx_common::plane).
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Plane<'a> {
    pub data: &'a [f32],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

#[derive(Debug)]
pub struct PlaneMut<'a> {
    pub data: &'a mut [f32],
    pub width: usize,
    pub height: usize,
    pub stride: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FxError {
    PlaneCount,
    Geometry,
    Stride,
}

impl fmt::Display for FxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FxError::PlaneCount => "wrong plane count",
            FxError::Geometry => "output geometry differs from input",
            FxError::Stride => "stride smaller than width or slice too short",
        })
    }
}
impl std::error::Error for FxError {}

pub(crate) fn check(len: usize, w: usize, h: usize, stride: usize) -> Result<(), FxError> {
    if w == 0 || h == 0 {
        return Ok(());
    }
    if stride < w || len < (h - 1) * stride + w {
        return Err(FxError::Stride);
    }
    Ok(())
}
