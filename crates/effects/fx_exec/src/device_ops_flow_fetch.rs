//! Launch functions of the optical flow fetch, resample and plane row kernels (`fx_device::kernels_flow_fetch`). No `DeviceOp` is
//! registered here: the optical flow op composes these launches with a device plane store. Each function is a launch over a whole
//! plane range, its operands are device buffers, and each returns `DevError::Unsupported` for a shape the kernel does not cover
//! (non positive sizes, a stride of zero where the CPU function writes one address from several columns, an element count outside
//! 32 bits). The tile ranges (`row0`, `rows`) are the crates' own arguments.
//!
//! Host values: the resample window table (`resample_table`) repeats the crate's `setup` and `window` arithmetic with the same
//! operations in the same order, because both passes read one window per output index and the crate computes it by a running sum.

use fx_common::nanop::NanOps;
use fx_common::x86::cvtss2si32_rne;
use fx_device::cl::{Arg as ClArg, Buffer};
use fx_device::{DevError, Session};
use std::sync::Arc;

fn unsup(m: &str) -> DevError {
    DevError::Unsupported(m.into())
}

fn launch(s: &Arc<Session>, name: &str, n: usize, args: &[ClArg<'_>]) -> Result<(), DevError> {
    s.launch(name, n, args)?;
    Ok(())
}

fn count(a: i32, b: i32) -> Result<u32, DevError> {
    if a <= 0 || b <= 0 {
        return Err(unsup("non positive extent"));
    }
    let n = (a as i64) * (b as i64);
    u32::try_from(n).map_err(|_| unsup("element count outside 32 bits"))
}

/// The four constants of the five point derivative, as bits: scalar scale, scalar weight, vector scale, vector weight.
#[derive(Clone, Copy, Debug)]
pub struct StencilBits {
    pub k_s: u32,
    pub c_s: u32,
    pub k_v: u32,
    pub c_v: u32,
}

#[allow(clippy::too_many_arguments)]
pub fn deriv_x_launch(s: &Arc<Session>, inp: &Buffer<f32>, out: &Buffer<f32>, width: i32, stride: i32, row0: i32, rows: i32, out_phase: u32, c: &StencilBits) -> Result<(), DevError> {
    let n = count(width, rows)?;
    if stride <= 0 {
        return Err(unsup("stride must be positive"));
    }
    launch(s, 
        "flow_b_deriv_x",
        n as usize,
        &[inp.arg(), out.arg(), ClArg::I32(width), ClArg::I32(stride), ClArg::I32(row0), ClArg::I32(rows), ClArg::U32(out_phase), ClArg::U32(c.k_s), ClArg::U32(c.c_s), ClArg::U32(c.k_v), ClArg::U32(c.c_v), ClArg::U32(n)],
    )
}

#[allow(clippy::too_many_arguments)]
pub fn deriv_y_launch(s: &Arc<Session>, inp: &Buffer<f32>, out: &Buffer<f32>, height: i32, stride: i32, col0: i32, cols: i32, c: &StencilBits) -> Result<(), DevError> {
    let n = count(height, cols)?;
    if stride <= 0 {
        return Err(unsup("stride must be positive"));
    }
    launch(s, 
        "flow_b_deriv_y",
        n as usize,
        &[inp.arg(), out.arg(), ClArg::I32(height), ClArg::I32(stride), ClArg::I32(col0), ClArg::I32(cols), ClArg::U32(c.k_s), ClArg::U32(c.c_s), ClArg::U32(c.k_v), ClArg::U32(c.c_v), ClArg::U32(n)],
    )
}

#[allow(clippy::too_many_arguments)]
pub fn warp_fetch_launch(s: &Arc<Session>, dx: &Buffer<f32>, dy: &Buffer<f32>, src: &Buffer<f32>, out: &Buffer<f32>, width: i32, height: i32, stride: i32, row0: i32, rows: i32, lo_bits: u32, margin_bits: u32) -> Result<(), DevError> {
    let n = count(width, rows)?;
    if stride <= 0 || height <= 0 {
        return Err(unsup("stride and height must be positive"));
    }
    launch(s, 
        "flow_b_warp_fetch",
        n as usize,
        &[dx.arg(), dy.arg(), src.arg(), out.arg(), ClArg::I32(width), ClArg::I32(height), ClArg::I32(stride), ClArg::I32(row0), ClArg::I32(rows), ClArg::U32(lo_bits), ClArg::U32(margin_bits), ClArg::U32(n)],
    )
}

/// `scale_copy_model` over `n` elements (the planes are read as words, so a copy keeps every payload).
pub fn scale_copy_launch(s: &Arc<Session>, src: &Buffer<f32>, dst: &Buffer<f32>, k_bits: u32, n: usize) -> Result<(), DevError> {
    let n32 = u32::try_from(n).map_err(|_| unsup("element count outside 32 bits"))?;
    if n == 0 {
        return Ok(());
    }
    launch(s, "flow_b_scale_copy", n, &[src.arg(), dst.arg(), ClArg::U32(k_bits), ClArg::U32(n32)])
}

#[allow(clippy::too_many_arguments)]
pub fn affine_rows_launch(s: &Arc<Session>, data: &Buffer<f32>, width: i32, stride: i32, row0: i32, rows: i32, scale_bits: u32, offset_bits: u32) -> Result<(), DevError> {
    let n = count(width, rows)?;
    if stride <= 0 {
        return Err(unsup("stride must be positive"));
    }
    launch(s, "flow_b_affine_rows", n as usize, &[data.arg(), ClArg::I32(width), ClArg::I32(stride), ClArg::I32(row0), ClArg::I32(rows), ClArg::U32(scale_bits), ClArg::U32(offset_bits), ClArg::U32(n)])
}

#[allow(clippy::too_many_arguments)]
pub fn sub_scaled_rows_launch(s: &Arc<Session>, a: &Buffer<f32>, b: &Buffer<f32>, out: &Buffer<f32>, width: i32, stride: i32, row0: i32, rows: i32, s_bits: u32) -> Result<(), DevError> {
    let n = count(width, rows)?;
    if stride <= 0 {
        return Err(unsup("stride must be positive"));
    }
    launch(s, "flow_b_sub_scaled_rows", n as usize, &[a.arg(), b.arg(), out.arg(), ClArg::I32(width), ClArg::I32(stride), ClArg::I32(row0), ClArg::I32(rows), ClArg::U32(s_bits), ClArg::U32(n)])
}

#[allow(clippy::too_many_arguments)]
pub fn flow_stencil_rows_launch(
    s: &Arc<Session>,
    out: &Buffer<f32>,
    b: &Buffer<f32>,
    c: &Buffer<f32>,
    d: &Buffer<f32>,
    width: i32,
    rows: i32,
    pitch: i32,
    scale_bits: u32,
    row0: i32,
    cnt: i32,
    align_b: u32,
) -> Result<(), DevError> {
    let n = count(width, cnt)?;
    if pitch < width || rows <= 0 || row0 < 0 {
        return Err(unsup("pitch below the width, no rows, or a negative first row"));
    }
    launch(s, 
        "flow_b_flow_stencil_rows",
        n as usize,
        &[out.arg(), b.arg(), c.arg(), d.arg(), ClArg::I32(width), ClArg::I32(rows), ClArg::I32(pitch), ClArg::U32(scale_bits), ClArg::I32(row0), ClArg::I32(cnt), ClArg::U32(align_b), ClArg::U32(n)],
    )
}

/// One work item over `rows` rows; the partial sum word lands in the one element buffer `dst`. See `sum_rows_value`.
#[allow(clippy::too_many_arguments)]
pub fn sum_rows_launch(s: &Arc<Session>, data: &Buffer<f32>, dst: &Buffer<f32>, width: i32, stride: i32, row0: i32, rows: i32, align0: u32) -> Result<(), DevError> {
    if stride <= 0 || rows < 0 {
        return Err(unsup("stride must be positive and rows not negative"));
    }
    launch(s, "flow_b_sum_rows", 1, &[data.arg(), dst.arg(), ClArg::I32(width), ClArg::I32(stride), ClArg::I32(row0), ClArg::I32(rows), ClArg::U32(align0)])
}

/// One work item; `dst` holds (max, min).
pub fn scan_range_launch(s: &Arc<Session>, pl: &Buffer<f32>, dst: &Buffer<f32>, w: i32, h: i32, pitch: i32) -> Result<(), DevError> {
    if pitch <= 0 {
        return Err(unsup("pitch must be positive"));
    }
    launch(s, "flow_b_scan_range", 1, &[pl.arg(), dst.arg(), ClArg::I32(w), ClArg::I32(h), ClArg::I32(pitch)])
}

/// The planes of the entry's final warp (device buffers), the CPU function's `WarpPlanes`.
pub struct FinalWarpPlanes<'a> {
    pub target: &'a Buffer<f32>,
    pub img: &'a Buffer<f32>,
    pub u: &'a Buffer<f32>,
    pub v: &'a Buffer<f32>,
    pub grad_a_x: &'a Buffer<f32>,
    pub grad_a_y: &'a Buffer<f32>,
    pub grad_b_x: &'a Buffer<f32>,
    pub grad_b_y: &'a Buffer<f32>,
}

#[allow(clippy::too_many_arguments)]
pub fn final_warp_launch(s: &Arc<Session>, p: &FinalWarpPlanes, out: &Buffer<f32>, width: i32, height: i32, stride: i32, row_start: i32, row_count: i32, scale_bits: u32) -> Result<(), DevError> {
    let n = count(width, row_count)?;
    if stride <= 0 || height <= 0 {
        return Err(unsup("stride and height must be positive"));
    }
    launch(s, 
        "flow_b_final_warp",
        n as usize,
        &[
            p.target.arg(), p.img.arg(), p.u.arg(), p.v.arg(), p.grad_a_x.arg(), p.grad_a_y.arg(), p.grad_b_x.arg(), p.grad_b_y.arg(), out.arg(),
            ClArg::I32(width), ClArg::I32(height), ClArg::I32(stride), ClArg::I32(row_start), ClArg::I32(row_count), ClArg::U32(scale_bits),
            ClArg::U32(optical_flow_entry::EDGE_EPS.to_bits()), ClArg::U32(optical_flow_entry::EDGE_MARGIN.to_bits()), ClArg::U32(n),
        ],
    )
}

const ONE: f32 = 1.0;
const TWO: f32 = 2.0;
const NEG_TWO: f32 = -2.0;

/// The window words of one resample pass from `n_src` to `n_dst` elements: per output index `(lo, hi, start_t bits)`, then the
/// scale bits and the scale-down flag. The arithmetic is the crate's `setup` and `window` with the running sums of the loops.
pub fn resample_table(n_src: i32, n_dst: i32) -> (Vec<u32>, u32, bool) {
    let a = (n_src.wrapping_sub(1)) as f32;
    let b = (n_dst.wrapping_sub(1)) as f32;
    let step = a.fdiv(b);
    let down = n_src > n_dst;
    let (scale, mut lo_pos, mut hi_pos) = if down {
        let x = step.fmul(TWO);
        (ONE.fdiv(step), f32::from_bits(x.to_bits() ^ 0x8000_0000), step.fadd(step))
    } else {
        (ONE, NEG_TWO, TWO)
    };
    let mut centre = 0.0f32;
    let mut t = Vec::with_capacity(3 * n_dst.max(0) as usize);
    for _ in 0..n_dst.max(0) {
        let r = lo_pos.ceil();
        let e = cvtss2si32_rne(r);
        let lo = if e <= 0 { 0 } else { e };
        let h = hi_pos.floor();
        let mut hi = cvtss2si32_rne(h).wrapping_add(1);
        if hi >= n_src {
            hi = n_src;
        }
        let start_t = scale.fmul((lo as f32).fsub(centre));
        t.push(lo as u32);
        t.push(hi as u32);
        t.push(start_t.to_bits());
        lo_pos = lo_pos.fadd(step);
        hi_pos = step.fadd(hi_pos);
        centre = step.fadd(centre);
    }
    (t, scale.to_bits(), down)
}

/// Horizontal pass over rows `row_start .. row_start + row_count`: `src` rows of width `w`, `dst` rows of width `wd`.
#[allow(clippy::too_many_arguments)]
pub fn resample_h_launch(s: &Arc<Session>, src: &Buffer<f32>, dst: &Buffer<f32>, w: i32, wd: i32, row_start: i32, row_count: i32, src_misalign: u32) -> Result<(), DevError> {
    let n = count(wd, row_count)?;
    if w <= 0 {
        return Err(unsup("non positive source width"));
    }
    let (t, scale, down) = resample_table(w, wd);
    let tb = s.input(&t[..])?;
    launch(s, 
        "flow_b_resample_h",
        n as usize,
        &[src.arg(), dst.arg(), tb.arg(), ClArg::I32(w), ClArg::I32(wd), ClArg::I32(row_start), ClArg::I32(row_count), ClArg::U32(src_misalign), ClArg::I32(down as i32), ClArg::U32(scale), ClArg::U32(n)],
    )
}

/// Vertical pass over columns `col_start .. col_start + col_count` of a `stride` wide plane: `hs` rows into `hd` rows.
#[allow(clippy::too_many_arguments)]
pub fn resample_v_launch(s: &Arc<Session>, src: &Buffer<f32>, dst: &Buffer<f32>, stride: i32, hs: i32, hd: i32, col_start: i32, col_count: i32) -> Result<(), DevError> {
    let n = count(hd, col_count)?;
    if hs <= 0 || stride <= 0 {
        return Err(unsup("non positive source height or stride"));
    }
    let (t, scale, down) = resample_table(hs, hd);
    let tb = s.input(&t[..])?;
    launch(s, 
        "flow_b_resample_v",
        n as usize,
        &[src.arg(), dst.arg(), tb.arg(), ClArg::I32(stride), ClArg::I32(hs), ClArg::I32(hd), ClArg::I32(col_start), ClArg::I32(col_count), ClArg::I32(down as i32), ClArg::U32(scale), ClArg::U32(n)],
    )
}

/// `sum_rows` returning the f32 partial sum (one work item, one word read back). The plane may be longer than `stride * rows`.
#[allow(clippy::too_many_arguments)]
pub fn sum_rows_value(s: &Arc<Session>, data: &Buffer<f32>, width: i32, stride: i32, row0: i32, rows: i32, align0: u32) -> Result<f32, DevError> {
    let out = fx_device::DevImage::alloc(s, 1, 1)?;
    sum_rows_launch(s, data, out.plane(0), width, stride, row0, rows, align0)?;
    Ok(out.download()?[0][0])
}
