//! Perspective path of the tilt body (swivel or tilt non-zero): each element is projected through the tilted plane,
//! optionally rotated and sheared, then clamped per axis when the wrap flag is zero. The compiler of the original
//! split the loop by rotation, shear and wrap flags; each variant keeps its own operand order, because the first
//! NaN operand of an add, multiply or fused step decides the payload of the result.
//!
//! Every add, sub, mul, div and fused step goes through `fx_common::nanop` in source operand order. A fresh NaN is
//! 0xffc00000.
use crate::{clamp, flip, Setup, State, C_EPS, C_ONE_HALF};
use fx_common::nanop::{fma132, fma213, fma231, fms132, fms213, fms231, fnma213, fnma231, fnms213, NanOps};

/// Typed values the loops read: the operand block and the constants the prologue derives from it.
#[derive(Clone, Copy)]
pub(crate) struct Persp {
    pub scale_x: f32,
    pub scale_y: f32,
    pub shear_x: f32,
    pub shear_y: f32,
    pub perspective: f32,
    pub shift_x: f32,
    pub shift_y: f32,
    pub rot_sin: f32,
    pub rot_cos: f32,
    pub tilt_sin: f32,
    pub tilt_cos: f32,
    /// Tilt direction components scaled by the inverse length.
    pub axis_u: f32,
    pub axis_v: f32,
    pub sin_over_persp: f32,
    pub cos_over_persp: f32,
    pub sin_times_persp: f32,
    pub cos_times_persp: f32,
    pub shear_flag: bool,
    /// Bit patterns: zero clamps the coordinate, anything else passes it.
    pub wrap_x: u32,
    pub wrap_y: u32,
}

/// The two input planes and the two output planes.
pub(crate) struct Planes<'a> {
    pub x: &'a [f32],
    pub y: &'a [f32],
    pub first_out: &'a mut [f32],
    pub second_out: &'a mut [f32],
}

/// Reciprocal depth factor: a denominator equal to zero (ordered) gives the cosine term over the epsilon; otherwise
/// the sign of the denominator against the sign of the cosine term picks the quotient. An unordered compare (NaN) skips
/// the zero test and takes the sign test with "not greater than zero".
fn inverse_depth(denominator: f32, ca_positive: bool, half_minus_ca: f32, cos_over_persp: f32, ca_over_eps: f32) -> f32 {
    let unordered = denominator.is_nan();
    if !unordered && denominator == 0.0 {
        return ca_over_eps;
    }
    if ca_positive == (denominator > 0.0) {
        half_minus_ca.fdiv(denominator)
    } else {
        cos_over_persp.fdiv(flip(denominator))
    }
}

/// No rotation: the shear step and the two wrap clamps are chosen per element by the flags.
fn general(p: &Persp, planes: Planes, n: usize) {
    let Persp { scale_x, scale_y, shear_x, shear_y, perspective, shift_x, shift_y, tilt_sin, tilt_cos, axis_u, axis_v, sin_over_persp, cos_over_persp, sin_times_persp, cos_times_persp, shear_flag, wrap_x, wrap_y, .. } = *p;
    let Planes { x, y, first_out, second_out } = planes;
    let ca_positive = cos_over_persp > 0.0;
    let ca_over_eps: f32 = cos_over_persp.fdiv(C_EPS);
    let half_minus_ca: f32 = C_ONE_HALF.fsub(cos_over_persp);
    for i in 0..n {
        let yv = y[i];
        let xv = x[i];
        let y_scaled_u: f32 = yv.fmul(axis_u);
        let cross: f32 = fnma231(y_scaled_u, xv, axis_v);
        let denominator: f32 = fnms213(sin_times_persp, cross, tilt_cos);
        let inv_depth: f32 = inverse_depth(denominator, ca_positive, half_minus_ca, cos_over_persp, ca_over_eps);
        let x_scaled_u: f32 = xv.fmul(axis_u);
        let depth_gain: f32 = perspective.fmul(inv_depth);
        let numer_a: f32 = fms213(cross, cos_times_persp, tilt_sin);
        let dot_along: f32 = fma213(yv, axis_v, x_scaled_u);
        let numer_b: f32 = fma213(numer_a, inv_depth, sin_over_persp);
        let dot_gain: f32 = dot_along.fmul(depth_gain);
        let proj_u: f32 = fma213(axis_u, dot_gain, shift_x);
        let res_u: f32 = fms231(proj_u, numer_b, axis_v);
        let proj_v: f32 = fma213(numer_b, axis_u, shift_y);
        let scaled_u: f32 = res_u.fmul(scale_x);
        let res_v: f32 = fma231(proj_v, dot_gain, axis_v);
        let neg_u: f32 = flip(scaled_u);
        let scaled_v: f32 = res_v.fmul(scale_y);
        let (pre_p, pre_q) = if shear_flag {
            let shear_by_x: f32 = scaled_v.fmul(shear_x);
            let shear_by_y: f32 = neg_u.fmul(shear_y);
            let sheared_p: f32 = shear_by_x.fadd(neg_u);
            let sheared_q: f32 = shear_by_y.fadd(scaled_v);
            (sheared_p, sheared_q)
        } else {
            (neg_u, scaled_v)
        };
        let out_p = if wrap_x == 0 { clamp(pre_p) } else { pre_p };
        let out_q = if wrap_y == 0 { clamp(pre_q) } else { pre_q };
        first_out[i] = out_p;
        second_out[i] = out_q;
    }
}

/// Rotation and shear, both coordinates pass through unclamped.
fn rot_shear(p: &Persp, planes: Planes, n: usize) {
    let Persp { scale_x, scale_y, shear_x, shear_y, perspective, shift_x, shift_y, rot_sin, rot_cos, tilt_sin, tilt_cos, axis_u, axis_v, sin_over_persp, cos_over_persp, sin_times_persp, cos_times_persp, .. } = *p;
    let Planes { x, y, first_out, second_out } = planes;
    let ca_positive = cos_over_persp > 0.0;
    let half_minus_ca: f32 = C_ONE_HALF.fsub(cos_over_persp);
    let ca_over_eps: f32 = cos_over_persp.fdiv(C_EPS);
    let shear_y_scaled: f32 = shear_y.fmul(scale_x);
    for i in 0..n {
        let yv = y[i];
        let xv = x[i];
        let y_scaled_u: f32 = yv.fmul(axis_u);
        let cross: f32 = fnma231(y_scaled_u, axis_v, xv);
        let denominator: f32 = fnms213(sin_times_persp, cross, tilt_cos);
        let inv_depth: f32 = inverse_depth(denominator, ca_positive, half_minus_ca, cos_over_persp, ca_over_eps);
        let x_scaled_u: f32 = xv.fmul(axis_u);
        let depth_gain: f32 = perspective.fmul(inv_depth);
        let numer_a: f32 = fms213(cross, cos_times_persp, tilt_sin);
        let dot_along: f32 = fma213(yv, axis_v, x_scaled_u);
        let numer_b: f32 = fma213(numer_a, inv_depth, sin_over_persp);
        let dot_gain: f32 = dot_along.fmul(depth_gain);
        let proj_v: f32 = fma213(axis_u, numer_b, shift_y);
        let proj_u: f32 = fma213(axis_u, dot_gain, shift_x);
        let res_v: f32 = fma231(proj_v, dot_gain, axis_v);
        let tilt_u: f32 = fnma213(axis_v, numer_b, proj_u);
        let res_u: f32 = fms231(proj_u, numer_b, axis_v);
        let v_sin: f32 = res_v.fmul(rot_sin);
        let u_sin: f32 = res_u.fmul(rot_sin);
        let tilt_u_cos: f32 = tilt_u.fmul(rot_cos);
        let rot_v: f32 = fms231(u_sin, rot_cos, res_v);
        let v_sin_alt: f32 = rot_sin.fmul(res_v);
        let rot_p_diff: f32 = v_sin.fsub(tilt_u_cos);
        let rot_u: f32 = fms231(v_sin_alt, tilt_u, rot_cos);
        let rot_p_scaled: f32 = rot_p_diff.fmul(scale_x);
        let rot_q_scaled: f32 = scale_y.fmul(rot_v);
        let shear_prod_a: f32 = rot_u.fmul(shear_y_scaled);
        let shear_prod_b: f32 = rot_q_scaled.fmul(shear_x);
        let sheared_q_rot: f32 = shear_prod_a.fadd(rot_q_scaled);
        let neg_rot_p: f32 = flip(rot_p_scaled);
        let sheared_p_rot: f32 = shear_prod_b.fadd(neg_rot_p);
        first_out[i] = sheared_p_rot;
        second_out[i] = sheared_q_rot;
    }
}

/// Rotation without shear, the first coordinate passes through and the second is clamped when wrap-y is zero.
fn rot_plain(p: &Persp, planes: Planes, n: usize) {
    let Persp { scale_x, scale_y, perspective, shift_x, shift_y, rot_sin, rot_cos, tilt_sin, tilt_cos, axis_u, axis_v, sin_over_persp, cos_over_persp, sin_times_persp, cos_times_persp, wrap_y, .. } = *p;
    let Planes { x, y, first_out, second_out } = planes;
    let ca_positive = cos_over_persp > 0.0;
    let ca_over_eps: f32 = cos_over_persp.fdiv(C_EPS);
    let half_minus_ca: f32 = C_ONE_HALF.fsub(cos_over_persp);
    for i in 0..n {
        let yv = y[i];
        let xv = x[i];
        let y_scaled_u: f32 = yv.fmul(axis_u);
        let cross: f32 = fnma231(y_scaled_u, axis_v, xv);
        let denominator: f32 = fnms213(sin_times_persp, cross, tilt_cos);
        let inv_depth: f32 = inverse_depth(denominator, ca_positive, half_minus_ca, cos_over_persp, ca_over_eps);
        let x_scaled_u: f32 = xv.fmul(axis_u);
        let numer_a: f32 = fms213(cross, cos_times_persp, tilt_sin);
        let dot_along: f32 = fma213(yv, axis_v, x_scaled_u);
        let depth_gain: f32 = perspective.fmul(inv_depth);
        let numer_b: f32 = fma213(numer_a, inv_depth, sin_over_persp);
        let dot_gain: f32 = dot_along.fmul(depth_gain);
        let proj_v: f32 = fma213(axis_u, numer_b, shift_y);
        let proj_u: f32 = fma213(axis_u, dot_gain, shift_x);
        let res_v: f32 = fma231(proj_v, dot_gain, axis_v);
        let tilt_u: f32 = fnma213(axis_v, numer_b, proj_u);
        let res_u: f32 = fms231(proj_u, numer_b, axis_v);
        let res_v_cos: f32 = res_v.fmul(rot_cos);
        let v_sin: f32 = res_v.fmul(rot_sin);
        let rot_v_plain: f32 = fms213(res_u, rot_sin, res_v_cos);
        let rot_u: f32 = fms213(tilt_u, rot_cos, v_sin);
        let rot_q_plain: f32 = rot_v_plain.fmul(scale_y);
        let neg_rot_q: f32 = flip(rot_q_plain);
        let out_q = if wrap_y == 0 { clamp(neg_rot_q) } else { neg_rot_q };
        let rot_p_plain: f32 = scale_x.fmul(rot_u);
        first_out[i] = rot_p_plain;
        second_out[i] = out_q;
    }
}

/// Rotation and shear, only the second coordinate is clamped.
fn rot_shear_wrapy(p: &Persp, planes: Planes, n: usize) {
    let Persp { scale_x, scale_y, shear_x, shear_y, perspective, shift_x, shift_y, rot_sin, rot_cos, tilt_sin, tilt_cos, axis_u, axis_v, sin_over_persp, cos_over_persp, sin_times_persp, cos_times_persp, .. } = *p;
    let Planes { x, y, first_out, second_out } = planes;
    let ca_positive = cos_over_persp > 0.0;
    let half_minus_ca: f32 = C_ONE_HALF.fsub(cos_over_persp);
    let ca_over_eps: f32 = cos_over_persp.fdiv(C_EPS);
    let shear_y_scaled: f32 = shear_y.fmul(scale_x);
    for i in 0..n {
        let yv = y[i];
        let xv = x[i];
        let y_scaled_u: f32 = yv.fmul(axis_u);
        let cross: f32 = fnma231(y_scaled_u, axis_v, xv);
        let denominator: f32 = fnms213(sin_times_persp, cross, tilt_cos);
        let inv_depth: f32 = inverse_depth(denominator, ca_positive, half_minus_ca, cos_over_persp, ca_over_eps);
        let x_scaled_u: f32 = xv.fmul(axis_u);
        let depth_gain: f32 = inv_depth.fmul(perspective);
        let dot_along: f32 = fma213(yv, axis_v, x_scaled_u);
        let dot_gain: f32 = dot_along.fmul(depth_gain);
        let numer_a: f32 = fms132(cross, tilt_sin, cos_times_persp);
        let proj_u: f32 = fma213(axis_u, dot_gain, shift_x);
        let numer_b: f32 = fma213(numer_a, inv_depth, sin_over_persp);
        let proj_v: f32 = fma213(axis_u, numer_b, shift_y);
        let tilt_u: f32 = fnma213(axis_v, numer_b, proj_u);
        let res_u: f32 = fms231(proj_u, numer_b, axis_v);
        let res_v: f32 = fma231(proj_v, dot_gain, axis_v);
        let tilt_u_cos: f32 = tilt_u.fmul(rot_cos);
        let u_sin: f32 = res_u.fmul(rot_sin);
        let v_sin: f32 = res_v.fmul(rot_sin);
        let rot_v: f32 = fms231(u_sin, rot_cos, res_v);
        let v_sin_alt: f32 = rot_sin.fmul(res_v);
        let rot_p_diff: f32 = v_sin.fsub(tilt_u_cos);
        let rot_q_scaled: f32 = scale_y.fmul(rot_v);
        let rot_u: f32 = fms231(v_sin_alt, tilt_u, rot_cos);
        let rot_p_scaled: f32 = rot_p_diff.fmul(scale_x);
        let shear_prod_b: f32 = rot_q_scaled.fmul(shear_x);
        let sheared_q_fused: f32 = fma132(rot_u, rot_q_scaled, shear_y_scaled);
        let neg_rot_p: f32 = flip(rot_p_scaled);
        let sheared_p_rot: f32 = shear_prod_b.fadd(neg_rot_p);
        let wrapped_q_fused: f32 = clamp(sheared_q_fused);
        first_out[i] = sheared_p_rot;
        second_out[i] = wrapped_q_fused;
    }
}

/// Rotation without shear, only the first coordinate is clamped.
fn rot_plain_wrapx(p: &Persp, planes: Planes, n: usize) {
    let Persp { scale_x, scale_y, perspective, shift_x, shift_y, rot_sin, rot_cos, tilt_sin, tilt_cos, axis_u, axis_v, sin_over_persp, cos_over_persp, sin_times_persp, cos_times_persp, .. } = *p;
    let Planes { x, y, first_out, second_out } = planes;
    let ca_positive = cos_over_persp > 0.0;
    let half_minus_ca: f32 = C_ONE_HALF.fsub(cos_over_persp);
    let ca_over_eps: f32 = cos_over_persp.fdiv(C_EPS);
    for i in 0..n {
        let yv = y[i];
        let xv = x[i];
        let y_scaled_u: f32 = yv.fmul(axis_u);
        let cross: f32 = fnma231(y_scaled_u, axis_v, xv);
        let denominator: f32 = fnms213(sin_times_persp, cross, tilt_cos);
        let inv_depth: f32 = inverse_depth(denominator, ca_positive, half_minus_ca, cos_over_persp, ca_over_eps);
        let x_scaled_u: f32 = xv.fmul(axis_u);
        let depth_gain: f32 = inv_depth.fmul(perspective);
        let dot_along: f32 = fma213(yv, axis_v, x_scaled_u);
        let dot_gain: f32 = dot_along.fmul(depth_gain);
        let numer_a: f32 = fms213(cross, cos_times_persp, tilt_sin);
        let proj_u: f32 = fma213(axis_u, dot_gain, shift_x);
        let numer_b: f32 = fma213(numer_a, inv_depth, sin_over_persp);
        let proj_v: f32 = fma213(axis_u, numer_b, shift_y);
        let tilt_u: f32 = fnma213(axis_v, numer_b, proj_u);
        let res_u: f32 = fms231(proj_u, numer_b, axis_v);
        let res_v: f32 = fma231(proj_v, dot_gain, axis_v);
        let tilt_u_cos: f32 = tilt_u.fmul(rot_cos);
        let u_sin: f32 = res_u.fmul(rot_sin);
        let rot_v: f32 = fms231(u_sin, rot_cos, res_v);
        let v_sin: f32 = res_v.fmul(rot_sin);
        let rot_p_diff: f32 = v_sin.fsub(tilt_u_cos);
        let rot_p_scaled: f32 = rot_p_diff.fmul(scale_x);
        let neg_rot_p: f32 = flip(rot_p_scaled);
        let wrapped_neg_p: f32 = clamp(neg_rot_p);
        let rot_q_scaled: f32 = scale_y.fmul(rot_v);
        first_out[i] = wrapped_neg_p;
        second_out[i] = rot_q_scaled;
    }
}

/// Rotation without shear, both coordinates are clamped.
fn rot_plain_wrapxy(p: &Persp, planes: Planes, n: usize) {
    let Persp { scale_x, scale_y, perspective, shift_x, shift_y, rot_sin, rot_cos, tilt_sin, tilt_cos, axis_u, axis_v, sin_over_persp, cos_over_persp, sin_times_persp, cos_times_persp, .. } = *p;
    let Planes { x, y, first_out, second_out } = planes;
    let ca_positive = cos_over_persp > 0.0;
    let half_minus_ca: f32 = C_ONE_HALF.fsub(cos_over_persp);
    let ca_over_eps: f32 = cos_over_persp.fdiv(C_EPS);
    for i in 0..n {
        let yv = y[i];
        let xv = x[i];
        let y_scaled_u: f32 = yv.fmul(axis_u);
        let cross: f32 = fnma231(y_scaled_u, axis_v, xv);
        let denominator: f32 = fnms213(sin_times_persp, cross, tilt_cos);
        let inv_depth: f32 = inverse_depth(denominator, ca_positive, half_minus_ca, cos_over_persp, ca_over_eps);
        let x_scaled_u: f32 = xv.fmul(axis_u);
        let depth_gain: f32 = inv_depth.fmul(perspective);
        let numer_a: f32 = fms132(cross, tilt_sin, cos_times_persp);
        let dot_along: f32 = fma213(yv, axis_v, x_scaled_u);
        let numer_b: f32 = fma213(numer_a, inv_depth, sin_over_persp);
        let dot_gain: f32 = dot_along.fmul(depth_gain);
        let proj_v: f32 = fma213(axis_u, numer_b, shift_y);
        let proj_u: f32 = fma213(axis_u, dot_gain, shift_x);
        let res_v: f32 = fma231(proj_v, dot_gain, axis_v);
        let tilt_u: f32 = fnma213(axis_v, numer_b, proj_u);
        let res_u: f32 = fms231(proj_u, numer_b, axis_v);
        let v_sin: f32 = res_v.fmul(rot_sin);
        let tilt_u_cos: f32 = tilt_u.fmul(rot_cos);
        let u_sin: f32 = res_u.fmul(rot_sin);
        let rot_p_diff: f32 = v_sin.fsub(tilt_u_cos);
        let rot_v: f32 = fms231(u_sin, rot_cos, res_v);
        let rot_p_scaled: f32 = rot_p_diff.fmul(scale_x);
        let neg_rot_p: f32 = flip(rot_p_scaled);
        let rot_q_scaled: f32 = scale_y.fmul(rot_v);
        let wrapped_neg_p: f32 = clamp(neg_rot_p);
        let wrapped_q: f32 = clamp(rot_q_scaled);
        first_out[i] = wrapped_neg_p;
        second_out[i] = wrapped_q;
    }
}

/// Rotation and shear, only the first coordinate is clamped.
fn rot_shear_wrapx(p: &Persp, planes: Planes, n: usize) {
    let Persp { scale_x, scale_y, shear_x, shear_y, perspective, shift_x, shift_y, rot_sin, rot_cos, tilt_sin, tilt_cos, axis_u, axis_v, sin_over_persp, cos_over_persp, sin_times_persp, cos_times_persp, .. } = *p;
    let Planes { x, y, first_out, second_out } = planes;
    let ca_positive = cos_over_persp > 0.0;
    let half_minus_ca: f32 = C_ONE_HALF.fsub(cos_over_persp);
    let ca_over_eps: f32 = cos_over_persp.fdiv(C_EPS);
    let shear_y_scaled: f32 = shear_y.fmul(scale_x);
    for i in 0..n {
        let yv = y[i];
        let xv = x[i];
        let y_scaled_u: f32 = yv.fmul(axis_u);
        let cross: f32 = fnma231(y_scaled_u, axis_v, xv);
        let denominator: f32 = fnms213(sin_times_persp, cross, tilt_cos);
        let inv_depth: f32 = inverse_depth(denominator, ca_positive, half_minus_ca, cos_over_persp, ca_over_eps);
        let x_scaled_u: f32 = xv.fmul(axis_u);
        let depth_gain: f32 = perspective.fmul(inv_depth);
        let dot_along: f32 = fma213(yv, axis_v, x_scaled_u);
        let dot_gain: f32 = dot_along.fmul(depth_gain);
        let numer_a: f32 = fms213(cross, cos_times_persp, tilt_sin);
        let proj_u: f32 = fma213(axis_u, dot_gain, shift_x);
        let numer_b: f32 = fma213(numer_a, inv_depth, sin_over_persp);
        let proj_v: f32 = fma213(axis_u, numer_b, shift_y);
        let tilt_u: f32 = fnma213(axis_v, numer_b, proj_u);
        let res_u: f32 = fms231(proj_u, numer_b, axis_v);
        let res_v: f32 = fma231(proj_v, dot_gain, axis_v);
        let u_sin: f32 = res_u.fmul(rot_sin);
        let tilt_u_cos: f32 = tilt_u.fmul(rot_cos);
        let v_sin: f32 = res_v.fmul(rot_sin);
        let rot_v: f32 = fms231(u_sin, rot_cos, res_v);
        let v_sin_alt: f32 = rot_sin.fmul(res_v);
        let rot_p_diff: f32 = v_sin.fsub(tilt_u_cos);
        let rot_q_scaled: f32 = rot_v.fmul(scale_y);
        let rot_u: f32 = fms231(v_sin_alt, tilt_u, rot_cos);
        let rot_p_scaled: f32 = rot_p_diff.fmul(scale_x);
        let shear_prod_b: f32 = rot_q_scaled.fmul(shear_x);
        let shear_prod_a: f32 = rot_u.fmul(shear_y_scaled);
        let neg_rot_p: f32 = flip(rot_p_scaled);
        let sheared_q_rot: f32 = shear_prod_a.fadd(rot_q_scaled);
        let sheared_p_rot: f32 = shear_prod_b.fadd(neg_rot_p);
        let wrapped_sheared_p: f32 = clamp(sheared_p_rot);
        first_out[i] = wrapped_sheared_p;
        second_out[i] = sheared_q_rot;
    }
}

/// Rotation and shear, both coordinates are clamped.
fn rot_shear_wrapxy(p: &Persp, planes: Planes, n: usize) {
    let Persp { scale_x, scale_y, shear_x, shear_y, perspective, shift_x, shift_y, rot_sin, rot_cos, tilt_sin, tilt_cos, axis_u, axis_v, sin_over_persp, cos_over_persp, sin_times_persp, cos_times_persp, .. } = *p;
    let Planes { x, y, first_out, second_out } = planes;
    let ca_positive = cos_over_persp > 0.0;
    let half_minus_ca: f32 = C_ONE_HALF.fsub(cos_over_persp);
    let ca_over_eps: f32 = cos_over_persp.fdiv(C_EPS);
    let shear_y_scaled: f32 = shear_y.fmul(scale_x);
    for i in 0..n {
        let yv = y[i];
        let xv = x[i];
        let y_scaled_u: f32 = yv.fmul(axis_u);
        let cross: f32 = fnma231(y_scaled_u, axis_v, xv);
        let denominator: f32 = fnms213(sin_times_persp, cross, tilt_cos);
        let inv_depth: f32 = inverse_depth(denominator, ca_positive, half_minus_ca, cos_over_persp, ca_over_eps);
        let x_scaled_u: f32 = xv.fmul(axis_u);
        let dot_along: f32 = fma213(yv, axis_v, x_scaled_u);
        let depth_gain: f32 = perspective.fmul(inv_depth);
        let dot_gain: f32 = dot_along.fmul(depth_gain);
        let numer_a: f32 = fms213(cross, cos_times_persp, tilt_sin);
        let proj_u: f32 = fma213(axis_u, dot_gain, shift_x);
        let numer_b: f32 = fma213(numer_a, inv_depth, sin_over_persp);
        let proj_v: f32 = fma213(axis_u, numer_b, shift_y);
        let tilt_u: f32 = fnma213(axis_v, numer_b, proj_u);
        let res_u: f32 = fms231(proj_u, numer_b, axis_v);
        let res_v: f32 = fma231(proj_v, dot_gain, axis_v);
        let tilt_u_cos: f32 = tilt_u.fmul(rot_cos);
        let u_sin: f32 = res_u.fmul(rot_sin);
        let v_sin: f32 = res_v.fmul(rot_sin);
        let rot_v: f32 = fms231(u_sin, rot_cos, res_v);
        let v_sin_alt: f32 = rot_sin.fmul(res_v);
        let rot_p_diff: f32 = v_sin.fsub(tilt_u_cos);
        let rot_q_scaled: f32 = rot_v.fmul(scale_y);
        let rot_u: f32 = fms231(v_sin_alt, tilt_u, rot_cos);
        let rot_p_scaled: f32 = rot_p_diff.fmul(scale_x);
        let shear_prod_a: f32 = rot_u.fmul(shear_y_scaled);
        let neg_rot_p: f32 = flip(rot_p_scaled);
        let sheared_q_rot: f32 = shear_prod_a.fadd(rot_q_scaled);
        let shear_prod_b: f32 = rot_q_scaled.fmul(shear_x);
        let sheared_p_rot: f32 = shear_prod_b.fadd(neg_rot_p);
        let wrapped_sheared_p: f32 = clamp(sheared_p_rot);
        let wrapped_sheared_q: f32 = clamp(sheared_q_rot);
        first_out[i] = wrapped_sheared_p;
        second_out[i] = wrapped_sheared_q;
    }
}

/// Runs the perspective loops for `n > 0` elements, choosing the loop variant in the order rotation, shear, wrap-x, wrap-y.
pub(crate) fn render(st: &State, s: &Setup, x: &[f32], y: &[f32], first_out: &mut [f32], second_out: &mut [f32], n: usize) {
    let p = Persp {
        scale_x: st.scale[0],
        scale_y: st.scale[1],
        shear_x: st.shear[0],
        shear_y: st.shear[1],
        perspective: st.perspective,
        shift_x: st.shift[0],
        shift_y: st.shift[1],
        rot_sin: s.rot_sin,
        rot_cos: s.rot_cos,
        tilt_sin: s.tilt_sin,
        tilt_cos: s.tilt_cos,
        axis_u: s.axis_u,
        axis_v: s.axis_v,
        sin_over_persp: s.sin_over_persp,
        cos_over_persp: s.cos_over_persp,
        sin_times_persp: s.sin_times_persp,
        cos_times_persp: s.cos_times_persp,
        shear_flag: s.shear_flag,
        wrap_x: st.wrap[0],
        wrap_y: st.wrap[1],
    };
    let planes = Planes { x, y, first_out, second_out };
    if !s.rot_flag {
        general(&p, planes, n);
    } else if !s.shear_flag {
        if p.wrap_x != 0 {
            rot_plain(&p, planes, n);
        } else if p.wrap_y == 0 {
            rot_plain_wrapxy(&p, planes, n);
        } else {
            rot_plain_wrapx(&p, planes, n);
        }
    } else if p.wrap_x == 0 {
        if p.wrap_y == 0 {
            rot_shear_wrapxy(&p, planes, n);
        } else {
            rot_shear_wrapx(&p, planes, n);
        }
    } else if p.wrap_y == 0 {
        rot_shear_wrapy(&p, planes, n);
    } else {
        rot_shear(&p, planes, n);
    }
}
