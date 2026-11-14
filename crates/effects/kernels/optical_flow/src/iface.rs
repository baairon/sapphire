//! Typed interface of the flow-build body 0x47a380: level records, the plane scratch set, stage parameters, the row cursor and the
//! outcome types of the glue and tail stages. Values only: no register file, no frame offsets, no addresses of the original.
use crate::host_types::{PlaneId, Rect24};

pub use optical_flow_pyramid::LevelRecord;

/// The 24 byte rectangle record of a level, the form the plane services take by value.
pub fn level_rect(l: &LevelRecord) -> Rect24 {
    let lo = (l.width as u32 as u128) | ((l.height as u32 as u128) << 32) | ((l.stride as u32 as u128) << 64) | ((l.rect_word3 as u128) << 96);
    Rect24 { lo, hi: l.elem_bytes }
}

/// Previous and current level of one loop step.
#[derive(Clone, Copy, Debug)]
pub struct LevelPair<'a> {
    pub prev: &'a LevelRecord,
    pub cur: &'a LevelRecord,
}

/// The five scratch planes of a smoothing stage.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Scratch {
    /// plane the stage copies its result into
    pub result: PlaneId,
    pub flow_x: PlaneId,
    pub flow_y: PlaneId,
    pub work: PlaneId,
    pub source_copy: PlaneId,
}

/// Parameter words of the body (bit patterns of f32 values) and routes.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StageParams {
    /// params word 0x24: blend weight (bits)
    pub blend_weight: u32,
    /// params word 0x28: smoothing weight (bits)
    pub smooth_weight: u32,
    /// the host flag word is non zero (flag route)
    pub flag_route: bool,
    /// host cell value passed to the flag route blend step (bits)
    pub flag_cell_bits: u32,
}

/// Everything a smoothing stage or a tail stage needs besides the level.
#[derive(Clone, Copy, Debug)]
pub struct StageEnv {
    /// opaque handle of the parallel launcher
    pub launcher: u64,
    pub params: StageParams,
    pub scratch: Scratch,
    /// the opaque word of the state block table the image helpers take (carried unchanged)
    pub table_word: u64,
}

/// Outcome of the first smoothing stage.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FirstStage {
    /// smoothing weight zero: nothing was done
    Skipped,
    Ran,
}
/// Outcome of the second smoothing stage.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SecondStage {
    /// smoothing weight zero: nothing was done
    Skipped,
    /// result copied straight back (blend weight not above zero)
    CopiedBack,
    /// blended back with the image helper (flag route)
    FilledByHelper,
    /// blended back with the difference launch, or with nothing when the weight compares equal to zero
    FilledByLaunch,
}

/// Arguments of the pair of resample launches that open a level.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ResampleArgs {
    pub prev_width: i32,
    pub prev_height: i32,
    pub source: PlaneId,
    pub cur_width: i32,
    pub cur_height: i32,
}

/// Arguments of the tile launch pair of the tail.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct TileArgs {
    pub prev_width: i32,
    pub prev_height: i32,
    pub source: PlaneId,
    pub cur_width: i32,
    pub cur_height: i32,
    pub destination: PlaneId,
}

/// Outcome of the level checks at the head of a level.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LevelCheck {
    /// every dimension at least two: the level goes to the resample and tile launches and the per level stage
    Proceed,
    /// a side of the previous or current level is below two: refused (OPTICAL-FLOW-SMALL-LEVEL-REFUSED)
    SmallSide,
    /// the flag word is non zero: the flag route
    FlagRoute,
}
