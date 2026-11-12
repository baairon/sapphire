//! Glint diagonal box callback, AVX2 body A (listing 0x1a3f30..0x1a5ce4), window-sum scalar and vector routes.
//!
//! Per band row the callback builds two interleaved slices A and B along a zigzag of the source plane (E5), runs up to three
//! box passes on each (P1 on A, P2 on B), an optional shifted two-tap blend (S1 on A, S2 on B, when the shift cell is non-zero),
//! and writes `f * A[2p]`, `f * B[2p+1]` pairs along the destination diagonal (O), with f = 1 / ((2rx+1)(2ry+1)(2rz+1)).
//!
//! Every binary float operation goes through `fx_common::nanop` in the instruction's operand order.
//! The window sum of a pass has a scalar route (V1) and a vector route (V2 to V7), chosen by the listing's gate on n and the row phase.

use fx_common::nanop::NanOps;

/// Cell 0x15e8514 (0.5f); the 0x15e94a0 splat holds eight lanes of the same value.
const HALF: f32 = 0.5;
/// Cell 0x15e8490 (1.0 double).
const ONE: f64 = 1.0;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    /// A source read outside the plane slice.
    ReadRange(i64),
    /// A destination write outside the destination slice.
    WriteRange(i64),
    /// The shift cell is i32::MIN (abs wraps; the listing's sign test at 0x1a4ae4 changes the loop shape).
    ShiftMin,
    /// Plane slice shorter than width * height, or a zero dimension.
    Size,
}

/// Inputs that are cells or services in the original.
#[derive(Debug, Clone, Copy)]
pub struct Params {
    /// Radii pointer cell 0x1de1440: three ints.
    pub radii: [i32; 3],
    /// Cell 0x1de148c: shift (only |d| is read).
    pub d: i32,
}

/// BAND-AS-ARGUMENT: the [start, count] that 0x6701b0 returns.
#[derive(Debug, Clone, Copy)]
pub struct Band {
    pub start: i32,
    pub count: i32,
}

/// SCRATCH-ALIGNMENT-AS-ARGUMENT: (row address & 31) of the ring rows from 0x112f00 slices 0, 1, 2.
#[derive(Debug, Clone, Copy)]
pub struct RowPhases(pub [u32; 3]);

pub fn assumptions() -> &'static [&'static str] {
    &[
        "SCRATCH-ALIGNMENT-AS-ARGUMENT: the byte phase (address & 31) of each ring row is an argument; only the window-sum route selector reads it",
        "BAND-AS-ARGUMENT: the [start, count] of the band partition is an argument",
        "RING-ROWS-OWNED: the three ring rows of 0x112f00 are owned by the port; values outside the first L words of a row are never read",
        "DEGENERATE-ROW-UNREACHED (no served band has L <= 0; width 1 refused by the op)",
        "GLINT-SHIFT-REFUSED (a shift of 2^30 pixels or more, or a shift whose product is out of the integer range, is refused by the op; the callback itself returns Error::ShiftMin at i32::MIN)",
        "SLACK-WRITES: destination words past the plane that the original writes (epilogue, odd widths) land in caller-provided slack",
    ]
}

/// P1.G / P2 gate (0x1a4416..0x1a4453): the window sum takes the vector route only if n >= 64 and the row phase is 0 or a multiple of 4 and n >= p + 64.
pub fn vector_route(n: i32, phase: u32) -> bool {
    if n < 64 {
        return false;
    }
    let ph = phase & 31;
    let p = if ph == 0 {
        0
    } else if ph & 3 != 0 {
        return false;
    } else {
        (32 - ph) >> 2
    };
    if (n as i64) < p as i64 + 64 {
        false
    } else {
        true
    }
}

/// V1: S = +0 + (((+0 + x0) + x1) + ... + x(n-1)).
pub fn window_sum_scalar(x: &[f32], n: usize) -> f32 {
    let mut t = 0.0f32;
    for v in &x[..n] {
        t = t.fadd(*v);
    }
    0.0f32.fadd(t)
}

/// V2 to V7 (vector route; P1 0x1a4465..0x1a4580, P2 0x1a4fb6..0x1a50d0). Precondition: `vector_route(n, phase)`.
/// T = peel (first p elements, index order, accumulator first, from +0), eight lane-wise accumulators over 64-float trips from index p,
/// the tree ((a0 + a1) + (a2 + a3)) + ((a4 + a5) + (a6 + a7)), wide remainder trips of 8 floats added lane-wise (accumulator first) when rem >= 8,
/// then the scalar tail added to T in index order; h = lo128 + hi128; g0 = h0 + h2, g1 = h1 + h3; S = (g0 + g1) + T.
pub fn window_sum_vector(x: &[f32], n: usize, phase: u32) -> f32 {
    let ph = phase & 31;
    let p = if ph == 0 { 0 } else { ((32 - ph) >> 2) as usize };
    let mut t = 0.0f32;
    for v in &x[..p] {
        t = t.fadd(*v);
    }
    let trips = (n - p) >> 6;
    let main_end = p + trips * 64;
    let mut acc = [[0.0f32; 8]; 8];
    let mut base = p;
    while base < main_end {
        for (k, a) in acc.iter_mut().enumerate() {
            for l in 0..8 {
                a[l] = a[l].fadd(x[base + 8 * k + l]);
            }
        }
        base += 64;
    }
    let add_lanes = |u: [f32; 8], v: [f32; 8]| {
        let mut r = [0.0f32; 8];
        for l in 0..8 {
            r[l] = u[l].fadd(v[l]);
        }
        r
    };
    let sum_a = add_lanes(acc[0], acc[1]);
    let sum_b = add_lanes(acc[2], acc[3]);
    let sum_c = add_lanes(acc[4], acc[5]);
    let sum_d = add_lanes(acc[6], acc[7]);
    let mut v = add_lanes(add_lanes(sum_a, sum_b), add_lanes(sum_c, sum_d));
    let rem = n - main_end;
    let mut done = 0usize;
    if rem >= 8 {
        let wide = rem & !7;
        while done < wide {
            let mut c = [0.0f32; 8];
            c.copy_from_slice(&x[main_end + done..main_end + done + 8]);
            v = add_lanes(v, c);
            done += 8;
        }
    }
    for j in done..rem {
        t = t.fadd(x[main_end + j]);
    }
    let h = [v[0].fadd(v[4]), v[1].fadd(v[5]), v[2].fadd(v[6]), v[3].fadd(v[7])];
    let (half_a, half_b) = (h[0].fadd(h[2]), h[1].fadd(h[3]));
    half_a.fadd(half_b).fadd(t)
}

/// Window sum entry point of a pass: the route selector, then the scalar or the vector route.
pub fn window_sum(x: &[f32], n: i32, phase: u32, pass: usize) -> Result<f32, Error> {
    let _ = pass;
    if vector_route(n, phase) {
        Ok(window_sum_vector(x, n as usize, phase))
    } else {
        Ok(window_sum_scalar(x, n as usize))
    }
}

/// One box pass (P1.L, P1.M, P1.R) for a clamped radius r >= 0 over a slice of length l: out[i] = sum of x[max(0,i-r) ..= min(l-1,i+r)] in the
/// running-sum order of the listing. `s` is the window sum of x[0..=r].
pub fn box_pass(x: &[f32], out: &mut [f32], l: usize, r: usize, mut s: f32) {
    // left edge, growing window
    for i in 0..r {
        out[i] = s;
        s = s.fadd(x[r + 1 + i]);
    }
    // middle: form A (diff first) at the first step of a pair, form B (S + x_in, then - x_out) at the second step and at an odd remainder
    let m = l as i64 - 2 * r as i64 - 1;
    if m > 0 {
        let m = m as usize;
        let paired = 2 * (m / 2);
        for k in 0..m {
            out[r + k] = s;
            let (x_in, x_out) = (x[2 * r + 1 + k], x[k]);
            if k % 2 == 0 && k < paired {
                s = x_in.fsub(x_out).fadd(s);
            } else {
                s = s.fadd(x_in).fsub(x_out);
            }
        }
    }
    // right edge, shrinking window
    let mid_start = (l - 2 * r - 1) as usize;
    for j in 0..=r {
        out[l - r - 1 + j] = s;
        if j < r {
            s = s.fsub(x[mid_start + j]);
        }
    }
}

/// S1 / S2 as one elementwise function: out[i] = 0.5 * (x[i+D] + x[i-D]) with absent terms dropped, +0 where both are absent;
/// the scalar and vector loops of the listing write the same word per element.
pub fn shift_blend(x: &[f32], out: &mut [f32], l: usize, dd: usize) {
    if l <= dd {
        for o in &mut out[..l] {
            *o = 0.0;
        }
        return;
    }
    for i in 0..l {
        let hi = i + dd < l;
        let lo = i >= dd;
        out[i] = match (hi, lo) {
            (true, true) => HALF.fmul(x[i + dd].fadd(x[i - dd])),
            (true, false) => HALF.fmul(x[i + dd]),
            (false, true) => HALF.fmul(x[i - dd]),
            (false, false) => 0.0,
        };
    }
}

/// E1: the normaliser, f = (float)(1.0 / ((double)(2rx+1) * (double)(2ry+1) * (double)(2rz+1))), 2r+1 in 32-bit wrapping arithmetic.
pub fn normaliser(radii: [i32; 3]) -> f32 {
    let m = |r: i32| r.wrapping_mul(2).wrapping_add(1) as f64;
    let prod = (m(radii[0]) * m(radii[1])) * m(radii[2]);
    (ONE / prod) as f32
}

struct Ring {
    rows: [Vec<f32>; 3],
    ph: [u32; 3],
}

fn rd(src: &[f32], i: i64) -> Result<f32, Error> {
    if i < 0 || i as usize >= src.len() {
        Err(Error::ReadRange(i))
    } else {
        Ok(src[i as usize])
    }
}

fn wr(dst: &mut [f32], i: i64, v: f32) -> Result<(), Error> {
    if i < 0 || i as usize >= dst.len() {
        Err(Error::WriteRange(i))
    } else {
        dst[i as usize] = v;
        Ok(())
    }
}

/// E5: slice setup. Returns (al, cl): al = B[0] forced to 0, cl = A[l-1] forced to 0 at the bottom edge.
fn setup_slices(src: &[f32], w: i64, h: i64, bb: i64, off: i64, l: i64, a: &mut [f32], b: &mut [f32]) -> Result<(bool, bool), Error> {
    let (mut first_zeroed, mut last_zeroed) = (false, false);
    a[0] = rd(src, off)?;
    if bb == 0 {
        b[0] = 0.0;
        first_zeroed = true;
    } else {
        b[0] = HALF.fmul(rd(src, off)?.fadd(rd(src, off - w)?));
    }
    if l < 2 {
        return Ok((first_zeroed, last_zeroed));
    }
    let mut p = off + 1;
    let mut j: i64 = 1;
    if l - 1 > 1 {
        let pairs = (l - 1) >> 1;
        for k in 0..pairs {
            // odd step j = 2k+1
            let x = rd(src, p)?;
            a[(2 * k + 1) as usize] = HALF.fmul(x.fadd(rd(src, p + w)?));
            b[(2 * k + 1) as usize] = x;
            p += w + 1;
            j = 2 * k + 2;
            if j >= l - 1 {
                break;
            }
            // even step j = 2k+2
            let x = rd(src, p)?;
            a[(2 * k + 2) as usize] = x;
            b[(2 * k + 2) as usize] = HALF.fmul(x.fadd(rd(src, p - w)?));
            p += 1;
            j = 2 * k + 3;
        }
    }
    let ju = j as usize;
    if j & 1 == 1 {
        let x = rd(src, p)?;
        if bb + (l >> 1) < h - 1 {
            a[ju] = HALF.fmul(x.fadd(rd(src, p + w)?));
        } else {
            a[ju] = 0.0;
            last_zeroed = true;
        }
        b[ju] = x;
    } else {
        let x = rd(src, p)?;
        a[ju] = x;
        b[ju] = HALF.fmul(x.fadd(rd(src, p - w)?));
    }
    Ok((first_zeroed, last_zeroed))
}

fn run_pass(ring: &mut Ring, src_row: usize, dst_row: usize, l: usize, r: i32, pass: usize) -> Result<(), Error> {
    let phx = ring.ph[src_row];
    let s = window_sum(&ring.rows[src_row], r + 1, phx, pass)?;
    let mut out = std::mem::take(&mut ring.rows[dst_row]);
    box_pass(&ring.rows[src_row], &mut out, l, r as usize, s);
    ring.rows[dst_row] = out;
    Ok(())
}

fn run_shift(ring: &mut Ring, src_row: usize, dst_row: usize, l: usize, dd: usize) {
    let mut out = std::mem::take(&mut ring.rows[dst_row]);
    shift_blend(&ring.rows[src_row], &mut out, l, dd);
    ring.rows[dst_row] = out;
}

/// The callback. `dst` may be longer than width * height (slack for writes past the plane).
pub fn run(p: &Params, ph: &RowPhases, band: Band, src: &[f32], dst: &mut [f32], width: usize, height: usize) -> Result<(), Error> {
    if width == 0 || height == 0 || src.len() < width * height {
        return Err(Error::Size);
    }
    if p.d == i32::MIN {
        return Err(Error::ShiftMin);
    }
    let (w, h) = (width as i64, height as i64);
    let f = normaliser(p.radii);
    let dd = p.d.wrapping_abs() as usize;
    let rmax_clamp = |l: i64| ((l - 1) >> 1) as i32;
    // The three ring rows come from 0x112f00 once, before the band loop (0x1a3fc8..0x1a3ffa); the pointer roles A, B and spare persist across band rows,
    // so the row phases a pass sees rotate with the swaps of earlier band rows.
    let cap = (w.min(2 * h)).max(1) as usize + 4;
    let mut ring = Ring { rows: [vec![0.0; cap], vec![0.0; cap], vec![0.0; cap]], ph: ph.0 };
    let (mut ra, mut rb, mut rc) = (0usize, 1usize, 2usize);
    if band.count > 0 {
        for counter in 0..band.count as i64 {
            let c = counter + 1;
            let t = c + (band.start as i64 - h) + 1;
            let a_ = t.max(0);
            let b_ = (-t).max(0);
            let l = (w - 2 * a_).min(2 * h - 2 * b_);
            let off = ((w * b_) as i32).wrapping_add((2 * a_) as i32) as i64;
            if l <= 0 {
                continue;
            }
            let lu = l as usize;
            let (mut ba, mut bb_) = (std::mem::take(&mut ring.rows[ra]), std::mem::take(&mut ring.rows[rb]));
            let (first_zeroed, last_zeroed) = setup_slices(src, w, h, b_, off, l, &mut ba, &mut bb_)?;
            ring.rows[ra] = ba;
            ring.rows[rb] = bb_;
            let clamp = rmax_clamp(l);
            // P1 on A
            for k in 0..3 {
                let radius = p.radii[k];
                if radius <= 0 {
                    continue;
                }
                run_pass(&mut ring, ra, rc, lu, radius.min(clamp), k)?;
                std::mem::swap(&mut ra, &mut rc);
                if last_zeroed {
                    ring.rows[ra][lu - 1] = 0.0;
                }
            }
            if dd != 0 {
                run_shift(&mut ring, ra, rc, lu, dd);
                std::mem::swap(&mut ra, &mut rc);
                if last_zeroed {
                    ring.rows[ra][lu - 1] = 0.0;
                }
            }
            // P2 on B
            for k in 0..3 {
                let radius = p.radii[k];
                if radius <= 0 {
                    continue;
                }
                run_pass(&mut ring, rb, rc, lu, radius.min(clamp), 3 + k)?;
                std::mem::swap(&mut rb, &mut rc);
                if first_zeroed {
                    ring.rows[rb][0] = 0.0;
                }
            }
            if dd != 0 {
                run_shift(&mut ring, rb, rc, lu, dd);
                std::mem::swap(&mut rb, &mut rc);
                if first_zeroed {
                    ring.rows[rb][0] = 0.0;
                }
            }
            // O
            let (xa, xb) = (&ring.rows[ra], &ring.rows[rb]);
            let mut pos = off;
            for q in 0..lu / 2 {
                wr(dst, pos, f.fmul(xa[2 * q]))?;
                wr(dst, pos + 1, f.fmul(xb[2 * q + 1]))?;
                pos += w + 2;
            }
            if lu & 1 == 1 {
                wr(dst, pos, f.fmul(xa[lu - 1]))?;
            }
        }
    }
    // X: epilogue
    if band.start == 0 {
        wr(dst, (h - 1) * w, 0.0)?;
        wr(dst, (h - 1) * w + 1, 0.0)?;
        wr(dst, w - 1, 0.0)?;
        if w & 1 == 0 {
            wr(dst, w - 2, 0.0)?;
        }
    }
    Ok(())
}

/// The two bodies of the callback: A (entry 0x1a3ed0, listing 0x1a3f30..0x1a5ce4) and its twin B (entry 0x1a96e0, listing 0x1a9740..0x1ab4cc); `mirrored` selects B.
/// The twin B is body A with the row direction reversed: B starts a band row at source row H - 1 - b instead of b
/// (0x1a995b..0x1a99a1), reads the blend neighbours of the zigzag with the two row offsets exchanged (-W and +W), steps the slice by -W + 1 instead of
/// W + 1, writes the destination diagonal with step -W + 2, and sets the four epilogue words at rows 0 and H - 1 mirrored (0x1ab1d3..0x1ab203).
/// Everything between (window sums, shifts, normaliser) is the same code up to registers and placement. The port therefore runs the
/// one routine `run` on vertically flipped planes: flip the source, flip the destination in, run, flip the destination out.
pub fn run_body(mirrored: bool, p: &Params, ph: &RowPhases, band: Band, src: &[f32], dst: &mut [f32], width: usize, height: usize) -> Result<(), Error> {
    match mirrored {
        false => run(p, ph, band, src, dst, width, height),
        true => {
            if width == 0 || height == 0 || src.len() < width * height || dst.len() < width * height {
                return Err(Error::Size);
            }
            let flip = |v: &[f32]| -> Vec<f32> {
                let mut o = v.to_vec();
                for r in 0..height {
                    o[r * width..(r + 1) * width].copy_from_slice(&v[(height - 1 - r) * width..(height - r) * width]);
                }
                o
            };
            let fs = flip(src);
            let mut fd = flip(dst);
            // words past the plane (slack) stay where they are
            let res = run(p, ph, band, &fs, &mut fd, width, height);
            let back = flip(&fd);
            dst[..width * height].copy_from_slice(&back[..width * height]);
            let total = dst.len();
            if total > width * height {
                dst[width * height..].copy_from_slice(&fd[width * height..total]);
            }
            res
        }
    }
}
