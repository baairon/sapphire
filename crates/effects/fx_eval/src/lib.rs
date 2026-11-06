//! fx_eval: native reader of the effect program language (piece NV-1).
//!
//! Label: EVALUATOR-UNCHECKED (our evaluator, not yet compared with the original's plan).
//! Std only, safe Rust, no panics on malformed input (errors are returned).
//!
//! Pipeline: `parse::tokenize` -> `parse::parse_item` (AST in an `Arena`) -> `corpus::Corpus`
//! (all defuns and effects) -> `build::build_effect` (inlining builder) -> `build::Genotype`
//! (genotype-equivalent node list) -> `build::genotype_tsv`.

pub mod detmath;
pub mod build;
pub mod compile;
pub mod fold;
pub mod plan;
pub mod cpuplan;
pub mod layout;
pub mod ranges;
pub mod lgpass;
pub mod preserve;
pub mod xyplace;
pub mod judge_keys;
pub mod corpus;
/// the registered program text as the original's parser sees it (layout row with its:extra)
pub mod layoutmerge;
pub mod parse;
/// Native builders of the programs the thirteen styles reach (no program text is read)
pub mod programs_native;
pub mod svml_trig;
/// the per-argument column of record (argcol_rs with ARGCOL-VACUOUS and the two tie-break guards), report-only
pub mod argcol;
/// (rule 5): the checkout rule ported as read, report-only
pub mod checkout;
pub mod hostcurve;

pub use build::{build_effect, genotype_tsv, BuildError, GNode, Genotype, InputRec, NodeKind, ParamRec};
pub use compile::{NodeState, compile, enumerate_branches, enumerate_branches_opt, branch_params, BranchSet, CNode, CKind, CompileError, CompileOptions, CompiledProgram, Combo, ComboTable, PlanNode, PrunedRec, prune_arm, ArgView2};
pub use corpus::{Corpus, Failure};
pub use parse::{Arena, DefaultVal, DefunDef, EffectDef, FlagInfo, InputDef, Item, PKind, ParseError, Param};

/// the program-text default number of an input (`( Mask 1)`), as the harness reads it.
pub fn orig_number_pub(t: &str) -> Option<f64> {
    parse::orig_number(t)
}

/// (FIELDS.md): the compile context as explicit native inputs. No Default impl on purpose.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CompileContext {
    /// host time t in frames ([ctx] is derived from it); recorded, not consumed by the native compiler (Time_Warp_RGB's frame fold site is unread)
    pub host_time: f32,
    /// stack word 2 of the call to (`[owner] != 0 || (ctx == 0 && ctx == 0)`)
    pub stack_word2: bool,
    /// [owner]: 3 or 4 in the plugin (setter), 0 in the stored state A harness
    pub owner_mode_byte: u8,
    /// has-data flags [owner+i], bit i
    pub has_data: u32,
}

impl CompileContext {
    /// Parse the four required flags from a command line: `--ctx-time F --ctx-word2 0|1 OPT_CTX_OWNER_MODE N --ctx-has-data HEX`. Any missing or unparsable one is an error.
    pub fn from_args(a: &[String]) -> Result<CompileContext, String> {
        let get = |k: &str| -> Result<&str, String> {
            let p = a.iter().position(|x| x == k).ok_or_else(|| format!("{} is required (compile context, Batch 36 rule 3, no default)", k))?;
            a.get(p + 1).map(|s| s.as_str()).ok_or_else(|| format!("{} needs a value", k))
        };
        let t: f32 = get("--ctx-time")?.parse().map_err(|_| "--ctx-time must be a float".to_string())?;
        let w2 = match get("--ctx-word2")? {
            "0" => false,
            "1" => true,
            _ => return Err("--ctx-word2 must be 0 or 1".into()),
        };
        let o: u8 = get(crate::judge_keys::OPT_CTX_OWNER_MODE)?.parse().map_err(|_| format!("{} must be an integer", crate::judge_keys::OPT_CTX_OWNER_MODE))?;
        let hd = u32::from_str_radix(get("--ctx-has-data")?.trim_start_matches("0x"), 16).map_err(|_| "--ctx-has-data must be hex".to_string())?;
        Ok(CompileContext { host_time: t, stack_word2: w2, owner_mode_byte: o, has_data: hd })
    }
    /// the check of a caller that carries `stack_word2` and `has_data` into `CompileOptions::{ctx_word2, ctx_has_data}` (plancmp). Word 2 = 1 is then planned by
    /// the preserve-group model (`preserve.rs`); the only other difference from `check_native` is that the word 2 refusal is lifted.
    pub fn check_native_preserve(&self) -> Result<(), String> {
        let mut c = *self;
        c.stack_word2 = false;
        c.check_native()
    }
    /// What the native compiler can honour today.
    pub fn check_native(&self) -> Result<(), String> {
        if self.stack_word2 {
            return Err("STACK-WORD2-NOT-IMPLEMENTED: stack word 2 = 1 enables the node-grouping pass (in the planner) which is not ported; native plans equal the word 2 = 0 originals only".into());
        }
        if !matches!(self.owner_mode_byte, 0 | 3 | 4) {
            return Err(format!("[owner] = {} is outside {{0 (harness), 3, 4}}", self.owner_mode_byte));
        }
        if !self.host_time.is_finite() {
            return Err("host time must be finite".into());
        }
        Ok(())
    }
}

///: PARAM-UNITS, the host value to record value conversion (lane 0, ns 2 and 4)
pub mod units;
