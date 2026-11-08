//! Scale-or-copy (with job) and clear on dense buffers of w*h floats, render flag 0,
//! single worker (partition gives start 0, count total). Roi record: c == w, element 4 bytes (stand-in B-A2, B-U6).

use fx_common::nanop::NanOps;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelperError {
    /// A buffer is shorter than w*h floats.
    ShortBuffer,
}

/// Element count of the job: `imul` of two i32 (wraps), a non positive count does nothing.
fn count(w: i32, h: i32) -> usize {
    let n = w.wrapping_mul(h);
    if n > 0 { n as usize } else { 0 }
}

/// (dst, src, fl = 0, scale, roi). `src` is `None` when the original is called with dst == src.
/// scale == 1.0 (ucomiss equal, NaN is not equal): return when dst == src, else copy w*h floats.
/// Any other scale (NaN included) runs job : scale == 0.0 (-0.0 too) memsets w*h floats to 0 and ignores src,
/// otherwise dst[i] = src[i] * scale (mulss, f32).
pub fn scale_or_copy(dst: &mut [f32], src: Option<&[f32]>, scale: f32, w: i32, h: i32) -> Result<(), HelperError> {
    let n = count(w, h);
    if scale == 1.0 {
        if let Some(s) = src {
            if s.len() < n || dst.len() < n {
                return Err(HelperError::ShortBuffer);
            }
            dst[..n].copy_from_slice(&s[..n]);
        }
        return Ok(());
    }
    if dst.len() < n || src.map_or(false, |s| s.len() < n) {
        return Err(HelperError::ShortBuffer);
    }
    if scale == 0.0 {
        dst[..n].fill(0.0);
    } else {
        match src {
            Some(s) => fx_common::nanop::scale_into(&mut dst[..n], &s[..n], scale),
            None => fx_common::nanop::scale_in_place(&mut dst[..n], scale),
        }
    }
    Ok(())
}

/// (dst, fl = 0, 0, roi): memset(dst, 0, h*c*e) with c == w and e == 4, so w*h floats of +0.0.
pub fn clear(dst: &mut [f32], w: i32, h: i32) -> Result<(), HelperError> {
    let n = count(w, h);
    if dst.len() < n {
        return Err(HelperError::ShortBuffer);
    }
    dst[..n].fill(0.0);
    Ok(())
}
