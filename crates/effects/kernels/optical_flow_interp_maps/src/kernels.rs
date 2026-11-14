//! The five tile kernels the interpolation map generator runs, AVX2 and FMA route: forward splat, neighbourhood fill,
//! masked copy, zero count and the interpolation of the output maps. Every kernel works on the row chunk
//! `[start, start + count)` that one worker receives. Buffers are plain word slices; an index outside a slice or below zero
//! panics (the original would read outside the buffer: outside the contract of the cases run).
//! Constants: see `kernel_assumptions()`.
use crate::{HALF, UNIT};
use fx_common::nanop::{fma213, fma231, fnma213, NanOps};
use fx_common::x86::cvttss2si32;

/// Scalar constant two.
pub const TWO: f32 = 2.0;
/// Sign mask (lane 0 of the sign-mask constant), as bits.
pub const SIGN_BITS: u32 = 0x8000_0000;
/// Each lane of the all-ones vector constant, the weight of one zero word in the count kernel.
pub const VEC_ONE: f32 = 1.0;

pub struct KAssumption {
    pub id: &'static str,
    pub label: &'static str,
    pub basis: &'static str,
}

pub fn kernel_assumptions() -> Vec<KAssumption> {
    vec![
        KAssumption { id: "TWO", label: "MEASURED-EMULATOR-READ", basis: "scalar f32 2.0, measured at the readers of other routines (bits 00000040)" },
        KAssumption { id: "SIGN-MASK", label: "MEASURED-EMULATOR-READ", basis: "sign bit 0x80000000 in lane 0, read from the original at load (MEASURED-EMULATOR-READ)" },
        KAssumption { id: "VEC-ONE", label: "MEASURED-EMULATOR-READ", basis: "1.0 in every lane, read from the original at load (MEASURED-EMULATOR-READ)" },
    ]
}

#[inline]
fn ix(i: i64) -> usize {
    usize::try_from(i).expect("negative buffer index")
}
#[inline]
fn f(b: u32) -> f32 {
    f32::from_bits(b)
}
#[inline]
fn flip(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ SIGN_BITS)
}
/// 32 bit product of a row index and the stride, sign extended (imul r32 then movsxd).
#[inline]
fn rowoff(y: i32, stride: i32) -> i64 {
    y.wrapping_mul(stride) as i64
}

/// Floor split of one source coordinate: lower and upper sample (64 bit) and fraction.
fn split64(c: f32, n: i32) -> (i64, i64, f32) {
    if 0.0f32 > c {
        (0, 1, 0.0)
    } else if c >= (n.wrapping_sub(1)) as f32 {
        ((n as i64) - 2, (n as i64) - 1, UNIT)
    } else {
        let i = fx_common::x86::cvtss2si32_rne(c.floor());
        (i as i64, i as i64 + 1, c.fsub(i as f32))
    }
}
/// Same split where the 32 bit variant is used for the row index (lea r32).
fn split32(c: f32, n: i32) -> (i32, i32, f32) {
    let (a, b, fr) = split64(c, n);
    (a as i32, b as i32, fr)
}

/// EXTENT-ONE-REFUSED: at width or height 1 every sample fetch reads outside the frame in the original; the port refuses by name and models no read.
#[derive(Debug, PartialEq, Eq)]
pub struct ExtentOneRefused;

pub struct K1<'a> {
    pub w: i32,
    pub h: i32,
    pub stride: i32,
    pub nch: i32,
    pub scale: f32,
    pub neg1: f32,
    pub prev_planes: [&'a [u32]; 3],
    pub next_planes: [&'a [u32]; 3],
    pub a: &'a [u32],
    pub b: &'a [u32],
    pub target_flow_x: &'a mut [u32],
    pub target_flow_y: &'a mut [u32],
    pub best_cost: &'a mut [u32],
    pub covered: &'a mut [u32],
    pub source_done: &'a mut [u32],
}

/// Forward splat of the flow of the rows `[start, start+count)`. Every pixel moves
/// to the position given by its flow; the target keeps the smallest matching error; the source word of source_done is set.
pub fn k1_splat(k: &mut K1, start: i32, count: i32) -> Result<(), ExtentOneRefused> {
    let end = start.wrapping_add(count);
    let (w, h, stride) = (k.w, k.h, k.stride);
    if w == 1 || h == 1 {
        return Err(ExtentOneRefused);
    }
    if start >= end {
        return Ok(());
    }
    let nc = k.nch.min(3);
    let mut rowbase: i64 = start.wrapping_mul(stride) as i64;
    let mut row = start;
    while row < end {
        if w > 0 {
            let yf = row as f32;
            let row_centre = HALF.fadd(yf);
            for c in 0..w as i64 {
                let idx = ix(rowbase + c);
                let abits = k.a[idx];
                let bbits = k.b[idx];
                let (av, bv) = (f(abits), f(bbits));
                let sb = k.scale.fmul(bv);
                let tyf = fma213(sb, k.neg1, row_centre);
                let shifted_x_base = fma231(HALF, k.scale, av);
                let xf = (c as i32) as f32;
                let txf = shifted_x_base.fadd(xf);
                let ty = cvttss2si32(tyf);
                let tx = cvttss2si32(txf);
                if ty < start || ty >= end {
                    continue;
                }
                if tx < 0 || tx >= w || ty < 0 || ty >= h {
                    k.source_done[idx] = 1;
                    continue;
                }
                let mut cost = 0.0f32;
                if nc > 0 {
                    let src_x = av.fadd(xf);
                    let src_y = bv.fadd(row as f32);
                    let (x_lo, x_hi, fx) = split64(src_x, w);
                    let (y_lo, y_hi, fy) = split32(src_y, h);
                    let row_lo = rowoff(y_lo, stride);
                    let row_hi = rowoff(y_hi, stride);
                    let p1i = rowoff(row, stride) + c;
                    for channel in 0..nc as usize {
                        let pl = k.next_planes[channel];
                        let sample_tr = f(pl[ix(x_hi + row_lo)]);
                        let sample_tl = f(pl[ix(x_lo + row_lo)]);
                        let sample_lower_left = f(pl[ix(x_lo + row_hi)]);
                        let sample_br = f(pl[ix(x_hi + row_hi)]);
                        let p = f(k.prev_planes[channel][ix(p1i)]);
                        let e = crate::channel_error(sample_tl, sample_tr, sample_lower_left, sample_br, fx, fy, p);
                        cost = if channel == 0 { e } else { e.fadd(cost) };
                    }
                }
                let d = ix((tx.wrapping_add(ty.wrapping_mul(stride))) as i64);
                if k.covered[d] != 0 {
                    if f(k.best_cost[d]) > cost {
                        k.target_flow_x[d] = abits;
                        k.target_flow_y[d] = bbits;
                        k.best_cost[d] = cost.to_bits();
                    }
                } else {
                    k.target_flow_x[d] = abits;
                    k.target_flow_y[d] = bbits;
                    k.best_cost[d] = cost.to_bits();
                    k.covered[d] = 1;
                }
                k.source_done[idx] = 1;
            }
        }
        row += 1;
        rowbase += stride as i64;
    }
    Ok(())
}

pub struct K2<'a> {
    pub w: i32,
    pub h: i32,
    pub stride: i32,
    /// flag of the destination pixel: nonzero keeps its value
    pub covered: &'a [u32],
    /// value plane read
    pub value_in: &'a [u32],
    /// value plane written
    pub value_out: &'a mut [u32],
    /// flag plane of the neighbours
    pub valid_in: &'a [u32],
    /// flag plane written
    pub valid_out: &'a mut [u32],
}

/// Fill each pixel whose covered flag is zero with the mean of the flagged values of its 3x3 neighbourhood
/// (the pixel itself included); a filled pixel gets its flag set.
pub fn k2_fill(k: &mut K2, start: i32, count: i32) {
    let end = start.wrapping_add(count);
    if start >= end {
        return;
    }
    let (w, h, stride) = (k.w, k.h, k.stride);
    for row_step in 0..(count as u32) {
        let y = start.wrapping_add(row_step as i32);
        let rb = rowoff(y, stride);
        if w <= 0 {
            continue;
        }
        let rows: [(i32, i64); 3] = [(y.wrapping_sub(1), rowoff(y.wrapping_sub(1), stride)), (y, rb), (y.wrapping_add(1), rowoff(y.wrapping_add(1), stride))];
        for x in 0..(w as u32) {
            let xi = x as i32;
            if k.covered[ix(rb + xi as i64)] != 0 {
                continue;
            }
            let mut cnt = 0.0f32;
            let mut sum = 0.0f32;
            for (ri, (yy, rbn)) in rows.iter().enumerate() {
                if *yy < 0 || h <= *yy {
                    continue;
                }
                for col_delta in [-1i32, 0, 1] {
                    let xx = xi.wrapping_add(col_delta);
                    if xx < 0 || w <= xx {
                        continue;
                    }
                    let i = ix(rbn + xx as i64);
                    if k.valid_in[i] == 0 {
                        continue;
                    }
                    let v = f(k.value_in[i]);
                    if ri == 0 && col_delta == -1 {
                        // the first neighbour is loaded (movss), not added
                        cnt = UNIT;
                        sum = v;
                    } else {
                        cnt = cnt + UNIT;
                        sum = sum.fadd(v);
                    }
                }
            }
            if cnt > 0.0 {
                let r = sum.fdiv(cnt);
                k.value_out[ix(rb + xi as i64)] = r.to_bits();
                k.valid_out[ix(rb + xi as i64)] = 1;
            }
        }
    }
}

/// Masked copy: dst[i] = src[i] where flag[i] == 0.
pub fn k3_masked_copy(w: i32, stride: i32, flag: &[u32], src: &[u32], dst: &mut [u32], start: i32, count: i32) {
    let end = start.wrapping_add(count);
    if start >= end || w <= 0 {
        return;
    }
    for row_step in 0..(count as u32) {
        let y = start.wrapping_add(row_step as i32);
        let rb = rowoff(y, stride);
        for x in 0..w as i64 {
            let i = ix(rb + x);
            if flag[i] == 0 {
                dst[i] = src[i];
            }
        }
    }
}

/// The number of zero words of the plane over the chunk, as an f32. Every sum is an exact integer in f32 for counts below 2^24 (the cases run).
pub fn k4_count_zero(w: i32, stride: i32, plane: &[u32], start: i32, count: i32) -> f32 {
    let end = start.wrapping_add(count);
    let mut n: u64 = 0;
    if start < end && w > 0 {
        for row_step in 0..(count as u32) {
            let y = start.wrapping_add(row_step as i32);
            let rb = rowoff(y, stride);
            for x in 0..w as i64 {
                if plane[ix(rb + x)] == 0 {
                    n += 1;
                }
            }
        }
    }
    (n as f32) * VEC_ONE
}

pub struct K5<'a> {
    pub w: i32,
    pub stride: i32,
    pub scale: f32,
    /// additive term of the second and fourth outputs
    pub flow_offset: f32,
    pub a: &'a [u32],
    pub b: &'a [u32],
    /// byte position of the second flow plane; only its alignment modulo 32 selects the vector or scalar path of a row
    pub flow_b_origin: u64,
    pub out: [&'a mut [u32]; 4],
}

/// Per pixel, four outputs from the flow words a and b. The row is processed by a
/// scalar head, an eight lane body and a scalar tail; the two forms differ only on NaN operands, so the split is modelled.
pub fn k5_interp(k: &mut K5, start: i32, count: i32) {
    let end = start.wrapping_add(count);
    if start >= end {
        return;
    }
    let w = k.w as i64;
    let c = TWO.fdiv(k.w as f32);
    let cs = c.fmul(k.scale);
    let oms = UNIT.fsub(k.scale);
    let gain_rest = c.fmul(oms);
    let flow_offset = k.flow_offset;
    for row_step in 0..(count as u32) {
        let y = start.wrapping_add(row_step as i32);
        let rb = rowoff(y, k.stride);
        if w <= 0 {
            continue;
        }
        // path selection: scalar head, eight lane body, scalar tail
        let addr = k.flow_b_origin.wrapping_add((rb as u64).wrapping_mul(4));
        let (head, vend): (i64, i64) = if w < 8 {
            (0, 0)
        } else if w < 0x4b0 {
            (0, w & !7)
        } else {
            let m = (addr & 0x1f) as i64;
            if m == 0 {
                (0, w - ((w - 0) & 7))
            } else if m & 3 != 0 {
                (0, 0)
            } else {
                let hd = (32 - m) / 4;
                if w < hd + 8 {
                    (0, 0)
                } else {
                    (hd, w - ((w - hd) & 7))
                }
            }
        };
        for xi in 0..w {
            let i = ix(rb + xi);
            let a = f(k.a[i]);
            let b = f(k.b[i]);
            let vector = xi >= head && xi < vend && vend > 0;
            let (res_a, res_b, res_c, res_d);
            if vector {
                res_a = flip(cs).fmul(a);
                res_c = gain_rest.fmul(a);
                res_b = c.fmul(fma213(flip(k.scale), b, flow_offset));
                res_d = c.fmul(fma213(b, oms, flow_offset));
            } else {
                res_a = flip(cs.fmul(a));
                res_c = a.fmul(gain_rest);
                res_b = c.fmul(fnma213(k.scale, b, flow_offset));
                res_d = c.fmul(fma213(b, oms, flow_offset));
            }
            k.out[0][i] = res_a.to_bits();
            k.out[1][i] = res_b.to_bits();
            k.out[2][i] = res_c.to_bits();
            k.out[3][i] = res_d.to_bits();
        }
    }
}
