//! Device kernel source of the glint diagonal passes: `glint_diag_box::run_body` (one work item per band row) and
//! `diagonal_box_sum::box_sum_diagonals` (one work item per diagonal). OpenCL C 1.2 as a Rust string constant;
//! registered in [`crate::kernels::SOURCES`]. It follows [`crate::kernels_pointwise::POINTWISE_PW`] in the program and uses its
//! `nanx_add`, `nanx_sub` and `nanx_mul` helpers (the two-NaN rule of `fx_common::nanop`).
//!
//! Both crates share three routines, which are one OpenCL function each: the window sum (scalar route, or the vector route chosen by
//! the row phase and n), the running box pass (grow, slide in pairs, shrink) and the two-tap shifted blend. Band rows and diagonals
//! are independent of each other, so a work item runs the CPU's serial order for its own row with private scratch rows in a global
//! scratch buffer; the row phases of the CPU's rotating scratch rows are computed on the host and passed as a table. Loops use constant
//! bounds with a `break`.

/// Glint diagonal passes: the window sum, box pass and blend routines, the glint band-row kernel and its epilogue, the diagonal kernel.
pub const GLINT_DIAG: &str = r#"
/* glint_diag_box::window_sum_scalar and diagonal_box_sum::window_sum (scalar arm): S = +0 + (((+0 + x0) + x1) + ...). */
float gdb_wsum_scalar(__global const float* x, int n) {
    float t = 0x0p+0f;
    for (int i = 0; i < 2147483647; i++) {
        if (i >= n) {
            break;
        }
        t = nanx_add(t, x[i]);
    }
    return nanx_add(0x0p+0f, t);
}

/* glint_diag_box::window_sum_vector (eight lane accumulators over 64 float trips, the fixed tree, wide remainder trips, scalar tail). */
float gdb_wsum_vector(__global const float* x, int n, uint phase) {
    uint ph = phase & 31u;
    int p = (ph == 0u) ? 0 : (int)((32u - ph) >> 2);
    float t = 0x0p+0f;
    for (int i = 0; i < 2147483647; i++) {
        if (i >= p) {
            break;
        }
        t = nanx_add(t, x[i]);
    }
    int trips = (n - p) >> 6;
    int main_end = p + trips * 64;
    float acc[64];
    for (int i = 0; i < 64; i++) {
        acc[i] = 0x0p+0f;
    }
    for (int tr = 0; tr < 2147483647; tr++) {
        if (tr >= trips) {
            break;
        }
        int base = p + tr * 64;
        for (int i = 0; i < 64; i++) {
            acc[i] = nanx_add(acc[i], x[base + i]);
        }
    }
    float v[8];
    for (int l = 0; l < 8; l++) {
        float sa = nanx_add(acc[l], acc[8 + l]);
        float sb = nanx_add(acc[16 + l], acc[24 + l]);
        float sc = nanx_add(acc[32 + l], acc[40 + l]);
        float sd = nanx_add(acc[48 + l], acc[56 + l]);
        v[l] = nanx_add(nanx_add(sa, sb), nanx_add(sc, sd));
    }
    int rem = n - main_end;
    int done = 0;
    if (rem >= 8) {
        int wide = rem & ~7;
        for (int q = 0; q < 2147483647; q++) {
            if (done >= wide) {
                break;
            }
            for (int l = 0; l < 8; l++) {
                v[l] = nanx_add(v[l], x[main_end + done + l]);
            }
            done += 8;
        }
    }
    for (int j = 0; j < 2147483647; j++) {
        if (done + j >= rem) {
            break;
        }
        t = nanx_add(t, x[main_end + done + j]);
    }
    float h0 = nanx_add(v[0], v[4]);
    float h1 = nanx_add(v[1], v[5]);
    float h2 = nanx_add(v[2], v[6]);
    float h3 = nanx_add(v[3], v[7]);
    float ha_sum = nanx_add(h0, h2);
    float hb_sum = nanx_add(h1, h3);
    return nanx_add(nanx_add(ha_sum, hb_sum), t);
}

/* The route selector of both crates: the vector route needs n >= 64, a row phase of 0 or a multiple of 4, and n >= p + 64. */
float gdb_wsum(__global const float* x, int n, uint phase) {
    if (n < 64) {
        return gdb_wsum_scalar(x, n);
    }
    uint ph = phase & 31u;
    long p = 0;
    if (ph != 0u) {
        if ((ph & 3u) != 0u) {
            return gdb_wsum_scalar(x, n);
        }
        p = (long)((32u - ph) >> 2);
    }
    if ((long)n < p + 64) {
        return gdb_wsum_scalar(x, n);
    }
    return gdb_wsum_vector(x, n, phase);
}

/* glint_diag_box::box_pass and diagonal_box_sum::box_pass: s is the window sum of x[0..=r]. */
void gdb_box_pass(__global const float* x, __global float* out, int l, int r, float s) {
    for (int i = 0; i < 2147483647; i++) {
        if (i >= r) {
            break;
        }
        out[i] = s;
        s = nanx_add(s, x[r + 1 + i]);
    }
    long m = (long)l - 2 * (long)r - 1;
    if (m > 0) {
        long paired = 2 * (m / 2);
        for (int k = 0; k < 2147483647; k++) {
            if ((long)k >= m) {
                break;
            }
            out[r + k] = s;
            float x_in = x[2 * r + 1 + k];
            float x_out = x[k];
            if ((k & 1) == 0 && (long)k < paired) {
                s = nanx_add(nanx_sub(x_in, x_out), s);
            } else {
                s = nanx_sub(nanx_add(s, x_in), x_out);
            }
        }
    }
    int mid_start = l - 2 * r - 1;
    for (int j = 0; j < 2147483647; j++) {
        if (j > r) {
            break;
        }
        out[l - r - 1 + j] = s;
        if (j < r) {
            s = nanx_sub(s, x[mid_start + j]);
        }
    }
}

/* glint_diag_box::shift_blend and diagonal_box_sum::blend: 0.5 * (x[i+a] + x[i-a]) with absent taps dropped, +0 where both are absent. */
void gdb_blend(__global const float* x, __global float* out, int l, long a) {
    if ((long)l <= a) {
        for (int i = 0; i < 2147483647; i++) {
            if (i >= l) {
                break;
            }
            out[i] = 0x0p+0f;
        }
        return;
    }
    for (int i = 0; i < 2147483647; i++) {
        if (i >= l) {
            break;
        }
        long hi_i = (long)i + a;
        long lo_i = (long)i - a;
        int hi = hi_i < (long)l;
        int lo = lo_i >= 0;
        float r;
        if (hi && lo) {
            r = nanx_mul(0x1p-1f, nanx_add(x[hi_i], x[lo_i]));
        } else if (hi) {
            r = nanx_mul(0x1p-1f, x[hi_i]);
        } else if (lo) {
            r = nanx_mul(0x1p-1f, x[lo_i]);
        } else {
            r = 0x0p+0f;
        }
        out[i] = r;
    }
}

/* The flipped plane index of body B (rows reversed inside the plane, slack words unchanged). */
long gdb_map(long i, int w, int h, int mirrored) {
    if (mirrored != 0 && i < (long)w * (long)h) {
        return ((long)h - 1 - i / w) * w + i % w;
    }
    return i;
}

#define GDB_RD(I, OUT) { long ri_ = (I); if (ri_ < 0 || ri_ >= slen) { atomic_or(err, 1u); OUT = 0x0p+0f; } else { OUT = src[gdb_map(ri_, w, h, mirrored)]; } }
#define GDB_WR(I, V) { long wi_ = (I); if (wi_ < 0 || wi_ >= dlen) { atomic_or(err, 1u); } else { dst[gdb_map(wi_, w, h, mirrored)] = (V); } }

/* glint_diag_box::run for one band row (counter = first + gid). phases holds six words per counter: the row phase each of the six passes sees. */
__kernel void glint_diag_band_f32(__global const float* src, __global float* dst, __global float* scratch, __global const uint* phases,
                                  __global uint* err, int w, int h, long slen, long dlen, int mirrored, int band_start, long first,
                                  long nitems, int r0, int r1, int r2, long dd, uint fbits, long cap) {
    long gid = (long)get_global_id(0);
    if (gid >= nitems) {
        return;
    }
    long counter = first + gid;
    float f = as_float(fbits);
    long c = counter + 1;
    long t = c + ((long)band_start - (long)h) + 1;
    long a_ = (t > 0) ? t : 0;
    long b_ = (-t > 0) ? -t : 0;
    long l1 = (long)w - 2 * a_;
    long l2 = 2 * (long)h - 2 * b_;
    long l = (l1 < l2) ? l1 : l2;
    long off = (long)(int)((uint)(ulong)((long)w * b_) + (uint)(ulong)(2 * a_));
    if (l <= 0) {
        return;
    }
    int lu = (int)l;
    __global float* ra = scratch + gid * 3 * cap;
    __global float* rb = ra + cap;
    __global float* rc = rb + cap;
    int first_zeroed = 0;
    int last_zeroed = 0;
    int bb = (int)b_;
    float x0;
    float x1;
    GDB_RD(off, x0);
    ra[0] = x0;
    if (b_ == 0) {
        rb[0] = 0x0p+0f;
        first_zeroed = 1;
    } else {
        GDB_RD(off - w, x1);
        rb[0] = nanx_mul(0x1p-1f, nanx_add(x0, x1));
    }
    if (l >= 2) {
        long p = off + 1;
        long j = 1;
        if (l - 1 > 1) {
            long pairs = (l - 1) >> 1;
            for (long k = 0; k < 1073741824; k++) {
                if (k >= pairs) {
                    break;
                }
                float x;
                float y;
                GDB_RD(p, x);
                GDB_RD(p + w, y);
                ra[2 * k + 1] = nanx_mul(0x1p-1f, nanx_add(x, y));
                rb[2 * k + 1] = x;
                p += (long)w + 1;
                j = 2 * k + 2;
                if (j >= l - 1) {
                    break;
                }
                GDB_RD(p, x);
                GDB_RD(p - w, y);
                ra[2 * k + 2] = x;
                rb[2 * k + 2] = nanx_mul(0x1p-1f, nanx_add(x, y));
                p += 1;
                j = 2 * k + 3;
            }
        }
        float x;
        float y;
        if ((j & 1) == 1) {
            GDB_RD(p, x);
            if (b_ + (l >> 1) < (long)h - 1) {
                GDB_RD(p + w, y);
                ra[j] = nanx_mul(0x1p-1f, nanx_add(x, y));
            } else {
                ra[j] = 0x0p+0f;
                last_zeroed = 1;
            }
            rb[j] = x;
        } else {
            GDB_RD(p, x);
            ra[j] = x;
            GDB_RD(p - w, y);
            rb[j] = nanx_mul(0x1p-1f, nanx_add(x, y));
        }
    }
    int clampv = (int)((l - 1) >> 1);
    int radii[3];
    radii[0] = r0;
    radii[1] = r1;
    radii[2] = r2;
    for (int k = 0; k < 3; k++) {
        int radius = radii[k];
        if (radius <= 0) {
            continue;
        }
        int rr = (radius < clampv) ? radius : clampv;
        float sm = gdb_wsum(ra, rr + 1, phases[counter * 6 + k]);
        gdb_box_pass(ra, rc, lu, rr, sm);
        __global float* tmp = ra;
        ra = rc;
        rc = tmp;
        if (last_zeroed != 0) {
            ra[lu - 1] = 0x0p+0f;
        }
    }
    if (dd != 0) {
        gdb_blend(ra, rc, lu, dd);
        __global float* tmp = ra;
        ra = rc;
        rc = tmp;
        if (last_zeroed != 0) {
            ra[lu - 1] = 0x0p+0f;
        }
    }
    for (int k = 0; k < 3; k++) {
        int radius = radii[k];
        if (radius <= 0) {
            continue;
        }
        int rr = (radius < clampv) ? radius : clampv;
        float sm = gdb_wsum(rb, rr + 1, phases[counter * 6 + 3 + k]);
        gdb_box_pass(rb, rc, lu, rr, sm);
        __global float* tmp = rb;
        rb = rc;
        rc = tmp;
        if (first_zeroed != 0) {
            rb[0] = 0x0p+0f;
        }
    }
    if (dd != 0) {
        gdb_blend(rb, rc, lu, dd);
        __global float* tmp = rb;
        rb = rc;
        rc = tmp;
        if (first_zeroed != 0) {
            rb[0] = 0x0p+0f;
        }
    }
    long pos = off;
    for (int q = 0; q < 1073741824; q++) {
        if (q >= lu / 2) {
            break;
        }
        GDB_WR(pos, nanx_mul(f, ra[2 * q]));
        GDB_WR(pos + 1, nanx_mul(f, rb[2 * q + 1]));
        pos += (long)w + 2;
    }
    if ((lu & 1) == 1) {
        GDB_WR(pos, nanx_mul(f, ra[lu - 1]));
    }
}

/* glint_diag_box::run epilogue X (band start 0): the four edge words are +0. */
__kernel void glint_diag_edge_f32(__global const float* src, __global float* dst, __global uint* err, int w, int h, long slen, long dlen,
                                  int mirrored) {
    if (get_global_id(0) != 0) {
        return;
    }
    GDB_WR(((long)h - 1) * w, 0x0p+0f);
    GDB_WR(((long)h - 1) * w + 1, 0x0p+0f);
    GDB_WR((long)w - 1, 0x0p+0f);
    if ((w & 1) == 0) {
        GDB_WR((long)w - 2, 0x0p+0f);
    }
}

/* diagonal_box_sum::box_sum_diagonals for one diagonal (index kk = first + gid), in place on the plane. phases holds three words per
   position in the group of four: the row buffer phase each of the three passes sees. kind 0 is Anti, 1 is Main. */
__kernel void diag_box_sum_f32(__global float* img, __global float* scratch, __global const uint* phases, int w, int h, int m, int kind,
                               int base_t, long a, int r0, int r1, int r2, int shift_nz, uint sbits, int rcl, long first, long nitems) {
    long gid = (long)get_global_id(0);
    if (gid >= nitems) {
        return;
    }
    long kk = first + gid;
    int cur = (int)(kk % 4);
    long t = (long)base_t + kk + 1;
    float scale = as_float(sbits);
    long row0;
    long col0;
    long len;
    long step;
    if (kind == 0) {
        row0 = (t > 0) ? t : 0;
        col0 = (long)w - 1 + ((t < 0) ? t : 0);
        len = (long)w + ((t < 0) ? t : 0);
        long l2 = (long)h - row0;
        len = (len < l2) ? len : l2;
        step = (long)w - 1;
    } else {
        col0 = (t > 0) ? t : 0;
        row0 = (-t > 0) ? -t : 0;
        len = (long)w - col0;
        long l2 = (long)h - row0;
        len = (len < l2) ? len : l2;
        step = (long)w + 1;
    }
    if (len < 0) {
        len = 0;
    }
    if (len == 0) {
        return;
    }
    long start = row0 * w + col0;
    __global float* inb = scratch + gid * 2 * (long)m;
    __global float* outb = inb + m;
    for (int i = 0; i < 2147483647; i++) {
        if (i >= m) {
            break;
        }
        inb[i] = 0x0p+0f;
    }
    for (long i = 0; i < 2147483647; i++) {
        if (i >= len) {
            break;
        }
        inb[i] = img[start + i * step];
    }
    int radii[3];
    radii[0] = r0;
    radii[1] = r1;
    radii[2] = r2;
    for (int s = 0; s < 3; s++) {
        int radius = radii[s];
        if (radius <= 0) {
            continue;
        }
        int r = (radius < rcl) ? radius : rcl;
        float sm = gdb_wsum(inb, r + 1, phases[cur * 3 + s]);
        gdb_box_pass(inb, outb, m, r, sm);
        __global float* tmp = inb;
        inb = outb;
        outb = tmp;
    }
    if (shift_nz != 0) {
        gdb_blend(inb, outb, (int)len, a);
        __global float* tmp = inb;
        inb = outb;
        outb = tmp;
    }
    for (long i = 0; i < 2147483647; i++) {
        if (i >= len) {
            break;
        }
        img[start + i * step] = nanx_mul(scale, inb[i]);
    }
}
"#;
