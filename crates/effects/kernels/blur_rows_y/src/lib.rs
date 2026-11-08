//! blur_rows_y: native port of the vertical box-window pass K3 of the shared blur core
//! (AVX2 body, head, class id `blur`). Std only, no unsafe.
//!
//! Source of truth: the listing of K3.
//! The pass works on a dense `width x height` f32 plane. Each column is run through
//! `passes` box windows (unnormalised sums, radius A[p] / B[p], optional subpixel blend).
//! Values depend on the scratch buffer alignment only through windows of 64 or more samples
//! (assumption B-A4, see `base_phase`). Single worker (B-A6), cancel polls never abort (B-A1),
//! half-height mode (K2) is out of scope.

use fx_common::nanop::{fma231, fms213, NanOps};

/// Edge rule of the window sums. Other values are rejected (the original asserts, lines).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    /// 0: samples outside the column are zero.
    Zero,
    /// 1: mirror with edge duplication, in[-1-k] = in[k], in[H+k] = in[H-1-k].
    Mirror,
}

impl Edge {
    pub fn from_i32(v: i32) -> Result<Edge, RowsYError> {
        match v {
            0 => Ok(Edge::Zero),
            1 => Ok(Edge::Mirror),
            other => Err(RowsYError::BadEdge(other)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RowsYError {
    BadEdge(i32),
    /// radius must satisfy r >= 0 and (r == 0 or 2*r + 2 <= height), the driver cap (H >> 1) - 1.
    BadRadius { pass: usize, radius: i32, height: usize },
    /// A, B or C shorter than `passes`.
    ShortParams,
    /// src or dst shorter than width * height.
    ShortPlane,
}

/// Job record of one K3 call (SPEC 2d): the explicit stand-in for the global cells.
#[derive(Clone, Debug)]
pub struct PassYJob<'a> {
    pub width: usize,
    pub height: usize,
    /// 0 zero outside, 1 mirror with duplication; anything else is an error.
    pub edge: i32,
    /// number of passes n (the driver gives 1..=3).
    pub passes: usize,
    /// floor radius per pass A[p] (second window, always used).
    pub a: &'a [i32],
    /// ceil radius per pass B[p] (first window, used only when `subpixel` and A != B).
    pub b: &'a [i32],
    /// blend coefficient per pass C[p].
    pub c: &'a [f32],
    /// blend weight frac (f32).
    pub frac: f32,
    /// subpixel flag (nonzero in the original).
    pub subpixel: bool,
    /// ASSUMPTION B-A4: address of the scratch base modulo 32, in bytes (0 = 32 byte aligned).
    pub base_phase: usize,
}

const STRIP: usize = 32;
const PINGPONG: usize = 32;
const BUF_Z: usize = 33;

/// Run the vertical pass: `dst` = K3(`src`). Dense planes of `width * height` floats.
/// `src` and `dst` must be different slices (use `pass_y_inplace` for the in-place case).
pub fn pass_y(job: &PassYJob, src: &[f32], dst: &mut [f32]) -> Result<(), RowsYError> {
    let (w, h) = (job.width, job.height);
    if src.len() < w * h || dst.len() < w * h {
        return Err(RowsYError::ShortPlane);
    }
    let n = job.passes;
    if n > 0 && (job.a.len() < n || job.b.len() < n || job.c.len() < n) {
        return Err(RowsYError::ShortParams);
    }
    let edge = if n > 0 { Some(Edge::from_i32(job.edge)?) } else { None };
    for p in 0..n {
        for &r in &[job.a[p], job.b[p]] {
            if r < 0 || (r > 0 && 2 * (r as usize) + 2 > h) {
                return Err(RowsYError::BadRadius { pass: p, radius: r, height: h });
            }
        }
    }
    if w == 0 || h == 0 {
        return Ok(());
    }
    let stride_bytes = (4 * h + 15) & !15;
    let phase = |idx: usize| (job.base_phase + idx * stride_bytes) & 31;
    // buffer table: 0..31 column buffers, 32 ping-pong, 33 blend buffer (only with subpixel)
    let mut bufs: Vec<Vec<f32>> = (0..34).map(|_| vec![0.0f32; h]).collect();
    let mut slots: [usize; STRIP] = core::array::from_fn(|j| j);
    let mut pp = PINGPONG;
    let mut col0 = 0;
    while col0 < w {
        let ncols = (w - col0).min(STRIP);
        for j in 0..ncols {
            let b = &mut bufs[slots[j]];
            for row in 0..h {
                b[row] = src[row * w + col0 + j];
            }
        }
        let mut r15 = pp;
        for j in 0..ncols {
            let mut r14 = slots[j];
            for p in 0..n {
                let e = edge.unwrap();
                let (ra, rb) = (job.a[p] as usize, job.b[p] as usize);
                let blend = job.subpixel && job.a[p] != job.b[p];
                let inp = std::mem::take(&mut bufs[r14]);
                let mut out = std::mem::take(&mut bufs[r15]);
                if blend {
                    let mut z = std::mem::take(&mut bufs[BUF_Z]);
                    column_window(&inp, &mut z, rb, e, phase(r14));
                    column_window(&inp, &mut out, ra, e, phase(r14));
                    let cp = job.c[p];
                    fx_common::nanop::blend_in_place(&mut out[..h], &z[..h], cp, job.frac);
                    bufs[BUF_Z] = z;
                } else {
                    column_window(&inp, &mut out, ra, e, phase(r14));
                }
                bufs[r14] = inp;
                bufs[r15] = out;
                std::mem::swap(&mut r14, &mut r15);
            }
            slots[j] = r14;
        }
        pp = r15;
        for j in 0..ncols {
            let b = &bufs[slots[j]];
            for row in 0..h {
                dst[row * w + col0 + j] = b[row];
            }
        }
        col0 += STRIP;
    }
    Ok(())
}

/// In-place variant (the driver Y pass reads the X result from dst): copies the plane first.
pub fn pass_y_inplace(job: &PassYJob, plane: &mut [f32]) -> Result<(), RowsYError> {
    let tmp: Vec<f32> = plane.to_vec();
    pass_y(job, &tmp, plane)
}

fn reduce8(v: [f32; 8]) -> f32 {
    let h = [v[0].fadd(v[4]), v[1].fadd(v[5]), v[2].fadd(v[6]), v[3].fadd(v[7])];
    let t0 = h[0].fadd(h[2]);
    let t1 = h[1].fadd(h[3]);
    t0.fadd(t1)
}

fn vadd(a: [f32; 8], inp: &[f32], at: usize) -> [f32; 8] {
    core::array::from_fn(|i| a[i].fadd(inp[at + i]))
}

/// S0 = sum of the first `len` samples (mode 1, mode 0), exact summation order.
/// `phase` = address of inp[0] modulo 32 in bytes.
pub fn prefix_sum(inp: &[f32], len: usize, edge: Edge, phase: usize) -> f32 {
    let a = phase & 31;
    let mut peel = 0usize;
    let mut vec_ok = len >= 64;
    if vec_ok && a != 0 {
        if a & 3 != 0 {
            vec_ok = false;
        } else {
            peel = (32 - a) / 4;
        }
    }
    if vec_ok && len < peel + 64 {
        vec_ok = false;
    }
    let mut acc = 0.0f32;
    if !vec_ok {
        for i in 0..len {
            acc = acc.fadd(inp[i]);
        }
        return 0.0f32.fadd(acc);
    }
    for i in 0..peel {
        acc = acc.fadd(inp[i]);
    }
    let end = len - ((len - peel) & 63);
    let nblk = (end - peel) / 64;
    let rem = len - end;
    let nb8 = rem / 8;
    let total;
    match edge {
        Edge::Mirror => {
            let mut x = [0.0f32; 8];
            let mut y = [0.0f32; 8];
            for b in 0..nblk {
                let base = peel + 64 * b;
                x = vadd(x, inp, base);
                y = vadd(y, inp, base + 8);
                x = vadd(x, inp, base + 16);
                y = vadd(y, inp, base + 24);
                x = vadd(x, inp, base + 32);
                y = vadd(y, inp, base + 40);
                x = vadd(x, inp, base + 48);
                y = vadd(y, inp, base + 56);
            }
            for k in 0..nb8 {
                x = vadd(x, inp, end + 8 * k);
            }
            let v: [f32; 8] = core::array::from_fn(|i| x[i].fadd(y[i]));
            total = reduce8(v);
        }
        Edge::Zero => {
            let mut s = [[0.0f32; 8]; 8];
            for b in 0..nblk {
                let base = peel + 64 * b;
                for k in 0..8 {
                    s[k] = vadd(s[k], inp, base + 8 * k);
                }
            }
            let s01: [f32; 8] = core::array::from_fn(|i| s[0][i].fadd(s[1][i]));
            let s23: [f32; 8] = core::array::from_fn(|i| s[2][i].fadd(s[3][i]));
            let s45: [f32; 8] = core::array::from_fn(|i| s[4][i].fadd(s[5][i]));
            let s67: [f32; 8] = core::array::from_fn(|i| s[6][i].fadd(s[7][i]));
            let l: [f32; 8] = core::array::from_fn(|i| s01[i].fadd(s23[i]));
            let r: [f32; 8] = core::array::from_fn(|i| s45[i].fadd(s67[i]));
            let mut v: [f32; 8] = core::array::from_fn(|i| l[i].fadd(r[i]));
            for k in 0..nb8 {
                v = vadd(v, inp, end + 8 * k);
            }
            total = reduce8(v);
        }
    }
    for i in (end + 8 * nb8)..len {
        acc = acc.fadd(inp[i]);
    }
    total.fadd(acc)
}

/// One window pass over a column: out[i] = sum of inp over [i-r, i+r] with the edge rule,
/// as the listing sliding sum (S0, head, middle, tail with their pair groupings).
/// Preconditions: out.len() == inp.len() == H, r >= 0, r == 0 or 2r + 2 <= H (driver cap).
/// `phase` = address of inp[0] modulo 32 (bytes), only used for windows of 64 or more samples.
pub fn column_window(inp: &[f32], out: &mut [f32], r: usize, edge: Edge, phase: usize) {
    let h = inp.len();
    assert!(out.len() == h && h > 0 && (r == 0 || 2 * r + 2 <= h));
    let s0 = prefix_sum(inp, r + 1, edge, phase);
    let mir = edge == Edge::Mirror;
    // vfmsub213ss: 2.0 * S0 - in[r], one rounding
    let mut y = if mir { fms213(2.0, s0, inp[r]) } else { s0 };
    // head: out[0..r)
    let p = r >> 1;
    for k in 0..p {
        out[2 * k] = y;
        let a = inp[r + 1 + 2 * k];
        if mir {
            let s = inp[r - 1 - 2 * k];
            let y1 = a.fsub(s).fadd(y);
            out[2 * k + 1] = y1;
            let a2 = inp[r + 2 + 2 * k];
            let s2 = inp[r - 2 - 2 * k];
            y = y1.fadd(a2).fsub(s2);
        } else {
            let y1 = y.fadd(a);
            out[2 * k + 1] = y1;
            y = y1.fadd(inp[r + 2 + 2 * k]);
        }
    }
    if r > 2 * p {
        out[2 * p] = y;
        let a = inp[r + 2 * p + 1];
        y = if mir { y.fadd(a).fsub(inp[r - 2 * p - 1]) } else { y.fadd(a) };
    }
    // middle: out[r .. M), M = h - r - 1
    let m = h - r - 1;
    let n = h - 2 * r - 1;
    if r < m {
        let q = n >> 1;
        for k in 0..q {
            out[r + 2 * k] = y;
            let a = inp[2 * r + 1 + 2 * k];
            let s = inp[2 * k];
            let y1 = a.fsub(s).fadd(y);
            out[r + 2 * k + 1] = y1;
            let s2 = inp[2 * k + 1];
            let a2 = inp[2 * r + 2 + 2 * k];
            y = y1.fsub(s2).fadd(a2);
        }
        if n > 2 * q {
            out[r + 2 * q] = y;
            let s = inp[2 * q];
            let a = inp[2 * r + 1 + 2 * q];
            y = y.fsub(s).fadd(a);
        }
    }
    // tail: out[M ..= H-1], r + 1 outputs; the removal index starts at hn = M - r
    let hn = m - r.min(m);
    let p2 = (r + 1) >> 1;
    for k in 0..p2 {
        out[m + 2 * k] = y;
        let s = inp[hn + 2 * k];
        if mir {
            let a = inp[h - 1 - 2 * k];
            let y1 = a.fsub(s).fadd(y);
            out[m + 2 * k + 1] = y1;
            let a2 = inp[h - 2 - 2 * k];
            let s2 = inp[hn + 2 * k + 1];
            y = y1.fadd(a2).fsub(s2);
        } else {
            let y1 = y.fsub(s);
            out[m + 2 * k + 1] = y1;
            y = y1.fsub(inp[hn + 2 * k + 1]);
        }
    }
    if r + 1 > 2 * p2 {
        out[m + 2 * p2] = y;
    }
}
