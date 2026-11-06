//! blur_warp_bidirectional body D (mode 4), pieces 1 and 2: S0..S7 (shared with A through
//! bwb_bodya::{s0_scales, s2_x, s3_y, run}), the zero-length stub with P1, the finalizers, and the X-major flag_cell_a != 0
//! GENERAL loop (S8, S9, S14, S15) with four interleaved channels and four output planes.
//! Not ported (reported as Unsupported, never guessed): specialised loops S10..S13, flag_cell_a == 0 X-major S16..S20, Y-major S22..S27.
//! Source of truth: bodyD/D.lst. VEX follows the SDM, not silicon.

pub mod p3;
pub mod p4;
thread_local! {
    /// per-sample path labels of the last run_planes_full (T2 coverage only)
    pub static LOG: std::cell::RefCell<Vec<&'static str>> = std::cell::RefCell::new(Vec::new());
}
use bwb_bodya::p2::{Host, P1Cfg, Plane};
pub use bwb_bodya::{cvtt, run, s0_scales, s2_x, s3_y, Inputs, Output, Params, Rec, S5State};

#[derive(Debug, PartialEq)]
pub enum Unsupported {
    /// error of the shared start (A's enum)
    Shared(bwb_bodya::Unsupported),
    /// X-major flag_cell_a != 0 with the edge-free selector (dl == 0): loops L1..L4, S10..S13
    SpecialisedLoop,
    /// X-major flag_cell_a == 0 (S16..S20)
    FlagZero,
    /// |dx| < |dy| (S22..S27)
    YMajor,
    /// zero-length segment needs P1
    P1Needed,
    /// source index outside the modelled plane buffer or P1 panicked
    Oob,
}

impl From<bwb_bodya::Unsupported> for Unsupported {
    fn from(e: bwb_bodya::Unsupported) -> Self {
        Unsupported::Shared(e)
    }
}

pub struct DAssumption {
    pub id: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<DAssumption> {
    vec![
        DAssumption { id: "BD1", basis: "S0..S6 of D are the same arithmetic as A with other registers; reused from bwb_bodya (s0_scales, s2_x, s3_y, run); proved only by the call-record T2 and the plane T2", label: "MEASURED listing mnemonic diff (BW7A) + T2" },
        DAssumption { id: "BD2", basis: "source is 4 interleaved floats per pixel: idx = 4*col - stride + t*stride (i32 wrapping), lerp pair is idx and idx+stride; stride (result, floats) is an explicit input as in A (B2)", label: "MEASURED D.lst " },
        DAssumption { id: "BD3", basis: "selector: after n<=0 (jle, straight to the finalizer) the general loop runs iff dl==1, dl = (mode_x!=0 || mode_y>0) && (col c<0 || c>=width || 0>y5 || y5>=hs || esi<0 || esi>=width || 0>y9 || y9>=hs); dl==0 selects L1..L4 (not ported); flag_cell_a==0 selects S16..S20; |dx|<|dy| (vcomiss jb, NaN included) selects Y-major", label: "MEASURED D.lst, " },
        DAssumption { id: "BD4", basis: "U4 resolved: the weight ramp divides k by n with idiv; k < n always, so the quotient is 0 and alpha = fma(a3 - 0, step_b, a1) (vfmadd213ss); n > 0 so no #DE", label: "MEASURED D.lst " },
        DAssumption { id: "BD5", basis: "vcomiss ... jb on the y wrap sends NaN to the else branch, the port uses (a >= b) as first branch (false for NaN) as in C; NaN not in the T2 grid", label: "INFERRED from listing" },
        DAssumption { id: "BD6", basis: "zero-length stub : P1 writes n planes of the output object (n = image, explicit input, T2 uses 4); afterwards planes 0..3 at the sample index are each multiplied by a4 ([rbp])", label: "MEASURED D.lst; P1 plane count INFERRED from bwb_p1 contract (n <= 12)" },
        DAssumption { id: "BD7", basis: "host statics ..2b (bytes), (hs) explicit inputs; the general loop does not read (that is Y-major)", label: "ASSUMPTION" },
        DAssumption { id: "BD8", basis: "FMA sites in the ported sections: S2, S3, S5, S8, S14 (11 of 51); modelled with mul_add; no rcp/rsqrt/x87", label: "MEASURED listing grep (SPEC.md)" },
        DAssumption { id: "BD9", basis: "specialised loops S10..S13 are chosen by the two clamp bytes: [rbp]==0 -> S12 (row clamp only if [rbp]!=0) or S13 (none); else [rbp]==0 -> S11 (column clamp only); else S10 (both). SECTIONS.tsv label of S10 ('both clamps off') is inverted. They are the general loop minus the edge wraps; channel 3 uses a re-associated FMA chain when the row clamp is off (S11, S13)", label: "MEASURED D.lst " },
        DAssumption { id: "BD10", basis: "flag_cell_a==0 X-major (S16..S20): nearest-neighbour, weight alpha only, no 1-fx/1-c0 end weights; start/end are (x+0.5) and (e2x+0.5), n = |cvtt(e2x+.5)-cvtt(x+.5)|+1 (slope and step_a use the uncapped n, the loops the capped n); loops by cl (column range), dil (row range), [rbp] (wrap): cl==0 M4, else dil==0 M3, else wrap==0 M2, else M1; wrap checks run after the step (not before as in the general loop)", label: "MEASURED D.lst " },
        DAssumption { id: "BD11", basis: "constants = 0.5, = 2.0, = 1.0, = -1.0 as in SPEC.md (A's measured set); not re-read here", label: "INFERRED from SPEC.md" },
        DAssumption { id: "BD12", basis: "FMA sites added by BW7C: S10; S11; S12; S13; flag_cell_a==0: (S17), (S18), (S19), (S20) (24 + 7 = 31 sites; total with BW7B 42 of 51)", label: "MEASURED D.lst vfmadd/vfmsub grep" },
        DAssumption { id: "BD13", basis: "Y-major (|dx| < |dy| or NaN) has only two loops, one per flag_cell_a value; it has no specialised variants: the wrap block of the flag_cell_a != 0 loop is gated by dl (r11b) inside one loop, the column and row clamps by bytes [rbp] and cl; the minor axis is x (wraps use host port_blk_a/cassumption_b, bound ws, twice ws minus one), the major axis is the row (wraps use cassumption_a/cassumption_c, bound = image height); pairs are adjacent pixels (idx-4, idx) not rows", label: "MEASURED D.lst " },
        DAssumption { id: "BD14", basis: "FMA sites in Y-major (9 of 51):; with BW7B/BW7C 51 of 51", label: "MEASURED D.lst vf*madd/msub grep" },
        DAssumption { id: "BD15", basis: "Y-major bounds: x5, x9, the x clamp and the x wrap compare against ws (host static, explicit input), not the image width; rows compare against the image height as integers; hs is not read in Y-major (grep)", label: "MEASURED D.lst " },
        DAssumption { id: "BD16", basis: "the Y-major n <= 0 path jumps to the finalizer without loading the host flags; result is 0 * (a4 / 0) = NaN, same as X-major", label: "MEASURED D.lst " },
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

fn lerp_term(v0: f32, v1: f32, frac: f32, wgt: f32) -> f32 {
    (v0 - v1).mul_add(frac, -v0) * wgt
}

/// Result of the S8/S9 selector for an X-major flag_cell_a != 0 sample.
#[derive(Debug, PartialEq)]
pub enum XSel {
    /// n <= 0: straight to the finalizer
    Empty,
    General,
    Specialised,
    /// flag_cell_a == 0 (S16..S20), added by BW7C
    FlagZero,
}

/// X-major flag_cell_a != 0 sample (S8, S9, S14, S15); returns out[0..4][sample].
pub fn x_major_sample(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane) -> Result<[f32; 4], Unsupported> {
    x_major_sample_ex(st, a, hst, pl, &mut None)
}

/// Same, and reports the selected path in `sel`.
pub fn x_major_sample_ex(st: &S5State, a: &[f32; 5], hst: &Host, pl: &Plane, sel: &mut Option<XSel>) -> Result<[f32; 4], Unsupported> {
    let (w, h, stride) = (st.width, st.height, st.stride);
    let wf = w as f32;
    let hs = hst.hs;
    let (mx0, my) = (st.mode_x, st.mode_y);
    let cond_any = mx0 != 0 || my > 0;
    let (a0, a1, a3, a4) = (a[0], a[1], a[3], a[4]);
    if st.flag_cell_a == 0 {
        *sel = Some(XSel::FlagZero);
        return p3::flag0_sample(st, a, hst, pl);
    }
    let mut sums = [0.0f32; 4];
    let mut wsum = 0.0f32;
    let (x, y, dx, dy) = (st.x, st.y, st.dx, st.dy);
    let (mut dir, mut c, esi, n0, fx, c0, dxs);
    if dx > 0.0 {
        let fl = x.floor();
        dir = 1i32;
        fx = x - fl;
        c = cvtt(fl);
        let ce = st.e2x.ceil();
        esi = cvtt(ce);
        c0 = ce - st.e2x;
        n0 = esi.wrapping_sub(c).wrapping_add(1);
        dxs = dx;
    } else {
        let ce = x.ceil();
        dir = -1;
        c = cvtt(ce);
        let fl = st.e2x.floor();
        c0 = st.e2x - fl;
        fx = ce - x;
        esi = cvtt(fl);
        n0 = c.wrapping_sub(esi).wrapping_add(1);
        dxs = -dx;
    }
    let slope0 = dy / dxs;
    let mut yy = (-slope0).mul_add(fx, y); // vfnmadd231ss 
    let step_a = st.k_a1_a0 / ((n0 as f32) * a3);
    let y5 = yy.floor();
    let y9 = slope0 + st.e2y;
    let dl = if cond_any {
        c < 0 || c >= w || ja(0.0, y5) || jae(y5, hs) || esi < 0 || esi >= w || ja(0.0, y9) || jae(y9, hs)
    } else {
        false
    };
    let direct0 = cond_any && !dl;
    let b_cell_m = if direct0 || mx0 != 0 { false } else { c < 0 || c >= w || esi < 0 || esi >= w };
    let b_cell_l = if my != 0 { false } else { ja(0.0, y5) || jae(y5, hs) || ja(0.0, y9) || jae(y9, hs) };
    let n = n0.min(1000).max(0);
    *sel = Some(if n <= 0 { XSel::Empty } else if dl { XSel::General } else { XSel::Specialised });
    if n > 0 && !dl {
        let si = p3::SpecIn {
            c, dir, n, fx, c0, slope: slope0, yy, step_a, w, h, stride, a: *a, k_a2_a1: st.k_a2_a1, k_1_a3: st.k_1_a3,
            col_clamp: b_cell_m, row_clamp: b_cell_l,
        };
        let (s, ws) = p3::spec_loop(&si, pl)?;
        sums = s;
        wsum = ws;
    } else if n > 0 {
        let mut slope = slope0;
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
                let q = k / n; // idiv, always 0
                alpha = (a3 - (q as f32)).mul_add(step_b, a1); // vfmadd213ss 
                step = step_b;
                al = true;
            }
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
            let wgt = if k == 0 { alpha * omfx } else if k == n - 1 { alpha * omc0 } else { alpha };
            let cc = if b_cell_m { if c >= -1 { c.min(w) } else { -1 } } else { c };
            let ycl = if b_cell_l { if ja(-1.0, yy) { -1.0 } else { vmin(hf, yy) } } else { yy };
            wsum = wgt + wsum;
            yy = slope + yy;
            alpha += step;
            c = c.wrapping_add(dir);
            let fr = 1.0 + ycl;
            let t = cvtt(fr);
            let frac = fr - (t as f32);
            let idx = cc.wrapping_mul(4).wrapping_sub(stride).wrapping_add(t.wrapping_mul(stride)) as i64;
            for ch in 0..4 {
                let v0 = at(pl, idx + ch as i64)?;
                let v1 = at(pl, idx + stride as i64 + ch as i64)?;
                sums[ch] = sums[ch] - lerp_term(v0, v1, frac, wgt);
            }
        }
    }
    // S15 finalizer : k = a4 / wsum, planes = acc * k
    let r = a4 / wsum;
    Ok([sums[0] * r, sums[1] * r, sums[2] * r, sums[3] * r])
}

/// Selector only (no sampling): which path the original takes for this sample.
pub fn x_selector(st: &S5State, a: &[f32; 5], hst: &Host) -> Result<XSel, Unsupported> {
    let pl = Plane { buf: std::sync::Arc::new(vec![]), origin: 0 };
    let mut sel = None;
    let _ = x_major_sample_ex(st, a, hst, &pl, &mut sel);
    sel.ok_or(Unsupported::Oob)
}

/// Zero-length stub : P1 renders (x, y) into the planes; then planes 0..3 at the sample index are multiplied by a4.
fn p1_stub(c: &P1Cfg, inp: &Inputs, pl: &Plane, x: f32, y: f32, i: usize, out: &mut Vec<Vec<f32>>) -> Result<(), Unsupported> {
    let pp = bwb_p1::Params { n: c.n, width: c.w0, height: c.h0, flag_x: inp.mode_x, flag_y: inp.mode_y };
    let g = bwb_p1::Geometry { base: pl.origin, stride: inp.acc_stride };
    let n = c.n.max(0) as usize;
    if out.len() < n.max(4) || out.iter().any(|v| i >= v.len()) {
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
    for k in 0..4 {
        out[k][i] = inp.a[4] * out[k][i];
    }
    Ok(())
}

/// Ported part of body D: zero-length samples and X-major flag_cell_a != 0 general loop; out[0..4] are the four planes.
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
        let e2x = (l_p2x - l_p1x).mul_add(scx, x); // 
        let e2y = (-(l_p2y - l_p1y)).mul_add(scy, y); // 
        if x == e2x && y == e2y {
            let c = p1.ok_or(Unsupported::P1Needed)?;
            p1_stub(c, inp, pl, x, y, i, out)?;
            continue;
        }
        let dx = e2x - x;
        let dy = e2y - y;
        let (adx, ady) = (dx.abs(), dy.abs());
        let y_major = !(adx >= ady); // vcomiss |dx|,|dy|; jb (NaN included)
        let st = S5State {
            sample: i, x, y, p1x: l_p1x, p1y: l_p1y, p2x: l_p2x, p2y: l_p2y, e2x, e2y, dx, dy, adx, ady, y_major,
            data_ptr: inp.acc_ptr, width: p.img_w, height: p.img_h, stride: inp.acc_stride, scx, scy, k_a1_a0, k_a2_a1, k_1_a3,
            scale: inp.a[4], flag_cell_a: inp.flag_cell_a, mode_x: inp.mode_x, mode_y: inp.mode_y,
        };
        let mut sel = None;
        let v = if y_major { p4::y_major_sample(&st, &inp.a, hst, pl)? } else { x_major_sample_ex(&st, &inp.a, hst, pl, &mut sel)? };
        if let Some(XSel::General) = sel { LOG.with(|l| l.borrow_mut().push("G")); }
        if let Some(XSel::Empty) = sel { LOG.with(|l| l.borrow_mut().push("E")); }
        for k in 0..4 {
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

    fn base(flag: u32) -> (S5State, [f32; 5], Host, Plane) {
        let st = S5State {
            sample: 0, x: -0.5, y: 1.0, p1x: 0.0, p1y: 0.0, p2x: 0.0, p2y: 0.0, e2x: 1.5, e2y: 1.0, dx: 2.0, dy: 0.0, adx: 2.0, ady: 0.0, y_major: false,
            data_ptr: 0, width: 8, height: 8, stride: 40, scx: 1.0, scy: 1.0, k_a1_a0: 0.0, k_a2_a1: 0.0, k_1_a3: 0.5, scale: 1.0, flag_cell_a: flag, mode_x: 1, mode_y: 0,
        };
        // channel c of pixel (col, row) = c*100 + row*10 + col, 4 floats per pixel, stride 40 floats
        let mut buf = vec![0.0f32; 1000];
        for row in 0..12i64 {
            for col in 0..10i64 {
                for ch in 0..4i64 {
                    buf[(200 + row * 40 + col * 4 + ch) as usize] = (ch * 100 + row * 10 + col) as f32;
                }
            }
        }
        (st, [1.0, 1.0, 1.0, 0.5, 1.0], Host { port_blk_a: 1, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 8.0, ws: 8.0 }, Plane { buf: std::sync::Arc::new(buf), origin: 200 })
    }

    #[test]
    fn general_loop_hand() {
        // x=-0.5 e2x=1.5: c=-1, n=4, fx=0.5, c0=0.5; mode_x=1 and c<0 so dl=1. weights .5,1,1,.5; k0 wraps c=-1 -> 0 (port_blk_a).
        // columns 0..3 of row 1 (frac 0): (0.5*10+11+12+0.5*13)/3 = 11.5 plus 100*ch
        let (s, a, h, p) = base(1);
        assert_eq!(x_selector(&s, &a, &h), Ok(XSel::General));
        let v = x_major_sample(&s, &a, &h, &p).unwrap();
        for ch in 0..4 {
            assert!((v[ch] - (11.5 + 100.0 * ch as f32)).abs() < 1e-3, "{:?}", v);
        }
    }

    #[test]
    fn selector_cases() {
        let (mut s, a, h, _) = base(1);
        s.x = 1.0;
        s.dx = 0.5;
        s.e2x = 1.5;
        assert_eq!(x_selector(&s, &a, &h), Ok(XSel::Specialised));
        s.mode_x = 0;
        s.mode_y = 0;
        assert_eq!(x_selector(&s, &a, &h), Ok(XSel::Specialised)); // cond_any false: dl = 0
        let (s0, a, h, _) = base(0);
        assert_eq!(x_selector(&s0, &a, &h), Ok(XSel::FlagZero)); // BW7C: flag_cell_a == 0 is ported
        let (mut s, a, h, p) = base(1);
        s.e2x = s.x; // n = 1 would still run; make n <= 0: dx>0 but esi < c
        s.dx = 1.0;
        s.e2x = -3.5;
        let _ = x_selector(&s, &a, &h);
        let _ = p;
    }

    #[test]
    fn determinism_finite_edges() {
        for (port_blk_a, cassumption_a, cassumption_b, cassumption_c) in [(0, 0, 0, 0), (1, 1, 1, 1), (0, 1, 0, 1), (1, 0, 1, 0)] {
            for (mx, my) in [(1, 0), (2, 1), (1, 2), (0, 2)] {
                let (mut s, a, mut h, p) = base(1);
                h.port_blk_a = port_blk_a;
                h.cassumption_a = cassumption_a;
                h.cassumption_b = cassumption_b;
                h.cassumption_c = cassumption_c;
                s.mode_x = mx;
                s.mode_y = my;
                s.e2x = 9.5;
                s.dx = 10.0;
                let r1 = x_major_sample(&s, &a, &h, &p);
                let r2 = x_major_sample(&s, &a, &h, &p);
                assert_eq!(r1.as_ref().map(|v| v.map(f32::to_bits)), r2.as_ref().map(|v| v.map(f32::to_bits)));
                if let Ok(v) = r1 {
                    assert!(v.iter().all(|f| f.is_finite()), "{:?}", v);
                }
            }
        }
        let (mut s, a, h, p) = base(1);
        s.y = f32::NAN;
        let _ = x_major_sample(&s, &a, &h, &p);
        s.x = f32::INFINITY;
        let _ = x_major_sample(&s, &a, &h, &p);
    }

    #[test]
    fn empty_n_goes_to_finalizer() {
        let (mut s, a, h, p) = base(1);
        s.x = 3.0;
        s.e2x = 0.5;
        s.dx = 1.0; // forced dx>0 branch with esi < c: n <= 0
        let r = x_major_sample(&s, &a, &h, &p).unwrap();
        assert!(r.iter().all(|f| f.is_nan()), "{:?}", r); // 0 * (a4/0) = NaN
    }

    #[test]
    fn zero_length_without_p1_is_reported() {
        let p = Params { count: 1, aspect_block_d: 2.0, assumption_e: 7.0, assumption_c: 0, assumption_d: 0, assumption_f: 0.0, assumption_g: 10.0, assumption_h: 0.0, img_w: 8, img_h: 6 };
        let inp = Inputs {
            a: [0.0, 1.0, 3.0, 0.5, 2.0], p1x: vec![0.25], p1y: vec![3.0], p2x: vec![0.25], p2y: vec![3.0],
            p1_const: true, p2_const: true, flag_cell_a: 0, mode_x: 0, mode_y: 0, acc_ptr: 0, acc_stride: 40,
        };
        let h = Host { port_blk_a: 0, cassumption_a: 0, cassumption_b: 0, cassumption_c: 0, hs: 6.0, ws: 8.0 };
        let (_, _, _, pl) = base(0);
        let mut out = vec![vec![0.0; 1]; 4];
        assert_eq!(run_planes_full(&p, &inp, &h, &pl, None, &mut out), Err(Unsupported::P1Needed));
    }
}
