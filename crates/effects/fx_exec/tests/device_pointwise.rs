//! Every pointwise op is admitted by the self-check on a real device (prints NOT RUN without one).

use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend as _, DeviceBackend};
use fx_exec::device_ops_pointwise;

#[test]
fn pointwise_ops_pass_the_self_check_on_the_device() {
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            return;
        }
    };
    let names: Vec<&str> = device_ops_pointwise::ops().iter().map(|o| o.class).collect();
    let t = std::time::Instant::now();
    let b = DeviceBackend::with_allowlist(ctx, &names);
    println!("pointwise self-check of {} ops took {:?}", names.len(), t.elapsed());
    let r = b.report();
    for (k, v) in &r.refused {
        println!("REFUSED {k}: {v}");
    }
    assert!(r.refused.is_empty(), "{:?}", r.refused);
    let mut admitted = b.admitted();
    admitted.sort();
    let mut want = names.clone();
    want.sort();
    assert_eq!(admitted, want);
}

#[test]
fn pointwise_corpora_are_not_empty() {
    for op in device_ops_pointwise::ops() {
        let n = (op.corpus)().len();
        println!("{} corpus {}", op.class, n);
        assert!(n > 0, "{} has an empty corpus", op.class);
    }
}

#[test]
fn pointwise_selfcheck_time_per_op() {
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        _ => return,
    };
    for op in device_ops_pointwise::ops() {
        let t = std::time::Instant::now();
        let _ = DeviceBackend::with_allowlist(ctx.clone(), &[op.class]);
        println!("TIME {:28} {:?}", op.class, t.elapsed());
    }
}

/// A test that needs a device and found none: it prints NOT RUN, or fails when `SAPPHIRE_REQUIRE_DEVICE=1`.
fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}
