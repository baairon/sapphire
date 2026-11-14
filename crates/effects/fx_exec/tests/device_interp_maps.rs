//! Compare driver of the optical_flow_interp_maps device kernels against the crate's `native::interp_maps` (through the executor's
//! CPU kernel), bit for bit: the self-check corpus of `device_ops_interp_maps`, the refusals of `accepts`, a deliberately wrong
//! stub kernel (the harness check) and single-edit mutants of the kernel source, each of which the compare must refuse.
//! Prints NOT RUN without an OpenCL device (run with --test-threads=1).

use fx_device::selfcheck::NanPayloads;
use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend, DeviceBackend, NodeCall};
use fx_exec::device_ops::ArgShape;
use fx_exec::device_ops_interp_maps::{ALLOW, INTERP_MAPS};
use fx_exec::types::KParams;

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
fn the_interp_maps_kernels_equal_the_crate_on_the_corpus() {
    let Some(ctx) = device() else { return };
    let corpus = (INTERP_MAPS.corpus)();
    let tally = fx_exec::device_ops::selfcheck(&ctx.session, &INTERP_MAPS).expect("device equals the CPU");
    println!("INTERP_MAPS cases {} outputs {} payloads {}", corpus.len(), tally.outputs, tally);
    assert!(matches!(tally.nan_payloads, NanPayloads::Same), "NaN payloads differ: {}", tally);
    let b = DeviceBackend::with_allowlist(ctx, ALLOW);
    let rep = b.report();
    assert!(rep.refused.is_empty(), "{:?}", rep.refused);
    assert_eq!(b.admitted(), vec!["optical_flow_interp_maps"]);
}

#[test]
fn accepts_refuses_what_the_cpu_refuses_by_name() {
    let kp = KParams::standin();
    let img = |c: usize| ArgShape::Img { channels: c };
    let cst = ArgShape::Const { n: 1 };
    let call = |tag: i32, dims: (usize, usize)| NodeCall { idx: 0, class: "optical_flow_interp_maps", tag, n: dims.0 * dims.1, dims, kp: &kp };
    let ok = [img(3), img(3), img(2), cst];
    assert!((INTERP_MAPS.accepts)(&call(3, (8, 8)), &ok).is_ok());
    assert!((INTERP_MAPS.accepts)(&call(3, (1, 8)), &ok).is_err(), "width 1");
    assert!((INTERP_MAPS.accepts)(&call(3, (8, 1)), &ok).is_err(), "height 1");
    assert!((INTERP_MAPS.accepts)(&call(0, (8, 8)), &ok).is_err(), "tag 0");
    assert!((INTERP_MAPS.accepts)(&call(5, (8, 8)), &ok).is_err(), "tag 5");
    assert!((INTERP_MAPS.accepts)(&call(3, (8, 8)), &[img(2), img(3), img(2), cst]).is_err(), "previous below the tag");
    assert!((INTERP_MAPS.accepts)(&call(3, (8, 8)), &[img(3), img(2), img(2), cst]).is_err(), "next below the tag");
    assert!((INTERP_MAPS.accepts)(&call(3, (8, 8)), &[img(3), img(3), img(1), cst]).is_err(), "flow below two channels");
    assert!((INTERP_MAPS.accepts)(&call(3, (8, 8)), &[img(3), img(3), img(2), img(1)]).is_err(), "scale not constant");
    assert!((INTERP_MAPS.accepts)(&call(3, (8, 8)), &[cst, img(3), img(2), cst]).is_err(), "constant colour");
    assert!((INTERP_MAPS.accepts)(&call(3, (8, 8)), &[img(3), img(3), img(2)]).is_err(), "three operands");
}

fn open_with(ctx: &fx_device::DeviceCtx, text: &str) -> std::sync::Arc<fx_device::Session> {
    let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
    std::sync::Arc::new(fx_device::Session::open_source(&dev, &info, text, fx_device::kernels::BUILD_OPTIONS).unwrap())
}

#[test]
fn harness_an_existing_op_passes_and_a_wrong_stub_is_refused() {
    let Some(ctx) = device() else { return };
    let existing = fx_exec::device_ops::DEVICE_OPS.iter().find(|d| d.class == "copy").unwrap();
    fx_exec::device_ops::selfcheck(&ctx.session, existing).expect("an existing allowlisted op passes");
    let base = fx_device::kernels::device_program();
    let old = "o0[i] = as_uint(res_a);";
    assert_eq!(base.matches(old).count(), 1);
    let stub = open_with(&ctx, &base.replacen(old, "o0[i] = 0u;", 1));
    match fx_exec::device_ops::selfcheck(&stub, &INTERP_MAPS) {
        Err(m) => println!("STUB refused: {}", m.lines().next().unwrap_or("")),
        Ok(_) => panic!("the wrong stub kernel was accepted"),
    }
}

/// Single-edit mutants: (name, text in the program, replacement). Each must occur once in the program.
const MUTANTS: &[(&str, &str, &str)] = &[
    ("splat x base unfused", "float shifted = im_fma231(0x1p-1f, scale, av);", "float shifted = nanx_add(nanx_mul(scale, av), 0x1p-1f);"),
    ("splat store compare direction", "if (as_float(best[d]) > cost) {", "if (as_float(best[d]) >= cost) {"),
    ("clamp fraction constant", "*fr = 0x1p+0f;", "*fr = 0x1p-1f;"),
    ("cost accumulation dropped", "cost = (ch == 0) ? e : nanx_add(e, cost);", "cost = e;"),
    ("cost error operand order", "float t = nanx_sub(delta_bottom, dxa);", "float t = nanx_sub(dxa, delta_bottom);"),
    ("fill first neighbour load", "if (ri == 0 && k == 0) {", "if (ri == 0 && k == 1) {"),
    ("fill count test", "if (cnt > 0x0p+0f) {", "if (cnt >= 0x0p+0f) {"),
    ("fill sum becomes difference", "sum = nanx_add(sum, v);", "sum = nanx_sub(sum, v);"),
    ("count test", "if (plane[y * w + x] == 0u) {", "if (plane[y * w + x] != 0u) {"),
    ("interp vector arm sign placement", "res_a = nanx_mul(im_flip(cs), a);", "res_a = im_flip(nanx_mul(cs, a));"),
    ("interp body end", "vend = w & ~7;", "vend = w & ~15;"),
    ("interp scalar arm multiplier", "res_c = nanx_mul(a, gain_rest);", "res_c = nanx_mul(a, c);"),
    ("interp scalar arm fused form", "res_b = nanx_mul(c, im_fnma213(scale, b, flow_offset));", "res_b = nanx_mul(c, im_fma213(im_flip(scale), b, flow_offset));"),
    ("interp scalar arm sign dropped", "res_a = im_flip(nanx_mul(cs, a));", "res_a = nanx_mul(cs, a);"),
    ("splat out of range source done", "sdone[idx] = 1u;\n                continue;", "continue;"),
    ("masked copy flag test", "if (flag[i] == 0u) {", "if (flag[i] != 0u) {"),
    ("interp phase mask", "int m = (int)(addr & 0x1ful);", "int m = (int)(addr & 0xful);"),
    ("interp row start phase", "ulong addr = ((ulong)(y * w)) * 4ul;", "ulong addr = ((ulong)(y * w)) * 2ul;"),
];

#[test]
fn every_interp_maps_mutant_is_refused_by_the_compare() {
    let Some(ctx) = device() else { return };
    let base = fx_device::kernels::device_program();
    let binary_hash = |s: &fx_device::Session| fx_device::progcache::fnv(&s.program.binary().unwrap());
    let clean = open_with(&ctx, &base);
    let base_hash = binary_hash(&clean);
    let t = fx_exec::device_ops::selfcheck(&clean, &INTERP_MAPS).expect("the unmutated program passes");
    assert!(matches!(t.nan_payloads, NanPayloads::Same));
    println!("MUTANT base binary {base_hash:016x}");
    let mut prev = base_hash;
    let mut killed = 0usize;
    for (name, old, new) in MUTANTS {
        assert_eq!(base.matches(old).count(), 1, "mutant `{name}`: the text must occur once");
        let s = open_with(&ctx, &base.replacen(old, new, 1));
        let h = binary_hash(&s);
        assert_ne!(h, base_hash, "mutant `{name}` built the base binary");
        assert_ne!(h, prev, "mutant `{name}` built the previous mutant's binary");
        prev = h;
        let verdict = match fx_exec::device_ops::selfcheck(&s, &INTERP_MAPS) {
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
    assert_eq!(killed, MUTANTS.len(), "a mutant survived the compare");
}
