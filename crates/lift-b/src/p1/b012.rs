//! Batch 012: 10-bit unpack/pack kernels, a state reset with two pointer-pair modes, and a constructor.
use crate::cov::br;
use crate::*;

/// parity: P1 @0xc5cac size=93
/// Unpacks 10-10-10 packed dwords into three words each: w[3k] = v & 0x3ff, w[3k+1] = (v >> 10) & 0x3ff,
/// w[3k+2] = (v >> 20) & 0x3ff. The loop runs while the word index is below 2*edx (32 bit); does nothing when
/// 2*edx == 0 (mod 2^32).
pub fn f_000c5cac(src: &[u8], dst: &mut [u8], edx: u32) {
    let end = edx.wrapping_add(edx);
    if br(end == 0) {
        return;
    }
    let (mut r8, mut r10) = (0u32, 0u32);
    loop {
        let v = rd32(src, 4 * r8 as usize);
        wr16(dst, 2 * r10 as usize, (v & 0x3ff) as u16);
        wr16(dst, 2 * r10.wrapping_add(1) as usize, ((v >> 10) & 0x3ff) as u16);
        wr16(dst, 2 * r10.wrapping_add(2) as usize, ((v >> 20) & 0x3ff) as u16);
        r10 = r10.wrapping_add(3);
        r8 = r8.wrapping_add(1);
        if !br(r10 < end) {
            break;
        }
    }
}

/// parity: P1 @0x153cc size=100
/// Buffer-pointer reset on a stream-buffer-like object (inferred): returns false when bit 3 of the byte at
/// +0x18c is set. Otherwise clears the qwords at +0x28, +0x30, +0x38, picks (base, size) from +0x40/+0x60 when the
/// byte at +0x192 is non-zero, else from +0x68/+0x70, stores base at +0x10 and base + size at +0x18 and +0x20,
/// sets the dword at +0x18c to 8 and returns true.
pub fn f_000153cc(obj: &mut [u8]) -> u8 {
    if br(rd8(obj, 0x18c) & 8 != 0) {
        return 0;
    }
    wr64(obj, 0x38, 0);
    wr64(obj, 0x30, 0);
    wr64(obj, 0x28, 0);
    let (base, size) = if br(rd8(obj, 0x192) != 0) {
        (rd64(obj, 0x40), rd64(obj, 0x60))
    } else {
        (rd64(obj, 0x68), rd64(obj, 0x70))
    };
    let end = size.wrapping_add(base);
    wr64(obj, 0x10, base);
    wr64(obj, 0x18, end);
    wr64(obj, 0x20, end);
    wr32(obj, 0x18c, 8);
    1
}

/// parity: P1 @0xc604d size=111
/// In-place repack of esi pixels of 4 words (w0 w1 w2 w3, 8 bytes) into one dword each:
/// v = ((w2 & 3) << 28) | ((w1 & 3) << 26) | ((w0 & 3) << 24) | ((w2 << 14) & 0xff0000) | ((w1 << 6) & 0xff00)
/// | ((w0 >> 2) & 0xff), that is the top 8 of each 10-bit sample in bytes 0..2 and the low 2 bits above.
/// Does nothing for esi == 0.
pub fn f_000c604d(obj: &mut [u8], esi: u32) {
    if br(esi == 0) {
        return;
    }
    let mut k = 0usize;
    loop {
        let w2 = rd16(obj, 8 * k + 4) as u32;
        let w1 = rd16(obj, 8 * k + 2) as u32;
        let w0 = rd16(obj, 8 * k) as u32;
        let v = ((w2 & 3) << 28)
            | ((w2 << 14) & 0x00ff_0000)
            | ((w1 << 6) & 0xff00)
            | ((w1 & 3) << 26)
            | ((w0 >> 2) & 0xff)
            | ((w0 & 3) << 24);
        wr32(obj, 4 * k, v);
        k += 1;
        if !br(k as u32 != esi) {
            break;
        }
    }
}

/// parity: P1 @0xc613e size=115
/// Converts edx packed 10-10-10 dwords (src) to 4 bytes each (dst: b0 b1 b2 0xff). With the low byte of ecx
/// non-zero the bytes are ((v >> 12) & 0xf0) | (v >> 28), ((v >> 6) & 0xfc) | ((v >> 22) & 3), v & 0xff;
/// otherwise v >> 22, v >> 12, v >> 2 (truncated to bytes). Does nothing for edx == 0.
pub fn f_000c613e(dst: &mut [u8], src: &[u8], edx: u32, ecx: u32) {
    if br(edx == 0) {
        return;
    }
    let mut n = edx;
    let (mut s, mut d) = (0usize, 0usize);
    loop {
        let v = rd32(src, s);
        let (b0, b1, b2) = if br(ecx as u8 != 0) {
            (((v >> 12) & 0xf0) | (v >> 28), ((v >> 6) & 0xfc) | ((v >> 22) & 3), v & 0xff)
        } else {
            (v >> 22, v >> 12, v >> 2)
        };
        dst[d] = b0 as u8;
        dst[d + 1] = b1 as u8;
        dst[d + 2] = b2 as u8;
        dst[d + 3] = 0xff;
        s += 4;
        d += 4;
        n = n.wrapping_sub(1);
        if !br(n != 0) {
            break;
        }
    }
}

/// parity: P1 @0x2ce42 size=118
/// Constructor: dwords +0..+0xc zero, +0x10 = 0x58, +0x14 = 8, +0x18 = +0x1c = 0, +0x20 = esi, +0x24 = r8b
/// (zero-extended), qword +0x28 = rdx, dword +0x30 = ecx, qwords +0x34, +0x3c, +0x44, +0x4c zero, dword +0x54 zero.
pub fn f_0002ce42(obj: &mut [u8], esi: u32, rdx: u64, ecx: u32, r8d: u32) {
    wr32(obj, 0, 0);
    wr32(obj, 4, 0);
    wr32(obj, 8, 0);
    wr32(obj, 0xc, 0);
    wr32(obj, 0x10, 0x58);
    wr32(obj, 0x14, 8);
    wr32(obj, 0x18, 0);
    wr32(obj, 0x1c, 0);
    wr32(obj, 0x20, esi);
    wr32(obj, 0x24, r8d & 0xff);
    wr64(obj, 0x28, rdx);
    wr32(obj, 0x30, ecx);
    wr64(obj, 0x4c, 0);
    wr64(obj, 0x44, 0);
    wr64(obj, 0x3c, 0);
    wr64(obj, 0x34, 0);
    wr32(obj, 0x54, 0);
}
