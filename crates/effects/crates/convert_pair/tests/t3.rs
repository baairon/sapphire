//! CV4 T3: the two mutation survivors of CV3.
//! #4 (clamp_alpha: `1.0 <= a` instead of `1.0 < a`): equivalent mutant, proved by EXHAUSTION over all 2^32 f32 bit patterns (bit equal results).
//! #15 (replicate_band: count = cell instead of cell - 1): lives in the cell > 1 sketch (outside HOST-RES-FULL); killed here by a T1 test of the
//! sketch's own specification (listing reading of cfedb..cff9a: count = min(cell - 1, last_row - y)); the sketch is not oracle validated and not claimed.
//! run: cargo test --release --features mutants --test t3 -- --test-threads=1 (the mutant switch is global)
use convert_pair::{clamp_alpha, replicate_band, set_mutant};

#[test]
fn survivor4_exhaustive_equivalence() {
    let block: u64 = 1 << 22;
    let mut start: u64 = 0;
    let mut checked: u64 = 0;
    let mut a = vec![0u32; block as usize];
    let mut b = vec![0u32; block as usize];
    while start < (1u64 << 32) {
        let end = (start + block).min(1u64 << 32);
        let len = (end - start) as usize;
        set_mutant(0);
        for i in 0..len {
            a[i] = clamp_alpha(f32::from_bits((start + i as u64) as u32)).to_bits();
        }
        set_mutant(4);
        for i in 0..len {
            b[i] = clamp_alpha(f32::from_bits((start + i as u64) as u32)).to_bits();
        }
        set_mutant(0);
        for i in 0..len {
            assert_eq!(a[i], b[i], "bit pattern {:08x}", (start + i as u64) as u32);
        }
        checked += len as u64;
        start = end;
    }
    assert_eq!(checked, 1u64 << 32);
    println!("survivor4: {} bit patterns, 0 differ", checked);
}

fn dest_rows(rows: usize, n: usize) -> Vec<f32> {
    let mut d = vec![0.0f32; rows * n * 4];
    for (i, x) in d.iter_mut().enumerate() {
        *x = (i / (n * 4)) as f32 + 1000.0; // row r holds the value 1000 + r everywhere
    }
    d
}

#[test]
fn survivor15_replicate_band_spec() {
    set_mutant(0);
    let n = 3;
    // cell 3, band at y = 0 of a 8 row destination: rows 1 and 2 receive row 0, row 3 is untouched
    let mut d = dest_rows(8, n);
    for k in 0..n * 4 {
        d[k] = 7.0;
    }
    replicate_band(&mut d, n, 0, 3, 7);
    assert!(d[(n * 4)..(3 * n * 4)].iter().all(|&x| x == 7.0));
    assert!(d[(3 * n * 4)..(4 * n * 4)].iter().all(|&x| x == 1003.0));
    // last band cut by the last row: y = 6, cell 4, last row 7: only row 7 is written
    let mut d = dest_rows(8, n);
    for k in 0..n * 4 {
        d[6 * n * 4 + k] = 9.0;
    }
    replicate_band(&mut d, n, 6, 4, 7);
    assert!(d[(7 * n * 4)..(8 * n * 4)].iter().all(|&x| x == 9.0));
    // cell 1: nothing
    let mut d = dest_rows(4, n);
    let before = d.clone();
    replicate_band(&mut d, n, 1, 1, 3);
    assert_eq!(d, before);
}

// INT14: the mutant switch acts only under --features mutants (mu is cfg!(feature = "mutants")), so this kill test is
// compiled only there; INT13's default-feature failure was this test configuration, not a survivor (MEASURED: passes under the feature).
#[cfg(feature = "mutants")]
#[test]
fn survivor15_mutant_is_killed_by_the_spec_test() {
    // global mutant switch: run this file with --test-threads=1
    set_mutant(15);
    let r = std::panic::catch_unwind(survivor15_replicate_band_spec_body);
    set_mutant(0);
    assert!(r.is_err(), "mutant 15 survived the specification test");
}

#[cfg(feature = "mutants")]
fn survivor15_replicate_band_spec_body() {
    let n = 3;
    let mut d = dest_rows(8, n);
    for k in 0..n * 4 {
        d[k] = 7.0;
    }
    replicate_band(&mut d, n, 0, 3, 7);
    assert!(d[(3 * n * 4)..(4 * n * 4)].iter().all(|&x| x == 1003.0));
}
