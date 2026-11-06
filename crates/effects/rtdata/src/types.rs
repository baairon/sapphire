//! Plain value types of the recorded host data. Every field is a value some effect code reads; nothing is stored as text to be parsed.

/// One registered host parameter of a style (a row of the host's registration of the style): by record name, with the name the host shows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyleParam {
    /// the program the host registered the parameter for (for example `Trails_Autogen`)
    pub program: &'static str,
    /// the style the program belongs to (for example `Trails`)
    pub style: &'static str,
    /// the parameter name in the program (`-` when the host shows a parameter that has no program name)
    pub record_name: &'static str,
    /// the name the host shows
    pub display_name: &'static str,
    /// the host's index of the parameter
    pub host_index: i32,
    /// the host's parameter type (2 = fixed slider)
    pub host_type: i32,
    /// the number of choices of a popup
    pub popup_choices: Option<i32>,
    /// the one-based checkout integer of a popup's registered default
    pub default_checkout_integer: Option<i32>,
    /// the first four bytes of the registered default value of a host type 2 row, as an int32 (16.16 fixed point)
    pub registered_value_bits: Option<i64>,
    /// the name of the recorded run the row was read from (printed in labels)
    pub record: &'static str,
}

/// A parameter record of the host curve route: what the host stores for a curve or point parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TickmapRecord {
    pub program: &'static str,
    pub param: &'static str,
    pub ptype: i64,
    pub nlanes: i64,
    pub kind_byte: i64,
    pub subtype_byte: i64,
    pub flags_a_byte: i64,
    pub flags_b_byte: i64,
    pub index: i64,
}

/// The value range of one program parameter and its text default.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RangeDef {
    pub program: &'static str,
    pub parameter: &'static str,
    /// scalar, boolean, popup, int, color, vector or angle
    pub ptype: &'static str,
    pub lanes: usize,
    pub min: f32,
    pub max: f32,
    /// the default as the program text writes it (one value per lane, space separated)
    pub default: &'static str,
    pub min_enforced: bool,
    pub max_enforced: bool,
}

/// The input an image or mask parameter belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParamInput {
    pub program: &'static str,
    pub parameter: &'static str,
    pub input_index: usize,
}

/// A recorded run confirms the input index of the parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParamInputRecord {
    pub program: &'static str,
    pub parameter: &'static str,
}

/// The normalize_scale of a registered parameter and its text default (None: not a single number).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NsParam {
    pub program: &'static str,
    pub param: &'static str,
    pub text_default: Option<f32>,
    pub normalize_scale: Option<f32>,
}

/// One style: its programs and the recorded data of its parameters.
#[derive(Debug)]
pub struct Style {
    pub name: &'static str,
    pub programs: &'static [&'static str],
    pub style_params: &'static [StyleParam],
    pub tickmap: &'static [TickmapRecord],
    pub ranges: &'static [RangeDef],
    pub inputs: &'static [ParamInput],
    pub input_records: &'static [ParamInputRecord],
    pub ns: &'static [NsParam],
}

/// One entry of a compiler combination table: the operand element counts it matches and the result count.
#[derive(Debug, Clone, Copy)]
pub struct ComboEntry {
    pub vals: &'static [i32],
    pub tag: i32,
}

/// The compiler's combination table of one class (entries in table order; the first match wins).
#[derive(Debug, Clone, Copy)]
pub struct ComboClass {
    pub class: &'static str,
    pub entries: &'static [ComboEntry],
}

/// One entry of the selector's table.
#[derive(Debug, Clone, Copy)]
pub struct ExecEntry {
    pub vals: &'static [i32],
    pub tag: i32,
    pub len: i32,
}

/// The selector's table of one class (entries in table order; the first match wins).
#[derive(Debug, Clone, Copy)]
pub struct ExecClass {
    pub class: &'static str,
    pub entries: &'static [ExecEntry],
}
