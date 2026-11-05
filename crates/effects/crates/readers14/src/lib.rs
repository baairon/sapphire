//! FMT3a: the format 14 source readers R4, R3 and R1, variant A, stack word zero.
//!
//! Each original body is a plain scalar function (no VEX, no vector block, no tail jump; read instruction by instruction in asm/R4.txt, R3.txt, R1.txt).
//! Entry registers: rdi = host pixels (16 bytes per pixel, memory order A,R,G,B f32), rsi = pointer to the plane pointer array (4, 3 or 1 used),
//! edx = end (signed 32 bit), r9d = start (signed 32 bit), r8 = 0, ecx = 0. The pixel counter runs from start to end (exclusive), the SOURCE pointer
//! always starts at rdi (it is not offset by start), the plane index k starts at 0. A word at a CPU-flag cell (pointer cell -> word
//! ) selects one of two loop shapes: nonzero = 16-pixel blocks with a prefetchnta, zero = one plain loop. Both compute the same planes.
//!
//! Per pixel (m0..m3 = the four host words, fin(x) = x if (bits & 0x7fffffff) < 0x7f800000 else +0.0, the call):
//!   R4: a = fin(m0); p0 = fin(m1) * a; p1 = fin(m2) * a; p2 = fin(m3) * a; p3 = a.
//!   R3: a = fin(m0); p0 = fin(m1) * a; p1 = fin(m2) * a; p2 = fin(m3) * a. (alpha is used as the weight and not stored)
//!   R1: p0 = fin(m0). (m1..m3 are never read)
//! All values are handled as raw u32 bit patterns so NaN payloads cannot be altered by the language; a product of two finite values is never NaN.
//!
//! Mutants: the compile-time constant FMT3A_MUT selects one numbered change (own build per mutant, SHA-256 recorded by the driver script).

const fn parse(s: &str) -> u32 {
    let b = s.as_bytes();
    let mut i = 0;
    let mut v = 0u32;
    while i < b.len() {
        v = v * 10 + (b[i] - b'0') as u32;
        i += 1;
    }
    v
}
const MUT_ID: u32 = match option_env!("FMT3A_MUT") {
    Some(s) => parse(s),
    None => 0,
};
#[inline(always)]
fn mu(n: u32) -> bool {
    MUT_ID == n
}

/// Refusals by name (ORIGINAL-UNDEFINED or a short buffer given to the port); the port refuses before writing anything.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadErr {
    /// block loop only: some block start r13 < end has r13 + 16 > i32::MAX (the original's block bound wraps negative and the loop restarts from i32::MIN, reading far outside the source)
    BlockCounterOverflow,
    /// the source slice holds fewer than 4 words per pixel of the range
    SrcShort { need: usize, have: usize },
    /// a plane slice holds fewer elements than the range
    PlaneShort { plane: usize, need: usize, have: usize },
}

/// 1 when the sample is finite (bits & 0x7fffffff < 0x7f800000), adc of the borrow.
#[inline(always)]
pub fn is_finite_bits(b: u32) -> bool {
    let m = b & 0x7fff_ffff;
    if mu(1) {
        return m <= 0x7f80_0000;
    }
    if mu(2) {
        return b < 0x7f80_0000;
    }
    m < 0x7f80_0000
}

#[inline(always)]
fn fin(b: u32) -> u32 {
    if is_finite_bits(b) {
        b
    } else {
        if mu(3) {
            return b;
        }
        0 // +0.0
    }
}

#[inline(always)]
fn mulb(x: u32, y: u32) -> u32 {
    if mu(4) {
        return (f32::from_bits(x) + f32::from_bits(y)).to_bits();
    }
    (f32::from_bits(x) * f32::from_bits(y)).to_bits()
}

/// Number of pixels processed and the refusal for the block loop (shared by all three bodies).
fn range(start: i32, end: i32, block: bool) -> Result<usize, ReadErr> {
    // `cmp r9d,edx; jge exit` (signed): nothing happens unless start < end.
    let ge = if mu(5) { start > end } else { start >= end };
    if ge {
        return Ok(0);
    }
    // block loop: r13 = start; repeat { ecx = min(r13 + 16, end); do pixels r13..ecx; r13 += 16 } while r13 < end. Plain loop: do { pixel } while ++r13 < end.
    let mut n = 0usize;
    if block {
        let mut r13 = start;
        let step: i32 = if mu(6) { 0x20 } else { 0x10 };
        loop {
            // the original computes esi = r13 + 16 in 32 bits; a wrapped (negative) esi makes the block empty and the outer loop restart from i32::MIN
            let esi = match r13.checked_add(step) {
                Some(v) => v,
                None => return Err(ReadErr::BlockCounterOverflow),
            };
            let ecx = if esi < end { esi } else { end };
            n += (ecx - r13) as usize;
            r13 = esi;
            if r13 >= end {
                break;
            }
        }
        if mu(7) {
            n = n.saturating_sub(1);
        }
    } else {
        n = (end as i64 - start as i64) as usize;
        if mu(8) {
            n += 1;
        }
    }
    Ok(n)
}

fn check(src: &[u32], planes: &[&mut [u32]], n: usize, words_per_pixel: usize) -> Result<(), ReadErr> {
    if src.len() < n * words_per_pixel {
        return Err(ReadErr::SrcShort { need: n * words_per_pixel, have: src.len() });
    }
    for (k, p) in planes.iter().enumerate() {
        if p.len() < n {
            return Err(ReadErr::PlaneShort { plane: k, need: n, have: p.len() });
        }
    }
    Ok(())
}

/// R4: 4-plane reader. `src` is 4 words per pixel (A,R,G,B).
pub fn read4(src: &[u32], planes: &mut [&mut [u32]; 4], start: i32, end: i32, block: bool) -> Result<(), ReadErr> {
    let n = range(start, end, block)?;
    check(src, &planes[..], n, 4)?;
    for i in 0..n {
        let m = &src[4 * i..4 * i + 4];
        let a = fin(m[0]);
        planes[0][i] = if mu(10) { fin(m[1]) } else { mulb(fin(m[1]), a) };
        planes[1][i] = if mu(11) { mulb(fin(m[1]), a) } else { mulb(fin(m[2]), a) };
        planes[2][i] = if mu(12) { mulb(fin(m[3]), fin(m[3])) } else { mulb(fin(m[3]), a) };
        planes[3][i] = if mu(13) { 0x3f80_0000 } else { a };
    }
    Ok(())
}

/// R3: 3-plane reader. Alpha weights the colour planes and is not stored.
pub fn read3(src: &[u32], planes: &mut [&mut [u32]; 3], start: i32, end: i32, block: bool) -> Result<(), ReadErr> {
    let n = range(start, end, block)?;
    check(src, &planes[..], n, 4)?;
    for i in 0..n {
        let m = &src[4 * i..4 * i + 4];
        let a = fin(m[0]);
        planes[0][i] = if mu(20) { fin(m[1]) } else { mulb(fin(m[1]), a) };
        planes[1][i] = if mu(21) { mulb(fin(m[3]), a) } else { mulb(fin(m[2]), a) };
        planes[2][i] = if mu(22) { mulb(fin(m[2]), a) } else { mulb(fin(m[3]), a) };
    }
    Ok(())
}

/// R1: 1-plane reader. Reads only the first word of each pixel.
pub fn read1(src: &[u32], planes: &mut [&mut [u32]; 1], start: i32, end: i32, block: bool) -> Result<(), ReadErr> {
    let n = range(start, end, block)?;
    check(src, &planes[..], n, 4)?;
    for i in 0..n {
        let m = &src[4 * i..4 * i + 4];
        planes[0][i] = if mu(30) { fin(m[1]) } else if mu(31) { mulb(fin(m[0]), fin(m[0])) } else { fin(m[0]) };
    }
    Ok(())
}
