//! Batch 005: bit-table predicates, set predicates, constructors, byte swapping, small fill and convert loops.
use crate::cov::br;
use crate::*;

/// parity: P1 @0x5cb0c size=31
/// Bit table lookup: for edi-2 in 0..=0x16 returns bit (edi-2) of 0x2393c8, otherwise true.
pub fn f_0005cb0c(edi: u32) -> u8 {
    let n = edi.wrapping_sub(2);
    if br(n <= 0x16) {
        ((0x2393c8u32 >> (n & 31)) & 1) as u8
    } else {
        1
    }
}

/// parity: P1 @0xa1e74 size=32
/// test: addr=0
/// Constructor-style: zeroes the three qwords at +0, +8, +0x10 and returns the object pointer (rdi).
pub fn f_000a1e74(rdi: u64, obj: &mut [u8]) -> u64 {
    wr64(obj, 0x10, 0);
    wr64(obj, 8, 0);
    wr64(obj, 0, 0);
    rdi
}

/// parity: P1 @0xe2c06 size=32
/// Packs three small coordinates into a flat index: 0 if ecx > 3, esi > 1 or edx > 7 (unsigned), otherwise
/// 3 * (ecx + 4 * (edx + 4 * esi)).
pub fn f_000e2c06(esi: u32, edx: u32, ecx: u32) -> u32 {
    if br(ecx > 3) || br(esi > 1) || br(edx > 7) {
        return 0;
    }
    let a = edx.wrapping_add(esi.wrapping_mul(4));
    let b = ecx.wrapping_add(a.wrapping_mul(4));
    b.wrapping_add(b.wrapping_mul(2))
}

/// parity: P1 @0xcc89b size=34
/// Predicate: edi in {0x10518400, 0x10538200, 0x10565400}.
pub fn f_000cc89b(edi: u32) -> u8 {
    if br(edi == 0x1051_8400) || br(edi == 0x1053_8200) || br(edi == 0x1056_5400) {
        1
    } else {
        0
    }
}

/// parity: P1 @0xd113f size=34
/// Predicate: edi in {0x10280300, 0x10478300, 0x10478350}.
pub fn f_000d113f(edi: u32) -> u8 {
    if br(edi == 0x1028_0300) || br(edi == 0x1047_8300) || br(edi == 0x1047_8350) {
        1
    } else {
        0
    }
}

/// parity: P1 @0x5895f size=35
/// Converts 2*edx 16-bit samples to bytes: dst[i] = (src16[i] >> 2) as u8. Does nothing when the low 31
/// bits of edx are zero. (Looks like 10-bit to 8-bit sample conversion; inferred, not established.)
pub fn f_0005895f(src: &[u8], dst: &mut [u8], edx: u32) {
    if br(edx & 0x7fff_ffff == 0) {
        return;
    }
    let mut n = edx.wrapping_add(edx);
    let mut i = 0usize;
    loop {
        let v = rd16(src, 2 * i) as u32;
        dst[i] = (v >> 2) as u8;
        i += 1;
        n = n.wrapping_sub(1);
        if !br(n != 0) {
            break;
        }
    }
}

/// parity: P1 @0x5934f size=35
/// Fills (si) dwords at obj[0], obj[4], ... with 0x03ac0200, where si is the low word of esi.
pub fn f_0005934f(obj: &mut [u8], esi: u32) {
    let si = esi as u16;
    if br(si == 0) {
        return;
    }
    let end = 2 * si as i64;
    let mut rcx: i64 = 0;
    loop {
        wr32(obj, (rcx * 2) as usize, 0x03ac_0200);
        rcx += 2;
        if !br(rcx < end) {
            break;
        }
    }
}

/// parity: P1 @0x25e54 size=36
/// Zeroes the first 0x1c bytes (same code as f_00025e30).
pub fn f_00025e54(obj: &mut [u8]) {
    wr32(obj, 0x18, 0);
    wr64(obj, 0x10, 0);
    wr64(obj, 8, 0);
    wr64(obj, 0, 0);
}

/// parity: P1 @0x25fe4 size=36
/// Zeroes the first 0x1c bytes (same code as f_00025e30).
pub fn f_00025fe4(obj: &mut [u8]) {
    wr32(obj, 0x18, 0);
    wr64(obj, 0x10, 0);
    wr64(obj, 8, 0);
    wr64(obj, 0, 0);
}

/// parity: P1 @0xd3142 size=39
/// test: opt=0
/// Byte-swaps (in place) the three dwords at +0, +4, +8 and returns true; returns false for a null pointer.
/// The null branch cannot be exercised by the oracle (pointer arguments are always mapped buffers).
pub fn f_000d3142(obj: Option<&mut [u8]>) -> u8 {
    if br(obj.is_none()) {
        return 0;
    }
    let o = obj.unwrap();
    for off in [0usize, 4, 8] {
        let v = rd32(o, off).swap_bytes();
        wr32(o, off, v);
    }
    1
}

/// parity: P1 @0x2abce size=40
/// The count at +0 is stored first; counts of 0 or 1 then clear the first 16 bytes, larger counts store
/// edx, ecx, r8d at +4, +8, +0xc.
pub fn f_0002abce(obj: &mut [u8], esi: u32, edx: u32, ecx: u32, r8d: u32) {
    wr32(obj, 0, esi);
    if br(esi <= 1) {
        wr64(obj, 8, 0);
        wr64(obj, 0, 0);
    } else {
        wr32(obj, 4, edx);
        wr32(obj, 8, ecx);
        wr32(obj, 0xc, r8d);
    }
}

/// parity: P1 @0x2abf6 size=40
/// Same code as f_0002abce.
pub fn f_0002abf6(obj: &mut [u8], esi: u32, edx: u32, ecx: u32, r8d: u32) {
    wr32(obj, 0, esi);
    if br(esi <= 1) {
        wr64(obj, 8, 0);
        wr64(obj, 0, 0);
    } else {
        wr32(obj, 4, edx);
        wr32(obj, 8, ecx);
        wr32(obj, 0xc, r8d);
    }
}

/// parity: P1 @0x5b426 size=40
/// Selects one of two dwords at +0 and +4: for esi in {0, 1} returns the dword at +0 when
/// (flag byte at +8 != 0) == (esi == 0), else the dword at +4; returns 0 for esi > 1.
pub fn f_0005b426(obj: &[u8], esi: u32) -> u32 {
    if br(esi > 1) {
        return 0;
    }
    let flag = rd8(obj, 8) != 0;
    let first = if br(esi == 0) { flag } else { !flag };
    rd32(obj, if first { 0 } else { 4 })
}

/// parity: P1 @0xe4bac size=40
/// Clears the qword at +0x90, the bytes at +0x8c, +0xc, +0xd and the dword at +8.
pub fn f_000e4bac(obj: &mut [u8]) {
    wr64(obj, 0x90, 0);
    wr8(obj, 0x8c, 0);
    wr8(obj, 0xc, 0);
    wr8(obj, 0xd, 0);
    wr32(obj, 8, 0);
}

/// parity: P1 @0xcf778 size=41
/// Predicate (zero-extended to 32 bits): edi in {0x10352300, 0x10266401, 0x10266400}.
pub fn f_000cf778(edi: u32) -> u32 {
    if br(edi == 0x1035_2300) || br(edi == 0x1026_6401) || br(edi == 0x1026_6400) {
        1
    } else {
        0
    }
}

/// parity: P1 @0x2bfa2 size=42
/// Treats the qwords at s[0] (begin) and s[8] (cursor) as numbers: when they differ, advances the cursor
/// to cursor + (!(cursor - 0x10 - begin) & !0xf). Returns true.
pub fn f_0002bfa2(s: &mut [u8]) -> u8 {
    let begin = rd64(s, 0);
    let cur = rd64(s, 8);
    if br(cur != begin) {
        let d = cur.wrapping_sub(0x10).wrapping_sub(begin);
        let r = (!d & 0xffff_ffff_ffff_fff0).wrapping_add(cur);
        wr64(s, 8, r);
    }
    1
}

/// parity: P1 @0xd3169 size=42
/// test: opt=0
/// Like f_000d3142 (byte-swaps the dwords at +0, +4, +8 and returns true) but only when esi >= 0x40;
/// returns false for a null pointer or esi < 0x40. The null branch cannot be exercised by the oracle.
pub fn f_000d3169(obj: Option<&mut [u8]>, esi: u32) -> u8 {
    if br(obj.is_none()) {
        return 0;
    }
    if br(esi < 0x40) {
        return 0;
    }
    let o = obj.unwrap();
    for off in [0usize, 4, 8] {
        let v = rd32(o, off).swap_bytes();
        wr32(o, off, v);
    }
    1
}
