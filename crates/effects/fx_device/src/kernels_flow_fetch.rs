//! Device kernel sources of the optical flow fetch, resample and plane row kernels (ports of the CPU functions named at each kernel).
//!
//! Every kernel computes each output word with the CPU function's operations in the CPU's order. Two-NaN operations use the
//! `nanx_` helpers of `kernels_pointwise` (first NaN operand, quieted, else the fresh NaN 0xffc00000); the fused steps use the
//! `flow_b_` helpers below, which are the named fused forms of `fx_common::nanop` (NaN priority destination, second, third source).
//! Host values (constants of the crates, window tables of the resample pair, plane phases) arrive as kernel arguments computed by
//! the same Rust expressions the CPU functions use. The reductions (`flow_b_sum_rows`, `flow_b_scan_range`) run as one work item in the
//! CPU's accumulation order.

pub const FLOW_FETCH: &str = r#"
/* MIRRORS_MUL_ADD: optical_flow_solve::fetch (fma213, fms213, fms231, fnma231), optical_flow_resample (fma213, fma231, fms213, fms231),
   optical_flow::callbacks and flowbuild (fma213, fms213, fnma213), optical_flow_entry::bilinear and final_warp_rows (fma213, fms213) */

float flow_b_neg(float x) { return as_float(as_uint(x) ^ 0x80000000u); }
float flow_b_abs(float x) { return as_float(as_uint(x) & 0x7fffffffu); }
float flow_b_f213(float d, float second, float third) { float r = fma(d, second, third); return isnan(r) ? nanx_pick3(d, second, third) : r; }
float flow_b_m213(float d, float second, float third) { float r = fma(d, second, -third); return isnan(r) ? nanx_pick3(d, second, third) : r; }
float flow_b_f231(float d, float second, float third) { float r = fma(second, third, d); return isnan(r) ? nanx_pick3(d, second, third) : r; }
float flow_b_m231(float d, float second, float third) { float r = fma(second, third, -d); return isnan(r) ? nanx_pick3(d, second, third) : r; }
float flow_b_n213(float d, float second, float third) { float r = fma(-d, second, third); return isnan(r) ? nanx_pick3(d, second, third) : r; }
float flow_b_n231(float d, float second, float third) { float r = fma(-second, third, d); return isnan(r) ? nanx_pick3(d, second, third) : r; }

/* vcvtss2si of the rounded value: round to nearest even, then the integer indefinite outside the range and for NaN */
int flow_b_cvt_rne(float x) {
    float r = rint(x);
    if (r >= -0x1p31f && r < 0x1p31f) {
        return (int)r;
    }
    return INT_MIN;
}

/* the crates' split(w, lanes, min_aligned, phase): head, vector end, all scalar */
void flow_b_split(long w, long lanes, long min_aligned, long ph, long* head, long* vend, int* all) {
    *head = 0;
    *vend = 0;
    *all = 0;
    if (w < lanes) {
        *all = 1;
        return;
    }
    if (w < min_aligned) {
        *vend = w & ~(lanes - 1);
        return;
    }
    long hd = 0;
    if (ph != 0) {
        if ((ph & 3) != 0) {
            *all = 1;
            return;
        }
        hd = (0x20 - ph) >> 2;
    }
    if (w < hd + lanes) {
        *all = 1;
        return;
    }
    *head = hd;
    *vend = w - ((w - hd) & (lanes - 1));
}

int flow_b_idx(int a, int b) { return (int)((uint)a * (uint)b); }

/* fetch.rs deriv_x: element i of row y; the vector form for head <= i < vend, the scalar form elsewhere */
__kernel void flow_b_deriv_x(__global const float* inp, __global float* out, int width, int stride, int row0, int rows, uint out_phase,
                          uint ks_b, uint cs_b, uint kv_b, uint cv_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int k = (int)(g / (uint)width);
    int i = (int)(g - (uint)k * (uint)width);
    int y = (int)((uint)row0 + (uint)k);
    int roff = flow_b_idx(y, stride);
    long ph = ((long)out_phase + (long)roff * 4) & 31;
    long head; long vend; int all;
    flow_b_split((long)width, 8, 0x4b0, ph, &head, &vend, &all);
    if (all != 0) {
        head = 0;
        vend = 0;
    }
    float ks = as_float(ks_b);
    float cs = as_float(cs_b);
    float kv = as_float(kv_b);
    float cv = as_float(cv_b);
    int w = width;
    int vec = (all == 0) && ((long)i >= head) && ((long)i < vend);
    float r;
    if (vec != 0) {
        int prev_idx = i - 1; if (prev_idx < 0) { prev_idx = 0; }
        int p2 = i - 2; if (p2 < 0) { p2 = 0; }
        int next_idx = i + 1; if (next_idx > w - 1) { next_idx = w - 1; }
        int after_next_idx = i + 2; if (after_next_idx > w - 1) { after_next_idx = w - 1; }
        float t = flow_b_m231(inp[(long)roff + p2], inp[(long)roff + prev_idx], cv);
        t = flow_b_n231(t, inp[(long)roff + next_idx], cv);
        float s = nanx_add(t, inp[(long)roff + after_next_idx]);
        r = nanx_mul(kv, s);
    } else {
        int lo1 = i - 1; if (lo1 < 0) { lo1 = 0; }
        int hi1 = i + 1; if (!((w - 1) >= hi1)) { hi1 = w - 1; }
        int lo2 = i - 2; if (lo2 < 0) { lo2 = 0; }
        int hi2 = i + 2; if ((w - 1) < hi2) { hi2 = w - 1; }
        float first_diff = nanx_sub(inp[(long)roff + lo1], inp[(long)roff + hi1]);
        float second_diff = nanx_sub(inp[(long)roff + lo2], inp[(long)roff + hi2]);
        float t = flow_b_m213(first_diff, cs, second_diff);
        r = nanx_mul(t, ks);
    }
    out[(long)roff + i] = r;
}

/* fetch.rs deriv_y: element (row y, column col0 + kc); the vector form for the rows below h & ~7 when stride != 0 and h >= 8 */
__kernel void flow_b_deriv_y(__global const float* inp, __global float* out, int height, int stride, int col0, int cols,
                          uint ks_b, uint cs_b, uint kv_b, uint cv_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int kc = (int)(g / (uint)height);
    int y = (int)(g - (uint)kc * (uint)height);
    int col = (int)((uint)col0 + (uint)kc);
    int h = height;
    float ks = as_float(ks_b);
    float cs = as_float(cs_b);
    float kv = as_float(kv_b);
    float cv = as_float(cv_b);
    int vec = (stride != 0) && (h >= 8) && (y < (h & ~7));
    float r;
    if (vec != 0) {
        int prev_idx = y - 1; if (prev_idx < 0) { prev_idx = 0; }
        int p2 = y - 2; if (p2 < 0) { p2 = 0; }
        int next_idx = y + 1; if (next_idx > h - 1) { next_idx = h - 1; }
        int after_next_idx = y + 2; if (after_next_idx > h - 1) { after_next_idx = h - 1; }
        float t = flow_b_m231(inp[(long)flow_b_idx(p2, stride) + col], inp[(long)flow_b_idx(prev_idx, stride) + col], cv);
        t = flow_b_n231(t, inp[(long)flow_b_idx(next_idx, stride) + col], cv);
        float s = nanx_add(t, inp[(long)flow_b_idx(after_next_idx, stride) + col]);
        r = nanx_mul(kv, s);
    } else {
        int lo1 = y - 1; if (lo1 < 0) { lo1 = 0; }
        int hi1 = y + 1; if (!((h - 1) >= hi1)) { hi1 = h - 1; }
        int lo2 = y - 2; if (lo2 < 0) { lo2 = 0; }
        int hi2 = y + 2; if ((h - 1) < hi2) { hi2 = h - 1; }
        float first_diff = nanx_sub(inp[(long)flow_b_idx(lo1, stride) + col], inp[(long)flow_b_idx(hi1, stride) + col]);
        float second_diff = nanx_sub(inp[(long)flow_b_idx(lo2, stride) + col], inp[(long)flow_b_idx(hi2, stride) + col]);
        float t = flow_b_m213(first_diff, cs, second_diff);
        r = nanx_mul(t, ks);
    }
    out[(long)col + (long)stride * (long)y] = r;
}

int flow_b_clamp_idx(int v, int dim) {
    if (v < 0) {
        return 0;
    }
    int m = dim - 1;
    return (m < v) ? m : v;
}

/* the four taps and the three step bilinear blend shared by warp_fetch and the entry final warp (same operation order) */
float flow_b_bilinear(__global const float* src, int width, int height, int stride, int ix0, int iy0, float fx, float fy) {
    int ix1 = (int)((uint)ix0 + 1u);
    int iy1 = (int)((uint)iy0 + 1u);
    float tap_top_left = src[(long)flow_b_idx(flow_b_clamp_idx(iy0, height), stride) + flow_b_clamp_idx(ix0, width)];
    float tap_top_right = src[(long)flow_b_idx(flow_b_clamp_idx(iy0, height), stride) + flow_b_clamp_idx(ix1, width)];
    float tap_bot_left = src[(long)flow_b_idx(flow_b_clamp_idx(iy1, height), stride) + flow_b_clamp_idx(ix0, width)];
    float tap_bot_right = src[(long)flow_b_idx(flow_b_clamp_idx(iy1, height), stride) + flow_b_clamp_idx(ix1, width)];
    float top_diff = nanx_sub(tap_top_right, tap_top_left);
    float bottom_diff = nanx_sub(tap_bot_right, tap_bot_left);
    float left_diff = nanx_sub(tap_top_left, tap_bot_left);
    float cross_diff = nanx_sub(bottom_diff, top_diff);
    float top = flow_b_f213(top_diff, fx, tap_top_left);
    float cross = flow_b_m213(cross_diff, fx, left_diff);
    return flow_b_f213(fy, cross, top);
}

/* fetch.rs warp_fetch */
__kernel void flow_b_warp_fetch(__global const float* dxp, __global const float* dyp, __global const float* src, __global float* out,
                             int width, int height, int stride, int row0, int rows, uint lo_b, uint margin_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int k = (int)(g / (uint)width);
    int x = (int)(g - (uint)k * (uint)width);
    int y = (int)((uint)row0 + (uint)k);
    long idx = (long)flow_b_idx(y, stride) + x;
    float lo = as_float(lo_b);
    float margin = as_float(margin_b);
    float px = nanx_add((float)x, dxp[idx]);
    float py = nanx_add((float)y, dyp[idx]);
    int a = lo >= px;
    int b = px >= nanx_sub((float)width, margin);
    int c = lo >= py;
    float hm = nanx_sub((float)height, margin);
    int d = !((py < hm) || isnan(py) || isnan(hm));
    if (a || b || c || d) {
        out[idx] = 0x0p+0f;
        return;
    }
    int ix0 = flow_b_cvt_rne(floor(px));
    int iy0 = flow_b_cvt_rne(floor(py));
    float fx = nanx_sub(px, (float)ix0);
    float fy = nanx_sub(py, (float)iy0);
    out[idx] = flow_b_bilinear(src, width, height, stride, ix0, iy0, fx, fy);
}

/* entry lib.rs scale_copy_model: a copy when k == 1, positive zero when k == 0, else src * k with the NaN rule */
__kernel void flow_b_scale_copy(__global const uint* src, __global uint* dst, uint k_b, uint n) {
    uint i = (uint)get_global_id(0);
    if (i >= n) {
        return;
    }
    float k = as_float(k_b);
    if (k == 0x1p+0f) {
        dst[i] = src[i];
    } else if (k == 0x0p+0f) {
        dst[i] = 0u;
    } else {
        dst[i] = as_uint(nanx_mul(as_float(src[i]), k));
    }
}

/* optical_flow callbacks.rs affine_rows: every head, vector and tail element is r = (r - offset) * scale - 1 (fused), in place */
__kernel void flow_b_affine_rows(__global float* data, int width, int stride, int row0, int rows, uint scale_b, uint offset_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int k = (int)(g / (uint)width);
    int x = (int)(g - (uint)k * (uint)width);
    int row = (int)((uint)row0 + (uint)k);
    long idx = (long)flow_b_idx(row, stride) + x;
    data[idx] = flow_b_m213(nanx_sub(data[idx], as_float(offset_b)), as_float(scale_b), 0x1p+0f);
}

/* optical_flow callbacks.rs sub_scaled_rows: every element is out = b - a * s with one rounding */
__kernel void flow_b_sub_scaled_rows(__global const float* a, __global const float* b, __global float* out, int width, int stride, int row0,
                                  int rows, uint s_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int k = (int)(g / (uint)width);
    int x = (int)(g - (uint)k * (uint)width);
    int row = (int)((uint)row0 + (uint)k);
    long idx = (long)flow_b_idx(row, stride) + x;
    out[idx] = flow_b_n213(a[idx], as_float(s_b), b[idx]);
}

/* optical_flow flowbuild.rs flow_stencil_rows, element x of row (row0 + k) */
__kernel void flow_b_flow_stencil_rows(__global float* out, __global const float* b, __global const float* c, __global const float* d,
                                    int width, int rows, int pitch, uint scale_b, int row0, int count, uint align_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int k = (int)(g / (uint)width);
    int x = (int)(g - (uint)k * (uint)width);
    int row = (int)((uint)row0 + (uint)k);
    float scale = as_float(scale_b);
    int kind = (row == 0) ? 0 : ((row == (int)((uint)rows - 1u)) ? 2 : 1);
    long ro = (long)flow_b_idx(pitch, row);
    long cur = ro + x;
    long prv = ro - (long)pitch + x;
    if (x == 0) {
        float xv;
        if (kind == 0) {
            xv = c[ro];
        } else if (kind == 2) {
            xv = flow_b_neg(c[ro - (long)pitch]);
        } else {
            xv = nanx_sub(c[ro], c[ro - (long)pitch]);
        }
        out[ro] = flow_b_f213(nanx_add(b[ro], xv), scale, d[ro]);
        return;
    }
    int nn = width - 1;
    long phase = ((long)align_b + 4 * ro) & 63;
    long head = 0;
    long vend = 0;
    if (nn >= 16) {
        if (nn < 0x4b0) {
            vend = (long)(nn & ~15);
        } else {
            long m = phase & 63;
            int ok = 1;
            long hd = 0;
            if (m != 0) {
                if ((m & 3) != 0) {
                    ok = 0;
                } else {
                    hd = (64 - m) / 4;
                }
            }
            if (ok != 0 && (long)nn >= hd + 16) {
                head = hd;
                vend = (long)nn - (((long)nn - hd) & 15);
            }
        }
    }
    long i = (long)x - 1;
    int vec = (i >= head) && (i < vend);
    float fwd_diff;
    if (i == (long)width - 2) {
        fwd_diff = flow_b_neg(b[ro + i]);
    } else {
        fwd_diff = nanx_sub(b[ro + i + 1], b[ro + i]);
    }
    float r;
    if (kind == 0) {
        r = flow_b_f213(nanx_add(fwd_diff, c[cur]), scale, d[cur]);
    } else if (kind == 2) {
        if (vec != 0) {
            r = flow_b_f213(nanx_add(fwd_diff, flow_b_neg(c[prv])), scale, d[cur]);
        } else {
            r = flow_b_n213(nanx_sub(c[prv], fwd_diff), scale, d[cur]);
        }
    } else {
        if (vec != 0) {
            r = flow_b_f213(nanx_add(fwd_diff, nanx_sub(c[cur], c[prv])), scale, d[cur]);
        } else {
            float t = flow_b_f213(fwd_diff, scale, d[cur]);
            r = flow_b_f213(nanx_sub(c[cur], c[prv]), scale, t);
        }
    }
    out[cur] = r;
}

/* optical_flow callbacks.rs sum_rows: one work item, rows in order; dst[0] is the returned partial sum */
__kernel void flow_b_sum_rows(__global const float* data, __global float* dst, int width, int stride, int row0, int rows, uint align0) {
    if (get_global_id(0) != 0) {
        return;
    }
    float total = 0x0p+0f;
    if (row0 >= (int)((uint)row0 + (uint)rows)) {
        dst[0] = total;
        return;
    }
    for (int k = 0; k < 2147483647; k++) {
        if (k >= rows) {
            break;
        }
        int row = row0 + k;
        if (width <= 0) {
            continue;
        }
        long w = (long)width;
        long base = (long)flow_b_idx(stride, row);
        float y[8];
        for (int l = 0; l < 8; l++) {
            y[l] = 0x0p+0f;
        }
        long head = -1;
        if (w >= 0x40) {
            long m = ((long)align0 + 4 * base) & 31;
            if (m == 0) {
                head = 0;
            } else if ((m & 3) == 0) {
                head = (32 - m) / 4;
            }
        }
        int vec_path = (head >= 0) && (w >= head + 0x40);
        if (vec_path == 0) {
            for (long i = 0; i < 4611686018427387904; i++) {
                if (i >= w) {
                    break;
                }
                total = nanx_add(total, data[base + i]);
            }
        } else {
            for (long i = 0; i < 4611686018427387904; i++) {
                if (i >= head) {
                    break;
                }
                total = nanx_add(total, data[base + i]);
            }
            long end = w - ((w - head) & 0x3f);
            float a[64];
            for (int q = 0; q < 64; q++) {
                a[q] = 0x0p+0f;
            }
            for (long p = head; p < 4611686018427387904; p += 64) {
                if (p >= end) {
                    break;
                }
                for (int q = 0; q < 64; q++) {
                    a[q] = nanx_add(a[q], data[base + p + q]);
                }
            }
            for (int l = 0; l < 8; l++) {
                float pair01 = nanx_add(a[l], a[8 + l]);
                float pair23 = nanx_add(a[16 + l], a[24 + l]);
                float pair45 = nanx_add(a[32 + l], a[40 + l]);
                float pair67 = nanx_add(a[48 + l], a[56 + l]);
                float quad0123 = nanx_add(pair01, pair23);
                float quad4567 = nanx_add(pair45, pair67);
                y[l] = nanx_add(quad0123, quad4567);
            }
            if (end < w) {
                long cnt = w - end;
                long vecn = (cnt < 8) ? 0 : (cnt & ~7L);
                for (long j = 0; j < 4611686018427387904; j += 8) {
                    if (j >= vecn) {
                        break;
                    }
                    for (int l = 0; l < 8; l++) {
                        y[l] = nanx_add(y[l], data[base + end + j + l]);
                    }
                }
                for (long j = vecn; j < 4611686018427387904; j++) {
                    if (j >= cnt) {
                        break;
                    }
                    total = nanx_add(total, data[base + end + j]);
                }
            }
        }
        float pair_a = nanx_add(y[0], y[4]);
        float pair_b = nanx_add(y[1], y[5]);
        float pair_c = nanx_add(y[2], y[6]);
        float pair_d = nanx_add(y[3], y[7]);
        float pair_even = nanx_add(pair_a, pair_c);
        float pair_odd = nanx_add(pair_b, pair_d);
        total = nanx_add(nanx_add(pair_even, pair_odd), total);
    }
    dst[0] = total;
}

/* optical_flow flowstage.rs scan_range: one work item; dst[0] = max, dst[1] = min */
__kernel void flow_b_scan_range(__global const float* pl, __global float* dst, int w, int h, int pitch) {
    if (get_global_id(0) != 0) {
        return;
    }
    float mx = pl[0];
    float mn = mx;
    if (h > 0 && w > 0) {
        for (int y = 0; y < 2147483647; y++) {
            if (y >= h) {
                break;
            }
            int pw2 = (int)((uint)w >> 1);
            long row_base = (long)(int)((uint)y * (uint)pitch);
            for (int k = 0; k < 2147483647; k++) {
                if (k >= pw2) {
                    break;
                }
                float pair_a = pl[row_base + 2 * k];
                if (mn > pair_a) { mn = pair_a; } else if (pair_a > mx) { mx = pair_a; }
                float pair_b = pl[row_base + 2 * k + 1];
                if (mn > pair_b) { mn = pair_b; } else if (pair_b > mx) { mx = pair_b; }
            }
            if ((uint)w > 2u * (uint)pw2) {
                float xt = pl[(long)y * (long)pitch + 2 * (long)pw2];
                if (mn > xt) { mn = xt; } else if (xt > mx) { mx = xt; }
            }
        }
    }
    dst[0] = mx;
    dst[1] = mn;
}

/* the sqrt of the entry's final warp: a NaN is quieted, a negative operand gives the fresh NaN */
float flow_b_sqrt_x86(float x) {
    if (isnan(x)) {
        return as_float(as_uint(x) | 0x00400000u);
    }
    if (x < 0x0p+0f) {
        return as_float(0xffc00000u);
    }
    return sqrt(x);
}

int flow_b_cvt_floor(float x) {
    if (isnan(x) || x >= 0x1p31f || x < -0x1p31f) {
        return INT_MIN;
    }
    return (int)x;
}

/* optical_flow_entry final_warp_rows */
__kernel void flow_b_final_warp(__global const float* target, __global const float* img, __global const float* u, __global const float* v,
                             __global const float* gax, __global const float* gay, __global const float* gbx, __global const float* gby,
                             __global float* out, int width, int height, int stride, int row_start, int row_count,
                             uint scale_b, uint eps_b, uint margin_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int r = (int)(g / (uint)width);
    int col = (int)(g - (uint)r * (uint)width);
    int row = (int)((uint)row_start + (uint)r);
    long i = (long)flow_b_idx(row, stride) + col;
    float eps = as_float(eps_b);
    float margin = as_float(margin_b);
    float scale = as_float(scale_b);
    float limx = nanx_sub((float)width, margin);
    float limy = nanx_sub((float)height, margin);
    float sy = nanx_add((float)row, v[i]);
    float sx = nanx_add((float)col, u[i]);
    int outside = (eps >= sx) || (sx >= limx) || (eps >= sy) || (sy >= limy);
    float sample = 0x0p+0f;
    if (outside == 0) {
        float fxq = isnan(sx) ? as_float(as_uint(sx) | 0x00400000u) : floor(sx);
        float fyq = isnan(sy) ? as_float(as_uint(sy) | 0x00400000u) : floor(sy);
        int ix = flow_b_cvt_floor(fxq);
        int iy = flow_b_cvt_floor(fyq);
        float fx = nanx_sub(sx, (float)ix);
        float fy = nanx_sub(sy, (float)iy);
        sample = flow_b_bilinear(img, width, height, stride, ix, iy, fx, fy);
    }
    float ay = gay[i];
    float by = gby[i];
    float sq_a_y = nanx_mul(ay, ay);
    float sq_b_y = nanx_mul(by, by);
    float mag_a = flow_b_sqrt_x86(flow_b_f213(gax[i], gax[i], sq_a_y));
    float mag_b = flow_b_sqrt_x86(flow_b_f213(gbx[i], gbx[i], sq_b_y));
    float diff = nanx_sub(target[i], sample);
    float sum = nanx_add(mag_a, mag_b);
    float t = flow_b_f213(flow_b_abs(diff), scale, 0x1p+0f);
    out[i] = nanx_div(0x1p+0f, nanx_add(t, sum));
}
"#;

pub const FLOW_RESAMPLE: &str = r#"
/* MIRRORS_MUL_ADD: optical_flow_resample (fma213, fma231, fms213, fms231 in ks_down, ks_up, kv_down, kv_up and the base of the lane ramp) */

/* scalar kernel (ks_down, ks_up), t already folded to |t| */
float flow_b_ks(float t, int down) {
    if (down != 0) {
        if (t >= 0x1p+1f) {
            return 0x0p+0f;
        }
        if (0x1p+0f > t) {
            float x = flow_b_m213(0x1.8p+1f, t, 0x1.8p+2f);
            float t_sq = nanx_mul(t, t);
            return flow_b_f213(x, t_sq, 0x1p+2f);
        }
        float x = nanx_sub(0x1.8p+2f, t);
        x = flow_b_m213(x, t, 0x1.8p+3f);
        return flow_b_f213(x, t, 0x1p+3f);
    }
    if (t >= 0x1p+1f) {
        return 0x0p+0f;
    }
    if (0x1p+0f > t) {
        float x = flow_b_m213(t, 0x1.8p+0f, 0x1.4p+1f);
        float t_sq = nanx_mul(t, t);
        return flow_b_f213(x, t_sq, 0x1p+0f);
    }
    float x = flow_b_f213(-0x1p-1f, t, 0x1.4p+1f);
    x = flow_b_m213(x, t, 0x1p+2f);
    return flow_b_f213(x, t, 0x1p+1f);
}

/* eight lane kernel (kv_down, kv_up), one lane */
float flow_b_kv(float t, int down) {
    int ge = t >= 0x1p+1f;
    int lt = t < 0x1p+0f;
    if (down != 0) {
        if (ge == 0 && lt != 0) {
            float x = flow_b_m213(0x1.8p+1f, t, 0x1.8p+2f);
            float y = nanx_mul(t, x);
            return flow_b_f231(0x1p+2f, y, t);
        }
        if (ge == 0) {
            float x = nanx_sub(0x1.8p+2f, t);
            x = flow_b_m231(0x1.8p+3f, x, t);
            return flow_b_f213(t, x, 0x1p+3f);
        }
        return 0x0p+0f;
    }
    if (ge == 0 && lt != 0) {
        float x = flow_b_m213(0x1.8p+0f, t, 0x1.4p+1f);
        float y = nanx_mul(t, x);
        return flow_b_f231(0x1p+0f, y, t);
    }
    if (ge == 0) {
        float x = flow_b_f231(0x1.4p+1f, -0x1p-1f, t);
        x = flow_b_m231(0x1p+2f, x, t);
        return flow_b_f213(t, x, 0x1p+1f);
    }
    return 0x0p+0f;
}

/* lane reduction ((lane0+lane4)+(lane2+lane6)) + ((l1+lane5)+(lane3+lane7)) */
float flow_b_reduce8(float* l) {
    float pair0 = nanx_add(l[0], l[4]);
    float pair1 = nanx_add(l[1], l[5]);
    float pair2 = nanx_add(l[2], l[6]);
    float pair3 = nanx_add(l[3], l[7]);
    float quad0 = nanx_add(pair0, pair2);
    float quad1 = nanx_add(pair1, pair3);
    return nanx_add(quad0, quad1);
}

/* resample_h, output element x of row (row_start + r). tbl holds (lo, hi, start_t bits) per output x, computed on the host with the
   crate's own window arithmetic. */
__kernel void flow_b_resample_h(__global const float* src, __global float* dst, __global const uint* tbl, int w, int wd, int row_start,
                             int row_count, uint misalign, int down, uint scale_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int r = (int)(g / (uint)wd);
    int x = (int)(g - (uint)r * (uint)wd);
    int row = (int)((uint)row_start + (uint)r);
    long sbase = (long)(int)((uint)w * (uint)row);
    long dbase = (long)(int)((uint)wd * (uint)row);
    float scale = as_float(scale_b);
    long lo = (long)(int)tbl[3 * x];
    long hi = (long)(int)tbl[3 * x + 1];
    float start_t = as_float(tbl[3 * x + 2]);
    float acc = 0x0p+0f;
    float wsum = 0x0p+0f;
    if (lo < hi) {
        long count = hi - lo;
        long peel = 0;
        long vec_end = 0;
        int vec_ok = 0;
        if (count >= 8) {
            if (count < 0x4b0) {
                vec_end = count & ~7L;
                vec_ok = 1;
            } else {
                long addr = ((long)misalign + 4 * (sbase + lo)) & 0x1f;
                if (addr == 0) {
                    peel = 0;
                } else if ((addr & 3) != 0) {
                    peel = -1;
                } else {
                    peel = (32 - addr) >> 2;
                }
                if (peel >= 0 && count >= peel + 8) {
                    int count_i32 = (int)count;
                    vec_end = (long)(count_i32 - ((count_i32 - (int)peel) & 7));
                    vec_ok = 1;
                } else {
                    peel = 0;
                }
            }
        }
        float tcur = start_t;
        float step8 = nanx_mul(scale, 0x1p+3f);
        long sb = sbase + lo;
        if (vec_ok != 0) {
            for (long i = 0; i < 4611686018427387904; i++) {
                if (i >= peel) {
                    break;
                }
                float w_ = flow_b_ks(flow_b_abs(tcur), down);
                float prod = nanx_mul(w_, src[sb + i]);
                wsum = nanx_add(wsum, w_);
                tcur = nanx_add(tcur, scale);
                acc = nanx_add(prod, acc);
            }
            float base_t = (down != 0) ? start_t : nanx_add(0x0p+0f, start_t);
            base_t = flow_b_f231(base_t, (float)peel, scale);
            float tv[8];
            float wl[8];
            float lanes[8];
            for (int k = 0; k < 8; k++) {
                tv[k] = nanx_add(base_t, nanx_mul(scale, (float)k));
                wl[k] = 0x0p+0f;
                lanes[k] = 0x0p+0f;
            }
            wl[0] = wsum;
            lanes[0] = acc;
            for (long i = peel; i < 4611686018427387904; i += 8) {
                if (i >= vec_end) {
                    break;
                }
                for (int k = 0; k < 8; k++) {
                    float wv = flow_b_kv(flow_b_abs(tv[k]), down);
                    float prod = nanx_mul(wv, src[sb + i + k]);
                    wl[k] = nanx_add(wv, wl[k]);
                    lanes[k] = nanx_add(prod, lanes[k]);
                    tv[k] = nanx_add(tv[k], step8);
                }
                tcur = nanx_add(tcur, step8);
            }
            wsum = flow_b_reduce8(wl);
            acc = flow_b_reduce8(lanes);
        }
        long start = (vec_ok != 0) ? vec_end : 0;
        if (vec_ok == 0) {
            tcur = start_t;
        }
        for (long i = start; i < 4611686018427387904; i++) {
            if (i >= count) {
                break;
            }
            float w_ = flow_b_ks(flow_b_abs(tcur), down);
            float prod = nanx_mul(w_, src[sb + i]);
            wsum = nanx_add(wsum, w_);
            tcur = nanx_add(tcur, scale);
            acc = nanx_add(prod, acc);
        }
    }
    dst[dbase + x] = nanx_div(acc, wsum);
}

/* resample_v, output element (row y, column col_start + c) */
__kernel void flow_b_resample_v(__global const float* src, __global float* dst, __global const uint* tbl, int stride, int hs, int hd,
                             int col_start, int col_count, int down, uint scale_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int c = (int)(g / (uint)hd);
    int y = (int)(g - (uint)c * (uint)hd);
    long col = (long)col_start + (long)c;
    float scale = as_float(scale_b);
    long lo = (long)(int)tbl[3 * y];
    long hi = (long)(int)tbl[3 * y + 1];
    float start_t = as_float(tbl[3 * y + 2]);
    float acc = 0x0p+0f;
    float wsum = 0x0p+0f;
    if (lo < hi) {
        int count = (int)(hi - lo);
        float tcur = start_t;
        float step8 = nanx_mul(scale, 0x1p+3f);
        int vec_end = 0;
        int lo_off = flow_b_idx((int)lo, stride);
        if (stride != 0 && count >= 8) {
            vec_end = count & ~7;
            float tv[8];
            float wl[8];
            float lanes[8];
            for (int k = 0; k < 8; k++) {
                tv[k] = nanx_add(start_t, nanx_mul(scale, (float)k));
                wl[k] = 0x0p+0f;
                lanes[k] = 0x0p+0f;
            }
            for (int i = 0; i < 2147483647; i += 8) {
                if (i >= vec_end) {
                    break;
                }
                for (int k = 0; k < 8; k++) {
                    float wv = flow_b_kv(flow_b_abs(tv[k]), down);
                    int ii = i + k;
                    float prod = nanx_mul(src[col + (long)(int)((uint)flow_b_idx(stride, ii) + (uint)lo_off)], wv);
                    wl[k] = nanx_add(wv, wl[k]);
                    lanes[k] = nanx_add(prod, lanes[k]);
                    tv[k] = nanx_add(tv[k], step8);
                }
                tcur = nanx_add(tcur, step8);
            }
            wsum = flow_b_reduce8(wl);
            acc = flow_b_reduce8(lanes);
        }
        for (int i = vec_end; i < 2147483647; i++) {
            if (i >= count) {
                break;
            }
            float w_ = flow_b_ks(flow_b_abs(tcur), down);
            float prod = nanx_mul(w_, src[col + (long)(int)((uint)flow_b_idx(stride, i) + (uint)lo_off)]);
            wsum = nanx_add(wsum, w_);
            tcur = nanx_add(tcur, scale);
            acc = nanx_add(prod, acc);
        }
    }
    dst[col + (long)stride * (long)y] = nanx_div(acc, wsum);
}
"#;
