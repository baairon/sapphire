//! screen, overlay_super and alpha_from_rgb_max on a real device: the self-check passes bit for bit, and single-edit mutants of
//! their OpenCL source are each refused by it (prints NOT RUN without a device).

use fx_device::kernels::{self, BUILD_OPTIONS, COPY_F32};
use fx_device::kernels_glow_ops::{GLOW_OPS_FMA, GLOW_OPS_PW};
use fx_device::kernels_pointwise::{POINTWISE_FMA, POINTWISE_PW, POINTWISE_PW2};
use fx_device::selfcheck::NanPayloads;
use fx_device::{Backend as Handle, DeviceKind, Session};
use fx_exec::backend::{Backend as _, DeviceBackend};
use fx_exec::device_ops;
use fx_exec::device_ops_glow_ops;
use std::sync::Arc;

fn device() -> Option<Arc<fx_device::DeviceCtx>> {
    match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => Some(d),
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            None
        }
    }
}

#[test]
fn glow_ops_pass_the_self_check_on_the_device() {
    let Some(ctx) = device() else { return };
    for op in device_glow_ops_list() {
        let t = std::time::Instant::now();
        let tally = device_ops::selfcheck(&ctx.session, op).unwrap_or_else(|e| panic!("{}: {e}", op.class));
        let payloads = match &tally.nan_payloads {
            NanPayloads::Same => "same".to_string(),
            NanPayloads::Differ(m) => format!("differ ({m})"),
        };
        println!("SELFCHECK {} cases {} outputs {} nan payloads {} in {:?}", op.class, (op.corpus)().len(), tally.outputs, payloads, t.elapsed());
    }
    let names: Vec<&str> = device_glow_ops_list().iter().map(|o| o.class).collect();
    let b = DeviceBackend::with_allowlist(ctx, &names);
    assert!(b.report().refused.is_empty(), "{:?}", b.report().refused);
    let mut admitted = b.admitted();
    admitted.sort();
    let mut want = names.clone();
    want.sort();
    assert_eq!(admitted, want);
}

fn device_glow_ops_list() -> Vec<&'static device_ops::DeviceOp> {
    device_ops_glow_ops::ops()
}

/// Single-edit mutants of the kernel source: (name, text found exactly once, replacement).
const MUTANTS: &[(&str, &str, &str)] = &[
    ("screen_unfused", "float r = fma(om, b, a);", "float r = om * b + a;"),
    ("screen_fma_operand_order", "float r = fma(om, b, a);", "float r = fma(om, a, b);"),
    ("screen_one_minus_b", "float om = 0x1p+0f - a;", "float om = 0x1p+0f - b;"),
    ("screen_ext_a_simple", "r = nanx_sub(nanx_add(0x1p+0f, x), x);", "r = 0x1p+0f;"),
    ("screen_ext_a_order", "r = nanx_sub(nanx_add(0x1p+0f, x), x);", "r = nanx_add(nanx_sub(0x1p+0f, x), x);"),
    ("overlay_half_ge", "if (!(b > 0x1p-1f)) {\n        float p", "if (!(b >= 0x1p-1f)) {\n        float p"),
    ("overlay_no_double", "return nanx_add(p, p);", "return p;"),
    ("overlay_one_constant", "if (a > 0x1p+0f || b > 0x1p+0f) {", "if (a > 0x1p+1f || b > 0x1p+0f) {"),
    ("overlay_only_a_test", "if (a > 0x1p+0f || b > 0x1p+0f) {", "if (a > 0x1p+0f) {"),
    ("overlay_max_swap", "        return (a > b) ? a : b;", "        return (b > a) ? b : a;"),
    ("overlay_unfused", "float r = fma(-a, b, a);", "float r = a - a * b;"),
    ("overlay_diff_swap", "float d = b - 0x1p+0f;", "float d = 0x1p+0f - b;"),
    ("overlay_sum_order", "return (s + t) + t;", "return (t + t) + s;"),
    ("overlay_ext_b_no_neg", "float nd = -dd;", "float nd = dd;"),
    ("overlay_ext_b_low_plain", "r = x + x;", "r = x;"),
    ("overlay_ext_a_max_swap", "r = (x > one) ? x : one;", "r = (one > x) ? one : x;"),
    ("afrm_max_operand_order", "float m = (x0 > x1) ? x0 : x1;", "float m = (x1 > x0) ? x1 : x0;"),
    ("afrm_max_ge", "m = (m > x2) ? m : x2;", "m = (m >= x2) ? m : x2;"),
    ("afrm_select_arm", "(use_scale != 0u) ? nanx_mul(as_float(sbits), m) : m", "(use_scale != 0u) ? m : nanx_mul(as_float(sbits), m)"),
    ("afrm_mul_dropped", "out[i] = nanx_mul(as_float(sbits), c2[i]);", "out[i] = c2[i];"),
];

/// The program the mutants are built from: the parts the three ops launch (copy, fill, the shared NaN helpers and their own
/// sources).
fn base_program() -> String {
    kernels::program(&[COPY_F32, POINTWISE_PW, POINTWISE_PW2, POINTWISE_FMA, GLOW_OPS_PW, GLOW_OPS_FMA])
}

fn refused_by(session: &Arc<Session>) -> Vec<String> {
    let mut out = Vec::new();
    for op in device_glow_ops_list() {
        if let Err(e) = device_ops::selfcheck(session, op) {
            out.push(format!("{}: {}", op.class, e.lines().next().unwrap_or("")));
        }
    }
    out
}

#[test]
#[ignore = "builds 21 device programs; run with --include-ignored"]
fn every_source_mutant_is_refused_by_the_self_check() {
    let Some(ctx) = device() else { return };
    let (dev, info) = fx_device::probe::open(ctx.info.index).expect("the device opens again");
    let base = base_program();
    let control = Arc::new(Session::open_source(&dev, &info, &base, BUILD_OPTIONS).expect("the control program builds"));
    let r = refused_by(&control);
    assert!(r.is_empty(), "the unmutated program is refused: {r:?}");
    println!("CONTROL program fnv {:016x} passes", fx_device::progcache::fnv(base.as_bytes()));
    let mut seen = vec![fx_device::progcache::fnv(base.as_bytes())];
    let mut survivors = Vec::new();
    for (name, from, to) in MUTANTS {
        assert_eq!(base.matches(from).count(), 1, "{name}: the edit site must occur exactly once");
        let text = base.replacen(from, to, 1);
        let digest = fx_device::progcache::fnv(text.as_bytes());
        assert!(!seen.contains(&digest), "{name}: the mutant program equals an earlier one");
        seen.push(digest);
        let s = Arc::new(Session::open_source(&dev, &info, &text, BUILD_OPTIONS).unwrap_or_else(|e| panic!("{name}: build: {e}")));
        let r = refused_by(&s);
        if r.is_empty() {
            println!("MUTANT {name} fnv {digest:016x} SURVIVED");
            survivors.push(*name);
        } else {
            println!("MUTANT {name} fnv {digest:016x} KILLED by {}", r.join(" | "));
        }
    }
    assert!(survivors.is_empty(), "surviving mutants: {survivors:?}");
}

/// A test that needs a device and found none: it prints NOT RUN, or fails when `SAPPHIRE_REQUIRE_DEVICE=1`.
fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}
