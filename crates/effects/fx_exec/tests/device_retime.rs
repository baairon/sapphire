//! The four S_Retime device ops (`xy_to_polar`, `hsv_to_rgb`, `optical_flow_interp_maps`, `optical_flow`) through the device backend
//! against the CPU kernels on synthetic frames: every output word equal, no fallback, plus one `optical_flow` frame on the
//! smoothing route and one refusal shape (a width one frame for the interp maps, which the CPU refuses and the device declines).
//! Prints NOT RUN without an OpenCL device (run with --test-threads=1).

use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{self, Backend, DeviceBackend, Fallback, NodeCall};
use fx_exec::device_ops_flow as dv;
use fx_exec::kernels2::Out;
use fx_exec::types::{Arg, Image, KParams, Val};
use std::rc::Rc;

fn device() -> Option<std::sync::Arc<fx_device::DeviceCtx>> {
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

fn synth(chan_count: usize, n: usize, seed: u64, lo: f32, span: f32) -> Image {
    let mut s = seed;
    let planes = (0..chan_count)
        .map(|_| {
            (0..n)
                .map(|_| {
                    s = s.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
                    lo + span * ((s >> 40) as f32 / (1u64 << 24) as f32)
                })
                .collect()
        })
        .collect();
    Image::from_planes(planes).unwrap()
}

fn konst(x: f32) -> [u32; 4] {
    [x.to_bits(); 4]
}

fn bits(i: &Image) -> Vec<Vec<u32>> {
    i.planes.iter().map(|p| p.iter().map(|v| v.to_bits()).collect()).collect()
}

/// Runs `class` on the CPU and through the forced device backend and requires equal bits and no fallback.
fn same(b: &mut DeviceBackend, class: &'static str, tag: i32, dims: (usize, usize), imgs: &[Image], consts: &[(usize, [u32; 4])]) {
    let kp = KParams::host_defaults();
    let n = dims.0 * dims.1;
    let mut cpu_args: Vec<Arg> = imgs.iter().map(Arg::Img).collect();
    cpu_args.extend(consts.iter().map(|&(n, lanes)| Arg::Const { n, lanes }));
    let want = match fx_exec::kernels::run_any(class, tag, &cpu_args, n, dims, &kp) {
        Ok(Out::Img(i)) => i,
        _ => panic!("{class}: the CPU kernel did not give an image"),
    };
    let mut vals: Vec<Val> = imgs.iter().map(|i| Val::Img(Rc::new(i.clone()))).collect();
    vals.extend(consts.iter().map(|&(n, lanes)| Val::Const { n, lanes }));
    let call = NodeCall { idx: 0, class, tag, n, dims, kp: &kp };
    let out = match b.try_run(&call, &vals) {
        Ok(v) => v,
        Err(f) => panic!("{class} {dims:?}: fell back ({f:?})"),
    };
    assert!(matches!(out, Val::Dev(_)), "{class}: the result stays on the device");
    let host = backend::host_args(std::slice::from_ref(&out)).unwrap();
    match host[0] {
        Arg::Img(g) => assert_eq!(bits(g), bits(&want), "{class} {dims:?}: device differs from the CPU"),
        _ => panic!("{class}: not an image"),
    }
}

#[test]
fn the_four_retime_ops_equal_the_cpu_through_the_forced_backend() {
    let Some(ctx) = device() else { return };
    let mut b = DeviceBackend::new(ctx);
    for (w, h) in [(8usize, 8usize), (17, 9), (33, 5)] {
        let n = w * h;
        same(&mut b, "xy_to_polar", 2, (w, h), &[synth(2, n, 11, -2.0, 4.0)], &[(1, konst(0.25))]);
        same(&mut b, "hsv_to_rgb", 3, (w, h), &[synth(3, n, 12, 0.0, 1.0)], &[]);
        same(&mut b, "optical_flow_interp_maps", 3, (w, h), &[synth(3, n, 13, 0.0, 1.0), synth(3, n, 14, 0.0, 1.0), synth(2, n, 15, -1.5, 3.0)], &[(1, konst(0.5))]);
    }
    // optical_flow: the first corpus cases of the op, the smoothing route (smooth above zero) among them
    let mut smoothing = 0;
    for c in dv::flow_cases().into_iter().take(4) {
        let (a, bb) = dv::case_frames(&c);
        let n = c.w * c.h;
        let planes = |v: &Vec<f32>| {
            if v.len() == 3 * n { Image::from_planes(v.chunks(n).map(|p| p.to_vec()).collect()).unwrap() } else { Image::from_planes(vec![v.clone(); 3]).unwrap() }
        };
        let consts: Vec<(usize, [u32; 4])> = dv::case_consts(&c).into_iter().map(|(_, n, l)| (n, l)).collect();
        same(&mut b, "optical_flow", 3, (c.w, c.h), &[planes(&a), planes(&bb)], &consts);
        if c.smooth > 0.0 {
            smoothing += 1;
        }
    }
    assert!(smoothing > 0, "no smoothing route frame among the cases");
}

#[test]
fn a_refused_shape_is_declined_by_the_device_and_refused_by_the_cpu() {
    let Some(ctx) = device() else { return };
    let mut b = DeviceBackend::new(ctx);
    let kp = KParams::host_defaults();
    let imgs = [synth(3, 8, 1, 0.0, 1.0), synth(3, 8, 2, 0.0, 1.0), synth(2, 8, 3, -1.0, 2.0)];
    let mut cpu_args: Vec<Arg> = imgs.iter().map(Arg::Img).collect();
    cpu_args.push(Arg::Const { n: 1, lanes: konst(0.5) });
    assert!(fx_exec::kernels::run_any("optical_flow_interp_maps", 3, &cpu_args, 8, (1, 8), &kp).is_err(), "the CPU refuses a width one frame");
    let mut vals: Vec<Val> = imgs.iter().map(|i| Val::Img(Rc::new(i.clone()))).collect();
    vals.push(Val::Const { n: 1, lanes: konst(0.5) });
    let call = NodeCall { idx: 0, class: "optical_flow_interp_maps", tag: 3, n: 8, dims: (1, 8), kp: &kp };
    assert!(matches!(b.try_run(&call, &vals), Err(Fallback::Unsupported(_))), "the device declines the same shape");
}
