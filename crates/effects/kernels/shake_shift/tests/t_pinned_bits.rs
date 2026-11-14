//! Output bits of the shake_shift body at inputs chosen for each route and each stage, needing no stored data. The expected words are
//! this repository's own output (as the golden table's are). The inputs of the first test are the ones where a fused multiply-add of the first
//! stage (`x13 * x10 + x12`, route A) and the same product rounded twice give different bits, so a change of that rounding fails here.
//! Stage 1 is a call with the guard word 0 (it builds the table); stage 2 is a call on the built table.
use shake_shift::body::{shake_shift_body, Config, Globals, Params, TAB_LEN};
use shake_shift::weights;
use shake_shift::RefServices;

fn f(w: u32) -> f32 {
    f32::from_bits(w)
}

fn globals() -> Globals {
    let wt = weights::build_avx();
    Globals { guard: 0, tab: vec![[0; 3]; TAB_LEN], tail: [0; 3], weights: wt.float.clone(), scale: weights::scale_from_cutoff(9074) }
}

fn run(p: &Params, g: &mut Globals) -> u32 {
    shake_shift_body(p, &Config::default(), g, &mut RefServices::new()).to_bits()
}

/// An ordinary binding with the route selector `sel` (0 is route A, 1 is route B).
fn plain(sel: f32) -> Params {
    Params { seed_x: 0.37, time: 1.25, amp_x: 0.8, rate_x: 2.5, sine_amp: 1.5, sine_freq: 0.6, amp_y: 0.9, rate_y: 3.0, seed_y: 0.11, route: sel, limit_gap: 0.4, gain: 0.7 }
}

/// Route A binding whose first-stage product and sum differ between one and two roundings; `d` is parameters 3 and 7 (equal).
fn fused_site(d: u32) -> Params {
    Params {
        seed_x: f(0),
        time: f(0x3f00_0000),
        amp_x: f(0x3f4c_cccd),
        rate_x: f(d),
        sine_amp: f(0x3fc0_0000),
        sine_freq: f(0x3f19_999a),
        amp_y: f(0x3f66_6666),
        rate_y: f(d),
        seed_y: f(0),
        route: f(0),
        limit_gap: f(0x3ecc_cccd),
        gain: f(0x3f33_3333),
    }
}

/// Globals of a stage 2 call: the table that a stage 1 call builds, the guard word 1 and the word past the table zero.
fn built() -> Globals {
    let mut b = globals();
    run(&plain(1.0), &mut b);
    assert_eq!(b.guard, 1);
    let mut g = globals();
    g.guard = 1;
    g.tab = b.tab.clone();
    g
}

#[test]
fn route_a_bits_at_the_fused_sites_of_the_second_stage() {
    assert_eq!(run(&fused_site(0xbeea_6911), &mut built()), 0x3f61_d125);
    assert_eq!(run(&fused_site(0xbe29_d573), &mut built()), 0x3f65_f042);
}

#[test]
fn bits_of_each_route_and_stage_on_an_ordinary_binding() {
    let plain_a = run(&plain(0.0), &mut globals());
    let plain_b = run(&plain(1.0), &mut globals());
    let built_a = run(&plain(1.0), &mut built());
    let built_b = run(&plain(0.0), &mut built());
    assert_eq!(plain_a, 0xbfc8_b309);
    assert_eq!(plain_b, 0xbfad_eb79);
    assert_eq!(built_a, 0xbfad_eb79);
    assert_eq!(built_b, 0xbfc8_b309);
}
