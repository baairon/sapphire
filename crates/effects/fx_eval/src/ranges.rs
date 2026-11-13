//! The per-program parameter range table and the measured clamp rule, owned by the evaluator.
//!
//! The table is defined in `effects_rtdata` (styles/<style>.rs, field `ranges`: program, parameter, type, lanes, min, max, default and the
//! two enforcement flags; text-derived from the shipped Autogen/layout, NEG1-A1). The REC-HARNESS set is test data and holds no rows here. The rule is the
//! arithmetic of the original compiler pass (measurement runs D, D2, D3):
//!   * the clamp is lane-wise on every bound parameter value, before fold and compile;
//!   * ORDER: the upper bound first, the lower bound last (`if v > max {v = max}; if v < min {v = min}`), which only matters when min > max;
//!   * plain compares: NaN fails both and passes through, -0 compares equal to 0 and is kept, +-inf clamp to the bounds;
//!   * a record bound of exactly +1e6 (as max) or -1e6 (as min), the create_param default, is NOT enforced (INFERRED from three probes, NEG1-A2).
//! Switch RANGES-OFF: `CompileOptions.ranges_off` or the environment variable `NEG2_RANGES_OFF` (any value) turns the clamp off.

use std::collections::HashMap;
use std::sync::OnceLock;

/// Row labels of the core-key rows (RANGE-ROWS-CORE-COPY / RANGE-ROWS-CORE-BY-TEXT).
pub const RANGE_ROW_LABELS_TSV: &str = include_str!("../data/RANGE_ROW_LABELS.tsv");

/// Which embedded range table is active. REAL (the program text and the registered record) is the default; REC-HARNESS only by an explicit CLI option.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RangeSet {
    Real,
    RecHarness,
}
static RANGE_SET: OnceLock<RangeSet> = OnceLock::new();
/// Fix the table of this process once, before the first lookup. A second, different value is an error.
pub fn set_range_set(s: RangeSet) -> Result<(), String> {
    match RANGE_SET.set(s) {
        Ok(()) => Ok(()),
        Err(_) if RANGE_SET.get() == Some(&s) => Ok(()),
        Err(_) => Err("the range table of this process is already fixed to the other set".into()),
    }
}
pub fn range_set() -> RangeSet {
    *RANGE_SET.get_or_init(|| RangeSet::Real)
}
/// `--ranges REAL|REC-HARNESS` parsed from a command line (None = option absent; Some(Err) = unknown name).
pub fn range_set_from_args(a: &[String]) -> Option<Result<RangeSet, String>> {
    let p = a.iter().position(|x| x == "--ranges")?;
    Some(match a.get(p + 1).map(|s| s.as_str()) {
        Some("REAL") => Ok(RangeSet::Real),
        Some("REC-HARNESS") => Ok(RangeSet::RecHarness),
        other => Err(format!("--ranges takes REAL or REC-HARNESS, got {:?}", other)),
    })
}
/// The label of a core-key program's rows, if any.
pub fn row_label(program: &str) -> Option<String> {
    RANGE_ROW_LABELS_TSV.lines().skip(1).filter_map(|l| {
        let c: Vec<&str> = l.split('\t').collect();
        if c.len() >= 2 && c[0] == program { Some(c[1].to_string()) } else { None }
    }).next()
}

/// The create_param default bound that the original does not enforce (NEG1-A2).
pub const UNENFORCED_BOUND: f32 = 1.0e6;

#[derive(Debug, Clone, PartialEq)]
pub struct RangeRow {
    pub program: String,
    pub parameter: String,
    pub ptype: String,
    pub lanes: usize,
    pub min: f32,
    pub max: f32,
    pub default: String,
    /// (b): per-bound enforcement flags (columns 13 and 14, Y or N). A table without them falls back to the +-1e6 rule.
    pub min_enforced: bool,
    pub max_enforced: bool,
}

#[derive(Debug, Clone, Default)]
pub struct RangeTable {
    pub rows: Vec<RangeRow>,
    index: HashMap<(String, String), usize>,
}

impl RangeTable {
    /// Parse a table in PARAM_RANGES.tsv format (first line is the header). Malformed lines are skipped.
    pub fn parse(text: &str) -> RangeTable {
        let mut t = RangeTable::default();
        for l in text.lines().skip(1) {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() < 7 {
                continue;
            }
            let (min, max) = match (c[4].trim().parse::<f64>(), c[5].trim().parse::<f64>()) {
                (Ok(a), Ok(b)) => (a as f32, b as f32),
                _ => continue,
            };
            let row = RangeRow {
                program: c[0].to_string(),
                parameter: c[1].to_string(),
                ptype: c[2].to_string(),
                lanes: c[3].trim().parse().unwrap_or(1),
                min,
                max,
                default: c[6].to_string(),
                min_enforced: if c.len() > 13 { c[12].trim() != "N" } else { min != -UNENFORCED_BOUND },
                max_enforced: if c.len() > 13 { c[13].trim() != "N" } else { max != UNENFORCED_BOUND },
            };
            t.index.insert((row.program.clone(), row.parameter.clone()), t.rows.len());
            t.rows.push(row);
        }
        t
    }
    /// The table of the defined rows (`effects_rtdata`), in table order.
    pub fn from_defs() -> RangeTable {
        let mut t = RangeTable::default();
        for d in effects_rtdata::range_defs() {
            let row = RangeRow {
                program: d.program.to_string(),
                parameter: d.parameter.to_string(),
                ptype: d.ptype.to_string(),
                lanes: d.lanes,
                min: d.min,
                max: d.max,
                default: d.default.to_string(),
                min_enforced: d.min_enforced,
                max_enforced: d.max_enforced,
            };
            t.index.insert((row.program.clone(), row.parameter.clone()), t.rows.len());
            t.rows.push(row);
        }
        t
    }
    /// The embedded table, built once. The REC-HARNESS set (an explicit option of the stored state A runs) is test data: it holds no rows
    /// here, so no parameter of it is clamped; tests parse the stored copy with `parse`.
    pub fn builtin() -> &'static RangeTable {
        static T: OnceLock<RangeTable> = OnceLock::new();
        static TR: OnceLock<RangeTable> = OnceLock::new();
        match range_set() {
            RangeSet::Real => T.get_or_init(RangeTable::from_defs),
            RangeSet::RecHarness => TR.get_or_init(RangeTable::default),
        }
    }
    /// Row of a program parameter. the lookup is EXACT. Every row is keyed by the program whose record it was measured on; there is no
    /// fallback from a wrapper name to its core name (the `_Autogen` strip and the recorded exact-first fix are removed).
    pub fn lookup(&self, program: &str, parameter: &str) -> Option<&RangeRow> {
        self.index.get(&(program.to_string(), parameter.to_string())).map(|&i| &self.rows[i])
    }
    /// A claimed program: the table holds at least one row keyed by exactly this name.
    pub fn has_program(&self, program: &str) -> bool {
        self.rows.iter().any(|r| r.program == program)
    }
    pub fn programs(&self) -> Vec<String> {
        let mut v: Vec<String> = self.rows.iter().map(|r| r.program.clone()).collect();
        v.dedup();
        v
    }
}

/// clamp_lane and clamp_lanes (the clamp without per-bound flags) had no caller on any route; they are removed
/// from the crate and kept, unchanged, as test-local functions of tests/t8.rs, whose asserts are unchanged. The route clamp is clamp_lane_row.
/// The clamp of ONE lane with the per-bound enforcement flags (b). Operand order of the listing : vminss with the max cell first
/// (`max < v ? max : v`: above max gives the max cell, equal and NaN keep the value), then vmaxss with the min cell first (`min > v ? min : v`: below min gives the min cell, equal and NaN keep the value; -0 stays).
pub fn clamp_lane_row(x: f32, r: &RangeRow) -> f32 {
    let mut v = x;
    if r.max_enforced && r.max < v {
        v = r.max;
    }
    if r.min_enforced && r.min > v {
        v = r.min;
    }
    v
}

/// RANGES-OFF through the environment (any value).
pub fn ranges_off_env() -> bool {
    std::env::var_os("NEG2_RANGES_OFF").is_some()
}

/// Clamp the bound value of `program.parameter` with the embedded table. `None` = no row (value unchanged), `Some(changed)`.
pub fn clamp_param(program: &str, parameter: &str, v: &mut [f32]) -> Option<bool> {
    let r = RangeTable::builtin().lookup(program, parameter)?;
    let mut changed = false;
    for x in v.iter_mut() {
        let y = clamp_lane_row(*x, r);
        // `changed` is the float compare of the plugin (NaN on either side: changed), not a bit compare
        if !(y == *x) {
            changed = true;
        }
        *x = y;
    }
    Some(changed)
}

// ---------------------------------------------------------------- the unbound-input exception

/// A parameter whose PARAM_INPUTS row names an input that is not bound (its bit of the bound-input mask is clear) takes the copy-default path and skips the range clamp
/// (measured on the Drop_Shadow range rows). `data/PARAM_INPUTS.tsv` lists (program, parameter, input_index); defined in effects_rtdata (field `inputs`).
///
/// Contract of the PARAM_INPUTS rows: the input index of each row is the position of the input in the registered program's input list (the layout order),
/// so the compile uses it as given. For Warp_Perspective_Autogen the three rows blur_mask, invert_mask and mask_use name input 1.
/// The rows that a record's input index cell confirms are listed in effects_rtdata (field `input_records`); only `host_value_index_check` reads them.
pub const PARAM_INPUT_INDEX_BY_RECORD: &str = "PARAM-INPUT-INDEX-BY-RECORD (Warp_Perspective_Autogen: blur_mask, invert_mask, mask_use input index 1 from the registered records' input index cell)";

/// True when a record's input index cell confirms the row (exact program name first, then the core name, as param_input_index).
pub fn param_input_recorded(program: &str, parameter: &str) -> bool {
    let has = |p: &str| effects_rtdata::has_input_record(p, parameter);
    let exact = effects_rtdata::param_inputs().any(|r| r.program == program && r.parameter == parameter);
    if exact {
        return has(program);
    }
    has(program.strip_suffix("_Autogen").unwrap_or(program))
}

/// a host value fed for a parameter whose PARAM_INPUTS row has no record refuses.
pub fn host_value_index_check(program: &str, parameter: &str) -> Result<(), String> {
    if param_input_index(program, parameter).is_some() && !param_input_recorded(program, parameter) {
        return Err(format!("PARAM-INPUT-INDEX-UNRECORDED ({} {}: no record of the parameter's input index)", program, parameter));
    }
    Ok(())
}

/// Input index of a program parameter's record, if its record carries one (>= 0). A trailing `_Autogen` is ignored.
pub fn param_input_index(program: &str, parameter: &str) -> Option<usize> {
    let find = |p: &str| effects_rtdata::param_inputs().find(|r| r.program == p && r.parameter == parameter).map(|r| r.input_index);
    // exact program name first (the wrapper's own records), then the core name
    if let Some(k) = find(program) {
        return Some(k);
    }
    find(program.strip_suffix("_Autogen").unwrap_or(program))
}

/// the plugin's node store rewrites a node cell only when old and new DIFFER as floats (ucomiss), so +0 and -0 never
/// replace each other; NaN on either side counts as different (written). `old` is the node's previous cell (the program default at genotype creation, then the previous
/// compile's stored value: COMPILE-HISTORY); a lane without an old value is written.
pub fn node_store(old: &[f32], new: &mut [f32]) {
    for (i, x) in new.iter_mut().enumerate() {
        if let Some(&o) = old.get(i) {
            if o == *x {
                *x = o;
            }
        }
    }
}
