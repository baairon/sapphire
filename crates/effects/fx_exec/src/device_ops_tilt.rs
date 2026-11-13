//! Device op `tilt` (the S_WarpTransform tilt stage): a port of `kernels2::tilt_k` onto `tilt::render` without the host offload flag.
//!
//! `tilt_k` refuses a non-constant parameter operand (unsupported) and an operand 0 that is not a 2 channel plane pair; `accepts`
//! refuses exactly those shapes, so the CPU path produces the same named error. The flag path of the crate (byte argument non-zero)
//! is never set by `tilt_k` (it passes `false`), so the device has no flag route either.
//!
//! The host runs the crate's own prologue (`tilt::setup`: both sincosf calls, the flags, every derived constant) and passes the
//! results as bit patterns; the kernel `tilt_f32` in `fx_device::kernels_tilt` runs the per element loops and picks the loop variant
//! from the flags in the crate's order. Constants that the loops derive once before the element loop (`ca_over_eps`,
//! `half_minus_ca`, the sign of the cosine term, `shear_y_scaled`) are computed here with the crate's `NanOps` and passed too.

use crate::backend::NodeCall;
use crate::device_ops::{ArgShape, DevArg, DeviceOp, SelfCase};
use fx_common::nanop::NanOps;
use fx_device::cl::Arg as ClArg;
use fx_device::selfcheck::adversarial_f32;
use fx_device::{DevError, DevImage, Session};
use std::sync::Arc;

fn unsup(m: &str) -> DevError {
    DevError::Unsupported(m.into())
}

pub const TILT: DeviceOp = DeviceOp { class: "tilt", accepts: tilt_accepts, run: tilt_run, corpus: tilt_corpus };

pub fn tilt_accepts(c: &NodeCall, shapes: &[ArgShape]) -> Result<(), &'static str> {
    if shapes.len() != 8 {
        return Err("unexpected operand count");
    }
    if shapes[0] != (ArgShape::Img { channels: 2 }) {
        return Err("operand 0 must be a 2 channel plane pair");
    }
    if shapes[1..].iter().any(|s| !matches!(s, ArgShape::Const { .. })) {
        return Err("non-constant parameter operand at run time");
    }
    if c.n == 0 || c.n > i32::MAX as usize {
        return Err("sample count outside the kernel's range");
    }
    Ok(())
}

/// The state of `tilt_k` from the constant operands.
pub fn state_of(lanes: &[[u32; 4]; 7]) -> tilt::State {
    let f = |i: usize, k: usize| f32::from_bits(lanes[i][k]);
    tilt::State {
        swivel: f(0, 0),
        tilt: f(0, 1),
        perspective: f(1, 0),
        shift: [f(2, 0), f(2, 1)],
        rotate: f(3, 0),
        scale: [f(4, 0), f(4, 1)],
        shear: [f(5, 0), f(5, 1)],
        wrap: [lanes[6][0], lanes[6][1]],
    }
}

/// The parameter words of the kernel (layout in `fx_device::kernels_tilt`), from the crate's prologue.
pub fn params_of(st: &tilt::State) -> [u32; 26] {
    let s = tilt::setup(st);
    let cop = s.cos_over_persp;
    let w = |v: f32| v.to_bits();
    [
        s.persp_flag as u32,
        s.rot_flag as u32,
        s.shear_flag as u32,
        st.wrap[0],
        st.wrap[1],
        w(st.scale[0]),
        w(st.scale[1]),
        w(st.shear[0]),
        w(st.shear[1]),
        w(st.perspective),
        w(st.shift[0]),
        w(st.shift[1]),
        w(s.rot_sin),
        w(s.rot_cos),
        w(s.tilt_sin),
        w(s.tilt_cos),
        w(s.axis_u),
        w(s.axis_v),
        w(s.sin_over_persp),
        w(cop),
        w(s.sin_times_persp),
        w(s.cos_times_persp),
        (cop > 0.0) as u32,
        w(cop.fdiv(tilt::C_EPS)),
        w(tilt::C_ONE_HALF.fsub(cop)),
        w(st.shear[1].fmul(st.scale[0])),
    ]
}

/// One launch of `tilt_f32` on the two input planes with the given parameter words.
pub fn tilt_launch(s: &Arc<Session>, x: &fx_device::cl::Buffer<f32>, y: &fx_device::cl::Buffer<f32>, params: &[u32; 26], n: usize) -> Result<DevImage, DevError> {
    let out = DevImage::alloc(s, 2, n)?;
    let pr = s.input(&params[..])?;
    s.launch("tilt_f32", n, &[x.arg(), y.arg(), out.plane(0).arg(), out.plane(1).arg(), pr.arg(), ClArg::U32(n as u32)])?;
    Ok(out)
}

pub fn tilt_run(s: &Arc<Session>, c: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    if args.len() != 8 {
        return Err(unsup("eight operands expected"));
    }
    let DevArg::Img(uv) = &args[0] else { return Err(unsup("operand 0 is not an image")) };
    if uv.channels() != 2 || uv.n != c.n || c.n == 0 || c.n > i32::MAX as usize {
        return Err(unsup("operand 0 must be a 2 channel image of the frame size"));
    }
    let mut lanes = [[0u32; 4]; 7];
    for (i, a) in args[1..].iter().enumerate() {
        match a {
            DevArg::Const { lanes: l, .. } => lanes[i] = *l,
            _ => return Err(unsup("non-constant parameter operand")),
        }
    }
    let params = params_of(&state_of(&lanes));
    tilt_launch(s, uv.plane(0), uv.plane(1), &params, c.n)
}

/// Parameter sets of the self-check: (swivel, tilt), perspective, shift, rotate, scale, shear, wrap words.
type Set = ([f32; 2], f32, [f32; 2], f32, [f32; 2], [f32; 2], [u32; 2]);

pub fn param_sets() -> Vec<Set> {
    let nz = 0x3f80_0000u32;
    let mut v: Vec<Set> = Vec::new();
    for wrap in [[0, 0], [nz, 0], [0, nz], [nz, nz]] {
        // plain (swivel and tilt zero): rotate and shear on and off
        v.push(([0.0, 0.0], 1.0, [0.25, -0.5], 0.0, [1.0, 1.0], [0.0, 0.0], wrap));
        v.push(([0.0, 0.0], 1.0, [0.25, -0.5], 30.0, [1.5, 0.75], [0.0, 0.0], wrap));
        v.push(([0.0, 0.0], 1.0, [0.0, 0.0], 0.0, [2.0, 0.5], [0.3, -0.2], wrap));
        v.push(([0.0, 0.0], 1.0, [-1.0, 2.0], 90.0, [1.0, 1.0], [0.1, 0.4], wrap));
        // perspective: every loop variant
        for rot in [0.0, 25.0, 180.0] {
            for shear in [[0.0, 0.0], [0.2, -0.1]] {
                v.push(([20.0, -35.0], 1.5, [0.1, 0.2], rot, [1.1, 0.9], shear, wrap));
            }
        }
    }
    v.push(([90.0, 0.0], 1.0, [0.0, 0.0], 0.0, [1.0, 1.0], [0.0, 0.0], [0, 0]));
    v.push(([0.0, 180.0], 0.5, [0.0, 0.0], 45.0, [1.0, 1.0], [0.0, 0.0], [0, 0]));
    v.push(([0.0, 90.0], 2.0, [0.0, 0.0], 45.0, [1.0, 1.0], [0.3, 0.0], [0, nz]));
    v.push(([0.5, 0.5], f32::from_bits(1), [0.0, 0.0], 0.0, [1.0, 1.0], [0.0, 0.0], [nz, nz]));
    v.push(([-0.0, 0.0], 1.0, [-0.0, -0.0], -0.0, [1.0, 1.0], [-0.0, -0.0], [0, 0]));
    v.push(([0.0, 0.0], 1.0, [0.25, -0.5], f32::NAN, [1.0, 1.0], [0.0, 0.0], [0, 0]));
    v.push(([f32::NAN, 20.0], 1.5, [0.1, 0.2], 25.0, [1.1, 0.9], [0.0, 0.0], [0, 0]));
    v.push(([60.0, 60.0], 1.0, [0.0, 0.0], 0.0, [1.0, 1.0], [0.0, 0.0], [0, 0]));
    v
}

pub fn lanes_of(set: &Set) -> [[u32; 4]; 7] {
    let (sv, pa, sh, ro, sc, sr, wr) = set;
    [
        [sv[0].to_bits(), sv[1].to_bits(), 0, 0],
        [pa.to_bits(), 0, 0, 0],
        [sh[0].to_bits(), sh[1].to_bits(), 0, 0],
        [ro.to_bits(), 0, 0, 0],
        [sc[0].to_bits(), sc[1].to_bits(), 0, 0],
        [sr[0].to_bits(), sr[1].to_bits(), 0, 0],
        [wr[0], wr[1], 0, 0],
    ]
}

fn tilt_corpus() -> Vec<SelfCase> {
    let vals = adversarial_f32(300, 91);
    let mut out = Vec::new();
    let mut idx = 0usize;
    for (si, set) in param_sets().iter().enumerate() {
        let lanes = lanes_of(set);
        for &(w, h) in &[(1usize, 1usize), (9, 1), (45, 23)] {
            let n = w * h;
            let pick = |k: usize, i: usize| -> f32 {
                match (si + k) % 3 {
                    0 => vals[(i * 7 + k * 13 + idx) % vals.len()],
                    1 => ((i as f32) * 0.37 - (n as f32) * 0.18) * if k == 0 { 1.0 } else { -0.8 },
                    _ => {
                        if i % 11 == 0 {
                            vals[(i * 5 + idx) % vals.len()]
                        } else {
                            ((i as f32) * 0.011 - 2.0) * (1 + k) as f32
                        }
                    }
                }
            };
            let x: Vec<f32> = (0..n).map(|i| pick(0, i)).collect();
            let y: Vec<f32> = (0..n).map(|i| pick(1, i)).collect();
            out.push(SelfCase {
                name: format!("tilt set{si} {w}x{h}"),
                dims: (w, h),
                tag: 2,
                args: vec![crate::types::Image { n, planes: vec![x, y] }],
                consts: lanes.iter().enumerate().map(|(k, l)| (k + 1, 2, *l)).collect(),
                kp: None,
            });
            idx += 1;
        }
    }
    out
}
