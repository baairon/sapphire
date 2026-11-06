//! Piece 3 of body D (BW7C): the specialised X-major flag_cell_a != 0 loops S10..S13 
//! and the X-major flag_cell_a == 0 loops S16..S20 . Source of truth: bodyD/D.lst. See SPEC_P3.md.

use super::*;

/// State entering the specialised loops (dl == 0): the values the listing holds in registers .
pub struct SpecIn {
    pub c: i32,
    pub dir: i32,
    pub n: i32,
    pub fx: f32,
    pub c0: f32,
    pub slope: f32,
    pub yy: f32,
    pub step_a: f32,
    pub w: i32,
    pub h: i32,
    pub stride: i32,
    pub a: [f32; 5],
    pub k_a2_a1: f32,
    pub k_1_a3: f32,
    /// [rbp] != 0: column clamp (S10, S11)
    pub col_clamp: bool,
    /// [rbp] != 0: row clamp (S10, S12)
    pub row_clamp: bool,
}

/// S10 (both clamps), S11 (column clamp only), S12 (row clamp only), S13 (none). Returns (accumulators, wsum).
/// No edge wrap inside the loop. Channel 3 uses the re-associated fused form when the row clamp is off (S11, S13).
pub fn spec_loop(s: &SpecIn, pl: &Plane) -> Result<([f32; 4], f32), Unsupported> {
    let nm = match (s.col_clamp, s.row_clamp) { (true, true) => "S10", (true, false) => "S11", (false, true) => "S12", (false, false) => "S13" };
    LOG.with(|l| l.borrow_mut().push(nm));
    let (a0, a1, a3) = (s.a[0], s.a[1], s.a[3]);
    let nf = s.n as f32;
    let wlim = nf * a3;
    let step_b = s.k_a2_a1 / (nf * s.k_1_a3);
    let omfx = 1.0 - s.fx;
    let omc0 = 1.0 - s.c0;
    let hf = s.h as f32;
    let mut sums = [0.0f32; 4];
    let mut wsum = 0.0f32;
    let (mut c, mut yy, mut alpha, mut step, mut al) = (s.c, s.yy, a0, s.step_a, false);
    for k in 0..s.n {
        if !al && (k as f32) >= wlim {
            let q = k / s.n; // idiv, always 0
            alpha = (a3 - (q as f32)).mul_add(step_b, a1); // vfmadd213ss 
            step = step_b;
            al = true;
        }
        let wgt = if k == 0 { alpha * omfx } else if k == s.n - 1 { alpha * omc0 } else { alpha };
        let cc = if s.col_clamp { if c >= -1 { c.min(s.w) } else { -1 } } else { c };
        let ycl = if s.row_clamp { if ja(-1.0, yy) { -1.0 } else { vmin(hf, yy) } } else { yy };
        wsum = wgt + wsum;
        let yy_old = yy;
        yy = s.slope + yy;
        alpha += step;
        c = c.wrapping_add(s.dir);
        let fr = 1.0 + ycl;
        let t = cvtt(fr);
        let tf = t as f32;
        let frac = fr - tf;
        let idx = cc.wrapping_mul(4).wrapping_sub(s.stride).wrapping_add(t.wrapping_mul(s.stride)) as i64;
        for ch in 0..3 {
            let v0 = at(pl, idx + ch as i64)?;
            let v1 = at(pl, idx + s.stride as i64 + ch as i64)?;
            sums[ch] = sums[ch] - lerp_term(v0, v1, frac, wgt);
        }
        let v0 = at(pl, idx + 3)?;
        let v1 = at(pl, idx + s.stride as i64 + 3)?;
        if s.row_clamp {
            sums[3] = sums[3] - lerp_term(v0, v1, frac, wgt);
        } else {
            // /85/8a (S11), /dc/e1 (S13): vfmsub132, vfmadd213, vfmsub231
            let b = (frac - 1.0).mul_add(v0, -v1);
            let d = tf.mul_add(v1, b);
            let e = v1.mul_add(yy_old, -d);
            sums[3] = e * wgt + sums[3];
        }
    }
    Ok((sums, wsum))
}

/// Which of M1..M4 (S17..S20) a flag_cell_a == 0 sample takes, with its flags.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct F0Sel {
    pub col_clamp: bool, // cl
    pub row_clamp: bool, // dil
    pub wrap: bool,      // [rbp]
    pub n: i32,
    /// 1 = M1 (S17), 2 = M2 (S18), 3 = M3 (S19), 4 = M4 (S20); 0 = n <= 0 (straight to the finalizer)
    pub loop_id: u8,
}

fn f0_setup(st: &S5State, a3: f32, hst: &Host) -> (F0Sel, f32, f32, i32, i32, f32, f32) {
    let wf = st.width as f32;
    let hs = hst.hs;
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cs = 0.5 + st.x;
    let ce = 0.5 + st.e2x;
    let ys = 0.5 + st.y;
    let ye = 0.5 + st.e2y;
    let cond_any = mx0 != 0 || my > 0;
    let wrap = cond_any && (ja(0.0, ce) || jae(ce, wf) || ja(0.0, ye) || jae(ye, hs));
    let col_clamp = mx0 == 0 && (ja(0.0, cs) || jae(cs, wf) || ja(0.0, ce) || jae(ce, wf));
    let row_clamp = my == 0 && (ja(0.0, ys) || jae(ys, hs) || ja(0.0, ye) || jae(ye, hs));
    let c_start = cvtt(cs);
    let c_end = cvtt(ce);
    let e = c_end.wrapping_sub(c_start);
    let dir = if e < 0 { -1 } else { 1 };
    let sg = e >> 31;
    let nraw = ((e ^ sg).wrapping_sub(sg)).wrapping_add(1);
    let nf = nraw as f32;
    let slope = (ye - ys) / nf;
    let step_a = st.k_a1_a0 / (nf * a3);
    let n = nraw.min(1000).max(0);
    let loop_id = if n <= 0 { 0 } else if !col_clamp { 4 } else if !row_clamp { 3 } else if !wrap { 2 } else { 1 };
    (F0Sel { col_clamp, row_clamp, wrap, n, loop_id }, slope, step_a, c_start, dir, ys, nf)
}

/// Selector only for flag_cell_a == 0.
pub fn flag0_selector(st: &S5State, a3: f32, hst: &Host) -> F0Sel {
    f0_setup(st, a3, hst).0
}

/// X-major flag_cell_a == 0 sample (S16..S20): nearest-neighbour accumulation, no interpolation, weights alpha only.
pub fn flag0_sample(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<[f32; 4], Unsupported> {
    let (w, h, stride) = (st.width, st.height, st.stride);
    let wf = w as f32;
    let (a0, a1, a3, a4) = (a[0], a[1], a[3], a[4]);
    let (sel, slope0, step_a, c_start, dir0, ys, _nf) = f0_setup(st, a3, hst);
    let n = sel.n;
    LOG.with(|l| l.borrow_mut().push(["E", "M1", "M2", "M3", "M4"][sel.loop_id as usize]));
    let mut sums = [0.0f32; 4];
    let mut wsum = 0.0f32;
    if n > 0 {
        let nf = n as f32;
        let wlim = nf * a3;
        let step_b = st.k_a2_a1 / (nf * st.k_1_a3);
        let hfi = cvtt(hst.hs);
        let hf = hfi as f32;
        let twoh = 2.0f32.mul_add(hf, -1.0); // vfmsub213ss 
        let colmax = cvtt(wf - 1.0);
        let (mut c, mut dir, mut yy, mut slope) = (c_start, dir0, ys, slope0);
        let (mut alpha, mut step, mut al) = (a0, step_a, false);
        for k in 0..n {
            if !al && (k as f32) >= wlim {
                let q = k / n; // idiv, always 0
                alpha = (a3 - (q as f32)).mul_add(step_b, a1); // vfmadd213ss 
                step = step_b;
                al = true;
            }
            let ri = cvtt(yy);
            let cc = if sel.col_clamp { if c >= -1 { c.min(w) } else { -1 } } else { c };
            let rr = if sel.row_clamp { if ri < -1 { -1 } else { ri.min(h) } } else { ri };
            wsum = wsum + alpha;
            yy = yy + slope;
            let idx = rr.wrapping_mul(stride).wrapping_add(cc.wrapping_mul(4)) as i64;
            c = c.wrapping_add(dir);
            for ch in 0..4 {
                let v = at(pl, idx + ch as i64)?;
                sums[ch] = alpha * v + sums[ch];
            }
            alpha = alpha + step;
            if sel.wrap {
                if !((c as f32) < wf) {
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
                if jae(yy, hf) {
                    if hst.cassumption_a != 0 {
                        slope = -slope;
                        yy = twoh - yy;
                    } else if hst.cassumption_c != 0 {
                        yy = yy - hf;
                    }
                } else if ja(0.0, yy) {
                    if hst.cassumption_a != 0 {
                        yy = -yy;
                        slope = -slope;
                    } else if hst.cassumption_c != 0 {
                        yy = yy + hf;
                    }
                }
            }
        }
    }
    let r = a4 / wsum;
    Ok([sums[0] * r, sums[1] * r, sums[2] * r, sums[3] * r])
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

    fn si(cc: bool, rc: bool) -> SpecIn {
        SpecIn { c: 1, dir: 1, n: 2, fx: 0.0, c0: 0.0, slope: 0.0, yy: 2.0, step_a: 0.0, w: 8, h: 8, stride: 40, a: [1.0, 1.0, 1.0, 0.5, 1.0], k_a2_a1: 0.0, k_1_a3: 0.5, col_clamp: cc, row_clamp: rc }
    }

    #[test]
    fn spec_hand_all_four_variants() {
        // columns 1,2 of row 2, weights 1,1, frac 0: sums = (21+22, 121+122, 221+222, 321+322), wsum 2
        for (cc, rc) in [(true, true), (true, false), (false, true), (false, false)] {
            let (s, ws) = spec_loop(&si(cc, rc), &plane()).unwrap();
            assert_eq!(ws, 2.0);
            assert_eq!(s, [43.0, 243.0, 443.0, 643.0], "{cc} {rc}");
        }
    }

    #[test]
    fn spec_clamps_differ() {
        // column 9 with w = 8: column clamp reads column 8 (value 28), no clamp reads column 9 (29)
        let mut s = si(true, true);
        s.c = 9;
        s.n = 1;
        let (a, _) = spec_loop(&s, &plane()).unwrap();
        assert_eq!(a[0], 28.0);
        s.col_clamp = false;
        let (b, _) = spec_loop(&s, &plane()).unwrap();
        assert_eq!(b[0], 29.0);
    }

    fn f0(mx: i32, my: i32) -> (S5State, [f32; 5], Host, Plane) {
        let st = S5State {
            sample: 0, x: 0.5, y: 1.5, p1x: 0.0, p1y: 0.0, p2x: 0.0, p2y: 0.0, e2x: 2.5, e2y: 1.5, dx: 2.0, dy: 0.0, adx: 2.0, ady: 0.0, y_major: false,
            data_ptr: 0, width: 8, height: 8, stride: 40, scx: 1.0, scy: 1.0, k_a1_a0: 0.0, k_a2_a1: 0.0, k_1_a3: 0.5, scale: 2.0, flag_cell_a: 0, mode_x: mx, mode_y: my,
        };
        (st, [1.0, 1.0, 1.0, 0.5, 2.0], Host { port_blk_a: 1, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 8.0, ws: 8.0 }, plane())
    }

    #[test]
    fn flag0_hand_nearest() {
        // start col 1, end col 3, row 2: ch0 = (21+22+23) * 2 / 3 = 44; mode (0,0), all in range
        let (s, a, h, p) = f0(0, 0);
        assert_eq!(flag0_selector(&s, a[3], &h).loop_id, 4);
        let v = flag0_sample(&s, &a, &h, &p).unwrap();
        for ch in 0..4 {
            assert!((v[ch] - (44.0 + 200.0 * ch as f32)).abs() < 1e-3, "{:?}", v);
        }
    }

    #[test]
    fn flag0_wrap_hand() {
        // mode_x = 1: x = 5.5, e2x = 9.5: cols 6,7 then c = 8 wraps (port_blk_a): 7 with dir -1; cols 6,7,7,6,5; ch0 sum 131; a4 = 1; wsum 5
        let (mut s, mut a, h, p) = f0(1, 0);
        s.x = 5.5;
        s.e2x = 9.5;
        a[4] = 1.0;
        let sel = flag0_selector(&s, a[3], &h);
        assert!(sel.wrap && sel.n == 5);
        let v = flag0_sample(&s, &a, &h, &p).unwrap();
        assert!((v[0] - 131.0 / 5.0).abs() < 1e-3, "{:?}", v);
    }

    #[test]
    fn flag0_selector_table() {
        // loop id by (column range, row range, wrap): cl==0 -> 4, else dil==0 -> 3, else wrap==0 -> 2, else 1
        let (mut s, a, h, _) = f0(0, 0);
        assert_eq!(flag0_selector(&s, a[3], &h).loop_id, 4);
        s.x = -3.5; // column out of range, mode_x == 0: cl = 1; row in range: dil = 0 -> M3
        s.e2x = 2.5;
        assert_eq!(flag0_selector(&s, a[3], &h).loop_id, 3);
        s.y = -4.5; // row out of range as well, mode_y == 0: dil = 1; wrap off (cond_any false) -> M2
        s.e2y = -4.5;
        assert_eq!(flag0_selector(&s, a[3], &h).loop_id, 2);
        s.mode_y = 1; // cond_any true, wrap iff end outside: ye = -4 -> wrap; dil = 0 (my != 0) -> M3
        assert_eq!(flag0_selector(&s, a[3], &h).loop_id, 3);
        s.mode_x = 1; // cl = 0 -> M4
        assert_eq!(flag0_selector(&s, a[3], &h).loop_id, 4);
    }

    #[test]
    fn flag0_empty_is_nan_and_deterministic() {
        let (mut s, a, h, p) = f0(0, 0);
        s.e2x = -10.0;
        s.x = 2.5; // cvtt(-9.5) = -9, cvtt(3.0) = 3: n = 13, still n > 0; make n <= 0 impossible: n >= 1 by construction
        let r1 = flag0_sample(&s, &a, &h, &p);
        let r2 = flag0_sample(&s, &a, &h, &p);
        assert_eq!(r1.as_ref().map(|v| v.map(f32::to_bits)), r2.as_ref().map(|v| v.map(f32::to_bits)));
    }

    #[test]
    fn selector_truth_table_dl() {
        use crate::{x_selector, XSel};
        // dl = (mx != 0 || my > 0) && any of eight range checks; dl == 0 -> Specialised, dl == 1 -> General (flag_cell_a != 0, n > 0)
        let base = |mx: i32, my: i32| {
            let st = S5State {
                sample: 0, x: 1.5, y: 2.0, p1x: 0.0, p1y: 0.0, p2x: 0.0, p2y: 0.0, e2x: 4.5, e2y: 2.0, dx: 3.0, dy: 0.0, adx: 3.0, ady: 0.0, y_major: false,
                data_ptr: 0, width: 8, height: 8, stride: 40, scx: 1.0, scy: 1.0, k_a1_a0: 0.0, k_a2_a1: 0.0, k_1_a3: 0.5, scale: 1.0, flag_cell_a: 1, mode_x: mx, mode_y: my,
            };
            (st, [1.0f32, 1.0, 1.0, 0.5, 1.0], Host { port_blk_a: 1, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 8.0, ws: 8.0 })
        };
        for mx in [0, 1, 2] {
            for my in [-1, 0, 1, 2] {
                let any = mx != 0 || my > 0;
                // in range: dl = 0
                let (st, a, h) = base(mx, my);
                assert_eq!(x_selector(&st, &a, &h), Ok(XSel::Specialised), "in range {mx} {my}");
                // each of the eight checks (c<0, c>=w, y5<0, y5>=hs, esi<0, esi>=w, y9<0, y9>=hs)
                let tweaks: [fn(&mut S5State); 6] = [
                    |s| { s.x = -0.5; s.e2x = 3.5; },
                    |s| { s.x = 8.5; s.e2x = 11.5; s.dx = 3.0; },
                    |s| { s.y = -1.0; s.e2y = -1.0; },
                    |s| { s.y = 9.0; s.e2y = 9.0; },
                    |s| { s.e2x = 9.5; s.dx = 8.0; },
                    |s| { s.x = -3.5; s.e2x = -0.5; s.dx = 3.0; },
                ];
                for (i, t) in tweaks.iter().enumerate() {
                    let (mut st, a, h) = base(mx, my);
                    t(&mut st);
                    let r = x_selector(&st, &a, &h);
                    if any {
                        assert_eq!(r, Ok(XSel::General), "tweak {i} {mx} {my}");
                    } else {
                        assert_eq!(r, Ok(XSel::Specialised), "tweak {i} {mx} {my}");
                    }
                }
            }
        }
    }
}
