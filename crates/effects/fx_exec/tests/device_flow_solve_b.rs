//! The optical flow solve kernels, part B, on the device against the crate functions: `smooth_normalise` (both passes), `median_rows`,
//! `median_pair` (with the copy back), and the vector lanes `powf_lane` and `expf_lane` with their scalar special-lane routines.
//! Every output word is compared (NaN payloads included); refusals are compared as refusals; a zero-writing stub must be refused by the
//! compare; single-edit mutants of the program text must each be refused. Prints NOT RUN without an OpenCL device (run with
//! --test-threads=1: platform discovery is not thread safe).

use fx_device::cl::Buffer;
use fx_device::selfcheck::adversarial_f32;
use fx_device::{Backend as Handle, DevImage, DeviceKind, Session};
use fx_exec::device_ops_flow_solve_b::{median_pair_dev, median_rows_dev, smooth_normalise_dev, vmath_dev, DevPlanes, SmoothPass};
use optical_flow_solve::smooth::{smooth_normalise, Geo, Pass};
use optical_flow_solve::tail::{median_pair, TailHost};
use optical_flow_solve::{LaunchArgs, Level};
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
    fn word(&mut self) -> u32 {
        ((self.next() << 17) ^ self.next()) as u32
    }
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    equal_words: usize,
    refusals_equal: usize,
    nan_words_same: usize,
    mismatches: usize,
    first_mismatch: Option<String>,
}
impl Tally {
    fn note(&mut self, ok: bool, what: impl FnOnce() -> String) {
        self.cases += 1;
        if !ok {
            self.mismatches += 1;
            if self.first_mismatch.is_none() {
                self.first_mismatch = Some(what());
            }
        }
    }
    fn words(&mut self, a: &[u32], b: &[u32]) -> bool {
        let same = a == b;
        if same {
            self.equal_words += a.len();
            self.nan_words_same += a.iter().filter(|w| f32::from_bits(**w).is_nan()).count();
        }
        same
    }
}

fn up(s: &Arc<Session>, v: &[u32]) -> DevImage {
    let f: Vec<f32> = v.iter().map(|b| f32::from_bits(*b)).collect();
    DevImage::upload(s, &[f.as_slice()]).unwrap()
}
fn down(img: &DevImage) -> Vec<u32> {
    img.download().unwrap()[0].iter().map(|v| v.to_bits()).collect()
}
fn fb(b: u32) -> f32 {
    f32::from_bits(b)
}

// ---------------------------------------------------------------- vector lanes

fn ulps(center: u32, n: i32) -> Vec<u32> {
    (-n..=n).map(|d| center.wrapping_add(d as u32)).collect()
}

fn lane_inputs() -> (Vec<u32>, Vec<u32>) {
    use optical_flow_solve::consts::*;
    use optical_flow_solve::scalar_consts::*;
    let pool: Vec<u32> = adversarial_f32(300, 91).iter().map(|v| v.to_bits()).collect();
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for &x in &pool {
        for &y in &pool {
            xs.push(x);
            ys.push(y);
        }
    }
    let mut r = Lcg(0x1234_5678);
    let yvals: Vec<f32> = vec![0.0, -0.0, 1.0, -1.0, 2.0, -2.0, 3.0, -3.0, 0.5, -0.5, 1.5, 2.5, 7.0, 8.0, 10.0, 16777216.0, 16777218.0, 8388609.0, 8388608.0, 33554432.0, 1.0e20, -1.0e20, 0.3333, 100.0, -100.0, 126.0, 127.0, 128.0];
    for e in (0u32..=255).step_by(3) {
        for sign in [0u32, 1] {
            let x = (sign << 31) | (e << 23) | (r.word() & 0x7f_ffff);
            for y in &yvals {
                xs.push(x);
                ys.push(y.to_bits());
            }
        }
    }
    for center in [A_000, A_040, A_080, A_0C0, A_440, A_100, B_300, B_2C0, EXP_X_MAX, EXP_X_MIN, EXP_X_DENORM, SCALE_2P64, MIN_NORMAL, F32_MAX, ONE, MINUS_ONE] {
        for x in ulps(center, 3).into_iter().chain(ulps(center ^ 0x8000_0000, 3)) {
            for y in &yvals {
                xs.push(x);
                ys.push(y.to_bits());
            }
        }
    }
    for _ in 0..150_000 {
        xs.push(r.word());
        ys.push(r.word());
    }
    // moderate range pairs: the fast path
    for _ in 0..60_000 {
        let x = ((r.word() & 0x7f_ffff) | ((120 + (r.next() % 14) as u32) << 23)) | (((r.next() & 1) as u32) << 31 & 0);
        let y = (r.word() & 0x7f_ffff) | ((120 + (r.next() % 12) as u32) << 23) | (((r.next() & 1) as u32) << 31);
        xs.push(x);
        ys.push(y);
    }
    // exponent inputs share the same list (x only): add the sweep of the exponential's range
    let mut k = -120.0f32;
    while k < 100.0 {
        xs.push(k.to_bits());
        ys.push(0);
        k += 0.0371;
    }
    (xs, ys)
}

fn cmp_vmath(s: &Arc<Session>, stub: bool, first: bool) -> Tally {
    let mut t = Tally::default();
    let (xs, ys) = lane_inputs();
    let n = xs.len();
    let xi = up(s, &xs);
    let yi = up(s, &ys);
    for pow in [true, false] {
        let out = DevImage::alloc(s, 1, n).unwrap();
        if stub {
            s.queue.fill(out.plane(0), 0.0f32).unwrap();
        } else {
            vmath_dev(s, pow, xi.plane(0), yi.plane(0), out.plane(0), n).unwrap();
        }
        let got = down(&out);
        let want: Vec<u32> = (0..n)
            .map(|i| if pow { optical_flow_solve::vmath::powf_lane(xs[i], ys[i]) } else { optical_flow_solve::vmath::expf_lane(xs[i]) })
            .collect();
        let same = t.words(&want, &got);
        t.note(same, || {
            let i = (0..n).find(|&i| want[i] != got[i]).unwrap();
            format!("{} lane {i}: x {:08x} y {:08x} cpu {:08x} dev {:08x}", if pow { "pow" } else { "exp" }, xs[i], ys[i], want[i], got[i])
        });
        if first && !same {
            break;
        }
    }
    t
}

// ---------------------------------------------------------------- smooth_normalise

#[derive(Clone, Debug)]
struct SmoothCase {
    w: i32,
    h: i32,
    stride: i32,
    scale: bool,
    c: u32,
    relax: u32,
    weight: u32,
    kind: usize,
    start: i32,
    count: u32,
    seed: u64,
}

fn smooth_cases() -> Vec<SmoothCase> {
    let cvals = [0x3f00_0000u32, 0x4000_0000, 0x3a83_126f, 0, 0xbf80_0000, 0x7fc0_0001, 0x7f80_0000, 0x8000_0000];
    let relax = [0x3f80_0000u32, 0x3f00_0000, 0, 0x7fc0_0002, 0x4120_0000];
    let weights = [0x3e80_0000u32, 0x3f80_0000, 0x4000_0000, 0xffc0_0003];
    let mut out = Vec::new();
    let mut i = 0usize;
    for &w in &[2, 3, 7, 8, 9, 15, 16, 17, 33, 50, 64] {
        for &h in &[2, 3, 7, 32] {
            for extra in [0, 3, 5] {
                for scale in [false, true] {
                    for kind in 0..3 {
                        let ranges: Vec<(i32, u32)> = vec![(0, h as u32), (1, (h - 1) as u32), (h - 1, 1), (0, 0)];
                        let (start, count) = ranges[i % ranges.len()];
                        out.push(SmoothCase {
                            w,
                            h,
                            stride: w + extra,
                            scale,
                            c: cvals[i % cvals.len()],
                            relax: relax[(i / 2) % relax.len()],
                            weight: weights[(i / 3) % weights.len()],
                            kind,
                            start,
                            count,
                            seed: 1000 + i as u64,
                        });
                        i += 1;
                    }
                }
            }
        }
    }
    // exact unit length (greater and equal arms) and NaN payload priority between the damping factor and the centre element
    for k in 0..40u64 {
        let unit = k < 4;
        out.push(SmoothCase {
            w: if unit { 3 } else { 9 },
            h: if unit { 2 } else { 7 },
            stride: if unit { 3 } else { 9 },
            scale: k % 2 == 1 && !unit,
            c: if unit { 0xbf80_0000 } else if k % 3 == 0 { 0 } else { 0x8000_0000 },
            relax: 0x3f80_0000,
            weight: if unit { 0xbf80_0000 } else { 0xffc0_0003 },
            kind: if unit { 3 } else { 4 },
            start: 0,
            count: if unit { 2 } else { 7 },
            seed: 5000 + k,
        });
    }
    out
}

fn plane_data(c: &SmoothCase, salt: u64, pool: &[u32], n: usize) -> Vec<u32> {
    let mut r = Lcg(c.seed.wrapping_mul(0x9e37_79b9).wrapping_add(salt));
    (0..n)
        .map(|_| {
            let moderate = |r: &mut Lcg| ((r.word() & 0x7f_ffff) | ((122 + (r.next() % 8) as u32) << 23)) | (((r.next() & 1) as u32) << 31);
            match c.kind {
                4 => {
                    let nl = [0x7fc0_0011u32, 0xffc0_0022, 0x7f80_0033, 0x3f80_0000, 0xbf00_0000, 0x4000_0000, 0x3f80_0000, 0xff80_0044];
                    nl[(r.next() as usize) % nl.len()]
                }
                3 => {
                    if salt == 15 || salt == 36 {
                        0x3f80_0000
                    } else {
                        0
                    }
                }
                0 => moderate(&mut r),
                1 => pool[(r.next() as usize) % pool.len()],
                _ => {
                    if r.next() % 25 == 0 {
                        pool[(r.next() as usize) % pool.len()]
                    } else {
                        moderate(&mut r)
                    }
                }
            }
        })
        .collect()
}

fn cmp_smooth(s: &Arc<Session>, stub: bool, first: bool) -> Tally {
    let pool: Vec<u32> = adversarial_f32(120, 55).iter().map(|v| v.to_bits()).collect();
    let mut t = Tally::default();
    for (ci, c) in smooth_cases().iter().enumerate() {
        let n = (c.stride * c.h) as usize;
        let planes: Vec<Vec<u32>> = (0..7).map(|k| plane_data(c, k as u64 * 7 + 1, &pool, n)).collect();
        // planes: 0 src1, 1 p1, 2 q1, 3 src2, 4 p2, 5 q2, 6 scale
        let g = Geo { width: c.w, height: c.h, stride: c.stride, relax_scale: fb(c.relax), smooth_weight: fb(c.weight), c: fb(c.c) };
        let f = |v: &Vec<u32>| -> Vec<f32> { v.iter().map(|b| fb(*b)).collect() };
        let (s1, mut p1, mut q1, s2, mut p2, mut q2, sc) = (f(&planes[0]), f(&planes[1]), f(&planes[2]), f(&planes[3]), f(&planes[4]), f(&planes[5]), f(&planes[6]));
        {
            let mut a = Pass { src: &s1, p: &mut p1, q: &mut q1 };
            let mut b = Pass { src: &s2, p: &mut p2, q: &mut q2 };
            smooth_normalise(&g, if c.scale { Some(&sc) } else { None }, &mut a, &mut b, c.start, c.count);
        }
        let imgs: Vec<DevImage> = planes.iter().map(|p| up(s, p)).collect();
        if !stub {
            let a = SmoothPass { src: imgs[0].plane(0), p: imgs[1].plane(0), q: imgs[2].plane(0) };
            let b = SmoothPass { src: imgs[3].plane(0), p: imgs[4].plane(0), q: imgs[5].plane(0) };
            smooth_normalise_dev(s, &g, if c.scale { Some(imgs[6].plane(0)) } else { None }, &a, &b, c.start, c.count).unwrap();
        } else {
            for k in [1, 2, 4, 5] {
                s.queue.fill(imgs[k].plane(0), 0.0f32).unwrap();
            }
        }
        let want: Vec<Vec<u32>> = [&p1, &q1, &p2, &q2].iter().map(|v| v.iter().map(|x| x.to_bits()).collect()).collect();
        let got: Vec<Vec<u32>> = [1, 2, 4, 5].iter().map(|k| down(&imgs[*k])).collect();
        let mut ok = true;
        for (a, b) in want.iter().zip(&got) {
            ok &= t.words(a, b);
        }
        t.note(ok, || format!("smooth case {ci} {c:?}"));
        if first && !ok {
            break;
        }
    }
    t
}

// ---------------------------------------------------------------- median

#[derive(Clone, Debug)]
struct MedCase {
    w: i32,
    rows: i32,
    pitch: i32,
    start: i32,
    count: i32,
    buf_rows: i32,
    kind: usize,
    seed: u64,
}

fn med_cases() -> Vec<MedCase> {
    let mut out = Vec::new();
    let mut i = 0u64;
    for &w in &[1, 2, 3, 4, 7, 8, 9, 15, 16, 17, 33, 50, 64] {
        for &rows in &[1, 2, 3, 5, 7, 32] {
            for extra in [0, 3] {
                for kind in 0..3 {
                    let pitch = w + extra;
                    let ranges = [(0, rows), (1, (rows - 1).max(0)), (rows - 1, 1), (0, rows + 1), (0, 0)];
                    let (start, count) = ranges[(i as usize) % ranges.len()];
                    let buf_rows = if i % 11 == 5 { rows - 1 } else { rows };
                    out.push(MedCase { w, rows, pitch, start, count, buf_rows: buf_rows.max(1), kind, seed: 40 + i });
                    i += 1;
                }
            }
        }
    }
    out
}

fn med_data(kind: usize, seed: u64, n: usize, pool: &[u32]) -> Vec<u32> {
    let small = [0u32, 0x8000_0000, 0x3f80_0000, 0xbf80_0000, 0x4000_0000, 0x7fc0_0001, 0xffc0_0002, 0x7f80_0000, 0xff80_0000, 0x0000_0001];
    let mut r = Lcg(seed * 31 + 7);
    (0..n)
        .map(|_| match kind {
            0 => small[(r.next() as usize) % small.len()],
            1 => pool[(r.next() as usize) % pool.len()],
            _ => ((r.word() & 0x7f_ffff) | ((124 + (r.next() % 6) as u32) << 23)) | (((r.next() & 1) as u32) << 31),
        })
        .collect()
}

fn cmp_median(s: &Arc<Session>, stub: bool, first: bool) -> Tally {
    let pool: Vec<u32> = adversarial_f32(100, 33).iter().map(|v| v.to_bits()).collect();
    let mut t = Tally::default();
    for (ci, c) in med_cases().iter().enumerate() {
        let n = (c.buf_rows * c.pitch) as usize;
        let src = med_data(c.kind, c.seed, n, &pool);
        let dst0 = med_data(2, c.seed + 999, n, &pool);
        let mut cpu = dst0.clone();
        let want = optical_flow_solve::median::median_rows(&src, &mut cpu, c.w, c.rows, c.pitch, c.start, c.count);
        let si = up(s, &src);
        let di = up(s, &dst0);
        if stub {
            s.queue.fill(di.plane(0), 0.0f32).unwrap();
            let ok = t.words(&cpu, &down(&di));
            t.note(ok, || format!("median case {ci} {c:?}"));
            if first && !ok {
                break;
            }
            continue;
        }
        let got = median_rows_dev(s, si.plane(0), di.plane(0), c.w, c.rows, c.pitch, c.start, c.count);
        let ok = match (&want, &got) {
            (Ok(()), Ok(())) => t.words(&cpu, &down(&di)),
            (Err(_), Err(fx_device::DevError::Unsupported(_))) => {
                t.refusals_equal += 1;
                // a refusal leaves the destination untouched
                down(&di) == dst0
            }
            _ => false,
        };
        t.note(ok, || format!("median case {ci} {c:?}: cpu {:?} dev {:?}", want.is_ok(), got.is_ok()));
        if first && !ok {
            break;
        }
    }
    t
}

// ---------------------------------------------------------------- median_pair

struct CpuTail {
    planes: Vec<Vec<u32>>,
    l: Level,
}
impl TailHost for CpuTail {
    fn launch(&mut self, args: LaunchArgs) {
        let LaunchArgs::Median(m) = args else { panic!() };
        let src = self.planes[m.plane as usize - 1].clone();
        let mut d = self.planes[m.median_out as usize - 1].clone();
        let _ = optical_flow_solve::median::median_rows(&src, &mut d, self.l.width, self.l.height, self.l.stride, 0, self.l.height);
        self.planes[m.median_out as usize - 1] = d;
    }
    fn copy_back(&mut self, plane: u64, median_out: u64, _pitch: u64, _head: [u64; 3]) {
        let (cw, rows_h, st) = (self.l.width as u32, self.l.height as u32, self.l.stride as u32 as usize);
        if cw > 0 && cw < 0x10000 && rows_h > 0 && rows_h < 0x10000 {
            let from = self.planes[median_out as usize - 1].clone();
            let d = &mut self.planes[plane as usize - 1];
            for y in 0..rows_h as usize {
                for x in 0..cw as usize {
                    d[y * st + x] = from[y * st + x];
                }
            }
        }
    }
}

struct Imgs(Vec<DevImage>);
impl DevPlanes for Imgs {
    fn plane(&self, handle: u64) -> Option<&Buffer<f32>> {
        self.0.get(handle as usize - 1).map(|i| i.plane(0))
    }
}

fn cmp_pair(s: &Arc<Session>, stub: bool, first: bool) -> Tally {
    let pool: Vec<u32> = adversarial_f32(100, 34).iter().map(|v| v.to_bits()).collect();
    let mut t = Tally::default();
    let mut i = 0u64;
    for &w in &[1, 2, 3, 4, 7, 8, 9, 17, 33] {
        for &h in &[1, 2, 3, 4, 7, 9, 16] {
            for extra in [0, 5] {
                for kind in 0..3 {
                    let stride = w + extra;
                    let l = Level { width: w, height: h, stride, elem_bytes: 4, flow_u: 1, flow_v: 2, median_out: 3, ..Default::default() };
                    let n = (stride * h) as usize;
                    let planes: Vec<Vec<u32>> = (0..3).map(|k| med_data(if k == 2 { 2 } else { kind }, 900 + i * 3 + k as u64, n, &pool)).collect();
                    let mut cpu = CpuTail { planes: planes.clone(), l };
                    median_pair(&l, &mut cpu);
                    let imgs = Imgs(planes.iter().map(|p| up(s, p)).collect());
                    let ok = if stub {
                        for im in &imgs.0 {
                            s.queue.fill(im.plane(0), 0.0f32).unwrap();
                        }
                        (0..3).all(|k| t.words(&cpu.planes[k], &down(&imgs.0[k])))
                    } else {
                        match median_pair_dev(s, &l, &imgs) {
                            Ok(()) => (0..3).all(|k| t.words(&cpu.planes[k], &down(&imgs.0[k]))),
                            Err(e) => {
                                println!("pair refused: {e:?}");
                                false
                            }
                        }
                    };
                    t.note(ok, || format!("pair case {i} w {w} h {h} stride {stride} kind {kind}"));
                    i += 1;
                    if first && !ok {
                        return t;
                    }
                }
            }
        }
    }
    t
}

fn cmp_all(s: &Arc<Session>, stub: bool, first: bool) -> Tally {
    let mut all = Tally::default();
    for (name, f) in [("vmath", cmp_vmath as fn(&Arc<Session>, bool, bool) -> Tally), ("smooth", cmp_smooth), ("median", cmp_median), ("pair", cmp_pair)] {
        let t = f(s, stub, first);
        println!("  {name}: cases {} equal_words {} refusals_equal {} nan_words_same {} mismatches {}", t.cases, t.equal_words, t.refusals_equal, t.nan_words_same, t.mismatches);
        all.cases += t.cases;
        all.equal_words += t.equal_words;
        all.refusals_equal += t.refusals_equal;
        all.nan_words_same += t.nan_words_same;
        all.mismatches += t.mismatches;
        if all.first_mismatch.is_none() {
            all.first_mismatch = t.first_mismatch;
        }
        if first && all.mismatches > 0 {
            break;
        }
    }
    all
}

#[test]
fn the_flow_solve_b_kernels_equal_the_crate_bit_for_bit() {
    let Some(ctx) = device() else { return };
    let t = cmp_all(&ctx.session, false, false);
    println!("FLOW_SOLVE_B cases {} equal_words {} refusals_equal {} nan_words_same {} mismatches {}", t.cases, t.equal_words, t.refusals_equal, t.nan_words_same, t.mismatches);
    assert_eq!(t.mismatches, 0, "{:?}", t.first_mismatch);
}

#[test]
fn a_zero_writing_stub_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    for (name, f) in [("vmath", cmp_vmath as fn(&Arc<Session>, bool, bool) -> Tally), ("smooth", cmp_smooth), ("median", cmp_median), ("pair", cmp_pair)] {
        let t = f(&ctx.session, true, true);
        println!("STUB {name} mismatches {} first {:?}", t.mismatches, t.first_mismatch);
        assert!(t.mismatches > 0, "the {name} compare accepted a stub that writes zeros");
    }
}

#[test]
fn a_refused_access_leaves_the_planes_untouched_and_the_cpu_refuses_too() {
    let Some(ctx) = device() else { return };
    let s = &ctx.session;
    let src = up(s, &[0x3f80_0000u32; 8]);
    let dst = up(s, &[0x4000_0000u32; 8]);
    // 3 x 3 plane in 8 words: the last row's next-row access leaves the plane on the CPU (an error) and the device refuses
    let cpu = optical_flow_solve::median::median_rows(&[0u32; 8], &mut [0u32; 8], 3, 3, 3, 0, 3);
    let dev = median_rows_dev(s, src.plane(0), dst.plane(0), 3, 3, 3, 0, 3);
    assert!(cpu.is_err() && dev.is_err());
    assert_eq!(down(&dst), vec![0x4000_0000u32; 8]);
}

/// Single-edit mutants of the program text: (group, name, text, replacement). Each text must occur once in the program.
const MUTANTS: &[(&str, &str, &str, &str)] = &[
    ("smooth", "operand swap in the down update", "float u_down = ofb_fms213(down_diff, sv, c_cc);", "float u_down = ofb_fms213(down_diff, sv, c_bb);"),
    ("smooth", "direction of the length test", "if (grad_sq > as_float(one_bits)) {", "if (grad_sq < as_float(one_bits)) {"),
    ("smooth", "operand roles of the centre term", "float b_term_centre = ofb_fms231(bb, d, centre);", "float b_term_centre = ofb_fms231(centre, d, bb);"),
    ("smooth", "last column test", "bool last_col = (long)i >= (long)w - 1;", "bool last_col = (long)i > (long)w - 1;"),
    ("smooth", "plane swap", "float cc = as_float(qw[ix]);", "float cc = as_float(pw[ix]);"),
    ("smooth", "unfused 231 form", "float ofb_fma231(float d, float s2, float s3) {\n    float r = fma(s2, s3, d);", "float ofb_fma231(float d, float s2, float s3) {\n    float r = s2 * s3 + d;"),
    ("smooth", "third NaN operand not quieted", "return as_float(as_uint(c) | 0x400000u);\n    }\n    return as_float(0xffc00000u);\n}\nfloat ofb_add", "return as_float(as_uint(c));\n    }\n    return as_float(0xffc00000u);\n}\nfloat ofb_add"),
    ("smooth", "pass two formula arm", "float b_term_right = (second != 0u) ? ofb_fms213(c, bb, right) : ofb_fms231(right, bb, c);\n            float b_term_centre = ofb_fms231(bb, d, centre);", "float b_term_right = (second == 0u) ? ofb_fms213(c, bb, right) : ofb_fms231(right, bb, c);\n            float b_term_centre = ofb_fms231(bb, d, centre);"),
    ("median", "minss compare", "uint ofb_minss(uint a, uint b) {\n    return (as_float(a) < as_float(b)) ? a : b;", "uint ofb_minss(uint a, uint b) {\n    return (as_float(a) <= as_float(b)) ? a : b;"),
    ("median", "maxss compare", "uint ofb_maxss(uint a, uint b) {\n    return (as_float(a) > as_float(b)) ? a : b;", "uint ofb_maxss(uint a, uint b) {\n    return (as_float(a) >= as_float(b)) ? a : b;"),
    ("median", "first row clamp", "if (row <= 0) {\n            pv = cu;", "if (row < 0) {\n            pv = cu;"),
    ("median", "column loop bound", "if (!(col < width_i - 1)) {", "if (!(col <= width_i - 1)) {"),
    ("pair", "copy stride", "d[y * (size_t)s + xx] = from[y * (size_t)s + xx];", "d[y * (size_t)cw + xx] = from[y * (size_t)s + xx];"),
    ("vmath", "table index mask of the power lane", "uint table_index = (rcp_masked >> 20) & 7;", "uint table_index = (rcp_masked >> 20) & 3;"),
    ("vmath", "scale shift of the exponential lane", "uint scale_bits = as_uint(tmp[1]) << 23;", "uint scale_bits = as_uint(tmp[1]) << 22;"),
    ("vmath", "biased exponent of the scalar exponential", "uint biased = (uint)(n + 127);", "uint biased = (uint)(n + 126);"),
    ("vmath", "odd integer test of the integer class", "if ((moved & 0xffffffu) == 0x800000u) {", "if ((moved & 0xffffffu) == 0x400000u) {"),
    ("vmath", "exponent accumulation of the power core", "exp_a = (exp_a + sel_a);", "exp_a = (exp_a - sel_a);"),
    ("vmath", "minus one test of the scalar power", "(yint != 0u || !y_finite)) {\n        return kc[KC_SIGN_ONE + (yint & 1u)];", "(yint != 0u && !y_finite)) {\n        return kc[KC_SIGN_ONE + (yint & 1u)];"),
    ("vmath", "operand roles of the power lane reduction", "tmp[2] = ofb_fms231(as_float(KC_A_100), as_float(rcp_masked), x);", "tmp[2] = ofb_fms231(x, as_float(rcp_masked), as_float(KC_A_100));"),
];

#[test]
fn every_flow_solve_b_mutant_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev_h, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let bin = |s: &Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(Session::open_source(&dev_h, &info, &base, opts).unwrap());
    let base_hash = bin(&clean);
    let t = cmp_all(&clean, false, true);
    assert_eq!(t.mismatches, 0, "the unmutated program must pass: {:?}", t.first_mismatch);
    let mut killed = 0usize;
    for (group, name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(Session::open_source(&dev_h, &info, &text, opts).unwrap());
        let h = bin(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        let m = match *group {
            "smooth" => cmp_smooth(&s, false, true),
            "median" => cmp_median(&s, false, true),
            "pair" => cmp_pair(&s, false, true),
            _ => cmp_vmath(&s, false, true),
        };
        let verdict = if m.mismatches > 0 { "KILLED" } else { "SURVIVED" };
        if m.mismatches > 0 {
            killed += 1;
        }
        println!("MUTANT {group} {name} binary {h:016x}: {verdict}");
    }
    println!("MUTANTS killed {killed} of {}", MUTANTS.len());
    assert_eq!(killed, MUTANTS.len(), "a flow solve B mutant survived the compare");
}
