//! fx_common: shared scratch helpers of the lean effect rebuild (DECISIONS_20261030 S3 to S8). std only, no unsafe.
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
