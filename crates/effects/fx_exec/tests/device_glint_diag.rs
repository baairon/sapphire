//! Device helpers of the glint diagonal passes against the CPU crates, word for word: `glint_diag_box::run_body` (both bodies) and
//! `diagonal_box_sum::box_sum_diagonals` (both diagonal kinds). Also: a zero-writing stub kernel must fail the comparison, and
//! single-edit mutants of the OpenCL source must each be refused. Prints NOT RUN without an OpenCL device (run with --test-threads=1).

use diagonal_box_sum::{box_sum_diagonals, Diagonal, Params as BoxParams};
use fx_device::{Backend as Handle, DeviceKind, Session};
use fx_exec::device_ops_glint_diag::{diag_box_sum_dev, glint_diag_box_dev, DiagDevError};
use glint_diag_box::{run_body, Band, Error, Params, RowPhases};
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

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u32 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        (self.0 >> 32) as u32
    }
}

const SPECIALS: [u32; 14] = [
    0x7fc0_0001, 0xffc0_0123, 0x7fa0_0001, 0xff80_0001, 0x7f80_0000, 0xff80_0000, 0x0000_0000, 0x8000_0000, 0x0000_0001, 0x807f_ffff,
    0x7f7f_ffff, 0xff7f_ffff, 0x7fff_ffff, 0xffc0_0000,
];

/// mode 0: finite values of wide magnitude (cancellation prone); 1: mode 0 with a sparse special value (1 in 40); 2: dense specials (1 in 6).
fn vals(rng: &mut Rng, n: usize, mode: u32) -> Vec<f32> {
    (0..n)
        .map(|_| {
            let r = rng.next();
            let dens = match mode {
                1 => 40,
                2 => 6,
                _ => 0,
            };
            if dens != 0 && rng.next() % dens == 0 {
                return f32::from_bits(SPECIALS[(rng.next() as usize) % SPECIALS.len()]);
            }
            let mant = 1.0 + (r & 0xffff) as f32 / 65536.0;
            let e = (rng.next() % 41) as i32 - 20;
            let s = if r >> 31 == 1 { -1.0 } else { 1.0 };
            s * mant * 2f32.powi(e)
        })
        .collect()
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    words: usize,
    nan_words: usize,
    errors_equal: usize,
    vector_route_cases: usize,
    mismatches: usize,
    first: Option<String>,
}

impl Tally {
    fn fail(&mut self, what: String) {
        self.mismatches += 1;
        if self.first.is_none() {
            self.first = Some(what);
        }
    }
}

const SIZES: [(usize, usize); 17] = [
    (1, 1), (2, 2), (3, 3), (7, 7), (8, 8), (9, 9), (16, 16), (31, 31), (33, 33), (64, 32), (50, 7), (257, 5), (5, 257), (1201, 3),
    (301, 150), (200, 180), (130, 70),
];
const RADII: [[i32; 3]; 12] = [
    [0, 0, 0], [1, 0, 0], [0, 2, 0], [1, 2, 1], [2, 2, 2], [63, 1, 0], [64, 2, 0], [70, 65, 2], [130, 0, 1], [100000, 5, 3], [i32::MAX, 0, 0],
    [-3, 1, 0],
];
const SHIFTS: [i32; 7] = [0, 1, -1, 3, 100, 3000, 0x4000_0000];
const PHASES: [[u32; 3]; 9] = [[0, 0, 0], [4, 8, 12], [28, 16, 0], [1, 2, 3], [31, 5, 24], [8, 8, 8], [12, 0, 20], [2, 6, 10], [20, 4, 16]];

fn compare_words(t: &mut Tally, tag: &str, cpu: &[f32], dev: &[f32]) {
    for (i, (a, b)) in cpu.iter().zip(dev).enumerate() {
        t.words += 1;
        if a.is_nan() {
            t.nan_words += 1;
        }
        if a.to_bits() != b.to_bits() {
            t.fail(format!("{tag}: word {i} cpu {:08x} device {:08x}", a.to_bits(), b.to_bits()));
            return;
        }
    }
}

/// Runs the glint corpus; stops at the first mismatch when `stop`.
fn glint_corpus(s: &Arc<Session>, stop: bool) -> Tally {
    let mut t = Tally::default();
    let mut rng = Rng(0x1234_5678_9abc_def1);
    let mut idx = 0usize;
    for &(w, h) in &SIZES {
        let bands = [(0, (h + w / 2 + 3) as i32), (0, 3), (2, 5), (h as i32 / 2, (w + h) as i32), (1, 0)];
        for radii in RADII {
            for d in SHIFTS {
                for mirrored in [false, true] {
                    idx += 1;
                    let (ph, b, mode) = (PHASES[idx % PHASES.len()], bands[idx % bands.len()], (idx % 3) as u32);
                    let slack = if idx % 7 == 0 { 6 } else { 0 };
                    let n = w * h;
                    let src = vals(&mut rng, n, mode);
                    let dst0 = vals(&mut rng, n + slack, 0);
                    let p = Params { radii, d };
                    let band = Band { start: b.0, count: b.1 };
                    let mut want = dst0.clone();
                    let cpu = run_body(mirrored, &p, &RowPhases(ph), band, &src, &mut want, w, h);
                    let mut got = dst0.clone();
                    let dev = glint_diag_box_dev(s, mirrored, &p, &RowPhases(ph), band, &src, &mut got, w, h);
                    t.cases += 1;
                    let tag = format!("glint {w}x{h} mirrored {mirrored} radii {radii:?} d {d} ph {ph:?} band {b:?} mode {mode} slack {slack}");
                    match (&cpu, &dev) {
                        (Ok(()), Ok(())) => compare_words(&mut t, &tag, &want, &got),
                        (Err(Error::Size), Err(DiagDevError::Glint(Error::Size))) | (Err(Error::ShiftMin), Err(DiagDevError::Glint(Error::ShiftMin))) => {
                            t.errors_equal += 1
                        }
                        (Err(Error::ReadRange(_) | Error::WriteRange(_)), Err(DiagDevError::Range)) => t.errors_equal += 1,
                        _ => t.fail(format!("{tag}: cpu {cpu:?} device {dev:?}")),
                    }
                    if radii.iter().any(|&r| r > 62) && w.min(2 * h) > 127 {
                        t.vector_route_cases += 1;
                    }
                    if stop && t.mismatches > 0 {
                        return t;
                    }
                }
            }
        }
    }
    // explicit refusals
    let p = Params { radii: [1, 1, 1], d: i32::MIN };
    let mut dst = vec![0f32; 16];
    let src = vec![0f32; 16];
    for mirrored in [false, true] {
        let cpu = run_body(mirrored, &p, &RowPhases([0; 3]), Band { start: 0, count: 2 }, &src, &mut dst.clone(), 4, 4);
        let dev = glint_diag_box_dev(s, mirrored, &p, &RowPhases([0; 3]), Band { start: 0, count: 2 }, &src, &mut dst, 4, 4);
        t.cases += 1;
        if cpu == Err(Error::ShiftMin) && matches!(dev, Err(DiagDevError::Glint(Error::ShiftMin))) {
            t.errors_equal += 1;
        } else {
            t.fail(format!("shift min: cpu {cpu:?} device {dev:?}"));
        }
        let p2 = Params { radii: [1, 1, 1], d: 1 };
        let cpu = run_body(mirrored, &p2, &RowPhases([0; 3]), Band { start: 0, count: 2 }, &src[..8], &mut dst.clone(), 4, 4);
        let dev = glint_diag_box_dev(s, mirrored, &p2, &RowPhases([0; 3]), Band { start: 0, count: 2 }, &src[..8], &mut dst, 4, 4);
        t.cases += 1;
        if cpu == Err(Error::Size) && matches!(dev, Err(DiagDevError::Glint(Error::Size))) {
            t.errors_equal += 1;
        } else {
            t.fail(format!("short plane: cpu {cpu:?} device {dev:?}"));
        }
    }
    t
}

fn box_corpus(s: &Arc<Session>, stop: bool) -> Tally {
    let mut t = Tally::default();
    let mut rng = Rng(0x0fed_cba9_8765_4321);
    let mut idx = 0usize;
    let phase_sets: [[u8; 5]; 6] = [[0; 5], [4, 8, 12, 16, 20], [28, 0, 4, 8, 12], [1, 2, 3, 5, 6], [31, 24, 16, 8, 4], [12, 4, 20, 28, 0]];
    for &(w, h) in &SIZES {
        let total = (w + h - 1) as i32;
        let bands = [(0, total), (0, 1), (1, 3), (total / 2, total - total / 2), (0, 0), (-1, 2), (total - 1, 5)];
        for radii in RADII {
            for shift in SHIFTS {
                for kind in [Diagonal::Anti, Diagonal::Main] {
                    idx += 1;
                    let (b, mode) = (bands[idx % bands.len()], (idx % 3) as u32);
                    let phases = phase_sets[idx % phase_sets.len()];
                    let img0 = vals(&mut rng, w * h, mode);
                    let p = BoxParams { w: w as i32, h: h as i32, radii, shift, phases, start: b.0, count: b.1, kind };
                    let mut want = img0.clone();
                    let cpu = box_sum_diagonals(&mut want, &p);
                    let mut got = img0.clone();
                    let dev = diag_box_sum_dev(s, &mut got, &p);
                    t.cases += 1;
                    let tag = format!("box {w}x{h} {kind:?} radii {radii:?} shift {shift} phases {phases:?} band {b:?} mode {mode}");
                    match (&cpu, &dev) {
                        (Ok(()), Ok(())) => compare_words(&mut t, &tag, &want, &got),
                        (Err(a), Err(DiagDevError::BoxSum(b))) if a == b => t.errors_equal += 1,
                        _ => t.fail(format!("{tag}: cpu {cpu:?} device {dev:?}")),
                    }
                    if radii.iter().any(|&r| r > 62) && w.min(h) > 127 {
                        t.vector_route_cases += 1;
                    }
                    if stop && t.mismatches > 0 {
                        return t;
                    }
                }
            }
        }
    }
    // explicit refusals
    let mut img = vec![0f32; 12];
    let p = BoxParams { w: 4, h: 3, radii: [1, 0, 0], shift: 0, phases: [0; 5], start: 0, count: 2, kind: Diagonal::Main };
    for (name, q) in [
        ("bad size", BoxParams { w: 0, ..p.clone() }),
        ("bad length", BoxParams { w: 5, ..p.clone() }),
        ("bad band", BoxParams { start: 30, ..p.clone() }),
        ("bad shift", BoxParams { shift: i32::MIN, ..p.clone() }),
    ] {
        let cpu = box_sum_diagonals(&mut img.clone(), &q);
        let dev = diag_box_sum_dev(s, &mut img, &q);
        t.cases += 1;
        match (&cpu, &dev) {
            (Err(a), Err(DiagDevError::BoxSum(b))) if a == b => t.errors_equal += 1,
            _ => t.fail(format!("{name}: cpu {cpu:?} device {dev:?}")),
        }
    }
    t
}

fn report(name: &str, t: &Tally) {
    let nan = if t.mismatches == 0 { "NanPayloads::Same" } else { "NanPayloads: not established" };
    println!(
        "CORPUS {name}: cases {} words {} nan_words {} named_errors_equal {} vector_route_cases {} mismatches {} {nan} first {:?}",
        t.cases, t.words, t.nan_words, t.errors_equal, t.vector_route_cases, t.mismatches, t.first
    );
}

#[test]
fn the_glint_diagonal_helpers_equal_the_cpu_crates() {
    let Some(ctx) = device() else { return };
    let g = glint_corpus(&ctx.session, false);
    report("glint_diag_box", &g);
    let b = box_corpus(&ctx.session, false);
    report("diagonal_box_sum", &b);
    assert_eq!(g.mismatches, 0, "{:?}", g.first);
    assert_eq!(b.mismatches, 0, "{:?}", b.first);
    assert!(g.nan_words > 1000 && b.nan_words > 1000, "the corpus must reach NaN outputs");
    assert!(g.vector_route_cases > 20 && b.vector_route_cases > 20);
}

const STUB: &str = r#"
__kernel void glint_diag_band_f32(__global const float* src, __global float* dst, __global float* scratch, __global const uint* phases,
    __global uint* err, int w, int h, long slen, long dlen, int mirrored, int band_start, long first, long nitems, int r0, int r1, int r2,
    long dd, uint fbits, long cap) {
    long g = (long)get_global_id(0);
    if (g < nitems && g < dlen) { dst[g] = 0.0f; }
}
__kernel void glint_diag_edge_f32(__global const float* src, __global float* dst, __global uint* err, int w, int h, long slen, long dlen,
    int mirrored) {
}
__kernel void diag_box_sum_f32(__global float* img, __global float* scratch, __global const uint* phases, int w, int h, int m, int kind,
    int base_t, long a, int r0, int r1, int r2, int shift_nz, uint sbits, int rcl, long first, long nitems) {
    long g = (long)get_global_id(0);
    if (g < nitems) { img[g] = 0.0f; }
}
"#;

#[test]
fn a_zero_writing_stub_kernel_fails_the_harness() {
    let Some(ctx) = device() else { return };
    let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let text = format!("{}\n{}", fx_device::prelude::PRELUDE, STUB);
    let s = Arc::new(Session::open_source(&dev, &info, &text, fx_device::kernels::BUILD_OPTIONS).unwrap());
    let g = glint_corpus(&s, true);
    let b = box_corpus(&s, true);
    println!("STUB glint mismatches {} first {:?}", g.mismatches, g.first);
    println!("STUB box mismatches {} first {:?}", b.mismatches, b.first);
    assert!(g.mismatches > 0 && b.mismatches > 0, "the harness accepted a stub that writes zeros");
}

/// Single-edit mutants: (name, text in the program, replacement). Each must occur once in the program.
const MUTANTS: &[(&str, &str, &str)] = &[
    ("operand swap in the diff-first update", "s = nanx_add(nanx_sub(x_in, x_out), s);", "s = nanx_add(nanx_sub(x_out, x_in), s);"),
    ("NaN operand order of the running add", "s = nanx_sub(nanx_add(s, x_in), x_out);", "s = nanx_sub(nanx_add(x_in, s), x_out);"),
    ("constant in the one-sided blend", "r = nanx_mul(0x1p-1f, x[hi_i]);", "r = nanx_mul(0x1.000002p-1f, x[hi_i]);"),
    ("compare direction of the paired test", "if ((k & 1) == 0 && (long)k < paired) {", "if ((k & 1) == 0 && (long)k <= paired) {"),
    ("fused multiply-add at the output scale", "GDB_WR(pos, nanx_mul(f, ra[2 * q]));", "GDB_WR(pos, fma(f, ra[2 * q], 0x0p+0f));"),
    ("reduction order of the lane tree", "v[l] = nanx_add(nanx_add(sa, sb), nanx_add(sc, sd));", "v[l] = nanx_add(nanx_add(sa, sc), nanx_add(sb, sd));"),
    ("select arm of the peel length", "int p = (ph == 0u) ? 0 : (int)((32u - ph) >> 2);", "int p = (ph == 0u) ? (int)((32u - ph) >> 2) : 0;"),
    ("diagonal index", "long t = (long)base_t + kk + 1;", "long t = (long)base_t + kk + 2;"),
    ("route gate on n and the peel", "if ((long)n < p + 64) {", "if ((long)n <= p + 64) {"),
    ("route gate on the phase bits", "if ((ph & 3u) != 0u) {\n            return gdb_wsum_scalar(x, n);", "if ((ph & 1u) != 0u) {\n            return gdb_wsum_scalar(x, n);"),
    ("compare direction of the lower tap", "int lo = lo_i >= 0;", "int lo = lo_i > 0;"),
    ("index of the shrink step subtrahend", "s = nanx_sub(s, x[mid_start + j]);", "s = nanx_sub(s, x[mid_start + j + 1]);"),
    ("compare direction of the bottom edge test", "if (b_ + (l >> 1) < (long)h - 1) {", "if (b_ + (l >> 1) <= (long)h - 1) {"),
    ("NaN operand order of the first mirrored word", "rb[0] = nanx_mul(0x1p-1f, nanx_add(x0, x1));", "rb[0] = nanx_mul(0x1p-1f, nanx_add(x1, x0));"),
    ("NaN operand order of the odd step", "ra[2 * k + 1] = nanx_mul(0x1p-1f, nanx_add(x, y));", "ra[2 * k + 1] = nanx_mul(0x1p-1f, nanx_add(y, x));"),
];

#[test]
fn every_glint_diagonal_mutant_is_refused() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let hash = |s: &Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(Session::open_source(&dev, &info, &base, opts).unwrap());
    let base_hash = hash(&clean);
    assert_eq!(glint_corpus(&clean, true).mismatches, 0);
    assert_eq!(box_corpus(&clean, true).mismatches, 0);
    let mut prev = base_hash;
    let mut killed = 0usize;
    for (name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(Session::open_source(&dev, &info, &text, opts).unwrap_or_else(|e| panic!("{name}: build: {e}")));
        let h = hash(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        assert_ne!(h, prev, "mutant `{name}` built the previous mutant's binary");
        prev = h;
        let g = glint_corpus(&s, true);
        let b = box_corpus(&s, true);
        let verdict = match (&g.first, &b.first) {
            (None, None) => "SURVIVED".to_string(),
            (Some(m), _) => format!("KILLED by glint corpus ({m})"),
            (None, Some(m)) => format!("KILLED by box corpus ({m})"),
        };
        if !verdict.starts_with("SURVIVED") {
            killed += 1;
        }
        println!("MUTANT {name} binary {h:016x}: {verdict}");
    }
    println!("MUTANTS killed {killed} of {}", MUTANTS.len());
    assert_eq!(killed, MUTANTS.len(), "a mutant survived");
}
