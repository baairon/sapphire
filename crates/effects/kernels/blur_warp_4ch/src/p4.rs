//! Body D: the Y-major half (flag_cell_a != 0 general loop, flag_cell_a == 0
//! nearest loop, finalizer stubs). Source of truth: the body D listing.

use super::*;

/// Y-major sample. Returns the four plane values (out[0..4][sample]).
pub fn y_major_sample(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<[f32; 4], Unsupported> {
    if st.flag_cell_a == 0 {
        return flag0_y(st, a, hst, pl);
    }
    let (w, h, stride) = (st.width, st.height, st.stride);
    let wf = w as f32; // [rbp], cvtsi2ss of [rbp]
    let ws = hst.ws; // 
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cond_any = mx0 != 0 || my > 0;
    let (a0, a1, a3, a4) = (a[0], a[1], a[3], a[4]);
    let (x, y, dx, dy) = (st.x, st.y, st.dx, st.dy);
    let (dir0, fy, r_start, r_end, c0, n0, dys);
    if dy > 0.0 {
        let fl = y.floor();
        fy = y - fl;
        r_start = cvtt(fl);
        let ce = st.e2y.ceil();
        r_end = cvtt(ce);
        c0 = ce - st.e2y;
        n0 = r_end.wrapping_sub(r_start).wrapping_add(1);
        dir0 = 1i32;
        dys = dy;
    } else {
        let ce = y.ceil();
        r_start = cvtt(ce);
        fy = ce - y;
        let fl = st.e2y.floor();
        c0 = st.e2y - fl;
        r_end = cvtt(fl);
        n0 = r_start.wrapping_sub(r_end).wrapping_add(1);
        dir0 = -1;
        dys = -dy;
    }
    let slope0 = dx / dys;
    let mut xx = (-slope0).mul_add(fy, x); // vfnmadd231ss 
    let x9 = slope0 + st.e2x;
    let step_a = st.k_a1_a0 / ((n0 as f32) * a3);
    let x5 = xx.floor();
    let dl = cond_any && (ja(0.0, x5) || jae(x5, ws) || r_start < 0 || r_start >= h || ja(0.0, x9) || jae(x9, ws) || r_end < 0 || r_end >= h);
    let xclamp = mx0 == 0 && (ja(0.0, x5) || jae(x5, ws) || ja(0.0, x9) || jae(x9, ws));
    let yclamp = my == 0 && (r_start < 0 || r_start >= h || r_end < 0 || r_end >= h);
    let n = n0.min(1000).max(0);
    let mut sums = [0.0f32; 4];
    let mut wsum = 0.0f32;
    LOG.with(|l| l.borrow_mut().push(if n <= 0 { "YE" } else if dl { "Y1" } else { "Y0" }));
    // coverage only: does ws change a decision compared with using the image width?
    let dl_w = cond_any && (ja(0.0, x5) || jae(x5, wf) || r_start < 0 || r_start >= h || ja(0.0, x9) || jae(x9, wf) || r_end < 0 || r_end >= h);
    let xc_w = mx0 == 0 && (ja(0.0, x5) || jae(x5, wf) || ja(0.0, x9) || jae(x9, wf));
    if n > 0 && (dl != dl_w || xclamp != xc_w) {
        LOG.with(|l| l.borrow_mut().push("Yws"));
    }
    if n > 0 && xclamp {
        LOG.with(|l| l.borrow_mut().push("Yxc"));
    }
    if n > 0 && yclamp {
        LOG.with(|l| l.borrow_mut().push("Yyc"));
    }
    if n > 0 {
        let omfy = 1.0 - fy;
        let omc0 = 1.0 - c0;
        let twows = 2.0f32.mul_add(ws, -1.0); // vfmsub213ss 
        let nf = n as f32;
        let wlim = nf * a3;
        let step_b = st.k_a2_a1 / (nf * st.k_1_a3);
        let rowmax = h.wrapping_sub(1);
        let mut slope = slope0;
        let (mut alpha, mut step, mut al) = (a0, step_a, false);
        let (mut r, mut dir) = (r_start, dir0);
        for k in 0..n {
            if !al && (k as f32) >= wlim {
                let q = k / n; // idiv, always 0
                alpha = (a3 - (q as f32)).mul_add(step_b, a1); // vfmadd213ss 
                step = step_b;
                al = true;
            }
            if dl {
                if jae(xx, ws) {
                    if hst.port_blk_a != 0 {
                        slope = -slope;
                        xx = twows - xx;
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
                if r >= h {
                    if hst.cassumption_a != 0 {
                        r = rowmax;
                        dir = -1;
                    } else if hst.cassumption_c != 0 {
                        r = 0;
                    }
                } else if r < 0 {
                    if hst.cassumption_a != 0 {
                        r = 0;
                        dir = 1;
                    } else if hst.cassumption_c != 0 {
                        r = rowmax;
                    }
                }
            }
            let wgt = if k == 0 { alpha * omfy } else if k == n - 1 { alpha * omc0 } else { alpha };
            let xcl = if xclamp { if ja(-1.0, xx) { -1.0 } else { vmin(wf, xx) } } else { xx };
            let rr = if yclamp { if r >= -1 { r.min(h) } else { -1 } } else { r };
            let rowoff = rr.wrapping_mul(stride);
            wsum = wgt + wsum;
            xx = slope + xx;
            alpha += step;
            let fr = 1.0 + xcl;
            let t = cvtt(fr);
            let frac = fr - (t as f32);
            r = r.wrapping_add(dir);
            let idx = rowoff.wrapping_add(t.wrapping_mul(4)) as i64;
            for ch in 0..4 {
                let v0 = at(pl, idx - 4 + ch as i64)?;
                let v1 = at(pl, idx + ch as i64)?;
                sums[ch] = sums[ch] - lerp_term(v0, v1, frac, wgt);
            }
        }
    }
    // finalizer (and stub): k = a4 / wsum, planes = acc * k
    let rk = a4 / wsum;
    Ok([sums[0] * rk, sums[1] * rk, sums[2] * rk, sums[3] * rk])
}

/// Y-major flag_cell_a == 0 : nearest neighbour, weights alpha only.
fn flag0_y(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<[f32; 4], Unsupported> {
    let (w, h, stride) = (st.width, st.height, st.stride);
    let hf = h as f32;
    let ws = hst.ws;
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cond_any = mx0 != 0 || my > 0;
    let (a0, a1, a3, a4) = (a[0], a[1], a[3], a[4]);
    let cs = 0.5 + st.x;
    let ce = 0.5 + st.e2x;
    let ys = 0.5 + st.y;
    let ye = 0.5 + st.e2y;
    let wrap = cond_any && (ja(0.0, ce) || jae(ce, ws) || ja(0.0, ye) || jae(ye, hf));
    let xclamp = mx0 == 0 && (ja(0.0, cs) || jae(cs, ws) || ja(0.0, ce) || jae(ce, ws));
    let yclamp = my == 0 && (ja(0.0, ys) || jae(ys, hf) || ja(0.0, ye) || jae(ye, hf));
    let r_start = cvtt(ys);
    let r_end = cvtt(ye);
    let e = r_end.wrapping_sub(r_start);
    let dir0 = if e < 0 { -1i32 } else { 1 };
    let sg = e >> 31;
    let nraw = ((e ^ sg).wrapping_sub(sg)).wrapping_add(1);
    let nf0 = nraw as f32;
    let slope_x = (ce - cs) / nf0;
    let step_a = st.k_a1_a0 / (nf0 * a3);
    let n = nraw.min(1000).max(0);
    LOG.with(|l| l.borrow_mut().push(if n <= 0 { "YFE" } else { "YF" }));
    let wrap_w = cond_any && (ja(0.0, ce) || jae(ce, w as f32) || ja(0.0, ye) || jae(ye, hf));
    let xc_w = mx0 == 0 && (ja(0.0, cs) || jae(cs, w as f32) || ja(0.0, ce) || jae(ce, w as f32));
    if n > 0 && (wrap != wrap_w || xclamp != xc_w) {
        LOG.with(|l| l.borrow_mut().push("YFws"));
    }
    if n > 0 && wrap {
        LOG.with(|l| l.borrow_mut().push("YFw"));
    }
    let mut sums = [0.0f32; 4];
    let mut wsum = 0.0f32;
    if n > 0 {
        let nf = n as f32;
        let twows = 2.0f32.mul_add(ws, -1.0); // vfmsub213ss 
        let wlim = nf * a3;
        let step_b = st.k_a2_a1 / (nf * st.k_1_a3);
        let rowmax = h.wrapping_sub(1);
        let (mut xs, mut slope, mut r, mut dir) = (cs, slope_x, r_start, dir0);
        let (mut alpha, mut step, mut al) = (a0, step_a, false);
        for k in 0..n {
            if !al && (k as f32) >= wlim {
                let q = k / n; // idiv, always 0
                alpha = (a3 - (q as f32)).mul_add(step_b, a1); // vfmadd213ss 
                step = step_b;
                al = true;
            }
            let col = cvtt(xs);
            let cc = if xclamp { if col >= -1 { col.min(w) } else { -1 } } else { col };
            let rr = if yclamp { if r >= -1 { r.min(h) } else { -1 } } else { r };
            let rowoff = rr.wrapping_mul(stride);
            r = r.wrapping_add(dir);
            wsum = wsum + alpha;
            xs = xs + slope;
            let idx = rowoff.wrapping_add(cc.wrapping_mul(4)) as i64;
            for ch in 0..4 {
                let v = at(pl, idx + ch as i64)?;
                sums[ch] = alpha * v + sums[ch];
            }
            alpha = alpha + step;
            if wrap {
                if jae(xs, ws) {
                    if hst.port_blk_a != 0 {
                        slope = -slope;
                        xs = twows - xs;
                    } else if hst.cassumption_b != 0 {
                        xs = xs - ws;
                    }
                } else if ja(0.0, xs) {
                    if hst.port_blk_a != 0 {
                        xs = -xs;
                        slope = -slope;
                    } else if hst.cassumption_b != 0 {
                        xs = xs + ws;
                    }
                }
                if r >= h {
                    if hst.cassumption_a != 0 {
                        dir = -1;
                        r = rowmax;
                    } else if hst.cassumption_c != 0 {
                        r = 0;
                    }
                } else if r < 0 {
                    if hst.cassumption_a != 0 {
                        r = 0;
                        dir = 1;
                    } else if hst.cassumption_c != 0 {
                        r = rowmax;
                    }
                }
            }
        }
    }
    // finalizer (stub)
    let rk = a4 / wsum;
    Ok([sums[0] * rk, sums[1] * rk, sums[2] * rk, sums[3] * rk])
}

#[cfg(test)]
mod tests {
    use super::*;

    // pixel (col,row) channel ch = ch*100 + row*10 + col; 4 floats per pixel; stride 40; origin 200
    fn plane() -> Plane {
        let mut buf = vec![0.0f32; 1000];
        for row in 0..12i64 {
            for col in 0..10i64 {
                for ch in 0..4i64 {
                    buf[(200 + row * 40 + col * 4 + ch) as usize] = (ch * 100 + row * 10 + col) as f32;
                }
            }
        }
        Plane { buf: std::sync::Arc::new(buf), origin: 200 }
    }

    fn base(flag: u32, mx: i32, my: i32) -> (S5State, [f32; 5], Host, Plane) {
        // x = 2.0, y from 1.0 to 3.0 (dy = 2): rows 1,2,3 of column 2 (frac 0)
        let st = S5State {
            sample: 0, x: 2.0, y: 1.0, p1x: 0.0, p1y: 0.0, p2x: 0.0, p2y: 0.0, e2x: 2.0, e2y: 3.0, dx: 0.0, dy: 2.0, adx: 0.0, ady: 2.0, y_major: true,
            data_ptr: 0, width: 8, height: 8, stride: 40, scx: 1.0, scy: 1.0, k_a1_a0: 0.0, k_a2_a1: 0.0, k_1_a3: 0.5, scale: 1.0, flag_cell_a: flag, mode_x: mx, mode_y: my,
        };
        (st, [1.0, 1.0, 1.0, 0.5, 1.0], Host { port_blk_a: 1, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 8.0, ws: 8.0 }, plane())
    }

    #[test]
    fn general_hand_rows() {
        // y: 1..3, fy = 0, c0 = 0 -> n = 3 rows 1,2,3, weights 0,1,0 (end weights 1-fy = 1, 1-c0 = 1): weights 1,1,1
        // x = 2.0 -> fr = 3.0, t = 3, frac 0, pair (col 2, col 3): v0 = col 2. ch0 = (12 + 22 + 32)/3 = 22 + 100*ch
        let (s, a, h, p) = base(1, 0, 0);
        let v = y_major_sample(&s, &a, &h, &p).unwrap();
        for ch in 0..4 {
            assert!((v[ch] - (22.0 + 100.0 * ch as f32)).abs() < 1e-3, "{:?}", v);
        }
    }

    #[test]
    fn flag0_hand_rows() {
        // ys = 1.5, ye = 3.5 -> rows 1,2,3 of column cvtt(2.5) = 2: same values
        let (s, a, h, p) = base(0, 0, 0);
        let v = y_major_sample(&s, &a, &h, &p).unwrap();
        for ch in 0..4 {
            assert!((v[ch] - (22.0 + 100.0 * ch as f32)).abs() < 1e-3, "{:?}", v);
        }
    }

    #[test]
    fn downward_direction_matches() {
        // dy < 0: y 3 -> 1, same rows, same sum
        for flag in [0u32, 1] {
            let (mut s, a, h, p) = base(flag, 0, 0);
            s.y = 3.0;
            s.e2y = 1.0;
            s.dy = -2.0;
            let v = y_major_sample(&s, &a, &h, &p).unwrap();
            for ch in 0..4 {
                assert!((v[ch] - (22.0 + 100.0 * ch as f32)).abs() < 1e-3, "{flag} {:?}", v);
            }
        }
    }

    #[test]
    fn column_clamp_reads_edge() {
        // x = 9.0 with width 8, mode_x = 0: xclamp -> xcl = min(8, 9) = 8, fr = 9, t = 9: pair (col 8, col 9)... frac 0 -> v0 = col 8
        let (mut s, a, h, p) = base(1, 0, 0);
        s.x = 9.0;
        s.e2x = 9.0;
        let v = y_major_sample(&s, &a, &h, &p).unwrap();
        assert!((v[0] - (18.0 + 28.0 + 38.0 - 0.0) / 3.0).abs() < 1e-3, "{:?}", v);
    }

    #[test]
    fn ws_binds_the_selector() {
        // with mode_x = 1 the dl test uses ws: x5 = 2 >= ws = 2 -> wrap loop (Y1); ws = 8 -> direct loop (Y0)
        let (s, a, mut h, p) = base(1, 1, 0);
        h.ws = 8.0;
        LOG.with(|l| l.borrow_mut().clear());
        let _ = y_major_sample(&s, &a, &h, &p).unwrap();
        h.ws = 2.0;
        let _ = y_major_sample(&s, &a, &h, &p).unwrap();
        let got: Vec<&str> = LOG.with(|l| l.borrow().iter().copied().filter(|s| *s == "Y0" || *s == "Y1").collect());
        assert_eq!(got, vec!["Y0", "Y1"]);
        let all = LOG.with(|l| l.borrow().clone());
        assert!(all.contains(&"Yws"), "{:?}", all);
    }

    #[test]
    fn determinism_finite_and_nan_edges() {
        for flag in [0u32, 1] {
            for (port_blk_a, cassumption_a, cassumption_b, cassumption_c) in [(0, 0, 0, 0), (1, 1, 1, 1), (0, 1, 0, 1), (1, 0, 1, 0)] {
                for (mx, my) in [(0, 0), (1, 0), (2, 1), (1, 2), (0, 2)] {
                    let (mut s, a, mut h, p) = base(flag, mx, my);
                    h.port_blk_a = port_blk_a;
                    h.cassumption_a = cassumption_a;
                    h.cassumption_b = cassumption_b;
                    h.cassumption_c = cassumption_c;
                    s.y = -1.5;
                    s.e2y = 6.5;
                    s.dy = 8.0;
                    s.x = 3.0;
                    s.e2x = 4.0;
                    s.dx = 1.0;
                    let r1 = y_major_sample(&s, &a, &h, &p);
                    let r2 = y_major_sample(&s, &a, &h, &p);
                    assert_eq!(r1.as_ref().map(|v| v.map(f32::to_bits)), r2.as_ref().map(|v| v.map(f32::to_bits)));
                    if let Ok(v) = r1 {
                        assert!(v.iter().all(|f| f.is_finite()), "{:?}", v);
                    }
                }
            }
        }
        let (mut s, a, h, p) = base(1, 1, 0);
        s.x = f32::NAN;
        let _ = y_major_sample(&s, &a, &h, &p);
        s.x = f32::INFINITY;
        let _ = y_major_sample(&s, &a, &h, &p);
    }

    #[test]
    fn empty_n_goes_to_finalizer() {
        // dy > 0 with ceil(e2y) < floor(y): n <= 0 -> 0 * (a4 / 0) = NaN
        // flag_cell_a != 0 only: the flag_cell_a == 0 count is |row_end - row_start| + 1 >= 1 and never empty
        let (mut s, a, h, p) = base(1, 0, 0);
        s.y = 5.0;
        s.e2y = 1.5;
        s.dy = 1.0;
        let r = y_major_sample(&s, &a, &h, &p).unwrap();
        assert!(r.iter().all(|f| f.is_nan()), "{:?}", r);
        let (mut s, a, h, p) = base(0, 0, 0);
        s.y = 5.0;
        s.e2y = 1.5;
        s.dy = 1.0;
        let r = y_major_sample(&s, &a, &h, &p).unwrap();
        assert!(r.iter().all(|f| f.is_finite()), "{:?}", r);
    }

    #[test]
    fn selector_nan_takes_y_major() {
        // run_planes_full: !(adx >= ady) is true for NaN (vcomiss ... jb)
        let (adx, ady) = (f32::NAN, 1.0f32);
        assert!(!(adx >= ady));
        let (adx, ady) = (1.0f32, f32::NAN);
        assert!(!(adx >= ady));
        assert!(!(1.0f32 >= 2.0f32));
        assert!(2.0f32 >= 2.0f32);
    }
}
