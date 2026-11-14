//! Optical flow solver, AVX2 and FMA route: the warp fetch, the two five-point derivative kernels and the polar magnitude kernel.
//!
//! - `warp_fetch`: bilinear fetch of a source plane at positions displaced by two flow planes; positions outside the
//!   plane give +0.0, the four taps are clamped to the plane.
//! - `deriv_x` / `deriv_y`: five-point derivative along a row / along a column. Eight-wide groups gather the taps with
//!   a masked gather (all lanes enabled); the scalar form differs in rounding from the vector form.
//! - `polar`: magnitude sqrt(p*p + q*q) followed by two vector math routines (the ported routines of the vmath module).
//! Every kernel takes a row (or column) range and the alignment phase of the plane that selects the vector and scalar split.
//! Model labels: GATHER-MODEL (`gather8`), APPROX-MODEL (`rsqrt_model`), ALIGN-MODEL (phase arguments), LANE-INDEX-RAMP.
use fx_common::nanop as nn;
use fx_common::nanop::NanOps;
use fx_common::x86::{cvtss2si32_rne, sqrtss};

use crate::split;

/// One recorded gather: the lane indices relative to the plane base and the enabled lanes.
#[derive(Clone, Debug, PartialEq)]
pub struct GatherRec {
    pub site: &'static str,
    pub idx: [i32; 8],
    pub mask: [bool; 8],
}

/// GATHER-MODEL: `vgatherdps` with scale 4: lane j loads `plane[idx[j]]` when its mask lane is set, else keeps zero
/// (the destination is cleared before the instruction). The index is sign-extended.
pub fn gather8(plane: &[f32], idx: &[i32; 8], mask: &[bool; 8], site: &'static str, log: &mut Option<&mut Vec<GatherRec>>) -> [f32; 8] {
    if let Some(l) = log.as_mut() {
        l.push(GatherRec { site, idx: *idx, mask: *mask });
    }
    let mut r = [0.0f32; 8];
    for j in 0..8 {
        if mask[j] {
            r[j] = plane[idx[j] as i64 as usize];
        }
    }
    r
}

const ALL: [bool; 8] = [true; 8];

/// Cells read as constants. Listing cells in the comments; see `assumptions()`.
#[derive(Clone, Copy, Debug)]
pub struct StencilConsts {
    /// scalar scale, cell 0x15f1248
    pub k_s: f32,
    /// vector scale (splat), cell 0x15f0a20
    pub k_v: f32,
    /// scalar weight, cell 0x15e8544
    pub c_s: f32,
    /// vector weight (splat), cell 0x15f0a40
    pub c_v: f32,
}

fn lane_ramp(k: i32) -> [i32; 8] {
    let mut r = [0i32; 8];
    for (j, v) in r.iter_mut().enumerate() {
        *v = j as i32 + k;
    }
    r
}

fn row_phase_bytes(base_phase: u32, off: i32) -> u32 {
    ((base_phase as i64 + (off as i64) * 4) & 31) as u32
}

/// Five-point derivative along each row. `width`, `stride` as the task fields; rows `row0 .. row0 + rows`.
/// `out_phase` is the byte address of the output plane's first element modulo 32.
#[allow(clippy::too_many_arguments)]
pub fn deriv_x(inp: &[f32], out: &mut [f32], width: i32, stride: i32, row0: i32, rows: i32, out_phase: u32, c: &StencilConsts, mut log: Option<&mut Vec<GatherRec>>) {
    let w = width;
    for k in 0..rows.max(0) {
        let y = row0.wrapping_add(k);
        if w <= 0 {
            continue;
        }
        let roff = y.wrapping_mul(stride);
        let ph = row_phase_bytes(out_phase, roff);
        let (head, vec_end, scalar_all) = split(w as i64, 8, 0x4b0, ph);
        let (head, vec_end) = if scalar_all { (0i64, 0i64) } else { (head, vec_end) };
        let ro = |i: i32| -> usize { (roff.wrapping_add(i)) as i64 as usize };
        let scalar_at = |out: &mut [f32], i: i32| {
            let lo1 = { let t = i.wrapping_sub(1); if t < 0 { 0 } else { t } };
            let hi1 = { let t = i.wrapping_add(1); if (w - 1) >= t { t } else { w - 1 } };
            let lo2 = { let t = i.wrapping_sub(2); if t < 0 { 0 } else { t } };
            let hi2 = { let t = i.wrapping_add(2); if (w - 1) < t { w - 1 } else { t } };
            let first_lo = inp[ro(lo1)];
            let first_diff = first_lo.fsub(inp[ro(hi1)]);
            let second_lo = inp[ro(lo2)];
            let second_diff = second_lo.fsub(inp[ro(hi2)]);
            let t = nn::fms213(first_diff, c.c_s, second_diff);
            out[ro(i)] = t.fmul(c.k_s);
        };
        for i in 0..head as i32 {
            scalar_at(out, i);
        }
        let mut i = head as i32;
        if !scalar_all {
            loop {
                let ramp_prev1 = lane_ramp(i - 1);
                let ramp_prev2 = lane_ramp(i - 2);
                let ramp_next1 = lane_ramp(i + 1);
                let ramp_next2 = lane_ramp(i + 2);
                let ix = |v: &[i32; 8], f: &dyn Fn(i32) -> i32| -> [i32; 8] {
                    let mut r = [0i32; 8];
                    for j in 0..8 {
                        r[j] = f(v[j]).wrapping_add(roff);
                    }
                    r
                };
                let tap_prev1 = gather8(inp, &ix(&ramp_prev1, &|a| a.max(0)), &ALL, "dx.g1", &mut log);
                let tap_prev2 = gather8(inp, &ix(&ramp_prev2, &|a| a.max(0)), &ALL, "dx.g2", &mut log);
                let mut t = [0.0f32; 8];
                for j in 0..8 {
                    t[j] = nn::fms231(tap_prev2[j], tap_prev1[j], c.c_v);
                }
                let tap_next1 = gather8(inp, &ix(&ramp_next1, &|a| a.min(w - 1)), &ALL, "dx.g3", &mut log);
                for j in 0..8 {
                    t[j] = nn::fnma231(t[j], tap_next1[j], c.c_v);
                }
                let tap_next2 = gather8(inp, &ix(&ramp_next2, &|a| a.min(w - 1)), &ALL, "dx.g4", &mut log);
                for j in 0..8 {
                    let s = t[j].fadd(tap_next2[j]);
                    out[ro(i + j as i32)] = c.k_v.fmul(s);
                }
                i += 8;
                if (i as i64) >= vec_end {
                    break;
                }
            }
        }
        let tail_from = if scalar_all { 0 } else { vec_end as i32 };
        for i in tail_from..w {
            scalar_at(out, i);
        }
    }
}

/// Five-point derivative along each column. Columns `col0 .. col0 + cols`, `height` rows, `stride` between rows.
#[allow(clippy::too_many_arguments)]
pub fn deriv_y(inp: &[f32], out: &mut [f32], height: i32, stride: i32, col0: i32, cols: i32, c: &StencilConsts, mut log: Option<&mut Vec<GatherRec>>) {
    let h = height;
    for kc in 0..cols.max(0) {
        let col = col0.wrapping_add(kc);
        if h <= 0 {
            continue;
        }
        let col_i = col as i64;
        let scalar_at = |out: &mut [f32], y: i32, oidx: i64| {
            let lo1 = { let t = y.wrapping_sub(1); if t < 0 { 0 } else { t } };
            let hi1 = { let t = y.wrapping_add(1); if (h - 1) >= t { t } else { h - 1 } };
            let lo2 = { let t = y.wrapping_sub(2); if t < 0 { 0 } else { t } };
            let hi2 = { let t = y.wrapping_add(2); if (h - 1) < t { h - 1 } else { t } };
            let at = |r: i32| -> usize { (col_i + (r.wrapping_mul(stride)) as i64) as usize };
            let first_lo = inp[at(lo1)];
            let first_diff = first_lo.fsub(inp[at(hi1)]);
            let second_lo = inp[at(lo2)];
            let second_diff = second_lo.fsub(inp[at(hi2)]);
            let t = nn::fms213(first_diff, c.c_s, second_diff);
            out[oidx as usize] = t.fmul(c.k_s);
        };
        let mut scalar_from_row = 0i32;
        if stride != 0 && h >= 8 {
            let vend = h & !7;
            let mut r = 0i32;
            loop {
                let ix = |k: i32, f: &dyn Fn(i32) -> i32| -> [i32; 8] {
                    let ramp = lane_ramp(r + k);
                    let mut o = [0i32; 8];
                    for j in 0..8 {
                        o[j] = f(ramp[j]).wrapping_mul(stride).wrapping_add(col);
                    }
                    o
                };
                let tap_prev1 = gather8(inp, &ix(-1, &|a| a.max(0)), &ALL, "dy.g1", &mut log);
                let tap_prev2 = gather8(inp, &ix(-2, &|a| a.max(0)), &ALL, "dy.g2", &mut log);
                let mut t = [0.0f32; 8];
                for j in 0..8 {
                    t[j] = nn::fms231(tap_prev2[j], tap_prev1[j], c.c_v);
                }
                let tap_next1 = gather8(inp, &ix(1, &|a| a.min(h - 1)), &ALL, "dy.g3", &mut log);
                for j in 0..8 {
                    t[j] = nn::fnma231(t[j], tap_next1[j], c.c_v);
                }
                let tap_next2 = gather8(inp, &ix(2, &|a| a.min(h - 1)), &ALL, "dy.g4", &mut log);
                for j in 0..8 {
                    let s = t[j].fadd(tap_next2[j]);
                    let v = c.k_v.fmul(s);
                    out[(col_i + (stride as i64) * ((r + j as i32) as i64)) as usize] = v;
                }
                r += 8;
                if r >= vend {
                    break;
                }
            }
            scalar_from_row = vend;
        }
        let mut o = (stride.wrapping_mul(scalar_from_row)) as i64;
        for y in scalar_from_row..h {
            scalar_at(out, y, col_i + o);
            o += stride as i64;
        }
    }
}

/// Cells of `warp_fetch`: the lower bound (cell 0x15e852c) and the upper margin (cell 0x15f1244).
#[derive(Clone, Copy, Debug)]
pub struct FetchConsts {
    pub lo: f32,
    pub margin: f32,
}

/// Bilinear fetch. `dx`, `dy` are flow planes, `src` the source plane, rows `row0 .. row0 + rows`.
#[allow(clippy::too_many_arguments)]
pub fn warp_fetch(disp_x: &[f32], disp_y: &[f32], src: &[f32], out: &mut [f32], width: i32, height: i32, stride: i32, row0: i32, rows: i32, c: &FetchConsts) {
    for k in 0..rows.max(0) {
        let y = row0.wrapping_add(k);
        let roff = y.wrapping_mul(stride) as i64;
        if (width as i64) <= 0 {
            continue;
        }
        let yf = y as f32;
        for x in 0..width as i64 {
            let px = (x as i32 as f32).fadd(disp_x[(roff + x) as usize]);
            let py = yf.fadd(disp_y[(roff + x) as usize]);
            let zero = {
                let a = c.lo >= px;
                let b = px >= (width as f32).fsub(c.margin);
                let cc = c.lo >= py;
                let hm = (height as f32).fsub(c.margin);
                let d = !(py < hm || py.is_nan() || hm.is_nan());
                a || b || cc || d
            };
            if zero {
                out[(roff + x) as usize] = 0.0;
                continue;
            }
            let ix0 = cvtss2si32_rne(px.floor());
            let iy0 = cvtss2si32_rne(py.floor());
            let ix1 = ix0.wrapping_add(1);
            let iy1 = iy0.wrapping_add(1);
            let ix0_f = ix0 as f32;
            let fx = px.fsub(ix0_f);
            let clamp_x = |v: i32| -> i32 { if v < 0 { 0 } else { (width - 1).min(v) } };
            let cy = |v: i32| -> i32 { if v < 0 { 0 } else { (height - 1).min(v) } };
            let at = |xx: i32, yy: i32| -> f32 { src[(cy(yy).wrapping_mul(stride) as i64 + clamp_x(xx) as i64) as usize] };
            let tap_00 = at(ix0, iy0);
            let tap_10 = at(ix1, iy0);
            let tap_01 = at(ix0, iy1);
            let tap_11 = at(ix1, iy1);
            let iy0_f = iy0 as f32;
            let fy = py.fsub(iy0_f);
            let top_diff = tap_10.fsub(tap_00);
            let bottom_diff = tap_11.fsub(tap_01);
            let left_diff = tap_00.fsub(tap_01);
            let cross_diff = bottom_diff.fsub(top_diff);
            let top = nn::fma213(top_diff, fx, tap_00);
            let cross_diff = nn::fms213(cross_diff, fx, left_diff);
            out[(roff + x) as usize] = nn::fma213(fy, cross_diff, top);
        }
    }
}

/// Cells of `polar`.
#[derive(Clone, Copy, Debug)]
pub struct PolarConsts {
    /// second argument of the first routine (splat), cell 0x15f0c40
    pub arg2: f32,
    /// scalar multiplier, cell 0x15f1250
    pub mul_s: f32,
    /// vector multiplier (splat), cell 0x15f0c60
    pub mul_v: f32,
    /// absolute value mask bits, cell 0x15f0c20
    pub abs_mask: u32,
    /// subtracted constant of the refinement, cell 0x15f0c80
    pub sub: f32,
    /// final factor of the refinement, cell 0x15f0ca0
    pub fin: f32,
    /// threshold of the lane compare, cell 0x15f0cc0
    pub thr: f32,
}

/// APPROX-MODEL: the reciprocal square root estimate (vrsqrtps) modelled as the exact 1 / sqrt(x), two roundings.
pub fn rsqrt_model(x: f32) -> f32 {
    1.0f32.fdiv(sqrtss(x))
}

/// The refinement that follows the estimate (listing 0x479626..0x479651), for one lane, given the estimate `est`.
pub fn sqrt_refine(s: f32, est: f32, c: &PolarConsts) -> f32 {
    let a = f32::from_bits(s.to_bits() & c.abs_mask);
    let m = if a >= c.thr { u32::MAX } else { 0 };
    let est_masked = f32::from_bits(m & est.to_bits());
    let est_scaled = s.fmul(est_masked);
    let est_sq = est_scaled.fmul(est_masked);
    let est_corr = est_sq.fsub(c.sub);
    let est_prod = est_scaled.fmul(est_corr);
    est_prod.fmul(c.fin)
}

/// Magnitude and the two routines. `phase` is the byte address of the `p` plane's first element modulo 32.
#[allow(clippy::too_many_arguments)]
pub fn polar(p: &[f32], q: &[f32], out: &mut [f32], width: i32, stride: i32, row0: i32, rows: i32, phase: u32, c: &PolarConsts) {
    let w = width as i64;
    for k in 0..rows.max(0) {
        let y = row0.wrapping_add(k);
        if w <= 0 {
            continue;
        }
        let roff = y.wrapping_mul(stride) as i64;
        let ph = row_phase_bytes(phase, roff as i32);
        let (head, vec_end, scalar_all) = split(w, 8, 0x4b0, ph);
        let (head, vec_end) = if scalar_all { (0, 0) } else { (head, vec_end) };
        let scalar_at = |out: &mut [f32], i: i64| {
            let pv = p[(roff + i) as usize];
            let p_sq = pv.fmul(pv);
            let qv = q[(roff + i) as usize];
            let s = sqrtss(nn::fma213(qv, qv, p_sq));
            let r = f32::from_bits(crate::vmath::powf_lane(s.to_bits(), c.arg2.to_bits()));
            let m = c.mul_s.fmul(r);
            out[(roff + i) as usize] = f32::from_bits(crate::vmath::expf_lane(m.to_bits()));
        };
        for i in 0..head {
            scalar_at(out, i);
        }
        let mut i = head;
        if !scalar_all {
            loop {
                for j in 0..8i64 {
                    let idx = (roff + i + j) as usize;
                    let pv = p[idx];
                    let p_sq = pv.fmul(pv);
                    let qv = q[idx];
                    let norm_sq = nn::fma213(qv, qv, p_sq);
                    let est = rsqrt_model(norm_sq);
                    let norm = sqrt_refine(norm_sq, est, c);
                    let r = f32::from_bits(crate::vmath::powf_lane(norm.to_bits(), c.arg2.to_bits()));
                    let m = c.mul_v.fmul(r);
                    out[idx] = f32::from_bits(crate::vmath::expf_lane(m.to_bits()));
                }
                i += 8;
                if i >= vec_end {
                    break;
                }
            }
        }
        let tail = if scalar_all { 0 } else { vec_end };
        for i in tail..w {
            scalar_at(out, i);
        }
    }
}

/// Stand-in labels of this module.
pub fn fetch_assumptions() -> Vec<(&'static str, &'static str)> {
    vec![
        ("GATHER-MODEL", "vgatherdps scale 4, all lanes enabled in every use; lane j loads plane[idx[j]], index sign-extended"),
        ("APPROX-MODEL", "the vector reciprocal square root modelled as the exact 1/sqrt(x), two roundings; APPROX-BOUND is measured on the port"),
        ("LIBM-PORTED", "the library routines are the image's own CPU-dispatched vector routines; the AVX2/FMA variants are ported in vmath (ROUTE AVX2-ONLY, tables computed by code)"),
        ("CELL-MEASURED", "the warp fetch cells for the lower bound and the upper margin are words read from the original at their readers (MEASURED-EMULATOR-READ), carried as the named constants WARP_LO_BITS and MARGIN_BITS"),
        ("LANE-INDEX-RAMP", "the lane ramp cells taken as the lane ramps minus 1, minus 2, plus 1, plus 2 by analogy with the gather launch, computed by code"),
        ("ALIGN-MODEL", "the output (or p) plane's byte address modulo 32 is an argument"),
    ]
}
