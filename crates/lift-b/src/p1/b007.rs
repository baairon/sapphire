//! Batch 007: larger set predicates with signed splits, wrappers around the constant-zero stub f_0004eeb4,
//! 16-bit byte swapping, struct equality and a pointer-swapping exchange.
use crate::cov::br;
use crate::p1::b001;
use crate::*;

/// parity: P1 @0xcc4b2 size=63
/// Set predicate: edi in {0x10266400, 0x10266401, 0x10294700, 0x10378800, 0x10478350, 0x10518450},
/// evaluated with the original's signed split at 0x1047834f.
pub fn f_000cc4b2(edi: u32) -> u8 {
    if br((edi as i32) > 0x1047_834f) {
        (br(edi == 0x1047_8350) || br(edi == 0x1051_8450)) as u8
    } else {
        (br(edi.wrapping_sub(0x1026_6400) < 2) || br(edi == 0x1029_4700) || br(edi == 0x1037_8800)) as u8
    }
}

/// parity: P1 @0xcc106 size=65
/// Set predicate with a signed split at 0x106466ff: below, edi in {0x10518400, 0x10538200, 0x10565400};
/// above, edi - 0x10646700 in {0, 1, 5, 6} (bit table 0x63 over 0..=6).
pub fn f_000cc106(edi: u32) -> u8 {
    if br((edi as i32) > 0x1064_66ff) {
        let n = edi.wrapping_sub(0x1064_6700);
        (br(n <= 6) && br((0x63u32 >> n) & 1 != 0)) as u8
    } else {
        (br(edi == 0x1051_8400) || br(edi == 0x1053_8200) || br(edi == 0x1056_5400)) as u8
    }
}

/// parity: P1 @0xce321 size=66
/// Set predicate with a signed split at 0x106466ff: below, edi in {0x10478300, 0x10518400}; above, edi in
/// 0x10646700..=0x10646706, {0x10710850, 0x10710851} or 0x10710800. (33 callers in the dump.)
pub fn f_000ce321(edi: u32) -> u8 {
    if br((edi as i32) > 0x1064_66ff) {
        (br(edi.wrapping_sub(0x1064_6700) < 7) || br(edi.wrapping_sub(0x1071_0850) < 2) || br(edi == 0x1071_0800))
            as u8
    } else {
        (br(edi == 0x1047_8300) || br(edi == 0x1051_8400)) as u8
    }
}

/// parity: P1 @0x44e74 size=68
/// Wrapper that re-packs its arguments (edx, ecx, r8d, r9d and three stack arguments) and calls
/// f_0004eeb4, a stub that ignores everything and returns 0. Returns that result (eax).
pub fn f_00044e74(edx: u32, ecx: u32, r8d: u32, r9d: u32, stk0: u32, stk1: u32, stk2: u64) -> u32 {
    let _ = (edx, ecx, r8d, r9d, stk0, stk1, stk2);
    b001::f_0004eeb4()
}

/// parity: P1 @0x44e2c size=72
/// Wrapper that calls f_0004eeb4 (a stub that returns 0, ignoring its nine arguments) with constant
/// zero/one arguments mixed with esi, edx, ecx and r8. Returns that result (eax).
pub fn f_00044e2c(esi: u32, edx: u32, ecx: u32, r8: u64) -> u32 {
    let _ = (esi, edx, ecx, r8);
    b001::f_0004eeb4()
}

/// parity: P1 @0xc7bec size=69
/// Swaps the bytes inside each 16-bit word of groups of two words: (b0 b1 b2 b3) -> (b1 b0 b3 b2), rounds
/// while the byte index is below 2*esi (32 bit). For odd esi the last round also touches the two bytes
/// after the data. Does nothing when 2*esi == 0 (mod 2^32).
pub fn f_000c7bec(obj: &mut [u8], esi: u32) {
    let end = esi.wrapping_add(esi);
    if br(end == 0) {
        return;
    }
    let mut eax = 0u32;
    loop {
        let i0 = eax as usize;
        let i1 = eax.wrapping_add(1) as usize;
        let i2 = eax.wrapping_add(2) as usize;
        let i3 = eax.wrapping_add(3) as usize;
        let (b0, b1, b2, b3) = (obj[i0], obj[i1], obj[i2], obj[i3]);
        obj[i0] = b1;
        obj[i1] = b0;
        obj[i2] = b3;
        obj[i3] = b2;
        eax = eax.wrapping_add(4);
        if !br(eax < end) {
            break;
        }
    }
}

/// parity: P1 @0x29088 size=70
/// Exchange of two {ptr at +0, count at +8, word at +0xc} records: fails (false) when either pointer or
/// count is zero, or the counts or the words differ; otherwise swaps the two pointers (when they differ)
/// and returns true. The pointers are only compared and swapped, never dereferenced.
pub fn f_00029088(a: &mut [u8], b: &mut [u8]) -> u8 {
    let ptr_b = rd64(b, 0);
    if br(ptr_b == 0) {
        return 0;
    }
    let nb = rd32(b, 8);
    if br(nb == 0) {
        return 0;
    }
    let ptr_a = rd64(a, 0);
    if br(ptr_a == 0) {
        return 0;
    }
    let na = rd32(a, 8);
    if br(na == 0) {
        return 0;
    }
    if br(nb != na) {
        return 0;
    }
    if br(rd32(a, 0xc) != rd32(b, 0xc)) {
        return 0;
    }
    if br(ptr_b != ptr_a) {
        wr64(a, 0, ptr_b);
        wr64(b, 0, ptr_a);
    }
    1
}

/// parity: P1 @0x18050 size=72
/// Equality of two records: true when the dwords at +0, +4, +8 and the qword at +0x10 are all equal.
/// (Counts the differing fields among those four with the +8 dword tested against 0 or -1; the -1 case
/// can never match a count, so the result is plain equality.)
pub fn f_00018050(a: &[u8], b: &[u8]) -> u8 {
    let ne0 = br(rd32(a, 0) != rd32(b, 0)) as u64;
    let ne4 = br(rd32(a, 4) != rd32(b, 4)) as u64;
    let ne10 = br(rd64(a, 0x10) != rd64(b, 0x10)) as u64;
    let total = ne0.wrapping_add(ne4).wrapping_add(ne10);
    let rcx = if br(rd32(a, 8) == rd32(b, 8)) { 0 } else { u64::MAX };
    br(total == rcx) as u8
}

/// parity: P1 @0x674a6 size=72
/// Widens edx pixels of 4 bytes to 4 words each, every byte shifted left by 2 (8-bit to 10-bit).
/// Does nothing for edx == 0.
pub fn f_000674a6(src: &[u8], dst: &mut [u8], edx: u32) {
    if br(edx == 0) {
        return;
    }
    let mut n = edx;
    let (mut s, mut d) = (0usize, 0usize);
    loop {
        for k in 0..4 {
            wr16(dst, d + 2 * k, (src[s + k] as u16) << 2);
        }
        s += 4;
        d += 8;
        n = n.wrapping_sub(1);
        if !br(n != 0) {
            break;
        }
    }
}
