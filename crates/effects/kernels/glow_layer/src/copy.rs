//! Host copy helper 0x672c60 (plane copy over the ROI record), host path (render flag 0). Written to move into fx_common unchanged:
//! it needs only `FxError` from the plane module of the same shape (`crate::plane::FxError`).
//!
//! Semantics, read from the listing (MEASURED 672c60..672d4a): arguments rdi = dst, rsi = src, edx = flag, and the 24 byte ROI
//! record by value on the stack (rec+0 a, rec+4 b, rec+8 c, rec+0xc float, rec+0x10 element size in bytes).
//! * dst == src: return at once.
//! * flag != 0: strided device copy through 0xb5c2c0 (not ported; a nonzero render flag is outside this crate).
//! * flag == 0: if c != a the diagnostic 0x66a800 runs (it only logs, the copy goes on); then
//!   bytes = (i32)(c * b) (32 bit imul, sign extended) * element size (64 bit), and memcpy(dst, src, bytes) is done through
//!   0x136fbb0 directly (pool flag word nonzero) or through 0x670340 (flag word zero), which is the same memcpy in tiles.
//! So the copy is ONE contiguous run of c*b elements from the start of both planes. It does not look at a stride.
use crate::plane::FxError;

/// The 24 byte ROI record built by 0x5b9b10 (listing 5b9b10..5b9bce).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RoiRecord {
    /// rec+0: src+0x28 (width)
    pub a: i32,
    /// rec+4: src+0x2c (height)
    pub b: i32,
    /// rec+8: src+0x28 in the first form (src+8 == 0), src+0x44 in the second form (src+8 != 0)
    pub c: i32,
    /// rec+0xc: float from the ROI constants (0.0, captured), not read by the copy
    pub aux: f32,
    /// rec+0x10: element size in bytes (qword 4 from the ROI constants, captured)
    pub elem_bytes: i64,
}

impl RoiRecord {
    /// First form of 0x5b9b10: {w, h, w, 0.0, 4}.
    pub fn form1(w: i32, h: i32) -> Self {
        RoiRecord { a: w, b: h, c: w, aux: 0.0, elem_bytes: 4 }
    }
    /// Second form of 0x5b9b10: {w, h, p, 0.0, 4} with p = src+0x44.
    pub fn form2(w: i32, h: i32, p: i32) -> Self {
        RoiRecord { a: w, b: h, c: p, aux: 0.0, elem_bytes: 4 }
    }
    /// Element count n = c * b with the 32 bit wrap of `imul esi, [rbp+0x14]` (672d27) and of the bright pass (674101).
    pub fn count(&self) -> i32 {
        self.c.wrapping_mul(self.b)
    }
    /// Byte length passed to memcpy: sign extended count times the 64 bit element size (672d2e, 672d34).
    pub fn byte_len(&self) -> i64 {
        (self.count() as i64).wrapping_mul(self.elem_bytes)
    }
}

/// What the call did besides copying.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CopyReport {
    /// Elements copied (count).
    pub elements: usize,
    /// c != a: the original calls the diagnostic 0x66a800 (log only) and copies anyway.
    pub width_mismatch: bool,
}

/// Copy `roi.count()` f32 elements from the start of `src` to the start of `dst` (host path, flag 0).
/// Errors (outside the original's contract, which would fault or copy a huge size): negative or non f32 byte length gives
/// `FxError::Geometry`; a slice shorter than the count gives `FxError::Stride`.
/// `dst == src` cannot be expressed with Rust slices (the original returns early there).
pub fn copy_plane(dst: &mut [f32], src: &[f32], roi: &RoiRecord) -> Result<CopyReport, FxError> {
    let bytes = roi.byte_len();
    if bytes < 0 || bytes % 4 != 0 {
        return Err(FxError::Geometry);
    }
    let n = (bytes / 4) as usize;
    if n > dst.len() || n > src.len() {
        return Err(FxError::Stride);
    }
    dst[..n].copy_from_slice(&src[..n]);
    Ok(CopyReport { elements: n, width_mismatch: roi.c != roi.a })
}
