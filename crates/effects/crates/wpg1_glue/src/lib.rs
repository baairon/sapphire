//! WPG1 glue (2026-11-02). Three named slot functions, each composing finished body crates by read-only path dependency:
//!
//! * `slot52_perspective`: class perspective (vtable), slot 52 = dispatcher over the AVX2 body (PX1).
//! * `slot52_perspective_alpha`: class perspective_alpha (vtable), slot 52 = dispatcher over the AVX2 body (PX2).
//! * `slot53_warp`: class warp (vtable), slot 53 = (context build, optional image geometry override,
//!   then the driver whose AVX2 body is), the F1 sampler inside it from the warp_f1 crate.
//!
//! Engine statement: the emulator follows the Intel SDM, not silicon. ROUTE AVX2-ONLY: the feature word is an explicit input and any
//! tier other than the AVX2 one is reported as `RouteNotRebuilt`. HOST-FP-ASSUMED (round to nearest, no FTZ, no DAZ).

pub use perspective_alpha::{Error as AlphaError, Info as AlphaInfo, Outcome as AlphaOutcome, Params as AlphaParams, Route as AlphaRoute};
pub use warp_driver::{Args as DriverArgs, DriverError, Image as DriverImage, Obj as DriverObj, Rec as DriverRec};
pub use warp_f1::{Lattice, Params as F1Params};

/// Dispatcher tests shared by (MEASURED):
/// `w = w & 0x9d97ff == 0x9d97ff` jumps to the AVX2 body; else `w & 0x117ff == 0x117ff` jumps to the AVX middle tier
/// else ` & 1` jumps to the SSE twin; else (CPU init) and retry.
pub const FW_AVX2: u64 = 0x9d97ff;
pub const FW_MID: u64 = 0x117ff;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Avx2,
    AvxMiddle,
    Sse,
    CpuInit,
}

/// The tier the dispatcher selects for the feature word `w`.
pub fn tier(w: u64) -> Tier {
    if w & FW_AVX2 == FW_AVX2 {
        Tier::Avx2
    } else if w & FW_MID == FW_MID {
        Tier::AvxMiddle
    } else if w & 1 != 0 {
        Tier::Sse
    } else {
        Tier::CpuInit
    }
}

#[derive(Debug, PartialEq)]
pub enum Slot52Error {
    /// the dispatcher picked a tier whose code is not ported (ROUTE AVX2-ONLY)
    RouteNotRebuilt(Tier),
    /// flag != 0 reached the GPU stub (class perspective): arguments recorded, NOT REBUILT
    Gpu { b: f32 },
    /// [this] is null (the original faults on the first read)
    NullInputs,
    Alpha(AlphaError),
}

// ---------------------------------------------------------------------------------------------------------------------------------
// class perspective, slot 52 

/// The fields slot 52 reads of the primitive `this` (MEASURED, PX1 SPEC): f32 a, gpu_call_a (angle parameter), b, pointer
/// to the input image object whose and are the u and v arrays.
#[derive(Clone, Debug)]
pub struct PerspThis<'a> {
    pub a: f32,
    pub gpu_call_a: f32,
    pub b: f32,
    pub inputs: Option<(&'a [f32], &'a [f32])>,
}

/// Slot 52 of class perspective. `n` is [args]; `out2_phase_bytes` = (address of out2[0]) & 0x1f (the buffer phase, an explicit input
/// per W55; out1 is not address dependent). Writes out1[..n] and out2[..n].
pub fn slot52_perspective(fw: u64, t: &PerspThis, flag: bool, n: i32, out2_phase_bytes: u32, out1: &mut [f32], out2: &mut [f32]) -> Result<(), Slot52Error> {
    match tier(fw) {
        Tier::Avx2 => {}
        other => return Err(Slot52Error::RouteNotRebuilt(other)),
    }
    let (u, v): (&[f32], &[f32]) = t.inputs.unwrap_or((&[], &[]));
    let o = perspective::Obj { port_obj_c: t.a, gpu_call_a: t.gpu_call_a, port_obj_d: t.b };
    match perspective::run_full(&o, t.inputs.is_some(), flag, n, out2_phase_bytes, u, v, out1, out2) {
        Ok(()) => Ok(()),
        Err(perspective::Path::NullInputs) => Err(Slot52Error::NullInputs),
        Err(perspective::Path::GpuStub(g)) => Err(Slot52Error::Gpu { b: g.b }),
    }
}

// ---------------------------------------------------------------------------------------------------------------------------------
// class perspective_alpha, slot 52 

/// Slot 52 of class perspective_alpha: one alpha ramp plane over `src` (the v array of the input image object), or the ROI zero fill when
/// the product x8 = r1 * params_a is zero. `dst_phase` = (address of dst[0]) & 0x1f.
pub fn slot52_perspective_alpha(fw: u64, par: &AlphaParams, info: &AlphaInfo, flag: bool, src: &[f32], dst: &mut [f32], dst_phase: usize) -> Result<AlphaOutcome, Slot52Error> {
    match tier(fw) {
        Tier::Avx2 => {}
        other => return Err(Slot52Error::RouteNotRebuilt(other)),
    }
    perspective_alpha::perspective_alpha(par, info, flag, src, dst, dst_phase).map_err(Slot52Error::Alpha)
}

/// Test hook: the same slot with the uncaptured vector clamp cell (PX2-A1) as an explicit value (T2 varies it on both sides).
#[allow(clippy::too_many_arguments)]
pub fn slot52_perspective_alpha_with(fw: u64, ramp_max: f32, par: &AlphaParams, info: &AlphaInfo, flag: bool, src: &[f32], dst: &mut [f32], dst_phase: usize) -> Result<AlphaOutcome, Slot52Error> {
    match tier(fw) {
        Tier::Avx2 => {}
        other => return Err(Slot52Error::RouteNotRebuilt(other)),
    }
    perspective_alpha::perspective_alpha_with(&perspective_alpha::sincos_native, ramp_max, par, info, flag, src, dst, dst_phase).map_err(Slot52Error::Alpha)
}

// ---------------------------------------------------------------------------------------------------------------------------------
// class warp, slot 53 

/// CONTRACT-LIMIT (W61, inherited from the driver crate): plane count n = [image] above 10 overwrites the driver's frame; F1 (flag path)
/// is contracted to n <= 4.
pub const CONTRACT_MAX_N_BLEND: i32 = warp_driver::MAX_N_BLEND;
pub const CONTRACT_MAX_N_F1: i32 = 4;

/// The primitive instance (rdi of). Fields as read by .
#[derive(Clone, Debug)]
pub struct WarpPrim {
    /// byte [P]: nonzero = constant coordinates (a1 = &P[0x6c], a2 = &P[0x70])
    pub const_coords: bool,
    pub const_a1: f32,
    pub const_a2: f32,
    /// arrays [[P]] and [[P]] (used when const_coords is false; base + 4 * idx)
    pub a1_arr: Vec<f32>,
    pub a2_arr: Vec<f32>,
    /// arrays [P], [P] (written by slot 33; base + 4 * idx)
    pub c1_arr: Vec<f32>,
    pub c2_arr: Vec<f32>,
    /// byte [P]
    pub flag: u8,
    /// i32 [P], [P]
    pub mx: i32,
    pub my: i32,
}

/// The byte argument record (rsi of), only the fields the driver and the glue read (MEASURED, and the
/// driver SPEC): count, aspect_block_d, assumption_e, idx, assumption_c, assumption_d, assumption_f, assumption_g, assumption_h.
#[derive(Clone, Copy, Debug)]
pub struct WarpRec {
    pub count: i32,
    pub aspect_block_d: f32,
    pub assumption_e: f32,
    pub idx: i32,
    pub assumption_c: i32,
    pub assumption_d: i32,
    pub assumption_f: f32,
    pub assumption_g: f32,
    pub assumption_h: f32,
}

/// The image object's geometry block, read only when byte [img] != 0 : [img] -> ctx,
/// [img] -> ctx, [img] -> ctx, [img] = aspect (argument of), [img] -> ctx,
/// [img] -> ctx.
#[derive(Clone, Copy, Debug)]
pub struct ImageGeom {
    pub left: f32,
    pub right: f32,
    pub assumption_h: f32,
    pub aspect: f32,
    pub assumption_c: i32,
    pub assumption_d: i32,
}

#[derive(Debug, PartialEq)]
pub enum WarpError {
    /// plane count above the contract (see the constants)
    ContractLimit { n: i32 },
    /// idx negative or beyond an array (the original would read outside the array)
    BadIndex,
    /// aspect == 0 in the geometry block: logs the error and returns; aspect_block_d stays the record's. Not an error here, see `build_ctx`.
    Driver(DriverError),
}

/// Context build (copies the record to the frame; then the geometry override of). Returns the driver object.
/// (ctx, aspect): for aspect != 0 stores [ctx] = 1.0f / aspect (divss of the captured 1.0f);
/// for aspect == 0 it only logs and returns, leaving aspect_block_d. The other fields it writes are not read
/// by the driver. After the call `mov [rbp], 0` zeroes [ctx] = assumption_e.
pub fn build_ctx(rec: &WarpRec, geom: Option<&ImageGeom>) -> DriverObj {
    let mut o = DriverObj { count: rec.count, aspect_block_d: rec.aspect_block_d, assumption_e: rec.assumption_e, assumption_c: rec.assumption_c, assumption_d: rec.assumption_d, assumption_f: rec.assumption_f, assumption_g: rec.assumption_g, assumption_h: rec.assumption_h };
    if let Some(g) = geom {
        o.assumption_f = g.left;
        o.assumption_g = g.right;
        o.assumption_h = g.assumption_h;
        o.assumption_c = g.assumption_c;
        o.assumption_d = g.assumption_d;
        if g.aspect != 0.0 {
            o.aspect_block_d = 1.0f32 / g.aspect;
        }
        o.assumption_e = 0.0;
    }
    o
}

fn tail(v: &[f32], idx: i32) -> Result<&[f32], WarpError> {
    let i = usize::try_from(idx).map_err(|_| WarpError::BadIndex)?;
    if i > v.len() {
        return Err(WarpError::BadIndex);
    }
    Ok(&v[i..])
}

/// What the wrapper hands the driver : slices are the arrays offset by the element index `idx` = [rec].
#[derive(Clone, Debug, PartialEq)]
pub struct WrapArgs {
    pub a1: Vec<f32>,
    pub a2: Vec<f32>,
    pub const_coords: bool,
    pub c1: Vec<f32>,
    pub c2: Vec<f32>,
    pub flag: i32,
    pub mx: i32,
    pub my: i32,
}

pub fn wrapper_args(p: &WarpPrim, idx: i32) -> Result<WrapArgs, WarpError> {
    let (a1, a2) = if p.const_coords { (vec![p.const_a1], vec![p.const_a2]) } else { (tail(&p.a1_arr, idx)?.to_vec(), tail(&p.a2_arr, idx)?.to_vec()) };
    Ok(WrapArgs { a1, a2, const_coords: p.const_coords, c1: tail(&p.c1_arr, idx)?.to_vec(), c2: tail(&p.c2_arr, idx)?.to_vec(), flag: p.flag as i32, mx: p.mx, my: p.my })
}

/// True when takes its error path (aspect == 0 in the image geometry block: it logs through and returns).
pub fn ctx_logs_error(geom: Option<&ImageGeom>) -> bool {
    geom.map_or(false, |g| g.aspect == 0.0)
}

/// Slot 53 of class warp : `img` is the driver's image context (plane count, tables, lattice dims; [this]), `lat` the host
/// buffers behind F1/the accessors, `out` one Vec per plane (count entries each). Returns the driver's call record.
pub fn slot53_warp(
    p: &WarpPrim,
    rec: &WarpRec,
    geom: Option<&ImageGeom>,
    img: &mut DriverImage,
    lat: &Lattice,
    out: &mut [Vec<f32>],
) -> Result<Vec<DriverRec>, WarpError> {
    let n = img.f1.n;
    if n > CONTRACT_MAX_N_BLEND {
        return Err(WarpError::ContractLimit { n });
    }
    if p.flag != 0 && n > CONTRACT_MAX_N_F1 {
        // F1 (reached only on the flag path) smashes its frame above n = 4 (CONTRACT-LIMIT, W61)
        return Err(WarpError::ContractLimit { n });
    }
    let obj = build_ctx(rec, geom);
    let w = wrapper_args(p, rec.idx)?;
    let a = DriverArgs { a1: &w.a1, a2: &w.a2, const_coords: w.const_coords, c1: &w.c1, c2: &w.c2, flag: w.flag, mx: w.mx, my: w.my };
    warp_driver::run(&obj, img, lat, &a, out).map_err(WarpError::Driver)
}

#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub basis: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "WPG1-A1", basis: "ROUTE AVX2-ONLY for slots 52 (perspective, perspective_alpha): dispatcher mask 0x9d97ff selects the AVX2 body; the AVX middle tier and SSE twin are not ported; the feature word is an explicit input", label: "ROUTE AVX2-ONLY" },
        Assumption { id: "WPG1-A2", basis: "slot 53 of class warp enters the driver whose AVX2 body is ported (warp_driver); ROUTE AVX2-ONLY inherited; the ISA dispatcher is entered in the emulator with the feature word served", label: "ROUTE AVX2-ONLY" },
        Assumption { id: "WPG1-A3", basis: "byte [img] (image geometry present) is an explicit input: None keeps the record's fields (the path the executor feeds, same geometry as warp_rel and blur_warp); Some applies the listing's override and (aspect_block_d = 1/aspect, assumption_e = 0). Which path a real render takes is not measured", label: "PARAM-UNCAPTURED 1" },
        Assumption { id: "WPG1-A4", basis: "slot 33 runs before slot 53 on the flag 0 route (WG, MEASURED order) and writes P, P, P; here c1, c2 and the flag byte are explicit inputs of WarpPrim (as in warp_rel WR1-A2)", label: "SLOT33-INPUTS" },
        Assumption { id: "WPG1-A5", basis: "memcpy and memset in are functional models in the emulator T2 (W89 LIBC-MODEL); the plane fill and descriptor of the empty-image path are call records only (inherited from the driver crate)", label: "LIBC-MODEL" },
    ]
}
