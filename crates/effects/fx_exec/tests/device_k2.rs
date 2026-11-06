//! K2 ops (warp_rel, warp, rotate, perspective, perspective_alpha): each passes its device self-check against the CPU kernel.
//! Prints NOT RUN without an OpenCL device.

use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend, DeviceBackend};
use fx_exec::device_ops::DEVICE_OPS;
use fx_exec::device_ops_k2::{selfcheck_cov, CLASSES};

fn device() -> Option<std::sync::Arc<fx_device::DeviceCtx>> {
    match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => Some(d),
        other => {
            println!("NOT RUN: no usable OpenCL device ({})", other.describe());
            None
        }
    }
}

#[test]
fn the_k2_ops_pass_their_self_check_and_are_admitted() {
    let Some(ctx) = device() else { return };
    for class in CLASSES {
        let op = DEVICE_OPS.iter().find(|o| o.class == *class).expect("op row");
        let t = std::time::Instant::now();
        match selfcheck_cov(&ctx.session, op) {
            Ok((tally, cov)) => println!("K2 {class}: {tally}; {cov:?}; {:.2}s", t.elapsed().as_secs_f64()),
            Err(m) => panic!("K2 {class}: {m}"),
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
