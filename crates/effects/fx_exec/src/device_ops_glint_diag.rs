//! Host helpers of the glint diagonal passes on the device: `glint_diag_box::run_body` and `diagonal_box_sum::box_sum_diagonals`.
//!
//! Each helper refuses by the crate's own error before any launch (a zero dimension, a short plane, the shift `i32::MIN`, a bad band),
//! computes the row phase tables on the host with the CPU's rotation of its scratch rows, and launches the kernels of
//! [`fx_device::kernels_glint_diag`] in chunks so the scratch rows stay bounded. A range error the CPU would report (a read or write
//! outside the plane) is flagged by the kernel and returned as [`DiagDevError::Range`]; the CPU path then reports the named error.

use diagonal_box_sum::{scale_factor, BoxSumError, Diagonal, Params as BoxParams};
use fx_device::cl::{Access, Arg as ClArg, Buffer};
use fx_device::Session;
use glint_diag_box::{normaliser, Band, Error, Params, RowPhases};
use std::sync::Arc;

/// Scratch words per launch chunk (128 MB).
const SCRATCH_WORDS: usize = 32 << 20;

#[derive(Debug)]
pub enum DiagDevError {
    /// The error `glint_diag_box` returns for the same input.
    Glint(Error),
    /// The error `diagonal_box_sum` returns for the same input.
    BoxSum(BoxSumError),
    /// The kernel saw an access outside the plane.
    Range,
    /// A shape the device path does not take (index range of the kernel's 32 bit plane indices).
    Unsupported(&'static str),
    Device(String),
}

fn dev<E: std::fmt::Debug>(e: E) -> DiagDevError {
    DiagDevError::Device(format!("{e:?}"))
}

fn ring_chunk(count: usize, words_per_item: usize) -> usize {
    (SCRATCH_WORDS / words_per_item.max(1)).clamp(1, count.max(1))
}

/// `glint_diag_box::run_body(mirrored, p, ph, band, src, dst, width, height)` on the device. `dst` keeps its words that the CPU leaves alone.
#[allow(clippy::too_many_arguments)]
pub fn glint_diag_box_dev(
    s: &Arc<Session>, mirrored: bool, p: &Params, ph: &RowPhases, band: Band, src: &[f32], dst: &mut [f32], width: usize, height: usize,
) -> Result<(), DiagDevError> {
    if mirrored && (width == 0 || height == 0 || src.len() < width * height || dst.len() < width * height) {
        return Err(DiagDevError::Glint(Error::Size));
    }
    if width == 0 || height == 0 || src.len() < width * height {
        return Err(DiagDevError::Glint(Error::Size));
    }
    if p.d == i32::MIN {
        return Err(DiagDevError::Glint(Error::ShiftMin));
    }
    if width.saturating_mul(height) > (i32::MAX as usize) / 4 || src.len() > i32::MAX as usize || dst.len() > i32::MAX as usize {
        return Err(DiagDevError::Unsupported("plane larger than the 32 bit index range"));
    }
    let (w, h) = (width as i64, height as i64);
    let f = normaliser(p.radii);
    let dd = p.d.wrapping_abs() as i64;
    let count = band.count.max(0) as usize;
    // row phases: the CPU rotates the roles of its three scratch rows with every pass, so a pass sees the phase of whichever row holds its input
    let mut table = vec![0u32; count.max(1) * 6];
    let (mut ra, mut rb, mut rc) = (0usize, 1usize, 2usize);
    for counter in 0..count {
        let t = counter as i64 + 1 + (band.start as i64 - h) + 1;
        let (a_, b_) = (t.max(0), (-t).max(0));
        let l = (w - 2 * a_).min(2 * h - 2 * b_);
        if l <= 0 {
            continue;
        }
        for k in 0..3 {
            if p.radii[k] <= 0 {
                continue;
            }
            table[counter * 6 + k] = ph.0[ra];
            std::mem::swap(&mut ra, &mut rc);
        }
        if dd != 0 {
            std::mem::swap(&mut ra, &mut rc);
        }
        for k in 0..3 {
            if p.radii[k] <= 0 {
                continue;
            }
            table[counter * 6 + 3 + k] = ph.0[rb];
            std::mem::swap(&mut rb, &mut rc);
        }
        if dd != 0 {
            std::mem::swap(&mut rb, &mut rc);
        }
    }
    let cap = (w.min(2 * h)).max(1) + 4;
    let chunk = ring_chunk(count, 3 * cap as usize);
    let ctx = &s.context;
    let sbuf = Buffer::from_slice(ctx, src, Access::ReadWrite).map_err(dev)?;
    let dbuf = Buffer::from_slice(ctx, dst, Access::ReadWrite).map_err(dev)?;
    let tbuf = Buffer::from_slice(ctx, &table, Access::ReadWrite).map_err(dev)?;
    let ebuf = Buffer::from_slice(ctx, &[0u32], Access::ReadWrite).map_err(dev)?;
    let scratch = Buffer::<f32>::new(ctx, chunk * 3 * cap as usize, Access::ReadWrite).map_err(dev)?;
    let mut first = 0usize;
    while first < count {
        let n = chunk.min(count - first);
        s.launch(
            "glint_diag_band_f32",
            n,
            &[
                sbuf.arg(), dbuf.arg(), scratch.arg(), tbuf.arg(), ebuf.arg(), ClArg::I32(width as i32), ClArg::I32(height as i32),
                ClArg::I64(src.len() as i64), ClArg::I64(dst.len() as i64), ClArg::I32(mirrored as i32), ClArg::I32(band.start),
                ClArg::I64(first as i64), ClArg::I64(n as i64), ClArg::I32(p.radii[0]), ClArg::I32(p.radii[1]), ClArg::I32(p.radii[2]),
                ClArg::I64(dd), ClArg::U32(f.to_bits()), ClArg::I64(cap),
            ],
        )
        .map_err(dev)?;
        first += n;
    }
    if band.start == 0 {
        s.launch(
            "glint_diag_edge_f32",
            1,
            &[
                sbuf.arg(), dbuf.arg(), ebuf.arg(), ClArg::I32(width as i32), ClArg::I32(height as i32), ClArg::I64(src.len() as i64),
                ClArg::I64(dst.len() as i64), ClArg::I32(mirrored as i32),
            ],
        )
        .map_err(dev)?;
    }
    let flag = s.queue.read(&ebuf).map_err(dev)?;
    let out = s.queue.read(&dbuf).map_err(dev)?;
    dst.copy_from_slice(&out);
    if flag[0] != 0 {
        return Err(DiagDevError::Range);
    }
    Ok(())
}

/// `diagonal_box_sum::box_sum_diagonals` in place on a device buffer. The crate's own error is returned apart from a device error.
pub fn diag_box_sum_buf(s: &Arc<Session>, img: &Buffer<f32>, p: &BoxParams) -> Result<Result<(), BoxSumError>, fx_device::DevError> {
    use fx_device::DevError;
    if p.w < 1 || p.h < 1 {
        return Ok(Err(BoxSumError::BadSize));
    }
    if img.len() != (p.w as usize) * (p.h as usize) {
        return Ok(Err(BoxSumError::BadLength));
    }
    if p.count <= 0 {
        return Ok(Ok(()));
    }
    if p.start < 0 || (p.start as i64 + p.count as i64) > (p.w as i64 + p.h as i64 - 1) {
        return Ok(Err(BoxSumError::BadBand));
    }
    if p.shift == i32::MIN || p.shift.abs() > 0x3fff_ffff {
        return Ok(Err(BoxSumError::BadShift));
    }
    if img.len() > i32::MAX as usize / 2 {
        return Err(DevError::Unsupported("plane larger than the 32 bit index range".into()));
    }
    let m = p.w.min(p.h) as usize;
    let a = p.shift.unsigned_abs() as i64;
    let rc = (m as i32 - 1) >> 1;
    let scale = scale_factor(p.radii, p.kind);
    let base_t = match p.kind {
        Diagonal::Anti => p.start - p.w,
        Diagonal::Main => p.start - p.h,
    };
    // the CPU rotates which of its five row buffers holds a diagonal's data across the passes of a group of four diagonals
    let mut table = [0u32; 12];
    let mut slot = [1usize, 2, 3, 4];
    let mut spare = 0usize;
    for cur in 0..4 {
        for k in 0..3 {
            if p.radii[k] <= 0 {
                continue;
            }
            let ib = slot[cur];
            table[cur * 3 + k] = p.phases[ib] as u32;
            slot[cur] = spare;
            spare = ib;
        }
        if p.shift != 0 {
            let ib = slot[cur];
            slot[cur] = spare;
            spare = ib;
        }
    }
    let count = p.count as usize;
    let chunk = ring_chunk(count, 2 * m);
    let ctx = &s.context;
    let tbuf = Buffer::from_slice(ctx, &table, Access::ReadWrite)?;
    let scratch = Buffer::<f32>::new(ctx, chunk * 2 * m, Access::ReadWrite)?;
    let mut first = 0usize;
    while first < count {
        let n = chunk.min(count - first);
        s.launch(
            "diag_box_sum_f32",
            n,
            &[
                img.arg(), scratch.arg(), tbuf.arg(), ClArg::I32(p.w), ClArg::I32(p.h), ClArg::I32(m as i32),
                ClArg::I32(matches!(p.kind, Diagonal::Main) as i32), ClArg::I32(base_t), ClArg::I64(a), ClArg::I32(p.radii[0]),
                ClArg::I32(p.radii[1]), ClArg::I32(p.radii[2]), ClArg::I32((p.shift != 0) as i32), ClArg::U32(scale.to_bits()),
                ClArg::I32(rc), ClArg::I64(first as i64), ClArg::I64(n as i64),
            ],
        )?;
        first += n;
    }
    Ok(Ok(()))
}

/// `diagonal_box_sum::box_sum_diagonals(img, p)` on the device, in place on a host slice (upload, [`diag_box_sum_buf`], read back).
pub fn diag_box_sum_dev(s: &Arc<Session>, img: &mut [f32], p: &BoxParams) -> Result<(), DiagDevError> {
    let ibuf = Buffer::from_slice(&s.context, img, Access::ReadWrite).map_err(dev)?;
    match diag_box_sum_buf(s, &ibuf, p) {
        Ok(Ok(())) => {}
        Ok(Err(e)) => return Err(DiagDevError::BoxSum(e)),
        Err(fx_device::DevError::Unsupported(w)) => return Err(DiagDevError::Device(format!("unsupported: {w}"))),
        Err(e) => return Err(dev(e)),
    }
    let out = s.queue.read(&ibuf).map_err(dev)?;
    img.copy_from_slice(&out);
    Ok(())
}
