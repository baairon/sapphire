//! the on-route destination bodies of format code 14 (zero stack-word arm) for 4, 3, 2 and 1 planes.
//! Ported from the listing: 4 planes (A, B, C), 2 planes (A, B, C),
//! 3 planes (shared, tier bodies inside) and 1 plane (shared). The tiers store the same words; the
//! vector blocks versus scalar remainder split is output-invisible (checked by the T2, not assumed).
//! Destination layout: 4 f32 words per pixel (word 0 alpha, words 1..3 colour). Source planes are f32 bit patterns.

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CErr {
    /// the original logs (9, text, mode) and writes nothing
    BadMode(i32),
    /// cell <= 0: the original divides by it (cell 0 raises #DE); out of contract
    BadCell(i32),
    /// planes or destination too short for the request
    Shape,
}

/// Alpha clamp of the 4-plane, 2-plane and 1-plane mode-1 stores: a < 0 gives +0.0, 1.0 < a gives 1.0, otherwise a
/// (NaN and -0.0 pass through bit for bit). Source: vcomiss 0, a / jbe, vminss 1.0, a .
pub fn clamp01(a: u32) -> u32 {
    let f = f32::from_bits(a);
    if f < 0.0 {
        0
    } else if 1.0f32 < f {
        1.0f32.to_bits()
    } else {
        a
    }
}

pub fn nsrc(count: i32, cell: i32) -> usize {
    if count <= 0 || cell <= 0 {
        0
    } else {
        ((count as i64 + cell as i64 - 1) / cell as i64) as usize
    }
}

/// np in 1..=4. planes: np slices. dst: 4 words per pixel. mode: the stack word [rbp] (r8d for 3 and 1 planes).
pub fn convert14(np: usize, planes: &[&[u32]], dst: &mut [u32], count: i32, cell: i32, mode: i32) -> Result<(), CErr> {
    // mode gate (the original checks it before anything else, and even for count 0)
    let ok_mode = match np {
        4 | 2 => mode == 1,
        3 | 1 => mode == 1 || mode == 2 || mode == 3,
        _ => return Err(CErr::Shape),
    };
    if !ok_mode {
        return Err(CErr::BadMode(mode));
    }
    if count <= 0 {
        return Ok(());
    }
    if cell <= 0 {
        return Err(CErr::BadCell(cell));
    }
    let n = nsrc(count, cell);
    if planes.len() < np || planes.iter().take(np).any(|p| p.len() < n) || dst.len() < (count as usize) * 4 {
        return Err(CErr::Shape);
    }
    let cell = cell as usize;
    let count = count as usize;
    for s in 0..n {
        let p0 = s * cell;
        let d = &mut dst[p0 * 4..p0 * 4 + 4];
        match np {
            4 => {
                d[1] = planes[0][s];
                d[2] = planes[1][s];
                d[3] = planes[2][s];
                d[0] = clamp01(planes[3][s]);
            }
            3 => {
                d[1] = planes[0][s];
                d[2] = planes[1][s];
                d[3] = planes[2][s];
                if mode != 2 {
                    d[0] = 1.0f32.to_bits();
                }
            }
            2 => {
                d[1] = planes[0][s];
                d[2] = planes[0][s];
                d[3] = planes[0][s];
                d[0] = clamp01(planes[1][s]);
            }
            _ => {
                d[1] = planes[0][s];
                d[2] = planes[0][s];
                d[3] = planes[0][s];
                if mode == 1 {
                    d[0] = clamp01(planes[0][s]);
                } else if mode == 3 {
                    d[0] = 1.0f32.to_bits();
                }
            }
        }
        let first = [dst[p0 * 4], dst[p0 * 4 + 1], dst[p0 * 4 + 2], dst[p0 * 4 + 3]];
        let end = ((s + 1) * cell).min(count);
        for p in p0 + 1..end {
            dst[p * 4..p * 4 + 4].copy_from_slice(&first);
        }
    }
    Ok(())
}

/// The row copier body for the format pair (14, 14): (selector, dispatch; store).
/// Word 0 of each source pixel (16 bytes) goes to word 0 of the destination pixel; the other words are untouched.
/// Source: test r8d,r8d / je; do { dst[0] = src[0]; dst += 16; src += 16; eax++ } while (eax < n) with n signed, so
/// n == 0 does nothing and n < 0 stores exactly one pixel (out of contract, ported as read).
pub fn copy_word0(src: &[u32], dst: &mut [u32], n: i32) -> Result<(), CErr> {
    if n == 0 {
        return Ok(());
    }
    let k = if n > 0 { n as usize } else { 1 };
    if src.len() < k * 4 || dst.len() < k * 4 {
        return Err(CErr::Shape);
    }
    for i in 0..k {
        dst[i * 4] = src[i * 4];
    }
    Ok(())
}
