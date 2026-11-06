//! NV-2: the compiler's result-changing semantics, natively (label EVALUATOR-UNCHECKED).
//!
//! Rules are numbered R1.. and tabulated in README.md with their listing address and
//! MEASURED / INFERRED status. Listing offsets (VA = offset). Source of truth:
//! `neutral_1.asm` functions (set elnum), (visit node: forward, prune, elnum,
//! alias check), (prune dispatch), class slot 38 of the if family (if+,
//!  if>, if_eq, if_neq_const, switch, if_has_alpha)
//! with the constant-compare helpers,
//! and the selector (COMBO_SELECT.md, tables in COMBOS.tsv).
//!
//! NV-3: collapse (slot 38) and folding are ported in fold.rs; see README.

use crate::build::{GNode, Genotype, NodeKind};
use crate::fold::{can_fold_el, collapse_arm, fold_kind, fold_lanes, FoldKind};
use std::collections::{BTreeMap, HashMap, HashSet};

// ---------------------------------------------------------------- combos

#[derive(Debug, Clone)]
pub struct Combo {
    pub vals: Vec<i32>,
    pub tag: i32,
}

/// Class combo tables (defined in effects_rtdata; EV-T). Selection is the first entry whose values equal
/// the per-argument element counts (R3, MEASURED by EV-T over 27,650 emulator cases).
#[derive(Debug, Clone, Default)]
pub struct ComboTable {
    pub classes: HashMap<String, Vec<Combo>>,
}

impl ComboTable {
    /// The combination tables of the classes the effects use, as defined in effects_rtdata (entries in table order).
    pub fn from_defs() -> ComboTable {
        let mut t = ComboTable::default();
        for c in effects_rtdata::classes::COMBOS {
            let v: Vec<Combo> = c.entries.iter().map(|e| Combo { vals: e.vals.to_vec(), tag: e.tag }).collect();
            t.classes.insert(c.class.to_string(), v);
        }
        t
    }

    /// A table parsed from `class .. args tag ..` TSV text (tests that sweep a stored copy).
    pub fn from_tsv(text: &str) -> ComboTable {
        let mut t = ComboTable::default();
        for (i, line) in text.lines().enumerate() {
            if i == 0 || line.is_empty() {
                continue;
            }
            let f: Vec<&str> = line.split('\t').collect();
            if f.len() < 11 {
                continue;
            }
            let args = f[9].trim();
            let tag: i32 = match f[10].trim().parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            if args.is_empty() {
                continue;
            }
            let mut vals = Vec::new();
            for a in args.split(',') {
                match a.trim().parse::<i32>() {
                    Ok(v) => vals.push(v),
                    Err(_) => {
                        vals.clear();
                        break;
                    }
                }
            }
            if vals.is_empty() {
                continue;
            }
            t.classes.entry(f[0].to_string()).or_default().push(Combo { vals, tag });
        }
        t
    }

    /// first entry (table order) with `vals[i] == operands[i]` for all i < n; vals past
    /// the entry length are -1 and never match. None = the selector's -1.
    pub fn select(&self, class: &str, operands: &[i32]) -> Option<i32> {
        let tab = self.classes.get(class)?;
        for e in tab {
            let ok = operands.iter().enumerate().all(|(i, o)| e.vals.get(i).copied().unwrap_or(-1) == *o);
            if ok {
                return Some(e.tag);
            }
        }
        None
    }

    pub fn has(&self, class: &str) -> bool {
        self.classes.contains_key(class)
    }
}

// ---------------------------------------------------------------- compile

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CKind {
    Const,
    Func,
    Input,
    Special,
}

/// A compiler node (the original's -byte record, reduced to the fields that change results).
#[derive(Debug, Clone)]
pub struct CNode {
    pub id: usize,
    pub kind: CKind,
    /// primitive name (Func) / input name (Input, Special)
    pub name: String,
    /// parameter name when this constant is a parameter
    pub param: Option<String>,
    pub args: Vec<usize>,
    /// four constant lanes as f32 bits (last element replicated, R7)
    pub lanes: [u32; 4],
    pub nlanes: usize,
    pub elnum: Option<i32>,
    /// index of the argument this node collapses to (prune), R5
    pub alias: Option<usize>,
    /// node holds constants (kind Const, or an aliased node that became constant)
    pub is_const: bool,
    pub auto: Option<&'static str>,
    pub folded: bool,
    /// FLD1: the class whose code produced a folded constant (the class of a folded function node, or of the alias target of an R6 constant); empty for parameter constants
    pub cls: String,
    pub input_pos: usize,
}

#[derive(Debug, Clone)]
pub struct PrunedRec {
    pub name: String,
    pub class: String,
    /// argument index kept
    pub arm: usize,
    pub arms: usize,
}

#[derive(Debug, Clone)]
pub struct PlanNode {
    /// genotype node id (auto nodes: ids past the genotype)
    pub gid: usize,
    pub idx: usize,
    pub name: String,
    pub kind: &'static str,
    pub prim: String,
    pub elnum: i32,
    pub fmt: &'static str,
    pub args: Vec<usize>,
    pub lanes: [u32; 4],
    pub param: Option<String>,
    pub identity: bool,
    /// all arguments are constants (a folding candidate, FOLDING-UNMEASURED; nothing is folded)
    pub all_const_args: bool,
    /// const flag (constant node, folded node or alias-to-constant)
    pub b160: bool,
    pub folded: bool,
    /// FLD1: class of a folded constant (see CNode.cls)
    pub cls: String,
    pub input_pos: usize,
}

#[derive(Debug, Clone)]
pub struct CompiledProgram {
    pub program: String,
    pub paramset: String,
    pub nodes: Vec<PlanNode>,
    pub root: usize,
    pub pruned: Vec<PrunedRec>,
    /// type errors and unknown classes (strict mode turns the first one into an Err)
    pub diagnostics: Vec<String>,
    /// the parameter binding used: (name, lane bits)
    pub params: Vec<(String, Vec<u32>)>,
    pub labels: Vec<&'static str>,
    pub folds: Vec<String>,
    pub node_state: Vec<NodeState>,
    /// DSW5: clone nodes made by the pass (one rule, see `clone_pass`); only their folded lanes are observable in the W113 bits comparison
    pub clones: Vec<NodeState>,
    /// NZ1: the node cells of this compile (genotype node index -> lane bits) of every parameter node, to be passed as `CompileOptions::history` to the next compile of the same instance
    pub cells: BTreeMap<usize, Vec<u32>>,
    /// STK2: stack word 2 of the call to (guard); the preserve-group model runs only when set
    pub ctx_word2: bool,
    /// STK2: has-data flags [owner+i] as bits (bit j + 3 = declared input j), read by the model of preserve.rs
    pub ctx_has_data: u32,
    /// XWP2 (W746): per node id (CNode id = PlanNode gid) computed by the model of BEFORE the fold (planner order READ:
    /// ); read by preserve.rs only when `xy_place` is set
    pub fx_eval_pres_a_pre: HashMap<usize, bool>,
    /// XWP2: the xy placement model is on (preserve.rs: the xy clones are not frame dependent, from `fx_eval_pres_a_pre`)
    pub xy_place: bool,
}

#[derive(Debug, Clone, Default)]
pub struct CompileOptions {
    /// XWP2 (W746): skip the Special sharing call 2 of (one xy clone per consumer slot survives); used by xyplace::compile_placed only
    pub xy_unshared: bool,
    /// XWP2: the xy placement model (see `CompiledProgram::xy_place`)
    pub xy_place: bool,
    /// XWP2: preserve group ids by node id, given to the group pass of the sharing step (READ: runs, before)
    pub xy_pres_keys: Option<HashMap<usize, i64>>,
    /// parameter overrides (name -> lanes); anything absent takes the PROGRAM-DEFAULT (W83)
    pub overrides: BTreeMap<String, Vec<f32>>,
    /// element count per input name; absent: Source-like inputs 4, `xy` 2 (INFERRED)
    pub input_elnum: BTreeMap<String, i32>,
    pub strict: bool,
    pub paramset: String,
    /// disable constant folding (NV-2 behaviour)
    pub no_fold: bool,
    /// unbound inputs the harness binds to a constant (name -> lanes), e.g. Mask = 1 (NV3-A5)
    pub const_inputs: BTreeMap<String, Vec<f32>>,
    /// host getter lanes (get_frame_number, get_curve_val); absent = zeros
    pub host: BTreeMap<String, Vec<f32>>,
    /// NV3b HOST-CONTEXT-MINIMAL binding (M2, W112): every input with a declared default is bound to that
    /// constant, parameters flagged `default_to_1_if_postdiv` default to 1 (M11), Source has 3 channels for
    /// `no_alpha_only` programs (the original rejects 4). Off by default (NV-5 binds real images).
    pub minimal_context: bool,
    /// MASK1: inputs that stay real images in the minimal context (name -> channel count); the default constant binding is skipped for them
    pub bound_elnum: BTreeMap<String, i32>,
    /// DSW5 (W215 (c), W216 (d)): the bound-input mask of the compile context ([ctx]); bit k set = input k is bound. EXPLICIT: input 0 is never implied.
    /// Default 0 = the plan harness's zero-filled compile context. Inputs named in `bound_elnum` (MASK1 bindings) set their bit (position in the declared input list) in addition.
    /// A parameter whose record input index (`data/PARAM_INPUTS.tsv`) is >= 0 and whose bit is clear is NOT clamped . No environment override (INT6, Batch 36 rule 1).
    pub bound_input_mask: u32,
    /// DSW3: turn the unbound-input exception (no clamp for a parameter whose record input is unbound) off (also: env DSW3_UNBOUND_OFF)
    pub unbound_exception_off: bool,
    /// SHE4 (W215 b): host mocha data present ([this] non-null); the native compiler errors when set (MOCHA-OFF)
    pub mocha_data_present: bool,
    /// HOST-CURVES (CV1-A1): an unkeyed host curve value is the parameter's own current cells, so
    /// `get_curve_val p ..` folds to the lanes of its first argument. Off: zeros (the EP-3 harness read zeros).
    pub host_curves_unkeyed: bool,
    /// HCK1 (W275, Batch 36 rule 6): HOST-CURVES keyed per (curve name, requested time). Key = (parameter name of the first argument of
    /// `get_curve_val`, f32 bits of the time argument); value = the answer lanes the host gave. `Some(map)` is the keyed input: a request the map does not
    /// answer (no entry, or a non-constant time argument) is a compile error "HC-KEYED-MISSING" and no default exists. `None` leaves the earlier
    /// interfaces (the unkeyed state A `host_curves_unkeyed`, or zeros). The keyed input takes precedence over both.
    pub host_keyed: Option<BTreeMap<(String, u32), Vec<f32>>>,
    /// W479/W490 (TMR1, step T): HOST-CURVE-VALUES, the claimed get_curve_val input. When set, the fold calls tickmap::map_param (hostcurve.rs). The keyed and unkeyed modes are test interfaces and are refused together with it.
    pub host_curve_values: Option<crate::hostcurve::HostCurveValues>,
    /// INT8 (Batch 40 rule 4, W310) HC-NO-INTERFACE: a get_curve_val request with neither `host_keyed` nor `host_curves_unkeyed` set is a compile error.
    /// `hc_zero_legacy` is the named option HC-ZERO-LEGACY (the zero fold the EP-3 state A harness read) for the definitions of state A suites that need it;
    /// never in counted runs or frozen predictions. No environment switch.
    pub hc_zero_legacy: bool,
    /// NEG-2 RANGES-OFF: do not clamp the bound parameter values with the range table (also: env NEG2_RANGES_OFF)
    pub ranges_off: bool,
    /// NZ1 COMPILE-HISTORY: the node cells left by the previous compile of this program instance (`CompiledProgram::cells`); None = a fresh instance after the setup commands
    /// (the cell is the program-text default). Explicit, no default other than None.
    pub history: Option<BTreeMap<usize, Vec<u32>>>,
    /// INT9 (Batch 41 rule 4, W323; WPG15 PATCH 2) INPUT-PRESENCE-MASK: the byte [H] (H = [ctx]; bit j = declared input j has a host layer and data,
    /// writer) as ONE native input. Both readers are derived from it: the range pass (`bt [owner], input_index`input_index = record
    /// field = declared position j) and the bind pass (`bt byte [compiler], r8d` with r8d = node field minus 3; the node
    /// index of declared input j is j + 3, INFERRED from the same r8 indexing the program's per-input tables and and from EP-3b's "Source =
    /// input number 3"; so both readers test bit j). A set bit keeps input j an image: its channel count must be given in `bound_elnum` (a host fact, no default);
    /// a clear bit binds an input with a declared default to that default. The native has no compile cache (the cache key reader is not modelled).
    /// None = the state A form (`bound_input_mask` for the range pass and `bound_elnum` for the bind pass given separately).
    pub presence_mask: Option<u8>,
    /// INT9: the named option MIXED-MASK (attribution only): allow `bound_input_mask` or `bound_elnum` to disagree with `presence_mask`.
    pub mixed_mask: bool,
    /// INT9 internal: set when the options were derived from `presence_mask` (inputs with a set bit are not aliased to the input their default names).
    pub presence_derived: bool,
    /// INT10 (BCX1 PATCH 2): `__do_postdiv__` as a host input (the SMART context compiles 1 for Blur_Color, BCX1, from the plan text).
    /// None = the state A derivation from the program flag `dynamic` (glue: instance, MEASURED by the compiler reader on three effects).
    pub do_postdiv: Option<bool>,
    /// STK2: stack word 2 (see `CompiledProgram::ctx_word2`); default false = the existing word 2 = 0 behaviour
    pub ctx_word2: bool,
    /// STK2: has-data flags (see `CompiledProgram::ctx_has_data`)
    pub ctx_has_data: u32,
    /// step DX2 v2 (W652 (d)): a v3_to_v4_if4 node takes its bound image argument's count (the count the route records) only when true;
    /// fx_run sets it on the 'ae32' route only; every other route keeps the node's own count and the earlier compile
    pub v3_bound_count: bool,
}

/// INT9 (Batch 41 rule 4): the per-program bit-order rows of the INPUT-PRESENCE-MASK: (declared position j, input name, declared default, bind-pass node
/// index j + 3 (INFERRED), bind-pass bit j, range-pass bit j). Inputs past position 7 have no bit in the byte.
pub fn presence_bit_rows(g: &Genotype) -> Vec<(usize, String, String, usize, Option<usize>, Option<usize>)> {
    g.inputs
        .iter()
        .enumerate()
        .map(|(j, i)| {
            let b = if j < 8 { Some(j) } else { None };
            (j, i.name.clone(), i.default.clone().unwrap_or_else(|| "-".into()), j + 3, b, b)
        })
        .collect()
}

/// INT9: derive the compile options of one INPUT-PRESENCE-MASK byte (see `CompileOptions::presence_mask`). Errors: MIXED-MASK (a separately given range mask
/// or bind entry disagrees with the byte, unless the named option `mixed_mask`), INPUT-CHANNELS-REQUIRED (a set bit for an input with a declared default and no
/// channel count), PRESENCE-MASK-WIDTH (a set bit with no declared input).
pub fn derive_presence(g: &Genotype, opt: &CompileOptions) -> Result<(CompileOptions, String), CompileError> {
    let m = match opt.presence_mask {
        Some(m) => m,
        None => return Ok((opt.clone(), String::new())),
    };
    let mut o = opt.clone();
    let err = |s: String| CompileError { msg: s };
    if opt.bound_input_mask != 0 && opt.bound_input_mask != m as u32 && !opt.mixed_mask {
        return Err(err(format!("MIXED-MASK: range-pass mask 0x{:x} given beside INPUT-PRESENCE-MASK 0x{:02x} (one cell, one input; attribution only under the named option --mixed-mask)", opt.bound_input_mask, m)));
    }
    if (m as usize) >> g.inputs.len().min(8) != 0 {
        return Err(err(format!("PRESENCE-MASK-WIDTH: INPUT-PRESENCE-MASK 0x{:02x} sets a bit above the {} declared inputs of {}", m, g.inputs.len(), g.name)));
    }
    let mut bits = Vec::new();
    for (j, i) in g.inputs.iter().enumerate() {
        let set = j < 8 && (m >> j) & 1 == 1;
        let named = opt.bound_elnum.contains_key(&i.name);
        if set && i.default.is_some() && !named {
            return Err(err(format!("INPUT-CHANNELS-REQUIRED: INPUT-PRESENCE-MASK bit {} keeps input {} an image; its channel count is a host input with no default", j, i.name)));
        }
        if !set && named && !opt.mixed_mask {
            return Err(err(format!("MIXED-MASK: input {} is bound as an image but bit {} of INPUT-PRESENCE-MASK 0x{:02x} is clear", i.name, j, m)));
        }
        if !set {
            o.bound_elnum.remove(&i.name);
        }
        bits.push(format!("{}:{}={}", j, i.name, if set { 1 } else { 0 }));
    }
    if !opt.mixed_mask {
        o.bound_input_mask = m as u32;
    }
    o.presence_mask = None;
    o.presence_derived = true;
    let label = format!(
        "INPUT-PRESENCE-MASK 0x{:02x} ([H]; one native input; bind pass bit = node index - 3 = declared position, range pass bit = = declared position; bits {}; range mask used 0x{:x}{}; entry path and compile cache not modelled)",
        m,
        bits.join(" "),
        o.bound_input_mask,
        if opt.mixed_mask { "; MIXED-MASK (attribution only)" } else { "" }
    );
    Ok((o, label))
}

#[derive(Debug, Clone)]
pub struct CompileError {
    pub msg: String,
}
impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

pub const COND_CLASSES: [&str; 8] = ["if+", "if>", "if_neq_const", "if_eq", "switch", "if_has_alpha", "if+_coerce", "if>_coerce"];

// eq tolerance of the compare helper (double constant): VALUE NOT READ
// (data section not in the listing). 1e-6 is a placeholder (UNKNOWN); integers compare exactly.
const EQ_EPS: f64 = crate::fold::ZERO_EPS;

fn f32b(x: f32) -> u32 {
    x.to_bits()
}

fn bits_f(b: u32) -> f32 {
    f32::from_bits(b)
}

fn lanes_of(vals: &[f32]) -> ([u32; 4], usize) {
    let n = vals.len().clamp(1, 4);
    let mut l = [0u32; 4];
    for k in 0..4 {
        let src = if k < n { vals.get(k).copied().unwrap_or(0.0) } else { vals.get(n - 1).copied().unwrap_or(0.0) };
        l[k] = f32b(src);
    }
    (l, n)
}

struct ArgView {
    is_const: bool,
    lanes: [f32; 4],
    n: usize,
    elnum: Option<i32>,
}

fn view(nodes: &[CNode], id: usize) -> ArgView {
    let c = &nodes[id];
    let n = c.elnum.unwrap_or(c.nlanes as i32).clamp(1, 4) as usize;
    ArgView { is_const: c.is_const, lanes: [bits_f(c.lanes[0]), bits_f(c.lanes[1]), bits_f(c.lanes[2]), bits_f(c.lanes[3])], n, elnum: c.elnum }
}

/// All lanes of `a` (first n of arg0) satisfy `cmp(lane, other)` (shape).
fn all_lanes(a: &ArgView, b: Option<&ArgView>, t: f32, cmp: &dyn Fn(f32, f32) -> bool) -> bool {
    if !a.is_const {
        return false;
    }
    if let Some(b) = b {
        if !b.is_const {
            return false;
        }
    }
    (0..a.n).all(|i| {
        let other = match b {
            Some(b) => b.lanes[i.min(3)],
            None => t,
        };
        cmp(a.lanes[i.min(3)], other)
    })
}

/// INT18 (W422 (c), Batch 51 rule 8): the lane-count scan. Each eq_const or eq_const_int call with two constant operands counts one call;
/// a call whose operand counts differ counts one row (the listing reads b past its count where the native pads). Read and reset by
/// eq_const_scan_take; fx_run prints the totals as a label and plan_dump as an EQCONST line. Observe only: no result changes.
static EQ_SCAN_CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
static EQ_SCAN_DIFF: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn eq_scan_note(a: &ArgView, b: &ArgView) {
    use std::sync::atomic::Ordering::Relaxed;
    EQ_SCAN_CALLS.fetch_add(1, Relaxed);
    if a.n != b.n {
        EQ_SCAN_DIFF.fetch_add(1, Relaxed);
    }
}

/// (calls with two constant operands, calls whose operand counts differ) since the last take.
pub fn eq_const_scan_take() -> (usize, usize) {
    use std::sync::atomic::Ordering::Relaxed;
    (EQ_SCAN_CALLS.swap(0, Relaxed), EQ_SCAN_DIFF.swap(0, Relaxed))
}

/// INT21 step Q (W491, W487 (e)): if_neq_const nodes whose second argument is not constant natively (EQC1's c7 form, unprobed). fx_run refuses
/// them on the claim route as IFNEQ-CONST-ARG1-NONCONST. Read and reset by ifneq_arg1_nonconst_take.
static IFNEQ_ARG1_NONCONST: std::sync::Mutex<Vec<usize>> = std::sync::Mutex::new(Vec::new());

/// The node ids noted since the last take.
pub fn ifneq_arg1_nonconst_take() -> Vec<usize> {
    std::mem::take(&mut *IFNEQ_ARG1_NONCONST.lock().unwrap())
}

/// INT25 (W503 (b), INT21-A2): every if_neq_const decision of a compile, observe only: (node id, argument 0 constant flag, argument 1 constant
/// flag, argument 0 count, argument 1 count, arm kept or None). The flags and counts are the operand views prune_arm reads (the cells
/// [this], [this], [this], [this] of, EQC1). Nothing here changes a plan. Read and reset by ifneq_trace_take.
pub type IfneqTrace = (usize, bool, bool, usize, usize, Option<usize>);
static IFNEQ_TRACE: std::sync::Mutex<Vec<IfneqTrace>> = std::sync::Mutex::new(Vec::new());

/// The if_neq_const decisions noted since the last take.
pub fn ifneq_trace_take() -> Vec<IfneqTrace> {
    std::mem::take(&mut *IFNEQ_TRACE.lock().unwrap())
}

fn eq_const(a: &ArgView, b: &ArgView) -> bool {
    if !a.is_const || !b.is_const {
        return false;
    }
    eq_scan_note(a, b);
    (0..a.n).all(|i| {
        let (x, y) = (a.lanes[i.min(3)] as f64, b.lanes[i.min(3)] as f64);
        // INT17 (Batch 52 rule 12, W435 (b) 6): the listing order and form, `b + eps > a` then `a > b - eps`, in f64
        y + EQ_EPS > x && x > y - EQ_EPS
    })
}

/// DSW4 (f04): `cvttss2si` as the original's set_arg_const applies to a lane bound to an integer-typed argument
/// descriptor ([desc] set): truncation toward zero, NaN and out of range give 0x80000000. MEASURED: switch arg 0 and the odd key
/// arguments are integer typed (out/dd/DS_f04.json: cell holds int32 1 for combine_masks 1.4); 0.6 -> 0, 1.99 -> 1, 2.5 -> 2.
pub fn cvtt_i32(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

///  with both operands integer typed: cell = cvttss2si(lane), then cvtsi2ss to f32, widened to f64, window 1e-10 (DSW4).
fn eq_const_int(a: &ArgView, b: &ArgView) -> bool {
    if !a.is_const || !b.is_const {
        return false;
    }
    eq_scan_note(a, b);
    (0..a.n).all(|i| {
        let (x, y) = (cvtt_i32(a.lanes[i.min(3)]) as f32 as f64, cvtt_i32(b.lanes[i.min(3)]) as f32 as f64);
        // INT17 (Batch 52 rule 12, W435 (b) 6): the listing order and form, `b + eps > a` then `a > b - eps`, in f64
        y + EQ_EPS > x && x > y - EQ_EPS
    })
}

/// Class slot 38: the argument index to keep when the condition is constant (R5), else None.
/// MEASURED shapes (see module doc); the "else" helpers are INFERRED to be the
/// complements (<= and not-equal).
pub fn prune_arm(class: &str, a: &[ArgView2]) -> Option<usize> {
    let v: Vec<ArgView> = a.iter().map(|x| ArgView { is_const: x.is_const, lanes: x.lanes, n: x.n, elnum: x.elnum }).collect();
    match class {
        "if+" | "if+_coerce" => {
            if v.len() < 3 {
                return None;
            }
            if all_lanes(&v[0], None, 0.0, &|l, t| l > t) {
                Some(1)
            } else if all_lanes(&v[0], None, 0.0, &|l, t| l <= t) {
                Some(2)
            } else {
                None
            }
        }
        "if>" | "if>_coerce" => {
            if v.len() < 4 {
                return None;
            }
            if all_lanes(&v[0], Some(&v[1]), 0.0, &|l, o| l > o) {
                Some(2)
            } else if all_lanes(&v[0], Some(&v[1]), 0.0, &|l, o| l <= o) {
                Some(3)
            } else {
                None
            }
        }
        "if_eq" => {
            if v.len() < 4 {
                return None;
            }
            if eq_const(&v[0], &v[1]) {
                Some(2)
            } else if v[0].is_const && v[1].is_const {
                Some(3)
            } else {
                None
            }
        }
        // always decides: 3 when the condition is set and equals k, else 2 (a non-constant
        // condition silently takes arm 2: MEASURED shape)
        "if_neq_const" => {
            if v.len() < 4 {
                return None;
            }
            // EQC1 : the original calls only when the flag byte [this] is nonzero and the
            // dword [this] equals [this] (MEASURED EQC1 W487: argument 0 constant flag, operand counts of arguments 0 and 1).
            // INT21 step Q: the feature eqc1_mut_nocount is the W170 mutant without the count conjunct (never in a shipped build).
            if v[0].is_const && (cfg!(feature = "eqc1_mut_nocount") || v[0].n == v[1].n) && v[1].is_const && eq_const(&v[0], &v[1]) {
                Some(3)
            } else {
                Some(2)
            }
        }
        // pairs (key, value) from arg 1; default = last argument; compare via 
        "switch" => {
            let n = v.len();
            if n < 3 {
                return None;
            }
            let pairs = n / 2 - 1;
            for i in 0..pairs {
                if 2 + 2 * i < n && eq_const_int(&v[0], &v[1 + 2 * i]) {
                    return Some(2 + 2 * i);
                }
            }
            Some(n - 1)
        }
        // INFERRED that [this] is the first argument's element count
        "if_has_alpha" => {
            if v.len() < 3 {
                return None;
            }
            match v[0].elnum {
                Some(2) | Some(4) => Some(1),
                Some(_) => Some(2),
                None => None,
            }
        }
        _ => None,
    }
}

/// public mirror of the private view so `prune_arm` can be called from tests
#[derive(Debug, Clone)]
pub struct ArgView2 {
    pub is_const: bool,
    pub lanes: [f32; 4],
    pub n: usize,
    pub elnum: Option<i32>,
}

/// The node's lanes from its program-text default. INT32 step DF (W605 (a)): an unbound fixed-slider (host type 2) parameter outside the
/// normalize_scale table reads its registered 16.16 default, f32(trunc(d x 65536) / 65536) of each text default lane d
/// (STYLE_PARAM_INDEX.tsv names the type); bound values replace these lanes as given (the harness's snap_16_16). The mutant df_bound_mut
/// (at the override below) applies the rule to bound values too.
fn default_lanes(g: &Genotype, n: &GNode, param: Option<&String>, bound: bool) -> Result<Vec<f32>, CompileError> {
    let k = n.nlanes.clamp(1, 4);
    let mut v: Vec<f32> = (0..k).map(|i| n.lanes[i]).collect();
    if let Some(p) = param {
        if !bound && crate::units::registered_fixed_slider(&g.name, p) {
            for x in v.iter_mut() {
                *x = crate::units::registered_16_16(&g.name, p, *x).map_err(|msg| CompileError { msg })?;
            }
        }
    }
    Ok(v)
}


#[derive(Debug, Clone)]
pub struct NodeState {
    pub gid: usize,
    pub b160: bool,
    pub folded: bool,
    pub lanes: [u32; 4],
    pub elnum: i32,
    pub alias: Option<usize>,
    pub name: String,
    pub auto: Option<&'static str>,
    pub nargs: usize,
}

/// Compile a genotype under a parameter binding (NV-3: with the original's collapses and folding).
pub fn compile(g0: &Genotype, combos: &ComboTable, opt: &CompileOptions) -> Result<CompiledProgram, CompileError> {
    if opt.mocha_data_present {
        return Err(CompileError { msg: "MOCHA-DATA-PRESENT: host mocha data ([this] non-null) is outside the claimed context (W215 b)".into() });
    }
    if opt.presence_mask.is_some() {
        // INT9 INPUT-PRESENCE-MASK: one input, both readers derived (derive_presence), then the ordinary compile
        let (o2, label) = derive_presence(g0, opt)?;
        let mut cp = compile(g0, combos, &o2)?;
        cp.diagnostics.insert(0, label);
        return Ok(cp);
    }
    // NV-3e (EP-3c fix list item 1, MEASURED by value on 5 pairs): an unbound input whose declared default names another input is that input
    let mut g_owned = g0.clone();
    if opt.minimal_context {
        let mut amap: HashMap<usize, usize> = HashMap::new();
        for i in &g0.inputs {
            if opt.presence_derived && opt.bound_elnum.contains_key(&i.name) {
                continue; // INT9: a set presence bit keeps the input an image (the bind pass leaves it alone)
            }
            if let Some(t) = &i.default {
                if let Some(b) = g0.inputs.iter().find(|x| x.name == t.trim() && x.name != i.name) {
                    amap.insert(i.node, b.node);
                }
            }
        }
        if !amap.is_empty() {
            for nd in g_owned.nodes.iter_mut() {
                for c in nd.children.iter_mut() {
                    if let Some(&b) = amap.get(c) {
                        *c = b;
                    }
                }
            }
        }
    }
    let g = &g_owned;
    let mut eff = opt.clone();
    let mut pd1: HashSet<usize> = HashSet::new();
    if opt.minimal_context {
        for i in &g.inputs {
            if let Some(&c) = opt.bound_elnum.get(&i.name) {
                eff.input_elnum.insert(i.name.clone(), c);
                continue;
            }
            if let Some(t) = &i.default {
                if let Some(v) = crate::parse::orig_number(t) {
                    eff.const_inputs.entry(i.name.clone()).or_insert_with(|| vec![v as f32]);
                }
            }
        }
        if g.flags.iter().any(|f| f == "no_alpha_only") {
            eff.input_elnum.entry("Source".to_string()).or_insert(3);
        }
        for p in &g.params {
            if let (Some(ni), true) = (p.node, p.flags.iter().any(|f| f == "default_to_1_if_postdiv")) {
                pd1.insert(ni);
            }
        }
    }
    let ranges_off = opt.ranges_off || crate::ranges::ranges_off_env();
    // DSW3: parameters whose record input is UNBOUND take the copy-default path of (no clamp). The bound-input mask is explicit (`bound_input_mask`default 0; input 0 is not implied);
    // inputs named in `bound_elnum` are bound too. INFERRED: the input index is the position in the declared input list.
    let mut noclamp: std::collections::HashSet<String> = std::collections::HashSet::new();
    // step TPU2 (W708 (e)): a host point parameter fed from the run's point lines (fx_run's TPU2 block) is the point arm's stored result;
    // the original's plan carries it unclamped (TB_R1's recorded plan: '- xy [ 0.5625 -1.0625 ]' with the record's min 0), so the range
    // clamp pass does not apply to it.
    if let Some(h) = opt.host_curve_values.as_ref() {
        for p in &g.params {
            if h.points.contains_key(&p.name) && opt.overrides.get(&p.name).map(|o| !o.is_empty()).unwrap_or(false) {
                noclamp.insert(p.name.clone());
            }
        }
    }
    if !opt.unbound_exception_off && std::env::var_os("DSW3_UNBOUND_OFF").is_none() {
        for p in &g.params {
            if let Some(k) = crate::ranges::param_input_index(&g.name, &p.name) {
                // LYM3: PARAM_INPUTS.tsv is measured in the def's input order; a merged input group is translated by input name (an input the layout row
                // does not carry has no position: 99, unbound)
                // INT28 step PI (W563): a row confirmed by a record's input index cell is already the registered program's index
                let k = if g.def_input_order.is_empty() || crate::ranges::param_input_recorded(&g.name, &p.name) { k } else { g.def_input_order.get(k).and_then(|n| g.inputs.iter().position(|i| &i.name == n)).unwrap_or(99) };
                let bm = opt.bound_input_mask; // INT6 rule 1: no environment override (DSW5_BOUND_MASK removed)
                let bound = (k < 32 && (bm >> k) & 1 == 1) || g.inputs.get(k).map(|i| opt.bound_elnum.contains_key(&i.name)).unwrap_or(false);
                if !bound {
                    noclamp.insert(p.name.clone());
                }
            }
        }
    }
    let opt = &eff;
    let n = g.nodes.len();
    let mut diags: Vec<String> = Vec::new();
    let mut pnoflags: std::collections::HashSet<usize> = std::collections::HashSet::new();
    let mut pname: HashMap<usize, String> = HashMap::new();
    for p in &g.params {
        if let Some(ni) = p.node {
            pname.insert(ni, p.name.clone());
            if p.flags.is_empty() {
                pnoflags.insert(ni);
            }
        }
    }
    let mut nodes: Vec<CNode> = Vec::with_capacity(n + 8);
    let mut cells: BTreeMap<usize, Vec<u32>> = BTreeMap::new();
    for gn in &g.nodes {
        let mut cn = CNode {
            id: gn.index,
            kind: CKind::Const,
            name: String::new(),
            param: None,
            args: gn.children.clone(),
            lanes: [0; 4],
            nlanes: 1,
            elnum: None,
            alias: None,
            is_const: false,
            auto: None,
            folded: false,
            cls: String::new(),
            input_pos: gn.input_pos,
        };
        match gn.kind {
            NodeKind::Function => {
                cn.kind = CKind::Func;
                cn.name = gn.name.clone();
            }
            NodeKind::Input if opt.const_inputs.contains_key(&gn.name) => {
                cn.kind = CKind::Const;
                cn.is_const = true;
                let (l, k) = lanes_of(&opt.const_inputs[&gn.name]);
                cn.lanes = l;
                cn.nlanes = k;
                cn.elnum = Some(k as i32);
                cn.name = gn.name.clone();
            }
            NodeKind::Input => {
                cn.kind = CKind::Input;
                cn.name = gn.name.clone();
                cn.elnum = Some(*opt.input_elnum.get(&gn.name).unwrap_or(&4));
            }
            NodeKind::Special => {
                cn.kind = CKind::Special;
                cn.name = gn.name.clone();
                let d = if gn.name == "xy" { 2 } else if gn.name == "Output" && opt.minimal_context { 3 } else { 1 };
                cn.elnum = Some(*opt.input_elnum.get(&gn.name).unwrap_or(&d));
            }
            NodeKind::Scalar | NodeKind::Vector | NodeKind::Str => {
                cn.kind = CKind::Const;
                cn.is_const = true;
                let bound_p = pname.get(&gn.index).map(|p| opt.overrides.get(p).map(|o| !o.is_empty()).unwrap_or(false)).unwrap_or(false);
                // the registration is keyed by the layout name (Trails' fade is registered as prev_brightness)
                let lname = g.params.iter().find(|q| q.node == Some(gn.index)).map(|q| if q.layout_name.is_empty() { q.name.clone() } else { q.layout_name.clone() });
                let mut vals = default_lanes(g, gn, lname.as_ref(), bound_p)?;
                if pd1.contains(&gn.index) {
                    vals = vec![1.0];
                }
                let init_cell = vals.clone();
                let mut row_applied = false;
                if let Some(p) = pname.get(&gn.index) {
                    cn.param = Some(p.clone());
                    // NV-3f: a parameter with NO flag text has the create_param default range [0, 1e6] (NEG1, PARAM_RANGES.tsv): the original also clamps the program-text default lane-wise at 0 (Drop_Shadow shift [0.042 -0.042] is [0.042 0] in the dumps and in LIT1; MEASURED once, rule for flagless parameters INFERRED; flagged parameters (widget, angle, screen...) keep their negative defaults, 5 programs checked in the dump scan)
                    if !ranges_off && pnoflags.contains(&gn.index) && crate::ranges::RangeTable::builtin().lookup(&g.name, p).is_none() && !opt.overrides.get(p).map(|o| !o.is_empty()).unwrap_or(false) {
                        for x in vals.iter_mut() {
                            if *x < 0.0 {
                                *x = 0.0;
                            }
                        }
                    }
                    // DSW2 (merged in INT4 step 1): a parameter with a range row has its program-text default clamped by the same pass (clamps the record's current cell, which the binding fills from the default when no override is given; MEASURED: Drop_Shadow PREC rows show cur shift [0.042, +0] from default [0.042, -0.042] with record min 0). A row for a parameter therefore replaces the flagless-default rule above.
                    if !ranges_off && !noclamp.contains(p) && !opt.overrides.get(p).map(|o| !o.is_empty()).unwrap_or(false) {
                        if crate::ranges::clamp_param(&g.name, p, &mut vals).is_some() {
                            row_applied = true;
                        }
                    }
                    // INT4 (RNG1 PATCH 3 a): an error, never a guess, for a claimed program (one with range rows) whose parameter has no row. Popup parameters keep NV-3g rule P.
                    if !ranges_off && crate::ranges::RangeTable::builtin().has_program(&g.name) && crate::ranges::RangeTable::builtin().lookup(&g.name, p).is_none() && g.params.iter().find(|q| q.name == *p).map(|q| q.info.popup.is_empty()).unwrap_or(true) {
                        return Err(CompileError { msg: format!("RANGE-ROW-MISSING {}.{}", g.name, p) });
                    }
                    if let Some(o) = opt.overrides.get(p) {
                        if !o.is_empty() {
                            vals = o.clone();
                            let ln = g.params.iter().find(|q| q.name == *p).map(|q| if q.layout_name.is_empty() { q.name.clone() } else { q.layout_name.clone() }).unwrap_or_else(|| p.clone());
                            if cfg!(feature = "df_bound_mut") && crate::units::registered_fixed_slider(&g.name, &ln) {
                                for x in vals.iter_mut() {
                                    *x = crate::units::registered_16_16(&g.name, &ln, *x).map_err(|msg| CompileError { msg })?;
                                }
                            }
                            // NEG-2: the measured range clamp of the original (pass), lane-wise, before fold and compile
                            if !ranges_off && !noclamp.contains(p) {
                                if crate::ranges::clamp_param(&g.name, p, &mut vals).is_some() {
                                    row_applied = true;
                                }
                            }
                            // NV-3g rule P (RULE-FITTED, InfiniteZoom nf2: transparent_area = 2 is [1] in the original plan): a popup parameter with no range row clamps lane-wise to [0, items - 1]
                            // (the same shape as the four measured popup rows in PARAM_RANGES.tsv: record max = items - 1)
                            if !ranges_off && crate::ranges::RangeTable::builtin().lookup(&g.name, p).is_none() {
                                if let Some(pr) = g.params.iter().find(|q| q.name == *p) {
                                    let np = pr.info.popup.len();
                                    if np > 0 {
                                        for x in vals.iter_mut() {
                                            if *x > (np - 1) as f32 {
                                                *x = (np - 1) as f32;
                                            }
                                            if *x < 0.0 {
                                                *x = 0.0;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    // NZ1 (nz1_negzero_clamp PATCH 2; node store): the clamp pass writes the node cell only where old and new differ as floats; equal under == (+0 vs -0) keeps the OLD bits.
                    // old = the previous compile's cell (opt.history) or, for a fresh instance, the program-text default. COMPILE-HISTORY is the caller's statement.
                    // INT8 (Batch 41 rule 5, W325 (d)): every record the range pass visits, not only rows that applied (NODE-STORE-PARTIAL closed); `row_applied` is kept for reading only.
                    let _ = row_applied;
                    if !ranges_off {
                        let old: Vec<f32> = match opt.history.as_ref().and_then(|h| h.get(&gn.index)) {
                            Some(b) => b.iter().map(|x| f32::from_bits(*x)).collect(),
                            None => init_cell.clone(),
                        };
                        crate::ranges::node_store(&old, &mut vals);
                    }
                    cells.insert(gn.index, vals.iter().map(|x| x.to_bits()).collect());
                }
                let (l, k) = lanes_of(&vals);
                cn.lanes = l;
                cn.nlanes = k;
                cn.elnum = Some(k as i32);
            }
        }
        nodes.push(cn);
    }
    // WPG9/WPG7 READ RULE (compiler pass, listing neutral_1; INT7 step 1: the only compile path, no environment switch): for each argument slot i of a
    // function node for which the class predicate (slot 9, : false for the last argument when class flag 0x40 is set, else true when descriptor byte is 0,
    // which holds for every argument of every class seen except get_frame / time_warp arg 0) holds, a kind-4 child (Special or Input) is replaced by a fresh clone, one
    // clone per slot (store into [node+8i] and [node+8i]). The sharing step below (filter) then merges clones.
    {
        let n0 = nodes.len();
        for ci in 0..n0 {
            if nodes[ci].kind != CKind::Func {
                continue;
            }
            for ai in 0..nodes[ci].args.len() {
                let t = nodes[ci].args[ai];
                if t >= n0 {
                    continue;
                }
                let k = nodes[t].kind;
                let last_excl = (crate::layout::flags_of(&nodes[ci].name) & 0x40) != 0 && ai + 1 == nodes[ci].args.len();
                let b0a = (nodes[ci].name == "get_frame" || nodes[ci].name == "time_warp") && ai == 0;
                if last_excl || b0a || !(k == CKind::Special || k == CKind::Input) {
                    continue;
                }
                let mut cp = nodes[t].clone();
                cp.id = nodes.len();
                nodes.push(cp);
                let nid = nodes.len() - 1;
                nodes[ci].args[ai] = nid;
            }
        }
    }
    let n = nodes.len();
    if g.dest.len() != 1 {
        return Err(CompileError { msg: format!("{}: {} dest nodes (single dest required)", g.name, g.dest.len()) });
    }
    let dest = g.dest[0];
    if dest >= n {
        return Err(CompileError { msg: "dest out of range".into() });
    }
    // reachable postorder of the genotype DAG (children before parents)
    let mut order: Vec<usize> = Vec::new();
    {
        let mut seen = vec![false; n];
        let mut stack: Vec<(usize, usize)> = vec![(dest, 0)];
        seen[dest] = true;
        while let Some(&(id, k)) = stack.last() {
            if k < nodes[id].args.len() && nodes[id].kind == CKind::Func {
                let l = stack.len();
                stack[l - 1].1 += 1;
                let c = nodes[id].args[k];
                if c >= n {
                    return Err(CompileError { msg: format!("node {} has child {} out of range", id, c) });
                }
                if !seen[c] {
                    seen[c] = true;
                    stack.push((c, 0));
                }
            } else {
                order.push(id);
                stack.pop();
            }
        }
    }
    let mut pruned: Vec<PrunedRec> = Vec::new();
    let mut folds: Vec<String> = Vec::new();
    let ifneq_refusal: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    let process = |ids: &[usize], nodes: &mut Vec<CNode>, pruned: &mut Vec<PrunedRec>, diags: &mut Vec<String>, folds: &mut Vec<String>| {
        for &id in ids {
            if nodes[id].kind != CKind::Func {
                continue;
            }
            // R4 (step 1): forward arguments that collapsed
            for i in 0..nodes[id].args.len() {
                let c = nodes[id].args[i];
                if let Some(k) = nodes[c].alias {
                    if let Some(&t) = nodes[c].args.get(k) {
                        nodes[id].args[i] = t;
                        // NV-3g rule D (RULE-FITTED, Sharpen_Best_Quality nf1 and nf2): forwarding an argument through a collapsed node onto a FOLDED constant
                        // function node leaves a second constant node (a per-use copy) in the original (node 71 next to node 19); recorded as an unreferenced copy
                        // that only the constant-lane comparison sees (it is not in the evaluation order and changes no plan text).
                        if nodes[t].kind == CKind::Func && nodes[t].is_const && nodes[t].folded {
                            let mut cp = nodes[t].clone();
                            cp.id = nodes.len();
                            cp.args = Vec::new();
                            nodes.push(cp);
                        }
                    }
                }
            }
            let name = nodes[id].name.clone();
            let views: Vec<ArgView2> = nodes[id]
                .args
                .iter()
                .map(|&c| {
                    let v = view(nodes, c);
                    ArgView2 { is_const: v.is_const, lanes: v.lanes, n: v.n, elnum: v.elnum }
                })
                .collect();
            let if_family = COND_CLASSES.contains(&name.as_str());
            if name == "if_neq_const" && nodes[id].args.len() >= 2 {
                // EQC1 (W480 (c)): the compare uses the clamped counts; outside 1..4 the original's dword is unmeasured, so refuse.
                let raw = |c: usize| nodes[c].elnum.unwrap_or(nodes[c].nlanes as i32);
                let (n0, n1) = (raw(nodes[id].args[0]), raw(nodes[id].args[1]));
                if !(1..=4).contains(&n0) || !(1..=4).contains(&n1) {
                    ifneq_refusal.borrow_mut().get_or_insert(format!("IFNEQ-CONST-COUNT-OUT-OF-RANGE ({}: node {}: {}, {})", g.name, id, n0, n1));
                    continue;
                }
                // INT21 step Q (W491): a non-constant second argument is noted for fx_run's claim-route refusal; no result changes here
                if !views[1].is_const {
                    IFNEQ_ARG1_NONCONST.lock().unwrap().push(id);
                }
            }
            let cond_arm = if if_family { prune_arm(&name, &views) } else { None }.filter(|&a| a < nodes[id].args.len());
            // INT25 (W503 (b)): observe-only trace of the if_neq_const operand views and the arm
            if name == "if_neq_const" && views.len() >= 2 {
                IFNEQ_TRACE.lock().unwrap().push((id, views[0].is_const, views[1].is_const, views[0].n, views[1].n, cond_arm));
            }
            // R2/R3 (class slot 31): elnum
            let arg_el: Vec<Option<i32>> = nodes[id].args.iter().map(|&c| nodes[c].elnum).collect();
            let mut el: Option<i32> = None;
            if let (true, Some(a)) = (if_family && name != "if_has_alpha", cond_arm) {
                el = arg_el[a];
            } else if name.ends_with("_if_needed") && !combos.has(&name) {
                // output has the wider of the two operands (Z_Glow: 3 and 4 give 4), INFERRED
                el = arg_el.iter().copied().flatten().max();
            } else if name == "save_prev_dest" {
                el = arg_el.first().copied().flatten();
            } else if !combos.has(&name) && fixed_out(&name).is_some() {
                el = fixed_out(&name);
            } else if arg_el.iter().all(|e| e.is_some()) {
                let ops: Vec<i32> = arg_el.iter().map(|e| e.unwrap_or(-1)).collect();
                match combos.select(&name, &ops) {
                    Some(t) => el = Some(t),
                    None => {
                        if combos.has(&name) {
                            diags.push(format!("Node {} (prim {}) has an invalid combo of arg types: {:?}", id, name, ops));
                        } else {
                            diags.push(format!("UNKNOWN-CLASS {} no combo table in COMBOS.tsv (node {}), elnum unknown", name, id));
                        }
                    }
                }
            } else {
                diags.push(format!("Node {} (prim {}): untyped argument", id, name));
            }
            // R5 / R5b (slot 38): the class decides the collapse
            // DCP1 (W603 DX2): a v3_to_v4_if4 node takes the element count of its bound image argument for the collapse and the fold
            // step DX2 v2 (W652 (d)): A RULE THAT TAKES A RECORDED COUNT APPLIES ONLY ON THE ROUTE THAT RECORDS IT
            let v3_route = opt.v3_bound_count || cfg!(feature = "dx2v2_allroutes_mut");
            let v3cnt: Option<i32> = if name == "v3_to_v4_if4" && v3_route && !cfg!(feature = "dx2_owncount_mut") {
                match crate::fold::v3_count(&views) {
                    Some(c) => Some(c),
                    None if cfg!(feature = "dx2v2_norefuse_mut") => el,
                    None => {
                        ifneq_refusal.borrow_mut().get_or_insert(format!("V3TOV4IF4-COUNT-UNBOUND ({}: node {}: the element count of the bound image argument is not known at compile; the original stores it from the bound source; DCP1, W603)", g.name, id));
                        el
                    }
                }
            } else {
                el
            };
            let arm = if if_family { cond_arm } else { collapse_arm(&name, &views, v3cnt).filter(|&a| a < nodes[id].args.len()) };
            // NV3b M12 (INFERRED, Warp_Transform): a warp whose coordinates are the raw xy input (wrap zero) is the identity
            let arm = if arm.is_none() && name == "warp" && nodes[id].args.len() >= 4 && nodes[nodes[id].args[1]].kind == CKind::Special && nodes[nodes[id].args[1]].name == "xy" && crate::fold::is_zero(&views[2]) { Some(0) } else { arm };
            nodes[id].elnum = el;
            nodes[id].alias = arm;
            if std::env::var("DSW3_TRACE").is_ok() {
                eprintln!("DSW3TRACE\tid={}\t{}\targs={:?}\tviews={:?}\tcond_arm={:?}\tarm={:?}\tel={:?}", id, name, nodes[id].args, views.iter().map(|v| (v.is_const, v.lanes[0], v.n, v.elnum)).collect::<Vec<_>>(), cond_arm, arm, el);
            }
            // R6 (alias check)
            if let Some(a) = arm {
                let t = nodes[id].args[a];
                if nodes[t].elnum != nodes[id].elnum {
                    nodes[id].alias = None;
                    if !nodes[t].is_const { diags.push(format!("R6-CANCEL {}#{} arm {} el {:?} vs target el {:?}", name, id, a, nodes[id].elnum, nodes[t].elnum)); }
                    if nodes[t].is_const {
                        let k = (nodes[t].elnum.unwrap_or(1) - 1).max(0) as usize;
                        let tl = nodes[t].lanes;
                        let l0 = tl[k.min(3)];
                        nodes[id].lanes = [l0, tl[k.min(1)], tl[k.min(2)], tl[k.min(3)]];
                        nodes[id].is_const = true;
                        nodes[id].folded = true;
                        nodes[id].cls = name.clone();
                        nodes[id].args.clear();
                        diags.push(format!("R6 alias->constant at node {} ({})", id, name));
                    }
                } else {
                    pruned.push(PrunedRec { name: format!("{}#{}", name, id), class: name.clone(), arm: a, arms: nodes[id].args.len() });
                }
            }
            // fold constant-only nodes (slot 30 true), only when no alias and not already constant
            if nodes[id].alias.is_none() && !nodes[id].is_const && !opt.no_fold {
                if let (Some((_, _)), Some(oe)) = (fold_kind(&name), nodes[id].elnum) {
                    let oe = if name == "v3_to_v4_if4" { v3cnt.unwrap_or(oe) } else { oe };
                    let views: Vec<ArgView2> = nodes[id]
                        .args
                        .iter()
                        .map(|&c| {
                            let v = view(nodes, c);
                            ArgView2 { is_const: v.is_const, lanes: v.lanes, n: v.n, elnum: v.elnum }
                        })
                        .collect();
                    let _ = FoldKind::Host;
                    // W215 (d) (1): a blur with a constant zero opacity and an EVEN combo tag is an error until the writer of [this] is read
                    // W262 (b) READ RULE (EVT1: blur [this] = argument 0's bound count, written by the value setter; OPERAND MEASURED,
                    // 8 calls in 4 compiles): fold.rs reads the count of argument 0 for the test "tag odd, or tag = [this]". The error stays only as a TRIPWIRE where the
                    // count the native binds to argument 0 differs from an even tag (by the combo table this does not occur when a row matches). SHE4_OFF=blur_ac is the named
                    // off option of the W236 (b) ablation (the even-tag fold is then not taken and the tripwire is not applied).
                    // Batch 38 (INT5-A5): the clause "constant zero opacity" is not a separate condition: the compare of the tag with [this] is reached only after the opacity-zero test at
                    //  (fold.rs, DSW3 READ), so the tripwire fires for every even tag that reaches the compare with [this] different. Cited, kept as the reach condition; UNREACHABLE-BY-CONSTRUCTION.
                    if !crate::fold::off("blur_ac") && name == "blur" && views.len() >= 4 && crate::fold::is_zero(&views[3]) && nodes[id].elnum.map(|t| t & 1 == 0).unwrap_or(false) && views[0].elnum != nodes[id].elnum {
                        diags.push(format!("BLUR-EVEN-TAG-OPEN: blur node {} has a constant zero opacity and even combo tag {:?} but argument 0 has count {:?} (W262 tripwire; [this] = argument 0's bound count, setter)", id, nodes[id].elnum, views[0].elnum));
                    }
                    if can_fold_el(&name, &views, nodes[id].elnum) {
                        let mut own: Vec<f32> = if opt.host_curves_unkeyed && name == "get_curve_val" && !views.is_empty() { views[0].lanes.to_vec() } else { Vec::new() };
                        let mut hcv_used = false;
                        if let (Some(hcv), true) = (opt.host_curve_values.as_ref(), name == "get_curve_val") {
                            hcv_used = true;
                            if opt.host_keyed.is_some() || opt.host_curves_unkeyed || opt.hc_zero_legacy {
                                diags.push("HOST-CURVE-VALUES-WITH-TEST-INTERFACE\tthe claim input excludes --hc, --hc-keyed and --hc-zero-legacy".to_string());
                            } else {
                                let pn = nodes[id].args.first().and_then(|&c| nodes[c].param.clone()).unwrap_or_default();
                                if views.len() < 2 || !views[1].is_const {
                                    diags.push(format!("HOST-CURVE-VALUE-MISSING ({}: time argument not a constant)", g.name));
                                } else if let Some(l) = crate::hostcurve::fold(&g.name, &pn, views[1].lanes[0], &views[0].lanes, views[0].n.min(4), hcv, diags) {
                                    own = l;
                                }
                            }
                        }
                        if let (Some(km), true) = (opt.host_keyed.as_ref(), name == "get_curve_val") {
                            // HCK1: keyed answer or an error; never a default
                            let pn = nodes[id].args.first().and_then(|&c| nodes[c].param.clone());
                            let tb = if views.len() > 1 && views[1].is_const { Some(views[1].lanes[0].to_bits()) } else { None };
                            match (pn, tb) {
                                (Some(pn), Some(tb)) => match km.get(&(pn.clone(), tb)) {
                                    Some(l) => {
                                        diags.push(format!("HCK-REQ\t{}\tt={}\tlanes={}", pn, f32::from_bits(tb), l.iter().map(|x| format!("{:08x}", x.to_bits())).collect::<Vec<_>>().join(",")));
                                        own = l.clone();
                                    }
                                    None => diags.push(format!("HC-KEYED-MISSING\t{}\tt={}\tbits={:08x}", pn, f32::from_bits(tb), tb)),
                                },
                                (pn, _) => diags.push(format!("HC-KEYED-MISSING\t{:?}\ttime argument not a constant or curve name unknown", pn)),
                            }
                        }
                        if name == "get_curve_val" && !hcv_used && !opt.host_curves_unkeyed && opt.host_keyed.is_none() && !opt.hc_zero_legacy {
                            // INT8 HC-NO-INTERFACE (Batch 40 rule 4): same request population as HCK1's HC-KEYED-MISSING (the fold site of get_curve_val)
                            diags.push(format!("HC-NO-INTERFACE\tnode {}\tget_curve_val with neither the keyed nor the unkeyed host-curve interface set; HC-ZERO-LEGACY is the named state A option", id));
                        }
                        let hv = if own.is_empty() { opt.host.get(&name).map(|v| v.as_slice()) } else { Some(own.as_slice()) };
                        if let Some(l) = fold_lanes(&name, &views, oe, hv) {
                            if name == "v3_to_v4_if4" {
                                // INT22 step B (W509 (b), W521 (a)): census and the claim-route refusal of unmeasured forms (fx_run reads v3_take)
                                crate::fold::v3_note(id, &views, oe, &l);
                            }
                            if name.starts_with("coerce") && std::env::var("FX1_TRACE").is_ok() {
                                eprintln!("FX1TRACE\t{}\targ_n={}\targ_lanes={:?}\tout_el={}\tfolded={:?}", name, views[0].n, views[0].lanes, oe, l.map(f32::from_bits));
                            }
                            nodes[id].lanes = l;
                            nodes[id].is_const = true;
                            nodes[id].folded = true;
                            nodes[id].cls = name.clone();
                            nodes[id].args.clear();
                            folds.push(format!("{}#{}", name, id));
                        }
                    }
                }
            }
        }
    };
    // XWP2 (W746): before the fold (model of, READ: kind 4 input = has-data byte, kind 2 = class flag or 0x80 or any child,
    // kind 1 = parameter record bits & 0xc0, ASSUMED 0 as in preserve.rs); the xy clones (Special) are not frame dependent (INFERRED from the
    // recorded plans: xy - centre is a preserve head in 33 Transform_and_Blur and 13 Blur_Only records)
    let fx_eval_pres_a_pre: HashMap<usize, bool> = {
        let mut m: HashMap<usize, bool> = HashMap::new();
        for &i in &order {
            let nd = &nodes[i];
            let mut v = match nd.kind {
                CKind::Input => (opt.ctx_has_data >> (nd.input_pos.saturating_sub(1) + 3)) & 1 != 0,
                CKind::Func => {
                    let f = crate::layout::flags_of(&nd.name);
                    f & 0x180 != 0
                }
                _ => false,
            };
            if nd.kind == CKind::Func && nd.args.iter().any(|a| *m.get(a).unwrap_or(&false)) {
                v = true;
            }
            m.insert(nd.id, v);
        }
        m
    };
    let has_prev_pre = order.iter().any(|&i| nodes[i].kind == CKind::Func && nodes[i].name == "get_prev_dest");
    process(&order, &mut nodes, &mut pruned, &mut diags, &mut folds);
    if let Some(m) = ifneq_refusal.borrow_mut().take() {
        return Err(CompileError { msg: m });
    }
    // R8 auto nodes (MEASURED order and collapse by the EP-3 plans): save_prev_dest (iff get_prev_dest is
    // in the surviving graph), gamma(amount 1), if+(__do_postdiv__, postdivide(.., copy_alpha=-1), image)
    let flags: HashSet<&str> = g.flags.iter().map(|s| s.as_str()).collect();
    let mut root = dest;
    {
        let mut c = dest;
        while let Some(k) = nodes[c].alias {
            match nodes[c].args.get(k) {
                Some(&t) => c = t,
                None => break,
            }
        }
        root = c;
    }
    let mut has_prev = false;
    {
        let mut seen = HashSet::new();
        let mut st = vec![root];
        while let Some(i) = st.pop() {
            if !seen.insert(i) {
                continue;
            }
            if nodes[i].kind == CKind::Func {
                if nodes[i].name == "get_prev_dest" {
                    has_prev = true;
                }
                for &c in &nodes[i].args {
                    st.push(c);
                }
            }
        }
    }
    let mk = |nodes: &mut Vec<CNode>, name: &str, tag: Option<&'static str>, args: Vec<usize>, cval: Option<f32>| -> usize {
        let id = nodes.len();
        let (lanes, isc, el) = match cval {
            Some(v) => ([v.to_bits(); 4], true, Some(1)),
            None => ([0u32; 4], false, None),
        };
        nodes.push(CNode { id, kind: if isc { CKind::Const } else { CKind::Func }, name: name.to_string(), param: None, args, lanes, nlanes: 1, elnum: el, alias: None, is_const: isc, auto: tag, folded: false, cls: String::new(), input_pos: 0 });
        id
    };
    let mut cur = dest;
    let mut auto_ids: Vec<usize> = Vec::new();
    if has_prev || has_prev_pre {
        cur = mk(&mut nodes, "save_prev_dest", Some("save_prev_dest"), vec![cur], None);
        auto_ids.push(cur);
    }
    let amount = mk(&mut nodes, "", Some("gamma_amount"), vec![], Some(1.0));
    cur = mk(&mut nodes, "gamma", Some("gamma"), vec![cur, amount], None);
    auto_ids.push(cur);
    let do_pd_on = match opt.do_postdiv {
        Some(v) => v,
        None => flags.contains("dynamic"),
    };
    let do_pd = mk(&mut nodes, "", Some("do_postdiv"), vec![], Some(if do_pd_on { 1.0 } else { 0.0 }));
    // DCP1 (W588 (a), W603): the tail's copy_alpha constant is 0 in the 3-channel (class-3) compile and -1 otherwise (MEASURED by
    // DCP1's write watch on Distort, Hue_Sat_Bright and Blur_Color; the host rule that picks it is not read, INFERRED from the root count).
    let ca_val: f32 = if nodes[root].elnum == Some(3) { 0.0 } else { -1.0 };
    let copy_alpha = mk(&mut nodes, "", Some("copy_alpha"), vec![], Some(ca_val));
    let pd = mk(&mut nodes, "postdivide", Some("postdivide"), vec![cur, copy_alpha], None);
    auto_ids.push(pd);
    let top = mk(&mut nodes, "if+", Some("if_postdiv"), vec![do_pd, pd, cur], None);
    auto_ids.push(top);
    process(&auto_ids, &mut nodes, &mut pruned, &mut diags, &mut folds);
    if let Some(m) = ifneq_refusal.borrow_mut().take() {
        return Err(CompileError { msg: m });
    }
    if let Some(d) = diags.iter().find(|d| d.starts_with("BLUR-EVEN-TAG-OPEN")) {
        return Err(CompileError { msg: d.clone() });
    }
    if opt.strict {
        if let Some(d) = diags.iter().find(|d| !d.starts_with("R6")) {
            return Err(CompileError { msg: d.clone() });
        }
    }
    let mut final_root = top;
    while let Some(k) = nodes[final_root].alias {
        match nodes[final_root].args.get(k) {
            Some(&t) => final_root = t,
            None => break,
        }
    }
    let labels = vec!["EVALUATOR-PLAN-CHECKED", "FOLDING-MEASURED-PARTIAL", "AUTO-NODES-MEASURED-TRAILS-FLAG-INFERRED"];
    // WPG9 (READ + MEASURED; INT7 step 1: unconditional): the clone sharing step is the filter function, called three times from the pass sequence 
    // (Input policy, vtable slots test / find / insert / no-op group change; Special policy,
    //  group change clears the table; constant policy: CONST-SHARE-UNEXERCISED, not ported, WPG12 CONST.md). Over the
    // evaluation array (post-order of the surviving graph) the FIRST leaf per input is kept; a later leaf of the same input is dropped and every
    // consumer slot that read it is rewritten to the kept one (store). Inputs: whole array. Specials: per linewise group id (
    // assigned by between call 1 and call 2), the table is cleared whenever the group id of the visited node changes.
    // SHARE-GROUPS-PORT: the group ids come from the port of run on an array without constants and with fmt 2 (3 for make_mipmap); the original array holds constants.
    {
        let post = |nodes: &Vec<CNode>, root: usize| -> Vec<usize> {
            let mut ex: Vec<usize> = Vec::new();
            let mut st: Vec<(usize, usize)> = vec![(root, 0)];
            let mut seen: HashSet<usize> = HashSet::new();
            seen.insert(root);
            while let Some(&(id, k)) = st.last() {
                let na = if nodes[id].kind == CKind::Func { nodes[id].args.len() } else { 0 };
                if k < na {
                    let l = st.len();
                    st[l - 1].1 += 1;
                    let c = nodes[id].args[k];
                    if seen.insert(c) {
                        st.push((c, 0));
                    }
                } else {
                    ex.push(id);
                    st.pop();
                }
            }
            ex
        };
        let apply = |nodes: &mut Vec<CNode>, arr: &[usize], red: &HashMap<usize, usize>| {
            for &i in arr {
                if nodes[i].kind == CKind::Func {
                    for k in 0..nodes[i].args.len() {
                        if let Some(&r) = red.get(&nodes[i].args[k]) {
                            nodes[i].args[k] = r;
                        }
                    }
                }
            }
        };
        // call 1: Input policy (global, group ignored)
        let arr = post(&nodes, final_root);
        let mut first: HashMap<String, usize> = HashMap::new();
        let mut red: HashMap<usize, usize> = HashMap::new();
        for &i in &arr {
            if nodes[i].kind == CKind::Input {
                match first.get(&nodes[i].name) {
                    Some(&f) => {
                        red.insert(i, f);
                    }
                    None => {
                        first.insert(nodes[i].name.clone(), i);
                    }
                }
            }
        }
        apply(&mut nodes, &arr, &red);
        // groups on the array after call 1
        let arr = post(&nodes, final_root);
        let inarr: HashSet<usize> = arr.iter().copied().filter(|&i| !nodes[i].is_const && nodes[i].kind != CKind::Const).collect();
        // WPG12 PATCH 2 (FORMAT-READ): the group pass takes the formats of the propagation pass 
        let gfmt: Vec<u32> = {
            let kinds: Vec<u8> = nodes.iter().map(|x| match x.kind { CKind::Func => 0, CKind::Input => 1, _ => 2 }).collect();
            let prims: Vec<String> = nodes.iter().map(|x| x.name.clone()).collect();
            let argv: Vec<Vec<usize>> = nodes.iter().map(|x| if x.kind == CKind::Func { x.args.clone() } else { vec![] }).collect();
            crate::cpuplan::fmt_pass_gen(final_root, &kinds, &prims, &argv)
        };
        let mut ln: Vec<crate::lgpass::LNode> = Vec::new();
        for &i in &arr {
            if !inarr.contains(&i) {
                continue;
            }
            let nd = &nodes[i];
            ln.push(crate::lgpass::LNode {
                id: i,
                is_func: nd.kind == CKind::Func,
                prim: if nd.kind == CKind::Func { nd.name.clone() } else { String::new() },
                elnum: nd.elnum.unwrap_or(1).max(1),
                fmt: gfmt[i] as u8,
                args: if nd.kind == CKind::Func { nd.args.iter().copied().filter(|a| inarr.contains(a)).collect() } else { vec![] },
                g: -1,
            });
        }
        let lres = match opt.xy_pres_keys.as_ref() {
            Some(k) => crate::lgpass::run_pass_keys(ln, k.clone()),
            None => crate::lgpass::run_pass(ln),
        };
        // call 2: Special policy, per group
        let mut table: HashMap<String, usize> = HashMap::new();
        let mut red2: HashMap<usize, usize> = HashMap::new();
        let mut last: i32 = -2;
        for r in &lres {
            if r.g != last {
                last = r.g;
                table.clear();
            }
            if nodes[r.id].kind == CKind::Special {
                match table.get(&nodes[r.id].name) {
                    Some(&f) => {
                        red2.insert(r.id, f);
                    }
                    None => {
                        table.insert(nodes[r.id].name.clone(), r.id);
                    }
                }
            }
        }
        if !opt.xy_unshared {
            apply(&mut nodes, &arr, &red2);
        }
    }
    // execution order: postorder over the surviving DAG
    let mut exec: Vec<usize> = Vec::new();
    let mut pos: HashMap<usize, usize> = HashMap::new();
    {
        let mut stack: Vec<(usize, usize)> = vec![(final_root, 0)];
        let mut seen: HashSet<usize> = HashSet::new();
        seen.insert(final_root);
        while let Some(&(id, k)) = stack.last() {
            let na = if nodes[id].kind == CKind::Func { nodes[id].args.len() } else { 0 };
            if k < na {
                let l = stack.len();
                stack[l - 1].1 += 1;
                let c = nodes[id].args[k];
                if seen.insert(c) {
                    stack.push((c, 0));
                }
            } else {
                pos.insert(id, exec.len());
                exec.push(id);
                stack.pop();
            }
        }
    }
    let mut out: Vec<PlanNode> = Vec::new();
    for (i, &id) in exec.iter().enumerate() {
        let c = &nodes[id];
        let kind = match (c.kind, c.auto) {
            (CKind::Func, _) if c.is_const => "const",
            (CKind::Func, Some(_)) => "auto",
            (CKind::Const, _) => "const",
            (CKind::Func, _) => "func",
            (CKind::Input, _) => "input",
            (CKind::Special, _) => "special",
        };
        let args: Vec<usize> = if c.kind == CKind::Func { c.args.iter().map(|a| pos[a]).collect() } else { vec![] };
        let all_const = c.kind == CKind::Func && !c.args.is_empty() && c.args.iter().all(|&a| nodes[a].is_const);
        out.push(PlanNode {
            gid: id,
            idx: i,
            name: if c.kind == CKind::Const { c.param.clone().unwrap_or_default() } else { c.name.clone() },
            kind,
            prim: if c.kind == CKind::Func && !c.is_const { c.name.clone() } else { "-".into() },
            elnum: c.elnum.unwrap_or(-1),
            fmt: "float",
            args,
            lanes: c.lanes,
            param: c.param.clone(),
            identity: false,
            all_const_args: all_const,
            b160: c.is_const,
            folded: c.folded,
            cls: c.cls.clone(),
            input_pos: c.input_pos,
        });
    }
    let mut params: Vec<(String, Vec<u32>)> = Vec::new();
    for p in &g.params {
        if let Some(ni) = p.node {
            let c = &nodes[ni];
            params.push((p.name.clone(), c.lanes[..c.nlanes.min(4)].to_vec()));
        }
    }
    let node_state: Vec<NodeState> = nodes
        .iter()
        .map(|c| NodeState { gid: c.id, b160: c.is_const, folded: c.folded, lanes: c.lanes, elnum: c.elnum.unwrap_or(-1), alias: c.alias, name: c.name.clone(), auto: c.auto, nargs: c.args.len() })
        .collect();
    let clones = clone_pass(&nodes, &exec, final_root);
    if let Some(d) = diags.iter().find(|d| d.starts_with("HC-KEYED-MISSING") || d.starts_with("HC-NO-INTERFACE") || d.starts_with("TICKMAP-") || d.starts_with("ROUTE-SELECTOR-NOT-CLAIMED") || d.starts_with("HOST-CURVE-VALUE")) {
        return Err(CompileError { msg: d.clone() });
    }
    Ok(CompiledProgram { program: g.name.clone(), paramset: opt.paramset.clone(), nodes: out, root: pos[&final_root], pruned, diagnostics: diags, params, labels, folds, node_state, clones, cells, ctx_word2: opt.ctx_word2, ctx_has_data: opt.ctx_has_data, fx_eval_pres_a_pre, xy_place: opt.xy_place })
}

/// DSW5: the clone pass (SHE4 FOLDS.md R1; DSW4 FOLDS2.md R-F03 "clone node trigger"), ONE rule.
/// READ (SHE4): for every function node (kind 2), every argument index passing the slot 9 test whose argument node has set (a literal
/// constant, a folded node, or an alias that became constant) and whose class slot 25 ([desc field]) is 0 is replaced by a CLONE (bytes copied,
/// appended to the program node vector). The original stays in the vector. Mode (env DSW5_CLONE): `she4` (default, one clone per such argument occurrence),
/// `dsw4` (the earlier INFERRED trigger: one clone per folded constant that has both an image-only and a scalar-capable use), `off`.
/// Scope (env DSW5_CLONE_SCOPE): `live` (default, nodes reachable from the root) or `all` (every node of the vector).
fn clone_pass(nodes: &[CNode], exec: &[usize], final_root: usize) -> Vec<NodeState> {
    let mode = std::env::var("DSW5_CLONE").unwrap_or_else(|_| "she4".into());
    if mode == "off" {
        return Vec::new();
    }
    let scope_all = std::env::var("DSW5_CLONE_SCOPE").map(|v| v == "all").unwrap_or(false);
    let ids: Vec<usize> = if scope_all { (0..nodes.len()).collect() } else { exec.to_vec() };
    let resolve = |a: usize| -> usize {
        let mut t = a;
        while let Some(kk) = nodes[t].alias {
            match nodes[t].args.get(kk) {
                Some(&x) => t = x,
                None => break,
            }
        }
        t
    };
    let mk = |t: usize, gid: usize| {
        let tg = &nodes[t];
        NodeState { gid, b160: true, folded: tg.folded, lanes: tg.lanes, elnum: tg.elnum.unwrap_or(-1), alias: None, name: tg.name.clone(), auto: tg.auto, nargs: 0 }
    };
    let mut v: Vec<NodeState> = Vec::new();
    let mut next_gid = nodes.len();
    if mode == "dsw4" {
        let mut img: std::collections::BTreeMap<usize, (usize, usize)> = std::collections::BTreeMap::new();
        for &id in &ids {
            let c = &nodes[id];
            if c.kind != CKind::Func || c.is_const {
                continue;
            }
            for (k, &a) in c.args.iter().enumerate() {
                let t = resolve(a);
                if nodes[t].is_const {
                    let e = img.entry(t).or_insert((0, 0));
                    if crate::cpuplan::image_only_arg(&c.name, k) { e.0 += 1 } else { e.1 += 1 }
                }
            }
        }
        for (&t, &(ni, no)) in img.iter() {
            if nodes[t].folded && t != final_root && ni > 0 && no > 0 {
                v.push(mk(t, next_gid));
                next_gid += 1;
            }
        }
        return v;
    }
    for &id in &ids {
        let c = &nodes[id];
        if c.kind != CKind::Func || c.is_const {
            continue;
        }
        for (k, &a) in c.args.iter().enumerate() {
            let t = resolve(a);
            if nodes[t].is_const && crate::cpuplan::image_only_arg(&c.name, k) {
                v.push(mk(t, next_gid));
                next_gid += 1;
            }
        }
    }
    v
}

impl CompiledProgram {
    /// Stable TSV, shaped like PLAN_FORMAT.md section 2 (PLAN / NODE / PRUNED rows). Floating constants
    /// are bit patterns; the command rows come from `plan::plan_text`.
    pub fn plan_tsv(&self) -> String {
        let mut s = String::new();
        s.push_str(&format!("PLAN\t{}\t{}\t{}\n", self.program, if self.paramset.is_empty() { "default" } else { &self.paramset }, self.nodes.len()));
        for n in &self.nodes {
            let refs: Vec<String> = n.args.iter().map(|a| a.to_string()).collect();
            let lanes = if n.kind == "const" { n.lanes.iter().map(|b| format!("{:08x}", b)).collect::<Vec<_>>().join(";") } else { "-".to_string() };
            s.push_str(&format!("NODE\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n", n.idx, n.name, n.kind, n.prim, n.elnum, n.fmt, refs.join(";"), lanes));
        }
        for p in &self.pruned {
            let kind = match p.class.as_str() {
                "if+" | "if+_coerce" => "ifplus",
                "if>" | "if>_coerce" => "ifgt",
                "if_neq_const" => "ifneqc",
                "if_eq" => "ifeq",
                "switch" => "switch",
                "if_has_alpha" => "ifhasalpha",
                _ => "collapse",
            };
            s.push_str(&format!("PRUNED\t{}\t{}\t{}\n", p.name, kind, p.arm));
        }
        s
    }

    pub fn count(&self, kind: &str) -> usize {
        self.nodes.iter().filter(|n| n.kind == kind).count()
    }

    /// signature of the surviving structure (names, elnums, args): equal signatures = same branch set
    pub fn signature(&self) -> String {
        let mut s = String::new();
        for n in &self.nodes {
            s.push_str(&format!("{}:{}:{}:{:?};", n.name, n.kind, n.elnum, n.args));
        }
        s
    }
}
// ---------------------------------------------------------------- branch sets

/// Parameters that are direct constant arguments of a conditional node (candidates for branching),
/// with literal constants seen in the same conditional nodes.
pub fn branch_params(g: &Genotype) -> (Vec<String>, Vec<f32>) {
    let mut pn: HashMap<usize, String> = HashMap::new();
    for p in &g.params {
        if let Some(ni) = p.node {
            pn.insert(ni, p.name.clone());
        }
    }
    let mut params: Vec<String> = Vec::new();
    let mut lits: Vec<f32> = Vec::new();
    for n in &g.nodes {
        if n.kind == NodeKind::Function && COND_CLASSES.contains(&n.name.as_str()) {
            // condition arguments: arg 0 (and arg 1 for if>, if_eq, if_neq_const, switch keys)
            let cond_args: Vec<usize> = match n.name.as_str() {
                "if+" | "if+_coerce" | "if_has_alpha" => vec![0],
                "switch" => (0..n.children.len().saturating_sub(1)).filter(|i| *i == 0 || i % 2 == 1).collect(),
                _ => vec![0, 1],
            };
            for ai in cond_args {
                if let Some(&c) = n.children.get(ai) {
                    if let Some(p) = pn.get(&c) {
                        if !params.contains(p) {
                            params.push(p.clone());
                        }
                    } else if matches!(g.nodes[c].kind, NodeKind::Scalar | NodeKind::Vector) {
                        lits.push(g.nodes[c].lanes[0]);
                    }
                }
            }
        }
    }
    params.sort();
    (params, lits)
}

pub struct BranchSet {
    pub paramset: String,
    pub overrides: BTreeMap<String, Vec<f32>>,
    pub signature: String,
    pub nodes: usize,
    pub funcs: usize,
}

/// Enumerate distinct surviving structures over candidate values of the branch parameters
/// (candidates: default, 0, 1, each literal seen in conditionals and its +-1; one-at-a-time,
/// then the full product when it has at most 4096 combinations).
pub fn enumerate_branches(g: &Genotype, combos: &ComboTable, input_elnum: &BTreeMap<String, i32>) -> Vec<BranchSet> {
    enumerate_branches_opt(g, combos, input_elnum, false)
}

/// INT8: the branch enumerator with the host-curve option explicit (`hc_zero_legacy` = HC-ZERO-LEGACY, the state A zero fold; false = HC-NO-INTERFACE,
/// so a program that requests a host curve yields no compiled set).
pub fn enumerate_branches_opt(g: &Genotype, combos: &ComboTable, input_elnum: &BTreeMap<String, i32>, hc_zero_legacy: bool) -> Vec<BranchSet> {
    let (params, lits) = branch_params(g);
    let mut default_of: HashMap<String, f32> = HashMap::new();
    for p in &g.params {
        if let Some(ni) = p.node {
            default_of.insert(p.name.clone(), g.nodes[ni].lanes[0]);
        }
    }
    let mut cand: Vec<f32> = vec![0.0, 1.0];
    for l in &lits {
        cand.push(*l);
        cand.push(*l - 1.0);
        cand.push(*l + 1.0);
    }
    cand.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    cand.dedup();
    let mut out: Vec<BranchSet> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut run = |ov: BTreeMap<String, Vec<f32>>, out: &mut Vec<BranchSet>| {
        let label = if ov.is_empty() { "default".to_string() } else { ov.iter().map(|(k, v)| format!("{}={}", k, v[0])).collect::<Vec<_>>().join(",") };
        let opt = CompileOptions { overrides: ov.clone(), input_elnum: input_elnum.clone(), strict: false, paramset: label.clone(), hc_zero_legacy, ..Default::default() };
        if let Ok(cp) = compile(g, combos, &opt) {
            let sig = cp.signature();
            if seen.insert(sig.clone()) {
                out.push(BranchSet { paramset: label, overrides: ov, signature: sig, nodes: cp.nodes.len(), funcs: cp.count("func") });
            }
        }
    };
    run(BTreeMap::new(), &mut out);
    let mut per: Vec<Vec<f32>> = Vec::new();
    for p in &params {
        let mut c = cand.clone();
        if let Some(d) = default_of.get(p) {
            c.push(*d);
        }
        c.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        c.dedup();
        per.push(c);
    }
    for (pi, p) in params.iter().enumerate() {
        for &v in &per[pi] {
            let mut ov = BTreeMap::new();
            ov.insert(p.clone(), vec![v]);
            run(ov, &mut out);
        }
    }
    let total: usize = per.iter().map(|c| c.len()).product::<usize>().max(1);
    if params.len() >= 2 && total <= 4096 {
        let mut idx = vec![0usize; params.len()];
        loop {
            let mut ov = BTreeMap::new();
            for (pi, p) in params.iter().enumerate() {
                ov.insert(p.clone(), vec![per[pi][idx[pi]]]);
            }
            run(ov, &mut out);
            let mut k = 0;
            loop {
                if k == params.len() {
                    return out;
                }
                idx[k] += 1;
                if idx[k] < per[k].len() {
                    break;
                }
                idx[k] = 0;
                k += 1;
            }
        }
    }
    out
}

impl CompiledProgram {
    /// `ELN gid elnum prim` rows for comparison with the original's node dump (EP-3 out_ch)
    pub fn elnum_rows(&self) -> String {
        let mut s = String::new();
        for n in &self.nodes {
            s.push_str(&format!("ELN\t{}\t{}\t{}\n", n.gid, n.elnum, n.prim));
        }
        s
    }
}

/// Output element counts of table-less classes. MEASURED from the original's compiled plans
/// (audit_ep3 NODE out_ch): static2 = 2, get_frame_number = 1. INFERRED (host scalars): the rest.
pub fn fixed_out(class: &str) -> Option<i32> {
    match class {
        "static2" | "get_frame_half_dims" => Some(2),
        "get_frame_number" | "get_time" | "get_percent_time" | "get_frames_per_second" | "get_aspect_ratio" | "mocha_render" => Some(1),
        _ => None,
    }
}
