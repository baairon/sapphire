//! Device ops of the ripmap family: make_mipmap (the pyramid stays on the device as a `DevMipVal`) and blur_warp.
//!
//! Kernels live in `fx_device::kernels_k2b`. `accepts` is the shape level test; `run` is the value level one: a constant or a
//! frame that puts the node outside the kernel's contract returns `DevError::Unsupported` and the executor runs the CPU kernel
//! (which then reports its own error where it has one). The self-check runs every corpus case on the device and through
//! `kernels::run_any` and compares every output bit.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp};
use crate::kernels;
use crate::kernels2::{cvt_i32, geom_of, Out};
use crate::types::{Arg, DevMipVal, Image, KParams, MipImage};
use fx_device::cl::{Access, Arg as ClArg, Buffer};
use fx_device::selfcheck::{adversarial_f32, compare_f32, Mismatch, NanPayloads, Tally};
use fx_device::sim::Rng;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

/// The classes this module serves.
pub const CLASSES: &[&str] = &["make_mipmap", "blur_warp"];

pub fn handles(class: &str) -> bool {
    CLASSES.contains(&class)
}

pub fn empty_corpus() -> Vec<crate::device_ops::SelfCase> {
    Vec::new()
}

fn unsup(m: &str) -> DevError {
    DevError::Unsupported(m.into())
}

fn lane(a: &DevArg, i: usize) -> Option<f32> {
    match a {
        DevArg::Const { lanes, .. } => Some(f32::from_bits(lanes[i.min(3)])),
        _ => None,
    }
}

fn is_const(s: &ArgShape) -> bool {
    matches!(s, ArgShape::Const { .. })
}

struct ErrCell {
    buf: Buffer<u32>,
}

impl ErrCell {
    fn new(s: &Session) -> Result<ErrCell, DevError> {
        Ok(ErrCell { buf: Buffer::from_slice(&s.context, &[0u32], Access::ReadWrite)? })
    }
    fn arg(&self) -> ClArg<'_> {
        self.buf.arg()
    }
    fn check(&self, s: &Session, what: &str) -> Result<(), DevError> {
        if s.queue.read(&self.buf)?[0] != 0 {
            return Err(unsup(what));
        }
        Ok(())
    }
}

// ---- make_mipmap --------------------------------------------------------------------------------------------------

pub fn make_mipmap_accepts(c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    if shapes.len() != 3 {
        return Err("three operands expected");
    }
    if !(1..=4).contains(&c.tag) {
        return Err("combination tag outside 1..4");
    }
    match shapes[0] {
        ArgShape::Img { channels } if (1..=4).contains(&channels) => {}
        ArgShape::Const { n } if (1..=4).contains(&n.max(1)) => {}
        _ => return Err("the source must be an image of 1 to 4 channels or a constant"),
    }
    if !is_const(&shapes[1]) || !is_const(&shapes[2]) {
        return Err("wrap and filter operands must be constants");
    }
    Ok(())
}

/// The layout of one make_mipmap call: what `make_mipmap_k` derives before it touches a pixel.
struct MmPlan {
    w: usize,
    h: usize,
    c: usize,
    wx: i32,
    wy: i32,
    filter: i32,
    widths: Vec<usize>,
    heights: Vec<usize>,
    scale: Option<Vec<u32>>,
    pad: [u32; 4],
}

fn mm_plan(c: &NodeCall, args: &[DevArg]) -> Result<MmPlan, DevError> {
    if args.len() != 3 {
        return Err(unsup("three operands expected"));
    }
    let (w, h) = c.dims;
    if w == 0 || h == 0 || w * h != c.n || w >= (1 << 24) || h >= (1 << 24) {
        return Err(unsup("frame outside the kernel's range"));
    }
    let ch = match &args[0] {
        DevArg::Img(d) if d.n == c.n => d.channels(),
        DevArg::Const { n, .. } => (*n).max(1),
        _ => return Err(unsup("the source is not an image of the frame size")),
    };
    if !(1..=4).contains(&ch) {
        return Err(unsup("channel count outside 1..4"));
    }
    let wx = cvt_i32(lane(&args[1], 0).ok_or_else(|| unsup("wrap is not a constant"))?);
    let wy = cvt_i32(lane(&args[1], 1).ok_or_else(|| unsup("wrap is not a constant"))?);
    let filter = cvt_i32(lane(&args[2], 0).ok_or_else(|| unsup("filter is not a constant"))?);
    let o = make_mipmap::mm2::ObjParams { w: w as i32, h: h as i32, c: ch, d0: 1 };
    let a = make_mipmap::mm2::alloc(&o, filter, make_mipmap::mm2::Mut2::None).map_err(|_| unsup("allocator refused (the CPU reports it)"))?;
    if a.widths.iter().chain(a.heights.iter()).any(|&v| v <= 0) {
        return Err(unsup("non positive level extent (the CPU reports it)"));
    }
    let widths: Vec<usize> = a.widths.iter().map(|&v| v as usize).collect();
    let heights: Vec<usize> = a.heights.iter().map(|&v| v as usize).collect();
    if filter != 0 {
        for l in 1..widths.len() {
            if 2 * widths[l] > widths[l - 1] + 2 {
                return Err(unsup("level extents outside the pyramid contract (the CPU reports it)"));
            }
        }
        for j in 1..heights.len() {
            if 2 * heights[j] > heights[j - 1] + 2 {
                return Err(unsup("level extents outside the pyramid contract (the CPU reports it)"));
            }
        }
    }
    let mut pad = [0u32; 4];
    for (k, p) in pad.iter_mut().enumerate().take(ch) {
        *p = c.kp.pad_pixel[k].to_bits();
    }
    let total: u64 = widths.iter().map(|&wl| heights.iter().map(|&hj| ((hj + make_mipmap::EXTRA_ROWS) * (wl + 3) * ch) as u64).sum::<u64>()).sum();
    if total == 0 || total >= (1u64 << 31) {
        return Err(unsup("ripmap larger than the kernel's index range"));
    }
    Ok(MmPlan { w, h, c: ch, wx, wy, filter, widths, heights, scale: a.scale, pad })
}

pub fn make_mipmap_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    let p = mm_plan(c, args)?;
    let (nl, np, ch) = (p.widths.len(), p.heights.len(), p.c);
    let stride = |l: usize| ((p.widths[l] + 3) * ch) as u32;
    let mut offs = Vec::with_capacity(nl * np);
    let mut at = 0u32;
    for l in 0..nl {
        for j in 0..np {
            offs.push(at);
            at += ((p.heights[j] + make_mipmap::EXTRA_ROWS) * (p.widths[l] + 3) * ch) as u32;
        }
    }
    let total = at as usize;
    // a constant source is the broadcast image the CPU builds
    let bcast;
    let src: &DevImage = match &args[0] {
        DevArg::Img(d) => d,
        DevArg::Const { lanes, .. } => {
            let im = Image::from_const(lanes, ch, c.n);
            bcast = DevImage::upload(s, &im.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>())?;
            &bcast
        }
        _ => return Err(unsup("the source is not an image")),
    };
    let rip = DevImage::alloc(s, 1, total)?;
    let rb = rip.plane(0);
    let padb = Buffer::from_slice(&s.context, &p.pad[..], Access::ReadOnly)?;
    let zero = (0..ch).all(|k| f32::from_bits(p.pad[k]) == 0.0);
    let sp = |k: usize| src.plane(k.min(ch - 1)).arg();
    s.launch("k2b_mm_fill", total, &[rb.arg(), ClArg::U32(total as u32)])?;
    s.launch("k2b_mm_base", c.n, &[sp(0), sp(1), sp(2), sp(3), rb.arg(), ClArg::I32(p.w as i32), ClArg::I32(p.h as i32), ClArg::I32(ch as i32), ClArg::U32(stride(0))])?;
    let padcols = |l: usize, j: usize, rows: usize| -> Result<(), DevError> {
        s.launch(
            "k2b_mm_padcols",
            rows * ch,
            &[rb.arg(), ClArg::U32(offs[l * np + j]), ClArg::I32(p.widths[l] as i32), ClArg::I32(ch as i32), ClArg::U32(stride(l)), ClArg::I32(rows as i32), ClArg::I32(p.wx), padb.arg()],
        )?;
        Ok(())
    };
    let padrows = |l: usize, j: usize| -> Result<(), DevError> {
        s.launch(
            "k2b_mm_padrows",
            3 * stride(l) as usize,
            &[rb.arg(), ClArg::U32(offs[l * np + j]), ClArg::I32(p.heights[j] as i32), ClArg::I32(ch as i32), ClArg::U32(stride(l)), ClArg::I32(p.wy), ClArg::I32(zero as i32), padb.arg()],
        )?;
        Ok(())
    };
    padcols(0, 0, p.h)?;
    padrows(0, 0)?;
    if p.filter != 0 {
        for l in 1..nl {
            s.launch(
                "k2b_mm_hor",
                p.widths[l] * ch * p.heights[0],
                &[rb.arg(), ClArg::U32(offs[(l - 1) * np]), ClArg::U32(stride(l - 1)), ClArg::U32(offs[l * np]), ClArg::U32(stride(l)), ClArg::I32(p.widths[l] as i32), ClArg::I32(p.heights[0] as i32), ClArg::I32(ch as i32)],
            )?;
            padcols(l, 0, p.heights[0])?;
            padrows(l, 0)?;
        }
        for j in 1..np {
            for l in 0..nl {
                s.launch(
                    "k2b_mm_ver",
                    p.widths[l] * ch * p.heights[j],
                    &[rb.arg(), ClArg::U32(offs[l * np + j - 1]), ClArg::U32(offs[l * np + j]), ClArg::U32(stride(l)), ClArg::I32(p.widths[l] as i32), ClArg::I32(p.heights[j] as i32), ClArg::I32(ch as i32)],
                )?;
                padcols(l, j, p.heights[j])?;
            }
            for l in 0..nl {
                padrows(l, j)?;
            }
        }
    }
    Ok(rip)
}

/// The device resident ripmap object for the buffer `make_mipmap_run` returned.
pub fn wrap_ripmap(c: &NodeCall, args: &[DevArg], out: DevImage) -> Result<DevMipVal, DevError> {
    let p = mm_plan(c, args)?;
    Ok(DevMipVal::new(out, p.w, p.h, p.c, p.filter, [p.wx, p.wy], p.widths, p.heights, p.scale))
}

// ---- blur_warp ----------------------------------------------------------------------------------------------------

pub fn blur_warp_accepts(c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    if shapes.len() != 8 {
        return Err("eight operands expected");
    }
    if !(1..=4).contains(&c.tag) {
        return Err("combination tag outside 1..4");
    }
    if shapes[0] != ArgShape::Mip {
        return Err("the first operand is not a ripmap");
    }
    for s in &shapes[1..3] {
        match s {
            ArgShape::Const { .. } | ArgShape::Img { channels: 2 } => {}
            _ => return Err("endpoint operand must be a constant or a 2 channel image"),
        }
    }
    if shapes[3..].iter().any(|s| !is_const(s)) {
        return Err("the parameter operands must be constants");
    }
    Ok(())
}

pub fn blur_warp_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    if args.len() != 8 {
        return Err(unsup("eight operands expected"));
    }
    let mipref = match &args[0] {
        DevArg::Mip(m) => crate::device_ops_k2::MipRef::Host(m),
        DevArg::DevMip(m) => crate::device_ops_k2::MipRef::Dev(m),
        _ => return Err(unsup("the first operand is not a ripmap")),
    };
    let (w, h) = c.dims;
    let n = c.n;
    if mipref.width() != w || mipref.height() != h || w * h != n || w == 0 || h == 0 || n >= u32::MAX as usize {
        return Err(unsup("ripmap built for another frame size"));
    }
    let ch = mipref.channels();
    if !(1..=4).contains(&ch) || ch as i32 != c.tag {
        return Err(unsup("tag differs from the ripmap channels"));
    }
    let (plane_len, w0, h0) = mipref.plane0_shape().ok_or_else(|| unsup("ripmap without level 0"))?;
    if plane_len == 0 || plane_len >= i32::MAX as usize || w >= (1 << 24) || h >= (1 << 24) || w0 >= (1 << 24) || h0 >= (1 << 24) {
        return Err(unsup("frame or ripmap plane outside the kernel's index range"));
    }
    let l = |i: usize, k: usize| lane(&args[i], k).ok_or_else(|| unsup("a parameter operand is not a constant"));
    let f_cc = l(3, 0)?;
    let assumption_a = l(3, 1)?;
    let assumption_b = l(4, 0)?;
    let mode_x = cvt_i32(l(5, 0)?);
    let mode_y = cvt_i32(l(5, 1)?);
    let flag_cell_a = cvt_i32(l(6, 0)?);
    l(7, 0)?;
    let endpoint = |a: &DevArg| -> Result<(bool, f32, f32, Option<usize>), DevError> {
        match a {
            DevArg::Const { lanes, .. } => Ok((true, f32::from_bits(lanes[0]), f32::from_bits(lanes[1]), None)),
            DevArg::Img(d) if d.channels() == 2 && d.n == n => Ok((false, 0.0, 0.0, Some(0))),
            _ => Err(unsup("endpoint operand must be a constant or a 2 channel image of the frame size")),
        }
    };
    let (p1c, p1x0, p1y0, _) = endpoint(&args[1])?;
    let (p2c, p2x0, p2y0, _) = endpoint(&args[2])?;
    let g = geom_of(c.kp, w, h);
    let cw = ch as i32;
    let stride = cw.wrapping_mul((w0 as i32).wrapping_add(3));
    let origin = cw as i64 + cw as i64 * (w0 as i64 + 3);
    let ip: Vec<i32> = vec![
        cw,
        plane_len as i32,
        origin as i32,
        stride,
        w as i32,
        h as i32,
        w0 as i32,
        h0 as i32,
        mode_x,
        mode_y,
        flag_cell_a,
        (mode_x == 2) as i32,
        (mode_y == 2) as i32,
        (mode_x == 1) as i32,
        (mode_y == 1) as i32,
        p1c as i32,
        p2c as i32,
        g.roi_x,
        g.roi_y,
        n as i32,
    ];
    let fp: Vec<f32> = vec![f_cc, assumption_a, assumption_a, 1.0, assumption_b, h as f32, w as f32, g.aspect, g.assumption_e, g.left, g.right, g.assumption_h, p1x0, p1y0, p2x0, p2y0];
    let ipb = Buffer::from_slice(&s.context, &ip[..], Access::ReadOnly)?;
    let fpb = Buffer::from_slice(&s.context, &fp[..], Access::ReadOnly)?;
    let uploaded = match &mipref {
        crate::device_ops_k2::MipRef::Host(m) => {
            let plane_f: Vec<f32> = m.ripmap.planes[m.ripmap.idx(0, 0)].iter().map(|b| f32::from_bits(*b)).collect();
            Some(DevImage::upload(s, &[plane_f.as_slice()])?)
        }
        crate::device_ops_k2::MipRef::Dev(_) => None,
    };
    let pb = match (&uploaded, &mipref) {
        (Some(u), _) => u.plane(0),
        (None, crate::device_ops_k2::MipRef::Dev(m)) => m.dev.plane(0),
        _ => unreachable!("a host ripmap is uploaded"),
    };
    let dm = Buffer::from_slice(&s.context, &[0f32], Access::ReadOnly)?;
    fn pl<'a>(a: &DevArg<'a>, k: usize, dm: &'a Buffer<f32>) -> ClArg<'a> {
        match a {
            DevArg::Img(d) => d.plane(k).arg(),
            _ => dm.arg(),
        }
    }
    let out = DevImage::alloc(s, ch, n)?;
    let err = ErrCell::new(s)?;
    let o = |k: usize| out.plane(k.min(ch - 1)).arg();
    s.launch(
        "k2b_blur_warp_f32",
        n,
        &[pb.arg(), ipb.arg(), fpb.arg(), pl(&args[1], 0, &dm), pl(&args[1], 1, &dm), pl(&args[2], 0, &dm), pl(&args[2], 1, &dm), o(0), o(1), o(2), o(3), err.arg()],
    )?;
    err.check(s, "a tap outside the ripmap plane (the CPU kernel reports it)")?;
    Ok(out)
}

// ---- the self-check -----------------------------------------------------------------------------------------------

pub enum Kb {
    Img(Image),
    Const(usize, [u32; 4]),
    Mip(Box<MipImage>),
}

pub struct KbCase {
    pub name: String,
    pub dims: (usize, usize),
    pub tag: i32,
    pub args: Vec<Kb>,
    pub kp: KParams,
    /// the device must run this case whenever the CPU kernel succeeds
    pub strict: bool,
}

fn cst(vals: &[f32]) -> Kb {
    let mut l = [0u32; 4];
    for (i, v) in vals.iter().enumerate().take(4) {
        l[i] = v.to_bits();
    }
    Kb::Const(vals.len(), l)
}

fn unit(r: &mut Rng) -> f32 {
    (r.next_u32() >> 8) as f32 / 16777216.0
}

fn span(r: &mut Rng, lo: f32, hi: f32) -> f32 {
    lo + (hi - lo) * unit(r)
}

fn plane_of(r: &mut Rng, n: usize, f: &mut dyn FnMut(&mut Rng) -> f32) -> Vec<f32> {
    (0..n).map(|_| f(r)).collect()
}

fn call_cpu(class: &str, tag: i32, args: &[Kb], dims: (usize, usize), kp: &KParams) -> Result<Out, crate::types::ExecError> {
    let cpu: Vec<Arg> = args
        .iter()
        .map(|a| match a {
            Kb::Img(i) => Arg::Img(i),
            Kb::Const(n, l) => Arg::Const { n: *n, lanes: *l },
            Kb::Mip(m) => Arg::Mip(m.as_ref()),
        })
        .collect();
    kernels::run_any(class, tag, &cpu, dims.0 * dims.1, dims, kp)
}

fn src_image(r: &mut Rng, ch: usize, n: usize, dirty: bool) -> Image {
    let sp = adversarial_f32(40, r.next_u64());
    let planes = (0..ch)
        .map(|_| {
            (0..n)
                .map(|_| {
                    if dirty && r.below(7) == 0 {
                        sp[r.next_u32() as usize % sp.len()]
                    } else {
                        span(r, -0.5, 1.5)
                    }
                })
                .collect()
        })
        .collect();
    Image { n, planes }
}

const MM_FRAMES: &[(usize, usize)] = &[(1, 1), (2, 1), (1, 3), (3, 5), (7, 2), (5, 8), (16, 9), (33, 2), (45, 23), (129, 3)];
const BW_FRAMES: &[(usize, usize)] = &[(1, 1), (2, 1), (1, 3), (3, 5), (7, 2), (16, 9), (33, 2), (45, 23)];

fn mm_cases() -> Vec<KbCase> {
    let mut r = Rng::new(0x5eed_00b1);
    let mut out = Vec::new();
    let pads: [[f32; 4]; 4] = [[0.25, -0.5, 2.0, 1.0], [0.0, 0.0, 0.0, 0.0], [-0.0, 0.0, -0.0, 0.0], [1.0e-40, -3.0e38, 0.5, 7.0]];
    let wraps: [(i32, i32); 10] = [(0, 0), (1, 1), (2, 2), (1, 2), (2, 1), (0, 1), (2, 0), (3, 3), (0, 3), (3, 1)];
    let mut k = 0usize;
    for &(w, h) in MM_FRAMES {
        let n = w * h;
        for ch in 1..=4usize {
            for &(wx, wy) in &wraps {
                k += 1;
                if (k + ch) % 3 != 0 && w * h > 60 {
                    continue;
                }
                let filter = [1.0f32, 0.0, 1.0, 2.0, 1.0][k % 5];
                let mut kp = KParams::standin();
                kp.pad_pixel = pads[k % pads.len()];
                let dirty = k % 4 == 3;
                let src = src_image(&mut r, ch, n, dirty);
                let args = if k % 11 == 10 { vec![cst(&[0.375, 2.0, -1.0, 4.0][..ch]), cst(&[wx as f32, wy as f32]), cst(&[filter])] } else { vec![Kb::Img(src), cst(&[wx as f32, wy as f32]), cst(&[filter])] };
                out.push(KbCase { name: format!("make_mipmap {w}x{h} ch{ch} wrap({wx},{wy}) filter{filter} pad{}", k % pads.len()), dims: (w, h), tag: ch as i32, args, kp, strict: !dirty });
            }
        }
    }
    out
}

fn mip_of(src: &Image, ch: usize, wrap: (i32, i32), dims: (usize, usize), kp: &KParams) -> Option<MipImage> {
    let args = [Kb::Img(src.clone()), cst(&[wrap.0 as f32, wrap.1 as f32]), cst(&[0.0])];
    match call_cpu("make_mipmap", ch as i32, &args, dims, kp) {
        Ok(Out::Mip(m)) => Some(m),
        _ => None,
    }
}

fn geom_variant(k: usize, w: usize) -> Option<crate::types::Geom> {
    use crate::types::Geom;
    match k % 3 {
        0 => None,
        1 => Some(Geom { left: -0.5, right: w as f32 - 0.5, aspect: 1.0, assumption_e: 0.25, assumption_h: 0.75, roi_x: 3, roi_y: -2 }),
        _ => Some(Geom { left: 1.5, right: w as f32 + 2.25, aspect: 0.75, assumption_e: -1.5, assumption_h: 0.3, roi_x: -5, roi_y: 7 }),
    }
}

fn bw_cases() -> Vec<KbCase> {
    let mut r = Rng::new(0x5eed_00b2);
    let mut out = Vec::new();
    let modes: [(i32, i32); 8] = [(0, 0), (1, 1), (2, 2), (1, 2), (2, 0), (0, 1), (2, 1), (3, 0)];
    let ramps: [(f32, f32); 5] = [(1.0, 1.0), (0.5, 1.0), (0.0, 1.0), (2.0, 0.25), (1.0, 3.0)];
    let mut k = 0usize;
    for &(w, h) in BW_FRAMES {
        let n = w * h;
        for ch in 1..=4usize {
            for &(mx, my) in &modes {
                k += 1;
                if w * h > 200 && k % 2 == 1 {
                    continue;
                }
                let mut kp = KParams::standin();
                kp.geom = geom_variant(k, w);
                kp.pad_pixel = [0.25, -0.5, 2.0, 1.0];
                let wrap = (mx.clamp(0, 2), my.clamp(0, 2));
                for kind in 0..6 {
                    // kind 0: image endpoints near the frame, 1: constant endpoints, 2: long segments, 3: image p1 and constant p2,
                    // 4: p2 equal to p1 on some samples (zero length), 5: adversarial values
                    let dirty = kind == 5;
                    let src = src_image(&mut r, ch, n, dirty && k % 2 == 0);
                    let Some(mip) = mip_of(&src, ch, wrap, (w, h), &kp) else { continue };
                    let sp = adversarial_f32(30, r.next_u64());
                    let coord = |r: &mut Rng, lo: f32, hi: f32| -> f32 {
                        if kind == 5 && r.below(3) == 0 {
                            sp[r.next_u32() as usize % sp.len()]
                        } else {
                            span(r, lo, hi)
                        }
                    };
                    let range = if kind == 2 { 3.0 } else { 1.3 };
                    let p1 = Image { n, planes: vec![plane_of(&mut r, n, &mut |r| coord(r, -range, range)), plane_of(&mut r, n, &mut |r| coord(r, -range, range))] };
                    let mut p2 = Image { n, planes: vec![plane_of(&mut r, n, &mut |r| coord(r, -range, range)), plane_of(&mut r, n, &mut |r| coord(r, -range, range))] };
                    if kind == 4 {
                        for i in 0..n {
                            if r.below(3) == 0 {
                                p2.planes[0][i] = p1.planes[0][i];
                                p2.planes[1][i] = p1.planes[1][i];
                            } else if r.below(3) == 0 {
                                // nearly equal: sub-pixel segment
                                p2.planes[0][i] = p1.planes[0][i] + span(&mut r, -1.0e-3, 1.0e-3);
                                p2.planes[1][i] = p1.planes[1][i] + span(&mut r, -1.0e-3, 1.0e-3);
                            }
                        }
                    }
                    let (e1, e2) = match kind {
                        1 => (cst(&[span(&mut r, -1.0, 1.0), span(&mut r, -1.0, 1.0)]), cst(&[span(&mut r, -1.0, 1.0), span(&mut r, -1.0, 1.0)])),
                        3 => (Kb::Img(p1), cst(&[span(&mut r, -1.0, 1.0), span(&mut r, -1.0, 1.0)])),
                        _ => (Kb::Img(p1), Kb::Img(p2)),
                    };
                    let (a0, a1) = ramps[(k + kind) % ramps.len()];
                    let flag = [1.0f32, 0.0, 2.0, 1.0][(k + kind) % 4];
                    let scale = [1.0f32, 0.5, 2.0][k % 3];
                    let args = vec![Kb::Mip(Box::new(mip)), e1, e2, cst(&[a0, a1]), cst(&[scale]), cst(&[mx as f32, my as f32]), cst(&[flag]), cst(&[0.0])];
                    out.push(KbCase { name: format!("blur_warp {w}x{h} ch{ch} mode({mx},{my}) kind{kind} flag{flag} ramp({a0},{a1}) scale{scale}"), dims: (w, h), tag: ch as i32, args, kp, strict: kind < 2 && mx <= 2 && my <= 2 });
                }
            }
        }
    }
    out
}

pub fn cases(class: &str) -> Vec<KbCase> {
    match class {
        "make_mipmap" => mm_cases(),
        "blur_warp" => bw_cases(),
        _ => Vec::new(),
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Coverage {
    pub compared: usize,
    pub declined_cpu_ok: usize,
    pub cpu_error_declined: usize,
}

fn note(tally: &mut Tally, t: Tally) {
    tally.outputs += t.outputs;
    if let (NanPayloads::Same, NanPayloads::Differ(m)) = (&tally.nan_payloads, t.nan_payloads) {
        tally.nan_payloads = NanPayloads::Differ(m);
    }
}

/// Bit comparison of two word arrays: equal words pass, two NaNs with other bits are a payload difference (the first is kept),
/// anything else is a mismatch.
fn cmp_words(name: &str, what: &str, want: &[u32], got: &[u32], tally: &mut Tally) -> Result<(), String> {
    if want.len() != got.len() {
        return Err(format!("make_mipmap {name} {what}: length {} on the CPU, {} on the device", want.len(), got.len()));
    }
    let is_nan = |b: u32| (b & 0x7f80_0000) == 0x7f80_0000 && (b & 0x007f_ffff) != 0;
    for (i, (&e, &g)) in want.iter().zip(got).enumerate() {
        if e == g {
            continue;
        }
        let m = format!("make_mipmap {name} {what}: word {i} expected 0x{e:08x}, got 0x{g:08x}");
        if is_nan(e) && is_nan(g) {
            if tally.nan_payloads == NanPayloads::Same {
                tally.nan_payloads = NanPayloads::Differ(Mismatch { kernel: "make_mipmap".into(), case: name.into(), index: i, input: what.into(), expected: format!("0x{e:08x}"), got: format!("0x{g:08x}") });
            }
        } else {
            return Err(m);
        }
    }
    tally.outputs += want.len();
    Ok(())
}

/// Bit comparison of a device ripmap against the CPU's.
fn compare_mip(name: &str, want: &MipImage, got: &MipImage, tally: &mut Tally) -> Result<(), String> {
    if want.ripmap.widths != got.ripmap.widths || want.ripmap.heights != got.ripmap.heights || want.channels != got.channels || want.scale != got.scale || want.filter != got.filter || want.wrap != got.wrap {
        return Err(format!("{name}: ripmap layout differs"));
    }
    for (pi, (wp, gp)) in want.ripmap.planes.iter().zip(&got.ripmap.planes).enumerate() {
        cmp_words(name, &format!("plane {pi}"), wp, gp, tally)?;
    }
    if want.base.planes.len() != got.base.planes.len() {
        return Err(format!("{name}: base planes differ in count"));
    }
    for (p, (w, g)) in want.base.planes.iter().zip(&got.base.planes).enumerate() {
        let wb: Vec<u32> = w.iter().map(|v| v.to_bits()).collect();
        let gb: Vec<u32> = g.iter().map(|v| v.to_bits()).collect();
        cmp_words(name, &format!("base {p}"), &wb, &gb, tally)?;
    }
    Ok(())
}

pub fn selfcheck_cov(session: &Arc<Session>, op: &DeviceOp) -> Result<(Tally, Coverage), String> {
    let mut tally = Tally { outputs: 0, nan_payloads: NanPayloads::Same };
    let mut cov = Coverage::default();
    for case in cases(op.class) {
        let n = case.dims.0 * case.dims.1;
        let call = NodeCall { idx: 0, class: op.class, tag: case.tag, n, dims: case.dims, kp: &case.kp };
        let shapes: Vec<ArgShape> = case
            .args
            .iter()
            .map(|a| match a {
                Kb::Img(i) => ArgShape::Img { channels: i.channels() },
                Kb::Const(n, _) => ArgShape::Const { n: *n },
                Kb::Mip(_) => ArgShape::Mip,
            })
            .collect();
        if let Err(why) = (op.accepts)(&call, &shapes) {
            return Err(format!("{} {}: the corpus case is refused by accepts: {why}", op.class, case.name));
        }
        let want = call_cpu(op.class, case.tag, &case.args, case.dims, &case.kp);
        let uploads: Result<Vec<Option<DevImage>>, DevError> = case
            .args
            .iter()
            .map(|a| match a {
                Kb::Img(i) => DevImage::upload(session, &i.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>()).map(Some),
                _ => Ok(None),
            })
            .collect();
        let uploads = uploads.map_err(|e| format!("{} {}: upload: {e}", op.class, case.name))?;
        let dargs: Vec<DevArg> = case
            .args
            .iter()
            .zip(&uploads)
            .map(|(a, u)| match (a, u) {
                (Kb::Img(_), Some(d)) => DevArg::Img(d),
                (Kb::Const(n, l), _) => DevArg::Const { n: *n, lanes: *l },
                (Kb::Mip(m), _) => DevArg::Mip(m.as_ref()),
                _ => unreachable!("an image argument has an upload"),
            })
            .collect();
        let got = (op.run)(session, &call, &dargs);
        match (want, got) {
            (Ok(Out::Img(w)), Ok(d)) => {
                let planes = d.download().map_err(|e| format!("{} {}: {e}", op.class, case.name))?;
                if planes.len() != w.planes.len() {
                    return Err(format!("{} {}: {} planes on the device, {} on the CPU", op.class, case.name, planes.len(), w.planes.len()));
                }
                for (p, (g, wp)) in planes.iter().zip(&w.planes).enumerate() {
                    match compare_f32(op.class, &format!("{} plane {p}", case.name), |i| format!("plane {p} sample {i}"), wp, g) {
                        Ok(t) => note(&mut tally, t),
                        Err(m @ Mismatch { .. }) => return Err(m.to_string()),
                    }
                }
                cov.compared += 1;
            }
            (Ok(Out::Mip(w)), Ok(d)) => {
                let val = wrap_ripmap(&call, &dargs, d).map_err(|e| format!("{} {}: {e}", op.class, case.name))?;
                let host = val.materialize().map_err(|e| format!("{} {}: {e:?}", op.class, case.name))?;
                compare_mip(&case.name, &w, host, &mut tally)?;
                cov.compared += 1;
            }
            (Ok(_), Err(DevError::Unsupported(why))) => {
                if case.strict {
                    return Err(format!("{} {}: the device declined a case the CPU runs: {why}", op.class, case.name));
                }
                cov.declined_cpu_ok += 1;
            }
            (Ok(_), Err(e)) => return Err(format!("{} {}: {e}", op.class, case.name)),
            (Err(_), Err(DevError::Unsupported(_))) => cov.cpu_error_declined += 1,
            (Err(_), Ok(_)) => return Err(format!("{} {}: the CPU kernel fails but the device produced an output", op.class, case.name)),
            (Err(_), Err(e)) => return Err(format!("{} {}: {e}", op.class, case.name)),
        }
    }
    if cov.compared == 0 {
        return Err(format!("{}: no corpus case ran on the device", op.class));
    }
    Ok((tally, cov))
}

pub fn selfcheck(session: &Arc<Session>, op: &DeviceOp) -> Result<Tally, String> {
    selfcheck_cov(session, op).map(|r| r.0)
}
