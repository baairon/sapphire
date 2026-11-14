//! blur_warp_bidirectional body B (mode 2) as a delta against body A (crate blur_warp_1ch).
//! S0..S5 are shared with A (same arithmetic, other registers and frame offsets: body B listing vs body A listing); they are reused through
//! blur_warp_1ch::{s0_scales, s2_x, s3_y}. The delta is in S6..S12: B samples a TWO-CHANNEL source (interleaved float pairs, index 2*col) and writes TWO output planes
//! (the two output pointers); the lerp is computed in negated form (v0-v1)*frac-v0 and subtracted. Source of truth: the body B listing. Names INFERRED, arithmetic MEASURED.

use blur_warp_1ch::p2::{Host, P1Cfg, Plane};
use blur_warp_1ch::*;

pub use blur_warp_1ch::Assumption;

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "BB1", basis: "S0..S5 of B are mnemonic- and operand-identical to A up to register and frame-offset renaming (diff of the body B and body A listings); the A helpers s0_scales/s2_x/s3_y are reused; the same FMA sites (vfmadd132ss), (vfmsub213ss), (vfmadd132ss), (vfnmadd132ss)", label: "MEASURED listing diff" },
        Assumption { id: "BB2", basis: "source plane holds interleaved float pairs: B reads v[idx] and v[idx+1], idx = 2*col + row*stride (i32 wrapping lea, movsxd); stride (result) counts floats per row", label: "MEASURED " },
        Assumption { id: "BB3", basis: "two output planes: epilogue stores sum0*scale to [[out]+4*i] and scale*sum1 to [[second output]+4*i]; zero-length stub multiplies planes 0 AND 1 by a4; the stub needs P1 to have written plane 1 (P1 nch >= 2 in T2)", label: "MEASURED " },
        Assumption { id: "BB4", basis: "negated lerp: lerp' = fma(v0-v1, frac, -v0) = -lerp exactly (sub and FMA round symmetrically), sum = sum - lerp'*w; ported literally. FMA sites X-major: (lerp ch0, ch1), Y-major: ", label: "MEASURED listing" },
        Assumption { id: "BB5", basis: "other FMA sites: (vfnmadd231ss, X-major y start), (twoh vfmsub213ss), (alpha vfmadd213ss), (vfnmadd231ss Y-major x start), (twow vfmsub213ss), (alpha); 18 sites in total, no rcp/rsqrt/x87", label: "MEASURED listing grep vf" },
        Assumption { id: "BB6", basis: "section bodies S6..S10 beyond the loops (floor/ceil, n, clamps b_cell_c etc.) assumed equal to A (scheduling differs); verified only by the plane T2 against the emulator", label: "ASSUMPTION, verified by T2" },
        Assumption { id: "BB7", basis: "host statics ..2b, (ws), (hs) and results are explicit inputs as in A (B2, B9)", label: "ASSUMPTION" },
        Assumption { id: "BB8", basis: "emulator follows the SDM, not silicon; plane reads outside the modelled buffer give Unsupported::Oob (as B8 in A)", label: "ASSUMPTION" },
    ]
}

fn ja(a: f32, b: f32) -> bool {
    a > b
}
fn jae(a: f32, b: f32) -> bool {
    a >= b
}
fn vmin(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

fn at(pl: &Plane, idx: i64) -> Result<f32, Unsupported> {
    let k = pl.origin + idx;
    if k < 0 || k as usize >= pl.buf.len() {
        return Err(Unsupported::Oob);
    }
    Ok(pl.buf[k as usize])
}

/// 32-bit index arithmetic as in the listing (imul / lea / add on 32-bit registers), then sign extension.
fn ix(row_stride: i32, col2: i32) -> i64 {
    row_stride.wrapping_add(col2) as i64
}

/// negated lerp term and accumulate: s - (fma(v0 - v1, frac, -v0) * w)
fn acc_lerp(s: f32, sample_a: f32, sample_b: f32, frac: f32, w: f32) -> f32 {
    let l = (sample_a - sample_b).mul_add(frac, -sample_a);
    s - l * w
}

/// X-major sample (S6, S7/S8, epilogue): returns (plane0, plane1)
pub fn x_major_sample2(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<(f32, f32), Unsupported> {
    let (w, h, stride) = (st.width, st.height, st.stride);
    let wf = w as f32;
    let hs = hst.hs;
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cond_any = mx0 != 0 || my > 0;
    let (alpha_init, alpha_origin, tap_width, norm_num) = (a[0], a[1], a[3], a[4]);
    let (sum0, sum1, wsum);
    if st.flag_cell_a != 0 {
        let (x, y, delta_x, dy) = (st.x, st.y, st.delta_x, st.dy);
        let (mut tap_a, mut tap_c, tap_b, tap_span, fx, edge_gap, dxs);
        if delta_x > 0.0 {
            let fl = x.floor();
            tap_a = 1i32;
            fx = x - fl;
            tap_c = cvtt(fl);
            let ce = st.e2x.ceil();
            tap_b = cvtt(ce);
            edge_gap = ce - st.e2x;
            tap_span = tap_b.wrapping_sub(tap_c).wrapping_add(1);
            dxs = delta_x;
        } else {
            let ce = x.ceil();
            tap_a = -1;
            tap_c = cvtt(ce);
            let fl = st.e2x.floor();
            edge_gap = st.e2x - fl;
            fx = ce - x;
            tap_b = cvtt(fl);
            tap_span = tap_c.wrapping_sub(tap_b).wrapping_add(1);
            dxs = -delta_x;
        }
        let mut slope = dy / dxs;
        let mut yy = (-slope).mul_add(fx, y); // vfnmadd231ss 
        let step_a = st.a_gap_first / ((tap_span as f32) * tap_width);
        let floor_y = yy.floor();
        let y_end = slope + st.e2y;
        let (b_cell_c, direct0) = if cond_any {
            let o = tap_c < 0 || tap_c >= w || ja(0.0, floor_y) || jae(floor_y, hs) || tap_b < 0 || tap_b >= w || ja(0.0, y_end) || jae(y_end, hs);
            (o, !o)
        } else {
            (false, false)
        };
        let b_cell_g = if direct0 || mx0 != 0 { false } else { tap_c < 0 || tap_c >= w || tap_b < 0 || tap_b >= w };
        let b_cell_h = if my != 0 { false } else { ja(0.0, floor_y) || jae(floor_y, hs) || ja(0.0, y_end) || jae(y_end, hs) };
        let n = tap_span.min(1000).max(0);
        let (mut s0, mut sum_ch1, mut ws) = (0.0f32, 0.0f32, 0.0f32);
        if n > 0 {
            let hfi = cvtt(hs) as f32;
            let omfx = 1.0 - fx;
            let omc0 = 1.0 - edge_gap;
            let nf = n as f32;
            let wlim = nf * tap_width;
            let step_b = st.a_gap_second / (nf * st.one_minus_a_last);
            let twoh = 2.0f32.mul_add(hfi, -1.0); // vfmsub213ss 
            let colmax = cvtt(wf - 1.0);
            let hf = h as f32;
            let mut alpha = alpha_init;
            let mut step = step_a;
            let mut at_limit = false;
            for k in 0..n {
                if !at_limit && (k as f32) >= wlim {
                    let q = k / n;
                    alpha = (tap_width - (q as f32)).mul_add(step_b, alpha_origin); // vfmadd213ss 
                    step = step_b;
                    at_limit = true;
                }
                if b_cell_c {
                    if (tap_c as f32) >= wf {
                        if hst.port_blk_a != 0 {
                            tap_c = colmax;
                            tap_a = -1;
                        } else if hst.cassumption_b != 0 {
                            tap_c = 0;
                        }
                    } else if tap_c < 0 {
                        if hst.port_blk_a != 0 {
                            tap_c = 0;
                            tap_a = 1;
                        } else if hst.cassumption_b != 0 {
                            tap_c = colmax;
                        }
                    }
                    if !(yy < hfi) {
                        if hst.cassumption_a != 0 {
                            slope = -slope;
                            yy = twoh - yy;
                        } else if hst.cassumption_c != 0 {
                            yy = yy - hfi;
                        }
                    } else if ja(0.0, yy) {
                        if hst.cassumption_a != 0 {
                            yy = -yy;
                            slope = -slope;
                        } else if hst.cassumption_c != 0 {
                            yy = yy + hfi;
                        }
                    }
                }
                let wgt = if k == 0 { alpha * omfx } else if k == n - 1 { alpha * omc0 } else { alpha };
                let c = if b_cell_g { if tap_c >= -1 { tap_c.min(w) } else { -1 } } else { tap_c };
                let ycl = if b_cell_h { if ja(-1.0, yy) { -1.0 } else { vmin(hf, yy) } } else { yy };
                ws = wgt + ws;
                yy = slope + yy;
                alpha += step;
                tap_c = tap_c.wrapping_add(tap_a);
                let fr = 1.0 + ycl;
                let t = cvtt(fr);
                let frac = fr - (t as f32);
                // edx = 2*c - stride + t*stride (32-bit)
                let idx = c.wrapping_add(c).wrapping_sub(stride).wrapping_add(t.wrapping_mul(stride)) as i64;
                let (sample_0_a, sample_0_b) = (at(pl, idx)?, at(pl, idx + 1)?);
                let (sample_1_a, sample_1_b) = (at(pl, idx + stride as i64)?, at(pl, idx + stride as i64 + 1)?);
                s0 = acc_lerp(s0, sample_0_a, sample_1_a, frac, wgt); // vfmsub213ss 
                sum_ch1 = acc_lerp(sum_ch1, sample_0_b, sample_1_b, frac, wgt); // vfmsub213ss 
            }
        }
        sum0 = s0;
        sum1 = sum_ch1;
        wsum = ws;
    } else {
        let xa = 0.5 + st.x;
        let xe = 0.5 + st.e2x;
        let mut ya = 0.5 + st.y;
        let ye = 0.5 + st.e2y;
        let b_cell_i = if cond_any { ja(0.0, xe) || jae(xe, wf) || ja(0.0, ye) || jae(ye, hs) } else { false };
        let b_cell_k = if mx0 != 0 { false } else { ja(0.0, xa) || jae(xa, wf) || ja(0.0, xe) || jae(xe, wf) };
        let b_cell_j = if my != 0 { false } else { ja(0.0, ya) || jae(ya, hs) || ja(0.0, ye) || jae(ye, hs) };
        let mut cs = cvtt(xa);
        let e = cvtt(xe).wrapping_sub(cs);
        let m = (e ^ (e >> 31)).wrapping_sub(e >> 31).wrapping_add(1);
        let mut tap_a = if e < 0 { -1i32 } else { 1 };
        let mf = m as f32;
        let mut ystep = (ye - ya) / mf;
        let step_a = st.a_gap_first / (mf * tap_width);
        let count = m.min(1000).max(0);
        let (mut s0, mut sum_ch1, mut ws) = (0.0f32, 0.0f32, 0.0f32);
        if count > 0 {
            let cf = count as f32;
            let step_b = st.a_gap_second / (cf * st.one_minus_a_last);
            let wlim = cf * tap_width;
            let hsi = cvtt(hs) as f32;
            let twoh = 2.0f32.mul_add(hsi, -1.0); // vfmsub213ss 
            let colmax = cvtt(wf - 1.0);
            let mut alpha = alpha_init;
            let mut step = step_a;
            let mut at_limit = false;
            for k in 0..count {
                let mut rown = cvtt(ya);
                if !at_limit && (k as f32) >= wlim {
                    let q = k / count;
                    alpha = (tap_width - (q as f32)).mul_add(step_b, alpha_origin); // vfmadd213ss 
                    step = step_b;
                    at_limit = true;
                }
                let col: i32 = if b_cell_k { if cs >= -1 { cs.min(w) } else { -1 } } else { cs };
                if b_cell_j {
                    rown = if rown >= -1 { rown.min(h) } else { -1 };
                }
                let idx = ix(rown.wrapping_mul(stride), col.wrapping_mul(2));
                cs = cs.wrapping_add(tap_a);
                ws = ws + alpha;
                ya = ya + ystep;
                let va = at(pl, idx)?;
                let vb = at(pl, idx + 1)?;
                let p0 = alpha * va;
                let weighted_b = alpha * vb;
                alpha = alpha + step;
                s0 = p0 + s0;
                sum_ch1 = weighted_b + sum_ch1;
                if b_cell_i {
                    if (cs as f32) >= wf {
                        if hst.port_blk_a != 0 {
                            tap_a = -1;
                            cs = colmax;
                        } else if hst.cassumption_b != 0 {
                            cs = 0;
                        }
                    } else if cs < 0 {
                        if hst.port_blk_a != 0 {
                            cs = 0;
                            tap_a = 1;
                        } else if hst.cassumption_b != 0 {
                            cs = colmax;
                        }
                    }
                    if !(ya < hsi) {
                        if hst.cassumption_a != 0 {
                            ystep = -ystep;
                            ya = twoh - ya;
                        } else if hst.cassumption_c != 0 {
                            ya = ya - hsi;
                        }
                    } else if ja(0.0, ya) {
                        if hst.cassumption_a != 0 {
                            ya = -ya;
                            ystep = -ystep;
                        } else if hst.cassumption_c != 0 {
                            ya = ya + hsi;
                        }
                    }
                }
            }
        }
        sum0 = s0;
        sum1 = sum_ch1;
        wsum = ws;
    }
    let scale = norm_num / wsum;
    Ok((sum0 * scale, scale * sum1))
}

/// Y-major sample (S9, S10, epilogue): returns (plane0, plane1)
pub fn y_major_sample2(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<(f32, f32), Unsupported> {
    let (w, h, stride) = (st.width, st.height, st.stride);
    let (wf, hf) = (w as f32, h as f32);
    let ws = hst.ws;
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cond_any = mx0 != 0 || my > 0;
    let (alpha_init, alpha_origin, tap_width, norm_num) = (a[0], a[1], a[3], a[4]);
    let (sum0, sum1, wsum);
    if st.flag_cell_a != 0 {
        let (x, y, delta_x, dy) = (st.x, st.y, st.delta_x, st.dy);
        let (mut tap_a, tap_b, tap_span, fy, edge_gap, dys, mut dir);
        if dy > 0.0 {
            let fl = y.floor();
            fy = y - fl;
            tap_a = cvtt(fl);
            let ce = st.e2y.ceil();
            edge_gap = ce - st.e2y;
            tap_b = cvtt(ce);
            tap_span = tap_b.wrapping_sub(tap_a).wrapping_add(1);
            dir = 1i32;
            dys = dy;
        } else {
            let ce = y.ceil();
            tap_a = cvtt(ce);
            fy = ce - y;
            let fl = st.e2y.floor();
            edge_gap = st.e2y - fl;
            tap_b = cvtt(fl);
            tap_span = tap_a.wrapping_sub(tap_b).wrapping_add(1);
            dir = -1;
            dys = -dy;
        }
        let mut slope = delta_x / dys;
        let mut xx = (-slope).mul_add(fy, x); // vfnmadd231ss 
        let step_a = st.a_gap_first / ((tap_span as f32) * tap_width);
        let floor_x = xx.floor();
        let sum_9 = slope + st.e2x;
        let (b_cell_c, direct0) = if cond_any {
            let o = ja(0.0, floor_x) || jae(floor_x, ws) || tap_a < 0 || tap_a >= h || ja(0.0, sum_9) || jae(sum_9, ws) || tap_b < 0 || tap_b >= h;
            (o, !o)
        } else {
            (false, false)
        };
        let outside_x = if direct0 || mx0 != 0 { false } else { ja(0.0, floor_x) || jae(floor_x, ws) || ja(0.0, sum_9) || jae(sum_9, ws) };
        let tap_c = if my != 0 { false } else { tap_a < 0 || tap_a >= h || tap_b < 0 || tap_b >= h };
        let n = tap_span.min(1000).max(0);
        let (mut s0, mut sum_ch1, mut wsm) = (0.0f32, 0.0f32, 0.0f32);
        if n > 0 {
            let omfy = 1.0 - fy;
            let omc0 = 1.0 - edge_gap;
            let nf = n as f32;
            let wlim = nf * tap_width;
            let step_b = st.a_gap_second / (nf * st.one_minus_a_last);
            let twow = 2.0f32.mul_add(ws, -1.0); // vfmsub213ss 
            let mut alpha = alpha_init;
            let mut step = step_a;
            let mut at_limit = false;
            for k in 0..n {
                if !at_limit && (k as f32) >= wlim {
                    let q = k / n;
                    alpha = (tap_width - (q as f32)).mul_add(step_b, alpha_origin); // vfmadd213ss 
                    step = step_b;
                    at_limit = true;
                }
                if b_cell_c {
                    if !(xx < ws) {
                        if hst.port_blk_a != 0 {
                            slope = -slope;
                            xx = twow - xx;
                        } else if hst.cassumption_b != 0 {
                            xx = xx - ws;
                        }
                    } else if ja(0.0, xx) {
                        if hst.port_blk_a != 0 {
                            xx = -xx;
                            slope = -slope;
                        } else if hst.cassumption_b != 0 {
                            xx = xx + ws;
                        }
                    }
                    if tap_a >= h {
                        if hst.cassumption_a != 0 {
                            tap_a = h - 1;
                            dir = -1;
                        } else if hst.cassumption_c != 0 {
                            tap_a = 0;
                        }
                    } else if tap_a < 0 {
                        if hst.cassumption_a != 0 {
                            tap_a = 0;
                            dir = 1;
                        } else if hst.cassumption_c != 0 {
                            tap_a = h - 1;
                        }
                    }
                }
                let wgt = if k == 0 { alpha * omfy } else if k == n - 1 { alpha * omc0 } else { alpha };
                let xc = if outside_x { if ja(-1.0, xx) { -1.0 } else { vmin(wf, xx) } } else { xx };
                let rr: i32 = if tap_c { if tap_a >= -1 { tap_a.min(h) } else { -1 } } else { tap_a };
                let rs = rr.wrapping_mul(stride);
                wsm = wgt + wsm;
                xx = slope + xx;
                alpha += step;
                let fr = 1.0 + xc;
                let t = cvtt(fr);
                let frac = fr - (t as f32);
                tap_a = tap_a.wrapping_add(dir);
                let idx = ix(rs, t.wrapping_mul(2));
                let (sample_0_a, sample_0_b) = (at(pl, idx - 2)?, at(pl, idx - 1)?);
                let (sample_1_a, sample_1_b) = (at(pl, idx)?, at(pl, idx + 1)?);
                s0 = acc_lerp(s0, sample_0_a, sample_1_a, frac, wgt); // vfmsub213ss 
                sum_ch1 = acc_lerp(sum_ch1, sample_0_b, sample_1_b, frac, wgt); // vfmsub213ss 
            }
        }
        sum0 = s0;
        sum1 = sum_ch1;
        wsum = wsm;
    } else {
        let xe = 0.5 + st.e2x;
        let xa = 0.5 + st.x;
        let ya = 0.5 + st.y;
        let ye = 0.5 + st.e2y;
        let b_cell_d = if cond_any { ja(0.0, xe) || jae(xe, ws) || ja(0.0, ye) || jae(ye, hf) } else { false };
        let b_cell_f = if mx0 != 0 { false } else { ja(0.0, xa) || jae(xa, ws) || ja(0.0, xe) || jae(xe, ws) };
        let b_cell_e = if my != 0 { false } else { ja(0.0, ya) || jae(ya, hf) || ja(0.0, ye) || jae(ye, hf) };
        let mut tap_c = cvtt(ya);
        let e = cvtt(ye).wrapping_sub(tap_c);
        let m = (e ^ (e >> 31)).wrapping_sub(e >> 31).wrapping_add(1);
        let mut dir = if e < 0 { -1i32 } else { 1 };
        let mf = m as f32;
        let mut xstep = (xe - xa) / mf;
        let step_a = st.a_gap_first / (mf * tap_width);
        let count = m.min(1000).max(0);
        let (mut s0, mut sum_ch1, mut wsm) = (0.0f32, 0.0f32, 0.0f32);
        if count > 0 {
            let cf = count as f32;
            let step_b = st.a_gap_second / (cf * st.one_minus_a_last);
            let wlim = cf * tap_width;
            let twow = 2.0f32.mul_add(ws, -1.0); // vfmsub213ss 
            let mut alpha = alpha_init;
            let mut step = step_a;
            let mut at_limit = false;
            let mut x = xa;
            for k in 0..count {
                if !at_limit && (k as f32) >= wlim {
                    let q = k / count;
                    alpha = (tap_width - (q as f32)).mul_add(step_b, alpha_origin); // vfmadd213ss 
                    step = step_b;
                    at_limit = true;
                }
                let xi = cvtt(x);
                let xcl: i32 = if b_cell_f { if xi >= -1 { xi.min(w) } else { -1 } } else { xi };
                let rowc = if b_cell_e { if tap_c >= -1 { tap_c.min(h) } else { -1 } } else { tap_c };
                let idx = ix(rowc.wrapping_mul(stride), xcl.wrapping_mul(2));
                tap_c = tap_c.wrapping_add(dir);
                wsm = wsm + alpha;
                x = x + xstep;
                let va = at(pl, idx)?;
                let vb = at(pl, idx + 1)?;
                let p0 = alpha * va;
                let weighted_b = alpha * vb;
                alpha = alpha + step;
                s0 = p0 + s0;
                sum_ch1 = weighted_b + sum_ch1;
                if b_cell_d {
                    if !(x < ws) {
                        if hst.port_blk_a != 0 {
                            xstep = -xstep;
                            x = twow - x;
                        } else if hst.cassumption_b != 0 {
                            x = x - ws;
                        }
                    } else if ja(0.0, x) {
                        if hst.port_blk_a != 0 {
                            x = -x;
                            xstep = -xstep;
                        } else if hst.cassumption_b != 0 {
                            x = x + ws;
                        }
                    }
                    if tap_c >= h {
                        if hst.cassumption_a != 0 {
                            dir = -1;
                            tap_c = h - 1;
                        } else if hst.cassumption_c != 0 {
                            tap_c = 0;
                        }
                    } else if tap_c < 0 {
                        if hst.cassumption_a != 0 {
                            tap_c = 0;
                            dir = 1;
                        } else if hst.cassumption_c != 0 {
                            tap_c = h - 1;
                        }
                    }
                }
            }
        }
        sum0 = s0;
        sum1 = sum_ch1;
        wsum = wsm;
    }
    let scale = norm_num / wsum;
    Ok((sum0 * scale, scale * sum1))
}

fn p1_stub(c: &P1Cfg, inp: &Inputs, pl: &Plane, x: f32, y: f32, i: usize, out: &mut Vec<Vec<f32>>) -> Result<(), Unsupported> {
    let pp = blur_warp_point::Params { n: c.n, width: c.plane_width, height: c.plane_height, flag_x: inp.mode_x, flag_y: inp.mode_y };
    let g = blur_warp_point::Geometry { base: pl.origin, stride: inp.acc_stride };
    let n = c.n.max(0) as usize;
    if out.len() < n.max(2) || out.iter().any(|v| i >= v.len()) {
        return Err(Unsupported::Oob);
    }
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| blur_warp_point::sample(&pp, &g, &pl.buf, x, y)));
    std::panic::set_hook(prev);
    let tmp = r.map_err(|_| Unsupported::Oob)?;
    for (k, v) in tmp.iter().enumerate() {
        out[k][i] = *v;
    }
    out[0][i] = inp.a[4] * out[0][i];
    out[1][i] = inp.a[4] * out[1][i]; //  (B multiplies plane 1 as well)
    Ok(())
}

/// Whole body B (S0..S12): `out[0]`, `out[1]` receive the two result planes; `p1` supplies the P1 stub parameters.
pub fn run_planes_b(p: &Params, inp: &Inputs, hst: &Host, pl: &Plane, weighted_b: Option<&P1Cfg>, out: &mut Vec<Vec<f32>>) -> Result<(), Unsupported> {
    if p.count <= 0 {
        return Ok(());
    }
    let g = |v: &Vec<f32>, i: usize| v.get(i).copied().unwrap_or(0.0);
    let (mut l_p1x, mut l_p1y, mut l_p2x, mut l_p2y) = (g(&inp.p1x, 0), g(&inp.p1y, 0), g(&inp.p2x, 0), g(&inp.p2y, 0));
    let (scx, scy) = s0_scales(p);
    let one_minus_a_last = 1.0f32 - inp.a[3];
    let a_gap_first = inp.a[1] - inp.a[0];
    let a_gap_second = inp.a[2] - inp.a[1];
    let mut recs = Vec::new();
    for i in 0..(p.count as usize) {
        if !inp.first_end_const {
            l_p1x = g(&inp.p1x, i);
            l_p1y = g(&inp.p1y, i);
        }
        if !inp.second_end_const {
            l_p2x = g(&inp.p2x, i);
            l_p2y = g(&inp.p2y, i);
        }
        let x = s2_x(&mut recs, p, inp, scx, l_p1x)?;
        let y = s3_y(&mut recs, p, inp, scy, l_p1y)?;
        let e2x = (l_p2x - l_p1x).mul_add(scx, x);
        let e2y = (-(l_p2y - l_p1y)).mul_add(scy, y);
        if x == e2x && y == e2y {
            let c = weighted_b.ok_or(Unsupported::P1Needed)?;
            p1_stub(c, inp, pl, x, y, i, out)?;
            continue;
        }
        let delta_x = e2x - x;
        let dy = e2y - y;
        let (adx, ady) = (delta_x.abs(), dy.abs());
        let st = S5State {
            sample: i, x, y, p1x: l_p1x, p1y: l_p1y, p2x: l_p2x, p2y: l_p2y, e2x, e2y, delta_x, dy, adx, ady, y_major: !(adx >= ady),
            data_buf_handle: inp.acc_buf_handle, width: p.img_w, height: p.img_h, stride: inp.acc_stride, scx, scy, a_gap_first, a_gap_second, one_minus_a_last,
            scale: inp.a[4], flag_cell_a: inp.flag_cell_a, mode_x: inp.mode_x, mode_y: inp.mode_y,
        };
        let (sample_a, sample_b) = if adx >= ady { x_major_sample2(&st, &inp.a, hst, pl)? } else { y_major_sample2(&st, &inp.a, hst, pl)? };
        if out.len() < 2 || i >= out[0].len() || i >= out[1].len() {
            return Err(Unsupported::Oob);
        }
        out[0][i] = sample_a;
        out[1][i] = sample_b;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn st(flag: u32) -> (S5State, [f32; 5], Host, Plane) {
        let st = S5State {
            sample: 0, x: 1.0, y: 1.0, p1x: 0.0, p1y: 0.0, p2x: 0.0, p2y: 0.0, e2x: 3.0, e2y: 1.0, delta_x: 2.0, dy: 0.0, adx: 2.0, ady: 0.0, y_major: false,
            data_buf_handle: 0, width: 8, height: 8, stride: 32, scx: 1.0, scy: 1.0, a_gap_first: 1.0, a_gap_second: 1.0, one_minus_a_last: 0.5, scale: 2.0, flag_cell_a: flag, mode_x: 0, mode_y: 0,
        };
        // pixel (r,c) channel k = 100*r + 10*c + k, row stride 32 floats (2 per pixel)
        let mut buf = vec![0.0f32; 600];
        for r in 0..8 {
            for c in 0..8 {
                for k in 0..2 {
                    buf[100 + r * 32 + 2 * c + k] = (100 * r + 10 * c + k) as f32;
                }
            }
        }
        (st, [0.0, 1.0, 2.0, 0.5, 2.0], Host { port_blk_a: 0, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 8.0, ws: 8.0 }, Plane { buf: std::sync::Arc::new(buf), origin: 100 })
    }

    #[test]
    fn x_major_noflag_hand() {
        // xa=1.5 -> col 1, xe=3.5 -> 3, m=3; ya=1.5 -> row 1 (cvtt), ystep 0; alphas 0, 2/3, 4/3; wsum 2
        // ch0 values (row 1, cols 1,2,3) = 110,120,130; sum = 0 + 80 + 173.33 ; scale = 2/2 = 1
        let (s, a, h, p) = st(0);
        let (sample_a, sample_b) = x_major_sample2(&s, &a, &h, &p).unwrap();
        assert!((sample_a - (120.0 * 2.0 / 3.0 + 130.0 * 4.0 / 3.0)).abs() < 1e-3, "{}", sample_a);
        assert!((sample_b - (sample_a + 2.0 / 3.0 + 4.0 / 3.0)).abs() < 1e-3, "{} {}", sample_b, sample_a);
    }

    #[test]
    fn lerp_negation_equals_plain_lerp() {
        let (sample_a, sample_b, f, w) = (1.25f32, 7.5, 0.3, 0.7);
        let plain = (sample_b - sample_a).mul_add(f, sample_a) * w + 2.0;
        assert_eq!(acc_lerp(2.0, sample_a, sample_b, f, w).to_bits(), plain.to_bits());
    }

    #[test]
    fn determinism_edges_nan() {
        for flag in [0u32, 1] {
            let (mut s, a, h, p) = st(flag);
            s.mode_x = 1;
            s.mode_y = 2;
            let r1 = x_major_sample2(&s, &a, &h, &p);
            let r2 = x_major_sample2(&s, &a, &h, &p);
            assert_eq!(r1.is_ok(), r2.is_ok());
            if let (Ok(alpha_origin), Ok(a2)) = (r1, r2) {
                assert_eq!((alpha_origin.0.to_bits(), alpha_origin.1.to_bits()), (a2.0.to_bits(), a2.1.to_bits()));
            }
            s.y = f32::NAN;
            let _ = x_major_sample2(&s, &a, &h, &p);
        }
    }

    #[test]
    fn y_major_hand_noflag() {
        // x=1.0,y=1.0 -> e2=(1,3); dy=2: ya=1.5,ye=3.5; rows 1,2,3 at xa=1.5 -> col 1 : ch0 = 110, 210, 310 ; alphas 0,2/3,4/3
        let (mut s, a, h, p) = st(0);
        s.e2x = 1.0;
        s.e2y = 3.0;
        s.delta_x = 0.0;
        s.dy = 2.0;
        s.adx = 0.0;
        s.ady = 2.0;
        let (sample_a, _v1) = y_major_sample2(&s, &a, &h, &p).unwrap();
        assert!((sample_a - (210.0 * 2.0 / 3.0 + 310.0 * 4.0 / 3.0)).abs() < 1e-3, "{}", sample_a);
    }

    #[test]
    fn zero_length_needs_p1() {
        let p = Params { count: 1, aspect_block_d: 2.0, assumption_e: 7.0, assumption_c: 0, assumption_d: 0, assumption_f: 0.0, assumption_g: 10.0, assumption_h: 0.0, img_w: 8, img_h: 6 };
        let inp = Inputs {
            a: [0.0, 1.0, 3.0, 0.5, 2.0], p1x: vec![0.25], p1y: vec![3.0], p2x: vec![0.25], p2y: vec![3.0],
            first_end_const: true, second_end_const: true, flag_cell_a: 0, mode_x: 0, mode_y: 0, acc_buf_handle: 0, acc_stride: 32,
        };
        let h = Host { port_blk_a: 0, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 6.0, ws: 8.0 };
        let pl = Plane { buf: std::sync::Arc::new(vec![0.0; 100]), origin: 50 };
        let mut out = vec![vec![0.0; 1], vec![0.0; 1]];
        assert_eq!(run_planes_b(&p, &inp, &h, &pl, None, &mut out), Err(Unsupported::P1Needed));
    }
}
