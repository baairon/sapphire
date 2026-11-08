//! get_time folds to the explicit host value and has no default.
use fx_eval::fold::fold_lanes;

#[test]
fn get_time_takes_the_host_value_and_has_no_default() {
    let t = 7f32 / 24f32;
    assert_eq!(fold_lanes("get_time", &[], 1, Some(&[t])).unwrap()[0], t.to_bits());
    assert!(fold_lanes("get_time", &[], 1, None).is_none());
}
