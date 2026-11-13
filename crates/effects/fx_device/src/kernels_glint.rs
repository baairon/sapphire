//! Device kernel source of the glint scaled screen blend (`scaled_screen_blend::blend`, the tail of every glint pass).
//! OpenCL C 1.2 as a Rust string constant; registered in [`crate::kernels::SOURCES`] after the neighbourhood kernels, whose
//! `nf_sub`, `nf_mul` and `nf_fma213` it calls.
//!
//! One work item per element, in place on `dst` with `src` a different plane. The route choice `scale == 1.0` is made on the
//! host and passed as `one_route`; a NaN scale takes the scaled route as on the CPU.

/// Glint blend.
pub const GLINT_BLEND: &str = r#"
/* MIRRORS_MUL_ADD: scaled_screen_blend::scaled, scaled_screen_blend::plain */
__kernel void glint_blend_f32(__global float* dst, __global const float* src, uint n, float scale, int one_route) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float s = src[i];
    if (!(s > 0x0p+0f)) {
        return;
    }
    float d = dst[i];
    if (one_route != 0) {
        if (0x1p+0f > s) {
            float omt = nf_sub(0x1p+0f, s);
            float f = nf_fma213(omt, d, s);
            float m = (d > f) ? d : f;
            dst[i] = (s > m) ? s : m;
        } else {
            dst[i] = 0x1p+0f;
        }
    } else {
        float t = nf_mul(scale, s);
        if (0x1p+0f > t) {
            float omt = nf_sub(0x1p+0f, t);
            dst[i] = nf_fma213(omt, d, t);
        } else {
            dst[i] = 0x1p+0f;
        }
    }
}
"#;
