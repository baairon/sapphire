//! RADII-ALIGN: the radii slice's phase (mod 32) selects the order of the f64 product of 2r + 1.
use glint_pass_rows::*;

const PHASES: [u8; 12] = [0, 4, 8, 12, 16, 20, 24, 28, 1, 2, 3, 6];
const COUNTS: [usize; 11] = [1, 5, 15, 16, 17, 40, 56, 57, 58, 100, 130];

fn scale_word(radii: &[i32], phase: u8) -> u32 {
    // radii <= 0 skip every pass but stay in the product; width 1 and source 1.0 make dst the scale.
    let c = Config { edge: 0, radii, shift: 0 };
    let phases = Alignment { scratch: [0, 0], radii: phase };
    let mut d = [0f32; 1];
    run(&c, &phases, &[1.0], &mut d, 1, 1).unwrap();
    d[0].to_bits()
}

/// Radii whose items 2r + 1 are 1, -1 and -3 only: every partial product has magnitude below 2^53 (at most 3^26).
fn below_bound(n: usize) -> Vec<i32> {
    (0..n).map(|i| if i % 5 == 0 { -2 } else if i % 5 == 1 { -1 } else { 0 }).collect()
}

#[test]
fn phase_free_below_the_bound() {
    for &n in &COUNTS {
        let r = below_bound(n);
        let threes = r.iter().filter(|&&x| x == -2).count();
        assert!(threes <= 30, "bound: 3^{threes} < 2^53");
        let first_phase = radii_product(&r, 0);
        assert!(first_phase.abs() < 9007199254740992.0);
        for &ph in &PHASES {
            assert_eq!(radii_product(&r, ph).to_bits(), first_phase.to_bits(), "n {n} phase {ph}");
            assert_eq!(scale_word(&r, ph), scale_word(&r, 0), "n {n} phase {ph}");
        }
    }
}

#[test]
fn phase_free_with_larger_odd_items_below_the_bound() {
    // items 2r + 1 up to 2^10 with 5 of them (positions 0, 4, 8, 12, 16): product below 2^53 for every order.
    for &n in &COUNTS {
        let r: Vec<i32> = (0..n).map(|i| if i < 20 && i % 4 == 0 { -(((i as i32 * 37) % 500) + 1) } else { 0 }).collect();
        let mut bound = 1.0f64;
        for &x in &r {
            bound *= ((2 * x + 1) as f64).abs();
        }
        assert!(bound < 9007199254740992.0);
        let first_phase = radii_product(&r, 0);
        for &ph in &PHASES {
            assert_eq!(radii_product(&r, ph).to_bits(), first_phase.to_bits(), "n {n} phase {ph}");
        }
    }
}

#[test]
fn phase_live_at_or_above_the_bound() {
    // 70 passes of which 8 carry odd items near 2^27: partial products pass 2^53 and the order changes the f64 rounding.
    let mut s = 0x9e3779b9u32;
    let mut active = 0;
    let mut tried = 0;
    for _ in 0..40 {
        let r: Vec<i32> = (0..70)
            .map(|i| {
                s = s.wrapping_mul(1664525).wrapping_add(1013904223);
                if i % 9 == 0 { -(((s >> 4) % (1 << 26)) as i32 + (1 << 26)) } else { 0 }
            })
            .collect();
        let mut mag = 1.0f64;
        for &x in &r {
            mag *= ((2 * x + 1) as f64).abs();
        }
        assert!(mag.is_finite() && mag >= 9007199254740992.0);
        tried += 1;
        let first_phase = radii_product(&r, 0);
        if PHASES.iter().any(|&ph| radii_product(&r, ph).to_bits() != first_phase.to_bits()) {
            active += 1;
        }
    }
    assert!(active > 0, "phase never changed a word in {tried} constructed sets");
}

#[test]
fn small_pass_counts_ignore_the_phase_at_any_size() {
    // P below 16 always takes the scalar loop, so the phase is dead even above the bound.
    let r: Vec<i32> = vec![-(1 << 26) - 3, -(1 << 25) - 7, -(1 << 26) - 11, -(1 << 24) - 1, -(1 << 26) - 5, -(1 << 25) - 9];
    let first_phase = radii_product(&r, 0);
    for &ph in &PHASES {
        assert_eq!(radii_product(&r, ph).to_bits(), first_phase.to_bits());
    }
}
