//! `xy_to_polar` and `hsv_to_rgb` on the device against the crate functions `xy_to_polar::xy_to_polar` and `hsv_to_rgb::hsv_to_rgb`,
//! every output word compared (NaN payloads included), the data dependent refusal compared (CPU `Atan2General` against the device
//! flag), a zero-writing stub that the compare must refuse, the executor self-check of both ops (`NanPayloads::Same`), a check that the
//! kernel text carries the crate's bit constants, and single-edit mutants of the kernel source, each of which the compare must
//! refuse. Prints NOT RUN without an OpenCL device (run with --test-threads=1: platform discovery is not thread safe).

use fx_device::selfcheck::NanPayloads;
use fx_device::{Backend as Handle, DevImage, DeviceKind, Session};
use fx_exec::device_ops::selfcheck;
use fx_exec::device_ops_retime_polar::{hsv_planes, hsv_pool, hsv_to_rgb_launch, polar_pairs, polar_pairs_raw, polar_pool, xy_to_polar_launch, HSV_TO_RGB, XY_TO_POLAR};
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

struct XyCase {
    xs: Vec<f32>,
    ys: Vec<f32>,
    dir: u32,
    phase: u32,
}

fn xy_cases() -> Vec<XyCase> {
    let pool = polar_pool();
    let mut out = Vec::new();
    let dirs = [0x0000_0000u32, 0x3e80_0000, 0xbf00_0000, 0x8000_0000, 0x3f80_0000, 0x7fc0_0000, 0x3f7f_ffff];
    let mut k = 0u64;
    // small frames and the census frames, filtered pairs (no refusal)
    for &n in &[1usize, 2, 3, 7, 8, 9, 15, 16, 17, 33, 350, 2048] {
        for (i, &dir) in dirs.iter().enumerate() {
            let (xs, ys) = polar_pairs(n, k * 13 + i as u64, &pool);
            out.push(XyCase { xs, ys, dir, phase: 0 });
            k += 1;
        }
    }
    // the body partition: every phase value 0..31 on frames around the 1,200 pixel threshold
    for &n in &[1199usize, 1200, 1201, 1207, 1208, 1300, 1500] {
        for phase in 0..32u32 {
            let (xs, ys) = polar_pairs(n, k * 7 + phase as u64, &pool);
            out.push(XyCase { xs, ys, dir: dirs[(k % 7) as usize], phase });
            k += 1;
        }
    }
    // unfiltered pairs: the refusal path (any lane on the table branch refuses the whole run)
    for &n in &[1usize, 2, 7, 9, 17, 33, 350] {
        for rep in 0..40u64 {
            let (xs, ys) = polar_pairs_raw(n, 1000 + k + rep, &pool);
            out.push(XyCase { xs, ys, dir: dirs[(rep % 7) as usize], phase: 0 });
            k += 1;
        }
    }
    // one refused lane at every position of a 17 lane row
    let refused = {
        let mut r = fx_exec::device_ops_retime_polar::Lcg(99);
        loop {
            let (x, y) = fx_exec::device_ops_retime_polar::gen_pair(&mut r, &pool);
            if xy_to_polar::atan2_lane(y, x, 0).is_err() {
                break (x, y);
            }
        }
    };
    for pos in 0..17usize {
        let (mut xs, mut ys) = polar_pairs(17, 5000 + pos as u64, &pool);
        xs[pos] = refused.0;
        ys[pos] = refused.1;
        out.push(XyCase { xs, ys, dir: 0x3e80_0000, phase: 0 });
    }
    // the thresholds of the scalar atan2: a single lane whose exponent distance is at, below or above -54, 54 and 74, in a filtered row
    let mut bk = 0u64;
    for &e1 in &[1i32, 250, 252, 254] {
        for d in (-56..=-50).chain(52..=56).chain(72..=76) {
            for outer_is_y in [true, false] {
                for m in [0x12_3456u32, 0x7f_ffff] {
                    let e2 = if outer_is_y { e1 - d } else { e1 + d };
                    if !(1..=254).contains(&e2) {
                        continue;
                    }
                    let sg = ((bk & 1) as u32) << 31;
                    let sg2 = (((bk >> 1) & 1) as u32) << 31;
                    let outer = f32::from_bits(sg | ((e1 as u32) << 23) | m);
                    let partner = f32::from_bits(sg2 | ((e2 as u32) << 23) | (m ^ 0x2a_aaaa));
                    let (mut xs, mut ys) = polar_pairs(9, 9000 + bk, &pool);
                    if outer_is_y {
                        xs[4] = partner;
                        ys[4] = outer;
                    } else {
                        xs[4] = outer;
                        ys[4] = partner;
                    }
                    out.push(XyCase { xs, ys, dir: 0x3e80_0000, phase: 0 });
                    bk += 1;
                }
            }
        }
    }
    out
}

/// CPU result as words, or `None` for the crate's refusal.
fn xy_cpu(c: &XyCase) -> Option<(Vec<u32>, Vec<u32>)> {
    let n = c.xs.len();
    let mut ang = vec![0.0f32; n];
    let mut mag = vec![0.0f32; n];
    match xy_to_polar::xy_to_polar(&c.xs, &c.ys, f32::from_bits(c.dir), &mut ang, &mut mag, c.phase as usize) {
        Ok(()) => Some((ang.iter().map(|v| v.to_bits()).collect(), mag.iter().map(|v| v.to_bits()).collect())),
        Err(xy_to_polar::Refusal::Atan2General { .. }) => None,
    }
}

fn xy_dev(s: &Arc<Session>, c: &XyCase, stub: bool) -> Option<(Vec<u32>, Vec<u32>)> {
    let n = c.xs.len();
    let src = DevImage::upload(s, &[c.xs.as_slice(), c.ys.as_slice()]).unwrap();
    if stub {
        let out = DevImage::upload(s, &[vec![0.0f32; n].as_slice(), vec![0.0f32; n].as_slice()]).unwrap();
        let d = out.download().unwrap();
        return Some((d[0].iter().map(|v| v.to_bits()).collect(), d[1].iter().map(|v| v.to_bits()).collect()));
    }
    match xy_to_polar_launch(s, &src, n, c.dir, c.phase) {
        Ok(o) => {
            let d = o.download().unwrap();
            Some((d[0].iter().map(|v| v.to_bits()).collect(), d[1].iter().map(|v| v.to_bits()).collect()))
        }
        Err(fx_device::DevError::Unsupported(w)) if w == fx_exec::device_ops_retime_polar::ATAN2_REFUSAL => None,
        Err(e) => panic!("device error {e}"),
    }
}

#[derive(Default, Debug)]
struct Tally {
    cases: usize,
    words: usize,
    refusals_equal: usize,
    nan_words: usize,
    mismatches: usize,
    first: Option<String>,
}

fn compare_xy(s: &Arc<Session>, stub: bool, stop: bool) -> Tally {
    let mut t = Tally::default();
    for (i, c) in xy_cases().iter().enumerate() {
        let want = xy_cpu(c);
        let got = xy_dev(s, c, stub);
        t.cases += 1;
        let ok = match (&want, &got) {
            (Some(a), Some(b)) => {
                let same = a == b;
                if same {
                    t.words += a.0.len() * 2;
                    t.nan_words += a.0.iter().chain(a.1.iter()).filter(|w| f32::from_bits(**w).is_nan()).count();
                }
                same
            }
            (None, None) => {
                t.refusals_equal += 1;
                true
            }
            _ => false,
        };
        if !ok {
            t.mismatches += 1;
            if t.first.is_none() {
                t.first = Some(format!("xy case {i} n {} dir {:08x} phase {} cpu_refused {} dev_refused {}", c.xs.len(), c.dir, c.phase, want.is_none(), got.is_none()));
            }
            if stop {
                break;
            }
        }
    }
    t
}

fn hsv_cases() -> Vec<Vec<Vec<f32>>> {
    let pool = hsv_pool();
    let mut out = Vec::new();
    for (k, &n) in [1usize, 2, 3, 7, 8, 9, 15, 16, 17, 33, 350, 2048].iter().enumerate() {
        for rep in 0..6 {
            out.push(hsv_planes(n, (k * 6 + rep) as u64, &pool));
        }
    }
    // the grid: every hue against every saturation against a value subset
    let hv: Vec<f32> = pool.clone();
    let sv: Vec<f32> = pool.iter().step_by(7).copied().collect();
    let vv: Vec<f32> = pool.iter().step_by(23).copied().collect();
    let (mut h, mut s, mut v) = (Vec::new(), Vec::new(), Vec::new());
    for &a in &hv {
        for &b in &sv {
            for &c in vv.iter().step_by(3) {
                h.push(a);
                s.push(b);
                v.push(c);
            }
        }
    }
    out.push(vec![h, s, v]);
    out
}

fn compare_hsv(s: &Arc<Session>, stub: bool, stop: bool) -> Tally {
    let mut t = Tally::default();
    for (i, planes) in hsv_cases().iter().enumerate() {
        let n = planes[0].len();
        let want = hsv_to_rgb::hsv_to_rgb(&planes[0], &planes[1], &planes[2]).unwrap();
        let src = DevImage::upload(s, &[planes[0].as_slice(), planes[1].as_slice(), planes[2].as_slice()]).unwrap();
        let got: Vec<Vec<f32>> = if stub {
            vec![vec![0.0; n]; 3]
        } else {
            hsv_to_rgb_launch(s, &src, n).unwrap().download().unwrap()
        };
        t.cases += 1;
        let same = (0..3).all(|k| want[k].iter().zip(&got[k]).all(|(a, b)| a.to_bits() == b.to_bits()));
        if same {
            t.words += 3 * n;
            t.nan_words += want.iter().flatten().filter(|w| w.is_nan()).count();
        } else {
            t.mismatches += 1;
            if t.first.is_none() {
                t.first = Some(format!("hsv case {i} n {n}"));
            }
            if stop {
                break;
            }
        }
    }
    t
}

#[test]
fn xy_to_polar_equals_the_crate_bit_for_bit() {
    let Some(ctx) = device() else { return };
    let t = compare_xy(&ctx.session, false, false);
    println!("XY_TO_POLAR cases {} equal_words {} refusals_equal {} nan_words {} mismatches {}", t.cases, t.words, t.refusals_equal, t.nan_words, t.mismatches);
    assert_eq!(t.mismatches, 0, "{:?}", t.first);
    assert!(t.refusals_equal >= 100, "the refusal path must be shown on the corpus");
}

#[test]
fn hsv_to_rgb_equals_the_crate_bit_for_bit() {
    let Some(ctx) = device() else { return };
    let t = compare_hsv(&ctx.session, false, false);
    println!("HSV_TO_RGB cases {} equal_words {} nan_words {} mismatches {}", t.cases, t.words, t.nan_words, t.mismatches);
    assert_eq!(t.mismatches, 0, "{:?}", t.first);
}

#[test]
fn zero_writing_stubs_are_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let a = compare_xy(&ctx.session, true, true);
    let b = compare_hsv(&ctx.session, true, true);
    println!("STUB xy mismatches {} first {:?}; hsv mismatches {} first {:?}", a.mismatches, a.first, b.mismatches, b.first);
    assert!(a.mismatches > 0 && b.mismatches > 0, "the compare accepted a stub that writes zeros");
}

#[test]
fn the_executor_self_check_of_both_ops_is_same() {
    let Some(ctx) = device() else { return };
    for op in [&XY_TO_POLAR, &HSV_TO_RGB] {
        let t = selfcheck(&ctx.session, op).unwrap_or_else(|e| panic!("{e}"));
        println!("SELFCHECK {} outputs {} nan_payloads {:?}", op.class, t.outputs, matches!(t.nan_payloads, NanPayloads::Same));
        assert!(matches!(t.nan_payloads, NanPayloads::Same), "{} NaN payloads differ", op.class);
    }
}

/// Hex digits after the first `0x` of `line`, underscores removed, lower case.
fn hex_of(line: &str) -> Option<String> {
    let p = line.find("0x")?;
    let h: String = line[p + 2..].chars().take_while(|c| c.is_ascii_hexdigit() || *c == '_').filter(|c| *c != '_').collect();
    Some(h.to_ascii_lowercase())
}

#[test]
fn the_kernel_text_carries_the_crates_bit_constants() {
    let src = include_str!("../../kernels/xy_to_polar/src/lib.rs");
    let kernel = fx_device::kernels_retime_polar::RETIME_POLAR;
    let mut seen = 0;
    for line in src.lines() {
        let l = line.trim_start();
        let named = ["INV_TWO_PI", "PI_HALF_F", "PI_F", "P_A", "P_B", "P_C", "P_D", "P_E", "P_F", "P_G", "P_H", "RANGE_BIAS", "QUARTER_PI_HI", "QUARTER_PI_LO", "HALF_PI_HI", "HALF_PI_LO", "PI_HI", "PI_LO", "THREE_QUARTER_PI_HI", "THREE_QUARTER_PI_LO"];
        if let Some(name) = named.iter().find(|n| l.starts_with(&format!("const {n}:")) || l.starts_with(&format!("pub const {n}:"))) {
            let h = hex_of(l).unwrap_or_else(|| panic!("{name}: no hex"));
            assert!(kernel.contains(&h), "{name}: bits {h} are not in the kernel text");
            seen += 1;
        }
    }
    assert_eq!(seen, 20, "every named constant of the crate was checked");
}

const XY_MUTANTS: &[(&str, &str, &str)] = &[
    ("operand swap of the denominator", "float den = lt ? abs_x : ay;", "float den = lt ? ay : abs_x;"),
    ("constant of the pi offset", "float pi_offset = le ? as_float(0x40490fdbu) : 0x0p+0f;", "float pi_offset = le ? as_float(0x40490fdau) : 0x0p+0f;"),
    ("compare direction of the octant test", "int lt = ay < abs_x;", "int lt = ay <= abs_x;"),
    ("unfused final polynomial step", "poly_hi = fma(poly_hi, z, poly_lo);", "poly_hi = poly_hi * z + poly_lo;"),
    ("constant of the large exponent branch", "double res = (as_double(0x3c91a62633145c07UL) - xd) + as_double(0x3ff921fb54442d18UL);", "double res = (as_double(0x3c91a62633145c07UL) - xd) + as_double(0x3ff921fc54442d18UL);"),
    ("select arm of the quadrant offset", "float quad_offset = lt ? 0x0p+0f : as_float(0x3fc90fdbu);", "float quad_offset = lt ? as_float(0x3fc90fdbu) : 0x0p+0f;"),
    ("small exponent threshold", "if (d <= -54) {", "if (d <= -53) {"),
    ("large exponent threshold", "if (d >= 54) {", "if (d >= 55) {"),
    ("constant of the vector magnitude step", "float v = rtp_sub(u, 0x1.8p+1f);", "float v = rtp_sub(u, 0x1.7ffffep+1f);"),
    ("lower edge of the vector body", "(i >= (size_t)lo && i < (size_t)hi)", "(i > (size_t)lo && i < (size_t)hi)"),
    ("upper edge of the vector body", "(i >= (size_t)lo && i < (size_t)hi)", "(i >= (size_t)lo && i <= (size_t)hi)"),
    ("fused site of the angle", "float v = rtp_fms213(a, as_float(0x3e22f983u), as_float(dirbits));", "float v = rtp_fma213(a, as_float(0x3e22f983u), as_float(dirbits));"),
    ("operand swap of the angle reduction", "ang[i] = rtp_sub(v, fl);", "ang[i] = rtp_sub(fl, v);"),
    ("NaN pick order of the add", "float r = x + y;\n    return isnan(r) ? rtp_pick2(x, y) : r;", "float r = x + y;\n    return isnan(r) ? rtp_pick2(y, x) : r;"),
    ("sign of the pair form", "(neg ? 0x8000000000000000UL : 0UL)", "(neg ? 0UL : 0UL)"),
    ("exponent shift", "(as_ulong((double)v) >> 52)", "(as_ulong((double)v) >> 51)"),
    ("range test bias", "(as_uint(v) & 0x7fffffffu) + 0x7f000000u", "(as_uint(v) & 0x7fffffffu) + 0x7f000001u"),
];

const HSV_MUTANTS: &[(&str, &str, &str)] = &[
    ("operand swap of the chroma difference", "float q = rtp_sub(v, p);", "float q = rtp_sub(p, v);"),
    ("negation of a negative saturation", "float neg_s = as_float(as_uint(s) ^ 0x80000000u);", "float neg_s = s;"),
    ("constant of the half turn", "float hh = rtp_add(0x1p-1f, h);", "float hh = rtp_add(0x1p-2f, h);"),
    ("unfused negated multiply add", "float r = fma(-d, s2, s3);\n    return isnan(r) ? rtp_pick3", "float r = s3 - d * s2;\n    return isnan(r) ? rtp_pick3"),
    ("select arm of the zero saturation", "o[2] = vb;\n    } else if (0x0p+0f > s) {", "o[2] = as_uint(0x0p+0f);\n    } else if (0x0p+0f > s) {"),
    ("compare direction of the sign test", "} else if (0x0p+0f > s) {", "} else if (s > 0x0p+0f) {"),
    ("sextant boundary of the switch", "if (idx < 3) {", "if (idx <= 3) {"),
    ("compare direction of the wrap", "if (t >= 0x1.8p+2f) {", "if (t > 0x1.8p+2f) {"),
    ("operand swap of the complement", "float w = rtp_sub(0x1p+0f, f);\n            out[0] = vb;", "float w = rtp_sub(f, 0x1p+0f);\n            out[0] = vb;"),
    ("NaN pick order of the fused site", "float r = fma(-d, s2, s3);\n    return isnan(r) ? rtp_pick3(d, s2, s3) : r;", "float r = fma(-d, s2, s3);\n    return isnan(r) ? rtp_pick3(s3, s2, d) : r;"),
    ("addend of the sextant three green", "out[1] = as_uint(rtp_fnma213(p, f, v));\n        out[2] = vb;", "out[1] = as_uint(rtp_fnma213(p, f, q));\n        out[2] = vb;"),
    ("scale of the sextant", "float t = rtp_mul(0x1.8p+2f, frac);", "float t = rtp_mul(0x1.7ffffep+2f, frac);"),
];

#[test]
fn every_mutant_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev_h, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let bin = |s: &Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(Session::open_source(&dev_h, &info, &base, opts).unwrap());
    let base_hash = bin(&clean);
    assert_eq!(compare_xy(&clean, false, true).mismatches, 0, "the unmutated program must pass the xy compare");
    assert_eq!(compare_hsv(&clean, false, true).mismatches, 0, "the unmutated program must pass the hsv compare");
    let mut killed = 0usize;
    let all: Vec<(&str, &str, &str, bool)> = XY_MUTANTS.iter().map(|m| (m.0, m.1, m.2, true)).chain(HSV_MUTANTS.iter().map(|m| (m.0, m.1, m.2, false))).collect();
    for (name, old, new, is_xy) in &all {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(Session::open_source(&dev_h, &info, &text, opts).unwrap());
        let h = bin(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        let t = if *is_xy { compare_xy(&s, false, true) } else { compare_hsv(&s, false, true) };
        let verdict = if t.mismatches > 0 { "KILLED" } else { "SURVIVED" };
        if t.mismatches > 0 {
            killed += 1;
        }
        println!("MUTANT {} {name} binary {h:016x}: {verdict}", if *is_xy { "xy" } else { "hsv" });
    }
    println!("MUTANTS killed {killed} of {}", all.len());
    assert_eq!(killed, all.len(), "a mutant survived the compare");
}
