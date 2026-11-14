//! Level loop of the solver body. The body only sets up tile launches; it computes one value itself, the convergence ratio.
//! Tile launches are described by `LaunchArgs`; the executor is `solve::run_body`.

/// One pyramid level record: the rectangle, the level number and the plane handles.
pub use optical_flow_pyramid::LevelRecord as Level;

/// Loop bounds and thresholds of the parameter block.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// outer iteration count
    pub outer_iters: i32,
    /// middle loop bound
    pub mid_iters: i32,
    /// inner loop bound
    pub inner_iters: i32,
    /// nonzero selects the counted route after the loops
    pub counted_route: i32,
    /// flow update gain (f32 bits)
    pub flow_gain: u32,
    /// relax and smoothing scale (f32 bits)
    pub relax_scale: u32,
    /// convergence threshold
    pub tol: f32,
    /// smoothing bias (f32 bits)
    pub smooth_bias: u32,
    /// nonzero runs the alternate-launch block (derivative pair on image_a and the polar magnitude) at the top of every level
    pub alt_launches: i32,
}

/// The parallel-for callbacks the level loop and the stretch launch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Callback {
    DerivX,
    DerivY,
    Polar,
    Warp,
    Mix,
    Flow,
    Relax,
    Smooth,
    Diff,
    Median,
    TilePairFirst,
    TilePairSecond,
    Scale,
    Border,
}

/// Geometry shared by every tile launch: the packed width and height word, the stride word and the element size of the level record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileShape {
    /// width (low half) and height (high half) of the level
    pub size: u64,
    /// row stride word of the level
    pub stride: u64,
    /// bytes per element of the level
    pub elem_bytes: u64,
}

impl TileShape {
    pub fn of(l: &Level) -> TileShape {
        TileShape {
            size: (l.width as u32 as u64) | ((l.height as u32 as u64) << 32),
            stride: (l.stride as u32 as u64) | ((l.rect_word3 as u64) << 32),
            elem_bytes: l.elem_bytes,
        }
    }
}

/// Derivative launch: the source plane handle and the destination plane handle.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GatherArgs {
    pub shape: TileShape,
    pub src_plane: u64,
    pub dst_plane: u64,
}

/// Warp launch: the flow planes, the plane to warp and its output.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WarpArgs {
    pub shape: TileShape,
    pub flow_u: u64,
    pub flow_v: u64,
    pub src_plane: u64,
    pub out_plane: u64,
}

/// Mix launch: the derivative and warped planes combined into the coefficient plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MixArgs {
    pub shape: TileShape,
    pub flow_u: u64,
    pub flow_v: u64,
    pub in_a: u64,
    pub in_b: u64,
    pub in_c: u64,
    pub in_d: u64,
    pub out_plane: u64,
}

/// Flow update launch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlowArgs {
    pub shape: TileShape,
    pub flow_u: u64,
    pub flow_v: u64,
    pub flow_u_out: u64,
    pub flow_v_out: u64,
    pub coeff_a: u64,
    pub coeff_b: u64,
    pub coeff_c: u64,
    /// relax scale (f32 bits)
    pub relax_scale: u32,
    /// flow gain (f32 bits)
    pub flow_gain: u32,
}

/// Relax launch: two nests of planes and their accumulators.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RelaxArgs {
    pub shape: TileShape,
    pub relax_out_first: u64,
    pub nest1_t: u64,
    pub nest1_u: u64,
    pub relax_out_second: u64,
    pub nest2_t: u64,
    pub nest2_u: u64,
    pub nest1_acc: u64,
    pub nest2_acc: u64,
    /// relax scale (f32 bits)
    pub relax_scale: u32,
}

/// Smoothing launch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SmoothArgs {
    pub shape: TileShape,
    pub relax_out_first: u64,
    pub nest1_t: u64,
    pub nest1_u: u64,
    pub relax_out_second: u64,
    pub nest2_t: u64,
    pub nest2_u: u64,
    /// relax scale (f32 bits)
    pub relax_scale: u32,
    /// smoothing weight (f32 bits)
    pub smooth_weight: u32,
    /// smoothing bias (f32 bits)
    pub smooth_bias: u32,
}

/// Difference-sum launch: the flow planes against the two relax outputs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiffArgs {
    pub shape: TileShape,
    pub flow_u: u64,
    pub flow_v: u64,
    pub relax_out_first: u64,
    pub relax_out_second: u64,
}

/// Median launch: the input plane and the output plane.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MedianArgs {
    pub shape: TileShape,
    pub plane: u64,
    pub median_out: u64,
}

/// The typed launch arguments of one parallel-for callback.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaunchArgs {
    DerivX(GatherArgs),
    DerivY(GatherArgs),
    Warp(WarpArgs),
    Mix(MixArgs),
    Flow(FlowArgs),
    Relax(RelaxArgs),
    Smooth(SmoothArgs),
    Diff(DiffArgs),
    Median(MedianArgs),
}

impl LaunchArgs {
    pub fn callback(&self) -> Callback {
        match self {
            LaunchArgs::DerivX(_) => Callback::DerivX,
            LaunchArgs::DerivY(_) => Callback::DerivY,
            LaunchArgs::Warp(_) => Callback::Warp,
            LaunchArgs::Mix(_) => Callback::Mix,
            LaunchArgs::Flow(_) => Callback::Flow,
            LaunchArgs::Relax(_) => Callback::Relax,
            LaunchArgs::Smooth(_) => Callback::Smooth,
            LaunchArgs::Diff(_) => Callback::Diff,
            LaunchArgs::Median(_) => Callback::Median,
        }
    }
}

/// The refusal of a level table with a side below 2: the level build stores no such level, so the route is not rebuilt.
pub const SMALL_LEVEL_REFUSAL: &str = "OPTICAL-FLOW-SMALL-LEVEL-REFUSED (optical_flow: a pyramid level with a side below 2 reached the flow solve; the level build stores no such level, so this route is not rebuilt)";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    SmallLevel,
}

/// True when a level of `levels[0..=first]` has a width or height below 2 (the level build keeps every side at max(2, levels) or more).
pub fn small_level(levels: &[Level], first: i32) -> bool {
    if first < 0 {
        return false;
    }
    levels.iter().take(first as usize + 1).any(|l| l.width < 2 || l.height < 2)
}
