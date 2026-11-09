//! The warp footprint prefill (slot 33) on the device: its self-checks pass against the CPU crate, and every single edit mutant
//! of its OpenCL source, built in place of the real source, is refused by one of them (the mutants listed as equivalent on an x86
//! device are run and reported). Prints NOT RUN without an OpenCL device.

use fx_device::kernels::{program, BUILD_OPTIONS};
use fx_device::kernels_footprint::{FOOTPRINT_PREFILL, FOOTPRINT_WARP};
use fx_device::kernels_ripmap::RIPMAP_MIPMAP;
use fx_device::kernels_warp::{WARP_HELPERS, WARP_WARP};
use fx_exec::device_ops::DEVICE_OPS;
use fx_exec::device_ops_warp::selfcheck_cov;
use fx_device::{Backend as Handle, DeviceKind, Session};
use fx_exec::device_ops_footprint::{ops_selfcheck, prefill_selfcheck, route_selfcheck, driver_selfcheck, PrefillTally, EQUIVALENT_MUTANTS, MUTANTS, WARP_EQUIVALENT_MUTANTS, WARP_MUTANTS};
use std::sync::Arc;

fn device() -> Option<Arc<fx_device::DeviceCtx>> {
    match fx_device::select(DeviceKind::Gpu) {
        Handle::Device(d) => Some(d),
        other => {
            not_run(&format!("no usable OpenCL device ({})", other.describe()));
            None
        }
    }
}

type Check = fn(&Arc<Session>) -> Result<PrefillTally, String>;
const CHECKS: &[(&str, Check)] = &[("prefill", prefill_selfcheck), ("ops", ops_selfcheck), ("route", route_selfcheck)];

#[test]
fn the_footprint_prefill_passes_its_self_checks() {
    let Some(ctx) = device() else { return };
    for (name, check) in CHECKS {
        let t = std::time::Instant::now();
        match check(&ctx.session) {
            Ok(tally) => println!("footprint {name}: {tally}; {:.2}s", t.elapsed().as_secs_f64()),
            Err(m) => panic!("footprint {name}: {m}"),
        }
    }
}

#[test]
fn every_footprint_mutant_is_refused() {
    let Some(ctx) = device() else { return };
    let (dev, info) = fx_device::probe::open(ctx.info.index).expect("device reopens");
    // the real source alone passes (so a refusal below is the edit's doing, not the reduced program)
    let base_text = program(&[FOOTPRINT_PREFILL]);
    let base = Arc::new(Session::open_source(&dev, &info, &base_text, BUILD_OPTIONS).expect("base program builds"));
    for (name, check) in CHECKS {
        check(&base).unwrap_or_else(|m| panic!("base program, {name}: {m}"));
    }
    let mut digests = vec![fx_device::progcache::fnv(base_text.as_bytes())];
    println!("mutant base program fnv {:016x}", digests[0]);
    let mut survivors = Vec::new();
    let all: Vec<(&str, &str, &str, bool)> = MUTANTS.iter().map(|m| (m.0, m.1, m.2, true)).chain(EQUIVALENT_MUTANTS.iter().map(|m| (m.0, m.1, m.2, false))).collect();
    for &(mname, from, to, required) in &all {
        let text = program(&[&FOOTPRINT_PREFILL.replacen(from, to, 1)]);
        let digest = fx_device::progcache::fnv(text.as_bytes());
        assert!(!digests.contains(&digest), "mutant {mname} is not a distinct program");
        digests.push(digest);
        let s = match Session::open_source(&dev, &info, &text, BUILD_OPTIONS) {
            Ok(s) => Arc::new(s),
            Err(e) => panic!("mutant {mname} does not build: {e}"),
        };
        let mut refused = None;
        for (cname, check) in CHECKS {
            if let Err(m) = check(&s) {
                refused = Some(format!("{cname}: {m}"));
                break;
            }
        }
        match refused {
            Some(why) => println!("mutant {mname} fnv {digest:016x} REFUSED by {why}"),
            None if !required => println!("mutant {mname} fnv {digest:016x} SURVIVED (listed as equivalent on this device)"),
            None => {
                println!("mutant {mname} fnv {digest:016x} SURVIVED");
                survivors.push(mname);
            }
        }
    }
    assert!(survivors.is_empty(), "mutants not refused: {survivors:?}");
}

/// The warp and warp_rel self-checks (footprint cases included) on a program of the warp sources only.
fn warp_checks(s: &Arc<Session>) -> Result<String, String> {
    let mut msg = Vec::new();
    for class in ["warp", "warp_rel"] {
        let op = DEVICE_OPS.iter().find(|o| o.class == class).expect("op row");
        let d = driver_selfcheck(s, class == "warp_rel")?;
        let (tally, cov) = selfcheck_cov(s, op)?;
        msg.push(format!("{class}: driver {d}; corpus {} outputs, {cov:?}", tally.outputs));
    }
    Ok(msg.join("; "))
}

#[test]
fn every_footprint_warp_mutant_is_refused() {
    let Some(ctx) = device() else { return };
    let (dev, info) = fx_device::probe::open(ctx.info.index).expect("device reopens");
    let parts = |fw: &str| program(&[WARP_HELPERS, WARP_WARP, RIPMAP_MIPMAP, FOOTPRINT_PREFILL, fw]);
    let base_text = parts(FOOTPRINT_WARP);
    let base = Arc::new(Session::open_source(&dev, &info, &base_text, BUILD_OPTIONS).expect("base program builds"));
    let t = std::time::Instant::now();
    let ok = warp_checks(&base).unwrap_or_else(|m| panic!("base program: {m}"));
    println!("warp mutant base program: {ok}; {:.2}s", t.elapsed().as_secs_f64());
    let mut digests = vec![fx_device::progcache::fnv(base_text.as_bytes())];
    println!("warp mutant base program fnv {:016x}", digests[0]);
    let mut survivors = Vec::new();
    let all: Vec<(&str, &str, &str, bool)> = WARP_MUTANTS.iter().map(|m| (m.0, m.1, m.2, true)).chain(WARP_EQUIVALENT_MUTANTS.iter().map(|m| (m.0, m.1, m.2, false))).collect();
    for &(mname, from, to, required) in &all {
        let text = parts(&FOOTPRINT_WARP.replacen(from, to, 1));
        let digest = fx_device::progcache::fnv(text.as_bytes());
        assert!(!digests.contains(&digest), "mutant {mname} is not a distinct program");
        digests.push(digest);
        let s = match Session::open_source(&dev, &info, &text, BUILD_OPTIONS) {
            Ok(s) => Arc::new(s),
            Err(e) => panic!("mutant {mname} does not build: {e}"),
        };
        match warp_checks(&s) {
            Err(m) => println!("mutant {mname} fnv {digest:016x} REFUSED by {}", m.chars().take(300).collect::<String>()),
            Ok(_) if !required => println!("mutant {mname} fnv {digest:016x} SURVIVED (listed as equivalent)"),
            Ok(_) => {
                println!("mutant {mname} fnv {digest:016x} SURVIVED");
                survivors.push(mname);
            }
        }
    }
    assert!(survivors.is_empty(), "mutants not refused: {survivors:?}");
}

/// A test that needs a device and found none: it prints NOT RUN, or fails when `SAPPHIRE_REQUIRE_DEVICE=1`.
fn not_run(why: &str) {
    if std::env::var("SAPPHIRE_REQUIRE_DEVICE").as_deref() == Ok("1") {
        panic!("device required but unavailable: {why}");
    }
    println!("NOT RUN: {why}");
}
