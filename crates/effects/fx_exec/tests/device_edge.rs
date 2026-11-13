//! Self-check driver of the `edge_detect` device op against `edge_detect::render` (through the CPU dispatch), bit for bit.
//! Three tests: the existing `blur` self-check passes on the program; the `edge_detect` self-check refuses a stub gradient kernel
//! that returns the blurred plane; the `edge_detect` self-check of the real program passes with equal NaN payloads.
//! Prints NOT RUN without an OpenCL device (run with --test-threads=1: platform discovery is not thread safe).

use fx_device::selfcheck::NanPayloads;
use fx_device::{Backend as Handle, DeviceCtx, DeviceKind, Session};
use fx_exec::device_ops::{self, DeviceOp};
use std::sync::Arc;

fn device() -> Option<Arc<DeviceCtx>> {
    match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => Some(d),
        other => {
            if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
                panic!("device required but unavailable: {}", other.describe());
            }
            println!("NOT RUN: no usable OpenCL device ({})", other.describe());
            None
        }
    }
}

fn op(class: &str) -> &'static DeviceOp {
    device_ops::DEVICE_OPS.iter().find(|o| o.class == class).expect("the op has a kernel")
}

fn session_for(ctx: &DeviceCtx, source: &str) -> Arc<Session> {
    let (dev, info) = fx_device::probe::open(ctx.info.index).unwrap();
    Arc::new(Session::open_source(&dev, &info, source, fx_device::kernels::BUILD_OPTIONS).unwrap())
}

/// The program with the gradient kernel replaced by a stub that returns the blurred plane unchanged.
fn stub_program() -> String {
    let base = fx_device::kernels::device_program();
    let head = "__kernel void edge_grad_f32(";
    let at = base.find(head).expect("the gradient kernel is in the program");
    let end = at + base[at..].find("\n}\n").expect("the kernel ends") + 3;
    let stub = "__kernel void edge_grad_f32(__global const float* b, __global float* out, uint w, uint h, float c, float c2, float k) {\n    size_t i = get_global_id(0);\n    if (i >= (size_t)w * (size_t)h) {\n        return;\n    }\n    out[i] = b[i];\n}\n";
    format!("{}{}{}", &base[..at], stub, &base[end..])
}

#[test]
fn the_blur_self_check_passes() {
    let Some(ctx) = device() else { return };
    let s = session_for(&ctx, &fx_device::kernels::device_program());
    let t = device_ops::selfcheck(&s, op("blur")).expect("the blur self-check passes");
    println!("blur self-check: {} outputs, {:?}", t.outputs, t.nan_payloads);
}

#[test]
fn a_stub_gradient_kernel_is_refused_by_the_edge_detect_self_check() {
    let Some(ctx) = device() else { return };
    let s = session_for(&ctx, &stub_program());
    match device_ops::selfcheck(&s, op("edge_detect")) {
        Err(e) => println!("stub refused: {}", e.chars().take(200).collect::<String>()),
        Ok(_) => panic!("the stub gradient kernel passed the self-check"),
    }
}

#[test]
fn the_edge_detect_self_check_passes_with_equal_nan_payloads() {
    let Some(ctx) = device() else { return };
    let s = session_for(&ctx, &fx_device::kernels::device_program());
    let o = op("edge_detect");
    println!("edge_detect corpus: {} cases", (o.corpus)().len());
    let t = device_ops::selfcheck(&s, o).expect("the edge_detect self-check passes");
    println!("edge_detect self-check: {} outputs, NanPayloads {:?}", t.outputs, t.nan_payloads);
    assert_eq!(t.nan_payloads, NanPayloads::Same);
}
