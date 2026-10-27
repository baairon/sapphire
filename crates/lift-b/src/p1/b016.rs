//! Batch 016: format-id tables (jump tables) that fill small output records, and a format predicate with a
//! level limit.
use crate::cov::{br, hit};
use crate::*;

/// parity: P1 @0xdf4cc size=248
/// Fills two dword outputs from the format id (dword at +0x274, jump table at 0xdf540); ids above 0x20 write
/// nothing. (out1, out2): id 0 -> (0x800, 0x560); 1, 5 -> (0x800, 0x400); 2, 3, 4, 6, 7, 15, 20 -> (0x800, 0x800);
/// 12, 13 -> (0x800, 0x600); 16 -> (0x800, 0xc00); every other id up to 0x20 -> (0, 0).
pub fn f_000df4cc(obj: &[u8], out1: &mut [u8], out2: &mut [u8]) {
    let id = rd32(obj, 0x274);
    let (a, b) = match id {
        0 => {
            hit();
            (0x800, 0x560)
        }
        1 | 5 => {
            hit();
            (0x800, 0x400)
        }
        2 | 3 | 4 | 6 | 7 | 15 | 20 => {
            hit();
            (0x800, 0x800)
        }
        12 | 13 => {
            hit();
            (0x800, 0x600)
        }
        16 => {
            hit();
            (0x800, 0xc00)
        }
        8..=11 | 14 | 17..=19 | 21..=32 => {
            hit();
            (0, 0)
        }
        _ => {
            hit();
            return;
        }
    };
    wr32(out1, 0, a);
    wr32(out2, 0, b);
}

/// parity: P1 @0xdf6e4 size=288
/// Same id table as f_000df4cc with three dword outputs (out1, out2, out3): id 0 -> (0x1000, 0xac0, 0x870);
/// 1, 5 -> (0x1000, 0x800, 0x870); 2, 3, 4, 6, 7, 15, 20 -> (0x1000, 0x1000, 0x870); 12, 13 -> (0x1000, 0xc00,
/// 0x870); 16 -> (0x1000, 0x1800, 0x870); every other id up to 0x20 -> (0, 0, 0); ids above 0x20 write nothing.
pub fn f_000df6e4(obj: &[u8], out1: &mut [u8], out2: &mut [u8], out3: &mut [u8]) {
    let id = rd32(obj, 0x274);
    let (a, b, c) = match id {
        0 => {
            hit();
            (0x1000, 0xac0, 0x870)
        }
        1 | 5 => {
            hit();
            (0x1000, 0x800, 0x870)
        }
        2 | 3 | 4 | 6 | 7 | 15 | 20 => {
            hit();
            (0x1000, 0x1000, 0x870)
        }
        12 | 13 => {
            hit();
            (0x1000, 0xc00, 0x870)
        }
        16 => {
            hit();
            (0x1000, 0x1800, 0x870)
        }
        8..=11 | 14 | 17..=19 | 21..=32 => {
            hit();
            (0, 0, 0)
        }
        _ => {
            hit();
            return;
        }
    };
    wr32(out1, 0, a);
    wr32(out2, 0, b);
    wr32(out3, 0, c);
}

/// parity: P1 @0xcaf40 size=364
/// Format predicate with a level limit: true when edi is in the set {0x10266400, 0x10266401, 0x10293000,
/// 0x10294700, 0x10294900, 0x10322950, 0x10378800, 0x10402100, 0x10478300, 0x10478350, 0x10518400, 0x10518450,
/// 0x10538200, 0x10565400, 0x10646700..=0x10646706, 0x10710800, 0x10710850, 0x10710851} and esi < 6, or
/// edi == 0x10352300 and esi < 3.
pub fn f_000caf40(edi: u32, esi: u32) -> u8 {
    let class = match edi {
        0x1026_6400..=0x1026_6401
        | 0x1029_3000
        | 0x1029_4700
        | 0x1029_4900
        | 0x1032_2950
        | 0x1037_8800
        | 0x1040_2100
        | 0x1047_8300
        | 0x1047_8350
        | 0x1051_8400
        | 0x1051_8450
        | 0x1053_8200
        | 0x1056_5400
        | 0x1064_6700..=0x1064_6706
        | 0x1071_0800
        | 0x1071_0850..=0x1071_0851 => 1,
        0x1035_2300 => 2,
        _ => 0,
    };
    if br(class == 1) {
        br(esi < 6) as u8
    } else if br(class == 2) {
        br(esi < 3) as u8
    } else {
        0
    }
}
