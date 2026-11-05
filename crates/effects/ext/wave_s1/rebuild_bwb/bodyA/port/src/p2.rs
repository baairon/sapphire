//! BW5B: S6, S7, S8 and the epilogue of body A for X-major samples .
//! Source of truth: A.lst. SPEC_P2.md has the instruction-order derivation. Names INFERRED, arithmetic MEASURED.

use crate::*;

/// Source plane as the original sees it: `buf[origin + idx]` is data[idx] (the data pointer of).
#[derive(Clone, Debug)]
pub struct Plane {
    pub buf: Vec<f32>,
    pub origin: i64,
}

impl Plane {
    fn at(&self, idx: i64) -> Result<f32, Unsupported> {
        let k = self.origin + idx;
        if k < 0 || k as usize >= self.buf.len() {
            return Err(Unsupported::Oob);
        }
        Ok(self.buf[k as usize])
    }
}

/// Host statics (W44): edge flag bytes ..2b and the float .
#[derive(Clone, Copy, Debug)]
pub struct Host {
    pub port_blk_a: u8,
    pub cassumption_a: u8,
    pub cassumption_b: u8,
    pub cassumption_c: u8,
    pub hs: f32,
    /// float (host width, read raw by S9 and S10; BW5C, assumption B12)
    pub ws: f32,
}

/// Parameters of the P1 stub as the body reaches it: P1's params object is the image object (rdx = r13),
/// so n = [img], width = [[img]], height = [[img]] are explicit inputs (BW5C, assumption B13).
#[derive(Clone, Copy, Debug)]
pub struct P1Cfg {
    pub n: i32,
    pub w0: i32,
    pub h0: i32,
}

/// S9 (flag_cell_a != 0) and S10 (flag_cell_a == 0) for one Y-major sample; returns out[sample].
pub fn y_major_sample(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<f32, Unsupported> {
    let (w, h, stride) = (st.width, st.height, st.stride);
    let (wf, hf) = (w as f32, h as f32);
    let ws = hst.ws;
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cond_any = mx0 != 0 || my > 0;
    let (a0, a1, a3, a4) = (a[0], a[1], a[3], a[4]);
    let (sum, wsum);
    if st.flag_cell_a != 0 {
        let (x, y, dx, dy) = (st.x, st.y, st.dx, st.dy);
        let (mut r9, esi, n0, fy, c0, dys, mut dir);
        if dy > 0.0 {
            let fl = y.floor();
            fy = y - fl;
            r9 = cvtt(fl);
            let ce = st.e2y.ceil();
            c0 = ce - st.e2y;
            esi = cvtt(ce);
            n0 = esi.wrapping_sub(r9).wrapping_add(1);
            dir = 1i32;
            dys = dy;
        } else {
            let ce = y.ceil();
            r9 = cvtt(ce);
            fy = ce - y;
            let fl = st.e2y.floor();
            c0 = st.e2y - fl;
            esi = cvtt(fl);
            n0 = r9.wrapping_sub(esi).wrapping_add(1);
            dir = -1;
            dys = -dy;
        }
        let mut slope = dx / dys;
        let mut xx = (-slope).mul_add(fy, x); // vfnmadd231ss 
        let step_a = st.k_a1_a0 / ((n0 as f32) * a3);
        let x5 = xx.floor();
        let x9 = slope + st.e2x;
        let (b_cell_c, direct0) = if cond_any {
            let o = ja(0.0, x5) || jae(x5, ws) || r9 < 0 || r9 >= h || ja(0.0, x9) || jae(x9, ws) || esi < 0 || esi >= h;
            (o, !o)
        } else {
            (false, false)
        };
        let r10 = if direct0 || mx0 != 0 { false } else { ja(0.0, x5) || jae(x5, ws) || ja(0.0, x9) || jae(x9, ws) };
        let r11 = if my != 0 { false } else { r9 < 0 || r9 >= h || esi < 0 || esi >= h };
        let n = n0.min(1000).max(0);
        let mut s = 0.0f32;
        let mut wsm = 0.0f32;
        if n > 0 {
            let omfy = 1.0 - fy;
            let omc0 = 1.0 - c0;
            let nf = n as f32;
            let wlim = nf * a3;
            let step_b = st.k_a2_a1 / (nf * st.k_1_a3);
            let twow = 2.0f32.mul_add(ws, -1.0); // vfmsub213ss 
            let mut alpha = a0;
            let mut step = step_a;
            let mut al = false;
            for k in 0..n {
                if !al && (k as f32) >= wlim {
                    let q = k / n;
                    alpha = (a3 - (q as f32)).mul_add(step_b, a1); // vfmadd213ss 
                    step = step_b;
                    al = true;
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
                    if r9 >= h {
                        if hst.cassumption_a != 0 {
                            r9 = h - 1;
                            dir = -1;
                        } else if hst.cassumption_c != 0 {
                            r9 = 0;
                        }
                    } else if r9 < 0 {
                        if hst.cassumption_a != 0 {
                            r9 = 0;
                            dir = 1;
                        } else if hst.cassumption_c != 0 {
                            r9 = h - 1;
                        }
                    }
                }
                let wgt = if k == 0 { alpha * omfy } else if k == n - 1 { alpha * omc0 } else { alpha };
                let xc = if r10 { if ja(-1.0, xx) { -1.0 } else { vmin(wf, xx) } } else { xx };
                let rr: i64 = if r11 { if r9 >= -1 { r9.min(h) as i64 } else { -1 } } else { r9 as i64 };
                let rs = rr.wrapping_mul(stride as i64);
                wsm = wgt + wsm;
                xx = slope + xx;
                alpha += step;
                let fr = 1.0 + xc;
                let t = cvtt(fr);
                let frac = fr - (t as f32);
                r9 = r9.wrapping_add(dir);
                let idx = rs.wrapping_add(t as i64);
                let v1 = pl.at(idx)?;
                let v0 = pl.at(idx - 1)?;
                let lerp = (v1 - v0).mul_add(frac, v0); // vfmadd213ss 
                s = lerp * wgt + s;
            }
        }
        sum = s;
        wsum = wsm;
    } else {
        let xe = 0.5 + st.e2x;
        let xa = 0.5 + st.x;
        let ya = 0.5 + st.y;
        let ye = 0.5 + st.e2y;
        let b_cell_d = if cond_any { ja(0.0, xe) || jae(xe, ws) || ja(0.0, ye) || jae(ye, hf) } else { false };
        let b_cell_f = if mx0 != 0 { false } else { ja(0.0, xa) || jae(xa, ws) || ja(0.0, xe) || jae(xe, ws) };
        let b_cell_e = if my != 0 { false } else { ja(0.0, ya) || jae(ya, hf) || ja(0.0, ye) || jae(ye, hf) };
        let mut r11 = cvtt(ya);
        let e = cvtt(ye).wrapping_sub(r11);
        let m = (e ^ (e >> 31)).wrapping_sub(e >> 31).wrapping_add(1);
        let mut dir = if e < 0 { -1i32 } else { 1 };
        let mf = m as f32;
        let mut xstep = (xe - xa) / mf;
        let step_a = st.k_a1_a0 / (mf * a3);
        let count = m.min(1000).max(0);
        let mut s = 0.0f32;
        let mut wsm = 0.0f32;
        if count > 0 {
            let cf = count as f32;
            let step_b = st.k_a2_a1 / (cf * st.k_1_a3);
            let wlim = cf * a3;
            let twow = 2.0f32.mul_add(ws, -1.0); // vfmsub213ss 
            let mut alpha = a0;
            let mut step = step_a;
            let mut al = false;
            let mut x = xa;
            for k in 0..count {
                if !al && (k as f32) >= wlim {
                    let q = k / count;
                    alpha = (a3 - (q as f32)).mul_add(step_b, a1); // vfmadd213ss 
                    step = step_b;
                    al = true;
                }
                let xi = cvtt(x);
                let xcl: i64 = if b_cell_f { if xi >= -1 { xi.min(w) as i64 } else { -1 } } else { xi as i64 };
                let rowc = if b_cell_e { if r11 >= -1 { r11.min(h) } else { -1 } } else { r11 };
                let idx = xcl + (rowc.wrapping_mul(stride)) as i64;
                r11 = r11.wrapping_add(dir);
                wsm = wsm + alpha;
                x = x + xstep;
                let v = pl.at(idx)?;
                let prod = alpha * v;
                alpha = alpha + step;
                s = prod + s;
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
                    if r11 >= h {
                        if hst.cassumption_a != 0 {
                            dir = -1;
                            r11 = h - 1;
                        } else if hst.cassumption_c != 0 {
                            r11 = 0;
                        }
                    } else if r11 < 0 {
                        if hst.cassumption_a != 0 {
                            r11 = 0;
                            dir = 1;
                        } else if hst.cassumption_c != 0 {
                            r11 = h - 1;
                        }
                    }
                }
            }
        }
        sum = s;
        wsum = wsm;
    }
    Ok(sum * (a4 / wsum))
}

// vcomiss + branch semantics: ja is a>b, jae is a>=b (both false on NaN)
fn ja(a: f32, b: f32) -> bool {
    a > b
}
fn jae(a: f32, b: f32) -> bool {
    a >= b
}
// vminss dst,a,b: a if a<b else b
fn vmin(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

/// S6 + S7 (flag_cell_a != 0) or S8 (flag_cell_a == 0) + epilogue arithmetic for one X-major sample; returns out[sample].
pub fn x_major_sample(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<f32, Unsupported> {
    let (w, h, stride) = (st.width, st.height, st.stride);
    let wf = w as f32;
    let hs = hst.hs;
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cond_any = mx0 != 0 || my > 0;
    let (a0, a1, a3, a4) = (a[0], a[1], a[3], a[4]);
    let (sum, wsum);
    if st.flag_cell_a != 0 {
        let (x, y, dx, dy) = (st.x, st.y, st.dx, st.dy);
        let (mut r9, mut r11, esi, n0, fx, c0, dxs);
        if dx > 0.0 {
            let fl = x.floor();
            r9 = 1i32;
            fx = x - fl;
            r11 = cvtt(fl);
            let ce = st.e2x.ceil();
            esi = cvtt(ce);
            c0 = ce - st.e2x;
            n0 = esi.wrapping_sub(r11).wrapping_add(1);
            dxs = dx;
        } else {
            let ce = x.ceil();
            r9 = -1;
            r11 = cvtt(ce);
            let fl = st.e2x.floor();
            c0 = st.e2x - fl;
            fx = ce - x;
            esi = cvtt(fl);
            n0 = r11.wrapping_sub(esi).wrapping_add(1);
            dxs = -dx;
        }
        let mut slope = dy / dxs;
        let mut yy = (-slope).mul_add(fx, y); // vfnmadd231ss 
        let step_a = st.k_a1_a0 / ((n0 as f32) * a3);
        let y5 = yy.floor();
        let y9 = slope + st.e2y;
        let (b_cell_c, direct0) = if cond_any {
            let o = r11 < 0 || r11 >= w || ja(0.0, y5) || jae(y5, hs) || esi < 0 || esi >= w || ja(0.0, y9) || jae(y9, hs);
            (o, !o)
        } else {
            (false, false)
        };
        let b_cell_g = if direct0 || mx0 != 0 { false } else { r11 < 0 || r11 >= w || esi < 0 || esi >= w };
        let b_cell_h = if my != 0 { false } else { ja(0.0, y5) || jae(y5, hs) || ja(0.0, y9) || jae(y9, hs) };
        let n = n0.min(1000).max(0);
        let mut s = 0.0f32;
        let mut ws = 0.0f32;
        if n > 0 {
            let hfi = cvtt(hs) as f32;
            let omfx = 1.0 - fx;
            let omc0 = 1.0 - c0;
            let nf = n as f32;
            let wlim = nf * a3;
            let step_b = st.k_a2_a1 / (nf * st.k_1_a3);
            let twoh = 2.0f32.mul_add(hfi, -1.0); // vfmsub213ss 
            let colmax = cvtt(wf - 1.0);
            let hf = h as f32;
            let mut alpha = a0;
            let mut step = step_a;
            let mut al = false;
            for k in 0..n {
                if !al && (k as f32) >= wlim {
                    let q = k / n;
                    alpha = (a3 - (q as f32)).mul_add(step_b, a1); // vfmadd213ss 
                    step = step_b;
                    al = true;
                }
                if b_cell_c {
                    if (r11 as f32) >= wf {
                        if hst.port_blk_a != 0 {
                            r11 = colmax;
                            r9 = -1;
                        } else if hst.cassumption_b != 0 {
                            r11 = 0;
                        }
                    } else if r11 < 0 {
                        if hst.port_blk_a != 0 {
                            r11 = 0;
                            r9 = 1;
                        } else if hst.cassumption_b != 0 {
                            r11 = colmax;
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
                let c = if b_cell_g { if r11 >= -1 { r11.min(w) } else { -1 } } else { r11 };
                let ycl = if b_cell_h { if ja(-1.0, yy) { -1.0 } else { vmin(hf, yy) } } else { yy };
                ws = wgt + ws;
                yy = slope + yy;
                alpha += step;
                r11 = r11.wrapping_add(r9);
                let fr = 1.0 + ycl;
                let t = cvtt(fr);
                let frac = fr - (t as f32);
                let idx = (c.wrapping_sub(stride)).wrapping_add(t.wrapping_mul(stride)) as i64;
                let v0 = pl.at(idx)?;
                let v1 = pl.at(idx + stride as i64)?;
                let lerp = (v1 - v0).mul_add(frac, v0); // vfmadd213ss 
                s = lerp * wgt + s;
            }
        }
        sum = s;
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
        let mut r9 = if e < 0 { -1i32 } else { 1 };
        let mf = m as f32;
        let mut ystep = (ye - ya) / mf;
        let step_a = st.k_a1_a0 / (mf * a3);
        let count = m.min(1000).max(0);
        let mut s = 0.0f32;
        let mut ws = 0.0f32;
        if count > 0 {
            let cf = count as f32;
            let step_b = st.k_a2_a1 / (cf * st.k_1_a3);
            let wlim = cf * a3;
            let hsi = cvtt(hs) as f32;
            let twoh = 2.0f32.mul_add(hsi, -1.0); // vfmsub213ss 
            let colmax = cvtt(wf - 1.0);
            let mut alpha = a0;
            let mut step = step_a;
            let mut al = false;
            for k in 0..count {
                let mut rown = cvtt(ya);
                if !al && (k as f32) >= wlim {
                    let q = k / count;
                    alpha = (a3 - (q as f32)).mul_add(step_b, a1); // vfmadd213ss 
                    step = step_b;
                    al = true;
                }
                let col: i64 = if b_cell_k { if cs >= -1 { cs.min(w) as i64 } else { -1 } } else { cs as i64 };
                if b_cell_j {
                    rown = if rown >= -1 { rown.min(h) } else { -1 };
                }
                let idx = col + (rown.wrapping_mul(stride)) as i64;
                cs = cs.wrapping_add(r9);
                ws = ws + alpha;
                ya = ya + ystep;
                let v = pl.at(idx)?;
                let prod = alpha * v;
                alpha = alpha + step;
                s = prod + s;
                if b_cell_i {
                    if (cs as f32) >= wf {
                        if hst.port_blk_a != 0 {
                            r9 = -1;
                            cs = colmax;
                        } else if hst.cassumption_b != 0 {
                            cs = 0;
                        }
                    } else if cs < 0 {
                        if hst.port_blk_a != 0 {
                            cs = 0;
                            r9 = 1;
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
        sum = s;
        wsum = ws;
    }
    Ok(sum * (a4 / wsum))
}

/// Whole body for X-major inputs: fills `out[i]` (plane 0). Y-major samples give Unsupported::YMajor,
/// zero-length samples (the P1 stub) give Unsupported::P1Needed.
/// BW5B-compatible entry: no P1 configuration, so zero-length samples still give P1Needed.
pub fn run_planes(p: &Params, inp: &Inputs, hst: &Host, pl: &Plane, out: &mut Vec<f32>) -> Result<(), Unsupported> {
    let mut planes = vec![std::mem::take(out)];
    let r = run_planes_full(p, inp, hst, pl, None, &mut planes);
    *out = planes.swap_remove(0);
    r
}

/// Whole body A (S0..S12) for X-major, Y-major and zero-length samples. `out[k]` are the output planes (plane 0 receives the result);
/// `p1` supplies the P1 stub parameters (None: zero-length samples give P1Needed).
pub fn run_planes_full(p: &Params, inp: &Inputs, hst: &Host, pl: &Plane, p1: Option<&P1Cfg>, out: &mut Vec<Vec<f32>>) -> Result<(), Unsupported> {
    if p.count <= 0 {
        return Ok(());
    }
    let g = |v: &Vec<f32>, i: usize| v.get(i).copied().unwrap_or(0.0);
    let (mut l_p1x, mut l_p1y, mut l_p2x, mut l_p2y) = (g(&inp.p1x, 0), g(&inp.p1y, 0), g(&inp.p2x, 0), g(&inp.p2y, 0));
    let (scx, scy) = s0_scales(p);
    let k_1_a3 = 1.0f32 - inp.a[3];
    let k_a1_a0 = inp.a[1] - inp.a[0];
    let k_a2_a1 = inp.a[2] - inp.a[1];
    let mut recs = Vec::new();
    for i in 0..(p.count as usize) {
        if !inp.p1_const {
            l_p1x = g(&inp.p1x, i);
            l_p1y = g(&inp.p1y, i);
        }
        if !inp.p2_const {
            l_p2x = g(&inp.p2x, i);
            l_p2y = g(&inp.p2y, i);
        }
        let x = s2_x(&mut recs, p, inp, scx, l_p1x)?;
        let y = s3_y(&mut recs, p, inp, scy, l_p1y)?;
        let e2x = (l_p2x - l_p1x).mul_add(scx, x);
        let e2y = (-(l_p2y - l_p1y)).mul_add(scy, y);
        if x == e2x && y == e2y {
            let c = p1.ok_or(Unsupported::P1Needed)?;
            p1_stub(c, p, inp, pl, x, y, i, out)?;
            continue;
        }
        let dx = e2x - x;
        let dy = e2y - y;
        let (adx, ady) = (dx.abs(), dy.abs());
        let st = S5State {
            sample: i, x, y, p1x: l_p1x, p1y: l_p1y, p2x: l_p2x, p2y: l_p2y, e2x, e2y, dx, dy, adx, ady, y_major: !(adx >= ady),
            data_ptr: inp.acc_ptr, width: p.img_w, height: p.img_h, stride: inp.acc_stride, scx, scy, k_a1_a0, k_a2_a1, k_1_a3,
            scale: inp.a[4], flag_cell_a: inp.flag_cell_a, mode_x: inp.mode_x, mode_y: inp.mode_y,
        };
        let v = if adx >= ady { x_major_sample(&st, &inp.a, hst, pl)? } else { y_major_sample(&st, &inp.a, hst, pl)? };
        if let Some(o0) = out.get_mut(0) {
            if i < o0.len() {
                o0[i] = v;
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
            sample: 0, x: 1.0, y: 1.0, p1x: 0.0, p1y: 0.0, p2x: 0.0, p2y: 0.0, e2x: 3.0, e2y: 1.0, dx: 2.0, dy: 0.0, adx: 2.0, ady: 0.0, y_major: false,
            data_ptr: 0, width: 8, height: 8, stride: 8, scx: 1.0, scy: 1.0, k_a1_a0: 1.0, k_a2_a1: 1.0, k_1_a3: 0.5, scale: 2.0, flag_cell_a: flag, mode_x: 0, mode_y: 0,
        };
        let buf: Vec<f32> = (0..400).map(|i| (i % 8) as f32).collect();
        (st, [0.0, 1.0, 2.0, 0.5, 2.0], Host { port_blk_a: 0, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 8.0, ws: 8.0 }, Plane { buf, origin: 100 })
    }

    #[test]
    fn s8_single_row_hand() {
        // 0.5 shift: cs=1, xe=3.5 -> cvtt 3, m=3; alpha ramp 0, step 1/(3*.5)=0.667; columns 1,2,3 => values 1,2,3
        let (s, a, h, p) = st(0);
        let v = x_major_sample(&s, &a, &h, &p).unwrap();
        // hand: k0 alpha 0 (value 5), k1 alpha 2/3 (value 6, sum 4), k2 phase switch alpha 1+0.5*2/3=4/3 (value 7); wsum 2; out 40/3 * 2/2
        assert!((v - 40.0 / 3.0).abs() < 1e-4, "{}", v);
    }

    #[test]
    fn s7_deterministic_finite() {
        let (s, a, h, p) = st(1);
        let v1 = x_major_sample(&s, &a, &h, &p).unwrap();
        let v2 = x_major_sample(&s, &a, &h, &p).unwrap();
        assert_eq!(v1.to_bits(), v2.to_bits());
        assert!(v1.is_finite());
    }

    #[test]
    fn one_sample_segment_and_long_segment() {
        let (mut s, a, h, p) = st(1);
        s.e2x = 1.25;
        s.dx = 0.25;
        assert!(x_major_sample(&s, &a, &h, &p).unwrap().is_finite());
        let (mut s, a, h, _) = st(0);
        s.e2x = 3000.0;
        s.dx = 2999.0;
        let big = Plane { buf: vec![1.0; 5000], origin: 2000 };
        let r = x_major_sample(&s, &a, &h, &big);
        assert!(r.is_ok() || r == Err(Unsupported::Oob));
    }

    #[test]
    fn edge_modes_and_nan() {
        for flag in [0u32, 1] {
            for (port_blk_a, cassumption_a, cassumption_b, cassumption_c) in [(0, 0, 0, 0), (1, 1, 1, 1), (0, 1, 0, 1), (1, 0, 1, 0)] {
                let (mut s, a, mut h, p) = st(flag);
                h.port_blk_a = port_blk_a;
                h.cassumption_a = cassumption_a;
                h.cassumption_b = cassumption_b;
                h.cassumption_c = cassumption_c;
                s.mode_x = 1;
                s.mode_y = 2;
                s.e2x = 9.5;
                s.dx = 8.5;
                let r = x_major_sample(&s, &a, &h, &p);
                assert!(r.is_ok());
            }
        }
        let (mut s, a, h, p) = st(1);
        s.y = f32::NAN;
        let _ = x_major_sample(&s, &a, &h, &p);
        s.x = f32::INFINITY;
        let _ = x_major_sample(&s, &a, &h, &p);
    }

    #[test]
    fn y_major_supported_now() {
        let p = Params { count: 1, aspect_block_d: 2.0, assumption_e: 7.0, assumption_c: 0, assumption_d: 0, assumption_f: 0.0, assumption_g: 10.0, assumption_h: 0.0, img_w: 8, img_h: 6 };
        let inp = Inputs {
            a: [0.0, 1.0, 3.0, 0.5, 2.0], p1x: vec![0.25], p1y: vec![3.0], p2x: vec![0.25], p2y: vec![0.0],
            p1_const: true, p2_const: true, flag_cell_a: 0, mode_x: 0, mode_y: 0, acc_ptr: 0, acc_stride: 8,
        };
        let h = Host { port_blk_a: 0, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 6.0, ws: 8.0 };
        let pl = Plane { buf: vec![0.0; 100], origin: 50 };
        let mut out = vec![0.0; 1];
        assert_ne!(run_planes(&p, &inp, &h, &pl, &mut out), Err(Unsupported::YMajor));
    }

    fn ys(flag: u32) -> (S5State, [f32; 5], Host, Plane) {
        let (mut s, a, h, p) = st(flag);
        s.e2x = 1.0;
        s.e2y = 3.0;
        s.dx = 0.0;
        s.dy = 2.0;
        s.adx = 0.0;
        s.ady = 2.0;
        s.y_major = true;
        (s, a, h, p)
    }

    #[test]
    fn s10_hand() {
        // columns fixed at 1 for rows 1,2,3: value (101+8r)%8 = 5 each; weights 0, 2/3, 4/3 (wsum 2); out = 10 * (2/2)
        let (s, a, h, p) = ys(0);
        let v = y_major_sample(&s, &a, &h, &p).unwrap();
        assert!((v - 10.0).abs() < 1e-4, "{}", v);
    }

    #[test]
    fn s9_hand() {
        // rows 1,2,3 at x=1: t=2, frac 0, lerp = v0 = (101+8r)%8 = 5; same weights; out = 10
        let (s, a, h, p) = ys(1);
        let v = y_major_sample(&s, &a, &h, &p).unwrap();
        assert!((v - 10.0).abs() < 1e-4, "{}", v);
    }

    #[test]
    fn y_deterministic_edges_nan() {
        for flag in [0u32, 1] {
            for (port_blk_a, cassumption_a, cassumption_b, cassumption_c) in [(0, 0, 0, 0), (1, 1, 1, 1), (0, 1, 0, 1), (1, 0, 1, 0)] {
                for (mx, my) in [(0, 0), (1, 2), (2, 1), (0, 2), (1, 0)] {
                    let (mut s, a, mut h, p) = ys(flag);
                    h.port_blk_a = port_blk_a;
                    h.cassumption_a = cassumption_a;
                    h.cassumption_b = cassumption_b;
                    h.cassumption_c = cassumption_c;
                    s.mode_x = mx;
                    s.mode_y = my;
                    s.e2y = 9.5;
                    s.dy = 8.5;
                    let r1 = y_major_sample(&s, &a, &h, &p);
                    let r2 = y_major_sample(&s, &a, &h, &p);
                    assert!(r1.is_ok());
                    assert_eq!(r1.unwrap().to_bits(), r2.unwrap().to_bits());
                }
            }
        }
        let (mut s, a, h, p) = ys(1);
        s.x = f32::NAN;
        let _ = y_major_sample(&s, &a, &h, &p);
        s.y = f32::INFINITY;
        let _ = y_major_sample(&s, &a, &h, &p);
        let (mut s, a, h, p) = ys(0);
        s.y = f32::NAN;
        let _ = y_major_sample(&s, &a, &h, &p);
    }

    #[test]
    fn transpose_symmetry() {
        // square plane: X-major problem vs its transposed Y-major counterpart (swapped coordinates, flags, bounds, modes)
        let n = 8usize;
        let mut seed = 12345u32;
        let mut rnd = move || { seed = seed.wrapping_mul(1664525).wrapping_add(1013904223); (seed >> 8) as f32 / 16777216.0 };
        let mut cnt = 0;
        for t in 0..300 {
            let flag = (t % 2) as u32;
            let (mx, my) = ((t / 2) % 3, (t / 6) % 3);
            let fl = [(t / 3) % 2, (t / 5) % 2, (t / 7) % 2, (t / 11) % 2];
            let m = n + 4;
            let buf: Vec<f32> = (0..m * m).map(|_| rnd() * 4.0 - 1.0).collect();
            let origin = (2 * m + 2) as i64;
            let mut tb = buf.clone();
            for r in 0..m { for c in 0..m { tb[c * m + r] = buf[r * m + c]; } }
            let x = rnd() * 5.0 + 1.0; let y = rnd() * 5.0 + 1.0;
            let dxp = (rnd() - 0.5) * 3.0; let dyp = (rnd() - 0.5) * 1.0;
            let (e2x, e2y) = (x + dxp, y + dyp);
            let mk = |x: f32, y: f32, e2x: f32, e2y: f32, mxx: i32, myy: i32| S5State {
                sample: 0, x, y, p1x: 0.0, p1y: 0.0, p2x: 0.0, p2y: 0.0, e2x, e2y, dx: e2x - x, dy: e2y - y, adx: (e2x - x).abs(), ady: (e2y - y).abs(), y_major: false,
                data_ptr: 0, width: n as i32, height: n as i32, stride: m as i32, scx: 1.0, scy: 1.0, k_a1_a0: 1.0, k_a2_a1: 1.0, k_1_a3: 0.5, scale: 2.0, flag_cell_a: flag, mode_x: mxx, mode_y: myy,
            };
            let a = [0.0, 1.0, 2.0, 0.5, 2.0];
            let hx = Host { port_blk_a: fl[0] as u8, cassumption_a: fl[1] as u8, cassumption_b: fl[2] as u8, cassumption_c: fl[3] as u8, hs: n as f32, ws: n as f32 };
            let hy = Host { port_blk_a: fl[1] as u8, cassumption_a: fl[0] as u8, cassumption_b: fl[3] as u8, cassumption_c: fl[2] as u8, hs: n as f32, ws: n as f32 };
            let sx = mk(x, y, e2x, e2y, mx, my);
            let sy = mk(y, x, e2y, e2x, my, mx);
            let px = Plane { buf: buf.clone(), origin };
            let py = Plane { buf: tb, origin };
            let a1 = x_major_sample(&sx, &a, &hx, &px);
            let b1 = y_major_sample(&sy, &a, &hy, &py);
            if let (Ok(u), Ok(v)) = (a1, b1) {
                cnt += 1;
                assert!(u.to_bits() == v.to_bits() || (u.is_nan() && v.is_nan()), "case {} {} vs {}", t, u, v);
            }
        }
        assert!(cnt > 100, "{}", cnt);
    }
}

/// Zero-length stub : P1 renders the sample at (x, y) into the output planes, then
/// plane 0 at the sample index is multiplied by a4 (the scale saved at rbp).
fn p1_stub(c: &P1Cfg, p: &Params, inp: &Inputs, pl: &Plane, x: f32, y: f32, i: usize, out: &mut Vec<Vec<f32>>) -> Result<(), Unsupported> {
    let _ = p;
    let pp = bwb_p1::Params { n: c.n, width: c.w0, height: c.h0, flag_x: inp.mode_x, flag_y: inp.mode_y };
    let g = bwb_p1::Geometry { base: pl.origin, stride: inp.acc_stride };
    let n = c.n.max(0) as usize;
    if out.len() < n.max(1) || out.iter().any(|v| i >= v.len()) {
        return Err(Unsupported::Oob);
    }
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let r = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| bwb_p1::sample(&pp, &g, &pl.buf, x, y)));
    std::panic::set_hook(prev);
    let tmp = r.map_err(|_| Unsupported::Oob)?;
    for (k, v) in tmp.iter().enumerate() {
        out[k][i] = *v;
    }
    out[0][i] = inp.a[4] * out[0][i];
    Ok(())
}
