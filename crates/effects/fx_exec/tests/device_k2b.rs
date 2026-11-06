//! K2B ops (make_mipmap, blur_warp): each passes its device self-check against the CPU kernel, and a device built ripmap
//! feeds the device samplers (blur_warp, warp_rel, warp) with the same bytes as the host ripmap.
//! Prints NOT RUN without an OpenCL device.

use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend, DeviceBackend};
use fx_exec::device_ops::DEVICE_OPS;
use fx_exec::device_ops_k2b::{selfcheck_cov, CLASSES};

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
fn the_k2b_ops_pass_their_self_check_and_are_admitted() {
    let Some(ctx) = device() else { return };
    for class in CLASSES {
        let op = DEVICE_OPS.iter().find(|o| o.class == *class).expect("op row");
        let t = std::time::Instant::now();
        match selfcheck_cov(&ctx.session, op) {
            Ok((tally, cov)) => println!("K2B {class}: {tally}; {cov:?}; {:.2}s", t.elapsed().as_secs_f64()),
            Err(m) => panic!("K2B {class}: {m}"),
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

#[test]
fn a_device_built_ripmap_feeds_the_device_samplers_with_the_cpu_bytes() {
    use fx_exec::backend::NodeCall;
    use fx_exec::kernels;
    use fx_exec::kernels2::Out;
    use fx_exec::types::{Arg, Image, KParams, Val};
    use std::rc::Rc;
    let Some(ctx) = device() else { return };
    let mut b = DeviceBackend::with_allowlist(ctx, &["make_mipmap", "blur_warp", "warp_rel", "warp"]);
    let (w, h) = (23usize, 11usize);
    let n = w * h;
    let kp = KParams::standin();
    let mk = |ch: usize| Image { n, planes: (0..ch).map(|p| (0..n).map(|i| ((i * 7 + p * 13) % 31) as f32 * 0.07 - 0.4).collect()).collect() };
    let cst = |v: &[f32]| {
        let mut l = [0u32; 4];
        for (i, x) in v.iter().enumerate() {
            l[i] = x.to_bits();
        }
        Val::Const { n: v.len(), lanes: l }
    };
    let coords = |s: f32| Image { n, planes: vec![(0..n).map(|i| ((i % 17) as f32 / 8.0 - 1.0) * s).collect(), (0..n).map(|i| ((i % 5) as f32 / 2.0 - 1.0) * s).collect()] };
    for ch in 1..=4usize {
        let src = Val::Img(Rc::new(mk(ch)));
        let mm_args = vec![src.clone(), cst(&[1.0, 2.0]), cst(&[1.0])];
        let call = NodeCall { idx: 0, class: "make_mipmap", tag: ch as i32, n, dims: (w, h), kp: &kp };
        let mip = b.try_run(&call, &mm_args).expect("make_mipmap on the device");
        assert!(matches!(mip, Val::DevMip(_)));
        let cpu_mip = match kernels::run_any("make_mipmap", ch as i32, &[Arg::Img(&mk(ch)), Arg::Const { n: 2, lanes: [1.0f32.to_bits(), 2.0f32.to_bits(), 0, 0] }, Arg::Const { n: 1, lanes: [1.0f32.to_bits(), 0, 0, 0] }], n, (w, h), &kp).unwrap() {
            Out::Mip(m) => m,
            _ => panic!("ripmap expected"),
        };
        let cases: Vec<(&str, Vec<Val>)> = vec![
            ("blur_warp", vec![mip.clone(), Val::Img(Rc::new(coords(0.9))), Val::Img(Rc::new(coords(0.6))), cst(&[0.5, 1.0]), cst(&[1.0]), cst(&[1.0, 2.0]), cst(&[1.0]), cst(&[0.0])]),
            ("warp_rel", vec![mip.clone(), Val::Img(Rc::new(coords(0.9))), cst(&[1.0, 2.0]), cst(&[0.0])]),
            ("warp", vec![mip.clone(), Val::Img(Rc::new(coords(0.9))), cst(&[1.0, 2.0]), cst(&[0.0])]),
        ];
        for (class, args) in cases {
            let call = NodeCall { idx: 1, class, tag: ch as i32, n, dims: (w, h), kp: &kp };
            let got = b.try_run(&call, &args).unwrap_or_else(|e| panic!("{class} ch{ch}: {e:?}"));
            let Val::Dev(d) = got else { panic!("device image expected") };
            let dev = d.materialize().unwrap().clone();
            let cpu_args: Vec<Arg> = args.iter().enumerate().map(|(i, a)| if i == 0 { Arg::Mip(&cpu_mip) } else { a.as_arg() }).collect();
            let Out::Img(want) = kernels::run_any(class, ch as i32, &cpu_args, n, (w, h), &kp).unwrap() else { panic!("image expected") };
            for (p, (g, e)) in dev.planes.iter().zip(&want.planes).enumerate() {
                assert!(g.iter().zip(e).all(|(a, c)| a.to_bits() == c.to_bits()), "{class} ch{ch} plane {p}");
            }
        }
        // the host copy of the device ripmap equals the CPU ripmap
        if let Val::DevMip(m) = &mip {
            assert_eq!(m.materialize().unwrap().ripmap.planes, cpu_mip.ripmap.planes);
        }
    }
}
