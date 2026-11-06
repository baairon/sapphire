//! Builder k1: every k1 op is admitted by the self-check on a real device (prints NOT RUN without one).

use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend as _, DeviceBackend};
use fx_exec::device_ops_k1;

#[test]
fn k1_ops_pass_the_self_check_on_the_device() {
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        other => {
            println!("NOT RUN: no usable OpenCL device ({})", other.describe());
            return;
        }
    };
    let names: Vec<&str> = device_ops_k1::ops().iter().map(|o| o.class).collect();
    let t = std::time::Instant::now();
    let b = DeviceBackend::with_allowlist(ctx, &names);
    println!("k1 self-check of {} ops took {:?}", names.len(), t.elapsed());
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
fn k1_corpora_are_not_empty() {
    for op in device_ops_k1::ops() {
        let n = (op.corpus)().len();
        println!("{} corpus {}", op.class, n);
        assert!(n > 0, "{} has an empty corpus", op.class);
    }
}

#[test]
fn k1_selfcheck_time_per_op() {
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        _ => return,
    };
    for op in device_ops_k1::ops() {
        let t = std::time::Instant::now();
        let _ = DeviceBackend::with_allowlist(ctx.clone(), &[op.class]);
        println!("TIME {:28} {:?}", op.class, t.elapsed());
    }
}
