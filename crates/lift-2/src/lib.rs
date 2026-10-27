#![forbid(unsafe_code)]
//! lift_2: Rust ports of the neutral_3 functions assigned to group B (see findings/fleet-2/SUMMARY.md).
//!
//! Struct pointers are raw byte views (`&[u8]` / `&mut [u8]` starting at the pointer) until the layouts are
//! established; fields are read and written at the same offsets the original code uses.
//! Modules: `p1` (verified against oracle vectors), `p2` (faithful, unverified), `missing` (placeholders for
//! data and imports that are not in the dump), `dup` (copies of group A functions this group calls).
//! `cov` is a branch-coverage helper used only by the parity tests (feature `cov`).

pub mod cov;
pub mod dup;
pub mod missing;
pub mod p1;
pub mod p2;
pub mod p2v;
pub mod x86rt;

#[inline]
pub fn rd8(b: &[u8], off: usize) -> u8 {
    b[off]
}

#[inline]
pub fn rd16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes(b[off..off + 2].try_into().unwrap())
}

#[inline]
pub fn rd32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(b[off..off + 4].try_into().unwrap())
}

#[inline]
pub fn rd64(b: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(b[off..off + 8].try_into().unwrap())
}

#[inline]
pub fn wr8(b: &mut [u8], off: usize, v: u8) {
    b[off] = v;
}

#[inline]
pub fn wr16(b: &mut [u8], off: usize, v: u16) {
    b[off..off + 2].copy_from_slice(&v.to_le_bytes());
}

#[inline]
pub fn wr32(b: &mut [u8], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

#[inline]
pub fn wr64(b: &mut [u8], off: usize, v: u64) {
    b[off..off + 8].copy_from_slice(&v.to_le_bytes());
}
