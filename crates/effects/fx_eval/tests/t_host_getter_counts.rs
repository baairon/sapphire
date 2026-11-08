//! Every host scalar getter has the fixed element count 1, so a product folded from a getter (for example a blur width that
//! multiplies get_total_frames) is a typed constant and not an untyped node with count -1.
use fx_eval::compile::fixed_out;

#[test]
fn host_scalar_getters_have_one_element() {
    for c in ["get_time", "get_percent_time", "get_total_frames", "get_aspect_ratio", "get_frame_number", "get_frames_per_second"] {
        assert_eq!(fixed_out(c), Some(1), "{}", c);
    }
    assert_eq!(fixed_out("static2"), Some(2));
    assert_eq!(fixed_out("no_such_class"), None);
}
