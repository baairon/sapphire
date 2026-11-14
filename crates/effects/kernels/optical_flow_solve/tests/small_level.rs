//! Refusal of a level table with a side below 2: no host call, no plane written; a table with every side at 2 proceeds.
use optical_flow_solve::body::*;
#[path = "support/region_store.rs"]
mod region_store;
use region_store::{run_body, World};

const ORIGIN: u64 = 0x10_0000;

fn table(side: i32) -> [Level; 3] {
    let p = |k: usize, i: usize| ORIGIN + ((k * 24 + i) as u64) * 0x4000;
    let mk = |k: usize| Level {
        width: side,
        height: side,
        stride: side,
        rect_word3: 0,
        elem_bytes: p(k, 2),
        index: p(k, 3) as u32,
        image_a: p(k, 4),
        image_b: p(k, 5),
        grad_x: p(k, 6),
        grad_y: p(k, 7),
        warped_image: p(k, 8),
        warped_grad_x: p(k, 9),
        warped_grad_y: p(k, 10),
        coeff: p(k, 11),
        flow_u: p(k, 12),
        flow_v: p(k, 13),
        flow_u_next: p(k, 14),
        flow_v_next: p(k, 15),
        nest1_t: p(k, 16),
        nest1_u: p(k, 17),
        nest2_t: p(k, 18),
        nest2_u: p(k, 19),
        relax_first: p(k, 20),
        relax_second: p(k, 21),
        median_out: p(k, 22),
        copied_plane: p(k, 23),
    };
    [mk(0), mk(1), mk(2)]
}

fn quiet() -> Params {
    Params { outer_iters: 0, mid_iters: 0, inner_iters: 0, counted_route: 0, flow_gain: 0, relax_scale: 0, tol: 0.0, smooth_bias: 0, alt_launches: 0 }
}

fn world() -> World {
    World { pl: vec![1.0; 76 * 0x1000 + 0x400], origin: ORIGIN, acc: 0.0, launches: vec![] }
}

#[test]
fn every_side_below_two_is_refused_before_any_call_or_write() {
    let before = world().pl;
    for level in 0..3usize {
        for which in 0..2 {
            for v in [1i32, 0, -1, i32::MIN] {
                let mut t = table(2);
                let (w, h) = (if which == 0 { v } else { 2 }, if which == 1 { v } else { 2 });
                t[level].width = w;
                t[level].height = h;
                let mut w = world();
                assert_eq!(run_body(&t, 2, &quiet(), &[], &mut w), Err(Refused::SmallLevel));
                assert!(w.launches.is_empty() && w.pl == before);
            }
        }
    }
}

#[test]
fn a_table_with_every_side_at_two_proceeds() {
    let t = table(2);
    let mut w = world();
    assert_eq!(run_body(&t, 2, &quiet(), &[], &mut w), Ok(()));
    assert!(!w.launches.is_empty());
}

#[test]
fn levels_above_the_first_level_are_not_judged() {
    let mut t = table(2);
    t[2].width = 0;
    t[2].height = 0;
    let mut w = world();
    assert_eq!(run_body(&t, 1, &quiet(), &[], &mut w), Ok(()));
}
