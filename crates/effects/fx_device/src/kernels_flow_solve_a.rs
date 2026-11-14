//! Device kernel source of the optical flow solve tile kernels, first group: `fma_mix`, `flow_update`, `diff_sum`, `zero_border`,
//! `scale_plane` and `relax_callback` of the `optical_flow_solve` crate (the plane scale and border zeroing are the passes of its
//! stretch). OpenCL C 1.2 as a Rust string constant; registered in [`crate::kernels::SOURCES`].
//!
//! One work item per element, except `diff_sum`, whose accumulation order is the crate's (one work item walks every row in order,
//! with the eight lane partial sums and the fixed lane reduction of the vector form). The vector and scalar forms of a row are
//! chosen per element from the plane byte phase with the same split rule as the crate (`flow_a_split`). Every add, subtract, multiply,
//! divide and fused step has the x86 two-NaN rule of `fx_common::nanop` (the `flow_a_` helpers), so a NaN is computed by explicit tests.

/// Solve tile kernels A.
pub const FLOW_SOLVE_A: &str = r#"
/* MIRRORS_MUL_ADD: fx_common::nanop fma213, fma231, fms213, fnma213 and fnms132 (the crate's mul_add sites) */

float flow_a_pick2(float a, float b) {
    if (a != a) {
        return as_float(as_uint(a) | 0x400000u);
    }
    if (b != b) {
        return as_float(as_uint(b) | 0x400000u);
    }
    return as_float(0xffc00000u);
}

float flow_a_pick3(float a, float b, float c) {
    if (a != a) {
        return as_float(as_uint(a) | 0x400000u);
    }
    if (b != b) {
        return as_float(as_uint(b) | 0x400000u);
    }
    if (c != c) {
        return as_float(as_uint(c) | 0x400000u);
    }
    return as_float(0xffc00000u);
}

float flow_a_add(float a, float b) {
    float r = a + b;
    return (r != r) ? flow_a_pick2(a, b) : r;
}

float flow_a_sub(float a, float b) {
    float r = a - b;
    return (r != r) ? flow_a_pick2(a, b) : r;
}

float flow_a_mul(float a, float b) {
    float r = a * b;
    return (r != r) ? flow_a_pick2(a, b) : r;
}

float flow_a_div(float a, float b) {
    float r = a / b;
    return (r != r) ? flow_a_pick2(a, b) : r;
}

float flow_a_flip(float x) {
    return as_float(as_uint(x) ^ 0x80000000u);
}

/* d * second + third */
float flow_a_fma213(float d, float second, float third) {
    float r = fma(d, second, third);
    return (r != r) ? flow_a_pick3(d, second, third) : r;
}

/* second * third + d */
float flow_a_fma231(float d, float second, float third) {
    float r = fma(second, third, d);
    return (r != r) ? flow_a_pick3(d, second, third) : r;
}

/* d * second - third */
float flow_a_fms213(float d, float second, float third) {
    float r = fma(d, second, -third);
    return (r != r) ? flow_a_pick3(d, second, third) : r;
}

/* -(d * second) + third */
float flow_a_fnma213(float d, float second, float third) {
    float r = fma(-d, second, third);
    return (r != r) ? flow_a_pick3(d, second, third) : r;
}

/* -(d * third) - second */
float flow_a_fnms132(float d, float second, float third) {
    float r = fma(-d, third, -second);
    return (r != r) ? flow_a_pick3(d, second, third) : r;
}

/* Split of a row of w elements: (head, vec_end, all_scalar). lanes is the vector width, min_aligned the width from which the
   head is aligned, lb the alignment line in bytes (32, or 64 for the relax planes). */
long3 flow_a_split(long w, long lanes, long min_aligned, uint phase, long lb) {
    if (w < lanes) {
        return (long3)(0, 0, 1);
    }
    if (w < min_aligned) {
        return (long3)(0, w & ~(lanes - 1), 0);
    }
    long head = 0;
    long ph = (long)phase;
    if (ph != 0) {
        if ((ph & 3) != 0) {
            return (long3)(0, 0, 1);
        }
        head = (lb - ph) >> 2;
    }
    if (w < head + lanes) {
        return (long3)(0, 0, 1);
    }
    return (long3)(head, w - ((w - head) & (lanes - 1)), 0);
}

__kernel void flow_a_fma_mix(__global const float* pa, __global const float* pb, __global const float* pc, __global const float* pd,
                          __global const float* pe, __global const float* pf, __global float* out,
                          int w, int stride, int row0, uint phase) {
    size_t g = get_global_id(0);
    long r = (long)row0 + (long)(g / (size_t)w);
    long i = (long)(g % (size_t)w);
    long off = r * (long)stride;
    long k = off + i;
    uint ph = (phase + (uint)off * 4u) & 31u;
    long3 row_split = flow_a_split((long)w, 16, 29, ph, 32);
    int vec = (row_split.z == 0 && i >= row_split.x && i < row_split.y) ? 1 : 0;
    float e = pe[k];
    float c = pc[k];
    float b = pb[k];
    float f = pf[k];
    float d = pd[k];
    float a = pa[k];
    float x;
    float y;
    if (vec != 0) {
        float t = flow_a_fms213(f, d, b);
        x = flow_a_fnms132(e, t, c);
        y = a;
    } else {
        float p = flow_a_fms213(e, c, b);
        x = flow_a_flip(p);
        y = flow_a_fma213(f, d, a);
    }
    out[k] = flow_a_sub(x, y);
}

__kernel void flow_a_flow_update(__global const float* pu, __global const float* pv, __global const float* pa, __global const float* pb,
                              __global const float* pc, __global float* out0, __global float* out1,
                              int w, int stride, int row0, uint s_bits, uint eps_bits) {
    size_t g = get_global_id(0);
    long r = (long)row0 + (long)(g / (size_t)w);
    long i = (long)(g % (size_t)w);
    long k = r * (long)stride + i;
    float s = as_float(s_bits);
    float eps = as_float(eps_bits);
    float b = pb[k];
    float det0 = flow_a_mul(b, b);
    float a = pa[k];
    float u = pu[k];
    float v = pv[k];
    float det = flow_a_fma231(det0, a, a);
    float partial_num = flow_a_fma231(pc[k], u, a);
    float thr = flow_a_mul(s, det);
    float num = flow_a_fma231(partial_num, v, b);
    float neg_thr = flow_a_flip(thr);
    float du;
    float dv;
    if (neg_thr > num) {
        du = flow_a_mul(a, s);
        dv = flow_a_mul(b, s);
    } else if (num > thr) {
        du = flow_a_flip(flow_a_mul(a, s));
        dv = flow_a_flip(flow_a_mul(b, s));
    } else if (eps > det) {
        du = as_float(0u);
        dv = as_float(0u);
    } else {
        float p = flow_a_mul(a, num);
        float q = flow_a_mul(b, num);
        du = flow_a_div(flow_a_flip(p), det);
        dv = flow_a_div(flow_a_flip(q), det);
    }
    out0[k] = flow_a_add(u, du);
    out1[k] = flow_a_add(v, dv);
}

float flow_a_diff_one(__global const float* pp, __global const float* pq, __global float* px, __global float* py, long k) {
    float diff_p = flow_a_sub(pp[k], px[k]);
    float diff_q = flow_a_sub(pq[k], py[k]);
    float t = flow_a_mul(diff_p, diff_p);
    float s = flow_a_fma213(diff_q, diff_q, t);
    py[k] = pq[k];
    px[k] = pp[k];
    return s;
}

/* One work item: the crate's serial accumulation over rows, eight lane partial sums in the vector form. */
__kernel void flow_a_diff_sum(__global const float* pp, __global const float* pq, __global float* px, __global float* py,
                           __global float* sum, int w, int stride, int row0, int rows, uint phase) {
    float acc = as_float(0u);
    for (int row = 0; row < 2000000000; row++) {
        if (row >= rows) {
            break;
        }
        long off = ((long)row0 + (long)row) * (long)stride;
        uint ph = (phase + (uint)off * 4u) & 31u;
        long3 row_split = flow_a_split((long)w, 8, 907, ph, 32);
        if (row_split.z != 0) {
            for (int i = 0; i < 2000000000; i++) {
                if (i >= w) {
                    break;
                }
                float s = flow_a_diff_one(pp, pq, px, py, off + i);
                acc = flow_a_add(s, acc);
            }
            continue;
        }
        for (int i = 0; i < 2000000000; i++) {
            if ((long)i >= row_split.x) {
                break;
            }
            float s = flow_a_diff_one(pp, pq, px, py, off + i);
            acc = flow_a_add(s, acc);
        }
        float lanes[8];
        for (int j = 0; j < 8; j++) {
            lanes[j] = as_float(0u);
        }
        lanes[0] = acc;
        long p = row_split.x;
        for (int blk = 0; blk < 2000000000; blk++) {
            for (int j = 0; j < 8; j++) {
                float s = flow_a_diff_one(pp, pq, px, py, off + p + j);
                lanes[j] = flow_a_add(s, lanes[j]);
            }
            p += 8;
            if (p >= row_split.y) {
                break;
            }
        }
        float ls0 = flow_a_add(lanes[0], lanes[4]);
        float ls1 = flow_a_add(lanes[1], lanes[5]);
        float ls2 = flow_a_add(lanes[2], lanes[6]);
        float ls3 = flow_a_add(lanes[3], lanes[7]);
        float lp0 = flow_a_add(ls0, ls2);
        float lp1 = flow_a_add(ls1, ls3);
        acc = flow_a_add(lp0, lp1);
        for (int i = 0; i < 2000000000; i++) {
            if ((long)i + row_split.y >= (long)w) {
                break;
            }
            float s = flow_a_diff_one(pp, pq, px, py, off + row_split.y + i);
            acc = flow_a_add(s, acc);
        }
    }
    sum[0] = acc;
}

__kernel void flow_a_zero_border(__global float* pl, int w, int h, int stride, int start, uint count) {
    size_t g = get_global_id(0);
    long k = (long)(g / (size_t)w);
    int j = (int)(g % (size_t)w);
    long y = (long)start + k;
    if (y == 0 || y == (long)h - 1 || j == 0 || j == w - 1) {
        pl[y * (long)stride + j] = as_float(0u);
    }
}

__kernel void flow_a_scale_plane(__global float* pl, int w, int stride, int start, uint scale_bits) {
    size_t g = get_global_id(0);
    long y = (long)start + (long)(g / (size_t)w);
    long ix = y * (long)stride + (long)(g % (size_t)w);
    pl[ix] = flow_a_mul(pl[ix], as_float(scale_bits));
}

/* Both nests of relax_callback. Work item g: nest = g / (count * w), then row and column. */
__kernel void flow_a_relax(__global const float* nest_zero_t, __global const float* nest_zero_u, __global const float* nest_zero_acc, __global float* nest_zero_out,
                        __global const float* nest_one_t, __global const float* nest_one_u, __global const float* nest_one_acc, __global float* nest_one_out,
                        __global const float* pv, uint scale_bits, int has_v,
                        int w, int h, int stride, int start, int count, uint phase) {
    size_t g = get_global_id(0);
    size_t per = (size_t)count * (size_t)w;
    int nest = (int)(g / per);
    size_t q = g % per;
    long y = (long)start + (long)(q / (size_t)w);
    long j = (long)(q % (size_t)w);
    __global const float* tt = (nest == 0) ? nest_zero_t : nest_one_t;
    __global const float* uu = (nest == 0) ? nest_zero_u : nest_one_u;
    __global const float* aa = (nest == 0) ? nest_zero_acc : nest_one_acc;
    __global float* oo = (nest == 0) ? nest_zero_out : nest_one_out;
    float scale = as_float(scale_bits);
    int kind = (y == 0) ? 0 : ((y == (long)h - 1) ? 1 : 2);
    long st = (long)stride;
    long rb = y * st;
    if (j == 0) {
        float t = tt[rb];
        float d;
        if (kind == 0) {
            d = uu[rb];
        } else if (kind == 1) {
            d = flow_a_flip(uu[rb - st]);
        } else {
            d = flow_a_sub(uu[rb], uu[rb - st]);
        }
        float s = flow_a_add(t, d);
        if (has_v != 0) {
            oo[rb] = flow_a_fma213(flow_a_mul(scale, pv[rb]), s, aa[rb]);
        } else {
            oo[rb] = flow_a_fma213(s, scale, aa[rb]);
        }
        return;
    }
    long n = (long)w - 1;
    long x = j - 1;
    uint ph = (uint)((((long)phase + rb * 4) & 63));
    long lb = 64;
    long3 row_split;
    if (n < 16) {
        row_split = (long3)(0, 0, 1);
    } else if (n < 1200) {
        row_split = (long3)(0, n & ~15, 0);
    } else {
        row_split = flow_a_split(n, 16, 1200, ph, lb);
    }
    int vec = (row_split.z == 0 && x >= row_split.x && x < row_split.y) ? 1 : 0;
    float delta_x = (x == n - 1) ? flow_a_flip(tt[rb + x]) : flow_a_sub(tt[rb + x + 1], tt[rb + x]);
    float uc = uu[rb + j];
    float ua = (kind == 0) ? as_float(0u) : uu[rb - st + j];
    float a = aa[rb + j];
    float r;
    if (has_v != 0) {
        float pvv = flow_a_mul(scale, pv[rb + j]);
        if (kind == 0) {
            r = flow_a_fma213(pvv, flow_a_add(delta_x, uc), a);
        } else if (kind == 1) {
            if (vec != 0) {
                r = flow_a_fma213(pvv, flow_a_add(delta_x, flow_a_flip(ua)), a);
            } else {
                r = flow_a_fnma213(pvv, flow_a_sub(ua, delta_x), a);
            }
        } else {
            r = flow_a_fma213(pvv, flow_a_add(delta_x, flow_a_sub(uc, ua)), a);
        }
    } else {
        if (kind == 0) {
            r = flow_a_fma213(flow_a_add(delta_x, uc), scale, a);
        } else if (kind == 1) {
            if (vec != 0) {
                r = flow_a_fma213(flow_a_add(delta_x, flow_a_flip(ua)), scale, a);
            } else {
                r = flow_a_fnma213(flow_a_sub(ua, delta_x), scale, a);
            }
        } else if (vec != 0) {
            r = flow_a_fma213(flow_a_add(delta_x, flow_a_sub(uc, ua)), scale, a);
        } else {
            float tm = flow_a_fma213(delta_x, scale, a);
            r = flow_a_fma213(flow_a_sub(uc, ua), scale, tm);
        }
    }
    oo[rb + j] = r;
}
"#;
