//! Device launch functions of the optical flow solve kernels, part B: `smooth_normalise` (both passes), the 3x3 median network
//! (`median_rows`) with the strided copy of `median_pair`, and the vector power and exponential lanes of `vmath`.
//!
//! These are `pub` launch functions with no `DeviceOp` entry: the composed `optical_flow` op calls them. Every plane is a device buffer
//! of 32 bit words. The host computes what the CPU functions compute before their loops (the damping factor, the product of the
//! constant and the damping factor, the row range tests) with the crate's own functions, checks every index the kernel will touch
//! (the CPU would panic or return `MedianError`; the device refuses with `DevError::Unsupported` before any launch), and uploads the
//! crate constants and the crate's computed tables (`device_ops_flow_solve_b_gen`).

use fx_common::nanop::NanOps;
use fx_device::ocl::{Arg as ClArg, Buffer};
use fx_device::{DevError, Session};
use optical_flow_solve::smooth::{damping, Geo, ONE};
use optical_flow_solve::tail::{median_pair, TailHost};
use optical_flow_solve::{Level, LaunchArgs};
use std::sync::Arc;

fn refuse<T>(why: &str) -> Result<T, DevError> {
    Err(DevError::Unsupported(format!("flow solve B: {why}")))
}

/// The uploaded crate constants and tables.
struct Uploads {
    kc: Buffer<u32>,
    tab: Buffer<u32>,
}

fn uploads(s: &Arc<Session>) -> Result<Uploads, DevError> {
    let kc = s.input(&crate::device_ops_flow_solve_b_gen::kconsts())?;
    let tab = s.input(&crate::device_ops_flow_solve_b_gen::tables())?;
    Ok(Uploads { kc, tab })
}

/// One plane triple of a smoothing pass: the read only source and the two planes updated in place.
pub struct SmoothPass<'a> {
    pub src: &'a Buffer<f32>,
    pub p: &'a Buffer<f32>,
    pub q: &'a Buffer<f32>,
}

/// Index checks of one pass: every row's start and the next row's start must keep the touched elements inside the planes.
fn check_pass(g: &Geo, has_scale: bool, pl: &SmoothPass, scale: Option<&Buffer<f32>>, start: i32, count: u32) -> Result<(), DevError> {
    let w = g.width as i64;
    let mut lens = vec![pl.src.len(), pl.p.len(), pl.q.len()];
    if has_scale {
        lens.push(scale.map(|b| b.len()).unwrap_or(0));
    }
    let min_len = *lens.iter().min().unwrap() as i64;
    for k in 0..count {
        let y = start.wrapping_add(k as i32);
        let off = y.wrapping_mul(g.stride) as i64;
        if off < 0 || off + w > min_len {
            return refuse("a row outside the planes");
        }
        let last_row = y >= g.height.wrapping_sub(1);
        if !last_row {
            let nxt = off + g.stride as i64;
            if nxt < 0 || nxt + w > pl.src.len() as i64 {
                return refuse("the next row outside the source plane");
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn launch_pass(
    s: &Arc<Session>,
    g: &Geo,
    pl: &SmoothPass,
    scale: Option<&Buffer<f32>>,
    start: i32,
    count: u32,
    second: bool,
) -> Result<(), DevError> {
    let w = g.width as i64;
    if w <= 0 {
        return Ok(());
    }
    let has_scale = scale.is_some();
    check_pass(g, has_scale, pl, scale, start, count)?;
    let total = w * count as i64;
    if total == 0 {
        return Ok(());
    }
    if total > u32::MAX as i64 {
        return refuse("a pass larger than 2^32 elements");
    }
    let (c, d) = (g.c, damping(g));
    let cd = c.fmul(d);
    let dummy = scale.unwrap_or(pl.src);
    s.launch(
        "ofb_smooth_u32",
        total as usize,
        &[
            pl.src.arg(),
            pl.p.arg(),
            pl.q.arg(),
            dummy.arg(),
            ClArg::U32(u32::from(has_scale)),
            ClArg::U32(w as u32),
            ClArg::I32(g.height),
            ClArg::I32(g.stride),
            ClArg::I32(start),
            ClArg::U32(c.to_bits()),
            ClArg::U32(d.to_bits()),
            ClArg::U32(cd.to_bits()),
            ClArg::U32(ONE.to_bits()),
            ClArg::U32(u32::from(second)),
            ClArg::U32(total as u32),
        ],
    )?;
    Ok(())
}

/// `optical_flow_solve::smooth::smooth_normalise` on the device: pass one over the whole row range, then pass two.
/// The planes of the two passes and the scale plane must be distinct buffers (the crate's slices cannot alias).
pub fn smooth_normalise_dev(
    s: &Arc<Session>,
    g: &Geo,
    scale: Option<&Buffer<f32>>,
    pass1: &SmoothPass,
    pass2: &SmoothPass,
    start: i32,
    count: u32,
) -> Result<(), DevError> {
    if start >= start.wrapping_add(count as i32) {
        return Ok(());
    }
    // check both passes before any launch, so a refusal leaves the planes untouched
    if g.width > 0 {
        check_pass(g, scale.is_some(), pass1, scale, start, count)?;
        check_pass(g, scale.is_some(), pass2, scale, start, count)?;
    }
    launch_pass(s, g, pass1, scale, start, count, false)?;
    launch_pass(s, g, pass2, scale, start, count, true)
}

/// Index range check of `median_rows`: the CPU returns `MedianError::OutOfRange` (after partial writes) when any touched index is
/// outside a plane; the device refuses before the launch instead.
fn check_median(src_len: usize, dst_len: usize, width: i32, rows: i32, pitch: i32, start: i32, count: i32) -> Result<(), DevError> {
    let row_end = count.wrapping_add(start);
    if start >= row_end {
        return Ok(());
    }
    let len = src_len.min(dst_len) as i64;
    let w = width as i64;
    let pitch_i = pitch as i64;
    let rows_minus_one = rows.wrapping_sub(1);
    let mut row = start;
    let mut row_off = pitch.wrapping_mul(row) as i64;
    let span = if w > 2 { w - 1 } else { 1 };
    while row < row_end {
        let cu = row_off;
        let pv = if row <= 0 { cu } else { row_off - pitch_i };
        let nx = if row >= rows_minus_one { cu } else { row_off + pitch_i };
        let lo = cu.min(pv).min(nx);
        let hi = cu.max(pv).max(nx) + span;
        if lo < 0 || hi >= len {
            return refuse("a median access outside the planes");
        }
        row = row.wrapping_add(1);
        row_off += pitch_i;
    }
    Ok(())
}

/// `optical_flow_solve::median::median_rows` on the device: one work item per row (the rows carry no state between them).
pub fn median_rows_dev(
    s: &Arc<Session>,
    src: &Buffer<f32>,
    dst: &Buffer<f32>,
    width: i32,
    rows: i32,
    pitch: i32,
    start: i32,
    count: i32,
) -> Result<(), DevError> {
    let row_end = count.wrapping_add(start);
    if start >= row_end {
        return Ok(());
    }
    check_median(src.len(), dst.len(), width, rows, pitch, start, count)?;
    let n = (row_end as i64 - start as i64) as usize;
    s.launch(
        "ofb_median_u32",
        n,
        &[
            src.arg(),
            dst.arg(),
            ClArg::I32(width),
            ClArg::I32(rows),
            ClArg::I32(pitch),
            ClArg::I32(start),
            ClArg::U32(1),
            ClArg::U32(n as u32),
            ClArg::U32(n as u32),
        ],
    )?;
    Ok(())
}

/// `solve::copy_rect`: the strided copy of `median_pair`'s write back (dst rectangle from src rectangle).
pub fn copy_rect_dev(s: &Arc<Session>, dst: &Buffer<f32>, src: &Buffer<f32>, l: &Level) -> Result<(), DevError> {
    let (cw, rows_h, stride) = (l.width as u32, l.height as u32, l.stride as u32 as usize);
    if !(cw > 0 && cw < 0x10000 && rows_h > 0 && rows_h < 0x10000) {
        return Ok(());
    }
    let need = (rows_h as usize - 1) * stride + cw as usize;
    if need > dst.len() || need > src.len() {
        return refuse("a copy outside the planes");
    }
    s.launch(
        "ofb_copy_rect_u32",
        cw as usize * rows_h as usize,
        &[dst.arg(), src.arg(), ClArg::U32(cw), ClArg::U32(rows_h), ClArg::U32(stride as u32)],
    )?;
    Ok(())
}

/// Resolves a plane handle of the level record to its device buffer.
pub trait DevPlanes {
    fn plane(&self, handle: u64) -> Option<&Buffer<f32>>;
}

struct DevTail<'a, P: DevPlanes> {
    s: &'a Arc<Session>,
    planes: &'a P,
    l: Level,
    err: Option<DevError>,
}

impl<'a, P: DevPlanes> TailHost for DevTail<'a, P> {
    fn launch(&mut self, args: LaunchArgs) {
        if self.err.is_some() {
            return;
        }
        let LaunchArgs::Median(m) = args else {
            self.err = Some(DevError::Unsupported("flow solve B: the tail launches only the median".into()));
            return;
        };
        let (Some(src), Some(dst)) = (self.planes.plane(m.plane), self.planes.plane(m.median_out)) else {
            self.err = Some(DevError::Unsupported("flow solve B: a median plane is not resident".into()));
            return;
        };
        let l = self.l;
        if let Err(e) = median_rows_dev(self.s, src, dst, l.width, l.height, l.stride, 0, l.height) {
            self.err = Some(e);
        }
    }
    fn copy_back(&mut self, plane: u64, median_out: u64, _pitch: u64, _head: [u64; 3]) {
        if self.err.is_some() {
            return;
        }
        let (Some(dst), Some(src)) = (self.planes.plane(plane), self.planes.plane(median_out)) else {
            self.err = Some(DevError::Unsupported("flow solve B: a median plane is not resident".into()));
            return;
        };
        if let Err(e) = copy_rect_dev(self.s, dst, src, &self.l) {
            self.err = Some(e);
        }
    }
}

/// `optical_flow_solve::tail::median_pair` on the device: the crate's own control code runs on the host through its `TailHost` trait; the
/// median launch and the copy back are device kernels on resident planes.
pub fn median_pair_dev<P: DevPlanes>(s: &Arc<Session>, l: &Level, planes: &P) -> Result<(), DevError> {
    let mut host = DevTail { s, planes, l: *l, err: None };
    median_pair(l, &mut host);
    match host.err {
        Some(e) => Err(e),
        None => Ok(()),
    }
}

/// The vector power lane (`vmath::powf_lane`, mode 0) or exponential lane (`vmath::expf_lane`, mode 1) on `n` words.
pub fn vmath_dev(s: &Arc<Session>, pow: bool, xs: &Buffer<f32>, ys: &Buffer<f32>, out: &Buffer<f32>, n: usize) -> Result<(), DevError> {
    if n == 0 {
        return Ok(());
    }
    if xs.len() < n || out.len() < n || (pow && ys.len() < n) {
        return refuse("a short plane");
    }
    let u = uploads(s)?;
    s.launch(
        "ofb_vmath_u32",
        n,
        &[xs.arg(), ys.arg(), out.arg(), u.kc.arg(), u.tab.arg(), ClArg::U32(if pow { 0 } else { 1 }), ClArg::U32(n as u32)],
    )?;
    Ok(())
}
