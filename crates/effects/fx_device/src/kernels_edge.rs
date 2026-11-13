//! Device kernel source of the edge_detect gradient body (`edge_detect::edge_rows`): one work item per output pixel.
//! OpenCL C 1.2 as a Rust string constant; registered in [`crate::kernels::SOURCES`].
//!
//! The input is the blurred plane (the neighbourhood blur kernels produce it). Every subtract and multiply uses the x86 two-NaN
//! rule of `fx_common::nanop` through the `nanx_sub` and `nanx_mul` helpers of the pointwise source, which precedes this source in
//! the program. The three fused multiply-adds are `vfmadd213ss d, s2, s3` (d * s2 + s3, one rounding, NaN priority d, s2, s3);
//! the square root is correctly rounded (build option and probe) and its NaN is made by explicit tests. The host passes the row
//! factor `c`, the doubled factor `c + c` and the one-sided scale `k` as f32 values.

/// Edge detect gradient body.
pub const EDGE_GRAD: &str = r#"
/* MIRRORS_MUL_ADD: edge_detect::edge_rows (fma213 at the three squared-sum sites) */

float ed_fma213(float d, float s2, float s3) {
    float r = fma(d, s2, s3);
    if (r == r) {
        return r;
    }
    if (d != d) {
        return as_float(as_uint(d) | 0x400000u);
    }
    if (s2 != s2) {
        return as_float(as_uint(s2) | 0x400000u);
    }
    if (s3 != s3) {
        return as_float(as_uint(s3) | 0x400000u);
    }
    return as_float(0xffc00000u);
}

float ed_sqrt(float x) {
    float r = sqrt(x);
    if (r == r) {
        return r;
    }
    if (x != x) {
        return as_float(as_uint(x) | 0x400000u);
    }
    return as_float(0xffc00000u);
}

__kernel void edge_grad_f32(__global const float* b, __global float* out, uint w, uint h, float c, float c2, float k) {
    size_t i = get_global_id(0);
    if (i >= (size_t)w * (size_t)h) {
        return;
    }
    size_t wd = (size_t)w;
    size_t x = i % wd;
    size_t y = i / wd;
    size_t cur = y * wd;
    size_t up = (y == 0) ? cur : cur - wd;
    size_t down = (y + 1 == (size_t)h) ? cur : cur + wd;
    float v = (y == 0 || y + 1 == (size_t)h) ? c2 : c;
    float r;
    if (x == 0) {
        float gx0 = nanx_mul(k, nanx_sub(b[cur + 1], b[cur]));
        float gx0sq = nanx_mul(gx0, gx0);
        float gy = nanx_mul(nanx_sub(b[up], b[down]), v);
        r = ed_fma213(gy, gy, gx0sq);
    } else if (x == wd - 1) {
        float g = nanx_mul(k, nanx_sub(b[cur + wd - 1], b[cur + wd - 2]));
        float gxl_sq = nanx_mul(g, g);
        float gyl = nanx_mul(nanx_sub(b[up + wd - 1], b[down + wd - 1]), v);
        r = ed_fma213(gyl, gyl, gxl_sq);
    } else {
        float gyr = nanx_mul(v, nanx_sub(b[up + x], b[down + x]));
        float gysq = nanx_mul(gyr, gyr);
        float gx = nanx_sub(b[cur + x + 1], b[cur + x - 1]);
        r = ed_fma213(gx, gx, gysq);
    }
    out[i] = ed_sqrt(r);
}
"#;
