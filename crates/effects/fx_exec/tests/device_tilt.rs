//! The tilt kernel on the device against `tilt::render` (the CPU crate) on the equality corpus: every output word compared (NaN
//! payloads included), a stub kernel that writes its input (the compare must refuse it), the existing `perspective` self-check as
//! the harness control, the DeviceOp self-check of `tilt` with its NaN payload report, and single-edit mutants (kernel source and
//! host parameter words), each of which the compare must refuse. Prints NOT RUN without an OpenCL device.

use fx_device::selfcheck::adversarial_f32;
use fx_device::{Backend as Handle, DevImage, DeviceKind, Session};
use fx_exec::device_ops::{selfcheck, DEVICE_OPS};
use fx_exec::device_ops_tilt::{params_of, state_of, tilt_launch};
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
    fn pick<T: Copy>(&mut self, v: &[T]) -> T {
        v[(self.next() as usize) % v.len()]
    }
}

type Lanes = [[u32; 4]; 7];

/// The parameter sets: sampled from the value lists below, plus the fixed corner sets.
fn sets() -> Vec<Lanes> {
    let nz = 0x3f80_0000u32;
    let angles: &[f32] = &[0.0, -0.0, 1.0e-3, 5.0, 20.0, 45.0, 90.0, 180.0, -35.0, 359.0];
    let persp: &[f32] = &[1.0, 0.25, 4.0, 1.0e-4, f32::MIN_POSITIVE, f32::from_bits(1), 0.0, -1.0, 1.0e6, f32::NAN, f32::INFINITY];
    let rots: &[f32] = &[0.0, -0.0, 15.0, 90.0, 180.0, -270.0, 1.0e-6, 8191.0, 10000.0];
    let shifts: &[f32] = &[0.0, -0.0, 0.25, -1.5, 100.0, 70.0, f32::NAN];
    let scales: &[f32] = &[1.0, 0.5, 2.0, -1.0, 0.0, 1.0e-20, 1.0e30];
    let shears: &[f32] = &[0.0, -0.0, 0.3, -0.2, 2.0, f32::INFINITY];
    let wraps: &[[u32; 2]] = &[[0, 0], [nz, 0], [0, nz], [nz, nz], [0x8000_0000, 0x7fc0_0000]];
    let mut r = Lcg(0x5eed);
    let mut out: Vec<Lanes> = Vec::new();
    for i in 0..700 {
        let (sv0, sv1) = if i % 5 == 0 { (0.0, 0.0) } else { (r.pick(angles), r.pick(angles)) };
        let sv0 = if i % 7 == 0 { 0.0 } else { sv0 };
        let sh = [r.pick(shifts), r.pick(shifts)];
        let sc = [r.pick(scales), r.pick(scales)];
        let sr = if i % 3 == 0 { [0.0, 0.0] } else { [r.pick(shears), r.pick(shears)] };
        let ro = if i % 4 == 0 { 0.0 } else { r.pick(rots) };
        let wr = r.pick(wraps);
        let pa = r.pick(persp);
        out.push([
            [sv0.to_bits(), sv1.to_bits(), 0, 0],
            [pa.to_bits(), 0, 0, 0],
            [sh[0].to_bits(), sh[1].to_bits(), 0, 0],
            [ro.to_bits(), 0, 0, 0],
            [sc[0].to_bits(), sc[1].to_bits(), 0, 0],
            [sr[0].to_bits(), sr[1].to_bits(), 0, 0],
            [wr[0], wr[1], 0, 0],
        ]);
    }
    // every loop variant with ordinary values: perspective on, rotate and shear on and off, all four wraps
    for rot in [0.0f32, 25.0] {
        for shear in [[0.0f32, 0.0], [0.2, -0.1]] {
            for wr in wraps.iter().take(4) {
                for (a, b) in [(20.0f32, -35.0f32), (0.0, 90.0), (180.0, 0.0), (1.0e-3, 0.0)] {
                    out.push([
                        [a.to_bits(), b.to_bits(), 0, 0],
                        [1.5f32.to_bits(), 0, 0, 0],
                        [0.1f32.to_bits(), 0.2f32.to_bits(), 0, 0],
                        [rot.to_bits(), 0, 0, 0],
                        [1.1f32.to_bits(), 0.9f32.to_bits(), 0, 0],
                        [shear[0].to_bits(), shear[1].to_bits(), 0, 0],
                        [wr[0], wr[1], 0, 0],
                    ]);
                }
            }
        }
    }
    out
}

const SIZES: [usize; 10] = [1, 7, 8, 9, 511, 512, 513, 1199, 1200, 1201];

fn planes(n: usize, kind: usize, seed: u64, pool: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let mut r = Lcg(seed ^ 0x9e37_79b9_7f4a_7c15);
    let mut one = |scale: f32| -> f32 {
        match kind {
            0 => (r.next() as f32 / 1073741824.0 - 1.0) * scale,
            1 => pool[(r.next() as usize) % pool.len()],
            _ => {
                if r.next() % 12 == 0 {
                    pool[(r.next() as usize) % pool.len()]
                } else {
                    (r.next() as f32 / 1073741824.0 - 1.0) * scale
                }
            }
        }
    };
    let x: Vec<f32> = (0..n).map(|_| one(3.0)).collect();
    let y: Vec<f32> = (0..n).map(|_| one(300.0)).collect();
    (x, y)
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    equal_words: usize,
    nan_words_same: usize,
    mismatches: usize,
    first: Option<String>,
}

fn cpu(lanes: &Lanes, x: &[f32], y: &[f32]) -> Vec<u32> {
    let st = state_of(lanes);
    let n = x.len();
    let (mut a, mut b) = (vec![f32::from_bits(0x7fc0_ffee); n], vec![f32::from_bits(0x7fc0_ffee); n]);
    tilt::render(&st, x, y, &mut a, &mut b, n as i32, false).unwrap();
    a.iter().chain(b.iter()).map(|v| v.to_bits()).collect()
}

/// kind of launch: 0 the kernel, 1 the stub (a kernel that writes its input planes), others by the caller's hook
fn dev(s: &Arc<Session>, lanes: &Lanes, x: &[f32], y: &[f32], stub: bool, hook: &dyn Fn(&mut [u32; 26])) -> Vec<u32> {
    let n = x.len();
    let up = DevImage::upload(s, &[x, y]).unwrap();
    let out = if stub {
        DevImage::upload(s, &[x, y]).unwrap()
    } else {
        let mut p = params_of(&state_of(lanes));
        hook(&mut p);
        tilt_launch(s, up.plane(0), up.plane(1), &p, n).unwrap()
    };
    let d = out.download().unwrap();
    d[0].iter().chain(d[1].iter()).map(|v| v.to_bits()).collect()
}

fn compare(s: &Arc<Session>, stub: bool, stop_at_first: bool, hook: &dyn Fn(&mut [u32; 26])) -> Tally {
    let pool = adversarial_f32(300, 77);
    let mut t = Tally::default();
    for (i, lanes) in sets().iter().enumerate() {
        for rep in 0..2usize {
            let n = SIZES[(i * 3 + rep * 7) % SIZES.len()];
            let kind = (i + rep) % 3;
            let (x, y) = planes(n, kind, (i * 2 + rep) as u64 + 1, &pool);
            let want = cpu(lanes, &x, &y);
            let got = dev(s, lanes, &x, &y, stub, hook);
            t.cases += 1;
            if want == got {
                t.equal_words += want.len();
                t.nan_words_same += want.iter().filter(|w| f32::from_bits(**w).is_nan()).count();
            } else {
                t.mismatches += 1;
                if t.first.is_none() {
                    let k = want.iter().zip(&got).position(|(a, b)| a != b).unwrap_or(0);
                    t.first = Some(format!("set {i} n {n} kind {kind} word {k}: cpu {:08x} dev {:08x} lanes {lanes:x?}", want[k], got[k]));
                }
                if stop_at_first {
                    break;
                }
            }
        }
        if stop_at_first && t.mismatches > 0 {
            break;
        }
    }
    // depth exactly zero: tilt 90 degrees, swivel 0, perspective 1, the x plane at the negated tilt cosine (and neighbours)
    for rot in [0.0f32, 25.0] {
        for sh in [[0.0f32, 0.0], [0.2, -0.1]] {
            for wr in [[0u32, 0u32], [0x3f80_0000, 0x3f80_0000]] {
                let lanes: Lanes = [
                    [0, 90.0f32.to_bits(), 0, 0],
                    [1.0f32.to_bits(), 0, 0, 0],
                    [0, 0, 0, 0],
                    [rot.to_bits(), 0, 0, 0],
                    [1.0f32.to_bits(), 1.0f32.to_bits(), 0, 0],
                    [sh[0].to_bits(), sh[1].to_bits(), 0, 0],
                    [wr[0], wr[1], 0, 0],
                ];
                let c = tilt::setup(&state_of(&lanes)).tilt_cos;
                let mut x = Vec::new();
                let mut y = Vec::new();
                for k in -2i32..=2 {
                    for base in [-c, c] {
                        for yy in [1.0f32, -1.0, 0.0, 2.0] {
                            x.push(f32::from_bits((base.to_bits() as i32 + k) as u32));
                            y.push(yy);
                        }
                    }
                }
                let want = cpu(&lanes, &x, &y);
                let got = dev(s, &lanes, &x, &y, stub, hook);
                t.cases += 1;
                if want == got {
                    t.equal_words += want.len();
                } else {
                    t.mismatches += 1;
                    if t.first.is_none() {
                        t.first = Some(format!("zero depth case rot {rot} shear {sh:?}"));
                    }
                    if stop_at_first {
                        return t;
                    }
                }
            }
        }
    }
    t
}

fn no_hook(_: &mut [u32; 26]) {}

#[test]
fn the_stub_is_refused_and_the_perspective_selfcheck_passes() {
    let Some(ctx) = device() else { return };
    let perspective = DEVICE_OPS.iter().find(|d| d.class == "perspective").unwrap();
    let t = selfcheck(&ctx.session, perspective).expect("the perspective self-check");
    println!("HARNESS control: perspective self-check passed, outputs {} nan_payloads {:?}", t.outputs, t.nan_payloads);
    let stub = compare(&ctx.session, true, true, &no_hook);
    println!("HARNESS stub: mismatches {} first {:?}", stub.mismatches, stub.first);
    assert!(stub.mismatches > 0, "the compare accepted a stub that writes its input");
}

#[test]
fn tilt_equals_the_crate_bit_for_bit() {
    let Some(ctx) = device() else { return };
    let t = compare(&ctx.session, false, false, &no_hook);
    println!("TILT cases {} equal_words {} nan_words_same {} mismatches {}", t.cases, t.equal_words, t.nan_words_same, t.mismatches);
    assert_eq!(t.mismatches, 0, "{:?}", t.first);
}

#[test]
fn the_tilt_selfcheck_reports_nan_payloads() {
    let Some(ctx) = device() else { return };
    let op = DEVICE_OPS.iter().find(|d| d.class == "tilt").unwrap();
    let t = selfcheck(&ctx.session, op).expect("the tilt self-check");
    println!("TILT selfcheck outputs {} NanPayloads {:?}", t.outputs, t.nan_payloads);
    assert!(matches!(t.nan_payloads, fx_device::selfcheck::NanPayloads::Same), "NanPayloads differ: tilt cannot be on NAN_EXACT");
}

/// Single-edit mutants of the kernel source: (name, text in the program, replacement). Each must occur once in the program.
const MUTANTS: &[(&str, &str, &str)] = &[
    ("operand swap in the cross product", "float cross = tl_fnma231(y_scaled_u, xv, axis_v);", "float cross = tl_fnma231(xv, y_scaled_u, axis_v);"),
    ("constant of the lower clamp bound", "0xc04ff33333333333ul", "0xc04ff40000000000ul"),
    ("compare direction of the depth sign", "int pos = (den > 0x0p+0f) ? 1 : 0;", "int pos = (den < 0x0p+0f) ? 1 : 0;"),
    ("contracted multiply-add in the plain rotation", "u = tl_sub(a, b);", "u = fma(t, rot_cos, -b);"),
    ("branch arm of the shear and wrap-y variant", "tilt_v_rot_shear_wrapy(pr, xv, yv, &p, &q);", "tilt_v_rot_shear_wrapx(pr, xv, yv, &p, &q);"),
    ("shear factor of the plain loop", "float k = tl_mul(q, shear_x);", "float k = tl_mul(q, shear_y);"),
    ("sign flip as an or", "return as_float(as_uint(x) ^ 0x80000000u);", "return as_float(as_uint(x) | 0x80000000u);"),
    ("sign of the subtrahend in a negated fused step", "tl_fz(fma(-d, s2, -s3), d, s2, s3)", "tl_fz(fma(-d, s2, s3), d, s2, s3)"),
    ("NaN of the clamp replaced by the default NaN", "return as_float(as_uint(x) | 0x00400000u);\n    }\n    double d = (double)x;", "return as_float(0xffc00000u);\n    }\n    double d = (double)x;"),
    ("value of the zero depth route", "return ca_over_eps;", "return cos_over_persp;"),
];

#[test]
fn every_tilt_mutant_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev_h, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let bin = |s: &Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(Session::open_source(&dev_h, &info, &base, opts).unwrap());
    let base_hash = bin(&clean);
    let t = compare(&clean, false, true, &no_hook);
    assert_eq!(t.mismatches, 0, "the unmutated program must pass: {:?}", t.first);
    let mut killed = 0usize;
    let mut total = 0usize;
    for (name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(Session::open_source(&dev_h, &info, &text, opts).unwrap());
        let h = bin(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        let t = compare(&s, false, true, &no_hook);
        total += 1;
        let k = t.mismatches > 0;
        killed += k as usize;
        println!("MUTANT {name} binary {h:016x}: {}", if k { "KILLED" } else { "SURVIVED" });
    }
    // host mutants: the sincos words passed swapped
    let swap_rot: &dyn Fn(&mut [u32; 26]) = &|p| p.swap(12, 13);
    let swap_tilt: &dyn Fn(&mut [u32; 26]) = &|p| p.swap(14, 15);
    for (name, hook) in [("rotate sin and cos bits swapped", swap_rot), ("tilt sin and cos bits swapped", swap_tilt)] {
        let t = compare(&clean, false, true, hook);
        total += 1;
        let k = t.mismatches > 0;
        killed += k as usize;
        println!("MUTANT {name}: {}", if k { "KILLED" } else { "SURVIVED" });
    }
    println!("MUTANTS killed {killed} of {total}");
    assert_eq!(killed, total, "a tilt mutant survived the compare");
}
