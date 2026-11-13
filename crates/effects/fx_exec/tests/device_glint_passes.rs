//! The glint row and column box passes on the device against `glint_pass_rows::run` and `glint_pass_cols::pass_b::run_columns`,
//! every output word compared (NaN payloads included), the crates' refusals compared by value, a zero-writing stub that the compare
//! must refuse, and single-edit mutants of the kernel source, each of which the compare must refuse. Prints NOT RUN without an OpenCL
//! device (run with --test-threads=1: platform discovery is not thread safe).

use fx_device::cl::Buffer;
use fx_device::selfcheck::adversarial_f32;
use fx_device::{Backend as Handle, DevImage, DeviceKind, Session};
use fx_exec::device_ops_glint_passes::{glint_cols_dev, glint_rows_dev};
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
    cols: bool,
    len: usize,
    lines: usize,
    edge: i32,
    radii: Vec<i32>,
    shift: i32,
    scratch: [u8; 2],
    radii_phase: u8,
    kind: usize,
    seed: u64,
    /// Shrink the source buffer by one word (the ShortBuffer refusal).
    short: bool,
}

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.0 >> 33
    }
}

fn data(c: &Case, pool: &[f32]) -> Vec<f32> {
    let n = c.len * c.lines;
    let mut r = Lcg(c.seed ^ 0x9e37_79b9_7f4a_7c15);
    (0..n)
        .map(|_| match c.kind {
            0 => {
                let m = (r.next() & 0x7f_ffff) as u32;
                let e = 117 + (r.next() % 20) as u32;
                let s = ((r.next() & 1) as u32) << 31;
                f32::from_bits(s | (e << 23) | m)
            }
            1 => pool[(r.next() as usize) % pool.len()],
            _ => {
                if r.next() % 40 == 0 {
                    pool[(r.next() as usize) % pool.len()]
                } else {
                    let m = (r.next() & 0x7f_ffff) as u32;
                    let e = 120 + (r.next() % 12) as u32;
                    let s = ((r.next() & 1) as u32) << 31;
                    f32::from_bits(s | (e << 23) | m)
                }
            }
        })
        .collect()
}

fn cases() -> Vec<Case> {
    let sizes = [1usize, 2, 7, 8, 9, 31, 33, 257, 1199, 1200, 1201];
    let shifts_for = |l: usize| [0i32, 1, -2, 3, l as i32 + 3];
    let scratch_set: [[u8; 2]; 7] = [[0, 0], [4, 8], [28, 12], [1, 2], [16, 0], [8, 28], [20, 4]];
    let rphase = [0u8, 4, 3, 12, 20, 28];
    let mut out = Vec::new();
    let mut idx = 0usize;
    for cols in [false, true] {
        for &l in &sizes {
            let max = ((l - 1) / 2) as i32;
            let cl = |r: i32| r.min(max);
            let mut sets: Vec<Vec<i32>> = vec![
                vec![],
                vec![0],
                vec![-1, cl(2)],
                vec![cl(1)],
                vec![cl(2)],
                vec![cl(1), cl(2)],
                vec![cl(3), cl(1), 0, cl(2)],
                vec![max],
                vec![max, cl(1), max],
                (0..20).map(|i| cl(1 + (i % 5)) * (i % 7 != 3) as i32).collect(),
                (0..60).map(|i| cl(1 + (i * 3 % 7))).collect(),
            ];
            for r in [63, 64, 70, 127] {
                if r <= max {
                    sets.push(vec![r]);
                    sets.push(vec![r, cl(2)]);
                }
            }
            for radii in &sets {
                for edge in [0, 1] {
                    for shift in shifts_for(l) {
                        let lines = if l >= 1199 { [1usize, 2, 3][idx % 3] } else { [1usize, 2, 3, 5, 6][idx % 5] };
                        out.push(Case {
                            cols,
                            len: l,
                            lines,
                            edge,
                            radii: radii.clone(),
                            shift,
                            scratch: scratch_set[idx % scratch_set.len()],
                            radii_phase: rphase[idx % rphase.len()],
                            kind: idx % 3,
                            seed: idx as u64 * 7919 + 13,
                            short: false,
                        });
                        idx += 1;
                    }
                }
            }
            // refusals
            let base = Case { cols, len: l, lines: 2, edge: 0, radii: vec![1], shift: 0, scratch: [0, 0], radii_phase: 0, kind: 0, seed: 5 + l as u64, short: false };
            if l >= 3 {
                out.push(base.clone());
            }
            out.push(Case { edge: 2, radii: vec![0, cl(1).max(1)], ..base.clone() });
            out.push(Case { edge: 2, radii: vec![0, -3], ..base.clone() });
            out.push(Case { radii: vec![1, l as i32], ..base.clone() });
            out.push(Case { radii: vec![l as i32], edge: 5, ..base.clone() });
            out.push(Case { shift: i32::MIN, radii: vec![], ..base.clone() });
            out.push(Case { short: true, radii: vec![], ..base.clone() });
        }
    }
    // Window lengths that leave rest = 8 after the 64-sample blocks (and the peel): every radius to 140 at box sum phases 0 and 4.
    for cols in [false, true] {
        for edge in [0, 1] {
            for phase in [0u8, 4] {
                for r in 1..=140i32 {
                    out.push(Case { cols, len: 301, lines: 1, edge, radii: vec![r], shift: 0, scratch: [0, 0], radii_phase: phase, kind: (r as usize) % 3, seed: 1000 + r as u64 * 31 + phase as u64, short: false });
                }
            }
        }
    }
    out
}

/// The CPU result as words, or the crate's refusal as text.
fn cpu(c: &Case, d: &[f32]) -> Result<Vec<u32>, String> {
    let n = c.len * c.lines;
    let mut dst = vec![f32::from_bits(0x7fc0_1234); n];
    let src: &[f32] = if c.short { &d[..d.len() - 1] } else { d };
    let r = if c.cols {
        let cfg = glint_pass_cols::pass_b::Config { edge: c.edge, radii: &c.radii, shift: c.shift };
        let ph = glint_pass_cols::pass_b::Alignment { scratch: c.scratch, radii: c.radii_phase };
        glint_pass_cols::pass_b::run_columns(&cfg, &ph, src, &mut dst, c.lines, c.len).map_err(|e| format!("{e:?}"))
    } else {
        let cfg = glint_pass_rows::Config { edge: c.edge, radii: &c.radii, shift: c.shift };
        let ph = glint_pass_rows::Alignment { scratch: c.scratch, radii: c.radii_phase };
        glint_pass_rows::run(&cfg, &ph, src, &mut dst, c.len, c.lines).map_err(|e| format!("{e:?}"))
    };
    r.map(|_| dst.iter().map(|v| v.to_bits()).collect())
}

fn dev(s: &Arc<Session>, c: &Case, d: &[f32], stub: bool) -> Result<Vec<u32>, String> {
    let n = c.len * c.lines;
    let src_words: &[f32] = if c.short { &d[..d.len() - 1] } else { d };
    let src = DevImage::upload(s, &[src_words]).map_err(|e| e.to_string())?;
    let sentinel = vec![f32::from_bits(0x7fc0_1234); n];
    let dst = DevImage::upload(s, &[sentinel.as_slice()]).map_err(|e| e.to_string())?;
    let sb: &Buffer<f32> = src.plane(0);
    let db: &Buffer<f32> = dst.plane(0);
    if stub {
        // a stub kernel that writes zeros: the same call shapes, no pass arithmetic
        s.queue.fill(db, 0.0f32).map_err(|e| e.to_string())?;
        return Ok(dst.download().map_err(|e| e.to_string())?[0].iter().map(|v| v.to_bits()).collect());
    }
    let r = if c.cols {
        let cfg = glint_pass_cols::pass_b::Config { edge: c.edge, radii: &c.radii, shift: c.shift };
        let ph = glint_pass_cols::pass_b::Alignment { scratch: c.scratch, radii: c.radii_phase };
        glint_cols_dev(s, &cfg, &ph, sb, db, c.lines, c.len).map_err(|e| e.to_string())?.map_err(|e| format!("{e:?}"))
    } else {
        let cfg = glint_pass_rows::Config { edge: c.edge, radii: &c.radii, shift: c.shift };
        let ph = glint_pass_rows::Alignment { scratch: c.scratch, radii: c.radii_phase };
        glint_rows_dev(s, &cfg, &ph, sb, db, c.len, c.lines).map_err(|e| e.to_string())?.map_err(|e| format!("{e:?}"))
    };
    r?;
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
        let d = data(c, &pool);
        let want = cpu(c, &d);
        let got = dev(s, c, &d, stub);
        t.cases += 1;
        let ok = match (&want, &got) {
            (Ok(a), Ok(b)) => {
                let same = a == b;
                if same {
                    t.equal_words += a.len();
                    if a.iter().any(|w| f32::from_bits(*w).is_nan()) {
                        t.nan_cases_same += 1;
                    }
                }
                same
            }
            (Err(a), Err(b)) => {
                let same = a == b;
                if same {
                    t.refusals_equal += 1;
                }
                same
            }
            _ => false,
        };
        if !ok {
            t.mismatches += 1;
            if t.first_mismatch.is_none() {
                t.first_mismatch = Some(format!("case {i} {c:?}: cpu {:?} dev {:?}", want.as_ref().map(|v| v.len()), got.as_ref().map(|v| v.len())));
            }
            if stop_at_first {
                break;
            }
        }
    }
    t
}

#[test]
fn the_glint_line_passes_equal_the_crates_bit_for_bit() {
    let Some(ctx) = device() else { return };
    let t = compare(&ctx.session, false, false);
    println!("GLINT_PASSES cases {} equal_words {} refusals_equal {} nan_cases_same {} mismatches {}", t.cases, t.equal_words, t.refusals_equal, t.nan_cases_same, t.mismatches);
    println!("NanPayloads::Same on {} cases with a NaN word in the output", t.nan_cases_same);
    assert_eq!(t.mismatches, 0, "{:?}", t.first_mismatch);
}

#[test]
fn a_zero_writing_stub_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let t = compare(&ctx.session, true, true);
    println!("STUB mismatches {} first {:?}", t.mismatches, t.first_mismatch);
    assert!(t.mismatches > 0, "the compare accepted a stub that writes zeros");
}

/// Single-edit mutants of the kernel source: (name, text in the program, replacement). Each must occur once in the program.
const MUTANTS: &[(&str, &str, &str)] = &[
    ("operand swap in the middle delta", "float delta = gp_sub(src[2 * r + 1 + 2 * k], src[2 * k]);", "float delta = gp_sub(src[2 * k], src[2 * r + 1 + 2 * k]);"),
    ("constant 2 of the mirrored start", "gp_fms213(0x1p+1f, s, src[r])", "gp_fms213(0x1p+0f, s, src[r])"),
    ("compare direction of the vector threshold", "(n >= 64 && peel >= 0 && n >= peel + 64)", "(n > 64 && peel >= 0 && n >= peel + 64)"),
    ("contracted multiply-add in the blend", "dst[i] = gp_mul(hv, gp_add(buf[i + m], buf[i - m]));", "dst[i] = fma(hv, buf[i + m], hv * buf[i - m]);"),
    ("sum order of the mode 0 blocks", "float u = gp_add(p, q);", "float u = gp_add(p, r);"),
    ("select arm of the blend buffers", "gp_blend(b0, b1, w, (int)shift);", "gp_blend(b1, b0, w, (int)shift);"),
    ("edge index of the mirrored right edge", "gp_sub(src[w - 1 - 2 * k], src[nmid + 2 * k])", "gp_sub(src[w - 1 - 2 * k], src[nmid + 2 * k + 1])"),
    ("alignment arm of the peel", "return (32 - a) / 4;", "return (31 - a) / 4;"),
    ("edge mode select", "    if (mode == 1) {\n        float s = gp_sum(src, r + 1, align, 1);", "    if (mode == 0) {\n        float s = gp_sum(src, r + 1, align, 1);"),
    ("buffer parity of the next line", "int cur = (int)((j * (size_t)flip) & 1u);", "int cur = (int)(j & 1u);"),
    ("reduction order", "float lo = gp_add(q0, q2);", "float lo = gp_add(q0, q1);"),
    ("tail threshold of the line sum", "if (rest >= 8) {", "if (rest > 8) {"),
    ("fused start without the fusion", "float r = fma(d, s2, -s3);\n    if (r == r) {\n        return r;\n    }\n    if (d != d) {", "float r = d * s2 - s3;\n    if (r == r) {\n        return r;\n    }\n    if (d != d) {"),
];

#[test]
fn every_line_pass_mutant_is_refused_by_the_compare() {
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
    assert_eq!(killed, MUTANTS.len(), "a line pass mutant survived the compare");
}
