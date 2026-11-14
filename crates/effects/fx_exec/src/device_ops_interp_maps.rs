//! Device kernels of `optical_flow_interp_maps` (`kernels2::optical_flow_interp_maps_k`, crate `optical_flow_interp_maps`).
//!
//! The CPU runs `native::interp_maps`: a forward splat and a scalar scan (both serial and order dependent), two fill sequences
//! (a zero count read by the host loop, a Jacobi fill step per iteration, at most 25 iterations, then a masked copy) and the
//! interpolation of the four output maps. The device runs the same sequence: the splat and the scan as one work item each in the
//! crate's raster order, the fill step, the masked copy and the interpolation as one work item per pixel, the zero count as one
//! work item per row and a one work item sum. The host keeps the crate's control loop (stop test, role swap, cap of 25 passes)
//! and reads one word per pass.
//!
//! Host scalars of the interpolation (`c`, `cs`, `oms`, `gain_rest`) are computed with the crate's `NanOps` and passed as bits.
//! Refusals in `accepts` mirror the CPU's shape errors and the named EXTENT-ONE refusal, so the CPU produces the named error.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use crate::types::Image;
use fx_common::nanop::NanOps;
use fx_device::ocl::{Access, Arg as ClArg, Buffer};
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

pub const INTERP_MAPS: DeviceOp = DeviceOp { class: "optical_flow_interp_maps", accepts: im_accepts, run: im_run, corpus: im_corpus };

/// The classes of this module that may run on a device.
pub const ALLOW: &[&str] = &["optical_flow_interp_maps"];

/// Upper bound of the fill passes of one sequence (the crate's `MAX_FILL_ITERATIONS`).
const MAX_FILL_ITERATIONS: u8 = 25;

fn unsup<T>(why: &str) -> Result<T, DevError> {
    Err(DevError::Unsupported(why.to_string()))
}

fn im_accepts(c: &NodeCall, a: &[ArgShape]) -> Result<(), &'static str> {
    let (w, h) = c.dims;
    if w.checked_mul(h) != Some(c.n) || c.n == 0 {
        return Err("the frame size does not match the sample count");
    }
    if c.n > i32::MAX as usize {
        return Err("more samples than an i32 index");
    }
    if w == 1 || h == 1 {
        return Err("width or height 1 is refused by name on the CPU (EXTENT-ONE)");
    }
    if !(1..=4).contains(&c.tag) {
        return Err("combination tag outside 1..4");
    }
    if a.len() != 4 {
        return Err("four operands expected");
    }
    let tag = c.tag as usize;
    match a[0] {
        ArgShape::Img { channels } if channels >= tag => {}
        _ => return Err("the previous colour is not an image with at least tag channels"),
    }
    match a[1] {
        ArgShape::Img { channels } if channels >= tag => {}
        _ => return Err("the next colour is not an image with at least tag channels"),
    }
    match a[2] {
        ArgShape::Img { channels } if channels >= 2 => {}
        _ => return Err("the flow is not an image with at least two channels"),
    }
    if !matches!(a[3], ArgShape::Const { .. }) {
        return Err("the scale must be a constant");
    }
    Ok(())
}

fn scale_of(a: &DevArg) -> Result<f32, DevError> {
    match a {
        DevArg::Const { lanes, .. } => Ok(f32::from_bits(lanes[0])),
        _ => unsup("the scale is not a constant"),
    }
}

fn zero(s: &Arc<Session>, img: &DevImage) -> Result<(), DevError> {
    for p in 0..img.channels() {
        s.queue.fill(img.plane(p), 0.0f32)?;
    }
    Ok(())
}

/// The zero words of `plane` as the host loop reads them (row counts, then their sum).
fn count_zero(s: &Arc<Session>, plane: &Buffer<f32>, rows: &Buffer<u32>, total: &Buffer<u32>, w: i32, h: i32) -> Result<u32, DevError> {
    s.launch("im_count_rows", h as usize, &[plane.arg(), rows.arg(), ClArg::I32(w), ClArg::I32(h)])?;
    s.launch("im_count_sum", 1, &[rows.arg(), total.arg(), ClArg::I32(h)])?;
    Ok(s.queue.read(total)?[0])
}

fn im_run(s: &Arc<Session>, c: &NodeCall, a: &[DevArg]) -> Result<DevImage, DevError> {
    let (w, h) = c.dims;
    let n = c.n;
    if a.len() != 4 || !(1..=4).contains(&c.tag) {
        return unsup("optical_flow_interp_maps takes four operands and a tag of 1 to 4");
    }
    if w.checked_mul(h) != Some(n) || n == 0 || n > i32::MAX as usize || w == 1 || h == 1 {
        return unsup("the frame size is outside the kernel's coverage");
    }
    let (DevArg::Img(prev), DevArg::Img(next), DevArg::Img(flow)) = (&a[0], &a[1], &a[2]) else {
        return unsup("the colour and flow operands must be images");
    };
    let tag = c.tag as usize;
    if prev.channels() < tag || next.channels() < tag || flow.channels() < 2 || prev.n != n || next.n != n || flow.n != n {
        return unsup("operands smaller than the combination tag or than the frame");
    }
    let scale = scale_of(&a[3])?;
    let nc = tag.min(3);
    let (wi, hi) = (w as i32, h as i32);
    // planes of the work image: target flow x and y, the two value planes of the fill passes
    let out = DevImage::alloc(s, 4, n)?;
    let work = DevImage::alloc(s, 4, n)?;
    zero(s, &out)?;
    zero(s, &work)?;
    let (tfx, tfy, va, vb) = (work.plane(0), work.plane(1), work.plane(2), work.plane(3));
    let (out_a, out_b, out_c, out_d) = (out.plane(0), out.plane(1), out.plane(2), out.plane(3));
    let col = |_img: &DevImage, k: usize| -> usize { if k < nc { k } else { 0 } };
    s.launch(
        "im_splat",
        1,
        &[
            flow.plane(0).arg(),
            flow.plane(1).arg(),
            next.plane(col(next, 0)).arg(),
            next.plane(col(next, 1)).arg(),
            next.plane(col(next, 2)).arg(),
            prev.plane(col(prev, 0)).arg(),
            prev.plane(col(prev, 1)).arg(),
            prev.plane(col(prev, 2)).arg(),
            tfx.arg(),
            tfy.arg(),
            out_a.arg(),
            out_b.arg(),
            out_c.arg(),
            ClArg::I32(wi),
            ClArg::I32(hi),
            ClArg::I32(nc as i32),
            ClArg::F32(scale),
        ],
    )?;
    let rows: Buffer<u32> = Buffer::new(&s.context, h, Access::ReadWrite)?;
    let total: Buffer<u32> = Buffer::new(&s.context, 1, Access::ReadWrite)?;
    // the roles of two planes alternate: (read, written) indices
    let pick = |_pair: [&Buffer<f32>; 2], swapped: bool| -> (usize, usize) { if swapped { (1, 0) } else { (0, 1) } };
    let values = [va, vb];
    let valids = [out_a, out_c];
    let sequence = |source: &Buffer<f32>, start_swapped: bool| -> Result<bool, DevError> {
        let mut swapped = start_swapped;
        let (vi, vo) = pick(values, swapped);
        s.queue.copy(source, values[vi])?;
        s.queue.fill(values[vo], 0.0f32)?;
        let (fi, fo) = pick(valids, swapped);
        s.queue.copy(out_b, valids[fi])?;
        s.queue.fill(valids[fo], 0.0f32)?;
        let mut iterations: u8 = 0;
        loop {
            let (fi, fo) = pick(valids, swapped);
            if count_zero(s, valids[fi], &rows, &total, wi, hi)? == 0 {
                break;
            }
            let (vi, vo) = pick(values, swapped);
            s.launch("im_fill", n, &[out_b.arg(), values[vi].arg(), values[vo].arg(), valids[fi].arg(), valids[fo].arg(), ClArg::I32(wi), ClArg::I32(hi)])?;
            iterations += 1;
            swapped = !swapped;
            if iterations >= MAX_FILL_ITERATIONS {
                break;
            }
        }
        Ok(swapped)
    };
    // first sequence from the first target flow plane, then the masked copy; the second with the second plane
    let swapped = sequence(tfx, false)?;
    let (vi, _) = pick(values, swapped);
    s.launch("im_masked_copy", n, &[out_b.arg(), values[vi].arg(), tfx.arg(), ClArg::I32(wi), ClArg::I32(hi)])?;
    let swapped = sequence(tfy, swapped)?;
    let (vi, _) = pick(values, swapped);
    s.launch("im_masked_copy", n, &[out_b.arg(), values[vi].arg(), tfy.arg(), ClArg::I32(wi), ClArg::I32(hi)])?;
    // interpolation: host scalars by the crate's own operations
    let cc = 2.0f32.fdiv(w as f32);
    let cs = cc.fmul(scale);
    let oms = 1.0f32.fsub(scale);
    let gain_rest = cc.fmul(oms);
    s.launch(
        "im_interp",
        n,
        &[
            tfx.arg(),
            tfy.arg(),
            out_a.arg(),
            out_b.arg(),
            out_c.arg(),
            out_d.arg(),
            ClArg::I32(wi),
            ClArg::I32(hi),
            ClArg::F32(scale),
            ClArg::F32(cc),
            ClArg::F32(cs),
            ClArg::F32(oms),
            ClArg::F32(gain_rest),
            ClArg::F32(0.0f32),
        ],
    )?;
    Ok(out)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// self-check corpus

/// Frame shapes: width 2 and 3 (smallest legal), 7, 8, 9, 15, 16, 17, 33, the census sizes 50x7 and 64x32, and widths on both sides
/// of the 0x4b0 limit of the interpolation's row path.
const DIMS_IM: &[(usize, usize)] = &[(2, 2), (3, 2), (2, 5), (7, 3), (8, 8), (9, 7), (15, 4), (16, 16), (17, 5), (33, 3), (50, 7), (64, 32), (1199, 3), (1200, 3), (1201, 3), (1203, 4), (1209, 3), (1216, 3)];

const SCALES: &[f32] = &[0.5, 1.0, 0.0, -1.0, 2.5, 0.25, -0.0, f32::NAN, 1.0e30, 0.75, 1.0e-40];

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn unit(&mut self) -> f32 {
        (self.next() & 0xff_ffff) as f32 / (1u64 << 24) as f32
    }
}

/// Specials: signed zeros, NaN of both signs, infinities, subnormals, large values.
const SPECIAL: [u32; 14] = [
    0x0000_0000, 0x8000_0000, 0x7fc0_0000, 0xffc0_0001, 0x7f80_0001, 0xff81_2345, 0x7f80_0000, 0xff80_0000, 0x0000_0001, 0x8000_0001, 0x007f_ffff, 0x7f7f_ffff,
    0xff7f_ffff, 0x4b00_0000,
];

/// Flow forms: 0 small fractional, 1 integer steps (equal costs on equal colours), 2 constant (every pixel one target), 3 mixed with
/// specials, 4 large (targets outside), 5 a zero field.
fn flow_planes(form: usize, n: usize, w: usize, scale: f32, seed: u64) -> (Vec<f32>, Vec<f32>) {
    let mut r = Rng(0x9e37_79b9_7f4a_7c15 ^ seed.wrapping_mul(0x1234_5678_9abc_def1));
    let adv = adversarial_f32(1000, 77);
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    for i in 0..n {
        let (x, y) = match form {
            0 => (r.unit() * 6.0 - 3.0, r.unit() * 6.0 - 3.0),
            1 => (((r.next() % 5) as f32) - 2.0, ((r.next() % 5) as f32) - 2.0),
            2 => (1.0, -1.0),
            3 => {
                let pick = |r: &mut Rng, base: f32| -> f32 {
                    match r.next() % 10 {
                        0 => f32::from_bits(SPECIAL[(r.next() as usize) % SPECIAL.len()]),
                        1 => adv[(r.next() as usize) % adv.len()],
                        _ => base,
                    }
                };
                let jitter_x = r.unit() * 4.0 - 2.0;
                let by = r.unit() * 4.0 - 2.0;
                (pick(&mut r, jitter_x), pick(&mut r, by))
            }
            4 => ((r.unit() - 0.5) * (4.0 * w as f32), (r.unit() - 0.5) * (4.0 * w as f32)),
            // x components on and within three ulps of the values whose scaled sum with the pixel centre falls on an integer: the
            // fused and the separately rounded x target then disagree on some pixels
            7 => {
                let m = (r.next() % (w as u64 + 1)) as f32 + 0.5;
                let base = (m / scale).to_bits() as i64;
                let off = (r.next() % 7) as i64 - 3;
                (f32::from_bits((base + off) as u32), r.unit() * 2.0 - 1.0)
            }
            // one finite vector, every other pixel a NaN flow: the fill spreads one pixel per pass and reaches the cap of 25 passes
            6 => {
                if i == n / 3 { (0.25, -0.25) } else { (f32::NAN, f32::NAN) }
            }
            _ => (0.0, 0.0),
        };
        a.push(x);
        b.push(y);
    }
    (a, b)
}

/// Colour planes: 0 random in [0, 1), 1 constant (ties), 2 random with a NaN, an infinity and signed zeros, 3 adversarial values.
fn colour_planes(form: usize, count: usize, n: usize, seed: u64) -> Vec<Vec<f32>> {
    let adv = adversarial_f32(1000, 91);
    (0..count)
        .map(|p| {
            let mut r = Rng(0x2545_f491_4f6c_dd1d ^ (seed * 31 + p as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15));
            (0..n)
                .map(|_| match form {
                    0 => r.unit(),
                    1 => 0.5,
                    2 => match r.next() % 12 {
                        0 => f32::from_bits(SPECIAL[(r.next() as usize) % SPECIAL.len()]),
                        _ => r.unit(),
                    },
                    _ => match r.next() % 3 {
                        0 => adv[(r.next() as usize) % adv.len()],
                        _ => r.unit(),
                    },
                })
                .collect()
        })
        .collect()
}

fn one_case(w: usize, h: usize, tag: i32, extra_chans: usize, cform: usize, fform: usize, scale: f32, seed: u64) -> SelfCase {
    let n = w * h;
    let chans = tag as usize + extra_chans;
    let (fa, fb) = flow_planes(fform, n, w, scale, seed);
    SelfCase {
        name: format!("{w}x{h} tag {tag} planes {chans} colour {cform} flow {fform} scale {:08x}", scale.to_bits()),
        dims: (w, h),
        tag,
        args: vec![Image { n, planes: colour_planes(cform, chans, n, seed) }, Image { n, planes: colour_planes(cform, chans, n, seed + 1000) }, Image { n, planes: vec![fa, fb] }],
        consts: vec![crate::device_ops_neighbourhood::cst(3, &[scale])],
        kp: None,
    }
}

fn im_corpus() -> Vec<SelfCase> {
    let mut out = Vec::new();
    let mut k = 0usize;
    for (dim_idx, &(w, h)) in DIMS_IM.iter().enumerate() {
        // the large frames run fewer combinations (the serial kernels are slow on a CPU device)
        let big = w * h > 1000;
        for tag in 1..=4i32 {
            for fform in 0..6usize {
                k += 1;
                if big && (tag != 3 && fform != 0 || fform > 3) {
                    continue;
                }
                let cform = (k + dim_idx) % 4;
                let scale = SCALES[(k * 3 + dim_idx) % SCALES.len()];
                let extra = usize::from(k % 5 == 0);
                out.push(one_case(w, h, tag, extra, cform, fform, scale, (dim_idx * 977 + k) as u64));
            }
        }
    }
    // the fill cap: a single covered pixel on frames wider than 25 passes can spread
    for (dim_idx, &(w, h)) in [(64usize, 32usize), (50, 7), (33, 3), (16, 16)].iter().enumerate() {
        out.push(one_case(w, h, 3, 0, 0, 6, 0.5, 4000 + dim_idx as u64));
        out.push(one_case(w, h, 1, 0, 1, 6, 1.0, 4100 + dim_idx as u64));
    }
    // fused against separately rounded x target of the splat
    for (dim_idx, &(w, h)) in [(33usize, 3usize), (64, 4), (9, 7)].iter().enumerate() {
        for (set_idx, &sc) in [0.75f32, 0.3, 0.9, 1.7, 0.6].iter().enumerate() {
            out.push(one_case(w, h, 1 + (set_idx % 3) as i32, 0, 1, 7, sc, 8000 + (dim_idx * 10 + set_idx) as u64));
        }
    }
    // every scale value on a frame with ties and with specials
    for (set_idx, &sc) in SCALES.iter().enumerate() {
        out.push(one_case(9, 7, 3, 0, 1, 1, sc, 5000 + set_idx as u64));
        out.push(one_case(16, 16, 4, 0, 2, 3, sc, 6000 + set_idx as u64));
        out.push(one_case(7, 3, 1, 0, 3, 0, sc, 7000 + set_idx as u64));
    }
    out
}
