//! Slot 52 entry of the optical flow class (listing 0x48cf80) and its final callback 0x478790 (AVX2 and FMA arm 0x4787e0..0x478d80).
//!
//! Scope: the parameter block the entry builds for the pyramid body and the solve body, the fixed launch order of the entry, the scale
//! copy helper model, and the per pixel body of callback 0x478790. The two bodies come from `optical_flow` and `optical_flow_solve`, the
//! callbacks 0x46dca0 and 0x46e550 are the solve body's fetch::deriv_x and fetch::deriv_y, and the host region plumbing (0xa993e, 0x6ad1c0, 0xb629f0, 0xb5fe20, 0x6b1340, 0x48dbd0, 0x490640, 0x48f770)
//! is a host service that is not ported.
//! The slot 60 descriptor 0x48cda0 is a combo-table builder (one nine-operand row) and is not ported.

use fx_common::nanop::{fma213, fms213, NanOps};

/// A stand-in or a cell value used by the port, with how it is fixed.
pub struct Assumption {
    pub id: &'static str,
    pub label: &'static str,
    pub basis: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "EDGE-EPS", label: "BYTES-AT-LOAD", basis: "scalar 0x38d1b717 (1.0e-4) (MEASURED-EMULATOR-READ there)" },
        Assumption { id: "EDGE-MARGIN", label: "MEASURED-EMULATOR-READ", basis: "scalar 0x3f800347" },
        Assumption { id: "ONE", label: "MEASURED-EMULATOR-READ", basis: "scalar 1.0, lane 0 of the 32 byte constant block" },
        Assumption { id: "ABS-MASK", label: "MEASURED-EMULATOR-READ", basis: "lane 0 of the 16 byte vandps operand is 0x7fffffff (read in other pieces); modelled as a sign clear" },
        Assumption { id: "SCALE-THRESHOLD", label: "MEASURED-EMULATOR-READ", basis: "the cell compared with the texture scale holds the word 0x3c23d70a (0.01); the named constant SCALE_THRESHOLD" },
        Assumption { id: "NEG-ONE", label: "BYTES-AT-LOAD", basis: "lane 7 of the 32 byte constant block, -1.0; the scale argument of the scale copy helper call" },
        Assumption { id: "SCALE-COPY", label: "PORTED-LISTING-READ", basis: "the original's scale-copy routine and its row worker, read in the listing: a plain copy when the scale equals one; on the parallel route, a zero fill when the scale compares equal to zero, else one unfused f32 multiply of each source element by the scale; the routine's other path (a call to a separate routine) is not on the entry's route and is not ported; the entry calls it once, with scale minus one, on level 0's flow_v" },
        Assumption { id: "PLANE-MAP", label: "INFERRED-PLANE-MAP", basis: "the entry's planes are the level 0 record fields flow_u, flow_v, flow_u_next, flow_v_next, nest1_t, nest2_t and median_out; the final warp reads image_a as target, image_b as image, flow_u and flow_v as displacement, flow_u_next, flow_v_next, nest1_t and nest2_t as gradient planes and writes median_out, with the lambda operand as its scale; the outputs copy flow_u, flow_v scaled by -1 and median_out" },
        Assumption { id: "OBJECT-LAYOUT", label: "INFERRED-OBJECT-LAYOUT", basis: "the seven operands sit at object offsets 0x8c + 0x1c * k in program order: max_warps, max_outer, max_inner, lambda, the constant 1.0, the texture scale, std_smooth" },
    ]
}

pub const EDGE_EPS: f32 = f32::from_bits(0x38d1_b717);
pub const EDGE_MARGIN: f32 = f32::from_bits(0x3f80_0347);
pub const ONE: f32 = 1.0;
const QUIET_BIT: u32 = 1 << 22;
pub const NEG_ONE: f32 = -1.0;
/// Threshold the texture scale is compared with (MEASURED-EMULATOR-READ, word 0x3c23d70a).
pub const SCALE_THRESHOLD: f32 = f32::from_bits(0x3c23_d70a);

/// Fields of the operator object passed to the entry, as the entry reads them.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ObjFields {
    /// +0x8c: warp count bound.
    pub max_warps: i32,
    /// +0xa8: outer iteration bound.
    pub max_outer: i32,
    /// +0xc4: inner iteration bound.
    pub max_inner: i32,
    /// +0xfc: the use_median flag (an int).
    pub use_median: i32,
    /// +0xe0: bits of the lambda operand.
    pub lambda_bits: i32,
    /// +0x118: texture scale.
    pub texture_scale: f32,
    /// +0x134: bits of the std_smooth operand.
    pub std_smooth_bits: i32,
}

/// The 0x30 byte block passed by address to the flow-build body (0x47a380) and the solve body (0x483ab0), at [rbp-0xd8] .. [rbp-0xa4].
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LaneParams {
    pub max_warps: i32,
    pub max_outer: i32,
    pub max_inner: i32,
    pub use_median: i32,
    pub lambda_bits: i32,
    pub smooth_weight: f32,
    pub tiny_epsilon: f32,
    pub four: i32,
    pub scale_gt: i32,
    pub scale_bits: u32,
    pub std_smooth_bits: i32,
    pub diag_weight: f32,
    pub small_weight: f32,
    pub one: i32,
}

/// Block built by the entry from the object (0x48d004..0x48d0d7). `threshold` is the cell 0x15e8af8, SCALE_THRESHOLD in the shipped host.
pub fn lane_params(o: &ObjFields, threshold: f32) -> LaneParams {
    LaneParams {
        max_warps: o.max_warps,
        max_outer: o.max_outer,
        max_inner: o.max_inner,
        use_median: o.use_median,
        lambda_bits: o.lambda_bits,
        smooth_weight: f32::from_bits(0x3e66_6666),
        tiny_epsilon: f32::from_bits(0x2edb_e6ff),
        four: 4,
        // comiss xmm0,[cell]; jbe -> 0: the flag is 1 only when scale > threshold and ordered
        scale_gt: if o.texture_scale > threshold { 1 } else { 0 },
        scale_bits: o.texture_scale.to_bits(),
        std_smooth_bits: o.std_smooth_bits,
        diag_weight: f32::from_bits(0x3f34_fdf4),
        small_weight: f32::from_bits(0x3ca3_d70a),
        one: 1,
    }
}

/// The six planes the solve body leaves and the final plane, X0..X6 of the entry (level 0 record fields, INFERRED-PLANE-MAP).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryPlane {
    FlowU,
    FlowV,
    FlowUNext,
    FlowVNext,
    Nest1T,
    Nest1U,
    Nest2T,
    /// Level 0's median_out plane (the state block's image_table plane), which the final warp writes.
    MedianOut,
}

/// The three output planes of the entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OutPlane {
    FlowU,
    FlowV,
    Confidence,
}

/// One launch of the in-line path (0x48d1f7..0x48d375).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Launch {
    /// 0x46dca0, the solve body's x derivative (fetch::deriv_x) of plane `src` into plane `dst`.
    DerivX { src: EntryPlane, dst: EntryPlane },
    /// 0x46e550, the solve body's y derivative (fetch::deriv_y) of plane `src` into plane `dst`.
    DerivY { src: EntryPlane, dst: EntryPlane },
    /// 0x478790 over the nine plane block.
    FinalWarp,
}

/// The fixed launch order of the in-line path.
pub const LAUNCHES: [Launch; 5] = [
    Launch::DerivX { src: EntryPlane::FlowU, dst: EntryPlane::FlowUNext },
    Launch::DerivY { src: EntryPlane::FlowU, dst: EntryPlane::FlowVNext },
    Launch::DerivX { src: EntryPlane::FlowV, dst: EntryPlane::Nest1T },
    Launch::DerivY { src: EntryPlane::FlowV, dst: EntryPlane::Nest2T },
    Launch::FinalWarp,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EntryPath {
    /// The pyramid body returned non-zero: the entry returns at once.
    PyramidStopped,
    /// The poll 0x6e8c10 returned non-zero after the solve body.
    Aborted,
    /// Operand flag set: the deferred region path (0x48dbd0, 0x490640, 0x48f770). Host plumbing, not ported.
    Deferred,
    /// The in-line path ran the five launches and the three output copies.
    InLine,
}

/// The services the entry calls. The two bodies and the two warp callbacks come from `optical_flow` and `optical_flow_solve`.
pub trait EntryHost {
    fn pyramid_body(&mut self, p: &LaneParams) -> i32;
    fn solve_body(&mut self, p: &LaneParams);
    fn poll_abort(&mut self) -> bool;
    fn launch(&mut self, l: Launch);
    fn copy_plane(&mut self, src: EntryPlane, out: OutPlane);
    fn scale_copy(&mut self, src: EntryPlane, out: OutPlane, k: f32);
}

/// Control flow of slot 52 entry 0x48cf80. `deferred` is the low byte of the fourth argument.
pub fn run_entry<H: EntryHost>(h: &mut H, o: &ObjFields, threshold: f32, deferred: bool) -> EntryPath {
    let p = lane_params(o, threshold);
    if h.pyramid_body(&p) != 0 {
        return EntryPath::PyramidStopped;
    }
    h.solve_body(&p);
    if h.poll_abort() {
        return EntryPath::Aborted;
    }
    if deferred {
        return EntryPath::Deferred;
    }
    for l in LAUNCHES {
        h.launch(l);
    }
    h.copy_plane(EntryPlane::FlowU, OutPlane::FlowU);
    h.scale_copy(EntryPlane::FlowV, OutPlane::FlowV, NEG_ONE);
    h.copy_plane(EntryPlane::MedianOut, OutPlane::Confidence);
    EntryPath::InLine
}

/// Port of 0x672f80 and its row worker 0x672ee0: out = in * k element wise, positive zero when k equals zero, a plain copy when k equals one (see assumptions()).
pub fn scale_copy_model(dst: &mut [f32], src: &[f32], k: f32) {
    if k == ONE {
        dst.copy_from_slice(src);
    } else if k == 0.0 {
        dst.fill(0.0);
    } else {
        for (d, s) in dst.iter_mut().zip(src) {
            *d = s.fmul(k);
        }
    }
}

/// Plane block of callback 0x478790 (the structure at the address held in cell 0x1a991a0).
pub struct WarpPlanes<'a> {
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub target_plane: &'a [f32],
    pub img: &'a [f32],
    pub u: &'a [f32],
    pub v: &'a [f32],
    pub grad_a_x: &'a [f32],
    pub grad_a_y: &'a [f32],
    pub grad_b_x: &'a [f32],
    pub grad_b_y: &'a [f32],
    pub scale: f32,
}

#[inline(always)]
fn andabs(x: f32) -> f32 {
    f32::from_bits(x.to_bits() & 0x7fff_ffff)
}

/// sqrtss: a NaN input is returned quieted, a negative operand gives the default NaN.
#[inline(always)]
fn sqrt_x86(x: f32) -> f32 {
    if x.is_nan() {
        f32::from_bits(x.to_bits() | QUIET_BIT)
    } else if x < 0.0 {
        f32::from_bits(0xffc0_0000)
    } else {
        x.sqrt()
    }
}

/// vroundss mode 1 (floor): a NaN is returned quieted.
#[inline(always)]
fn floor_x86(x: f32) -> f32 {
    if x.is_nan() { f32::from_bits(x.to_bits() | QUIET_BIT) } else { x.floor() }
}

/// vcvtss2si of an already integral value: NaN and out of range give the integer indefinite.
#[inline(always)]
fn cvtss2si_x86(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 { i32::MIN } else { x as i32 }
}

#[inline(always)]
fn clamp_idx(i: i32, dim: i32) -> i32 {
    if i < 0 { 0 } else { (dim.wrapping_sub(1)).min(i) }
}

/// Bilinear fetch at (x, y) with clamped indices (0x478ac1..0x478d77).
pub fn bilinear(img: &[f32], w: i32, h: i32, stride: i32, x: f32, y: f32) -> f32 {
    let ix = cvtss2si_x86(floor_x86(x));
    let iy = cvtss2si_x86(floor_x86(y));
    let next_x = ix.wrapping_add(1);
    let next_y = iy.wrapping_add(1);
    let fx = x.fsub(ix as f32);
    let at = |xx: i32, yy: i32| -> f32 {
        let col_clamped = clamp_idx(xx, w);
        let row_clamped = clamp_idx(yy, h);
        img[(col_clamped as i64 + (row_clamped.wrapping_mul(stride)) as i64) as usize]
    };
    let a = at(ix, iy);
    let b = at(next_x, iy);
    let c = at(ix, next_y);
    let d = at(next_x, next_y);
    let top_delta = b.fsub(a);
    let fy = y.fsub(iy as f32);
    let bottom_delta = d.fsub(c);
    let left_delta = a.fsub(c);
    let delta_diff = bottom_delta.fsub(top_delta);
    let top_lerp = fma213(top_delta, fx, a);
    let bottom_lerp = fms213(delta_diff, fx, left_delta);
    fma213(fy, bottom_lerp, top_lerp)
}

/// Callback 0x478790 over rows `row_start .. row_start + row_count`, writing plane `out` (index = row * stride + column).
pub fn final_warp_rows(p: &WarpPlanes, out: &mut [f32], row_start: i32, row_count: i32) {
    if p.width <= 0 {
        return;
    }
    let limx = (p.width as f32).fsub(EDGE_MARGIN);
    let limy = (p.height as f32).fsub(EDGE_MARGIN);
    for r in 0..row_count.max(0) {
        let row = row_start.wrapping_add(r);
        let off = row.wrapping_mul(p.stride) as i64;
        let rowf = row as f32;
        for col in 0..p.width {
            let i = (off + col as i64) as usize;
            let colf = col as f32;
            let sy = rowf.fadd(p.v[i]);
            let sx = colf.fadd(p.u[i]);
            let outside = EDGE_EPS >= sx || sx >= limx || EDGE_EPS >= sy || sy >= limy;
            let sample = if outside { 0.0 } else { bilinear(p.img, p.width, p.height, p.stride, sx, sy) };
            let grad_a_y = p.grad_a_y[i];
            let grad_b_y = p.grad_b_y[i];
            let sq_a_y = grad_a_y.fmul(grad_a_y);
            let sq_b_y = grad_b_y.fmul(grad_b_y);
            let mag_a = sqrt_x86(fma213(p.grad_a_x[i], p.grad_a_x[i], sq_a_y));
            let mag_b = sqrt_x86(fma213(p.grad_b_x[i], p.grad_b_x[i], sq_b_y));
            let diff = p.target_plane[i].fsub(sample);
            let sum = mag_a.fadd(mag_b);
            let t = fma213(andabs(diff), p.scale, ONE);
            out[i] = ONE.fdiv(t.fadd(sum));
        }
    }
}

pub mod compose;
pub mod placement;
pub mod wiring;
