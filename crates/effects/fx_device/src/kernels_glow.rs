//! Device kernel source of the glow bright pass (`glow_layer::bright`): one work item per sample, in place on the copied planes.
//! OpenCL C 1.2 as a Rust string constant; registered in [`crate::kernels::SOURCES`].
//!
//! Exactness rules (see the crate docs): `FP_CONTRACT OFF` (prelude), no `fma` (the CPU pass has no fused multiply-add), one
//! correctly rounded division (build option and probe), every compare written in the CPU's form so NaN takes the same branch,
//! `maxss(r, 0)` as the select `(r > 0) ? r : 0`. The binary operations use the two-NaN rule helpers `nanx_add`, `nanx_sub`,
//! `nanx_mul` and `nanx_div` of [`crate::kernels_pointwise::POINTWISE_PW`], which precedes this source in the program.
//! The scale factor `k` and the thresholds are computed on the host by `glow_layer::bright::scale`.
//! The blur passes after the bright pass are the neighbourhood blur kernels.

/// Glow bright pass, one plane and three planes.
pub const GLOW_BRIGHT: &str = r#"
/* glow_layer::bright::one_plane: zero when t >= x or 0 >= x (ordered compares, NaN passes), else min-like select of
   d = k * (x - t) and x. */
__kernel void glow_bright1_f32(__global float* p, uint n, float t, float k) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float x = p[i];
    if (t >= x || 0x0p+0f >= x) {
        p[i] = 0x0p+0f;
        return;
    }
    float d = nanx_mul(k, nanx_sub(x, t));
    p[i] = (x > d) ? d : x;
}

/* glow_layer::bright::three_plane: excess e over the thresholds, colour sum s, scale f = (k * e) / s when s > k * e. */
__kernel void glow_bright3_f32(__global float* p0, __global float* p1, __global float* p2, uint n, float t0, float t1, float t2, float k) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float r = p0[i];
    float g = p1[i];
    float b = p2[i];
    float e = (r > t0) ? nanx_sub(r, t0) : 0x0p+0f;
    if (g > t1) {
        e = nanx_add(e, nanx_sub(g, t1));
    }
    if (b > t2) {
        e = nanx_add(e, nanx_sub(b, t2));
    }
    if (!(e > 0x0p+0f)) {
        p0[i] = 0x0p+0f;
        p1[i] = 0x0p+0f;
        p2[i] = 0x0p+0f;
        return;
    }
    float mr = (r > 0x0p+0f) ? r : 0x0p+0f;
    float s1 = (g > 0x0p+0f) ? nanx_add(g, mr) : mr;
    float s = (b > 0x0p+0f) ? nanx_add(b, s1) : s1;
    if (!(s > 0x0p+0f)) {
        p0[i] = 0x0p+0f;
        p1[i] = 0x0p+0f;
        p2[i] = 0x0p+0f;
        return;
    }
    float tt = nanx_mul(k, e);
    if (s > tt) {
        float f = nanx_div(tt, s);
        p0[i] = nanx_mul(r, f);
        p1[i] = nanx_mul(f, g);
        p2[i] = nanx_mul(f, b);
    }
}
"#;
