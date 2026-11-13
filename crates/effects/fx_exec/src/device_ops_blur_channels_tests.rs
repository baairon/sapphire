//! Compare driver of blur_channels: the device route against the CPU kernel (`device_ops::selfcheck`), and single-edit stand-ins
//! of the host route that the driver must refuse. Run with `--test-threads=1`. Prints NOT RUN without an OpenCL device.
use super::{BLUR_CHANNELS, MUTANT};
use crate::device_ops::{self, DEVICE_OPS};
use fx_device::selfcheck::NanPayloads;
use fx_device::{Backend as Handle, DeviceKind};

/// Mutants: 1 plane 0 parameters for every plane, 2 parameters of the next plane, 3 copy source plane off by one, 4 gain of plane 0
/// for all, 5 pass count off by one, 6 edge mode flipped, 7 subpixel flipped, 8 render scale plus one, 9 aspect dropped, 10 sigma axes swapped.
const MUTANTS: &[(u8, &str)] = &[
    (1, "plane 0 parameters used for every plane"),
    (2, "plane parameters swapped with the next plane"),
    (3, "copy source plane off by one"),
    (4, "gain of plane 0 used for all"),
    (5, "pass count off by one"),
    (6, "edge mode flipped"),
    (7, "subpixel flipped"),
    (8, "render scale plus one"),
    (9, "aspect dropped"),
    (10, "sigma axes swapped"),
];

#[test]
fn blur_channels_device_route_equals_the_cpu_and_refuses_every_mutant() {
    let ctx = match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => d,
        other => {
            if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
                panic!("device required but unavailable: {}", other.describe());
            }
            println!("NOT RUN: no usable OpenCL device ({})", other.describe());
            return;
        }
    };
    let s = &ctx.session;
    // the existing blur self-check passes (harness line)
    let blur = DEVICE_OPS.iter().find(|o| o.class == "blur").unwrap();
    device_ops::selfcheck(s, blur).expect("blur self-check");
    println!("HARNESS blur self-check: pass");
    let cases = (BLUR_CHANNELS.corpus)().len();
    let t = device_ops::selfcheck(s, &BLUR_CHANNELS).expect("blur_channels self-check against the CPU");
    println!("EQUAL blur_channels: corpus {cases} cases, outputs {}, NanPayloads {:?}", t.outputs, t.nan_payloads);
    assert_eq!(t.nan_payloads, NanPayloads::Same);
    let mut survivors = Vec::new();
    for (m, name) in MUTANTS {
        MUTANT.with(|x| x.set(*m));
        let r = device_ops::selfcheck(s, &BLUR_CHANNELS);
        MUTANT.with(|x| x.set(0));
        let killed = match &r {
            Err(e) => {
                println!("MUTANT {m} {name}: KILLED ({})", e.chars().take(110).collect::<String>());
                true
            }
            Ok(t) if t.nan_payloads != NanPayloads::Same => {
                println!("MUTANT {m} {name}: KILLED (NaN payloads differ)");
                true
            }
            Ok(_) => {
                println!("MUTANT {m} {name}: SURVIVED");
                false
            }
        };
        if !killed {
            survivors.push(*m);
        }
    }
    assert!(survivors.is_empty(), "surviving mutants {survivors:?}");
}
