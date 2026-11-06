//! native port of pool job (listing address; AVX2 body behind the dispatcher, ROUTE AVX2-ONLY).
//!
//! The job computes two "weight" planes for a recursive edge-aware filter (domain-transform style):
//!   * `a`: a line-major plane, `a[line * w + j]` (element 0 of each line is 1.0f);
//!   * `t`: the transposed plane, `t[row * s + line]` (row = element index j, line = column), `t[0][0]` and column 0 are 1.0f for start == 0.
//! Inputs are three line-major planes `p0`, `p1` (absolute derivative magnitudes, read through |x|) and `cur` (the guide plane; read only when
//! `flag_cell_b == 0`).
//!
//! Labels used below: MEASURED-DYLIB constants (approved original file, read only); APPROX-MODEL (vrcpps modelled as the exact f32 1/x on both
//! sides); ALIGN-DEPENDENT (the vector/scalar split of a line depends on the address of the `cur` line, explicit input `cur_addr`);
//! HOST: the partition service (start, count) is an explicit input; the job context is the explicit `Ctx`.
//!
//! Every arithmetic site follows the listing: f64 compares and scaling constants, `vminss` operand order (NaN yields the second operand),
//! FMA only where the listing fuses (`vfmadd*`), the reciprocal-Newton step `2r - (r*m)*r` in vector lanes, `1/m` (vdivss) in scalar lanes.

/// MEASURED-DYLIB (captured): double 2.687.
pub const D1: f64 = 2.687;
/// MEASURED-DYLIB : double 1e-10, bits 0x3ddb7cdfd9d7bdbb.
pub const LOW: f64 = f64::from_bits(0x3ddb_7cdf_d9d7_bdbb);
/// MEASURED-DYLIB : double 0.9999999999, bits 0x3feffffffff24190.
pub const HIGH: f64 = f64::from_bits(0x3fef_ffff_fff2_4190);
/// MEASURED-DYLIB : 1.0f.
pub const ONE: f32 = 1.0;

/// Lane kind that processes one element of a line (scalar loop, 8-wide ymm main loop, 4-wide xmm loop).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Lane {
    Scalar,
    Ymm,
    Xmm,
}

/// The job context (fields by offset) plus the K cell `[ctx]`.
#[derive(Clone, Copy, Debug)]
pub struct Ctx {
    pub w: i32,      //  elements per line
    pub s: i32,      //  number of lines (stride of the transposed plane), also the partition total
    pub flag_cell_b: i32, //  0: guide-plane mode (B); non zero: constant mode (A)
    pub f0: f32,     //  scale of |p0|
    pub f4: f32,     //
    pub f8: f32,     //  scale of |p1|
    pub fc: f32,     //
    pub esh2_job_ctx_a: f32,   // (mode A only)
    pub k: f32,      // [ctx]
}

#[derive(Debug)]
pub enum Err {
    Contract(String),
}

/// vminss dest, a, b: `a < b ? a : b` (NaN or equal gives b).
#[inline]
pub fn minss(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}

/// vrcpps modelled as the exact f32 reciprocal (APPROX-MODEL) followed by the code's Newton step `2r - (r*m)*r`.
#[inline]
pub fn newton(m: f32) -> f32 {
    let r = 1.0f32 / m;
    let t = r * m;
    let t2 = t * r;
    (r + r) - t2
}

/// The same Newton step from a given estimate r (used by the APPROX-BOUND sensitivity test only, never by the port).
pub fn newton_from(r: f32, m: f32) -> f32 {
    let t = r * m;
    let t2 = t * r;
    (r + r) - t2
}

#[inline]
fn recip(m: f32, lane: Lane) -> f32 {
    match lane {
        Lane::Scalar => ONE / m,
        _ => newton(m),
    }
}

/// Scalar constants derived from the context: cA = (D1 * (double)(1 + a)) as f32, cB likewise with fc.
#[inline]
pub fn c_a(ctx: &Ctx) -> f32 {
    (D1 * ((ONE + ctx.f4) as f64)) as f32
}
#[inline]
pub fn c_b(ctx: &Ctx) -> f32 {
    (D1 * ((ONE + ctx.fc) as f64)) as f32
}

/// hA(m): m < 1e-10 -> cA; m > 0.9999999999 -> 1; else min(1/m, cA). Compares in double, ordered (NaN falls through to the reciprocal arm).
#[inline]
pub fn h_a(m: f32, lane: Lane, ca: f32) -> f32 {
    let md = m as f64;
    if LOW > md {
        ca
    } else if md > HIGH {
        ONE
    } else {
        minss(recip(m, lane), ca)
    }
}

/// gT(m): m < 1e-10 -> K*cB; m > 0.9999999999 -> K; else K * min(1/m, cB).
#[inline]
pub fn g_t(m: f32, lane: Lane, k: f32, cb: f32, kcb: f32) -> f32 {
    let md = m as f64;
    if LOW > md {
        kcb
    } else if md > HIGH {
        k
    } else {
        k * minss(recip(m, lane), cb)
    }
}

/// Lane plan of one line of `n` elements whose `cur` line starts at byte address `line_addr` (ALIGN-DEPENDENT, and the
/// row 0 twin). Returns, per element m (0-based; element j = m + 1), the lane kind.
pub fn lane_plan(n: usize, line_addr: u64) -> Vec<Lane> {
    let mut v = vec![Lane::Scalar; n];
    if n < 16 {
        return v;
    }
    let (h, main_end);
    if n < 0x4b0 {
        h = 0usize;
        main_end = n & !15;
    } else {
        let p = (line_addr.wrapping_add(4) & 0x3f) as usize;
        if p == 0 {
            h = 0;
        } else if p & 3 != 0 {
            return v;
        } else {
            h = (0x40 - p) / 4;
        }
        if n < h + 16 {
            return v;
        }
        main_end = n - ((n - h) & 15);
    }
    for x in v.iter_mut().take(main_end).skip(h) {
        *x = Lane::Ymm;
    }
    if n >= main_end + 1 {
        let rest = n - main_end;
        if rest >= 4 {
            let q = rest & !3;
            for x in v.iter_mut().skip(main_end).take(q) {
                *x = Lane::Xmm;
            }
        }
    }
    v
}

pub struct Bufs<'a> {
    pub p0: &'a [f32],
    pub p1: &'a [f32],
    pub cur: &'a [f32],
    pub a: &'a mut [f32],
    pub t: &'a mut [f32],
}

/// Runs the job for the partition tile (`start`, `count`). `cur_addr` is the byte address of `cur[0]` (only its value modulo 64 matters).
/// CONTRACT-LIMIT: w >= 1, s >= 1, 0 <= start < s, 0 <= count <= s - start, slices of at least w*s floats (the original does not check).
pub fn job_1(ctx: &Ctx, start: i32, count: i32, cur_addr: u64, b: &mut Bufs) -> Result<(), Err> {
    if ctx.w < 1 || ctx.s < 1 || start < 0 || start >= ctx.s || count < 0 || count > ctx.s - start {
        return Err(Err::Contract(format!("w {} s {} start {} count {}", ctx.w, ctx.s, start, count)));
    }
    let (w, s, st) = (ctx.w as usize, ctx.s as usize, start as usize);
    let total = w * s;
    if b.p0.len() < total || b.p1.len() < total || b.a.len() < total || b.t.len() < total || (ctx.flag_cell_b == 0 && b.cur.len() < total) {
        return Err(Err::Contract("plane too small".into()));
    }
    let mode_b = ctx.flag_cell_b == 0; // r14d == 1
    let (d, e, k) = (ctx.f0, ctx.f8, ctx.k);
    let ca = c_a(ctx);
    let cb = c_b(ctx);
    let kcb = k * cb;
    // constants of mode A : c10 added to |p0| * d, c11 added to |p1| * e
    let (mut c10, mut c11) = (0.0f32, 0.0f32);
    if !mode_b {
        let ca_a = (D1 * ((ctx.f4 + ONE) as f64)) as f32;
        c10 = ca_a;
        let bd = ctx.esh2_job_ctx_a as f64;
        if LOW > bd {
            c11 = k * ((D1 * ((ctx.fc + ONE) as f64)) as f32);
        } else if bd > HIGH {
            c10 = ONE;
            c11 = k;
        } else {
            let x3 = ONE / ctx.esh2_job_ctx_a;
            let cb_a = (D1 * ((ONE + ctx.fc) as f64)) as f32;
            c10 = minss(x3, c10);
            c11 = k * minss(x3, cb_a);
        }
    }
    // pre-stores : a[start*w] = 1, t[start] = 1
    b.a[st * w] = ONE;
    b.t[st] = ONE;
    let n = w - 1;
    let mut first_line = 0usize;
    if st == 0 {
        if w > 1 {
            for row in 1..=n {
                b.t[row * s] = ONE;
            }
            if !mode_b {
                for j in 1..=n {
                    b.a[j] = b.p0[j].abs().mul_add(d, c10);
                }
            } else {
                let plan = lane_plan(n, cur_addr);
                for j in 1..=n {
                    let m = minss(b.cur[j - 1], b.cur[j]);
                    b.a[j] = b.p0[j].abs().mul_add(d, h_a(m, plan[j - 1], ca));
                }
            }
        }
        first_line = 1;
    }
    let lines_end = st + count as usize;
    let line0 = st + first_line;
    for line in line0..lines_end.max(line0) {
        let base = line * w;
        b.a[base] = ONE;
        if !mode_b {
            b.t[line] = b.p1[base].abs().mul_add(e, c11);
            for j in 1..=n {
                b.a[base + j] = b.p0[base + j].abs().mul_add(d, c10);
                b.t[j * s + line] = b.p1[base + j].abs().mul_add(e, c11);
            }
        } else {
            let prev = base - w;
            let m0 = minss(b.cur[prev], b.cur[base]);
            b.t[line] = b.p1[base].abs().mul_add(e, g_t(m0, Lane::Scalar, k, cb, kcb));
            if w > 1 {
                let plan = lane_plan(n, cur_addr.wrapping_add((4 * base) as u64));
                for j in 1..=n {
                    let lane = plan[j - 1];
                    let ma = minss(b.cur[base + j - 1], b.cur[base + j]);
                    b.a[base + j] = b.p0[base + j].abs().mul_add(d, h_a(ma, lane, ca));
                    let mt = minss(b.cur[prev + j], b.cur[base + j]);
                    b.t[j * s + line] = b.p1[base + j].abs().mul_add(e, g_t(mt, lane, k, cb, kcb));
                }
            }
        }
    }
    Ok(())
}
