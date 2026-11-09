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

/// The postdivide copy-alpha arm (three channels, copy_alpha 0, tag 3) refuses on the device exactly where the CPU refuses: no host
/// alpha plane, or one of another length. With the plane it equals the CPU kernel; the other three channel shapes stay refused.
#[test]
fn postdivide_copy_alpha_arm_on_the_device() {
    use fx_device::DevImage;
    use fx_exec::backend::NodeCall;
    use fx_exec::device_ops::{ArgShape, DevArg};
    use fx_exec::kernels2::set_copy_alpha_planes;
    use fx_exec::types::KParams;
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            return;
        }
    };
    let op = &device_ops_pointwise::POSTDIVIDE;
    // the copy-alpha cases alone first, so a kernel edit on that arm is caught by its own cases
    let arm = fx_exec::device_ops::DeviceOp { corpus: device_ops_pointwise::postdiv_copy_alpha_cases, ..device_ops_pointwise::POSTDIVIDE };
    let ca = (arm.corpus)().len();
    let t_arm = fx_exec::device_ops::selfcheck(&ctx.session, &arm).expect("copy-alpha arm equals the CPU kernel");
    println!("postdivide copy-alpha self-check: {ca} cases, {} outputs compared, nan payloads {:?}", t_arm.outputs, t_arm.nan_payloads);
    assert!(ca >= 40);
    let cases = (op.corpus)();
    let tally = fx_exec::device_ops::selfcheck(&ctx.session, op).expect("postdivide equals the CPU kernel");
    println!("postdivide full self-check: {} cases, {} outputs compared, nan payloads {:?}", cases.len(), tally.outputs, tally.nan_payloads);
    let kp = KParams::standin();
    let (w, h) = (9usize, 3usize);
    let n = w * h;
    let src: Vec<Vec<f32>> = (0..3).map(|p| (0..n).map(|i| (i * 3 + p) as f32 * 0.37 - 2.0).collect()).collect();
    let planes: Vec<&[f32]> = src.iter().map(|p| p.as_slice()).collect();
    let dev = DevImage::upload(&ctx.session, &planes).expect("upload");
    let call = |tag: i32| NodeCall { idx: 0, class: "postdivide", tag, n, dims: (w, h), kp: &kp };
    let shapes = [ArgShape::Img { channels: 3 }, ArgShape::Const { n: 1 }];
    assert!((op.accepts)(&call(3), &shapes).is_ok());
    for tag in [1, 2, 4] {
        let why = (op.accepts)(&call(tag), &shapes).expect_err("three channels off tag 3 are refused");
        assert!(why.contains("three channel"), "{why}");
    }
    let one = (op.accepts)(&call(1), &[ArgShape::Img { channels: 1 }, ArgShape::Const { n: 1 }]).expect_err("one channel is refused");
    assert!(one.contains("one channel"), "{one}");
    let run = |ca: f32| (op.run)(&ctx.session, &call(3), &[DevArg::Img(&dev), DevArg::Const { n: 1, lanes: [ca.to_bits(), 0, 0, 0] }]);
    set_copy_alpha_planes(None);
    let e = run(0.0).expect_err("no host plane");
    assert!(e.to_string().contains("no host alpha plane"), "{e}");
    set_copy_alpha_planes(Some(vec![vec![0.5; n - 1]]));
    let e = run(0.0).expect_err("short host plane");
    assert!(e.to_string().contains("samples"), "{e}");
    set_copy_alpha_planes(Some(vec![]));
    assert!(run(0.0).is_err(), "no plane for input 0");
    let alpha: Vec<f32> = (0..n).map(|i| [0.0f32, -0.0, 1e-4, 0.25, 0.999, 1.0, 2.0, f32::INFINITY, f32::NAN][i % 9]).collect();
    set_copy_alpha_planes(Some(vec![alpha.clone()]));
    for ca in [-1.0f32, 1.0, 2.5] {
        assert!(run(ca).is_err(), "copy_alpha {ca} is refused on three channels");
    }
    let got = run(0.0).and_then(|d| d.download()).expect("copy-alpha arm on the device");
    let img = fx_exec::types::Image { n, planes: src.clone() };
    let want = match fx_exec::kernels::run_any("postdivide", 3, &[fx_exec::types::Arg::Img(&img), fx_exec::types::Arg::Const { n: 1, lanes: [0, 0, 0, 0] }], n, (w, h), &kp) {
        Ok(fx_exec::kernels2::Out::Img(i)) => i,
        _ => panic!("the CPU kernel refused the copy-alpha arm"),
    };
    let bits = |v: &Vec<Vec<f32>>| v.iter().map(|p| p.iter().map(|x| x.to_bits()).collect::<Vec<u32>>()).collect::<Vec<_>>();
    assert_eq!(bits(&got), bits(&want.planes));
    // a second host plane of the same length is used, not the first upload
    let alpha2: Vec<f32> = alpha.iter().map(|a| a * 0.5).collect();
    set_copy_alpha_planes(Some(vec![alpha2]));
    let got2 = run(0.0).and_then(|d| d.download()).expect("second run");
    let want2 = match fx_exec::kernels::run_any("postdivide", 3, &[fx_exec::types::Arg::Img(&img), fx_exec::types::Arg::Const { n: 1, lanes: [0, 0, 0, 0] }], n, (w, h), &kp) {
        Ok(fx_exec::kernels2::Out::Img(i)) => i,
        _ => panic!("the CPU kernel refused the copy-alpha arm"),
    };
    assert_eq!(bits(&got2), bits(&want2.planes));
    assert_ne!(bits(&got2), bits(&got));
    // the self-check hook installs the case plane (no corpus frame has 27 samples, so the run refuses on its length) and
    // restores the planes that were in place when it drops
    set_copy_alpha_planes(Some(vec![vec![0.75f32; n]]));
    let name = cases.iter().find(|c| c.name.contains("copy-alpha") && c.dims.0 * c.dims.1 != n).expect("copy-alpha cases").name.clone();
    {
        let _g = device_ops_pointwise::selfcheck_host_planes("postdivide", &name);
        assert!(run(0.0).is_err(), "the case plane is installed");
    }
    let after = run(0.0).and_then(|d| d.download()).expect("restored plane");
    let want3 = match fx_exec::kernels::run_any("postdivide", 3, &[fx_exec::types::Arg::Img(&img), fx_exec::types::Arg::Const { n: 1, lanes: [0, 0, 0, 0] }], n, (w, h), &kp) {
        Ok(fx_exec::kernels2::Out::Img(i)) => i,
        _ => panic!("the CPU kernel refused the restored plane"),
    };
    set_copy_alpha_planes(None);
    assert_eq!(bits(&after), bits(&want3.planes));
}

/// A test that needs a device and found none: it prints NOT RUN, or fails when `SAPPHIRE_REQUIRE_DEVICE=1`.
fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}
