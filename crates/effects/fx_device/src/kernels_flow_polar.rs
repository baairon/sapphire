//! Device kernel source of the optical flow polar kernel (port of `optical_flow_solve::fetch::polar`).
//!
//! One work item per output element. The scalar head and tail and the eight lane body are chosen per row by the crate's own
//! split of the width from the 32 byte phase of the row (`flow_b_split` of `kernels_flow_fetch`). The power and exponential steps
//! call `ofb_powf_lane` and `ofb_expf_lane` of `kernels_flow_solve_b`, so this text holds no copy of those lanes and no table
//! value: the crate constants, the lane constants and the lane tables arrive as run time arguments uploaded from the crate.

pub const FLOW_POLAR: &str = r#"
/* MIRRORS_MUL_ADD: optical_flow_solve::fetch (fma213) */

/* fetch.rs rsqrt_model: 1 / sqrt(x), two roundings, the NaN rule of the two operand division */
float flow_c_rsqrt_model(float x) {
    return ofb_div(0x1p+0f, flow_b_sqrt_x86(x));
}

/* fetch.rs sqrt_refine for one lane */
float flow_c_refine(float s, float est, uint abs_mask, uint sub_b, uint fin_b, uint thr_b) {
    float a = as_float(as_uint(s) & abs_mask);
    uint m = (a >= as_float(thr_b)) ? 0xffffffffu : 0u;
    float est_masked = as_float(m & as_uint(est));
    float est_scaled = ofb_mul(s, est_masked);
    float est_sq = ofb_mul(est_scaled, est_masked);
    float est_corr = ofb_sub(est_sq, as_float(sub_b));
    float est_prod = ofb_mul(est_scaled, est_corr);
    return ofb_mul(est_prod, as_float(fin_b));
}

/* fetch.rs polar: element i of row y; the vector form for head <= i < vend, the scalar form elsewhere */
__kernel void flow_c_polar(__global const float* p, __global const float* q, __global float* out,
                         __global const uint* kc, __global const uint* tab,
                         int width, int stride, int row0, int rows, uint phase,
                         uint arg_two_b, uint mul_s_b, uint mul_v_b, uint abs_mask, uint sub_b, uint fin_b, uint thr_b, uint n) {
    uint g = (uint)get_global_id(0);
    if (g >= n) {
        return;
    }
    int k = (int)(g / (uint)width);
    int i = (int)(g - (uint)k * (uint)width);
    int y = (int)((uint)row0 + (uint)k);
    int roff = flow_b_idx(y, stride);
    long ph = ((long)phase + (long)roff * 4) & 31;
    long head; long vend; int all;
    flow_b_split((long)width, 8, 0x4b0, ph, &head, &vend, &all);
    if (all != 0) {
        head = 0;
        vend = 0;
    }
    int vec = (all == 0) && ((long)i >= head) && ((long)i < vend);
    long idx = (long)roff + (long)i;
    float pv = p[idx];
    float p_sq = ofb_mul(pv, pv);
    float qv = q[idx];
    float norm_sq = flow_b_f213(qv, qv, p_sq);
    float norm;
    float mul;
    if (vec != 0) {
        float est = flow_c_rsqrt_model(norm_sq);
        norm = flow_c_refine(norm_sq, est, abs_mask, sub_b, fin_b, thr_b);
        mul = as_float(mul_v_b);
    } else {
        norm = flow_b_sqrt_x86(norm_sq);
        mul = as_float(mul_s_b);
    }
    float r = as_float(ofb_powf_lane(kc, tab, as_uint(norm), arg_two_b));
    float m = ofb_mul(mul, r);
    out[idx] = as_float(ofb_expf_lane(kc, as_uint(m)));
}
"#;
