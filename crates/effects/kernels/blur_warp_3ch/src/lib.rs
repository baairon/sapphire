//! blur_warp_bidirectional body C (mode 3) as a delta against body A (crate blur_warp_1ch).
//! S0..S5 (prologue, wrap, S4 end points, S5 hand-off) are shared with A through blur_warp_1ch::{s0_scales, s2_x, s3_y}.
//! The sampling sections S6..S10 are three-channel versions (RGB planes out[0], out[1], out[2]) with the source
//! interleaved (3 floats per pixel). Source of truth: the body C listing. VEX follows the SDM, not silicon.

use blur_warp_1ch::p2::{Host, P1Cfg, Plane};
pub use blur_warp_1ch::*;

pub struct CAssumption {
    pub id: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<CAssumption> {
    vec![
        CAssumption { id: "BC1", basis: "prologue S0..S5 of C is the same arithmetic as A with other registers; reused from blur_warp_1ch (s0_scales, s2_x, s3_y); proved by the call-record T2 and the plane T2", label: "MEASURED listing + T2" },
        CAssumption { id: "BC2", basis: "source is 3 interleaved floats per pixel: idx = 3*col + row*stride (i32 wrapping); the lerp pair is rows (X-major flag_cell_a!=0: idx and idx+stride) or columns (Y-major flag_cell_a!=0: idx-3 and idx); stride is the result in floats, explicit input as in A (B2)", label: "MEASURED body C listing" },
        CAssumption { id: "BC3", basis: "flag_cell_a!=0 lerp term per channel = fma(v0-v1, frac, -v0) * wgt, subtracted from the channel sum (s = s - term); flag_cell_a==0: s = alpha*v + s", label: "MEASURED listing" },
        CAssumption { id: "BC4", basis: "vcomiss ... jb on the row/column bound sends NaN to the else branch; the port uses (a >= b) as the first branch (false for NaN). A's port uses !(a<b); NaN is outside the T2 grid", label: "INFERRED from listing; not compared" },
        CAssumption { id: "BC5", basis: "zero-length stub: after P1 returns, planes 0, 1 and 2 at the sample index are each multiplied by a4 (A: plane 0 only); P1 n (image) is an explicit input (A's B13), T2 uses n = 3", label: "MEASURED " },
        CAssumption { id: "BC6", basis: "host statics ..2b, (ws), (hs) explicit inputs, same as A (B9, B12)", label: "ASSUMPTION" },
        CAssumption { id: "BC7", basis: "FMA sites of C (listing grep of every vf*fma/fms/fnm): 20 sites: (S2,S3,S4); X-major; Y-major; modelled with mul_add; no rcp/rsqrt/x87", label: "MEASURED listing" },
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

/// idx = 3*c wrapping (lea), per channel taps
fn lerp_term(sample_a: f32, sample_b: f32, frac: f32, wgt: f32) -> f32 {
    (sample_a - sample_b).mul_add(frac, -sample_a) * wgt
}

/// X-major sample (S6, S7/S8 of C); returns out[0..3][sample].
pub fn x_major_sample(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<[f32; 3], Unsupported> {
    let (w, h, stride) = (st.width, st.height, st.stride);
    let wf = w as f32;
    let hs = hst.hs;
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cond_any = mx0 != 0 || my > 0;
    let (alpha_init, alpha_origin, tap_width, norm_num) = (a[0], a[1], a[3], a[4]);
    let mut sums = [0.0f32; 3];
    let mut wsum = 0.0f32;
    if st.flag_cell_a != 0 {
        let (x, y, delta_x, dy) = (st.x, st.y, st.delta_x, st.dy);
        let (mut dir, mut c, tap_b, tap_span, fx, edge_gap, dxs);
        if delta_x > 0.0 {
            let fl = x.floor();
            dir = 1i32;
            fx = x - fl;
            c = cvtt(fl);
            let ce = st.e2x.ceil();
            tap_b = cvtt(ce);
            edge_gap = ce - st.e2x;
            tap_span = tap_b.wrapping_sub(c).wrapping_add(1);
            dxs = delta_x;
        } else {
            let ce = x.ceil();
            dir = -1;
            c = cvtt(ce);
            let fl = st.e2x.floor();
            edge_gap = st.e2x - fl;
            fx = ce - x;
            tap_b = cvtt(fl);
            tap_span = c.wrapping_sub(tap_b).wrapping_add(1);
            dxs = -delta_x;
        }
        let slope = dy / dxs;
        let mut yy = (-slope).mul_add(fx, y); // vfnmadd231ss 
        let step_a = st.a_gap_first / ((tap_span as f32) * tap_width);
        let floor_y = yy.floor();
        let y_end = slope + st.e2y;
        let (b_cell_c, direct0) = if cond_any {
            let o = c < 0 || c >= w || ja(0.0, floor_y) || jae(floor_y, hs) || tap_b < 0 || tap_b >= w || ja(0.0, y_end) || jae(y_end, hs);
            (o, !o)
        } else {
            (false, false)
        };
        let b_cell_g = if direct0 || mx0 != 0 { false } else { c < 0 || c >= w || tap_b < 0 || tap_b >= w };
        let b_cell_h = if my != 0 { false } else { ja(0.0, floor_y) || jae(floor_y, hs) || ja(0.0, y_end) || jae(y_end, hs) };
        let n = tap_span.min(1000).max(0);
        let mut slope = slope;
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
                    if (c as f32) >= wf {
                        if hst.port_blk_a != 0 {
                            c = colmax;
                            dir = -1;
                        } else if hst.cassumption_b != 0 {
                            c = 0;
                        }
                    } else if c < 0 {
                        if hst.port_blk_a != 0 {
                            c = 0;
                            dir = 1;
                        } else if hst.cassumption_b != 0 {
                            c = colmax;
                        }
                    }
                    if yy >= hfi {
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
                let cc = if b_cell_g { if c >= -1 { c.min(w) } else { -1 } } else { c };
                let ycl = if b_cell_h { if ja(-1.0, yy) { -1.0 } else { vmin(hf, yy) } } else { yy };
                wsum = wgt + wsum;
                yy = slope + yy;
                alpha += step;
                c = c.wrapping_add(dir);
                let fr = 1.0 + ycl;
                let t = cvtt(fr);
                let frac = fr - (t as f32);
                let idx = cc.wrapping_mul(3).wrapping_sub(stride).wrapping_add(t.wrapping_mul(stride)) as i64;
                for chan in 0..3 {
                    let sample_a = at(pl, idx + chan as i64)?;
                    let sample_b = at(pl, idx + stride as i64 + chan as i64)?;
                    sums[chan] = sums[chan] - lerp_term(sample_a, sample_b, frac, wgt);
                }
            }
        }
    } else {
        let xa = 0.5 + st.x;
        let xe = 0.5 + st.e2x;
        let mut ya = 0.5 + st.y;
        let ye = 0.5 + st.e2y;
        let b_cell_n = if cond_any { ja(0.0, xe) || jae(xe, wf) || ja(0.0, ye) || jae(ye, hs) } else { false };
        let b_cell_m = if mx0 != 0 { false } else { ja(0.0, xa) || jae(xa, wf) || ja(0.0, xe) || jae(xe, wf) };
        let b_cell_l = if my != 0 { false } else { ja(0.0, ya) || jae(ya, hs) || ja(0.0, ye) || jae(ye, hs) };
        let mut cs = cvtt(xa);
        let e = cvtt(xe).wrapping_sub(cs);
        let m = (e ^ (e >> 31)).wrapping_sub(e >> 31).wrapping_add(1);
        let mut dir = if e < 0 { -1i32 } else { 1 };
        let mf = m as f32;
        let mut ystep = (ye - ya) / mf;
        let step_a = st.a_gap_first / (mf * tap_width);
        let count = m.min(1000).max(0);
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
                let col = if b_cell_m { if cs >= -1 { cs.min(w) } else { -1 } } else { cs };
                if b_cell_l {
                    rown = if rown >= -1 { rown.min(h) } else { -1 };
                }
                let idx = col.wrapping_mul(3).wrapping_add(rown.wrapping_mul(stride)) as i64;
                wsum = wsum + alpha;
                ya = ya + ystep;
                cs = cs.wrapping_add(dir);
                for chan in 0..3 {
                    let v = at(pl, idx + chan as i64)?;
                    sums[chan] = alpha * v + sums[chan];
                }
                alpha = alpha + step;
                if b_cell_n {
                    if (cs as f32) >= wf {
                        if hst.port_blk_a != 0 {
                            dir = -1;
                            cs = colmax;
                        } else if hst.cassumption_b != 0 {
                            cs = 0;
                        }
                    } else if cs < 0 {
                        if hst.port_blk_a != 0 {
                            cs = 0;
                            dir = 1;
                        } else if hst.cassumption_b != 0 {
                            cs = colmax;
                        }
                    }
                    if ya >= hsi {
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
    }
    let r = norm_num / wsum;
    Ok([sums[0] * r, sums[1] * r, sums[2] * r])
}

/// Y-major sample (flag_cell_a != 0, flag_cell_a == 0); returns out[0..3][sample].
pub fn y_major_sample(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<[f32; 3], Unsupported> {
    let (w, h, stride) = (st.width, st.height, st.stride);
    let (wf, hf) = (w as f32, h as f32);
    let ws = hst.ws;
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cond_any = mx0 != 0 || my > 0;
    let (alpha_init, alpha_origin, tap_width, norm_num) = (a[0], a[1], a[3], a[4]);
    let mut sums = [0.0f32; 3];
    let mut wsum = 0.0f32;
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
        let xclamp = if direct0 || mx0 != 0 { false } else { ja(0.0, floor_x) || jae(floor_x, ws) || ja(0.0, sum_9) || jae(sum_9, ws) };
        let rowclamp = if my != 0 { false } else { tap_a < 0 || tap_a >= h || tap_b < 0 || tap_b >= h };
        let n = tap_span.min(1000).max(0);
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
                    if xx >= ws {
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
                let xc = if xclamp { if ja(-1.0, xx) { -1.0 } else { vmin(wf, xx) } } else { xx };
                let rr = if rowclamp { if tap_a >= -1 { tap_a.min(h) } else { -1 } } else { tap_a };
                wsum = wgt + wsum;
                xx = slope + xx;
                alpha += step;
                let fr = 1.0 + xc;
                let t = cvtt(fr);
                let frac = fr - (t as f32);
                let idx = rr.wrapping_mul(stride).wrapping_add(t.wrapping_mul(3)) as i64;
                tap_a = tap_a.wrapping_add(dir);
                for chan in 0..3 {
                    let sample_a = at(pl, idx - 3 + chan as i64)?;
                    let sample_b = at(pl, idx + chan as i64)?;
                    sums[chan] = sums[chan] - lerp_term(sample_a, sample_b, frac, wgt);
                }
            }
        }
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
                let xcl = if b_cell_f { if xi >= -1 { xi.min(w) } else { -1 } } else { xi };
                let rowc = if b_cell_e { if tap_c >= -1 { tap_c.min(h) } else { -1 } } else { tap_c };
                let idx = xcl.wrapping_mul(3).wrapping_add(rowc.wrapping_mul(stride)) as i64;
                wsum = wsum + alpha;
                x = x + xstep;
                tap_c = tap_c.wrapping_add(dir);
                for chan in 0..3 {
                    let v = at(pl, idx + chan as i64)?;
                    sums[chan] = alpha * v + sums[chan];
                }
                alpha = alpha + step;
                if b_cell_d {
                    if x >= ws {
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
    }
    let r = norm_num / wsum;
    Ok([sums[0] * r, sums[1] * r, sums[2] * r])
}

/// Zero-length stub : P1 renders (x, y); then planes 0, 1, 2 at the sample index are multiplied by a4.
fn p1_stub(c: &P1Cfg, inp: &Inputs, pl: &Plane, x: f32, y: f32, i: usize, out: &mut Vec<Vec<f32>>) -> Result<(), Unsupported> {
    let pp = blur_warp_point::Params { n: c.n, width: c.plane_width, height: c.plane_height, flag_x: inp.mode_x, flag_y: inp.mode_y };
    let g = blur_warp_point::Geometry { base: pl.origin, stride: inp.acc_stride };
    let n = c.n.max(0) as usize;
    if out.len() < n.max(3) || out.iter().any(|v| i >= v.len()) {
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
    for k in 0..3 {
        out[k][i] = inp.a[4] * out[k][i];
    }
    Ok(())
}

/// Whole body C for X-major, Y-major and zero-length samples; out[0..3] are the three output planes.
pub fn run_planes_full(p: &Params, inp: &Inputs, hst: &Host, pl: &Plane, pass1_cfg: Option<&P1Cfg>, out: &mut Vec<Vec<f32>>) -> Result<(), Unsupported> {
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
            let c = pass1_cfg.ok_or(Unsupported::P1Needed)?;
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
        let v = if adx >= ady { x_major_sample(&st, &inp.a, hst, pl)? } else { y_major_sample(&st, &inp.a, hst, pl)? };
        for k in 0..3 {
            if let Some(o) = out.get_mut(k) {
                if i < o.len() {
                    o[i] = v[k];
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn st(flag: u32) -> (S5State, [f32; 5], Host, Plane) {
        let st = S5State {
            sample: 0, x: 1.0, y: 1.0, p1x: 0.0, p1y: 0.0, p2x: 0.0, p2y: 0.0, e2x: 3.0, e2y: 1.0, delta_x: 2.0, dy: 0.0, adx: 2.0, ady: 0.0, y_major: false,
            data_buf_handle: 0, width: 8, height: 8, stride: 30, scx: 1.0, scy: 1.0, a_gap_first: 1.0, a_gap_second: 1.0, one_minus_a_last: 0.5, scale: 2.0, flag_cell_a: flag, mode_x: 0, mode_y: 0,
        };
        // channel c of pixel (col, row) = c*100 + row*10 + col (row stride 30 floats, 3 floats per pixel)
        let mut buf = vec![0.0f32; 1000];
        for row in 0..12i64 {
            for col in 0..10i64 {
                for chan in 0..3i64 {
                    let k = 200 + row * 30 + col * 3 + chan;
                    buf[k as usize] = (chan * 100 + row * 10 + col) as f32;
                }
            }
        }
        (st, [0.0, 1.0, 2.0, 0.5, 2.0], Host { port_blk_a: 0, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 8.0, ws: 8.0 }, Plane { buf: std::sync::Arc::new(buf), origin: 200 })
    }

    #[test]
    fn x_flag0_hand() {
        // xa=1.5->cs 1, xe=3.5->3, m=3; ya=1.5 row 1 const; alpha 0, 2/3, 4/3; values (ch*100+10+col) at cols 1,2,3 -> 11,12,13
        let (s, a, h, p) = st(0);
        let v = x_major_sample(&s, &a, &h, &p).unwrap();
        let sum0 = 0.0 * 11.0 + (2.0f32 / 3.0) * 12.0 + (4.0f32 / 3.0) * 13.0;
        assert!((v[0] - sum0 * 2.0 / 2.0).abs() < 1e-3, "{:?}", v);
        assert!((v[1] - (sum0 + 100.0 * 2.0)).abs() < 1e-3, "{:?}", v);
        assert!((v[2] - (sum0 + 200.0 * 2.0)).abs() < 1e-3, "{:?}", v);
    }

    #[test]
    fn deterministic_finite_both_flags() {
        for flag in [0u32, 1] {
            let (s, a, h, p) = st(flag);
            let sample_b = x_major_sample(&s, &a, &h, &p).unwrap();
            let v2 = x_major_sample(&s, &a, &h, &p).unwrap();
            assert_eq!(sample_b.map(f32::to_bits), v2.map(f32::to_bits));
            assert!(sample_b.iter().all(|f| f.is_finite()));
        }
    }

    #[test]
    fn y_flag0_hand() {
        let (mut s, a, h, p) = st(0);
        s.e2x = 1.0;
        s.e2y = 3.0;
        s.delta_x = 0.0;
        s.dy = 2.0;
        s.adx = 0.0;
        s.ady = 2.0;
        // x const col 1 (xa 1.5), rows 1,2,3 -> values ch*100 + 10r + 1, weights 0, 2/3, 4/3
        let v = y_major_sample(&s, &a, &h, &p).unwrap();
        let sum0 = (2.0f32 / 3.0) * 21.0 + (4.0f32 / 3.0) * 31.0;
        assert!((v[0] - sum0).abs() < 1e-3, "{:?}", v);
        assert!((v[1] - (sum0 + 200.0)).abs() < 1e-3, "{:?}", v);
    }

    #[test]
    fn edge_modes_nan_and_flags() {
        for flag in [0u32, 1] {
            for (port_blk_a, cassumption_a, cassumption_b, cassumption_c) in [(0, 0, 0, 0), (1, 1, 1, 1), (0, 1, 0, 1), (1, 0, 1, 0)] {
                for (mx, my) in [(0, 0), (1, 2), (2, 1), (0, 2), (1, 0)] {
                    let (mut s, a, mut h, p) = st(flag);
                    h.port_blk_a = port_blk_a;
                    h.cassumption_a = cassumption_a;
                    h.cassumption_b = cassumption_b;
                    h.cassumption_c = cassumption_c;
                    s.mode_x = mx;
                    s.mode_y = my;
                    s.e2x = 9.5;
                    s.delta_x = 8.5;
                    assert!(x_major_sample(&s, &a, &h, &p).is_ok());
                }
            }
        }
        let (mut s, a, h, p) = st(1);
        s.y = f32::NAN;
        let _ = x_major_sample(&s, &a, &h, &p);
        s.x = f32::INFINITY;
        let _ = x_major_sample(&s, &a, &h, &p);
        let (mut s, a, h, p) = st(0);
        s.e2y = 9.5;
        s.dy = 8.5;
        s.delta_x = 0.0;
        s.e2x = s.x;
        s.adx = 0.0;
        s.ady = 8.5;
        let r1 = y_major_sample(&s, &a, &h, &p);
        let r2 = y_major_sample(&s, &a, &h, &p);
        assert_eq!(r1.map(|v| v.map(f32::to_bits)), r2.map(|v| v.map(f32::to_bits)));
    }

    #[test]
    fn zero_length_without_p1_is_reported() {
        let p = Params { count: 1, aspect_block_d: 2.0, assumption_e: 7.0, assumption_c: 0, assumption_d: 0, assumption_f: 0.0, assumption_g: 10.0, assumption_h: 0.0, img_w: 8, img_h: 6 };
        let inp = Inputs {
            a: [0.0, 1.0, 3.0, 0.5, 2.0], p1x: vec![0.25], p1y: vec![3.0], p2x: vec![0.25], p2y: vec![3.0],
            first_end_const: true, second_end_const: true, flag_cell_a: 0, mode_x: 0, mode_y: 0, acc_buf_handle: 0, acc_stride: 30,
        };
        let h = Host { port_blk_a: 0, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 6.0, ws: 8.0 };
        let (_, _, _, pl) = st(0);
        let mut out = vec![vec![0.0; 1]; 3];
        assert_eq!(run_planes_full(&p, &inp, &h, &pl, None, &mut out), Err(Unsupported::P1Needed));
    }
}
