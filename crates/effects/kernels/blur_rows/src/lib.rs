//! blur_rows: native port of the blur core horizontal row kernel K1
//! (multiversion head, AVX2 body of neutral_1).
//!
//! Source of truth: the listing of K1 (listing addresses are dump offsets, VA = offset).
//! Scope: AVX2 body only. FMA sites use `f32::mul_add` with the listing's operand pairing. Plain f32 arithmetic elsewhere
//! (silicon NaN rule: NaN payloads are not modelled, tests compare NaN as a class).
//!
//! Output of `pass_x` is the UNNORMALISED box sum; the driver scales afterwards.
//!
//! Stand-ins (S7): see `assumptions()`.

/// Edge mode 0: samples outside the row are zero.
pub const EDGE_ZERO: i32 = 0;
/// Edge mode 1: mirror with edge duplication, in[-1-k] = in[k].
pub const EDGE_MIRROR: i32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RowsError {
    /// The original asserts (call, lines) for an edge mode other than 0 or 1.
    EdgeMode(i32),
    /// Radius outside what the driver can produce: r == 0, or width > 1 and 0 <= r <= (width >> 1) - 1.
    Radius { radius: i32, width: usize },
    /// A slice length does not match the geometry.
    Geometry,
}

/// One stand-in or assumption, S7 style.
#[derive(Debug, Clone)]
pub struct Assumption {
    pub id: &'static str,
    pub va: Option<u64>,
    pub value: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "B-A1", va: None, value: "poll returns 0 on every 64th row, never aborts", basis: "outcome depends on elapsed time (B-U7)", label: "ASSUMED" },
        Assumption { id: "B-A3", va: None, value: "2^63 as f32 (0x5f000000)", basis: "NOT_CAPTURED; only matters if 4*width >= 2^63 (B-U8, B-U11)", label: "ASSUMED" },
        Assumption { id: "B-A4", va: None, value: "scratch base phase (address mod 32) is the explicit input `base_phase`; worker 0; stride (4*width+15)&!15", basis: "allocator virtual call is not in the binaries (B-U1)", label: "ASSUMED" },
        Assumption { id: "B-A6", va: None, value: "single worker, one tile covering every row", basis: "DECISIONS W/S7, U06", label: "ASSUMED" },
    ]
}

type V8 = [f32; 8];

#[inline]
fn vload(x: &[f32], at: usize) -> V8 {
    let mut v = [0f32; 8];
    v.copy_from_slice(&x[at..at + 8]);
    v
}

#[inline]
fn vadd(a: &V8, b: &V8) -> V8 {
    let mut o = [0f32; 8];
    for i in 0..8 {
        o[i] = a[i] + b[i];
    }
    o
}

/// Horizontal reduction used :
/// t[k] = v[k] + v[k+4] (vextractf128 + vaddps), u0 = t0 + t2 (vmovhlps + vaddps), u1 = t1 + t3, result u0 + u1 (shufps 0xf5 + vaddss).
#[inline]
fn hred(v: &V8) -> f32 {
    let t = [v[0] + v[4], v[1] + v[5], v[2] + v[6], v[3] + v[7]];
    let u0 = t[0] + t[2];
    let u1 = t[1] + t[3];
    u0 + u1
}

/// Initial window sum S0 = sum of x[0..len] (len = r + 1), exact groupings of the listing.
/// `phase` is the byte address of x modulo 32 (B-A4).
/// len < 64, or an address that is not 4-byte aligned, or len < peel + 64: sequential scalar from +0.0 (.).
/// Otherwise: peel scalar elements up to 32-byte alignment, blocks of 64 floats, 8-float blocks, scalar tail, horizontal reduction.
fn s0(x: &[f32], len: usize, edge: i32, phase: usize) -> f32 {
    let ph = phase & 31;
    let peel = if len < 64 {
        None
    } else if ph == 0 {
        Some(0usize)
    } else if ph & 3 != 0 {
        None
    } else {
        Some((32 - ph) / 4)
    };
    let peel = match peel {
        Some(p) if len >= p + 64 => Some(p),
        _ => None,
    };
    let p = match peel {
        None => {
            // scalar from +0.0, then the vector registers (all zero) reduce to +0.0 and are added in front (vaddss xmm0, xmm7, xmm0)
            let mut acc = 0f32;
            for i in 0..len {
                acc += x[i];
            }
            return 0f32 + acc;
        }
        Some(p) => p,
    };
    let mut acc = 0f32;
    for i in 0..p {
        acc += x[i];
    }
    let nb = (len - p) / 64;
    let end = p + 64 * nb;
    let rem = len - end;
    let n8 = rem / 8;
    if edge == EDGE_MIRROR {
        // two chains: P over even 8-blocks (b0,b2,b4,b6), Q over odd 8-blocks 
        let mut pp = [0f32; 8];
        let mut qq = [0f32; 8];
        for k in 0..nb {
            let b = p + 64 * k;
            let blk = |j: usize| vload(x, b + 8 * j);
            pp = vadd(&vadd(&vadd(&vadd(&pp, &blk(0)), &blk(2)), &blk(4)), &blk(6));
            qq = vadd(&vadd(&vadd(&vadd(&qq, &blk(1)), &blk(3)), &blk(5)), &blk(7));
        }
        for k in 0..n8 {
            pp = vadd(&pp, &vload(x, end + 8 * k));
        }
        for i in (end + 8 * n8)..len {
            acc += x[i];
        }
        let v = vadd(&pp, &qq);
        hred(&v) + acc
    } else {
        // eight independent accumulators, combined ((a0+a1)+(a2+a3))+((a4+a5)+(a6+a7))
        let mut a = [[0f32; 8]; 8];
        for k in 0..nb {
            let b = p + 64 * k;
            for j in 0..8 {
                a[j] = vadd(&a[j], &vload(x, b + 8 * j));
            }
        }
        let c01 = vadd(&a[0], &a[1]);
        let c23 = vadd(&a[2], &a[3]);
        let c45 = vadd(&a[4], &a[5]);
        let c67 = vadd(&a[6], &a[7]);
        let mut v = vadd(&vadd(&c01, &c23), &vadd(&c45, &c67));
        for k in 0..n8 {
            v = vadd(&v, &vload(x, end + 8 * k));
        }
        for i in (end + 8 * n8)..len {
            acc += x[i];
        }
        hred(&v) + acc
    }
}

/// Window sum: out[i] = sum of x over [i-r, i+r] with the edge rule, as a sliding sum with the listing's pair groupings.
/// `in_phase` = byte address of `x` modulo 32 (B-A4), only used for r + 1 >= 64.
/// Listing: mode 1 head, middle, tail; mode 0 head, middle same, tail .
pub fn box_row(x: &[f32], out: &mut [f32], r: i32, edge: i32, in_phase: usize) -> Result<(), RowsError> {
    let n = x.len();
    if out.len() != n {
        return Err(RowsError::Geometry);
    }
    if edge != EDGE_ZERO && edge != EDGE_MIRROR {
        return Err(RowsError::EdgeMode(edge));
    }
    if n == 0 {
        return Ok(());
    }
    let ok = r == 0 || (n > 1 && r >= 0 && (r as usize) <= (n >> 1) - 1);
    if !ok {
        return Err(RowsError::Radius { radius: r, width: n });
    }
    let r = r as usize;
    let mirror = edge == EDGE_MIRROR;
    let s = s0(x, r + 1, edge, in_phase);
    // out[0]: mode 0 = S0, mode 1 = vfmsub213ss(2.0, S0, x[r]) = fma(2, S0, -x[r]) 
    let mut y = if mirror { 2.0f32.mul_add(s, -x[r]) } else { s };

    // head: steps i -> i+1 for i in 0..r, storing out[i] first (y_r is not stored yet)
    let hp = r / 2;
    for j in 0..hp {
        let i = 2 * j;
        out[i] = y;
        if mirror {
            y = (x[r + 1 + i] - x[r - 1 - i]) + y;
            out[i + 1] = y;
            y = (y + x[r + 2 + i]) - x[r - 2 - i];
        } else {
            y += x[r + 1 + i];
            out[i + 1] = y;
            y += x[r + 2 + i];
        }
    }
    if r % 2 == 1 {
        let i = r - 1;
        out[i] = y;
        if mirror {
            y = (y + x[r + 1 + i]) - x[r - 1 - i];
        } else {
            y += x[r + 1 + i];
        }
    }

    // middle: m = n - 2r - 1 steps, i = r + t
    let m = n - 2 * r - 1;
    let mp = m / 2;
    for j in 0..mp {
        let i = r + 2 * j;
        out[i] = y;
        y = (x[i + r + 1] - x[i - r]) + y;
        out[i + 1] = y;
        y = (y - x[i + 1 - r]) + x[i + r + 2];
    }
    if m % 2 == 1 {
        let i = r + 2 * mp;
        out[i] = y;
        y = (y - x[i - r]) + x[i + r + 1];
    }

    // tail: outputs i = n-r-1+t for t in 0..=r; step t removes x[m+t] and (mirror) adds x[n-1-t]
    for t in 0..=r {
        let i = n - r - 1 + t;
        out[i] = y;
        if t == r {
            break;
        }
        if mirror {
            if t % 2 == 0 {
                y = (x[n - 1 - t] - x[m + t]) + y;
            } else {
                y = (y + x[n - 1 - t]) - x[m + t];
            }
        } else {
            y -= x[m + t];
        }
    }
    Ok(())
}

/// Blend of the subpixel path : out[i] = fma(c, out[i], frac * z[i]) (vmulss frac*z, then vfmadd231ss c*out + that).
/// The vector form (vmulps + vfmadd231ps) is element-wise identical.
pub fn blend(out: &mut [f32], z: &[f32], c: f32, frac: f32) {
    for i in 0..out.len() {
        out[i] = c.mul_add(out[i], frac * z[i]);
    }
}

/// The job record of K1 (SPEC 2d), explicit parameters.
#[derive(Debug, Clone)]
pub struct JobX<'a> {
    /// 0 zero, 1 mirror. Other values are an error (the original asserts).
    pub edge: i32,
    /// Number of passes (1..3 from the driver; any count <= a.len() is accepted, 0 copies).
    pub npasses: usize,
    /// Nonzero: subpixel path (a != b passes also compute the b-radius window and blend).
    pub subpixel: i32,
    /// Fractional radius part (f32).
    pub frac: f32,
    /// Floor radius per pass.
    pub a: &'a [i32],
    /// Ceil radius per pass.
    pub b: &'a [i32],
    /// Blend coefficient per pass.
    pub c: &'a [f32],
    /// Scratch base address modulo 32 in bytes (B-A4). Buffer idx lives at (base_phase + idx * stride) mod 32 with stride (4*width+15)&!15.
    pub base_phase: usize,
}

/// Horizontal pass over dense planes (width * height floats), K1 body semantics, single worker, tile = all rows.
/// Per row: copy the source row into buffer 0; per pass: window of radius a (and radius b into buffer 2 when blending), blend, swap buffers 0/1;
/// finally copy the last buffer to the destination row. Result is the unnormalised sum.
pub fn pass_x(src: &[f32], dst: &mut [f32], width: usize, height: usize, job: &JobX) -> Result<(), RowsError> {
    if src.len() < width * height || dst.len() < width * height {
        return Err(RowsError::Geometry);
    }
    if job.edge != EDGE_ZERO && job.edge != EDGE_MIRROR {
        return Err(RowsError::EdgeMode(job.edge));
    }
    if job.a.len() < job.npasses || job.b.len() < job.npasses || job.c.len() < job.npasses {
        return Err(RowsError::Geometry);
    }
    if width == 0 || height == 0 {
        return Ok(());
    }
    let stride = (4 * width + 15) & !15;
    let phase = |idx: usize| (job.base_phase + idx * stride) & 31;
    let mut buf = [vec![0f32; width], vec![0f32; width]];
    let mut z = vec![0f32; width];
    for row in 0..height {
        let srow = &src[row * width..(row + 1) * width];
        buf[0].copy_from_slice(srow);
        let mut cur = 0usize;
        for p in 0..job.npasses {
            let (ra, rb) = (job.a[p], job.b[p]);
            let blend_on = job.subpixel != 0 && ra != rb;
            let (first, second) = buf.split_at_mut(1);
            let (inp, outp) = if cur == 0 { (&first[0], &mut second[0]) } else { (&second[0], &mut first[0]) };
            if blend_on {
                box_row(inp, &mut z, rb, job.edge, phase(cur))?;
            }
            box_row(inp, outp, ra, job.edge, phase(cur))?;
            if blend_on {
                blend(outp, &z, job.c[p], job.frac);
            }
            cur ^= 1;
        }
        dst[row * width..(row + 1) * width].copy_from_slice(&buf[cur]);
    }
    Ok(())
}
