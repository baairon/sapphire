//! Device kernel source of the tilt_alpha op: kernel `tilt_alpha_f32`, a port of the element loops of `tilt_alpha::tilt_alpha_with`
//! (sections S8 and S9 of the crate header). The host runs the crate itself for the scalar sections (S1 to S5) and passes the results as
//! bit patterns, together with one `(peel, vend)` word pair per row from the crate's `plan`; element `j` of a row takes the vector form
//! (S9) when `peel <= j < vend` and the scalar form (S8) otherwise, the crate's `i >= pl.peel && i < pl.vend`.
//!
//! Word layout of the parameter buffer: 0 signed host scale ls, 1 knt, 2 ksw, 3 cos_ang, 4 a = sinpa * ls, 5 b = cos_ang * ls,
//! 6 -ksw, 7 -sinpa. Every multiply goes through `nanx_mul`; every fused step is the explicit `fma` with the NaN priority
//! (destination, second, third source) of `fx_common::nanop`.

pub const TILT_ALPHA: &str = r#"
/* MIRRORS_MUL_ADD: tilt_alpha::tilt_alpha_with (fms231, fnms213 in the scalar element; fma231, fms213 in the vector element) */

float ta_fz(float r, float d, float s2, float s3) {
    return isnan(r) ? nanx_pick3(d, s2, s3) : r;
}

/* clamp shared by every element: a strictly negative value gives +0, else min(1.0, t) with the minss rule */
float ta_clamp(float t) {
    if (0x0p+0f > t) {
        return 0x0p+0f;
    }
    if (0x1p+0f < t) {
        return 0x1p+0f;
    }
    return t;
}

__kernel void tilt_alpha_f32(__global const float* pa, __global const float* pb, __global float* dst, __global const uint* pr,
                             __global const uint* rows, uint n, uint rowlen) {
    uint i = (uint)get_global_id(0);
    if (i >= n) {
        return;
    }
    float ls = as_float(pr[0]);
    float knt = as_float(pr[1]);
    float ksw = as_float(pr[2]);
    float cos_ang = as_float(pr[3]);
    float ca = as_float(pr[4]);
    float cb = as_float(pr[5]);
    float nksw = as_float(pr[6]);
    float nsin = as_float(pr[7]);
    uint row = i / rowlen;
    uint j = i - row * rowlen;
    uint peel = rows[2u * row];
    uint vend = rows[2u * row + 1u];
    float av = pa[i];
    float bv = pb[i];
    float o;
    if (j >= peel && j < vend) {
        float t = nanx_mul(knt, bv);
        t = ta_fz(fma(nksw, av, t), t, nksw, av);
        t = ta_fz(fma(t, nsin, -cos_ang), t, nsin, cos_ang);
        o = ta_clamp(nanx_mul(ls, t));
    } else {
        float t = nanx_mul(ksw, av);
        t = ta_fz(fma(knt, bv, -t), t, knt, bv);
        o = ta_clamp(ta_fz(fma(-t, ca, -cb), t, ca, cb));
    }
    dst[i] = o;
}
"#;
