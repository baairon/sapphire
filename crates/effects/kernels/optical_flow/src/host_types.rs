//! Typed host calls of the optical flow body: the rectangle record, the launch callbacks and their argument blocks, the call records and the
//! `Host` trait the composed host implements. Plane handles are arena identifiers chosen by the caller, never addresses.

/// Arena identifier of a float plane in the host.
pub use optical_flow_pyramid::PlaneId;

/// A 24 byte rectangle record passed by value to the plane services.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Rect24 {
    pub lo: u128,
    pub hi: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Callback {
    /// Resample callbacks
    ResampleHoriz,
    ResampleVert,
    /// Smoothing callback
    Smooth,
    /// Blend callback
    Blend,
    FillDiff,
}

/// Argument block of a launch.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ctx {
    /// shared by the two resample callbacks
    Pair { prev_width: u32, prev_height: u32, source: u64, cur_width: u32, cur_height: u32 },
    Smooth { rect: Rect24, work: u64, flow_x: u64, flow_y: u64, source_copy: u64, weight: u32 },
    Blend { rect: Rect24, work: u64, flow_x: u64, flow_y: u64, weight: u32, pair_weight: u32 },
    Fill { rect: Rect24, operand_plane: u64, source_plane: u64, destination: u64, weight: u32 },
    /// argument block of the tile launch
    Tile { prev_width: u32, prev_height: u32, source: u64, cur_width: u32, cur_height: u32, destination: u64 },
}

/// Which stage of the glue made the call (selects nothing in the call; the argument block's place is the host's).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    Resample,
    TailLaunch,
    TailA,
    TailB,
    First,
    Second,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Call {
    /// the parallel launcher
    ParallelFor { stage: Stage, handle: u64, cb: Callback, ctx: Ctx },
    /// copy of a host input image into a plane
    CopyIn { dst: u64, flag: u32, rect: Rect24 },
    /// strided plane copy
    CopyPlane { dst: u64, src: u64, flag: u32, rect: Rect24 },
    /// image helper, flag route
    Fill { a: u64, b: u64, c: u64, x: u32, rect: Rect24 },
    /// image helper, flag route
    SmoothStep { a: u64, b: u64, c: u64, d: u64, x: u32, rect: Rect24 },
    /// image helper, flag route
    BlendStep { a: u64, b: u64, c: u64, x: u32, y: u32, rect: Rect24 },
}

pub trait Host {
    fn call(&mut self, c: Call);
}
