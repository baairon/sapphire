//! Batch 004: predicates over constants, clamping selects, std::string::data() style accessor, small copies.
use crate::cov::br;
use crate::*;

/// parity: P1 @0xd0e8c size=20
/// Returns 2 when edi == 0x10646701, else 0.
pub fn f_000d0e8c(edi: u32) -> u32 {
    2 * (edi == 0x1064_6701) as u32
}

/// parity: P1 @0x2aba2 size=22
/// Zeroes the qwords at +0 and +8.
pub fn f_0002aba2(obj: &mut [u8]) {
    wr64(obj, 8, 0);
    wr64(obj, 0, 0);
}

/// parity: P1 @0x2eb4e size=22
/// Returns the qword at +8 plus esi * dword at +0x18, both factors sign-extended to 64 bits
/// (base + row * stride style address computation, no memory access beyond the two fields).
pub fn f_0002eb4e(obj: &[u8], esi: u32) -> u64 {
    let stride = rd32(obj, 0x18) as i32 as i64;
    let row = esi as i32 as i64;
    (row.wrapping_mul(stride)).wrapping_add(rd64(obj, 8) as i64) as u64
}

/// parity: P1 @0xd5d2a size=22
/// Returns 0 (`xor eax,eax`, followed by nop padding).
pub fn f_000d5d2a() -> u32 {
    0
}

/// parity: P1 @0xe55f2 size=22
/// Predicate on a mode value: esi (or the dword at +8 when esi is 0) with bit 2 cleared equals 2,
/// that is the value is 2 or 6.
pub fn f_000e55f2(obj: &[u8], esi: u32) -> u8 {
    let mut e = esi;
    if br(e == 0) {
        e = rd32(obj, 8);
    }
    ((e & !4) == 2) as u8
}

/// parity: P1 @0xb8d54 size=23
/// Sets the qwords at out[8] and out[0] to all ones and returns 0.
pub fn f_000b8d54(out: &mut [u8]) -> u32 {
    wr64(out, 8, u64::MAX);
    wr64(out, 0, u64::MAX);
    0
}

/// parity: P1 @0x2b6a4 size=24
/// Setter with range check: if esi <= 0x1f (unsigned) stores it at +0x248 and returns true, else false.
pub fn f_0002b6a4(obj: &mut [u8], esi: u32) -> u8 {
    if br(esi <= 0x1f) {
        wr32(obj, 0x248, esi);
        1
    } else {
        0
    }
}

/// parity: P1 @0xb9680 size=24
/// Getter: returns the byte at +0xa in al (upper bits of rax not set).
pub fn f_000b9680(obj: &[u8]) -> u8 {
    rd8(obj, 0xa)
}

/// parity: P1 @0x2f014 size=26
/// Chooses between 0x1000 and edi: edi when esi >= 0x20 or edi >= 0x20001 (unsigned), else 0x1000.
pub fn f_0002f014(edi: u32, esi: u32) -> u32 {
    let mut eax = 0x1000;
    if br(esi >= 0x20) {
        eax = edi;
    }
    if br(edi >= 0x20001) {
        eax = edi;
    }
    eax
}

/// parity: P1 @0xcb56a size=26
/// Predicate: edi is 0x10294700 or 0x10322950.
pub fn f_000cb56a(edi: u32) -> u8 {
    if br(edi == 0x1029_4700) || br(edi == 0x1032_2950) {
        1
    } else {
        0
    }
}

/// parity: P1 @0xcd01e size=26
/// Predicate: edi is 0x10538200 or 0x10565400.
pub fn f_000cd01e(edi: u32) -> u8 {
    if br(edi == 0x1053_8200) || br(edi == 0x1056_5400) {
        1
    } else {
        0
    }
}

/// parity: P1 @0x2ebe4 size=28
/// test: addr=0
/// Data pointer of the std::string-like member at +0x28 (libc++ layout, inferred from the access pattern,
/// confidence med): if bit 0 of the byte at +0x28 is set the pointer stored at +0x38 is returned,
/// otherwise the inline buffer at +0x29. `rdi` is the address of the object.
pub fn f_0002ebe4(rdi: u64, obj: &[u8]) -> u64 {
    if br(rd8(obj, 0x28) & 1 != 0) {
        rd64(obj, 0x38)
    } else {
        rdi.wrapping_add(0x28).wrapping_add(1)
    }
}

/// parity: P1 @0x516d0 size=28
/// Copies the 0x28 bytes at src[0..0x28] to dst[0..0x28] (movups, qword, movups; rax/xmm0 are scratch).
pub fn f_000516d0(src: &[u8], dst: &mut [u8]) {
    dst[0..0x28].copy_from_slice(&src[0..0x28]);
}

/// parity: P1 @0xe62be size=28
/// Copies the qword at +0x80 to out[0] and the dword at +0x88 to out[8]; returns the byte at +0xc in al.
/// (The upper bits of rax are residue of the qword load and are not part of the result.)
pub fn f_000e62be(obj: &[u8], out: &mut [u8]) -> u8 {
    wr32(out, 8, rd32(obj, 0x88));
    wr64(out, 0, rd64(obj, 0x80));
    rd8(obj, 0xc)
}

/// parity: P1 @0xcb89b size=29
/// Predicate: edi in {0x10266400, 0x10266401, 0x10352300}.
pub fn f_000cb89b(edi: u32) -> u8 {
    if br(edi.wrapping_sub(0x1026_6400) < 2) || br(edi == 0x1035_2300) {
        1
    } else {
        0
    }
}

/// parity: P1 @0xce375 size=29
/// Predicate: edi in {0x10646705, 0x10646706, 0x10710851}.
pub fn f_000ce375(edi: u32) -> u8 {
    if br(edi.wrapping_sub(0x1064_6705) < 2) || br(edi == 0x1071_0851) {
        1
    } else {
        0
    }
}

/// parity: P1 @0xce392 size=29
/// Predicate: edi in {0x10710850, 0x10710851, 0x10710800}.
pub fn f_000ce392(edi: u32) -> u8 {
    if br(edi.wrapping_sub(0x1071_0850) < 2) || br(edi == 0x1071_0800) {
        1
    } else {
        0
    }
}

/// parity: P1 @0xce3be size=29
/// Predicate: edi in {0x10710850, 0x10710851, 0x10710800} (identical code to f_000ce392).
pub fn f_000ce3be(edi: u32) -> u8 {
    if br(edi.wrapping_sub(0x1071_0850) < 2) || br(edi == 0x1071_0800) {
        1
    } else {
        0
    }
}

/// parity: P1 @0xd1161 size=29
/// Predicate: edi in {0x10710850, 0x10710851, 0x10710800} (identical code to f_000ce392).
pub fn f_000d1161(edi: u32) -> u8 {
    if br(edi.wrapping_sub(0x1071_0850) < 2) || br(edi == 0x1071_0800) {
        1
    } else {
        0
    }
}

/// parity: P1 @0x2b73c size=30
/// Clears the qwords at +0x210 and +0x208 and returns true.
pub fn f_0002b73c(obj: &mut [u8]) -> u8 {
    wr64(obj, 0x210, 0);
    wr64(obj, 0x208, 0);
    1
}

/// parity: P1 @0x5cacf size=30
/// Bit table lookup: for edi-1 in 0..=10 returns bit (edi-1) of 0x7a9, else 0.
pub fn f_0005cacf(edi: u32) -> u32 {
    let n = edi.wrapping_sub(1);
    if br(n <= 10) {
        (0x7a9u32 >> (n & 31)) & 1
    } else {
        0
    }
}

/// parity: P1 @0xe5a70 size=30
/// Predicate: the mode value (esi, or the dword at +8 when esi is 0) is 4 or 8.
pub fn f_000e5a70(obj: &[u8], esi: u32) -> u8 {
    let mut e = esi;
    if br(e == 0) {
        e = rd32(obj, 8);
    }
    if br(e == 4) || br(e == 8) {
        1
    } else {
        0
    }
}

/// parity: P1 @0x2f02e size=31
/// Returns 0xdc when esi >= 0x20 or edi >= 0x20001 (unsigned), else 0x21.
pub fn f_0002f02e(edi: u32, esi: u32) -> u32 {
    let mut eax = 0x21;
    if br(esi >= 0x20) {
        eax = 0xdc;
    }
    if br(edi >= 0x20001) {
        eax = 0xdc;
    }
    eax
}

/// parity: P1 @0x5c65d size=31
/// Bit table lookup: for edi-2 in 0..=15 returns bit (edi-2) of 0xf0c3, else 0.
pub fn f_0005c65d(edi: u32) -> u32 {
    let n = edi.wrapping_sub(2);
    if br(n <= 0xf) {
        (0xf0c3u32 >> (n & 31)) & 1
    } else {
        0
    }
}
