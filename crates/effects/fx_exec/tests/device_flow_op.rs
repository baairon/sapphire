//! The composed `optical_flow` device op: the compose kernels against the crate functions, the sequence guard of `dev_solve_body` against
//! the crate's `run_body`, the refusal guard, the whole-op self-check, the stub harness of the self-check driver, and forced device runs against the
//! CPU over the corpus, the census frame sizes and the counted parameter sets. Prints NOT RUN without an OpenCL device
//! (run with --test-threads=1: platform discovery is not thread safe).

use fx_device::ocl::Buffer;
use fx_device::selfcheck::{adversarial_f32, compare_f32, NanPayloads};
use fx_device::{Backend as Handle, DevError, DevImage, DeviceKind, Session};
use fx_exec::backend::NodeCall;
use fx_exec::device_ops::{DevArg, DeviceOp};
use fx_exec::device_ops_flow as dv;
use fx_exec::types::{Arg, Image, KParams};
use optical_flow::{build_to_cut, BodyIn, Rect};
use optical_flow_entry::compose::{level0_record, run_flow, run_flow_at, run_flow_uniform, served_image, FlowConstants, PLANE_SLACK};
use optical_flow_entry::{lane_params, ObjFields, SCALE_THRESHOLD};
use optical_flow_pyramid::LevelRecord as Level;
use optical_flow_solve::body::{Callback, Params};
use optical_flow_solve::solve::{run_body, PlaneStore};
use std::collections::BTreeMap;
use std::sync::Arc;

/// Set once the polar kernel is part of the tree: the alternate launch block (every served node) runs it.
const POLAR_READY: bool = true;

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
    fn unit(&mut self) -> f32 {
        (self.next() % 20_000) as f32 / 10_000.0 - 1.0
    }
}

fn pool_data(n: usize, seed: u64, pool: &[f32], scale: f32) -> Vec<f32> {
    let mut r = Lcg(seed);
    (0..n)
        .map(|_| match r.next() % 4 {
            0 => pool[(r.next() as usize) % pool.len()],
            _ => r.unit() * scale,
        })
        .collect()
}

fn read(s: &Arc<Session>, b: &Buffer<f32>) -> Vec<f32> {
    s.queue.read(b).unwrap()
}

// ---------------------------------------------------------------------------------------------------------------------------------
// compose kernels
// ---------------------------------------------------------------------------------------------------------------------------------

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    mismatches: usize,
    nan_differ: usize,
    first: Option<String>,
}

impl Tally {
    fn add(&mut self, r: Result<fx_device::selfcheck::Tally, fx_device::selfcheck::Mismatch>) {
        self.cases += 1;
        match r {
            Ok(t) => {
                if matches!(t.nan_payloads, NanPayloads::Differ(_)) {
                    self.nan_differ += 1;
                }
            }
            Err(m) => {
                self.mismatches += 1;
                self.first.get_or_insert_with(|| m.to_string());
            }
        }
    }
    fn fail(&mut self, why: String) {
        self.cases += 1;
        self.mismatches += 1;
        self.first.get_or_insert(why);
    }
}

fn cmp(t: &mut Tally, k: &str, case: &str, want: &[f32], got: &[f32]) {
    t.add(compare_f32(k, case, |_| String::new(), want, got));
}

/// Every compose kernel against its host reference over the corpus. `which` selects the kernel groups (0 add_const, 1 normalize, 2 clear,
/// 3 copy_rect, 4 prefix, 5 compact).
fn compare_kernels(s: &Arc<Session>, which: &[usize]) -> Tally {
    let pool = adversarial_f32(600, 17);
    let mut t = Tally::default();
    if which.contains(&0) {
        let specials = [0.0f32, -0.0, 1.5, -2.25, f32::from_bits(0x7fc0_1234), f32::from_bits(0xffc0_4321), f32::INFINITY, f32::from_bits(5)];
        for (ci, &c) in specials.iter().enumerate() {
            for &n in &[1usize, 7, 8, 33, 100] {
                let data = pool_data(n + 5, 100 + ci as u64 * 31 + n as u64, &pool, 3.0);
                let mut want = data.clone();
                optical_flow::add_const(&mut want, n as i64, c);
                let b = s.input(&data).unwrap();
                match dv::add_const_dev(s, &b, n, c) {
                    Ok(()) => cmp(&mut t, "add_const", &format!("n {n} c {:08x}", c.to_bits()), &want, &read(s, &b)),
                    Err(e) => t.fail(format!("add_const refused: {e:?}")),
                }
            }
        }
    }
    if which.contains(&1) {
        let sizes = [(1usize, 1usize), (2, 2), (3, 5), (7, 4), (9, 3), (16, 2), (17, 5), (33, 3)];
        let scales: [(f32, f32); 6] = [(1.0, 2.0), (0.5, 0.25), (f32::from_bits(0x7fc0_00aa), 1.0), (1.0, 0.0), (0.0, 0.0), (3.0, 7.0)];
        let mut idx = 0u64;
        for &(w, h) in &sizes {
            for &pad in &[0usize, 3] {
                let stride = w + pad;
                let len = h * stride + 4;
                for &(num, den) in &scales {
                    for &(row0, rows) in &[(0usize, h), (h / 2, h - h / 2), (0, 1)] {
                        idx += 1;
                        let big = 1.0 + (idx % 3) as f32 * 2.0;
                        let p = pool_data(len, idx * 3 + 1, &pool, big);
                        let fx0 = pool_data(len, idx * 3 + 2, &pool, big);
                        let fy0 = pool_data(len, idx * 3 + 3, &pool, big);
                        let (mut fx, mut fy) = (fx0.clone(), fy0.clone());
                        optical_flow::normalize_flow_rows(&p, &mut fx, &mut fy, w as i32, h as i32, stride as i32, row0 as i32, rows as i32, num, den);
                        let (pb, fxb, fyb) = (s.input(&p).unwrap(), s.input(&fx0).unwrap(), s.input(&fy0).unwrap());
                        let case = format!("{w}x{h} stride {stride} rows {row0}+{rows} scale {:08x}/{:08x}", num.to_bits(), den.to_bits());
                        match dv::normalize_flow_rows_dev(s, &pb, &fxb, &fyb, w as i32, h as i32, stride as i32, row0 as i32, rows as i32, num, den) {
                            Ok(()) => {
                                cmp(&mut t, "normalize fx", &case, &fx, &read(s, &fxb));
                                cmp(&mut t, "normalize fy", &case, &fy, &read(s, &fyb));
                            }
                            Err(e) => t.fail(format!("normalize refused {case}: {e:?}")),
                        }
                    }
                }
            }
        }
    }
    if which.contains(&1) {
        // squared lengths just above and just below one: the threshold compare decides whether the pair is rescaled
        for (i, &(w, h)) in [(5usize, 3usize), (6, 4)].iter().enumerate() {
            let len = w * h + 4;
            let p = vec![0.0f32; len];
            let near = [1.00002f32, 1.00004, 0.99998, -1.00003, 1.00006, 0.9999, 1.0, -1.00001];
            let fx0: Vec<f32> = (0..len).map(|k| near[(k + i) % near.len()]).collect();
            let fy0 = vec![0.0f32; len];
            let (mut fx, mut fy) = (fx0.clone(), fy0.clone());
            optical_flow::normalize_flow_rows(&p, &mut fx, &mut fy, w as i32, h as i32, w as i32, 0, h as i32, 0.0, 1.0);
            let (pb, fxb, fyb) = (s.input(&p).unwrap(), s.input(&fx0).unwrap(), s.input(&fy0).unwrap());
            let case = format!("near one {w}x{h}");
            match dv::normalize_flow_rows_dev(s, &pb, &fxb, &fyb, w as i32, h as i32, w as i32, 0, h as i32, 0.0, 1.0) {
                Ok(()) => {
                    cmp(&mut t, "normalize fx", &case, &fx, &read(s, &fxb));
                    cmp(&mut t, "normalize fy", &case, &fy, &read(s, &fyb));
                }
                Err(e) => t.fail(format!("normalize refused {case}: {e:?}")),
            }
        }
    }
    let lv = |w: i32, h: i32, st: i32| Level { width: w, height: h, stride: st, ..Default::default() };
    let rect_cases = [lv(1, 1, 1), lv(5, 3, 5), lv(7, 4, 9), lv(16, 2, 16), lv(9, 9, 12)];
    if which.contains(&2) || which.contains(&3) {
        for (i, l) in rect_cases.iter().enumerate() {
            let len = (l.stride * l.height) as usize + 5;
            let dst0 = pool_data(len, 500 + i as u64, &pool, 1.0);
            let src = pool_data(len, 600 + i as u64, &pool, 1.0);
            let (cw, rows, st) = (l.width as usize, l.height as usize, l.stride as usize);
            if which.contains(&2) {
                let mut want = dst0.clone();
                for y in 0..rows {
                    for x in 0..cw {
                        want[y * st + x] = f32::from_bits(0x7fc0_beef);
                    }
                }
                let b = s.input(&dst0).unwrap();
                match dv::flow_clear_rect(s, &b, 0x7fc0_beef, l) {
                    Ok(()) => cmp(&mut t, "clear_rect", &format!("{cw}x{rows}/{st}"), &want, &read(s, &b)),
                    Err(e) => t.fail(format!("clear refused: {e:?}")),
                }
            }
            if which.contains(&3) {
                let mut want = dst0.clone();
                for y in 0..rows {
                    for x in 0..cw {
                        want[y * st + x] = src[y * st + x];
                    }
                }
                let (b, sb) = (s.input(&dst0).unwrap(), s.input(&src).unwrap());
                match dv::flow_copy_rect(s, &b, &sb, l) {
                    Ok(()) => cmp(&mut t, "copy_rect", &format!("{cw}x{rows}/{st}"), &want, &read(s, &b)),
                    Err(e) => t.fail(format!("copy_rect refused: {e:?}")),
                }
            }
        }
    }
    if which.contains(&4) {
        for &n in &[1usize, 9, 64, 100] {
            let dst0 = pool_data(120, 700 + n as u64, &pool, 1.0);
            let src = pool_data(120, 800 + n as u64, &pool, 1.0);
            let mut want = dst0.clone();
            want[..n].copy_from_slice(&src[..n]);
            let (b, sb) = (s.input(&dst0).unwrap(), s.input(&src).unwrap());
            match dv::flow_copy_prefix(s, &b, &sb, n) {
                Ok(()) => cmp(&mut t, "copy_prefix", &format!("n {n}"), &want, &read(s, &b)),
                Err(e) => t.fail(format!("prefix refused: {e:?}")),
            }
        }
    }
    if which.contains(&5) {
        for (i, l) in rect_cases.iter().enumerate() {
            let len = (l.stride * l.height) as usize + 5;
            let src = pool_data(len, 900 + i as u64, &pool, 1.0);
            let (cw, rows, st) = (l.width as usize, l.height as usize, l.stride as usize);
            let want = optical_flow_entry::compose::compact(&src, cw, rows, st);
            let (b, sb) = (s.output::<f32>(cw * rows).unwrap(), s.input(&src).unwrap());
            match dv::flow_compact(s, &b, &sb, l.width, l.height, l.stride) {
                Ok(()) => cmp(&mut t, "compact", &format!("{cw}x{rows}/{st}"), &want, &read(s, &b)),
                Err(e) => t.fail(format!("compact refused: {e:?}")),
            }
        }
    }
    t
}

#[test]
fn compose_kernels_equal_the_crate_functions() {
    let Some(ctx) = device() else { return };
    let t = compare_kernels(&ctx.session, &[0, 1, 2, 3, 4, 5]);
    println!("COMPOSE KERNELS cases {} mismatches {} nan payload differences {}", t.cases, t.mismatches, t.nan_differ);
    assert_eq!(t.mismatches, 0, "{:?}", t.first);
    assert_eq!(t.nan_differ, 0, "NaN payloads differ");
    assert!(t.cases > 500);
}

#[test]
fn arguments_outside_the_model_are_refused() {
    let Some(ctx) = device() else { return };
    let s = &ctx.session;
    let b = s.input(&[0.0f32; 16]).unwrap();
    assert!(dv::normalize_flow_rows_dev(s, &b, &b, &b, 4, 4, 3, 0, 4, 1.0, 2.0).is_err(), "stride below the width");
    assert!(dv::normalize_flow_rows_dev(s, &b, &b, &b, 4, 8, 4, 0, 8, 1.0, 2.0).is_err(), "planes shorter than rows times stride");
    assert!(dv::normalize_flow_rows_dev(s, &b, &b, &b, 4, 4, 4, 0, 0, 1.0, 2.0).is_ok(), "an empty row range does nothing, as the crate");
    let l = Level { width: 8, height: 8, stride: 8, ..Default::default() };
    assert!(dv::flow_clear_rect(s, &b, 0, &l).is_err(), "clear outside the plane");
    assert!(dv::flow_copy_rect(s, &b, &b, &l).is_err(), "copy outside the planes");
    let big = Level { width: 0x10000, height: 1, stride: 0x10000, ..Default::default() };
    assert!(dv::flow_clear_rect(s, &b, 0, &big).is_ok(), "the crate's size guard makes this a no-op");
    assert!(dv::flow_copy_prefix(s, &b, &b, 17).is_err());
    assert!(dv::flow_compact(s, &b, &b, 5, 4, 4).is_err(), "stride below the width");
}

/// Single-edit mutants of the compose kernel source: (group, name, text, replacement). Each text must occur once.
const KERNEL_MUTANTS: &[(usize, &str, &str, &str)] = &[
    (0, "add_const operand order", "d[g] = flow_a_add(c, d[g]);", "d[g] = flow_a_add(d[g], c);"),
    (0, "add_const zero fill dropped", "if (c == as_float(0u)) {\n        d[g] = as_float(0u);", "if (c == as_float(0x44961000u)) {\n        d[g] = as_float(0u);"),
    (1, "normalize threshold", "if (norm_sq > as_float(0x3f800000u)) {\n                float n = sqrt(norm_sq);\n                fx[i] = flow_a_div(gx, n);", "if (norm_sq > as_float(0x3f800347u)) {\n                float n = sqrt(norm_sq);\n                fx[i] = flow_a_div(gx, n);"),
    (1, "normalize unfused gradient", "float gy = flow_a_fma213(gy0, s, fy[i]);\n            float gx = flow_a_fma213(gx0, s, fx[i]);", "float gy = flow_a_add(flow_a_mul(gy0, s), fy[i]);\n            float gx = flow_a_fma213(gx0, s, fx[i]);"),
    (1, "normalize last row test", "if (y < last) {", "if (y < last - 1) {"),
    (1, "normalize axis swap", "float gy0 = flow_a_sub(b, a);\n            float gx0 = flow_a_sub(c, a);", "float gy0 = flow_a_sub(c, a);\n            float gx0 = flow_a_sub(b, a);"),
    (0, "add_const zero branch keeps the word", "if (c == as_float(0u)) {\n        d[g] = as_float(0u);", "if (c == as_float(0u)) {\n        d[g] = d[g];"),
    (1, "normalize corner norm operand order", "float norm_sq = flow_a_fma231(flow_a_mul(fyo, fyo), fxo, fxo);", "float norm_sq = flow_a_fma231(flow_a_mul(fxo, fxo), fyo, fyo);"),
    (2, "clear value", "d[y * (size_t)stride + x] = val;", "d[y * (size_t)stride + x] = val ^ 1u;"),
    (3, "copy_rect stride use", "size_t i = y * (size_t)stride + x;\n    d[i] = s[i];", "size_t i = y * (size_t)stride + x;\n    d[i] = s[y * (size_t)width + x];"),
    (4, "prefix off by one", "d[g] = s[g];", "d[g] = s[g + 1];"),
    (5, "compact source index", "d[g] = s[y * (size_t)stride + x];", "d[g] = s[y * (size_t)stride + x + (y > 0 ? 1 : 0)];"),
];

#[test]
fn every_compose_kernel_mutant_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev_h, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let bin = |s: &Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(Session::open_source(&dev_h, &info, &base, opts).unwrap());
    let base_hash = bin(&clean);
    let t = compare_kernels(&clean, &[0, 1, 2, 3, 4, 5]);
    assert_eq!(t.mismatches, 0, "the unmutated program must pass: {:?}", t.first);
    let mut killed = 0usize;
    for (g, name, old, new) in KERNEL_MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(Session::open_source(&dev_h, &info, &text, opts).unwrap());
        let h = bin(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        let t = compare_kernels(&s, &[*g]);
        let dead = t.mismatches > 0 || t.nan_differ > 0;
        let verdict = if dead { "KILLED" } else { "SURVIVED" };
        if dead {
            killed += 1;
        }
        println!("MUTANT {name} binary {h:016x}: {verdict} (cases run {})", t.cases);
    }
    println!("KERNEL MUTANTS killed {killed} of {}", KERNEL_MUTANTS.len());
    assert_eq!(killed, KERNEL_MUTANTS.len(), "a compose kernel mutant survived");
}

// ---------------------------------------------------------------------------------------------------------------------------------
// sequence guard
// ---------------------------------------------------------------------------------------------------------------------------------

/// The registered byte phase of a plane in the guard: every handle gets its own phase (a multiple of 4 below 64), so a mask changed at a call site
/// is seen.
fn phase_of(id: u64) -> usize {
    (4 * ((id * 5 + 3) % 16)) as usize
}

struct RecStore {
    planes: BTreeMap<u64, Vec<f32>>,
    seq: Vec<Callback>,
}

impl PlaneStore for RecStore {
    fn read(&self, id: u64) -> Vec<f32> {
        self.planes.get(&id).cloned().unwrap_or_default()
    }
    fn write(&mut self, id: u64, v: &[f32]) {
        if let Some(p) = self.planes.get_mut(&id) {
            let n = v.len().min(p.len());
            p[..n].copy_from_slice(&v[..n]);
        }
    }
    fn align_bytes(&self, id: u64) -> u64 {
        phase_of(id) as u64
    }
    fn launched(&mut self, c: Callback) {
        self.seq.push(c);
    }
}

fn zero_frame_levels(w: usize, h: usize) -> Vec<Level> {
    let p = lane_params(&ObjFields::default(), SCALE_THRESHOLD);
    let len = w * h * 4 + 256;
    let (src_a, src_b) = (vec![0u8; len], vec![0u8; len]);
    let (mut da, mut db) = (vec![0u8; len], vec![0u8; len]);
    let inp = BodyIn { rect: Rect { w: w as i32, h: h as i32, pitch: w as i32, reserved: 0, q: 4 }, levels: p.four, scale: p.diag_weight, flag: 0, smoothing_word: 0, planes: level0_record(), src_a: &src_a, src_b: &src_b };
    let mut img = served_image;
    build_to_cut(&inp, &mut da, &mut db, &mut img).1.levels
}

fn plane_ids(r: &Level) -> [u64; 20] {
    [r.image_a, r.image_b, r.grad_x, r.grad_y, r.warped_image, r.warped_grad_x, r.warped_grad_y, r.coeff, r.flow_u, r.flow_v, r.flow_u_next, r.flow_v_next, r.nest1_t, r.nest1_u, r.nest2_t, r.nest2_u, r.relax_first, r.relax_second, r.median_out, r.copied_plane]
}

fn initial_planes(levels: &[Level], seed: u64) -> BTreeMap<u64, Vec<f32>> {
    let mut m: BTreeMap<u64, Vec<f32>> = BTreeMap::new();
    let mut r = Lcg(seed);
    let pool = adversarial_f32(400, 5);
    for l in levels {
        let n = (l.stride.max(0) as usize) * (l.height.max(0) as usize) + PLANE_SLACK;
        for id in plane_ids(l) {
            m.entry(id).or_insert_with(|| (0..n).map(|_| if r.next() % 25 == 0 { pool[(r.next() as usize) % pool.len()] } else { r.unit() * 0.5 }).collect());
        }
    }
    m
}

struct GuardCase {
    outer: i32,
    mid: i32,
    inner: i32,
    counted: i32,
    tol: f32,
    alt: i32,
}

fn guard_params(c: &GuardCase) -> Params {
    Params { outer_iters: c.outer, mid_iters: c.mid, inner_iters: c.inner, counted_route: c.counted, flow_gain: 1.0f32.to_bits(), relax_scale: 0.4f32.to_bits(), tol: c.tol, smooth_bias: 0.02f32.to_bits(), alt_launches: c.alt }
}

/// Runs the crate's `run_body` and `dev_solve_body` on the same table and planes; returns (sequence equal, planes equal, diff launches).
fn guard_run(s: &Arc<Session>, levels: &[Level], idx: i32, c: &GuardCase, seed: u64) -> Result<(usize, usize), String> {
    let init = initial_planes(levels, seed);
    let params = guard_params(c);
    let mut store = RecStore { planes: init.clone(), seq: Vec::new() };
    let mut poll = || false;
    let cpu_out = run_body(levels, idx, &params, &mut store, &mut poll);
    let mut pl = dv::DevPlanes::new(s);
    for (id, d) in &init {
        pl.register(*id, d, phase_of(*id));
    }
    let mut rec = Vec::new();
    let mut poll2 = || false;
    let dev_out = dv::dev_solve_body(levels, idx, &params, &mut pl, &mut poll2, &mut rec);
    if let Some(e) = pl.take_dev_err() {
        return Err(format!("device error {e:?}"));
    }
    if cpu_out != dev_out {
        return Err(format!("outcomes differ {cpu_out:?} {dev_out:?}"));
    }
    if store.seq != rec {
        let at = store.seq.iter().zip(&rec).position(|(a, b)| a != b).unwrap_or(store.seq.len().min(rec.len()));
        return Err(format!("launch sequences differ at {at} (crate {} launches, device {})", store.seq.len(), rec.len()));
    }
    let mut bad = 0usize;
    for (id, want) in &store.planes {
        let got = pl.words(*id).ok_or("plane missing")?;
        let t = compare_f32("solve plane", &format!("plane {id}"), |_| String::new(), want, got);
        if t.is_err() {
            bad += 1;
        }
    }
    if bad > 0 {
        return Err(format!("{bad} planes differ"));
    }
    Ok((store.seq.iter().filter(|c| **c == Callback::Diff).count(), store.seq.len()))
}

#[test]
fn dev_solve_body_follows_the_crates_run_body() {
    let Some(ctx) = device() else { return };
    let s = &ctx.session;
    let mut runs = 0usize;
    let mut exit_points = std::collections::BTreeSet::new();
    let mut counted_runs = 0usize;
    for (w, h) in [(40usize, 32usize), (64, 48)] {
        let levels = zero_frame_levels(w, h);
        assert!(!levels.is_empty());
        println!("GUARD frame {w}x{h}: {} levels", levels.len());
        let top = levels.len() as i32 - 1;
        for idx in [0, top] {
            for &alt in if POLAR_READY { &[0, 1][..] } else { &[0][..] } {
                for &counted in &[0, 1] {
                    for &(outer, mid, inner) in &[(1, 1, 1), (2, 2, 3)] {
                        for &tol in &[1e30f32, 0.0, 1e-4, 1e-2, 1.0] {
                            let c = GuardCase { outer, mid, inner, counted, tol, alt };
                            match guard_run(s, &levels, idx, &c, 77 + runs as u64) {
                                Ok((diffs, _)) => {
                                    runs += 1;
                                    exit_points.insert((outer, mid, inner, diffs));
                                    counted_runs += counted as usize;
                                }
                                Err(e) => panic!("guard failed at frame {w}x{h} idx {idx} alt {alt} counted {counted} iters {outer}/{mid}/{inner} tol {tol}: {e}"),
                            }
                        }
                    }
                }
            }
        }
    }
    println!("SEQUENCE GUARD runs {runs} (counted route {counted_runs}), distinct (iters, diff launches) {}", exit_points.len());
    assert!(runs >= 80);
    // the early exit is taken at the first inner step, never, and at a later step: more than one Diff count for the (2,2,3) set
    let counts: std::collections::BTreeSet<usize> = exit_points.iter().filter(|e| (e.0, e.1, e.2) == (2, 2, 3)).map(|e| e.3).collect();
    assert!(counts.len() >= 2, "the tol values must reach different exit points, got {counts:?}");
}

// ---------------------------------------------------------------------------------------------------------------------------------
// refusal guard, self-check, stub harness
// ---------------------------------------------------------------------------------------------------------------------------------

fn const_args(c: &dv::FlowCase) -> Vec<(usize, [u32; 4])> {
    dv::case_consts(c).into_iter().map(|(_, n, l)| (n, l)).collect()
}

fn cpu_node(w: usize, h: usize, c: &dv::FlowCase, prev: &Image, next: &Image) -> Result<Image, String> {
    let mut args: Vec<Arg> = vec![Arg::Img(prev), Arg::Img(next)];
    for (n, lanes) in const_args(c) {
        args.push(Arg::Const { n, lanes });
    }
    let kp = KParams::host_defaults();
    match fx_exec::kernels::run_any("optical_flow", 3, &args, w * h, (w, h), &kp) {
        Ok(fx_exec::kernels2::Out::Img(i)) => Ok(i),
        Ok(_) => Err("not an image".into()),
        Err(e) => Err(format!("{e}")),
    }
}

fn std_case(w: usize, h: usize) -> dv::FlowCase {
    dv::FlowCase { w, h, kind: 0, iters: (1.0, 1.0, 1.0), lambda: 1.0, unit: 1.0, texture: 0.0, smooth: 0.0 }
}

#[test]
fn every_refusal_shape_gives_the_cpu_nodes_error_through_accepts() {
    let kp = KParams::host_defaults();
    let mut refused = 0usize;
    let mut run = 0usize;
    let mut shapes: Vec<(usize, usize)> = Vec::new();
    for w in 1..=12usize {
        for h in 1..=12usize {
            shapes.push((w, h));
        }
    }
    shapes.extend([(1, 5), (5, 1), (3, 3), (3, 40), (40, 3), (2, 200), (50, 7), (64, 32), (16, 16), (9, 7), (8, 8)]);
    for (w, h) in shapes {
        let c = std_case(w, h);
        let (a, b) = dv::case_frames(&c);
        let n = w * h;
        let (ia, ib) = (Image { n, planes: vec![a] }, Image { n, planes: vec![b] });
        let cpu = cpu_node(w, h, &c, &ia, &ib);
        let reason = dv::flow_shape_refusal(w, h);
        let shapes_arg: Vec<fx_exec::device_ops::ArgShape> = (0..9).map(|i| if i < 2 { fx_exec::device_ops::ArgShape::Img { channels: 1 } } else { fx_exec::device_ops::ArgShape::Const { n: 1 } }).collect();
        let call = NodeCall { idx: 0, class: "optical_flow", tag: 3, n, dims: (w, h), kp: &kp };
        let acc = dv::optical_flow_accepts(&call, &shapes_arg);
        match (&cpu, reason) {
            (Err(e), Some(r)) => {
                refused += 1;
                let label = r.split(' ').next().unwrap_or(r);
                assert!(e.contains(label), "{w}x{h}: the CPU error `{e}` does not carry the refusal label `{label}` of `{r}`");
                assert_eq!(acc, Err(r));
            }
            (Ok(_), None) => {
                run += 1;
                assert_eq!(acc, Ok(()), "{w}x{h}");
            }
            (cpu, reason) => panic!("{w}x{h}: CPU {:?} but flow_shape_refusal {:?}", cpu.as_ref().map(|_| "ok"), reason),
        }
    }
    println!("REFUSAL GUARD refused {refused} shapes, accepted and run {run} shapes, every one equal to the CPU node");
    assert!(refused >= 20 && run >= 10);
}

#[test]
fn accepts_refuses_wrong_operand_shapes() {
    use fx_exec::device_ops::ArgShape as S;
    let kp = KParams::host_defaults();
    let call = |tag| NodeCall { idx: 0, class: "optical_flow", tag, n: 64, dims: (8, 8), kp: &kp };
    let good: Vec<S> = (0..9).map(|i| if i < 2 { S::Img { channels: 3 } } else { S::Const { n: 1 } }).collect();
    assert_eq!(dv::optical_flow_accepts(&call(3), &good), Ok(()));
    assert!(dv::optical_flow_accepts(&call(2), &good).is_err(), "tag");
    assert!(dv::optical_flow_accepts(&call(3), &good[..8]).is_err(), "operand count");
    let mut v = good.clone();
    v[0] = S::Mip;
    assert!(dv::optical_flow_accepts(&call(3), &v).is_err(), "mip operand");
    let mut v = good.clone();
    v[1] = S::Img { channels: 0 };
    assert!(dv::optical_flow_accepts(&call(3), &v).is_err(), "no plane");
    let mut v = good.clone();
    v[5] = S::Img { channels: 1 };
    assert!(dv::optical_flow_accepts(&call(3), &v).is_err(), "a parameter image");
    let mut v = good.clone();
    v[7] = S::Const { n: 0 };
    assert!(dv::optical_flow_accepts(&call(3), &v).is_err(), "an empty constant");
    let mut v = good;
    v[0] = S::Const { n: 1 };
    assert_eq!(dv::optical_flow_accepts(&call(3), &v), Ok(()), "a constant colour operand is accepted");
}

fn upload_cpu_result(s: &Arc<Session>, call: &NodeCall, args: &[DevArg], zero_conf: bool) -> Result<DevImage, DevError> {
    let mut imgs: Vec<Image> = Vec::new();
    for a in args.iter().take(2) {
        match a {
            DevArg::Img(d) => imgs.push(Image { n: d.plane(0).len(), planes: d.download()? }),
            DevArg::Const { n, lanes } => imgs.push(Image::from_const(lanes, (*n).max(1), call.n)),
            _ => return Err(DevError::Unsupported("operand".into())),
        }
    }
    let mut cargs: Vec<Arg> = vec![Arg::Img(&imgs[0]), Arg::Img(&imgs[1])];
    for a in &args[2..] {
        match a {
            DevArg::Const { n, lanes } => cargs.push(Arg::Const { n: *n, lanes: *lanes }),
            _ => return Err(DevError::Unsupported("operand".into())),
        }
    }
    let Ok(fx_exec::kernels2::Out::Img(mut i)) = fx_exec::kernels::run_any("optical_flow", 3, &cargs, call.n, call.dims, call.kp) else {
        return Err(DevError::Unsupported("cpu".into()));
    };
    if zero_conf {
        i.planes[2].iter_mut().for_each(|v| *v = 0.0);
    }
    DevImage::upload(s, &i.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>())
}

fn stub_cpu(s: &Arc<Session>, call: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    upload_cpu_result(s, call, args, false)
}

fn stub_zero_confidence(s: &Arc<Session>, call: &NodeCall, args: &[DevArg]) -> Result<DevImage, DevError> {
    upload_cpu_result(s, call, args, true)
}

#[test]
fn the_self_check_driver_passes_a_cpu_stub_and_refuses_a_zeroed_confidence_plane() {
    let Some(ctx) = device() else { return };
    let ok = DeviceOp { class: "optical_flow", accepts: dv::optical_flow_accepts, run: stub_cpu, corpus: dv::optical_flow_corpus };
    let t = fx_exec::device_ops::selfcheck(&ctx.session, &ok).expect("a stub that uploads the CPU result must pass");
    println!("HARNESS stub passes: {} outputs compared, NaN payloads {:?}", t.outputs, t.nan_payloads);
    assert!(t.outputs > 0);
    let bad = DeviceOp { class: "optical_flow", accepts: dv::optical_flow_accepts, run: stub_zero_confidence, corpus: dv::optical_flow_corpus };
    let r = fx_exec::device_ops::selfcheck(&ctx.session, &bad);
    println!("HARNESS zeroed confidence: {}", match &r { Ok(_) => "PASSED (wrong)".to_string(), Err(e) => format!("refused: {}", &e[..e.len().min(120)]) });
    assert!(r.is_err(), "a stub that zeroes the confidence plane must fail the self-check");
}

#[test]
fn the_op_self_check_passes_bit_for_bit() {
    let Some(ctx) = device() else { return };
    if !POLAR_READY {
        println!("POLAR PENDING: whole-op self-check not run");
        return;
    }
    let t = fx_exec::device_ops::selfcheck(&ctx.session, &dv::OPTICAL_FLOW).expect("the self-check must pass");
    println!("SELF-CHECK optical_flow outputs {} NanPayloads {:?} corpus cases {}", t.outputs, t.nan_payloads, dv::optical_flow_corpus().len());
}

fn planes_equal(t: &mut Tally, case: &str, want: &[Vec<f32>], got: &[Vec<f32>]) {
    for (p, (w, g)) in want.iter().zip(got).enumerate() {
        cmp(t, "optical_flow", &format!("{case} plane {p}"), w, g);
    }
}

fn consts_of(c: &dv::FlowCase) -> FlowConstants {
    FlowConstants { max_warps: c.iters.0 as i32, max_outer: c.iters.1 as i32, max_inner: c.iters.2 as i32, lambda: c.lambda, use_median: c.unit as i32, texture_scale: c.texture, std_smooth: c.smooth }
}

fn device_vs_cpu(s: &Arc<Session>, c: &dv::FlowCase, t: &mut Tally, log: &mut Vec<String>) {
    let (a, b) = dv::case_frames(c);
    let want = run_flow(&a, &b, c.w as i32, c.h as i32, &consts_of(c), || false);
    let got = dv::run_dev_flow(s, &a, &b, c.w as i32, c.h as i32, &consts_of(c));
    let name = format!("{}x{} kind {} iters {:?} lambda {} unit {} tex {} smooth {}", c.w, c.h, c.kind, c.iters, c.lambda, c.unit, c.texture, c.smooth);
    match (want, got) {
        (Ok(w), Ok((d, st))) => {
            let downloaded = d.download().unwrap();
            planes_equal(t, &name, &[w.flow_u, w.flow_v, w.confidence], &downloaded);
            log.push(format!("{name}: plane reads {} uploads {} solve launches {}", st.plane_reads, st.plane_uploads, st.solve_launches.len()));
        }
        (Err(e), Err(DevError::Unsupported(_))) => log.push(format!("{name}: CPU error {e:?} and device refusal (equal outcome class)")),
        (w, g) => t.fail(format!("{name}: CPU {:?} device {:?}", w.map(|_| "ok"), g.map(|_| "ok"))),
    }
}

#[test]
fn forced_device_equals_the_cpu_node_over_the_corpus_and_the_census_runs() {
    let Some(ctx) = device() else { return };
    if !POLAR_READY {
        println!("POLAR PENDING: whole runs not run");
        return;
    }
    let s = &ctx.session;
    let mut t = Tally::default();
    let mut log = Vec::new();
    for c in dv::flow_cases() {
        device_vs_cpu(s, &c, &mut t, &mut log);
    }
    // the census frame sizes with the counted parameter sets (max_warps 5, max_outer 3, max_inner 1, std_smooth 0.2, unit constant 1.0)
    for (w, h) in [(50usize, 7usize), (64, 32)] {
        for &lambda in &[0.0f32, 1.0] {
            for &texture in &[0.0f32, 0.25] {
                for &kind in &[0u8, 1] {
                    device_vs_cpu(s, &dv::FlowCase { w, h, kind, iters: (5.0, 3.0, 1.0), lambda, unit: 1.0, texture, smooth: 0.2 }, &mut t, &mut log);
                }
            }
        }
    }
    for l in &log {
        println!("RUN {l}");
    }
    println!("DEVICE VS CPU runs {} mismatching planes {} NaN payload differences {}", log.len(), t.mismatches, t.nan_differ);
    assert_eq!(t.mismatches, 0, "{:?}", t.first);
    assert_eq!(t.nan_differ, 0);
}

#[test]
fn accepts_refuses_a_frame_size_that_overflows_usize() {
    use fx_exec::device_ops::ArgShape as S;
    let kp = KParams::host_defaults();
    let good: Vec<S> = (0..9).map(|i| if i < 2 { S::Img { channels: 1 } } else { S::Const { n: 1 } }).collect();
    for dims in [(usize::MAX, 2usize), (1usize << 40, 1usize << 40), (2usize, usize::MAX)] {
        let call = NodeCall { idx: 0, class: "optical_flow", tag: 3, n: 0, dims, kp: &kp };
        assert_eq!(dv::optical_flow_accepts(&call, &good), Err("optical_flow: frame size and plane length differ"), "{dims:?}");
    }
}

/// Phase cases (ALIGN-PHASE-HOST): the device flow takes the placement's phases from the same host value as the CPU and sums rows in the
/// same order. Host phase 16 gives (image_a, image_b) = (0, 16) at 64x32 and (16, 0) at 72x24; host phase 0 gives the swapped pairs; the
/// uniform runs give (0, 0) and (16, 16).
#[test]
fn forced_device_equals_the_cpu_node_at_every_placement_phase() {
    let Some(ctx) = device() else { return };
    if !POLAR_READY {
        println!("POLAR PENDING: phase runs not run");
        return;
    }
    let s = &ctx.session;
    let mut t = Tally::default();
    let mut runs = 0usize;
    for (w, h) in [(64usize, 32usize), (72, 24), (50, 7), (128, 64)] {
        for &kind in &[0u8, 1] {
            for &lambda in &[0.0f32, 1.0] {
                let c = dv::FlowCase { w, h, kind, iters: (5.0, 3.0, 1.0), lambda, unit: 1.0, texture: 0.25, smooth: 0.2 };
                let (a, b) = dv::case_frames(&c);
                let k = consts_of(&c);
                let name = format!("{w}x{h} kind {kind} lambda {lambda}");
                for hp in [0u32, 4, 8, 16, 32, 48] {
                    let want = run_flow_at(&a, &b, w as i32, h as i32, &k, hp, || false).expect("cpu");
                    let (d, _) = dv::run_dev_flow_at(s, &a, &b, w as i32, h as i32, &k, hp).expect("device");
                    planes_equal(&mut t, &format!("{name} host phase {hp}"), &[want.flow_u, want.flow_v, want.confidence], &d.download().unwrap());
                    runs += 1;
                }
                for u in [0u32, 16] {
                    let want = run_flow_uniform(&a, &b, w as i32, h as i32, &k, u, || false).expect("cpu");
                    let (d, _) = dv::run_dev_flow_uniform(s, &a, &b, w as i32, h as i32, &k, u).expect("device");
                    planes_equal(&mut t, &format!("{name} uniform {u}"), &[want.flow_u, want.flow_v, want.confidence], &d.download().unwrap());
                    runs += 1;
                }
            }
        }
    }
    println!("PHASE RUNS {runs} mismatching planes {} NaN payload differences {}", t.mismatches, t.nan_differ);
    assert_eq!(t.mismatches, 0, "{:?}", t.first);
    assert_eq!(t.nan_differ, 0);
}

#[test]
fn device_placement_equals_the_cpu_placement_plane_by_plane() {
    let Some(ctx) = device() else { return };
    let c = consts_of(&dv::FlowCase { w: 0, h: 0, kind: 0, iters: (5.0, 3.0, 1.0), lambda: 1.0, unit: 1.0, texture: 0.25, smooth: 0.2 });
    let mut total = 0usize;
    for (w, h) in [(64i32, 32i32), (72, 24), (50, 7), (128, 64)] {
        for hp in [optical_flow_entry::placement::HOST_PHASE_DEFAULT, 0, 4, 8, 16, 32, 48] {
            total += dv::check_device_placement(&ctx.session, w, h, &c, hp).unwrap_or_else(|e| panic!("{e}"));
        }
    }
    println!("PLACEMENT planes compared {total}");
    assert!(total > 0);
}

#[test]
fn accepts_refuses_an_invalid_host_phase() {
    use fx_exec::device_ops::ArgShape as S;
    let mut kp = KParams::host_defaults();
    kp.flow_host_phase = 6;
    let good: Vec<S> = (0..9).map(|i| if i < 2 { S::Img { channels: 1 } } else { S::Const { n: 1 } }).collect();
    let call = NodeCall { idx: 0, class: "optical_flow", tag: 3, n: 64 * 32, dims: (64, 32), kp: &kp };
    assert!(dv::optical_flow_accepts(&call, &good).is_err());
    kp.flow_host_phase = 8;
    let call = NodeCall { idx: 0, class: "optical_flow", tag: 3, n: 64 * 32, dims: (64, 32), kp: &kp };
    assert!(dv::optical_flow_accepts(&call, &good).is_ok());
}
