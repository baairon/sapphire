//! Self-check of the glow device kernel against the CPU kernel, bit for bit: the full op (bright pass and blur passes), the bright
//! pass alone (threshold display on), a call the device declines, and single-edit mutants of the bright pass source, each of which
//! the self-check must refuse. Prints NOT RUN without an OpenCL device (run with --test-threads=1: platform discovery is not
//! thread safe).

use fx_device::selfcheck::NanPayloads;
use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend, DeviceBackend};
use fx_exec::device_ops_glow::{bright_corpus, ALLOW, GLOW};

/// A test that needs a device and found none: it prints NOT RUN, or fails when `SAPPHIRE_REQUIRE_DEVICE=1`.
fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}

fn device() -> Option<std::sync::Arc<fx_device::DeviceCtx>> {
    match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => Some(d),
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            None
        }
    }
}

#[test]
fn the_glow_kernel_passes_the_self_check_against_the_cpu() {
    let Some(ctx) = device() else { return };
    let tally = fx_exec::device_ops::selfcheck(&ctx.session, &GLOW).expect("device equals the CPU");
    println!("GLOW cases {} outputs {} payloads {}", (GLOW.corpus)().len(), tally.outputs, tally);
    assert!(matches!(tally.nan_payloads, NanPayloads::Same), "NaN payloads differ: {}", tally);
    let b = DeviceBackend::with_allowlist(ctx, ALLOW);
    let rep = b.report();
    assert!(rep.refused.is_empty(), "{:?}", rep.refused);
    assert_eq!(b.admitted(), vec!["glow"]);
}

#[test]
fn the_glow_bright_pass_passes_the_self_check_against_the_cpu() {
    let Some(ctx) = device() else { return };
    let mut op = GLOW;
    op.corpus = bright_corpus;
    let tally = fx_exec::device_ops::selfcheck(&ctx.session, &op).expect("device equals the CPU");
    println!("BRIGHT cases {} outputs {} payloads {}", bright_corpus().len(), tally.outputs, tally);
    assert!(matches!(tally.nan_payloads, NanPayloads::Same), "NaN payloads differ: {}", tally);
}

#[test]
fn an_edge_mode_the_cpu_refuses_is_declined_on_the_device() {
    use fx_exec::backend::NodeCall;
    use fx_exec::device_ops::DevArg;
    use fx_exec::types::{Arg, Image, KParams};
    let Some(ctx) = device() else { return };
    let (w, h) = (16usize, 16usize);
    let n = w * h;
    let src = Image { n, planes: (0..3).map(|p| (0..n).map(|i| ((i * 7 + p * 3) % 17) as f32 / 16.0).collect()).collect() };
    let c = |v: &[f32]| -> (usize, [u32; 4]) {
        let last = *v.last().unwrap();
        (v.len(), core::array::from_fn(|i| v.get(i).copied().unwrap_or(last).to_bits()))
    };
    let consts = [c(&[0.5]), c(&[1.0, 1.0]), c(&[0.3]), c(&[1.0]), c(&[1.0]), c(&[2.0]), c(&[0.0])];
    let kp = KParams::standin();
    let mut cpu: Vec<Arg> = vec![Arg::Img(&src)];
    cpu.extend(consts.iter().map(|&(n, lanes)| Arg::Const { n, lanes }));
    assert!(fx_exec::kernels::run_any("glow", 3, &cpu, n, (w, h), &kp).is_err(), "the CPU refuses edge mode 2");
    let up = fx_device::DevImage::upload(&ctx.session, &src.planes.iter().map(|p| p.as_slice()).collect::<Vec<_>>()).unwrap();
    let mut dev: Vec<DevArg> = vec![DevArg::Img(&up)];
    dev.extend(consts.iter().map(|&(n, lanes)| DevArg::Const { n, lanes }));
    let call = NodeCall { idx: 0, class: "glow", tag: 3, n, dims: (w, h), kp: &kp };
    match (GLOW.run)(&ctx.session, &call, &dev) {
        Err(fx_device::DevError::Unsupported(why)) => println!("DECLINED: {why}"),
        other => panic!("expected a decline, got {other:?}"),
    }
}

/// Single-edit mutants of the bright pass source: (name, text in the program, replacement). Each must occur once in the program.
const MUTANTS: &[(&str, &str, &str)] = &[
    ("compare direction r > t0", "float e = (r > t0) ? nanx_sub(r, t0) : 0x0p+0f;", "float e = (r >= t0) ? nanx_sub(r, t0) : 0x0p+0f;"),
    ("NaN form of the e test", "if (!(e > 0x0p+0f)) {", "if (e < 0x0p+0f) {"),
    ("select arm of maxss(r, 0)", "float mr = (r > 0x0p+0f) ? r : 0x0p+0f;", "float mr = (r < 0x0p+0f) ? 0x0p+0f : r;"),
    ("reciprocal multiply for the division", "float f = nanx_div(tt, s);", "float f = nanx_mul(tt, nanx_div(0x1p+0f, s));"),
    ("compare direction s > tt", "if (s > tt) {", "if (s >= tt) {"),
    ("dropped g guard", "    if (g > t1) {\n        e = nanx_add(e, nanx_sub(g, t1));\n    }", "    e = nanx_add(e, nanx_sub(g, t1));"),
    ("operand swap in the excess", "float e = (r > t0) ? nanx_sub(r, t0) : 0x0p+0f;", "float e = (r > t0) ? nanx_sub(t0, r) : 0x0p+0f;"),
    ("operand swap in the green excess", "e = nanx_add(e, nanx_sub(g, t1));", "e = nanx_add(e, nanx_sub(t1, g));"),
    ("contracted multiply-add in the one plane scale", "float d = nanx_mul(k, nanx_sub(x, t));", "float d = fma(k, x, -(k * t));"),
    ("sum order of the colour sum", "float s = (b > 0x0p+0f) ? nanx_add(b, s1) : s1;", "float s = (b > 0x0p+0f) ? nanx_add(nanx_add(b, g), mr) : s1;"),
    ("constant in the one plane zero test", "if (t >= x || 0x0p+0f >= x) {", "if (t >= x || 0x1p-7f >= x) {"),
    ("select arm of the one plane result", "p[i] = (x > d) ? d : x;", "p[i] = (x > d) ? x : d;"),
];

#[test]
fn every_bright_pass_mutant_is_refused_by_the_self_check() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let opts = fx_device::kernels::BUILD_OPTIONS;
    let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
    let binary_hash = |s: &fx_device::Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = std::sync::Arc::new(fx_device::Session::open_source(&dev, &info, &base, opts).unwrap());
    let base_hash = binary_hash(&clean);
    let mut op = GLOW;
    op.corpus = bright_corpus;
    let t = fx_exec::device_ops::selfcheck(&clean, &op).expect("the unmutated program passes");
    assert!(matches!(t.nan_payloads, NanPayloads::Same));
    println!("MUTANT base binary {base_hash:016x}");
    let mut prev = base_hash;
    let mut killed = 0usize;
    for (name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let text = base.replacen(old, new, 1);
        let s = std::sync::Arc::new(fx_device::Session::open_source(&dev, &info, &text, opts).unwrap());
        let h = binary_hash(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        assert_ne!(h, prev, "mutant `{name}` built the previous mutant's binary");
        prev = h;
        let verdict = match fx_exec::device_ops::selfcheck(&s, &op) {
            Err(m) => format!("KILLED (mismatch: {})", m.lines().next().unwrap_or("")),
            Ok(t) => match t.nan_payloads {
                NanPayloads::Differ(m) => format!("KILLED (NaN payload: {m})"),
                NanPayloads::Same => "SURVIVED".to_string(),
            },
        };
        if verdict.starts_with("KILLED") {
            killed += 1;
        }
        println!("MUTANT {name} binary {h:016x}: {verdict}");
    }
    println!("MUTANTS killed {killed} of {}", MUTANTS.len());
    assert_eq!(killed, MUTANTS.len(), "a bright pass mutant survived the self-check");
}
