//! Callback 0x460fd0, AVX2 and FMA arm, first half: two loop nests over the row range that update two planes.
//!
//! Each nest has an input plane `t`, a vertical-difference plane `u`, an accumulator plane `acc` and an output plane;
//! a scale plane `v` and a scalar `scale` are shared by the two nests. Per element j of a row y:
//!   out[j] = fma((scale * v[j]), (dx + dy), acc[j])
//! where dx is the forward difference of `t` along the row (the last difference is -t[w-2]) and dy is the vertical
//! difference of `u` (u[j] at the first row, -u_above[j] at the last row, u[j] - u_above[j] elsewhere). Column 0 uses
//! the plain sum t[0] + dy. The vector form (16 elements per block, an optional alignment head, scalar tail) and the
//! scalar form round differently at the last row, so the split between them is reproduced from the row's byte phase.
//! With no scale plane the same two nests run with the scalar `scale` as the product factor (listing 0x462327 and 0x462aef): the
//! scalar middle row then uses two fused steps (`abs_scalar`). The whole callback body from 0x461030 is ported.
use fx_common::nanop::{fma213, fnma213, NanOps};
use fx_common::x86::flip_sign;

#[derive(Clone, Copy)]
pub struct NestIn<'a> {
    pub t: &'a [f32],
    pub u: &'a [f32],
    pub acc: &'a [f32],
}

pub struct Ctx<'a> {
    pub width: i32,
    pub height: i32,
    pub stride: i32,
    pub scale: f32,
    pub v: Option<&'a [f32]>,
    pub nests: [NestIn<'a>; 2],
    /// byte address of every plane's first element, modulo 64 (host fact; the planes share it)
    pub phase_bytes: u32,
}

pub struct Assumption {
    pub id: &'static str,
    pub label: &'static str,
    pub basis: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption {
            id: "ALL-ONES-MASK",
            label: "BYTES-AT-LOAD",
            basis: "eight lanes of 0xffffffff, ported as a compare result; every consumer reads only the lane sign bit (vmaskmovps, vblendvps) or ands it with a compare mask",
        },
        Assumption {
            id: "SIGN-MASK-256",
            label: "MEASURED-EMULATOR-READ",
            basis: "eight lanes of 0x80000000 (vxorps flips the sign of a plane element), read at 14 readers in the recorded reads of the solve body",
        },
        Assumption {
            id: "LANE-INDEX-RAMP",
            label: "MEASURED-EMULATOR-READ",
            basis: "eight int lanes 0..7 (vpaddd with a broadcast of the first element index, compared with width - 1), read at 6 readers in the recorded reads of the solve body",
        },
        Assumption {
            id: "SIGN-MASK-LANE0",
            label: "BYTES-AT-LOAD",
            basis: "lane 0 0x80000000, lanes 1 to 3 zero; every consumer is a scalar sign flip",
        },
        Assumption {
            id: "ALIGN-MODEL",
            label: "ASSUMED",
            basis: "the byte address of the planes' first element modulo 64 is an argument; the listing picks the vector or scalar form from it",
        },
    ]
}

pub fn row_off(row: i32, stride: i32) -> i64 {
    row.wrapping_mul(stride) as i64
}

/// Split of the n = width - 1 columns of a row: (head, vec_end, all_scalar). Mirrors the listing's tests on n (16 and 1200)
/// and on the row's byte phase modulo 64.
pub fn split_cols(n: i64, row_phase64: u32) -> (i64, i64, bool) {
    if n < 16 {
        return (0, 0, true);
    }
    if n < 1200 {
        return (0, n & !15, false);
    }
    let ph = row_phase64 as i64;
    let mut head = 0i64;
    if ph != 0 {
        if ph & 3 != 0 {
            return (0, 0, true);
        }
        head = (64 - ph) >> 2;
    }
    if n < head + 16 {
        return (0, 0, true);
    }
    (head, n - ((n - head) & 15), false)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RowKind {
    First,
    Last,
    Mid,
}

#[inline(always)]
fn diff_x_at(t: &[f32], row_start: usize, x: i64, n: i64) -> f32 {
    let cur = t[row_start + x as usize];
    if x == n - 1 {
        flip_sign(cur)
    } else {
        t[row_start + x as usize + 1].fsub(cur)
    }
}

/// One element of the scalar form (column j = x + 1).
#[inline(always)]
fn elem_scalar(kind: RowKind, diff_x: f32, u_cur: f32, u_abv: f32, pv: f32, a: f32) -> f32 {
    match kind {
        RowKind::First => fma213(pv, diff_x.fadd(u_cur), a),
        RowKind::Last => fnma213(pv, u_abv.fsub(diff_x), a),
        RowKind::Mid => fma213(pv, diff_x.fadd(u_cur.fsub(u_abv)), a),
    }
}

/// One element of the vector form.
#[inline(always)]
fn elem_vector(kind: RowKind, diff_x: f32, u_cur: f32, u_abv: f32, pv: f32, a: f32) -> f32 {
    match kind {
        RowKind::First => fma213(pv, diff_x.fadd(u_cur), a),
        RowKind::Last => fma213(pv, diff_x.fadd(flip_sign(u_abv)), a),
        RowKind::Mid => fma213(pv, diff_x.fadd(u_cur.fsub(u_abv)), a),
    }
}

fn nest(ctx: &Ctx, nin: &NestIn, v: &[f32], out: &mut [f32], start: i32, count: i32) {
    let w = ctx.width as i64;
    let st = ctx.stride as i64;
    for k in 0..(count as u32) {
        let y = start.wrapping_add(k as i32);
        if w <= 0 {
            continue;
        }
        let kind = if y == 0 {
            RowKind::First
        } else if y == ctx.height.wrapping_sub(1) {
            RowKind::Last
        } else {
            RowKind::Mid
        };
        let row_base = row_off(y, ctx.stride);
        let ab = |j: i64| -> usize { (row_base - st + j) as usize };
        let cu = |j: i64| -> usize { (row_base + j) as usize };
        // column 0
        {
            let t = nin.t[cu(0)];
            let d = match kind {
                RowKind::First => nin.u[cu(0)],
                RowKind::Last => flip_sign(nin.u[ab(0)]),
                RowKind::Mid => nin.u[cu(0)].fsub(nin.u[ab(0)]),
            };
            let s = t.fadd(d);
            out[cu(0)] = fma213(ctx.scale.fmul(v[cu(0)]), s, nin.acc[cu(0)]);
        }
        if w < 2 {
            continue;
        }
        let n = w - 1;
        let phase = ((ctx.phase_bytes as i64 + row_base * 4) & 63) as u32;
        let (head, vec_end, all_scalar) = split_cols(n, phase);
        let elem = |x: i64, vector: bool, out: &mut [f32]| {
            let j = x + 1;
            let diff_x = diff_x_at(nin.t, cu(0), x, n);
            let uc = nin.u[cu(j)];
            let ua = if kind == RowKind::First { 0.0 } else { nin.u[ab(j)] };
            let pv = ctx.scale.fmul(v[cu(j)]);
            let a = nin.acc[cu(j)];
            out[cu(j)] = if vector { elem_vector(kind, diff_x, uc, ua, pv, a) } else { elem_scalar(kind, diff_x, uc, ua, pv, a) };
        };
        if all_scalar {
            for x in 0..n {
                elem(x, false, out);
            }
            continue;
        }
        for x in 0..head {
            elem(x, false, out);
        }
        let mut x = head;
        loop {
            for l in 0..16 {
                elem(x + l, true, out);
            }
            x += 16;
            if x >= vec_end {
                break;
            }
        }
        for x in vec_end..n {
            elem(x, false, out);
        }
    }
}


/// Scale-absent scalar form (listing 0x462b47 and following): the product factor is `scale` itself, dest operand first.
#[inline(always)]
fn abs_scalar(kind: RowKind, diff_x: f32, u_cur: f32, u_abv: f32, scale: f32, a: f32) -> f32 {
    match kind {
        RowKind::First => fma213(diff_x.fadd(u_cur), scale, a),
        RowKind::Last => fnma213(u_abv.fsub(diff_x), scale, a),
        RowKind::Mid => {
            let t = fma213(diff_x, scale, a);
            fma213(u_cur.fsub(u_abv), scale, t)
        }
    }
}

/// Scale-absent vector form.
#[inline(always)]
fn abs_vector(kind: RowKind, diff_x: f32, u_cur: f32, u_abv: f32, scale: f32, a: f32) -> f32 {
    match kind {
        RowKind::First => fma213(diff_x.fadd(u_cur), scale, a),
        RowKind::Last => fma213(diff_x.fadd(flip_sign(u_abv)), scale, a),
        RowKind::Mid => fma213(diff_x.fadd(u_cur.fsub(u_abv)), scale, a),
    }
}

fn nest_abs(ctx: &Ctx, nin: &NestIn, out: &mut [f32], start: i32, count: i32) {
    let w = ctx.width as i64;
    let st = ctx.stride as i64;
    for k in 0..(count as u32) {
        let y = start.wrapping_add(k as i32);
        if w <= 0 {
            continue;
        }
        let kind = if y == 0 {
            RowKind::First
        } else if y == ctx.height.wrapping_sub(1) {
            RowKind::Last
        } else {
            RowKind::Mid
        };
        let row_base = row_off(y, ctx.stride);
        let ab = |j: i64| -> usize { (row_base - st + j) as usize };
        let cu = |j: i64| -> usize { (row_base + j) as usize };
        // column 0
        {
            let t = nin.t[cu(0)];
            let d = match kind {
                RowKind::First => nin.u[cu(0)],
                RowKind::Last => flip_sign(nin.u[ab(0)]),
                RowKind::Mid => nin.u[cu(0)].fsub(nin.u[ab(0)]),
            };
            let s = t.fadd(d);
            out[cu(0)] = fma213(s, ctx.scale, nin.acc[cu(0)]);
        }
        if w < 2 {
            continue;
        }
        let n = w - 1;
        let phase = ((ctx.phase_bytes as i64 + row_base * 4) & 63) as u32;
        let (head, vec_end, all_scalar) = split_cols(n, phase);
        let elem = |x: i64, vector: bool, out: &mut [f32]| {
            let j = x + 1;
            let diff_x = diff_x_at(nin.t, cu(0), x, n);
            let uc = nin.u[cu(j)];
            let ua = if kind == RowKind::First { 0.0 } else { nin.u[ab(j)] };
            let a = nin.acc[cu(j)];
            out[cu(j)] = if vector { abs_vector(kind, diff_x, uc, ua, ctx.scale, a) } else { abs_scalar(kind, diff_x, uc, ua, ctx.scale, a) };
        };
        if all_scalar {
            for x in 0..n {
                elem(x, false, out);
            }
            continue;
        }
        for x in 0..head {
            elem(x, false, out);
        }
        let mut x = head;
        loop {
            for l in 0..16 {
                elem(x + l, true, out);
            }
            x += 16;
            if x >= vec_end {
                break;
            }
        }
        for x in vec_end..n {
            elem(x, false, out);
        }
    }
}

/// Runs both nests over rows `start..start+count` (the whole callback body from 0x461030). `out` holds the two output planes
/// in nest order (listing offsets 0x28 and 0x40 of the task record). An absent scale plane selects the scale-absent arms.
pub fn relax_callback(ctx: &Ctx, out: [&mut [f32]; 2], start: i32, count: i32) {
    if !(start < start.wrapping_add(count)) {
        return;
    }
    let [out_first, out_second] = out;
    match ctx.v {
        Some(v) => {
            nest(ctx, &ctx.nests[0], v, out_first, start, count);
            nest(ctx, &ctx.nests[1], v, out_second, start, count);
        }
        None => {
            nest_abs(ctx, &ctx.nests[0], out_first, start, count);
            nest_abs(ctx, &ctx.nests[1], out_second, start, count);
        }
    }
}

#[cfg(test)]
mod first_row_fused_step {
    use super::*;

    /// Operands of one first-row element whose fused and unfused results differ in their bits. Found by a fixed integer sequence, so the
    /// choice is the same on every run; the caller asserts the difference, so the test cannot pass without it.
    fn operands() -> (f32, f32, f32, f32, f32) {
        let mut s: u32 = 0x1357_9bdf;
        let mut next = || {
            s = s.wrapping_mul(1664525).wrapping_add(1013904223);
            1.0 + (s >> 9) as f32 / 8388608.0
        };
        for _ in 0..10_000 {
            let (diff_x, u_cur, pv, a) = (next(), next(), next(), -next());
            let sum = diff_x.fadd(u_cur);
            let fused = fma213(pv, sum, a);
            let unfused = pv.fmul(sum).fadd(a);
            if fused.to_bits() != unfused.to_bits() {
                return (diff_x, u_cur, 0.0, pv, a);
            }
        }
        panic!("no operands with distinct fused and unfused words");
    }

    #[test]
    fn the_first_row_step_is_fused_in_both_forms() {
        let (diff_x, u_cur, u_abv, pv, a) = operands();
        let sum = diff_x.fadd(u_cur);
        let fused = fma213(pv, sum, a).to_bits();
        let unfused = pv.fmul(sum).fadd(a).to_bits();
        assert_ne!(fused, unfused, "the operands must separate the fused word from the unfused word");
        assert_eq!(elem_scalar(RowKind::First, diff_x, u_cur, u_abv, pv, a).to_bits(), fused);
        assert_eq!(elem_vector(RowKind::First, diff_x, u_cur, u_abv, pv, a).to_bits(), fused);
    }
}
