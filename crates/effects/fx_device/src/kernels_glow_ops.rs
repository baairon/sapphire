//! Device kernels of `screen`, `overlay_super` and `alpha_from_rgb_max`. OpenCL C 1.2 as Rust string constants, registered in
//! [`crate::kernels::SOURCES`].
//!
//! Each kernel computes an output sample with the operations of the CPU function it ports, in the CPU's order. Operand
//! shapes (constant or plane operand, channel clamps, the channel 3 extension, the alpha branch) are resolved on the host,
//! exactly as the CPU kernels resolve them. Selects follow the x86 operand order (`maxss(a, b)` is `(a > b) ? a : b`).
//! Operations the CPU writes with the x86 NaN rule (`fx_common::nanop`: the first NaN operand in instruction order,
//! quieted, else the default NaN 0xffc00000) use the shared `nanx_` helpers of [`crate::kernels_pointwise`] (`nanx_add`,
//! `nanx_sub`, `nanx_mul` in `POINTWISE_PW`, `nanx_pick3` in `POINTWISE_FMA`), which precede these sources in the program;
//! plain CPU operations stay plain.

/// The `alpha_from_rgb_max` kernels (no fusion).
pub const GLOW_OPS_PW: &str = r#"
/* alpha_from_rgb_max, one channel input: out3 = imul(scale, c2), c2 being the copy of channel 0. */
__kernel void glow_afrm_mul_f32(__global const float* c2, __global float* out, uint n, uint sbits) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    out[i] = nanx_mul(as_float(sbits), c2[i]);
}

/* alpha_from_rgb_max: m = imax(imax(c0, c1), c2) over the copied channels; out3 = m, or imul(scale, m) when use_scale. */
__kernel void glow_afrm_max_f32(__global const float* c0, __global const float* c1, __global const float* c2, __global float* out,
                                uint n, uint sbits, uint use_scale) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float x0 = c0[i];
    float x1 = c1[i];
    float x2 = c2[i];
    float m = (x0 > x1) ? x0 : x1;
    m = (m > x2) ? m : x2;
    out[i] = (use_scale != 0u) ? nanx_mul(as_float(sbits), m) : m;
}
"#;

/// `screen` and `overlay_super`. Each fma below stands for one `mul_add` of the CPU function named in the marker.
/// MIRRORS_MUL_ADD: screen::screen (fma231), overlay_super::overlay (vfnmadd231ss)
pub const GLOW_OPS_FMA: &str = r#"
/* MIRRORS_MUL_ADD: screen::screen (fma231: (1 - a) * b + a), overlay_super::overlay (vfnmadd231ss: (-a) * b + a) */

/* screen: om = 1 - a (plain), then fma231(a, om, b) = om * b + a in one rounding, NaN priority a, om, b. */
float glow_screen(float a, float b) {
    float om = 0x1p+0f - a;
    float r = fma(om, b, a);
    return (r == r) ? r : nanx_pick3(a, om, b);
}

/* mode 1: A constant (ca), B plane; mode 2: A plane, B constant (cb); else both planes. */
__kernel void glow_screen_f32(__global const float* pa, __global const float* pb, __global float* out, uint n, uint mode, uint ca, uint cb) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float r;
    if (mode == 1u) {
        r = glow_screen(as_float(ca), pb[i]);
    } else if (mode == 2u) {
        r = glow_screen(pa[i], as_float(cb));
    } else {
        r = glow_screen(pa[i], pb[i]);
    }
    out[i] = r;
}

/* screen channel 3: side != 0 (counts 3 and 4) is the fill with one (the vector constant is one); side == 0 (counts 4 and 3)
   is (1 + x) - x with the NaN rule, x from A's plane 3 or A's constant lane 3. */
__kernel void glow_screen_ext_f32(__global const float* src, __global float* out, uint n, uint side, uint is_const, uint cbits) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float r;
    if (side != 0u) {
        r = 0x1p+0f;
    } else {
        float x = (is_const != 0u) ? as_float(cbits) : src[i];
        r = nanx_sub(nanx_add(0x1p+0f, x), x);
    }
    out[i] = r;
}

/* overlay_super element: the low branch (b not above one half, NaN b included), the maximum branch, the blend branch. */
float glow_overlay(float a, float b, uint b_first) {
    if (!(b > 0x1p-1f)) {
        float p = (b_first != 0u) ? nanx_mul(b, a) : nanx_mul(a, b);
        return nanx_add(p, p);
    }
    if (a > 0x1p+0f || b > 0x1p+0f) {
        return (a > b) ? a : b;
    }
    float t;
    if (a != a) {
        t = as_float(as_uint(a) | 0x400000u);
    } else {
        float r = fma(-a, b, a);
        t = (r == r) ? r : as_float(0xffc00000u);
    }
    float d = b - 0x1p+0f;
    float s = b + d;
    return (s + t) + t;
}

/* mode 1: A constant (ca, the product takes b first), B plane; mode 2: A plane, B constant (cb); else both planes. */
__kernel void glow_overlay_f32(__global const float* pa, __global const float* pb, __global float* out, uint n, uint mode, uint ca, uint cb) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float r;
    if (mode == 1u) {
        r = glow_overlay(as_float(ca), pb[i], 1u);
    } else if (mode == 2u) {
        r = glow_overlay(pa[i], as_float(cb), 0u);
    } else {
        r = glow_overlay(pa[i], pb[i], 0u);
    }
    out[i] = r;
}

/* overlay_super channel 3: side != 0 (counts 3 and 4) from B's plane 3 or constant lane 3; side == 0 (counts 4 and 3) from A's. */
__kernel void glow_overlay_ext_f32(__global const float* src, __global float* out, uint n, uint side, uint is_const, uint cbits) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float x = (is_const != 0u) ? as_float(cbits) : src[i];
    float one = 0x1p+0f;
    float r;
    if (side != 0u) {
        if (!(x > 0x1p-1f)) {
            r = x + x;
        } else if (x > one) {
            r = (one > x) ? one : x;
        } else {
            float dd = x - one;
            float nd = -dd;
            r = ((x + dd) + nd) + nd;
        }
    } else {
        r = (x > one) ? x : one;
    }
    out[i] = r;
}
"#;
