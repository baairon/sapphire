//! Scalar callback 0x460450: a 3x3 neighbourhood median network over a float plane, one row range per call.
//! The network is the listing's chain of minss and maxss instructions (generated, median_chunks.rs); minss and maxss return the
//! second operand when either operand is NaN or the two compare equal, so the result with NaN or signed zero inputs is
//! decided by the operand order, which is kept. Rows above the first and below the last repeat the current row; the first and
//! last columns reuse the network with the column pair loaded as the listing loads it.
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum MedianError {
    /// a read or write outside the plane slices
    OutOfRange(i64),
}

impl fmt::Display for MedianError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MedianError::OutOfRange(i) => write!(f, "plane access at element {} is outside the slice", i),
        }
    }
}

#[inline(always)]
fn minss(a: u32, b: u32) -> u32 {
    if f32::from_bits(a) < f32::from_bits(b) { a } else { b }
}

#[inline(always)]
fn maxss(a: u32, b: u32) -> u32 {
    if f32::from_bits(a) > f32::from_bits(b) { a } else { b }
}

#[inline(always)]
fn ld(s: &[u32], i: i64) -> Result<u32, MedianError> {
    if i < 0 || i as usize >= s.len() { Err(MedianError::OutOfRange(i)) } else { Ok(s[i as usize]) }
}

#[inline(always)]
fn st(d: &mut [u32], i: i64, v: u32) -> Result<(), MedianError> {
    if i < 0 || i as usize >= d.len() { Err(MedianError::OutOfRange(i)) } else { d[i as usize] = v; Ok(()) }
}

include!("median_chunks.rs");

/// Runs rows `start .. start + count` of the callback. `width`, `rows` and `pitch` are the launch context's first three
/// words (pitch in floats); `src` and `dst` are the two planes as bit patterns. Row splitting (start, count) is the host's.
pub fn median_rows(src: &[u32], dst: &mut [u32], width: i32, rows: i32, pitch: i32, start: i32, count: i32) -> Result<(), MedianError> {
    let row_end = count.wrapping_add(start);
    let mut row = start;
    if row >= row_end {
        return Ok(());
    }
    let pitch_i = pitch as i64;
    let mut row_off = pitch.wrapping_mul(row) as i64;
    let mut x = [0u32; 16];
    let mut spill = [0u32; 0x80];
    let rows_minus_one = rows.wrapping_sub(1);
    loop {
        let cu = row_off;
        let mut pv = row_off - pitch_i;
        if row <= 0 {
            pv = cu;
        }
        let mut nx = row_off + pitch_i;
        if row >= rows_minus_one {
            nx = cu;
        }
        let dst_row = row_off;
        let mut col: i64 = 1;
        let width_i = width as i64;
        chunk_A!(x, spill, src, dst, nx, cu, pv, dst_row, col, width_i);
        if width_i > 2 {
            chunk_P!(x, spill, src, dst, nx, cu, pv, dst_row, col, width_i);
            loop {
                chunk_B!(x, spill, src, dst, nx, cu, pv, dst_row, col, width_i);
                col += 1;
                if !(col < width_i - 1) {
                    break;
                }
            }
            x[13] = spill[0x38];
        }
        chunk_C!(x, spill, src, dst, nx, cu, pv, dst_row, col, width_i);
        row = row.wrapping_add(1);
        row_off += pitch_i;
        if !(row < row_end) {
            break;
        }
    }
    Ok(())
}
