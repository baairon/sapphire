//! Batch 013: a prefix-sum search over an entry table, 8-bit repacking with null checks, BGR to 10-bit YCbCr.
use crate::cov::br;
use crate::*;

/// parity: P1 @0x32f38 size=122
/// Searches a table of dword weights at +0x24 (n = word at +0x34 entries, each multiplied by the dword at +0)
/// for the first index whose running sum (32 bit, wrapping) exceeds esi (unsigned); returns that index, or 0xffff
/// when there is none. Two code paths (a short one for n <= 1 unless the mode dword at +0x18 minus 10 is in
/// the bit set 0x3f2001, and the loop) compute the same result.
pub fn f_00032f38(obj: &[u8], esi: u32) -> u32 {
    let n = rd16(obj, 0x34) as u32;
    let mode = rd32(obj, 0x18);
    let mut use_loop = false;
    if br(n > 1) {
        use_loop = true;
    } else {
        let m = mode.wrapping_sub(10);
        if br(m <= 0x15) && br((0x3f2001u32 >> m) & 1 != 0) {
            use_loop = true;
        }
    }
    if !use_loop {
        let eax = if br(n != 0) { rd32(obj, 0x24).wrapping_mul(rd32(obj, 0)) } else { 0 };
        return if br(eax > esi) { 0 } else { 0xffff };
    }
    let mut k = 0u64;
    let mut sum = 0u32;
    loop {
        let term = if br(n > (k as u16 as u32)) {
            rd32(obj, 0x24 + 4 * k as usize).wrapping_mul(rd32(obj, 0))
        } else {
            0
        };
        sum = sum.wrapping_add(term);
        if br(sum > esi) {
            return k as u16 as u32;
        }
        k += 1;
        if !br(((k as u16) as u32) < n) {
            return 0xffff;
        }
    }
}

/// parity: P1 @0x65891 size=123
/// test: opt=0,1
/// Repacks 4-byte groups into 3 bytes, 2*edx rounds of dst indexes: out0 = (b0 >> 2) | (b1 << 6),
/// out1 = (b1 >> 4) | (b2 << 4), out2 = (b2 >> 6) | (b3 << 2) (8-bit shifts, high bits dropped). Returns false
/// for a null src or dst or edx == 0, true otherwise (also when 2*edx wraps to 0). The null branches cannot be
/// exercised by the oracle (pointer arguments are always mapped buffers).
pub fn f_00065891(src: Option<&[u8]>, dst: Option<&mut [u8]>, edx: u32) -> u8 {
    if br(src.is_none()) {
        return 0;
    }
    if br(dst.is_none()) {
        return 0;
    }
    if br(edx == 0) {
        return 0;
    }
    let (src, dst) = (src.unwrap(), dst.unwrap());
    let end = edx.wrapping_add(edx);
    if br(end == 0) {
        return 1;
    }
    let (mut r8, mut r9) = (0u32, 0u32);
    loop {
        let s = 4 * r8 as usize;
        let d = r9 as usize;
        let (b0, b1, b2, b3) = (src[s], src[s + 1], src[s + 2], src[s + 3]);
        dst[d] = (b0 >> 2) | (b1 << 6);
        dst[d + 1] = (b1 >> 4) | (b2 << 4);
        dst[d + 2] = (b2 >> 6) | (b3 << 2);
        r9 = r9.wrapping_add(3);
        r8 = r8.wrapping_add(1);
        if !br(r9 < end) {
            break;
        }
    }
    1
}

/// parity: P1 @0xc8b9d size=131
/// Converts one BGR byte triple to three 10-bit words (U, Y, V order at dst[0], dst[2], dst[4]) with 14-bit
/// fixed point coefficients: Y = ((0x41bc R + 0x810f G + 0x1910 B) >> 14) + 0x40, U = ((0x7070 B - 0x4a7e G
/// - 0x25f1 R) >> 14) + 0x200 and V = ((0x7070 R - 0x5e27 G - 0x1249 B) >> 14) + 0x200, both masked to 10 bits
/// (BT.601 studio range; inferred from the coefficients, confidence med). R = src[2], G = src[1], B = src[0].
pub fn f_000c8b9d(src: &[u8], dst: &mut [u8]) {
    let b = src[0] as u32;
    let g = src[1] as u32;
    let r = src[2] as u32;
    let y = (b.wrapping_mul(0x1910).wrapping_add(g.wrapping_mul(0x810f)).wrapping_add(r.wrapping_mul(0x41bc)) >> 14)
        .wrapping_add(0x40);
    wr16(dst, 2, y as u16);
    let u = (b.wrapping_mul(0x7070).wrapping_add(g.wrapping_mul(0xffff_b582)).wrapping_add(r.wrapping_mul(0xffff_da0f))
        >> 14)
        .wrapping_add(0x200)
        & 0x3ff;
    wr16(dst, 0, u as u16);
    let v = (b.wrapping_mul(0xffff_edb7).wrapping_add(g.wrapping_mul(0xffff_a1d9)).wrapping_add(r.wrapping_mul(0x7070))
        >> 14)
        .wrapping_add(0x200)
        & 0x3ff;
    wr16(dst, 4, v as u16);
}
