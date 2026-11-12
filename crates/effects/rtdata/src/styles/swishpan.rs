// Generated from the recorded host data of the styles; plain definitions, no file is read.
// Rows are the ones the effects read; each field is a value the code uses. Do not edit by hand.
//! Style SwishPan: program SwishPan.
use crate::types::*;

pub static STYLE: Style = Style {
    name: "SwishPan",
    programs: &["SwishPan"],
    style_params: &[
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "-", display_name: "Load Preset", param: "-", host_index: 1, host_type: 15, popup_choices: None, default_checkout_integer: None, registered_value_bits: None, record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "-", display_name: "Save Preset", param: "-", host_index: 2, host_type: 15, popup_choices: None, default_checkout_integer: None, registered_value_bits: None, record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "-", display_name: "Background", param: "-", host_index: 3, host_type: 0, popup_choices: None, default_checkout_integer: None, registered_value_bits: None, record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "transition_dir", display_name: "Transition Dir", param: "transition_dir", host_index: 4, host_type: 7, popup_choices: Some(2), default_checkout_integer: Some(1), registered_value_bits: None, record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "auto_trans", display_name: "Auto Trans", param: "auto_trans", host_index: 5, host_type: 7, popup_choices: Some(2), default_checkout_integer: Some(2), registered_value_bits: None, record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "wipe_percent", display_name: "Wipe Percent", param: "wipe_percent", host_index: 6, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(0), record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "direction", display_name: "Direction", param: "direction", host_index: 7, host_type: 7, popup_choices: Some(4), default_checkout_integer: Some(1), registered_value_bits: None, record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "blur_amount", display_name: "Blur Amount", param: "blur_amount", host_index: 8, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(31457278), record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "overlap", display_name: "Overlap", param: "overlap", host_index: 9, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(0), record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "slow_in", display_name: "Slow In", param: "slow_in", host_index: 10, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(32768), record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "slow_out", display_name: "Slow Out", param: "slow_out", host_index: 11, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(32768), record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "opacity", display_name: "Opacity", param: "opacity", host_index: 12, host_type: 7, popup_choices: Some(3), default_checkout_integer: Some(2), registered_value_bits: None, record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "swish_auto_trans", display_name: "Swish Auto Trans", param: "swish_auto_trans", host_index: 13, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(0), record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
        StyleParam { program: "SwishPan", style: "SwishPan", record_name: "-", display_name: "Help", param: "-", host_index: 14, host_type: 15, popup_choices: None, default_checkout_integer: None, registered_value_bits: None, record: "CTX_SwishPan_d_SwishPan_64_64x32.json" },
    ],
    tickmap: &[
        TickmapRecord { program: "SwishPan", param: "wipe_percent", ptype: 1, nlanes: 1, kind_byte: 1, subtype_byte: 64, flags_a_byte: 64, flags_b_byte: 0, index: 6 },
        TickmapRecord { program: "SwishPan", param: "wipe_amt", ptype: 1, nlanes: 1, kind_byte: 1, subtype_byte: 64, flags_a_byte: 64, flags_b_byte: 0, index: 6 },
    ],
    ranges: &[
        RangeDef { program: "SwishPan", parameter: "transition_dir", ptype: "popup", lanes: 1, min: 0.0, max: 1.0, default: "0", min_enforced: true, max_enforced: true },
        RangeDef { program: "SwishPan", parameter: "auto_trans", ptype: "popup", lanes: 1, min: 0.0, max: 1.0, default: "0", min_enforced: true, max_enforced: true },
        RangeDef { program: "SwishPan", parameter: "wipe_amt", ptype: "scalar", lanes: 1, min: 0.0, max: 1.0, default: "0", min_enforced: true, max_enforced: true },
        RangeDef { program: "SwishPan", parameter: "direction", ptype: "popup", lanes: 1, min: 0.0, max: 3.0, default: "0", min_enforced: true, max_enforced: true },
        RangeDef { program: "SwishPan", parameter: "blur_amount", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "2", min_enforced: true, max_enforced: false },
        RangeDef { program: "SwishPan", parameter: "overlap", ptype: "scalar", lanes: 1, min: -1e+06, max: 1e+06, default: "0", min_enforced: false, max_enforced: false },
        RangeDef { program: "SwishPan", parameter: "slow_in", ptype: "scalar", lanes: 1, min: 0.0, max: 1.0, default: "0.5", min_enforced: true, max_enforced: true },
        RangeDef { program: "SwishPan", parameter: "slow_out", ptype: "scalar", lanes: 1, min: 0.0, max: 1.0, default: "0.5", min_enforced: true, max_enforced: true },
        RangeDef { program: "SwishPan", parameter: "opacity", ptype: "popup", lanes: 1, min: 0.0, max: 2.0, default: "1", min_enforced: true, max_enforced: true },
        RangeDef { program: "SwishPan", parameter: "swish_auto_trans", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "0", min_enforced: true, max_enforced: false },
    ],
    inputs: &[
    ],
    input_records: &[
    ],
    ns: &[
        NsParam { program: "SwishPan", param: "blur_amount", text_default: Some(2.0), normalize_scale: Some(4.0) },
    ],
};
