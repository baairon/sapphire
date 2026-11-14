//! Unit tests of the render body port: structure, determinism, NaN flow, table build invariants, constants against the image cells.
use shake_shift::body::*;
use shake_shift::weights;
use shake_shift::RefServices;

fn globals() -> Globals {
    let wt = weights::build_avx();
    Globals { guard: 0, tab: vec![[0; 3]; TAB_LEN], tail: [0; 3], weights: wt.float.clone(), scale: weights::scale_from_cutoff(9074) }
}
fn params(route: f32) -> Params {
    Params { seed_x: 0.37, time: 1.25, amp_x: 0.8, rate_x: 2.5, sine_amp: 1.5, sine_freq: 0.6, amp_y: 0.9, rate_y: 3.0, seed_y: 0.11, route, limit_gap: 0.4, gain: 0.7 }
}
fn run(p: &Params, g: &mut Globals) -> f32 {
    let mut sv = RefServices::new();
    shake_shift_body(p, &Config::default(), g, &mut sv)
}

#[test]
fn deterministic_both_routes() {
    for route in [0.0f32, 1.0] {
        let a = run(&params(route), &mut globals());
        let b = run(&params(route), &mut globals());
        assert_eq!(a.to_bits(), b.to_bits());
        assert!(a.is_finite(), "route {} gave {}", route, a);
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
    for route in [0.0f32, 1.0] {
        let mut p = params(route);
        p.gain = f32::NAN;
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
    p.amp_x = 0.0;
    p.sine_amp = 1.0;
    p.sine_freq = 0.5;
    p.amp_y = 0.0;
    p.time = 0.5; // 0.25 cycle
    let quarter = run(&p, &mut globals());
    assert!((quarter - 1.0).abs() < 1e-5, "quarter cycle gave {}", quarter);
    p.time = 1.0; // 0.5 cycle
    let half = run(&p, &mut globals());
    assert!(half.abs() < 1e-5, "half cycle gave {}", half);
    p.time = 1.5; // 0.75 cycle
    let three_q = run(&p, &mut globals());
    assert!((three_q + 1.0).abs() < 1e-5, "three quarter cycle gave {}", three_q);
}
