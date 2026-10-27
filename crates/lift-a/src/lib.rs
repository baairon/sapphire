//! lift-a: group A of neutral_3 ported to Rust.
//!
//! Tiers (see docs/PORTING_GUIDE.md): `p1` holds functions verified bit for bit against the
//! emulated original, `p2` holds faithful but unverified ports with named placeholders for
//! missing data (see `missing`), `dup` holds minimal copies of group B functions that group A
//! code calls. Struct pointers are raw byte views until layouts are established.
#![forbid(unsafe_code)]
#![allow(non_snake_case)]
#![allow(clippy::all)]

pub mod dup;
pub mod lift_rt;
#[cfg(feature = "liftcheck")]
pub mod liftcheck;
pub mod missing;
pub mod p1;
pub mod p2;
pub mod p2v;

/// Flat little-endian memory of the original address space, for ports that follow pointers stored in
/// objects. Parity tests implement it over the oracle's buffers (tests/common).
pub trait Mem {
    fn rd8(&self, a: u64) -> u8;
    fn wr8(&mut self, a: u64, v: u8);
    fn rd16(&self, a: u64) -> u16 {
        u16::from_le_bytes([self.rd8(a), self.rd8(a.wrapping_add(1))])
    }
    fn rd32(&self, a: u64) -> u32 {
        u32::from_le_bytes([self.rd8(a), self.rd8(a.wrapping_add(1)), self.rd8(a.wrapping_add(2)), self.rd8(a.wrapping_add(3))])
    }
    fn rd64(&self, a: u64) -> u64 {
        self.rd32(a) as u64 | (self.rd32(a.wrapping_add(4)) as u64) << 32
    }
    fn wr16(&mut self, a: u64, v: u16) {
        for (i, b) in v.to_le_bytes().iter().enumerate() {
            self.wr8(a.wrapping_add(i as u64), *b);
        }
    }
    fn wr32(&mut self, a: u64, v: u32) {
        for (i, b) in v.to_le_bytes().iter().enumerate() {
            self.wr8(a.wrapping_add(i as u64), *b);
        }
    }
    fn wr64(&mut self, a: u64, v: u64) {
        for (i, b) in v.to_le_bytes().iter().enumerate() {
            self.wr8(a.wrapping_add(i as u64), *b);
        }
    }
}

/// Little-endian reads and writes at fixed offsets of a byte view (the original's field accesses).
#[inline]
pub(crate) fn rd8(b: &[u8], off: usize) -> u8 {
    b[off]
}
#[inline]
pub(crate) fn rd16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes(b[off..off + 2].try_into().unwrap())
}
#[inline]
pub(crate) fn rd32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes(b[off..off + 4].try_into().unwrap())
}
#[inline]
pub(crate) fn rd64(b: &[u8], off: usize) -> u64 {
    u64::from_le_bytes(b[off..off + 8].try_into().unwrap())
}
#[inline]
pub(crate) fn wr8(b: &mut [u8], off: usize, v: u8) {
    b[off] = v;
}
#[inline]
pub(crate) fn wr16(b: &mut [u8], off: usize, v: u16) {
    b[off..off + 2].copy_from_slice(&v.to_le_bytes());
}
#[inline]
pub(crate) fn wr32(b: &mut [u8], off: usize, v: u32) {
    b[off..off + 4].copy_from_slice(&v.to_le_bytes());
}
#[inline]
pub(crate) fn wr64(b: &mut [u8], off: usize, v: u64) {
    b[off..off + 8].copy_from_slice(&v.to_le_bytes());
}
