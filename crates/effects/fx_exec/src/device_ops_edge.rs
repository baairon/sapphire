//! Device op `edge_detect` (S_GlowEdges): the blur of `edge_detect::render_rows` through the neighbourhood blur route, then the
//! gradient body kernel `edge_grad_f32` of `fx_device::kernels_edge`.
//!
//! The host computes what `kernels2::edge_detect_k` computes before its call: the aspect block, the `EdgeParams`, the blur scalars
//! (`edge_detect::sigma_gain`) and the body factors (`edge_detect::body_for`). `accepts` refuses what `edge_detect_k` refuses by
//! name (a source of 2 or 3 elements, width 1, non-constant parameters); the radius and edge mode refusals come from the blur
//! route as `Unsupported`, so the CPU path then reports its own result.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::device_ops_neighbourhood::{blur_dev, cst, edge_of, frame_ok, img, is_const, lane, plane_channels, BlurJob};
use crate::kernels2::cvt_i32;
use crate::types::{Image, KParams};
use fx_device::cl::Arg as ClArg;
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

pub const EDGE_DETECT: DeviceOp = DeviceOp { class: "edge_detect", accepts: edge_accepts, run: edge_run, corpus: edge_corpus };

fn edge_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 4 {
        return Err("four operands expected");
    }
    if c.tag != 1 {
        return Err("source of 2 or 3 elements (combo rows 1 and 2): only the one-element row is ported");
    }
    if c.dims.0 < 2 {
        return Err("width 1: the body reads one float past the plane (host memory)");
    }
    match plane_channels(&a[0]) {
        Some(ch) if ch >= 1 => {}
        Some(_) => return Err("fewer channels than the combination tag"),
        None => return Err("the source is not a plain image"),
    }
    if !a[1..].iter().all(is_const) {
        return Err("amount, gain and subpixel must be constants");
    }
    Ok(())
}

fn edge_params(kp: &KParams, w: usize, amount: f32, gain: f32, sub: i32) -> edge_detect::EdgeParams {
    let a_eff = if crate::aspect::in_caller_range(kp.host_aspect) { kp.host_aspect } else { crate::aspect::FALLBACK_A };
    let asp_blk = crate::aspect::set_aspect(a_eff, 0.0).expect("in-range aspect is nonzero");
    edge_detect::EdgeParams {
        amount,
        gain_param: gain,
        subpixel: sub,
        edge_mode: kp.blur_edge,
        render_scale: if kp.blur_render_scale > 0 { kp.blur_render_scale } else { w as i32 },
        aspect: if asp_blk.aspect_block_b != 0 { Some(f32::from_bits(asp_blk.aspect_block_d)) } else { None },
        info3c: asp_blk.aspect_block_e != 0,
        flag_cell: 1,
        past_end: 0.0,
    }
}

fn edge_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    let (w, h) = c.dims;
    let src_op = img(s, &a[0], c.n)?;
    let src = src_op.get();
    if src.n != w * h || src.channels() < 1 {
        return Err(DevError::Unsupported("source smaller than the frame".into()));
    }
    let p = edge_params(c.kp, w, lane(&a[1], 0)?, lane(&a[2], 0)?, cvt_i32(lane(&a[3], 0)?));
    let edge = edge_of(p.edge_mode)?;
    let (sigma, gain) = edge_detect::sigma_gain(&p);
    let bd = edge_detect::body_for(&p);
    let scr = DevImage::alloc(s, 1, 3 * c.n)?;
    let blurred = DevImage::alloc(s, 1, c.n)?;
    let job = BlurJob { w, h, sx: sigma, sy: sigma, gain, edge, passes: 3, subpixel: p.subpixel != 0, aspect: p.aspect };
    blur_dev(s, &scr, src.plane(0), blurred.plane(0), &job)?;
    let out = DevImage::alloc(s, 1, c.n)?;
    s.launch(
        "edge_grad_f32",
        c.n,
        &[blurred.plane(0).arg(), out.plane(0).arg(), ClArg::U32(w as u32), ClArg::U32(h as u32), ClArg::F32(bd.c), ClArg::F32(bd.c + bd.c), ClArg::F32(bd.k)],
    )?;
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// self-check corpus

/// Frame shapes: squares and rectangles, plus single row frames.
const DIMS_EDGE: &[(usize, usize)] = &[(2, 2), (3, 3), (7, 7), (8, 8), (9, 9), (31, 31), (33, 33), (64, 32), (50, 7), (257, 2), (2, 257), (2, 1), (5, 1), (3, 2)];

fn special_plane(n: usize, seed: usize) -> Vec<f32> {
    let vals = adversarial_f32(4000, 41);
    (0..n).map(|i| vals[(i * 7 + seed * 13 + n) % vals.len()]).collect()
}

fn plain_plane(n: usize, seed: usize) -> Vec<f32> {
    let mut s = 0x9e37_79b9_7f4a_7c15u64 ^ (seed as u64).wrapping_mul(0x1234_5678_9abc_def1);
    (0..n)
        .map(|_| {
            s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((s >> 40) as f32 / (1u64 << 24) as f32) * 1.5 - 0.25
        })
        .collect()
}

/// A plane with signed zeros, denormals, infinities and NaN of both signs and several payloads between finite values.
fn nasty_plane(n: usize, seed: usize) -> Vec<f32> {
    const W: [u32; 16] = [
        0x8000_0000, 0x0000_0000, 0x0000_0001, 0x8000_0001, 0x007f_ffff, 0x807f_ffff, 0x7f80_0000, 0xff80_0000, 0x7f80_0001, 0xffc0_0000, 0x7fc0_0002,
        0xff80_0003, 0x7f7f_ffff, 0xff7f_ffff, 0x3f80_0000, 0xbf80_0000,
    ];
    let base = plain_plane(n, seed + 99);
    (0..n).map(|i| if (i * 5 + seed) % 3 == 0 { f32::from_bits(W[(i * 11 + seed * 3) % 16]) } else { base[i] }).collect()
}

fn edge_corpus() -> Vec<SelfCase> {
    // (amount, gain, subpixel, edge mode, render scale override, host aspect)
    type P = (f32, f32, f32, i32, i32, f32);
    let sets: [P; 12] = [
        (0.0, 1.0, 0.0, 1, 20, 2.0),
        (0.0, 2.0, 1.0, 0, 0, 0.0),
        (0.0, 0.0, 0.0, 1, 0, 0.0),
        (0.5, 1.0, 0.0, 1, 0, 0.0),
        (0.5, 0.7, 1.0, 0, 0, 0.0),
        (1.3, 0.7, 1.0, 1, 0, 0.0),
        (3.0, 2.0, 0.0, 0, 0, 0.0),
        (0.4, 0.0, 0.0, 1, 0, 0.0),
        (2.5, 1.0, 1.0, 1, 100, 0.75),
        (8.0, 1.5, 1.0, 0, 40, 0.0),
        (-1.0, 1.0, 0.0, 1, 0, 0.0),
        (0.9, -2.0, 1.0, 0, 0, 2.0),
    ];
    let mut out = Vec::new();
    for (di, &(w, h)) in DIMS_EDGE.iter().enumerate() {
        let n = w * h;
        for (si, p) in sets.iter().enumerate() {
            // small frames take only the pass-through blur; the others every set
            if (w <= 3 || h == 1) && si >= 3 {
                continue;
            }
            if n > 2000 && (si + di) % 2 == 1 {
                continue;
            }
            for kind in 0..3usize {
                if kind == 2 && si % 2 == 1 {
                    continue;
                }
                let plane = match kind {
                    0 => plain_plane(n, di * 17 + si),
                    1 => special_plane(n, di * 17 + si),
                    _ => nasty_plane(n, di * 17 + si),
                };
                let mut kp = KParams::standin();
                kp.blur_edge = p.3;
                kp.blur_render_scale = p.4;
                if p.5 != 0.0 {
                    kp.host_aspect = p.5;
                }
                out.push(SelfCase {
                    name: format!("{w}x{h} set {si} kind {kind}"),
                    dims: (w, h),
                    tag: 1,
                    args: vec![Image { n, planes: vec![plane] }],
                    consts: vec![cst(1, &[p.0]), cst(2, &[p.1]), cst(3, &[p.2])],
                    kp: Some(kp),
                });
            }
        }
    }
    out
}
