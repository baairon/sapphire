//! Device kernel sources of make_mipmap (the ripmap pyramid) and blur_warp (the four bodies A to D of the segment sampler).
//!
//! Every kernel reproduces the operation order and rounding of its CPU function (`make_mipmap::mm2`, `make_mipmap::build`,
//! `blur_warp::render` with `bwb_bodya`..`bwb_bodyd` and `bwb_p1`). `fma()` appears only where the CPU calls `mul_add`.
//!
//! * make_mipmap: the ripmap of one object is ONE device buffer holding the NL x NP padded planes one after the other (plane
//!   (l, j) at the offset the host passes); every pad row and column is written by the kernel that the CPU writes it with, in
//!   the CPU's order (base fill, base row pads, then per level the horizontal 2:1 kernel with its column pads and its row pads,
//!   then per row level the vertical 2:1 kernel with column pads and row pads).
//! * blur_warp: one work item per output sample (the CPU samples are independent: each reads only the endpoints at its own
//!   index and the plane). The four bodies share their control flow; the per body differences (channel count, which test form
//!   is used for NaN, the accumulation form, 64 bit index sums of body A, the specialised loop of body D) are the `bw_cfg`
//!   fields. The x86 vector head and tail split of the P1 sample (channel count 16 and over) cannot arise (1 to 4 channels),
//!   so the scalar element form is the only one reproduced.
//!
//! Out of contract data (a tap outside the ripmap plane, where the CPU kernel returns an error or panics) sets the error word
//! of the launch; the host then takes the node to the CPU, which reports its own error.

pub const RIPMAP_MIPMAP: &str = r#"
/* make_mipmap::mm2::base_fill, one work item per base pixel (all channels): format 2 (f32 bits) copy of source row y into
   the interior of plane (0, 0). Plane (0,0) starts at offset 0 of the ripmap buffer. */
__kernel void ripmap_mm_base(__global const float* s0, __global const float* s1, __global const float* s2, __global const float* s3,
                          __global uint* rip, int w, int h, int c, uint stride) {
    uint gid = (uint)get_global_id(0);
    uint total = (uint)w * (uint)h;
    if (gid >= total) {
        return;
    }
    uint y = gid / (uint)w;
    uint x = gid % (uint)w;
    uint base = (1u + y) * stride + (1u + x) * (uint)c;
    rip[base] = as_uint(s0[gid]);
    if (c >= 2) {
        rip[base + 1u] = as_uint(s1[gid]);
    }
    if (c >= 3) {
        rip[base + 2u] = as_uint(s2[gid]);
    }
    if (c >= 4) {
        rip[base + 3u] = as_uint(s3[gid]);
    }
}

/* the zero fill of a freshly allocated ripmap (the CPU object is filled with 0 bits) */
__kernel void ripmap_mm_fill(__global uint* rip, uint n) {
    uint gid = (uint)get_global_id(0);
    if (gid >= n) {
        return;
    }
    rip[gid] = 0u;
}

/* Column pads of one row (interior row y of the plane at `off`), width wl, channels c: left pad pixel, two right pad pixels.
   mode 1 tile (last, first, first), 2 clamp (first, last, last), else the constant pad pixel. One work item per (row, channel).
   Shared by slot 32's base fill and `make_mipmap::pad_cols`. */
__kernel void ripmap_mm_padcols(__global uint* rip, uint off, int wl, int c, uint stride, int rows, int mode, __global const uint* pad) {
    uint gid = (uint)get_global_id(0);
    uint total = (uint)rows * (uint)c;
    if (gid >= total) {
        return;
    }
    uint y = gid / (uint)c;
    uint ch = gid % (uint)c;
    uint base = off + (1u + y) * stride;
    uint first = rip[base + (uint)c + ch];
    uint last = rip[base + (uint)wl * (uint)c + ch];
    uint l;
    uint r1;
    uint r2;
    if (mode == 1) {
        l = last;
        r1 = first;
        r2 = first;
    } else if (mode == 2) {
        l = first;
        r1 = last;
        r2 = last;
    } else {
        l = pad[ch];
        r1 = pad[ch];
        r2 = pad[ch];
    }
    rip[base + ch] = l;
    rip[base + ((uint)wl + 1u) * (uint)c + ch] = r1;
    rip[base + ((uint)wl + 2u) * (uint)c + ch] = r2;
}

/* make_mipmap::pad_rows for the plane at `off` (stride floats per row, interior height h, c channels): rows 0, h+1, h+2.
   wy 0 constant (zero = all pad pixel lanes compare equal to 0.0: write +0), 1 tile (row h, row 1, row 1), 2 clamp
   (row 1, row h, row h), else nothing. One work item per (pad row, element). */
__kernel void ripmap_mm_padrows(__global uint* rip, uint off, int h, int c, uint stride, int wy, int zero, __global const uint* pad) {
    uint gid = (uint)get_global_id(0);
    uint total = 3u * stride;
    if (gid >= total) {
        return;
    }
    uint which = gid / stride;
    uint k = gid % stride;
    uint dst_row;
    if (which == 0u) {
        dst_row = 0u;
    } else if (which == 1u) {
        dst_row = (uint)h + 1u;
    } else {
        dst_row = (uint)h + 2u;
    }
    uint v;
    if (wy == 0) {
        if (zero) {
            v = 0u;
        } else {
            v = pad[(dst_row * stride + k) % (uint)c];
        }
    } else if (wy == 1) {
        uint srow;
        if (which == 0u) {
            srow = (uint)h;
        } else {
            srow = 1u;
        }
        v = rip[off + srow * stride + k];
    } else if (wy == 2) {
        uint srow;
        if (which == 0u) {
            srow = 1u;
        } else {
            srow = (uint)h;
        }
        v = rip[off + srow * stride + k];
    } else {
        return;
    }
    rip[off + dst_row * stride + k] = v;
}

/* kernel_a: horizontal 2:1 of level l from level l-1 (row level 0), one work item per output sample of the level.
   dst = 0.5 * (b + a) with a the even source pixel and b the odd one (avg(b, a): the sum is taken as b + a). */
__kernel void ripmap_mm_hor(__global uint* rip, uint soff, uint sstride, uint doff, uint dstride, int wl, int h0, int c) {
    uint gid = (uint)get_global_id(0);
    uint per_row = (uint)wl * (uint)c;
    uint total = per_row * (uint)h0;
    if (gid >= total) {
        return;
    }
    uint y = gid / per_row;
    uint rem = gid % per_row;
    uint x = rem / (uint)c;
    uint ch = rem % (uint)c;
    float a = as_float(rip[soff + (1u + y) * sstride + (1u + 2u * x) * (uint)c + ch]);
    float b = as_float(rip[soff + (1u + y) * sstride + (2u + 2u * x) * (uint)c + ch]);
    float s = b + a;
    rip[doff + (1u + y) * dstride + (1u + x) * (uint)c + ch] = as_uint(0x1p-1f * s);
}

/* kernel_b: vertical 2:1 of plane (l, j) from plane (l, j-1), one work item per output sample. */
__kernel void ripmap_mm_ver(__global uint* rip, uint soff, uint doff, uint stride, int wl, int hj, int c) {
    uint gid = (uint)get_global_id(0);
    uint per_row = (uint)wl * (uint)c;
    uint total = per_row * (uint)hj;
    if (gid >= total) {
        return;
    }
    uint y = gid / per_row;
    uint k = (1u * (gid % per_row)) + (uint)c;
    float a = as_float(rip[soff + (1u + 2u * y) * stride + k]);
    float b = as_float(rip[soff + (2u + 2u * y) * stride + k]);
    float s = b + a;
    rip[doff + (1u + y) * stride + k] = as_uint(0x1p-1f * s);
}
"#;

pub const RIPMAP_BLUR_WARP: &str = r#"
/* MIRRORS_MUL_ADD: bwb_bodya::s2_x, bwb_bodya::s3_y, bwb_bodya::run_planes_full, bwb_bodya::p2::x_major_sample, bwb_bodya::p2::y_major_sample, bwb_bodyb::x_major_sample2, bwb_bodyb::y_major_sample2, bwb_bodyc::x_major_sample, bwb_bodyc::y_major_sample, bwb_bodyd::x_major_sample_ex, bwb_bodyd::p3::spec_loop, bwb_bodyd::p3::flag0_sample, bwb_bodyd::p4::y_major_sample, bwb_p1::sample */

typedef struct {
    float x;
    float y;
    float e2x;
    float e2y;
    float dx;
    float dy;
    int w;
    int h;
    int stride;
    float k_a1_a0;
    float k_a2_a1;
    float k_1_a3;
    uint flag;
    int mx;
    int my;
} bw_s5;

typedef struct {
    float a0;
    float a1;
    float a3;
    float a4;
} bw_in;

/* the host statics: port_blk_a, cassumption_a, cassumption_b, cassumption_c, hs, ws */
typedef struct {
    int pa;
    int ca;
    int cb;
    int cc;
    float hs;
    float ws;
} bw_host;

typedef struct {
    __global const float* pl;
    long origin;
    long plen;
    int bad;
} bw_plane;

/* per body: nc channels; notlt 1 = the test `!(a < b)` (bodies A, B), 0 = `a >= b` (bodies C, D); acc_a 1 = body A's
   `lerp * w + s` accumulation, 0 = the `s - fma(v0 - v1, f, -v0) * w` form of B, C, D; idx64 1 = body A's 64 bit index sums;
   spec 1 = body D's specialised X-major loop. */
typedef struct {
    int nc;
    int notlt;
    int acc_a;
    int idx64;
    int spec;
} bw_cfg;

#define BW_ADD(a, b) ((int)((uint)(a) + (uint)(b)))
#define BW_SUB(a, b) ((int)((uint)(a) - (uint)(b)))
#define BW_MUL(a, b) ((int)((uint)(a) * (uint)(b)))

float bw_at(bw_plane* P, long idx) {
    long k = P->origin + idx;
    if (k < 0 || k >= P->plen) {
        P->bad = 1;
        return 0x0p+0f;
    }
    return P->pl[k];
}

float bw_vmin(float a, float b) {
    if (a < b) {
        return a;
    }
    return b;
}

int bw_nlt(bw_cfg g, float a, float b) {
    if (g.notlt) {
        return !(a < b);
    }
    return a >= b;
}

int bw_n_of(int n0) {
    int n = n0;
    if (n > 1000) {
        n = 1000;
    }
    if (n < 0) {
        n = 0;
    }
    return n;
}

/* abs of a 32 bit difference as the listing computes it: (e ^ (e >> 31)) - (e >> 31) + 1, wrapping */
int bw_count_of(int e) {
    int sg = e >> 31;
    return BW_ADD(BW_SUB(e ^ sg, sg), 1);
}

/* the accumulation of one tap pair into one channel sum */
float bw_acc(bw_cfg g, float s, float v0, float v1, float frac, float wgt) {
    if (g.acc_a) {
        float lerp = fma(v1 - v0, frac, v0);
        return lerp * wgt + s;
    }
    float t = fma(v0 - v1, frac, -v0);
    return s - t * wgt;
}

/* X-major sample, flag_cell_a != 0 (S6, S7) or == 0 (S8): res[0..nc] */
void bw_xmajor(bw_cfg g, bw_s5 st, bw_in a, bw_host H, bw_plane* P, float* res) {
    int w = st.w;
    int h = st.h;
    int stride = st.stride;
    float wf = (float)w;
    float hs = H.hs;
    int mx0 = st.mx;
    int my = st.my;
    int cond_any = (mx0 != 0) || (my > 0);
    float a0 = a.a0;
    float a1 = a.a1;
    float a3 = a.a3;
    float a4 = a.a4;
    int M = g.nc;
    float sums[4];
    for (int q0 = 0; q0 < 4; q0++) {
        sums[q0] = 0x0p+0f;
    }
    float wsum = 0x0p+0f;
    if (st.flag != 0u) {
        float x = st.x;
        float y = st.y;
        float dx = st.dx;
        float dy = st.dy;
        int dir;
        int c;
        int esi;
        int n0;
        float fx;
        float c0;
        float dxs;
        if (dx > 0x0p+0f) {
            float fl = floor(x);
            dir = 1;
            fx = x - fl;
            c = cvtt_i32_f(fl);
            float ce = ceil(st.e2x);
            esi = cvtt_i32_f(ce);
            c0 = ce - st.e2x;
            n0 = BW_ADD(BW_SUB(esi, c), 1);
            dxs = dx;
        } else {
            float ce = ceil(x);
            dir = -1;
            c = cvtt_i32_f(ce);
            float fl = floor(st.e2x);
            c0 = st.e2x - fl;
            fx = ce - x;
            esi = cvtt_i32_f(fl);
            n0 = BW_ADD(BW_SUB(c, esi), 1);
            dxs = -dx;
        }
        float slope = dy / dxs;
        float yy = fma(-slope, fx, y);
        float step_a = st.k_a1_a0 / ((float)n0 * a3);
        float y5 = floor(yy);
        float y9 = slope + st.e2y;
        int b_cell_c = 0;
        int direct0 = 0;
        if (cond_any) {
            int o = c < 0 || c >= w || 0x0p+0f > y5 || y5 >= hs || esi < 0 || esi >= w || 0x0p+0f > y9 || y9 >= hs;
            b_cell_c = o;
            direct0 = !o;
        }
        int b_cell_g = 0;
        if (!(direct0 || mx0 != 0)) {
            b_cell_g = c < 0 || c >= w || esi < 0 || esi >= w;
        }
        int b_cell_h = 0;
        if (my == 0) {
            b_cell_h = 0x0p+0f > y5 || y5 >= hs || 0x0p+0f > y9 || y9 >= hs;
        }
        int n = bw_n_of(n0);
        if (n > 0) {
            if (g.spec && !b_cell_c) {
                /* body D, specialised loops S10..S13: no edge wrap inside the loop */
                float nf = (float)n;
                float wlim = nf * a3;
                float step_b = st.k_a2_a1 / (nf * st.k_1_a3);
                float omfx = 0x1p+0f - fx;
                float omc0 = 0x1p+0f - c0;
                float hf = (float)h;
                float alpha = a0;
                float step = step_a;
                int al = 0;
                for (int k = 0; k < 1000; k++) {
                    if (k >= n) {
                        break;
                    }
                    if (!al && (float)k >= wlim) {
                        int q = k / n;
                        alpha = fma(a3 - (float)q, step_b, a1);
                        step = step_b;
                        al = 1;
                    }
                    float wgt;
                    if (k == 0) {
                        wgt = alpha * omfx;
                    } else if (k == n - 1) {
                        wgt = alpha * omc0;
                    } else {
                        wgt = alpha;
                    }
                    int cc = c;
                    if (b_cell_g) {
                        if (c >= -1) {
                            cc = c < w ? c : w;
                        } else {
                            cc = -1;
                        }
                    }
                    float ycl = yy;
                    if (b_cell_h) {
                        if (-0x1p+0f > yy) {
                            ycl = -0x1p+0f;
                        } else {
                            ycl = bw_vmin(hf, yy);
                        }
                    }
                    wsum = wgt + wsum;
                    float yy_old = yy;
                    yy = slope + yy;
                    alpha = alpha + step;
                    c = BW_ADD(c, dir);
                    float fr = 0x1p+0f + ycl;
                    int t = cvtt_i32_f(fr);
                    float tf = (float)t;
                    float frac = fr - tf;
                    int idx = BW_ADD(BW_SUB(BW_MUL(cc, 4), stride), BW_MUL(t, stride));
                    for (int ch = 0; ch < 3; ch++) {
                        float v0 = bw_at(P, (long)idx + (long)ch);
                        float v1 = bw_at(P, (long)idx + (long)stride + (long)ch);
                        sums[ch] = bw_acc(g, sums[ch], v0, v1, frac, wgt);
                    }
                    float v0 = bw_at(P, (long)idx + 3L);
                    float v1 = bw_at(P, (long)idx + (long)stride + 3L);
                    if (b_cell_h) {
                        sums[3] = bw_acc(g, sums[3], v0, v1, frac, wgt);
                    } else {
                        float b = fma(frac - 0x1p+0f, v0, -v1);
                        float d = fma(tf, v1, b);
                        float e = fma(v1, yy_old, -d);
                        sums[3] = e * wgt + sums[3];
                    }
                }
            } else {
                float hfi = (float)cvtt_i32_f(hs);
                float omfx = 0x1p+0f - fx;
                float omc0 = 0x1p+0f - c0;
                float nf = (float)n;
                float wlim = nf * a3;
                float step_b = st.k_a2_a1 / (nf * st.k_1_a3);
                float twoh = fma(0x1p+1f, hfi, -0x1p+0f);
                int colmax = cvtt_i32_f(wf - 0x1p+0f);
                float hf = (float)h;
                float alpha = a0;
                float step = step_a;
                int al = 0;
                for (int k = 0; k < 1000; k++) {
                    if (k >= n) {
                        break;
                    }
                    if (!al && (float)k >= wlim) {
                        int q = k / n;
                        alpha = fma(a3 - (float)q, step_b, a1);
                        step = step_b;
                        al = 1;
                    }
                    if (b_cell_c) {
                        if ((float)c >= wf) {
                            if (H.pa != 0) {
                                c = colmax;
                                dir = -1;
                            } else if (H.cb != 0) {
                                c = 0;
                            }
                        } else if (c < 0) {
                            if (H.pa != 0) {
                                c = 0;
                                dir = 1;
                            } else if (H.cb != 0) {
                                c = colmax;
                            }
                        }
                        if (bw_nlt(g, yy, hfi)) {
                            if (H.ca != 0) {
                                slope = -slope;
                                yy = twoh - yy;
                            } else if (H.cc != 0) {
                                yy = yy - hfi;
                            }
                        } else if (0x0p+0f > yy) {
                            if (H.ca != 0) {
                                yy = -yy;
                                slope = -slope;
                            } else if (H.cc != 0) {
                                yy = yy + hfi;
                            }
                        }
                    }
                    float wgt;
                    if (k == 0) {
                        wgt = alpha * omfx;
                    } else if (k == n - 1) {
                        wgt = alpha * omc0;
                    } else {
                        wgt = alpha;
                    }
                    int cc = c;
                    if (b_cell_g) {
                        if (c >= -1) {
                            cc = c < w ? c : w;
                        } else {
                            cc = -1;
                        }
                    }
                    float ycl = yy;
                    if (b_cell_h) {
                        if (-0x1p+0f > yy) {
                            ycl = -0x1p+0f;
                        } else {
                            ycl = bw_vmin(hf, yy);
                        }
                    }
                    wsum = wgt + wsum;
                    yy = slope + yy;
                    alpha = alpha + step;
                    c = BW_ADD(c, dir);
                    float fr = 0x1p+0f + ycl;
                    int t = cvtt_i32_f(fr);
                    float frac = fr - (float)t;
                    int idx = BW_ADD(BW_SUB(BW_MUL(cc, M), stride), BW_MUL(t, stride));
                    for (int ch = 0; ch < 4; ch++) {
                        if (ch >= M) {
                            break;
                        }
                        float v0 = bw_at(P, (long)idx + (long)ch);
                        float v1 = bw_at(P, (long)idx + (long)stride + (long)ch);
                        sums[ch] = bw_acc(g, sums[ch], v0, v1, frac, wgt);
                    }
                }
            }
        }
    } else {
        float xa = 0x1p-1f + st.x;
        float xe = 0x1p-1f + st.e2x;
        float ya = 0x1p-1f + st.y;
        float ye = 0x1p-1f + st.e2y;
        int b_cell_i = 0;
        if (cond_any) {
            b_cell_i = 0x0p+0f > xe || xe >= wf || 0x0p+0f > ye || ye >= hs;
        }
        int b_cell_k = 0;
        if (mx0 == 0) {
            b_cell_k = 0x0p+0f > xa || xa >= wf || 0x0p+0f > xe || xe >= wf;
        }
        int b_cell_j = 0;
        if (my == 0) {
            b_cell_j = 0x0p+0f > ya || ya >= hs || 0x0p+0f > ye || ye >= hs;
        }
        int cs = cvtt_i32_f(xa);
        int e = BW_SUB(cvtt_i32_f(xe), cs);
        int m = bw_count_of(e);
        int r9 = 1;
        if (e < 0) {
            r9 = -1;
        }
        float mf = (float)m;
        float ystep = (ye - ya) / mf;
        float step_a = st.k_a1_a0 / (mf * a3);
        int count = bw_n_of(m);
        if (count > 0) {
            float cf = (float)count;
            float step_b = st.k_a2_a1 / (cf * st.k_1_a3);
            float wlim = cf * a3;
            float hsi = (float)cvtt_i32_f(hs);
            float twoh = fma(0x1p+1f, hsi, -0x1p+0f);
            int colmax = cvtt_i32_f(wf - 0x1p+0f);
            float alpha = a0;
            float step = step_a;
            int al = 0;
            for (int k = 0; k < 1000; k++) {
                if (k >= count) {
                    break;
                }
                int rown = cvtt_i32_f(ya);
                if (!al && (float)k >= wlim) {
                    int q = k / count;
                    alpha = fma(a3 - (float)q, step_b, a1);
                    step = step_b;
                    al = 1;
                }
                int col = cs;
                if (b_cell_k) {
                    if (cs >= -1) {
                        col = cs < w ? cs : w;
                    } else {
                        col = -1;
                    }
                }
                if (b_cell_j) {
                    if (rown >= -1) {
                        rown = rown < h ? rown : h;
                    } else {
                        rown = -1;
                    }
                }
                long idx;
                if (g.idx64) {
                    idx = (long)col + (long)BW_MUL(rown, stride);
                } else {
                    idx = (long)BW_ADD(BW_MUL(col, M), BW_MUL(rown, stride));
                }
                cs = BW_ADD(cs, r9);
                wsum = wsum + alpha;
                ya = ya + ystep;
                for (int ch = 0; ch < 4; ch++) {
                    if (ch >= M) {
                        break;
                    }
                    float v = bw_at(P, idx + (long)ch);
                    float prod = alpha * v;
                    sums[ch] = prod + sums[ch];
                }
                alpha = alpha + step;
                if (b_cell_i) {
                    if ((float)cs >= wf) {
                        if (H.pa != 0) {
                            r9 = -1;
                            cs = colmax;
                        } else if (H.cb != 0) {
                            cs = 0;
                        }
                    } else if (cs < 0) {
                        if (H.pa != 0) {
                            cs = 0;
                            r9 = 1;
                        } else if (H.cb != 0) {
                            cs = colmax;
                        }
                    }
                    if (bw_nlt(g, ya, hsi)) {
                        if (H.ca != 0) {
                            ystep = -ystep;
                            ya = twoh - ya;
                        } else if (H.cc != 0) {
                            ya = ya - hsi;
                        }
                    } else if (0x0p+0f > ya) {
                        if (H.ca != 0) {
                            ya = -ya;
                            ystep = -ystep;
                        } else if (H.cc != 0) {
                            ya = ya + hsi;
                        }
                    }
                }
            }
        }
    }
    float scale = a4 / wsum;
    for (int ch = 0; ch < 4; ch++) {
        res[ch] = sums[ch] * scale;
    }
}

/* Y-major sample, flag_cell_a != 0 (S9) or == 0 (S10) */
void bw_ymajor(bw_cfg g, bw_s5 st, bw_in a, bw_host H, bw_plane* P, float* res) {
    int w = st.w;
    int h = st.h;
    int stride = st.stride;
    float wf = (float)w;
    float hf = (float)h;
    float ws = H.ws;
    int mx0 = st.mx;
    int my = st.my;
    int cond_any = (mx0 != 0) || (my > 0);
    float a0 = a.a0;
    float a1 = a.a1;
    float a3 = a.a3;
    float a4 = a.a4;
    int M = g.nc;
    float sums[4];
    for (int q0 = 0; q0 < 4; q0++) {
        sums[q0] = 0x0p+0f;
    }
    float wsum = 0x0p+0f;
    if (st.flag != 0u) {
        float x = st.x;
        float y = st.y;
        float dx = st.dx;
        float dy = st.dy;
        int r9;
        int esi;
        int n0;
        float fy;
        float c0;
        float dys;
        int dir;
        if (dy > 0x0p+0f) {
            float fl = floor(y);
            fy = y - fl;
            r9 = cvtt_i32_f(fl);
            float ce = ceil(st.e2y);
            c0 = ce - st.e2y;
            esi = cvtt_i32_f(ce);
            n0 = BW_ADD(BW_SUB(esi, r9), 1);
            dir = 1;
            dys = dy;
        } else {
            float ce = ceil(y);
            r9 = cvtt_i32_f(ce);
            fy = ce - y;
            float fl = floor(st.e2y);
            c0 = st.e2y - fl;
            esi = cvtt_i32_f(fl);
            n0 = BW_ADD(BW_SUB(r9, esi), 1);
            dir = -1;
            dys = -dy;
        }
        float slope = dx / dys;
        float xx = fma(-slope, fy, x);
        float step_a = st.k_a1_a0 / ((float)n0 * a3);
        float x5 = floor(xx);
        float x9 = slope + st.e2x;
        int b_cell_c = 0;
        int direct0 = 0;
        if (cond_any) {
            int o = 0x0p+0f > x5 || x5 >= ws || r9 < 0 || r9 >= h || 0x0p+0f > x9 || x9 >= ws || esi < 0 || esi >= h;
            b_cell_c = o;
            direct0 = !o;
        }
        int r10 = 0;
        if (!(direct0 || mx0 != 0)) {
            r10 = 0x0p+0f > x5 || x5 >= ws || 0x0p+0f > x9 || x9 >= ws;
        }
        int r11 = 0;
        if (my == 0) {
            r11 = r9 < 0 || r9 >= h || esi < 0 || esi >= h;
        }
        int n = bw_n_of(n0);
        if (n > 0) {
            float omfy = 0x1p+0f - fy;
            float omc0 = 0x1p+0f - c0;
            float nf = (float)n;
            float wlim = nf * a3;
            float step_b = st.k_a2_a1 / (nf * st.k_1_a3);
            float twow = fma(0x1p+1f, ws, -0x1p+0f);
            float alpha = a0;
            float step = step_a;
            int al = 0;
            for (int k = 0; k < 1000; k++) {
                if (k >= n) {
                    break;
                }
                if (!al && (float)k >= wlim) {
                    int q = k / n;
                    alpha = fma(a3 - (float)q, step_b, a1);
                    step = step_b;
                    al = 1;
                }
                if (b_cell_c) {
                    if (bw_nlt(g, xx, ws)) {
                        if (H.pa != 0) {
                            slope = -slope;
                            xx = twow - xx;
                        } else if (H.cb != 0) {
                            xx = xx - ws;
                        }
                    } else if (0x0p+0f > xx) {
                        if (H.pa != 0) {
                            xx = -xx;
                            slope = -slope;
                        } else if (H.cb != 0) {
                            xx = xx + ws;
                        }
                    }
                    if (r9 >= h) {
                        if (H.ca != 0) {
                            r9 = h - 1;
                            dir = -1;
                        } else if (H.cc != 0) {
                            r9 = 0;
                        }
                    } else if (r9 < 0) {
                        if (H.ca != 0) {
                            r9 = 0;
                            dir = 1;
                        } else if (H.cc != 0) {
                            r9 = h - 1;
                        }
                    }
                }
                float wgt;
                if (k == 0) {
                    wgt = alpha * omfy;
                } else if (k == n - 1) {
                    wgt = alpha * omc0;
                } else {
                    wgt = alpha;
                }
                float xc = xx;
                if (r10) {
                    if (-0x1p+0f > xx) {
                        xc = -0x1p+0f;
                    } else {
                        xc = bw_vmin(wf, xx);
                    }
                }
                int rr = r9;
                if (r11) {
                    if (r9 >= -1) {
                        rr = r9 < h ? r9 : h;
                    } else {
                        rr = -1;
                    }
                }
                wsum = wgt + wsum;
                xx = slope + xx;
                alpha = alpha + step;
                float fr = 0x1p+0f + xc;
                int t = cvtt_i32_f(fr);
                float frac = fr - (float)t;
                long idx;
                if (g.idx64) {
                    idx = (long)rr * (long)stride + (long)t;
                } else {
                    idx = (long)BW_ADD(BW_MUL(rr, stride), BW_MUL(t, M));
                }
                r9 = BW_ADD(r9, dir);
                for (int ch = 0; ch < 4; ch++) {
                    if (ch >= M) {
                        break;
                    }
                    float v0 = bw_at(P, idx - (long)M + (long)ch);
                    float v1 = bw_at(P, idx + (long)ch);
                    sums[ch] = bw_acc(g, sums[ch], v0, v1, frac, wgt);
                }
            }
        }
    } else {
        float xe = 0x1p-1f + st.e2x;
        float xa = 0x1p-1f + st.x;
        float ya = 0x1p-1f + st.y;
        float ye = 0x1p-1f + st.e2y;
        int b_cell_d = 0;
        if (cond_any) {
            b_cell_d = 0x0p+0f > xe || xe >= ws || 0x0p+0f > ye || ye >= hf;
        }
        int b_cell_f = 0;
        if (mx0 == 0) {
            b_cell_f = 0x0p+0f > xa || xa >= ws || 0x0p+0f > xe || xe >= ws;
        }
        int b_cell_e = 0;
        if (my == 0) {
            b_cell_e = 0x0p+0f > ya || ya >= hf || 0x0p+0f > ye || ye >= hf;
        }
        int r11 = cvtt_i32_f(ya);
        int e = BW_SUB(cvtt_i32_f(ye), r11);
        int m = bw_count_of(e);
        int dir = 1;
        if (e < 0) {
            dir = -1;
        }
        float mf = (float)m;
        float xstep = (xe - xa) / mf;
        float step_a = st.k_a1_a0 / (mf * a3);
        int count = bw_n_of(m);
        if (count > 0) {
            float cf = (float)count;
            float step_b = st.k_a2_a1 / (cf * st.k_1_a3);
            float wlim = cf * a3;
            float twow = fma(0x1p+1f, ws, -0x1p+0f);
            float alpha = a0;
            float step = step_a;
            int al = 0;
            float x = xa;
            for (int k = 0; k < 1000; k++) {
                if (k >= count) {
                    break;
                }
                if (!al && (float)k >= wlim) {
                    int q = k / count;
                    alpha = fma(a3 - (float)q, step_b, a1);
                    step = step_b;
                    al = 1;
                }
                int xi = cvtt_i32_f(x);
                int xcl = xi;
                if (b_cell_f) {
                    if (xi >= -1) {
                        xcl = xi < w ? xi : w;
                    } else {
                        xcl = -1;
                    }
                }
                int rowc = r11;
                if (b_cell_e) {
                    if (r11 >= -1) {
                        rowc = r11 < h ? r11 : h;
                    } else {
                        rowc = -1;
                    }
                }
                long idx;
                if (g.idx64) {
                    idx = (long)xcl + (long)BW_MUL(rowc, stride);
                } else {
                    idx = (long)BW_ADD(BW_MUL(xcl, M), BW_MUL(rowc, stride));
                }
                r11 = BW_ADD(r11, dir);
                wsum = wsum + alpha;
                x = x + xstep;
                for (int ch = 0; ch < 4; ch++) {
                    if (ch >= M) {
                        break;
                    }
                    float v = bw_at(P, idx + (long)ch);
                    float prod = alpha * v;
                    sums[ch] = prod + sums[ch];
                }
                alpha = alpha + step;
                if (b_cell_d) {
                    if (bw_nlt(g, x, ws)) {
                        if (H.pa != 0) {
                            xstep = -xstep;
                            x = twow - x;
                        } else if (H.cb != 0) {
                            x = x - ws;
                        }
                    } else if (0x0p+0f > x) {
                        if (H.pa != 0) {
                            x = -x;
                            xstep = -xstep;
                        } else if (H.cb != 0) {
                            x = x + ws;
                        }
                    }
                    if (r11 >= h) {
                        if (H.ca != 0) {
                            dir = -1;
                            r11 = h - 1;
                        } else if (H.cc != 0) {
                            r11 = 0;
                        }
                    } else if (r11 < 0) {
                        if (H.ca != 0) {
                            r11 = 0;
                            dir = 1;
                        } else if (H.cc != 0) {
                            r11 = h - 1;
                        }
                    }
                }
            }
        }
    }
    float scale = a4 / wsum;
    for (int ch = 0; ch < 4; ch++) {
        res[ch] = sums[ch] * scale;
    }
}

/* bwb_p1::sample for 1 to 4 channels (the scalar element form), then the planes times a4. The CPU indexes the whole plane
   buffer by `i as usize` (a panic outside it, which is an error of the call): the same range test sets `bad`. */
void bw_p1(bw_cfg g, int mode_x, int mode_y, int width, int height, int stride, float x, float y, float a4, bw_plane* P, float* res) {
    int n = g.nc;
    float xc = x;
    if (mode_x == 0) {
        if (-0x1p+0f > x) {
            xc = -0x1p+0f;
        } else {
            xc = bw_vmin((float)width, x);
        }
    }
    float yc = y;
    if (mode_y == 0) {
        if (-0x1p+0f > y) {
            yc = -0x1p+0f;
        } else {
            yc = bw_vmin((float)height, y);
        }
    }
    float x1 = 0x1p+0f + xc;
    float y1 = 0x1p+0f + yc;
    int ix = cvtt_i32_f(x1);
    int iy = cvtt_i32_f(y1);
    float fx = x1 - (float)ix;
    float fy = y1 - (float)iy;
    long offx = (long)BW_MUL(BW_SUB(ix, 1), n);
    long offy = (long)BW_MUL(BW_SUB(iy, 1), stride);
    long pos = P->origin + offx + offy;
    float fxy = fx * fy;
    float w00 = ((0x1p+0f - fx) - fy) + fxy;
    float w10 = fx - fxy;
    float w01 = fy - fxy;
    float w11 = fxy;
    long st = (long)stride;
    long nl = (long)n;
    for (int j = 0; j < 4; j++) {
        res[j] = 0x0p+0f;
    }
    for (int j = 0; j < 4; j++) {
        if (j >= n) {
            break;
        }
        long q = pos + (long)j;
        long ia = q;
        long ib = q + nl;
        long ic = q + st;
        long id = q + nl + st;
        if (ia < 0 || ia >= P->plen || ib < 0 || ib >= P->plen || ic < 0 || ic >= P->plen || id < 0 || id >= P->plen) {
            P->bad = 1;
            return;
        }
        float va = P->pl[ia];
        float vb = P->pl[ib];
        float vc = P->pl[ic];
        float vd = P->pl[id];
        float s1 = w11 * vd;
        float p = fma(w10, vb, s1);
        float s2 = w01 * vc;
        float r = fma(w00, va, s2);
        res[j] = p + r;
    }
    for (int j = 0; j < 4; j++) {
        if (j >= n) {
            break;
        }
        res[j] = a4 * res[j];
    }
}

__kernel void ripmap_blur_warp_f32(__global const float* pl, __global const int* ip, __global const float* fp,
                                __global const float* p1x, __global const float* p1y, __global const float* p2x, __global const float* p2y,
                                __global float* o0, __global float* o1, __global float* o2, __global float* o3, __global uint* err) {
    uint gid = (uint)get_global_id(0);
    uint total = (uint)ip[19];
    if (gid >= total) {
        return;
    }
    bw_cfg g;
    int body = ip[0];
    g.nc = body;
    g.notlt = (body <= 2) ? 1 : 0;
    g.acc_a = (body == 1) ? 1 : 0;
    g.idx64 = (body == 1) ? 1 : 0;
    g.spec = (body == 4) ? 1 : 0;
    bw_plane P;
    P.pl = pl;
    P.plen = (long)ip[1];
    P.origin = (long)ip[2];
    P.bad = 0;
    int stride = ip[3];
    int img_w = ip[4];
    int img_h = ip[5];
    int w0 = ip[6];
    int h0 = ip[7];
    int mode_x = ip[8];
    int mode_y = ip[9];
    uint flag = (uint)ip[10];
    bw_host H;
    H.pa = ip[11];
    H.ca = ip[12];
    H.cb = ip[13];
    H.cc = ip[14];
    H.hs = fp[5];
    H.ws = fp[6];
    int p1c = ip[15];
    int p2c = ip[16];
    int roi_x = ip[17];
    int roi_y = ip[18];
    bw_in a;
    float fa0 = fp[0];
    float fa1 = fp[1];
    float fa2 = fp[2];
    float fa3 = fp[3];
    float fa4 = fp[4];
    a.a0 = fa0;
    a.a1 = fa1;
    a.a3 = fa3;
    a.a4 = fa4;
    float aspect = fp[7];
    float geo_e = fp[8];
    float left = fp[9];
    float right = fp[10];
    float geo_h = fp[11];
    float l_p1x = fp[12];
    float l_p1y = fp[13];
    float l_p2x = fp[14];
    float l_p2y = fp[15];
    if (!p1c) {
        l_p1x = p1x[gid];
        l_p1y = p1y[gid];
    }
    if (!p2c) {
        l_p2x = p2x[gid];
        l_p2y = p2y[gid];
    }
    float k_1_a3 = 0x1p+0f - fa3;
    float k_a1_a0 = fa1 - fa0;
    float k_a2_a1 = fa2 - fa1;
    float x12 = 0x1p-1f * aspect;
    float x13 = right - left;
    float scx = 0x1p-1f * x13;
    float scy = x12 * x13;
    /* S2, S3 */
    float tx = l_p1x + 0x1p+0f;
    float basex = left - (float)roi_x;
    float xr = fma(tx, scx, basex);
    float x = warp_wrap(xr, img_w, mode_x);
    float ty = l_p1y - 0x1p+0f;
    float my_ = fma(scy, ty, -geo_h);
    float topy = geo_e - (float)roi_y;
    float yr = topy - my_;
    float y = warp_wrap(yr, img_h, mode_y);
    float e2x = fma(l_p2x - l_p1x, scx, x);
    float e2y = fma(-(l_p2y - l_p1y), scy, y);
    float res[4];
    for (int j = 0; j < 4; j++) {
        res[j] = 0x0p+0f;
    }
    if (x == e2x && y == e2y) {
        bw_p1(g, mode_x, mode_y, w0, h0, stride, x, y, fa4, &P, res);
    } else {
        bw_s5 st;
        st.x = x;
        st.y = y;
        st.e2x = e2x;
        st.e2y = e2y;
        st.dx = e2x - x;
        st.dy = e2y - y;
        st.w = img_w;
        st.h = img_h;
        st.stride = stride;
        st.k_a1_a0 = k_a1_a0;
        st.k_a2_a1 = k_a2_a1;
        st.k_1_a3 = k_1_a3;
        st.flag = flag;
        st.mx = mode_x;
        st.my = mode_y;
        float adx = fabs(st.dx);
        float ady = fabs(st.dy);
        if (adx >= ady) {
            bw_xmajor(g, st, a, H, &P, res);
        } else {
            bw_ymajor(g, st, a, H, &P, res);
        }
    }
    if (P.bad) {
        atomic_or(err, 1u);
        return;
    }
    o0[gid] = res[0];
    if (body >= 2) {
        o1[gid] = res[1];
    }
    if (body >= 3) {
        o2[gid] = res[2];
    }
    if (body >= 4) {
        o3[gid] = res[3];
    }
}
"#;
