//! Optical flow solver, AVX2 and FMA route: the level loop of the body and three per-tile kernels.
//!
//! Kernels (each takes a row range and the alignment phase of the plane that selects the vector and scalar split,
//! because the vector and scalar forms round differently):
//! - `fma_mix`: out = -(e*c - b) - (f*d + a) per element, eight lane groups of 16 in the vector form.
//! - `flow_update`: a 2x2 normal-equation step with a clamp, producing two updated planes.
//! - `diff_sum`: copies two planes and returns the sum of squared differences of the old and new values.
//! The body (`solve::run_body`) walks the pyramid levels and the nested iteration loops over planes addressed by handle.
//! Assumption labels: see `assumptions()`.
use fx_common::nanop::{fma213, fma231, fms213, fnms132, NanOps};
use fx_common::x86::flip_sign;

pub mod body;
pub use body::*;
pub mod smooth;
pub mod reduce;
pub mod stretch;
pub mod median;
pub mod tail;
pub mod relax;
pub mod fetch;
pub mod consts;
pub mod exact;
pub mod scalar;
pub mod scalar_consts;
mod scalar_core;
pub mod tables;
pub mod vmath;
pub mod solve;
pub use fetch::*;

/// Assumed values of cells that are not part of the test image. The kernels take them as arguments.
pub struct Assumption {
    pub id: &'static str,
    pub label: &'static str,
    pub basis: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption {
            id: "SIGN-MASK",
            label: "ASSUMED",
            basis: "16 byte operand of vxorps used to negate lane 0; modelled as the f32 sign bit flip (0x80000000 in lane 0)",
        },
        Assumption {
            id: "DET-EPS",
            label: "ASSUMED",
            basis: "f32 threshold compared with the determinant in flow_update; an argument of the kernel, tested at several values",
        },
        Assumption {
            id: "ALIGN-MODEL",
            label: "ASSUMED",
            basis: "the byte address of the first plane element modulo 32 is an argument; the listing chooses vector or scalar code from it",
        },
    ]
}

/// Plane view shared by the kernels: `stride` in elements between rows, `width` columns.
#[derive(Clone, Copy)]
pub struct Shape {
    pub width: i32,
    pub stride: i32,
    /// byte address of the aligned-test plane's first element, modulo 32
    pub phase_bytes: u32,
}

pub fn row_off(row: i32, stride: i32) -> usize {
    (row.wrapping_mul(stride)) as i64 as usize
}

pub fn row_phase(sh: &Shape, off: usize) -> u32 {
    (sh.phase_bytes.wrapping_add((off as u32).wrapping_mul(4))) & 0x1f
}

/// Split of one row of `w` elements into scalar head, vector range and scalar tail for a vector width of `lanes` (16 or 8):
/// returns (head, vec_end, scalar_all). Mirrors the listing's alignment tests; `min_aligned` is the width from which the head
/// is aligned (29 for 16 lanes, 907 for 8 lanes).
pub fn split(w: i64, lanes: i64, min_aligned: i64, addr_phase: u32) -> (i64, i64, bool) {
    if w < lanes {
        return (0, 0, true);
    }
    if w < min_aligned {
        return (0, w & !(lanes - 1), false);
    }
    let mut head = 0i64;
    let ph = addr_phase as i64;
    if ph != 0 {
        if ph & 3 != 0 {
            return (0, 0, true);
        }
        head = (0x20 - ph) >> 2;
    }
    if w < head + lanes {
        return (0, 0, true);
    }
    let vec_end = w - ((w - head) & (lanes - 1));
    (head, vec_end, false)
}

/// Operands of the final subtraction of the scalar form (first, second).
fn mix_ops_scalar(e: f32, c: f32, b: f32, f: f32, d: f32, a: f32) -> (f32, f32) {
    let first_operand = fms213(e, c, b);
    let second_operand = fma213(f, d, a);
    (flip_sign(first_operand), second_operand)
}

/// Operands of the final subtraction of the vector form (first, second).
fn mix_ops_vector(e: f32, c: f32, b: f32, f: f32, d: f32, a: f32) -> (f32, f32) {
    let t = fms213(f, d, b);
    let u = fnms132(e, t, c);
    (u, a)
}

fn mix_scalar(e: f32, c: f32, b: f32, f: f32, d: f32, a: f32) -> f32 {
    let (x, y) = mix_ops_scalar(e, c, b, f, d, a);
    x.fsub(y)
}

fn mix_vector(e: f32, c: f32, b: f32, f: f32, d: f32, a: f32) -> f32 {
    let (x, y) = mix_ops_vector(e, c, b, f, d, a);
    x.fsub(y)
}

/// Diagnostic twin: the two operands of the final subtraction at (row, col) and whether the vector form computed it.
/// Gives the operands from which the NaN of a two-NaN subtraction is taken.
pub fn mix_final_operands(inp: &MixIn, sh: &Shape, row: i32, col: usize) -> (f32, f32, bool) {
    let off = row_off(row, sh.stride);
    let w = sh.width as i64;
    let (head, vec_end, all_scalar) = split(w, 16, 29, row_phase(sh, off));
    let vector = !all_scalar && (col as i64) >= head && (col as i64) < vec_end;
    let k = off + col;
    let (x, y) = if vector {
        mix_ops_vector(inp.e[k], inp.c[k], inp.b[k], inp.f[k], inp.d[k], inp.a[k])
    } else {
        mix_ops_scalar(inp.e[k], inp.c[k], inp.b[k], inp.f[k], inp.d[k], inp.a[k])
    };
    (x, y, vector)
}

/// Planes for `fma_mix`; all share `Shape::stride`. `out` is the plane whose alignment decides the split.
pub struct MixIn<'a> {
    pub a: &'a [f32],
    pub b: &'a [f32],
    pub c: &'a [f32],
    pub d: &'a [f32],
    pub e: &'a [f32],
    pub f: &'a [f32],
}

/// Rows `row0..row0+rows` of out = -(e*c - b) - (f*d + a).
pub fn fma_mix(inp: &MixIn, out: &mut [f32], sh: &Shape, row0: i32, rows: i32) {
    let w = sh.width as i64;
    for row in row0..row0.wrapping_add(rows) {
        let off = row_off(row, sh.stride);
        if w <= 0 {
            continue;
        }
        let (head, vec_end, all_scalar) = split(w, 16, 29, row_phase(sh, off));
        let sc = |i: usize, out: &mut [f32]| {
            let k = off + i;
            out[k] = mix_scalar(inp.e[k], inp.c[k], inp.b[k], inp.f[k], inp.d[k], inp.a[k]);
        };
        if all_scalar {
            for i in 0..w as usize {
                sc(i, out);
            }
            continue;
        }
        for i in 0..head as usize {
            sc(i, out);
        }
        let mut p = head as usize;
        loop {
            for j in 0..16 {
                let k = off + p + j;
                out[k] = mix_vector(inp.e[k], inp.c[k], inp.b[k], inp.f[k], inp.d[k], inp.a[k]);
            }
            p += 16;
            if p as i64 >= vec_end {
                break;
            }
        }
        for i in vec_end as usize..w as usize {
            sc(i, out);
        }
    }
}

/// Planes of `flow_update`. `u`, `v` are the current values; `a`, `b`, `c` the normal-equation planes.
pub struct FlowIn<'a> {
    pub u: &'a [f32],
    pub v: &'a [f32],
    pub a: &'a [f32],
    pub b: &'a [f32],
    pub c: &'a [f32],
}

/// Rows of the update: with det = a*a + b*b, num = c + u*a + v*b and thr = s0*s1*det (s = s0*s1), the step is
/// (a*s, b*s) when num < -thr, (-(a*s), -(b*s)) when num > thr, zero when det < eps, else (-(a*num)/det, -(b*num)/det).
/// `out0 = u + step0`, `out1 = v + step1`.
pub fn flow_update(inp: &FlowIn, out0: &mut [f32], out1: &mut [f32], width: i32, stride: i32, row0: i32, rows: i32, relax_scale: f32, flow_gain: f32, eps: f32) {
    let s = relax_scale.fmul(flow_gain);
    for row in row0..row0.wrapping_add(rows) {
        let off = row_off(row, stride);
        for i in 0..width.max(0) as usize {
            let k = off + i;
            let b = inp.b[k];
            let det0 = b.fmul(b);
            let a = inp.a[k];
            let u = inp.u[k];
            let v = inp.v[k];
            let det = fma231(det0, a, a);
            let partial_num = fma231(inp.c[k], u, a);
            let thr = s.fmul(det);
            let num = fma231(partial_num, v, b);
            let neg_thr = flip_sign(thr);
            let (delta_u, delta_v);
            if neg_thr > num {
                delta_u = a.fmul(s);
                delta_v = b.fmul(s);
            } else if num > thr {
                delta_u = flip_sign(a.fmul(s));
                delta_v = flip_sign(b.fmul(s));
            } else if eps > det {
                delta_u = 0.0;
                delta_v = 0.0;
            } else {
                let p = a.fmul(num);
                let q = b.fmul(num);
                delta_u = flip_sign(p).fdiv(det);
                delta_v = flip_sign(q).fdiv(det);
            }
            out0[k] = u.fadd(delta_u);
            out1[k] = v.fadd(delta_v);
        }
    }
}

/// Result of `diff_sum`: the partial sum handed to the host reduction.
pub struct DiffPlanes<'a> {
    pub p: &'a [f32],
    pub q: &'a [f32],
}

/// For each element: s = d2*d2 + d1*d1 with d1 = p - x and d2 = q - y; then x = p and y = q. Returns the accumulated sum
/// (vector accumulation over 8 lane groups with a fixed reduction tree, scalar head and tail).
pub fn diff_sum(pl: &DiffPlanes, x: &mut [f32], y: &mut [f32], sh: &Shape, row0: i32, rows: i32) -> f32 {
    let w = sh.width as i64;
    let mut acc = 0.0f32;
    let one = |k: usize, x: &mut [f32], y: &mut [f32]| -> f32 {
        let diff_p = pl.p[k].fsub(x[k]);
        let diff_q = pl.q[k].fsub(y[k]);
        let t = diff_p.fmul(diff_p);
        let s = fma213(diff_q, diff_q, t);
        y[k] = pl.q[k];
        x[k] = pl.p[k];
        s
    };
    for row in row0..row0.wrapping_add(rows) {
        let off = row_off(row, sh.stride);
        if w <= 0 {
            continue;
        }
        let (head, vec_end, all_scalar) = split(w, 8, 907, row_phase(sh, off));
        let scalar_all = all_scalar;
        if scalar_all {
            for i in 0..w as usize {
                let s = one(off + i, x, y);
                acc = s.fadd(acc);
            }
            continue;
        }
        for i in 0..head as usize {
            let s = one(off + i, x, y);
            acc = s.fadd(acc);
        }
        let mut lanes = [0.0f32; 8];
        lanes[0] = acc;
        let mut p = head as usize;
        loop {
            for j in 0..8 {
                let s = one(off + p + j, x, y);
                lanes[j] = s.fadd(lanes[j]);
            }
            p += 8;
            if p as i64 >= vec_end {
                break;
            }
        }
        let lane_sums = [lanes[0].fadd(lanes[4]), lanes[1].fadd(lanes[5]), lanes[2].fadd(lanes[6]), lanes[3].fadd(lanes[7])];
        let lane_pairs = [lane_sums[0].fadd(lane_sums[2]), lane_sums[1].fadd(lane_sums[3])];
        acc = lane_pairs[0].fadd(lane_pairs[1]);
        for i in vec_end as usize..w as usize {
            let s = one(off + i, x, y);
            acc = s.fadd(acc);
        }
    }
    acc
}
