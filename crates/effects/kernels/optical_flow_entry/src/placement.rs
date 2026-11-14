//! Address phases (byte address modulo 64) of the op's planes in the host the op's output depends on (ALIGN-PHASE-HOST).
//!
//! The low 5 bits of a phase choose the row sum's vector order; bit 5 chooses the relaxation's and the flow-build stencil's split
//! on rows of 1,200 or more elements. The row sum of the pyramid stages takes its vector order only on rows of 64 or more elements whose start is 32-byte aligned, so
//! the op's output depends on where its host places the planes. The phase of the first level-0 image (the plane the op reads
//! through descriptor word 9) is a host value, ALIGN-PHASE-HOST: a host's allocator sets it, the product never models that
//! allocator. Every other op plane is placed from it by the in-op chain, which is arithmetic on sizes: an allocation of n bytes
//! advances a bump cursor by round16(n) + 16, every image plane is a memory handle (a data block of n + 16 bytes and a 16-byte
//! master cell, the data at the cursor), and the op's image class allocates fixed object bytes between two handles. The chain is
//! the 17 level-0 descriptor planes in word order (step round16(n0) + 0x1f0), the 3 output planes ahead of word 0, and the 8 served
//! planes of every further level. The default 0 is the value recorded at 64x32, 72x24 and 128x64.

/// Phase of the first level-0 image in bytes (ALIGN-PHASE-HOST), default.
pub const HOST_PHASE_DEFAULT: u32 = 0;

/// A host phase is a multiple of 4 from 0 to 60 (a float-aligned address modulo 64). The low 5 bits choose the row sum's order;
/// bit 5 chooses the relaxation's and the stencil's split at 1,200 or more elements.
pub fn host_phase_valid(phase: u32) -> bool {
    phase < 64 && phase % 4 == 0
}

/// Object bytes the evaluator's image class allocates between two image handles.
const OBJ_NODE: u64 = 0x70;
/// Object bytes the op's level image class allocates between two image handles.
const OBJ_FLOW: u64 = 0x1b0;
/// Object bytes between the last output plane and the first descriptor plane.
const GAP_DESC: u64 = 0x250;

fn r16(n: u64) -> u64 {
    (n.max(1) + 15) & !15
}

fn alloc_step(n: u64) -> u64 {
    r16(n) + 16
}

fn handle_step(n: u64) -> u64 {
    alloc_step(n + 16) + alloc_step(16)
}

/// Level sides of the flow pyramid, the build loop of the body: side' = trunc(fma(side, scale, 0.5)), a level is stored while its
/// height is at least `min_side` (at least 2) and the next width is computed from it.
pub fn flow_level_dims(width: i32, height: i32, scale: f32, min_side: i32) -> Vec<(i32, i32)> {
    let min = if min_side >= 2 { min_side } else { 2 };
    let mut out = Vec::new();
    let (mut w, mut h) = (width, height);
    if w < min {
        return out;
    }
    while h >= min {
        out.push((w, h));
        let nw = (w as f32).mul_add(scale, 0.5) as i32;
        if nw < min {
            break;
        }
        h = (h as f32).mul_add(scale, 0.5) as i32;
        w = nw;
    }
    out
}

/// Phases of the op's planes: the 17 descriptor planes of level 0 in descriptor word order (warped_image, warped_grad_x,
/// warped_grad_y, coeff, flow_u_next, flow_v_next, relax_first, relax_second, median_out, image_a, image_b, grad_x, grad_y, flow_u,
/// flow_v, nest1_t, copied_plane), the three output planes (level 0's nest1_u, nest2_t, nest2_u) and, per level from 1, the served
/// planes in call order (image_a, image_b, flow_u, flow_v, nest1_t, nest2_t, nest1_u, nest2_u).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowPlacement {
    pub descriptor: [u32; 17],
    pub outputs: [u32; 3],
    pub served: Vec<[u32; 8]>,
}

struct Cursor {
    pos: u64,
    prev: Option<u64>,
}

impl Cursor {
    fn next(&mut self, size: u64, gap: u64) -> u64 {
        if let Some(p) = self.prev {
            self.pos = self.pos.wrapping_add(handle_step(p)).wrapping_add(gap);
        }
        self.prev = Some(size);
        self.pos
    }
}

/// Byte offsets of the op's planes from the first level-0 image (descriptor word 9), same layout as `FlowPlacement`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FlowOffsets {
    pub descriptor: [i64; 17],
    pub outputs: [i64; 3],
    pub served: Vec<[i64; 8]>,
}

/// The in-op chain of a `width` by `height` frame whose flow levels are `levels` (from `flow_level_dims`): plane offsets in bytes
/// relative to the first level-0 image. Arithmetic on sizes only.
pub fn offsets(width: i32, height: i32, levels: &[(i32, i32)]) -> FlowOffsets {
    let (w, h) = (width.max(0) as u64, height.max(0) as u64);
    let frame_bytes = 4 * w * h;
    let mut c = Cursor { pos: 0, prev: None };
    let mut out_pos = [0u64; 3];
    for (k, p) in out_pos.iter_mut().enumerate() {
        *p = c.next(frame_bytes, if k == 0 { 0 } else { OBJ_NODE });
    }
    let mut desc_pos = [0u64; 17];
    for (k, d) in desc_pos.iter_mut().enumerate() {
        *d = c.next(frame_bytes, if k == 0 { GAP_DESC } else { OBJ_FLOW });
    }
    let anchor = desc_pos[9];
    let rel = |pos: u64| pos as i64 - anchor as i64;
    let mut descriptor = [0i64; 17];
    for (d, p) in descriptor.iter_mut().zip(desc_pos) {
        *d = rel(p);
    }
    let outputs = [rel(out_pos[0]), rel(out_pos[1]), rel(out_pos[2])];
    let mut served = Vec::new();
    for &(lw, lh) in levels.iter().skip(1) {
        let size = 4 * lw.max(0) as u64 * lh.max(0) as u64;
        let mut row = [0i64; 8];
        for r in row.iter_mut() {
            *r = rel(c.next(size, OBJ_FLOW));
        }
        served.push(row);
    }
    FlowOffsets { descriptor, outputs, served }
}

/// The placement of a `width` by `height` frame whose flow levels are `levels` (from `flow_level_dims`), for the host value
/// `host_phase` (ALIGN-PHASE-HOST, the phase of the first level-0 image, descriptor word 9).
pub fn placement(width: i32, height: i32, levels: &[(i32, i32)], host_phase: u32) -> FlowPlacement {
    let o = offsets(width, height, levels);
    let phase = |off: i64| (host_phase as i64 + off).rem_euclid(64) as u32;
    FlowPlacement {
        descriptor: o.descriptor.map(phase),
        outputs: o.outputs.map(phase),
        served: o.served.iter().map(|r| r.map(phase)).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dims(w: i32, h: i32) -> Vec<(i32, i32)> {
        flow_level_dims(w, h, f32::from_bits(0x3f34_fdf4), 4)
    }

    #[test]
    fn level_sides_follow_the_build_loop() {
        assert_eq!(dims(64, 32), vec![(64, 32), (45, 23), (32, 16), (23, 11), (16, 8), (11, 6), (8, 4)]);
        assert_eq!(dims(50, 7), vec![(50, 7), (35, 5), (25, 4)]);
    }

    #[test]
    fn level0_image_phases_by_size() {
        for (w, h) in [(64, 32), (72, 24), (128, 64)] {
            let p = placement(w, h, &dims(w, h), HOST_PHASE_DEFAULT);
            // (0, 16) modulo 32; image_b sits 48 bytes after image_a modulo 64
            assert_eq!((p.descriptor[9], p.descriptor[10]), (0, 48), "{w}x{h}");
            assert_eq!((p.descriptor[9] % 32, p.descriptor[10] % 32), (0, 16), "{w}x{h}");
        }
        let p = placement(64, 32, &dims(64, 32), 16);
        assert_eq!((p.descriptor[9], p.descriptor[10]), (16, 0));
        let p = placement(64, 32, &dims(64, 32), 32);
        assert_eq!((p.descriptor[9], p.descriptor[10]), (32, 16));
        assert_eq!((p.descriptor[9] % 32, p.descriptor[10] % 32), (0, 16));
    }

    #[test]
    fn host_phase_is_a_multiple_of_four_below_64() {
        assert!([0, 28, 32, 60].iter().all(|&p| host_phase_valid(p)));
        assert!([2, 64, 65].iter().all(|&p| !host_phase_valid(p)));
    }

    #[test]
    fn served_planes_alternate_where_the_step_is_odd() {
        let p = placement(64, 32, &dims(64, 32), HOST_PHASE_DEFAULT);
        assert_eq!(p.served.len(), 6);
        let m32 = |r: [u32; 8]| r.map(|x| x % 32);
        assert!(p.served[0].iter().all(|&x| x % 32 == 0));
        assert_eq!(m32(p.served[1]), [0, 16, 0, 16, 0, 16, 0, 16]);
        // the 64 byte phase is the 32 byte phase plus bit 5, and anchor 32 flips bit 5 of every plane
        let q = placement(64, 32, &dims(64, 32), 32);
        for (r, t) in p.served.iter().zip(&q.served) {
            for (a, b) in r.iter().zip(t) {
                assert_eq!(a ^ 32, *b);
            }
        }
    }
}
