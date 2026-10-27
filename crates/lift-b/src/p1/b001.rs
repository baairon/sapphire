//! Batch 001: the 51 smallest group-B P1 candidates (empty functions, constant returns, identity).
//! Shapes: `push rbp; mov rbp,rsp; <body>; pop rbp; ret` with the body below.

/// parity: P1 @0xa1d66 size=6
/// No-op (empty body).
pub fn f_000a1d66() {}

/// parity: P1 @0xa1d72 size=6
/// No-op (empty body).
pub fn f_000a1d72() {}

/// parity: P1 @0xa1d7e size=6
/// No-op (empty body).
pub fn f_000a1d7e() {}

/// parity: P1 @0xa1d8a size=6
/// No-op (empty body).
pub fn f_000a1d8a() {}

/// parity: P1 @0xa1d96 size=6
/// No-op (empty body).
pub fn f_000a1d96() {}

/// parity: P1 @0xa1da2 size=6
/// No-op (empty body).
pub fn f_000a1da2() {}

/// parity: P1 @0xa1dae size=6
/// No-op (empty body).
pub fn f_000a1dae() {}

/// parity: P1 @0xa1dba size=6
/// No-op (empty body).
pub fn f_000a1dba() {}

/// parity: P1 @0xa1dc6 size=6
/// No-op (empty body).
pub fn f_000a1dc6() {}

/// parity: P1 @0xa1dd2 size=6
/// No-op (empty body).
pub fn f_000a1dd2() {}

/// parity: P1 @0xa1dde size=6
/// No-op (empty body).
pub fn f_000a1dde() {}

/// parity: P1 @0xa1dea size=6
/// No-op (empty body).
pub fn f_000a1dea() {}

/// parity: P1 @0xa1df6 size=6
/// No-op (empty body).
pub fn f_000a1df6() {}

/// parity: P1 @0xa1e02 size=6
/// No-op (empty body).
pub fn f_000a1e02() {}

/// parity: P1 @0xa1e0e size=6
/// No-op (empty body).
pub fn f_000a1e0e() {}

/// parity: P1 @0xa1e1a size=6
/// No-op (empty body).
pub fn f_000a1e1a() {}

/// parity: P1 @0xa1e26 size=6
/// No-op (empty body).
pub fn f_000a1e26() {}

/// parity: P1 @0xa1e32 size=6
/// No-op (empty body).
pub fn f_000a1e32() {}

/// parity: P1 @0xa1e3e size=6
/// No-op (empty body).
pub fn f_000a1e3e() {}

/// parity: P1 @0xa1e4a size=6
/// No-op (empty body).
pub fn f_000a1e4a() {}

/// parity: P1 @0xa1e56 size=6
/// No-op (empty body).
pub fn f_000a1e56() {}

/// parity: P1 @0xa1e62 size=6
/// No-op (empty body).
pub fn f_000a1e62() {}

/// parity: P1 @0xa1e6e size=6
/// No-op (empty body).
pub fn f_000a1e6e() {}

/// parity: P1 @0xd4ae8 size=6
/// No-op (empty body).
pub fn f_000d4ae8() {}

/// parity: P1 @0xd4aee size=6
/// No-op (empty body).
pub fn f_000d4aee() {}

/// parity: P1 @0x11734 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_00011734() -> u32 {
    0
}

/// parity: P1 @0x119c0 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000119c0() -> u32 {
    0
}

/// parity: P1 @0x2ebb8 size=8
/// Returns 1 in al (`mov al,1`; the upper bits of rax are not set).
pub fn f_0002ebb8() -> u8 {
    1
}

/// parity: P1 @0x2ebdc size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_0002ebdc() -> u32 {
    0
}

/// parity: P1 @0x4eeb4 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_0004eeb4() -> u32 {
    0
}

/// parity: P1 @0x559f4 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000559f4() -> u32 {
    0
}

/// parity: P1 @0x5c698 size=8
/// Returns its first argument (`mov eax,edi`).
pub fn f_0005c698(edi: u32) -> u32 {
    edi
}

/// parity: P1 @0x6590c size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_0006590c() -> u32 {
    0
}

/// parity: P1 @0xb9660 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000b9660() -> u32 {
    0
}

/// parity: P1 @0xb9670 size=8
/// Returns 1 in al (`mov al,1`; the upper bits of rax are not set).
pub fn f_000b9670() -> u8 {
    1
}

/// parity: P1 @0xcb923 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000cb923() -> u32 {
    0
}

/// parity: P1 @0xcb92b size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000cb92b() -> u32 {
    0
}

/// parity: P1 @0xcc27d size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000cc27d() -> u32 {
    0
}

/// parity: P1 @0xccb8e size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000ccb8e() -> u32 {
    0
}

/// parity: P1 @0xccb96 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000ccb96() -> u32 {
    0
}

/// parity: P1 @0xccc92 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000ccc92() -> u32 {
    0
}

/// parity: P1 @0xcd639 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000cd639() -> u32 {
    0
}

/// parity: P1 @0xce291 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000ce291() -> u32 {
    0
}

/// parity: P1 @0xd0134 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000d0134() -> u32 {
    0
}

/// parity: P1 @0xd4baa size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000d4baa() -> u32 {
    0
}

/// parity: P1 @0xd4bba size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000d4bba() -> u32 {
    0
}

/// parity: P1 @0xd4bca size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000d4bca() -> u32 {
    0
}

/// parity: P1 @0xd5e40 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000d5e40() -> u32 {
    0
}

/// parity: P1 @0xd5e50 size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000d5e50() -> u32 {
    0
}

/// parity: P1 @0xe0a2e size=8
/// Returns 0 (`xor eax,eax`).
pub fn f_000e0a2e() -> u32 {
    0
}

/// parity: P1 @0xe80f8 size=8
/// Returns 1 in al (`mov al,1`; the upper bits of rax are not set).
pub fn f_000e80f8() -> u8 {
    1
}
