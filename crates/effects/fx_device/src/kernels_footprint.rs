//! Device kernel source of the warp footprint prefill: the per pixel log2 of gradient map that `warp_log_gradient::prefill::run_body`
//! computes for each coordinate plane (warp slot 33), plus a probe kernel that exposes its single precision helpers.
//!
//! Every single precision value is carried as its `uint` bits and every operation follows the SSE NaN rule of the CPU port through
//! explicit tests (first NaN operand in instruction order, quieted; a NaN produced from non NaN operands is 0xffc00000), so the
//! device's own NaN propagation never decides a word. The combine step of `finish` runs in double precision with the CPU's fused
//! forms, then the exponent plus mantissa bit trick gives the pseudo log2.
//!
//! * `fp_prefill_u32`: one work item per pixel. Each pixel reads its own row and the rows above and below (clamped at the frame
//!   edge), so no reduction and no order dependence; the x == 0, middle and x == w - 1 forms are the CPU's three loop parts.
//! * `fp33_ops_u32`: the helpers `fp33_add`, `fp33_sub`, `fp33_mul`, `fp33_min`, `fp33_fma213` (both signs) and `fp33_fmsub231` on
//!   operand triples, compared bit for bit (NaN payloads included) with the public helpers of the CPU port.

pub const FOOTPRINT_PREFILL: &str = r#"
/* MIRRORS_MUL_ADD: warp_log_gradient::prefill::fma213, warp_log_gradient::prefill::fmsub231, warp_log_gradient::prefill::finish */
int fp33_isn(uint b) {
    return (b & 0x7fffffffu) > 0x7f800000u;
}
int fp33_isn64(ulong b) {
    return (b & 0x7ffffffffffffffful) > 0x7ff0000000000000ul;
}
/* A NaN produced from non NaN operands becomes the default NaN. */
uint fp33_fix(float r) {
    return isnan(r) ? 0xffc00000u : as_uint(r);
}
ulong fp33_fix64(double r) {
    return isnan(r) ? 0xfff8000000000000ul : as_ulong(r);
}
uint fp33_add(uint a, uint b) {
    if (fp33_isn(a)) {
        return a | 0x400000u;
    }
    if (fp33_isn(b)) {
        return b | 0x400000u;
    }
    return fp33_fix(as_float(a) + as_float(b));
}
uint fp33_sub(uint a, uint b) {
    if (fp33_isn(a)) {
        return a | 0x400000u;
    }
    if (fp33_isn(b)) {
        return b | 0x400000u;
    }
    return fp33_fix(as_float(a) - as_float(b));
}
uint fp33_mul(uint a, uint b) {
    if (fp33_isn(a)) {
        return a | 0x400000u;
    }
    if (fp33_isn(b)) {
        return b | 0x400000u;
    }
    return fp33_fix(as_float(a) * as_float(b));
}
/* vminss: a < b ? a : b (NaN or equal gives b). */
uint fp33_min(uint a, uint b) {
    return (as_float(a) < as_float(b)) ? a : b;
}
/* vfmadd213ss / vfmsub213ss dst, s2, s3: s2 * dst (+|-) s3, one rounding; NaN priority dst, s2, s3. */
uint fp33_fma213(uint dst, uint s2, uint s3, int sub) {
    if (fp33_isn(dst)) {
        return dst | 0x400000u;
    }
    if (fp33_isn(s2)) {
        return s2 | 0x400000u;
    }
    if (fp33_isn(s3)) {
        return s3 | 0x400000u;
    }
    float c = sub ? -as_float(s3) : as_float(s3);
    return fp33_fix(fma(as_float(s2), as_float(dst), c));
}
/* vfmsub231ss dst, s2, s3: s2 * s3 - dst, one rounding; NaN priority dst, s2, s3. */
uint fp33_fmsub231(uint dst, uint s2, uint s3) {
    if (fp33_isn(dst)) {
        return dst | 0x400000u;
    }
    if (fp33_isn(s2)) {
        return s2 | 0x400000u;
    }
    if (fp33_isn(s3)) {
        return s3 | 0x400000u;
    }
    return fp33_fix(fma(as_float(s2), as_float(s3), -as_float(dst)));
}
/* cvtss2sd: a NaN keeps its sign and payload (shifted into the double fraction) and is quieted. */
ulong fp33_cvtss2sd(uint b) {
    if (fp33_isn(b)) {
        return (((ulong)(b & 0x80000000u)) << 32) | 0x7ff8000000000000ul | (((ulong)(b & 0x7fffffu)) << 29);
    }
    return as_ulong((double)as_float(b));
}
/* cvtsd2ss: a NaN keeps its sign and the top of its payload and is quieted; otherwise round to nearest even. */
uint fp33_cvtsd2ss(ulong b) {
    if (fp33_isn64(b)) {
        return (((uint)(b >> 63)) << 31) | 0x7fc00000u | (((uint)(b >> 29)) & 0x3fffffu);
    }
    return fp33_fix((float)as_double(b));
}
/* vfmadd231sd dst, s2, s3 = s2 * s3 + dst; NaN priority dst, s2, s3. */
ulong fp33_fma231_sd(ulong dst, ulong s2, ulong s3) {
    if (fp33_isn64(dst)) {
        return dst | 0x8000000000000ul;
    }
    if (fp33_isn64(s2)) {
        return s2 | 0x8000000000000ul;
    }
    if (fp33_isn64(s3)) {
        return s3 | 0x8000000000000ul;
    }
    return fp33_fix64(fma(as_double(s2), as_double(s3), as_double(dst)));
}
/* vfmadd213sd dst, s2, s3 = s2 * dst + s3; NaN priority dst, s2, s3. */
ulong fp33_fma213_sd(ulong dst, ulong s2, ulong s3) {
    if (fp33_isn64(dst)) {
        return dst | 0x8000000000000ul;
    }
    if (fp33_isn64(s2)) {
        return s2 | 0x8000000000000ul;
    }
    if (fp33_isn64(s3)) {
        return s3 | 0x8000000000000ul;
    }
    return fp33_fix64(fma(as_double(s2), as_double(dst), as_double(s3)));
}
/* Compare xa with d in double, combine as max + 0.5 * min with one rounding, round to single, then exponent plus mantissa. */
uint fp33_finish(ulong xa, ulong d) {
    ulong r;
    if (as_double(xa) > as_double(d)) {
        r = fp33_fma231_sd(xa, d, 0x3fe0000000000000ul);
    } else {
        r = fp33_fma213_sd(xa, 0x3fe0000000000000ul, d);
    }
    uint m32 = fp33_cvtsd2ss(r);
    if (!(as_float(m32) > 0x0p+0f)) {
        return 0u;
    }
    int e = (int)((m32 >> 23) & 0xffu) - 128;
    uint mant = (m32 & 0x807fffffu) + 0x3f800000u;
    return fp33_add(fp33_fix((float)e), mant);
}

/* One pixel of run_body per work item: src and dst are w * h planes with row stride w (w >= 2, h >= 1, checked by the host).
   flag != 0 selects the min variant, else the sum variant. v, pre_a and pre_b are single precision bits. */
__kernel void fp_prefill_u32(__global const uint* src, __global uint* dst, int w, int h, uint v, uint pre_a, uint pre_b, int flag) {
    size_t gid = get_global_id(0);
    if (gid >= (size_t)w * (size_t)h) {
        return;
    }
    int y = (int)(gid / (size_t)w);
    int x = (int)(gid - (size_t)y * (size_t)w);
    __global const uint* cur = src + (size_t)y * (size_t)w;
    __global const uint* prev = (y == 0) ? cur : cur - w;
    __global const uint* next = (y == h - 1) ? cur : cur + w;
    uint hb0 = fp33_mul(0x3f000000u, pre_a);
    uint hb8 = fp33_mul(0x3f000000u, pre_b);
    uint two_v = fp33_mul(0x40000000u, v);
    uint s = (y == 0 || y == h - 1) ? 0x40000000u : 0x3f800000u;
    uint sy = fp33_mul(v, s);
    ulong xa;
    ulong d;
    if (x == 0) {
        uint c0 = cur[0];
        uint c1 = cur[1];
        uint a = fp33_sub(c0, prev[0]);
        uint d1 = fp33_sub(c1, c0);
        uint up = fp33_fmsub231(hb8, a, sy);
        uint vx0 = fp33_fma213(two_v, d1, pre_a, 0);
        uint dn0 = fp33_sub(next[0], c0);
        uint down = fp33_fmsub231(hb8, dn0, sy);
        xa = fp33_cvtss2sd(vx0 & 0x7fffffffu);
        uint ua = up & 0x7fffffffu;
        uint da = down & 0x7fffffffu;
        if (flag != 0) {
            uint mn = fp33_min(ua, da);
            d = fp33_cvtss2sd(fp33_add(mn, mn));
        } else {
            d = fp33_cvtss2sd(fp33_add(da, ua));
        }
    } else if (x < w - 1) {
        uint cx = cur[x];
        uint cp = cur[x - 1];
        uint cn = cur[x + 1];
        uint dl = fp33_sub(cx, cp);
        uint dr = fp33_sub(cn, cx);
        uint dn = fp33_sub(next[x], cx);
        uint dp = fp33_sub(cx, prev[x]);
        uint lv = fp33_fma213(dl, v, hb0, 0) & 0x7fffffffu;
        uint rv = fp33_fma213(v, dr, hb0, 0) & 0x7fffffffu;
        uint nv = fp33_fma213(dn, sy, hb8, 1) & 0x7fffffffu;
        uint pv = fp33_fma213(dp, sy, hb8, 1) & 0x7fffffffu;
        if (flag != 0) {
            uint mx = fp33_min(lv, rv);
            uint my = fp33_min(pv, nv);
            xa = fp33_cvtss2sd(fp33_add(mx, mx));
            d = fp33_cvtss2sd(fp33_add(my, my));
        } else {
            xa = fp33_cvtss2sd(fp33_add(rv, lv));
            d = fp33_cvtss2sd(fp33_add(nv, pv));
        }
    } else {
        uint cl = cur[w - 1];
        uint dr_last = fp33_sub(cl, cur[w - 2]);
        uint vxl = fp33_fma213(dr_last, two_v, pre_a, 0) & 0x7fffffffu;
        xa = fp33_cvtss2sd(vxl);
        uint dn = fp33_sub(next[w - 1], cl);
        uint dp = fp33_sub(cl, prev[w - 1]);
        uint pv = fp33_fma213(dp, sy, hb8, 1) & 0x7fffffffu;
        uint nv = fp33_fma213(sy, dn, hb8, 1) & 0x7fffffffu;
        if (flag != 0) {
            uint mn = fp33_min(pv, nv);
            d = fp33_cvtss2sd(fp33_add(mn, mn));
        } else {
            d = fp33_cvtss2sd(fp33_add(nv, pv));
        }
    }
    dst[gid] = fp33_finish(xa, d);
}

/* The single precision helpers on n operand triples (a, b, c): seven words per triple. */
__kernel void fp33_ops_u32(__global const uint* a, __global const uint* b, __global const uint* c, __global uint* out, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    uint x = a[i];
    uint y = b[i];
    uint z = c[i];
    __global uint* o = out + i * 7;
    o[0] = fp33_add(x, y);
    o[1] = fp33_sub(x, y);
    o[2] = fp33_mul(x, y);
    o[3] = fp33_min(x, y);
    o[4] = fp33_fma213(x, y, z, 0);
    o[5] = fp33_fma213(x, y, z, 1);
    o[6] = fp33_fmsub231(x, y, z);
}
"#;

/// The footprint branch of the warp drivers (`warp_driver::run` for `warp`, `warp_rel_sampler::run` for `warp_rel`) with the F1
/// lattice sampler (`warp_sampler::sample`) over every ripmap plane. Kernel `fp_warp_f32`: one work item per pixel; the coordinate
/// and the wrap are those of `warp_warp_f32` (`warp_wrap` from `WARP_WARP`), then the pixel takes the footprint branch when c1 or c2
/// (the prefill maps) is positive or NaN, else the normal path on plane (0, 0). Every table index and every tap is range checked
/// against the CPU's own bounds (the scale table, the level tables and each plane's length); a miss sets the error word and the
/// host takes the node to the CPU, which reports its error.
pub const FOOTPRINT_WARP: &str = r#"
/* MIRRORS_MUL_ADD: warp_sampler::sample, warp_sampler::nest, warp_driver::x_raw, warp_driver::y_raw, warp_driver::blend1, warp_rel_sampler::x_raw, warp_rel_sampler::y_raw, warp_rel_sampler::blend1 */

/* clamp_dir: (float) vminsd((double) dim - 1.01, (double) c), the first operand when it is smaller, else the second. */
float fpw_clamp_dir(float c, int dim) {
    double d = (double)dim - as_double(0x3ff028f5c28f5c29ul);
    double cd = (double)c;
    double m = (d < cd) ? d : cd;
    return (float)m;
}

/* clamp_pair: -1 > raw ? -1 : minss(w, raw). */
float fpw_clamp_pair(float raw, int w) {
    if (-0x1p+0f > raw) {
        return -0x1p+0f;
    }
    float wf = (float)w;
    return (wf < raw) ? wf : raw;
}

/* (a - 1) * b with 32 bit wrapping. */
int fpw_m1mul(int a, int b) {
    return (int)(((uint)a - 1u) * (uint)b);
}

/* One bilinear nest of n channels from the plane at lat + off (pl floats), S form; 0 when a tap is outside the plane. */
int fpw_nest(__global const float* lat, uint off, uint pl, long start, int n, int st, float fn, float fs, int first_is_stride, float* t) {
    float tt = fn * fs;
    float w0;
    if (first_is_stride) {
        w0 = ((0x1p+0f - fs) - fn) + tt;
    } else {
        w0 = ((0x1p+0f - fn) - fs) + tt;
    }
    float w1 = fn - tt;
    float w2 = fs - tt;
    float w3 = tt;
    long lim = (long)pl;
    for (int e = 0; e < 4; e++) {
        if (e < n) {
            long o = start + (long)e;
            long k1 = o + (long)n;
            long k2 = o + (long)st;
            long k3 = o + (long)n + (long)st;
            if (o < 0 || o >= lim || k1 < 0 || k1 >= lim || k2 < 0 || k2 >= lim || k3 < 0 || k3 >= lim) {
                return 0;
            }
            float s0 = lat[(ulong)off + (ulong)o];
            float s1 = lat[(ulong)off + (ulong)k1];
            float s2 = lat[(ulong)off + (ulong)k2];
            float s3 = lat[(ulong)off + (ulong)k3];
            float p5 = fma(w1, s1, w3 * s3);
            float p6 = fma(w0, s0, w2 * s2);
            t[e] = p5 + p6;
        }
    }
    return 1;
}

void fpw_store(__global float* o0, __global float* o1, __global float* o2, __global float* o3, uint gid, int k, float v) {
    if (k == 0) {
        o0[gid] = v;
    } else if (k == 1) {
        o1[gid] = v;
    } else if (k == 2) {
        o2[gid] = v;
    } else {
        o3[gid] = v;
    }
}

/* warp_sampler::sample for one pixel: returns 0 when an index or a tap is outside the tables or planes. */
int fpw_sample(__global const float* lat, __global const uint* offs, __global const uint* plen, __global const int* wtab,
               __global const int* htab, __global const uint* ttab, int nl, int np, int tlen, int n, int fx, int fy,
               float x, float y, float a, float b, int i, int j, uint gid,
               __global float* o0, __global float* o1, __global float* o2, __global float* o3) {
    long il = (long)i;
    long jl = (long)j;
    if (il < 0 || il + 1 >= (long)tlen || jl < 0 || jl + 1 >= (long)tlen || il + 1 >= (long)nl || jl + 1 >= (long)np) {
        return 0;
    }
    float sx = 0x1p-1f + x;
    float t6 = fma(sx, as_float(ttab[il]), -0x1p-1f);
    float u = fma(sx, as_float(ttab[il + 1]), -0x1p-1f);
    if (fx == 0) {
        t6 = fpw_clamp_pair(t6, wtab[il]);
        u = fpw_clamp_pair(u, wtab[il + 1]);
    }
    float sy = 0x1p-1f + y;
    float t7 = fma(sy, as_float(ttab[jl]), -0x1p-1f);
    float v = fma(sy, as_float(ttab[jl + 1]), -0x1p-1f);
    if (fy == 0) {
        t7 = fpw_clamp_pair(t7, htab[jl]);
        v = fpw_clamp_pair(v, htab[jl + 1]);
    }
    float s3 = 0x1p+0f + t6;
    float s6 = 0x1p+0f + t7;
    float s2 = 0x1p+0f + u;
    float s1 = 0x1p+0f + v;
    int p0 = cvtt_i32_f(s3);
    int p1 = cvtt_i32_f(s6);
    int p2 = cvtt_i32_f(s2);
    int p3 = cvtt_i32_f(s1);
    float fxa = s3 - (float)p0;
    float fya = s6 - (float)p1;
    float fxb = s2 - (float)p2;
    float fyb = s1 - (float)p3;
    /* B: plane (i, j) */
    int st1 = (int)(((uint)wtab[il] + 3u) * (uint)n);
    long base1 = (long)(int)((uint)n + (uint)st1);
    long start_b = base1 + (long)fpw_m1mul(p0, n) + (long)fpw_m1mul(p1, st1);
    float t1[4];
    float t2[4];
    float t3[4];
    uint pb = (uint)(il * (long)np + jl);
    if (!fpw_nest(lat, offs[pb], plen[pb], start_b, n, st1, fxa, fya, 0, t1)) {
        return 0;
    }
    /* C: plane (i + 1, j + 1) */
    int st2 = (int)(((uint)wtab[il + 1] + 3u) * (uint)n);
    long base2 = (long)(int)((uint)n + (uint)st2);
    int c_n_off = fpw_m1mul(p2, n);
    long start_c = base2 + (long)c_n_off + (long)fpw_m1mul(p3, st2);
    uint pc = (uint)((il + 1) * (long)np + jl + 1);
    if (!fpw_nest(lat, offs[pc], plen[pc], start_c, n, st2, fxb, fyb, 0, t2)) {
        return 0;
    }
    if (a > b) {
        /* F: plane (i + 1, j) */
        long start_f = base2 + (long)c_n_off + (long)fpw_m1mul(p1, st2);
        uint pf = (uint)((il + 1) * (long)np + jl);
        if (!fpw_nest(lat, offs[pf], plen[pf], start_f, n, st2, fxb, fya, 1, t3)) {
            return 0;
        }
        float w1 = 0x1p+0f - a;
        float w0 = a - b;
        for (int k = 0; k < 4; k++) {
            if (k < n) {
                float r = b * t2[k];
                r = fma(w1, t1[k], r);
                r = fma(w0, t3[k], r);
                fpw_store(o0, o1, o2, o3, gid, k, r);
            }
        }
    } else if (b > a) {
        /* E: plane (i, j + 1) */
        long start_e = base1 + (long)fpw_m1mul(p0, n) + (long)fpw_m1mul(p3, st1);
        uint pe = (uint)(il * (long)np + jl + 1);
        if (!fpw_nest(lat, offs[pe], plen[pe], start_e, n, st1, fxa, fyb, 0, t3)) {
            return 0;
        }
        float w1 = 0x1p+0f - b;
        float w0 = b - a;
        for (int k = 0; k < 4; k++) {
            if (k < n) {
                float r = a * t2[k];
                r = fma(w1, t1[k], r);
                r = fma(w0, t3[k], r);
                fpw_store(o0, o1, o2, o3, gid, k, r);
            }
        }
    } else {
        /* D: lerp of the first two nests by a */
        for (int k = 0; k < 4; k++) {
            if (k < n) {
                fpw_store(o0, o1, o2, o3, gid, k, fma(t2[k] - t1[k], a, t1[k]));
            }
        }
    }
    return 1;
}

__kernel void fp_warp_f32(__global const float* lat, __global const uint* offs, __global const uint* plen,
                          __global const int* wtab, __global const int* htab, __global const uint* ttab, int nl, int np, int tlen,
                          __global const float* a1, __global const float* a2, __global const float* c1, __global const float* c2,
                          __global float* o0, __global float* o1, __global float* o2, __global float* o3,
                          int n, int w, int h, int mx, int my,
                          float aspect, float left, float right, float geo_e, float geo_h, int roi_x, int roi_y,
                          int rel, __global uint* err) {
    uint gid = (uint)get_global_id(0);
    uint total = (uint)w * (uint)h;
    if (gid >= total) {
        return;
    }
    int y = (int)(gid / (uint)w);
    int xi = (int)(gid % (uint)w);
    float x0 = a1[gid];
    float y0 = a2[gid];
    float x13 = right - left;
    float scx = 0x1p-1f * x13;
    float scy = (0x1p-1f * aspect) * x13;
    float xr;
    float yr;
    if (rel) {
        int r8 = (int)(((uint)(xi + 1) + (uint)roi_x) - 1u);
        float base = (float)r8 - (float)roi_x;
        xr = fma(x0, scx, base);
        float cy = (float)roi_y;
        float dy = (float)((int)((uint)y + (uint)roi_y));
        float t = fma(-scy, y0, -cy);
        yr = t + dy;
    } else {
        float t = x0 + 0x1p+0f;
        float base = left - (float)roi_x;
        xr = fma(t, scx, base);
        float t2 = y0 - 0x1p+0f;
        float m = fma(scy, t2, -geo_h);
        float top = geo_e - (float)roi_y;
        yr = top - m;
    }
    float x = warp_wrap(xr, w, mx);
    float y1 = warp_wrap(yr, h, my);
    /* footprint branch */
    float cv1 = c1[gid];
    float cv2 = c2[gid];
    int c1pos = (0x0p+0f < cv1) || isnan(cv1);
    int c2pos = (0x0p+0f < cv2) || isnan(cv2);
    if (c1pos || c2pos) {
        float p;
        if (c1pos) {
            p = fpw_clamp_dir(cv1, nl);
        } else if (0x0p+0f > cv1) {
            p = 0x0p+0f;
        } else {
            p = fpw_clamp_dir(cv1, nl);
        }
        float q;
        if ((0x0p+0f <= cv2) || isnan(cv2)) {
            q = fpw_clamp_dir(cv2, np);
        } else {
            q = 0x0p+0f;
        }
        int ip = cvtt_i32_f(p);
        int jq = cvtt_i32_f(q);
        float fa = p - (float)ip;
        float fb = q - (float)jq;
        if (!fpw_sample(lat, offs, plen, wtab, htab, ttab, nl, np, tlen, n, mx, my, x, y1, fa, fb, ip, jq, gid, o0, o1, o2, o3)) {
            atomic_or(err, 1u);
        }
        return;
    }
    /* normal path: bilinear tap on plane (0, 0) */
    int wt0 = wtab[0];
    int ht0 = htab[0];
    float xc = x;
    if (mx == 0) {
        if (-0x1p+0f > x) {
            xc = -0x1p+0f;
        } else {
            float wb = (float)wt0;
            xc = (wb < x) ? wb : x;
        }
    }
    float yc = y1;
    if (my == 0) {
        if (-0x1p+0f > y1) {
            yc = -0x1p+0f;
        } else {
            float hb = (float)ht0;
            yc = (hb < y1) ? hb : y1;
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
    long off0 = (long)n + (long)((int)((uint)n * (uint)wp3));
    int ecx = fpw_m1mul(ix, n);
    int r14 = fpw_m1mul(iy, stride);
    long base0 = off0 + (long)ecx + (long)r14;
    float t = fx * fy;
    float w0 = ((0x1p+0f - fx) - fy) + t;
    float wn = fx - t;
    float ws = fy - t;
    float wd = t;
    long lim = (long)plen[0];
    uint off = offs[0];
    for (int j = 0; j < 4; j++) {
        if (j < n) {
            long k0 = base0 + (long)j;
            long k1 = base0 + (long)n + (long)j;
            long k2 = base0 + (long)stride + (long)j;
            long k3 = base0 + (long)n + (long)stride + (long)j;
            if (k0 < 0 || k0 >= lim || k1 < 0 || k1 >= lim || k2 < 0 || k2 >= lim || k3 < 0 || k3 >= lim) {
                atomic_or(err, 1u);
                return;
            }
            float r0 = lat[(ulong)off + (ulong)k0];
            float rn = lat[(ulong)off + (ulong)k1];
            float rs = lat[(ulong)off + (ulong)k2];
            float rd = lat[(ulong)off + (ulong)k3];
            float s = wd * rd;
            float u = ws * rs;
            s = fma(wn, rn, s);
            u = fma(w0, r0, u);
            fpw_store(o0, o1, o2, o3, gid, j, s + u);
        }
    }
}
"#;
