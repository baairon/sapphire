use fx_eval::units::angle_host_lane;
#[test]
fn swing_roll_negated() {
    assert_eq!(angle_host_lane("Warp_Perspective_Autogen", "swing", 25.0), Some(-25.0));
    assert_eq!(angle_host_lane("Warp_Perspective_Autogen", "roll", -15.0).map(|v| v.to_bits()), Some(15.0f32.to_bits()));
    // unbound +0 host gives the recorded -0.0 lane
    assert_eq!(angle_host_lane("Warp_Perspective", "swing", 0.0).map(|v| v.to_bits()), Some(0x8000_0000));
    assert_eq!(angle_host_lane("Warp_Perspective_Autogen", "latitude", 35.0), None);
    assert_eq!(angle_host_lane("Drop_Shadow", "swing", 1.0), None);
}
