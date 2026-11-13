//! Device kernel source of the tilt op: kernel `tilt_f32`, a port of `tilt::render` (the tilt2 = 0 loop, and in `perspective.rs`
//! the eight perspective loops). The host computes the prologue with the crate's own `tilt::setup` (both sincosf calls, the flags
//! and every derived constant) and passes the results as bit patterns in one word buffer, so no sincos runs on the device.
//!
//! Word layout of the parameter buffer: 0 perspective flag, 1 rotate flag, 2 shear flag, 3 wrap x, 4 wrap y, 5 scale x, 6 scale y,
//! 7 shear x, 8 shear y, 9 perspective, 10 shift x, 11 shift y, 12 rotate sin, 13 rotate cos, 14 tilt sin, 15 tilt cos, 16 axis u,
//! 17 axis v, 18 sin over perspective, 19 cos over perspective, 20 sin times perspective, 21 cos times perspective, 22 cosine term
//! positive, 23 cosine term over epsilon, 24 one and a half minus the cosine term, 25 shear y times scale x.
//!
//! The variant is chosen once per launch from the flags in the crate's order (rotate, shear, wrap x, wrap y). Every add, subtract,
//! multiply and divide goes through the `nanx_*` helpers (first NaN operand quieted, else the default NaN); every fused step is the
//! explicit `fma` with the crate's `nanop` NaN priority (destination, second, third source).

pub const TILT: &str = r#"
/* MIRRORS_MUL_ADD: tilt::perspective::general, rot_shear, rot_plain, rot_shear_wrapy, rot_plain_wrapx, rot_plain_wrapxy, rot_shear_wrapx, rot_shear_wrapxy */

float tl_add(float a, float b) { return nanx_add(a, b); }
float tl_sub(float a, float b) { return nanx_sub(a, b); }
float tl_mul(float a, float b) { return nanx_mul(a, b); }
float tl_div(float a, float b) { return nanx_div(a, b); }

float tl_flip(float x) {
    return as_float(as_uint(x) ^ 0x80000000u);
}

/* fused step with the NaN priority d, s2, s3 of fx_common::nanop */
float tl_fz(float r, float d, float s2, float s3) {
    return isnan(r) ? nanx_pick3(d, s2, s3) : r;
}
float tl_fma213(float d, float s2, float s3) { return tl_fz(fma(d, s2, s3), d, s2, s3); }
float tl_fma231(float d, float s2, float s3) { return tl_fz(fma(s2, s3, d), d, s2, s3); }
float tl_fma132(float d, float s2, float s3) { return tl_fz(fma(d, s3, s2), d, s2, s3); }
float tl_fms213(float d, float s2, float s3) { return tl_fz(fma(d, s2, -s3), d, s2, s3); }
float tl_fms231(float d, float s2, float s3) { return tl_fz(fma(s2, s3, -d), d, s2, s3); }
float tl_fms132(float d, float s2, float s3) { return tl_fz(fma(d, s3, -s2), d, s2, s3); }
float tl_fnma213(float d, float s2, float s3) { return tl_fz(fma(-d, s2, s3), d, s2, s3); }
float tl_fnma231(float d, float s2, float s3) { return tl_fz(fma(-s2, s3, d), d, s2, s3); }
float tl_fnms213(float d, float s2, float s3) { return tl_fz(fma(-d, s2, -s3), d, s2, s3); }

/* tilt::clamp: a NaN is quieted, otherwise the double precision clamp to [-63.9, 63.9] */
float tl_clamp(float x) {
    if (isnan(x)) {
        return as_float(as_uint(x) | 0x00400000u);
    }
    double d = (double)x;
    double lo = as_double(0xc04ff33333333333ul);
    double hi = as_double(0x404ff33333333333ul);
    double r = (lo > d) ? lo : ((hi < d) ? hi : d);
    return (float)r;
}

/* perspective::inverse_depth */
float tl_invd(float den, int ca_positive, float hm_ca, float cos_over_persp, float ca_over_eps) {
    int unordered = isnan(den) ? 1 : 0;
    if (unordered == 0 && den == 0x0p+0f) {
        return ca_over_eps;
    }
    int pos = (den > 0x0p+0f) ? 1 : 0;
    if ((ca_positive != 0) == (pos != 0)) {
        return tl_div(hm_ca, den);
    }
    return tl_div(cos_over_persp, tl_flip(den));
}

void tilt_v_general(__global const uint* pr, float xv, float yv, float* o1, float* o2) {
    float scale_x = as_float(pr[5]);
    float scale_y = as_float(pr[6]);
    float shear_x = as_float(pr[7]);
    float shear_y = as_float(pr[8]);
    float perspective = as_float(pr[9]);
    float shift_x = as_float(pr[10]);
    float shift_y = as_float(pr[11]);
    float rot_sin = as_float(pr[12]);
    float rot_cos = as_float(pr[13]);
    float tilt_sin = as_float(pr[14]);
    float tilt_cos = as_float(pr[15]);
    float axis_u = as_float(pr[16]);
    float axis_v = as_float(pr[17]);
    float sin_over_persp = as_float(pr[18]);
    float cos_over_persp = as_float(pr[19]);
    float sin_times_persp = as_float(pr[20]);
    float cos_times_persp = as_float(pr[21]);
    float ca_over_eps = as_float(pr[23]);
    float hm_ca = as_float(pr[24]);
    float shear_y_scaled = as_float(pr[25]);
    uint shear_flag = pr[2];
    uint wrap_x = pr[3];
    uint wrap_y = pr[4];
    int ca_positive = (int)pr[22];
    float y_scaled_u = tl_mul(yv, axis_u);
    float cross = tl_fnma231(y_scaled_u, xv, axis_v);
    float denominator = tl_fnms213(sin_times_persp, cross, tilt_cos);
    float inv_depth = tl_invd(denominator, ca_positive, hm_ca, cos_over_persp, ca_over_eps);
    float x_scaled_u = tl_mul(xv, axis_u);
    float depth_gain = tl_mul(perspective, inv_depth);
    float numer_a = tl_fms213(cross, cos_times_persp, tilt_sin);
    float dot_along = tl_fma213(yv, axis_v, x_scaled_u);
    float numer_b = tl_fma213(numer_a, inv_depth, sin_over_persp);
    float dot_gain = tl_mul(dot_along, depth_gain);
    float proj_u = tl_fma213(axis_u, dot_gain, shift_x);
    float res_u = tl_fms231(proj_u, numer_b, axis_v);
    float proj_v = tl_fma213(numer_b, axis_u, shift_y);
    float scaled_u = tl_mul(res_u, scale_x);
    float res_v = tl_fma231(proj_v, dot_gain, axis_v);
    float neg_u = tl_flip(scaled_u);
    float scaled_v = tl_mul(res_v, scale_y);
    float pre_p; float pre_q;
    if (shear_flag != 0u) {
    float shear_by_x = tl_mul(scaled_v, shear_x);
    float shear_by_y = tl_mul(neg_u, shear_y);
    float sheared_p = tl_add(shear_by_x, neg_u);
    float sheared_q = tl_add(shear_by_y, scaled_v);
    pre_p = sheared_p; pre_q = sheared_q;
    } else {
    pre_p = neg_u; pre_q = scaled_v;
    }
    float out_p = (wrap_x == 0u) ? tl_clamp(pre_p) : pre_p;
    float out_q = (wrap_y == 0u) ? tl_clamp(pre_q) : pre_q;
    *o1 = out_p;
    *o2 = out_q;
}

void tilt_v_rot_shear(__global const uint* pr, float xv, float yv, float* o1, float* o2) {
    float scale_x = as_float(pr[5]);
    float scale_y = as_float(pr[6]);
    float shear_x = as_float(pr[7]);
    float shear_y = as_float(pr[8]);
    float perspective = as_float(pr[9]);
    float shift_x = as_float(pr[10]);
    float shift_y = as_float(pr[11]);
    float rot_sin = as_float(pr[12]);
    float rot_cos = as_float(pr[13]);
    float tilt_sin = as_float(pr[14]);
    float tilt_cos = as_float(pr[15]);
    float axis_u = as_float(pr[16]);
    float axis_v = as_float(pr[17]);
    float sin_over_persp = as_float(pr[18]);
    float cos_over_persp = as_float(pr[19]);
    float sin_times_persp = as_float(pr[20]);
    float cos_times_persp = as_float(pr[21]);
    float ca_over_eps = as_float(pr[23]);
    float hm_ca = as_float(pr[24]);
    float shear_y_scaled = as_float(pr[25]);
    uint shear_flag = pr[2];
    uint wrap_x = pr[3];
    uint wrap_y = pr[4];
    int ca_positive = (int)pr[22];
    float y_scaled_u = tl_mul(yv, axis_u);
    float cross = tl_fnma231(y_scaled_u, axis_v, xv);
    float denominator = tl_fnms213(sin_times_persp, cross, tilt_cos);
    float inv_depth = tl_invd(denominator, ca_positive, hm_ca, cos_over_persp, ca_over_eps);
    float x_scaled_u = tl_mul(xv, axis_u);
    float depth_gain = tl_mul(perspective, inv_depth);
    float numer_a = tl_fms213(cross, cos_times_persp, tilt_sin);
    float dot_along = tl_fma213(yv, axis_v, x_scaled_u);
    float numer_b = tl_fma213(numer_a, inv_depth, sin_over_persp);
    float dot_gain = tl_mul(dot_along, depth_gain);
    float proj_v = tl_fma213(axis_u, numer_b, shift_y);
    float proj_u = tl_fma213(axis_u, dot_gain, shift_x);
    float res_v = tl_fma231(proj_v, dot_gain, axis_v);
    float tilt_u = tl_fnma213(axis_v, numer_b, proj_u);
    float res_u = tl_fms231(proj_u, numer_b, axis_v);
    float v_sin = tl_mul(res_v, rot_sin);
    float u_sin = tl_mul(res_u, rot_sin);
    float tilt_u_cos = tl_mul(tilt_u, rot_cos);
    float rot_v = tl_fms231(u_sin, rot_cos, res_v);
    float v_sin_alt = tl_mul(rot_sin, res_v);
    float rot_p_diff = tl_sub(v_sin, tilt_u_cos);
    float rot_u = tl_fms231(v_sin_alt, tilt_u, rot_cos);
    float rot_p_scaled = tl_mul(rot_p_diff, scale_x);
    float rot_q_scaled = tl_mul(scale_y, rot_v);
    float shear_prod_a = tl_mul(rot_u, shear_y_scaled);
    float shear_prod_b = tl_mul(rot_q_scaled, shear_x);
    float sheared_q_rot = tl_add(shear_prod_a, rot_q_scaled);
    float neg_rot_p = tl_flip(rot_p_scaled);
    float sheared_p_rot = tl_add(shear_prod_b, neg_rot_p);
    *o1 = sheared_p_rot;
    *o2 = sheared_q_rot;
}

void tilt_v_rot_plain(__global const uint* pr, float xv, float yv, float* o1, float* o2) {
    float scale_x = as_float(pr[5]);
    float scale_y = as_float(pr[6]);
    float shear_x = as_float(pr[7]);
    float shear_y = as_float(pr[8]);
    float perspective = as_float(pr[9]);
    float shift_x = as_float(pr[10]);
    float shift_y = as_float(pr[11]);
    float rot_sin = as_float(pr[12]);
    float rot_cos = as_float(pr[13]);
    float tilt_sin = as_float(pr[14]);
    float tilt_cos = as_float(pr[15]);
    float axis_u = as_float(pr[16]);
    float axis_v = as_float(pr[17]);
    float sin_over_persp = as_float(pr[18]);
    float cos_over_persp = as_float(pr[19]);
    float sin_times_persp = as_float(pr[20]);
    float cos_times_persp = as_float(pr[21]);
    float ca_over_eps = as_float(pr[23]);
    float hm_ca = as_float(pr[24]);
    float shear_y_scaled = as_float(pr[25]);
    uint shear_flag = pr[2];
    uint wrap_x = pr[3];
    uint wrap_y = pr[4];
    int ca_positive = (int)pr[22];
    float y_scaled_u = tl_mul(yv, axis_u);
    float cross = tl_fnma231(y_scaled_u, axis_v, xv);
    float denominator = tl_fnms213(sin_times_persp, cross, tilt_cos);
    float inv_depth = tl_invd(denominator, ca_positive, hm_ca, cos_over_persp, ca_over_eps);
    float x_scaled_u = tl_mul(xv, axis_u);
    float numer_a = tl_fms213(cross, cos_times_persp, tilt_sin);
    float dot_along = tl_fma213(yv, axis_v, x_scaled_u);
    float depth_gain = tl_mul(perspective, inv_depth);
    float numer_b = tl_fma213(numer_a, inv_depth, sin_over_persp);
    float dot_gain = tl_mul(dot_along, depth_gain);
    float proj_v = tl_fma213(axis_u, numer_b, shift_y);
    float proj_u = tl_fma213(axis_u, dot_gain, shift_x);
    float res_v = tl_fma231(proj_v, dot_gain, axis_v);
    float tilt_u = tl_fnma213(axis_v, numer_b, proj_u);
    float res_u = tl_fms231(proj_u, numer_b, axis_v);
    float res_v_cos = tl_mul(res_v, rot_cos);
    float v_sin = tl_mul(res_v, rot_sin);
    float rot_v_plain = tl_fms213(res_u, rot_sin, res_v_cos);
    float rot_u = tl_fms213(tilt_u, rot_cos, v_sin);
    float rot_q_plain = tl_mul(rot_v_plain, scale_y);
    float neg_rot_q = tl_flip(rot_q_plain);
    float out_q = (wrap_y == 0u) ? tl_clamp(neg_rot_q) : neg_rot_q;
    float rot_p_plain = tl_mul(scale_x, rot_u);
    *o1 = rot_p_plain;
    *o2 = out_q;
}

void tilt_v_rot_shear_wrapy(__global const uint* pr, float xv, float yv, float* o1, float* o2) {
    float scale_x = as_float(pr[5]);
    float scale_y = as_float(pr[6]);
    float shear_x = as_float(pr[7]);
    float shear_y = as_float(pr[8]);
    float perspective = as_float(pr[9]);
    float shift_x = as_float(pr[10]);
    float shift_y = as_float(pr[11]);
    float rot_sin = as_float(pr[12]);
    float rot_cos = as_float(pr[13]);
    float tilt_sin = as_float(pr[14]);
    float tilt_cos = as_float(pr[15]);
    float axis_u = as_float(pr[16]);
    float axis_v = as_float(pr[17]);
    float sin_over_persp = as_float(pr[18]);
    float cos_over_persp = as_float(pr[19]);
    float sin_times_persp = as_float(pr[20]);
    float cos_times_persp = as_float(pr[21]);
    float ca_over_eps = as_float(pr[23]);
    float hm_ca = as_float(pr[24]);
    float shear_y_scaled = as_float(pr[25]);
    uint shear_flag = pr[2];
    uint wrap_x = pr[3];
    uint wrap_y = pr[4];
    int ca_positive = (int)pr[22];
    float y_scaled_u = tl_mul(yv, axis_u);
    float cross = tl_fnma231(y_scaled_u, axis_v, xv);
    float denominator = tl_fnms213(sin_times_persp, cross, tilt_cos);
    float inv_depth = tl_invd(denominator, ca_positive, hm_ca, cos_over_persp, ca_over_eps);
    float x_scaled_u = tl_mul(xv, axis_u);
    float depth_gain = tl_mul(inv_depth, perspective);
    float dot_along = tl_fma213(yv, axis_v, x_scaled_u);
    float dot_gain = tl_mul(dot_along, depth_gain);
    float numer_a = tl_fms132(cross, tilt_sin, cos_times_persp);
    float proj_u = tl_fma213(axis_u, dot_gain, shift_x);
    float numer_b = tl_fma213(numer_a, inv_depth, sin_over_persp);
    float proj_v = tl_fma213(axis_u, numer_b, shift_y);
    float tilt_u = tl_fnma213(axis_v, numer_b, proj_u);
    float res_u = tl_fms231(proj_u, numer_b, axis_v);
    float res_v = tl_fma231(proj_v, dot_gain, axis_v);
    float tilt_u_cos = tl_mul(tilt_u, rot_cos);
    float u_sin = tl_mul(res_u, rot_sin);
    float v_sin = tl_mul(res_v, rot_sin);
    float rot_v = tl_fms231(u_sin, rot_cos, res_v);
    float v_sin_alt = tl_mul(rot_sin, res_v);
    float rot_p_diff = tl_sub(v_sin, tilt_u_cos);
    float rot_q_scaled = tl_mul(scale_y, rot_v);
    float rot_u = tl_fms231(v_sin_alt, tilt_u, rot_cos);
    float rot_p_scaled = tl_mul(rot_p_diff, scale_x);
    float shear_prod_b = tl_mul(rot_q_scaled, shear_x);
    float sheared_q_fused = tl_fma132(rot_u, rot_q_scaled, shear_y_scaled);
    float neg_rot_p = tl_flip(rot_p_scaled);
    float sheared_p_rot = tl_add(shear_prod_b, neg_rot_p);
    float wrapped_q_fused = tl_clamp(sheared_q_fused);
    *o1 = sheared_p_rot;
    *o2 = wrapped_q_fused;
}

void tilt_v_rot_plain_wrapx(__global const uint* pr, float xv, float yv, float* o1, float* o2) {
    float scale_x = as_float(pr[5]);
    float scale_y = as_float(pr[6]);
    float shear_x = as_float(pr[7]);
    float shear_y = as_float(pr[8]);
    float perspective = as_float(pr[9]);
    float shift_x = as_float(pr[10]);
    float shift_y = as_float(pr[11]);
    float rot_sin = as_float(pr[12]);
    float rot_cos = as_float(pr[13]);
    float tilt_sin = as_float(pr[14]);
    float tilt_cos = as_float(pr[15]);
    float axis_u = as_float(pr[16]);
    float axis_v = as_float(pr[17]);
    float sin_over_persp = as_float(pr[18]);
    float cos_over_persp = as_float(pr[19]);
    float sin_times_persp = as_float(pr[20]);
    float cos_times_persp = as_float(pr[21]);
    float ca_over_eps = as_float(pr[23]);
    float hm_ca = as_float(pr[24]);
    float shear_y_scaled = as_float(pr[25]);
    uint shear_flag = pr[2];
    uint wrap_x = pr[3];
    uint wrap_y = pr[4];
    int ca_positive = (int)pr[22];
    float y_scaled_u = tl_mul(yv, axis_u);
    float cross = tl_fnma231(y_scaled_u, axis_v, xv);
    float denominator = tl_fnms213(sin_times_persp, cross, tilt_cos);
    float inv_depth = tl_invd(denominator, ca_positive, hm_ca, cos_over_persp, ca_over_eps);
    float x_scaled_u = tl_mul(xv, axis_u);
    float depth_gain = tl_mul(inv_depth, perspective);
    float dot_along = tl_fma213(yv, axis_v, x_scaled_u);
    float dot_gain = tl_mul(dot_along, depth_gain);
    float numer_a = tl_fms213(cross, cos_times_persp, tilt_sin);
    float proj_u = tl_fma213(axis_u, dot_gain, shift_x);
    float numer_b = tl_fma213(numer_a, inv_depth, sin_over_persp);
    float proj_v = tl_fma213(axis_u, numer_b, shift_y);
    float tilt_u = tl_fnma213(axis_v, numer_b, proj_u);
    float res_u = tl_fms231(proj_u, numer_b, axis_v);
    float res_v = tl_fma231(proj_v, dot_gain, axis_v);
    float tilt_u_cos = tl_mul(tilt_u, rot_cos);
    float u_sin = tl_mul(res_u, rot_sin);
    float rot_v = tl_fms231(u_sin, rot_cos, res_v);
    float v_sin = tl_mul(res_v, rot_sin);
    float rot_p_diff = tl_sub(v_sin, tilt_u_cos);
    float rot_p_scaled = tl_mul(rot_p_diff, scale_x);
    float neg_rot_p = tl_flip(rot_p_scaled);
    float wrapped_neg_p = tl_clamp(neg_rot_p);
    float rot_q_scaled = tl_mul(scale_y, rot_v);
    *o1 = wrapped_neg_p;
    *o2 = rot_q_scaled;
}

void tilt_v_rot_plain_wrapxy(__global const uint* pr, float xv, float yv, float* o1, float* o2) {
    float scale_x = as_float(pr[5]);
    float scale_y = as_float(pr[6]);
    float shear_x = as_float(pr[7]);
    float shear_y = as_float(pr[8]);
    float perspective = as_float(pr[9]);
    float shift_x = as_float(pr[10]);
    float shift_y = as_float(pr[11]);
    float rot_sin = as_float(pr[12]);
    float rot_cos = as_float(pr[13]);
    float tilt_sin = as_float(pr[14]);
    float tilt_cos = as_float(pr[15]);
    float axis_u = as_float(pr[16]);
    float axis_v = as_float(pr[17]);
    float sin_over_persp = as_float(pr[18]);
    float cos_over_persp = as_float(pr[19]);
    float sin_times_persp = as_float(pr[20]);
    float cos_times_persp = as_float(pr[21]);
    float ca_over_eps = as_float(pr[23]);
    float hm_ca = as_float(pr[24]);
    float shear_y_scaled = as_float(pr[25]);
    uint shear_flag = pr[2];
    uint wrap_x = pr[3];
    uint wrap_y = pr[4];
    int ca_positive = (int)pr[22];
    float y_scaled_u = tl_mul(yv, axis_u);
    float cross = tl_fnma231(y_scaled_u, axis_v, xv);
    float denominator = tl_fnms213(sin_times_persp, cross, tilt_cos);
    float inv_depth = tl_invd(denominator, ca_positive, hm_ca, cos_over_persp, ca_over_eps);
    float x_scaled_u = tl_mul(xv, axis_u);
    float depth_gain = tl_mul(inv_depth, perspective);
    float numer_a = tl_fms132(cross, tilt_sin, cos_times_persp);
    float dot_along = tl_fma213(yv, axis_v, x_scaled_u);
    float numer_b = tl_fma213(numer_a, inv_depth, sin_over_persp);
    float dot_gain = tl_mul(dot_along, depth_gain);
    float proj_v = tl_fma213(axis_u, numer_b, shift_y);
    float proj_u = tl_fma213(axis_u, dot_gain, shift_x);
    float res_v = tl_fma231(proj_v, dot_gain, axis_v);
    float tilt_u = tl_fnma213(axis_v, numer_b, proj_u);
    float res_u = tl_fms231(proj_u, numer_b, axis_v);
    float v_sin = tl_mul(res_v, rot_sin);
    float tilt_u_cos = tl_mul(tilt_u, rot_cos);
    float u_sin = tl_mul(res_u, rot_sin);
    float rot_p_diff = tl_sub(v_sin, tilt_u_cos);
    float rot_v = tl_fms231(u_sin, rot_cos, res_v);
    float rot_p_scaled = tl_mul(rot_p_diff, scale_x);
    float neg_rot_p = tl_flip(rot_p_scaled);
    float rot_q_scaled = tl_mul(scale_y, rot_v);
    float wrapped_neg_p = tl_clamp(neg_rot_p);
    float wrapped_q = tl_clamp(rot_q_scaled);
    *o1 = wrapped_neg_p;
    *o2 = wrapped_q;
}

void tilt_v_rot_shear_wrapx(__global const uint* pr, float xv, float yv, float* o1, float* o2) {
    float scale_x = as_float(pr[5]);
    float scale_y = as_float(pr[6]);
    float shear_x = as_float(pr[7]);
    float shear_y = as_float(pr[8]);
    float perspective = as_float(pr[9]);
    float shift_x = as_float(pr[10]);
    float shift_y = as_float(pr[11]);
    float rot_sin = as_float(pr[12]);
    float rot_cos = as_float(pr[13]);
    float tilt_sin = as_float(pr[14]);
    float tilt_cos = as_float(pr[15]);
    float axis_u = as_float(pr[16]);
    float axis_v = as_float(pr[17]);
    float sin_over_persp = as_float(pr[18]);
    float cos_over_persp = as_float(pr[19]);
    float sin_times_persp = as_float(pr[20]);
    float cos_times_persp = as_float(pr[21]);
    float ca_over_eps = as_float(pr[23]);
    float hm_ca = as_float(pr[24]);
    float shear_y_scaled = as_float(pr[25]);
    uint shear_flag = pr[2];
    uint wrap_x = pr[3];
    uint wrap_y = pr[4];
    int ca_positive = (int)pr[22];
    float y_scaled_u = tl_mul(yv, axis_u);
    float cross = tl_fnma231(y_scaled_u, axis_v, xv);
    float denominator = tl_fnms213(sin_times_persp, cross, tilt_cos);
    float inv_depth = tl_invd(denominator, ca_positive, hm_ca, cos_over_persp, ca_over_eps);
    float x_scaled_u = tl_mul(xv, axis_u);
    float depth_gain = tl_mul(perspective, inv_depth);
    float dot_along = tl_fma213(yv, axis_v, x_scaled_u);
    float dot_gain = tl_mul(dot_along, depth_gain);
    float numer_a = tl_fms213(cross, cos_times_persp, tilt_sin);
    float proj_u = tl_fma213(axis_u, dot_gain, shift_x);
    float numer_b = tl_fma213(numer_a, inv_depth, sin_over_persp);
    float proj_v = tl_fma213(axis_u, numer_b, shift_y);
    float tilt_u = tl_fnma213(axis_v, numer_b, proj_u);
    float res_u = tl_fms231(proj_u, numer_b, axis_v);
    float res_v = tl_fma231(proj_v, dot_gain, axis_v);
    float u_sin = tl_mul(res_u, rot_sin);
    float tilt_u_cos = tl_mul(tilt_u, rot_cos);
    float v_sin = tl_mul(res_v, rot_sin);
    float rot_v = tl_fms231(u_sin, rot_cos, res_v);
    float v_sin_alt = tl_mul(rot_sin, res_v);
    float rot_p_diff = tl_sub(v_sin, tilt_u_cos);
    float rot_q_scaled = tl_mul(rot_v, scale_y);
    float rot_u = tl_fms231(v_sin_alt, tilt_u, rot_cos);
    float rot_p_scaled = tl_mul(rot_p_diff, scale_x);
    float shear_prod_b = tl_mul(rot_q_scaled, shear_x);
    float shear_prod_a = tl_mul(rot_u, shear_y_scaled);
    float neg_rot_p = tl_flip(rot_p_scaled);
    float sheared_q_rot = tl_add(shear_prod_a, rot_q_scaled);
    float sheared_p_rot = tl_add(shear_prod_b, neg_rot_p);
    float wrapped_sheared_p = tl_clamp(sheared_p_rot);
    *o1 = wrapped_sheared_p;
    *o2 = sheared_q_rot;
}

void tilt_v_rot_shear_wrapxy(__global const uint* pr, float xv, float yv, float* o1, float* o2) {
    float scale_x = as_float(pr[5]);
    float scale_y = as_float(pr[6]);
    float shear_x = as_float(pr[7]);
    float shear_y = as_float(pr[8]);
    float perspective = as_float(pr[9]);
    float shift_x = as_float(pr[10]);
    float shift_y = as_float(pr[11]);
    float rot_sin = as_float(pr[12]);
    float rot_cos = as_float(pr[13]);
    float tilt_sin = as_float(pr[14]);
    float tilt_cos = as_float(pr[15]);
    float axis_u = as_float(pr[16]);
    float axis_v = as_float(pr[17]);
    float sin_over_persp = as_float(pr[18]);
    float cos_over_persp = as_float(pr[19]);
    float sin_times_persp = as_float(pr[20]);
    float cos_times_persp = as_float(pr[21]);
    float ca_over_eps = as_float(pr[23]);
    float hm_ca = as_float(pr[24]);
    float shear_y_scaled = as_float(pr[25]);
    uint shear_flag = pr[2];
    uint wrap_x = pr[3];
    uint wrap_y = pr[4];
    int ca_positive = (int)pr[22];
    float y_scaled_u = tl_mul(yv, axis_u);
    float cross = tl_fnma231(y_scaled_u, axis_v, xv);
    float denominator = tl_fnms213(sin_times_persp, cross, tilt_cos);
    float inv_depth = tl_invd(denominator, ca_positive, hm_ca, cos_over_persp, ca_over_eps);
    float x_scaled_u = tl_mul(xv, axis_u);
    float dot_along = tl_fma213(yv, axis_v, x_scaled_u);
    float depth_gain = tl_mul(perspective, inv_depth);
    float dot_gain = tl_mul(dot_along, depth_gain);
    float numer_a = tl_fms213(cross, cos_times_persp, tilt_sin);
    float proj_u = tl_fma213(axis_u, dot_gain, shift_x);
    float numer_b = tl_fma213(numer_a, inv_depth, sin_over_persp);
    float proj_v = tl_fma213(axis_u, numer_b, shift_y);
    float tilt_u = tl_fnma213(axis_v, numer_b, proj_u);
    float res_u = tl_fms231(proj_u, numer_b, axis_v);
    float res_v = tl_fma231(proj_v, dot_gain, axis_v);
    float tilt_u_cos = tl_mul(tilt_u, rot_cos);
    float u_sin = tl_mul(res_u, rot_sin);
    float v_sin = tl_mul(res_v, rot_sin);
    float rot_v = tl_fms231(u_sin, rot_cos, res_v);
    float v_sin_alt = tl_mul(rot_sin, res_v);
    float rot_p_diff = tl_sub(v_sin, tilt_u_cos);
    float rot_q_scaled = tl_mul(rot_v, scale_y);
    float rot_u = tl_fms231(v_sin_alt, tilt_u, rot_cos);
    float rot_p_scaled = tl_mul(rot_p_diff, scale_x);
    float shear_prod_a = tl_mul(rot_u, shear_y_scaled);
    float neg_rot_p = tl_flip(rot_p_scaled);
    float sheared_q_rot = tl_add(shear_prod_a, rot_q_scaled);
    float shear_prod_b = tl_mul(rot_q_scaled, shear_x);
    float sheared_p_rot = tl_add(shear_prod_b, neg_rot_p);
    float wrapped_sheared_p = tl_clamp(sheared_p_rot);
    float wrapped_sheared_q = tl_clamp(sheared_q_rot);
    *o1 = wrapped_sheared_p;
    *o2 = wrapped_sheared_q;
}

__kernel void tilt_f32(__global const float* x, __global const float* y, __global float* out1, __global float* out2,
                       __global const uint* pr, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float xv = x[i];
    float yv = y[i];
    float p = 0x0p+0f;
    float q = 0x0p+0f;
    if (pr[0] != 0u) {
        uint rot = pr[1];
        uint shear = pr[2];
        uint wx = pr[3];
        uint wy = pr[4];
        if (rot == 0u) {
            tilt_v_general(pr, xv, yv, &p, &q);
        } else if (shear == 0u) {
            if (wx != 0u) {
                tilt_v_rot_plain(pr, xv, yv, &p, &q);
            } else if (wy == 0u) {
                tilt_v_rot_plain_wrapxy(pr, xv, yv, &p, &q);
            } else {
                tilt_v_rot_plain_wrapx(pr, xv, yv, &p, &q);
            }
        } else if (wx == 0u) {
            if (wy == 0u) {
                tilt_v_rot_shear_wrapxy(pr, xv, yv, &p, &q);
            } else {
                tilt_v_rot_shear_wrapx(pr, xv, yv, &p, &q);
            }
        } else if (wy == 0u) {
            tilt_v_rot_shear_wrapy(pr, xv, yv, &p, &q);
        } else {
            tilt_v_rot_shear(pr, xv, yv, &p, &q);
        }
    } else {
        float shift_x = as_float(pr[10]);
        float shift_y = as_float(pr[11]);
        float scale_x = as_float(pr[5]);
        float scale_y = as_float(pr[6]);
        float u = tl_add(shift_x, xv);
        float v = tl_add(shift_y, yv);
        if (pr[1] != 0u) {
            float rot_sin = as_float(pr[12]);
            float rot_cos = as_float(pr[13]);
            float t = u;
            float a = tl_mul(u, rot_cos);
            float b = tl_mul(v, rot_sin);
            float c = tl_mul(v, rot_cos);
            float d = tl_mul(t, rot_sin);
            u = tl_sub(a, b);
            v = tl_add(c, d);
        }
        p = tl_mul(scale_x, u);
        q = tl_mul(scale_y, v);
        if (pr[2] != 0u) {
            float shear_x = as_float(pr[7]);
            float shear_y = as_float(pr[8]);
            float k = tl_mul(q, shear_x);
            float l = tl_mul(p, shear_y);
            float np = tl_add(k, p);
            float nq = tl_add(l, q);
            p = np;
            q = nq;
        }
        if (pr[3] == 0u) {
            p = tl_clamp(p);
        }
        if (pr[4] == 0u) {
            q = tl_clamp(q);
        }
    }
    out1[i] = p;
    out2[i] = q;
}
"#;
