//! Batch 020: composite format predicates built on the classifiers of b010, b017 and b019.
use crate::cov::br;
use crate::p1::{b010, b017, b019};
use crate::*;

/// parity: P1 @0xb794e size=32
/// True when (f_000d0ac8(dword at +0x10) & 0xffff) <= esi, unsigned.
pub fn f_000b794e(obj: &[u8], esi: u32) -> u8 {
    br((b019::f_000d0ac8(rd32(obj, 0x10)) & 0xffff) <= esi) as u8
}

/// parity: P1 @0xd1239 size=187
/// True when the low 16 bits of f_000d0ac8(edi) plus a per-format addend are non-zero. Addends: 10 for
/// {0x10266400, 0x10266401}; 16 for 0x10710800; 8 for {0x10668200, 0x10478300, 0x10478350, 0x10710850,
/// 0x10710851, 0x10378800, 0x10280300}; 2 for 0x10352300; 0 otherwise.
pub fn f_000d1239(edi: u32) -> u8 {
    let extra: u16 = match edi {
        0x1026_6400 | 0x1026_6401 => 10,
        0x1071_0800 => 16,
        0x1066_8200 | 0x1047_8300 | 0x1047_8350 | 0x1071_0850 | 0x1071_0851 | 0x1037_8800 | 0x1028_0300 => 8,
        0x1035_2300 => 2,
        _ => 0,
    };
    br((b019::f_000d0ac8(edi) as u16).wrapping_add(extra) != 0) as u8
}

/// parity: P1 @0xd117e size=187
/// True when the low 16 bits of f_000d01a9(edi) + f_000d0c6c(edi) plus a per-format addend are non-zero.
/// Addends: 8 for {0x10668200, 0x10710800, 0x10710850, 0x10710851, 0x10478300, 0x10478350, 0x10378800};
/// 2 for {0x10280300, 0x10352300, 0x10266400, 0x10266401}; 0 otherwise.
pub fn f_000d117e(edi: u32) -> u8 {
    let extra: u16 = match edi {
        0x1066_8200 | 0x1071_0800 | 0x1071_0850 | 0x1071_0851 | 0x1047_8300 | 0x1047_8350 | 0x1037_8800 => 8,
        0x1028_0300 | 0x1035_2300 | 0x1026_6400 | 0x1026_6401 => 2,
        _ => 0,
    };
    let sum = (b010::f_000d01a9(edi) as u16).wrapping_add(b019::f_000d0c6c(edi) as u16).wrapping_add(extra);
    br(sum != 0) as u8
}

/// parity: P1 @0xd1f0a size=415
/// f_000d19e0(edi) - (f_000cf820(edi) & 0xffff), 32-bit wrapping (the decision tree is the same as f_000d19e0's).
pub fn f_000d1f0a(edi: u32) -> u32 {
    b017::f_000d19e0(edi).wrapping_sub(b010::f_000cf820(edi) & 0xffff)
}

/// parity: P1 @0xd2ce8 size=468
/// Level check against channel counts from four classifiers: A = f_000d0ea0, B = f_000d0fb9, C = f_000d0ac8,
/// D = f_000d0c6c (each & 0xffff), cx = max(C, D), r12 = max(A, B). For esi in 0x13..=0x1a the format must be in
/// {0x10478300, 0x10478350, 0x10518400, 0x10518450, 0x10538200, 0x10565400, 0x10634500, 0x10646700..=0x10646706,
/// 0x10710800, 0x10710850, 0x10710851}; esi > 0x1a is false; otherwise by esi (jump table at 0xd2e50): 0 true;
/// 1, 5, 19 cx != 0; 2, 6, 20 cx > 1; 3, 13, 21 cx > 2; 4, 14, 22 cx > 3; 9, 15, 23 cx > 4; 10..=12, 16..=18,
/// 24..=26 cx > 5; 7 r12 != 0; 8 r12 > 1.
pub fn f_000d2ce8(edi: u32, esi: u32) -> u8 {
    let a = (b010::f_000d0ea0(edi) & 0xffff) as u16;
    let b = (b010::f_000d0fb9(edi) & 0xffff) as u16;
    let r12 = a.max(b);
    let c = (b019::f_000d0ac8(edi) & 0xffff) as u16;
    let d = (b019::f_000d0c6c(edi) & 0xffff) as u16;
    let cx = c.max(d);
    if br(esi.wrapping_sub(0x13) <= 7) {
        let in_set = matches!(edi, 0x1047_8300 | 0x1047_8350 | 0x1051_8400 | 0x1051_8450 | 0x1053_8200 | 0x1056_5400
            | 0x1063_4500 | 0x1064_6700..=0x1064_6706 | 0x1071_0800 | 0x1071_0850 | 0x1071_0851);
        if br(!in_set) {
            return 0;
        }
    }
    if br(esi > 0x1a) {
        return 0;
    }
    match esi {
        0 => 1,
        1 | 5 | 19 => (cx != 0) as u8,
        2 | 6 | 20 => (cx > 1) as u8,
        3 | 13 | 21 => (cx > 2) as u8,
        4 | 14 | 22 => (cx > 3) as u8,
        9 | 15 | 23 => (cx > 4) as u8,
        10 | 11 | 12 | 16 | 17 | 18 | 24 | 25 | 26 => (cx > 5) as u8,
        7 => (r12 != 0) as u8,
        _ => (r12 > 1) as u8,
    }
}
