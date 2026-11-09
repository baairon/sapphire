//! Unit tests of the render body port: structure, determinism, NaN flow, table build invariants, constants against the image cells.
use shake_shift::body::*;
use shake_shift::weights;
use shake_shift::RefServices;

fn globals() -> Globals {
    let wt = weights::build_avx();
    Globals { guard: 0, tab: vec![[0; 3]; TAB_LEN], tail: [0; 3], weights: wt.float.clone(), scale: weights::scale_from_cutoff(9074) }
}
fn params(p1fc: f32) -> Params {
    Params { p4c: 0.37, p7c: 1.25, pac: 0.8, pdc: 2.5, p10c: 1.5, p13c: 0.6, p16c: 0.9, p19c: 3.0, p1cc: 0.11, p1fc, p22c: 0.4, p25c: 0.7 }
}
fn run(p: &Params, g: &mut Globals) -> f32 {
    let mut sv = RefServices::new();
    let mut fr = Frame::new();
    shake_shift_body(p, &Config::default(), g, &mut sv, &mut fr)
}

#[test]
fn deterministic_both_routes() {
    for p1fc in [0.0f32, 1.0] {
        let a = run(&params(p1fc), &mut globals());
        let b = run(&params(p1fc), &mut globals());
        assert_eq!(a.to_bits(), b.to_bits());
        assert!(a.is_finite(), "route {} gave {}", p1fc, a);
    }
}

#[test]
fn signed_zero_selects_the_same_route() {
    let a = run(&params(0.0), &mut globals());
    let b = run(&params(-0.0), &mut globals());
    assert_eq!(a.to_bits(), b.to_bits());
}

#[test]
fn nan_flows_to_the_output() {
    for p1fc in [0.0f32, 1.0] {
        let mut p = params(p1fc);
        p.p25c = f32::NAN;
        assert!(run(&p, &mut globals()).is_nan());
    }
}

#[test]
fn nan_route_selector_takes_the_other_route() {
    // NaN is unordered: the equality test fails, so route B runs (and builds the table)
    let mut g = globals();
    let _ = run(&params(f32::NAN), &mut g);
    assert_eq!(g.guard, 1);
}

#[test]
fn table_build_invariants() {
    let mut g = globals();
    let _ = run(&params(1.0), &mut g);
    assert_eq!(g.guard, 1);
    assert_eq!(g.tab[0][1], 0, "first prefix is zero");
    let mut prev = -1.0f32;
    for e in &g.tab {
        let pre = f32::from_bits(e[1]);
        assert!(pre >= prev, "prefix sums do not decrease");
        prev = pre;
    }
    let last = &g.tab[TAB_LEN - 1];
    let total = f32::from_bits(last[1]) + f32::from_bits(last[0]);
    assert!((total - 1024.0).abs() < 0.5, "normalised total {}", total);
}

#[test]
fn route_a_leaves_the_table_alone() {
    let mut g = globals();
    let _ = run(&params(0.0), &mut g);
    assert_eq!(g.guard, 0);
    assert!(g.tab.iter().all(|e| *e == [0; 3]));
}

#[test]
fn k_sine_is_the_measured_cell() {
    assert_eq!(shake_shift::body::K_SINE.to_bits(), 0x40c90fdb);
    assert_eq!(shake_shift::body::Config::default().k_sine.to_bits(), 0x40c90fdb);
}

/// The wave term is wave_amplitude * sin(two pi * wave_frequency * time): with no random term (rand amplitude 0, route A) the
/// output at a quarter cycle is the wave amplitude, and at a half cycle it is zero.
#[test]
fn wave_has_one_cycle_per_reciprocal_frequency() {
    let mut p = params(0.0);
    p.pac = 0.0;
    p.p10c = 1.0;
    p.p13c = 0.5;
    p.p16c = 0.0;
    p.p7c = 0.5; // 0.25 cycle
    let quarter = run(&p, &mut globals());
    assert!((quarter - 1.0).abs() < 1e-5, "quarter cycle gave {}", quarter);
    p.p7c = 1.0; // 0.5 cycle
    let half = run(&p, &mut globals());
    assert!(half.abs() < 1e-5, "half cycle gave {}", half);
    p.p7c = 1.5; // 0.75 cycle
    let three_q = run(&p, &mut globals());
    assert!((three_q + 1.0).abs() < 1e-5, "three quarter cycle gave {}", three_q);
}
