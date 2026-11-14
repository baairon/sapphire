//! Glue of the inline reduction stretch of the solver body (listing 0x48446e to 0x486762, AVX2 route), per level step.
//!
//! The stretch carries six plane pairs (record words 12, 13, 16, 17, 18, 19) from the coarser level record `lp` to the finer level
//! record `l`. Each pair goes through the tile passes (horizontal then
//! vertical, crate `optical_flow_resample`); the body refuses a level with a side below 2 before the stretch runs. Between the pairs
//! the scalar passes `scale_plane` (words 12, 13) and `zero_border` (words 16 to 19) run. When the flag is nonzero the stretch
//! makes five helper calls instead and leaves the planes alone.
//!
//! Shipped form: `run_stretch` with the `Direct` host calls the passes in ordinary Rust. A test host may stand in for the `Host`
//! trait to observe the calls.
use crate::body::TileShape;
use crate::reduce::zero_border;
use crate::smooth::scale_plane;
use fx_common::nanop::NanOps;
use optical_flow_resample::{resample_h, resample_v};

pub use optical_flow_pyramid::LevelRecord as Level;

/// The six plane pairs the stretch carries, named by their level record field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PairPlane {
    FlowU,
    FlowV,
    Nest1T,
    Nest1U,
    Nest2T,
    Nest2U,
}

impl PairPlane {
    pub fn of(self, l: &Level) -> u64 {
        match self {
            PairPlane::FlowU => l.flow_u,
            PairPlane::FlowV => l.flow_v,
            PairPlane::Nest1T => l.nest1_t,
            PairPlane::Nest1U => l.nest1_u,
            PairPlane::Nest2T => l.nest2_t,
            PairPlane::Nest2U => l.nest2_u,
        }
    }
}

/// Labels of the stand-ins of this module.
pub fn assumptions() -> Vec<&'static str> {
    vec![
        "TILE-SEQUENCE: the tile pair runs the horizontal pass from the coarser plane into a scratch plane and the vertical pass from it into the finer plane, both with contiguous rows; the launches are hosted by the emulator, so the passes' effect on the planes is not compared here",
        "SCALAR-PASS-ARGS: the scale and border passes receive the plane, width, stride and row count of the level record",
        "HELPER-HOSTED: the helper calls (flag nonzero path) have no port; the shipped host ignores them, the arguments are compared to the recorded ones",
    ]
}

/// Plane access by record word: the effect's own plane slices, not addresses of the original.
pub trait Planes {
    /// source plane of word `j` (coarser level) with the byte phase modulo 16 of its first element
    fn src(&self, j: PairPlane) -> (&[f32], u32);
    /// source plane of word `j`, writable (the border pass zeroes it)
    fn src_mut(&mut self, j: PairPlane) -> &mut [f32];
    /// destination plane of word `j` (finer level)
    fn dst(&mut self, j: PairPlane) -> &mut [f32];
    /// Receives the horizontal pass's output of a tile pair. The original's first pass leaves it in the level record's median_out plane
    /// (compact rows of the finer width), where it stays until the next median launch rewrites that plane.
    fn first_pass(&mut self, _scratch: &[f32]) {}
}

/// One tile pair: both passes of word `j`.
pub struct Tile<'a> {
    pub j: PairPlane,
    pub l: &'a Level,
    pub lp: &'a Level,
    pub pair_state: u64,
}

/// The two helper services of the flag path.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HelperKind {
    /// the pair of flow planes
    Pair,
    /// one plane
    Plane,
}

/// One helper call of the flag path: five argument words and the five by-value words.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HelperCall {
    pub kind: HelperKind,
    pub args: [u64; 5],
    pub stack: [u64; 5],
}

/// Services of the glue.
pub trait Host {
    fn helper(&mut self, c: HelperCall);
    fn tile_pair<P: Planes>(&mut self, mem: &mut P, t: &Tile);
    fn scale<P: Planes>(&mut self, mem: &mut P, j: PairPlane, l: &Level, ratio: f32);
    fn border<P: Planes>(&mut self, mem: &mut P, j: PairPlane, lp: &Level);
}

/// The shipped host: ordinary calls into the resample crate and the scalar passes.
pub struct Direct;

impl Host for Direct {
    fn helper(&mut self, _c: HelperCall) {}

    fn tile_pair<P: Planes>(&mut self, mem: &mut P, t: &Tile) {
        let (sw, sh) = (t.lp.width, t.lp.height);
        let (dst_w, dst_h) = (t.l.width, t.l.height);
        let mut scratch = vec![0f32; (dst_w as usize) * (sh as usize)];
        {
            let (src, phase) = mem.src(t.j);
            resample_h(&src[..(sw as usize) * (sh as usize)], sw, sh, dst_w, &mut scratch, (phase & 15) as usize, 0, sh);
        }
        mem.first_pass(&scratch);
        resample_v(&scratch, dst_w, sh, dst_h, mem.dst(t.j), 0, dst_w);
    }

    fn scale<P: Planes>(&mut self, mem: &mut P, j: PairPlane, l: &Level, ratio: f32) {
        let (w, h) = (l.width, l.height);
        if w <= 0 || h <= 0 {
            return;
        }
        scale_plane(mem.dst(j), w, l.stride, ratio, 0, h as u32);
    }

    fn border<P: Planes>(&mut self, mem: &mut P, j: PairPlane, lp: &Level) {
        let (w, h) = (lp.width, lp.height);
        if w <= 0 || h <= 0 {
            return;
        }
        zero_border(mem.src_mut(j), w, h, lp.stride, 0, h as u32);
    }
}

const PAIRS: [PairPlane; 6] = [PairPlane::FlowU, PairPlane::FlowV, PairPlane::Nest1T, PairPlane::Nest1U, PairPlane::Nest2T, PairPlane::Nest2U];

fn helper_calls<H: Host>(l: &Level, lp: &Level, pair_state: u64, host: &mut H) {
    let (shape_p, shape_l) = (TileShape::of(lp), TileShape::of(l));
    let stack = [shape_p.size, shape_p.stride, shape_p.elem_bytes, shape_l.size, shape_l.stride];
    host.helper(HelperCall { kind: HelperKind::Pair, args: [lp.flow_u, lp.flow_v, l.flow_u, l.flow_v, pair_state], stack });
    for j in [PairPlane::Nest1T, PairPlane::Nest1U, PairPlane::Nest2T, PairPlane::Nest2U] {
        host.helper(HelperCall { kind: HelperKind::Plane, args: [j.of(lp), j.of(l), pair_state, 0, l.elem_bytes], stack });
    }
}

/// Runs the stretch. `s88` is the state word that fills the fifth word of every pair block; `flag` is the body's flag argument.
pub fn run_stretch<P: Planes, H: Host>(l: &Level, lp: &Level, pair_state: u64, flag: u32, mem: &mut P, host: &mut H) {
    if flag != 0 {
        helper_calls(l, lp, pair_state, host);
        return;
    }
    let step = |j: PairPlane, mem: &mut P, host: &mut H| host.tile_pair(mem, &Tile { j, l, lp, pair_state });
    for &j in PAIRS.iter().take(2) {
        step(j, mem, host);
        // scale factor: finer width over coarser width for word 12, heights for word 13
        let ratio = if j == PairPlane::FlowU { (l.width as f32).fdiv(lp.width as f32) } else { (l.height as f32).fdiv(lp.height as f32) };
        host.scale(mem, j, l, ratio);
    }
    for &j in PAIRS.iter().skip(2) {
        host.border(mem, j, lp);
        step(j, mem, host);
    }
}
