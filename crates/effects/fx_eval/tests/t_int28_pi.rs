//! the Warp_Perspective_Autogen matte rows by record, the record table, and PARAM-INPUT-INDEX-UNRECORDED.
use fx_eval::ranges::{host_value_index_check, param_input_index, param_input_recorded, PARAM_INPUT_INDEX_BY_RECORD};

#[test]
fn pi_warp_perspective_autogen_rows_by_record() {
    if !effects_rtdata::have_tables() {
        return;
    }
    for n in ["blur_mask", "invert_mask", "mask_use"] {
        assert_eq!(param_input_index("Warp_Perspective_Autogen", n), Some(1), "{}", n);
        assert!(param_input_recorded("Warp_Perspective_Autogen", n), "{}", n);
        assert!(host_value_index_check("Warp_Perspective_Autogen", n).is_ok(), "{}", n);
    }
    assert!(PARAM_INPUT_INDEX_BY_RECORD.starts_with("PARAM-INPUT-INDEX-BY-RECORD (Warp_Perspective_Autogen: blur_mask, invert_mask, mask_use input index 1"));
}

#[test]
fn pi_unchanged_rows_keep_their_index() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // rows the CTX records confirm are unchanged (a sample of each index value), and the core Warp_Perspective rows keep the def order's 0
    assert_eq!(param_input_index("Diffuse_Autogen", "blur_mask"), Some(1));
    assert_eq!(param_input_index("Distort_RGB_Autogen", "mask_use"), Some(2));
    assert_eq!(param_input_index("Drop_Shadow", "matte_use"), Some(2));
    assert_eq!(param_input_index("Trails_Autogen", "blur_mask"), Some(1));
    assert_eq!(param_input_index("Warp_Perspective", "invert_mask"), Some(0));
    assert_eq!(param_input_index("Warp_Perspective", "mask_use"), Some(0));
}

#[test]
fn pi_unrecorded_row_refuses_a_host_value() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // negative test: rows without a record of the input index
    for (p, n) in [("Warp_Perspective", "invert_mask"), ("Warp_Perspective", "mask_use"), ("Blur_Color", "invert_mask"), ("Distort_RGB", "mask_use")] {
        assert!(!param_input_recorded(p, n), "{} {}", p, n);
        let e = host_value_index_check(p, n).unwrap_err();
        assert_eq!(e, format!("PARAM-INPUT-INDEX-UNRECORDED ({} {}: no record of the parameter's input index)", p, n));
    }
    // a parameter with no PARAM_INPUTS row is not touched by the check
    assert!(host_value_index_check("Warp_Perspective", "blur_amount").is_ok());
    assert!(host_value_index_check("Trails_Autogen", "blur_mask").is_ok());
}
