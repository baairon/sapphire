//! Launch function of the optical flow polar kernel (`fx_device::kernels_flow_polar`), the device form of
//! `optical_flow_solve::fetch::polar`. The crate constants of the call (`PolarConsts`) travel as bits; the constants and tables of the
//! power and exponential lanes are uploaded from the crate (`device_ops_flow_solve_b_gen`) exactly as `vmath_dev` does.
//!
//! Refusals (`DevError::Unsupported`, before any launch): a non positive width, a stride of zero or below, a row range that would
//! reach outside a plane buffer (the CPU function would panic), an element count outside 32 bits. A non positive row count writes
//! nothing, as on the CPU.

use fx_device::ocl::{Arg as ClArg, Buffer};
use fx_device::{DevError, Session};
use optical_flow_solve::PolarConsts;
use std::sync::Arc;

fn unsup(m: &str) -> DevError {
    DevError::Unsupported(format!("flow polar: {m}"))
}

#[allow(clippy::too_many_arguments)]
pub fn polar_launch(
    s: &Arc<Session>,
    p: &Buffer<f32>,
    q: &Buffer<f32>,
    out: &Buffer<f32>,
    width: i32,
    stride: i32,
    row0: i32,
    rows: i32,
    phase: u32,
    c: &PolarConsts,
) -> Result<(), DevError> {
    if rows <= 0 {
        return Ok(());
    }
    if width <= 0 {
        return Ok(());
    }
    if stride <= 0 {
        return Err(unsup("stride must be positive"));
    }
    let n = (width as i64) * (rows as i64);
    let n = u32::try_from(n).map_err(|_| unsup("element count outside 32 bits"))?;
    // every index the crate function touches: rows row0 .. row0 + rows - 1, columns 0 .. width - 1; the CPU panics outside a plane
    let need = (row0 as i64 + rows as i64 - 1) * stride as i64 + width as i64;
    if row0 < 0 || need > i32::MAX as i64 {
        return Err(unsup("row range outside the plane"));
    }
    let need = need as usize;
    if p.len() < need || q.len() < need || out.len() < need {
        return Err(unsup("a short plane"));
    }
    let kc = s.input(&crate::device_ops_flow_solve_b_gen::kconsts())?;
    let tab = s.input(&crate::device_ops_flow_solve_b_gen::tables())?;
    s.launch(
        "flow_c_polar",
        n as usize,
        &[
            p.arg(),
            q.arg(),
            out.arg(),
            kc.arg(),
            tab.arg(),
            ClArg::I32(width),
            ClArg::I32(stride),
            ClArg::I32(row0),
            ClArg::I32(rows),
            ClArg::U32(phase),
            ClArg::U32(c.arg2.to_bits()),
            ClArg::U32(c.mul_s.to_bits()),
            ClArg::U32(c.mul_v.to_bits()),
            ClArg::U32(c.abs_mask),
            ClArg::U32(c.sub.to_bits()),
            ClArg::U32(c.fin.to_bits()),
            ClArg::U32(c.thr.to_bits()),
            ClArg::U32(n),
        ],
    )?;
    Ok(())
}
