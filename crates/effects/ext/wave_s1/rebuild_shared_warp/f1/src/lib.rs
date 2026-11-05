//! Port of the shared sampler F1, AVX2 variant (listing offsets). Sections A..G, H not ported.
//! Silicon NaN rule: plain Rust arithmetic, `mul_add` exactly where the listing fuses. Engine statement for the
//! parity claims: the emulator follows the SDM, not silicon.
//!
//! What it does (INFERRED from the listing): a lattice interpolation of a cell of up to `n` channel values. It reads
//! 2x2 bilinear taps from two to three host buffers (acquire / release pair), blends the results
//! with the weights `a` and `b` (D when neither is larger, E when b > a, F when a > b) and stores `n` floats, one per
//! output plane, at column `col`.

use fx_common::x86::{cvttss2si32, fma, minss};
use std::fmt;

/// Constants captured : ONE, HALF, NEG_ONE .
pub const ONE: f32 = 1.0;
pub const HALF: f32 = 0.5;
pub const NEG_ONE: f32 = -1.0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum F1Error {
    /// A table, buffer or output index fell outside what the caller supplied (the original would read or write arbitrary memory).
    Oob,
}
impl fmt::Display for F1Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("index outside the supplied tables, buffers or planes")
    }
}
impl std::error::Error for F1Error {}

/// Struct fields read by F1 (rdx): `n` at, `width_tab` at, `height_tab` at, `t` at,
/// host mode 0 (byte == 0) assumed for the acquire/release pair. No Default: every field is explicit.
pub struct Params {
    pub n: i32,
    pub width_tab: Vec<i32>,
    pub height_tab: Vec<i32>,
    pub t: Vec<f32>,
    /// ecx: nonzero skips the table clamp of the first coordinate pair.
    pub flag_x: i32,
    /// r8d: nonzero skips the table clamp of the second coordinate pair.
    pub flag_y: i32,
}

/// Host buffers: `bufs[i][j]` is the allocation behind acquire(i, j) (mode 0: pointer `[[s]+i*8]+j*8`).
pub struct Lattice {
    pub bufs: Vec<Vec<Vec<f32>>>,
}

/// Register and stack arguments: xmm0..3 = x, y, a, b; esi = col; r9d = i; stack word = j.
#[derive(Clone, Copy, Debug)]
pub struct Call {
    pub x: f32,
    pub y: f32,
    pub a: f32,
    pub b: f32,
    pub col: i32,
    pub i: i32,
    pub j: i32,
}

fn idx<T: Copy>(v: &[T], i: i64) -> Result<T, F1Error> {
    if i < 0 {
        return Err(F1Error::Oob);
    }
    v.get(i as usize).copied().ok_or(F1Error::Oob)
}

///  mode 0: (width_tab[i] + 3) * n, 32 bit wrapping.
fn release(p: &Params, i: i32) -> Result<i32, F1Error> {
    Ok(idx(&p.width_tab, i as i64)?.wrapping_add(3).wrapping_mul(p.n))
}

///  mode 0: returns (buffer, base offset in floats) with base = n + n * (width_tab[i] + 3) (BW2 reading).
fn acquire<'a>(p: &Params, l: &'a Lattice, i: i32, j: i32) -> Result<(&'a [f32], i64), F1Error> {
    let w = idx(&p.width_tab, i as i64)?;
    let base = (p.n.wrapping_add(w.wrapping_add(3).wrapping_mul(p.n))) as i64;
    let row = l.bufs.get(usize::try_from(i).map_err(|_| F1Error::Oob)?).ok_or(F1Error::Oob)?;
    let buf = row.get(usize::try_from(j).map_err(|_| F1Error::Oob)?).ok_or(F1Error::Oob)?;
    Ok((buf.as_slice(), base))
}

#[derive(Clone, Copy, PartialEq)]
enum Form {
    S,
    V,
}

/// Which code path computes element e of an n-wide row (sections B, C, E, F share the shape). `peel` is the scalar
/// peel count for 32 byte alignment of the temp row (B, E, F: 0; C: 4, because the C temp sits at rbp).
fn form_of(e: usize, n: usize, peel: usize) -> Form {
    if n < 16 {
        return Form::S;
    }
    let p = if n >= 29 { peel } else { 0 };
    if n < p + 16 {
        return Form::S;
    }
    let r15 = n - ((n - p) & 15);
    if e < p {
        return Form::S;
    }
    if e < r15 {
        return Form::V;
    }
    let rest = n - r15;
    if rest < 4 {
        return Form::S;
    }
    if e < r15 + (rest & !3) {
        Form::V
    } else {
        Form::S
    }
}

/// One bilinear nest. `fn_`: fraction along the n direction, `fs`: along the stride direction; `first_is_stride`
/// selects which fraction the weight 0 subtracts first (F does, B C E do not).
#[allow(clippy::too_many_arguments)]
fn nest(
    buf: &[f32],
    start: i64,
    n: usize,
    stride: i32,
    fn_: f32,
    fs: f32,
    first_is_stride: bool,
    peel: usize,
) -> Result<Vec<f32>, F1Error> {
    let t = fn_ * fs;
    let w0 = if first_is_stride { ((ONE - fs) - fn_) + t } else { ((ONE - fn_) - fs) + t };
    let w1 = fn_ - t;
    let w2 = fs - t;
    let w3 = t;
    let rd = |o: i64| -> Result<f32, F1Error> {
        if o < 0 {
            return Err(F1Error::Oob);
        }
        buf.get(o as usize).copied().ok_or(F1Error::Oob)
    };
    let mut out = Vec::with_capacity(n);
    let st = stride as i64;
    for e in 0..n {
        let o = start + e as i64;
        let (s0, s1, s2, s3) = (rd(o)?, rd(o + n as i64)?, rd(o + st)?, rd(o + n as i64 + st)?);
        let v = match form_of(e, n, peel) {
            Form::S => {
                let p5 = fma(w1, s1, w3 * s3);
                let p6 = fma(w0, s0, w2 * s2);
                p5 + p6
            }
            Form::V => {
                let v = w1 * s1;
                let v = fma(w0, s0, v);
                let v = fma(w3, s3, v);
                fma(w2, s2, v)
            }
        };
        out.push(v);
    }
    Ok(out)
}

fn clamp_pair(raw: f32, w: i32) -> f32 {
    if NEG_ONE > raw {
        NEG_ONE
    } else {
        minss(w as f32, raw)
    }
}

/// F1 AVX2 variant. `out[k]` is output plane k (k < n); element `col` of each is written.
pub fn sample(p: &Params, l: &Lattice, c: &Call, out: &mut [&mut [f32]]) -> Result<(), F1Error> {
    // A: prologue and clamps
    let n32 = p.n;
    let (i, j) = (c.i, c.j);
    let sx = HALF + c.x;
    let ti = idx(&p.t, i as i64)?;
    let t6_raw = fma(sx, ti, -HALF);
    let t1i = idx(&p.t, i as i64 + 1)?;
    let u_raw = fma(sx, t1i, -HALF);
    let (t6, u) = if p.flag_x == 0 {
        (clamp_pair(t6_raw, idx(&p.width_tab, i as i64)?), clamp_pair(u_raw, idx(&p.width_tab, i as i64 + 1)?))
    } else {
        (t6_raw, u_raw)
    };
    let sy = HALF + c.y;
    let tj = idx(&p.t, j as i64)?;
    let t7_raw = fma(sy, tj, -HALF);
    let t1j = idx(&p.t, j as i64 + 1)?;
    let v_raw = fma(sy, t1j, -HALF);
    let (t7, v) = if p.flag_y == 0 {
        (clamp_pair(t7_raw, idx(&p.height_tab, j as i64)?), clamp_pair(v_raw, idx(&p.height_tab, j as i64 + 1)?))
    } else {
        (t7_raw, v_raw)
    };
    let s3 = ONE + t6;
    let s6 = ONE + t7;
    let s2 = ONE + u;
    let s1 = ONE + v;
    let (p0, p1, p2, p3) = (cvttss2si32(s3), cvttss2si32(s6), cvttss2si32(s2), cvttss2si32(s1));
    let fx_a = s3 - p0 as f32;
    let fy_a = s6 - p1 as f32;
    let fx_b = s2 - p2 as f32;
    let fy_b = s1 - p3 as f32;
    // A7: the temp rows live in 16 byte stack slots (rbp, rbp), so n > 4 overruns into the next slot
    // (MEASURED by T2: channels 4.. differ for n = 5..8). Outside the contract; refused.
    if n32 > 4 {
        return Err(F1Error::Oob);
    }
    let n = if n32 > 0 { n32 as usize } else { 0 };
    let col = c.col as i64;
    let put = |out: &mut [&mut [f32]], k: usize, v: f32| -> Result<(), F1Error> {
        if col < 0 {
            return Err(F1Error::Oob);
        }
        let pl = out.get_mut(k).ok_or(F1Error::Oob)?;
        *pl.get_mut(col as usize).ok_or(F1Error::Oob)? = v;
        Ok(())
    };
    let m1 = |a: i32| a.wrapping_sub(1);
    // B: acquire(i, j) then release(i)
    let (b1, base1) = acquire(p, l, i, j)?;
    let st1 = release(p, i)?;
    let start_b = base1 + m1(p0).wrapping_mul(n32) as i64 + m1(p1).wrapping_mul(st1) as i64;
    let t1 = if n > 0 { nest(b1, start_b, n, st1, fx_a, fy_a, false, 0)? } else { Vec::new() };
    // C: acquire(i+1, j+1)
    let (b2, base2) = acquire(p, l, i.wrapping_add(1), j.wrapping_add(1))?;
    let st2 = release(p, i.wrapping_add(1))?;
    let c_n_off = m1(p2).wrapping_mul(n32);
    let start_c = base2 + c_n_off as i64 + m1(p3).wrapping_mul(st2) as i64;
    let t2 = if n > 0 { nest(b2, start_c, n, st2, fx_b, fy_b, false, 4)? } else { Vec::new() };
    // G: n <= 0 exits after the nests
    if n == 0 {
        return Ok(());
    }
    let (a, b) = (c.a, c.b);
    if a > b {
        // F: acquire(i+1, j)
        let (b4, base4) = acquire(p, l, i.wrapping_add(1), j)?;
        let st4 = release(p, i.wrapping_add(1))?;
        let start_f = base4 + c_n_off as i64 + m1(p1).wrapping_mul(st4) as i64;
        let t3 = nest(b4, start_f, n, st4, fx_b, fy_a, true, 0)?;
        let w1 = ONE - a;
        let w0 = a - b;
        for k in 0..n {
            let v = b * t2[k];
            let v = fma(w1, t1[k], v);
            let v = fma(w0, t3[k], v);
            put(out, k, v)?;
        }
    } else if b > a {
        // E: acquire(i, j+1)
        let (b3, base3) = acquire(p, l, i, j.wrapping_add(1))?;
        let st3 = release(p, i)?;
        let start_e = base3 + m1(p0).wrapping_mul(n32) as i64 + m1(p3).wrapping_mul(st3) as i64;
        let t3 = nest(b3, start_e, n, st3, fx_a, fy_b, false, 0)?;
        let w1 = ONE - b;
        let w0 = b - a;
        for k in 0..n {
            let v = a * t2[k];
            let v = fma(w1, t1[k], v);
            let v = fma(w0, t3[k], v);
            put(out, k, v)?;
        }
    } else {
        // D: lerp of the first two samples by a
        for k in 0..n {
            let v = fma(t2[k] - t1[k], a, t1[k]);
            put(out, k, v)?;
        }
    }
    Ok(())
}

/// Assumptions of this port (ids stable for the README and tests).
pub fn assumptions() -> &'static [(&'static str, &'static str)] {
    &[
        ("A1", "host mode 0 for (byte [s] == 0); mode 1 (tables at) not ported"),
        ("A2", "alignment of the temp rows: rbp = (rsp & ~31) - 16, so B/E/F temps are 32 byte aligned and C temp is 16 mod 32 (peel 4)"),
        ("A3", "section F vector tail wiring (xmm form) taken by analogy to B; confirmed or refuted by T2"),
        ("A4", "n (s) <= 0 skips the nests and stores nothing; negative n treated as 0"),
        ("A5", "any index outside the supplied tables/buffers/planes is an Err (the original reads or writes arbitrary memory)"),
        ("A7", "contract n <= 4 (channels of one pixel): temp rows are 4 floats; n > 4 refused, the n >= 16 vector paths are unreachable in contract (INFERRED from T2 diffs at n = 5..8)"),
        ("A6","MXCSR round to nearest even; cvtsi2ss is an `as f32` cast"),
    ]
}
