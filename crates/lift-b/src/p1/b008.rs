//! Batch 008: black/white fill patterns, a 10-bit repack, nested signed-split set predicates, initialisers
//! (including two HDR mastering-display style constant tables).
use crate::cov::br;
use crate::*;

/// parity: P1 @0x594ab size=75
/// Fills (si) byte pairs with (0x80, 0x10) when edx == 1, else (0x10, 0x80); si is the low word of esi.
/// (Looks like a black line in 4:2:2 chroma-first / luma-first byte orders; inferred.)
pub fn f_000594ab(obj: &mut [u8], esi: u32, edx: u32) {
    let (a, b) = if br(edx == 1) { (0x80u8, 0x10u8) } else { (0x10u8, 0x80u8) };
    let si = esi as u16;
    if br(si == 0) {
        return;
    }
    let end = 2 * si as i64;
    let mut rcx = 0i64;
    loop {
        obj[rcx as usize] = a;
        obj[rcx as usize + 1] = b;
        rcx += 2;
        if !br(rcx < end) {
            break;
        }
    }
}

/// parity: P1 @0x594f6 size=75
/// Fills (si) byte pairs with (0x80, 0xeb) when edx == 1, else (0xeb, 0x80); si is the low word of esi.
/// (Same shape as f_000594ab with the 235 white level; inferred.)
pub fn f_000594f6(obj: &mut [u8], esi: u32, edx: u32) {
    let (a, b) = if br(edx == 1) { (0x80u8, 0xebu8) } else { (0xebu8, 0x80u8) };
    let si = esi as u16;
    if br(si == 0) {
        return;
    }
    let end = 2 * si as i64;
    let mut rcx = 0i64;
    loop {
        obj[rcx as usize] = a;
        obj[rcx as usize + 1] = b;
        rcx += 2;
        if !br(rcx < end) {
            break;
        }
    }
}

/// parity: P1 @0xc7c31 size=78
/// In place over dwords (loop counter is 16 bit, compared with esi): v -> ((v >> 22) & 0xff) | (v << 26) |
/// ((v << 10) & 0xff0000) | ((v >> 6) & 0xff00). Does nothing for esi == 0.
pub fn f_000c7c31(obj: &mut [u8], esi: u32) {
    if br(esi == 0) {
        return;
    }
    let mut eax = 0u32;
    loop {
        let idx = eax as u16 as usize;
        let v = rd32(obj, idx * 4);
        let r10 = ((v >> 22) & 0xff) | v.wrapping_shl(26);
        let edx = (v.wrapping_shl(10) & 0x00ff_0000) | r10;
        wr32(obj, idx * 4, ((v >> 6) & 0xff00) | edx);
        eax = eax.wrapping_add(1);
        if !br((eax as u16 as u32) < esi) {
            break;
        }
    }
}

/// parity: P1 @0xcbc03 size=81
/// Set predicate: edi in {0x10266400, 0x10266401, 0x10294700, 0x10322950, 0x10352300, 0x10518400, 0x10518450}
/// (signed splits at 0x105183ff and 0x103522ff).
pub fn f_000cbc03(edi: u32) -> u8 {
    let s = edi as i32;
    if br(s > 0x1051_83ff) {
        (br(edi == 0x1051_8400) || br(edi == 0x1051_8450)) as u8
    } else if br(s > 0x1035_22ff) {
        br(edi == 0x1035_2300) as u8
    } else {
        (br(edi.wrapping_sub(0x1026_6400) < 2) || br(edi == 0x1029_4700) || br(edi == 0x1032_2950)) as u8
    }
}

/// parity: P1 @0xcd28e size=81
/// Set predicate: edi in {0x10378800, 0x10416000, 0x10478300, 0x10478350, 0x10710800, 0x10710850,
/// 0x10710851} (signed splits at 0x107107ff and 0x104782ff).
pub fn f_000cd28e(edi: u32) -> u8 {
    let s = edi as i32;
    if br(s > 0x1071_07ff) {
        (br(edi.wrapping_sub(0x1071_0850) < 2) || br(edi == 0x1071_0800)) as u8
    } else if br(s > 0x1047_82ff) {
        (br(edi == 0x1047_8300) || br(edi == 0x1047_8350)) as u8
    } else {
        (br(edi == 0x1037_8800) || br(edi == 0x1041_6000)) as u8
    }
}

/// parity: P1 @0xd10ee size=81
/// Set predicate: edi in {0x10280300, 0x10378800, 0x10478300, 0x10478350, 0x10710800, 0x10710850,
/// 0x10710851} (signed splits at 0x107107ff and 0x104782ff).
pub fn f_000d10ee(edi: u32) -> u8 {
    let s = edi as i32;
    if br(s > 0x1071_07ff) {
        (br(edi.wrapping_sub(0x1071_0850) < 2) || br(edi == 0x1071_0800)) as u8
    } else if br(s > 0x1047_82ff) {
        (br(edi == 0x1047_8300) || br(edi == 0x1047_8350)) as u8
    } else {
        (br(edi == 0x1028_0300) || br(edi == 0x1037_8800)) as u8
    }
}

/// parity: P1 @0x32578 size=82
/// Initialiser: qwords at +0, +8 zero; dwords +0x10 = 0xc, +0x14 = 0, +0x18 = 0x20, +0x1c = 3; byte +0x20 = 0;
/// qwords +0x24, +0x2c zero; word +0x34 = 0; dword +0x38 = 0x12.
pub fn f_00032578(obj: &mut [u8]) {
    wr64(obj, 8, 0);
    wr64(obj, 0, 0);
    wr32(obj, 0x10, 0xc);
    wr32(obj, 0x14, 0);
    wr32(obj, 0x18, 0x20);
    wr32(obj, 0x1c, 3);
    wr8(obj, 0x20, 0);
    wr16(obj, 0x34, 0);
    wr64(obj, 0x2c, 0);
    wr64(obj, 0x24, 0);
    wr32(obj, 0x38, 0x12);
}

/// parity: P1 @0x5c91a size=82
/// Relation between two small codes: true when edi == esi, or when (edi, esi) is one of
/// (1, 0x1c), (2, 0x1d), (3, 0x1e), (0x1d, 2), (0x1c, 1), (0x1e, 3). Signed comparisons on edi.
pub fn f_0005c91a(edi: u32, esi: u32) -> u8 {
    if br(edi == esi) {
        return 1;
    }
    let s = edi as i32;
    if br(s > 1) {
        if br(s < 3) {
            return (esi == 0x1d) as u8;
        }
        if br(s > 0x1c) {
            if br(edi == 0x1d) {
                (esi == 2) as u8
            } else if br(edi == 0x1e) {
                (esi == 3) as u8
            } else {
                0
            }
        } else if br(edi == 3) {
            (esi == 0x1e) as u8
        } else if br(edi == 0x1c) {
            (esi == 1) as u8
        } else {
            0
        }
    } else if br(edi == 1) {
        (esi == 0x1c) as u8
    } else {
        0
    }
}

/// parity: P1 @0x6474a size=83
/// Initialiser with constants: thirteen words at +0..+0x1a. The values match the BT.2020 mastering display
/// colour volume of SMPTE ST 2086 in 0.00002 units (G 8500,39850; B 6550,2300; R 35400,14600; D65 white
/// 15635,16450; max 10000, min 50) (inferred from the numbers, confidence med).
pub fn f_0006474a(obj: &mut [u8]) {
    wr16(obj, 0x0, 0x2134);
    wr16(obj, 0x2, 0x9baa);
    wr16(obj, 0x4, 0x1996);
    wr16(obj, 0x6, 0x08fc);
    wr16(obj, 0x8, 0x8a48);
    wr16(obj, 0xa, 0x3908);
    wr16(obj, 0xc, 0x3d13);
    wr16(obj, 0xe, 0x4042);
    wr16(obj, 0x10, 0x2710);
    wr16(obj, 0x12, 0x0032);
    wr16(obj, 0x14, 0);
    wr16(obj, 0x16, 0);
    wr16(obj, 0x18, 2);
}

/// parity: P1 @0x6479d size=83
/// Initialiser with constants: thirteen words at +0..+0x1a, the DCI-P3 primaries (G 13250,34500; B 7500,3000;
/// R 34000,16000) with the D65 white point and max 744, min 50 (inferred from the numbers, confidence med).
pub fn f_0006479d(obj: &mut [u8]) {
    wr16(obj, 0x0, 0x33c2);
    wr16(obj, 0x2, 0x86c4);
    wr16(obj, 0x4, 0x1d4c);
    wr16(obj, 0x6, 0x0bb8);
    wr16(obj, 0x8, 0x84d0);
    wr16(obj, 0xa, 0x3e80);
    wr16(obj, 0xc, 0x3d13);
    wr16(obj, 0xe, 0x4042);
    wr16(obj, 0x10, 0x02e8);
    wr16(obj, 0x12, 0x0032);
    wr16(obj, 0x14, 0);
    wr16(obj, 0x16, 0);
    wr16(obj, 0x18, 2);
}

/// parity: P1 @0xcf9e8 size=83
/// Set predicate returned as 0 or 1 in eax: edi in {0x10266400, 0x10266401, 0x10294700, 0x10378800,
/// 0x10478350, 0x10518450} (signed splits at 0x1047834f and 0x102946ff).
pub fn f_000cf9e8(edi: u32) -> u32 {
    let s = edi as i32;
    if br(s > 0x1047_834f) {
        (br(edi == 0x1047_8350) || br(edi == 0x1051_8450)) as u32
    } else if br(s > 0x1029_46ff) {
        (br(edi == 0x1029_4700) || br(edi == 0x1037_8800)) as u32
    } else {
        (br(edi == 0x1026_6400) || br(edi == 0x1026_6401)) as u32
    }
}

/// parity: P1 @0xd0a75 size=83
/// Same code as f_000cf9e8.
pub fn f_000d0a75(edi: u32) -> u32 {
    let s = edi as i32;
    if br(s > 0x1047_834f) {
        (br(edi == 0x1047_8350) || br(edi == 0x1051_8450)) as u32
    } else if br(s > 0x1029_46ff) {
        (br(edi == 0x1029_4700) || br(edi == 0x1037_8800)) as u32
    } else {
        (br(edi == 0x1026_6400) || br(edi == 0x1026_6401)) as u32
    }
}
