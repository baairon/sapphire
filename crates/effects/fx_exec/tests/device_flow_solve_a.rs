//! The optical flow solve tile kernels, first group, on the device against the crate functions (`optical_flow_solve::fma_mix`,
//! `flow_update`, `diff_sum`, `relax::relax_callback`, `smooth::scale_plane`, `reduce::zero_border` and the stretch's scale and
//! border passes), every output word compared (NaN payloads included), a stub that launches nothing which the compare must refuse,
//! and single-edit mutants of the kernel source, each of which the compare must refuse. Prints NOT RUN without an OpenCL device
//! (run with --test-threads=1: platform discovery is not thread safe).

use fx_device::cl::Buffer;
use fx_device::selfcheck::adversarial_f32;
use fx_device::{Backend as Handle, DevImage, DeviceKind, Session};
use fx_exec::device_ops_flow_solve_a as dv;
use optical_flow_pyramid::LevelRecord as Level;
use optical_flow_solve::stretch::{Direct, Host, PairPlane, Planes, Tile};
use optical_flow_solve::Shape;
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum K {
    Mix,
    Flow,
    Diff,
    Zero,
    Scale,
    Relax,
    Stretch,
}
const ALL: [K; 7] = [K::Mix, K::Flow, K::Diff, K::Zero, K::Scale, K::Relax, K::Stretch];

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

const SENTINEL: u32 = 0x7fc0_1234;

fn data(n: usize, kind: usize, seed: u64, pool: &[f32]) -> Vec<f32> {
    let mut r = Lcg(seed ^ 0x9e37_79b9_7f4a_7c15);
    (0..n)
        .map(|_| match kind {
            0 => {
                let m = (r.next() & 0x7f_ffff) as u32;
                let e = 120 + (r.next() % 14) as u32;
                let s = ((r.next() & 1) as u32) << 31;
                f32::from_bits(s | (e << 23) | m)
            }
            1 => pool[(r.next() as usize) % pool.len()],
            3 => {
                // NaN dense: quiet and signalling payloads of both signs
                let pl = (r.next() & 0x3f_ffff) as u32 | 1;
                let q = ((r.next() & 1) as u32) << 22;
                let sg = ((r.next() & 1) as u32) << 31;
                if r.next() % 4 == 0 { f32::from_bits(sg | 0x3f80_0000 | (r.next() as u32 & 0x7f_ffff)) } else { f32::from_bits(sg | 0x7f80_0000 | q | pl) }
            }
            _ => {
                if r.next() % 12 == 0 {
                    pool[(r.next() as usize) % pool.len()]
                } else {
                    let m = (r.next() & 0x7f_ffff) as u32;
                    let e = 122 + (r.next() % 8) as u32;
                    let s = ((r.next() & 1) as u32) << 31;
                    f32::from_bits(s | (e << 23) | m)
                }
            }
        })
        .collect()
}

fn words(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

#[derive(Clone, Debug)]
struct Case {
    k: K,
    w: usize,
    h: usize,
    stride: usize,
    row0: usize,
    rows: usize,
    phase: u32,
    kind: usize,
    seed: u64,
    has_v: bool,
    p0: u32,
    p1: u32,
    p2: u32,
}

const SPECIAL_SCALARS: [u32; 12] = [
    0x3f80_0000, 0x3f00_0000, 0x4000_0000, 0x0000_0000, 0xbf80_0000, 0x6000_0000, 0x7fc0_0001, 0xffc0_0002, 0x0000_0003, 0x7f80_0000, 0x8000_0000, 0x3dcc_cccd,
];
const EPS_SET: [u32; 8] = [0, 0x3586_37bd, 0x3a83_126f, 0x3f80_0000, 0x7fc0_0000, 0x8000_0000, 0x7f80_0000, 0x0000_0001];

fn cases(k: K) -> Vec<Case> {
    let mut widths: Vec<usize> = vec![1, 2, 3, 7, 8, 9, 15, 16, 17, 28, 29, 30, 31, 33, 45, 50, 61, 64];
    let big: Vec<usize> = match k {
        K::Diff => vec![905, 906, 907, 908, 915, 1000],
        K::Relax => vec![1199, 1200, 1201, 1202, 1217, 1260],
        _ => vec![],
    };
    let f32_phases: Vec<u32> = vec![0, 4, 8, 12, 16, 20, 24, 28, 1, 2, 3, 6];
    let nest_phases: Vec<u32> = (0..16).map(|i| i * 4).chain([1, 2, 6, 63]).collect();
    let phases = if k == K::Relax { nest_phases } else { f32_phases };
    widths.extend(&big);
    let pads = [0usize, 1, 3, 5];
    let mut out = Vec::new();
    let mut idx = 0usize;
    for &w in &widths {
        let is_big = big.contains(&w);
        for (pi, &ph) in phases.iter().enumerate() {
            if is_big && pi % 3 != 0 {
                continue;
            }
            for rep in 0..3usize {
                for has_v in [false, true] {
                    if k != K::Relax && has_v {
                        continue;
                    }
                    idx += 1;
                    let hs: &[usize] = if is_big { &[1, 2, 3] } else { &[1, 2, 3, 5, 7, 32] };
                    let h = hs[idx % hs.len()];
                    let row0 = (idx / 3) % h;
                    let rows = 1 + (idx / 5) % (h - row0);
                    let stride = w + pads[idx % 4];
                    out.push(Case {
                        k,
                        w,
                        h,
                        stride,
                        row0,
                        rows,
                        phase: ph,
                        kind: (idx + rep) % 3,
                        seed: idx as u64 * 7919 + 13,
                        has_v,
                        p0: SPECIAL_SCALARS[(idx / 2) % SPECIAL_SCALARS.len()],
                        p1: SPECIAL_SCALARS[(idx / 3 + 1) % SPECIAL_SCALARS.len()],
                        p2: EPS_SET[(idx / 7) % EPS_SET.len()],
                    });
                }
            }
        }
    }
    if k == K::Diff {
        // NaN heavy rows with an aligned head: both operands of the head accumulation are NaN
        for &w in &[907usize, 915, 1000] {
            for ph in [4u32, 8, 12, 16, 20] {
                for rep in 0..4u64 {
                    out.push(Case { k, w, h: 2, stride: w, row0: 0, rows: 1, phase: ph, kind: 4, seed: 90_000 + w as u64 * 31 + ph as u64 * 7 + rep, has_v: false, p0: 0, p1: 0, p2: 0 });
                }
            }
        }
    }
    out
}

type Pair = (Vec<u32>, Result<Vec<u32>, String>);

fn upload(s: &Arc<Session>, planes: &[&[f32]]) -> Result<DevImage, String> {
    DevImage::upload(s, planes).map_err(|e| e.to_string())
}

fn down(img: &DevImage, which: &[usize]) -> Result<Vec<u32>, String> {
    let d = img.download().map_err(|e| e.to_string())?;
    Ok(which.iter().flat_map(|&i| words(&d[i])).collect())
}

fn run(s: &Arc<Session>, c: &Case, pool: &[f32], stub: bool) -> Pair {
    let n = c.h * c.stride;
    let sentinel = vec![f32::from_bits(SENTINEL); n];
    let (w, h, st, r0, rows) = (c.w as i32, c.h as i32, c.stride as i32, c.row0 as i32, c.rows as i32);
    let d = |i: u64| {
        if c.kind == 4 {
            // NaN dense in the first three columns of a row only (inside the aligned head, rows 1, phase 4 to 20), finite beyond: the head accumulation is the only NaN source
            let (a, b) = (data(n, 3, c.seed + i * 101, pool), data(n, 0, c.seed + i * 101 + 5, pool));
            (0..n).map(|k| if k % c.stride < 3 { a[k] } else { b[k] }).collect::<Vec<f32>>()
        } else {
            data(n, c.kind, c.seed + i * 101, pool)
        }
    };
    match c.k {
        K::Mix => {
            let (a, b, cc, dd, e, f) = (d(0), d(1), d(2), d(3), d(4), d(5));
            let sh = Shape { width: w, stride: st, phase_bytes: c.phase };
            let mut out = sentinel.clone();
            optical_flow_solve::fma_mix(&optical_flow_solve::MixIn { a: &a, b: &b, c: &cc, d: &dd, e: &e, f: &f }, &mut out, &sh, r0, rows);
            let dev = (|| {
                let img = upload(s, &[&a, &b, &cc, &dd, &e, &f, &sentinel])?;
                if !stub {
                    let p = |i: usize| img.plane(i);
                    dv::fma_mix_dev(s, [p(0), p(1), p(2), p(3), p(4), p(5)], p(6), &sh, r0, rows).map_err(|e| e.to_string())?;
                }
                down(&img, &[6])
            })();
            (words(&out), dev)
        }
        K::Flow => {
            let (u, v, a, b, cc) = (d(0), d(1), d(2), d(3), d(4));
            let (rs, fg, eps) = (f32::from_bits(c.p0), f32::from_bits(c.p1), f32::from_bits(c.p2));
            let (mut o0, mut o1) = (sentinel.clone(), sentinel.clone());
            optical_flow_solve::flow_update(&optical_flow_solve::FlowIn { u: &u, v: &v, a: &a, b: &b, c: &cc }, &mut o0, &mut o1, w, st, r0, rows, rs, fg, eps);
            let dev = (|| {
                let img = upload(s, &[&u, &v, &a, &b, &cc, &sentinel, &sentinel])?;
                if !stub {
                    let fb = dv::FlowBufs { u: img.plane(0), v: img.plane(1), a: img.plane(2), b: img.plane(3), c: img.plane(4) };
                    dv::flow_update_dev(s, &fb, img.plane(5), img.plane(6), w, st, r0, rows, rs, fg, eps).map_err(|e| e.to_string())?;
                }
                down(&img, &[5, 6])
            })();
            (words(&o0).into_iter().chain(words(&o1)).collect(), dev)
        }
        K::Diff => {
            let (p, q, mut x, mut y) = (d(0), d(1), d(2), d(3));
            let (x0, y0) = (x.clone(), y.clone());
            let sh = Shape { width: w, stride: st, phase_bytes: c.phase };
            let sum = optical_flow_solve::diff_sum(&optical_flow_solve::DiffPlanes { p: &p, q: &q }, &mut x, &mut y, &sh, r0, rows);
            let mut want = words(&x);
            want.extend(words(&y));
            want.push(sum.to_bits());
            let dev = (|| {
                let img = upload(s, &[&p, &q, &x0, &y0])?;
                let mut sumw = 0u32;
                if !stub {
                    sumw = dv::diff_sum_dev(s, img.plane(0), img.plane(1), img.plane(2), img.plane(3), &sh, r0, rows).map_err(|e| e.to_string())?.to_bits();
                }
                let mut v = down(&img, &[2, 3])?;
                v.push(sumw);
                Ok(v)
            })();
            (want, dev)
        }
        K::Zero => {
            let mut p = d(0);
            let p0 = p.clone();
            optical_flow_solve::reduce::zero_border(&mut p, w, h, st, r0, rows as u32);
            let dev = (|| {
                let img = upload(s, &[&p0])?;
                if !stub {
                    dv::zero_border_dev(s, img.plane(0), w, h, st, r0, rows as u32).map_err(|e| e.to_string())?;
                }
                down(&img, &[0])
            })();
            (words(&p), dev)
        }
        K::Scale => {
            let mut p = d(0);
            let p0 = p.clone();
            let sc = f32::from_bits(c.p0);
            optical_flow_solve::smooth::scale_plane(&mut p, w, st, sc, r0, rows as u32);
            let dev = (|| {
                let img = upload(s, &[&p0])?;
                if !stub {
                    dv::scale_plane_dev(s, img.plane(0), w, st, sc, r0, rows as u32).map_err(|e| e.to_string())?;
                }
                down(&img, &[0])
            })();
            (words(&p), dev)
        }
        K::Relax => {
            let (t0, u0, a0, t1, u1, a1, v) = (d(0), d(1), d(2), d(3), d(4), d(5), d(6));
            let sc = f32::from_bits(c.p0);
            let ctx = optical_flow_solve::relax::Ctx {
                width: w,
                height: h,
                stride: st,
                scale: sc,
                v: if c.has_v { Some(&v) } else { None },
                nests: [
                    optical_flow_solve::relax::NestIn { t: &t0, u: &u0, acc: &a0 },
                    optical_flow_solve::relax::NestIn { t: &t1, u: &u1, acc: &a1 },
                ],
                phase_bytes: c.phase,
            };
            let (mut o0, mut o1) = (sentinel.clone(), sentinel.clone());
            optical_flow_solve::relax::relax_callback(&ctx, [&mut o0, &mut o1], r0, rows);
            let dev = (|| {
                let img = upload(s, &[&t0, &u0, &a0, &sentinel, &t1, &u1, &a1, &sentinel, &v])?;
                if !stub {
                    let p = |i: usize| img.plane(i);
                    let n0 = dv::RelaxNest { t: p(0), u: p(1), acc: p(2), out: p(3) };
                    let n1 = dv::RelaxNest { t: p(4), u: p(5), acc: p(6), out: p(7) };
                    let vb: Option<&Buffer<f32>> = if c.has_v { Some(p(8)) } else { None };
                    dv::relax_dev(s, w, h, st, sc, vb, [&n0, &n1], c.phase, r0, rows).map_err(|e| e.to_string())?;
                }
                down(&img, &[3, 7])
            })();
            (words(&o0).into_iter().chain(words(&o1)).collect(), dev)
        }
        K::Stretch => unreachable!(),
    }
}

// Stretch: the crate's run_stretch with a recording host that delegates the scalar passes to `Direct`.

#[derive(Debug, Clone, Copy, PartialEq)]
enum Call {
    Scale(PairPlane, u32),
    Border(PairPlane),
}

struct Mem {
    src: Vec<Vec<f32>>,
    dst: Vec<Vec<f32>>,
}

fn idx(j: PairPlane) -> usize {
    [PairPlane::FlowU, PairPlane::FlowV, PairPlane::Nest1T, PairPlane::Nest1U, PairPlane::Nest2T, PairPlane::Nest2U].iter().position(|x| *x == j).unwrap()
}

impl Planes for Mem {
    fn src(&self, j: PairPlane) -> (&[f32], u32) {
        (&self.src[idx(j)], 0)
    }
    fn src_mut(&mut self, j: PairPlane) -> &mut [f32] {
        &mut self.src[idx(j)]
    }
    fn dst(&mut self, j: PairPlane) -> &mut [f32] {
        &mut self.dst[idx(j)]
    }
}

struct Rec {
    log: Vec<Call>,
}

impl Host for Rec {
    fn helper(&mut self, _c: optical_flow_solve::stretch::HelperCall) {}
    fn tile_pair<P: Planes>(&mut self, _mem: &mut P, _t: &Tile) {}
    fn scale<P: Planes>(&mut self, mem: &mut P, j: PairPlane, l: &Level, ratio: f32) {
        self.log.push(Call::Scale(j, ratio.to_bits()));
        Direct.scale(mem, j, l, ratio);
    }
    fn border<P: Planes>(&mut self, mem: &mut P, j: PairPlane, lp: &Level) {
        self.log.push(Call::Border(j));
        Direct.border(mem, j, lp);
    }
}

fn level(w: i32, h: i32, stride: i32) -> Level {
    Level { width: w, height: h, stride, elem_bytes: 4, ..Default::default() }
}

fn stretch_pairs() -> Vec<(Level, Level)> {
    // (finer l, coarser lp)
    let dims = [(15, 9, 7, 5, 0), (33, 17, 16, 16, 2), (50, 7, 25, 4, 1), (64, 32, 32, 16, 3), (17, 5, 9, 3, 0), (3, 3, 2, 2, 1), (7, 7, 7, 7, 0)];
    dims.iter().map(|&(w, h, pw, ph, pad)| (level(w, h, w + pad), level(pw, ph, pw + pad + 1))).collect()
}

fn run_stretch(s: &Arc<Session>, pi: usize, kind: usize, pool: &[f32], stub: bool) -> (Pair, usize, bool) {
    let (l, lp) = stretch_pairs()[pi];
    let (dn, sn) = ((l.stride * l.height) as usize, (lp.stride * lp.height) as usize);
    let mk = |n: usize, off: u64| (0..6).map(|j| data(n, kind, 1000 * pi as u64 + off + j, pool)).collect::<Vec<_>>();
    let mut mem = Mem { src: mk(sn, 1), dst: mk(dn, 50) };
    let init_src = mem.src.clone();
    let init_dst = mem.dst.clone();
    let mut rec = Rec { log: Vec::new() };
    optical_flow_solve::stretch::run_stretch(&l, &lp, 0, 0, &mut mem, &mut rec);
    let want: Vec<u32> = mem.src.iter().chain(mem.dst.iter()).flat_map(|p| words(p)).collect();
    let has_nan = want.iter().any(|w| f32::from_bits(*w).is_nan());
    let ncalls = rec.log.len();
    let dev = (|| {
        let src_refs: Vec<&[f32]> = init_src.iter().map(|v| v.as_slice()).collect();
        let dst_refs: Vec<&[f32]> = init_dst.iter().map(|v| v.as_slice()).collect();
        let simg = upload(s, &src_refs)?;
        let dimg = upload(s, &dst_refs)?;
        if !stub {
            for call in &rec.log {
                match *call {
                    Call::Scale(j, bits) => {
                        if dv::stretch_ratio(j, &l, &lp).to_bits() != bits {
                            return Err(format!("ratio differs for {j:?}"));
                        }
                        dv::stretch_scale_dev(s, dimg.plane(idx(j)), j, &l, &lp).map_err(|e| e.to_string())?;
                    }
                    Call::Border(j) => dv::stretch_border_dev(s, simg.plane(idx(j)), &lp).map_err(|e| e.to_string())?,
                }
            }
        }
        let mut v = down(&simg, &[0, 1, 2, 3, 4, 5])?;
        v.extend(down(&dimg, &[0, 1, 2, 3, 4, 5])?);
        Ok(v)
    })();
    ((want, dev), ncalls, has_nan)
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    equal_words: usize,
    nan_cases_same: usize,
    mismatches: usize,
    first: Option<String>,
}

fn record(t: &mut Tally, label: String, want: &[u32], got: &Result<Vec<u32>, String>) -> bool {
    t.cases += 1;
    match got {
        Ok(g) if g.as_slice() == want => {
            t.equal_words += want.len();
            if want.iter().any(|w| f32::from_bits(*w).is_nan()) {
                t.nan_cases_same += 1;
            }
            true
        }
        other => {
            t.mismatches += 1;
            if t.first.is_none() {
                t.first = Some(format!("{label}: {}", match other {
                    Ok(_) => "words differ".to_string(),
                    Err(e) => e.clone(),
                }));
            }
            false
        }
    }
}

fn compare(s: &Arc<Session>, k: K, stub: bool, stop_at_first: bool) -> Tally {
    let pool = adversarial_f32(300, 77);
    let mut t = Tally::default();
    if k == K::Stretch {
        for pi in 0..stretch_pairs().len() {
            for kind in 0..3 {
                let ((want, got), ncalls, _) = run_stretch(s, pi, kind, &pool, stub);
                assert_eq!(ncalls, 6, "two scale calls and four border calls");
                if !record(&mut t, format!("stretch pair {pi} kind {kind}"), &want, &got) && stop_at_first {
                    return t;
                }
            }
        }
        return t;
    }
    for c in cases(k) {
        let (want, got) = run(s, &c, &pool, stub);
        if !record(&mut t, format!("{c:?}"), &want, &got) && stop_at_first {
            return t;
        }
    }
    t
}

#[test]
fn the_solve_tile_kernels_equal_the_crate_bit_for_bit() {
    let Some(ctx) = device() else { return };
    let mut bad = 0;
    for k in ALL {
        let t = compare(&ctx.session, k, false, false);
        println!("FLOW_SOLVE_A {k:?} cases {} equal_words {} nan_cases_same {} mismatches {}", t.cases, t.equal_words, t.nan_cases_same, t.mismatches);
        if t.mismatches > 0 {
            println!("first mismatch {:?}", t.first);
        }
        bad += t.mismatches;
    }
    assert_eq!(bad, 0);
}

#[test]
fn a_stub_that_launches_nothing_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    for k in ALL {
        let t = compare(&ctx.session, k, true, true);
        println!("STUB {k:?} mismatches {} first {:?}", t.mismatches, t.first);
        assert!(t.mismatches > 0, "the compare accepted a stub for {k:?}");
    }
}

#[test]
fn arguments_outside_the_model_are_refused_before_any_launch() {
    let Some(ctx) = device() else { return };
    let s = &ctx.session;
    let img = DevImage::upload(s, &[&[0.0f32; 64], &[0.0f32; 64], &[0.0f32; 64], &[0.0f32; 64], &[0.0f32; 64], &[0.0f32; 64], &[0.0f32; 64]]).unwrap();
    let p = |i: usize| img.plane(i);
    let bad = |sh: Shape, row0: i32, rows: i32| dv::fma_mix_dev(s, [p(0), p(1), p(2), p(3), p(4), p(5)], p(6), &sh, row0, rows);
    assert!(bad(Shape { width: 8, stride: 7, phase_bytes: 0 }, 0, 2).is_err(), "stride below width");
    assert!(bad(Shape { width: 0, stride: 8, phase_bytes: 0 }, 0, 2).is_err(), "width 0");
    assert!(bad(Shape { width: 8, stride: 8, phase_bytes: 0 }, -1, 2).is_err(), "negative first row");
    assert!(bad(Shape { width: 8, stride: 8, phase_bytes: 0 }, 0, 9).is_err(), "rows outside the buffer");
    assert!(bad(Shape { width: 8, stride: 8, phase_bytes: 0 }, 0, -1).is_err(), "negative count");
    assert!(bad(Shape { width: 8, stride: 8, phase_bytes: 0 }, 0, 0).is_ok(), "an empty range does nothing, as the crate");
    assert!(dv::zero_border_dev(s, p(0), 8, 0, 8, 0, 2).is_err(), "height 0");
    assert!(dv::scale_plane_dev(s, p(0), 8, 8, 1.0, 0, u32::MAX).is_err(), "count above i32");
}

/// Single-edit mutants of the kernel source: (kernel, name, text in the program, replacement). Each text must occur once.
const MUTANTS: &[(K, &str, &str, &str)] = &[
    (K::Mix, "operand swap of the vector fused step", "x = flow_a_fnms132(e, t, c);", "x = flow_a_fnms132(e, c, t);"),
    (K::Mix, "addend of the vector inner step", "float t = flow_a_fms213(f, d, b);", "float t = flow_a_fms213(f, d, a);"),
    (K::Mix, "alignment threshold of the split", "flow_a_split((long)w, 16, 29, ph, 32);\n    int vec", "flow_a_split((long)w, 16, 28, ph, 32);\n    int vec"),
    (K::Mix, "dropped sign flip of the scalar form", "x = flow_a_flip(p);", "x = p;"),
    (K::Mix, "unfused second operand of the scalar form", "y = flow_a_fma213(f, d, a);", "y = flow_a_add(flow_a_mul(f, d), a);"),
    (K::Flow, "compare of the upper clamp", "} else if (num > thr) {", "} else if (num >= thr) {"),
    (K::Flow, "sign of the divided step", "du = flow_a_div(flow_a_flip(p), det);", "du = flow_a_div(p, det);"),
    (K::Flow, "contracted determinant", "float det = flow_a_fma231(det0, a, a);", "float det = flow_a_add(det0, flow_a_mul(a, a));"),
    (K::Flow, "threshold operand", "float thr = flow_a_mul(s, det);", "float thr = flow_a_mul(s, b);"),
    (K::Flow, "determinant compare direction", "} else if (eps > det) {", "} else if (eps < det) {"),
    (K::Diff, "lane reduction order", "float lp0 = flow_a_add(ls0, ls2);", "float lp0 = flow_a_add(ls0, ls1);"),
    (K::Diff, "alignment threshold of the split", "flow_a_split((long)w, 8, 907, ph, 32);", "flow_a_split((long)w, 8, 906, ph, 32);"),
    (K::Diff, "lane zero seed", "\n        lanes[0] = acc;", "\n        lanes[0] = as_float(0u);"),
    (K::Diff, "unfused square sum", "float s = flow_a_fma213(diff_q, diff_q, t);", "float s = flow_a_add(flow_a_mul(diff_q, diff_q), t);"),
    (K::Diff, "accumulation operand order of the head", "float s = flow_a_diff_one(pp, pq, px, py, off + i);\n            acc = flow_a_add(s, acc);\n        }\n        float lanes[8];", "float s = flow_a_diff_one(pp, pq, px, py, off + i);\n            acc = flow_a_add(acc, s);\n        }\n        float lanes[8];"),
    (K::Zero, "last row compare", "y == (long)h - 1 ||", "y == (long)h ||"),
    (K::Zero, "last column", "j == w - 1)", "j == w - 2)"),
    (K::Scale, "operand order of the product", "pl[ix] = flow_a_mul(pl[ix], as_float(scale_bits));", "pl[ix] = flow_a_mul(as_float(scale_bits), pl[ix]);"),
    (K::Relax, "dropped sign flip of the last row column 0", "d = flow_a_flip(uu[rb - st]);", "d = uu[rb - st];"),
    (K::Relax, "fused arm of the scalar last row", "r = flow_a_fnma213(pvv, flow_a_sub(ua, dx), a);", "r = flow_a_fma213(pvv, flow_a_sub(ua, dx), a);"),
    (K::Relax, "vector block mask", "sp = (long3)(0, n & ~15, 0);", "sp = (long3)(0, n & ~7, 0);"),
    (K::Relax, "alignment line of the relax planes", "long lb = 64;", "long lb = 32;"),
    (K::Relax, "unfused middle row of the scale absent form", "float tm = flow_a_fma213(dx, scale, a);", "float tm = flow_a_add(flow_a_mul(dx, scale), a);"),
    (K::Relax, "nest select of the accumulator", "__global const float* aa = (nest == 0) ? a0 : a1;", "__global const float* aa = (nest == 0) ? a0 : a0;"),
    (K::Mix, "vector body end compare", "int vec = (sp.z == 0 && i >= sp.x && i < sp.y) ? 1 : 0;", "int vec = (sp.z == 0 && i >= sp.x && i <= sp.y) ? 1 : 0;"),
];

#[test]
fn every_solve_tile_mutant_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev_h, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let bin = |s: &Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(Session::open_source(&dev_h, &info, &base, opts).unwrap());
    let base_hash = bin(&clean);
    for k in ALL {
        let t = compare(&clean, k, false, true);
        assert_eq!(t.mismatches, 0, "the unmutated program must pass {k:?}: {:?}", t.first);
    }
    let mut killed = 0usize;
    for (k, name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(Session::open_source(&dev_h, &info, &text, opts).unwrap());
        let h = bin(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        let t = compare(&s, *k, false, true);
        let verdict = if t.mismatches > 0 { "KILLED" } else { "SURVIVED" };
        if t.mismatches > 0 {
            killed += 1;
        }
        println!("MUTANT {k:?} {name} binary {h:016x}: {verdict} (cases run {})", t.cases);
    }
    println!("MUTANTS killed {killed} of {}", MUTANTS.len());
    assert_eq!(killed, MUTANTS.len(), "a solve tile mutant survived the compare");
}
