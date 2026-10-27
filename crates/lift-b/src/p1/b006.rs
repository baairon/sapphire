//! Batch 006: flag bit setter, ladders over unsigned bounds, set predicates, nibble extraction and the first
//! pixel/sample repacking loops (4 bytes or 4 words per pixel to 3, 10-bit packing, byte reversal).
use crate::cov::br;
use crate::*;

/// parity: P1 @0xe5e18 size=42
/// Stores the low byte of esi at +0x15 and sets (nonzero) or clears (zero) bit 0x800 of the dword at +0x84.
pub fn f_000e5e18(obj: &mut [u8], esi: u32) {
    let flag = esi as u8;
    wr8(obj, 0x15, flag);
    let v = rd32(obj, 0x84);
    wr32(obj, 0x84, if br(flag != 0) { v | 0x800 } else { v & !0x800 });
}

/// parity: P1 @0x2f087 size=43
/// Scales esi by edi in three regimes: esi * edi when edi > 0x20000; (esi - 0x20) * edi + 0x20000 when
/// esi > 0x1f; otherwise esi << 12. All arithmetic is 32 bit and wraps.
pub fn f_0002f087(edi: u32, esi: u32) -> u32 {
    if br(edi > 0x20000) {
        esi.wrapping_mul(edi)
    } else if br(esi > 0x1f) {
        esi.wrapping_sub(0x20).wrapping_mul(edi).wrapping_add(0x20000)
    } else {
        esi.wrapping_shl(12)
    }
}

/// parity: P1 @0x5cbdb size=43
/// Predicate on a small code: for edi <= 99 (signed) true when edi - 23 < 3 (unsigned); for edi > 99 true
/// when edi - 100 <= 16 and bit (edi - 100) of 0x1ccff is set.
pub fn f_0005cbdb(edi: u32) -> u8 {
    if br((edi as i32) > 0x63) {
        let n = edi.wrapping_sub(100);
        (br(n <= 0x10) && br((0x1ccffu32 >> n) & 1 != 0)) as u8
    } else {
        br(edi.wrapping_sub(23) < 3) as u8
    }
}

/// parity: P1 @0x28c72 size=44
/// Initialiser: stores rsi at +0, edx at +8 and zeroes the dword at +0x24 and the qwords at +0x1c, +0x14, +0xc.
pub fn f_00028c72(obj: &mut [u8], rsi: u64, edx: u32) {
    wr64(obj, 0, rsi);
    wr32(obj, 8, edx);
    wr32(obj, 0x24, 0);
    wr64(obj, 0x1c, 0);
    wr64(obj, 0x14, 0);
    wr64(obj, 0xc, 0);
}

/// parity: P1 @0x5c6c4 size=46
/// Maps a small code: edi > 9 gives 5, edi in 2..=9 gives 0, edi == 1 gives 3, edi == 0 gives 2.
pub fn f_0005c6c4(edi: u32) -> u32 {
    if br(edi > 9) {
        5
    } else if br(edi > 1) {
        0
    } else if br(edi == 1) {
        3
    } else {
        2
    }
}

/// parity: P1 @0x66bd0 size=47
/// In-place repack of esi pixels from 4 bytes to 3 with the first three bytes reversed (b0 b1 b2 b3 ->
/// b2 b1 b0, e.g. BGRA to RGB). Reads of a pixel precede its writes. Does nothing for esi == 0.
pub fn f_00066bd0(obj: &mut [u8], esi: u32) {
    if br(esi == 0) {
        return;
    }
    let mut n = esi;
    let (mut s, mut d) = (0usize, 0usize);
    loop {
        let cl = obj[s + 2];
        let r8b = obj[s];
        let dl = obj[s + 1];
        obj[d] = cl;
        obj[d + 1] = dl;
        obj[d + 2] = r8b;
        s += 4;
        d += 3;
        n = n.wrapping_sub(1);
        if !br(n != 0) {
            break;
        }
    }
}

/// parity: P1 @0x67476 size=48
/// Packs edx pixels of 3 source bytes (stride 4) into 10-bit-per-channel dwords:
/// dst32 = b0 << 22 | b1 << 12 | b2 << 2. Does nothing for edx == 0.
pub fn f_00067476(src: &[u8], dst: &mut [u8], edx: u32) {
    if br(edx == 0) {
        return;
    }
    let mut n = edx;
    let (mut s, mut d) = (0usize, 0usize);
    loop {
        let v = ((src[s] as u32) << 22) | ((src[s + 1] as u32) << 12) | ((src[s + 2] as u32) << 2);
        wr32(dst, d, v);
        s += 4;
        d += 4;
        n = n.wrapping_sub(1);
        if !br(n != 0) {
            break;
        }
    }
}

/// parity: P1 @0xc601b size=50
/// In-place repack of esi pixels of 4 words (8 bytes, only the first 3 used) into one dword each:
/// w2 + (w1 << 10) + (w0 << 20) (additions, not ors). Does nothing for esi == 0.
pub fn f_000c601b(obj: &mut [u8], esi: u32) {
    if br(esi == 0) {
        return;
    }
    let mut k = 0usize;
    loop {
        let w0 = rd16(obj, 8 * k) as u32;
        let w1 = rd16(obj, 8 * k + 2) as u32;
        let w2 = rd16(obj, 8 * k + 4) as u32;
        wr32(obj, 4 * k, w2.wrapping_add(w1 << 10).wrapping_add(w0 << 20));
        k += 1;
        if !br(k as u32 != esi) {
            break;
        }
    }
}

/// parity: P1 @0xc7b03 size=50
/// Repacks edx pixels of 4 words (8 bytes) into 3 words with the order reversed: (w0 w1 w2 w3) -> (w2 w1 w0).
/// Does nothing for edx == 0.
pub fn f_000c7b03(src: &[u8], dst: &mut [u8], edx: u32) {
    if br(edx == 0) {
        return;
    }
    let mut n = edx;
    let (mut s, mut d) = (0usize, 0usize);
    loop {
        let w0 = rd16(src, s);
        let w1 = rd16(src, s + 2);
        let w2 = rd16(src, s + 4);
        wr16(dst, d, w2);
        wr16(dst, d + 2, w1);
        wr16(dst, d + 4, w0);
        s += 8;
        d += 6;
        n = n.wrapping_sub(1);
        if !br(n != 0) {
            break;
        }
    }
}

/// parity: P1 @0xe6598 size=50
/// Extracts nibble (esi - 1) of the 64-bit value formed by the dwords at +0x84 (nibbles 0..7) and +0x88
/// (nibbles 8..15); returns 0 when esi - 1 > 15 (unsigned).
pub fn f_000e6598(obj: &[u8], esi: u32) -> u32 {
    let ecx = esi.wrapping_sub(1);
    if br(ecx > 0xf) {
        return 0;
    }
    let (v, sh) = if br((ecx as i32) > 7) {
        (rd32(obj, 0x88), esi.wrapping_mul(4).wrapping_sub(0x24))
    } else {
        (rd32(obj, 0x84), ecx << 2)
    };
    (v >> (sh & 31)) & 0xf
}

/// parity: P1 @0x5cb7b size=51
/// Two-range bit-table predicate: true when edi - 0x56 < 24 and bit (edi - 0x56) of 0xfe3c3f is set;
/// else for edi - 0x50 < 23 the value is bit (edi - 0x50) of 0x70f03f; otherwise false.
pub fn f_0005cb7b(edi: u32) -> u8 {
    let n1 = edi.wrapping_sub(0x56);
    if br(n1 < 0x18) && br((0xfe3c3fu32 >> n1) & 1 != 0) {
        return 1;
    }
    let n2 = edi.wrapping_sub(0x50);
    if br(n2 < 0x17) {
        ((0x70f03fu32 >> (n2 & 31)) & 1) as u8
    } else {
        0
    }
}

/// parity: P1 @0xc7a9b size=52
/// Converts 4-byte pixels to 3 bytes with the order reversed (b0 b1 b2 b3 -> b2 b1 b0), edx pixels
/// (the byte count 4*edx wraps at 32 bits). Does nothing when 4*edx == 0.
pub fn f_000c7a9b(src: &[u8], dst: &mut [u8], edx: u32) {
    let n4 = edx.wrapping_shl(2);
    if br(n4 == 0) {
        return;
    }
    let mut rcx = 0u64;
    let mut d = 0usize;
    loop {
        let i = rcx as usize;
        let (b0, b1, b2) = (src[i], src[i + 1], src[i + 2]);
        dst[d] = b2;
        dst[d + 1] = b1;
        dst[d + 2] = b0;
        rcx += 4;
        d += 3;
        if !br(rcx < n4 as u64) {
            break;
        }
    }
}

/// parity: P1 @0xc7acf size=52
/// Converts 4-byte pixels to 3 bytes by dropping the fourth byte (b0 b1 b2 b3 -> b0 b1 b2), edx pixels
/// (the byte count 4*edx wraps at 32 bits). Does nothing when 4*edx == 0.
pub fn f_000c7acf(src: &[u8], dst: &mut [u8], edx: u32) {
    let n4 = edx.wrapping_shl(2);
    if br(n4 == 0) {
        return;
    }
    let mut rcx = 0u64;
    let mut d = 0usize;
    loop {
        let i = rcx as usize;
        dst[d] = src[i];
        dst[d + 1] = src[i + 1];
        dst[d + 2] = src[i + 2];
        rcx += 4;
        d += 3;
        if !br(rcx < n4 as u64) {
            break;
        }
    }
}

/// parity: P1 @0x260f4 size=54
/// Initialiser: obj = { 0, esi, 0, 0, edx, 8, 0, 0 } as eight dwords.
pub fn f_000260f4(obj: &mut [u8], esi: u32, edx: u32) {
    wr32(obj, 0, 0);
    wr32(obj, 4, esi);
    wr32(obj, 8, 0);
    wr32(obj, 0xc, 0);
    wr32(obj, 0x10, edx);
    wr32(obj, 0x14, 8);
    wr32(obj, 0x18, 0);
    wr32(obj, 0x1c, 0);
}

/// parity: P1 @0x58929 size=54
/// Fills words with the repeating pattern (dx, si, cx, si): 4 words per round, rounds while
/// r9 < 2 * r8d (32 bit, unsigned). Does nothing when 2 * r8d == 0 (mod 2^32).
pub fn f_00058929(obj: &mut [u8], esi: u32, edx: u32, ecx: u32, r8d: u32) {
    let end = r8d.wrapping_add(r8d);
    if br(end == 0) {
        return;
    }
    let mut r9 = 0u32;
    loop {
        wr16(obj, 2 * r9 as usize, edx as u16);
        wr16(obj, 2 * r9.wrapping_add(1) as usize, esi as u16);
        wr16(obj, 2 * r9.wrapping_add(2) as usize, ecx as u16);
        wr16(obj, 2 * r9.wrapping_add(3) as usize, esi as u16);
        r9 = r9.wrapping_add(4);
        if !br(r9 < end) {
            break;
        }
    }
}

/// parity: P1 @0xce24b size=55
/// Set predicate: edi in {0x10478300, 0x10478350, 0x10710800, 0x10710850, 0x10710851}, evaluated with the
/// original's signed split at 0x1047834f.
pub fn f_000ce24b(edi: u32) -> u8 {
    if br((edi as i32) > 0x1047_834f) {
        (br(edi.wrapping_sub(0x1071_0850) < 2) || br(edi == 0x1047_8350) || br(edi == 0x1071_0800)) as u8
    } else {
        br(edi == 0x1047_8300) as u8
    }
}

/// parity: P1 @0xce3db size=55
/// Set predicate: edi in {0x10478300, 0x10518400, 0x10710800, 0x10710850, 0x10710851}, evaluated with the
/// original's signed split at 0x105183ff.
pub fn f_000ce3db(edi: u32) -> u8 {
    if br((edi as i32) > 0x1051_83ff) {
        (br(edi.wrapping_sub(0x1071_0850) < 2) || br(edi == 0x1051_8400) || br(edi == 0x1071_0800)) as u8
    } else {
        br(edi == 0x1047_8300) as u8
    }
}

/// parity: P1 @0xc60bc size=57
/// In-place repack of esi pixels of 4 words into one dword each: (w2 << 22) + (w1 << 12) + (w0 << 2),
/// byte-swapped when the low byte of edx is non-zero. Does nothing for esi == 0.
pub fn f_000c60bc(obj: &mut [u8], esi: u32, edx: u32) {
    if br(esi == 0) {
        return;
    }
    let mut k = 0usize;
    loop {
        let w2 = rd16(obj, 8 * k + 4) as u32;
        let w1 = rd16(obj, 8 * k + 2) as u32;
        let w0 = rd16(obj, 8 * k) as u32;
        let mut v = (w2 << 22).wrapping_add(w1 << 12).wrapping_add(w0 << 2);
        if br(edx as u8 != 0) {
            v = v.swap_bytes();
        }
        wr32(obj, 4 * k, v);
        k += 1;
        if !br(k as u32 != esi) {
            break;
        }
    }
}

/// parity: P1 @0x675ee size=58
/// Repacks pixels of 4 words (8 bytes) into 3 words with the order reversed (w0 w1 w2 w3 -> w2 w1 w0);
/// edx pixels (the count 4*edx wraps at 32 bits). Does nothing when 4*edx == 0.
pub fn f_000675ee(src: &[u8], dst: &mut [u8], edx: u32) {
    let n4 = edx.wrapping_shl(2);
    if br(n4 == 0) {
        return;
    }
    let mut rcx = 0u64;
    let mut d = 0usize;
    loop {
        let s = 2 * rcx as usize;
        let w0 = rd16(src, s);
        let w1 = rd16(src, s + 2);
        let w2 = rd16(src, s + 4);
        wr16(dst, d, w2);
        wr16(dst, d + 2, w1);
        wr16(dst, d + 4, w0);
        rcx += 4;
        d += 6;
        if !br(rcx < n4 as u64) {
            break;
        }
    }
}

/// parity: P1 @0x2ac82 size=60
/// Initialiser: dword 4 at +0, zero elsewhere in 4..0x30.
pub fn f_0002ac82(obj: &mut [u8]) {
    wr32(obj, 0, 4);
    wr32(obj, 0x2c, 0);
    wr64(obj, 0x24, 0);
    wr64(obj, 0x1c, 0);
    wr64(obj, 0x14, 0);
    wr64(obj, 0xc, 0);
    wr64(obj, 4, 0);
}

/// parity: P1 @0xc7a37 size=60
/// Reverses the byte order of esi dwords in place (the byte count 4*esi wraps at 32 bits). Does nothing
/// when 4*esi == 0.
pub fn f_000c7a37(obj: &mut [u8], esi: u32) {
    let n4 = esi.wrapping_shl(2);
    if br(n4 == 0) {
        return;
    }
    let mut rcx = 0u64;
    loop {
        let i = rcx as usize;
        let (b0, b1, b2, b3) = (obj[i], obj[i + 1], obj[i + 2], obj[i + 3]);
        obj[i] = b3;
        obj[i + 1] = b2;
        obj[i + 2] = b1;
        obj[i + 3] = b0;
        rcx += 4;
        if !br(rcx < n4 as u64) {
            break;
        }
    }
}
