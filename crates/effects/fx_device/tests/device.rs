//! Tests that need an OpenCL device: they print `NOT RUN` and pass when the machine has no usable platform,
//! and panic instead when `SAPPHIRE_REQUIRE_DEVICE=1`.
//! Without a device the same kernels are covered by the simulator tests (`tests/sim.rs`) on every OS.

use fx_device::cl::{Arg, Buffer};
use fx_device::selfcheck::{adversarial_f32, compare_f32, SIZES};
use fx_device::{kernels, select, Backend, DeviceKind, DevImage};

fn device() -> Option<std::sync::Arc<fx_device::DeviceCtx>> {
    match select(DeviceKind::Gpu) {
        Backend::Device(d) => Some(d),
        other => {
            assert!(std::env::var("SAPPHIRE_REQUIRE_DEVICE").map_or(true, |v| v != "1"), "SAPPHIRE_REQUIRE_DEVICE=1 and no usable OpenCL device ({})", other.describe());
            println!("NOT RUN: no usable OpenCL device ({})", other.describe());
            None
        }
    }
}

#[test]
fn the_probe_and_the_base_self_check_pass_on_the_device() {
    let Some(d) = device() else { return };
    println!("device {} base {}", d.info.display_name(), d.base);
    assert!(fx_device::probe::requirements(&d.info).is_empty());
    assert!(d.base_cached || d.base.outputs > 0);
    // the uncached check, always run here
    let again = fx_device::selfcheck::check_base(&d.session).expect("the base self-check");
    assert!(again.outputs > 0);
}

#[test]
fn copy_one_minus_and_the_nan_scan_match_the_transcriptions() {
    let Some(d) = device() else { return };
    let s = &d.session;
    let words = adversarial_f32(3000, 21);
    for &n in SIZES {
        let src: Vec<f32> = (0..n).map(|i| words[i % words.len()]).collect();
        let a = s.input(&src).unwrap();
        // one_minus
        let b: Buffer<f32> = s.output(n).unwrap();
        s.launch("one_minus_f32", n, &[a.arg(), b.arg(), Arg::U32(n as u32)]).unwrap();
        let got = s.queue.read(&b).unwrap();
        let expected: Vec<f32> = (0..n).map(|i| kernels::rs::one_minus_f32_item(&src, i)).collect();
        compare_f32("one_minus_f32", &format!("n={n}"), |i| format!("0x{:08x}", src[i].to_bits()), &expected, &got).unwrap();
        // copy keeps every bit
        let bits: Vec<u32> = src.iter().map(|f| f.to_bits()).collect();
        let ab = s.input(&bits).unwrap();
        let bb: Buffer<u32> = s.output(n).unwrap();
        s.launch("copy_f32", n, &[ab.arg(), bb.arg(), Arg::U32(n as u32)]).unwrap();
        assert_eq!(s.queue.read(&bb).unwrap(), bits, "n={n}");
        // the NaN scan agrees with the host
        let img = DevImage::upload(s, &[&src]).unwrap();
        assert_eq!(img.has_nan().unwrap(), src.iter().any(|x| x.is_nan()), "n={n}");
        assert_eq!(img.download().unwrap()[0].iter().map(|f| f.to_bits()).collect::<Vec<_>>(), bits);
    }
    let clean = vec![1.0f32, -0.0, f32::INFINITY, f32::from_bits(1)];
    assert!(!DevImage::upload(s, &[&clean]).unwrap().has_nan().unwrap());
}

#[test]
fn the_memory_budget_refuses_an_oversized_upload() {
    let Some(d) = device() else { return };
    let big = (d.session.mem.budget() / 4) as usize + 1;
    // do not allocate a real huge host vector: the budget check happens before any buffer exists
    let r = DevImage::alloc(&d.session, 1, big);
    assert!(matches!(r, Err(fx_device::DevError::Memory { .. })), "{r:?}");
    assert_eq!(d.session.mem.used(), 0);
}

#[test]
fn a_contracting_build_is_caught_when_the_device_fuses() {
    let Some(d) = device() else { return };
    let bad = kernels::device_program().replacen("#pragma OPENCL FP_CONTRACT OFF", "#pragma OPENCL FP_CONTRACT ON", 1);
    let (dev, info) = fx_device::probe::open(d.info.index).unwrap();
    let s = fx_device::Session::open_source(&dev, &info, &bad, kernels::BUILD_OPTIONS).unwrap();
    let ok = fx_device::selfcheck::chain_matches(&s).unwrap();
    // A device whose compiler does not contract in this loop stays equal; the check is the detector, not a requirement.
    println!("FP_CONTRACT ON build matches the unfused loop: {ok}");
    let good = fx_device::selfcheck::chain_matches(&d.session).unwrap();
    assert!(good, "the strict build must match the unfused loop");
}
