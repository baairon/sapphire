// Generated from the recorded host data of the styles; plain definitions, no file is read.
// Rows are the ones the effects read; each field is a value the code uses. Do not edit by hand.
//! Style Flicker: program Flicker.
use crate::types::*;

pub static STYLE: Style = Style {
    name: "Flicker",
    programs: &["Flicker"],
    style_params: &[
        StyleParam { program: "Flicker", style: "Flicker", record_name: "-", display_name: "Load Preset", host_index: 1, host_type: 15, popup_choices: None, default_checkout_integer: None, registered_value_bits: None, record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "-", display_name: "Save Preset", host_index: 2, host_type: 15, popup_choices: None, default_checkout_integer: None, registered_value_bits: None, record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "amplitude", display_name: "Amplitude", host_index: 3, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(13107), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "rand_luma_amp", display_name: "Rand Luma Amp", host_index: 4, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(65536), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "rand_color_amp", display_name: "Rand Color Amp", host_index: 5, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(0), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "rand_freq", display_name: "Rand Freq", host_index: 6, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(1966080), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "-", display_name: "Wave", host_index: 7, host_type: 13, popup_choices: None, default_checkout_integer: None, registered_value_bits: None, record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "wave_amp", display_name: "Wave Amp", host_index: 8, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(0), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "wave_freq", display_name: "Wave Freq", host_index: 9, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(327680), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "wave_r_phase", display_name: "Wave R Phase", host_index: 10, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(0), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "wave_g_phase", display_name: "Wave G Phase", host_index: 11, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(0), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "wave_b_phase", display_name: "Wave B Phase", host_index: 12, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(0), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "-", display_name: "", host_index: 13, host_type: 14, popup_choices: None, default_checkout_integer: None, registered_value_bits: None, record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "red_amp", display_name: "Red Amp", host_index: 14, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(65536), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "green_amp", display_name: "Green Amp", host_index: 15, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(65536), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "blue_amp", display_name: "Blue Amp", host_index: 16, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(65536), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "brightness", display_name: "Brightness", host_index: 17, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(65536), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "seed", display_name: "Seed", host_index: 18, host_type: 2, popup_choices: None, default_checkout_integer: None, registered_value_bits: Some(8060), record: "CTX_Flicker_d_Flicker_64_64x32.json" },
        StyleParam { program: "Flicker", style: "Flicker", record_name: "-", display_name: "Help", host_index: 19, host_type: 15, popup_choices: None, default_checkout_integer: None, registered_value_bits: None, record: "CTX_Flicker_d_Flicker_64_64x32.json" },
    ],
    tickmap: &[
    ],
    ranges: &[
        RangeDef { program: "Flicker", parameter: "amplitude", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "0.2", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "rand_luma_amp", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "1", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "rand_color_amp", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "0", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "rand_freq", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "30", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "wave_amp", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "0", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "wave_freq", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "5", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "wave_r_phase", ptype: "scalar", lanes: 1, min: -1e+06, max: 1e+06, default: "0", min_enforced: false, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "wave_g_phase", ptype: "scalar", lanes: 1, min: -1e+06, max: 1e+06, default: "0", min_enforced: false, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "wave_b_phase", ptype: "scalar", lanes: 1, min: -1e+06, max: 1e+06, default: "0", min_enforced: false, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "red_amp", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "1", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "green_amp", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "1", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "blue_amp", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "1", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "brightness", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "1", min_enforced: true, max_enforced: false },
        RangeDef { program: "Flicker", parameter: "seed", ptype: "scalar", lanes: 1, min: 0.0, max: 1e+06, default: "0.123", min_enforced: true, max_enforced: false },
    ],
    inputs: &[
    ],
    input_records: &[
    ],
    ns: &[
    ],
};
