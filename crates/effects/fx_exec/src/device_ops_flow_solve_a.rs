//! Device launch functions of the optical flow solve tile kernels, first group: `optical_flow_solve::fma_mix`, `flow_update`,
//! `diff_sum`, `relax::relax_callback`, `smooth::scale_plane` and `reduce::zero_border` (the last two are the plane passes of the
//! stretch, `stretch::Direct::scale` and `Direct::border`). The kernels live in `fx_device::kernels_flow_solve_a`.
//!
//! These are `pub` launch functions with a compare driver (tests/device_flow_solve_a.rs); they add no `DeviceOp`. The host composition
//! of the whole solve keeps every control loop of the crate on the host and calls these per launch. Planes are device buffers that
//! stay resident (a buffer may be longer than stride times height); the plane byte phase is an argument, as in the crate.
//!
//! Argument model (refused with `DevError::Unsupported`, never approximated): widths of at least 1, a row stride of at least the width,
//! a non negative first row and row count, and every addressed row inside the buffer. The crate's loops do nothing for an empty row
//! range, and so do these functions. Host scalars (`relax_scale * flow_gain`) are computed with the crate's own `fmul`.

use fx_common::nanop::NanOps;
use fx_device::ocl::{Arg as ClArg, Buffer};
use fx_device::{DevError, DevImage, Session};
use optical_flow_pyramid::LevelRecord as Level;
use optical_flow_solve::stretch::PairPlane;
use std::sync::Arc;

fn refuse(why: &str) -> DevError {
    DevError::Unsupported(format!("optical_flow_solve tile kernel: {why}"))
}

/// Checks the row range of a plane launch; returns false for an empty range (nothing to do).
fn check_rows(len: usize, width: i32, stride: i32, row0: i32, rows: i32) -> Result<bool, DevError> {
    if rows <= 0 {
        if rows < 0 {
            return Err(refuse("negative row count"));
        }
        return Ok(false);
    }
    if width < 1 || stride < width || row0 < 0 {
        return Err(refuse("width below 1, stride below the width or negative first row"));
    }
    let end = (row0 as i64 + rows as i64) * stride as i64;
    if end > len as i64 || end > i32::MAX as i64 || (rows as i64) * (width as i64) > u32::MAX as i64 {
        return Err(refuse("rows outside the buffer"));
    }
    Ok(true)
}

/// `fma_mix`: rows `row0..row0+rows` of `out = -(e*c - b) - (f*d + a)`. `planes` is `[a, b, c, d, e, f]`.
pub fn fma_mix_dev(s: &Arc<Session>, planes: [&Buffer<f32>; 6], out: &Buffer<f32>, sh: &optical_flow_solve::Shape, row0: i32, rows: i32) -> Result<(), DevError> {
    let len = planes.iter().map(|p| p.len()).chain([out.len()]).min().unwrap_or(0);
    if !check_rows(len, sh.width, sh.stride, row0, rows)? {
        return Ok(());
    }
    s.launch(
        "flow_a_fma_mix",
        rows as usize * sh.width as usize,
        &[
            planes[0].arg(),
            planes[1].arg(),
            planes[2].arg(),
            planes[3].arg(),
            planes[4].arg(),
            planes[5].arg(),
            out.arg(),
            ClArg::I32(sh.width),
            ClArg::I32(sh.stride),
            ClArg::I32(row0),
            ClArg::U32(sh.phase_bytes),
        ],
    )?;
    Ok(())
}

/// The planes of `flow_update`.
pub struct FlowBufs<'a> {
    pub u: &'a Buffer<f32>,
    pub v: &'a Buffer<f32>,
    pub a: &'a Buffer<f32>,
    pub b: &'a Buffer<f32>,
    pub c: &'a Buffer<f32>,
}

/// `flow_update`: `out0 = u + step0`, `out1 = v + step1` over the rows.
#[allow(clippy::too_many_arguments)]
pub fn flow_update_dev(
    s: &Arc<Session>,
    inp: &FlowBufs,
    out0: &Buffer<f32>,
    out1: &Buffer<f32>,
    width: i32,
    stride: i32,
    row0: i32,
    rows: i32,
    relax_scale: f32,
    flow_gain: f32,
    eps: f32,
) -> Result<(), DevError> {
    let len = [inp.u.len(), inp.v.len(), inp.a.len(), inp.b.len(), inp.c.len(), out0.len(), out1.len()].into_iter().min().unwrap_or(0);
    if !check_rows(len, width, stride, row0, rows)? {
        return Ok(());
    }
    let sc = relax_scale.fmul(flow_gain);
    s.launch(
        "flow_a_flow_update",
        rows as usize * width as usize,
        &[
            inp.u.arg(),
            inp.v.arg(),
            inp.a.arg(),
            inp.b.arg(),
            inp.c.arg(),
            out0.arg(),
            out1.arg(),
            ClArg::I32(width),
            ClArg::I32(stride),
            ClArg::I32(row0),
            ClArg::U32(sc.to_bits()),
            ClArg::U32(eps.to_bits()),
        ],
    )?;
    Ok(())
}

/// `diff_sum`: copies `p` into `x` and `q` into `y` over the rows and returns the accumulated sum of squared differences, in the
/// crate's order (one work item, lane partial sums in the vector form). The caller's host loop reads the returned word.
pub fn diff_sum_dev(
    s: &Arc<Session>,
    p: &Buffer<f32>,
    q: &Buffer<f32>,
    x: &Buffer<f32>,
    y: &Buffer<f32>,
    sh: &optical_flow_solve::Shape,
    row0: i32,
    rows: i32,
) -> Result<f32, DevError> {
    let len = [p.len(), q.len(), x.len(), y.len()].into_iter().min().unwrap_or(0);
    if !check_rows(len, sh.width, sh.stride, row0, rows)? {
        return Ok(0.0);
    }
    let sum = DevImage::alloc(s, 1, 1)?;
    s.launch(
        "flow_a_diff_sum",
        1,
        &[
            p.arg(),
            q.arg(),
            x.arg(),
            y.arg(),
            sum.plane(0).arg(),
            ClArg::I32(sh.width),
            ClArg::I32(sh.stride),
            ClArg::I32(row0),
            ClArg::I32(rows),
            ClArg::U32(sh.phase_bytes),
        ],
    )?;
    Ok(sum.download()?[0][0])
}

/// `zero_border`: rows `start..start+count` of a `width` x `height` plane (first and last plane row whole, other rows at both ends).
pub fn zero_border_dev(s: &Arc<Session>, plane: &Buffer<f32>, width: i32, height: i32, stride: i32, start: i32, count: u32) -> Result<(), DevError> {
    if count > i32::MAX as u32 {
        return Err(refuse("row count above i32"));
    }
    if !check_rows(plane.len(), width, stride, start, count as i32)? {
        return Ok(());
    }
    if height < 1 {
        return Err(refuse("height below 1"));
    }
    s.launch(
        "flow_a_zero_border",
        count as usize * width as usize,
        &[plane.arg(), ClArg::I32(width), ClArg::I32(height), ClArg::I32(stride), ClArg::I32(start), ClArg::U32(count)],
    )?;
    Ok(())
}

/// `scale_plane`: `x = x * scale` in place over the rows.
pub fn scale_plane_dev(s: &Arc<Session>, plane: &Buffer<f32>, width: i32, stride: i32, scale: f32, start: i32, count: u32) -> Result<(), DevError> {
    if count > i32::MAX as u32 {
        return Err(refuse("row count above i32"));
    }
    if !check_rows(plane.len(), width, stride, start, count as i32)? {
        return Ok(());
    }
    s.launch(
        "flow_a_scale_plane",
        count as usize * width as usize,
        &[plane.arg(), ClArg::I32(width), ClArg::I32(stride), ClArg::I32(start), ClArg::U32(scale.to_bits())],
    )?;
    Ok(())
}

/// One nest of `relax_callback`: input plane, vertical difference plane, accumulator and output.
pub struct RelaxNest<'a> {
    pub t: &'a Buffer<f32>,
    pub u: &'a Buffer<f32>,
    pub acc: &'a Buffer<f32>,
    pub out: &'a Buffer<f32>,
}

/// `relax_callback` over rows `start..start+count`; `v` absent selects the crate's scale absent arms. Output planes must not alias
/// an input (the crate's slices cannot).
#[allow(clippy::too_many_arguments)]
pub fn relax_dev(
    s: &Arc<Session>,
    width: i32,
    height: i32,
    stride: i32,
    scale: f32,
    v: Option<&Buffer<f32>>,
    nests: [&RelaxNest; 2],
    phase_bytes: u32,
    start: i32,
    count: i32,
) -> Result<(), DevError> {
    let mut len = nests.iter().flat_map(|n| [n.t.len(), n.u.len(), n.acc.len(), n.out.len()]).min().unwrap_or(0);
    if let Some(v) = v {
        len = len.min(v.len());
    }
    if !check_rows(len, width, stride, start, count)? {
        return Ok(());
    }
    if height < 1 || start + count > height {
        return Err(refuse("rows outside the plane height"));
    }
    let vb = v.unwrap_or(nests[0].acc);
    s.launch(
        "flow_a_relax",
        2 * count as usize * width as usize,
        &[
            nests[0].t.arg(),
            nests[0].u.arg(),
            nests[0].acc.arg(),
            nests[0].out.arg(),
            nests[1].t.arg(),
            nests[1].u.arg(),
            nests[1].acc.arg(),
            nests[1].out.arg(),
            vb.arg(),
            ClArg::U32(scale.to_bits()),
            ClArg::I32(i32::from(v.is_some())),
            ClArg::I32(width),
            ClArg::I32(height),
            ClArg::I32(stride),
            ClArg::I32(start),
            ClArg::I32(count),
            ClArg::U32(phase_bytes),
        ],
    )?;
    Ok(())
}

/// The scale factor of the stretch for a flow plane: finer width over coarser width for `FlowU`, heights for `FlowV` (the text of
/// `stretch::run_stretch`).
pub fn stretch_ratio(j: PairPlane, l: &Level, lp: &Level) -> f32 {
    if j == PairPlane::FlowU {
        (l.width as f32).fdiv(lp.width as f32)
    } else {
        (l.height as f32).fdiv(lp.height as f32)
    }
}

/// `stretch::Direct::scale` on the device: the plane of finer level `l`, scaled by the ratio of the two levels.
pub fn stretch_scale_dev(s: &Arc<Session>, plane: &Buffer<f32>, j: PairPlane, l: &Level, lp: &Level) -> Result<(), DevError> {
    let (w, h) = (l.width, l.height);
    if w <= 0 || h <= 0 {
        return Ok(());
    }
    scale_plane_dev(s, plane, w, l.stride, stretch_ratio(j, l, lp), 0, h as u32)
}

/// `stretch::Direct::border` on the device: the plane of the coarser level `lp`.
pub fn stretch_border_dev(s: &Arc<Session>, plane: &Buffer<f32>, lp: &Level) -> Result<(), DevError> {
    let (w, h) = (lp.width, lp.height);
    if w <= 0 || h <= 0 {
        return Ok(());
    }
    zero_border_dev(s, plane, w, h, lp.stride, 0, h as u32)
}
