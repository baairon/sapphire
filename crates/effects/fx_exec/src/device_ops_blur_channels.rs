//! blur_channels on the device: one blur route launch per plane, each with that plane's sigma pair and gain.
//!
//! The host computes every parameter with the crate's own functions (`blur_channels::channel_sigmas`, `channel_gain`,
//! `blur::pass_count`), so the arithmetic outside `blur_core` is the CPU's. The launches are the blur route's (`blur_dev`).
//! A plane at or beyond the source plane count reads the last supplied source plane (the copy rule of `blur_channels::render`).

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::device_ops_neighbourhood::{blur_dev, const_n, cst, edge_of, frame_ok, img, is_const, kp_blur, lane, plane_channels, render_scale, BlurJob};
use crate::kernels2::cvt_i32;
use crate::types::Image;
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

pub const BLUR_CHANNELS: DeviceOp = DeviceOp { class: "blur_channels", accepts: accepts, run: run, corpus: corpus };

/// Test hook: a single-edit stand-in for the host route (0 is the real route). Never set outside the unit tests.
#[cfg(test)]
thread_local! {
    pub(crate) static MUTANT: std::cell::Cell<u8> = const { std::cell::Cell::new(0) };
}

#[cfg(test)]
fn mutant() -> u8 {
    MUTANT.with(|m| m.get())
}

#[cfg(not(test))]
fn mutant() -> u8 {
    0
}

fn accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 6 {
        return Err("six operands expected");
    }
    if !(1..=4).contains(&c.tag) {
        return Err("combination tag outside 1..4");
    }
    match plane_channels(&a[0]) {
        Some(ch) if ch >= 1 => {}
        Some(_) => return Err("a source without planes"),
        None => return Err("the source is not a plain image"),
    }
    if !a[1..].iter().all(is_const) {
        return Err("amount, channel amount, filter shape, scale and subpixel must be constants");
    }
    if !(0..=1).contains(&c.kp.blur_edge) {
        return Err("edge mode other than 0 and 1 (the CPU reports an error)");
    }
    Ok(())
}

fn run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    let m = mutant();
    let src_op = img(s, &a[0], c.n)?;
    let src = src_op.get();
    let (w, h) = c.dims;
    let planes = c.tag.max(0) as usize;
    if planes == 0 || planes > 4 || src.channels() == 0 || src.n != w * h {
        return Err(DevError::Unsupported("blur_channels: source without planes or not the frame size".into()));
    }
    let n_in = src.channels().min(planes);
    let wid = [lane(&a[2], 0)?, lane(&a[2], 1)?, lane(&a[2], 2)?, lane(&a[2], 3)?];
    let gain = [lane(&a[4], 0)?, lane(&a[4], 1)?, lane(&a[4], 2)?, lane(&a[4], 3)?];
    let mut cp = blur_channels::ChannelParams {
        amount: [lane(&a[1], 0)?, lane(&a[1], 1)?],
        widths: wid,
        gains: gain,
        filter_shape: cvt_i32(lane(&a[3], 0)?),
        scale_count: const_n(&a[4]) as i32,
        subpixel: cvt_i32(lane(&a[5], 0)?),
        edge_mode: c.kp.blur_edge,
        plane_count: planes,
        n_in,
        render_scale: render_scale(c.kp, w),
        aspect: c.kp.blur_aspect,
    };
    if m == 8 {
        cp.render_scale += 1;
    }
    if m == 9 {
        cp.aspect = None;
    }
    let mut edge = edge_of(cp.edge_mode)?;
    if m == 6 {
        edge = 1 - edge;
    }
    let mut passes = blur::pass_count(cp.filter_shape);
    if m == 5 {
        passes = if passes < 3 { passes + 1 } else { passes - 1 };
    }
    let out = DevImage::alloc(s, planes, c.n)?;
    let scr = DevImage::alloc(s, 1, 3 * c.n)?;
    let last = n_in - 1;
    for p in 0..planes {
        // the plane whose parameters this launch uses
        let q = match m {
            1 => 0,
            2 => (p + 1) % planes,
            _ => p,
        };
        let (mut sx, mut sy) = blur_channels::channel_sigmas(&cp, q);
        let mut g = blur_channels::channel_gain(&cp, q);
        if m == 4 {
            g = blur_channels::channel_gain(&cp, 0);
        }
        if m == 10 {
            std::mem::swap(&mut sx, &mut sy);
        }
        let mut sp = cp.subpixel != 0;
        if m == 7 {
            sp = !sp;
        }
        let mut sidx = if p < n_in { p } else { last };
        if m == 3 && p >= n_in && last > 0 {
            sidx = last - 1;
        }
        let job = BlurJob { w, h, sx, sy, gain: g, edge, passes, subpixel: sp, aspect: cp.aspect };
        blur_dev(s, &scr, src.plane(sidx), out.plane(p), &job)?;
    }
    Ok(out)
}

/// Dimensions of the corpus.
const DIMS: &[(usize, usize)] = &[(1, 1), (2, 1), (7, 3), (33, 2), (64, 32), (50, 7), (257, 1)];

fn values(n: usize, seed: usize, kind: usize) -> Vec<f32> {
    match kind {
        0 => {
            let mut s = 0x9e37_79b9_7f4a_7c15u64 ^ (seed as u64).wrapping_mul(0x1234_5678_9abc_def1);
            (0..n)
                .map(|_| {
                    s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    ((s >> 40) as f32 / (1u64 << 24) as f32) * 1.5 - 0.25
                })
                .collect()
        }
        1 => {
            let v = adversarial_f32(4000, 41);
            (0..n).map(|i| v[(i * 7 + seed * 13 + n) % v.len()]).collect()
        }
        _ => {
            // NaNs of both signs and payloads between finite values, infinities and signed zeros
            const W: [u32; 12] = [0x7f80_0001, 0xffc0_0000, 0x3f00_0000, 0x7f80_0000, 0xff80_0000, 0x8000_0000, 0x0000_0001, 0x7fc0_0002, 0x3e80_0000, 0xff80_0003, 0x0000_0000, 0xbf00_0000];
            (0..n).map(|i| f32::from_bits(W[(i * 5 + seed) % W.len()])).collect()
        }
    }
}

fn corpus() -> Vec<SelfCase> {
    // (amount x, amount y, widths, width lanes, shape, scale lanes, scale count, subpixel, edge, render scale override, aspect)
    type P = (f32, f32, [f32; 4], usize, f32, [f32; 4], usize, f32, i32, i32, Option<f32>);
    let sets: [P; 11] = [
        (0.0, 0.0, [1.0, 1.0, 1.0, 1.0], 4, 2.0, [1.0, 1.0, 1.0, 1.0], 4, 0.0, 1, 0, None),
        (0.4, 0.3, [1.0, 1.5, 0.5, 2.0], 4, 1.0, [1.0, 0.5, 2.0, 1.0], 4, 1.0, 1, 0, None),
        (1.7, 0.9, [0.0, 1.0, 0.25, 1.0], 4, 0.0, [0.75, 1.0, 1.0, 1.0], 1, 1.0, 0, 0, None),
        (0.6, 0.6, [1.0, 1.2, 1.39999, 0.0], 3, 3.0, [2.0, 2.0, 2.0, 2.0], 3, 0.0, 1, 0, None),
        (-1.0, -2.0, [1.0, 0.5, 1.0, 1.0], 2, 2.0, [1.5, 1.0, 1.0, 1.0], 2, 1.0, 1, 0, None),
        (0.6, 0.6, [1.0, 1.0, 1.0, 1.0], 4, 1.0, [0.0, 0.0, 0.0, 0.0], 4, 0.0, 0, 0, None),
        (8.0, 6.0, [1.0, 0.5, 0.25, 1.0], 4, 2.0, [1.0, 0.5, 0.25, 1.0], 4, 1.0, 1, 400, Some(0.75)),
        (5.9, 8.0, [-1.0, 2.0, 0.5, 1.0], 4, -1.0, [1.0, 1.0, 1.0, 1.0], 4, 0.0, 0, 400, None),
        (0.5, 0.5, [0.3, 0.3, 0.3, 0.3], 1, 4.0, [1.0, 1.0, 1.0, 1.0], 1, 1.0, 1, 0, None),
        (0.9, 1.1, [1.0, 0.5, 1.5, 1.0], 4, 2.0, [1.0, 1.0, 1.0, 1.0], 4, 1.0, 1, 0, Some(0.5)),
        (1.2, 0.7, [1.0, 2.0, 0.5, 1.0], 4, 1.0, [1.0, 1.0, 1.0, 1.0], 4, 0.0, 0, 0, Some(2.0)),
    ];
    let mut out = Vec::new();
    for (di, &(w, h)) in DIMS.iter().enumerate() {
        let n = w * h;
        for si in [1usize, 3, 9, 6, 2, 4, 7, 10, 0, 5, 8] {
            let p = &sets[si];
            for tag in 1usize..=4 {
                for src_planes in 1usize..=4 {
                    // sources of 1 to 4 planes: all against tag 4 and 3, the rest sampled
                    if (di + si + tag + src_planes) % 3 != 0 && !(tag >= 3 && src_planes <= 2) {
                        continue;
                    }
                    if n > 1000 && (si + tag + src_planes) % 2 == 1 {
                        continue;
                    }
                    let kind = (si + src_planes) % 3;
                    let mut kp = kp_blur(p.8, p.9);
                    kp.blur_aspect = p.10;
                    let img = Image { n, planes: (0..src_planes).map(|k| values(n, di * 31 + si * 5 + k, kind)).collect() };
                    out.push(SelfCase {
                        name: format!("{w}x{h} set {si} tag {tag} src {src_planes} kind {kind}"),
                        dims: (w, h),
                        tag: tag as i32,
                        args: vec![img],
                        consts: vec![cst(1, &[p.0, p.1]), cst(2, &p.2[..p.3]), cst(3, &[p.4]), cst(4, &p.5[..p.6]), cst(5, &[p.7])],
                        kp: Some(kp),
                    });
                }
            }
        }
    }
    out
}

#[cfg(test)]
#[path = "device_ops_blur_channels_tests.rs"]
mod tests;
