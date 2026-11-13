//! The tilt_alpha device kernel against `tilt_alpha::tilt_alpha` per row, every output word compared (NaN payloads included), the
//! host-branch refusal compared by value, a stub kernel that always takes the scalar form (S8) which the compare must refuse, and
//! single-edit mutants of the kernel source, each of which the compare must refuse. Prints NOT RUN without an OpenCL device (run with
//! --test-threads=1: platform discovery is not thread safe).

use fx_device::cl::Buffer;
use fx_device::selfcheck::adversarial_f32;
use fx_device::{Backend as Handle, DevImage, DeviceKind, Session};
use fx_exec::device_ops_tilt_alpha::{host_words, tilt_alpha_launch};
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

#[derive(Clone, Debug)]
struct Case {
    st: tilt_alpha::State,
    host: tilt_alpha::Host,
    w: usize,
    h: usize,
    base: usize,
    rowbuf: bool,
    kind: usize,
    seed: u64,
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

fn rows(c: &Case) -> Vec<(usize, usize, usize)> {
    (0..c.h).map(|y| (y * c.w, c.w, if c.rowbuf { c.base & 0x1f } else { (c.base + y * c.w * 4) & 0x1f })).collect()
}

fn planes(c: &Case, pool: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let n = c.w * c.h;
    let mut r = Lcg(c.seed ^ 0x9e37_79b9_7f4a_7c15);
    let special = [0x7fc0_0000u32, 0xffc0_0000, 0x7fa0_0001, 0xffa0_0001, 0x8000_0000, 0x0000_0001, 0x8000_0001, 0x7f80_0000, 0xff80_0000, 0x007f_ffff];
    let mut gen = |r: &mut Lcg| -> f32 {
        match c.kind {
            0 => f32::from_bits(((r.next() & 1) as u32) << 31 | (117 + (r.next() % 20) as u32) << 23 | (r.next() & 0x7f_ffff) as u32),
            1 => pool[(r.next() as usize) % pool.len()],
            _ => {
                let q = r.next() % 24;
                if q == 0 {
                    f32::from_bits(special[(r.next() as usize) % special.len()])
                } else if q == 1 {
                    pool[(r.next() as usize) % pool.len()]
                } else {
                    ((r.next() % 4000) as f32 - 2000.0) * 0.0007
                }
            }
        }
    };
    let a: Vec<f32> = (0..n).map(|_| gen(&mut r)).collect();
    let b: Vec<f32> = (0..n).map(|_| gen(&mut r)).collect();
    (a, b)
}

fn param_sets() -> Vec<(tilt_alpha::State, tilt_alpha::Host)> {
    let h = |w: i32, hh: i32, d: f64| tilt_alpha::Host { w, h: hh, d };
    let s = |swivel: f32, tilt: f32, amount: f32| tilt_alpha::State { swivel, tilt, amount };
    let mut v = vec![
        (s(15.0, 20.0, 1.0), h(64, 1, 1.0)),
        (s(20.0, 35.0, 1.5), h(640, 480, 2.0)),
        (s(-30.0, 10.0, 0.75), h(1920, 1080, 1.0)),
        (s(0.0, 0.0, 1.0), h(64, 1, 1.0)),
        (s(0.0, 60.0, 3.0), h(100, 7, 0.5)),
        (s(90.0, -45.0, -1.0), h(64, 1, 1.0)),
        (s(-0.0, 0.0, 2.0), h(33, 3, 1.5)),
        (s(1.0e-3, 2.0e-3, 0.25), h(64, 1, 1.0)),
        (s(45.0, 45.0, 0.1), h(64, 1, 1.0)),
        (s(10.0, 5.0, 1.0e6), h(64, 1, 1.0)),
    ];
    // around the S4 window edge cos(angle) / amount = +-1e-4: a swivel 0 / tilt 0 (angle 0, cos 1) with amount near 1e4 on both sides
    for k in [-64i32, -8, -1, 1, 8, 64] {
        let amt = f32::from_bits((10000.0f32).to_bits().wrapping_add(k as u32));
        v.push((s(0.0, 0.0, amt), h(64, 1, 1.0)));
        v.push((s(0.0, 0.0, -amt), h(64, 1, 1.0)));
    }
    v
}

fn cases() -> Vec<Case> {
    let sizes = [1usize, 7, 8, 9, 15, 16, 1199, 1200, 1201, 1207, 4100];
    let mut out = Vec::new();
    let mut idx = 0usize;
    for (st, host) in param_sets() {
        for &w in &sizes {
            for ph in 0..8usize {
                for rowbuf in [false, true] {
                    // the large rows are costly: every phase only on the first few sets
                    if w >= 1199 && (idx % 3) == 1 && ph % 2 == 1 {
                        idx += 1;
                        continue;
                    }
                    out.push(Case { st, host, w, h: 2, base: ph * 4, rowbuf, kind: idx % 3, seed: idx as u64 * 7919 + 13 });
                    idx += 1;
                }
            }
        }
    }
    out
}

/// The CPU result as words (or the refusal text of the host branch).
fn cpu(c: &Case, a: &[f32], b: &[f32]) -> Result<Vec<u32>, String> {
    let n = c.w * c.h;
    let mut dst = vec![f32::from_bits(0x7fc0_1234); n];
    for (lo, cnt, ph) in rows(c) {
        let o = tilt_alpha::tilt_alpha(&c.st, &c.host, false, cnt as i32, &a[lo..lo + cnt], &b[lo..lo + cnt], &mut dst[lo..lo + cnt], ph).map_err(|e| format!("{e:?}"))?;
        if matches!(o.route, tilt_alpha::Route::HostBranch | tilt_alpha::Route::Flag) {
            return Err("refused".into());
        }
    }
    Ok(dst.iter().map(|v| v.to_bits()).collect())
}

fn dev(s: &Arc<Session>, c: &Case, a: &[f32], b: &[f32], stub: bool) -> Result<Vec<u32>, String> {
    let n = c.w * c.h;
    let words = host_words(&c.st, &c.host).map_err(|_| "refused".to_string())?;
    let src = DevImage::upload(s, &[a, b]).map_err(|e| e.to_string())?;
    let sentinel = vec![f32::from_bits(0x7fc0_1234); n];
    let dst = DevImage::upload(s, &[sentinel.as_slice()]).map_err(|e| e.to_string())?;
    let (pa, pb): (&Buffer<f32>, &Buffer<f32>) = (src.plane(0), src.plane(1));
    tilt_alpha_launch(s, pa, pb, dst.plane(0), &words, &rows(c), n, stub).map_err(|e| e.to_string())?;
    Ok(dst.download().map_err(|e| e.to_string())?[0].iter().map(|v| v.to_bits()).collect())
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    equal_words: usize,
    refusals_equal: usize,
    nan_cases_same: usize,
    first_mismatch: Option<String>,
    mismatches: usize,
}

fn compare(s: &Arc<Session>, stub: bool, stop_at_first: bool) -> Tally {
    let pool = adversarial_f32(200, 77);
    let mut t = Tally::default();
    for (i, c) in cases().iter().enumerate() {
        let (a, b) = planes(c, &pool);
        let want = cpu(c, &a, &b);
        let got = dev(s, c, &a, &b, stub);
        t.cases += 1;
        let ok = match (&want, &got) {
            (Ok(x), Ok(y)) => {
                let same = x == y;
                if same {
                    t.equal_words += x.len();
                    if x.iter().any(|w| f32::from_bits(*w).is_nan()) {
                        t.nan_cases_same += 1;
                    }
                }
                same
            }
            (Err(_), Err(_)) => {
                t.refusals_equal += 1;
                true
            }
            _ => false,
        };
        if !ok {
            t.mismatches += 1;
            if t.first_mismatch.is_none() {
                t.first_mismatch = Some(format!("case {i} {c:?}: cpu ok {} dev ok {}", want.is_ok(), got.is_ok()));
            }
            if stop_at_first {
                break;
            }
        }
    }
    t
}

#[test]
fn the_tilt_alpha_kernel_equals_the_crate_bit_for_bit() {
    let Some(ctx) = device() else { return };
    let t = compare(&ctx.session, false, false);
    println!("TILT_ALPHA cases {} equal_words {} refusals_equal {} nan_cases_same {} mismatches {}", t.cases, t.equal_words, t.refusals_equal, t.nan_cases_same, t.mismatches);
    println!("NanPayloads::Same on {} cases with a NaN word in the output", t.nan_cases_same);
    assert!(t.refusals_equal > 0 && t.equal_words > 0, "the corpus must reach both the refusal and the ramp");
    assert_eq!(t.mismatches, 0, "{:?}", t.first_mismatch);
}

#[test]
fn a_stub_that_always_takes_the_scalar_form_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let t = compare(&ctx.session, true, true);
    println!("STUB mismatches {} first {:?}", t.mismatches, t.first_mismatch);
    assert!(t.mismatches > 0, "the compare accepted a stub that always takes S8");
    // the first refused case must be a row of at least 8 elements (a vector element exists)
    let first = t.first_mismatch.unwrap();
    assert!(!first.contains("w: 1,") && !first.contains("w: 7,"), "{first}");
}

/// Single-edit mutants of the kernel source: (name, text in the program, replacement). Each must occur once in the program.
const MUTANTS: &[(&str, &str, &str)] = &[
    ("S8 and S9 swapped", "if (j >= peel && j < vend) {", "if (!(j >= peel && j < vend)) {"),
    ("peel count off by one", "if (j >= peel && j < vend) {", "if (j > peel && j < vend) {"),
    ("vend bound off by one", "if (j >= peel && j < vend) {", "if (j >= peel && j <= vend) {"),
    ("fma operand role in S9", "fma(nksw, av, t)", "fma(nksw, bv, t)"),
    ("fma operand role in S8", "fma(knt, bv, -t)", "fma(knt, bv, t)"),
    ("clamp constant", "if (0x1p+0f < t) {\n        return 0x1p+0f;", "if (0x1p+0f < t) {\n        return 0x1.0000020p+0f;"),
    ("below zero compare direction", "if (0x0p+0f > t) {", "if (0x0p+0f < t) {"),
    ("sign of the host scale", "t = ta_fz(fma(t, nsin, -cos_ang), t, nsin, cos_ang);\n        o = ta_clamp(nanx_mul(ls, t));", "t = ta_fz(fma(t, nsin, -cos_ang), t, nsin, cos_ang);\n        o = ta_clamp(nanx_mul(-ls, t));"),
    ("contracted multiply-add in S8", "fma(-t, ca, -cb)", "(-t * ca - cb)"),
    ("scalar operand order", "float t = nanx_mul(ksw, av);", "float t = nanx_mul(ksw, bv);"),
];

#[test]
fn every_tilt_alpha_mutant_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev_h, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let bin = |s: &Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(Session::open_source(&dev_h, &info, &base, opts).unwrap());
    let base_hash = bin(&clean);
    let t = compare(&clean, false, true);
    assert_eq!(t.mismatches, 0, "the unmutated program must pass: {:?}", t.first_mismatch);
    let mut killed = 0usize;
    for (name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(Session::open_source(&dev_h, &info, &text, opts).unwrap());
        let h = bin(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        let t = compare(&s, false, true);
        let verdict = if t.mismatches > 0 { "KILLED" } else { "SURVIVED" };
        if t.mismatches > 0 {
            killed += 1;
        }
        println!("MUTANT {name} binary {h:016x}: {verdict}");
    }
    println!("MUTANTS killed {killed} of {}", MUTANTS.len());
    assert_eq!(killed, MUTANTS.len(), "a tilt_alpha mutant survived the compare");
}
