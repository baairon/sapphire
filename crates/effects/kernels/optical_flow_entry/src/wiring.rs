//! Shipped wiring of the entry: the path from the solve body's executor outcome to a result and the refusal rows of the run command.
//!
//! The entry passes one level table and one plane store to the solve body (`run_solve_body` takes the one `levels` slice and the one store).
//! An aborted run yields no frame and the planes are never read on that path.

use optical_flow_solve::solve::{run_body, Outcome, PlaneStore, RunRefused};
use optical_flow_solve::{Level, Params, SMALL_LEVEL_REFUSAL};

/// The refusals the run command maps to exit 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryRefusal {
    SmallLevel,
    LevelTableFull,
    NullImage,
    FlagRoute,
}

pub const LEVEL_TABLE_FULL_REFUSAL: &str = "OPTICAL-FLOW-LEVEL-TABLE-FULL-REFUSED (optical_flow: the pyramid would hold more levels than the level table's capacity; the original writes past its table there, so this route is not rebuilt)";
pub const NULL_IMAGE_REFUSAL: &str = "OPTICAL-FLOW-NULL-IMAGE-REFUSED (optical_flow: the host answered no image for a pyramid buffer; the original reads through the missing buffer there, so this route is not rebuilt)";
pub const FLAG_ROUTE_REFUSAL: &str = "OPTICAL-FLOW-FLAG-ROUTE-REFUSED (optical flow: the host image handle route is not rendered)";

/// The pyramid of the run would hold fewer than two levels: the original's flow entry stops there and writes no output.
pub const TOO_FEW_LEVELS_REFUSAL: &str = "OPTICAL-FLOW-TOO-FEW-LEVELS-REFUSED (optical_flow: the frame is too small for a pyramid of two levels; the original's flow entry stops there and writes no output)";

/// The refusal text of a row.
pub fn refusal_text(r: EntryRefusal) -> &'static str {
    match r {
        EntryRefusal::SmallLevel => SMALL_LEVEL_REFUSAL,
        EntryRefusal::LevelTableFull => LEVEL_TABLE_FULL_REFUSAL,
        EntryRefusal::NullImage => NULL_IMAGE_REFUSAL,
        EntryRefusal::FlagRoute => FLAG_ROUTE_REFUSAL,
    }
}

/// What the run command reports: status code, the message on the error stream, and the output (none on a refusal).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunReport {
    pub status_code: i32,
    pub stderr: String,
    pub output: Option<Vec<u8>>,
}

/// A refusal row: exit 2 with the exact text, before any output.
pub fn fx_run_refusal(r: EntryRefusal) -> RunReport {
    RunReport { status_code: 2, stderr: refusal_text(r).to_string(), output: None }
}

/// The flag route is refused from both entries: the stage route's flag word and the smoothing route's flag word.
pub fn flag_route_refusal(stage_flag: u32, smoothing_flag: u32) -> Option<EntryRefusal> {
    if stage_flag != 0 || smoothing_flag != 0 {
        Some(EntryRefusal::FlagRoute)
    } else {
        None
    }
}

/// How the solve body ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WiringOutcome {
    /// The run finished: the planes may be read.
    Done,
    /// The abort query stopped the run: no frame, no plane read.
    NoFrame,
    Refused(EntryRefusal),
    /// The first level index is outside the table (an empty table).
    NoLevels,
}

/// Runs the solve body's executor over the level table and the plane store and maps the outcome. Only `Done` lets the caller read planes.
pub fn run_solve_body<S: PlaneStore>(levels: &[Level], first: i32, p: &Params, store: &mut S, abort: &mut dyn FnMut() -> bool) -> WiringOutcome {
    match run_body(levels, first, p, store, abort) {
        Ok(Outcome::Done) => WiringOutcome::Done,
        Ok(Outcome::Aborted) => WiringOutcome::NoFrame,
        Err(RunRefused::SmallLevel) => WiringOutcome::Refused(EntryRefusal::SmallLevel),
        Err(RunRefused::FirstIndex) => WiringOutcome::NoLevels,
    }
}
