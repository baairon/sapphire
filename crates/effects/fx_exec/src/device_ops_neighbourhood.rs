//! Device kernels of the neighbourhood ops: `blur`, `masked_blur`, `grad`, `sharpen`, `edge_aware_sharpen`, `static2`.
//!
//! Every op follows its CPU function (`kernels2::*_k`) with the same host arithmetic (driver radii, coefficients, final scale, the
//! edge aware driver's radii and plane bookkeeping come from the CPU crates themselves) and runs the per sample or per row and
//! column work in the kernels of `fx_device::kernels_neighbourhood`, in the CPU's operation order. A call the kernels do not cover (a ripmap
//! or constant where a plane is read, a width the CPU refuses, an error the CPU would report, a driver state with neither pass)
//! returns `DevError::Unsupported` and the executor runs that node on the CPU with the same arguments.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::kernels2::cvt_i32;
use crate::types::{Image, KParams};
use blur_coef::{DriverParams, Plan};
use fx_device::cl::{Arg as ClArg, Buffer};
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use fx_common::nanop::NanOps;
use std::sync::Arc;

pub const BLUR: DeviceOp = DeviceOp { class: "blur", accepts: blur_accepts, run: blur_run, corpus: blur_corpus };
pub const MASKED_BLUR: DeviceOp = DeviceOp { class: "masked_blur", accepts: masked_blur_accepts, run: masked_blur_run, corpus: masked_blur_corpus };
pub const GRAD: DeviceOp = DeviceOp { class: "grad", accepts: grad_accepts, run: grad_run, corpus: grad_corpus };
pub const SHARPEN: DeviceOp = DeviceOp { class: "sharpen", accepts: sharpen_accepts, run: sharpen_run, corpus: sharpen_corpus };
pub const EDGE_AWARE_SHARPEN: DeviceOp = DeviceOp { class: "edge_aware_sharpen", accepts: esh_accepts, run: esh_run, corpus: esh_corpus };
pub const STATIC2: DeviceOp = DeviceOp { class: "static2", accepts: static2_accepts, run: static2_run, corpus: static2_corpus };

/// The classes of this module that may run on a device.
pub const ALLOW: &[&str] = &["blur", "masked_blur", "grad", "sharpen", "edge_aware_sharpen", "static2"];

fn unsup<T>(why: &str) -> Result<T, DevError> {
    Err(DevError::Unsupported(why.to_string()))
}

/// A plane operand: the resident image, or a constant operand laid out as the CPU's `Image::from_const` does (one plane per lane in use,
/// at least one, every sample the lane's bits).
pub(crate) enum Operand<'a> {
    Resident(&'a DevImage),
    Filled(DevImage),
}

impl Operand<'_> {
    pub(crate) fn get(&self) -> &DevImage {
        match self {
            Operand::Resident(i) => i,
            Operand::Filled(i) => i,
        }
    }
}

pub(crate) fn img<'a>(s: &Arc<Session>, a: &'a DevArg, n: usize) -> Result<Operand<'a>, DevError> {
    match a {
        DevArg::Img(i) => Ok(Operand::Resident(i)),
        DevArg::Const { n: ln, lanes } => {
            let d = DevImage::alloc(s, (*ln).max(1), n)?;
            for c in 0..d.channels() {
                s.queue.fill(d.plane(c), f32::from_bits(lanes[c.min(3)]))?;
            }
            Ok(Operand::Filled(d))
        }
        DevArg::Mip(_) | DevArg::DevMip(_) => unsup("a ripmap where a plane operand is read"),
    }
}

/// Channels of a plane operand as the CPU's `image_or_const` gives them (a ripmap is not one).
pub(crate) fn plane_channels(s: &ArgShape) -> Option<usize> {
    match s {
        ArgShape::Img { channels } => Some(*channels),
        ArgShape::Const { n } => Some((*n).max(1)),
        ArgShape::Mip => None,
    }
}

/// Lane `i` (clamped to 3 like the CPU's `const_lane`) of a constant operand.
pub(crate) fn lane(a: &DevArg, i: usize) -> Result<f32, DevError> {
    match a {
        DevArg::Const { lanes, .. } => Ok(f32::from_bits(lanes[i.min(3)])),
        DevArg::Img(_) | DevArg::Mip(_) | DevArg::DevMip(_) => unsup("an image where a constant is read"),
    }
}

pub(crate) fn const_n(a: &DevArg) -> usize {
    match a {
        DevArg::Const { n, .. } => *n,
        DevArg::Img(_) | DevArg::Mip(_) | DevArg::DevMip(_) => 0,
    }
}

pub(crate) fn is_const(s: &ArgShape) -> bool {
    matches!(s, ArgShape::Const { .. })
}

pub(crate) fn frame_ok(c: &NodeCall) -> Result<(), &'static str> {
    if c.n == 0 || c.dims.0 == 0 || c.dims.1 == 0 || c.dims.0.checked_mul(c.dims.1) != Some(c.n) {
        return Err("the frame size does not match the sample count");
    }
    if c.n > i32::MAX as usize {
        return Err("more samples than an i32 index");
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------------------------
// blur core on the device

#[derive(Clone, Copy)]
pub(crate) struct BlurJob {
    pub(crate) w: usize,
    pub(crate) h: usize,
    pub(crate) sx: f32,
    pub(crate) sy: f32,
    pub(crate) gain: f32,
    pub(crate) edge: i32,
    pub(crate) passes: i32,
    pub(crate) subpixel: bool,
    pub(crate) aspect: Option<f32>,
}

/// The scratch alignment replay of the CPU's vertical pass (`blur_rows_y::pass_y`): for every column and pass, the byte phase of the
/// input buffer slot (32 column slots, a ping-pong slot, 32 columns per strip).
fn y_phases(w: usize, h: usize, np: usize, base_phase: usize) -> Vec<u32> {
    let stride_bytes = (4 * h + 15) & !15;
    let phase = |idx: usize| ((base_phase + idx * stride_bytes) & 31) as u32;
    let mut out = vec![0u32; w * 3];
    let mut slots: [usize; 32] = core::array::from_fn(|j| j);
    let mut pp = 32usize;
    let mut col0 = 0usize;
    while col0 < w {
        let ncols = (w - col0).min(32);
        let mut r15 = pp;
        for j in 0..ncols {
            let mut r14 = slots[j];
            for p in 0..np.min(3) {
                out[(col0 + j) * 3 + p] = phase(r14);
                std::mem::swap(&mut r14, &mut r15);
            }
            slots[j] = r14;
        }
        pp = r15;
        col0 += 32;
    }
    out
}

fn scale_or_copy_dev(s: &Arc<Session>, src: Option<&Buffer<f32>>, dst: &Buffer<f32>, scale: f32, n: usize) -> Result<(), DevError> {
    if scale == 1.0 {
        if let Some(sb) = src {
            s.queue.copy(sb, dst)?;
        }
        return Ok(());
    }
    if scale == 0.0 {
        s.queue.fill(dst, 0.0f32)?;
        return Ok(());
    }
    let sb = src.unwrap_or(dst);
    s.launch("scale_f32", n, &[sb.arg(), dst.arg(), ClArg::U32(n as u32), ClArg::F32(scale)])?;
    Ok(())
}

/// `blur_core` on one plane: `dst` receives the blur of `src`. `scr` is a three plane scratch image of the frame size.
pub(crate) fn blur_dev(s: &Arc<Session>, scr: &DevImage, src: &Buffer<f32>, dst: &Buffer<f32>, j: &BlurJob) -> Result<(), DevError> {
    let n = j.w * j.h;
    if j.w == 0 || j.h == 0 || n > i32::MAX as usize {
        return unsup("empty or oversized frame");
    }
    let p = DriverParams { sigma_x: j.sx, sigma_y: j.sy, gain: j.gain, passes: j.passes, subpixel: j.subpixel, half: false, aspect: j.aspect, width: j.w as i32, height: j.h as i32 };
    match blur_coef::plan(&p) {
        Plan::Clear => {
            s.queue.fill(dst, 0.0f32)?;
        }
        Plan::ScaleOrCopy { gain } => scale_or_copy_dev(s, Some(src), dst, gain, n)?,
        Plan::Run { x, y, scale } => {
            if x.is_none() && y.is_none() {
                return unsup("neither blur pass runs (NaN sigma): the CPU scales the destination as it is");
            }
            let sub = j.subpixel as i32;
            let args_of = |pl: &blur_coef::PassPlan| -> (i32, [i32; 3], [i32; 3], [f32; 3]) { (pl.npasses, pl.a, pl.b, pl.c) };
            if let Some(xp) = &x {
                let (np, a, b, c) = args_of(xp);
                for q in 0..np as usize {
                    let (ra, rb) = (a[q] as i64, b[q] as i64);
                    let wd = j.w as i64;
                    let blend = j.subpixel && ra != rb;
                    let ok = |r: i64| r == 0 || (wd > 1 && r >= 0 && r <= (wd >> 1) - 1);
                    if !ok(ra) || (blend && !ok(rb)) {
                        return unsup("a row radius outside what the CPU kernel accepts");
                    }
                }
                let stride = (4 * j.w + 15) & !15;
                let (ph0, ph1) = (0u32, ((stride) & 31) as u32);
                s.launch(
                    "blur_x_f32",
                    j.h,
                    &[
                        src.arg(),
                        dst.arg(),
                        scr.plane(0).arg(),
                        ClArg::U32(j.w as u32),
                        ClArg::U32(j.h as u32),
                        ClArg::I32(np),
                        ClArg::I32(j.edge),
                        ClArg::I32(sub),
                        ClArg::F32(xp.frac),
                        ClArg::I32(a[0]),
                        ClArg::I32(a[1]),
                        ClArg::I32(a[2]),
                        ClArg::I32(b[0]),
                        ClArg::I32(b[1]),
                        ClArg::I32(b[2]),
                        ClArg::F32(c[0]),
                        ClArg::F32(c[1]),
                        ClArg::F32(c[2]),
                        ClArg::U32(ph0),
                        ClArg::U32(ph1),
                    ],
                )?;
            }
            if let Some(yp) = &y {
                let (np, a, b, c) = args_of(yp);
                for q in 0..np as usize {
                    for r in [a[q], b[q]] {
                        if r < 0 || (r > 0 && 2 * (r as i64) + 2 > j.h as i64) {
                            return unsup("a column radius outside what the CPU kernel accepts");
                        }
                    }
                }
                let phs = y_phases(j.w, j.h, np as usize, 0);
                let phb = s.input(&phs)?;
                let from = if x.is_some() { dst } else { src };
                s.launch(
                    "blur_y_f32",
                    j.w,
                    &[
                        from.arg(),
                        dst.arg(),
                        scr.plane(0).arg(),
                        phb.arg(),
                        ClArg::U32(j.w as u32),
                        ClArg::U32(j.h as u32),
                        ClArg::I32(np),
                        ClArg::I32(j.edge),
                        ClArg::I32(sub),
                        ClArg::F32(yp.frac),
                        ClArg::I32(a[0]),
                        ClArg::I32(a[1]),
                        ClArg::I32(a[2]),
                        ClArg::I32(b[0]),
                        ClArg::I32(b[1]),
                        ClArg::I32(b[2]),
                        ClArg::F32(c[0]),
                        ClArg::F32(c[1]),
                        ClArg::F32(c[2]),
                    ],
                )?;
            }
            scale_or_copy_dev(s, None, dst, scale, n)?;
        }
    }
    Ok(())
}

pub(crate) fn render_scale(kp: &KParams, w: usize) -> i32 {
    if kp.blur_render_scale > 0 {
        kp.blur_render_scale
    } else {
        w as i32
    }
}

pub(crate) fn edge_of(v: i32) -> Result<i32, DevError> {
    match v {
        0 | 1 => Ok(v),
        _ => unsup("edge mode other than 0 and 1 (the CPU reports an error)"),
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------
// blur

fn blur_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 5 {
        return Err("five operands expected");
    }
    if !(1..=4).contains(&c.tag) {
        return Err("combination tag outside 1..4");
    }
    match plane_channels(&a[0]) {
        Some(ch) if ch >= c.tag as usize => {}
        Some(_) => return Err("fewer channels than the combination tag"),
        None => return Err("the source is not a plain image"),
    }
    if !a[1..].iter().all(is_const) {
        return Err("amount, filter shape, scale and subpixel must be constants");
    }
    Ok(())
}

fn blur_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    let src_op = img(s, &a[0], c.n)?;
    let src = src_op.get();
    let (w, h) = c.dims;
    let planes = c.tag.max(0) as usize;
    if src.n != w * h || src.channels() < planes {
        return unsup("source smaller than the combo tag or than the frame");
    }
    let amount = [lane(&a[1], 0)?, lane(&a[1], 1)?];
    let shape = cvt_i32(lane(&a[2], 0)?);
    let sc = [lane(&a[3], 0)?, lane(&a[3], 1)?, lane(&a[3], 2)?, lane(&a[3], 3)?];
    let subpixel = cvt_i32(lane(&a[4], 0)?);
    let bp = blur::BlurParams {
        amount,
        filter_shape: shape,
        scale: sc,
        scale_count: const_n(&a[3]) as i32,
        subpixel,
        edge_mode: c.kp.blur_edge,
        plane_count: planes,
        render_scale: render_scale(c.kp, w),
        aspect: c.kp.blur_aspect,
    };
    let edge = edge_of(bp.edge_mode)?;
    let passes = blur::pass_count(bp.filter_shape);
    let sx = blur::lane_sigma(bp.amount[0], bp.render_scale);
    let sy = blur::lane_sigma(bp.amount[1], bp.render_scale);
    let out = DevImage::alloc(s, planes, c.n)?;
    let scr = DevImage::alloc(s, 1, 3 * c.n)?;
    for ch in 0..planes {
        let job = BlurJob { w, h, sx, sy, gain: blur::plane_gain(&bp, ch), edge, passes, subpixel: bp.subpixel != 0, aspect: bp.aspect };
        blur_dev(s, &scr, src.plane(ch), out.plane(ch), &job)?;
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// masked_blur

fn masked_blur_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 5 {
        return Err("five operands expected");
    }
    if !(1..=4).contains(&c.tag) {
        return Err("combination tag outside 1..4");
    }
    match plane_channels(&a[0]) {
        Some(ch) if ch >= c.tag as usize => {}
        _ => return Err("the source is not an image with the tag's channels"),
    }
    match plane_channels(&a[1]) {
        Some(ch) if ch >= 1 => {}
        _ => return Err("the mask is not an image"),
    }
    if !a[2..].iter().all(is_const) {
        return Err("amount, filter shape and subpixel must be constants");
    }
    Ok(())
}

fn masked_blur_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    let src_op = img(s, &a[0], c.n)?;
    let src = src_op.get();
    let mask_op = img(s, &a[1], c.n)?;
    let mask = mask_op.get();
    let (w, h) = c.dims;
    let planes = c.tag.max(0) as usize;
    if src.n != w * h || mask.n != w * h || src.channels() < planes || mask.channels() < 1 {
        return unsup("source or mask smaller than the combo tag or than the frame");
    }
    let amount = [lane(&a[2], 0)?, lane(&a[2], 1)?];
    let shape = cvt_i32(lane(&a[3], 0)?);
    let subpixel = cvt_i32(lane(&a[4], 0)?);
    let rs = render_scale(c.kp, w);
    let sx = masked_blur::lane_sigma(amount[0], rs);
    let sy = masked_blur::lane_sigma(amount[1], rs);
    let passes = blur::pass_count(shape);
    let n = c.n;
    let work = DevImage::alloc(s, 3, n)?; // clamped mask, blurred mask, blurred premultiplied plane
    let scr = DevImage::alloc(s, 1, 3 * n)?;
    let out = DevImage::alloc(s, planes, n)?;
    let job = BlurJob { w, h, sx, sy, gain: 1.0, edge: 0, passes, subpixel: subpixel != 0, aspect: None };
    s.launch("mb_clamp_f32", n, &[mask.plane(0).arg(), work.plane(0).arg(), ClArg::U32(n as u32)])?;
    blur_dev(s, &scr, work.plane(0), work.plane(1), &job)?;
    for ch in 0..planes {
        // t = premultiplied source, blurred into plane 2 through the scratch of the blur (the CPU blurs a copy back into t)
        let t = DevImage::alloc(s, 1, n)?;
        s.launch("mb_premul_f32", n, &[work.plane(0).arg(), src.plane(ch).arg(), t.plane(0).arg(), ClArg::U32(n as u32)])?;
        blur_dev(s, &scr, t.plane(0), work.plane(2), &job)?;
        s.launch(
            "mb_combine_f32",
            n,
            &[work.plane(0).arg(), work.plane(2).arg(), work.plane(1).arg(), src.plane(ch).arg(), out.plane(ch).arg(), ClArg::U32(n as u32), ClArg::F32(masked_blur::MASK_THRESHOLD)],
        )?;
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// grad

fn grad_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 4 {
        return Err("four operands expected");
    }
    if c.tag != 2 {
        return Err("the selector tag is not 2");
    }
    if c.dims.0 < 2 {
        return Err("width 1: the CPU kernel refuses it");
    }
    match plane_channels(&a[0]) {
        Some(ch) if ch >= 1 => {}
        _ => return Err("the source is not an image"),
    }
    if !a[1..].iter().all(is_const) {
        return Err("amount, gain and subpixel must be constants");
    }
    Ok(())
}

fn grad_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    let src_op = img(s, &a[0], c.n)?;
    let src = src_op.get();
    let (w, h) = c.dims;
    if src.n != w * h || src.channels() < 1 {
        return unsup("source smaller than the frame");
    }
    let kp = c.kp;
    let a_eff = if crate::aspect::in_caller_range(kp.host_aspect) { kp.host_aspect } else { crate::aspect::FALLBACK_A };
    let Some(asp_blk) = crate::aspect::set_aspect(a_eff, 0.0) else { return unsup("aspect block") };
    let amount = lane(&a[1], 0)?;
    let gain_param = lane(&a[2], 0)?;
    let subpixel = cvt_i32(lane(&a[3], 0)?);
    let gp = grad::GradParams {
        amount,
        gain_param,
        subpixel,
        edge_mode: kp.blur_edge,
        render_scale: render_scale(kp, w),
        aspect: if asp_blk.aspect_block_b != 0 { Some(f32::from_bits(asp_blk.aspect_block_d)) } else { None },
        info_aspect_set: asp_blk.aspect_block_e != 0,
        flag_1: 1,
        past_end: 0.0,
    };
    if let Some(v) = kp.kparams_a {
        if v != (asp_blk.aspect_block_e != 0) {
            return unsup("the test interface info aspect differs from the derivation (the CPU reports an error)");
        }
    }
    let edge = edge_of(gp.edge_mode)?;
    let (sigma, gain) = grad::sigma_gain(&gp);
    let n = c.n;
    let aux = DevImage::alloc(s, 1, n)?;
    let scr = DevImage::alloc(s, 1, 3 * n)?;
    let out = DevImage::alloc(s, 2, n)?;
    let job = BlurJob { w, h, sx: sigma, sy: sigma, gain, edge, passes: 3, subpixel: gp.subpixel != 0, aspect: gp.aspect };
    blur_dev(s, &scr, src.plane(0), aux.plane(0), &job)?;
    let x5 = if gp.info_aspect_set { 1.0f32 } else { 2.0f32 };
    s.launch("grad_stencil_f32", n, &[aux.plane(0).arg(), out.plane(0).arg(), out.plane(1).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32), ClArg::F32(x5)])?;
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// sharpen

fn sharpen_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 3 {
        return Err("three operands expected");
    }
    if !(1..=4).contains(&c.tag) {
        return Err("combination tag outside 1..4");
    }
    if plane_channels(&a[0]) != Some(c.tag as usize) {
        return Err("the source is not an image with exactly the tag's channels");
    }
    if !is_const(&a[1]) || !is_const(&a[2]) {
        return Err("gains and amount must be constants");
    }
    if c.kp.sharpen_count.is_none() {
        return Err("the host element count is not given");
    }
    Ok(())
}

fn sharpen_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    let src_op = img(s, &a[0], c.n)?;
    let src = src_op.get();
    let (w, h) = c.dims;
    let planes = src.channels();
    if planes != c.tag.max(0) as usize || src.n != w * h {
        return unsup("source channels differ from the combo tag");
    }
    let gains = [lane(&a[1], 0)?, lane(&a[1], 1)?, lane(&a[1], 2)?, lane(&a[1], 3)?];
    let amount = lane(&a[2], 0)?;
    let Some(count) = c.kp.sharpen_count else { return unsup("the host element count is not given") };
    if count > w * h {
        return unsup("the element count exceeds the frame");
    }
    let ctx_width = render_scale(c.kp, w);
    let r = sharpen::plan::radius(amount, ctx_width);
    let n = c.n;
    let out = DevImage::alloc(s, planes, n)?;
    let scr = DevImage::alloc(s, 1, 3 * n)?;
    for ch in 0..planes {
        let idx = sharpen::plan::gain_index(ch, planes);
        let Some(g) = gains.get(idx).copied() else { return unsup("gain index outside the four lanes") };
        match sharpen::plan::plane_plan(r, g) {
            sharpen::plan::PlanePlan::Copy => s.queue.copy(src.plane(ch), out.plane(ch))?,
            sharpen::plan::PlanePlan::Run { radius, gain } => {
                let job = BlurJob { w, h, sx: radius, sy: radius, gain: sharpen::plan::BLUR_GAIN, edge: sharpen::plan::BLUR_EDGE_MIRROR, passes: sharpen::plan::BLUR_PASSES, subpixel: false, aspect: None };
                blur_dev(s, &scr, src.plane(ch), out.plane(ch), &job)?;
                s.launch(
                    "sharpen_combine_f32",
                    n,
                    &[src.plane(ch).arg(), out.plane(ch).arg(), ClArg::U32(count as u32), ClArg::U32(n as u32), ClArg::F32(gain), ClArg::I32(c.kp.sharpen_alias as i32)],
                )?;
            }
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// static2

fn static2_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if !a.is_empty() {
        return Err("no operands expected");
    }
    Ok(())
}

fn static2_run(s: &Arc<Session>, c: &NodeCall, _a: &[DevArg]) -> Result<DevImage, DevError> {
    let t = static2::table();
    let base = static2::seed_base(&static2::Params { n: c.n as i32, seed_a: c.kp.static2_seed_a, seed_b: 0 });
    let tab = s.input(&t.t)?;
    let out = DevImage::alloc(s, 2, c.n)?;
    s.launch("static2_f32", c.n, &[tab.arg(), out.plane(0).arg(), out.plane(1).arg(), ClArg::U32(c.n as u32), ClArg::U32(base)])?;
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// edge_aware_sharpen

fn esh_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 10 {
        return Err("ten operands expected");
    }
    if !(1..=3).contains(&c.tag) {
        return Err("combination tag outside 1..3");
    }
    if plane_channels(&a[0]) != Some(c.tag as usize) {
        return Err("the source is not an image with exactly the tag's channels");
    }
    match &a[1] {
        ArgShape::Img { channels: 1 } | ArgShape::Const { .. } => {}
        _ => return Err("the mask is neither a one channel image nor a constant"),
    }
    if !a[2..9].iter().all(is_const) {
        return Err("the parameter operands must be constants");
    }
    if a.iter().any(|s| matches!(s, ArgShape::Mip)) {
        return Err("a ripmap operand");
    }
    Ok(())
}

fn esh_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    use edge_aware_sharpen_jobs as j3;
    let src_op = img(s, &a[0], c.n)?;
    let src = src_op.get();
    let (w, h) = c.dims;
    let nu = src.channels();
    let n = c.n;
    if nu != c.tag.max(0) as usize || !(1..=3).contains(&nu) || src.n != n {
        return unsup("source channels differ from the combo tag");
    }
    let (mask_buf, sig_byte, sigma0): (Option<&Buffer<f32>>, bool, f32) = match &a[1] {
        DevArg::Img(m) => {
            if m.channels() != 1 || m.n != n {
                return unsup("mask shape");
            }
            (Some(m.plane(0)), false, 0.0)
        }
        DevArg::Const { .. } => (None, true, lane(&a[1], 0)?),
        DevArg::Mip(_) | DevArg::DevMip(_) => return unsup("a ripmap mask"),
    };
    let blur_type = lane(&a[2], 0)?;
    let deriv_io = [lane(&a[3], 0)?, lane(&a[3], 1)?];
    let levels = lane(&a[4], 0)?;
    let tiny = lane(&a[5], 0)?;
    let edge_threshold = lane(&a[6], 0)?;
    let sigmoid = [lane(&a[7], 0)?, lane(&a[7], 1)?, lane(&a[7], 2)?, lane(&a[7], 3)?, f32::from_bits(3), f32::from_bits(1), 0.0f32];
    let rel_chan = [lane(&a[8], 0)?, lane(&a[8], 1)?, lane(&a[8], 2)?];
    let iters = j3::cvtt32(levels);
    if iters > 7 {
        return unsup("more than seven iterations (the CPU reports an error)");
    }
    let variant = j3::cvtt32(deriv_io[0]);
    if variant != 0 {
        return unsup("deriv_io variant other than 0: the second weight job (reciprocal estimate lanes) is not on the device");
    }
    let bc = j3::cvtt32(deriv_io[1]);
    if j3::cvtt32(blur_type) != 4 {
        return unsup("blur_type other than 4: the CPU driver ports only core arm 4 (it reports an error)");
    }
    let cw = render_scale(c.kp, w);
    if cw < 0 {
        return unsup("negative context width");
    }
    let mut arr = [0f32; 11];
    arr[..7].copy_from_slice(&sigmoid);
    arr[7..10].copy_from_slice(&rel_chan);
    let width_f = cw as f32;
    let mut base = fx_common::x86::nx_mul(0.125f32 * tiny, width_f);
    let dv = (nu as f32) * edge_threshold;
    let kcell = c.kp.host_aspect;

    // plane ids: 0..nu the inputs, then the 3 nu + 5 scratch planes
    let nt = 3 * nu + 5;
    let tpl = DevImage::alloc(s, nt, n)?;
    let out = DevImage::alloc(s, nu, n)?;
    for p in 0..nt {
        s.queue.fill(tpl.plane(p), 0.0f32)?;
    }
    for p in 0..nu {
        s.queue.fill(out.plane(p), 0.0f32)?;
    }
    let buf = |pid: usize| -> &Buffer<f32> {
        if pid < nu {
            src.plane(pid)
        } else {
            tpl.plane(pid - nu)
        }
    };
    let tid = |k: usize| nu + k;
    let in_ids: Vec<usize> = (0..nu).collect();
    let mut pa = in_ids.clone();
    let mut pb: Vec<usize> = (0..nu).map(tid).collect();
    let pd: Vec<usize> = (0..nu).map(|i| tid(2 * nu + i)).collect();
    let (t0, t1) = (tid(3 * nu), tid(3 * nu + 1));
    let (wa, wt) = (tid(3 * nu + 2), tid(3 * nu + 3));
    let c1 = buf(if nu > 1 { 1 } else { 0 });
    let c2 = buf(if nu > 2 { 2 } else { 0 });
    let cur = mask_buf.unwrap_or_else(|| src.plane(0));
    let pw = libm_powf::powf(4.0f32, 3.0f32);
    let (wu, hu) = (w as u32, h as u32);
    let nn = n as u32;
    for it in 0..iters.max(0) as usize {
        let s2 = base.fdiv(dv);
        // weight planes
        let ca = j3::c_a(base);
        let cb = j3::c_a(base);
        let kcb = kcell.fmul(cb);
        let (mut c10, mut c11) = (0.0f32, 0.0f32);
        if sig_byte {
            c10 = ca;
            let bd = sigma0 as f64;
            if j3::LOW > bd {
                c11 = kcell.fmul(cb);
            } else if bd > j3::HIGH {
                c10 = j3::ONE;
                c11 = kcell;
            } else {
                let x = j3::ONE / sigma0;
                c10 = j3::minss(x, c10);
                c11 = kcell.fmul(j3::minss(x, cb));
            }
        }
        s.launch(
            "esh_weights_f32",
            n,
            &[
                src.plane(0).arg(),
                c1.arg(),
                c2.arg(),
                ClArg::I32(nu as i32),
                cur.arg(),
                ClArg::I32((!sig_byte) as i32),
                buf(wa).arg(),
                buf(wt).arg(),
                ClArg::U32(wu),
                ClArg::U32(hu),
                ClArg::F32(s2),
                ClArg::F32(s2),
                ClArg::F32(kcell),
                ClArg::F32(ca),
                ClArg::F32(cb),
                ClArg::F32(kcb),
                ClArg::F32(c10),
                ClArg::F32(c11),
            ],
        )?;
        if bc == 0 {
            s.launch("esh_cumsum_x_f32", h, &[buf(wa).arg(), buf(t0).arg(), ClArg::U32(wu), ClArg::U32(hu)])?;
            s.launch("esh_cumsum_y_f32", w, &[buf(wt).arg(), buf(t1).arg(), ClArg::U32(wu), ClArg::U32(hu)])?;
        } else {
            s.queue.copy(buf(wa), buf(t0))?;
            s.queue.copy(buf(wt), buf(t1))?;
        }
        for ch in 0..nu {
            let zero = j3::LOW > (base as f64) && (base as f64) > -j3::LOW;
            if zero {
                s.queue.copy(buf(pa[ch]), buf(pb[ch]))?;
            } else {
                let (hx, hy) = match edge_aware_sharpen_blur_core::radii(base, base, pw, 3) {
                    Ok(v) => v,
                    Err(_) => return unsup("exp2 argument beyond the fast path"),
                };
                let mut sp = in_ids[ch];
                let mut dp = wa;
                for i in 0..3 {
                    for (rad, is_x, aux) in [(hx[i], true, t0), (hy[i], false, t1)] {
                        if rad > 1.0f32 {
                            let p = edge_aware_sharpen_box_filter::cb_cell_a(rad);
                            let name = if is_x { "esh_filter_x_f32" } else { "esh_filter_y_f32" };
                            s.launch(name, if is_x { h } else { w }, &[buf(sp).arg(), buf(dp).arg(), buf(aux).arg(), ClArg::U32(wu), ClArg::U32(hu), ClArg::F32(p[0]), ClArg::F32(p[1]), ClArg::F32(p[2])])?;
                            let (ns, nd) = if sp == in_ids[ch] { (wa, pb[ch]) } else { (dp, sp) };
                            sp = ns;
                            dp = nd;
                        }
                    }
                }
                if pb[ch] != sp {
                    s.queue.copy(buf(sp), buf(pb[ch]))?;
                }
            }
            let m = fx_common::nanop::fma132(arr[it].mul_add(edge_aware_sharpen_render::MIX_C, -edge_aware_sharpen_render::MIX_C), edge_aware_sharpen_render::MIX_C, arr[7 + ch]);
            s.launch("esh_level_f32", n, &[buf(pb[ch]).arg(), buf(pa[ch]).arg(), out.plane(ch).arg(), ClArg::U32(nn), ClArg::F32(m)])?;
            if pa[ch] == in_ids[ch] {
                pa[ch] = pb[ch];
                pb[ch] = pd[ch];
            } else {
                std::mem::swap(&mut pa[ch], &mut pb[ch]);
            }
        }
        base += base;
    }
    for ch in 0..nu {
        s.launch("esh_add_f32", n, &[buf(pa[ch]).arg(), out.plane(ch).arg(), ClArg::U32(nn)])?;
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// self-check corpora

/// Frame shapes of the neighbourhood corpora: the common ones plus frames wide and tall enough for 64 sample windows.
const DIMS_NEIGHBOURHOOD: &[(usize, usize)] = &[(1, 1), (2, 1), (1, 3), (3, 5), (7, 1), (31, 1), (33, 2), (16, 16), (257, 1), (45, 23), (300, 4), (5, 300), (129, 3)];

pub(crate) fn lanes4(v: &[f32]) -> [u32; 4] {
    let last = *v.last().unwrap_or(&0.0);
    core::array::from_fn(|i| v.get(i).copied().unwrap_or(last).to_bits())
}

pub(crate) fn cst(pos: usize, v: &[f32]) -> (usize, usize, [u32; 4]) {
    (pos, v.len(), lanes4(v))
}

/// Moderate values in [-0.25, 1.25] and the adversarial set, alternating, so both ordinary sums and the specials are exercised.
fn plane_values(n: usize, seed: usize, plain: bool) -> Vec<f32> {
    if plain {
        let mut s = 0x9e37_79b9_7f4a_7c15u64 ^ (seed as u64).wrapping_mul(0x1234_5678_9abc_def1);
        (0..n)
            .map(|_| {
                s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                ((s >> 40) as f32 / (1u64 << 24) as f32) * 1.5 - 0.25
            })
            .collect()
    } else {
        let vals = adversarial_f32(4000, 41);
        (0..n).map(|i| vals[(i * 7 + seed * 13 + n) % vals.len()]).collect()
    }
}

fn image_of(ch: usize, n: usize, seed: usize, plain: bool) -> Image {
    Image { n, planes: (0..ch).map(|p| plane_values(n, seed * 5 + p, plain)).collect() }
}

pub(crate) fn kp_blur(edge: i32, rs: i32) -> KParams {
    let mut kp = KParams::standin();
    kp.blur_edge = edge;
    kp.blur_render_scale = rs;
    kp
}

fn blur_corpus() -> Vec<SelfCase> {
    // (amount x, amount y, shape, scale lanes, scale count, subpixel, edge, render scale override, aspect)
    type P = (f32, f32, f32, [f32; 4], usize, f32, i32, i32, Option<f32>);
    let sets: [P; 8] = [
        (0.5, 0.5, 2.0, [1.0, 1.0, 1.0, 1.0], 4, 0.0, 1, 0, None),
        (0.3, 0.7, 0.0, [2.0, 0.5, 1.0, 1.0], 4, 1.0, 0, 0, None),
        (1.7, 0.9, 1.0, [0.75, 1.0, 1.0, 1.0], 1, 1.0, 1, 0, None),
        (0.0, 0.4, 2.0, [1.0, 1.0, 1.0, 1.0], 4, 0.0, 1, 0, None),
        (-1.0, -2.0, 2.0, [1.5, 1.0, 1.0, 1.0], 4, 1.0, 1, 0, None),
        (0.6, 0.6, 1.0, [0.0, 0.0, 0.0, 0.0], 4, 0.0, 0, 0, None),
        (8.0, 6.0, 2.0, [1.0, 0.5, 0.25, 1.0], 4, 1.0, 1, 400, Some(0.75)),
        (5.9, 8.0, -1.0, [1.0, 1.0, 1.0, 1.0], 4, 0.0, 0, 400, None),
    ];
    let mut out = Vec::new();
    for (di, &(w, h)) in DIMS_NEIGHBOURHOOD.iter().enumerate() {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for tag in [1usize, 2, 3, 4] {
                if (di + si + tag) % 2 == 1 && n > 2000 {
                    continue;
                }
                let mut kp = kp_blur(p.6, p.7);
                kp.blur_aspect = p.8;
                let src = image_of(tag + (di % 2), n, di * 17 + si, si % 2 == 0);
                out.push(SelfCase {
                    name: format!("{w}x{h} set {si} tag {tag}"),
                    dims: (w, h),
                    tag: tag as i32,
                    args: vec![src],
                    consts: vec![cst(1, &[p.0, p.1]), cst(2, &[p.2]), cst(3, &p.3[..p.4]), cst(4, &[p.5])],
                    kp: Some(kp),
                });
            }
        }
    }
    out.extend(blur_nan_pair_cases());
    out.extend(blur_nan_wide_cases());
    // a constant source (the CPU lays it out as an image of constant planes)
    for &(w, h) in &[(3usize, 5usize), (16, 16), (45, 23)] {
        for lanes in [&[0.5f32][..], &[0.25, 2.0, -1.0][..]] {
            out.push(SelfCase {
                name: format!("{w}x{h} constant source {}", lanes.len()),
                dims: (w, h),
                tag: lanes.len().min(4) as i32,
                args: vec![],
                consts: vec![cst(0, lanes), cst(1, &[0.9, 0.6]), cst(2, &[2.0]), cst(3, &[1.0]), cst(4, &[0.0])],
                kp: Some(kp_blur(1, 0)),
            });
        }
    }
    out
}

/// Planes in which NaNs of different signs and payloads (signalling and quiet) sit between finite values and infinities, so
/// window sums see two NaN operands in one add and subtract, and infinity minus infinity makes fresh NaNs. The device result
/// must equal the CPU bit for bit, payloads included.
pub fn blur_nan_pair_cases() -> Vec<SelfCase> {
    const NANS: [u32; 6] = [0x7f80_0001, 0xffc0_0000, 0x7fc0_0002, 0xff80_0003, 0x7fa0_0000, 0xffff_ffff];
    const SPECIAL: [u32; 6] = [0x7f80_0000, 0xff80_0000, 0x0000_0000, 0x8000_0000, 0x3f80_0000, 0x7f7f_ffff];
    let sets: [(f32, f32, f32, i32); 5] = [(0.0, 0.0, 2.0, 1), (0.5, 0.5, 2.0, 1), (1.7, 0.9, 1.0, 0), (3.0, 2.0, 0.0, 1), (8.0, 6.0, 2.0, 0)];
    let mut out = Vec::new();
    for &(w, h) in &[(5usize, 7usize), (16, 16), (45, 23), (130, 3), (3, 130)] {
        let n = w * h;
        for (si, &(ax, ay, shape, edge)) in sets.iter().enumerate() {
            for density in [3usize, 7, 1] {
                let mut s = 0x2545_f491_4f6c_dd1du64 ^ ((w * 131 + h * 7 + si) as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
                let plane: Vec<f32> = (0..n)
                    .map(|i| {
                        s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                        let r = (s >> 33) as usize;
                        if i % density == 0 && density > 1 || density == 1 && r % 2 == 0 {
                            f32::from_bits(NANS[r % NANS.len()])
                        } else if r % 11 == 0 {
                            f32::from_bits(SPECIAL[(r >> 4) % SPECIAL.len()])
                        } else {
                            ((s >> 40) as f32 / (1u64 << 24) as f32) * 1.5 - 0.25
                        }
                    })
                    .collect();
                let mut kp = kp_blur(edge, 0);
                kp.blur_aspect = None;
                out.push(SelfCase {
                    name: format!("{w}x{h} nan pairs set {si} density {density}"),
                    dims: (w, h),
                    tag: 1,
                    args: vec![Image { n, planes: vec![plane] }],
                    consts: vec![cst(1, &[ax, ay]), cst(2, &[shape]), cst(3, &[1.0]), cst(4, &[if si % 2 == 0 { 0.0 } else { 1.0 }])],
                    kp: Some(kp),
                });
            }
        }
    }
    out
}

/// NaN words of both signs, signalling and quiet, with different payloads (none is the bare 0x7fc00000 that the sharpen kernel maps
/// to the default NaN), and the finite or infinite values that sit between them.
const PAIR_NANS: [u32; 6] = [0x7f80_0001, 0xffc0_0000, 0x7fc0_0002, 0xff80_0003, 0x7fa0_0000, 0xffff_ffff];
const PAIR_SPECIAL: [u32; 6] = [0x7f80_0000, 0xff80_0000, 0x0000_0000, 0x8000_0000, 0x3f80_0000, 0x7f7f_ffff];

/// A plane in which every `density`-th sample (density 1: a random half) is a NaN, some are specials and the rest lie in [lo, hi).
fn pair_plane(n: usize, seed: u64, density: usize, lo: f32, hi: f32) -> Vec<f32> {
    let mut s = 0x2545_f491_4f6c_dd1du64 ^ seed.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    (0..n)
        .map(|i| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let r = (s >> 33) as usize;
            if i % density == 0 && density > 1 || density == 1 && r % 2 == 0 {
                f32::from_bits(PAIR_NANS[r % PAIR_NANS.len()])
            } else if r % 11 == 0 {
                f32::from_bits(PAIR_SPECIAL[(r >> 4) % PAIR_SPECIAL.len()])
            } else {
                ((s >> 40) as f32 / (1u64 << 24) as f32) * (hi - lo) + lo
            }
        })
        .collect()
}

/// masked_blur with NaN in the mask and in the source, so the premultiply sees two NaN operands and the combine sees a NaN source with a
/// NaN difference. The device result must equal the CPU bit for bit, payloads included.
pub fn masked_blur_nan_pair_cases() -> Vec<SelfCase> {
    let sets: [(f32, f32, f32, f32); 4] = [(0.0, 0.0, 2.0, 0.0), (0.5, 0.5, 2.0, 0.0), (1.3, 0.2, 1.0, 1.0), (3.0, 2.0, 0.0, 1.0)];
    let mut out = Vec::new();
    for &(w, h) in &[(5usize, 7usize), (16, 16), (45, 23), (130, 3)] {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for density in [3usize, 7, 1] {
                for tag in [1usize, 3] {
                    let seed = (w * 131 + h * 7 + si * 3 + density) as u64;
                    let src = Image { n, planes: (0..tag).map(|c| pair_plane(n, seed + 17 * c as u64, density, -0.25, 1.25)).collect() };
                    let mask = Image { n, planes: vec![pair_plane(n, seed ^ 0x55, density + 1, -0.1, 1.2)] };
                    let mut kp = KParams::standin();
                    kp.blur_render_scale = 0;
                    out.push(SelfCase {
                        name: format!("{w}x{h} nan pairs set {si} density {density} tag {tag}"),
                        dims: (w, h),
                        tag: tag as i32,
                        args: vec![src, mask],
                        consts: vec![cst(2, &[p.0, p.1]), cst(3, &[p.2]), cst(4, &[p.3])],
                        kp: Some(kp),
                    });
                }
            }
        }
    }
    out
}

/// sharpen with NaN samples, gain exactly 1.0 (the fused unit path), other gains and a NaN gain, so the fused step sees two NaN operands
/// in every pair of (source, difference, gain). The device result must equal the CPU bit for bit, payloads included.
pub fn sharpen_nan_pair_cases() -> Vec<SelfCase> {
    let nan_gain = f32::from_bits(0x7fc0_0005);
    let sets: [(f32, [f32; 4], bool); 5] = [
        (0.4, [1.0, 1.0, 1.0, 1.0], false),
        (0.4, [2.0, 0.5, 1.0, 1.0], true),
        (1.0, [nan_gain, 1.0, 3.0, 1.0], false),
        (2.5, [1.0, nan_gain, 1.0, 1.0], true),
        (0.112, [1.5, 1.0, nan_gain, 1.0], false),
    ];
    let mut out = Vec::new();
    for &(w, h) in &[(5usize, 7usize), (16, 16), (45, 23), (130, 3)] {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for density in [3usize, 7, 1] {
                for tag in [1usize, 3] {
                    let seed = (w * 131 + h * 7 + si * 3 + density) as u64;
                    let src = Image { n, planes: (0..tag).map(|c| pair_plane(n, seed + 17 * c as u64, density, -0.25, 1.25)).collect() };
                    let mut kp = KParams::standin();
                    kp.sharpen_count = Some(n);
                    kp.sharpen_alias = p.2;
                    out.push(SelfCase {
                        name: format!("{w}x{h} nan pairs set {si} density {density} tag {tag}"),
                        dims: (w, h),
                        tag: tag as i32,
                        args: vec![src],
                        consts: vec![cst(1, &p.1), cst(2, &[p.0])],
                        kp: Some(kp),
                    });
                }
            }
        }
    }
    out
}

/// A NaN gain lane (quiet, positive, payload 5), for the final scale and the per channel gains.
const NAN_LANE: u32 = 0x7fc0_0005;

/// Frames whose rows or columns are long enough for the eight and sixty-four lane window sums of the blur passes, with radii of
/// 15 to 75 samples, both edge modes, one to three passes, the subpixel blend, the aspect and a NaN gain lane; NaN of both signs
/// and several payloads sit between finite values, infinities and zeros. The device result must equal the CPU bit for bit.
pub fn blur_nan_wide_cases() -> Vec<SelfCase> {
    let nan = f32::from_bits(NAN_LANE);
    // (amount x, amount y, shape, subpixel, edge, aspect, scale lanes)
    type P = (f32, f32, f32, f32, i32, Option<f32>, [f32; 4]);
    let sets: [P; 5] = [
        (1.5, 1.5, 2.0, 1.0, 1, None, [1.0, 0.5, 2.0, 1.0]),
        (0.6, 0.3, 0.0, 0.0, 0, None, [1.0, 1.0, 1.0, 1.0]),
        (0.9, 1.4, 1.0, 1.0, 0, Some(0.75), [0.75, nan, 1.0, 1.0]),
        (1.1, 1.1, 2.0, 0.0, 1, None, [nan, 1.0, 1.0, 0.25]),
        (0.4, 0.8, -1.0, 1.0, 1, None, [1.0, 1.0, 1.0, 1.0]),
    ];
    let mut out = Vec::new();
    for &(w, h) in &[(200usize, 5usize), (5, 200), (257, 3), (96, 40)] {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for density in [5usize, 1] {
                for tag in [1usize, 3] {
                    let seed = (w * 31 + h * 17 + si * 5 + density) as u64;
                    let src = Image { n, planes: (0..tag).map(|c| pair_plane(n, seed + 23 * c as u64, density, -0.25, 1.25)).collect() };
                    let mut kp = kp_blur(p.4, 400);
                    kp.blur_aspect = p.5;
                    out.push(SelfCase {
                        name: format!("{w}x{h} wide nan set {si} density {density} tag {tag}"),
                        dims: (w, h),
                        tag: tag as i32,
                        args: vec![src],
                        consts: vec![cst(1, &[p.0, p.1]), cst(2, &[p.2]), cst(3, &p.6), cst(4, &[p.3])],
                        kp: Some(kp),
                    });
                }
            }
        }
    }
    out
}

/// masked_blur on the same long frames: NaN in the source and in the mask, so the clamp, the premultiply, both blurs, the
/// division by the blurred mask and the fused combine all see NaN operands. The device result must equal the CPU bit for bit.
pub fn masked_blur_nan_wide_cases() -> Vec<SelfCase> {
    let sets: [(f32, f32, f32, f32); 3] = [(1.5, 1.5, 2.0, 1.0), (0.6, 0.3, 0.0, 0.0), (0.9, 1.4, 1.0, 1.0)];
    let mut out = Vec::new();
    for &(w, h) in &[(200usize, 5usize), (5, 200), (96, 40)] {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for density in [5usize, 1] {
                for tag in [1usize, 3] {
                    let seed = (w * 37 + h * 11 + si * 7 + density) as u64;
                    let src = Image { n, planes: (0..tag).map(|c| pair_plane(n, seed + 19 * c as u64, density, -0.25, 1.25)).collect() };
                    let mask = Image { n, planes: vec![pair_plane(n, seed ^ 0x3c, density + 2, -0.1, 1.2)] };
                    let mut kp = KParams::standin();
                    kp.blur_render_scale = 400;
                    out.push(SelfCase {
                        name: format!("{w}x{h} wide nan set {si} density {density} tag {tag}"),
                        dims: (w, h),
                        tag: tag as i32,
                        args: vec![src, mask],
                        consts: vec![cst(2, &[p.0, p.1]), cst(3, &[p.2]), cst(4, &[p.3])],
                        kp: Some(kp),
                    });
                }
            }
        }
    }
    out
}

/// sharpen on the long frames (the blur's wide window sums under the combine), gain exactly 1.0, other gains and a NaN gain.
pub fn sharpen_nan_wide_cases() -> Vec<SelfCase> {
    let nan = f32::from_bits(NAN_LANE);
    let sets: [(f32, [f32; 4], bool); 3] = [(0.8, [1.0, 1.0, 1.0, 1.0], false), (0.5, [2.0, nan, 1.0, 0.5], true), (1.2, [nan, 1.0, 1.5, 1.0], false)];
    let mut out = Vec::new();
    for &(w, h) in &[(200usize, 5usize), (5, 200), (96, 40)] {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for density in [5usize, 1] {
                for tag in [1usize, 3] {
                    let seed = (w * 41 + h * 13 + si * 3 + density) as u64;
                    let src = Image { n, planes: (0..tag).map(|c| pair_plane(n, seed + 29 * c as u64, density, -0.25, 1.25)).collect() };
                    let mut kp = KParams::standin();
                    kp.sharpen_count = Some(if si == 1 { n / 2 } else { n });
                    kp.sharpen_alias = p.2;
                    kp.blur_render_scale = 400;
                    out.push(SelfCase {
                        name: format!("{w}x{h} wide nan set {si} density {density} tag {tag}"),
                        dims: (w, h),
                        tag: tag as i32,
                        args: vec![src],
                        consts: vec![cst(1, &p.1), cst(2, &[p.0])],
                        kp: Some(kp),
                    });
                }
            }
        }
    }
    out
}

/// edge_aware_sharpen with NaN of both signs and several payloads in the source channels (the absolute differences and their
/// sums in the weights), in a plane mask (the minimum and the reciprocal of the weight functions), infinities (fresh NaN in the
/// differences) and a NaN sigmoid or channel lane (the mix factor). The device result must equal the CPU bit for bit.
pub fn esh_nan_cases() -> Vec<SelfCase> {
    let nan = f32::from_bits(NAN_LANE);
    // (plane mask, scalar mask value, levels, tiny, threshold, sigmoid lanes, rel_chan lanes)
    type P = (bool, f32, f32, f32, f32, [f32; 4], [f32; 3]);
    let sets: [P; 5] = [
        (true, 0.0, 4.0, 0.5, 1.0, [1.0, 0.75, 0.5, 0.25], [1.0, 1.0, 1.0]),
        (false, 0.4, 3.0, 0.6, 2.0, [1.0, 1.0, 1.0, 1.0], [0.5, 1.5, 1.0]),
        (false, 1.0, 7.0, 0.9, 0.5, [1.5, 1.0, 0.5, 0.25], [1.0, 2.0, 0.25]),
        (true, 0.0, 3.0, 0.5, 1.0, [1.0, nan, 0.5, 0.25], [1.0, 1.0, 1.0]),
        (false, 0.4, 2.0, 0.3, 1.0, [1.0, 1.0, 1.0, 1.0], [1.0, nan, 2.0]),
    ];
    let mut out = Vec::new();
    for &(w, h) in &[(3usize, 5usize), (16, 16), (45, 23), (70, 9), (9, 70)] {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for density in [5usize, 1] {
                for tag in [1usize, 2, 3] {
                    let seed = (w * 43 + h * 19 + si * 11 + density) as u64;
                    let src = Image { n, planes: (0..tag).map(|c| pair_plane(n, seed + 31 * c as u64, density, -0.25, 1.25)).collect() };
                    let mut args = vec![src];
                    let mut consts = vec![];
                    if p.0 {
                        args.push(Image { n, planes: vec![pair_plane(n, seed ^ 0x5a, density + 3, 0.0, 1.0)] });
                    } else {
                        consts.push(cst(1, &[p.1]));
                    }
                    consts.push(cst(2, &[4.0]));
                    consts.push(cst(3, &[0.0, 0.0]));
                    consts.push(cst(4, &[p.2]));
                    consts.push(cst(5, &[p.3]));
                    consts.push(cst(6, &[p.4]));
                    consts.push(cst(7, &p.5));
                    consts.push(cst(8, &p.6));
                    consts.push(cst(9, &[1.0]));
                    let mut kp = KParams::standin();
                    kp.host_aspect = if si % 2 == 0 { 1.0 } else { 0.5 };
                    out.push(SelfCase { name: format!("{w}x{h} nan set {si} density {density} tag {tag}"), dims: (w, h), tag: tag as i32, args, consts, kp: Some(kp) });
                }
            }
        }
    }
    out
}

fn masked_blur_corpus() -> Vec<SelfCase> {
    let sets: [(f32, f32, f32, f32, i32); 5] = [(0.5, 0.5, 2.0, 0.0, 0), (0.3, 0.8, 0.0, 1.0, 0), (1.3, 0.2, 1.0, 1.0, 0), (0.0, 0.0, 2.0, 0.0, 0), (8.0, 6.0, 2.0, 1.0, 400)];
    let mut out = Vec::new();
    for (di, &(w, h)) in DIMS_NEIGHBOURHOOD.iter().enumerate() {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for tag in [1usize, 3, 4] {
                if (di + si + tag) % 2 == 1 && n > 2000 {
                    continue;
                }
                let plain = (si + di) % 2 == 0;
                let src = image_of(tag, n, di * 3 + si, plain);
                let mask = image_of(1, n, di * 11 + si + 1, plain || si == 3);
                let mut kp = KParams::standin();
                kp.blur_render_scale = p.4;
                out.push(SelfCase {
                    name: format!("{w}x{h} set {si} tag {tag}"),
                    dims: (w, h),
                    tag: tag as i32,
                    args: vec![src, mask],
                    consts: vec![cst(2, &[p.0, p.1]), cst(3, &[p.2]), cst(4, &[p.3])],
                    kp: Some(kp),
                });
            }
        }
    }
    for &(w, h) in &[(3usize, 5usize), (16, 16)] {
        out.push(SelfCase {
            name: format!("{w}x{h} constant source"),
            dims: (w, h),
            tag: 1,
            args: vec![image_of(1, w * h, 99, true)],
            consts: vec![cst(0, &[0.75]), cst(2, &[0.8]), cst(3, &[0.4]), cst(4, &[1.0])],
            kp: Some(KParams::standin()),
        });
    }
    out.extend(masked_blur_nan_pair_cases());
    out.extend(masked_blur_nan_wide_cases());
    out
}

fn grad_corpus() -> Vec<SelfCase> {
    let sets: [(f32, f32, f32, f32, i32, i32); 6] = [
        (0.5, 1.0, 0.0, 1.0, 1, 0),
        (0.25, 0.7, 1.0, 0.5, 0, 0),
        (1.9, 2.0, 1.0, 2.0, 1, 0),
        (0.0, 1.0, 0.0, 1.0, 1, 0),
        (6.0, 1.0, 0.0, 1.0, 1, 400),
        (3.0, 1.5, 1.0, 1.0, 0, 400),
    ];
    let mut out = Vec::new();
    for (di, &(w, h)) in DIMS_NEIGHBOURHOOD.iter().enumerate() {
        if w < 2 {
            continue;
        }
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            if (di + si) % 2 == 1 && n > 2000 {
                continue;
            }
            let src = image_of(1 + (si % 2), n, di * 7 + si, si % 2 == 0);
            let mut kp = kp_blur(p.4, p.5);
            kp.host_aspect = p.3;
            out.push(SelfCase {
                name: format!("{w}x{h} set {si}"),
                dims: (w, h),
                tag: 2,
                args: vec![src],
                consts: vec![cst(1, &[p.0]), cst(2, &[p.1]), cst(3, &[p.2])],
                kp: Some(kp),
            });
        }
    }
    for &(w, h) in &[(3usize, 5usize), (16, 16)] {
        out.push(SelfCase {
            name: format!("{w}x{h} constant source"),
            dims: (w, h),
            tag: 2,
            args: vec![],
            consts: vec![cst(0, &[0.6]), cst(1, &[0.5]), cst(2, &[1.0]), cst(3, &[1.0])],
            kp: Some(KParams::standin()),
        });
    }
    out
}

fn sharpen_corpus() -> Vec<SelfCase> {
    // (amount, gains, count fraction (0 = all, 1 = half), alias)
    let sets: [(f32, [f32; 4], u8, bool); 6] = [
        (0.112, [2.0, 2.0, 2.0, 2.0], 0, false),
        (0.4, [1.0, 0.5, 3.0, 1.0], 0, true),
        (0.9, [0.0, 1.5, 1.0, 0.25], 1, false),
        (2.5, [1.25, 1.0, 1.0, 1.0], 1, true),
        (-1.0, [2.0, 2.0, 2.0, 2.0], 0, false),
        (6.0, [1.0, 2.0, 0.5, 1.0], 0, false),
    ];
    let mut out = Vec::new();
    for (di, &(w, h)) in DIMS_NEIGHBOURHOOD.iter().enumerate() {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for tag in [1usize, 2, 3, 4] {
                if (di + si + tag) % 2 == 1 && n > 2000 {
                    continue;
                }
                let src = image_of(tag, n, di * 9 + si, (si + di) % 2 == 0);
                let mut kp = KParams::standin();
                kp.sharpen_count = Some(if p.2 == 0 { n } else { n / 2 });
                kp.sharpen_alias = p.3;
                if si == 5 {
                    kp.blur_render_scale = 400;
                }
                out.push(SelfCase {
                    name: format!("{w}x{h} set {si} tag {tag}"),
                    dims: (w, h),
                    tag: tag as i32,
                    args: vec![src],
                    consts: vec![cst(1, &p.1), cst(2, &[p.0])],
                    kp: Some(kp),
                });
            }
        }
    }
    for &(w, h) in &[(3usize, 5usize), (16, 16)] {
        let mut kp = KParams::standin();
        kp.sharpen_count = Some(w * h);
        out.push(SelfCase { name: format!("{w}x{h} constant source"), dims: (w, h), tag: 3, args: vec![], consts: vec![cst(0, &[0.5, 0.25, 0.75]), cst(1, &[2.0, 2.0, 2.0, 2.0]), cst(2, &[0.4])], kp: Some(kp) });
    }
    out.extend(sharpen_nan_pair_cases());
    out.extend(sharpen_nan_wide_cases());
    out
}

fn static2_corpus() -> Vec<SelfCase> {
    let mut out = Vec::new();
    for &(w, h) in DIMS_NEIGHBOURHOOD {
        for (i, seed) in [0i32, 1, -1, 4096, 0x7fff_ffff, i32::MIN, 123_456_789].iter().enumerate() {
            let mut kp = KParams::standin();
            kp.static2_seed_a = *seed;
            out.push(SelfCase { name: format!("{w}x{h} seed {seed}"), dims: (w, h), tag: 2 - (i as i32 % 2), args: vec![], consts: vec![], kp: Some(kp) });
        }
    }
    out
}

fn esh_corpus() -> Vec<SelfCase> {
    // (mask kind: 0 plane, 1 scalar), scalar mask value, blur_type, bc, levels, tiny, threshold, sigmoid lanes, rel_chan lanes
    type P = (u8, f32, f32, f32, f32, f32, f32, [f32; 4], [f32; 3]);
    let sets: [P; 7] = [
        (0, 0.0, 4.0, 0.0, 4.0, 0.5, 1.0, [1.0, 0.75, 0.5, 0.25], [1.0, 1.0, 1.0]),
        (1, 0.4, 4.0, 0.0, 3.0, 0.6, 2.0, [1.0, 1.0, 1.0, 1.0], [0.5, 1.5, 1.0]),
        (1, 0.0, 4.0, 1.0, 2.0, 0.3, 1.0, [0.5, 1.0, 0.0, 2.0], [1.0, 0.0, 2.0]),
        (0, 0.0, 4.0, 0.0, 1.0, 0.0, 1.0, [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0]),
        (1, 1.0, 4.0, 0.0, 7.0, 0.9, 0.5, [1.5, 1.0, 0.5, 0.25], [1.0, 2.0, 0.25]),
        (0, 0.0, 4.0, 0.0, 0.0, 0.5, 1.0, [1.0, 1.0, 1.0, 1.0], [1.0, 1.0, 1.0]),
        (1, 0.00001, 4.0, 0.0, 4.0, 4.0, 1.0, [1.0, 0.75, 0.5, 0.25], [1.0, 1.0, 1.0]),
    ];
    let dims: &[(usize, usize)] = &[(1, 1), (2, 1), (1, 3), (3, 5), (7, 1), (16, 16), (45, 23), (70, 9), (129, 3)];
    let mut out = Vec::new();
    for (di, &(w, h)) in dims.iter().enumerate() {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            for tag in [1usize, 2, 3] {
                if (di + si + tag) % 3 == 0 && n > 100 {
                    continue;
                }
                let plain = si % 3 != 2;
                let src = image_of(tag, n, di * 13 + si, plain);
                let mut args = vec![src];
                let mut consts = vec![];
                if p.0 == 0 {
                    let mut m = image_of(1, n, di * 29 + si, true);
                    for v in m.planes[0].iter_mut() {
                        *v = v.abs().min(1.0);
                    }
                    args.push(m);
                } else {
                    consts.push(cst(1, &[p.1]));
                }
                consts.push(cst(2, &[p.2]));
                consts.push(cst(3, &[0.0, p.3]));
                consts.push(cst(4, &[p.4]));
                consts.push(cst(5, &[p.5]));
                consts.push(cst(6, &[p.6]));
                consts.push(cst(7, &p.7));
                consts.push(cst(8, &p.8));
                consts.push(cst(9, &[1.0]));
                let mut kp = KParams::standin();
                kp.host_aspect = if si % 2 == 0 { 1.0 } else { 0.5 };
                out.push(SelfCase { name: format!("{w}x{h} set {si} tag {tag}"), dims: (w, h), tag: tag as i32, args, consts, kp: Some(kp) });
            }
        }
    }
    // mask values on both sides of the two double precision thresholds of the weight functions (compared after widening to double)
    {
        let edges: Vec<f32> = [edge_aware_sharpen_jobs::LOW, edge_aware_sharpen_jobs::HIGH]
            .iter()
            .flat_map(|&t| {
                let b = (t as f32).to_bits();
                (b.saturating_sub(2)..=b + 2).map(f32::from_bits)
            })
            .chain([0.0, 1.0, f32::MIN_POSITIVE, 1.0e-30])
            .collect();
        let (w, h) = (16usize, 16usize);
        let n = w * h;
        let mut m = image_of(1, n, 11, true);
        for (i, v) in m.planes[0].iter_mut().enumerate() {
            *v = edges[i % edges.len()];
        }
        out.push(SelfCase {
            name: format!("{w}x{h} mask at the double thresholds"),
            dims: (w, h),
            tag: 3,
            args: vec![image_of(3, n, 12, true), m],
            consts: vec![cst(2, &[4.0]), cst(3, &[0.0, 0.0]), cst(4, &[3.0]), cst(5, &[0.5]), cst(6, &[1.0]), cst(7, &[1.0, 0.75, 0.5, 0.25]), cst(8, &[1.0, 1.0, 1.0]), cst(9, &[1.0])],
            kp: Some(KParams::standin()),
        });
    }
    // a constant source with a plane mask
    for &(w, h) in &[(3usize, 5usize), (16, 16)] {
        let n = w * h;
        let mut m = image_of(1, n, 7, true);
        for v in m.planes[0].iter_mut() {
            *v = v.abs().min(1.0);
        }
        out.push(SelfCase {
            name: format!("{w}x{h} constant source"),
            dims: (w, h),
            tag: 3,
            args: vec![m],
            consts: vec![cst(0, &[0.5, 0.25, 0.75]), cst(2, &[4.0]), cst(3, &[0.0, 0.0]), cst(4, &[3.0]), cst(5, &[0.5]), cst(6, &[1.0]), cst(7, &[1.0, 0.75, 0.5, 0.25]), cst(8, &[1.0, 1.0, 1.0]), cst(9, &[1.0])],
            kp: Some(KParams::standin()),
        });
    }
    out.extend(esh_nan_cases());
    out
}

/// The NaN-exact neighbourhood kernels on a real device (prints NOT RUN without one, fails when `SAPPHIRE_REQUIRE_DEVICE=1`):
/// single edits of their OpenCL source, each its own program build, must be refused by the self-check of at least one op (a
/// mismatch, or other NaN payloads, which keeps the op off the NaN-exact set).
#[cfg(test)]
mod device_nan_exact_neighbourhood_tests {
    use crate::device_ops;
    use fx_device::selfcheck::{NanPayloads, Tally};
    use fx_device::{Backend as Handle, DeviceCtx, DeviceKind, Session};
    use std::sync::Arc;

    fn device() -> Option<Arc<DeviceCtx>> {
        match fx_device::select(DeviceKind::Gpu) {
            Handle::Device(d) => Some(d),
            other => {
                if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
                    panic!("device required but unavailable: {}", other.describe());
                }
                println!("NOT RUN: no usable OpenCL device ({})", other.describe());
                None
            }
        }
    }

    fn op(class: &str) -> &'static device_ops::DeviceOp {
        device_ops::DEVICE_OPS.iter().find(|o| o.class == class).expect("a NaN-exact op has a kernel")
    }

    fn mutant_session(ctx: &DeviceCtx, source: &str) -> Arc<Session> {
        let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
        let _ = Tally::empty();
        Arc::new(Session::open_source(&dev, &info, source, fx_device::kernels::BUILD_OPTIONS).unwrap())
    }

    /// (name, text, replacement, ops whose self-check must refuse it)
    const MUTANTS: &[(&str, &str, &str, &[&str])] = &[
        (
            "pick operand order",
            "float nf_pick(float a, float b) {\n    if (a != a) {\n        return as_float(as_uint(a) | 0x400000u);\n    }\n    if (b != b) {\n        return as_float(as_uint(b) | 0x400000u);\n    }",
            "float nf_pick(float a, float b) {\n    if (b != b) {\n        return as_float(as_uint(b) | 0x400000u);\n    }\n    if (a != a) {\n        return as_float(as_uint(a) | 0x400000u);\n    }",
            &["blur", "sharpen", "edge_aware_sharpen"],
        ),
        (
            "pick quiet bit constant",
            "float nf_pick(float a, float b) {\n    if (a != a) {\n        return as_float(as_uint(a) | 0x400000u);",
            "float nf_pick(float a, float b) {\n    if (a != a) {\n        return as_float(as_uint(a) | 0x200000u);",
            &["blur", "sharpen"],
        ),
        ("fresh NaN sign", "    return as_float(0xffc00000u);\n}\nfloat nf_pick3", "    return as_float(0x7fc00000u);\n}\nfloat nf_pick3", &["blur", "sharpen"]),
        (
            "fused pick operand order",
            "float nf_pick3(float a, float b, float c) {\n    if (a != a) {\n        return as_float(as_uint(a) | 0x400000u);\n    }\n    return nf_pick(b, c);",
            "float nf_pick3(float a, float b, float c) {\n    if (b != b) {\n        return as_float(as_uint(b) | 0x400000u);\n    }\n    return nf_pick(a, c);",
            &["sharpen", "masked_blur", "blur"],
        ),
        ("contracted multiply-add replaced by two roundings", "float nf_fms213(float d, float s2, float s3) {\n    float r = fma(d, s2, -s3);", "float nf_fms213(float d, float s2, float s3) {\n    float r = d * s2 - s3;", &["sharpen", "blur"]),
        (
            "window sum order",
            "        y = nf_add(nf_sub(BW_AT(x, i + r + 1), BW_AT(x, i - r)), y);",
            "        y = nf_add(y, nf_sub(BW_AT(x, i + r + 1), BW_AT(x, i - r)));",
            &["blur"],
        ),
        (
            "add NaN select arm",
            "float nf_add(float a, float b) {\n    float r = a + b;\n    return (r == r) ? r : nf_pick(a, b);",
            "float nf_add(float a, float b) {\n    float r = a + b;\n    return (r == r) ? r : nf_pick(b, a);",
            &["blur", "edge_aware_sharpen"],
        ),
        ("combine threshold compare direction", "    if (!(mv > thr)) {", "    if (!(mv >= thr)) {", &["masked_blur"]),
        ("box filter compare direction", "                if (t > b[(size_t)lo * bst]) {", "                if (t >= b[(size_t)lo * bst]) {", &["edge_aware_sharpen"]),
        ("negation constant", "    float c = nf_fma213(bb[i], -0x1p+0f, 0x0p+0f);", "    float c = nf_fma213(bb[i], 0x1p+0f, 0x0p+0f);", &["edge_aware_sharpen"]),
        ("box filter sum operand order", "            sum = nf_add(sum, a[(size_t)hh * ast]);", "            sum = nf_add(a[(size_t)hh * ast], sum);", &["edge_aware_sharpen"]),
        (
            "gain 1 lone NaN subtrahend negated",
            "    float r = fma(d, s2, -s3);\n    return (r == r) ? r : nf_pick3(d, s2, s3);",
            "    float r = fma(d, s2, -s3);\n    return (r == r) ? r : nf_pick3(d, s2, -s3);",
            &["sharpen"],
        ),
        ("masked division replaced", "            b = nf_div(b, mb_min_x86(wv, 0x1p+0f));", "            b = nf_mul(b, mb_min_x86(wv, 0x1p+0f));", &["masked_blur"]),
    ];

    #[test]
    #[ignore]
    fn nan_exact_neighbourhood_source_mutants_are_refused() {
        let Some(ctx) = device() else { return };
        let base = fx_device::kernels::device_program();
        let base_digest = fx_device::progcache::fnv(base.as_bytes());
        let mut prev = base_digest;
        let mut survivors = Vec::new();
        for (name, from, to, ops) in MUTANTS {
            assert_eq!(base.matches(from).count(), 1, "mutant `{name}`: the edited text must occur once");
            let src = base.replacen(from, to, 1);
            let digest = fx_device::progcache::fnv(src.as_bytes());
            assert!(digest != base_digest && digest != prev, "mutant `{name}` equals the base or the previous mutant");
            prev = digest;
            let session = mutant_session(&ctx, &src);
            let mut verdicts = Vec::new();
            let mut killed = false;
            for class in *ops {
                match device_ops::selfcheck(&session, op(class)) {
                    Err(e) => {
                        killed = true;
                        verdicts.push(format!("{class}: mismatch ({})", e.chars().take(160).collect::<String>()));
                    }
                    Ok(t) if t.nan_payloads != NanPayloads::Same => {
                        killed = true;
                        verdicts.push(format!("{class}: NaN payloads differ"));
                    }
                    Ok(_) => verdicts.push(format!("{class}: equal")),
                }
            }
            println!("MUTANT {name} program {digest:016x}: {} | {}", if killed { "KILLED" } else { "SURVIVED" }, verdicts.join("; "));
            if !killed {
                survivors.push(*name);
            }
        }
        assert!(survivors.is_empty(), "surviving mutants: {survivors:?}");
    }
}
