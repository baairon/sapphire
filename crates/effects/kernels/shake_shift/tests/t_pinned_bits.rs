//! Output bits of the shake_shift body at inputs chosen for each route and each stage, needing no stored data. The expected words are
//! this repository's own output (as the golden table's are). The inputs of the first test are the ones where a fused multiply-add of the first
//! stage (`x13 * x10 + x12`, route A) and the same product rounded twice give different bits, so a change of that rounding fails here.
//! Stage 1 is a call with the guard word 0 (it builds the table); stage 2 is a call on the built table.
use shake_shift::body::{shake_shift_body, Config, Frame, Globals, Params, TAB_LEN};
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
    shake_shift_body(p, &Config::default(), g, &mut RefServices::new(), &mut Frame::new()).to_bits()
}

/// An ordinary binding with the route selector `sel` (0 is route A, 1 is route B).
fn plain(sel: f32) -> Params {
    Params { p4c: 0.37, p7c: 1.25, pac: 0.8, pdc: 2.5, p10c: 1.5, p13c: 0.6, p16c: 0.9, p19c: 3.0, p1cc: 0.11, p1fc: sel, p22c: 0.4, p25c: 0.7 }
}

/// Route A binding whose first-stage product and sum differ between one and two roundings; `d` is parameters 3 and 7 (equal).
fn fused_site(d: u32) -> Params {
    Params {
        p4c: f(0),
        p7c: f(0x3f00_0000),
        pac: f(0x3f4c_cccd),
        pdc: f(d),
        p10c: f(0x3fc0_0000),
        p13c: f(0x3f19_999a),
        p16c: f(0x3f66_6666),
        p19c: f(d),
        p1cc: f(0),
        p1fc: f(0),
        p22c: f(0x3ecc_cccd),
        p25c: f(0x3f33_3333),
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
    let a1 = run(&plain(0.0), &mut globals());
    let b1 = run(&plain(1.0), &mut globals());
    let b2 = run(&plain(1.0), &mut built());
    let a2 = run(&plain(0.0), &mut built());
    assert_eq!(a1, 0xbfc8_b309);
    assert_eq!(b1, 0xbfad_eb79);
    assert_eq!(b2, 0xbfad_eb79);
    assert_eq!(a2, 0xbfc8_b309);
}
