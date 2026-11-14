//! Border zeroing of the solver body's inline reduction stretch (scalar callback 0x4748b0).
//!
//! The stretch carries six plane pairs from the coarser level to the finer one through the tile passes (module stretch) and
//! zeroes the border of the planes of words 16 to 19. A level with a side below 2 is refused by the body before any launch, so
//! the mean and fill passes the original runs for such a level have no counterpart here; they are a Rust-only oracle in
//! tests/support/small_side.rs.
/// Stand-in labels of this module (see `assumptions()`).
pub struct Assumption {
    pub id: &'static str,
    pub label: &'static str,
    pub basis: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "NO-ALIAS", label: "ASSUMED", basis: "the planes of one step do not overlap each other" },
        Assumption { id: "POSITIVE-DIMS", label: "ASSUMED", basis: "the glue enters a loop only when its width and height are positive; a width of 0 or less in the border callback is not modelled" },
        Assumption { id: "SPLITTER", label: "HOST-SERVICE", basis: "the row range of the border callback is an input (the row splitter is a host service)" },
    ]
}

/// Scalar callback 0x4748b0 (border zeroing). Rows `start .. start + count` of a plane of `width` x `height` with row step
/// `stride`: the first and the last row are zeroed whole, every other row gets zero in its first and last element.
pub fn zero_border(p: &mut [f32], width: i32, height: i32, stride: i32, start: i32, count: u32) {
    let end = start.wrapping_add(count as i32);
    if start >= end {
        return;
    }
    let mut off: i64 = start.wrapping_mul(stride) as i64;
    for k in 0..count {
        let y = start.wrapping_add(k as i32);
        if y == 0 || y == height.wrapping_sub(1) {
            for j in 0..width.max(0) as i64 {
                p[(off + j) as usize] = 0.0;
            }
        } else {
            p[off as usize] = 0.0;
            p[(off + width as i64 - 1) as usize] = 0.0;
        }
        off += stride as i64;
    }
}
