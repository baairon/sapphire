//! ESH3: native port of three pool jobs of edge_aware_sharpen's slot 52 (listing addresses; ROUTE AVX2-ONLY):
//!   * `job_2`: AVX2 body behind the dispatcher (job cell). Domain-transform style weight planes whose
//!     horizontal and vertical distances are L1 sums over the colour channels (`nch` planes, at most 4: the job loads four plane pointers).
//!   * `job_cell_a`: scalar job (called from slot 52): two running-sum passes (a -> p0 along lines, t -> p1 along columns).
//!   * `job_3`: AVX2 body behind the dispatcher (called from slot 52): in place fast exp2 based tanh-like
//!     transform of a plane tile.
//!
//! Labels: MEASURED-DYLIB constants (approved original file, read only, see CELLS.tsv); HOST: the partition service (start, count) is an
//! explicit input; the job context is the explicit `Ctx`. No approximate (rcp, rsqrt) instruction exists in these three jobs.
//! CONTRACT-LIMIT (host supplied, listed for Opus): nch <= 4, w >= 1, s >= 1, 0 <= start < total, 0 <= count <= total - start.

/// MEASURED-DYLIB : double 2.687.
pub const D1: f64 = 2.687;
/// MEASURED-DYLIB : double 1e-10, bits 0x3ddb7cdfd9d7bdbb.
pub const LOW: f64 = f64::from_bits(0x3ddb_7cdf_d9d7_bdbb);
/// MEASURED-DYLIB : double 0.9999999999, bits 0x3feffffffff24190.
pub const HIGH: f64 = f64::from_bits(0x3fef_ffff_fff2_4190);
/// MEASURED-DYLIB : 1.0f.
pub const ONE: f32 = 1.0;
/// MEASURED-DYLIB : 2.0f.
pub const TWO: f32 = 2.0;
/// MEASURED-DYLIB : 2^23.
pub const TWO23: f32 = 8388608.0;
/// MEASURED-DYLIB (bits c2fc0000, 42f28c51, 41ddd2fe, 409af5f8, 3fbebc8d, 3fb8aa3b).
pub const C_FLOOR: f32 = f32::from_bits(0xc2fc_0000);
pub const C_A: f32 = f32::from_bits(0x42f2_8c51);
pub const C_B: f32 = f32::from_bits(0x41dd_d2fe);
pub const C_C: f32 = f32::from_bits(0x409a_f5f8);
pub const C_D: f32 = f32::from_bits(0x3fbe_bc8d);
pub const C_LOG2E: f32 = f32::from_bits(0x3fb8_aa3b);

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

/// vmaxss dest, a, b: `a > b ? a : b` (NaN or equal gives b).
#[inline]
pub fn maxss(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

/// vcvttss2si r32: truncation; NaN or out of range gives the integer indefinite 0x80000000.
#[inline]
pub fn cvtt32(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// vcvttss2si r64: truncation; NaN or out of range gives 0x8000000000000000.
#[inline]
pub fn cvtt64(x: f32) -> i64 {
    if x.is_nan() || x >= 9.223372036854775808e18 || x < -9.223372036854775808e18 {
        i64::MIN
    } else {
        x as i64
    }
}

/// One element of job, operation by operation.
pub fn exp_tanh_elem(s0: f32, x: f32) -> f32 {
    let t1 = s0 * x;
    let t1 = C_LOG2E * t1;
    let p = f32::from_bits(t1.to_bits() ^ 0x8000_0000);
    let off = if 0.0f32 > p { ONE } else { 0.0f32 }; // vcomiss xmm12, xmm0 / ja: 0 > p, unordered is not above
    let clipp = maxss(C_FLOOR, p);
    let w = cvtt32(clipp);
    let wf = w as f32; // vcvtsi2ss
    let u = clipp - wf;
    let a1 = C_A + clipp;
    let z0 = u + off;
    let r = C_C - clipp;
    let x3 = r + wf;
    let x4 = x3 - off;
    let q = C_B / x4;
    let v = a1 + q;
    let v = (-z0).mul_add(C_D, v); // vfnmadd231ss: -(z0 * C_D) + v, fused
    let m = TWO23 * v;
    let lo = cvtt64(m) as u64 as u32; // esi
    let d = ONE + f32::from_bits(lo);
    let e = ONE / d;
    e.mul_add(TWO, -ONE) // vfmsub213ss: e * 2 - 1, fused
}

/// Job on the tile (`startcount`) of the plane `x` (in place). `s0` = ctx. CONTRACT-LIMIT: 0 <= start, start + count <= x.len.
pub fn job_3(s0: f32, start: i32, count: i32, x: &mut [f32]) -> Result<(), Err> {
    if count <= 0 {
        return Ok(()); // jle : nothing is touched
    }
    if start < 0 || (start as usize) + (count as usize) > x.len() {
        return Err(Err::Contract(format!("start {} count {} len {}", start, count, x.len())));
    }
    let st = start as usize;
    for i in 0..count as usize {
        x[st + i] = exp_tanh_elem(s0, x[st + i]);
    }
    Ok(())
}

/// Job . Phase 1: partition of `s` lines (`t1` = start, count): `p0[line*w] = running sum of a[line*w]`. Phase 2: partition of `w`
/// (`t2`): for `lines` of length `s` starting at `start2*s`: `p1 = running sum of t` (t is the transposed plane with row stride s).
/// Planes are w*s floats. CONTRACT-LIMIT: w >= 1, s >= 1, tiles inside the totals.
pub fn job_cell_a(w: i32, s: i32, t1: (i32, i32), t2: (i32, i32), a: &[f32], t: &[f32], p0: &mut [f32], p1: &mut [f32]) -> Result<(), Err> {
    // a phase whose count is <= 0 does nothing in the original (jle): no condition on its start
    let bad = |t: (i32, i32), total: i32| t.1 > 0 && (t.0 < 0 || t.0 + t.1 > total);
    if w < 1 || s < 1 || bad(t1, s) || bad(t2, w) {
        return Err(Err::Contract(format!("w {} s {} t1 {:?} t2 {:?}", w, s, t1, t2)));
    }
    let (wu, su) = (w as usize, s as usize);
    let n = wu * su;
    if a.len() < n || t.len() < n || p0.len() < n || p1.len() < n {
        return Err(Err::Contract("plane too small".into()));
    }
    for line in (t1.0.max(0) as usize)..((t1.0 + t1.1).max(0) as usize) {
        if t1.1 <= 0 {
            break;
        }
        let base = line * wu;
        let mut acc = 0.0f32;
        for j in 0..wu {
            acc += a[base + j];
            p0[base + j] = acc;
        }
    }
    for col in (t2.0.max(0) as usize)..((t2.0 + t2.1).max(0) as usize) {
        if t2.1 <= 0 {
            break;
        }
        let base = col * su;
        let mut acc = 0.0f32;
        for j in 0..su {
            acc += t[base + j];
            p1[base + j] = acc;
        }
    }
    Ok(())
}

/// The job context of (fields by offset).
#[derive(Clone, Copy, Debug)]
pub struct Ctx {
    pub w: i32,      //  elements per line
    pub s: i32,      //  number of lines (stride of the transposed plane), also the partition total
    pub nch: i32,    //  number of colour planes (the plane pointer array holds four)
    pub flag_cell_b: i32, //  0: guide-plane mode (B); non zero: constant mode (A)
    pub f0: f32,     //  scale of the horizontal distance (d)
    pub f4: f32,     //
    pub f8: f32,     //  scale of the vertical distance (e)
    pub fc: f32,     //
    pub esh2_job_ctx_a: f32,   // (mode A only)
    pub k: f32,      // [ctx]
}

/// cA = (D1 * (double)(1 + f4)) as f32, cB likewise with fc (vcvtss2sd, vmulsd, vcvtsd2ss).
#[inline]
pub fn c_a(f4: f32) -> f32 {
    (D1 * ((ONE + f4) as f64)) as f32
}

/// hA(m): m < 1e-10 -> cA; m > 0.9999999999 -> 1; else min(1/m, cA). Compares in double, ordered (NaN falls through to the divide).
#[inline]
pub fn h_a(m: f32, ca: f32) -> f32 {
    let md = m as f64;
    if LOW > md {
        ca
    } else if md > HIGH {
        ONE
    } else {
        minss(ONE / m, ca)
    }
}

/// gT(m): m < 1e-10 -> K*cB; m > 0.9999999999 -> K; else K * min(1/m, cB).
#[inline]
pub fn g_t(m: f32, k: f32, cb: f32, kcb: f32) -> f32 {
    let md = m as f64;
    if LOW > md {
        kcb
    } else if md > HIGH {
        k
    } else {
        k * minss(ONE / m, cb)
    }
}

pub struct Bufs<'a> {
    pub ch: [&'a [f32]; 4],
    pub cur: &'a [f32],
    pub a: &'a mut [f32],
    pub t: &'a mut [f32],
}

/// Sum over the channels of |x - y| (sequential, starting from +0.0, each `acc + |d|`): the scalar chains .
#[inline]
fn l1(ch: &[&[f32]; 4], nch: usize, i: usize, j: usize) -> f32 {
    let mut acc = 0.0f32;
    for c in ch.iter().take(nch) {
        acc += (c[i] - c[j]).abs();
    }
    acc
}

/// Job on the tile (`startcount`). Row 0 (start == 0) and the line loop follow the listing. The vector (vgatherqps) arms of the
/// original need nch >= 8 (sum chains) or nch >= 16 (other chains): beyond the four loaded plane pointers, so they are outside the contract.
pub fn job_2(ctx: &Ctx, start: i32, count: i32, b: &mut Bufs) -> Result<(), Err> {
    if ctx.w < 1 || ctx.s < 1 || ctx.nch < 0 || ctx.nch > 4 || start < 0 || start >= ctx.s || count < 0 || count > ctx.s - start {
        return Err(Err::Contract(format!("w {} s {} nch {} start {} count {}", ctx.w, ctx.s, ctx.nch, start, count)));
    }
    let (w, s, st, nch) = (ctx.w as usize, ctx.s as usize, start as usize, ctx.nch as usize);
    let total = w * s;
    if b.ch.iter().take(nch).any(|c| c.len() < total) || b.a.len() < total || b.t.len() < total || (ctx.flag_cell_b == 0 && b.cur.len() < total) {
        return Err(Err::Contract("plane too small".into()));
    }
    let mode_b = ctx.flag_cell_b == 0;
    let (d, e, k) = (ctx.f0, ctx.f8, ctx.k);
    let ca = c_a(ctx.f4);
    let cb = c_a(ctx.fc);
    let kcb = k * cb;
    let (mut c10, mut c11) = (0.0f32, 0.0f32);
    if !mode_b {
        // 
        c10 = ca;
        let bd = ctx.esh2_job_ctx_a as f64;
        if LOW > bd {
            c11 = k * cb;
        } else if bd > HIGH {
            c10 = ONE;
            c11 = k;
        } else {
            let x = ONE / ctx.esh2_job_ctx_a;
            c10 = minss(x, c10);
            c11 = k * minss(x, cb);
        }
    }
    b.a[st * w] = ONE; // pre-stores 
    b.t[st] = ONE;
    let n = w - 1;
    let mut first_line = 0usize;
    if st == 0 {
        if w > 1 {
            for row in 1..=n {
                b.t[row * s] = ONE;
            }
            for j in 1..=n {
                let sa = l1(&b.ch, nch, j, j - 1);
                let h = if mode_b { h_a(minss(b.cur[j - 1], b.cur[j]), ca) } else { c10 };
                b.a[j] = sa.mul_add(d, h);
            }
        }
        first_line = 1;
    }
    let line0 = st + first_line;
    let lines_end = st + count as usize;
    for line in line0..lines_end.max(line0) {
        let base = line * w;
        let prev = base - w;
        b.a[base] = ONE;
        let s0 = l1(&b.ch, nch, base, prev);
        let g0 = if mode_b { g_t(minss(b.cur[prev], b.cur[base]), k, cb, kcb) } else { c11 };
        b.t[line] = s0.mul_add(e, g0);
        for j in 1..=n {
            let sa = l1(&b.ch, nch, base + j, base + j - 1);
            let sv = l1(&b.ch, nch, base + j, prev + j);
            let (h, g) = if mode_b {
                (h_a(minss(b.cur[base + j - 1], b.cur[base + j]), ca), g_t(minss(b.cur[prev + j], b.cur[base + j]), k, cb, kcb))
            } else {
                (c10, c11)
            };
            b.a[base + j] = sa.mul_add(d, h);
            b.t[j * s + line] = sv.mul_add(e, g);
        }
    }
    Ok(())
}
