//! Glue stages between the stage groups as outcome functions. Every function takes typed values and returns an outcome or a loop step;
//! host calls go through the `Host` trait in program order. See `iface.rs`.
use crate::host_types::{Call, Callback, Ctx, Host, PlaneId, Rect24, Stage};
use crate::iface::*;

/// Number of launch pairs (or helper step pairs) of a smoothing stage.
pub(crate) const PAIR_COUNT: u32 = 30;
/// Weight bits (0.1225 as f32) passed with the second launch of each pair.
pub(crate) const PAIR_WEIGHT_BITS: u32 = 0x3e7a_e148;

/// Ordered greater-than: false for an unordered pair.
pub(crate) fn gt_ordered(a: f32, b: f32) -> bool {
    a > b
}
/// Ordered equal: false for an unordered pair.
pub(crate) fn eq_ordered(a: f32, b: f32) -> bool {
    a == b
}

/// How one smoothing stage ended.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum StageEnd {
    Skipped,
    CopiedBack,
    FilledByHelper,
    FilledByLaunch,
}

/// The 30 step loop of a smoothing stage.
fn steps(h: &mut dyn Host, e: &StageEnv, stage: Stage, rect: Rect24) {
    let (s, w) = (&e.scratch, e.params.smooth_weight);
    for _ in 0..PAIR_COUNT {
        if e.params.flag_route {
            h.call(Call::SmoothStep { a: s.work, b: s.flow_x, c: s.flow_y, d: s.source_copy, x: w, rect });
            h.call(Call::BlendStep { a: s.work, b: s.flow_x, c: s.flow_y, x: e.params.flag_cell_bits, y: w, rect });
        } else {
            let smooth_ctx = Ctx::Smooth { rect, work: s.work, flow_x: s.flow_x, flow_y: s.flow_y, source_copy: s.source_copy, weight: w };
            h.call(Call::ParallelFor { stage, handle: e.launcher, cb: Callback::Smooth, ctx: smooth_ctx });
            let blend_ctx = Ctx::Blend { rect, work: s.work, flow_x: s.flow_x, flow_y: s.flow_y, weight: w, pair_weight: PAIR_WEIGHT_BITS };
            h.call(Call::ParallelFor { stage, handle: e.launcher, cb: Callback::Blend, ctx: blend_ctx });
        }
    }
}

/// One smoothing stage over the plane `level_plane` of a level: copies, the step loop, the copy into the result plane and the blend back.
pub(crate) fn smooth_stage(h: &mut dyn Host, e: &StageEnv, stage: Stage, rect: Rect24, level_plane: PlaneId) -> StageEnd {
    let (p, s) = (&e.params, &e.scratch);
    let flag = p.flag_route as u32;
    if eq_ordered(f32::from_bits(p.smooth_weight), 0.0) {
        return StageEnd::Skipped;
    }
    // Both header stages (the alternate path) issue the first clear with an empty rectangle: the x flow scratch plane keeps the
    // words it holds (zero before the first stage, the first stage's final words before the second).
    let flow_x_rect = if matches!(stage, Stage::TailA | Stage::TailB) { Rect24 { lo: 0, hi: rect.hi } } else { rect };
    h.call(Call::CopyIn { dst: s.flow_x, flag, rect: flow_x_rect });
    h.call(Call::CopyIn { dst: s.flow_y, flag, rect });
    h.call(Call::CopyIn { dst: s.work, flag, rect });
    h.call(Call::CopyPlane { dst: s.source_copy, src: level_plane, flag, rect });
    steps(h, e, stage, rect);
    h.call(Call::CopyPlane { dst: s.result, src: s.work, flag, rect });
    let blend = f32::from_bits(p.blend_weight);
    if !gt_ordered(blend, 0.0) {
        h.call(Call::CopyPlane { dst: level_plane, src: s.result, flag, rect });
        return StageEnd::CopiedBack;
    }
    if p.flag_route {
        h.call(Call::Fill { a: level_plane, b: level_plane, c: s.result, x: p.blend_weight, rect });
        StageEnd::FilledByHelper
    } else {
        if !eq_ordered(blend, 0.0) {
            let ctx = Ctx::Fill { rect, operand_plane: level_plane, source_plane: s.result, destination: level_plane, weight: p.blend_weight };
            h.call(Call::ParallelFor { stage, handle: e.launcher, cb: Callback::FillDiff, ctx });
        }
        StageEnd::FilledByLaunch
    }
}


/// Opens a level: the two resample launches over one argument block.
pub fn launch_resample_pair(h: &mut dyn Host, launcher: u64, a: ResampleArgs) {
    let ctx = Ctx::Pair { prev_width: a.prev_width as u32, prev_height: a.prev_height as u32, source: a.source, cur_width: a.cur_width as u32, cur_height: a.cur_height as u32 };
    h.call(Call::ParallelFor { stage: Stage::Resample, handle: launcher, cb: Callback::ResampleHoriz, ctx });
    h.call(Call::ParallelFor { stage: Stage::Resample, handle: launcher, cb: Callback::ResampleVert, ctx });
}
/// First smoothing stage, over plane A of the current level.
pub fn smooth_first(h: &mut dyn Host, e: &StageEnv, lv: &LevelPair) -> FirstStage {
    match smooth_stage(h, e, Stage::First, level_rect(lv.cur), lv.cur.image_a) {
        StageEnd::Skipped => FirstStage::Skipped,
        _ => FirstStage::Ran,
    }
}
/// Second smoothing stage, over plane B of the current level.
pub fn smooth_second(h: &mut dyn Host, e: &StageEnv, lv: &LevelPair) -> SecondStage {
    match smooth_stage(h, e, Stage::Second, level_rect(lv.cur), lv.cur.image_b) {
        StageEnd::Skipped => SecondStage::Skipped,
        StageEnd::CopiedBack => SecondStage::CopiedBack,
        StageEnd::FilledByHelper => SecondStage::FilledByHelper,
        StageEnd::FilledByLaunch => SecondStage::FilledByLaunch,
    }
}
