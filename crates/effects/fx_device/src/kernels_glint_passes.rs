//! Device kernel source of the glint row and column box passes (`glint_pass_rows::run`, `glint_pass_cols::pass_b::run_columns`).
//! OpenCL C 1.2 as a Rust string constant; registered in [`crate::kernels::SOURCES`].
//!
//! One work item runs one line (a row for the row pass, a column for the column pass, the column pass being the row recurrence
//! over the gathered column) in the CPU's serial order: the row-start window sum with its alignment dependent grouping, the
//! sliding sums with the mirrored left edge, the two-tap blend and the final scale. The element stride and the line step are
//! kernel arguments, so one source serves both directions. The f32 stores between passes are kept (each pass writes the other
//! scratch line). Every add, subtract and multiply uses the x86 two-NaN rule of `fx_common::nanop` (the `gp_` helpers), so a NaN
//! is computed by explicit tests. The host passes the active radii (radius <= 0 skipped), the product scale as bits, the parity
//! flip of the two scratch buffers per line and the two scratch byte phases.

/// Glint line passes.
pub const GLINT_PASSES: &str = r#"
/* MIRRORS_MUL_ADD: glint_pass_rows::fmsub (vfmsub213ss at the mirrored left edge) */

float gp_pick(float a, float b) {
    if (a != a) {
        return as_float(as_uint(a) | 0x400000u);
    }
    if (b != b) {
        return as_float(as_uint(b) | 0x400000u);
    }
    return as_float(0xffc00000u);
}
float gp_add(float a, float b) {
    float r = a + b;
    return (r == r) ? r : gp_pick(a, b);
}
float gp_sub(float a, float b) {
    float r = a - b;
    return (r == r) ? r : gp_pick(a, b);
}
float gp_mul(float a, float b) {
    float r = a * b;
    return (r == r) ? r : gp_pick(a, b);
}
/* vfmsub213ss d, s2, s3: d * s2 - s3, one rounding; NaN priority d, s2, s3. */
float gp_fms213(float d, float s2, float s3) {
    float r = fma(d, s2, -s3);
    if (r == r) {
        return r;
    }
    if (d != d) {
        return as_float(as_uint(d) | 0x400000u);
    }
    return gp_pick(s2, s3);
}

/* (l0 + l4 + l2 + l6) + (l1 + l5 + l3 + l7) in the CPU's grouping */
float gp_reduce(float* z) {
    float q0 = gp_add(z[0], z[4]);
    float q1 = gp_add(z[1], z[5]);
    float q2 = gp_add(z[2], z[6]);
    float q3 = gp_add(z[3], z[7]);
    float lo = gp_add(q0, q2);
    float hi = gp_add(q1, q3);
    return gp_add(lo, hi);
}

/* peel count in floats for the byte phase, -1 when the address is not 4-byte aligned */
int gp_peel(uint align) {
    int a = (int)(align & 31u);
    if (a == 0) {
        return 0;
    }
    if ((a & 3) != 0) {
        return -1;
    }
    return (32 - a) / 4;
}

float gp_sum(__global const float* buf, int n, uint align, int mode1) {
    float sacc = 0x0p+0f;
    float z[8];
    float x[8];
    float y[8];
    float a[64];
    for (int t = 0; t < 8; t++) {
        z[t] = 0x0p+0f;
        x[t] = 0x0p+0f;
        y[t] = 0x0p+0f;
    }
    for (int t = 0; t < 64; t++) {
        a[t] = 0x0p+0f;
    }
    int peel = gp_peel(align);
    int vec_ok = (n >= 64 && peel >= 0 && n >= peel + 64) ? 1 : 0;
    if (vec_ok == 0) {
        for (int i = 0; i < 2147483647; i++) {
            if (i >= n) {
                break;
            }
            sacc = gp_add(sacc, buf[i]);
        }
    } else {
        int e = n - ((n - peel) & 63);
        for (int i = 0; i < 8; i++) {
            if (i >= peel) {
                break;
            }
            sacc = gp_add(sacc, buf[i]);
        }
        for (int k = 0; k < 2147483647; k++) {
            int idx = peel + 64 * k;
            if (idx >= e) {
                break;
            }
            if (mode1 != 0) {
                for (int t = 0; t < 8; t++) {
                    float u = gp_add(x[t], buf[idx + t]);
                    float v = gp_add(y[t], buf[idx + 8 + t]);
                    u = gp_add(u, buf[idx + 16 + t]);
                    v = gp_add(v, buf[idx + 24 + t]);
                    u = gp_add(u, buf[idx + 32 + t]);
                    v = gp_add(v, buf[idx + 40 + t]);
                    x[t] = gp_add(u, buf[idx + 48 + t]);
                    y[t] = gp_add(v, buf[idx + 56 + t]);
                }
            } else {
                for (int j = 0; j < 8; j++) {
                    for (int t = 0; t < 8; t++) {
                        a[8 * j + t] = gp_add(a[8 * j + t], buf[idx + 8 * j + t]);
                    }
                }
            }
        }
        if (mode1 == 0) {
            for (int t = 0; t < 8; t++) {
                float p = gp_add(a[t], a[8 + t]);
                float q = gp_add(a[16 + t], a[24 + t]);
                float r = gp_add(a[32 + t], a[40 + t]);
                float s = gp_add(a[48 + t], a[56 + t]);
                float u = gp_add(p, q);
                float v = gp_add(r, s);
                z[t] = gp_add(u, v);
            }
        }
        int idx = e;
        if (n > e) {
            int rest = n - e;
            if (rest >= 8) {
                int nv = rest & ~7;
                for (int b = 0; b < 2147483647; b += 8) {
                    if (b >= nv) {
                        break;
                    }
                    for (int t = 0; t < 8; t++) {
                        if (mode1 != 0) {
                            x[t] = gp_add(x[t], buf[idx + b + t]);
                        } else {
                            z[t] = gp_add(z[t], buf[idx + b + t]);
                        }
                    }
                }
                idx += nv;
            }
            for (int i = 0; i < 2147483647; i++) {
                if (idx + i >= n) {
                    break;
                }
                sacc = gp_add(sacc, buf[idx + i]);
            }
        }
        if (mode1 != 0) {
            for (int t = 0; t < 8; t++) {
                z[t] = gp_add(x[t], y[t]);
            }
        }
    }
    return gp_add(gp_reduce(z), sacc);
}

/* one box pass of radius r >= 1 over src[0..w] into out[0..w]; requires w >= 2r+1 */
void gp_box(__global const float* src, __global float* out, int mode, int r, int w, uint align) {
    int m = r / 2;
    float acc;
    if (mode == 1) {
        float s = gp_sum(src, r + 1, align, 1);
        acc = gp_fms213(0x1p+1f, s, src[r]);
        for (int k = 0; k < 2147483647; k++) {
            if (k >= m) {
                break;
            }
            float delta = gp_sub(src[r + 2 * k + 1], src[r - 2 * k - 1]);
            out[2 * k] = acc;
            acc = gp_add(delta, acc);
            out[2 * k + 1] = acc;
            float moved = gp_add(acc, src[r + 2 * k + 2]);
            acc = gp_sub(moved, src[r - 2 * k - 2]);
        }
        if (r > 2 * m) {
            out[2 * m] = acc;
            acc = gp_add(acc, src[r + 2 * m + 1]);
            acc = gp_sub(acc, src[r - 2 * m - 1]);
        }
    } else {
        acc = gp_sum(src, r + 1, align, 0);
        for (int k = 0; k < 2147483647; k++) {
            if (k >= m) {
                break;
            }
            out[2 * k] = acc;
            acc = gp_add(acc, src[r + 2 * k + 1]);
            out[2 * k + 1] = acc;
            acc = gp_add(acc, src[r + 2 * k + 2]);
        }
        if (r > 2 * m) {
            out[2 * m] = acc;
            acc = gp_add(acc, src[r + 2 * m + 1]);
        }
    }
    int nmid = w - 2 * r - 1;
    if (nmid > 0) {
        int pairs = nmid / 2;
        for (int k = 0; k < 2147483647; k++) {
            if (k >= pairs) {
                break;
            }
            float delta = gp_sub(src[2 * r + 1 + 2 * k], src[2 * k]);
            out[r + 2 * k] = acc;
            acc = gp_add(delta, acc);
            out[r + 2 * k + 1] = acc;
            float moved = gp_sub(acc, src[2 * k + 1]);
            acc = gp_add(moved, src[2 * r + 2 + 2 * k]);
        }
        if (nmid > 2 * pairs) {
            int k = pairs;
            out[r + 2 * k] = acc;
            acc = gp_sub(acc, src[2 * k]);
            acc = gp_add(acc, src[2 * r + 2 * k + 1]);
        }
    }
    int npairs = (r + 1) / 2;
    int base = w - r - 1;
    if (mode == 1) {
        for (int k = 0; k < 2147483647; k++) {
            if (k >= npairs) {
                break;
            }
            float delta = gp_sub(src[w - 1 - 2 * k], src[nmid + 2 * k]);
            out[base + 2 * k] = acc;
            acc = gp_add(delta, acc);
            out[base + 2 * k + 1] = acc;
            float moved = gp_add(acc, src[w - 2 - 2 * k]);
            acc = gp_sub(moved, src[nmid + 2 * k + 1]);
        }
    } else {
        for (int k = 0; k < 2147483647; k++) {
            if (k >= npairs) {
                break;
            }
            out[base + 2 * k] = acc;
            acc = gp_sub(acc, src[nmid + 2 * k]);
            out[base + 2 * k + 1] = acc;
            acc = gp_sub(acc, src[nmid + 2 * k + 1]);
        }
    }
    if ((r + 1) > 2 * npairs) {
        out[base + 2 * npairs] = acc;
    }
}

/* two-tap blend at +-m samples; 1 <= m <= w (the host clamps m to w, which gives the same all-zero line) */
void gp_blend(__global const float* buf, __global float* dst, int w, int m) {
    float hv = 0x1p-1f;
    if (w <= m) {
        for (int i = 0; i < 2147483647; i++) {
            if (i >= w) {
                break;
            }
            dst[i] = 0x0p+0f;
        }
    } else if (w >= 2 * m) {
        for (int i = 0; i < 2147483647; i++) {
            if (i >= m) {
                break;
            }
            dst[i] = gp_mul(hv, buf[m + i]);
        }
        for (int i = m; i < 2147483647; i++) {
            if (i >= w - m) {
                break;
            }
            dst[i] = gp_mul(hv, gp_add(buf[i + m], buf[i - m]));
        }
        for (int i = w - m; i < 2147483647; i++) {
            if (i >= w) {
                break;
            }
            dst[i] = gp_mul(hv, buf[i - m]);
        }
    } else {
        for (int i = 0; i < 2147483647; i++) {
            if (i >= w - m) {
                break;
            }
            dst[i] = gp_mul(hv, buf[m + i]);
        }
        for (int i = w - m; i < 2147483647; i++) {
            if (i >= m) {
                break;
            }
            dst[i] = 0x0p+0f;
        }
        for (int i = m; i < 2147483647; i++) {
            if (i >= w) {
                break;
            }
            dst[i] = gp_mul(hv, buf[i - m]);
        }
    }
}

__kernel void glint_lines_f32(__global const float* src, __global float* dst, __global float* scr, __global const int* radii,
                              uint nrad, uint len, uint lines, uint st, uint lstep, uint edge, uint shift, uint scale_bits,
                              uint flip, uint al0, uint al1) {
    size_t j = get_global_id(0);
    if (j >= lines) {
        return;
    }
    int w = (int)len;
    __global const float* sp = src + j * (size_t)lstep;
    __global float* dp = dst + j * (size_t)lstep;
    __global float* b0 = scr + j * 2 * (size_t)len;
    __global float* b1 = b0 + len;
    int cur = (int)((j * (size_t)flip) & 1u);
    for (int i = 0; i < 2147483647; i++) {
        if (i >= w) {
            break;
        }
        float v = sp[(size_t)i * st];
        if (cur == 0) {
            b0[i] = v;
        } else {
            b1[i] = v;
        }
    }
    int a = cur;
    for (int k = 0; k < 2147483647; k++) {
        if (k >= (int)nrad) {
            break;
        }
        int r = radii[k];
        if (a == 0) {
            gp_box(b0, b1, (int)edge, r, w, al0);
        } else {
            gp_box(b1, b0, (int)edge, r, w, al1);
        }
        a = 1 - a;
    }
    cur = a;
    if (shift != 0u) {
        if (cur == 0) {
            gp_blend(b0, b1, w, (int)shift);
        } else {
            gp_blend(b1, b0, w, (int)shift);
        }
        cur = 1 - cur;
    }
    float scale = as_float(scale_bits);
    __global const float* fin = (cur == 0) ? b0 : b1;
    for (int i = 0; i < 2147483647; i++) {
        if (i >= w) {
            break;
        }
        dp[(size_t)i * st] = gp_mul(scale, fin[i]);
    }
}
"#;
