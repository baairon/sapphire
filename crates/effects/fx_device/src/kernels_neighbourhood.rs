//! Device kernel sources of the neighbourhood ops: blur (row and column running window sums), masked_blur, grad, sharpen,
//! edge_aware_sharpen and static2. OpenCL C 1.2 as Rust string constants; registered in [`crate::kernels::SOURCES`].
//!
//! Exactness rules (see the crate docs): `FP_CONTRACT OFF` (prelude), `fma()` only where the CPU calls `mul_add`
//! (those sources carry the marker comment and the registry flag), x86 operand order for min and max, hexadecimal literals,
//! loops with constant bounds (variable trip counts use a large constant bound and a `break`).
//!
//! The box blur sums stay serial: one work item per row (horizontal pass) or per column (vertical pass), each running the
//! CPU's window routine in the CPU's operation order (`blur_rows::box_row`, `blur_rows_y::column_window`, which agree
//! statement by statement once the element stride is a parameter). The head, middle and tail of the sliding sum, the
//! eight-lane initial window sums (`s0` / `prefix_sum`, with the scratch alignment `phase`) and the subpixel blend are
//! transcribed one to one. The driver arithmetic (radii, coefficients, final scale) runs on the host with the CPU functions.

/// Blur: window routine, horizontal pass, vertical pass, scale.
pub const NEIGHBOURHOOD_BLUR: &str = r#"
/* MIRRORS_MUL_ADD: blur_rows::box_row (vfmsub213ss at out[0], blend), blur_rows_y::column_window, blend */
#define BW_AT(p, i) (p)[(size_t)(i) * st]

/* Binary operations with the x86 two-NaN rule (the CPU's fx_common::nanop): a NaN result is replaced by the first NaN operand in
   instruction order, quieted, else by the default NaN 0xffc00000; any other result is the plain IEEE result. */
float nf_pick(float a, float b) {
    if (a != a) {
        return as_float(as_uint(a) | 0x400000u);
    }
    if (b != b) {
        return as_float(as_uint(b) | 0x400000u);
    }
    return as_float(0xffc00000u);
}
float nf_pick3(float a, float b, float c) {
    if (a != a) {
        return as_float(as_uint(a) | 0x400000u);
    }
    return nf_pick(b, c);
}
float nf_add(float a, float b) {
    float r = a + b;
    return (r == r) ? r : nf_pick(a, b);
}
float nf_sub(float a, float b) {
    float r = a - b;
    return (r == r) ? r : nf_pick(a, b);
}
float nf_mul(float a, float b) {
    float r = a * b;
    return (r == r) ? r : nf_pick(a, b);
}
/* vfmsub213ss d, s2, s3: d * s2 - s3, one rounding; NaN priority d, s2, s3. */
float nf_fms213(float d, float s2, float s3) {
    float r = fma(d, s2, -s3);
    return (r == r) ? r : nf_pick3(d, s2, s3);
}
/* vfmadd231ss d, s2, s3: s2 * s3 + d, one rounding; NaN priority d, s2, s3. */
float nf_fma231(float d, float s2, float s3) {
    float r = fma(s2, s3, d);
    return (r == r) ? r : nf_pick3(d, s2, s3);
}

float bw_hred(float* v) {
    float t0 = nf_add(v[0], v[4]);
    float t1 = nf_add(v[1], v[5]);
    float t2 = nf_add(v[2], v[6]);
    float t3 = nf_add(v[3], v[7]);
    float u0 = nf_add(t0, t2);
    float u1 = nf_add(t1, t3);
    return nf_add(u0, u1);
}

/* Initial window sum of x[0..len): the CPU's s0 / prefix_sum, grouping included. */
float bw_prefix(__global const float* x, size_t st, int len, int edge, uint phase) {
    int ph = (int)(phase & 31u);
    int peel = 0;
    int vec_ok = (len >= 64) ? 1 : 0;
    if (vec_ok != 0) {
        if (ph != 0) {
            if ((ph & 3) != 0) {
                vec_ok = 0;
            } else {
                peel = (32 - ph) / 4;
            }
        }
    }
    if (vec_ok != 0 && len < peel + 64) {
        vec_ok = 0;
    }
    float acc = 0x0p+0f;
    if (vec_ok == 0) {
        for (int i = 0; i < 2147483647; i++) {
            if (i >= len) {
                break;
            }
            acc = nf_add(acc, BW_AT(x, i));
        }
        return nf_add(0x0p+0f, acc);
    }
    for (int i = 0; i < 8; i++) {
        if (i >= peel) {
            break;
        }
        acc = nf_add(acc, BW_AT(x, i));
    }
    int nb = (len - peel) / 64;
    int end = peel + 64 * nb;
    int rem = len - end;
    int n8 = rem / 8;
    float total;
    if (edge == 1) {
        float pp[8];
        float qq[8];
        for (int l = 0; l < 8; l++) {
            pp[l] = 0x0p+0f;
            qq[l] = 0x0p+0f;
        }
        for (int k = 0; k < 2147483647; k++) {
            if (k >= nb) {
                break;
            }
            int b = peel + 64 * k;
            for (int l = 0; l < 8; l++) {
                float p = pp[l];
                p = nf_add(p, BW_AT(x, b + l));
                p = nf_add(p, BW_AT(x, b + 16 + l));
                p = nf_add(p, BW_AT(x, b + 32 + l));
                p = nf_add(p, BW_AT(x, b + 48 + l));
                pp[l] = p;
                float q = qq[l];
                q = nf_add(q, BW_AT(x, b + 8 + l));
                q = nf_add(q, BW_AT(x, b + 24 + l));
                q = nf_add(q, BW_AT(x, b + 40 + l));
                q = nf_add(q, BW_AT(x, b + 56 + l));
                qq[l] = q;
            }
        }
        for (int k = 0; k < 8; k++) {
            if (k >= n8) {
                break;
            }
            for (int l = 0; l < 8; l++) {
                pp[l] = nf_add(pp[l], BW_AT(x, end + 8 * k + l));
            }
        }
        float v[8];
        for (int l = 0; l < 8; l++) {
            v[l] = nf_add(pp[l], qq[l]);
        }
        total = bw_hred(v);
    } else {
        float a[64];
        for (int l = 0; l < 64; l++) {
            a[l] = 0x0p+0f;
        }
        for (int k = 0; k < 2147483647; k++) {
            if (k >= nb) {
                break;
            }
            int b = peel + 64 * k;
            for (int j = 0; j < 8; j++) {
                for (int l = 0; l < 8; l++) {
                    a[j * 8 + l] = nf_add(a[j * 8 + l], BW_AT(x, b + 8 * j + l));
                }
            }
        }
        float v[8];
        for (int l = 0; l < 8; l++) {
            float c01 = nf_add(a[l], a[8 + l]);
            float c23 = nf_add(a[16 + l], a[24 + l]);
            float c45 = nf_add(a[32 + l], a[40 + l]);
            float c67 = nf_add(a[48 + l], a[56 + l]);
            float lh = nf_add(c01, c23);
            float rh = nf_add(c45, c67);
            v[l] = nf_add(lh, rh);
        }
        for (int k = 0; k < 8; k++) {
            if (k >= n8) {
                break;
            }
            for (int l = 0; l < 8; l++) {
                v[l] = nf_add(v[l], BW_AT(x, end + 8 * k + l));
            }
        }
        total = bw_hred(v);
    }
    for (int i = 0; i < 8; i++) {
        int ii = end + 8 * n8 + i;
        if (ii >= len) {
            break;
        }
        acc = nf_add(acc, BW_AT(x, ii));
    }
    return nf_add(total, acc);
}

/* out[i] = sum of x over [i-r, i+r] with the edge rule: sliding sum, head, middle, tail. */
void bw_window(__global const float* x, __global float* out, size_t st, int n, int r, int edge, uint phase) {
    int mirror = (edge == 1) ? 1 : 0;
    float s = bw_prefix(x, st, r + 1, edge, phase);
    float y = s;
    if (mirror != 0) {
        y = nf_fms213(0x1p+1f, s, BW_AT(x, r));
    }
    int hp = r / 2;
    for (int j = 0; j < 1073741824; j++) {
        if (j >= hp) {
            break;
        }
        int i = 2 * j;
        BW_AT(out, i) = y;
        if (mirror != 0) {
            y = nf_add(nf_sub(BW_AT(x, r + 1 + i), BW_AT(x, r - 1 - i)), y);
            BW_AT(out, i + 1) = y;
            y = nf_sub(nf_add(y, BW_AT(x, r + 2 + i)), BW_AT(x, r - 2 - i));
        } else {
            y = nf_add(y, BW_AT(x, r + 1 + i));
            BW_AT(out, i + 1) = y;
            y = nf_add(y, BW_AT(x, r + 2 + i));
        }
    }
    if ((r % 2) == 1) {
        int i = r - 1;
        BW_AT(out, i) = y;
        if (mirror != 0) {
            y = nf_sub(nf_add(y, BW_AT(x, r + 1 + i)), BW_AT(x, r - 1 - i));
        } else {
            y = nf_add(y, BW_AT(x, r + 1 + i));
        }
    }
    int m = n - 2 * r - 1;
    int mp = m / 2;
    for (int j = 0; j < 1073741824; j++) {
        if (j >= mp) {
            break;
        }
        int i = r + 2 * j;
        BW_AT(out, i) = y;
        y = nf_add(nf_sub(BW_AT(x, i + r + 1), BW_AT(x, i - r)), y);
        BW_AT(out, i + 1) = y;
        y = nf_add(nf_sub(y, BW_AT(x, i + 1 - r)), BW_AT(x, i + r + 2));
    }
    if ((m % 2) == 1) {
        int i = r + 2 * mp;
        BW_AT(out, i) = y;
        y = nf_add(nf_sub(y, BW_AT(x, i - r)), BW_AT(x, i + r + 1));
    }
    for (int t = 0; t < 1073741824; t++) {
        if (t > r) {
            break;
        }
        int i = n - r - 1 + t;
        BW_AT(out, i) = y;
        if (t == r) {
            break;
        }
        if (mirror != 0) {
            if ((t % 2) == 0) {
                y = nf_add(nf_sub(BW_AT(x, n - 1 - t), BW_AT(x, m + t)), y);
            } else {
                y = nf_sub(nf_add(y, BW_AT(x, n - 1 - t)), BW_AT(x, m + t));
            }
        } else {
            y = nf_sub(y, BW_AT(x, m + t));
        }
    }
}

/* Horizontal pass, one work item per row: np window passes with ping-pong row buffers, result copied to dst.
   scr holds three planes (two ping-pong, one blend). ph0 / ph1 are the scratch alignment phases of buffer 0 / 1. */
__kernel void blur_x_f32(__global const float* src, __global float* dst, __global float* scr,
                         uint w, uint h, int np, int edge, int sub, float frac,
                         int a0, int a1, int a2, int b0, int b1, int b2,
                         float c0, float c1, float c2, uint ph0, uint ph1) {
    size_t row = get_global_id(0);
    if (row >= h) {
        return;
    }
    size_t wh = (size_t)w * (size_t)h;
    size_t off = row * (size_t)w;
    __global const float* inp = src + off;
    __global float* buf_a = scr + off;
    __global float* buf_b = scr + wh + off;
    __global float* zb = scr + 2 * wh + off;
    for (int p = 0; p < 3; p++) {
        if (p >= np) {
            break;
        }
        int ra = (p == 0) ? a0 : ((p == 1) ? a1 : a2);
        int rb = (p == 0) ? b0 : ((p == 1) ? b1 : b2);
        float cc = (p == 0) ? c0 : ((p == 1) ? c1 : c2);
        uint phase = ((p & 1) != 0) ? ph1 : ph0;
        __global float* outp = ((p & 1) != 0) ? buf_b : buf_a;
        int blend = (sub != 0 && ra != rb) ? 1 : 0;
        if (blend != 0) {
            bw_window(inp, zb, 1, (int)w, rb, edge, phase);
        }
        bw_window(inp, outp, 1, (int)w, ra, edge, phase);
        if (blend != 0) {
            for (int i = 0; i < 2147483647; i++) {
                if (i >= (int)w) {
                    break;
                }
                outp[i] = nf_fma231(nf_mul(frac, zb[i]), cc, outp[i]);
            }
        }
        inp = outp;
    }
    __global float* drow = dst + off;
    for (int i = 0; i < 2147483647; i++) {
        if (i >= (int)w) {
            break;
        }
        drow[i] = inp[i];
    }
}

/* Vertical pass, one work item per column; phs holds the scratch alignment phase of the input buffer of each pass
   of each column (w x 3, from the host replay of the CPU's buffer slots). The scratch planes are laid out so that
   neighbouring columns are neighbouring words. dst may be src: every read of src happens in pass 0, before the copy back. */
__kernel void blur_y_f32(__global const float* src, __global float* dst, __global float* scr, __global const uint* phs,
                         uint w, uint h, int np, int edge, int sub, float frac,
                         int a0, int a1, int a2, int b0, int b1, int b2,
                         float c0, float c1, float c2) {
    size_t col = get_global_id(0);
    if (col >= w) {
        return;
    }
    size_t st = (size_t)w;
    size_t wh = (size_t)w * (size_t)h;
    __global const float* inp = src + col;
    __global float* buf_a = scr + col;
    __global float* buf_b = scr + wh + col;
    __global float* zb = scr + 2 * wh + col;
    for (int p = 0; p < 3; p++) {
        if (p >= np) {
            break;
        }
        int ra = (p == 0) ? a0 : ((p == 1) ? a1 : a2);
        int rb = (p == 0) ? b0 : ((p == 1) ? b1 : b2);
        float cc = (p == 0) ? c0 : ((p == 1) ? c1 : c2);
        uint phase = phs[col * 3 + (size_t)p];
        __global float* outp = ((p & 1) != 0) ? buf_b : buf_a;
        int blend = (sub != 0 && ra != rb) ? 1 : 0;
        if (blend != 0) {
            bw_window(inp, zb, st, (int)h, rb, edge, phase);
        }
        bw_window(inp, outp, st, (int)h, ra, edge, phase);
        if (blend != 0) {
            for (int i = 0; i < 2147483647; i++) {
                if (i >= (int)h) {
                    break;
                }
                BW_AT(outp, i) = nf_fma231(nf_mul(frac, BW_AT(zb, i)), cc, BW_AT(outp, i));
            }
        }
        inp = outp;
    }
    __global float* dcol = dst + col;
    for (int i = 0; i < 2147483647; i++) {
        if (i >= (int)h) {
            break;
        }
        BW_AT(dcol, i) = BW_AT(inp, i);
    }
}

/* dst[i] = src[i] * scale (scale_or_copy with a scale other than 1 and 0). */
__kernel void scale_f32(__global const float* src, __global float* dst, uint n, float scale) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    dst[i] = nf_mul(src[i], scale);
}
"#;

/// masked_blur: mask clamp, premultiply and the combine job.
pub const NEIGHBOURHOOD_MASKED: &str = r#"
/* MIRRORS_MUL_ADD: masked_blur::combine_px */
float mb_min_x86(float a, float b) {
    return (a < b) ? a : b;
}

__kernel void mb_clamp_f32(__global const float* src, __global float* dst, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float x = src[i];
    float r;
    if (0x0p+0f > x) {
        r = 0x0p+0f;
    } else {
        r = mb_min_x86(0x1p+0f, x);
    }
    dst[i] = r;
}

__kernel void mb_premul_f32(__global const float* m, __global const float* s, __global float* t, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float mv = m[i];
    float c;
    if (0x0p+0f > mv) {
        c = 0x0p+0f;
    } else {
        c = mb_min_x86(0x1p+0f, mv);
    }
    t[i] = nf_mul(c, s[i]);
}

__kernel void mb_combine_f32(__global const float* m, __global const float* bb, __global const float* ww,
                             __global const float* s, __global float* out, uint n, float thr) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float mv = m[i];
    float sv = s[i];
    float r;
    if (!(mv > thr)) {
        r = sv;
    } else {
        float b = bb[i];
        float wv = ww[i];
        if (wv > 0x0p+0f) {
            b = b / mb_min_x86(wv, 0x1p+0f);
        }
        float m1 = mb_min_x86(mv, 0x1p+0f);
        r = nf_fma231(sv, nf_sub(b, sv), m1);
    }
    out[i] = r;
}
"#;

/// grad: the gradient stencil over the blurred plane (flag_1 = 1 route).
pub const NEIGHBOURHOOD_GRAD: &str = r#"
/* x5 is 1.0 when the info aspect cell is set, else 2.0 (the mutants of the CPU crate are not reachable here). */
__kernel void grad_stencil_f32(__global const float* b, __global float* dx, __global float* dy, uint w, uint h, float x5) {
    size_t gid = get_global_id(0);
    size_t n = (size_t)w * (size_t)h;
    if (gid >= n) {
        return;
    }
    size_t y = gid / (size_t)w;
    size_t x = gid % (size_t)w;
    size_t cur = y * (size_t)w;
    float x1 = x5 + x5;
    size_t up = (y == 0) ? cur : (cur - (size_t)w);
    size_t down = (y + 1 == (size_t)h) ? cur : (cur + (size_t)w);
    float x3 = x1;
    if (y != 0) {
        x3 = x5;
    }
    if (y + 1 == (size_t)h) {
        x3 = x1;
    }
    float two = 0x1p+1f;
    float hlf = 0x1p-1f;
    if (x == 0) {
        float p0 = b[cur];
        float p1 = b[cur + 1];
        float v7 = p1 - p0;
        float v8 = b[up] - b[down];
        v7 = v7 * two;
        v8 = v8 * hlf;
        v8 = v8 * x3;
        dx[gid] = v7;
        dy[gid] = v8;
    } else if (x == (size_t)w - 1) {
        float lo = b[cur + (size_t)w - 2];
        float hi = b[cur + (size_t)w - 1];
        float l7 = b[up + (size_t)w - 1] - b[down + (size_t)w - 1];
        float l0 = hi - lo;
        l7 = l7 * hlf;
        l0 = l0 * two;
        l7 = l7 * x3;
        dx[gid] = l0;
        dy[gid] = l7;
    } else {
        float f = hlf * x3;
        dx[gid] = b[cur + x + 1] - b[cur + x - 1];
        float v = b[up + x] - b[down + x];
        dy[gid] = v * f;
    }
}
"#;

/// sharpen: the combine job over the blurred plane (in place in `out`), the tail keeps the blur or the input.
pub const NEIGHBOURHOOD_SHARPEN: &str = r#"
/* MIRRORS_MUL_ADD: sharpen::combine::combine_elem */
__kernel void sharpen_combine_f32(__global const float* x, __global float* out, uint count, uint n, float g, int alias) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    if (i < count) {
        float xv = x[i];
        float bv = out[i];
        float r;
        if (g == 0x1p+0f) {
            r = nf_fms213(xv, 0x1p+1f, bv);
        } else {
            float d = nf_sub(xv, bv);
            r = nf_fma231(xv, d, g);
        }
        out[i] = r;
    } else if (alias != 0) {
        out[i] = x[i];
    }
}
"#;

/// static2: the integer hash noise, two planes.
pub const NEIGHBOURHOOD_STATIC2: &str = r#"
__kernel void static2_f32(__global const uint* tab, __global float* o0, __global float* o1, uint n, uint base) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    uint v = (uint)i + base;
    for (int r = 0; r < 4; r++) {
        v = (v >> 8) + tab[v & 255u];
    }
    int xi = (int)(v & 4095u) - 2048;
    int yi = (int)((v >> 16) & 4095u) - 2048;
    o0[i] = (float)xi * 0x1p-12f;
    o1[i] = (float)yi * 0x1p-12f;
}
"#;

/// edge_aware_sharpen: weight planes, running sums, the variable window box filter along rows and columns, and the
/// tanh-like transform. The driver (iterations, plane bookkeeping, radii) runs on the host with the CPU functions.
pub const NEIGHBOURHOOD_ESH: &str = r#"
/* MIRRORS_MUL_ADD: edge_aware_sharpen_jobs::job_2, edge_aware_sharpen_jobs::exp_tanh_elem, edge_aware_sharpen_box_filter::filter3, edge_aware_sharpen_render::slot52 */
/* vfmadd213ss d, s2, s3: d * s2 + s3, one rounding; NaN priority d, s2, s3. */
float nf_fma213(float d, float s2, float s3) {
    float r = fma(d, s2, s3);
    return (r == r) ? r : nf_pick3(d, s2, s3);
}

float esh_minss(float a, float b) {
    return (a < b) ? a : b;
}

float esh_h_a(float m, float ca) {
    double md = (double)m;
    if (0x1.b7cdfd9d7bdbbp-34 > md) {
        return ca;
    }
    if (md > 0x1.ffffffff24190p-1) {
        return 0x1p+0f;
    }
    return esh_minss(0x1p+0f / m, ca);
}

float esh_g_t(float m, float k, float cb, float kcb) {
    double md = (double)m;
    if (0x1.b7cdfd9d7bdbbp-34 > md) {
        return kcb;
    }
    if (md > 0x1.ffffffff24190p-1) {
        return k;
    }
    return nf_mul(k, esh_minss(0x1p+0f / m, cb));
}

float esh_l1(__global const float* c0, __global const float* c1, __global const float* c2, int nch, size_t i, size_t j) {
    float acc = 0x0p+0f;
    acc = nf_add(acc, fabs(nf_sub(c0[i], c0[j])));
    if (nch > 1) {
        acc = nf_add(acc, fabs(nf_sub(c1[i], c1[j])));
    }
    if (nch > 2) {
        acc = nf_add(acc, fabs(nf_sub(c2[i], c2[j])));
    }
    return acc;
}

/* Weight planes a (line major) and t (transposed), one work item per sample. c10 / c11 are the constant mode values. */
__kernel void esh_weights_f32(__global const float* c0, __global const float* c1, __global const float* c2, int nch,
                              __global const float* cur, int mode_b, __global float* wa, __global float* wt,
                              uint w, uint s, float d, float e, float k, float ca, float cb, float kcb, float c10, float c11) {
    size_t gid = get_global_id(0);
    size_t total = (size_t)w * (size_t)s;
    if (gid >= total) {
        return;
    }
    size_t line = gid / (size_t)w;
    size_t j = gid % (size_t)w;
    size_t base = line * (size_t)w;
    if (j == 0) {
        wa[gid] = 0x1p+0f;
        if (line == 0) {
            wt[0] = 0x1p+0f;
        } else {
            size_t prev = base - (size_t)w;
            float s0 = esh_l1(c0, c1, c2, nch, base, prev);
            float g0 = c11;
            if (mode_b != 0) {
                g0 = esh_g_t(esh_minss(cur[prev], cur[base]), k, cb, kcb);
                wt[line] = nf_fma231(g0, s0, e);
            } else {
                wt[line] = nf_fma213(s0, e, g0);
            }
        }
        return;
    }
    float sa = esh_l1(c0, c1, c2, nch, base + j, base + j - 1);
    float hh = c10;
    if (mode_b != 0) {
        hh = esh_h_a(esh_minss(cur[base + j - 1], cur[base + j]), ca);
    }
    if (mode_b != 0) {
        wa[gid] = nf_fma231(hh, sa, d);
    } else {
        wa[gid] = nf_fma213(sa, d, hh);
    }
    if (line == 0) {
        wt[j * (size_t)s] = 0x1p+0f;
    } else {
        size_t prev = base - (size_t)w;
        float sv = esh_l1(c0, c1, c2, nch, base + j, prev + j);
        float gg = c11;
        if (mode_b != 0) {
            gg = esh_g_t(esh_minss(cur[prev + j], cur[base + j]), k, cb, kcb);
        }
        if (mode_b != 0) {
            wt[j * (size_t)s + line] = nf_fma231(gg, sv, e);
        } else {
            wt[j * (size_t)s + line] = nf_fma213(sv, e, gg);
        }
    }
}

/* Running sum along each line (one work item per line). */
__kernel void esh_cumsum_x_f32(__global const float* a, __global float* p0, uint w, uint s) {
    size_t line = get_global_id(0);
    if (line >= s) {
        return;
    }
    size_t base = line * (size_t)w;
    float acc = 0x0p+0f;
    for (int j = 0; j < 2147483647; j++) {
        if (j >= (int)w) {
            break;
        }
        acc = nf_add(acc, a[base + (size_t)j]);
        p0[base + (size_t)j] = acc;
    }
}

/* Running sum along each column of the transposed plane (one work item per column; columns have s elements). */
__kernel void esh_cumsum_y_f32(__global const float* t, __global float* p1, uint w, uint s) {
    size_t col = get_global_id(0);
    if (col >= w) {
        return;
    }
    size_t base = col * (size_t)s;
    float acc = 0x0p+0f;
    for (int j = 0; j < 2147483647; j++) {
        if (j >= (int)s) {
            break;
        }
        acc = nf_add(acc, t[base + (size_t)j]);
        p1[base + (size_t)j] = acc;
    }
}

/* The variable window box filter over n elements; a and out have strides ast / ost, the guide b has stride bst. */
void esh_filter3(__global const float* a, size_t ast, __global float* out, size_t ost, __global const float* b, size_t bst,
                 int n, float r, float p1, float p2) {
    if (n <= 0) {
        return;
    }
    float one = 0x1p+0f;
    float two = 0x1p+1f;
    float rm1 = r - one;
    int lo = 0;
    int hi = 1;
    int cnt = 1;
    float sum = a[0];
    for (int i = 0; i < 2147483647; i++) {
        if (i >= n) {
            break;
        }
        float g = b[(size_t)i * bst];
        float lo_thr = g - r;
        float hi_thr = r + g;
        for (int q = 0; q < 2147483647; q++) {
            if (!(lo < i)) {
                break;
            }
            if (!(lo_thr > b[(size_t)lo * bst])) {
                break;
            }
            cnt = cnt - 1;
            sum = nf_sub(sum, a[(size_t)lo * ast]);
            lo = lo + 1;
        }
        int hh = hi;
        for (int q = 0; q < 2147483647; q++) {
            if (!(hh < n)) {
                break;
            }
            if (!(hi_thr >= b[(size_t)hh * bst])) {
                break;
            }
            cnt = cnt + 1;
            sum = nf_add(sum, a[(size_t)hh * ast]);
            hh = hh + 1;
        }
        hi = hh;
        float v9;
        if (p1 > one) {
            float v = a[(size_t)i * ast];
            if (r > two) {
                float s8 = sum;
                int c8 = cnt;
                float t = g - rm1;
                if (t > b[(size_t)lo * bst]) {
                    s8 = nf_sub(sum, a[(size_t)lo * ast]);
                    c8 = cnt - 1;
                }
                float t2 = b[(size_t)(hh - 1) * bst];
                float u = rm1 + g;
                if (t2 > u) {
                    c8 = c8 - 1;
                    s8 = nf_sub(s8, a[(size_t)(hh - 1) * ast]);
                }
                if (c8 > 0) {
                    v = s8 / (float)c8;
                }
            }
            float mean = sum / (float)cnt;
            float dd = nf_sub(mean, v);
            v9 = nf_fma231(v, dd, p2);
        } else {
            v9 = sum / (float)cnt;
        }
        out[(size_t)i * ost] = v9;
    }
}

/* Pass job x: one work item per row. aux is the running sum plane of the weights (guide). */
__kernel void esh_filter_x_f32(__global const float* src, __global float* dst, __global const float* aux,
                               uint w, uint h, float p0, float p1, float p2) {
    size_t row = get_global_id(0);
    if (row >= h) {
        return;
    }
    size_t o = row * (size_t)w;
    esh_filter3(src + o, 1, dst + o, 1, aux + o, 1, (int)w, p0, p1, p2);
}

/* Pass job y: one work item per column; the guide is column major. */
__kernel void esh_filter_y_f32(__global const float* src, __global float* dst, __global const float* aux,
                               uint w, uint h, float p0, float p1, float p2) {
    size_t col = get_global_id(0);
    if (col >= w) {
        return;
    }
    esh_filter3(src + col, (size_t)w, dst + col, (size_t)w, aux + col * (size_t)h, 1, (int)h, p0, p1, p2);
}

float esh_tanh_elem(float s0, float x) {
    float t1 = s0 * x;
    t1 = as_float(0x3fb8aa3bu) * t1;
    float p = as_float(as_uint(t1) ^ 0x80000000u);
    float off = (0x0p+0f > p) ? 0x1p+0f : 0x0p+0f;
    float c_floor = as_float(0xc2fc0000u);
    float clipp = (c_floor > p) ? c_floor : p;
    int wi = cvtt_i32_f(clipp);
    float wf = (float)wi;
    float u = clipp - wf;
    float a1 = as_float(0x42f28c51u) + clipp;
    float z0 = u + off;
    float r = as_float(0x409af5f8u) - clipp;
    float x3 = r + wf;
    float x4 = x3 - off;
    float q = as_float(0x41ddd2feu) / x4;
    float v = a1 + q;
    v = fma(-z0, as_float(0x3fbebc8du), v);
    float m = 0x1p+23f * v;
    long l = cvtt_i64_f(m);
    uint lo = (uint)((ulong)l);
    float d = 0x1p+0f + as_float(lo);
    float e = 0x1p+0f / d;
    return fma(e, 0x1p+1f, -0x1p+0f);
}

/* out += tanh(s0, fma(b, -1, 0) + a): the "C = -B; C += A; transform; out += C" steps of one channel and iteration. */
__kernel void esh_level_f32(__global const float* bb, __global const float* aa, __global float* out, uint n, float s0) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float c = fma(bb[i], -0x1p+0f, 0x0p+0f);
    c = nf_add(c, aa[i]);
    c = esh_tanh_elem(s0, c);
    out[i] = nf_add(out[i], c);
}

/* out += a (the tail of the driver). */
__kernel void esh_add_f32(__global const float* aa, __global float* out, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    out[i] = nf_add(out[i], aa[i]);
}
"#;

#[cfg(test)]
mod tests {
    /// The two double literals of NEIGHBOURHOOD_ESH are the CPU constants (edge_aware_sharpen_jobs::LOW, HIGH): sign 0, exponent field and 52 bit mantissa.
    #[test]
    fn the_hex_double_constants_are_the_cpu_ones() {
        let low = 0x3ddb_7cdf_d9d7_bdbbu64;
        assert_eq!(low >> 52, 1023 - 34);
        assert_eq!(low & ((1u64 << 52) - 1), 0xb7cdfd9d7bdbb);
        assert!(super::NEIGHBOURHOOD_ESH.contains("0x1.b7cdfd9d7bdbbp-34"));
        let high = 0x3fef_ffff_fff2_4190u64;
        assert_eq!(high >> 52, 1023 - 1);
        assert_eq!(high & ((1u64 << 52) - 1), 0xffffffff24190);
        assert!(super::NEIGHBOURHOOD_ESH.contains("0x1.ffffffff24190p-1"));
    }
}
