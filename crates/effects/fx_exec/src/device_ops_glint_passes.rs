//! Device launch helpers of the glint row and column box passes: `glint_pass_rows::run` and `glint_pass_cols::pass_b::run_columns`.
//!
//! Both passes run one work item per line (a row, or a column) in the kernel `glint_lines_f32` of `fx_device::kernels_glint_passes`.
//! The host computes what the CPU functions compute before their loops: the product scale (`glint_pass_rows::radii_product`, the
//! same text as the column crate's scale), the active radii (radius <= 0 skipped), the parity of the two scratch buffers from line
//! to line, and the refusals. A configuration the crate refuses returns the crate's own `Error` value before any launch.

use fx_device::cl::Arg as ClArg;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

/// The refusals shared by both crates, in the order the CPU reports them.
enum Refusal {
    BadEdge(i32),
    RadiusTooLarge { radius: i32, width: usize },
    BadShift,
}

struct Plan {
    active: Vec<i32>,
    scale_bits: u32,
    flip: u32,
    shift: u32,
}

/// The checks of `run` after the buffer sizes (shift, then the first line's passes in radii order) and the host scalars.
fn plan(edge: i32, radii: &[i32], shift: i32, radii_align: u8, len: usize) -> Result<Plan, Refusal> {
    if shift == i32::MIN {
        return Err(Refusal::BadShift);
    }
    let mut active = Vec::new();
    for &r in radii {
        if r <= 0 {
            continue;
        }
        if edge != 0 && edge != 1 {
            return Err(Refusal::BadEdge(edge));
        }
        if len < 2 * (r as usize) + 1 {
            return Err(Refusal::RadiusTooLarge { radius: r, width: len });
        }
        active.push(r);
    }
    let scale = (1.0f64 / glint_pass_rows::radii_product(radii, radii_align)) as f32;
    let m = shift.unsigned_abs() as usize;
    // the all-zero blend is the same for every m >= len
    let shift = m.min(len) as u32;
    let flip = ((active.len() as u32) & 1) ^ u32::from(shift != 0);
    Ok(Plan { active, scale_bits: scale.to_bits(), flip, shift })
}

#[allow(clippy::too_many_arguments)]
fn launch(
    s: &Arc<Session>,
    src: &fx_device::cl::Buffer<f32>,
    dst: &fx_device::cl::Buffer<f32>,
    p: &Plan,
    edge: i32,
    scratch_phase: [u8; 2],
    len: usize,
    lines: usize,
    st: usize,
    lstep: usize,
) -> Result<(), DevError> {
    let scr = DevImage::alloc(s, 1, 2 * len * lines)?;
    let mut list = p.active.clone();
    if list.is_empty() {
        list.push(0);
    }
    let radii = s.input(&list)?;
    s.launch(
        "glint_lines_f32",
        lines,
        &[
            src.arg(),
            dst.arg(),
            scr.plane(0).arg(),
            radii.arg(),
            ClArg::U32(p.active.len() as u32),
            ClArg::U32(len as u32),
            ClArg::U32(lines as u32),
            ClArg::U32(st as u32),
            ClArg::U32(lstep as u32),
            ClArg::U32(edge as u32),
            ClArg::U32(p.shift),
            ClArg::U32(p.scale_bits),
            ClArg::U32(p.flip),
            ClArg::U32(u32::from(scratch_phase[0] & 31)),
            ClArg::U32(u32::from(scratch_phase[1] & 31)),
        ],
    )?;
    Ok(())
}

/// `glint_pass_rows::run` on the device: `rows` rows of `width` samples from `src` into `dst` (both row-major, dense).
/// The outer `Err` is a device problem; the inner `Err` is the crate's own refusal, returned before any launch.
pub fn glint_rows_dev(
    s: &Arc<Session>,
    cfg: &glint_pass_rows::Config,
    ph: &glint_pass_rows::Alignment,
    src: &fx_device::cl::Buffer<f32>,
    dst: &fx_device::cl::Buffer<f32>,
    width: usize,
    rows: usize,
) -> Result<Result<(), glint_pass_rows::Error>, DevError> {
    use glint_pass_rows::Error;
    if width == 0 || rows == 0 {
        return Ok(Ok(()));
    }
    if src.len() < width * rows || dst.len() < width * rows {
        return Ok(Err(Error::ShortBuffer));
    }
    let p = match plan(cfg.edge, cfg.radii, cfg.shift, ph.radii, width) {
        Ok(p) => p,
        Err(Refusal::BadEdge(e)) => return Ok(Err(Error::BadEdge(e))),
        Err(Refusal::RadiusTooLarge { radius, width }) => return Ok(Err(Error::RadiusTooLarge { radius, width })),
        Err(Refusal::BadShift) => return Ok(Err(Error::BadShift)),
    };
    launch(s, src, dst, &p, cfg.edge, ph.scratch, width, rows, 1, width)?;
    Ok(Ok(()))
}

/// `glint_pass_cols::pass_b::run_columns` on the device: line j is the column j (element stride `width`).
pub fn glint_cols_dev(
    s: &Arc<Session>,
    cfg: &glint_pass_cols::pass_b::Config,
    ph: &glint_pass_cols::pass_b::Alignment,
    src: &fx_device::cl::Buffer<f32>,
    dst: &fx_device::cl::Buffer<f32>,
    width: usize,
    height: usize,
) -> Result<Result<(), glint_pass_cols::pass_b::Error>, DevError> {
    use glint_pass_cols::pass_b::Error;
    if width == 0 || height == 0 {
        return Ok(Ok(()));
    }
    if src.len() < width * height || dst.len() < width * height {
        return Ok(Err(Error::ShortBuffer));
    }
    let p = match plan(cfg.edge, cfg.radii, cfg.shift, ph.radii, height) {
        Ok(p) => p,
        Err(Refusal::BadEdge(e)) => return Ok(Err(Error::BadEdge(e))),
        Err(Refusal::RadiusTooLarge { radius, width }) => return Ok(Err(Error::RadiusTooLarge { radius, width })),
        Err(Refusal::BadShift) => return Ok(Err(Error::BadShift)),
    };
    launch(s, src, dst, &p, cfg.edge, ph.scratch, height, width, width, 1)?;
    Ok(Ok(()))
}
