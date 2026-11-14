//! Tail stages of the body as outcome functions. See `iface.rs`.
use crate::gluefn::smooth_stage;
use crate::host_types::{Call, Callback, Ctx, Host, Stage};
use crate::iface::*;

/// The tile launch pair of the tail.
pub(crate) fn launch_tile(h: &mut dyn Host, launcher: u64, t: TileArgs) {
    let ctx = Ctx::Tile { prev_width: t.prev_width as u32, prev_height: t.prev_height as u32, source: t.source, cur_width: t.cur_width as u32, cur_height: t.cur_height as u32, destination: t.destination };
    h.call(Call::ParallelFor { stage: Stage::TailLaunch, handle: launcher, cb: Callback::ResampleHoriz, ctx });
    h.call(Call::ParallelFor { stage: Stage::TailLaunch, handle: launcher, cb: Callback::ResampleVert, ctx });
}
/// The alternate path: first stage, then second stage.
pub(crate) fn alt_stages(h: &mut dyn Host, e: &StageEnv, lv: &LevelPair) {
    let rect = level_rect(lv.cur);
    smooth_stage(h, e, Stage::TailA, rect, lv.cur.image_a);
    smooth_stage(h, e, Stage::TailB, rect, lv.cur.image_b);
}
