//! Rust-only oracle of the small-side branch of the stretch (a level with a side below 2): mean of the source region, fill of the
//! destination region. Not part of the shipped crate; the body refuses such a level before any launch.
#![allow(dead_code)]
use fx_common::nanop::NanOps;
use optical_flow_solve::stretch::*;

/// Accumulators of one sum loop: x0 is the scalar accumulator (xmm0 low lane), v[k] the eight four-lane accumulators.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Acc {
    pub scalar_sum: f32,
    pub v: [[f32; 4]; 8],
}

impl Acc {
    pub fn new() -> Acc {
        Acc { scalar_sum: 0.0, v: [[0.0; 4]; 8] }
    }
}

impl Default for Acc {
    fn default() -> Self {
        Acc::new()
    }
}

/// Row start and shape of the sum loop: elements `a[first .. first + w]`, `phase` = alignment of a[first] in bytes modulo 16.
fn sum_row(acc: &mut Acc, a: &[f32], first: usize, w: i32, phase: u32) {
    let mut scalar_only = w < 0x20;
    let mut pe: i32 = 0;
    if !scalar_only {
        if phase != 0 {
            if phase & 3 != 0 {
                scalar_only = true;
            } else {
                pe = ((16 - phase) >> 2) as i32;
            }
        }
        if !scalar_only && w < pe + 0x20 {
            scalar_only = true;
        }
    }
    if scalar_only {
        for j in 0..w as usize {
            acc.scalar_sum = acc.scalar_sum.fadd(a[first + j]);
        }
        return;
    }
    let end = w - ((w - pe) & 0x1f);
    for j in 0..pe as usize {
        acc.scalar_sum = acc.scalar_sum.fadd(a[first + j]);
    }
    let mut k = pe as usize;
    loop {
        for r in 0..8 {
            for l in 0..4 {
                acc.v[r][l] = acc.v[r][l].fadd(a[first + k + 4 * r + l]);
            }
        }
        k += 32;
        if k >= end as usize {
            break;
        }
    }
    if end as u32 >= w as u32 {
        return;
    }
    let rem = (w - end) as usize;
    let mut done = 0usize;
    if rem >= 4 {
        let quad_end = rem & !3;
        while done < quad_end {
            for l in 0..4 {
                acc.v[0][l] = acc.v[0][l].fadd(a[first + end as usize + done + l]);
            }
            done += 4;
        }
    }
    for j in done..rem {
        acc.scalar_sum = acc.scalar_sum.fadd(a[first + end as usize + j]);
    }
}

/// Sum loop over `h` rows of `w` elements. `off0` is the element index of the first row, `stride` the row step, `base_phase`
/// the byte address of element 0 of `a` modulo 16.
pub fn sum_plane(a: &[f32], base_phase: u32, start_index: i64, stride: i64, w: i32, h: u32) -> Acc {
    let mut acc = Acc::new();
    let mut off = start_index;
    for _ in 0..h {
        let phase = ((base_phase as i64 + 4 * off) & 15) as u32;
        sum_row(&mut acc, a, off as usize, w, phase);
        off += stride;
    }
    acc
}

/// Fold of the accumulators and division by the count (listing 0x484772..0x4847b5 and its clones). Returns (total, count, mean).
pub fn finish(acc: &Acc, count: i32) -> (f32, f32, f32) {
    let add_lanes4 = |p: [f32; 4], q: [f32; 4]| [p[0].fadd(q[0]), p[1].fadd(q[1]), p[2].fadd(q[2]), p[3].fadd(q[3])];
    let lanes_01 = add_lanes4(acc.v[0], acc.v[1]);
    let lanes_23 = add_lanes4(acc.v[2], acc.v[3]);
    let lanes_45 = add_lanes4(acc.v[4], acc.v[5]);
    let lanes_67 = add_lanes4(acc.v[6], acc.v[7]);
    let lanes_0123 = add_lanes4(lanes_01, lanes_23);
    let lanes_4567 = add_lanes4(lanes_45, lanes_67);
    let lanes_all = add_lanes4(lanes_0123, lanes_4567);
    let lanes = [lanes_all[2], lanes_all[3], lanes_all[2], lanes_all[3]];
    let lane_sum = add_lanes4(lanes_all, lanes);
    let term = lane_sum[0].fadd(lane_sum[1]);
    let total = term.fadd(acc.scalar_sum);
    let cnt = count as f32;
    (total, cnt, total.fdiv(cnt))
}

/// Fill loop: `h` rows of `w` elements starting at element `off0`, row step `stride`.
pub fn fill_plane(p: &mut [f32], start_index: i64, stride: i64, w: i32, h: u32, mean: f32) {
    let mut off = start_index;
    for _ in 0..h {
        for j in 0..w.max(0) as usize {
            p[off as usize + j] = mean;
        }
        off += stride;
    }
}

/// Mean of the source region and fill of the destination region for plane word `j`.
pub fn mean_step<P: Planes>(l: &Level, lp: &Level, j: PairPlane, mem: &mut P) {
    let (sw, sh) = (lp.width, lp.height);
    let (dst_w, dst_h) = (l.width, l.height);
    let acc = if sw > 0 && sh > 0 {
        let (src, phase) = mem.src(j);
        sum_plane(src, phase, 0, (lp.stride as u32) as i64, sw, sh as u32)
    } else {
        Acc::new()
    };
    let (_, _, mean) = finish(&acc, sw.wrapping_mul(sh));
    if dst_w > 0 && dst_h > 0 {
        fill_plane(mem.dst(j), 0, (l.stride as u32) as i64, dst_w, dst_h as u32, mean);
    }
}

/// Host wrapper: the tile pair of a step with a side below 2 runs the mean and fill instead, as the original's stretch does.
pub struct SmallSide<H>(pub H);

impl<H: Host> Host for SmallSide<H> {
    fn helper(&mut self, c: HelperCall) {
        self.0.helper(c)
    }
    fn tile_pair<P: Planes>(&mut self, mem: &mut P, t: &Tile) {
        let big = t.lp.width >= 2 && t.lp.height >= 2 && t.l.width >= 2 && t.l.height >= 2;
        if big {
            self.0.tile_pair(mem, t)
        } else {
            mean_step(t.l, t.lp, t.j, mem)
        }
    }
    fn scale<P: Planes>(&mut self, mem: &mut P, j: PairPlane, l: &Level, ratio: f32) {
        self.0.scale(mem, j, l, ratio)
    }
    fn border<P: Planes>(&mut self, mem: &mut P, j: PairPlane, lp: &Level) {
        self.0.border(mem, j, lp)
    }
}
