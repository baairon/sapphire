//! Device kernel sources of `optical_flow_interp_maps` (crate `optical_flow_interp_maps`, `native::interp_maps`): the forward splat and
//! the scalar scan (one work item each, the crate's serial loops in the crate's order), the fill step, the masked copy, the zero
//! count (one work item per row, then one work item sum) and the interpolation of the four output maps.
//! OpenCL C 1.2 as a Rust string constant; registered in [`crate::kernels::SOURCES`].
//!
//! Exactness rules: `FP_CONTRACT OFF` (prelude); `fma()` only at the named fused sites of the crate (`fma213`, `fma231`, `fms213`,
//! `fnms213`, `fnma213`), each with the crate's NaN priority (destination, second source, third source); the binary operations use the
//! `nanx_` helpers of [`crate::kernels_pointwise`], which precede this source in the program; conversions follow `cvttss2si`; every
//! constant is the crate's own value in hexadecimal. Host scalars of the interpolation (`c`, `cs`, `oms`, `gain_rest`) are computed
//! by the crate's own `NanOps` on the host and passed as bits.

/// The interpolation map kernels.
pub const INTERP_MAPS: &str = r#"
/* MIRRORS_MUL_ADD: optical_flow_interp_maps::channel_error, kernels::k1_splat, optical_flow_interp_maps::splat_scan, kernels::k5_interp */

/* vfmadd213ss d, second, third: d * second + third with one rounding; a NaN result is the first NaN of d, second, third quieted, else the default NaN */
float im_fma213(float d, float second, float third) {
    float r = fma(d, second, third);
    return isnan(r) ? nanx_pick3(d, second, third) : r;
}

/* vfmadd231ss d, second, third: second * third + d */
float im_fma231(float d, float second, float third) {
    float r = fma(second, third, d);
    return isnan(r) ? nanx_pick3(d, second, third) : r;
}

/* vfnmadd213ss d, second, third: -(d * second) + third */
float im_fnma213(float d, float second, float third) {
    float r = fma(-d, second, third);
    return isnan(r) ? nanx_pick3(d, second, third) : r;
}

/* vfmsub213ss d, second, third: d * second - third */
float im_fms213(float d, float second, float third) {
    float r = fma(d, second, -third);
    return isnan(r) ? nanx_pick3(d, second, third) : r;
}

/* vfnmsub213ss d, second, third: -(d * second) - third */
float im_fnms213(float d, float second, float third) {
    float r = fma(-d, second, -third);
    return isnan(r) ? nanx_pick3(d, second, third) : r;
}

float im_flip(float x) {
    return as_float(as_uint(x) ^ 0x80000000u);
}

/* split_coord: lower and upper sample and the fraction; c < 0 clamps to 0 and 1, c >= n - 1 to n - 2 and n - 1 with fraction 1 */
void im_split(float c, int n, int* lo, int* hi, float* fr) {
    if (0x0p+0f > c) {
        *lo = 0;
        *hi = 1;
        *fr = 0x0p+0f;
    } else if (c >= (float)(n - 1)) {
        *lo = n - 2;
        *hi = n - 1;
        *fr = 0x1p+0f;
    } else {
        float fl = floor(c);
        int i = (fl >= -0x1p31f && fl < 0x1p31f) ? (int)fl : INT_MIN;
        *lo = i;
        *hi = i + 1;
        *fr = nanx_sub(c, (float)i);
    }
}

float im_channel_error(float tl, float tr, float ll, float br, float fx, float fy, float p) {
    float dxa = nanx_sub(tr, tl);
    float delta_bottom = nanx_sub(br, ll);
    float dyl = nanx_sub(tl, ll);
    float t = nanx_sub(delta_bottom, dxa);
    float a = im_fms213(dxa, fx, p);
    float b = im_fms213(t, fx, dyl);
    float c = nanx_add(a, tl);
    float e = im_fnms213(b, fy, c);
    return nanx_mul(e, e);
}

__global const uint* im_sel(__global const uint* p0, __global const uint* mid_plane, __global const uint* p2, int k) {
    return (k == 0) ? p0 : ((k == 1) ? mid_plane : p2);
}

/* matching cost of one source pixel over up to three channels */
float im_cost(__global const uint* src_ch0, __global const uint* src_ch1, __global const uint* src_ch2,
              __global const uint* ref_ch0, __global const uint* ref_ch1, __global const uint* ref_ch2,
              float av, float bv, float xf, float yf, int col, int row, int w, int h, int nc) {
    float cost = 0x0p+0f;
    if (nc > 0) {
        float src_x = nanx_add(av, xf);
        float src_y = nanx_add(bv, yf);
        int x_lo; int x_hi; float fx;
        int y_lo; int y_hi; float fy;
        im_split(src_x, w, &x_lo, &x_hi, &fx);
        im_split(src_y, h, &y_lo, &y_hi, &fy);
        int row_lo = y_lo * w;
        int row_hi = y_hi * w;
        int pidx = row * w + col;
        for (int chan = 0; chan < 3; chan++) {
            if (chan >= nc) {
                break;
            }
            __global const uint* pl = im_sel(src_ch0, src_ch1, src_ch2, chan);
            float sample_tl = as_float(pl[x_lo + row_lo]);
            float sample_tr = as_float(pl[x_hi + row_lo]);
            float sample_lower_left = as_float(pl[x_lo + row_hi]);
            float sample_br = as_float(pl[x_hi + row_hi]);
            float p = as_float(im_sel(ref_ch0, ref_ch1, ref_ch2, chan)[pidx]);
            float e = im_channel_error(sample_tl, sample_tr, sample_lower_left, sample_br, fx, fy, p);
            cost = (chan == 0) ? e : nanx_add(e, cost);
        }
    }
    return cost;
}

/* the target pixel keeps the first visitor, then any later visitor with a strictly lower cost */
void im_store(__global uint* tfx, __global uint* tfy, __global uint* best, __global uint* cov, int d, uint abits, uint bbits, float cost) {
    if (cov[d] != 0u) {
        if (as_float(best[d]) > cost) {
            tfx[d] = abits;
            tfy[d] = bbits;
            best[d] = as_uint(cost);
        }
    } else {
        tfx[d] = abits;
        tfy[d] = bbits;
        best[d] = as_uint(cost);
        cov[d] = 1u;
    }
}

/* k1_splat over every row, then splat_scan: one work item, the crate's serial loops in raster order */
__kernel void im_splat(__global const uint* fa, __global const uint* fb,
                       __global const uint* src_ch0, __global const uint* src_ch1, __global const uint* src_ch2,
                       __global const uint* ref_ch0, __global const uint* ref_ch1, __global const uint* ref_ch2,
                       __global uint* tfx, __global uint* tfy, __global uint* best, __global uint* cov, __global uint* sdone,
                       int w, int h, int nc, float scale) {
    if (get_global_id(0) != 0) {
        return;
    }
    for (int row = 0; row < 2147483647; row++) {
        if (row >= h) {
            break;
        }
        float yf = (float)row;
        float row_centre = nanx_add(0x1p-1f, yf);
        for (int col = 0; col < 2147483647; col++) {
            if (col >= w) {
                break;
            }
            int idx = row * w + col;
            uint abits = fa[idx];
            uint bbits = fb[idx];
            float av = as_float(abits);
            float bv = as_float(bbits);
            float sb = nanx_mul(scale, bv);
            float tyf = im_fma213(sb, -0x1p+0f, row_centre);
            float shifted = im_fma231(0x1p-1f, scale, av);
            float xf = (float)col;
            float txf = nanx_add(shifted, xf);
            int ty = cvtt_i32_f(tyf);
            int tx = cvtt_i32_f(txf);
            if (ty < 0 || ty >= h) {
                continue;
            }
            if (tx < 0 || tx >= w || ty < 0 || ty >= h) {
                sdone[idx] = 1u;
                continue;
            }
            float cost = im_cost(src_ch0, src_ch1, src_ch2, ref_ch0, ref_ch1, ref_ch2, av, bv, xf, yf, col, row, w, h, nc);
            im_store(tfx, tfy, best, cov, tx + ty * w, abits, bbits, cost);
            sdone[idx] = 1u;
        }
    }
    for (int row = 0; row < 2147483647; row++) {
        if (row >= h) {
            break;
        }
        float yf = (float)row;
        float ysum = nanx_add(0x1p-1f, yf);
        for (int col = 0; col < 2147483647; col++) {
            if (col >= w) {
                break;
            }
            int idx = row * w + col;
            if (sdone[idx] != 0u) {
                continue;
            }
            uint abits = fa[idx];
            uint bbits = fb[idx];
            float av = as_float(abits);
            float bv = as_float(bbits);
            float xf = (float)col;
            float sb = nanx_mul(scale, bv);
            float tx0 = nanx_add(0x1p-1f, xf);
            float ty = im_fma213(sb, -0x1p+0f, ysum);
            float tx = im_fma231(tx0, scale, av);
            int ix = cvtt_i32_f(tx);
            int iy = cvtt_i32_f(ty);
            if (ix < 0 || ix >= w || iy < 0 || iy >= h) {
                continue;
            }
            float cost = im_cost(src_ch0, src_ch1, src_ch2, ref_ch0, ref_ch1, ref_ch2, av, bv, xf, yf, col, row, w, h, nc);
            im_store(tfx, tfy, best, cov, ix + iy * w, abits, bbits, cost);
        }
    }
}

/* k2_fill: one work item per pixel; a pixel whose covered flag is zero takes the mean of the flagged values of its 3x3 neighbourhood */
__kernel void im_fill(__global const uint* cov, __global const uint* vin, __global uint* vout,
                      __global const uint* flin, __global uint* flout, int w, int h) {
    int i = (int)get_global_id(0);
    if (i >= w * h) {
        return;
    }
    if (cov[i] != 0u) {
        return;
    }
    int y = i / w;
    int x = i - y * w;
    float cnt = 0x0p+0f;
    float sum = 0x0p+0f;
    for (int ri = 0; ri < 3; ri++) {
        int yy = y - 1 + ri;
        if (yy < 0 || h <= yy) {
            continue;
        }
        for (int k = 0; k < 3; k++) {
            int xx = x - 1 + k;
            if (xx < 0 || w <= xx) {
                continue;
            }
            int j = yy * w + xx;
            if (flin[j] == 0u) {
                continue;
            }
            float v = as_float(vin[j]);
            if (ri == 0 && k == 0) {
                cnt = 0x1p+0f;
                sum = v;
            } else {
                cnt = cnt + 0x1p+0f;
                sum = nanx_add(sum, v);
            }
        }
    }
    if (cnt > 0x0p+0f) {
        vout[i] = as_uint(nanx_div(sum, cnt));
        flout[i] = 1u;
    }
}

/* k3_masked_copy: dst = src where the flag word is zero */
__kernel void im_masked_copy(__global const uint* flag, __global const uint* src, __global uint* dst, int w, int h) {
    int i = (int)get_global_id(0);
    if (i >= w * h) {
        return;
    }
    if (flag[i] == 0u) {
        dst[i] = src[i];
    }
}

/* k4_count_zero: the zero words of each row, then the sum of the row counts (one work item) */
__kernel void im_count_rows(__global const uint* plane, __global uint* rows, int w, int h) {
    int y = (int)get_global_id(0);
    if (y >= h) {
        return;
    }
    uint n = 0u;
    for (int x = 0; x < 2147483647; x++) {
        if (x >= w) {
            break;
        }
        if (plane[y * w + x] == 0u) {
            n = n + 1u;
        }
    }
    rows[y] = n;
}

__kernel void im_count_sum(__global const uint* rows, __global uint* total, int h) {
    if (get_global_id(0) != 0) {
        return;
    }
    uint n = 0u;
    for (int y = 0; y < 2147483647; y++) {
        if (y >= h) {
            break;
        }
        n = n + rows[y];
    }
    total[0] = n;
}

/* k5_interp: one work item per pixel; the row path (scalar head, eight lane body, scalar tail) is chosen from the width and the row's byte phase */
__kernel void im_interp(__global const uint* fa, __global const uint* fb, __global uint* out_a, __global uint* out_b, __global uint* out_c, __global uint* out_d,
                        int w, int h, float scale, float c, float cs, float oms, float gain_rest, float flow_offset) {
    int i = (int)get_global_id(0);
    if (i >= w * h) {
        return;
    }
    int y = i / w;
    int xi = i - y * w;
    ulong addr = ((ulong)(y * w)) * 4ul;
    int head = 0;
    int vend = 0;
    if (w < 8) {
        head = 0;
        vend = 0;
    } else if (w < 0x4b0) {
        head = 0;
        vend = w & ~7;
    } else {
        int m = (int)(addr & 0x1ful);
        if (m == 0) {
            head = 0;
            vend = w - (w & 7);
        } else if ((m & 3) != 0) {
            head = 0;
            vend = 0;
        } else {
            int hd = (32 - m) / 4;
            if (w < hd + 8) {
                head = 0;
                vend = 0;
            } else {
                head = hd;
                vend = w - ((w - hd) & 7);
            }
        }
    }
    float a = as_float(fa[i]);
    float b = as_float(fb[i]);
    int vector = (xi >= head && xi < vend && vend > 0) ? 1 : 0;
    float res_a;
    float res_b;
    float res_c;
    float res_d;
    if (vector != 0) {
        res_a = nanx_mul(im_flip(cs), a);
        res_c = nanx_mul(gain_rest, a);
        res_b = nanx_mul(c, im_fma213(im_flip(scale), b, flow_offset));
        res_d = nanx_mul(c, im_fma213(b, oms, flow_offset));
    } else {
        res_a = im_flip(nanx_mul(cs, a));
        res_c = nanx_mul(a, gain_rest);
        res_b = nanx_mul(c, im_fnma213(scale, b, flow_offset));
        res_d = nanx_mul(c, im_fma213(b, oms, flow_offset));
    }
    out_a[i] = as_uint(res_a);
    out_b[i] = as_uint(res_b);
    out_c[i] = as_uint(res_c);
    out_d[i] = as_uint(res_d);
}
"#;
