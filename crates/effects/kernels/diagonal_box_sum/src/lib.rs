//! Diagonal box blur of a float plane (Glint diagonal callbacks, AVX2 bodies 0x1902e0 and 0x189de0).
//!
//! The plane is processed along diagonals, four at a time. Each diagonal is gathered into a zero filled row buffer
//! (row length m = min(w, h)), filtered by up to three zero padded box-sum passes of the radii r0, r1, r2 (each clipped to
//! (m - 1) / 2; a radius of 0 or less skips its pass), optionally blended with itself at +-|shift| samples
//! (0.5 * (right + left), a one-sided tap is 0.5 * tap, an uncovered position is +0), and written back
//! scaled by 1 / ((2 r0 + 1)(2 r1 + 1)(2 r2 + 1)).
//!
//! `Diagonal::Anti` is the callback 0x190280 (element step w - 1, scale 1.0 / product as an f64 division rounded to f32).
//! `Diagonal::Main` is the callback 0x189d80 (element step w + 1, scale 1.0f / f32(product)).
//!
//! The arithmetic follows the listing: operation order, the form of each running-sum update, and the order of the row-start
//! window sum, which depends on the byte phase of the row buffer (peel loop, 64-float blocks, 8-float blocks, scalar tail).
//! Every add, subtract and multiply goes through `fx_common::nanop` (first NaN operand in source order, quieted; a fresh NaN
//! is 0xffc00000). Element-wise loops are written as scalar loops.

use fx_common::nanop::NanOps;

/// Which callback is ported.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Diagonal {
    /// Anti-diagonals (i + j constant), element step w - 1.
    Anti,
    /// Main diagonals (j - i constant), element step w + 1.
    Main,
}

/// Errors for inputs the original reads or writes outside the plane for.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoxSumError {
    BadSize,
    BadLength,
    BadBand,
    BadShift,
}

/// Inputs of one callback call.
#[derive(Clone, Debug)]
pub struct Params {
    pub w: i32,
    pub h: i32,
    pub radii: [i32; 3],
    /// Shift of the two-tap blend; 0 disables it.
    pub shift: i32,
    /// Byte phase (address mod 32) of each of the five row buffers.
    pub phases: [u8; 5],
    /// First diagonal and number of diagonals of this thread's band.
    pub start: i32,
    pub count: i32,
    pub kind: Diagonal,
}

/// Stand-in labels of this port.
pub fn assumptions() -> Vec<&'static str> {
    vec![
        "SCRATCH-ALIGNMENT-AS-ARGUMENT: the byte phases of the five row buffers are arguments",
        "THREADS-AS-ARGUMENT: the band (start, count) of the calling thread is an argument",
        "ROW-BUFFERS-NON-NULL: the five row buffers are assumed allocated (the null-buffer variants of the gather and scatter loops are not rebuilt)",
        "NOT-REBUILT: the AVX and SSE arms, the row allocator, the partition helper, the minimum helper and the memset thunk",
    ]
}

const HALF: f32 = 0.5;

/// Normalisation factor.
pub fn scale_factor(radii: [i32; 3], kind: Diagonal) -> f32 {
    let a = (radii[0].wrapping_mul(2).wrapping_add(1)) as f64;
    let b = (radii[1].wrapping_mul(2).wrapping_add(1)) as f64;
    let c = (radii[2].wrapping_mul(2).wrapping_add(1)) as f64;
    let prod = a * b * c;
    match kind {
        Diagonal::Anti => (1.0f64 / prod) as f32,
        Diagonal::Main => 1.0f32 / (prod as f32),
    }
}

type V8 = [f32; 8];

fn vadd(a: &V8, b: &[f32]) -> V8 {
    let mut o = [0f32; 8];
    for i in 0..8 {
        o[i] = a[i].fadd(b[i]);
    }
    o
}

/// Horizontal reduction of the 8-lane accumulator: extract high half, add, movhlps, add, shuffle, add.
fn reduce(z: &V8) -> f32 {
    let q = [z[0].fadd(z[4]), z[1].fadd(z[5]), z[2].fadd(z[6]), z[3].fadd(z[7])];
    let lane_sum_even = q[0].fadd(q[2]);
    let lane_sum_odd = q[1].fadd(q[3]);
    lane_sum_even.fadd(lane_sum_odd)
}

/// Sum of buf[0..n] in the listing's order for a buffer at byte phase `phase`.
fn window_sum(buf: &[f32], n: usize, phase: u8) -> f32 {
    let mut scalar_sum = 0f32;
    let mut z: V8 = [0.0; 8];
    let ph = (phase & 31) as usize;
    let peel = if ph == 0 {
        Some(0)
    } else if ph & 3 != 0 {
        None
    } else {
        Some((32 - ph) / 4)
    };
    let vec_ok = n >= 0x40 && peel.map_or(false, |p| n >= p + 0x40);
    if !vec_ok {
        for i in 0..n {
            scalar_sum = scalar_sum.fadd(buf[i]);
        }
    } else {
        let peel_count = peel.unwrap();
        let e = n - ((n - peel_count) & 0x3f);
        for i in 0..peel_count {
            scalar_sum = scalar_sum.fadd(buf[i]);
        }
        let mut a: [V8; 8] = [[0.0; 8]; 8];
        let mut idx = peel_count;
        while idx < e {
            for j in 0..8 {
                a[j] = vadd(&a[j], &buf[idx + 8 * j..idx + 8 * j + 8]);
            }
            idx += 64;
        }
        let p = vadd(&a[0], &a[1]);
        let q = vadd(&a[2], &a[3]);
        let r = vadd(&a[4], &a[5]);
        let s = vadd(&a[6], &a[7]);
        let u = vadd(&p, &q);
        let v = vadd(&r, &s);
        z = vadd(&u, &v);
        let mut idx = e;
        if n > e {
            let rest = n - e;
            if rest >= 8 {
                let nv = rest & !7;
                let mut b = 0;
                while b < nv {
                    z = vadd(&z, &buf[idx + b..idx + b + 8]);
                    b += 8;
                }
                idx += nv;
            }
            for i in idx..n {
                scalar_sum = scalar_sum.fadd(buf[i]);
            }
        }
    }
    reduce(&z).fadd(scalar_sum)
}

/// One zero padded box-sum pass of radius r over l floats: out[i] = sum of inp[max(0, i-r) ..= min(l-1, i+r)],
/// formed by a running sum (grow, slide, shrink).
fn box_pass(inp: &[f32], out: &mut [f32], l: usize, r: usize, phase: u8) {
    let mut x = window_sum(inp, r + 1, phase);
    for i in 0..r {
        out[i] = x;
        x = x.fadd(inp[r + 1 + i]);
    }
    let slide_len = l - 2 * r - 1;
    if r < l - r - 1 {
        let pairs = slide_len / 2;
        for j in 0..slide_len {
            out[r + j] = x;
            let nw = inp[2 * r + 1 + j];
            let old = inp[j];
            if j % 2 == 0 && j / 2 < pairs {
                x = nw.fsub(old).fadd(x);
            } else {
                x = x.fadd(nw).fsub(old);
            }
        }
    }
    let mut oi = slide_len;
    for i in (l - r - 1)..l {
        out[i] = x;
        if i + 1 < l {
            x = x.fsub(inp[oi]);
        }
        oi += 1;
    }
}

/// Two-tap blend of the first `len` floats at +-a samples.
fn blend(inp: &[f32], out: &mut [f32], len: usize, a: usize) {
    if len <= a {
        for v in out[..len].iter_mut() {
            *v = 0.0;
        }
    } else if len >= 2 * a {
        for i in 0..a {
            out[i] = HALF.fmul(inp[i + a]);
        }
        for i in a..(len - a) {
            out[i] = HALF.fmul(inp[i + a].fadd(inp[i - a]));
        }
        for i in (len - a)..len {
            out[i] = HALF.fmul(inp[i - a]);
        }
    } else {
        for i in 0..(len - a) {
            out[i] = HALF.fmul(inp[i + a]);
        }
        for i in (len - a)..a {
            out[i] = 0.0;
        }
        for i in a..len {
            out[i] = HALF.fmul(inp[i - a]);
        }
    }
}

struct Diag {
    first: usize,
    len: usize,
}

fn diag_of(p: &Params, t: i32) -> Diag {
    let (w, h) = (p.w as i64, p.h as i64);
    let t = t as i64;
    match p.kind {
        Diagonal::Anti => {
            let row0 = t.max(0);
            let col0 = w - 1 + t.min(0);
            let len = (w + t.min(0)).min(h - row0);
            Diag { first: (row0 * w + col0) as usize, len: len.max(0) as usize }
        }
        Diagonal::Main => {
            let col0 = t.max(0);
            let row0 = (-t).max(0);
            let len = (w - col0).min(h - row0);
            Diag { first: (row0 * w + col0) as usize, len: len.max(0) as usize }
        }
    }
}

/// Run the callback over the plane in place.
pub fn box_sum_diagonals(img: &mut [f32], p: &Params) -> Result<(), BoxSumError> {
    if p.w < 1 || p.h < 1 {
        return Err(BoxSumError::BadSize);
    }
    if img.len() != (p.w as usize) * (p.h as usize) {
        return Err(BoxSumError::BadLength);
    }
    if p.count <= 0 {
        return Ok(());
    }
    if p.start < 0 || (p.start as i64 + p.count as i64) > (p.w as i64 + p.h as i64 - 1) {
        return Err(BoxSumError::BadBand);
    }
    if p.shift == i32::MIN || p.shift.abs() > 0x3fff_ffff {
        return Err(BoxSumError::BadShift);
    }
    let m = p.w.min(p.h) as usize;
    let a = p.shift.unsigned_abs() as usize;
    let rc = ((m as i32 - 1) >> 1) as usize;
    let scale = scale_factor(p.radii, p.kind);
    let step: i64 = match p.kind {
        Diagonal::Anti => p.w as i64 - 1,
        Diagonal::Main => p.w as i64 + 1,
    };
    let base_t = match p.kind {
        Diagonal::Anti => p.start - p.w,
        Diagonal::Main => p.start - p.h,
    };
    let mut bufs: [Vec<f32>; 5] = [vec![0.0; m], vec![0.0; m], vec![0.0; m], vec![0.0; m], vec![0.0; m]];
    let mut slot_buf = [1usize, 2, 3, 4];
    let mut spare = 0usize;
    let mut diags: [Diag; 4] = [Diag { first: 0, len: 0 }, Diag { first: 0, len: 0 }, Diag { first: 0, len: 0 }, Diag { first: 0, len: 0 }];
    let mut nmax = 0usize;
    let mut k = 0i32;
    while k < p.count {
        k += 1;
        let cur = ((k - 1) % 4) as usize;
        if cur == 0 {
            for b in bufs.iter_mut() {
                for v in b.iter_mut() {
                    *v = 0.0;
                }
            }
            slot_buf = [1, 2, 3, 4];
            spare = 0;
            nmax = 0;
            for j in 0..4usize {
                if k + j as i32 <= p.count {
                    let d = diag_of(p, base_t + k + j as i32);
                    for t in 0..d.len {
                        bufs[slot_buf[j]][t] = img[(d.first as i64 + t as i64 * step) as usize];
                    }
                    nmax = nmax.max(d.len);
                    diags[j] = d;
                } else {
                    diags[j] = Diag { first: 0, len: 0 };
                }
            }
        }
        // three box-sum passes
        for s in 0..3 {
            let radius_in = p.radii[s];
            if radius_in <= 0 {
                continue;
            }
            let r = (radius_in as usize).min(rc);
            let ib = slot_buf[cur];
            let (inb, outb) = two_bufs(&mut bufs, ib, spare);
            box_pass(inb, outb, m, r, p.phases[ib]);
            slot_buf[cur] = spare;
            spare = ib;
        }
        if p.shift != 0 {
            let ib = slot_buf[cur];
            let (inb, outb) = two_bufs(&mut bufs, ib, spare);
            blend(inb, outb, diags[cur].len, a);
            slot_buf[cur] = spare;
            spare = ib;
        }
        if cur == 3 || k >= p.count {
            for t in 0..nmax {
                for j in 0..4usize {
                    if t < diags[j].len {
                        let v = scale.fmul(bufs[slot_buf[j]][t]);
                        img[(diags[j].first as i64 + t as i64 * step) as usize] = v;
                    }
                }
            }
        }
    }
    Ok(())
}

fn two_bufs(bufs: &mut [Vec<f32>; 5], i: usize, o: usize) -> (&[f32], &mut [f32]) {
    assert!(i != o);
    if i < o {
        let (a, b) = bufs.split_at_mut(o);
        (&a[i], &mut b[0])
    } else {
        let (a, b) = bufs.split_at_mut(i);
        (&b[0], &mut a[o])
    }
}
