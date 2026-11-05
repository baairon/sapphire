//! NEG-2: the per-program parameter range table and the measured clamp rule, owned by the evaluator.
//!
//! The table is `data/PARAM_RANGES.tsv` (read at run time from the data folder, see `effects_rtdata`; format: program, parameter, type, lanes,
//! real_min, real_max, default, ...; text-derived from the shipped Autogen/layout, NEG1-A1). The rule is the
//! arithmetic of the original compiler pass -> (NEG-1 runs D, D2, D3):
//!   * the clamp is lane-wise on every bound parameter value, before fold and compile;
//!   * ORDER: the upper bound first, the lower bound last (`if v > max {v = max}; if v < min {v = min}`), which only matters when min > max;
//!   * plain compares: NaN fails both and passes through, -0 compares equal to 0 and is kept, +-inf clamp to the bounds;
//!   * a record bound of exactly +1e6 (as max) or -1e6 (as min), the create_param default, is NOT enforced (INFERRED from three probes, NEG1-A2).
//! Switch RANGES-OFF: `CompileOptions.ranges_off` or the environment variable `NEG2_RANGES_OFF` (any value) turns the clamp off.

use std::collections::HashMap;
use std::sync::OnceLock;

/// The embedded table (a copy of NEG-1 / DF-1 `PARAM_RANGES.tsv`).
pub static PARAM_RANGES_TSV: effects_rtdata::Table = effects_rtdata::Table::new("fx_eval/data/PARAM_RANGES.tsv");
/// INT6 (Batch 36 rule 5): the REC-HARNESS table (INT4's table, Drop_Shadow rows from the harness record PREC rows). A named state A option, never a default.
pub static PARAM_RANGES_REC_TSV: effects_rtdata::Table = effects_rtdata::Table::new("fx_eval/data/PARAM_RANGES_REC.tsv");
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
    /// INT4 (RNG1 PATCH 3 b): per-bound enforcement flags (columns 13 and 14, Y or N). A table without them falls back to the +-1e6 rule.
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
    /// The embedded table, parsed once.
    pub fn builtin() -> &'static RangeTable {
        static T: OnceLock<RangeTable> = OnceLock::new();
        static TR: OnceLock<RangeTable> = OnceLock::new();
        match range_set() {
            RangeSet::Real => T.get_or_init(|| RangeTable::parse(&PARAM_RANGES_TSV)),
            RangeSet::RecHarness => TR.get_or_init(|| RangeTable::parse(&PARAM_RANGES_REC_TSV)),
        }
    }
    /// Row of a program parameter. W224 (a): the lookup is EXACT. Every row is keyed by the program whose record it was measured on; there is no
    /// fallback from a wrapper name to its core name (the `_Autogen` strip and SHE3's exact-first fix are removed).
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

/// INT18 (W414 (c) step 7): clamp_lane and clamp_lanes (the clamp without per-bound flags) had no caller on any route; they are removed
/// from the crate and kept, unchanged, as test-local functions of tests/t8.rs, whose asserts are unchanged. The route clamp is clamp_lane_row.
/// The clamp of ONE lane with the per-bound enforcement flags (RNG1 PATCH 3 b). Operand order of the listing : vminss with the max cell first
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
        // NZ1 PATCH 2: `changed` is the float compare of the plugin (NaN on either side: changed), not a bit compare
        if !(y == *x) {
            changed = true;
        }
        *x = y;
    }
    Some(changed)
}

// ---------------------------------------------------------------- DSW3: the unbound-input exception

/// DSW3, MEASURED (Drop_Shadow PREC rows, n3 and n4) and READ : a parameter record whose input index is >= 0 and whose
/// input is not bound (bit of [ctx] clear) takes the copy-default path of and SKIPS the range clamp. `data/PARAM_INPUTS.tsv` lists
/// (program, parameter, input_index) read from the records after the original program load (tools/rec_inputs.py, emulator, observe-only).
pub static PARAM_INPUTS_TSV: effects_rtdata::Table = effects_rtdata::Table::new("fx_eval/data/PARAM_INPUTS.tsv");

/// INT28 step PI (W563 (b)): the PARAM_INPUTS rows that a record's input index cell confirms (tools/pi_fleet.py over the fleet's CTX records). The table rows were written by DSW3's tools/rec_inputs.py from the def-text parse; Warp_Perspective_Autogen's three rows held the def
/// order's 0 and take the registered program's 1 by record (WPR1 D2, L80e).
pub static PARAM_INPUT_RECORDS_TSV: effects_rtdata::Table = effects_rtdata::Table::new("fx_eval/data/PARAM_INPUT_RECORDS.tsv");

pub const PARAM_INPUT_INDEX_BY_RECORD: &str = "PARAM-INPUT-INDEX-BY-RECORD (Warp_Perspective_Autogen: blur_mask, invert_mask, mask_use input index 1 from the registered records' input index cell; WPR1, W563)";

/// True when a record's input index cell confirms the row (exact program name first, then the core name, as param_input_index).
pub fn param_input_recorded(program: &str, parameter: &str) -> bool {
    let has = |p: &str| PARAM_INPUT_RECORDS_TSV.lines().skip(1).any(|l| {
        let f: Vec<&str> = l.split('\t').collect();
        f.len() >= 2 && f[0] == p && f[1] == parameter
    });
    let exact = PARAM_INPUTS_TSV.lines().skip(1).any(|l| {
        let f: Vec<&str> = l.split('\t').collect();
        f.len() >= 2 && f[0] == program && f[1] == parameter
    });
    if exact {
        return has(program);
    }
    has(program.strip_suffix("_Autogen").unwrap_or(program))
}

/// W563 (b): a host value fed for a parameter whose PARAM_INPUTS row has no record refuses.
pub fn host_value_index_check(program: &str, parameter: &str) -> Result<(), String> {
    if param_input_index(program, parameter).is_some() && !param_input_recorded(program, parameter) {
        return Err(format!("PARAM-INPUT-INDEX-UNRECORDED ({} {}: no record of the parameter's input index; W563)", program, parameter));
    }
    Ok(())
}

/// Input index of a program parameter's record, if its record carries one (>= 0). A trailing `_Autogen` is ignored.
pub fn param_input_index(program: &str, parameter: &str) -> Option<usize> {
    static T: OnceLock<HashMap<(String, String), usize>> = OnceLock::new();
    let t = T.get_or_init(|| {
        let mut m = HashMap::new();
        for (i, line) in PARAM_INPUTS_TSV.lines().enumerate() {
            if i == 0 || line.trim().is_empty() {
                continue;
            }
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() >= 3 {
                if let Ok(k) = f[2].trim().parse::<usize>() {
                    m.insert((f[0].to_string(), f[1].to_string()), k);
                }
            }
        }
        m
    });
    // exact program name first (the wrapper's own records), then the core name
    if let Some(&k) = t.get(&(program.to_string(), parameter.to_string())) {
        return Some(k);
    }
    let p = program.strip_suffix("_Autogen").unwrap_or(program);
    t.get(&(p.to_string(), parameter.to_string())).copied()
}

/// NZ1 (W298, Batch 38 rule 8): the plugin's node store rewrites a node cell only when old and new DIFFER as floats (ucomiss), so +0 and -0 never
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
