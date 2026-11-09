//! Device kernels of the pointwise and elementwise ops: arithmetic, comparisons and selects, lane routing,
//! interpolation and compositing, colour.
//!
//! Every kernel computes each output sample with the operations of the CPU kernel it ports, in the CPU's order. Operand
//! shapes (which operand is a constant, which channel feeds which output) are resolved on the host from the operand
//! counts, exactly as the CPU kernel resolves them, and reach the kernel as a plane pointer or a constant word. `fma()`
//! appears only in sources registered as mirroring a CPU `mul_add` (marker `MIRRORS_MUL_ADD:`).
//! Constants are passed as 32 bit words and reinterpreted, so NaN payloads and signed zeros arrive unchanged.
//!
//! NaN rule: the `nanx_` helpers compute an add, subtract, multiply, divide or fused multiply add and, when the result is a
//! NaN, replace it by the NaN the CPU kernel gives: the first NaN operand in the CPU's operand order, quieted by OR
//! 0x00400000, or the default NaN 0xffc00000 when no operand is a NaN (an invalid operation). The CPU's operand order is
//! the order of the compiled instruction: an add or multiply with a constant operand takes the plane value first, and a
//! fused multiply add `fma(a, b, c)` takes `b`, then `a`, then `c`. So the hardware's own NaN propagation never reaches an
//! output word of a kernel that uses them; the min and max selects return an operand word as it is. The self-check compares
//! these NaN words with the CPU kernel on every device, so a CPU or device with another order keeps its NaN outputs on the CPU.

/// Fill, binary arithmetic with the extension channel, unary bit ops, compare select, median ladders, reductions.
pub const POINTWISE_PW: &str = r#"
__kernel void pointwise_fill_f32(__global uint* dst, uint bits, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    dst[i] = bits;
}

/* the NaN of an SSE scalar operation on x and y: the first NaN operand, quieted, else the default NaN */
float nanx_pick2(float x, float y) {
    if (isnan(x)) {
        return as_float(as_uint(x) | 0x00400000u);
    }
    if (isnan(y)) {
        return as_float(as_uint(y) | 0x00400000u);
    }
    return as_float(0xffc00000u);
}

float nanx_add(float x, float y) {
    float r = x + y;
    return isnan(r) ? nanx_pick2(x, y) : r;
}

float nanx_sub(float x, float y) {
    float r = x - y;
    return isnan(r) ? nanx_pick2(x, y) : r;
}

float nanx_mul(float x, float y) {
    float r = x * y;
    return isnan(r) ? nanx_pick2(x, y) : r;
}

float nanx_div(float x, float y) {
    float r = x / y;
    return isnan(r) ? nanx_pick2(x, y) : r;
}

float pointwise_div(float x, float y) {
    if (y == 0x0p+0f) {
        return nanx_mul(0x1.e848p+19f, x);
    }
    return nanx_div(x, y);
}

/* op: 0 add, 1 sub, 2 mul, 3 div (zero rule), 4 max, 5 min (x86 select), 6 diff (|x - y| by sign mask) */
float pointwise_bin_f(uint op, float x, float y) {
    if (op == 0u) {
        return nanx_add(x, y);
    }
    if (op == 1u) {
        return nanx_sub(x, y);
    }
    if (op == 2u) {
        return nanx_mul(x, y);
    }
    if (op == 3u) {
        return pointwise_div(x, y);
    }
    if (op == 4u) {
        return (x > y) ? x : y;
    }
    if (op == 5u) {
        return (x < y) ? x : y;
    }
    return as_float(as_uint(nanx_sub(x, y)) & 0x7fffffffu);
}

/* mode 0: both planes, 1: a is the constant ca, 2: b is the constant cb; add and mul (commutative, so only the NaN pick
   depends on it) take the plane value as the first operand when the other is a constant */
__kernel void pointwise_bin_f32(__global const float* pa, __global const float* pb, __global float* out, uint n, uint op, uint mode, uint ca, uint cb) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float r;
    if (mode == 1u) {
        if (op == 0u || op == 2u) {
            r = pointwise_bin_f(op, pb[i], as_float(ca));
        } else {
            r = pointwise_bin_f(op, as_float(ca), pb[i]);
        }
    } else if (mode == 2u) {
        r = pointwise_bin_f(op, pa[i], as_float(cb));
    } else {
        r = pointwise_bin_f(op, pa[i], pb[i]);
    }
    out[i] = r;
}

/* extension channel 3 when the operands have 3 and 4 channels; side 1 = B side (r14), 0 = A side (r12) */
__kernel void pointwise_ext_f32(__global const float* src, __global float* out, uint n, uint op, uint side, uint is_const, uint cbits) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float x = (is_const != 0u) ? as_float(cbits) : src[i];
    float one = 0x1p+0f;
    float r;
    if (op == 0u) {
        r = nanx_add(one, x);
    } else if (op == 1u) {
        r = (side != 0u) ? nanx_sub(one, x) : nanx_sub(x, one);
    } else if (op == 2u) {
        r = x;
    } else if (op == 3u) {
        if (side != 0u) {
            if (x == 0x0p+0f) {
                r = as_float(0x49742400u);
            } else {
                r = nanx_div(one, x);
            }
        } else {
            r = x;
        }
    } else if (op == 4u) {
        r = (side != 0u) ? ((one > x) ? one : x) : ((x > one) ? x : one);
    } else if (op == 5u) {
        r = (side != 0u) ? ((one < x) ? one : x) : ((x < one) ? x : one);
    } else {
        float v = (side != 0u) ? nanx_sub(one, x) : nanx_sub(x, one);
        r = as_float(as_uint(v) & 0x7fffffffu);
    }
    out[i] = r;
}

/* op 0 abs, 1 negate: sign bit masks */
__kernel void pointwise_un_f32(__global const uint* src, __global uint* dst, uint n, uint op) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    uint b = src[i];
    dst[i] = (op == 0u) ? (b & 0x7fffffffu) : (b ^ 0x80000000u);
}

/* if+ (kind 0: a > 0) and if> (kind 1: a > b); slot masks: bit0 a, bit1 b, bit2 x, bit3 y are constants (words ca..cy) */
__kernel void pointwise_sel_f32(__global const float* pa, __global const float* pb, __global const float* px, __global const float* py, __global float* out,
                         uint n, uint kind, uint cm, uint ca, uint cb, uint cx, uint cy) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float av = ((cm & 1u) != 0u) ? as_float(ca) : pa[i];
    bool c;
    if (kind == 0u) {
        c = av > 0x0p+0f;
    } else {
        float bv = ((cm & 2u) != 0u) ? as_float(cb) : pb[i];
        c = av > bv;
    }
    if (c) {
        out[i] = ((cm & 4u) != 0u) ? as_float(cx) : px[i];
    } else {
        out[i] = ((cm & 8u) != 0u) ? as_float(cy) : py[i];
    }
}

uint pointwise_lt(float a, float b) {
    return (a < b) ? 0xffffffffu : 0u;
}

float pointwise_andm(float a, uint m) {
    return as_float(as_uint(a) & m);
}

float pointwise_bl(float a, float b, uint m) {
    return ((m >> 31) != 0u) ? b : a;
}

float pointwise_minss(float a, float b) {
    return (a < b) ? a : b;
}

/* shapes: 0 S101, 1 S100, 2 S11, 3 S011, 4 S010, 5 S001, 6 S000 (which of a, b, c are constants), as the CPU ladders */
float pointwise_cl_scalar(uint s, float p0, float p1, float p2, float c0, float c1, float c2) {
    if (s == 0u) {
        float x = p1;
        if (c2 > x) {
            if (x > c0) {
                return x;
            }
            return pointwise_minss(c2, c0);
        } else if (c2 > c0) {
            return c2;
        }
        return pointwise_minss(x, c0);
    }
    if (s == 1u) {
        float x = p1;
        float y = p2;
        if (y > x) {
            if (x > c0) {
                return x;
            }
            return pointwise_minss(y, c0);
        } else if (y > c0) {
            return y;
        }
        return pointwise_minss(x, c0);
    }
    if (s == 2u) {
        float x = p2;
        if (x > c1) {
            if (c1 > c0) {
                return c1;
            }
            return pointwise_minss(x, c0);
        } else if (x > c0) {
            return x;
        }
        return pointwise_minss(c1, c0);
    }
    if (s == 3u) {
        float x = p0;
        if (c2 > c1) {
            if (c1 > x) {
                return c1;
            }
            return pointwise_minss(c2, x);
        } else if (c2 > x) {
            return c2;
        }
        return pointwise_minss(c1, x);
    }
    if (s == 4u) {
        float x = p0;
        float y = p2;
        if (y > c1) {
            if (c1 > x) {
                return c1;
            }
            return pointwise_minss(y, x);
        } else if (y > x) {
            return y;
        }
        return pointwise_minss(c1, x);
    }
    if (s == 5u) {
        float a = p0;
        float b = p1;
        if (c2 > b) {
            if (b > a) {
                return b;
            }
            return pointwise_minss(c2, a);
        } else if (c2 > a) {
            return c2;
        }
        return pointwise_minss(b, a);
    }
    float b = p1;
    float c = p2;
    float a = p0;
    if (c > b) {
        if (b > a) {
            return b;
        }
        return pointwise_minss(c, a);
    } else if (c > a) {
        return c;
    }
    return pointwise_minss(b, a);
}

/* the mask ladders of the vector body; k is the all ones word */
float pointwise_cl_lane(uint s, float p0, float p1, float p2, float c0, float c1, float c2, uint k) {
    if (s == 0u) {
        float x = p1;
        uint m2 = pointwise_lt(c0, c2);
        float v3 = pointwise_minss(c2, c0);
        uint m5 = m2 ^ k;
        uint m14 = pointwise_lt(x, c2);
        uint m10 = pointwise_lt(c0, x);
        uint m9 = m14 & m10;
        uint m11 = k ^ m10;
        float v12 = pointwise_andm(x, m9);
        uint m13 = m14 & m11;
        uint m10b = k ^ m14;
        float v15 = pointwise_bl(v12, v3, m13);
        float v1 = pointwise_minss(x, c0);
        uint m9b = m10b & m2;
        float v11 = pointwise_bl(v15, c2, m9b);
        uint m12 = m10b & m5;
        return pointwise_bl(v11, v1, m12);
    }
    if (s == 1u) {
        float x = p1;
        float y = p2;
        uint m5 = pointwise_lt(c0, x);
        uint m10 = pointwise_lt(x, y);
        float v8 = pointwise_minss(y, c0);
        uint m14 = pointwise_lt(c0, y);
        uint m4 = m10 & m5;
        uint m6 = k ^ m5;
        float v7 = pointwise_andm(x, m4);
        uint m9 = m10 & m6;
        uint m15 = k ^ m10;
        float v11 = pointwise_bl(v7, v8, m9);
        float v1 = pointwise_minss(x, c0);
        uint m13 = m15 & m14;
        uint m14b = k ^ m14;
        float v4 = pointwise_bl(v11, y, m13);
        uint m5b = m15 & m14b;
        return pointwise_bl(v4, v1, m5b);
    }
    if (s == 2u) {
        float x = p2;
        uint m3 = pointwise_lt(c0, c1);
        float v6 = pointwise_minss(c1, c0);
        uint m2 = m3 ^ k;
        uint m13 = pointwise_lt(c1, x);
        float v11 = pointwise_minss(x, c0);
        uint m8 = m13 & m3;
        float v10 = pointwise_andm(c1, m8);
        uint m8b = pointwise_lt(c0, x);
        uint m12 = m13 & m2;
        float v14 = pointwise_bl(v10, v11, m12);
        uint m10 = k ^ m13;
        uint m15 = m10 & m8b;
        float v11b = pointwise_bl(v14, x, m15);
        uint m1 = k ^ m8b;
        uint m12b = m10 & m1;
        return pointwise_bl(v11b, v6, m12b);
    }
    if (s == 3u) {
        float x = p0;
        if (c2 > c1) {
            uint m4 = pointwise_lt(x, c1);
            float v3 = pointwise_minss(c2, x);
            return pointwise_bl(v3, c1, m4);
        }
        uint m4 = pointwise_lt(x, c2);
        float v3 = pointwise_minss(c1, x);
        return pointwise_bl(v3, c2, m4);
    }
    if (s == 4u) {
        float x = p0;
        float y = p2;
        uint m10 = pointwise_lt(c1, y);
        uint m4 = pointwise_lt(x, c1);
        float v7 = pointwise_minss(y, x);
        uint m14 = pointwise_lt(x, y);
        float v0 = pointwise_minss(c1, x);
        uint m3 = m10 & m4;
        uint m5 = k ^ m4;
        float v6 = pointwise_andm(c1, m3);
        uint m8 = m10 & m5;
        uint m15 = k ^ m10;
        float v11 = pointwise_bl(v6, v7, m8);
        uint m13 = m15 & m14;
        uint m14b = k ^ m14;
        float v3 = pointwise_bl(v11, y, m13);
        uint m4b = m15 & m14b;
        return pointwise_bl(v3, v0, m4b);
    }
    if (s == 5u) {
        float a = p0;
        float b = p1;
        uint m11 = pointwise_lt(b, c2);
        uint m5 = pointwise_lt(a, b);
        float v9 = pointwise_minss(c2, a);
        uint m14 = pointwise_lt(a, c2);
        float v0 = pointwise_minss(b, a);
        uint m4 = m11 & m5;
        uint m6 = k ^ m5;
        float v7 = pointwise_andm(b, m4);
        uint m10 = m11 & m6;
        uint m15 = k ^ m11;
        float v12 = pointwise_bl(v7, v9, m10);
        uint m13 = m15 & m14;
        uint m14b = k ^ m14;
        float v4 = pointwise_bl(v12, c2, m13);
        uint m1 = m15 & m14b;
        return pointwise_bl(v4, v0, m1);
    }
    float b = p1;
    float c = p2;
    float a = p0;
    uint m7 = pointwise_lt(b, c);
    uint m2 = pointwise_lt(a, b);
    float v5 = pointwise_minss(c, a);
    uint m13 = pointwise_lt(a, c);
    uint m1 = m7 & m2;
    uint m3 = k ^ m2;
    float v4 = pointwise_andm(b, m1);
    uint m6 = m7 & m3;
    uint m14 = k ^ m7;
    float v8 = pointwise_bl(v4, v5, m6);
    float v2 = pointwise_minss(b, a);
    uint m10 = m14 & m13;
    uint m15 = k ^ m13;
    float v1 = pointwise_bl(v8, c, m10);
    uint m3b = m14 & m15;
    return pointwise_bl(v1, v2, m3b);
}

/* clamp: items below vend run the vector ladders, the rest the scalar ones (a 32 byte aligned destination) */
__kernel void pointwise_clamp_f32(__global const float* p0, __global const float* p1, __global const float* p2, __global float* out,
                           uint n, uint shape, uint vend, uint w0, uint w1, uint w2, uint allones) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float x0 = p0[i];
    float x1 = p1[i];
    float x2 = p2[i];
    float c0 = as_float(w0);
    float c1 = as_float(w1);
    float c2 = as_float(w2);
    if (i < vend) {
        out[i] = pointwise_cl_lane(shape, x0, x1, x2, c0, c1, c2, allones);
    } else {
        out[i] = pointwise_cl_scalar(shape, x0, x1, x2, c0, c1, c2);
    }
}

/* max_element (op 0) and min_element (op 1): left fold over the first `count` planes with the x86 select */
__kernel void pointwise_reduce_f32(__global const float* p0, __global const float* p1, __global const float* p2, __global const float* p3,
                            __global float* out, uint n, uint count, uint op) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float acc = p0[i];
    float v;
    for (int c = 1; c < 4; c++) {
        if ((uint)c >= count) {
            break;
        }
        v = (c == 1) ? p1[i] : ((c == 2) ? p2[i] : p3[i]);
        if (op == 0u) {
            acc = (acc > v) ? acc : v;
        } else {
            acc = (acc < v) ? acc : v;
        }
    }
    out[i] = acc;
}
"#;

/// Kernels without a fused multiply add: range fill, postdivide, the f64 free pieces.
pub const POINTWISE_PW2: &str = r#"
__kernel void pointwise_fillr_f32(__global uint* dst, uint bits, uint lo, uint hi) {
    size_t i = get_global_id(0);
    if (i < lo || i >= hi) {
        return;
    }
    dst[i] = bits;
}

/* postdivide: mode 0 colour with a divide (one or two outputs), 1 colour times the reciprocal (three or four outputs),
   2 the alpha word; pk is the source plane of this output, pa the alpha plane (plane e - 1) */
__kernel void pointwise_postdiv_f32(__global const float* pk, __global const float* pa, __global float* out, uint n, uint mode) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float a = pa[i];
    float x = pk[i];
    bool lt_eps = 0x1p-13f > a;
    bool lt_one = (!lt_eps) && (0x1p+0f > a);
    float r;
    if (mode == 2u) {
        r = (lt_eps || lt_one) ? a : 0x1p+0f;
    } else if (lt_eps) {
        r = 0x0p+0f;
    } else if (lt_one) {
        if (mode == 0u) {
            r = nanx_div(x, a);
        } else {
            float inv = nanx_div(0x1p+0f, a);
            r = nanx_mul(x, inv);
        }
    } else {
        r = x;
    }
    out[i] = r;
}
"#;

/// Kernels that fuse. Each fma below stands for one `mul_add` of the CPU function named in its comment.
/// MIRRORS_MUL_ADD: interp::run lerp, screen_super::screen, comp_rgba::mix, comp_premult::pick, dissolve::blend,
/// sat_scale::pixels, hue_shift::pixel, hue_shift_preserve_luma::sector, monochrome::luma, scale_rgb::snap
pub const POINTWISE_FMA: &str = r#"
/* MIRRORS_MUL_ADD: interp::run lerp, screen_super::screen, comp_rgba::mix, comp_premult::pick, dissolve::blend,
   sat_scale::pixels, hue_shift::pixel, hue_shift_preserve_luma::sector, monochrome::luma, scale_rgb::snap */
typedef struct { float x; float y; float z; } pointwise_f3;

/* the first NaN of three operands in the given order, quieted, else the default NaN */
float nanx_pick3(float a, float b, float c) {
    if (isnan(a)) {
        return as_float(as_uint(a) | 0x00400000u);
    }
    if (isnan(b)) {
        return as_float(as_uint(b) | 0x00400000u);
    }
    if (isnan(c)) {
        return as_float(as_uint(c) | 0x00400000u);
    }
    return as_float(0xffc00000u);
}

/* fma(a, b, c); a NaN result is the first NaN of b, a, c (the order of the CPU's mul_add), quieted, else the default NaN */
float nanx_fma(float a, float b, float c) {
    float r = fma(a, b, c);
    return isnan(r) ? nanx_pick3(b, a, c) : r;
}

/* interp: fma(b - a, t, a); slot masks bit0 a, bit1 b, bit2 t are constants (words) */
__kernel void pointwise_interp_f32(__global const float* pa, __global const float* pb, __global const float* pt, __global float* out,
                            uint n, uint cm, uint wa, uint wb, uint wt) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float va = ((cm & 1u) != 0u) ? as_float(wa) : pa[i];
    float vb = ((cm & 2u) != 0u) ? as_float(wb) : pb[i];
    float vt = ((cm & 4u) != 0u) ? as_float(wt) : pt[i];
    out[i] = fma(vb - va, vt, va);
}

float pointwise_screen(float a, float b) {
    float om = nanx_sub(0x1p+0f, a);
    float t = nanx_fma(om, b, a);
    float inner = (b > t) ? b : t;
    return (a > inner) ? a : inner;
}

__kernel void pointwise_screen_f32(__global const float* pa, __global const float* pb, __global float* out, uint n, uint op, uint mode, uint ca, uint cb) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float r;
    if (mode == 1u) {
        r = pointwise_screen(as_float(ca), pb[i]);
    } else if (mode == 2u) {
        r = pointwise_screen(pa[i], as_float(cb));
    } else {
        r = pointwise_screen(pa[i], pb[i]);
    }
    out[i] = r;
}

__kernel void pointwise_screen_ext_f32(__global const float* src, __global float* out, uint n, uint op, uint side, uint is_const, uint cbits) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float x = (is_const != 0u) ? as_float(cbits) : src[i];
    float one = 0x1p+0f;
    float r;
    if (side != 0u) {
        float m = (x > one) ? x : one;
        r = (one > m) ? one : m;
    } else {
        float y = nanx_sub(nanx_add(one, x), x);
        float inner = (one > y) ? one : y;
        r = (x > inner) ? x : inner;
    }
    out[i] = r;
}

/* comp_rgba: mode 0 mix in single precision, 1 mix with the double precision complement, 2 the alpha channel of the blend routes */
__kernel void pointwise_comp_f32(__global const float* pa, __global const float* pb, __global const float* pt, __global float* out,
                          uint n, uint cm, uint wa, uint wb, uint wt, uint mode) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float a = ((cm & 1u) != 0u) ? as_float(wa) : pa[i];
    float b = ((cm & 2u) != 0u) ? as_float(wb) : pb[i];
    float t = ((cm & 4u) != 0u) ? as_float(wt) : pt[i];
    float r;
    if (mode == 1u) {
        double td = (double)t;
        float omt = (float)(0x1p+0 - td);
        if (td >= 0x1p+0) {
            r = b;
        } else if (0x0p+0 >= td) {
            r = a + b;
        } else {
            r = fma(a, omt, b);
        }
    } else if (mode == 2u) {
        float omt = 0x1p+0f - t;
        if (t >= 0x1p+0f) {
            r = 0x1p+0f;
        } else if (0x0p+0f >= t) {
            r = a;
        } else {
            r = fma(a, omt, b);
        }
    } else {
        float omt = 0x1p+0f - t;
        if (t >= 0x1p+0f) {
            r = b;
        } else if (0x0p+0f >= t) {
            r = a + b;
        } else {
            r = fma(a, omt, b);
        }
    }
    out[i] = r;
}

/* comp_premult pick(); c3 selects the (-t) * a + a form of the constant a, plane b, constant t arm */
__kernel void pointwise_prem_f32(__global const float* pa, __global const float* pb, __global const float* pt, __global float* out,
                          uint n, uint cm, uint wa, uint wb, uint wt, uint c3) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float a = ((cm & 1u) != 0u) ? as_float(wa) : pa[i];
    float b = ((cm & 2u) != 0u) ? as_float(wb) : pb[i];
    float t = ((cm & 4u) != 0u) ? as_float(wt) : pt[i];
    float r;
    if (t >= 0x1p+0f) {
        r = b;
    } else if (t <= 0x0p+0f) {
        r = a + b;
    } else if (c3 != 0u) {
        float x = fma(-t, a, a);
        r = b + x;
    } else {
        r = fma(a, 0x1p+0f - t, b);
    }
    out[i] = r;
}

/* dissolve blend(t, a, b) */
__kernel void pointwise_dissolve_f32(__global const float* pa, __global const float* pb, __global const float* pt, __global float* out,
                              uint n, uint cm, uint wa, uint wb, uint wt) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float a = ((cm & 1u) != 0u) ? as_float(wa) : pa[i];
    float b = ((cm & 2u) != 0u) ? as_float(wb) : pb[i];
    float t = ((cm & 4u) != 0u) ? as_float(wt) : pt[i];
    float r;
    if (t <= 0x0p+0f) {
        r = a;
    } else if (t >= 0x1p+0f) {
        r = b;
    } else {
        r = nanx_fma(t, nanx_sub(b, a), a);
    }
    out[i] = r;
}

/* monochrome and monochrome_a luma in double precision, weights 0.3125 0.5 0.1875 */
__kernel void pointwise_luma_f32(__global const float* pr, __global const float* pg, __global const float* pb, __global float* out, uint n) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    double r = (double)pr[i];
    double g = (double)pg[i];
    double b = (double)pb[i];
    double t = 0x1p-1 * g;
    t = fma(r, 0x1.4p-2, t);
    t = fma(b, 0x1.8p-3, t);
    out[i] = (float)t;
}

/* sat_scale: flags bit0 constant colour (words cr cg cb), bit1 scalar scale (word sh); lw is luma_const of the colour */
double pointwise_sat_lw(double r, double g, double b) {
    double t = 0x1p-1 * g;
    t = fma(r, 0x1.4p-2, t);
    return fma(b, 0x1.8p-3, t);
}

__kernel void pointwise_sat_f32(__global const float* pr, __global const float* pg, __global const float* pb, __global const float* pm,
                         __global float* o0, __global float* o1, __global float* o2,
                         uint n, uint flags, uint cr, uint cg, uint cb, uint sh, uint lbits) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    bool cc = (flags & 1u) != 0u;
    float r = cc ? as_float(cr) : pr[i];
    float g = cc ? as_float(cg) : pg[i];
    float b = cc ? as_float(cb) : pb[i];
    float s = ((flags & 2u) != 0u) ? as_float(sh) : pm[i];
    float l = cc ? as_float(lbits) : (float)pointwise_sat_lw((double)r, (double)g, (double)b);
    float x0;
    float x1;
    float x2;
    if (s == 0x0p+0f) {
        x0 = l;
        x1 = l;
        x2 = l;
    } else if (cc) {
        float t = fma(-s, l, l);
        x0 = fma(s, r, t);
        x1 = fma(s, g, t);
        x2 = fma(s, b, t);
    } else if ((flags & 2u) != 0u) {
        float w1 = 0x1p+0f - s;
        float w = w1 * l;
        x0 = fma(s, r, w);
        x1 = fma(s, g, w);
        x2 = fma(b, s, w);
    } else {
        float t = fma(-l, s, l);
        x0 = fma(s, r, t);
        x1 = fma(s, g, t);
        x2 = fma(b, s, t);
    }
    o0[i] = x0;
    o1[i] = x1;
    o2[i] = x2;
}

float pointwise_mn(float a, float b) {
    return (a < b) ? a : b;
}

float pointwise_mx(float a, float b) {
    return (a > b) ? a : b;
}

pointwise_f3 pointwise_f3_make(float x, float y, float z) {
    pointwise_f3 r;
    r.x = x;
    r.y = y;
    r.z = z;
    return r;
}

/* hue_shift::pixel */
pointwise_f3 pointwise_hue_pixel(float r, float g, float b, float shift) {
    float mn = pointwise_mn(pointwise_mn(r, g), b);
    float mx = pointwise_mx(pointwise_mx(r, g), b);
    float neg_mn = -mn;
    float v;
    float hi;
    float lo;
    if (neg_mn > mx) {
        v = mn;
        r = -r;
        g = -g;
        b = -b;
        hi = neg_mn;
        lo = -mx;
    } else {
        v = mx;
        hi = mx;
        lo = mn;
    }
    float delta = hi - lo;
    if (hi == 0x0p+0f) {
        return pointwise_f3_make(v, v, v);
    }
    float s = delta / hi;
    if (s == 0x0p+0f) {
        return pointwise_f3_make(v, v, v);
    }
    float h;
    if (r == hi) {
        float q = (g - b) / delta;
        float q6 = q + 0x1.8p+2f;
        h = (0x0p+0f > q) ? q6 : q;
    } else if (g == hi) {
        h = 0x1p+1f + (b - r) / delta;
    } else {
        h = 0x1p+2f + (r - g) / delta;
    }
    float x = fma(h, as_float(0x3e2aaaadu), shift);
    float frac;
    if (0x0p+0f > s) {
        s = -s;
        float y = 0x1p-1f + x;
        frac = y - floor(y);
    } else {
        frac = x - floor(x);
    }
    float c = v * s;
    float t6 = 0x1.8p+2f * frac;
    float t = (t6 < 0x1.8p+2f || isnan(t6)) ? t6 : 0x0p+0f;
    int i = cvtt_i32_f(t);
    float f = t - (float)i;
    float omf = 0x1p+0f - f;
    float p = v - c;
    float tq = fma(-omf, c, v);
    float qq = fma(-f, c, v);
    if (i < 3) {
        if (i == 0) {
            return pointwise_f3_make(v, tq, p);
        } else if (i == 1) {
            return pointwise_f3_make(qq, v, p);
        }
        return pointwise_f3_make(p, v, tq);
    } else if (i == 3) {
        return pointwise_f3_make(p, qq, v);
    } else if (i == 4) {
        return pointwise_f3_make(tq, p, v);
    }
    return pointwise_f3_make(v, p, qq);
}

__kernel void pointwise_hue_f32(__global const float* pr, __global const float* pg, __global const float* pb, __global const float* pm,
                         __global float* o0, __global float* o1, __global float* o2,
                         uint n, uint flags, uint cr, uint cg, uint cb, uint sh) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    bool cc = (flags & 1u) != 0u;
    float r = cc ? as_float(cr) : pr[i];
    float g = cc ? as_float(cg) : pg[i];
    float b = cc ? as_float(cb) : pb[i];
    float s = ((flags & 2u) != 0u) ? as_float(sh) : pm[i];
    pointwise_f3 o = pointwise_hue_pixel(r, g, b, s);
    o0[i] = o.x;
    o1[i] = o.y;
    o2[i] = o.z;
}

/* hue_shift_preserve_luma: the scalar_pixel body (the per sample path of every call that is not a constant colour with a scalar shift) */
float pointwise_hsp_luma(float c0, float c1, float c2) {
    float rt = (c0 > 0x0p+0f) ? (0x1.4p-2f * c0) : 0x0p+0f;
    float gt = (c1 > 0x0p+0f) ? (0x1p-1f * c1) : 0x0p+0f;
    float bt = (c2 > 0x0p+0f) ? (0x1.8p-3f * c2) : 0x0p+0f;
    return (rt + gt) + bt;
}

pointwise_f3 pointwise_hsp_pixel(float r, float g, float b, float shift) {
    float lold = pointwise_hsp_luma(r, g, b);
    float mn = pointwise_mn(pointwise_mn(r, g), b);
    float mx = pointwise_mx(pointwise_mx(r, g), b);
    float v;
    float hi;
    float lo;
    float rr;
    float gg;
    float bb;
    if (-mn > mx) {
        v = mn;
        hi = -mn;
        lo = -mx;
        rr = -r;
        gg = -g;
        bb = -b;
    } else {
        v = mx;
        hi = mx;
        lo = mn;
        rr = r;
        gg = g;
        bb = b;
    }
    float delta = hi - lo;
    float o0 = v;
    float o1 = v;
    float o2 = v;
    bool grey = (hi == 0x0p+0f);
    if (!grey) {
        grey = (delta / hi) == 0x0p+0f;
    }
    if (!grey) {
        float s0 = delta / hi;
        float h;
        if (rr == hi) {
            float q = (gg - bb) / delta;
            h = (0x0p+0f > q) ? (q + 0x1.8p+2f) : q;
        } else if (gg == hi) {
            h = 0x1p+1f + (bb - rr) / delta;
        } else {
            h = 0x1p+2f + (rr - gg) / delta;
        }
        float x = fma(h, as_float(0x3e2aaaadu), shift);
        bool sneg = 0x0p+0f > s0;
        float s = sneg ? -s0 : s0;
        float frac;
        if (sneg) {
            float y = x + 0x1p-1f;
            frac = y - floor(y);
        } else {
            frac = x - floor(x);
        }
        float t6 = 0x1.8p+2f * frac;
        float t = (!(t6 >= 0x1.8p+2f)) ? t6 : 0x0p+0f;
        int i = cvtt_i32_f(t);
        float f = t - (float)i;
        float cc = v * s;
        float om = 0x1p+0f - f;
        float a1 = fma(-om, cc, v);
        float a2 = v - cc;
        float a3 = fma(-f, cc, v);
        if (3 > i) {
            if (i == 0) {
                o0 = v;
                o1 = a1;
                o2 = a2;
            } else if (i == 1) {
                o0 = a3;
                o1 = v;
                o2 = a2;
            } else {
                o0 = a2;
                o1 = v;
                o2 = a1;
            }
        } else if (i == 3) {
            o0 = a2;
            o1 = a3;
            o2 = v;
        } else if (i == 4) {
            o0 = a1;
            o1 = a2;
            o2 = v;
        } else {
            o0 = v;
            o1 = a2;
            o2 = a3;
        }
    }
    float lnew = pointwise_hsp_luma(o0, o1, o2);
    if (lold != lnew && lnew > 0x0p+0f) {
        float sc = lold / lnew;
        o0 = o0 * sc;
        o1 = o1 * sc;
        o2 = o2 * sc;
    }
    return pointwise_f3_make(o0, o1, o2);
}

__kernel void pointwise_hsp_f32(__global const float* pr, __global const float* pg, __global const float* pb, __global const float* pm,
                         __global float* o0, __global float* o1, __global float* o2,
                         uint n, uint flags, uint cr, uint cg, uint cb, uint sh) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    bool cc = (flags & 1u) != 0u;
    float r = cc ? as_float(cr) : pr[i];
    float g = cc ? as_float(cg) : pg[i];
    float b = cc ? as_float(cb) : pb[i];
    float s = ((flags & 2u) != 0u) ? as_float(sh) : pm[i];
    pointwise_f3 o = pointwise_hsp_pixel(r, g, b, s);
    o0[i] = o.x;
    o1[i] = o.y;
    o2[i] = o.z;
}

/* round (kind 0 snap(x, y), 1 snap_one_over(y), 2 snap_unit(x)); mode 0 both planes, 1 x is the constant ca, 2 y is the constant cb */
float pointwise_snap(float x, float y) {
    if (y == 0x0p+0f) {
        return x;
    }
    double yd = (double)y;
    if (!(x < 0x0p+0f)) {
        float q = floor(x / y);
        return (float)(yd * (double)q);
    }
    float q = ceil(x / y);
    return (float)fma((double)q, yd, -(double)fabs(y));
}

float pointwise_snap_one_over(float y) {
    if (y == 0x0p+0f) {
        return 0x1p+0f;
    }
    return (float)((double)y * floor((double)(0x1p+0f / y)));
}

float pointwise_snap_unit(float x) {
    double xd = (double)x;
    if (!(x < 0x0p+0f)) {
        return (float)floor(xd);
    }
    return (float)(ceil(xd) - 0x1p+0);
}

__kernel void pointwise_round_f32(__global const float* pa, __global const float* pb, __global float* out, uint n, uint kind, uint mode, uint ca, uint cb) {
    size_t i = get_global_id(0);
    if (i >= n) {
        return;
    }
    float x = (mode == 1u) ? as_float(ca) : pa[i];
    float y = (mode == 2u) ? as_float(cb) : pb[i];
    float r;
    if (kind == 0u) {
        r = pointwise_snap(x, y);
    } else if (kind == 1u) {
        r = pointwise_snap_one_over(y);
    } else {
        r = pointwise_snap_unit(x);
    }
    out[i] = r;
}
"#;
