//! FMT3e: the format 14 source readers, VARIANT B, stack word zero: 4 planes, 3 planes and 1 plane (the inline region
//!  of the convert function, with its two arms).
//!
//! Variant B differs from variant A (FMT3a: R4, R3, R1) in what it stores: the colour words are NOT weighted by alpha.
//! Per pixel (m0..m3 = the four host words in memory order A,R,G,B; fin(x) = x if (bits & 0x7fffffff) < 0x7f800000 else +0.0, the call; all values are
//! raw u32 bit patterns, so a stored word is a copy of the input word or zero):
//!   V4: p0 = fin(m1); p1 = fin(m2); p2 = fin(m3); p3 = fin(m0).     (alpha is stored as plane 3, no multiplication)
//!   V3: p0 = fin(m1); p1 = fin(m2); p2 = fin(m3).                   (alpha is not read)
//!   V1: p0 = fin(m0).                                               (m1..m3 are never read)
//! The two standalone bodies V4 and V3 are scalar functions (entry as in FMT3a: rdi = pixels, rsi = plane pointer array, edx = end, r9d = start); their loop
//! structure (a 16 pixel block loop when the CPU-flag word is nonzero, a plain loop when it is zero) is the one FMT3a ported, with the same wrap behaviour.
//! The 1 plane body is inline code with different counting (see `range1`): the destination and source indices both start at 0, the block loop skips (does not
//! restart) a block whose 32 bit bound wraps.
//!
//! Mutants: the compile-time constant FMT3E_MUT selects one numbered change (own build per mutant, SHA-256 recorded by the driver script).

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
const MUT_ID: u32 = match option_env!("FMT3E_MUT") {
    Some(s) => parse(s),
    None => 0,
};
#[inline(always)]
fn mu(n: u32) -> bool {
    MUT_ID == n
}

/// Refusals by name; the port refuses before writing anything.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadErr {
    /// V4 / V3 block loop only: some block start r13 < end has r13 + 16 > i32::MAX (the original's block bound wraps negative and the loop restarts from i32::MIN, reading far outside the source)
    BlockCounterOverflow,
    /// the source slice holds fewer than 4 words per pixel of the range
    SrcShort { need: usize, have: usize },
    /// a plane slice holds fewer elements than the range
    PlaneShort { plane: usize, need: usize, have: usize },
}

/// 1 when the sample is finite (bits & 0x7fffffff < 0x7f800000).
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

/// Pixel count of the V4 / V3 loops (the FMT3a structure: signed start >= end does nothing; block loop r13 .. min(r13 + 16, end)).
fn range(start: i32, end: i32, block: bool) -> Result<usize, ReadErr> {
    let ge = if mu(5) { start > end } else { start >= end };
    if ge {
        return Ok(0);
    }
    let mut n = 0usize;
    if block {
        let mut r13 = start;
        let step: i32 = if mu(6) { 0x20 } else { 0x10 };
        loop {
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

/// Pixel count of the inline 1 plane body : `cmp r8d,[rbx]; jle done` (signed end <= start does nothing). Plain loop (flag word zero):
/// count = end - start (64 bit, unsigned compare, at least one pass). Block loop: nb = ((end - start + 15) >> 4) blocks; block eax: r9d = 16*eax (32 bit),
/// r13d = start + r9d, edi = min(start + r9d + 16, end) with the sum in 32 bit signed arithmetic (a wrapped sum is negative, is not clamped, and the block is
/// skipped because edi <= r13d), count = edi - start - 16*eax (64 bit) when edi > r13d.
pub fn range1(start: i32, end: i32, block: bool) -> usize {
    let le = if mu(5) { end < start } else { end <= start };
    if le {
        return 0;
    }
    if !block {
        let mut n = (end as i64 - start as i64) as usize;
        if mu(8) {
            n += 1;
        }
        return n;
    }
    let step: i32 = if mu(6) { 0x20 } else { 0x10 };
    let diff = end as i64 - start as i64;
    let mut nb = ((diff + 15) >> 4) as u64;
    if mu(7) {
        nb = nb.saturating_sub(1);
    }
    let nb = (nb as u32) as u64; // ecx
    let mut total = 0usize;
    let mut eax: u32 = 0;
    while (eax as u64) < nb {
        let r9d: i32 = (eax as i32).wrapping_mul(16);
        let mut edi: i32 = start.wrapping_add(r9d).wrapping_add(step);
        let r13d: i32 = start.wrapping_add(r9d);
        if edi >= end {
            edi = end;
        }
        if edi > r13d {
            let cnt = edi as i64 - (eax as i32 as i64) * 16 - start as i64;
            total += cnt as u64 as usize;
        }
        eax = eax.wrapping_add(1);
    }
    total
}

fn check(src: &[u32], planes: &[&mut [u32]], n: usize) -> Result<(), ReadErr> {
    if src.len() < n * 4 {
        return Err(ReadErr::SrcShort { need: n * 4, have: src.len() });
    }
    for (k, p) in planes.iter().enumerate() {
        if p.len() < n {
            return Err(ReadErr::PlaneShort { plane: k, need: n, have: p.len() });
        }
    }
    Ok(())
}

/// V4 : 4 plane reader, no alpha weighting. `src` is 4 words per pixel (A,R,G,B).
pub fn read4b(src: &[u32], planes: &mut [&mut [u32]; 4], start: i32, end: i32, block: bool) -> Result<(), ReadErr> {
    let n = range(start, end, block)?;
    check(src, &planes[..], n)?;
    for i in 0..n {
        let m = &src[4 * i..4 * i + 4];
        planes[0][i] = if mu(10) { fin(m[2]) } else { fin(m[1]) };
        planes[1][i] = fin(m[2]);
        planes[2][i] = if mu(12) { fin(m[1]) } else { fin(m[3]) };
        planes[3][i] = if mu(13) { 0x3f80_0000 } else { fin(m[0]) };
    }
    Ok(())
}

/// V3 : 3 plane reader, alpha not read.
pub fn read3b(src: &[u32], planes: &mut [&mut [u32]; 3], start: i32, end: i32, block: bool) -> Result<(), ReadErr> {
    let n = range(start, end, block)?;
    check(src, &planes[..], n)?;
    for i in 0..n {
        let m = &src[4 * i..4 * i + 4];
        planes[0][i] = if mu(20) { fin(m[0]) } else { fin(m[1]) };
        planes[1][i] = if mu(21) { fin(m[3]) } else { fin(m[2]) };
        planes[2][i] = if mu(22) { fin(m[2]) } else { fin(m[3]) };
    }
    Ok(())
}

/// V1 (inline): 1 plane reader (both arms, stack word zero and nonzero, are the same code).
pub fn read1b(src: &[u32], planes: &mut [&mut [u32]; 1], start: i32, end: i32, block: bool) -> Result<(), ReadErr> {
    let n = range1(start, end, block);
    check(src, &planes[..], n)?;
    for i in 0..n {
        let m = &src[4 * i..4 * i + 4];
        planes[0][i] = if mu(30) { fin(m[1]) } else if mu(31) { fin(m[0]) & 0x7fff_ffff } else { fin(m[0]) };
    }
    Ok(())
}

/// The gate (for 4 planes, for 3, for 1): variant B is selected iff
/// ([rsi] != 0 && [ctx] == 0) || [ctx] != 0. `stack_word` ([rbx]) then picks the body: zero = the stack-word-zero bodies of this crate.
pub fn gate_is_b(row_gate_cell: u32, ctx_gate_a: u32, ctx_gate_b: u32) -> bool {
    (row_gate_cell != 0 && ctx_gate_a == 0) || ctx_gate_b != 0
}
