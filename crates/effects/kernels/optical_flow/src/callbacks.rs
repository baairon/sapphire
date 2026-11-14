use fx_common::nanop::sd;
use fx_common::nanop::{fma213, fma231, fms213, fnma213, NanOps};

const ONE: f32 = 1.0; // cell 0x15e850c and the eight lane cell 0x15f0ae0
const W_P1: f64 = 0.5; // cell 0x15e9378 and the four lane cell 0x160ee40
const W_P0: f64 = 0.3125; // cell 0x15eb108 and the four lane cell 0x160ee20
const W_P2: f64 = 0.1875; // cell 0x15eb110 and the four lane cell 0x160ee60

#[inline(always)]
fn at(row: i32, stride: i32, x: i64) -> usize {
    (stride.wrapping_mul(row) as i64 + x) as usize
}

/// Elements before the first 32 byte boundary of a row that starts `byte_phase` bytes (mod 32) into a block. None when the
/// row start is not 4 byte aligned.
#[inline(always)]
fn head_len(byte_phase: usize) -> Option<usize> {
    let m = byte_phase & 31;
    if m == 0 { Some(0) } else if m & 3 != 0 { None } else { Some((32 - m) / 4) }
}

/// Byte phase (mod 32) of element `idx` of a plane whose element 0 sits `align0` bytes past a 32 byte boundary.
#[inline(always)]
fn phase(align0: usize, idx: i64) -> usize {
    ((align0 as i64 + 4 * idx).rem_euclid(32)) as usize
}

/// Row sum callback: sums `rows` rows from `row0` of `data` (width `width`, row pitch `stride`) into one partial sum.
/// A vector row (width at least 64) adds eight accumulators of eight lanes, folds them as the original does and adds the
/// remainder; a short row adds element by element. The returned value is the register the callback hands to the
/// per thread slot store.
pub fn sum_rows(data: &[f32], width: i32, stride: i32, row0: i32, rows: i32, align0: usize) -> f32 {
    let mut total = 0.0f32;
    if row0 >= row0.wrapping_add(rows) {
        return total;
    }
    for k in 0..rows {
        let row = row0 + k;
        if width <= 0 {
            continue;
        }
        let w = width as i64;
        let base = at(row, stride, 0);
        let r = &data[base..];
        let mut y = [0.0f32; 8];
        let head = if w < 0x40 { None } else { head_len(phase(align0, base as i64)) };
        let vec_path = match head {
            Some(h) if w >= h as i64 + 0x40 => Some(h),
            _ => None,
        };
        match vec_path {
            None => {
                for i in 0..w as usize {
                    total = total.fadd(r[i]);
                }
            }
            Some(h) => {
                for i in 0..h {
                    total = total.fadd(r[i]);
                }
                let end = w - ((w - h as i64) & 0x3f);
                let mut a = [[0.0f32; 8]; 8];
                let mut p = h as i64;
                while p < end {
                    for blk in 0..8 {
                        for l in 0..8 {
                            a[blk][l] = a[blk][l].fadd(r[p as usize + 8 * blk + l]);
                        }
                    }
                    p += 64;
                }
                for l in 0..8 {
                    let pair_01 = a[0][l].fadd(a[1][l]);
                    let pair_23 = a[2][l].fadd(a[3][l]);
                    let pair_45 = a[4][l].fadd(a[5][l]);
                    let pair_67 = a[6][l].fadd(a[7][l]);
                    let quad_0123 = pair_01.fadd(pair_23);
                    let quad_4567 = pair_45.fadd(pair_67);
                    y[l] = quad_0123.fadd(quad_4567);
                }
                if end < w {
                    let cnt = (w - end) as usize;
                    let vecn = if cnt < 8 { 0 } else { cnt & !7 };
                    let mut j = 0;
                    while j < vecn {
                        for l in 0..8 {
                            y[l] = y[l].fadd(r[end as usize + j + l]);
                        }
                        j += 8;
                    }
                    for j in vecn..cnt {
                        total = total.fadd(r[end as usize + j]);
                    }
                }
            }
        }
        let x: [f32; 4] = [y[0].fadd(y[4]), y[1].fadd(y[5]), y[2].fadd(y[6]), y[3].fadd(y[7])];
        let half_even = x[0].fadd(x[2]);
        let half_odd = x[1].fadd(x[3]);
        total = half_even.fadd(half_odd).fadd(total);
    }
    total
}

/// In place affine map of rows: `v = (v - offset) * scale - 1.0` with the multiply and subtract fused.
pub fn affine_rows(data: &mut [f32], width: i32, stride: i32, row0: i32, rows: i32, scale: f32, offset: f32, align0: usize) {
    if row0 >= row0.wrapping_add(rows) {
        return;
    }
    for k in 0..rows {
        let row = row0 + k;
        if width <= 0 {
            continue;
        }
        let w = width as i64;
        let base = at(row, stride, 0);
        let (hc, vc) = if w < 16 {
            (0, 0)
        } else {
            match head_len(phase(align0, base as i64)) {
                Some(h) if w >= h as i64 + 16 => (h, (w - ((w - h as i64) & 15)) as usize),
                _ => (0, 0),
            }
        };
        let r = &mut data[base..];
        for i in 0..hc {
            r[i] = fms213(r[i].fsub(offset), scale, ONE); // site 46fba4
        }
        let mut i = hc;
        while i < vc {
            for l in 0..8 {
                r[i + l] = fms213(r[i + l].fsub(offset), scale, ONE); // site 46fbcc
            }
            for l in 8..16 {
                r[i + l] = fms213(r[i + l].fsub(offset), scale, ONE); // site 46fbd1
            }
            i += 16;
        }
        for i in vc..w as usize {
            r[i] = fms213(r[i].fsub(offset), scale, ONE); // site 46fc0c
        }
    }
}

/// Flow normalisation over rows: `p` is the reference plane, `fx` and `fy` the two flow planes updated in place.
/// `scale = num / den`. Each element forms the plane gradient, adds the flow scaled by `scale` and normalises the
/// pair to unit length when its squared length exceeds 1.0. The last row and the last column use the available
/// neighbours only.
pub fn normalize_flow_rows(p: &[f32], fx: &mut [f32], fy: &mut [f32], width: i32, total_rows: i32, stride: i32, row0: i32, rows: i32, num: f32, den: f32) {
    let s = num.fdiv(den); // site 477b1a
    if row0 >= row0.wrapping_add(rows) {
        return;
    }
    let last = total_rows.wrapping_sub(1);
    for k in 0..rows {
        let y = row0 + k;
        if width <= 0 {
            continue;
        }
        let w = width as i64;
        for x in 0..w {
            let i = at(y, stride, x);
            if y < last {
                let below = at(y.wrapping_add(1), stride, x);
                if x < w - 1 {
                    let a = p[i];
                    let b = p[below];
                    let c = p[i + 1];
                    let gy0 = b.fsub(a);
                    let gx0 = c.fsub(a);
                    let gy = fma213(gy0, s, fy[i]); // site 477bf7
                    let gx = fma213(gx0, s, fx[i]); // site 477bfd
                    let norm_sq = fma231(gy.fmul(gy), gx, gx); // site 477c07
                    if norm_sq > ONE {
                        let n = norm_sq.sqrt();
                        fx[i] = gx.fdiv(n);
                        fy[i] = gy.fdiv(n);
                    } else {
                        fx[i] = gx;
                        fy[i] = gy;
                    }
                } else {
                    let fxo = fx[i];
                    let gy0 = p[below].fsub(p[i]);
                    let gy = fma213(gy0, s, fy[i]); // site 477c4b
                    let norm_sq = fma231(gy.fmul(gy), fxo, fxo); // site 477c55
                    if norm_sq > ONE {
                        let n = norm_sq.sqrt();
                        fx[i] = fxo.fdiv(n);
                        fy[i] = gy.fdiv(n);
                    } else {
                        fy[i] = gy;
                    }
                }
            } else if x < w - 1 {
                let fyo = fy[i];
                let gx0 = p[i + 1].fsub(p[i]);
                let gx = fma213(gx0, s, fx[i]); // site 477d0e
                let norm_sq = fma231(fyo.fmul(fyo), gx, gx); // site 477d14
                if norm_sq > ONE {
                    let n = norm_sq.sqrt();
                    fx[i] = gx.fdiv(n);
                    fy[i] = fyo.fdiv(n);
                } else {
                    fx[i] = gx;
                }
            } else {
                let fyo = fy[i];
                let fxo = fx[i];
                let norm_sq = fma231(fyo.fmul(fyo), fxo, fxo); // site 477d52
                if norm_sq > ONE {
                    let n = norm_sq.sqrt();
                    fx[i] = fxo.fdiv(n);
                    fy[i] = fyo.fdiv(n);
                }
            }
        }
    }
}

/// Scaled difference over rows: `out = b - a * s` with one rounding.
pub fn sub_scaled_rows(a: &[f32], b: &[f32], out: &mut [f32], width: i32, stride: i32, row0: i32, rows: i32, s: f32, align0: usize) {
    if row0 >= row0.wrapping_add(rows) {
        return;
    }
    for k in 0..rows {
        let row = row0 + k;
        if width <= 0 {
            continue;
        }
        let w = width as i64;
        let base = at(row, stride, 0);
        let (hc, vc) = if w < 16 {
            (0, 0)
        } else if w < 0x1d {
            (0, (w & !15) as usize)
        } else {
            match head_len(phase(align0, base as i64)) {
                Some(h) if w >= h as i64 + 16 => (h, (w - ((w - h as i64) & 15)) as usize),
                _ => (0, 0),
            }
        };
        for i in 0..hc {
            out[base + i] = fnma213(a[base + i], s, b[base + i]); // site 47828e
        }
        let mut i = hc;
        while i < vc {
            for l in 0..8 {
                out[base + i + l] = fnma213(a[base + i + l], s, b[base + i + l]); // site 4782af
            }
            for l in 8..16 {
                out[base + i + l] = fnma213(a[base + i + l], s, b[base + i + l]); // site 4782b5
            }
            i += 16;
        }
        for i in vc..w as usize {
            out[base + i] = fnma213(a[base + i], s, b[base + i]); // site 4782f0
        }
    }
}

/// Weighted blend of three planes over `n` elements: `out = float(0.5 * middle_plane + 0.3125 * first_plane + 0.1875 * last_plane)` evaluated in
/// double precision with two fused steps and one narrowing rounding. `align0` is the byte phase of `out[0]`.
pub fn blend3(first_plane: &[f32], middle_plane: &[f32], last_plane: &[f32], out: &mut [f32], n: i64, align0: usize) {
    if n <= 0 {
        return;
    }
    let n = n;
    let (hc, vc) = if n < 8 {
        (0, 0)
    } else if n < 0x31d {
        (0, (n & !7) as usize)
    } else {
        match head_len(align0 & 31) {
            Some(h) if n >= h as i64 + 8 => (h, (n - ((n - h as i64) & 7)) as usize),
            _ => (0, 0),
        }
    };
    for i in 0..hc {
        let t = sd::mul(W_P1, sd::cvtss2sd(middle_plane[i]));
        let t = sd::fma231(t, sd::cvtss2sd(first_plane[i]), W_P0); // site 676320
        let t = sd::fma231(t, sd::cvtss2sd(last_plane[i]), W_P2); // site 676325
        out[i] = sd::cvtsd2ss(t);
    }
    let mut i = hc;
    while i < vc {
        for half in 0..2 {
            for l in 0..4 {
                let j = i + 4 * half + l;
                let t = sd::mul(W_P1, sd::cvtss2sd(middle_plane[j]));
                let t = if half == 0 { sd::fma231(t, sd::cvtss2sd(first_plane[j]), W_P0) } else { sd::fma231(t, sd::cvtss2sd(first_plane[j]), W_P0) }; // sites 676394, 676399
                let t = if half == 0 { sd::fma231(t, sd::cvtss2sd(last_plane[j]), W_P2) } else { sd::fma231(t, sd::cvtss2sd(last_plane[j]), W_P2) }; // sites 6763ae, 6763b3
                out[j] = sd::cvtsd2ss(t);
            }
        }
        i += 8;
    }
    for i in vc..n as usize {
        let t = sd::mul(W_P1, sd::cvtss2sd(middle_plane[i]));
        let t = sd::fma231(t, sd::cvtss2sd(first_plane[i]), W_P0); // site 676415
        let t = sd::fma231(t, sd::cvtss2sd(last_plane[i]), W_P2); // site 67641a
        out[i] = sd::cvtsd2ss(t);
    }
}

/// Constant add over a tile: `out = c + out`; a constant equal to zero (either sign) zero fills the tile.
pub fn add_const(out: &mut [f32], n: i64, c: f32) {
    if c == 0.0 {
        for v in out.iter_mut().take(n.max(0) as usize) {
            *v = 0.0;
        }
        return;
    }
    for v in out.iter_mut().take(n.max(0) as usize) {
        *v = c.fadd(*v);
    }
}
