//! Warp ops (warp_rel, warp, rotate, perspective, perspective_alpha): each passes its device self-check against the CPU kernel.
//! Prints NOT RUN without an OpenCL device.

use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend, DeviceBackend};
use fx_exec::device_ops::DEVICE_OPS;
use fx_exec::device_ops_warp::{selfcheck_cov, CLASSES};

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
fn the_warp_ops_pass_their_self_check_and_are_admitted() {
    let Some(ctx) = device() else { return };
    for class in CLASSES {
        let op = DEVICE_OPS.iter().find(|o| o.class == *class).expect("op row");
        let t = std::time::Instant::now();
        match selfcheck_cov(&ctx.session, op) {
            Ok((tally, cov)) => println!("warp {class}: {tally}; {cov:?}; {:.2}s", t.elapsed().as_secs_f64()),
            Err(m) => panic!("warp {class}: {m}"),
        }
    }
    let b = DeviceBackend::with_allowlist(ctx, CLASSES);
    assert!(b.report().refused.is_empty(), "{:?}", b.report().refused);
    let mut got = b.admitted();
    got.sort();
    let mut want: Vec<&str> = CLASSES.to_vec();
    want.sort();
    assert_eq!(got, want);
}

/// A test that needs a device and found none: it prints NOT RUN, or fails when `SAPPHIRE_REQUIRE_DEVICE=1`.
fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}
