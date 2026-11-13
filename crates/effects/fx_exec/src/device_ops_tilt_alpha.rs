//! Device op `tilt_alpha` (the S_WarpTransform alpha ramp): a port of `kernels2::tilt_alpha_k` onto `tilt_alpha::tilt_alpha`.
//!
//! `tilt_alpha_k` refuses a non-constant operand 1 or 2, an operand 0 that is not a 2 channel plane pair of the frame size and a
//! missing `palpha_cells` host value; `accepts` refuses the same shapes, so the CPU path produces the same named error. The host branch
//! (`Route::HostBranch`) and the flag route are value level refusals of `run` (Unsupported), so the CPU kernel gives the named error.
//!
//! The scalar sections of the crate (S1 to S5) run on the host through the crate itself on one element; the per row phase comes from
//! `kernels2::rows_of_p` (the same host input as the CPU kernel) through the crate's `plan`, and the kernel `tilt_alpha_f32` in
//! `fx_device::kernels_tilt_alpha` takes the scalar form (S8) or the vector form (S9) per element from the `(peel, vend)` words.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::types::{Image, KParams};
use fx_common::nanop::NanOps;
use fx_common::x86::flip_sign;
use fx_device::cl::{Arg as ClArg, Buffer};
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

fn unsup(m: &str) -> DevError {
    DevError::Unsupported(m.into())
}

pub const TILT_ALPHA: DeviceOp = DeviceOp { class: "tilt_alpha", accepts: tilt_alpha_accepts, run: tilt_alpha_run, corpus: tilt_alpha_corpus };

pub fn tilt_alpha_accepts(c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    if shapes.len() != 3 {
        return Err("unexpected operand count");
    }
    if shapes[0] != (ArgShape::Img { channels: 2 }) {
        return Err("operand 0 must be a 2 channel plane pair");
    }
    if shapes[1..].iter().any(|s| !matches!(s, ArgShape::Const { .. })) {
        return Err("non-constant parameter operand at run time");
    }
    if c.kp.palpha_cells.is_none() {
        return Err("the host record fields are not measured: palpha_cells is not given");
    }
    if c.n == 0 || c.n > i32::MAX as usize {
        return Err("sample count outside the kernel's range");
    }
    Ok(())
}

/// The eight parameter words of the kernel from the crate's own scalar sections, or the refusal text of the route the crate reports.
pub fn host_words(st: &tilt_alpha::State, host: &tilt_alpha::Host) -> Result<[u32; 8], &'static str> {
    let mut probe = [0.0f32; 1];
    let o = tilt_alpha::tilt_alpha(st, host, false, 1, &[0.0], &[0.0], &mut probe, 0).map_err(|_| "short buffer")?;
    match o.route {
        tilt_alpha::Route::HostBranch => return Err("HOST-BRANCH-REFUSED (|cos(angle) / amount| < 1e-4 takes the ROI helper and host dispatch, not rebuilt)"),
        tilt_alpha::Route::Flag => return Err("the flag path is not rebuilt"),
        _ => {}
    }
    let sinpa = o.sin_ang.fmul(st.amount);
    let a = sinpa.fmul(o.ls);
    let b = o.cos_ang.fmul(o.ls);
    Ok([o.ls.to_bits(), o.knt.to_bits(), o.ksw.to_bits(), o.cos_ang.to_bits(), a.to_bits(), b.to_bits(), flip_sign(o.ksw).to_bits(), flip_sign(sinpa).to_bits()])
}

/// One launch of `tilt_alpha_f32` over the rows `(lo, cnt, ph)`. `stub` replaces every plan by the all scalar plan (the harness's
/// deliberately wrong kernel: it always takes the scalar form).
pub fn tilt_alpha_launch(
    s: &Arc<Session>,
    a: &Buffer<f32>,
    b: &Buffer<f32>,
    dst: &Buffer<f32>,
    words: &[u32; 8],
    rows: &[(usize, usize, usize)],
    n: usize,
    stub: bool,
) -> Result<(), DevError> {
    let rowlen = rows.first().map(|r| r.1).unwrap_or(0);
    if rowlen == 0 || rows.iter().any(|r| r.1 != rowlen) || rows.len() * rowlen != n || rows.iter().enumerate().any(|(k, r)| r.0 != k * rowlen) {
        return Err(unsup("rows are not equal contiguous runs"));
    }
    let mut table = Vec::with_capacity(rows.len() * 2);
    for &(_, cnt, ph) in rows {
        let p = if stub { None } else { tilt_alpha::plan(cnt as i32, ph) };
        let p = p.unwrap_or(tilt_alpha::Plan { peel: 0, vend: 0 });
        table.push(p.peel as u32);
        table.push(p.vend as u32);
    }
    let pr = s.input(&words[..])?;
    let tb = s.input(&table[..])?;
    s.launch("tilt_alpha_f32", n, &[a.arg(), b.arg(), dst.arg(), pr.arg(), tb.arg(), ClArg::U32(n as u32), ClArg::U32(rowlen as u32)])?;
    Ok(())
}

pub fn tilt_alpha_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    if args.len() != 3 {
        return Err(unsup("three operands expected"));
    }
    let (w, h) = c.dims;
    let DevArg::Img(uv) = &args[0] else { return Err(unsup("operand 0 is not an image")) };
    if uv.channels() != 2 || uv.n != c.n || w * h != c.n || c.n == 0 || w == 0 || c.n > i32::MAX as usize {
        return Err(unsup("operand 0 must be a 2 channel image of the frame size"));
    }
    let lane = |a: &DevArg, i: usize| -> Result<f32, DevError> {
        match a {
            DevArg::Const { lanes, .. } => Ok(f32::from_bits(lanes[i.min(3)])),
            _ => Err(unsup("non-constant parameter operand")),
        }
    };
    let st = tilt_alpha::State { swivel: lane(&args[1], 0)?, tilt: lane(&args[1], 1)?, amount: lane(&args[2], 0)? };
    let Some((hw, hh, hd)) = c.kp.palpha_cells else { return Err(unsup("palpha_cells is not given")) };
    let host = tilt_alpha::Host { w: hw, d: hd, h: hh };
    let words = host_words(&st, &host).map_err(unsup)?;
    let rows = crate::kernels2::rows_of_p(w, h, if c.kp.palpha_base >= 0 { c.kp.palpha_base as usize } else { 0 }, c.kp.palpha_rowbuf);
    let out = DevImage::alloc(s, 1, c.n)?;
    tilt_alpha_launch(s, uv.plane(0), uv.plane(1), out.plane(0), &words, &rows, c.n, false)?;
    Ok(out)
}

fn cst(v: &[f32]) -> (usize, [u32; 4]) {
    let mut l = [0u32; 4];
    for (i, x) in v.iter().enumerate().take(4) {
        l[i] = x.to_bits();
    }
    (v.len(), l)
}

/// (swivel, tilt, amount, cells) sets of the self-check; none takes the host branch.
fn sets() -> Vec<([f32; 2], f32, (i32, i32, f64))> {
    vec![
        ([15.0, 20.0], 1.0, (64, 1, 1.0)),
        ([20.0, 35.0], 1.5, (640, 480, 2.0)),
        ([-30.0, 10.0], 0.75, (1920, 1080, 1.0)),
        ([0.0, 0.0], 1.0, (64, 1, 1.0)),
        ([0.0, 60.0], 3.0, (100, 7, 0.5)),
        ([90.0, -45.0], -1.0, (64, 1, 1.0)),
        ([-0.0, 0.0], 2.0, (33, 3, 1.5)),
        ([1.0e-3, 2.0e-3], 0.25, (64, 1, 1.0)),
        ([f32::NAN, 20.0], 1.0, (64, 1, 1.0)),
        ([15.0, f32::NAN], 1.5, (640, 480, 2.0)),
    ]
}

fn tilt_alpha_corpus() -> Vec<SelfCase> {
    let pool = adversarial_f32(300, 4242);
    let mut out = Vec::new();
    let mut k = 0usize;
    for (si, (sw_t, amount, cells)) in sets().into_iter().enumerate() {
        for &(w, h) in &[(1usize, 2usize), (7, 3), (8, 2), (9, 2), (15, 2), (16, 3), (1199, 2), (1200, 3), (1201, 3), (1207, 2)] {
            let n = w * h;
            for (base, rowbuf) in [(0i32, false), (4, false), (12, true), (28, false), (20, true)] {
                k += 1;
                if (k + si) % 3 == 2 {
                    continue;
                }
                let pick = |q: usize, i: usize| -> f32 {
                    match (si + q + i) % 5 {
                        0 => pool[(i * 7 + q * 13 + k) % pool.len()],
                        1 => ((i as f32) * 0.003 - 1.0) * (1 + q) as f32,
                        _ => (((i * 29 + k * 17) % 61) as f32 - 30.0) * 0.1,
                    }
                };
                let x: Vec<f32> = (0..n).map(|i| pick(0, i)).collect();
                let y: Vec<f32> = (0..n).map(|i| pick(1, i)).collect();
                let mut kp = KParams::standin();
                kp.palpha_base = base;
                kp.palpha_rowbuf = rowbuf;
                kp.palpha_cells = Some(cells);
                out.push(SelfCase {
                    name: format!("tilt_alpha set{si} {w}x{h} base{base} rowbuf{rowbuf}"),
                    dims: (w, h),
                    tag: 1,
                    args: vec![Image { n, planes: vec![x, y] }],
                    consts: vec![(1, 2, cst(&sw_t).1), (2, 1, cst(&[amount]).1)],
                    kp: Some(kp),
                });
            }
        }
    }
    out
}
