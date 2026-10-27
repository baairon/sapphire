//! Batch 021: the prefix-sum search f_00032f38 (b013) used to divide a remainder by the weight of the found
//! entry. A zero divisor faults in the original (those cases are skipped by the vectors).
use crate::cov::br;
use crate::p1::b013;
use crate::*;

/// parity: P1 @0x32fb2 size=142
/// idx = f_00032f38(obj, esi) (16 bit); 0xffff stays 0xffff. Otherwise returns
/// (esi - sum of the weights of entries 0..idx, each times the dword at +0, 32 bit) / weight[idx], low 16 bits
/// (entries beyond the count at +0x34 count as 0; the divisor is 0 when idx >= count, which faults). The guards
/// `n > k` and `n > idx` are always true here because f_00032f38 returns an index below the count (the false
/// outcomes are unreachable, not untested).
pub fn f_00032fb2(obj: &[u8], esi: u32) -> u32 {
    let idx = (b013::f_00032f38(obj, esi) & 0xffff) as u16;
    if br(idx == 0xffff) {
        return 0xffff;
    }
    let n = rd16(obj, 0x34) as u32;
    let mut sum = 0u32;
    if br(idx != 0) {
        let mut k = idx.wrapping_sub(1);
        loop {
            let term = if br(n > k as u32) { rd32(obj, 0x24 + 4 * k as usize).wrapping_mul(rd32(obj, 0)) } else { 0 };
            sum = sum.wrapping_add(term);
            k = k.wrapping_sub(1);
            if !br(k != 0xffff) {
                break;
            }
        }
    }
    let rem = esi.wrapping_sub(sum);
    let div = if br(n > idx as u32) { rd32(obj, 0x24 + 4 * idx as usize) } else { 0 };
    ((rem / div) & 0xffff) as u32
}

/// parity: P1 @0x33040 size=144
/// Exactness check: false when the dword at +0, the dword at +4, the count word at +0x34 or the first weight at
/// +0x24 is zero, or when idx = f_00032f38(obj, esi) >= count. Otherwise subtracts from esi (32 bit) weight[0] * X
/// when idx != 0 and, when idx >= 2, weight[1] * X (X = dword at +0); returns whether the remainder
/// of the division by weight[idx] is zero (a zero divisor faults). The guard `n >= 2` is always true when idx >= 2
/// (idx < n), so its false outcome is unreachable.
pub fn f_00033040(obj: &[u8], esi: u32) -> u8 {
    let x = rd32(obj, 0);
    if br(x == 0) {
        return 0;
    }
    if br(rd32(obj, 4) == 0) {
        return 0;
    }
    let n = rd16(obj, 0x34) as u32;
    if br(n == 0) {
        return 0;
    }
    let w0 = rd32(obj, 0x24);
    if br(w0 == 0) {
        return 0;
    }
    let idx = b013::f_00032f38(obj, esi) & 0xffff;
    if br(idx >= n) {
        return 0;
    }
    let mut r14 = esi;
    if br(idx as u16 != 0) {
        r14 = r14.wrapping_sub(w0.wrapping_mul(x));
        if br(idx >= 2) {
            let sub = if br(n >= 2) { x.wrapping_mul(rd32(obj, 0x28)) } else { 0 };
            r14 = r14.wrapping_sub(sub);
        }
    }
    let d = rd32(obj, 0x24 + 4 * idx as usize);
    (r14 % d == 0) as u8
}
