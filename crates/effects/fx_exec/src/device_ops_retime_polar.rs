//! Device ops `xy_to_polar` (`kernels2::xy_to_polar_k`, crate `xy_to_polar`) and `hsv_to_rgb` (`kernels2::hsv_to_rgb_k`, crate `hsv_to_rgb`).
//!
//! One work item per element. `xy_to_polar`: the angle is `atan2` (the polynomial form for lanes whose two magnitudes are in range,
//! the double precision pair form for every other lane) reduced by `floor`; the magnitude is the exact square root form outside the
//! vector body and the modelled reciprocal square root form inside it. The body range `[lo, hi)` is the crate's own `vector_body` of
//! the element count and `KParams::polar_mag_phase`, computed on the host. A lane whose components take the table branch of the
//! scalar atan2 (the crate's `Refusal::Atan2General`) raises a flag word; `run` reads it back and returns `DevError::Unsupported`, so
//! the executor runs the node on the CPU, which produces the named error. `hsv_to_rgb`: the plane form of `hsv_to_rgb::pixel`.
//! Shapes the CPU wrapper refuses by name (combination tag, constant operand, channel count) are refused in `accepts`.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::device_ops_pointwise::const_image;
use crate::types::{Image, KParams};
use fx_device::ocl::{Access, Arg as ClArg, Buffer};
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

pub const XY_TO_POLAR: DeviceOp = DeviceOp { class: "xy_to_polar", accepts: xy_accepts, run: xy_run, corpus: xy_corpus };
pub const HSV_TO_RGB: DeviceOp = DeviceOp { class: "hsv_to_rgb", accepts: hsv_accepts, run: hsv_run, corpus: hsv_corpus };

/// The reason `run` gives when a lane takes the branch the crate refuses.
pub const ATAN2_REFUSAL: &str = "ATAN2-GENERAL-NOT-PORTED: a lane takes the table branch of the scalar atan2";

fn unsup<T>(why: &str) -> Result<T, DevError> {
    Err(DevError::Unsupported(why.to_string()))
}

fn frame_ok(c: &NodeCall) -> Result<(), &'static str> {
    if c.n == 0 || c.n > u32::MAX as usize || c.dims.0.checked_mul(c.dims.1) != Some(c.n) {
        return Err("the frame size does not match the sample count or does not fit 32 bits");
    }
    Ok(())
}

fn xy_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 2 {
        return Err("two operands expected");
    }
    if c.tag != 2 {
        return Err("combination tag other than 2");
    }
    match a[0] {
        ArgShape::Img { channels: 2 } => {}
        _ => return Err("the vector operand must be a 2 channel image"),
    }
    match a[1] {
        ArgShape::Const { .. } => Ok(()),
        _ => Err("the dir_offset operand must be a constant"),
    }
}

/// Launches `rtp_xy_to_polar` over `n` pixels of the two plane image `src` (x then y); the result holds the angle plane and the
/// magnitude plane. `dir_bits` are the bits of the dir_offset lane and `mag_phase` is `KParams::polar_mag_phase`.
/// `Err(DevError::Unsupported)` when a lane takes the refused atan2 branch.
pub fn xy_to_polar_launch(s: &Arc<Session>, src: &DevImage, n: usize, dir_bits: u32, mag_phase: u32) -> Result<DevImage, DevError> {
    if src.channels() != 2 || src.n != n || n == 0 || n > u32::MAX as usize {
        return unsup("xy_to_polar takes a two plane image of the frame size");
    }
    let (lo, hi) = xy_to_polar::vector_body(n, mag_phase as usize);
    let out = DevImage::alloc(s, 2, n)?;
    let flag: Buffer<u32> = Buffer::from_slice(&s.context, &[0u32], Access::ReadWrite)?;
    s.launch(
        "rtp_xy_to_polar",
        n,
        &[
            src.plane(0).arg(),
            src.plane(1).arg(),
            out.plane(0).arg(),
            out.plane(1).arg(),
            ClArg::U32(n as u32),
            ClArg::U32(dir_bits),
            ClArg::U32(lo as u32),
            ClArg::U32(hi as u32),
            flag.arg(),
        ],
    )?;
    if s.queue.read(&flag)?[0] != 0 {
        return unsup(ATAN2_REFUSAL);
    }
    Ok(out)
}

fn xy_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    if a.len() != 2 || c.tag != 2 {
        return unsup("xy_to_polar takes two operands and the combination tag 2");
    }
    match (&a[0], &a[1]) {
        (DevArg::Img(src), DevArg::Const { lanes, .. }) => xy_to_polar_launch(s, src, c.n, lanes[0], c.kp.polar_mag_phase),
        _ => unsup("operands must be an image and a constant"),
    }
}

fn hsv_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    frame_ok(c)?;
    if a.len() != 1 {
        return Err("one operand expected");
    }
    if c.tag != 3 {
        return Err("combination tag other than 3");
    }
    match a[0] {
        ArgShape::Img { channels: 3 } => Ok(()),
        ArgShape::Const { .. } => Err("the constant input form is not wired"),
        _ => Err("the operand must be a 3 channel image"),
    }
}

/// Launches `rtp_hsv_to_rgb` over `n` pixels of the three plane image `src` (hue, saturation, value).
pub fn hsv_to_rgb_launch(s: &Arc<Session>, src: &DevImage, n: usize) -> Result<DevImage, DevError> {
    if src.channels() != 3 || src.n != n || n == 0 || n > u32::MAX as usize {
        return unsup("hsv_to_rgb takes a three plane image of the frame size");
    }
    let out = DevImage::alloc(s, 3, n)?;
    s.launch(
        "rtp_hsv_to_rgb",
        n,
        &[src.plane(0).arg(), src.plane(1).arg(), src.plane(2).arg(), out.plane(0).arg(), out.plane(1).arg(), out.plane(2).arg(), ClArg::U32(n as u32)],
    )?;
    Ok(out)
}

fn hsv_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    if a.len() != 1 || c.tag != 3 {
        return unsup("hsv_to_rgb takes one operand and the combination tag 3");
    }
    match &a[0] {
        DevArg::Img(src) => hsv_to_rgb_launch(s, src, c.n),
        _ => unsup("the operand must be an image"),
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------
// corpus

pub struct Lcg(pub u64);
impl Lcg {
    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

/// Values for the vector operand: the adversarial specials, one value per binary exponent in both signs (subnormals included), the
/// edges of the vector range (`2^-125`, `2^122` and their neighbours), and mantissa patterns.
pub fn polar_pool() -> Vec<f32> {
    let mut v = adversarial_f32(0, 1);
    for e in 0..=254u32 {
        for m in [0u32, 0x1, 0x400000, 0x7fffff, 0x2aaaaa] {
            for s in [0u32, 0x8000_0000] {
                v.push(f32::from_bits(s | (e << 23) | m));
            }
        }
    }
    for j in 0..23 {
        v.push(f32::from_bits(1 << j));
        v.push(f32::from_bits(0x8000_0000 | (1 << j)));
    }
    // 2^-125, 2^122, 2^-126 and 2^123 as exponent fields 2, 249, 1 and 250
    for edge in [2u32, 249, 1, 250].map(|e| f32::from_bits(e << 23)) {
        v.extend([edge, edge.next_up(), edge.next_down(), -edge, -edge.next_up()]);
    }
    v
}

/// NaN words of both signs: quiet, signalling and payload variants.
pub fn nan_words() -> Vec<f32> {
    let mut v = Vec::new();
    for s in [0u32, 0x8000_0000] {
        for m in [0x0040_0000u32, 0x0040_0001, 0x007f_ffff, 0x0000_0001, 0x0012_3456, 0x0040_0ffe, 0x0020_0000] {
            v.push(f32::from_bits(s | 0x7f80_0000 | m));
        }
    }
    v
}

/// One pair for the corpus: mostly two pool values; sometimes equal magnitudes, two NaN words, or a lane outside the vector range
/// with a partner whose binary exponent lies at a distance around the thresholds of the scalar atan2 (the branches at 54 and 74).
pub fn gen_pair(r: &mut Lcg, pool: &[f32]) -> (f32, f32) {
    match r.next() % 16 {
        0 => {
            let x = pool[(r.next() as usize) % pool.len()];
            (x, if r.next() & 1 == 0 { x } else { -x })
        }
        1 => {
            let nan = nan_words();
            (nan[(r.next() as usize) % nan.len()], nan[(r.next() as usize) % nan.len()])
        }
        2 | 3 | 4 | 5 => {
            let outer = [0u32, 1, 250, 251, 252, 253, 254];
            let outer_exp = outer[(r.next() as usize) % outer.len()] as i32;
            let ds = [-75i32, -56, -55, -54, -53, -52, -40, 0, 40, 52, 53, 54, 55, 56, 72, 73, 74, 75, 100];
            let inner_exp = (outer_exp + ds[(r.next() as usize) % ds.len()]).clamp(1, 254) as u32;
            let m = |r: &mut Lcg| (r.next() as u32) & 0x7f_ffff;
            let sg = |r: &mut Lcg| ((r.next() & 1) as u32) << 31;
            let a = f32::from_bits(sg(r) | ((outer_exp as u32) << 23) | m(r));
            let b = f32::from_bits(sg(r) | (inner_exp << 23) | m(r));
            if r.next() & 1 == 0 { (a, b) } else { (b, a) }
        }
        _ => (pool[(r.next() as usize) % pool.len()], pool[(r.next() as usize) % pool.len()]),
    }
}

/// `n` pairs `(x, y)` that may take the refused atan2 branch (the refusal path of the compare).
pub fn polar_pairs_raw(n: usize, seed: u64, pool: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mut r = Lcg(seed ^ 0x9e37_79b9_7f4a_7c15);
    let (mut xs, mut ys) = (Vec::with_capacity(n), Vec::with_capacity(n));
    for _ in 0..n {
        let (x, y) = gen_pair(&mut r, pool);
        xs.push(x);
        ys.push(y);
    }
    (xs, ys)
}

/// `n` pairs `(x, y)`, none of which takes the refused atan2 branch (filtered with the crate's own `atan2_lane`).
pub fn polar_pairs(n: usize, seed: u64, pool: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mut r = Lcg(seed ^ 0x9e37_79b9_7f4a_7c15);
    let (mut xs, mut ys) = (Vec::with_capacity(n), Vec::with_capacity(n));
    while xs.len() < n {
        let (x, y) = gen_pair(&mut r, pool);
        if xy_to_polar::atan2_lane(y, x, 0).is_ok() {
            xs.push(x);
            ys.push(y);
        }
    }
    (xs, ys)
}

/// Frame shapes of the corpus: the widths 2, 3, 7, 8, 9, 15, 16, 17 and 33, the census frames 50x7 and 64x32, and a frame past the
/// 1,200 pixel threshold of the vector body partition.
const DIMS_POLAR: &[(usize, usize)] = &[(1, 1), (2, 1), (3, 1), (7, 1), (8, 1), (9, 1), (15, 1), (16, 1), (17, 1), (33, 1), (50, 7), (64, 32)];

/// The dir_offset lanes of the corpus: zero, a typical value, negative, and values that make the angle reduction edge cases.
const DIRS: [u32; 6] = [0x0000_0000, 0x3e80_0000, 0xbf00_0000, 0x8000_0000, 0x3f80_0000, 0x3f7f_ffff];

fn xy_corpus() -> Vec<SelfCase> {
    let pool = polar_pool();
    let mut out = Vec::new();
    let mut k = 0usize;
    for &(w, h) in DIMS_POLAR {
        let n = w * h;
        for (ci, &dir) in DIRS.iter().enumerate() {
            let (xs, ys) = polar_pairs(n, (k * 31 + ci) as u64, &pool);
            out.push(SelfCase {
                name: format!("{w}x{h} dir {dir:08x}"),
                dims: (w, h),
                tag: 2,
                args: vec![Image { n, planes: vec![xs, ys] }, const_image(1, [dir, 0, 0, 0])],
                consts: vec![],
                kp: None,
            });
            k += 1;
        }
    }
    // frames at and past the 1,200 pixel threshold with every body phase class
    for (&(w, h), phases) in [(1199usize, 1usize), (1200, 1), (1201, 1), (1207, 1), (1300, 1)].iter().zip([&[0u32][..], &[0, 4, 8, 28, 3], &[12, 16], &[20, 1], &[24, 4]]) {
        let n = w * h;
        for &ph in phases {
            let (xs, ys) = polar_pairs(n, (k * 17 + ph as usize) as u64, &pool);
            let mut kp = KParams::host_defaults();
            kp.polar_mag_phase = ph;
            out.push(SelfCase {
                name: format!("{w}x{h} phase {ph}"),
                dims: (w, h),
                tag: 2,
                args: vec![Image { n, planes: vec![xs, ys] }, const_image(1, [0x3e80_0000, 0, 0, 0])],
                consts: vec![],
                kp: Some(kp),
            });
            k += 1;
        }
    }
    out
}

/// Values for the hue, saturation and value planes: the specials, sextant edges `k / 6` and their neighbours (also shifted by one half),
/// moderate values and mantissa patterns.
pub fn hsv_pool() -> Vec<f32> {
    let mut v = adversarial_f32(0, 1);
    for k in -14..=14 {
        for base in [k as f32 / 6.0, k as f32 / 6.0 - 0.5, k as f32 / 6.0 + 0.5, k as f32] {
            v.extend([base, base.next_up(), base.next_down()]);
        }
    }
    let mut r = Lcg(0xabcd_ef01);
    for _ in 0..400 {
        let m = ((r.next() & 0xff_ffff) as f32 / (1u32 << 24) as f32) * 4.0 - 2.0;
        v.push(m);
    }
    for e in [1u32, 2, 3, 60, 100, 126, 127, 128, 150, 200, 253, 254] {
        for m in [0u32, 0x1, 0x7fffff, 0x2aaaaa] {
            v.push(f32::from_bits((e << 23) | m));
            v.push(f32::from_bits(0x8000_0000 | (e << 23) | m));
        }
    }
    v
}

pub fn hsv_planes(n: usize, seed: u64, pool: &[f32]) -> Vec<Vec<f32>> {
    let mut r = Lcg(seed ^ 0x51ed_270b_9e37_79b9);
    let nan = nan_words();
    let mut planes: Vec<Vec<f32>> = vec![Vec::with_capacity(n), Vec::with_capacity(n), Vec::with_capacity(n)];
    for _ in 0..n {
        // one pixel in sixteen takes NaN words in every plane (the two and three NaN sites)
        let all_nan = r.next() % 16 == 0;
        for p in planes.iter_mut() {
            p.push(if all_nan { nan[(r.next() as usize) % nan.len()] } else { pool[(r.next() as usize) % pool.len()] });
        }
    }
    planes
}

fn hsv_corpus() -> Vec<SelfCase> {
    let pool = hsv_pool();
    let mut out = Vec::new();
    for (k, &(w, h)) in DIMS_POLAR.iter().enumerate() {
        let n = w * h;
        for rep in 0..3 {
            out.push(SelfCase { name: format!("{w}x{h} rep {rep}"), dims: (w, h), tag: 3, args: vec![Image { n, planes: hsv_planes(n, (k * 3 + rep) as u64, &pool) }], consts: vec![], kp: None });
        }
    }
    out
}
