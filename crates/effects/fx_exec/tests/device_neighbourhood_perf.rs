//! Timing of the neighbourhood kernels at 1080p, CPU kernel against the device (PoCL on the CPU here). Ignored by default:
//! `cargo test --release -p fx_exec --test device_neighbourhood_perf -- --ignored --nocapture --test-threads=1`.

use fx_device::{Backend as Handle, DeviceKind};
use fx_exec::backend::{Backend, DeviceBackend, NodeCall};
use fx_exec::types::{Arg, Image, KParams, Val};
use std::rc::Rc;
use std::time::Instant;

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

fn c(v: &[f32]) -> Val {
    let last = *v.last().unwrap();
    Val::Const { n: v.len(), lanes: std::array::from_fn(|i| v.get(i).copied().unwrap_or(last).to_bits()) }
}

#[test]
#[ignore]
fn time_blur_sharpen_grad_static2_and_edge_aware_sharpen_at_1080p() {
    let Handle::Device(ctx) = fx_device::select(DeviceKind::Gpu) else {
        not_run("no usable OpenCL device");
        return;
    };
    let mut b = DeviceBackend::new(ctx);
    let (w, h) = (1920usize, 1080usize);
    let n = w * h;
    let mut kp = KParams::standin();
    kp.sharpen_count = Some(n);
    let src3 = Val::Img(Rc::new(synth(3, n, 1)));
    let src1 = Val::Img(Rc::new(synth(1, n, 2)));
    let mask = Val::Img(Rc::new(synth(1, n, 3)));
    let cases: Vec<(&str, i32, Vec<Val>)> = vec![
        ("blur", 3, vec![src3.clone(), c(&[0.6, 0.6]), c(&[2.0]), c(&[1.0, 1.0, 1.0, 1.0]), c(&[0.0])]),
        ("sharpen", 3, vec![src3.clone(), c(&[2.0, 2.0, 2.0, 2.0]), c(&[0.112])]),
        ("grad", 2, vec![src1.clone(), c(&[0.5]), c(&[1.0]), c(&[0.0])]),
        ("masked_blur", 3, vec![src3.clone(), mask.clone(), c(&[0.5, 0.5]), c(&[2.0]), c(&[0.0])]),
        ("static2", 2, vec![]),
        (
            "edge_aware_sharpen",
            3,
            vec![src3.clone(), mask.clone(), c(&[4.0]), c(&[0.0, 0.0]), c(&[4.0]), c(&[0.5]), c(&[1.0]), c(&[1.0, 0.75, 0.5, 0.25]), c(&[1.0, 1.0, 1.0]), c(&[0.0])],
        ),
    ];
    for (class, tag, args) in cases {
        let call = NodeCall { idx: 0, class, tag, n, dims: (w, h), kp: &kp };
        let cpu_args: Vec<Arg> = args.iter().map(|v| v.as_arg()).collect();
        let t = Instant::now();
        let want = match fx_exec::kernels::run_any(class, tag, &cpu_args, n, (w, h), &kp) {
            Ok(fx_exec::kernels2::Out::Img(i)) => i,
            _ => panic!("cpu kernel failed for {class}"),
        };
        let cpu = t.elapsed();
        let t = Instant::now();
        let got = b.try_run(&call, &args).unwrap_or_else(|f| panic!("{class} fell back: {f:?}"));
        let dev_build = t.elapsed();
        let t = Instant::now();
        let got2 = b.try_run(&call, &args).unwrap();
        let dev = t.elapsed();
        drop(got2);
        // bring the first result back and compare
        let Val::Dev(d) = &got else { panic!("not resident") };
        d.materialize().unwrap();
        let host = d.dev.download().unwrap();
        let same = host.iter().zip(&want.planes).all(|(g, w)| g.iter().zip(w).all(|(a, b)| a.to_bits() == b.to_bits()));
        println!("PERF {class}: cpu {:.3} s, device first {:.3} s, device second {:.3} s, bytes equal {same}", cpu.as_secs_f64(), dev_build.as_secs_f64(), dev.as_secs_f64());
        assert!(same);
    }
}

/// A test that needs a device and found none: it prints NOT RUN, or fails when `SAPPHIRE_REQUIRE_DEVICE=1`.
fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}
