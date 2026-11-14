//! Compare driver of the optical flow polar kernel (`fx_device::kernels_flow_polar`) against `optical_flow_solve::polar`: every output
//! word equal (NaN payloads included) over widths, frame sizes, plane phases and adversarial words with the crate constants and with
//! varied constants, and single edit mutants of the kernel source that each must be refused.
//! Prints NOT RUN without an OpenCL device. Run with --test-threads=1 (platform discovery is not thread safe).

use fx_device::selfcheck::adversarial_f32;
use fx_device::{Backend as Handle, DevImage, DeviceKind, Session};
use fx_exec::device_ops_flow_polar::polar_launch;
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


type Runner = fn(&Arc<Session>, bool) -> Tally;

fn bits_plane(n: usize, r: &mut Lcg) -> Vec<f32> {
    (0..n).map(|_| f32::from_bits(r.next() as u32 ^ ((r.next() as u32) << 17))).collect()
}

/// One comparison; `consts` picks the crate constants (0), varied finite ones (1) or special ones (2).
#[allow(clippy::too_many_arguments)]
fn run_case(s: &Arc<Session>, t: &mut Tally, tag: String, w: i32, stride: i32, rows: i32, row0: i32, ph: u32, kind: usize, seed: u64, consts: usize, stop: bool, pool: &[f32]) -> bool {
    let mut r = Lcg(seed);
    let n = (stride * (rows + row0)) as usize + 5;
    let (p, q) = match kind {
        0 => (plane(n, 0, &mut r, pool, 3.0), plane(n, 0, &mut r, pool, 3.0)),
        1 => (plane(n, 1, &mut r, pool, 1.0), plane(n, 1, &mut r, pool, 1.0)),
        2 => (plane(n, 2, &mut r, pool, 3.0), plane(n, 2, &mut r, pool, 3.0)),
        3 => (plane(n, 0, &mut r, pool, 1.0e19), plane(n, 0, &mut r, pool, 1.0e19)),
        4 => (plane(n, 0, &mut r, pool, 1.0e-19), plane(n, 0, &mut r, pool, 1.0e-19)),
        5 => (bits_plane(n, &mut r), bits_plane(n, &mut r)),
        _ => (plane(n, 2, &mut r, pool, 0.01), plane(n, 2, &mut r, pool, 1.0e30)),
    };
    let mut c = optical_flow_solve::solve::POLAR_CONSTS;
    if consts == 1 {
        c.arg2 = cst(&mut r, pool, 0).abs();
        c.mul_s = cst(&mut r, pool, 0);
        c.mul_v = cst(&mut r, pool, 1);
        c.sub = cst(&mut r, pool, 1);
        c.fin = cst(&mut r, pool, 0);
    } else if consts == 2 {
        c.arg2 = f32::from_bits(SPECIAL[(r.next() as usize) % SPECIAL.len()]);
        c.mul_s = cst(&mut r, pool, 3);
        c.mul_v = cst(&mut r, pool, 3) * 0.5;
        c.thr = f32::from_bits(r.next() as u32 & 0x7f80_0000);
        c.abs_mask = r.next() as u32 | 0x8000_0000 >> (r.next() % 2);
    }
    let mut out = sent(n);
    optical_flow_solve::polar(&p, &q, &mut out, w, stride, row0, rows, ph, &c);
    let (a, b, o) = (up(s, &p), up(s, &q), up(s, &sent(n)));
    let got = polar_launch(s, a.plane(0), b.plane(0), o.plane(0), w, stride, row0, rows, ph, &c).map(|_| down(&o)).map_err(|e| e.to_string());
    t.check(tag, words(&out), got) || !stop
}

fn polar(s: &Arc<Session>, stop: bool) -> Tally {
    let pool = adversarial_f32(200, 11);
    let mut t = Tally::default();
    let mut idx = 0u64;
    for w in [1i32, 2, 3, 7, 8, 9, 15, 16, 17, 33, 50, 64, 1199, 1200, 1201, 1207, 1300] {
        for &ph in &PHASES {
            for kind in 0..7usize {
                idx += 1;
                let consts = if w >= 1199 { 0 } else { (idx % 3) as usize };
                let stride = w + (w % 3);
                let row0 = (idx % 2) as i32;
                let rows = if w >= 1199 { 2 } else { 3 };
                if !run_case(s, &mut t, format!("polar w {w} ph {ph} kind {kind} consts {consts}"), w, stride, rows, row0, ph, kind, idx * 7919 + 1, consts, stop, &pool) {
                    return t;
                }
            }
        }
    }
    for &(w, h) in &SIZES {
        for &ph in &PHASES {
            for kind in 0..7usize {
                idx += 1;
                if !run_case(s, &mut t, format!("polar frame {w}x{h} ph {ph} kind {kind}"), w, w + 2, h, 0, ph, kind, idx * 104729 + 3, (idx % 3) as usize, stop, &pool) {
                    return t;
                }
            }
        }
    }
    t
}

#[test]
fn polar_kernel_equals_its_crate_function_bit_for_bit() {
    let Some(ctx) = device() else { return };
    let t = polar(&ctx.session, false);
    println!("KERNEL polar cases {} equal_words {} nan_cases {} mismatches {} first {:?}", t.cases, t.equal_words, t.nan_cases, t.mismatches, t.first);
    assert_eq!(t.mismatches, 0, "polar mismatches: {:?}", t.first);
}

#[test]
fn polar_refuses_shapes_it_does_not_cover() {
    let Some(ctx) = device() else { return };
    let s = &ctx.session;
    let c = optical_flow_solve::solve::POLAR_CONSTS;
    let a = up(s, &[1.0f32; 64]);
    let o = up(s, &[0.0f32; 64]);
    assert!(polar_launch(s, a.plane(0), a.plane(0), o.plane(0), 8, 0, 0, 2, 0, &c).is_err());
    assert!(polar_launch(s, a.plane(0), a.plane(0), o.plane(0), 8, 8, -1, 2, 0, &c).is_err());
    assert!(polar_launch(s, a.plane(0), a.plane(0), o.plane(0), 8, 8, 0, 9, 0, &c).is_err());
    assert!(polar_launch(s, a.plane(0), a.plane(0), o.plane(0), 0, 8, 0, 2, 0, &c).is_ok());
    assert!(polar_launch(s, a.plane(0), a.plane(0), o.plane(0), 8, 8, 0, 0, 0, &c).is_ok());
}

/// (name, text in the program, replacement); each text occurs once.
const MUTANTS: &[(&str, &str, &str)] = &[
    ("row phase stride of eight bytes", "long ph = ((long)phase + (long)roff * 4) & 31;", "long ph = ((long)phase + (long)roff * 8) & 31;"),
    ("stub: always the scalar arm", "int vec = (all == 0) && ((long)i >= head) && ((long)i < vend);\n    long idx = (long)roff + (long)i;", "int vec = 0;\n    long idx = (long)roff + (long)i;"),
    ("stub: always the vector arm", "int vec = (all == 0) && ((long)i >= head) && ((long)i < vend);\n    long idx = (long)roff + (long)i;", "int vec = 1;\n    long idx = (long)roff + (long)i;"),
    ("vector start compare", "((long)i >= head) && ((long)i < vend);\n    long idx = (long)roff + (long)i;", "((long)i > head) && ((long)i < vend);\n    long idx = (long)roff + (long)i;"),
    ("vector end compare", "((long)i >= head) && ((long)i < vend);\n    long idx = (long)roff + (long)i;", "((long)i >= head) && ((long)i <= vend);\n    long idx = (long)roff + (long)i;"),
    ("vector multiplier is the scalar one", "mul = as_float(mul_v_b);", "mul = as_float(mul_s_b);"),
    ("scalar multiplier is the vector one", "mul = as_float(mul_s_b);", "mul = as_float(mul_v_b);"),
    ("norm_sq operand roles", "float norm_sq = flow_b_f213(qv, qv, p_sq);", "float norm_sq = flow_b_f213(pv, pv, p_sq);"),
    ("norm_sq contracted", "float norm_sq = flow_b_f213(qv, qv, p_sq);", "float norm_sq = qv * qv + p_sq;"),
    ("p square operands", "float p_sq = ofb_mul(pv, pv);", "float p_sq = ofb_mul(pv, 0x1.01p+0f);"),
    ("vector arm without refinement", "norm = flow_c_refine(norm_sq, est, abs_mask, sub_b, fin_b, thr_b);", "norm = flow_b_sqrt_x86(norm_sq);"),
    ("refine mask compare", "uint m = (a >= as_float(thr_b)) ? 0xffffffffu : 0u;", "uint m = (a > as_float(thr_b)) ? 0xffffffffu : 0u;"),
    ("refine abs mask unused", "float a = as_float(as_uint(s) & abs_mask);", "float a = s;"),
    ("refine scaled operand", "float est_sq = ofb_mul(est_scaled, est_masked);", "float est_sq = ofb_mul(est_scaled, est_scaled);"),
    ("refine subtract order", "float est_corr = ofb_sub(est_sq, as_float(sub_b));", "float est_corr = ofb_sub(as_float(sub_b), est_sq);"),
    ("refine final factor", "return ofb_mul(est_prod, as_float(fin_b));", "return ofb_mul(est_prod, as_float(sub_b));"),
    ("reciprocal numerator", "return ofb_div(0x1p+0f, flow_b_sqrt_x86(x));", "return ofb_div(0x1p+1f, flow_b_sqrt_x86(x));"),
    ("reciprocal operand order", "return ofb_div(0x1p+0f, flow_b_sqrt_x86(x));", "return ofb_div(flow_b_sqrt_x86(x), 0x1p+0f);"),
    ("power lane operands swapped", "ofb_powf_lane(kc, tab, as_uint(norm), arg_two_b)", "ofb_powf_lane(kc, tab, arg_two_b, as_uint(norm))"),
    ("exp of the power not the product", "ofb_expf_lane(kc, as_uint(m))", "ofb_expf_lane(kc, as_uint(r))"),
    ("product operands", "float m = ofb_mul(mul, r);", "float m = ofb_mul(r, r);"),
    ("row phase scale", "long ph = ((long)phase + (long)roff * 4) & 31;", "long ph = ((long)phase + (long)roff * 2) & 31;"),
    ("phase argument", "long ph = ((long)phase + (long)roff * 4) & 31;", "long ph = ((long)phase + 4 + (long)roff * 4) & 31;"),
];

#[test]
fn every_polar_mutant_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev_h, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let bin = |s: &Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = Arc::new(Session::open_source(&dev_h, &info, &base, opts).unwrap());
    let base_hash = bin(&clean);
    let mut killed = 0usize;
    let mut seen = Vec::new();
    for (name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = Arc::new(Session::open_source(&dev_h, &info, &text, opts).unwrap());
        let h = bin(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        assert!(!seen.contains(&h), "mutant `{name}` repeats a binary");
        seen.push(h);
        let t = polar(&s, true);
        let verdict = if t.mismatches > 0 { "KILLED" } else { "SURVIVED" };
        if t.mismatches > 0 {
            killed += 1;
        }
        println!("MUTANT polar: {name} binary {h:016x}: {verdict} after {} cases", t.cases);
    }
    println!("MUTANTS killed {killed} of {}", MUTANTS.len());
    assert_eq!(killed, MUTANTS.len(), "a polar mutant survived the compare");
}
