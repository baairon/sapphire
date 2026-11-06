//! The recorded host data of the effects, written as Rust definitions.
//!
//! The effects need facts about the host's registration of their parameters (the host index and type of each parameter, popup choice
//! counts, value ranges, the input a mask belongs to, normalize_scale factors, curve parameter records) and about the primitive classes
//! (flag words, image-only and integer-typed arguments, combination tables). They are written here as plain values, one module per style
//! (`styles/`) and one for the class tables (`classes.rs`), each holding only the rows the 13 styles read. No file is read at run time and
//! no environment variable is consulted: the effects need no external data. A lookup of a row that is not defined answers as an absent row.
//!
//! The generated modules are written by a generator that is kept outside the repository; `types.rs` holds the value types.
//!
//! `testdata` is test support only: the full copies of the host tables that tests compare the definitions with and sweep, read from
//! `$VEC_DIR/effects/rtdata` (see its module documentation). No product code calls it.
pub mod classes;
pub mod styles;
pub mod testdata;
pub mod types;

pub use testdata::{bytes, dir, have_tables, missing_message, require_all, text, Table, ALL};
pub use types::*;

fn all_styles() -> impl Iterator<Item = &'static Style> {
    styles::STYLES.iter().copied()
}

/// The registered parameter of PROGRAM named NAME (record name): the first defined row.
pub fn style_param(program: &str, record_name: &str) -> Option<&'static StyleParam> {
    all_styles().flat_map(|s| s.style_params.iter()).find(|r| r.program == program && r.record_name == record_name)
}

/// The registered parameters of PROGRAM, in table order.
pub fn style_params_of(program: &str) -> impl Iterator<Item = &'static StyleParam> + '_ {
    all_styles().flat_map(|s| s.style_params.iter()).filter(move |r| r.program == program)
}

/// The curve parameter record of PROGRAM's parameter PARAM.
pub fn tickmap_record(program: &str, param: &str) -> Option<&'static TickmapRecord> {
    all_styles().flat_map(|s| s.tickmap.iter()).find(|r| r.program == program && r.param == param)
}

/// Every defined range row, in table order.
pub fn range_defs() -> impl Iterator<Item = &'static RangeDef> {
    all_styles().flat_map(|s| s.ranges.iter())
}

/// Every defined input row, in table order.
pub fn param_inputs() -> impl Iterator<Item = &'static ParamInput> {
    all_styles().flat_map(|s| s.inputs.iter())
}

/// True when a recorded run confirms the input index of PROGRAM's parameter PARAMETER.
pub fn has_input_record(program: &str, parameter: &str) -> bool {
    all_styles().flat_map(|s| s.input_records.iter()).any(|r| r.program == program && r.parameter == parameter)
}

/// The normalize_scale row of PROGRAM's parameter PARAM.
pub fn ns_param(program: &str, param: &str) -> Option<&'static NsParam> {
    all_styles().flat_map(|s| s.ns.iter()).find(|r| r.program == program && r.param == param)
}

/// The normalize_scale rows of PROGRAM, in table order.
pub fn ns_params_of(program: &str) -> impl Iterator<Item = &'static NsParam> + '_ {
    all_styles().flat_map(|s| s.ns.iter()).filter(move |r| r.program == program)
}

/// The class flag word of a primitive, None when the class has no row.
pub fn class_flag(name: &str) -> Option<u32> {
    classes::CLASS_FLAGS.iter().find(|(n, _)| *n == name).map(|(_, f)| *f)
}

/// True when argument ARG of PRIM must be an image.
pub fn img_only_arg(prim: &str, arg: usize) -> bool {
    classes::IMG_ONLY_ARGS.iter().any(|(p, a)| *p == prim && *a == arg)
}

/// True when argument ARG of PRIM is integer typed.
pub fn int_typed_arg(prim: &str, arg: usize) -> bool {
    classes::INT_TYPED_ARGS.iter().any(|(p, a)| *p == prim && *a == arg)
}
