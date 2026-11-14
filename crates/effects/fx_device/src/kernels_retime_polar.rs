//! Device kernels of `xy_to_polar` and `hsv_to_rgb`: OpenCL C 1.2 as a Rust string constant, registered in
//! [`crate::kernels::SOURCES`].
//!
//! Each kernel computes one output element with the operations of the CPU crate function it ports (`xy_to_polar::xy_to_polar`,
//! `hsv_to_rgb::pixel`), in that function's order. Every NaN is produced by explicit tests: a two operand site takes the first NaN
//! operand in instruction order, quieted by OR 0x00400000, else the default NaN 0xffc00000 (the `rtp_` helpers below mirror
//! `fx_common::nanop`); a fused site is `fma` with the same priority order. `fma` appears only where the crate calls `mul_add`.
//! Host side: the vector body range of `xy_to_polar` (`vector_body`) is computed by the crate on the host and passed as the
//! index range `[lo, hi)`; a lane whose scalar atan2 falls in the branch the crate refuses raises the refusal flag word.

/// The kernels of both ops and their shared NaN helpers.
/// The `fma` calls mirror `mul_add` of `xy_to_polar` (atan2_vec, angle, mag_scalar, mag_vector) and `hsv_to_rgb` (sextant); the marker comment is in the text.
pub const RETIME_POLAR: &str = r#"
/* MIRRORS_MUL_ADD: xy_to_polar::atan2_vec (mul_add), xy_to_polar::angle (fms213), xy_to_polar::mag_scalar and mag_vector (fma213), hsv_to_rgb::sextant (fnma213) */
/* the first NaN of two operands, quieted, else the default NaN */
float rtp_pick2(float x, float y) {
    uint ux = as_uint(x);
    uint uy = as_uint(y);
    return isnan(x) ? as_float(ux | 0x00400000u) : (isnan(y) ? as_float(uy | 0x00400000u) : as_float(0xffc00000u));
}

/* the first NaN of three operands in instruction order, quieted, else the default NaN */
float rtp_pick3(float a, float b, float c) {
    uint ua = as_uint(a);
    uint ub = as_uint(b);
    uint uc = as_uint(c);
    return isnan(a) ? as_float(ua | 0x00400000u) : (isnan(b) ? as_float(ub | 0x00400000u) : (isnan(c) ? as_float(uc | 0x00400000u) : as_float(0xffc00000u)));
}

float rtp_add(float x, float y) {
    float r = x + y;
    return isnan(r) ? rtp_pick2(x, y) : r;
}

float rtp_sub(float x, float y) {
    float r = x - y;
    return isnan(r) ? rtp_pick2(x, y) : r;
}

float rtp_mul(float x, float y) {
    float r = x * y;
    return isnan(r) ? rtp_pick2(x, y) : r;
}

/* vfmadd213ss: d * second + third */
float rtp_fma213(float d, float second, float third) {
    float r = fma(d, second, third);
    return isnan(r) ? rtp_pick3(d, second, third) : r;
}

/* vfmsub213ss: d * second - third */
float rtp_fms213(float d, float second, float third) {
    float r = fma(d, second, -third);
    return isnan(r) ? rtp_pick3(d, second, third) : r;
}

/* vfnmadd213ss: -(d * second) + third */
float rtp_fnma213(float d, float second, float third) {
    float r = fma(-d, second, third);
    return isnan(r) ? rtp_pick3(d, second, third) : r;
}

float rtp_floor(float v) {
    return isnan(v) ? as_float(as_uint(v) | 0x00400000u) : floor(v);
}

/* xy_to_polar::in_vector_range */
int rtp_in_range(float v) {
    int t = as_int((as_uint(v) & 0x7fffffffu) + 0x7f000000u);
    return t < as_int(0xfc000000u);
}

/* xy_to_polar::atan2_vec: the polynomial for one lane whose two arguments are in range */
float rtp_atan2_vec(float y, float x) {
    float abs_x = as_float(as_uint(x) & 0x7fffffffu);
    float ay = as_float(as_uint(y) & 0x7fffffffu);
    int lt = ay < abs_x;
    float num = lt ? ay : as_float(as_uint(abs_x) | 0x80000000u);
    float den = lt ? abs_x : ay;
    int le = x <= 0x0p+0f;
    float q = num / den;
    uint sx = as_uint(x) ^ as_uint(abs_x);
    uint sy = as_uint(y) ^ as_uint(ay);
    float z = q * q;
    float z_sq = z * z;
    float poly_hi = as_float(0xbc7f2631u);
    float poly_lo = as_float(0x3b322cc0u);
    poly_hi = fma(poly_hi, z_sq, as_float(0xbd987629u));
    poly_lo = fma(poly_lo, z_sq, as_float(0x3d2bc384u));
    poly_hi = fma(poly_hi, z_sq, as_float(0xbe1161f8u));
    poly_lo = fma(poly_lo, z_sq, as_float(0x3dd96474u));
    poly_hi = fma(poly_hi, z_sq, as_float(0xbeaaaa49u));
    poly_lo = fma(poly_lo, z_sq, as_float(0x3e4cb79fu));
    poly_lo = fma(poly_lo, z_sq, 0x1p+0f);
    poly_hi = fma(poly_hi, z, poly_lo);
    float quad_offset = lt ? 0x0p+0f : as_float(0x3fc90fdbu);
    float r = fma(poly_hi, q, quad_offset);
    float pi_offset = le ? as_float(0x40490fdbu) : 0x0p+0f;
    float r_signed = as_float(as_uint(r) | sx);
    float r_adjusted = r_signed + pi_offset;
    return as_float(as_uint(r_adjusted) | sy);
}

double rtp_pair(ulong hi, ulong lo) {
    return as_double(hi) + as_double(lo);
}

float rtp_with_sign(double res, int neg) {
    ulong b = (as_ulong(res) & 0x7fffffffffffffffUL) | (neg ? 0x8000000000000000UL : 0UL);
    return (float)as_double(b);
}

int rtp_dexp(float v) {
    return (int)((as_ulong((double)v) >> 52) & 0x7ffUL);
}

/* xy_to_polar::atan2_scalar; *refuse is set for the branch the crate refuses by name */
float rtp_atan2_scalar(float y, float x, __private uint* refuse) {
    float yp = rtp_mul(y, 0x1p+0f);
    float xp = rtp_mul(x, 0x1p+0f);
    int yneg = (as_uint(yp) >> 31) != 0u;
    int xneg = (as_uint(xp) >> 31) != 0u;
    uint ye = (as_uint(yp) >> 23) & 0xffu;
    uint xe = (as_uint(xp) >> 23) & 0xffu;
    if (ye == 0xffu || xe == 0xffu) {
        if (isnan(yp) || isnan(xp)) {
            return rtp_add(yp, xp);
        }
        if (xe != 0xffu) {
            return rtp_with_sign(rtp_pair(0x3ff921fb54442d18UL, 0x3c91a62633145c07UL), yneg);
        }
        if (ye == 0xffu) {
            double p = !xneg ? rtp_pair(0x3fe921fb54442d18UL, 0x3c81a62633145c07UL) : rtp_pair(0x4002d97c7f3321d2UL, 0x3c9a79394c9e8a0aUL);
            return rtp_with_sign(p, yneg);
        }
        return !xneg ? rtp_with_sign(0x0p+0, yneg) : rtp_with_sign(rtp_pair(0x400921fb54442d18UL, 0x3ca1a62633145c07UL), yneg);
    }
    if (yp == 0x0p+0f) {
        return xneg ? rtp_with_sign(rtp_pair(0x400921fb54442d18UL, 0x3ca1a62633145c07UL), yneg) : rtp_with_sign(0x0p+0, yneg);
    }
    if (xp == 0x0p+0f) {
        return rtp_with_sign(rtp_pair(0x3ff921fb54442d18UL, 0x3c91a62633145c07UL), yneg);
    }
    int d = rtp_dexp(yp) - rtp_dexp(xp);
    if (d <= -54) {
        if (xneg) {
            return rtp_with_sign(rtp_pair(0x400921fb54442d18UL, 0x3ca1a62633145c07UL), yneg);
        }
        double q = (double)fabs(yp) / (double)fabs(xp);
        float qf = (float)q;
        double res;
        if (((as_uint(qf) >> 23) & 0xffu) != 0u) {
            res = q * (0x1p+0 + q);
        } else {
            float qf_sq = qf * qf;
            res = (double)qf_sq + q;
        }
        return rtp_with_sign(res, yneg);
    }
    if (d >= 74) {
        return rtp_with_sign(rtp_pair(0x3ff921fb54442d18UL, 0x3c91a62633145c07UL), yneg);
    }
    if (d >= 54) {
        double xd = (double)xp / (double)fabs(yp);
        double res = (as_double(0x3c91a62633145c07UL) - xd) + as_double(0x3ff921fb54442d18UL);
        return rtp_with_sign(res, yneg);
    }
    *refuse = 1u;
    return 0x0p+0f;
}

float rtp_atan2_lane(float y, float x, __private uint* refuse) {
    if (rtp_in_range(y) && rtp_in_range(x)) {
        return rtp_atan2_vec(y, x);
    }
    return rtp_atan2_scalar(y, x, refuse);
}

/* xy_to_polar::mag_scalar */
float rtp_mag_scalar(float x, float y) {
    float y_sq = rtp_mul(y, y);
    float s = rtp_fma213(x, x, y_sq);
    return isnan(s) ? as_float(as_uint(s) | 0x00400000u) : sqrt(s);
}

/* xy_to_polar::mag_vector with the modelled reciprocal square root 1 / sqrt(s) */
float rtp_mag_vector(float x, float y) {
    float y_sq = rtp_mul(y, y);
    float s = rtp_fma213(x, x, y_sq);
    float a = as_float(as_uint(s) & 0x7fffffffu);
    int m = a >= as_float(0x00800000u);
    float r = m ? (0x1p+0f / sqrt(s)) : 0x0p+0f;
    float t = rtp_mul(s, r);
    float u = rtp_mul(t, r);
    float v = rtp_sub(u, 0x1.8p+1f);
    float w = rtp_mul(t, v);
    return rtp_mul(w, -0x1p-1f);
}

/* One pixel of xy_to_polar: ang[i] = angle(atan2(y, x)), mag[i] by region. A refused lane raises the flag word and writes nothing. */
__kernel void rtp_xy_to_polar(__global const float* xs, __global const float* ys, __global float* ang, __global float* mag,
                              uint n, uint dirbits, uint lo, uint hi, __global uint* refused) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float x = xs[i];
    float y = ys[i];
    uint refuse = 0u;
    float a = rtp_atan2_lane(y, x, &refuse);
    if (refuse != 0u) {
        atomic_or(refused, 1u);
        return;
    }
    float v = rtp_fms213(a, as_float(0x3e22f983u), as_float(dirbits));
    float fl = rtp_floor(v);
    ang[i] = rtp_sub(v, fl);
    mag[i] = (i >= (size_t)lo && i < (size_t)hi) ? rtp_mag_vector(x, y) : rtp_mag_scalar(x, y);
}

/* hsv_to_rgb::sextant for the plane form (konst false); out[3] holds the red, green and blue words */
void rtp_sextant(float hh, float p, float v, uint vb, __private uint* out) {
    float frac = rtp_sub(hh, rtp_floor(hh));
    float t = rtp_mul(0x1.8p+2f, frac);
    if (t >= 0x1.8p+2f) {
        t = 0x0p+0f;
    }
    int idx = cvtt_i32_f(t);
    float f = rtp_sub(t, (float)idx);
    float q = rtp_sub(v, p);
    if (idx < 3) {
        if (idx == 0) {
            float w = rtp_sub(0x1p+0f, f);
            out[0] = vb;
            out[1] = as_uint(rtp_fnma213(w, p, v));
            out[2] = as_uint(q);
        } else if (idx == 1) {
            out[0] = as_uint(rtp_fnma213(f, p, v));
            out[1] = vb;
            out[2] = as_uint(q);
        } else {
            float w = rtp_sub(0x1p+0f, f);
            out[0] = as_uint(q);
            out[1] = vb;
            out[2] = as_uint(rtp_fnma213(p, w, v));
        }
    } else if (idx == 3) {
        out[0] = as_uint(q);
        out[1] = as_uint(rtp_fnma213(p, f, v));
        out[2] = vb;
    } else if (idx == 4) {
        float w = rtp_sub(0x1p+0f, f);
        out[0] = as_uint(rtp_fnma213(w, p, v));
        out[1] = as_uint(q);
        out[2] = vb;
    } else {
        out[0] = vb;
        out[1] = as_uint(q);
        out[2] = as_uint(rtp_fnma213(p, f, v));
    }
}

/* One pixel of hsv_to_rgb::pixel (plane form) */
__kernel void rtp_hsv_to_rgb(__global const float* hp, __global const float* sat_plane, __global const float* vp,
                             __global float* rp, __global float* gp, __global float* blue_plane, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float h = hp[i];
    float s = sat_plane[i];
    float v = vp[i];
    uint vb = as_uint(v);
    uint o[3];
    if (s == 0x0p+0f) {
        o[0] = vb;
        o[1] = vb;
        o[2] = vb;
    } else if (0x0p+0f > s) {
        float neg_s = as_float(as_uint(s) ^ 0x80000000u);
        float hh = rtp_add(0x1p-1f, h);
        float p = rtp_mul(v, neg_s);
        rtp_sextant(hh, p, v, vb, o);
    } else {
        float p = rtp_mul(v, s);
        rtp_sextant(h, p, v, vb, o);
    }
    rp[i] = as_float(o[0]);
    gp[i] = as_float(o[1]);
    blue_plane[i] = as_float(o[2]);
}
"#;
