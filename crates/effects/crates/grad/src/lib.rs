//! Class `grad` (identity PROVEN: id getter returns "grad", factory slot 43 installs vtable, and the
//! programs of effect_builtins.tsv list `grad`). CPU render entry = slot 52, flag 0.
//!
//! Sources of truth: neutral_1 dump listings (offset convention, VA = listing).
//!   slot 52 wrapper : sigma = (this[0x6c] * k) * 0.125, gain = (k * 0.05) * this[0x88], k = (float) info[0x20]
//!   launcher : ROI, blur core (class blur) with sx = sy = sigma, passes 3,
//!                     edge = **cell, subpixel = this[0xa4], half 0; poll; flag != 0 -> (GPU, NOT REBUILT);
//!                     flag 0 -> parallel_for with the gradient callback 
//!   gradient callback : reads the blurred plane `aux`writes dx (output 0) and dy (output 1).
//! Std only, no unsafe. The blur half is the finished crate `blur` (read-only path dependency, R-PARITY, ROUTE AVX2-ONLY).
use blur::{blur_core, Edge};
pub use fx_common::plane::{FxError, Plane, PlaneMut};

// DRW2: MUTANT-SWITCH (W187 e). With `--features mutants` the mutant id is read at RUN time from the env var COV_MUT (one binary, SHA recorded);
// every `mu(id)` that is true also bumps a reach counter, printed by `reach_report()`. Without the feature `mu` is the constant false, so the
// shipped build is GR1's T2 source (the only difference to GR1's crate is this block).
#[cfg(feature = "mutants")]
pub fn mut_id() -> u32 {
    static M: std::sync::OnceLock<u32> = std::sync::OnceLock::new();
    *M.get_or_init(|| std::env::var("COV_MUT").ok().and_then(|t| t.parse().ok()).unwrap_or(0))
}
#[cfg(feature = "mutants")]
static REACH: [std::sync::atomic::AtomicU64; 64] = [const { std::sync::atomic::AtomicU64::new(0) }; 64];
#[cfg(feature = "mutants")]
#[inline(always)]
pub fn mu(id: u32) -> bool {
    let on = mut_id() == id;
    if on {
        REACH[(id as usize) & 63].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
    on
}
#[cfg(feature = "mutants")]
pub fn reach_report() -> String {
    let id = mut_id();
    format!("MUTID {} REACH {}", id, REACH[(id as usize) & 63].load(std::sync::atomic::Ordering::Relaxed))
}
#[cfg(not(feature = "mutants"))]
#[inline(always)]
pub fn mu(_id: u32) -> bool {
    false
}

/// Fields the render reads. Object offsets are `this+` or `info+` (info = the second argument of the slot, the image object).
#[derive(Clone, Debug)]
pub struct GradParams {
    /// this: blur amount (f32).
    pub amount: f32,
    /// this: gain multiplier (f32).
    pub gain_param: f32,
    /// this: subpixel flag handed to blur_core (nonzero = fractional radii).
    pub subpixel: i32,
    /// `**(int**)` (listing cell): 0 zero, 1 mirror, else error (same cell and rule as class blur, U03).
    pub edge_mode: i32,
    /// info as an integer (render scale k).
    pub render_scale: i32,
    /// info != 0 gives Some(f32 at info): multiplies the vertical sigma inside blur_core (B-A5).
    pub aspect: Option<f32>,
    /// info != 0 (read). PARAM-UNCAPTURED: the writer of this image-object field is not located. No default.
    pub info_aspect_set: bool,
    /// dword . CAPTURED in the dylib image as 1 and never written by code in the neutral_1 listing
    /// that was searched (only compares and address takes), so 1 is the value of the standing image; explicit so the other value runs in T2.
    pub flag_1: i32,
    /// Value the original reads one float past the end of `aux` when width == 1 (p[1] of the last row). Host memory, PARAM-UNCAPTURED.
    pub past_end: f32,
}

/// Dense-plane CPU render (flag 0). `src` is the source plane (w*h), `aux` receives the blurred plane (the original's scratch
/// output this[0x28][0]), `dx` and `dy` receive the two outputs. All dense, length at least w*h.
pub fn render(p: &GradParams, src: &[f32], w: usize, h: usize, aux: &mut [f32], dx: &mut [f32], dy: &mut [f32]) -> Result<(), FxError> {
    let n = w.checked_mul(h).ok_or(FxError::Geometry)?;
    if w > i32::MAX as usize || h > i32::MAX as usize || n > i32::MAX as usize {
        return Err(FxError::Geometry);
    }
    if src.len() < n || aux.len() < n || dx.len() < n || dy.len() < n {
        return Err(FxError::Stride);
    }
    if n == 0 {
        return Ok(()); // GR-A1: the original loops over zero rows; the blur core with an empty frame is not entered by any caller
    }
    let edge = Edge::from_i32(if mu(14) { 1 - p.edge_mode } else { p.edge_mode })?;
    let (sigma, gain) = sigma_gain(p);
    let passes = if mu(8) { 2 } else { 3 };
    blur_core(&src[..n], &mut aux[..n], w, h, sigma, sigma, gain, edge, passes, if mu(9) { p.subpixel == 0 } else { p.subpixel != 0 }, p.aspect)?;
    gradient(p, &aux[..n], w, h, &mut dx[..n], &mut dy[..n]);
    Ok(())
}

/// Slot 52 arithmetic : returns (sigma, gain).
/// xmm1 = (float)k; xmm0 = this[0x6c]; xmm0 = (xmm0 * xmm1) * 0.125; xmm1 = (xmm1 * 0.05f) * this[0x88].
pub fn sigma_gain(p: &GradParams) -> (f32, f32) {
    let k = p.render_scale as f32;
    let sigma = if mu(6) { p.amount * 0.125f32 } else { (p.amount * k) * 0.125f32 };
    let gain = if mu(7) { (k * 0.5f32) * p.gain_param } else { (k * 0.05f32) * p.gain_param };
    (sigma, gain)
}

/// Gradient callback with tile_whole (rows 0..h). `b` is the blurred plane. Operation order kept literal (scalar SSE).
/// Row scale s: xmm5 = 2.0, or 1.0 when != 0 and info != 0; xmm1 = xmm5 + xmm5; per row xmm3 = xmm1 on the first and last
/// row, else xmm5.  dx[0] = (p1 - p0) * 2.0; dx[x] = p[x+1] - p[x-1] (1 <= x <= w-2); dx[w-1] = (p[w-1] - p[w-2]) * 2.0.
/// dy[x] = (up - down) * (0.5 * xmm3) for interior columns; dy[0] and dy[w-1] = ((up - down) * 0.5) * xmm3, with up = row y-1 (own row
/// when y == 0) and down = row y+1 (own row when y == h-1). For w == 1 the original reads p[1] one float past the row (the next row, or
/// past the buffer on the last row): modelled with `past_end`.
pub fn gradient(p: &GradParams, b: &[f32], w: usize, h: usize, dx: &mut [f32], dy: &mut [f32]) {
    let two = 2.0f32; // xmm6 = 
    let half = 0.5f32; // xmm4 = 
    let mut x5 = two;
    if p.flag_1 != 0 {
        x5 = 1.0f32; // 
        if !p.info_aspect_set && !mu(4) {
            x5 = two;
        }
    }
    let x1 = x5 + x5;
    let at = |i: usize| -> f32 { if i < b.len() { b[i] } else { p.past_end } };
    for y in 0..h {
        let cur = y * w;
        let up = if y == 0 { cur } else { cur - w };
        let down = if y + 1 == h || (mu(12) && y + 2 == h) { cur } else { cur + w };
        let mut x3 = x1;
        if y != 0 && !mu(17) {
            x3 = x5;
        }
        if y + 1 == h && !mu(15) {
            x3 = x1;
        }
        if mu(5) {
            x3 = x5;
        }
        // column 0
        let p0 = b[cur];
        let p1 = at(cur + 1);
        let mut v7 = p1 - p0;
        let mut v8 = b[up] - b[down];
        v7 *= two;
        v8 *= half;
        v8 *= x3;
        dx[cur] = if mu(1) { p1 - p0 } else { v7 };
        dy[cur] = if mu(11) { (b[up] - b[down]) * x3 } else if mu(16) { (b[up] - b[down]) * (half * x3) } else { v8 };
        if mu(2) {
            dy[cur] = (b[down] - b[up]) * (half * x3);
        }
        if w > 2 {
            let f = if mu(18) { half * x5 } else { half * x3 };
            for x in 1..w - 1 {
                let d = if mu(3) { b[cur + x + 1] - b[cur + x] } else { b[cur + x + 1] - b[cur + x - 1] };
                dx[cur + x] = d;
                let v = b[up + x] - b[down + x];
                dy[cur + x] = v * f;
            }
        }
        // last column (also column 0 again when w == 1: same value)
        let lo = if w >= 2 { b[cur + w - 2] } else { p0 };
        let hi = if w >= 2 { b[cur + w - 1] } else { p1 };
        let mut l7 = if mu(19) { b[down + w - 1] - b[up + w - 1] } else { b[up + w - 1] - b[down + w - 1] };
        let mut l0 = hi - lo;
        l7 *= half;
        l0 *= if mu(10) { 1.0f32 } else { two };
        l7 *= x3;
        dx[cur + w - 1] = l0;
        dy[cur + w - 1] = l7;
    }
}

/// Slot 57, the margin getter (NOT a pixel route; not part of R-PARITY): returns the two integer margins
/// (floor-like(sigma) + 1) for the plain and the aspect-scaled sigma. `floor_like` is the libm routine, INFERRED to be
/// floorf or ceilf: the caller passes it, no stand-in is baked.
pub fn margins(p: &GradParams, floor_like: impl Fn(f32) -> f32) -> (i32, i32) {
    let k = p.render_scale as f32;
    let x0 = (k * p.amount) * 0.125f32; // cvtsi2ss k; mulss this[0x6c]; mulss 0.125 (operand order: k * amount)
    let x1 = match p.aspect {
        Some(asp) => asp * x0,
        None => x0,
    };
    (floor_like(x0) as i32 + 1, floor_like(x1) as i32 + 1)
}

/// One stand-in or decision record (S7).
#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub what: &'static str,
    pub tier: &'static str,
    pub label: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "GR-A1", what: "info != 0 : selects the dy scale 1.0 versus 2.0 together with cell; writer of the image-object field not located, explicit parameter, no default, T2 runs both values", tier: "PARAM-UNCAPTURED", label: "PARAM-UNCAPTURED" },
        Assumption { id: "GR-A2", what: "dword = 1 in the captured image; only compared or address-taken in the searched listing; T2 patches it to 0 and 1", tier: "CAPTURED", label: "MEASURED-DYLIB, single-writer-search INFERRED" },
        Assumption { id: "GR-A3", what: "width == 1: the original reads one float past the plane (p[1] of the last row); the value is host memory, explicit parameter past_end", tier: "PARAM-UNCAPTURED", label: "PARAM-UNCAPTURED" },
        Assumption { id: "GR-A4", what: "tile partition returns one tile (start 0, count h) and parallel_for runs the callback once (single worker); the row loop is start-relative so a tile split would only change which rows the callback sees", tier: "ROUTE", label: "HOST-HOOKED" },
        Assumption { id: "GR-A5", what: "cancel poll returns 0 (as B-A1 of class blur)", tier: "SEMANTIC", label: "ASSUMED" },
        Assumption { id: "GR-A6", what: "render flag != 0 (GPU launcher) is not rebuilt; round trip through blur uses edge mode cell (U03), passes 3, half 0", tier: "ROUTE", label: "NOT REBUILT" },
        Assumption { id: "GR-A7", what: "blur ROI record via with info field == 0 only: dense planes {w,h,c=w,d=0,e=4} (B-A2); info field != 0 (the branch) not entered", tier: "ROUTE", label: "ASSUMED" },
        Assumption { id: "GR-A8", what: "blur_core feature route AVX2 only (word 0x9d97ff), scratch phase 0 (inherits B-A4 and ROUTE AVX2-ONLY from crate blur)", tier: "ROUTE", label: "ROUTE AVX2-ONLY, ALIGN-DEPENDENT" },
    ]
}
