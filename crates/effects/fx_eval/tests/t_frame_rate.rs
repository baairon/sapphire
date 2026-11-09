//! The frame rate host getter: the fold of a given rate through the context replacement, and the default of the minimal context.
use fx_eval::fold::{fold_lanes, frame_rate_value};

fn fold(v: Option<f32>) -> u32 {
    let h = v.map(|x| [x]);
    fold_lanes("get_frames_per_second", &[], 1, h.as_ref().map(|a| a.as_slice())).unwrap()[0]
}

#[test]
fn recorded_rates_pass_through() {
    assert_eq!(fold(Some(24.0)), 0x41c0_0000);
    assert_eq!(fold(Some(2997.0)), 0x453b_5000);
    assert_eq!(frame_rate_value(30000.0 / 1001.0).to_bits(), (30000f32 / 1001f32).to_bits());
}

#[test]
fn rates_below_the_threshold_give_the_default() {
    for v in [0.0f32, -1.0, 5e-5, f32::from_bits(0x38d1_b717)] {
        assert_eq!(fold(Some(v)), 30f32.to_bits(), "{}", v);
    }
    assert_eq!(fold(Some(f32::from_bits(0x38d1_b718))), 0x38d1_b718);
    assert!(f32::from_bits(fold(Some(f32::NAN))).is_nan());
}

#[test]
fn minimal_context_keeps_the_default() {
    assert_eq!(fold(None), 30f32.to_bits());
}
