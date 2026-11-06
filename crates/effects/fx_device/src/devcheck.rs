//! Comparison helpers of the byte gate (`devcheck`): a render on the CPU and a render on the device must agree byte for byte.
// Provenance: adapted from twixtor crates/fx-parity/src/devcheck.rs (compare, report parsing) (8087d06).

/// `None` when `cpu` and `dev` are identical, else a sentence that names the first difference.
pub fn first_difference(cpu: &[u8], dev: &[u8]) -> Option<String> {
    if cpu.len() != dev.len() {
        return Some(format!("length {} on the CPU, {} on the device", cpu.len(), dev.len()));
    }
    let i = (0..cpu.len()).find(|&i| cpu[i] != dev[i])?;
    Some(format!("first difference at byte {i}: 0x{:02x} on the CPU, 0x{:02x} on the device", cpu[i], dev[i]))
}

/// The tally of one device over a set of cases.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Tally {
    pub cases: usize,
    pub identical: usize,
    pub different: usize,
    /// Nodes that fell back to the CPU, counted but not a difference.
    pub fallbacks: u64,
}

impl Tally {
    pub fn record(&mut self, difference: Option<&str>, fallbacks: u64) {
        self.cases += 1;
        self.fallbacks += fallbacks;
        if difference.is_some() {
            self.different += 1;
        } else {
            self.identical += 1;
        }
    }

    pub fn passed(&self) -> bool {
        self.different == 0 && self.cases > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_the_first_difference() {
        assert_eq!(first_difference(b"abc", b"abc"), None);
        assert!(first_difference(b"abc", b"abd").unwrap().contains("byte 2"));
        assert!(first_difference(b"abc", b"ab").unwrap().contains("length"));
    }

    #[test]
    fn tally_counts() {
        let mut t = Tally::default();
        t.record(None, 2);
        assert!(t.passed());
        t.record(Some("x"), 0);
        assert!(!t.passed());
        assert_eq!((t.cases, t.identical, t.different, t.fallbacks), (2, 1, 1, 2));
    }
}
