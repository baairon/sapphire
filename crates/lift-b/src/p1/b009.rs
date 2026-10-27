//! Batch 009: an initialiser with computed fields, a jump-table switch, and the multi-level set predicates /
//! classifiers over the constant family 0x10266400..0x10710851.
use crate::cov::{br, hit};
use crate::*;

/// parity: P1 @0x32dec size=84
/// Initialiser: dwords esi, edx, ecx, r8d at +0, +4, +8, +0xc; +0x10 = 0xc, +0x14 = 0, +0x18 = 0x20, +0x1c = 3;
/// byte +0x20 = 0; dword +0x24 = ecx << 2; qword +0x28 = 0; dword +0x30 = 0; word +0x34 = 1; dword +0x38 = 0x12.
pub fn f_00032dec(obj: &mut [u8], esi: u32, edx: u32, ecx: u32, r8d: u32) {
    wr32(obj, 0, esi);
    wr32(obj, 4, edx);
    wr32(obj, 8, ecx);
    wr32(obj, 0xc, r8d);
    wr32(obj, 0x10, 0xc);
    wr32(obj, 0x14, 0);
    wr32(obj, 0x18, 0x20);
    wr32(obj, 0x1c, 3);
    wr8(obj, 0x20, 0);
    wr16(obj, 0x34, 1);
    wr32(obj, 0x38, 0x12);
    wr32(obj, 0x24, ecx.wrapping_shl(2));
    wr32(obj, 0x30, 0);
    wr64(obj, 0x28, 0);
}

/// parity: P1 @0x310b2 size=86
/// Switch on esi (jump table at 0x310ec): 0 -> 0x80000, 1 -> 0xc00000, 2..=5 -> 0x40000, 6 -> the dword at
/// +0x10, anything above 6 -> 0.
pub fn f_000310b2(obj: &[u8], esi: u32) -> u32 {
    match esi {
        0 => {
            hit();
            0x80000
        }
        1 => {
            hit();
            0xc00000
        }
        2..=5 => {
            hit();
            0x40000
        }
        6 => {
            hit();
            rd32(obj, 0x10)
        }
        _ => {
            hit();
            0
        }
    }
}

/// parity: P1 @0xcdaff size=88
/// Set predicate: edi in {0x10294700, 0x10322950, 0x10352300, 0x10378800, 0x10402100, 0x10416000}
/// (signed splits at 0x1032294f, 0x103522ff and 0x104020ff).
pub fn f_000cdaff(edi: u32) -> u8 {
    let s = edi as i32;
    if br(s > 0x1032_294f) {
        if br(s > 0x1035_22ff) {
            if br(s > 0x1040_20ff) {
                (br(edi == 0x1040_2100) || br(edi == 0x1041_6000)) as u8
            } else {
                (br(edi == 0x1035_2300) || br(edi == 0x1037_8800)) as u8
            }
        } else {
            br(edi == 0x1032_2950) as u8
        }
    } else {
        br(edi == 0x1029_4700) as u8
    }
}

/// parity: P1 @0xcb6a1 size=89
/// Set predicate: edi in {0x10478300, 0x10518400, 0x10538200, 0x10565400, 0x10634500, 0x10710800, 0x10710850,
/// 0x10710851} (signed splits at 0x106344ff and 0x105381ff).
pub fn f_000cb6a1(edi: u32) -> u8 {
    let s = edi as i32;
    if br(s > 0x1063_44ff) {
        (br(edi.wrapping_sub(0x1071_0850) < 2) || br(edi == 0x1063_4500) || br(edi == 0x1071_0800)) as u8
    } else if br(s > 0x1053_81ff) {
        (br(edi == 0x1053_8200) || br(edi == 0x1056_5400)) as u8
    } else {
        (br(edi == 0x1047_8300) || br(edi == 0x1051_8400)) as u8
    }
}

/// parity: P1 @0xcf682 size=89
/// Classifier returning 2 for edi in {0x10266400, 0x10266401, 0x10352300}, 8 for {0x10710800, 0x10710850,
/// 0x10710851} and 0 otherwise (signed splits at 0x103522ff and 0x1071084f).
pub fn f_000cf682(edi: u32) -> u32 {
    let s = edi as i32;
    if br(s > 0x1035_22ff) {
        if br(s > 0x1071_084f) {
            if br(edi == 0x1071_0850) || br(edi == 0x1071_0851) {
                8
            } else {
                0
            }
        } else if br(edi == 0x1035_2300) {
            2
        } else if br(edi == 0x1071_0800) {
            8
        } else {
            0
        }
    } else if br(edi == 0x1026_6400) || br(edi == 0x1026_6401) {
        2
    } else {
        0
    }
}
