//! Glint pass callback (AVX2 body, listing 0x160120..0x16165d): for every row of a plane, a chain of box-blur
//! passes over two ping-pong row buffers (edge mode 0 = zero padding, 1 = mirrored with the edge sample repeated),
//! an optional two-tap blend at +-m samples (m = |shift|) and a final scale by 1 / prod(2r+1).
//!
//! The arithmetic follows the listing: operation order, the single fused multiply-subtract at the mirrored left edge,
//! the x86 NaN rules of the scalar and packed adds and multiplies, and the reduction order of the row-start window sums,
//! which depends on the byte alignment of the row buffer (peel loop, 64-float blocks, 8-float blocks, scalar tail).
//! Element-wise loops (copies, scales, blends, running-sum updates) are lane independent and are written as scalar loops.

use fx_common::nanop::{self, NanOps};

/// vaddss / vaddps lane: a is the first source.
#[inline]
fn fadd(a: f32, b: f32) -> f32 {
    a.fadd(b)
}
#[inline]
fn fsub(a: f32, b: f32) -> f32 {
    a.fsub(b)
}
#[inline]
fn fmul(a: f32, b: f32) -> f32 {
    a.fmul(b)
}
/// vfmsub213ss: a * b - c with one rounding (operands in the order dest, src2, src3).
#[inline]
fn fmsub(a: f32, b: f32, c: f32) -> f32 {
    nanop::fms213(a, b, c)
}

/// Listing 0x15e94a0: eight equal lanes of one scalar (0.5f), splatted by code.
const HALF_LANE: f32 = 0.5;
#[inline]
fn half_const() -> f32 {
    HALF_LANE
}

type V8 = [f32; 8];

#[inline]
fn vadd(a: &V8, b: &[f32]) -> V8 {
    let mut o = [0f32; 8];
    for i in 0..8 {
        o[i] = fadd(a[i], b[i]);
    }
    o
}
#[inline]
fn vadd_acc(a: &V8, b: &V8) -> V8 {
    vadd(a, &b[..])
}

/// Reduction of an 8-lane accumulator: (l0+l4 + l2+l6) + (l1+l5 + l3+l7), the listing's extract / movhlps / shufps chain.
fn reduce(z: &V8) -> f32 {
    let q: [f32; 4] = [fadd(z[0], z[4]), fadd(z[1], z[5]), fadd(z[2], z[6]), fadd(z[3], z[7])];
    let half_lo = fadd(q[0], q[2]);
    let half_hi = fadd(q[1], q[3]);
    fadd(half_lo, half_hi)
}

/// Peel count in floats for a buffer at byte phase `align` (mod 32): None when the address is not 4-byte aligned.
fn peel_count(align: u8) -> Option<usize> {
    let a = (align & 31) as usize;
    if a == 0 {
        Some(0)
    } else if a & 3 != 0 {
        None
    } else {
        Some((32 - a) / 4)
    }
}

fn vec_threshold() -> usize {
    0x40
}
fn block_slack() -> usize {
    0x40
}

/// Window sum of buf[0..n] for edge mode 1 (two interleaved accumulator chains) plus the scalar part.
fn sum_mode1(buf: &[f32], n: usize, align: u8) -> f32 {
    let mut scalar_acc = 0f32;
    let mut z: V8 = [0.0; 8];
    let peel = peel_count(align);
    let vec_ok = n >= vec_threshold() && peel.map_or(false, |p| n >= p + block_slack());
    if !vec_ok {
        for i in 0..n {
            scalar_acc = fadd(scalar_acc, buf[i]);
        }
    } else {
        let peel = peel.unwrap();
        let e = n - ((n - peel) & 0x3f);
        for i in 0..peel {
            scalar_acc = fadd(scalar_acc, buf[i]);
        }
        let mut x: V8 = [0.0; 8];
        let mut y: V8 = [0.0; 8];
        let mut idx = peel;
        while idx < e {
            let l = |j: usize| &buf[idx + 8 * j..idx + 8 * j + 8];
            let lane_a = vadd(&x, l(0));
            let part_b = vadd(&y, l(1));
            let lane_c = vadd(&lane_a, l(2));
            let lane_d = vadd(&part_b, l(3));
            let lane_e = vadd(&lane_c, l(4));
            let lane_f = vadd(&lane_d, l(5));
            x = vadd(&lane_e, l(6));
            y = vadd(&lane_f, l(7));
            idx += 64;
        }
        let mut idx = e;
        if n > e {
            let rest = n - e;
            if rest >= 8 {
                let nv = rest & !7;
                let mut b = 0;
                while b < nv {
                    x = vadd(&x, &buf[idx + b..idx + b + 8]);
                    b += 8;
                }
                idx += nv;
            }
            for i in idx..n {
                scalar_acc = fadd(scalar_acc, buf[i]);
            }
        }
        z = vadd_acc(&x, &y);
    }
    fadd(reduce(&z), scalar_acc)
}

/// Window sum of buf[0..n] for edge mode 0 (eight accumulator chains) plus the scalar part.
fn sum_mode0(buf: &[f32], n: usize, align: u8) -> f32 {
    let mut scalar_acc = 0f32;
    let mut z: V8 = [0.0; 8];
    let peel = peel_count(align);
    let vec_ok = n >= vec_threshold() && peel.map_or(false, |p| n >= p + block_slack());
    if !vec_ok {
        for i in 0..n {
            scalar_acc = fadd(scalar_acc, buf[i]);
        }
    } else {
        let peel = peel.unwrap();
        let e = n - ((n - peel) & 0x3f);
        for i in 0..peel {
            scalar_acc = fadd(scalar_acc, buf[i]);
        }
        let mut a: [V8; 8] = [[0.0; 8]; 8];
        let mut idx = peel;
        while idx < e {
            for j in 0..8 {
                a[j] = vadd(&a[j], &buf[idx + 8 * j..idx + 8 * j + 8]);
            }
            idx += 64;
        }
        let (p, q, r, s);
        p = vadd_acc(&a[0], &a[1]);
        q = vadd_acc(&a[2], &a[3]);
        r = vadd_acc(&a[4], &a[5]);
        s = vadd_acc(&a[6], &a[7]);
        let u = vadd_acc(&p, &q);
        let v = vadd_acc(&r, &s);
        z = vadd_acc(&u, &v);
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
                scalar_acc = fadd(scalar_acc, buf[i]);
            }
        }
    }
    fadd(reduce(&z), scalar_acc)
}

/// Product of (2r+1) over the radii list as the listing forms it (scalar peel, 4-lane double chains, tail), then 1/prod as f32.
fn item(r: i32) -> f64 {
    let k = r.wrapping_mul(2).wrapping_add(1);
    k as f64
}

/// The f64 product of (2r+1) over the radii list in the listing's route order (RADII-ALIGN selects the route).
#[doc(hidden)]
pub fn radii_product(radii: &[i32], align: u8) -> f64 {
    let p = radii.len();
    let mut prod = 1.0f64;
    if p > 0 {
        let small = 16;
        let mut scalar_all = false;
        let mut peel = 0usize;
        let mut e = 0usize;
        let mut vector = false;
        if p < small {
            scalar_all = true;
        } else if p < 0x39 {
            e = p & !15;
            vector = true;
        } else {
            let a = (align & 31) as usize;
            let mut ok = true;
            if a != 0 {
                if a & 3 != 0 {
                    ok = false;
                } else {
                    peel = (32 - a) / 4;
                }
            }
            if !ok || p < peel + 16 {
                scalar_all = true;
                peel = 0;
            } else {
                e = p - ((p - peel) & 15);
                vector = true;
            }
        }
        if scalar_all {
            for &r in radii {
                prod *= item(r);
            }
        } else {
            if vector {
                for i in 0..peel {
                    prod *= item(radii[i]);
                }
                let mut chain_front = [prod, 1.0, 1.0, 1.0];
                let mut chain_back = [1.0f64; 4];
                let mut idx = peel;
                while idx < e {
                    for k in 0..4 {
                        let front_mid = chain_front[k] * item(radii[idx + k]);
                        let back_mid = chain_back[k] * item(radii[idx + 4 + k]);
                        chain_front[k] = item(radii[idx + 8 + k]) * front_mid;
                        chain_back[k] = item(radii[idx + 12 + k]) * back_mid;
                    }
                    idx += 16;
                }
                let m = [chain_front[0] * chain_back[0], chain_front[1] * chain_back[1], chain_front[2] * chain_back[2], chain_front[3] * chain_back[3]];
                prod = (m[0] * m[2]) * (m[1] * m[3]);
            }
            if p > e {
                let rest = p - e;
                let mut start = e;
                if rest >= 4 {
                    let nv = rest & !3;
                    let mut l = [prod, 1.0, 1.0, 1.0];
                    let mut b = 0;
                    while b < nv {
                        for k in 0..4 {
                            l[k] *= item(radii[e + b + k]);
                        }
                        b += 4;
                    }
                    prod = (l[0] * l[2]) * (l[1] * l[3]);
                    start = e + nv;
                }
                for i in start..p {
                    prod *= item(radii[i]);
                }
            }
        }
    }
    prod
}

fn radius_scale(radii: &[i32], align: u8) -> f32 {
    (1.0f64 / radii_product(radii, align)) as f32
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A pass with a positive radius ran with an edge mode other than 0 or 1 (the original logs an assertion and continues on garbage).
    BadEdge(i32),
    /// Width smaller than 2r+1 for a positive radius: the original reads and writes outside the row.
    RadiusTooLarge { radius: i32, width: usize },
    /// |shift| does not fit (i32::MIN).
    BadShift,
    /// Slice too short for width * rows.
    ShortBuffer,
}

/// One box-blur pass of radius r >= 1 over `src[0..w]` into `out[0..w]`; requires w >= 2r+1.
fn box_pass(src: &[f32], out: &mut [f32], mode: i32, r: usize, w: usize, align: u8) {
    let m = r / 2;
    let mut acc;
    if mode == 1 {
        let s = sum_mode1(src, r + 1, align);
        let c = 2.0f32;
        acc = fmsub(c, s, src[r]);
        for k in 0..m {
            let part_b = src[r + 2 * k + 1];
            let delta = fsub(part_b, src[r - 2 * k - 1]);
            out[2 * k] = acc;
            acc = fadd(delta, acc);
            out[2 * k + 1] = acc;
            let moved = fadd(acc, src[r + 2 * k + 2]);
            acc = fsub(moved, src[r - 2 * k - 2]);
        }
        if r > 2 * m {
            out[2 * m] = acc;
            acc = fadd(acc, src[r + 2 * m + 1]);
            acc = fsub(acc, src[r - 2 * m - 1]);
        }
    } else {
        acc = sum_mode0(src, r + 1, align);
        for k in 0..m {
            out[2 * k] = acc;
            acc = fadd(acc, src[r + 2 * k + 1]);
            out[2 * k + 1] = acc;
            acc = fadd(acc, src[r + 2 * k + 2]);
        }
        if r > 2 * m {
            out[2 * m] = acc;
            acc = fadd(acc, src[r + 2 * m + 1]);
        }
    }
    // middle: window moves one sample right per output
    let nmid = w - 2 * r - 1;
    if nmid > 0 {
        let pairs = nmid / 2;
        for k in 0..pairs {
            let delta = fsub(src[2 * r + 1 + 2 * k], src[2 * k]);
            out[r + 2 * k] = acc;
            acc = fadd(delta, acc);
            out[r + 2 * k + 1] = acc;
            let moved = fsub(acc, src[2 * k + 1]);
            acc = fadd(moved, src[2 * r + 2 + 2 * k]);
        }
        if nmid > 2 * pairs {
            let k = pairs;
            out[r + 2 * k] = acc;
            acc = fsub(acc, src[2 * k]);
            acc = fadd(acc, src[2 * r + 2 * k + 1]);
        }
    }
    // right edge: r+1 outputs
    let npairs = (r + 1) / 2;
    let base = w - r - 1;
    if mode == 1 {
        for k in 0..npairs {
            let hi = src[w - 1 - 2 * k];
            let delta = fsub(hi, src[nmid + 2 * k]);
            out[base + 2 * k] = acc;
            acc = fadd(delta, acc);
            out[base + 2 * k + 1] = acc;
            let moved = fadd(acc, src[w - 2 - 2 * k]);
            acc = fsub(moved, src[nmid + 2 * k + 1]);
        }
    } else {
        for k in 0..npairs {
            out[base + 2 * k] = acc;
            acc = fsub(acc, src[nmid + 2 * k]);
            out[base + 2 * k + 1] = acc;
            acc = fsub(acc, src[nmid + 2 * k + 1]);
        }
    }
    if (r + 1) > 2 * npairs {
        out[base + 2 * npairs] = acc;
    }
}

/// Two-tap blend at +-m samples from `buf` into `dst`; m >= 1.
fn blend(buf: &[f32], dst: &mut [f32], w: usize, m: usize) {
    let half = half_const();
    let all_zero = w <= m;
    if all_zero {
        for v in dst[..w].iter_mut() {
            *v = 0.0;
        }
    } else if w >= 2 * m {
        for i in 0..m {
            dst[i] = fmul(half, buf[m + i]);
        }
        for i in m..w - m {
            dst[i] = fmul(half, fadd(buf[i + m], buf[i - m]));
        }
        for i in w - m..w {
            dst[i] = fmul(half, buf[i - m]);
        }
    } else {
        for i in 0..w - m {
            dst[i] = fmul(half, buf[m + i]);
        }
        for v in dst[w - m..m].iter_mut() {
            *v = 0.0;
        }
        for i in m..w {
            dst[i] = fmul(half, buf[i - m]);
        }
    }
}

/// Parameters read from the callback's global cells.
pub struct Config<'a> {
    /// Edge mode (0 zero padding, 1 mirrored with the edge sample repeated).
    pub edge: i32,
    /// Radius per pass; radii <= 0 skip the pass but still enter the scale product.
    pub radii: &'a [i32],
    /// Shifted two-tap blend distance (sign ignored), 0 = off.
    pub shift: i32,
}

/// Byte addresses modulo 32 of the buffers whose alignment steers the reduction order.
pub struct Alignment {
    /// The two row buffers returned by the allocator for slots 0 and 1.
    pub scratch: [u8; 2],
    /// The radii array.
    pub radii: u8,
}

/// Process `rows` rows of `width` samples from `src` into `dst` (both row-major, dense).
pub fn run(cfg: &Config, phases: &Alignment, src: &[f32], dst: &mut [f32], width: usize, rows: usize) -> Result<(), Error> {
    if width == 0 || rows == 0 {
        return Ok(());
    }
    if src.len() < width * rows || dst.len() < width * rows {
        return Err(Error::ShortBuffer);
    }
    if cfg.shift == i32::MIN {
        return Err(Error::BadShift);
    }
    let scale = radius_scale(cfg.radii, phases.radii);
    let shift = cfg.shift.unsigned_abs() as usize;
    let mut bufs: [Vec<f32>; 2] = [vec![0.0; width], vec![0.0; width]];
    let mut cur = 0usize;
    for row in 0..rows {
        bufs[cur].copy_from_slice(&src[row * width..(row + 1) * width]);
        let mut a = cur;
        let mut b = 1 - cur;
        for &r in cfg.radii {
            let skip = r <= 0;
            if skip {
                continue;
            }
            if cfg.edge != 0 && cfg.edge != 1 {
                return Err(Error::BadEdge(cfg.edge));
            }
            let ru = r as usize;
            if width < 2 * ru + 1 {
                return Err(Error::RadiusTooLarge { radius: r, width });
            }
            let (x, y) = if a == 0 { let (l, h) = bufs.split_at_mut(1); (&l[0], &mut h[0]) } else { let (l, h) = bufs.split_at_mut(1); (&h[0], &mut l[0]) };
            box_pass(x, y, cfg.edge, ru, width, phases.scratch[a]);
            std::mem::swap(&mut a, &mut b);
        }
        cur = a;
        if shift != 0 {
            let other = 1 - cur;
            let (x, y) = if cur == 0 { let (l, h) = bufs.split_at_mut(1); (&l[0], &mut h[0]) } else { let (l, h) = bufs.split_at_mut(1); (&h[0], &mut l[0]) };
            blend(x, y, width, shift);
            cur = other;
        }
        for i in 0..width {
            dst[row * width + i] = fmul(scale, bufs[cur][i]);
        }
    }
    Ok(())
}

/// Stand-ins and modelling choices of this port.
pub fn assumptions() -> &'static [&'static str] {
    &[
        "SCRATCH-ALIGNMENT-AS-ARGUMENT: byte phase (mod 32) of the two allocator row buffers enters as an argument because the window-sum reduction order depends on it",
        "RADII-ALIGN (Glint pass callback: the radii slice's phase modulo 32 selects the scalar, peeled or vector order of the f64 product of 2r + 1 over the passes when there are 16 or more; the product is exact, so phase-free, while every partial product is below 2^53; a host value passed by the executor, 0 unless a counted render records another)",
        "COPY-AS-SLICE-COPY: the row copy (libc copy or vector loop) is a slice copy; source and scratch never overlap",
        "ASSERT-PATHS-AS-ERRORS: an edge mode outside {0,1} on an active pass, and width < 2r+1, are errors (the original logs and continues on out-of-row accesses)",
    ]
}
