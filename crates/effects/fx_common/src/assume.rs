//! Stand-in registry (S7). A value that is missing from the binaries is either a caller-supplied field or one `Assumption`
//! returned by the module's `assumptions()` and listed in its README. The harness patches the same bytes into emulated memory
//! and lists the patch in the run manifest. Never silent, never an invented default.

/// Label carried by every entry.
pub const LABEL: &str = "ASSUMED";

/// One labelled stand-in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assumption {
    /// Plan id (C02, U10, G01, A1, ...).
    pub id: &'static str,
    /// Virtual address in the captured image the real value would live at.
    pub va: u64,
    /// The stand-in bytes in memory order (little-endian), as lowercase hex.
    pub bytes_hex: &'static str,
    /// Why this value (what the code uses it for, and why the choice).
    pub basis: &'static str,
    /// Always "ASSUMED".
    pub label: &'static str,
}

impl Assumption {
    pub const fn new(id: &'static str, va: u64, bytes_hex: &'static str, basis: &'static str) -> Assumption {
        Assumption { id, va, bytes_hex, basis, label: LABEL }
    }

    /// Markdown table row for a README: `| id | 0xva | bytes | basis | ASSUMED |`.
    pub fn to_row(&self) -> String {
        format!("| {} | 0x{:x} | {} | {} | {} |", self.id, self.va, self.bytes_hex, self.basis, self.label)
    }

    /// The bytes as a Vec.
    pub fn bytes(&self) -> Vec<u8> {
        (0..self.bytes_hex.len() / 2).map(|i| u8::from_str_radix(&self.bytes_hex[2 * i..2 * i + 2], 16).unwrap()).collect()
    }
}

/// A1: the constant ONE of dist_from_screen_edge, not captured. 1.0f32.
pub const A1_ONE: Assumption = Assumption::new("A1", 0, "0000803f", "compare/subtract constant used as 1.0 by; value not captured");
/// A2: the fabs mask, not captured. 0x7fffffff in every lane (16 byte load in both listings).
pub const A2_FABS_MASK: Assumption = Assumption::new(
    "A2",
    0,
    "ffffff7fffffff7fffffff7fffffff7f",
    "andps/vandps abs mask of; value not captured",
);
/// A2: the sign mask, not captured. 0x80000000 in every lane (16 byte load in both listings).
pub const A2_SIGN_MASK: Assumption = Assumption::new(
    "A2",
    0,
    "00000080000000800000008000000080",
    "xorps/vxorps sign mask of; value not captured",
);

/// Markdown table of a set of assumptions.
pub fn to_markdown(list: &[Assumption]) -> String {
    let mut s = String::from("| id | va | bytes | basis | label |\n|---|---|---|---|---|\n");
    for a in list {
        s.push_str(&a.to_row());
        s.push('\n');
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a1_is_one() {
        let b = A1_ONE.bytes();
        assert_eq!(f32::from_le_bytes([b[0], b[1], b[2], b[3]]), 1.0);
        assert_eq!(A1_ONE.label, "ASSUMED");
    }

    #[test]
    fn a2_masks() {
        for lane in A2_FABS_MASK.bytes().chunks(4) {
            assert_eq!(u32::from_le_bytes([lane[0], lane[1], lane[2], lane[3]]), 0x7fff_ffff);
        }
        for lane in A2_SIGN_MASK.bytes().chunks(4) {
            assert_eq!(u32::from_le_bytes([lane[0], lane[1], lane[2], lane[3]]), 0x8000_0000);
        }
        assert_eq!(A2_SIGN_MASK.bytes().len(), 16);
    }

    #[test]
    fn markdown_has_one_row_per_entry() {
        let md = to_markdown(&[A1_ONE, A2_FABS_MASK]);
        assert_eq!(md.lines().count(), 4);
        assert!(md.contains(""));
    }
}
