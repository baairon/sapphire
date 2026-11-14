//! Pyramid body of the optical flow op: the flow stencil callback 0x474a20 (AVX2 and FMA arm 0x474a80..0x475280) and the
//! body prologue 0x47a400..0x47ab6c (pyramid table, host image slots, plane copies, range scan) up to the first stage
//! launch of 0x46fa00. Fused steps sit where the original fuses and go through `fx_common::nanop`.
use fx_common::nanop::{fma213, fnma213, NanOps};
use optical_flow_pyramid::{LevelPlane, LevelRecord, LevelTable};

/// Scalar cell 0x15e8514 (rounding bias of the pyramid size map). MEASURED-EMULATOR-READ, read from the original at load.
pub const C_ROUND: f32 = 0.5;
/// Scalar cell 0x15efc14 (minimum value range for the first stage). MEASURED-EMULATOR-READ, bits 0x3727c5ac.
pub const C_RANGE_BITS: u32 = 0x3727_c5ac;

#[inline(always)]
fn neg(x: f32) -> f32 {
    f32::from_bits(x.to_bits() ^ 0x8000_0000)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Row {
    First,
    Mid,
    Last,
}

/// Head and vector end of the loop elements (0 based, `n` = width - 1 elements): elements below `head` and from `vend`
/// run scalar, the elements between run in blocks of 16. (0, 0) = all scalar. `phase` = byte offset of the row's element 0 of
/// plane B modulo 64.
fn split(n: i32, phase: usize) -> (usize, usize) {
    if n < 16 {
        return (0, 0);
    }
    if n < 0x4b0 {
        return (0, (n & !0xf) as usize);
    }
    let m = phase & 63;
    let head = if m == 0 {
        0
    } else if m & 3 != 0 {
        return (0, 0);
    } else {
        (64 - m) / 4
    };
    if n < head as i32 + 16 {
        return (0, 0);
    }
    (head, (n - ((n - head as i32) & 0xf)) as usize)
}

/// Callback 0x474a20 (AVX2 arm). `out`, `b`, `c`, `d` are the planes of ctx+0x28, +0x30, +0x38 and +0x58 (separate buffers, element
/// index = row * pitch + x). Rows `row0 .. row0 + count` of `rows` (the tile the host row splitter hands out). `align_b` = byte
/// address of `b[0]` modulo 64 (label ALIGN-PHASE). `scale` = ctx+0x68.
#[allow(clippy::too_many_arguments)]
pub fn flow_stencil_rows(out: &mut [f32], b: &[f32], c: &[f32], d: &[f32], width: i32, rows: i32, pitch: i32, scale: f32, row0: i32, count: i32, align_b: usize) {
    if row0 >= row0.wrapping_add(count) {
        return;
    }
    let mut row = row0;
    let mut k: u32 = 0;
    loop {
        stencil_row(out, b, c, d, width, rows, pitch, scale, row, align_b);
        k = k.wrapping_add(1);
        row = row.wrapping_add(1);
        if k >= count as u32 {
            break;
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn stencil_row(out: &mut [f32], b: &[f32], c: &[f32], d: &[f32], width: i32, rows: i32, pitch: i32, scale: f32, row: i32, align_b: usize) {
    if width <= 0 {
        return;
    }
    let kind = if row == 0 {
        Row::First
    } else if row == rows.wrapping_sub(1) {
        Row::Last
    } else {
        Row::Mid
    };
    let ro = pitch.wrapping_mul(row) as i64;
    let at = |i: i64| (ro + i) as usize;
    let atp = |i: i64| (ro - pitch as i64 + i) as usize; // previous row of plane C
    // element 0
    let x = match kind {
        Row::First => c[at(0)],
        Row::Last => neg(c[atp(0)]),
        Row::Mid => c[at(0)].fsub(c[atp(0)]),
    };
    out[at(0)] = fma213(b[at(0)].fadd(x), scale, d[at(0)]);
    if width < 2 {
        return;
    }
    let n = width - 1;
    let phase = (align_b as i64 + 4 * ro).rem_euclid(64) as usize;
    let (head, vend) = split(n, phase);
    for i in 0..n as i64 {
        let vec = (i as usize) >= head && (i as usize) < vend;
        let g = if i == width as i64 - 2 { neg(b[at(i)]) } else { b[at(i + 1)].fsub(b[at(i)]) };
        let o = i + 1;
        out[at(o)] = match kind {
            Row::First => fma213(g.fadd(c[at(o)]), scale, d[at(o)]),
            Row::Last => {
                if vec {
                    fma213(g.fadd(neg(c[atp(o)])), scale, d[at(o)])
                } else {
                    fnma213(c[atp(o)].fsub(g), scale, d[at(o)])
                }
            }
            Row::Mid => {
                if vec {
                    fma213(g.fadd(c[at(o)].fsub(c[atp(o)])), scale, d[at(o)])
                } else {
                    let t = fma213(g, scale, d[at(o)]);
                    fma213(c[at(o)].fsub(c[atp(o)]), scale, t)
                }
            }
        };
    }
}

/// Image rectangle record returned by the host service 0x5b9b10 (24 bytes: w, h, pitch, reserved, element size).
#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Rect {
    pub w: i32,
    pub h: i32,
    pub pitch: i32,
    pub reserved: i32,
    pub q: u64,
}

/// One call of the host image slot service (vtable entry 0x10): the arguments the body passes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ImgCall {
    pub level: i32,
    /// the per-level plane the call requests
    pub plane: LevelPlane,
    pub h: i32,
    pub pitch: i32,
    pub flag01: u32,
}

/// Inputs of the body prologue (arguments and the fields it reads).
pub struct BodyIn<'a> {
    pub rect: Rect,
    /// params+0x1c, params+0x2c
    pub levels: i32,
    pub scale: f32,
    /// stack argument [rbx+0x18]
    pub flag: i32,
    /// params+0x20 (non-zero leaves the route for 0x47cbe1)
    pub smoothing_word: i32,
    /// the plane handles of the descriptor at arg5 and the stack argument [rbx+0x10], as the named fields of the level 0 record
    pub planes: LevelRecord,
    /// the two source planes (arg2, arg3) as bytes
    pub src_a: &'a [u8],
    pub src_b: &'a [u8],
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Cut {
    /// 0x47bd26: fewer than two usable levels
    TooFewLevels,
    /// stack flag set: the body continues in the image handle helpers (0x47aaba)
    Helper,
    /// range at least the cell 0x15efc14 or unordered: the first stage launch 0x47ab6c
    Launch,
    /// range below the cell: 0x47abdc
    Degenerate,
    /// params+0x20 non-zero: the body continues at 0x47cbe1 (outside this function's range)
    Alt20,
    /// OPTICAL-FLOW-LEVEL-TABLE-FULL-REFUSED: the next record index would be LEVEL_TABLE_CAPACITY or more
    TableFull,
    /// OPTICAL-FLOW-NULL-IMAGE-REFUSED: an image answer was 0
    NullImage,
}

/// Records the level table holds; the next record index must stay below it.
pub const LEVEL_TABLE_CAPACITY: i64 = 38;

/// The order in which the build requests the per-level planes from the host image service.
pub const SERVED_PLANES: [LevelPlane; 8] = [
    LevelPlane::ImageA,
    LevelPlane::ImageB,
    LevelPlane::FlowU,
    LevelPlane::FlowV,
    LevelPlane::Nest1T,
    LevelPlane::Nest2T,
    LevelPlane::Nest1U,
    LevelPlane::Nest2U,
];

pub struct CutOut {
    pub cut: Cut,
    /// max and min of plane A at the cut (xmm0 and xmm1)
    pub max: f32,
    pub min: f32,
    pub calls: Vec<ImgCall>,
}

fn cvtt(x: f32) -> i32 {
    if x.is_nan() || x >= 2147483648.0 || x < -2147483648.0 {
        i32::MIN
    } else {
        x as i32
    }
}

/// 0x6acee0: `(((a * b + c - 1) / c) * c) / b` in unsigned 64 bit arithmetic.
fn align_units(a: i64, b: u64, c: u64) -> u64 {
    let x = (a as u64).wrapping_mul(b);
    let q = c.wrapping_add(x).wrapping_sub(1) / c;
    q.wrapping_mul(c) / b
}

/// Body 0x47a400..0x47ab6c. Returns the cut and the level table the body builds; `dst_a` and `dst_b` = the host buffers of level 0's
/// image planes; `host(call)` = the host image slot service, returning the data pointer stored at the slot (0 = no image).
pub fn build_to_cut(inp: &BodyIn, dst_a: &mut [u8], dst_b: &mut [u8], host: &mut dyn FnMut(&ImgCall) -> u64) -> (CutOut, LevelTable) {
    let mut out = CutOut { cut: Cut::TooFewLevels, max: 0.0, min: 0.0, calls: Vec::new() };
    let mut table = LevelTable::default();
    // The build stores a level only when both of its sides are at least max(2, levels parameter), so no stored level has a side below
    // 2 and the body refuses such a table by name (OPTICAL-FLOW-SMALL-LEVEL-REFUSED).
    let min_side = if inp.levels >= 2 { inp.levels } else { 2 };
    let mut rect = inp.rect;
    if rect.w < min_side {
        return (out, table);
    }
    let mut cnt: i32 = -1;
    let mut new_pitch: i32;
    loop {
        if rect.h < min_side {
            break;
        }
        let old = rect;
        let neww = cvtt(fma213(old.w as f32, inp.scale, C_ROUND));
        rect.w = neww;
        if cnt as i64 + 1 >= LEVEL_TABLE_CAPACITY {
            out.cut = Cut::TableFull;
            return (out, table);
        }
        table.levels.push(LevelRecord { width: old.w, height: old.h, stride: old.pitch, rect_word3: old.reserved as u32, elem_bytes: old.q, ..Default::default() });
        cnt += 1;
        new_pitch = if inp.flag == 0 { neww } else { align_units(neww as i64, 4, 0x100) as u32 as i32 };
        if neww >= min_side {
            let newh = cvtt(fma213(old.h as f32, inp.scale, C_ROUND));
            rect.pitch = new_pitch;
            rect.h = newh;
            continue;
        }
        break;
    }
    if cnt == -1 {
        return (out, table);
    }
    table.min_side = min_side;
    let p = &inp.planes;
    let level0 = &mut table.levels[0];
    level0.index = 0;
    level0.warped_image = p.warped_image;
    level0.warped_grad_x = p.warped_grad_x;
    level0.warped_grad_y = p.warped_grad_y;
    level0.coeff = p.coeff;
    level0.flow_u_next = p.flow_u_next;
    level0.flow_v_next = p.flow_v_next;
    level0.relax_first = p.relax_first;
    level0.relax_second = p.relax_second;
    level0.median_out = p.median_out;
    level0.grad_x = p.grad_x;
    level0.grad_y = p.grad_y;
    level0.copied_plane = p.copied_plane;
    level0.image_a = p.image_a;
    level0.image_b = p.image_b;
    level0.flow_u = p.flow_u;
    level0.flow_v = p.flow_v;
    level0.nest1_t = p.nest1_t;
    level0.nest1_u = p.nest1_u;
    level0.nest2_t = p.nest2_t;
    level0.nest2_u = p.nest2_u;
    table.image_table = p.median_out;
    let flag01 = if inp.flag != 0 { 1 } else { 0 };
    for j in 1..=cnt as usize {
        let (h, pitch) = (table.levels[j].height, table.levels[j].stride);
        for plane in SERVED_PLANES {
            let call = ImgCall { level: j as i32, plane, h, pitch, flag01 };
            out.calls.push(call);
            let v = host(&call);
            if v == 0 {
                out.cut = Cut::NullImage;
                return (out, table);
            }
            *table.levels[j].plane_mut(plane) = v;
        }
    }
    for j in 1..=cnt as usize {
        let r = &mut table.levels[j];
        r.index = j as u32;
        r.warped_image = p.warped_image;
        r.grad_x = p.grad_x;
        r.grad_y = p.grad_y;
        r.copied_plane = p.copied_plane;
        r.warped_grad_x = p.warped_grad_x;
        r.warped_grad_y = p.warped_grad_y;
        r.coeff = p.coeff;
        r.flow_u_next = p.flow_u_next;
        r.flow_v_next = p.flow_v_next;
        r.relax_first = p.relax_first;
        r.relax_second = p.relax_second;
        r.median_out = p.median_out;
    }
    // host strided copy 0x672c60 on the stage route: a byte copy of pitch * h * q bytes with no conversion (HOST-COPY)
    let level0 = table.levels[0];
    let first_rect = Rect { w: level0.width, h: level0.height, pitch: level0.stride, reserved: level0.rect_word3 as i32, q: level0.elem_bytes };
    let nbytes = ((first_rect.pitch.wrapping_mul(first_rect.h)) as i64).wrapping_mul(first_rect.q as i64);
    if nbytes > 0 {
        let n = nbytes as usize;
        dst_a[..n].copy_from_slice(&inp.src_a[..n]);
        dst_b[..n].copy_from_slice(&inp.src_b[..n]);
    }
    if inp.smoothing_word != 0 {
        out.cut = Cut::Alt20;
        return (out, table);
    }
    if inp.flag != 0 {
        out.cut = Cut::Helper;
        return (out, table);
    }
    // range scan of plane A
    let fl = |i: i64| f32::from_le_bytes(dst_a[4 * i as usize..4 * i as usize + 4].try_into().unwrap());
    let (w, h, pitch) = (first_rect.w, first_rect.h, first_rect.pitch);
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
    out.max = mx;
    out.min = mn;
    let range = mx.fsub(mn);
    let c_range = f32::from_bits(C_RANGE_BITS);
    out.cut = if c_range > range { Cut::Degenerate } else { Cut::Launch };
    (out, table)
}

#[cfg(test)]
mod split_tests {
    use super::split;

    #[test]
    fn bit_5_of_the_plane_phase_moves_the_head_at_1232_elements_and_not_at_72() {
        // width 1,232: n = 1,231 elements after the first
        assert_eq!(split(1231, 0), (0, 1231 - (1231 & 15)));
        assert_eq!(split(1231, 32), (8, 1231 - ((1231 - 8) & 15)));
        assert_ne!(split(1231, 0), split(1231, 32));
        // width 72: below 1,200 the split ignores the phase
        assert_eq!(split(71, 0), split(71, 32));
    }
}
