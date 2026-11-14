//! Device kernel sources of the sampling and warp ops: warp_rel and warp (non footprint path), rotate, perspective and
//! perspective_alpha, plus the two helpers they share (an exact fmod for an integer divisor and the sincosf port).
//!
//! Every kernel reproduces the operation order and rounding of its CPU function. `fma()` appears only where the CPU calls
//! `mul_add` (the sources flagged `mirrors_mul_add`). Out of contract data (a tap outside the ripmap plane, a sincos
//! argument of 8192 or more) sets the error word of the launch; the host then takes the node to the CPU, so the CPU error
//! or the CPU bytes are produced there.
//!
//! * `WARP_HELPERS`: `warp_fmod` (exact fmod of a float widened to double by an integer valued divisor, integer arithmetic),
//!   `warp_sincosf` (port of `libm_sincos::sincosf_traced` for the table walk, tiny and non finite routes).
//! * `WARP_WARP`: kernel `warp_warp_f32`, ports `warp_rel::render` + `warp_rel_sampler::run` (`rel` = 1) and `warp_driver::run` (`rel` = 0)
//!   for the flag 0 path, host mode 0, n <= 4 planes.
//! * `WARP_ROTATE`: kernel `warp_rotate_f32`, ports `rotate2d::render` (seven operand paths).
//! * `WARP_PERSP`: kernels `warp_persp_f32` and `warp_persp_alpha_f32`, port `perspective::run` and `perspective_alpha`.

pub const WARP_HELPERS: &str = r#"
/* Exact fmod(x, y) for x a float widened to double and y an integer valued double in [1, 2^31]. Integer arithmetic only:
   x = mant * 2^e; for e >= 0 the remainder is (mant mod y) * (2^e mod y) mod y, for e < 0 it is (q mod y) + frac * 2^e
   with q the integer part of x. The result has the sign of x (zero keeps it). Inf and NaN give a NaN. */
double warp_fmod(double x, double y) {
    ulong xb = as_ulong(x);
    ulong sgn = xb & 0x8000000000000000ul;
    uint exp_field = (uint)((xb >> 52) & 0x7fful);
    if (exp_field == 0x7ffu) {
        return as_double(0x7ff8000000000000ul);
    }
    if (exp_field == 0u) {
        return as_double(sgn);
    }
    ulong ay = (ulong)y;
    ulong mant = (xb & 0xffffffffffffful) | 0x10000000000000ul;
    int e = (int)exp_field - 1075;
    double r;
    if (e >= 0) {
        ulong t = 1ul % ay;
        for (int i = 0; i < 128; i++) {
            if (i < e) {
                t = (t * 2ul) % ay;
            }
        }
        r = (double)(((mant % ay) * t) % ay);
    } else {
        int s = -e;
        ulong q = 0ul;
        ulong fr = mant;
        if (s < 64) {
            q = mant >> s;
            fr = mant & ((1ul << s) - 1ul);
        }
        double fs = as_double((ulong)(1023 - s) << 52);
        r = (double)(q % ay) + (double)fr * fs;
    }
    return as_double(as_ulong(r) | sgn);
}

/* cvtss2si: round to nearest even; INT_MIN when out of range or NaN. */
int warp_cvtss2si(float x) {
    if (isnan(x) || x >= 0x1p+31f || x < -0x1p+31f) {
        return INT_MIN;
    }
    return (int)rint(x);
}

/* sincosf as libm_sincos::sincosf_traced for the routes below 8192 in magnitude: table walk, zero and denormal, non finite.
   A finite argument of 8192 or more sets *big (the double precision routes are not ported); the outputs are then unused.
   st holds the 64 (sin, cos) entries of the sincosf table as double bit patterns. */
void warp_sincosf(float x, __global const ulong* st, float* so, float* co, int* big) {
    uint bits = as_uint(x);
    uint abs_x = (((bits >> 16) & 0x7fffu) - 0x80u) & 0xffffu;
    if (abs_x <= 0x457fu) {
        float t = as_float(0x4122f983u) * x;
        int copy_mode = warp_cvtss2si(t);
        float rt = (t + as_float(0x4b400000u)) - as_float(0x4b400000u);
        double xd = (double)x;
        ulong m = (ulong)bits & 0x80000000ul;
        ulong sg = (m << 32) ^ (m << 29);
        double a = as_double(0x77b921fb54444000ul ^ sg);
        double b = as_double(0x7532e7b9676733aful ^ sg);
        double rd = as_double(((ulong)as_uint(rt)) << 29);
        uint flag_0 = ((((uint)copy_mode + 0x1c7600u) << 4) & 0x3f0u) >> 4;
        uint flag_1 = ((((uint)copy_mode + 0x1c7610u) << 4) & 0x3f0u) >> 4;
        double r = xd - a * rd;
        double w = b * rd + r;
        double r2 = r * r;
        double p = (as_double(0xbfe0000000000000ul) * r2 + 0x1p+0) + as_double(0x3fa5555555555555ul) * (r2 * r2);
        double q = (as_double(0xbfc5555555555555ul) * r2 + 0x1p+0) + as_double(0x3f81111111111111ul) * (r2 * r2);
        double s0 = as_double(st[2u * flag_0]);
        double coef_s = as_double(st[2u * flag_0 + 1u]);
        double as_double_1 = as_double(st[2u * flag_1]);
        double c1 = as_double(st[2u * flag_1 + 1u]);
        double dst_ch0 = s0 * p + (w * coef_s) * q;
        double dst_ch1 = as_double_1 * p + (w * c1) * q;
        *so = (float)dst_ch0;
        *co = (float)dst_ch1;
        return;
    }
    if (abs_x >= 0x8000u) {
        double xd = (double)x;
        *so = (float)(xd * as_double(0x3fefffffff800000ul));
        *co = (float)(0x1p+0 - xd * xd);
        return;
    }
    if ((bits & 0x7f800000u) == 0x7f800000u) {
        float v;
        if (isnan(x)) {
            v = as_float(bits | 0x00400000u);
        } else {
            v = as_float(0xffc00000u);
        }
        *so = v;
        *co = v;
        return;
    }
    *big = 1;
    *so = 0x0p+0f;
    *co = 0x0p+0f;
}
"#;

pub const WARP_WARP: &str = r#"
/* MIRRORS_MUL_ADD: warp_rel_sampler::x_raw, warp_rel_sampler::y_raw, warp_rel_sampler::blend1, warp_driver::x_raw, warp_driver::y_raw, warp_driver::blend1 */

/* warp_rel_sampler::fm / warp_driver::fm: fmod of the coordinate by the dimension, plus the dimension for a negative coordinate. */
float warp_fm(float v, int dim) {
    float wf = (float)dim;
    int neg = 0x0p+0f > v;
    double r = warp_fmod((double)v, (double)wf);
    if (neg) {
        r = (double)fabs(wf) + r;
    }
    return (float)r;
}

/* The edge wrap of one coordinate: mode 1 wrap, mode 2 mirror, else unchanged. dim >= 1. */
float warp_wrap(float v, int dim, int mode) {
    if (mode == 1) {
        return warp_fm(v, dim);
    }
    if (mode == 2) {
        int q = cvtt_i32_f(v) / dim;
        int par = q & 1;
        int negb = !(v >= 0x0p+0f);
        int refl;
        if (negb) {
            refl = (par == 0);
        } else {
            refl = (par != 0);
        }
        float wf = (float)dim;
        float r = warp_fm(v, dim);
        if (refl) {
            return wf - r;
        }
        return r;
    }
    return v;
}

__kernel void warp_warp_f32(__global const float* plane, uint plen, __global const float* coord_x_in, __global const float* coord_y_in,
                       int const_coords, float coord_a_first, float coord_a_second,
                       __global float* dst_ch0, __global float* dst_ch1, __global float* dst_ch2, __global float* dst_ch3,
                       int n, int w, int h, int wt0, int ht0, int mx, int my,
                       float aspect, float left, float right, float geo_e, float geo_h, int roi_x, int roi_y,
                       int rel, __global uint* err) {
    uint gid = (uint)get_global_id(0);
    uint total = (uint)w * (uint)h;
    if (gid >= total) {
        return;
    }
    int y = (int)(gid / (uint)w);
    int xi = (int)(gid % (uint)w);
    float coord_x = coord_a_first;
    float coord_y = coord_a_second;
    if (!const_coords) {
        coord_x = coord_x_in[gid];
        coord_y = coord_y_in[gid];
    }
    float diff_13 = right - left;
    float scx = 0x1p-1f * diff_13;
    float scy = (0x1p-1f * aspect) * diff_13;
    float xr;
    float yr;
    if (rel) {
        int route_int_7 = (int)(((uint)(xi + 1) + (uint)roi_x) - 1u);
        float base = (float)route_int_7 - (float)roi_x;
        xr = fma(coord_x, scx, base);
        float cy = (float)roi_y;
        float dy = (float)((int)((uint)y + (uint)roi_y));
        float t = fma(-scy, coord_y, -cy);
        yr = t + dy;
    } else {
        float t = coord_x + 0x1p+0f;
        float base = left - (float)roi_x;
        xr = fma(t, scx, base);
        float shifted_y = coord_y - 0x1p+0f;
        float m = fma(scy, shifted_y, -geo_h);
        float top = geo_e - (float)roi_y;
        yr = top - m;
    }
    float x = warp_wrap(xr, w, mx);
    float warp_wrap_1 = warp_wrap(yr, h, my);
    float xc = x;
    if (mx == 0) {
        if (-0x1p+0f > x) {
            xc = -0x1p+0f;
        } else {
            float wb = (float)wt0;
            xc = (wb < x) ? wb : x;
        }
    }
    float yc = warp_wrap_1;
    if (my == 0) {
        if (-0x1p+0f > warp_wrap_1) {
            yc = -0x1p+0f;
        } else {
            float hb = (float)ht0;
            yc = (hb < warp_wrap_1) ? hb : warp_wrap_1;
        }
    }
    float sx = 0x1p+0f + xc;
    float sy = 0x1p+0f + yc;
    int ix = cvtt_i32_f(sx);
    int iy = cvtt_i32_f(sy);
    float fx = sx - (float)ix;
    float fy = sy - (float)iy;
    int wp3 = (int)((uint)wt0 + 3u);
    int stride = (int)((uint)wp3 * (uint)n);
    long buf_start = (long)n + (long)((int)((uint)n * (uint)wp3));
    int col_offset = (int)((uint)(ix - 1) * (uint)n);
    int prod_14 = (int)((uint)(iy - 1) * (uint)stride);
    long base0 = buf_start + (long)col_offset + (long)prod_14;
    float t = fx * fy;
    float corner_weight = ((0x1p+0f - fx) - fy) + t;
    float wn = fx - t;
    float ws = fy - t;
    float wd = t;
    long lim = (long)plen;
    for (int j = 0; j < 4; j++) {
        if (j < n) {
            long sum_0 = base0 + (long)j;
            long next_offset = base0 + (long)n + (long)j;
            long stride_offset = base0 + (long)stride + (long)j;
            long sum_3 = base0 + (long)n + (long)stride + (long)j;
            if (sum_0 < 0 || sum_0 >= lim || next_offset < 0 || next_offset >= lim || stride_offset < 0 || stride_offset >= lim || sum_3 < 0 || sum_3 >= lim) {
                atomic_or(err, 1u);
                return;
            }
            float tap_sample = plane[sum_0];
            float rn = plane[next_offset];
            float rs = plane[stride_offset];
            float rd = plane[sum_3];
            float s = wd * rd;
            float u = ws * rs;
            s = fma(wn, rn, s);
            u = fma(corner_weight, tap_sample, u);
            float v = s + u;
            if (j == 0) {
                dst_ch0[gid] = v;
            } else if (j == 1) {
                dst_ch1[gid] = v;
            } else if (j == 2) {
                dst_ch2[gid] = v;
            } else {
                dst_ch3[gid] = v;
            }
        }
    }
}
"#;

pub const WARP_ROTATE: &str = r#"
/* MIRRORS_MUL_ADD: rotate2d::render_cfg (FMA sites 0 to 31) */

/* path 1..4: the angle is a plane (v constant or planes, centre constant or planes: P1 vC cC, P2 vC cP, P3 vP cC, P4 vP cP).
   path 5..7: the angle is a constant, (s, c) come from the host (P5 vC cP, P6 vP cP, P7 vP cC with the host folded next_offset, stride_offset).
   cv = (a, b) constant v lanes, cc = (cc, d) constant centre lanes; for path 1 the host passes delta_x, dy in cv. */
__kernel void warp_rotate_f32(__global const float* ang, __global const float* vx, __global const float* vy,
                         __global const float* centre_x, __global const float* cy,
                         float va, float vb, float ca, float cb, float sconst, float cconst, float next_offset, float stride_offset,
                         __global const ulong* st, __global float* ox, __global float* oy, uint n, int path,
                         __global uint* err) {
    uint i = (uint)get_global_id(0);
    if (i >= n) {
        return;
    }
    float s = sconst;
    float c = cconst;
    if (path <= 4) {
        int big = 0;
        warp_sincosf(as_float(0x40c90fdbu) * ang[i], st, &s, &c, &big);
        if (big) {
            atomic_or(err, 1u);
            return;
        }
    }
    float rx;
    float ry;
    if (path == 1) {
        float delta_x = va;
        float dy = vb;
        float rx_part = fma(-dy, s, ca);
        float shifted_y = fma(s, delta_x, cb);
        rx = fma(delta_x, c, rx_part);
        ry = fma(c, dy, shifted_y);
    } else if (path == 2) {
        float cxi = centre_x[i];
        float cyi = cy[i];
        float delta_x = va - cxi;
        float dy = vb - cyi;
        float rx_part = fma(delta_x, c, cxi);
        rx = fma(-dy, s, rx_part);
        float shifted_y = fma(dy, c, cyi);
        ry = fma(s, delta_x, shifted_y);
    } else if (path == 3) {
        float vxi = vx[i];
        float vyi = vy[i];
        float delta_x = vxi - ca;
        float dy = vyi - cb;
        float rx_part = fma(delta_x, c, ca);
        float shifted_y = fma(delta_x, s, cb);
        rx = fma(-dy, s, rx_part);
        ry = fma(c, dy, shifted_y);
    } else if (path == 4) {
        float cxi = centre_x[i];
        float cyi = cy[i];
        float delta_x = vx[i] - cxi;
        float dy = vy[i] - cyi;
        float rx_part = fma(delta_x, c, cxi);
        rx = fma(-dy, s, rx_part);
        float shifted_y = fma(dy, c, cyi);
        ry = fma(s, delta_x, shifted_y);
    } else if (path == 5) {
        float cxi = centre_x[i];
        float cyi = cy[i];
        float delta_x = va - cxi;
        float dy = vb - cyi;
        float rx_part = fma(delta_x, c, cxi);
        rx = fma(-dy, s, rx_part);
        float shifted_y = fma(c, dy, cyi);
        ry = fma(delta_x, s, shifted_y);
    } else if (path == 6) {
        float cxi = centre_x[i];
        float cyi = cy[i];
        float delta_x = vx[i] - cxi;
        float dy = vy[i] - cyi;
        float rx_part = fma(delta_x, c, cxi);
        rx = fma(-dy, s, rx_part);
        float shifted_y = fma(c, dy, cyi);
        ry = fma(delta_x, s, shifted_y);
    } else {
        float vxi = vx[i];
        float vyi = vy[i];
        float rx_part = fma(c, vxi, next_offset);
        float shifted_y = fma(vxi, s, stride_offset);
        rx = fma(-vyi, s, rx_part);
        ry = fma(vyi, c, shifted_y);
    }
    ox[i] = rx;
    oy[i] = ry;
}
"#;

pub const WARP_PERSP: &str = r#"
/* MIRRORS_MUL_ADD: perspective::d_of, perspective::dneg_of, perspective::elem, perspective_alpha::ramp */

/* Section plan of one row of w elements for a destination phase (bytes mod 32): elements in [*pk, *pe) use the vector
   formula, all others the scalar one (perspective::plan, part_of). */
void warp_plan(int w, uint phase, int* pk, int* pe) {
    int nn = w;
    *pk = nn;
    *pe = nn;
    if (nn < 16) {
        return;
    }
    int peel;
    int tap_b;
    if (nn < 29) {
        peel = 0;
        tap_b = nn & ~15;
    } else {
        uint ph = phase & 31u;
        if (ph == 0u) {
            peel = 0;
        } else if ((ph & 3u) != 0u) {
            return;
        } else {
            peel = (int)((32u - ph) / 4u);
        }
        if (nn < peel + 16) {
            return;
        }
        tap_b = nn - ((nn - peel) & 15);
    }
    int rem = nn - tap_b;
    int end_tap = tap_b;
    if (rem >= 8) {
        end_tap = tap_b + (rem & ~7);
    }
    *pk = peel;
    *pe = end_tap;
}

float warp_nr_inv(float x) {
    float r = 0x1p+0f / x;
    return (r + r) - ((r * x) * r);
}

/* kc: a_sin p b_sin b_cos sin_k cos_k b (host constants, perspective::constants). out1 = (b * u) * q, out2 = fma(fma(v, b_cos, -sin_k), q, a_sin). */
__kernel void warp_persp_f32(__global const float* u, __global const float* v, __global float* out1, __global float* out2,
                        float a_sin, float p, float b_sin, float b_cos, float sin_k, float cos_k, float b,
                        int w, int h, uint base, int rowbuf) {
    uint gid = (uint)get_global_id(0);
    uint total = (uint)w * (uint)h;
    if (gid >= total) {
        return;
    }
    int y = (int)(gid / (uint)w);
    int xi = (int)(gid % (uint)w);
    uint phase;
    if (rowbuf) {
        phase = base & 31u;
    } else {
        phase = (base + (uint)y * (uint)w * 4u) & 31u;
    }
    int pk;
    int pe;
    warp_plan(w, phase, &pk, &pe);
    int vec = (xi >= pk && xi < pe);
    float vv = v[gid];
    float uu = u[gid];
    float d = fma(-b_sin, vv, -cos_k);
    float q;
    if (vec) {
        int zero = (d == 0x0p+0f);
        int dgt = (d > 0x0p+0f);
        int pgt = (p > 0x0p+0f);
        if (zero) {
            q = p;
        } else if (pgt == dgt) {
            q = (0x1.8p+0f - p) * warp_nr_inv(d);
        } else {
            float dn = fma(b_sin, vv, cos_k);
            q = p * warp_nr_inv(dn);
        }
    } else {
        if (d == 0x0p+0f) {
            q = p;
        } else {
            int dgt = (d > 0x0p+0f);
            if ((p > 0x0p+0f) == dgt) {
                q = (0x1.8p+0f - p) / d;
            } else {
                q = p / (-d);
            }
        }
    }
    float t = fma(vv, b_cos, -sin_k);
    out1[gid] = (b * uu) * q;
    out2[gid] = fma(t, q, a_sin);
}

/* perspective_alpha::ramp with the clamp maximum 1.0 in every part. cf = (float)col_offset, p, cos_t from the host. */
__kernel void warp_persp_alpha_f32(__global const float* v, __global float* dst, uint n, float cf, float p, float cos_t) {
    uint i = (uint)get_global_id(0);
    if (i >= n) {
        return;
    }
    float q = cf * fma(-p, v[i], -cos_t);
    float o;
    if (q < 0x0p+0f) {
        o = 0x0p+0f;
    } else if (0x1p+0f < q) {
        o = 0x1p+0f;
    } else {
        o = q;
    }
    dst[i] = o;
}
"#;
