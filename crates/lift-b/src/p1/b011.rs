//! Batch 011: thin wrappers over the format classifiers (b003, b004, b010): compare a classifier result with
//! a count, tail-call a classifier, or OR several predicates on a field of an object.
use crate::cov::br;
use crate::p1::{b003, b004, b010};
use crate::*;

/// parity: P1 @0xd2f10 size=10
/// Tail call: returns f_000cf820(edi).
pub fn f_000d2f10(edi: u32) -> u32 {
    b010::f_000cf820(edi)
}

/// parity: P1 @0xd2ebc size=28
/// True when esi < (f_000d0ea0(edi) & 0xffff), unsigned.
pub fn f_000d2ebc(edi: u32, esi: u32) -> u8 {
    br(esi < (b010::f_000d0ea0(edi) & 0xffff)) as u8
}

/// parity: P1 @0xd2ed8 size=28
/// True when esi < (f_000d0fb9(edi) & 0xffff), unsigned.
pub fn f_000d2ed8(edi: u32, esi: u32) -> u8 {
    br(esi < (b010::f_000d0fb9(edi) & 0xffff)) as u8
}

/// parity: P1 @0xd2ef4 size=28
/// True when esi < (f_000cf820(edi) & 0xffff), unsigned.
pub fn f_000d2ef4(edi: u32, esi: u32) -> u8 {
    br(esi < (b010::f_000cf820(edi) & 0xffff)) as u8
}

/// parity: P1 @0xd2f1a size=28
/// True when (f_000d0960(edi) & 0xffff) > esi, signed (the left side is non-negative).
pub fn f_000d2f1a(edi: u32, esi: u32) -> u8 {
    br(((b010::f_000d0960(edi) & 0xffff) as i32) > esi as i32) as u8
}

/// parity: P1 @0xd2f36 size=37
/// For edi in {0x10478300, 0x10710800} returns (f_000cf820(edi) & 0xffff) + 1, otherwise 8.
pub fn f_000d2f36(edi: u32) -> u32 {
    if br(edi == 0x1047_8300) || br(edi == 0x1071_0800) {
        (b010::f_000cf820(edi) & 0xffff).wrapping_add(1)
    } else {
        8
    }
}

/// parity: P1 @0xd8956 size=56
/// Predicate on the dword at +0x10: f_000cdb57(x) or f_000cdbbf(x), else the result of f_000ce3be(x)
/// (a tail call). Returned in al.
pub fn f_000d8956(obj: &[u8]) -> u8 {
    let x = rd32(obj, 0x10);
    if br(b010::f_000cdb57(x) != 0) {
        return 1;
    }
    if br(b003::f_000cdbbf(x) != 0) {
        return 1;
    }
    b004::f_000ce3be(x)
}
