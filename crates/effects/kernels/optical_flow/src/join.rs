//! The joined body: prologue (`build_to_cut`), the smoothing stages and range rescan on the params+0x20 route, the first stage group (`stage_to_cut`), then the level loop over the level table.
use crate::body::{level_loop, BodyResult, LevelRun, Refusal};
use crate::host::FlowServices;
use crate::iface::{LevelPair, StageEnv};
use crate::flowbuild::C_RANGE_BITS;
use crate::flowstage::scan_range;
use crate::level::has_small_side;
use fx_common::nanop::NanOps;
use crate::tailfn::alt_stages;
use crate::{build_to_cut, stage_to_cut, BodyIn, Cut, ImgCall, StageCut};
use optical_flow_pyramid::LevelPlane;

/// Where the joined body ended.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BodyEnd {
    /// fewer than two usable levels (returns 1)
    ReturnedOne,
    /// no level beyond the first (returns 0)
    ReturnedZero,
    /// the flag route (refused by name)
    Refused(Refusal),
    /// the level loop ended
    Levels(BodyResult),
}

fn f32s(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

/// Copies the words of the registered plane `id` back over `dst` (little-endian, up to the shorter length).
fn read_back(host: &dyn FlowServices, id: u64, dst: &mut [u8]) {
    if let Some(words) = host.plane_words(id) {
        for (c, x) in dst.chunks_exact_mut(4).zip(words) {
            c.copy_from_slice(&x.to_le_bytes());
        }
    }
}

/// The joined body. Order (listing addresses): the prologue `build_to_cut`; a non-zero flag word refuses the flag route
/// (0x47aab4, 0x47d1ff) before any host call; the small-level guard directly after the cut; on the params+0x20 route (0x47cbe1,
/// 0x47cf42, 0x47d1f6, 0x47d239, 0x47d2be, 0x47d322) the smoothing stages over the first level through the host, the two planes
/// read back, and the range scan of plane A again (0x47ab28) choosing Launch or Degenerate by the 0x15efc14 compare; then on every
/// route the first stage group `stage_to_cut`, the plane registration and the level loop.
#[allow(clippy::too_many_arguments)]
pub fn run_body(
    inp: &BodyIn,
    dst_a: &mut [u8],
    dst_b: &mut [u8],
    img: &mut dyn FnMut(&ImgCall) -> u64,
    align_a: usize,
    align_b: usize,
    host: &mut dyn FlowServices,
    env: StageEnv,
) -> BodyEnd {
    let (mut first, table) = build_to_cut(inp, dst_a, dst_b, img);
    let smoothing = inp.smoothing_word != 0;
    match first.cut {
        Cut::TooFewLevels => return BodyEnd::ReturnedOne,
        Cut::Helper => return BodyEnd::Refused(Refusal::FlagRoute),
        Cut::TableFull => return BodyEnd::Refused(Refusal::LevelTableFull),
        Cut::NullImage => return BodyEnd::Refused(Refusal::NullImage),
        Cut::Alt20 | Cut::Launch | Cut::Degenerate => {}
    }
    // 0x47aab4 and 0x47d1ff: on both routes a non-zero flag word enters the flag route (0x47aaba), before any host call
    if first.cut == Cut::Alt20 && inp.flag != 0 {
        return BodyEnd::Refused(Refusal::FlagRoute);
    }
    let count = table.first();
    let levels = &table.levels;
    if has_small_side(levels) {
        return BodyEnd::Refused(Refusal::SmallLevel);
    }
    if first.cut == Cut::Alt20 {
        // 0x47cbe1..0x47d322: the smoothing stages over the first level, then back to the range scan at 0x47ab28
        host.register_plane(levels[0].image_a, f32s(dst_a), align_a);
        host.register_plane(levels[0].image_b, f32s(dst_b), align_b);
        alt_stages(host, &env, &LevelPair { prev: &levels[0], cur: &levels[0] });
        read_back(host, levels[0].image_a, dst_a);
        read_back(host, levels[0].image_b, dst_b);
        let (w, h, pitch) = (levels[0].width, levels[0].height, levels[0].stride);
        let (mx, mn) = scan_range(&f32s(dst_a), w, h, pitch);
        first.max = mx;
        first.min = mn;
        first.cut = if f32::from_bits(C_RANGE_BITS) > mx.fsub(mn) { Cut::Degenerate } else { Cut::Launch };
    }
    let stage = stage_to_cut(&table, dst_a, dst_b, &first, align_a, align_b);
    host.register_plane(levels[0].image_a, f32s(dst_a), align_a);
    host.register_plane(levels[0].image_b, f32s(dst_b), align_b);
    if stage.cut == StageCut::Return {
        return BodyEnd::ReturnedZero;
    }
    for (j, lv) in levels.iter().enumerate().skip(1) {
        let n = (lv.stride.max(0) as usize) * (lv.height.max(0) as usize) + 8;
        host.ensure_level_plane(j, LevelPlane::ImageA.ordinal() as usize, lv.image_a, n);
        host.ensure_level_plane(j, LevelPlane::ImageB.ordinal() as usize, lv.image_b, n);
    }
    BodyEnd::Levels(level_loop(host, &LevelRun { levels, count, env, smoothing }))
}
