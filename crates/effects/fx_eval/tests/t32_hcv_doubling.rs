//! TMR1 (W479/W490 step T): unit tests of the wired fold. Doubling (T2 family D shape), global 0 and 1 give equal answers (W490 (e) 2),
//! the global refusal on the claim route, the failing-checkout initial lanes (W490 (g)), and the refusals by name.
use fx_eval::hostcurve::{fold, HostCurveValues};

const P: &str = "Blur_Motion_Curves_No_Transform";

fn hcv(global: Option<i64>, flag: i64, rows: &str, ae32: bool) -> HostCurveValues {
    let g = global.map(|g| format!("host global {}\n", g)).unwrap_or_default();
    let mut h = HostCurveValues::parse(&format!("host effect_ref 4660\nhost time_mult 256\nhost time_scale 1\nhost double_flag {}\n{}{}", flag, g, rows)).unwrap();
    h.time_frames = Some(1.0);
    h.ae32 = ae32;
    h.source = "x.hcv".to_string();
    h
}

#[test]
fn doubling_applies_on_global_4_with_flag() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let h = hcv(Some(4), 1, "value z_dist 0 512 512 1 0 131072\n", false);
    let mut d = Vec::new();
    assert_eq!(fold(P, "z_dist", 0.0, &[1.0], 1, &h, &mut d), Some(vec![2.0]), "{:?}", d);
}

#[test]
fn no_doubling_without_flag() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let h = hcv(Some(4), 0, "value z_dist 0 256 256 1 0 131072\n", false);
    let mut d = Vec::new();
    assert_eq!(fold(P, "z_dist", 0.0, &[1.0], 1, &h, &mut d), Some(vec![2.0]), "{:?}", d);
}

/// W490 (e) 2: by the tick rule (doubling only at global 4) global 0 and 1 give the same tick and the same lanes, over a grid of times, multipliers and flags.
#[test]
fn global_0_and_1_equal() {
    if !effects_rtdata::have_tables() {
        return;
    }
    for &m in &[1i32, 24, 256, 1000, 1 << 30] {
        for &fl in &[0i64, 1] {
            for &t in &[-1.5f32, -0.25, 0.0, 0.1, 0.5, 1.0, 2.7, 1e9, f32::NAN] {
                let run = |g: i64| {
                    let mut rows = String::new();
                    let tick = tickmap::shutter_tick(t, &tickmap::Pinfo { time_frames: 1.0, selector: 0, unit_x3: f32::NAN, unit_d4: f64::NAN, unit_d5: f64::NAN }, m);
                    rows.push_str(&format!("value z_dist 0 {} {} 1 0 65536\n", tick, m));
                    let mut h = HostCurveValues::parse(&format!("host effect_ref 1\nhost time_mult {}\nhost time_scale 1\nhost double_flag {}\nhost global {}\n{}", m, fl, g, rows)).unwrap();
                    h.time_frames = Some(1.0);
                    let mut d = Vec::new();
                    let r = fold(P, "z_dist", t, &[1.0], 1, &h, &mut d);
                    (r.map(|v| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>()), d)
                };
                assert_eq!(run(0), run(1), "m {} flag {} t {}", m, fl, t);
            }
        }
    }
}

#[test]
fn global_refused_on_claim_route() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let mut d = Vec::new();
    let h = hcv(Some(0), 0, "value z_dist 0 256 256 1 0 65536\n", true);
    assert!(fold(P, "z_dist", 0.0, &[1.0], 1, &h, &mut d).is_none());
    assert_eq!(d[0], "TICKMAP-GLOBAL-NOT-HOST-APP (x.hcv: global 0)");
    // absent line: HOST-APP's 1 is supplied, and the answer is the same as with global 1
    let h = hcv(None, 0, "value z_dist 0 256 256 1 0 65536\n", true);
    d.clear();
    assert_eq!(fold(P, "z_dist", 0.0, &[1.0], 1, &h, &mut d), Some(vec![1.0]), "{:?}", d);
}

/// W490 (g) B4: a failing checkout leaves the lane unwritten, so the fold returns the node's cells for that lane (the harness guard words of TICKMAP's T2 mark unwritten lanes;
/// crate lib.rs: "a non-zero status leaves the lane unwritten"). Constructed from T2's family F shape (status per index), not read from the T2 stored rows.
#[test]
fn failing_checkout_keeps_cells() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let h = hcv(Some(0), 0, "value shift 0 256 256 1 0 32768\nvalue shift 1 256 256 1 7 99999\n", false);
    let mut d = Vec::new();
    assert_eq!(fold(P, "shift", 0.0, &[0.1, 0.25], 2, &h, &mut d), Some(vec![0.5, 0.25]), "{:?}", d);
}

#[test]
fn refusals_by_name() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let mut d = Vec::new();
    let mut h = hcv(Some(0), 0, "", false);
    h.time_frames = None;
    assert!(fold(P, "z_dist", 0.0, &[1.0], 1, &h, &mut d).is_none() && d[0].starts_with("TICKMAP-HOST-FIELD-UNSOURCED (time_frames)"), "{:?}", d);
    h.time_frames = Some(1.0);
    d.clear();
    assert!(fold(P, "z_dist", 0.0, &[1.0], 1, &h, &mut d).is_none() && d.last().unwrap().starts_with(&format!("HOST-CURVE-VALUE-MISSING ({}: index 0, tick 256, step 256, scale 1)", P)), "{:?}", d);
    d.clear();
    assert!(fold(P, "nosuch", 0.0, &[1.0], 1, &h, &mut d).is_none() && d[0].starts_with("TICKMAP-HOST-FIELD-UNSOURCED (parameter record of"));
    d.clear();
    assert!(fold("Other_Program", "z_dist", 0.0, &[1.0], 1, &h, &mut d).is_none() && d[0].starts_with("TICKMAP-HOST-FIELD-UNSOURCED (parameter record of Other_Program"));
    let h2 = {
        let mut x = HostCurveValues::parse("host effect_ref 1\nhost time_mult 256\nhost time_scale 1\nhost double_flag 0\nhost global 0\nhost selector 1\n").unwrap();
        x.time_frames = Some(1.0);
        x
    };
    d.clear();
    assert!(fold(P, "z_dist", 0.0, &[1.0], 1, &h2, &mut d).is_none() && d[0] == format!("ROUTE-SELECTOR-NOT-CLAIMED ({})", P));
}
