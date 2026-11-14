//! Compare driver of the optical flow fetch, resample and plane row kernels (`fx_device::kernels_flow_fetch`) against the crate
//! functions: every output word equal (NaN payloads included) over a corpus of widths, frame sizes, plane phases and adversarial
//! words, a stub that always takes the scalar derivative arm, and single edit mutants of the kernel source that each must be refused.
//! Prints NOT RUN without an OpenCL device. Run with --test-threads=1 (platform discovery is not thread safe).

use fx_device::selfcheck::adversarial_f32;
use fx_device::{Backend as Handle, DevImage, DeviceKind, Session};
use fx_exec::device_ops_flow_fetch as ff;
use std::sync::Arc;

fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}

fn device() -> Option<Arc<fx_device::DeviceCtx>> {
    match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => Some(d),
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            None
        }
    }
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

const SPECIAL: [u32; 12] = [0x7fc0_0000, 0xffc0_0000, 0x7fa0_0001, 0xffa0_0001, 0x7fc0_1234, 0x8000_0000, 0x0000_0001, 0x8000_0001, 0x7f80_0000, 0xff80_0000, 0x007f_ffff, 0x0000_0000];

/// One value: kind 0 a finite value in [-range, range], kind 1 the adversarial pool, kind 2 mostly kind 0 with about 1 in 12 special.
fn gen(kind: usize, r: &mut Lcg, pool: &[f32], range: f32) -> f32 {
    let fin = |r: &mut Lcg| ((r.next() % 8001) as f32 - 4000.0) / 4000.0 * range;
    match kind {
        0 => fin(r),
        1 => pool[(r.next() as usize) % pool.len()],
        _ => match r.next() % 12 {
            0 => f32::from_bits(SPECIAL[(r.next() as usize) % SPECIAL.len()]),
            1 => pool[(r.next() as usize) % pool.len()],
            _ => fin(r),
        },
    }
}

fn plane(n: usize, kind: usize, r: &mut Lcg, pool: &[f32], range: f32) -> Vec<f32> {
    (0..n).map(|_| gen(kind, r, pool, range)).collect()
}

const SENT: u32 = 0x7fc0_1234;
fn sent(n: usize) -> Vec<f32> {
    vec![f32::from_bits(SENT); n]
}
fn words(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}
fn up(s: &Arc<Session>, v: &[f32]) -> DevImage {
    DevImage::upload(s, &[v]).unwrap()
}
fn down(i: &DevImage) -> Vec<u32> {
    i.download().unwrap()[0].iter().map(|x| x.to_bits()).collect()
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    equal_words: usize,
    nan_cases: usize,
    mismatches: usize,
    first: Option<String>,
}

impl Tally {
    fn check(&mut self, what: String, want: Vec<u32>, got: Result<Vec<u32>, String>) -> bool {
        self.cases += 1;
        let ok = matches!(&got, Ok(g) if *g == want);
        if ok {
            self.equal_words += want.len();
            if want.iter().any(|w| f32::from_bits(*w).is_nan() && *w != SENT) {
                self.nan_cases += 1;
            }
        } else {
            self.mismatches += 1;
            if self.first.is_none() {
                self.first = Some(format!("{what}: {}", if got.is_err() { format!("{:?}", got.err()) } else { "words differ".into() }));
            }
        }
        ok
    }
}

const PHASES: [u32; 11] = [0, 4, 8, 12, 16, 20, 24, 28, 1, 2, 3];
const SIZES: [(i32, i32); 11] = [(2, 2), (3, 3), (7, 5), (8, 8), (9, 7), (15, 16), (16, 15), (17, 9), (33, 5), (50, 7), (64, 32)];

fn cst(r: &mut Lcg, pool: &[f32], i: usize) -> f32 {
    match i % 4 {
        0 => gen(0, r, pool, 2.0),
        1 => gen(0, r, pool, 0.2),
        2 => f32::from_bits(SPECIAL[(r.next() as usize) % SPECIAL.len()]),
        _ => gen(1, r, pool, 1.0),
    }
}

fn deriv_x(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(200, 11);
    let mut t = Tally::default();
    let mut idx = 0u64;
    for w in [1i32, 2, 3, 7, 8, 9, 15, 16, 17, 33, 50, 64, 1199, 1200, 1201, 1207, 1300] {
        for &ph in &PHASES {
            for kind in 0..3usize {
                idx += 1;
                let mut r = Lcg(idx * 7919 + 1);
                let stride = w + (w % 3);
                let rows = 3;
                let n = (stride * rows) as usize + 5;
                let inp = plane(n, kind, &mut r, &pool, 3.0);
                let cs: Vec<f32> = (0..4).map(|i| cst(&mut r, &pool, i + (idx % 2) as usize * 0)).collect();
                let c = optical_flow_solve::StencilConsts { k_s: cs[0], k_v: cs[1], c_s: cs[2], c_v: cs[3] };
                let row0 = (idx % 2) as i32;
                let rcount = rows - row0;
                let mut out = sent(n);
                optical_flow_solve::deriv_x(&inp, &mut out, w, stride, row0, rcount, ph, &c, None);
                let b = ff::StencilBits { k_s: cs[0].to_bits(), c_s: cs[2].to_bits(), k_v: cs[1].to_bits(), c_v: cs[3].to_bits() };
                let (a, o) = (up(s, &inp), up(s, &sent(n)));
                let got = ff::deriv_x_launch(s, a.plane(0), o.plane(0), w, stride, row0, rcount, ph, &b).map(|_| down(&o)).map_err(|e| e.to_string());
                if !t.check(format!("deriv_x w {w} ph {ph} kind {kind}"), words(&out), got) && stop {
                    return t;
                }
            }
        }
    }
    t
}

fn deriv_y(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(200, 12);
    let mut t = Tally::default();
    let mut idx = 0u64;
    for h in [1i32, 2, 3, 7, 8, 9, 15, 16, 17, 33, 50, 64, 100] {
        for (col0, cols) in [(0i32, 5i32), (1, 3)] {
            for kind in 0..3usize {
                for set in 0..3usize {
                    idx += 1;
                    let mut r = Lcg(idx * 104729 + 3);
                    let stride = 7;
                    let n = (stride * h) as usize + 4;
                    let inp = plane(n, kind, &mut r, &pool, 3.0);
                    let cs: Vec<f32> = (0..4).map(|i| cst(&mut r, &pool, i + set)).collect();
                    let c = optical_flow_solve::StencilConsts { k_s: cs[0], k_v: cs[1], c_s: cs[2], c_v: cs[3] };
                    let mut out = sent(n);
                    optical_flow_solve::deriv_y(&inp, &mut out, h, stride, col0, cols, &c, None);
                    let b = ff::StencilBits { k_s: cs[0].to_bits(), c_s: cs[2].to_bits(), k_v: cs[1].to_bits(), c_v: cs[3].to_bits() };
                    let (a, o) = (up(s, &inp), up(s, &sent(n)));
                    let got = ff::deriv_y_launch(s, a.plane(0), o.plane(0), h, stride, col0, cols, &b).map(|_| down(&o)).map_err(|e| e.to_string());
                    if !t.check(format!("deriv_y h {h} cols {col0}+{cols} kind {kind}"), words(&out), got) && stop {
                        return t;
                    }
                }
            }
        }
    }
    t
}

/// Flow values: kind 0 spans the frame plus a margin; the edge words (the lower bound, width minus margin) appear as values.
fn flow(n: usize, kind: usize, r: &mut Lcg, pool: &[f32], extent: f32, edge: &[f32]) -> Vec<f32> {
    (0..n)
        .map(|_| if r.next() % 10 == 0 { edge[(r.next() as usize) % edge.len()] } else { gen(kind, r, pool, extent + 3.0) })
        .collect()
}

fn warp_fetch(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(200, 13);
    let mut t = Tally::default();
    let mut idx = 0u64;
    let lo0 = f32::from_bits(optical_flow_solve::solve::WARP_LO_BITS);
    let mg0 = f32::from_bits(optical_flow_solve::solve::MARGIN_BITS);
    let sets = [(lo0, mg0), (0.0, 1.0), (lo0, f32::NAN), (-0.5, 0.0)];
    for &(w, h) in &SIZES {
        for extra in [0i32, 2] {
            for kind in 0..3usize {
                for (si, &(lo, mg)) in sets.iter().enumerate() {
                    idx += 1;
                    let mut r = Lcg(idx * 6151 + 5);
                    let stride = w + extra;
                    let n = (stride * h) as usize + 3;
                    let edge = [lo, (w as f32) - mg, (h as f32) - mg, 0.0, -0.0, w as f32];
                    let dx = flow(n, kind, &mut r, &pool, w as f32, &edge);
                    let dy = flow(n, kind, &mut r, &pool, h as f32, &edge);
                    let src = plane(n, (kind + si) % 3, &mut r, &pool, 5.0);
                    let (row0, rows) = if idx % 3 == 0 && h > 2 { (1, h - 1) } else { (0, h) };
                    let mut out = sent(n);
                    optical_flow_solve::warp_fetch(&dx, &dy, &src, &mut out, w, h, stride, row0, rows, &optical_flow_solve::FetchConsts { lo, margin: mg });
                    let (a, b, c, o) = (up(s, &dx), up(s, &dy), up(s, &src), up(s, &sent(n)));
                    let got = ff::warp_fetch_launch(s, a.plane(0), b.plane(0), c.plane(0), o.plane(0), w, h, stride, row0, rows, lo.to_bits(), mg.to_bits()).map(|_| down(&o)).map_err(|e| e.to_string());
                    if !t.check(format!("warp_fetch {w}x{h} stride {stride} kind {kind} set {si}"), words(&out), got) && stop {
                        return t;
                    }
                }
            }
        }
    }
    t
}

fn scale_copy(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(100, 14);
    let mut t = Tally::default();
    let ks = [0x3f80_0000u32, 0, 0x8000_0000, 0xbf80_0000, 0x4020_0000, 0x7fc0_0001, 0xffa0_0002, 0x7f80_0000, 0x0000_03e8, 0x3f80_0001];
    let mut idx = 0u64;
    for n in [1usize, 7, 8, 100] {
        for &k in &ks {
            for kind in 0..3usize {
                idx += 1;
                let mut r = Lcg(idx * 31 + 9);
                let src = plane(n, kind, &mut r, &pool, 4.0);
                let mut dst = sent(n);
                optical_flow_entry::scale_copy_model(&mut dst, &src, f32::from_bits(k));
                let (a, o) = (up(s, &src), up(s, &sent(n)));
                let got = ff::scale_copy_launch(s, a.plane(0), o.plane(0), k, n).map(|_| down(&o)).map_err(|e| e.to_string());
                if !t.check(format!("scale_copy n {n} k {k:08x} kind {kind}"), words(&dst), got) && stop {
                    return t;
                }
            }
        }
    }
    t
}

const WIDTHS_ROW: [i32; 11] = [1, 7, 15, 16, 17, 28, 29, 30, 31, 64, 100];

fn affine(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(150, 15);
    let mut t = Tally::default();
    let mut idx = 0u64;
    for w in WIDTHS_ROW {
        for kind in 0..3usize {
            for set in 0..4usize {
                idx += 1;
                let mut r = Lcg(idx * 977 + 7);
                let stride = w + 1;
                let rows = 3;
                let n = (stride * rows) as usize + 2;
                let mut data = plane(n, kind, &mut r, &pool, 4.0);
                let (sc, off) = (cst(&mut r, &pool, set), cst(&mut r, &pool, set + 1));
                let a = up(s, &data);
                let (row0, rc) = if idx % 2 == 0 { (1, 2) } else { (0, 3) };
                optical_flow::affine_rows(&mut data, w, stride, row0, rc, sc, off, (idx % 8) as usize * 4);
                let got = ff::affine_rows_launch(s, a.plane(0), w, stride, row0, rc, sc.to_bits(), off.to_bits()).map(|_| down(&a)).map_err(|e| e.to_string());
                if !t.check(format!("affine w {w} kind {kind} set {set}"), words(&data), got) && stop {
                    return t;
                }
            }
        }
    }
    t
}

fn sub_scaled(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(150, 16);
    let mut t = Tally::default();
    let mut idx = 0u64;
    for w in WIDTHS_ROW {
        for kind in 0..3usize {
            for set in 0..4usize {
                idx += 1;
                let mut r = Lcg(idx * 1013 + 11);
                let stride = w + 1;
                let n = (stride * 3) as usize + 2;
                let a = plane(n, kind, &mut r, &pool, 4.0);
                let b = plane(n, (kind + 1) % 3, &mut r, &pool, 4.0);
                let sc = cst(&mut r, &pool, set);
                let mut out = sent(n);
                optical_flow::sub_scaled_rows(&a, &b, &mut out, w, stride, 0, 3, sc, (idx % 8) as usize * 4);
                let (da, db, o) = (up(s, &a), up(s, &b), up(s, &sent(n)));
                let got = ff::sub_scaled_rows_launch(s, da.plane(0), db.plane(0), o.plane(0), w, stride, 0, 3, sc.to_bits()).map(|_| down(&o)).map_err(|e| e.to_string());
                if !t.check(format!("sub_scaled w {w} kind {kind} set {set}"), words(&out), got) && stop {
                    return t;
                }
            }
        }
    }
    t
}

fn stencil(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(150, 17);
    let mut t = Tally::default();
    let mut idx = 0u64;
    let aligns = [0u32, 4, 8, 12, 16, 20, 24, 28, 32, 36, 60, 2];
    for w in [1i32, 2, 3, 16, 17, 18, 31, 32, 33, 50, 64, 1200, 1201, 1300] {
        for hh in [1i32, 2, 3, 5] {
            if w >= 1200 && hh != 3 {
                continue;
            }
            for &al in &aligns {
                for kind in 0..3usize {
                    idx += 1;
                    if w >= 1200 && idx % 2 == 1 {
                        continue;
                    }
                    let mut r = Lcg(idx * 1117 + 13);
                    let pitch = w + (idx % 2) as i32 * 3;
                    let n = (pitch * hh) as usize + 2;
                    let b = plane(n, kind, &mut r, &pool, 4.0);
                    let c = plane(n, (kind + 1) % 3, &mut r, &pool, 4.0);
                    let d = plane(n, (kind + 2) % 3, &mut r, &pool, 4.0);
                    let sc = cst(&mut r, &pool, idx as usize);
                    let (row0, cnt) = if hh >= 3 && idx % 3 == 0 { (1, hh - 1) } else { (0, hh) };
                    let mut out = sent(n);
                    optical_flow::flow_stencil_rows(&mut out, &b, &c, &d, w, hh, pitch, sc, row0, cnt, al as usize);
                    let (db, dc, dd, o) = (up(s, &b), up(s, &c), up(s, &d), up(s, &sent(n)));
                    let got = ff::flow_stencil_rows_launch(s, o.plane(0), db.plane(0), dc.plane(0), dd.plane(0), w, hh, pitch, sc.to_bits(), row0, cnt, al).map(|_| down(&o)).map_err(|e| e.to_string());
                    if !t.check(format!("stencil w {w} rows {hh} align {al} kind {kind}"), words(&out), got) && stop {
                        return t;
                    }
                }
            }
        }
    }
    t
}

fn sum_rows(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(150, 18);
    let mut t = Tally::default();
    let mut idx = 0u64;
    for w in [1i32, 2, 63, 64, 65, 100, 127, 128, 129, 191, 192, 200, 300] {
        for &al in &PHASES {
            for kind in 0..3usize {
                for extra in [0i32, 5] {
                    idx += 1;
                    let mut r = Lcg(idx * 1223 + 17);
                    let stride = w + extra;
                    let n = (stride * 3) as usize + 3;
                    let data = plane(n, kind, &mut r, &pool, 4.0);
                    let (row0, rows) = if idx % 3 == 0 { (1, 2) } else { (0, 3) };
                    let want = optical_flow::sum_rows(&data, w, stride, row0, rows, al as usize);
                    let d = up(s, &data);
                    let got = ff::sum_rows_value(s, d.plane(0), w, stride, row0, rows, al).map(|v| vec![v.to_bits()]).map_err(|e| e.to_string());
                    if !t.check(format!("sum_rows w {w} align {al} kind {kind}"), vec![want.to_bits()], got) && stop {
                        return t;
                    }
                }
            }
        }
    }
    t
}

fn scan_range(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(150, 19);
    let mut t = Tally::default();
    let mut idx = 0u64;
    for (w, h, pitch) in [(1i32, 1i32, 1i32), (2, 2, 2), (3, 3, 4), (7, 5, 8), (8, 8, 8), (9, 7, 12), (16, 4, 16), (17, 3, 20), (50, 7, 50), (64, 32, 64)] {
        for kind in 0..3usize {
            for rep in 0..4 {
                idx += 1;
                let mut r = Lcg(idx * 1319 + 19 + rep);
                let n = (pitch * h) as usize + 1;
                let data = plane(n, kind, &mut r, &pool, 4.0);
                let (mx, mn) = optical_flow::scan_range(&data, w, h, pitch);
                let d = up(s, &data);
                let o = DevImage::alloc(s, 1, 2).unwrap();
                let got = ff::scan_range_launch(s, d.plane(0), o.plane(0), w, h, pitch).map(|_| down(&o)).map_err(|e| e.to_string());
                if !t.check(format!("scan_range {w}x{h} pitch {pitch} kind {kind}"), vec![mx.to_bits(), mn.to_bits()], got) && stop {
                    return t;
                }
            }
        }
    }
    t
}

fn final_warp(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(200, 20);
    let mut t = Tally::default();
    let mut idx = 0u64;
    let eps = optical_flow_entry::EDGE_EPS;
    let mg = optical_flow_entry::EDGE_MARGIN;
    for &(w, h) in &SIZES {
        for extra in [0i32, 2] {
            for kind in 0..3usize {
                for si in 0..3usize {
                    idx += 1;
                    let mut r = Lcg(idx * 1427 + 23);
                    let stride = w + extra;
                    let n = (stride * h) as usize + 2;
                    let edge = [eps, (w as f32) - mg, (h as f32) - mg, 0.0, -0.0];
                    let u = flow(n, kind, &mut r, &pool, w as f32, &edge);
                    let v = flow(n, kind, &mut r, &pool, h as f32, &edge);
                    let target = plane(n, kind, &mut r, &pool, 2.0);
                    let img = plane(n, (kind + 1) % 3, &mut r, &pool, 2.0);
                    let g: Vec<Vec<f32>> = (0..4).map(|k| plane(n, (kind + k) % 3, &mut r, &pool, 2.0)).collect();
                    let scale = [1.0f32, 0.5, f32::from_bits(0x7fc0_0007)][si];
                    let (rs, rc) = if idx % 3 == 0 && h > 2 { (1, h - 1) } else { (0, h) };
                    let planes = optical_flow_entry::WarpPlanes {
                        width: w, height: h, stride, target_plane: &target, img: &img, u: &u, v: &v, grad_a_x: &g[0], grad_a_y: &g[1], grad_b_x: &g[2], grad_b_y: &g[3], scale,
                    };
                    let mut out = sent(n);
                    optical_flow_entry::final_warp_rows(&planes, &mut out, rs, rc);
                    let ups: Vec<DevImage> = [&target, &img, &u, &v, &g[0], &g[1], &g[2], &g[3]].iter().map(|p| up(s, p)).collect();
                    let o = up(s, &sent(n));
                    let fp = ff::FinalWarpPlanes { target: ups[0].plane(0), img: ups[1].plane(0), u: ups[2].plane(0), v: ups[3].plane(0), grad_a_x: ups[4].plane(0), grad_a_y: ups[5].plane(0), grad_b_x: ups[6].plane(0), grad_b_y: ups[7].plane(0) };
                    let got = ff::final_warp_launch(s, &fp, o.plane(0), w, h, stride, rs, rc, scale.to_bits()).map(|_| down(&o)).map_err(|e| e.to_string());
                    if !t.check(format!("final_warp {w}x{h} stride {stride} kind {kind} set {si}"), words(&out), got) && stop {
                        return t;
                    }
                }
            }
        }
    }
    t
}

fn pairs() -> Vec<(i32, i32)> {
    let a = [2i32, 3, 7, 8, 9, 15, 16, 17, 33, 50, 64];
    let b = [2i32, 3, 7, 8, 16, 33, 50, 64];
    let mut v = Vec::new();
    for &x in &a {
        for &y in &b {
            v.push((x, y));
        }
    }
    v.extend([(10, 9), (11, 7), (13, 10), (23, 17), (37, 29), (100, 83), (1500, 4), (3000, 6), (2400, 2), (1400, 3), (1, 4), (5, 1)]);
    v
}

fn resample_h(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(150, 21);
    let mut t = Tally::default();
    let mut idx = 0u64;
    for (w, wd) in pairs() {
        let big = w >= 1000;
        let phs: &[u32] = if big { &PHASES } else { &[0, 12] };
        for &mis in phs {
            for kind in 0..3usize {
                idx += 1;
                if !big && kind == 1 && idx % 2 == 0 {
                    continue;
                }
                let mut r = Lcg(idx * 1523 + 29);
                let h = if big { 2 } else { 3 };
                let src = plane((w * h) as usize + 3, kind, &mut r, &pool, 4.0);
                let (rs, rc) = if idx % 2 == 0 { (1, h - 1) } else { (0, h) };
                let mut dst = sent((wd * h) as usize);
                optical_flow_resample::resample_h(&src, w, h, wd, &mut dst, mis as usize, rs, rc);
                let (a, o) = (up(s, &src), up(s, &sent((wd * h) as usize)));
                let got = ff::resample_h_launch(s, a.plane(0), o.plane(0), w, wd, rs, rc, mis).map(|_| down(&o)).map_err(|e| e.to_string());
                if !t.check(format!("resample_h {w}->{wd} mis {mis} kind {kind}"), words(&dst), got) && stop {
                    return t;
                }
            }
        }
    }
    t
}

fn resample_v(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(150, 22);
    let mut t = Tally::default();
    let mut idx = 0u64;
    for (hs, hd) in pairs() {
        for kind in 0..3usize {
            for (cs, cc) in [(0i32, 5i32), (1, 3)] {
                idx += 1;
                let mut r = Lcg(idx * 1627 + 31);
                let stride = 6;
                let src = plane((hs * stride) as usize + 3, kind, &mut r, &pool, 4.0);
                let mut dst = sent((hd * stride) as usize);
                optical_flow_resample::resample_v(&src, stride, hs, hd, &mut dst, cs, cc);
                let (a, o) = (up(s, &src), up(s, &sent((hd * stride) as usize)));
                let got = ff::resample_v_launch(s, a.plane(0), o.plane(0), stride, hs, hd, cs, cc).map(|_| down(&o)).map_err(|e| e.to_string());
                if !t.check(format!("resample_v {hs}->{hd} cols {cs}+{cc} kind {kind}"), words(&dst), got) && stop {
                    return t;
                }
            }
        }
    }
    t
}

type Runner = fn(&Arc<Session>, bool) -> Tally;
const KERNELS: &[(&str, Runner)] = &[
    ("deriv_x", deriv_x),
    ("deriv_y", deriv_y),
    ("warp_fetch", warp_fetch),
    ("scale_copy", scale_copy),
    ("affine_rows", affine),
    ("sub_scaled_rows", sub_scaled),
    ("flow_stencil_rows", stencil),
    ("sum_rows", sum_rows),
    ("scan_range", scan_range),
    ("final_warp", final_warp),
    ("resample_h", resample_h),
    ("resample_v", resample_v),
];

fn runner(name: &str) -> Runner {
    KERNELS.iter().find(|k| k.0 == name).unwrap().1
}

#[test]
fn every_flow_fetch_kernel_equals_its_crate_function_bit_for_bit() {
    let Some(ctx) = device() else { return };
    let mut bad = Vec::new();
    for (name, f) in KERNELS {
        let t = f(&ctx.session, false);
        println!("KERNEL {name} cases {} equal_words {} nan_cases {} mismatches {} first {:?}", t.cases, t.equal_words, t.nan_cases, t.mismatches, t.first);
        if t.mismatches > 0 {
            bad.push(*name);
        }
    }
    assert!(bad.is_empty(), "kernels with mismatches: {bad:?}");
}

/// (kernel, name, text in the program, replacement); each text occurs once.
const MUTANTS: &[(&str, &str, &str, &str)] = &[
    ("deriv_x", "stub: always the scalar arm", "int vec = (all == 0) && ((long)i >= head) && ((long)i < vend);\n    float r;", "int vec = 0;\n    float r;"),
    ("deriv_x", "vector operand roles swapped", "float t = flow_b_m231(inp[(long)roff + p2], inp[(long)roff + p1], cv);", "float t = flow_b_m231(inp[(long)roff + p1], inp[(long)roff + p2], cv);"),
    ("deriv_y", "vector row bound off by one", "(y < (h & ~7))", "(y <= (h & ~7))"),
    ("warp_fetch", "lower bound compare direction", "int a = lo >= px;", "int a = lo > px;"),
    ("warp_fetch", "fraction operand order", "float fx = nanx_sub(px, (float)ix0);", "float fx = nanx_sub((float)ix0, px);"),
    ("scale_copy", "copy arm multiplies", "if (k == 0x1p+0f) {\n        dst[i] = src[i];", "if (k == 0x1p+0f) {\n        dst[i] = as_uint(nanx_mul(as_float(src[i]), k));"),
    ("affine_rows", "contracted multiply add", "data[idx] = flow_b_m213(nanx_sub(data[idx], as_float(offset_b)), as_float(scale_b), 0x1p+0f);", "data[idx] = nanx_sub(data[idx], as_float(offset_b)) * as_float(scale_b) - 0x1p+0f;"),
    ("sub_scaled_rows", "unfused multiply subtract", "out[idx] = flow_b_n213(a[idx], as_float(s_b), b[idx]);", "out[idx] = b[idx] - a[idx] * as_float(s_b);"),
    ("flow_stencil_rows", "last row index", "((row == (int)((uint)rows - 1u)) ? 2 : 1)", "((row == (int)((uint)rows - 2u)) ? 2 : 1)"),
    ("flow_stencil_rows", "vector start compare", "int vec = (i >= head) && (i < vend);", "int vec = (i > head) && (i < vend);"),
    ("sum_rows", "block end mask", "long end = w - ((w - head) & 0x3f);", "long end = w - ((w - head) & 0x1f);"),
    ("sum_rows", "half sum operands", "float pair_even = nanx_add(x0, x2);", "float pair_even = nanx_add(x0, x1);"),
    ("scan_range", "tail compare", "if ((uint)w > 2u * (uint)pw2) {", "if ((uint)w >= 2u * (uint)pw2) {"),
    ("final_warp", "outside compare direction", "int outside = (eps >= sx)", "int outside = (eps > sx)"),
    ("final_warp", "magnitude sum operands", "float sum = nanx_add(mag_a, mag_b);", "float sum = nanx_add(mag_a, mag_a);"),
    ("resample_h", "peel shift", "peel = (32 - addr) >> 2;", "peel = (32 - addr) >> 1;"),
    ("resample_h", "lane ramp base", "base_t = flow_b_f231(base_t, (float)peel, scale);", "base_t = flow_b_f231(base_t, (float)(peel + 1), scale);"),
    ("resample_v", "vector end mask", "vec_end = count & ~7;", "vec_end = count & ~15;"),
    ("resample_h", "scale down far polynomial contracted", "return flow_b_f213(x, t, 0x1p+3f);", "return x * t + 0x1p+3f;"),
    ("resample_h", "lane kernel constant", "return flow_b_f231(0x1p+2f, y, t);", "return flow_b_f231(0x1.8p+2f, y, t);"),
];

#[test]
fn every_flow_fetch_mutant_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev_h, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let bin = |s: &Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(Session::open_source(&dev_h, &info, &base, opts).unwrap());
    let base_hash = bin(&clean);
    let mut killed = 0usize;
    for (kernel, name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(Session::open_source(&dev_h, &info, &text, opts).unwrap());
        let h = bin(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        let t = runner(kernel)(&s, true);
        let verdict = if t.mismatches > 0 { "KILLED" } else { "SURVIVED" };
        if t.mismatches > 0 {
            killed += 1;
        }
        println!("MUTANT {kernel}: {name} binary {h:016x}: {verdict} after {} cases", t.cases);
    }
    println!("MUTANTS killed {killed} of {}", MUTANTS.len());
    assert_eq!(killed, MUTANTS.len(), "a flow fetch mutant survived the compare");
}
