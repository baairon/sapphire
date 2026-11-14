use optical_flow_entry::compose::{run_flow_at, FlowConstants};
use optical_flow_entry::placement::{flow_level_dims, placement};
use optical_flow_solve::relax::split_cols;

fn dims(w: i32, h: i32) -> Vec<(i32, i32)> {
    flow_level_dims(w, h, f32::from_bits(0x3f34_fdf4), 4)
}

fn consts() -> FlowConstants {
    FlowConstants { max_warps: 1, max_outer: 1, max_inner: 1, lambda: 0.1, use_median: 1, texture_scale: 0.5, std_smooth: 0.2 }
}

fn image(w: usize, h: usize, seed: f32) -> Vec<f32> {
    (0..w * h).map(|i| (((i % w) as f32) * 0.37 + ((i / w) as f32) * 0.21 + seed).sin()).collect()
}

#[test]
fn bit_5_of_the_anchor_moves_the_relax_head_at_width_1232() {
    let (w, h) = (1232, 8);
    let p0 = placement(w, h, &dims(w, h), 0);
    let p32 = placement(w, h, &dims(w, h), 32);
    let n = i64::from(w) - 1;
    // nest1_t is descriptor word 15
    let a = split_cols(n, p0.descriptor[15]);
    let b = split_cols(n, p32.descriptor[15]);
    assert_ne!(a, b);
    // below 1,200 elements the split does not read the phase
    assert_eq!(split_cols(1000, p0.descriptor[15]), split_cols(1000, p32.descriptor[15]));
}

#[test]
fn the_anchor_phase_pairs_follow_bit_5() {
    for (w, h) in [(64, 32), (72, 24), (128, 64)] {
        let p = placement(w, h, &dims(w, h), 32);
        assert_eq!((p.descriptor[9], p.descriptor[10]), (32, 16), "{w}x{h}");
    }
}

#[test]
fn width_72_outputs_are_byte_equal_for_anchors_0_and_32() {
    let (w, h) = (72usize, 24usize);
    let (a, b) = (image(w, h, 0.0), image(w, h, 0.4));
    let run = |ph: u32| {
        let p = run_flow_at(&a, &b, w as i32, h as i32, &consts(), ph, || false).expect("planes");
        p.flow_u.iter().chain(&p.flow_v).chain(&p.confidence).map(|x| x.to_bits()).collect::<Vec<u32>>()
    };
    assert_eq!(run(0), run(32));
}
