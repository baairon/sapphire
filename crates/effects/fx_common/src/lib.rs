//! fx_common: shared helpers of the effect crates. std only, no unsafe.
//! `x86` instruction semantics, `cmp` plane comparison, `plane` types and generators, `oracle_io` harness manifests,
//! `png` preview writer, `assume` stand-in registry, and COPIES of the Shake natives (`shake_natives`, `shake_sin`).
pub mod assume;
pub mod cmp;
pub mod oracle_io;
pub mod plane;
pub mod png;
pub mod shake_natives;
pub mod shake_sin;
pub mod x86;
