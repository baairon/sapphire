//! Shake shift route, callee layer: the CPU-feature dispatchers, the hash table builder and the two hashes, the
//! fill routine, the single-precision sine chain with its computed tables, and the weight-table builder.
//! Nothing here reads a data file: every table is computed from its mathematical definition.

pub mod body;
pub mod dispatch;
pub mod exact;
pub mod fill;
pub mod hash;
mod initlist;
pub mod sin;
pub mod tables;
pub mod weights;

/// Labelled stand-ins of this crate.
pub fn assumptions() -> &'static [&'static str] {
    &[
        "fill routine: counts of 128 and above store the same bytes as the original by construction (every jump-table block stores the n bytes before the pointer); the table entries past the first 0x1800 bytes of the table region were not read, so that path is K-PARITY-THIN",
        "route AVX2-ONLY: the SSE render body is not ported",
        "sine chain runs in round-to-nearest (the routine forces it and restores the caller's mode before the final narrowing)",
        "sine tables are computed from the definitions; the tiny scale is the scalar 1 - 2^-30",
        "the CPU-feature word is taken as an argument; the init call taken when no feature bit is set does not terminate and is reported as a distinct outcome",
        "weight builder cutoff uses the built value (index of the first zero entry doubled), not the stored image word",
    ]
}

/// Reference services built from this crate's own callees (hash table, computed sine tables).
pub struct RefServices {
    pub ht: hash::HashTable,
    pub st: &'static tables::SinTables,
}
impl RefServices {
    pub fn new() -> RefServices {
        RefServices { ht: hash::HashTable::build(), st: tables::SinTables::computed() }
    }
}
impl body::Services for RefServices {
    fn hash1(&mut self, x: u32) -> u32 {
        self.ht.hash1(x)
    }
    fn hash2(&mut self, x: u32, y: u32) -> u32 {
        self.ht.hash2(x, y)
    }
    fn sine(&mut self, x: f32) -> f32 {
        sin::sinf(x, self.st)
    }
}
