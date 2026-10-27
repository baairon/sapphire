//! Batch 003: small getters and setters, constant predicates, bounds-checked setters, indexed slots at +0x168.
use crate::cov::br;
use crate::*;

/// parity: P1 @0x14d86 size=14
/// Returns rdi + 0x2c8 (address of the member at +0x2c8, no memory access).
pub fn f_00014d86(rdi: u64) -> u64 {
    rdi.wrapping_add(0x2c8)
}

/// parity: P1 @0xba09e size=14
/// Getter: returns the word at +0x270, zero-extended.
pub fn f_000ba09e(obj: &[u8]) -> u32 {
    rd16(obj, 0x270) as u32
}

/// parity: P1 @0xba0de size=14
/// Returns rdi + 0x258 (address of the member at +0x258, no memory access).
pub fn f_000ba0de(rdi: u64) -> u64 {
    rdi.wrapping_add(0x258)
}

/// parity: P1 @0xba0fa size=14
/// Getter: returns the qword at +0x280.
pub fn f_000ba0fa(obj: &[u8]) -> u64 {
    rd64(obj, 0x280)
}

/// parity: P1 @0xba122 size=14
/// Setter: stores the low byte of esi at +0x27b.
pub fn f_000ba122(obj: &mut [u8], esi: u32) {
    wr8(obj, 0x27b, esi as u8);
}

/// parity: P1 @0xe55b2 size=14
/// Copies the dword at +0x54 to out[0] and returns the byte at +0xc in al. (The upper bits of eax are
/// residue of the dword load and are not part of the result.)
pub fn f_000e55b2(obj: &[u8], out: &mut [u8]) -> u8 {
    wr32(out, 0, rd32(obj, 0x54));
    rd8(obj, 0xc)
}

/// parity: P1 @0xe55ce size=14
/// Copies the dword at +0x5c to out[0] and returns the byte at +0xc in al. (The upper bits of eax are
/// residue of the dword load and are not part of the result.)
pub fn f_000e55ce(obj: &[u8], out: &mut [u8]) -> u8 {
    wr32(out, 0, rd32(obj, 0x5c));
    rd8(obj, 0xc)
}

/// parity: P1 @0xe5fca size=14
/// Getter: returns the byte at +0x83, zero-extended.
pub fn f_000e5fca(obj: &[u8]) -> u32 {
    rd8(obj, 0x83) as u32
}

/// parity: P1 @0xe5fe6 size=14
/// Getter: returns the byte at +0x80, zero-extended.
pub fn f_000e5fe6(obj: &[u8]) -> u32 {
    rd8(obj, 0x80) as u32
}

/// parity: P1 @0xcb55b size=15
/// Predicate: edi == 0x10294700.
pub fn f_000cb55b(edi: u32) -> u8 {
    (edi == 0x1029_4700) as u8
}

/// parity: P1 @0xcd8b2 size=15
/// Predicate: edi == 0x10402100.
pub fn f_000cd8b2(edi: u32) -> u8 {
    (edi == 0x1040_2100) as u8
}

/// parity: P1 @0xce282 size=15
/// Predicate: edi == 0x10634500.
pub fn f_000ce282(edi: u32) -> u8 {
    (edi == 0x1063_4500) as u8
}

/// parity: P1 @0xce3af size=15
/// Predicate: edi == 0x10710800.
pub fn f_000ce3af(edi: u32) -> u8 {
    (edi == 0x1071_0800) as u8
}

/// parity: P1 @0x11a02 size=16
/// Zeroes the qword at out[0] and returns 0 (eax).
pub fn f_00011a02(out: &mut [u8]) -> u32 {
    wr64(out, 0, 0);
    0
}

/// parity: P1 @0xba08e size=16
/// Setter: stores esi & 7 as a word at +0x270.
pub fn f_000ba08e(obj: &mut [u8], esi: u32) {
    wr16(obj, 0x270, (esi & 7) as u16);
}

/// parity: P1 @0xd5bb0 size=16
/// Increments (wrapping) the dword slot [index] of the array at +0x168; index = esi, zero-extended.
pub fn f_000d5bb0(obj: &mut [u8], esi: u32) {
    let o = 0x168 + 4 * esi as usize;
    wr32(obj, o, rd32(obj, o).wrapping_add(1));
}

/// parity: P1 @0x6818a size=18
/// Stores edx into dword slot [esi] of the array at +0x168 and returns true.
pub fn f_0006818a(obj: &mut [u8], esi: u32, edx: u32) -> u8 {
    wr32(obj, 0x168 + 4 * esi as usize, edx);
    1
}

/// parity: P1 @0xcdbbf size=18
/// Predicate: edi in 0x10646700..=0x10646706 (unsigned `edi - 0x10646700 < 7`).
pub fn f_000cdbbf(edi: u32) -> u8 {
    (edi.wrapping_add(0xef9b_9900) < 7) as u8
}

/// parity: P1 @0xce363 size=18
/// Predicate: edi in 0x10646702..=0x10646704 (unsigned `edi - 0x10646702 < 3`).
pub fn f_000ce363(edi: u32) -> u8 {
    (edi.wrapping_add(0xef9b_98fe) < 3) as u8
}

/// parity: P1 @0xd5e2e size=18
/// Returns 0 (`xor eax,eax`, followed by 10 bytes of nop padding).
pub fn f_000d5e2e() -> u32 {
    0
}

/// parity: P1 @0x28c1e size=20
/// Stores the constant 0x6e74763200000000 at obj[0] (high dword 0x6e747632 is ASCII "ntv2" read as
/// big-endian text, low dword 0; meaning not established). Returns nothing (rax is scratch).
pub fn f_00028c1e(obj: &mut [u8]) {
    wr64(obj, 0, 0x6e74_7632_0000_0000);
}

/// parity: P1 @0x3188c size=20
/// Setter with range check: if esi <= 2 (unsigned) stores it at +8 and returns true, else returns false.
pub fn f_0003188c(obj: &mut [u8], esi: u32) -> u8 {
    if br(esi <= 2) {
        wr32(obj, 8, esi);
        1
    } else {
        0
    }
}

/// parity: P1 @0x318b4 size=20
/// Setter with range check: if esi <= 2 (unsigned) stores it at +0x10 and returns true, else returns false.
pub fn f_000318b4(obj: &mut [u8], esi: u32) -> u8 {
    if br(esi <= 2) {
        wr32(obj, 0x10, esi);
        1
    } else {
        0
    }
}

/// parity: P1 @0x318dc size=20
/// Setter with range check: if esi <= 1 (unsigned) stores it at +0x18 and returns true, else returns false.
pub fn f_000318dc(obj: &mut [u8], esi: u32) -> u8 {
    if br(esi <= 1) {
        wr32(obj, 0x18, esi);
        1
    } else {
        0
    }
}

/// parity: P1 @0x46cd4 size=20
/// Clears the qword at +0x228 and returns true.
pub fn f_00046cd4(obj: &mut [u8]) -> u8 {
    wr64(obj, 0x228, 0);
    1
}

/// parity: P1 @0x681e0 size=20
/// Copies dword slot [esi] of the array at +0x168 to out[0] and returns true. (The upper bits of eax are
/// residue of the load and are not part of the result.)
pub fn f_000681e0(obj: &[u8], esi: u32, out: &mut [u8]) -> u8 {
    wr32(out, 0, rd32(obj, 0x168 + 4 * esi as usize));
    1
}

/// parity: P1 @0xd4afe size=26
/// If the low byte of esi is non-zero, clears dword slot [edx] of the array at +0x168. Returns true.
pub fn f_000d4afe(obj: &mut [u8], esi: u32, edx: u32) -> u8 {
    if br(esi as u8 != 0) {
        wr32(obj, 0x168 + 4 * edx as usize, 0);
    }
    1
}

/// parity: P1 @0x522e2 size=14
/// Zero-extends sil and tail-jumps to f_000d4afe (same semantics with the flag reduced to its low byte).
pub fn f_000522e2(obj: &mut [u8], esi: u32, edx: u32) -> u8 {
    f_000d4afe(obj, esi & 0xff, edx)
}
