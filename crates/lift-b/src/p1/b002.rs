//! Batch 002: field getters and setters, address computations and constant returns (10 to 14 bytes).
use crate::*;

/// parity: P1 @0x117de size=10
/// Getter: returns the qword at +0x20.
pub fn f_000117de(obj: &[u8]) -> u64 {
    rd64(obj, 0x20)
}

/// parity: P1 @0x1987a size=10
/// Returns rdi + 8 (address of the member at +8, no memory access).
pub fn f_0001987a(rdi: u64) -> u64 {
    rdi.wrapping_add(8)
}

/// parity: P1 @0x2eb3a size=10
/// Getter: returns the dword at +0x14.
pub fn f_0002eb3a(obj: &[u8]) -> u32 {
    rd32(obj, 0x14)
}

/// parity: P1 @0x2eb64 size=10
/// Getter: returns the dword at +0x20.
pub fn f_0002eb64(obj: &[u8]) -> u32 {
    rd32(obj, 0x20)
}

/// parity: P1 @0x2ebc8 size=10
/// Getter: returns the byte at +0x38 in al (upper bits of rax not set).
pub fn f_0002ebc8(obj: &[u8]) -> u8 {
    rd8(obj, 0x38)
}

/// parity: P1 @0x316b0 size=10
/// Setter: stores the low byte of esi (sil) at +8.
pub fn f_000316b0(obj: &mut [u8], esi: u32) {
    wr8(obj, 8, esi as u8);
}

/// parity: P1 @0xc3eb0 size=10
/// Getter: returns the dword at +0x18.
pub fn f_000c3eb0(obj: &[u8]) -> u32 {
    rd32(obj, 0x18)
}

/// parity: P1 @0xd4b18 size=10
/// Returns its first argument (`mov rax,rdi`).
pub fn f_000d4b18(rdi: u64) -> u64 {
    rdi
}

/// parity: P1 @0xe074e size=10
/// Returns rdi + 8 (address of the member at +8, no memory access).
pub fn f_000e074e(rdi: u64) -> u64 {
    rdi.wrapping_add(8)
}

/// parity: P1 @0x11912 size=12
/// Returns the constant 0x80000004.
pub fn f_00011912() -> u32 {
    0x8000_0004
}

/// parity: P1 @0x2dc1a size=12
/// Returns the constant 0x2c.
pub fn f_0002dc1a() -> u32 {
    0x2c
}

/// parity: P1 @0x2eb84 size=12
/// Getter: returns the f64 at +0x30 (movsd load, upper half of xmm0 zeroed).
pub fn f_0002eb84(obj: &[u8]) -> f64 {
    f64::from_bits(rd64(obj, 0x30))
}

/// parity: P1 @0x31904 size=12
/// Setter: stores the low word of esi at +0x20 and returns true (al = 1).
pub fn f_00031904(obj: &mut [u8], esi: u32) -> u8 {
    wr16(obj, 0x20, esi as u16);
    1
}

/// parity: P1 @0x55a14 size=12
/// Getter: returns the dword at +0x248.
pub fn f_00055a14(obj: &[u8]) -> u32 {
    rd32(obj, 0x248)
}

/// parity: P1 @0xb953c size=12
/// Returns the constant 0x14.
pub fn f_000b953c() -> u32 {
    0x14
}

/// parity: P1 @0xba082 size=12
/// Getter: returns the dword at +0x250.
pub fn f_000ba082(obj: &[u8]) -> u32 {
    rd32(obj, 0x250)
}

/// parity: P1 @0xba0b8 size=12
/// Getter: returns the dword at +0x274.
pub fn f_000ba0b8(obj: &[u8]) -> u32 {
    rd32(obj, 0x274)
}

/// parity: P1 @0xba116 size=12
/// Getter: returns the byte at +0x279 in al (upper bits of rax not set).
pub fn f_000ba116(obj: &[u8]) -> u8 {
    rd8(obj, 0x279)
}

/// parity: P1 @0xba14a size=12
/// Getter: returns the byte at +0x27a in al (upper bits of rax not set).
pub fn f_000ba14a(obj: &[u8]) -> u8 {
    rd8(obj, 0x27a)
}

/// parity: P1 @0xe48a4 size=12
/// Setter: stores esi at +0x28 and edx at +0x2c.
pub fn f_000e48a4(obj: &mut [u8], esi: u32, edx: u32) {
    wr32(obj, 0x28, esi);
    wr32(obj, 0x2c, edx);
}

/// parity: P1 @0x14d4e size=14
/// Returns rdi + 0x268 (address of the member at +0x268, no memory access).
pub fn f_00014d4e(rdi: u64) -> u64 {
    rdi.wrapping_add(0x268)
}

/// parity: P1 @0x14d6a size=14
/// Returns rdi + 0x298 (address of the member at +0x298, no memory access).
pub fn f_00014d6a(rdi: u64) -> u64 {
    rdi.wrapping_add(0x298)
}
