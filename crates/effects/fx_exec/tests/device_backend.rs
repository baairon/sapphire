//! The device backend seam: with a device the starter ops are admitted by their self-check, a plan renders to the CPU's bytes,
//! and every decline (no kernel, not allowlisted, NaN output) falls back per node. Prints NOT RUN without an OpenCL device.

use fx_device::{DeviceKind, Backend as Handle};
use fx_eval::{build_effect, Corpus};
use fx_exec::backend::{self, Backend, DeviceBackend, Fallback, NodeCall};
use fx_exec::{compile_for, execute_compiled, Binding, HostFrames, Image, Inputs, KParams, Val};
use std::collections::BTreeMap;
use std::rc::Rc;

fn device() -> Option<std::sync::Arc<fx_device::DeviceCtx>> {
    match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => Some(d),
        other => {
            println!("NOT RUN: no usable OpenCL device ({})", other.describe());
            None
        }
    }
}

fn synth(ch: usize, n: usize, seed: u64) -> Image {
    let mut s = seed;
    let planes = (0..ch)
        .map(|_| {
            (0..n)
                .map(|_| {
                    s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    0.05 + (s >> 40) as f32 / (1u64 << 24) as f32
                })
                .collect()
        })
        .collect();
    Image::from_planes(planes).unwrap()
}

#[test]
fn device_backend_admits_exactly_the_starter_ops() {
    let Some(ctx) = device() else { return };
    let b = DeviceBackend::new(ctx);
    assert_eq!(b.admitted(), vec!["copy", "one_minus"]);
    assert!(b.report().refused.is_empty(), "{:?}", b.report().refused);
}

#[test]
fn device_backend_declines_with_the_right_reason() {
    let Some(ctx) = device() else { return };
    let mut b = DeviceBackend::new(ctx);
    let kp = KParams::standin();
    let img = Val::Img(Rc::new(synth(3, 12, 1)));
    let call = |class: &'static str, tag| NodeCall { idx: 0, class, tag, n: 12, dims: (4, 3), kp: &kp };
    // no kernel at all
    assert_eq!(b.try_run(&call("blur", 3), &[img.clone()]).err(), Some(Fallback::NoKernel));
    // a kernel op that the shape does not fit (fewer channels than the tag)
    assert!(matches!(b.try_run(&call("one_minus", 4), &[img.clone()]), Err(Fallback::Unsupported(_))));
    // runs on the device and stays resident
    let out = b.try_run(&call("one_minus", 3), &[img.clone()]).unwrap();
    assert!(matches!(out, Val::Dev(_)));
    // chained: the next device op reads the resident value, and a CPU reader gets the host copy with the same bits
    let twice = b.try_run(&call("one_minus", 3), &[out.clone()]).unwrap();
    assert!(matches!(twice, Val::Dev(_)));
    let host = backend::host_args(std::slice::from_ref(&twice)).unwrap();
    let want: Vec<u32> = (0..12).map(|i| (1.0f32 - (1.0f32 - synth(3, 12, 1).planes[0][i])).to_bits()).collect();
    match host[0] {
        fx_exec::Arg::Img(i) => assert_eq!(i.planes[0].iter().map(|x| x.to_bits()).collect::<Vec<_>>(), want),
        _ => panic!("not an image"),
    }
    // a NaN in the output reruns on the CPU
    let mut nan = synth(1, 12, 2);
    nan.planes[0][5] = f32::NAN;
    assert_eq!(b.try_run(&call("copy", 1), &[Val::Img(Rc::new(nan))]).err(), Some(Fallback::NanOutput));
    // an allowlist that leaves an op out: the op has a kernel but is not admitted
    let mut only_copy = DeviceBackend::with_allowlist(device().unwrap(), &["copy"]);
    assert_eq!(only_copy.try_run(&call("one_minus", 3), &[img]).err(), Some(Fallback::NotAllowlisted));
}

#[test]
fn device_backend_renders_plans_to_the_cpu_bytes() {
    let Some(ctx) = device() else { return };
    let corpus = Corpus::native();
    let (w, h) = (23usize, 9usize);
    let mut ran = 0;
    for name in fx_eval::programs_native::registered_names() {
        let name = name.as_str();
        let Some(ei) = corpus.find_effect(name, None) else { println!("{name}: not found"); continue };
        let g = match build_effect(&corpus, ei) { Ok(g) => g, Err(e) => { println!("{name}: build {e:?}"); continue } };
        let mut b = Binding::defaults();
        b.hc_zero_legacy = true;
        b.const_inputs.insert("Mask".to_string(), vec![1.0]);
        let cp = match compile_for(&g, &b) { Ok(c) => c, Err(e) => { println!("{name}: compile {e}"); continue } };
        let mut images = BTreeMap::new();
        for (i, nm) in ["Source", "Back", "Background", "Front", "Matte", "SparksMatte", "Lens", "Foreground"].iter().enumerate() {
            images.insert(nm.to_string(), synth(4, w * h, 7 + i as u64));
        }
        let inputs = Inputs { width: w, height: h, images };
        let cpu = match execute_compiled(&cp, &b, &inputs, &HostFrames::default()) { Ok(c) => c, Err(e) => { println!("{name}: cpu {e}"); continue } };
        let guard = backend::install(Box::new(DeviceBackend::new(ctx.clone())));
        let dev = execute_compiled(&cp, &b, &inputs, &HostFrames::default()).expect("the device run succeeds when the CPU run does");
        let report = backend::current_report();
        drop(guard);
        assert_eq!(cpu.dest.planes.len(), dev.dest.planes.len(), "{name}");
        for (a, d) in cpu.dest.planes.iter().zip(&dev.dest.planes) {
            assert_eq!(a.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), d.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), "{name}");
        }
        println!("{name}: {}", report.line());
        ran += 1;
    }
    assert!(ran > 0, "no native effect ran on the CPU in this test");
}
