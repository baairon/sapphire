//! Device kernel source of the plane services that the composed `optical_flow` op needs besides the solve, fetch and resample kernels:
//! the constant add and the flow normalisation of `optical_flow::callbacks`, and the rectangle clear, the rectangle copy, the prefix copy
//! and the row compaction of the composed host. OpenCL C 1.2 as a Rust string constant; registered in [`crate::kernels::SOURCES`].
//!
//! One work item per element. The arithmetic kernels use the `flow_a_` helpers of the solve kernels (every add, subtract, multiply, divide and fused
//! step has the x86 two-NaN rule of `fx_common::nanop`); the copies and the clear move 32 bit words, so no float value is ever interpreted.

/// Compose kernels.
pub const FLOW_COMPOSE: &str = r#"
/* MIRRORS_MUL_ADD: fx_common::nanop fma213 and fma231 (the crate's mul_add sites of normalize_flow_rows) */

/* optical_flow::callbacks::add_const: out = c + out over n words; c equal to zero (either sign) zero fills. */
__kernel void flow_d_add_const(__global float* d, uint n, uint c_bits) {
    size_t g = get_global_id(0);
    if (g >= (size_t)n) {
        return;
    }
    float c = as_float(c_bits);
    if (c == as_float(0u)) {
        d[g] = as_float(0u);
    } else {
        d[g] = flow_a_add(c, d[g]);
    }
}

/* optical_flow::callbacks::normalize_flow_rows over `count` rows from row0 of a plane of total_rows rows. */
__kernel void flow_d_normalize_flow_rows(__global const float* p, __global float* fx, __global float* fy,
                                      int w, int total_rows, int stride, int row0, uint num_bits, uint den_bits, uint total) {
    size_t g = get_global_id(0);
    if (g >= (size_t)total) {
        return;
    }
    float s = flow_a_div(as_float(num_bits), as_float(den_bits));
    long y = (long)row0 + (long)(g / (size_t)w);
    long x = (long)(g % (size_t)w);
    long last = (long)total_rows - 1;
    long i = (long)stride * y + x;
    long wm1 = (long)w - 1;
    if (y < last) {
        long below = (long)stride * (y + 1) + x;
        if (x < wm1) {
            float a = p[i];
            float b = p[below];
            float c = p[i + 1];
            float gy0 = flow_a_sub(b, a);
            float gx0 = flow_a_sub(c, a);
            float gy = flow_a_fma213(gy0, s, fy[i]);
            float gx = flow_a_fma213(gx0, s, fx[i]);
            float norm_sq = flow_a_fma231(flow_a_mul(gy, gy), gx, gx);
            if (norm_sq > as_float(0x3f800000u)) {
                float n = sqrt(norm_sq);
                fx[i] = flow_a_div(gx, n);
                fy[i] = flow_a_div(gy, n);
            } else {
                fx[i] = gx;
                fy[i] = gy;
            }
        } else {
            float fxo = fx[i];
            float gy0 = flow_a_sub(p[below], p[i]);
            float gy = flow_a_fma213(gy0, s, fy[i]);
            float norm_sq = flow_a_fma231(flow_a_mul(gy, gy), fxo, fxo);
            if (norm_sq > as_float(0x3f800000u)) {
                float n = sqrt(norm_sq);
                fx[i] = flow_a_div(fxo, n);
                fy[i] = flow_a_div(gy, n);
            } else {
                fy[i] = gy;
            }
        }
    } else if (x < wm1) {
        float fyo = fy[i];
        float gx0 = flow_a_sub(p[i + 1], p[i]);
        float gx = flow_a_fma213(gx0, s, fx[i]);
        float norm_sq = flow_a_fma231(flow_a_mul(fyo, fyo), gx, gx);
        if (norm_sq > as_float(0x3f800000u)) {
            float n = sqrt(norm_sq);
            fx[i] = flow_a_div(gx, n);
            fy[i] = flow_a_div(fyo, n);
        } else {
            fx[i] = gx;
        }
    } else {
        float fyo = fy[i];
        float fxo = fx[i];
        float norm_sq = flow_a_fma231(flow_a_mul(fyo, fyo), fxo, fxo);
        if (norm_sq > as_float(0x3f800000u)) {
            float n = sqrt(norm_sq);
            fx[i] = flow_a_div(fxo, n);
            fy[i] = flow_a_div(fyo, n);
        }
    }
}

/* Rectangle clear: width x rows words of value `val` at the row stride, from the first word. */
__kernel void flow_d_clear_rect(__global uint* d, uint width, uint rows, uint stride, uint val) {
    size_t g = get_global_id(0);
    if (g >= (size_t)width * (size_t)rows) {
        return;
    }
    size_t y = g / (size_t)width;
    size_t x = g % (size_t)width;
    d[y * (size_t)stride + x] = val;
}

/* Rectangle copy: width x rows words at the row stride. */
__kernel void flow_d_copy_rect(__global uint* d, __global const uint* s, uint width, uint rows, uint stride) {
    size_t g = get_global_id(0);
    if (g >= (size_t)width * (size_t)rows) {
        return;
    }
    size_t y = g / (size_t)width;
    size_t x = g % (size_t)width;
    size_t i = y * (size_t)stride + x;
    d[i] = s[i];
}

/* Prefix copy of n words. */
__kernel void flow_d_copy_prefix(__global uint* d, __global const uint* s, uint n) {
    size_t g = get_global_id(0);
    if (g >= (size_t)n) {
        return;
    }
    d[g] = s[g];
}

/* Row compaction: width x rows words from a plane of row stride `stride` into a plane of row stride `width`. */
__kernel void flow_d_compact(__global uint* d, __global const uint* s, uint width, uint rows, uint stride) {
    size_t g = get_global_id(0);
    if (g >= (size_t)width * (size_t)rows) {
        return;
    }
    size_t y = g / (size_t)width;
    size_t x = g % (size_t)width;
    d[g] = s[y * (size_t)stride + x];
}
"#;
