//! Pyramid body of the optical flow op, second stage group: body 0x47ab6c up to the launch of the next stage callback at
//! 0x47bd65. It normalises plane A and then plane B with the affine launch (each scanned for its own range), centres both planes on their
//! common mean through the row sum and constant add launches, fills the two 2048 entry kernel tables and prepares the
//! frame of the next launch. Fused steps sit where the original fuses and go through `fx_common::nanop`.
use crate::callbacks::{add_const, affine_rows, sum_rows};
use crate::flowbuild::{CutOut, C_RANGE_BITS};
use fx_common::nanop::{fma213, fma231, fms213, fms231, NanOps};

/// Scalar cell 0x15e8510 (numerator of the affine scale). MEASURED-EMULATOR-READ, read from the original at load.
pub const C_TWO: f32 = 2.0;
/// Scalar cell 0x15e8514 (half). MEASURED-EMULATOR-READ.
pub const C_HALF: f32 = 0.5;
/// Lane cell 0x15f0d80: scale of the table index, 2^-10. MEASURED-EMULATOR-READ.
pub const C_TABLE_STEP_BITS: u32 = 0x3a80_0000;
/// Number of entries of each kernel table (256 iterations of eight lanes).
pub const TABLE_LEN: usize = 0x800;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LaunchKind {
    /// callback 0x46fa00: in place affine map of the plane
    Affine,
    /// callback 0x46f230: row sum, reduced by 0x670f30
    Sum,
    /// launcher 0x676140 with callback 0x675d00: constant add
    AddConst,
}

/// One launch record: what the body hands to the parallel launcher (plane 0 = state+0x20, 1 = state+0x28).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Launch {
    pub kind: LaunchKind,
    pub plane: u8,
    pub w: i32,
    pub h: i32,
    pub pitch: i32,
    /// affine: scale bits; add: constant bits; sum: 0
    pub a: u32,
    /// affine: offset bits; sum and add: 0
    pub b: u32,
    /// add: element count; others 0
    pub count: i32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StageCut {
    /// 0x47bd65: the launch of callback 0x470070
    NextLaunch,
    /// 0x47b5cd: no pyramid level beyond the first, the body returns 0
    Return,
}

pub struct StageOut {
    pub cut: StageCut,
    pub launches: Vec<Launch>,
    /// table at 0x1e14260 and table at 0x1e16460
    pub table1: Vec<f32>,
    pub table2: Vec<f32>,
    /// frame fields of the next launch: level 0 w, h, level 1 w, h, plane pointer of level 0 (+0x20), of level 1 (+0x20), state+0x1c88
    pub frame: [u64; 7],
    /// plane B scan (max and min)
    pub max: f32,
    pub min: f32,
}

fn to_f32(b: &[u8]) -> Vec<f32> {
    b.chunks_exact(4).map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}
fn from_f32(v: &[f32], b: &mut [u8]) {
    for (c, x) in b.chunks_exact_mut(4).zip(v) {
        c.copy_from_slice(&x.to_le_bytes());
    }
}

/// Max and min scan of a plane (code at 0x47abdc): start at element 0, per pair a compare chain.
pub fn scan_range(pl: &[f32], w: i32, h: i32, pitch: i32) -> (f32, f32) {
    let fl = |i: i64| pl[i as usize];
    let mut mx = fl(0);
    let mut mn = mx;
    if h > 0 && w > 0 {
        let step = |x: f32, mn: &mut f32, mx: &mut f32| {
            if *mn > x {
                *mn = x;
            } else if x > *mx {
                *mx = x;
            }
        };
        let (mut row_base, mut r13o): (i32, i64) = (0, 0);
        for _ in 0..h as u32 {
            let half = (w >> 1) as u32;
            for k in 0..half {
                let idx = row_base.wrapping_add(2 * k as i32) as i64;
                step(fl(idx), &mut mn, &mut mx);
                step(fl(idx + 1), &mut mn, &mut mx);
            }
            if (w as u32) > 2 * half {
                step(fl(r13o + 2 * half as i64), &mut mn, &mut mx);
            }
            r13o += pitch as i64;
            row_base = row_base.wrapping_add(pitch);
        }
    }
    (mx, mn)
}

/// Reduction of the 128 per thread slots (0x670f30, AVX2 arm): eight accumulators of eight lanes over two blocks of 64
/// slots, folded pairwise, then the lanes. Host state: one tile, so only slot 0 holds the partial sum.
pub fn slot_reduce(slots: &[f32; 128]) -> f32 {
    let mut acc = [[0.0f32; 8]; 8];
    for blk in 0..2 {
        for k in 0..8 {
            for l in 0..8 {
                acc[k][l] = acc[k][l].fadd(slots[blk * 64 + k * 8 + l]);
            }
        }
    }
    let add_lanes = |a: [f32; 8], b: [f32; 8]| {
        let mut o = [0.0f32; 8];
        for l in 0..8 {
            o[l] = a[l].fadd(b[l]);
        }
        o
    };
    let pair_a = add_lanes(acc[0], acc[1]);
    let pair_b = add_lanes(acc[2], acc[3]);
    let pair_c = add_lanes(acc[4], acc[5]);
    let pair_d = add_lanes(acc[6], acc[7]);
    let quad_a = add_lanes(pair_a, pair_b);
    let quad_b = add_lanes(pair_c, pair_d);
    let lanes_total = add_lanes(quad_a, quad_b);
    let mut four_lanes = [0.0f32; 4];
    for l in 0..4 {
        four_lanes[l] = lanes_total[l].fadd(lanes_total[l + 4]);
    }
    let two_lanes = [four_lanes[0].fadd(four_lanes[2]), four_lanes[1].fadd(four_lanes[3])];
    two_lanes[0].fadd(two_lanes[1])
}

/// One kernel table entry pair for index `i`: (first table, second table). Cubic kernel pieces of |i * 2^-10|.
fn table_entry(i: usize) -> (f32, f32) {
    let step = f32::from_bits(C_TABLE_STEP_BITS);
    let x = f32::from_bits((i as i32 as f32).fmul(step).to_bits() & 0x7fff_ffff);
    let at_least_two = x >= 2.0;
    let lt1 = x < 1.0;
    let a = !at_least_two && lt1;
    let b = !at_least_two && !lt1;
    // first table
    let t = fms213(3.0, x, 6.0); // 3x - 6
    let p = x.fmul(t);
    let poly_near = fma231(4.0, p, x); // x (3x - 6) x + 4
    let u = fms231(12.0, 6.0f32.fsub(x), x); // (6 - x) x - 12
    let poly_far = fma231(8.0, u, x); // ((6 - x) x - 12) x + 8
    let v = fms213(1.5, x, 2.5); // 1.5x - 2.5
    let w = x.fmul(v);
    let first = if b { poly_far } else if a { poly_near } else { 0.0 };
    // second table
    let poly_near_second = fma231(1.0, w, x); // x (1.5x - 2.5) x + 1
    let r = fma231(2.5, -0.5, x); // -0.5x + 2.5
    let s = fms231(4.0, r, x); // (-0.5x + 2.5) x - 4
    let poly_far_second = fma213(x, s, 2.0); // x s + 2
    let second = if b { poly_far_second } else if a { poly_near_second } else { 0.0 };
    (first, second)
}

/// Body 0x47ab6c..0x47bd65. `table`: the levels built by `build_to_cut` (read only here), `dst_a` and `dst_b`: the host
/// buffers at state+0x20 and state+0x28, `first`: the cut result of the prologue (Launch or Degenerate), `align_a` and
/// `align_b`: the byte addresses of the two buffers modulo 32 (label ALIGN-PHASE-D).
pub fn stage_to_cut(table: &optical_flow_pyramid::LevelTable, dst_a: &mut [u8], dst_b: &mut [u8], first: &CutOut, align_a: usize, align_b: usize) -> StageOut {
    let level0 = table.levels.first().copied().unwrap_or_default();
    let level1 = table.levels.get(1).copied().unwrap_or_default();
    let (w, h, pitch) = (level0.width, level0.height, level0.stride);
    let mut pa = to_f32(dst_a);
    let mut pb = to_f32(dst_b);
    let mut launches = Vec::new();
    let c_range = f32::from_bits(C_RANGE_BITS);
    let rec = |kind, plane, a, b, count| Launch { kind, plane, w, h, pitch, a, b, count };
    let mut mx = first.max;
    let mut mn = first.min;
    let range = mx.fsub(mn);
    if !(c_range > range) {
        let scale = C_TWO.fdiv(range);
        launches.push(rec(LaunchKind::Affine, 0, scale.to_bits(), mn.to_bits(), 0));
        affine_rows(&mut pa, w, pitch, 0, h, scale, mn, align_a);
    }
    // second plane: its own scan and affine launch
    let (max_b, min_b) = scan_range(&pb, w, h, pitch);
    mx = max_b;
    mn = min_b;
    let range = mx.fsub(mn);
    if !(c_range > range) {
        let scale = C_TWO.fdiv(range);
        launches.push(rec(LaunchKind::Affine, 1, scale.to_bits(), mn.to_bits(), 0));
        affine_rows(&mut pb, w, pitch, 0, h, scale, mn, align_b);
    }
    // means of both planes through the row sum launch and the slot reduction
    let area = (w.wrapping_mul(h)) as f32;
    let host_sum = |p: &[f32], phase_bytes: usize| {
        let mut slots = [0.0f32; 128];
        slots[0] = sum_rows(p, w, pitch, 0, h, phase_bytes);
        slot_reduce(&slots)
    };
    launches.push(rec(LaunchKind::Sum, 0, 0, 0, 0));
    let mean_a = host_sum(&pa, align_a).fdiv(area);
    launches.push(rec(LaunchKind::Sum, 1, 0, 0, 0));
    let mean_b = host_sum(&pb, align_b).fdiv(area);
    let t = mean_a.fadd(mean_b);
    let m = fms231(mean_a, t, C_HALF);
    let count = pitch.wrapping_mul(h);
    launches.push(rec(LaunchKind::AddConst, 0, m.to_bits(), 0, count));
    add_const(&mut pa, count as i64, m);
    let nm = f32::from_bits(m.to_bits() ^ 0x8000_0000);
    launches.push(rec(LaunchKind::AddConst, 1, nm.to_bits(), 0, count));
    add_const(&mut pb, count as i64, nm);
    from_f32(&pa, dst_a);
    from_f32(&pb, dst_b);
    // kernel tables
    let mut table1 = vec![0.0f32; TABLE_LEN];
    let mut table2 = vec![0.0f32; TABLE_LEN];
    for i in 0..TABLE_LEN {
        let (a, b) = table_entry(i);
        table1[i] = a;
        table2[i] = b;
    }
    let cnt = table.first();
    let cut = if cnt > 0 { StageCut::NextLaunch } else { StageCut::Return };
    let frame = [level0.width as u32 as u64, level0.height as u32 as u64, level1.width as u32 as u64, level1.height as u32 as u64, level0.image_a, level1.image_a, table.image_table];
    StageOut { cut, launches, table1, table2, frame, max: mx, min: mn }
}

/// Labels of this module.
pub fn stage_assumptions() -> Vec<&'static str> {
    vec![
        "HOST-SERVICES-D: the parallel launcher runs each callback once over the whole range (one tile, thread slot 0); the slot store and the slot clear are host state; the slot reduction is ported as slot_reduce over the 128 slots",
        "CONST-CELLS-D: the constant cells, 0.5 included, are the bytes read from the original at load, splat by code",
        "ALIGN-PHASE-D: the byte phase of the two host buffers modulo 32 is an argument",
        "LEVELS-ARITHMETIC: every pyramid entry has both sizes at least two, so the level checks before the next launch always pass and the per level mean fallback code is not reached",
    ]
}
