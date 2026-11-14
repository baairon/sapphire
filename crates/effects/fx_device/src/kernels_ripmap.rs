//! Device kernel sources of make_mipmap (the ripmap pyramid) and blur_warp (the four bodies A to D of the segment sampler).
//!
//! Every kernel reproduces the operation order and rounding of its CPU function (`make_mipmap::mm2`, `make_mipmap::build`,
//! `blur_warp::render` with `blur_warp_1ch`..`blur_warp_4ch` and `blur_warp_point`). `fma()` appears only where the CPU calls `mul_add`.
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
__kernel void ripmap_mm_base(__global const float* s0, __global const float* src_ch1, __global const float* src_ch2, __global const float* src_ch3,
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
        rip[base + 1u] = as_uint(src_ch1[gid]);
    }
    if (c >= 3) {
        rip[base + 2u] = as_uint(src_ch2[gid]);
    }
    if (c >= 4) {
        rip[base + 3u] = as_uint(src_ch3[gid]);
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
    uint chan = gid % (uint)c;
    uint base = off + (1u + y) * stride;
    uint first = rip[base + (uint)c + chan];
    uint last = rip[base + (uint)wl * (uint)c + chan];
    uint l;
    uint edge_sample;
    uint r2;
    if (mode == 1) {
        l = last;
        edge_sample = first;
        r2 = first;
    } else if (mode == 2) {
        l = first;
        edge_sample = last;
        r2 = last;
    } else {
        l = pad[chan];
        edge_sample = pad[chan];
        r2 = pad[chan];
    }
    rip[base + chan] = l;
    rip[base + ((uint)wl + 1u) * (uint)c + chan] = edge_sample;
    rip[base + ((uint)wl + 2u) * (uint)c + chan] = r2;
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

/* make_mipmap's sum of two words with every NaN computed by explicit tests, as `make_mipmap::fadd` writes it: two NaN give
   the quiet one when exactly one is signalling, else the first operand; one NaN gives that NaN; each quieted by OR 0x00400000.
   A NaN made by the sum itself (inf + -inf) is written as 0xffc00000. */
int mm_isnan(uint v) {
    return (v & 0x7f800000u) == 0x7f800000u && (v & 0x007fffffu) != 0u;
}

uint mm_fadd(uint a, uint b) {
    int na = mm_isnan(a);
    int nb = mm_isnan(b);
    if (na && nb) {
        int sa = (a & 0x00400000u) == 0u;
        int sb = (b & 0x00400000u) == 0u;
        if (sa && !sb) {
            return b;
        }
        if (!sa && sb) {
            return a;
        }
        return a | 0x00400000u;
    }
    if (na) {
        return a | 0x00400000u;
    }
    if (nb) {
        return b | 0x00400000u;
    }
    float s = as_float(a) + as_float(b);
    if (isnan(s)) {
        return 0xffc00000u;
    }
    return as_uint(s);
}

/* make_mipmap::avg: the sum, a NaN sum quieted, else 0.5 times the sum */
uint mm_avg(uint a, uint b) {
    uint s = mm_fadd(a, b);
    if (mm_isnan(s)) {
        return s | 0x00400000u;
    }
    return as_uint(0x1p-1f * as_float(s));
}

/* kernel_a: horizontal 2:1 of level l from level l-1 (row level 0), one work item per output sample of the level.
   dst = avg(b, a) with a the even source pixel and b the odd one. */
__kernel void ripmap_mm_hor(__global uint* rip, uint soff, uint sstride, uint doff, uint dstride, int wl, int grid_h, int c) {
    uint gid = (uint)get_global_id(0);
    uint per_row = (uint)wl * (uint)c;
    uint total = per_row * (uint)grid_h;
    if (gid >= total) {
        return;
    }
    uint y = gid / per_row;
    uint rem = gid % per_row;
    uint x = rem / (uint)c;
    uint chan = rem % (uint)c;
    uint a = rip[soff + (1u + y) * sstride + (1u + 2u * x) * (uint)c + chan];
    uint b = rip[soff + (1u + y) * sstride + (2u + 2u * x) * (uint)c + chan];
    rip[doff + (1u + y) * dstride + (1u + x) * (uint)c + chan] = mm_avg(b, a);
}

/* kernel_b: vertical 2:1 of plane (l, j) from plane (l, j-1), one work item per output sample: avg(b, a) with a the upper row. */
__kernel void ripmap_mm_ver(__global uint* rip, uint soff, uint doff, uint stride, int wl, int hj, int c) {
    uint gid = (uint)get_global_id(0);
    uint per_row = (uint)wl * (uint)c;
    uint total = per_row * (uint)hj;
    if (gid >= total) {
        return;
    }
    uint y = gid / per_row;
    uint k = (1u * (gid % per_row)) + (uint)c;
    uint a = rip[soff + (1u + 2u * y) * stride + k];
    uint b = rip[soff + (2u + 2u * y) * stride + k];
    rip[doff + (1u + y) * stride + k] = mm_avg(b, a);
}
"#;

pub const RIPMAP_BLUR_WARP: &str = r#"
/* MIRRORS_MUL_ADD: blur_warp_1ch::s2_x, blur_warp_1ch::s3_y, blur_warp_1ch::run_planes_full, blur_warp_1ch::p2::x_major_sample, blur_warp_1ch::p2::y_major_sample, blur_warp_2ch::x_major_sample2, blur_warp_2ch::y_major_sample2, blur_warp_3ch::x_major_sample, blur_warp_3ch::y_major_sample, blur_warp_4ch::x_major_sample_ex, blur_warp_4ch::p3::spec_loop, blur_warp_4ch::p3::flag0_sample, blur_warp_4ch::p4::y_major_sample, blur_warp_point::sample */

typedef struct {
    float x;
    float y;
    float e2x;
    float e2y;
    float delta_x;
    float dy;
    int w;
    int h;
    int stride;
    float a_gap_first;
    float a_gap_second;
    float one_minus_a_last;
    uint flag;
    int mx;
    int my;
} bw_s5;

typedef struct {
    float alpha_init;
    float alpha_origin;
    float tap_width;
    float norm_num;
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
   `lerp * w + s` accumulation, 0 = the `s - fma(sample_a - sample_b, f, -sample_a) * w` form of B, C, D; idx64 1 = body A's 64 bit index sums;
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

int bw_n_of(int tap_span) {
    int n = tap_span;
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
float bw_acc(bw_cfg g, float s, float sample_a, float sample_b, float frac, float wgt) {
    if (g.acc_a) {
        float lerp = fma(sample_b - sample_a, frac, sample_a);
        return lerp * wgt + s;
    }
    float t = fma(sample_a - sample_b, frac, -sample_a);
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
    float alpha_init = a.alpha_init;
    float alpha_origin = a.alpha_origin;
    float tap_width = a.tap_width;
    float norm_num = a.norm_num;
    int M = g.nc;
    float sums[4];
    for (int slot_no = 0; slot_no < 4; slot_no++) {
        sums[slot_no] = 0x0p+0f;
    }
    float wsum = 0x0p+0f;
    if (st.flag != 0u) {
        float x = st.x;
        float y = st.y;
        float delta_x = st.delta_x;
        float dy = st.dy;
        int dir;
        int c;
        int tap_b;
        int tap_span;
        float fx;
        float edge_gap;
        float dxs;
        if (delta_x > 0x0p+0f) {
            float fl = floor(x);
            dir = 1;
            fx = x - fl;
            c = cvtt_i32_f(fl);
            float ce = ceil(st.e2x);
            tap_b = cvtt_i32_f(ce);
            edge_gap = ce - st.e2x;
            tap_span = BW_ADD(BW_SUB(tap_b, c), 1);
            dxs = delta_x;
        } else {
            float ce = ceil(x);
            dir = -1;
            c = cvtt_i32_f(ce);
            float fl = floor(st.e2x);
            edge_gap = st.e2x - fl;
            fx = ce - x;
            tap_b = cvtt_i32_f(fl);
            tap_span = BW_ADD(BW_SUB(c, tap_b), 1);
            dxs = -delta_x;
        }
        float slope = dy / dxs;
        float yy = fma(-slope, fx, y);
        float step_a = st.a_gap_first / ((float)tap_span * tap_width);
        float floor_y = floor(yy);
        float y_end = slope + st.e2y;
        int b_cell_c = 0;
        int direct0 = 0;
        if (cond_any) {
            int o = c < 0 || c >= w || 0x0p+0f > floor_y || floor_y >= hs || tap_b < 0 || tap_b >= w || 0x0p+0f > y_end || y_end >= hs;
            b_cell_c = o;
            direct0 = !o;
        }
        int b_cell_g = 0;
        if (!(direct0 || mx0 != 0)) {
            b_cell_g = c < 0 || c >= w || tap_b < 0 || tap_b >= w;
        }
        int b_cell_h = 0;
        if (my == 0) {
            b_cell_h = 0x0p+0f > floor_y || floor_y >= hs || 0x0p+0f > y_end || y_end >= hs;
        }
        int n = bw_n_of(tap_span);
        if (n > 0) {
            if (g.spec && !b_cell_c) {
                /* body D, specialised loops S10..S13: no edge wrap inside the loop */
                float nf = (float)n;
                float wlim = nf * tap_width;
                float step_b = st.a_gap_second / (nf * st.one_minus_a_last);
                float omfx = 0x1p+0f - fx;
                float omc0 = 0x1p+0f - edge_gap;
                float hf = (float)h;
                float alpha = alpha_init;
                float step = step_a;
                int at_limit = 0;
                for (int k = 0; k < 1000; k++) {
                    if (k >= n) {
                        break;
                    }
                    if (!at_limit && (float)k >= wlim) {
                        int q = k / n;
                        alpha = fma(tap_width - (float)q, step_b, alpha_origin);
                        step = step_b;
                        at_limit = 1;
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
                    for (int chan = 0; chan < 3; chan++) {
                        float sample_a = bw_at(P, (long)idx + (long)chan);
                        float sample_b = bw_at(P, (long)idx + (long)stride + (long)chan);
                        sums[chan] = bw_acc(g, sums[chan], sample_a, sample_b, frac, wgt);
                    }
                    float sample_a = bw_at(P, (long)idx + 3L);
                    float sample_b = bw_at(P, (long)idx + (long)stride + 3L);
                    if (b_cell_h) {
                        sums[3] = bw_acc(g, sums[3], sample_a, sample_b, frac, wgt);
                    } else {
                        float b = fma(frac - 0x1p+0f, sample_a, -sample_b);
                        float d = fma(tf, sample_b, b);
                        float e = fma(sample_b, yy_old, -d);
                        sums[3] = e * wgt + sums[3];
                    }
                }
            } else {
                float hfi = (float)cvtt_i32_f(hs);
                float omfx = 0x1p+0f - fx;
                float omc0 = 0x1p+0f - edge_gap;
                float nf = (float)n;
                float wlim = nf * tap_width;
                float step_b = st.a_gap_second / (nf * st.one_minus_a_last);
                float twoh = fma(0x1p+1f, hfi, -0x1p+0f);
                int colmax = cvtt_i32_f(wf - 0x1p+0f);
                float hf = (float)h;
                float alpha = alpha_init;
                float step = step_a;
                int at_limit = 0;
                for (int k = 0; k < 1000; k++) {
                    if (k >= n) {
                        break;
                    }
                    if (!at_limit && (float)k >= wlim) {
                        int q = k / n;
                        alpha = fma(tap_width - (float)q, step_b, alpha_origin);
                        step = step_b;
                        at_limit = 1;
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
                    for (int chan = 0; chan < 4; chan++) {
                        if (chan >= M) {
                            break;
                        }
                        float sample_a = bw_at(P, (long)idx + (long)chan);
                        float sample_b = bw_at(P, (long)idx + (long)stride + (long)chan);
                        sums[chan] = bw_acc(g, sums[chan], sample_a, sample_b, frac, wgt);
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
        int tap_a = 1;
        if (e < 0) {
            tap_a = -1;
        }
        float mf = (float)m;
        float ystep = (ye - ya) / mf;
        float step_a = st.a_gap_first / (mf * tap_width);
        int count = bw_n_of(m);
        if (count > 0) {
            float cf = (float)count;
            float step_b = st.a_gap_second / (cf * st.one_minus_a_last);
            float wlim = cf * tap_width;
            float hsi = (float)cvtt_i32_f(hs);
            float twoh = fma(0x1p+1f, hsi, -0x1p+0f);
            int colmax = cvtt_i32_f(wf - 0x1p+0f);
            float alpha = alpha_init;
            float step = step_a;
            int at_limit = 0;
            for (int k = 0; k < 1000; k++) {
                if (k >= count) {
                    break;
                }
                int rown = cvtt_i32_f(ya);
                if (!at_limit && (float)k >= wlim) {
                    int q = k / count;
                    alpha = fma(tap_width - (float)q, step_b, alpha_origin);
                    step = step_b;
                    at_limit = 1;
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
                cs = BW_ADD(cs, tap_a);
                wsum = wsum + alpha;
                ya = ya + ystep;
                for (int chan = 0; chan < 4; chan++) {
                    if (chan >= M) {
                        break;
                    }
                    float v = bw_at(P, idx + (long)chan);
                    float prod = alpha * v;
                    sums[chan] = prod + sums[chan];
                }
                alpha = alpha + step;
                if (b_cell_i) {
                    if ((float)cs >= wf) {
                        if (H.pa != 0) {
                            tap_a = -1;
                            cs = colmax;
                        } else if (H.cb != 0) {
                            cs = 0;
                        }
                    } else if (cs < 0) {
                        if (H.pa != 0) {
                            cs = 0;
                            tap_a = 1;
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
    float scale = norm_num / wsum;
    for (int chan = 0; chan < 4; chan++) {
        res[chan] = sums[chan] * scale;
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
    float alpha_init = a.alpha_init;
    float alpha_origin = a.alpha_origin;
    float tap_width = a.tap_width;
    float norm_num = a.norm_num;
    int M = g.nc;
    float sums[4];
    for (int slot_no = 0; slot_no < 4; slot_no++) {
        sums[slot_no] = 0x0p+0f;
    }
    float wsum = 0x0p+0f;
    if (st.flag != 0u) {
        float x = st.x;
        float y = st.y;
        float delta_x = st.delta_x;
        float dy = st.dy;
        int tap_a;
        int tap_b;
        int tap_span;
        float fy;
        float edge_gap;
        float dys;
        int dir;
        if (dy > 0x0p+0f) {
            float fl = floor(y);
            fy = y - fl;
            tap_a = cvtt_i32_f(fl);
            float ce = ceil(st.e2y);
            edge_gap = ce - st.e2y;
            tap_b = cvtt_i32_f(ce);
            tap_span = BW_ADD(BW_SUB(tap_b, tap_a), 1);
            dir = 1;
            dys = dy;
        } else {
            float ce = ceil(y);
            tap_a = cvtt_i32_f(ce);
            fy = ce - y;
            float fl = floor(st.e2y);
            edge_gap = st.e2y - fl;
            tap_b = cvtt_i32_f(fl);
            tap_span = BW_ADD(BW_SUB(tap_a, tap_b), 1);
            dir = -1;
            dys = -dy;
        }
        float slope = delta_x / dys;
        float xx = fma(-slope, fy, x);
        float step_a = st.a_gap_first / ((float)tap_span * tap_width);
        float floor_5 = floor(xx);
        float sum_9 = slope + st.e2x;
        int b_cell_c = 0;
        int direct0 = 0;
        if (cond_any) {
            int o = 0x0p+0f > floor_5 || floor_5 >= ws || tap_a < 0 || tap_a >= h || 0x0p+0f > sum_9 || sum_9 >= ws || tap_b < 0 || tap_b >= h;
            b_cell_c = o;
            direct0 = !o;
        }
        int outside_x = 0;
        if (!(direct0 || mx0 != 0)) {
            outside_x = 0x0p+0f > floor_5 || floor_5 >= ws || 0x0p+0f > sum_9 || sum_9 >= ws;
        }
        int tap_c = 0;
        if (my == 0) {
            tap_c = tap_a < 0 || tap_a >= h || tap_b < 0 || tap_b >= h;
        }
        int n = bw_n_of(tap_span);
        if (n > 0) {
            float omfy = 0x1p+0f - fy;
            float omc0 = 0x1p+0f - edge_gap;
            float nf = (float)n;
            float wlim = nf * tap_width;
            float step_b = st.a_gap_second / (nf * st.one_minus_a_last);
            float twow = fma(0x1p+1f, ws, -0x1p+0f);
            float alpha = alpha_init;
            float step = step_a;
            int at_limit = 0;
            for (int k = 0; k < 1000; k++) {
                if (k >= n) {
                    break;
                }
                if (!at_limit && (float)k >= wlim) {
                    int q = k / n;
                    alpha = fma(tap_width - (float)q, step_b, alpha_origin);
                    step = step_b;
                    at_limit = 1;
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
                    if (tap_a >= h) {
                        if (H.ca != 0) {
                            tap_a = h - 1;
                            dir = -1;
                        } else if (H.cc != 0) {
                            tap_a = 0;
                        }
                    } else if (tap_a < 0) {
                        if (H.ca != 0) {
                            tap_a = 0;
                            dir = 1;
                        } else if (H.cc != 0) {
                            tap_a = h - 1;
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
                if (outside_x) {
                    if (-0x1p+0f > xx) {
                        xc = -0x1p+0f;
                    } else {
                        xc = bw_vmin(wf, xx);
                    }
                }
                int rr = tap_a;
                if (tap_c) {
                    if (tap_a >= -1) {
                        rr = tap_a < h ? tap_a : h;
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
                tap_a = BW_ADD(tap_a, dir);
                for (int chan = 0; chan < 4; chan++) {
                    if (chan >= M) {
                        break;
                    }
                    float sample_a = bw_at(P, idx - (long)M + (long)chan);
                    float sample_b = bw_at(P, idx + (long)chan);
                    sums[chan] = bw_acc(g, sums[chan], sample_a, sample_b, frac, wgt);
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
        int tap_c = cvtt_i32_f(ya);
        int e = BW_SUB(cvtt_i32_f(ye), tap_c);
        int m = bw_count_of(e);
        int dir = 1;
        if (e < 0) {
            dir = -1;
        }
        float mf = (float)m;
        float xstep = (xe - xa) / mf;
        float step_a = st.a_gap_first / (mf * tap_width);
        int count = bw_n_of(m);
        if (count > 0) {
            float cf = (float)count;
            float step_b = st.a_gap_second / (cf * st.one_minus_a_last);
            float wlim = cf * tap_width;
            float twow = fma(0x1p+1f, ws, -0x1p+0f);
            float alpha = alpha_init;
            float step = step_a;
            int at_limit = 0;
            float x = xa;
            for (int k = 0; k < 1000; k++) {
                if (k >= count) {
                    break;
                }
                if (!at_limit && (float)k >= wlim) {
                    int q = k / count;
                    alpha = fma(tap_width - (float)q, step_b, alpha_origin);
                    step = step_b;
                    at_limit = 1;
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
                int rowc = tap_c;
                if (b_cell_e) {
                    if (tap_c >= -1) {
                        rowc = tap_c < h ? tap_c : h;
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
                tap_c = BW_ADD(tap_c, dir);
                wsum = wsum + alpha;
                x = x + xstep;
                for (int chan = 0; chan < 4; chan++) {
                    if (chan >= M) {
                        break;
                    }
                    float v = bw_at(P, idx + (long)chan);
                    float prod = alpha * v;
                    sums[chan] = prod + sums[chan];
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
                    if (tap_c >= h) {
                        if (H.ca != 0) {
                            dir = -1;
                            tap_c = h - 1;
                        } else if (H.cc != 0) {
                            tap_c = 0;
                        }
                    } else if (tap_c < 0) {
                        if (H.ca != 0) {
                            tap_c = 0;
                            dir = 1;
                        } else if (H.cc != 0) {
                            tap_c = h - 1;
                        }
                    }
                }
            }
        }
    }
    float scale = norm_num / wsum;
    for (int chan = 0; chan < 4; chan++) {
        res[chan] = sums[chan] * scale;
    }
}

/* blur_warp_point::sample for 1 to 4 channels (the scalar element form), then the planes times norm_num. The CPU indexes the whole plane
   buffer by `i as usize` (a panic outside it, which is an error of the call): the same range test sets `bad`. */
void bw_p1(bw_cfg g, int mode_x, int mode_y, int width, int height, int stride, float x, float y, float norm_num, bw_plane* P, float* res) {
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
    float x_shifted = 0x1p+0f + xc;
    float sum_1 = 0x1p+0f + yc;
    int ix = cvtt_i32_f(x_shifted);
    int iy = cvtt_i32_f(sum_1);
    float fx = x_shifted - (float)ix;
    float fy = sum_1 - (float)iy;
    long offx = (long)BW_MUL(BW_SUB(ix, 1), n);
    long offy = (long)BW_MUL(BW_SUB(iy, 1), stride);
    long pos = P->origin + offx + offy;
    float fxy = fx * fy;
    float sum_00 = ((0x1p+0f - fx) - fy) + fxy;
    float diff_10 = fx - fxy;
    float diff_01 = fy - fxy;
    float frac_xy = fxy;
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
        float src_ch1 = frac_xy * vd;
        float p = fma(diff_10, vb, src_ch1);
        float src_ch2 = diff_01 * vc;
        float r = fma(sum_00, va, src_ch2);
        res[j] = p + r;
    }
    for (int j = 0; j < 4; j++) {
        if (j >= n) {
            break;
        }
        res[j] = norm_num * res[j];
    }
}

__kernel void ripmap_blur_warp_f32(__global const float* pl, __global const int* ip, __global const float* fp,
                                __global const float* p1x, __global const float* p1y, __global const float* p2x, __global const float* p2y,
                                __global float* dst_ch0, __global float* dst_ch1, __global float* dst_ch2, __global float* dst_ch3, __global uint* err) {
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
    int grid_w = ip[6];
    int grid_h = ip[7];
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
    int first_is_const = ip[15];
    int second_is_const = ip[16];
    int roi_x = ip[17];
    int roi_y = ip[18];
    bw_in a;
    float param_a_first = fp[0];
    float param_a_second = fp[1];
    float param_a_third = fp[2];
    float param_a_fourth = fp[3];
    float param_a_fifth = fp[4];
    a.alpha_init = param_a_first;
    a.alpha_origin = param_a_second;
    a.tap_width = param_a_fourth;
    a.norm_num = param_a_fifth;
    float aspect = fp[7];
    float geo_e = fp[8];
    float left = fp[9];
    float right = fp[10];
    float geo_h = fp[11];
    float l_p1x = fp[12];
    float l_p1y = fp[13];
    float l_p2x = fp[14];
    float l_p2y = fp[15];
    if (!first_is_const) {
        l_p1x = p1x[gid];
        l_p1y = p1y[gid];
    }
    if (!second_is_const) {
        l_p2x = p2x[gid];
        l_p2y = p2y[gid];
    }
    float one_minus_a_last = 0x1p+0f - param_a_fourth;
    float a_gap_first = param_a_second - param_a_first;
    float a_gap_second = param_a_third - param_a_second;
    float prod_12 = 0x1p-1f * aspect;
    float diff_13 = right - left;
    float scx = 0x1p-1f * diff_13;
    float scy = prod_12 * diff_13;
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
        bw_p1(g, mode_x, mode_y, grid_w, grid_h, stride, x, y, param_a_fifth, &P, res);
    } else {
        bw_s5 st;
        st.x = x;
        st.y = y;
        st.e2x = e2x;
        st.e2y = e2y;
        st.delta_x = e2x - x;
        st.dy = e2y - y;
        st.w = img_w;
        st.h = img_h;
        st.stride = stride;
        st.a_gap_first = a_gap_first;
        st.a_gap_second = a_gap_second;
        st.one_minus_a_last = one_minus_a_last;
        st.flag = flag;
        st.mx = mode_x;
        st.my = mode_y;
        float adx = fabs(st.delta_x);
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
    dst_ch0[gid] = res[0];
    if (body >= 2) {
        dst_ch1[gid] = res[1];
    }
    if (body >= 3) {
        dst_ch2[gid] = res[2];
    }
    if (body >= 4) {
        dst_ch3[gid] = res[3];
    }
}
"#;
