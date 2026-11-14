//! The per level loop of the flow-build body 0x47a380 as ordinary Rust: a loop over the pyramid levels whose head is `level_checks`, whose
//! `Proceed` arm runs the resample and tile launches and the level stage, and whose other arms are the small level path and the
//! refusal of the flag route (OPTICAL-FLOW-FLAG-ROUTE-REFUSED). Values pass through typed arguments and results only; the plane
//! services are the `ComposedHost`'s.
use crate::host::{FlowServices, ResampleTargets};
use crate::host_types::PlaneId;
use crate::iface::*;
use crate::level::{gt_ordered, has_small_side, level_checks};
use crate::{gluefn, tailfn};
use crate::{slot_reduce, scan_range};
use fx_common::nanop::{fms231, NanOps};

/// Scalar cell 0x15efc14 (bits): the smallest range the affine map is applied to.
pub const C_EPS_BITS: u32 = 0x3727_c5ac;
/// Numerator of the affine scale.
pub const C_TWO: f32 = 2.0;
/// Weight of the common mean.
pub const C_HALF: f32 = 0.5;

/// Why a body run was refused instead of judged.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// OPTICAL-FLOW-FLAG-ROUTE-REFUSED: the host flag word is non zero (the image handle helper route)
    FlagRoute,
    /// OPTICAL-FLOW-SMALL-LEVEL-REFUSED: a level of the table has a side below 2 (the level build stores none)
    SmallLevel,
    /// OPTICAL-FLOW-LEVEL-TABLE-FULL-REFUSED: the pyramid would hold more levels than the level table's capacity
    LevelTableFull,
    /// OPTICAL-FLOW-NULL-IMAGE-REFUSED (optical_flow: the host answered no image for a pyramid buffer; the original reads through the missing buffer there, so this route is not rebuilt)
    NullImage,
}

/// Outcome of the level loop.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BodyResult {
    /// the body returned this value (0 after the last level)
    Returned(i32),
    Refused(Refusal),
}

/// Inputs of the level loop. `levels[0]` is the header level; levels 1..=count are looped over.
pub struct LevelRun<'a> {
    pub levels: &'a [LevelRecord],
    pub count: i32,
    pub env: StageEnv,
    /// params word 0x20 is non zero: each level passes through the two smoothing stages first
    pub smoothing: bool,
}

/// Scan, affine map, row sums, common mean and mean removal of one level (the level stage). Plane A is scanned first, then plane B.
fn level_stage(host: &mut dyn FlowServices, cur: &LevelRecord) {
    let eps = f32::from_bits(C_EPS_BITS);
    let (w, h, pitch) = (cur.width, cur.height, cur.stride);
    for plane in [cur.image_a, cur.image_b] {
        let (mx, mn) = scan_range(host.plane_words(plane).expect("plane registered"), w, h, pitch);
        let range = mx.fsub(mn);
        if !gt_ordered(eps, range) {
            let scale = C_TWO.fdiv(range);
            host.affine_plane(plane, w, h, pitch, scale, mn);
        }
    }
    let area = w.wrapping_mul(h) as f32;
    let mean_of = |host: &mut dyn FlowServices, plane| {
        let slots = host.sum_slots(plane, w, h, pitch).expect("plane registered");
        slot_reduce(&slots).fdiv(area)
    };
    let mean_a = mean_of(host, cur.image_a);
    let mean_b = mean_of(host, cur.image_b);
    let sum = mean_a.fadd(mean_b);
    let m = fms231(mean_a, sum, C_HALF);
    let count = pitch.wrapping_mul(h) as i64;
    host.add_const_plane(cur.image_a, count, m);
    host.add_const_plane(cur.image_b, count, f32::from_bits(m.to_bits() ^ 0x8000_0000));
}

/// Arena identifier of the intermediate plane of the resample pair (current width by previous height).
pub const RESAMPLE_TEMP: PlaneId = u64::MAX - 1;

/// Points the host's resample launches at the intermediate plane and the destination plane of this pair.
pub fn resample_targets(host: &mut dyn FlowServices, prev: &LevelRecord, cur: &LevelRecord, dst: PlaneId) {
    let n = (cur.width.max(0) as usize) * (prev.height.max(0) as usize);
    host.register_plane(RESAMPLE_TEMP, vec![0.0; n], 0);
    host.set_resample_targets(ResampleTargets { h_dst: RESAMPLE_TEMP, v_src: RESAMPLE_TEMP, v_dst: dst });
}

/// One level on the served path: the resample pair, the tile pair, optional smoothing, then the level stage.
fn proceed(host: &mut dyn FlowServices, run: &LevelRun, prev: &LevelRecord, cur: &LevelRecord) {
    resample_targets(host, prev, cur, cur.image_a);
    gluefn::launch_resample_pair(
        host,
        run.env.launcher,
        ResampleArgs { prev_width: prev.width, prev_height: prev.height, source: prev.image_a, cur_width: cur.width, cur_height: cur.height },
    );
    resample_targets(host, prev, cur, cur.image_b);
    tailfn::launch_tile(host, run.env.launcher, TileArgs { prev_width: prev.width, prev_height: prev.height, source: prev.image_b, cur_width: cur.width, cur_height: cur.height, destination: cur.image_b });
    if run.smoothing {
        let pair = LevelPair { prev, cur };
        if gluefn::smooth_first(host, &run.env, &pair) == FirstStage::Skipped {
            // the second stage still runs: it reads its own weights
        }
        let _second = gluefn::smooth_second(host, &run.env, &pair);
    }
    level_stage(host, cur);
}

/// The level loop of the body (levels 1..=count). Returns `Returned(0)` after the last level.
pub fn level_loop(host: &mut dyn FlowServices, run: &LevelRun) -> BodyResult {
    let table = &run.levels[..(run.count.max(0) as usize + 1).min(run.levels.len())];
    if has_small_side(table) {
        return BodyResult::Refused(Refusal::SmallLevel);
    }
    for idx in 1..=run.count.max(0) as usize {
        let (prev, cur) = (&run.levels[idx - 1], &run.levels[idx]);
        match level_checks(prev, cur, run.env.params.flag_route) {
            LevelCheck::Proceed => proceed(host, run, prev, cur),
            LevelCheck::FlagRoute => return BodyResult::Refused(Refusal::FlagRoute),
            LevelCheck::SmallSide => return BodyResult::Refused(Refusal::SmallLevel),
        }
    }
    BodyResult::Returned(0)
}
