//! Constants of the AVX body and their capture status (S7). Addresses are VA = listing, with the
//! listing offset in parentheses. Capture status: `scout_b/lookup_raw.tsv` (written by `audit_addr/tools/lookup.py`).
use fx_common::assume::Assumption;

/// 1.0f32 : CAPTURED (bytes 0000803f, MEASURED, scout_b/lookup_raw.tsv). The scalar
/// compares `t >= 1` of the body read it. Not an assumption; kept as a constant for the harness patch list.
pub const ONE_VA: u64 = 0;
pub const ONE_LISTING: u64 = 0;
pub const ONE_HEX: &str = "0000803f";

/// SC2-A1 (a): ymm constant, 32 B, NOT captured. INFERRED 1.0f x8: the vector core
/// uses it as `vcmpgeps t, K` (t >= 1) and as the vector `1` compare operand.
pub const SC2_A1_ONE8: Assumption = Assumption::new(
    "SC2-A1",
    0,
    "0000803f0000803f0000803f0000803f0000803f0000803f0000803f0000803f",
    "ymm constant loaded / as the t >= 1 compare operand of the vector core; value not captured, 1.0f x8 inferred from use",
);
/// SC2-A1 (b): ymm constant, 32 B, NOT captured. INFERRED all ones: the vector core
/// xors it into the two compare masks to build their complements (vxorps K, mask) before and-ing and blending.
pub const SC2_A1_ONES: Assumption = Assumption::new(
    "SC2-A1",
    0,
    "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
    "ymm constant loaded // as the xor operand that complements a compare mask; value not captured, all ones inferred from use",
);

/// The two labelled stand-ins (S7). The harness patches the same 64 bytes in every run and lists them in each manifest.
pub fn assumptions() -> Vec<Assumption> {
    vec![SC2_A1_ONE8, SC2_A1_ONES]
}
