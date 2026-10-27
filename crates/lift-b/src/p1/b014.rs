//! Batch 014: more pixel/sample repacking kernels (10-bit unpack variants, component selection by mode and
//! parity, 12-bit packing).
use crate::cov::br;
use crate::*;

/// parity: P1 @0x59109 size=137
/// Converts edx packed 10-10-10 dwords (src) to three words each (dst: w0 w1 w2 plus an untouched fourth word,
/// 8 bytes per pixel). With the low byte of ecx zero: w0 = (v >> 2) & 0x3ff, w1 = (v >> 12) & 0x3ff, w2 = v >> 22.
/// Otherwise: w0 = ((v >> 12) & 0xf0) + (v >> 26), w1 = ((v >> 4) & 0x3f0) | ((v >> 20) & 0xf), w2 = (v << 2) & 0x3fc.
/// Does nothing for edx == 0. (Inverse direction of f_000c613e.)
pub fn f_00059109(dst: &mut [u8], src: &[u8], edx: u32, ecx: u32) {
    if br(edx == 0) {
        return;
    }
    let mut n = edx;
    let (mut s, mut d) = (0usize, 0usize);
    loop {
        let v = rd32(src, s);
        let (w0, w1, w2) = if br(ecx as u8 != 0) {
            (
                ((v >> 12) & 0xf0).wrapping_add(v >> 26),
                ((v >> 4) & 0x3f0) | ((v >> 20) & 0xf),
                v.wrapping_shl(2) & 0x3fc,
            )
        } else {
            ((v >> 2) & 0x3ff, (v >> 12) & 0x3ff, v >> 22)
        };
        wr16(dst, d, w0 as u16);
        wr16(dst, d + 2, w1 as u16);
        wr16(dst, d + 4, w2 as u16);
        s += 4;
        d += 8;
        n = n.wrapping_sub(1);
        if !br(n != 0) {
            break;
        }
    }
}

/// parity: P1 @0xc772c size=146
/// Gathers one 16-bit component from each of edx pixels of 4 words (8 bytes) into dst[2*j]. The component
/// offset (0, 2 or 4 bytes into the pixel) depends on the parity of j, on the mode r8d (2, 3, 4, other) and on
/// bit 0 of ecx. Offsets as (even j, odd j) per (mode, bit): mode 2: bit 0 -> (2,4), bit 1 -> (0,2);
/// mode 3: (0,2) / (2,4); mode 4: (2,0) / (4,2); other: (4,2) / (2,0). Does nothing for edx == 0.
pub fn f_000c772c(src: &[u8], dst: &mut [u8], edx: u32, ecx: u32, r8d: u32) {
    if br(edx == 0) {
        return;
    }
    let bit = ecx & 1;
    let (even, odd) = if br(r8d == 2) {
        if br(bit != 0) { (0, 2) } else { (2, 4) }
    } else if br(r8d == 3) {
        if br(bit != 0) { (2, 4) } else { (0, 2) }
    } else if br(r8d == 4) {
        if br(bit != 0) { (4, 2) } else { (2, 0) }
    } else if br(bit != 0) {
        (2, 0)
    } else {
        (4, 2)
    };
    let mut j = 0u32;
    loop {
        let off = if br(j & 1 == 0) { even } else { odd };
        let w = rd16(src, 8 * j as usize + off);
        wr16(dst, 2 * j as usize, w);
        j = j.wrapping_add(1);
        if !br(j != edx) {
            break;
        }
    }
}

/// parity: P1 @0xc7b35 size=183
/// Packs 12-bit samples: each round reads 8 words (16 bytes, words 3 and 7 unused) and writes 9 bytes:
/// (w2 >> 4), (w2 >> 12) | (w1 & 0xf0), (w1 >> 8), (w0 >> 4), (w0 >> 12) | (w6 & 0xf0), (w6 >> 8), (w5 >> 4),
/// (w5 >> 12) | (w4 & 0xf0), (w4 >> 8) (all truncated to bytes). The output byte count is (36 * edx) >> 3 in
/// 32-bit arithmetic and rounds run while the output index is below it. Does nothing when that count is 0.
pub fn f_000c7b35(src: &[u8], dst: &mut [u8], edx: u32) {
    let e = edx.wrapping_shl(2);
    let r8 = e.wrapping_add(e.wrapping_shl(3)) >> 3;
    if br(r8 == 0) {
        return;
    }
    let mut rcx = 0u64;
    let mut s = 0usize;
    loop {
        let w = |k: usize| rd16(src, s + 2 * k) as u32;
        let (w0, w1, w2, w4, w5, w6) = (w(0), w(1), w(2), w(4), w(5), w(6));
        let d = rcx as usize;
        dst[d] = (w2 >> 4) as u8;
        dst[d + 1] = ((w2 >> 12) | (w1 & 0xfff0)) as u8;
        dst[d + 2] = (w1 >> 8) as u8;
        dst[d + 3] = (w0 >> 4) as u8;
        dst[d + 4] = ((w0 >> 12) | (w6 & 0xfff0)) as u8;
        dst[d + 5] = (w6 >> 8) as u8;
        dst[d + 6] = (w5 >> 4) as u8;
        dst[d + 7] = ((w5 >> 12) | (w4 & 0xfff0)) as u8;
        dst[d + 8] = (w4 >> 8) as u8;
        rcx += 9;
        s += 0x10;
        if !br((rcx as u32) < r8) {
            break;
        }
    }
}

/// parity: P1 @0xc7978 size=191
/// Packs 12-bit components gathered like f_000c772c (same (mode, bit) -> (even, odd) offset table) from edx
/// pixels of 4 words: an even sample w writes dst[p] = w >> 4 and dst[p+1] = w >> 12; the following odd sample w
/// ORs (w & 0xf0) into dst[p+1] and writes dst[p+2] = w >> 8, giving 3 bytes per pair. Does nothing for edx == 0.
pub fn f_000c7978(src: &[u8], dst: &mut [u8], edx: u32, ecx: u32, r8d: u32) {
    if br(edx == 0) {
        return;
    }
    let bit = ecx & 1;
    let (even, odd) = if br(r8d == 2) {
        if br(bit != 0) { (0, 2) } else { (2, 4) }
    } else if br(r8d == 3) {
        if br(bit != 0) { (2, 4) } else { (0, 2) }
    } else if br(r8d == 4) {
        if br(bit != 0) { (4, 2) } else { (2, 0) }
    } else if br(bit != 0) {
        (2, 0)
    } else {
        (4, 2)
    };
    let mut j = 0u32;
    let mut p = 0usize;
    loop {
        if br(j & 1 == 0) {
            let w = rd16(src, 8 * j as usize + even) as u32;
            dst[p] = (w >> 4) as u8;
            dst[p + 1] = ((w & 0xf000) >> 12) as u8;
            p += 1;
        } else {
            let w = rd16(src, 8 * j as usize + odd) as u32;
            dst[p] |= (w as u8) & 0xf0;
            dst[p + 1] = (w >> 8) as u8;
            p += 2;
        }
        j = j.wrapping_add(1);
        if !br(j != edx) {
            break;
        }
    }
}
