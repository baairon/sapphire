//! Rust-only tests of the NaN flags of job_2's line passes: which written elements make the plain pass hand a line to the NaN-rule pass.
//! Each case isolates one flag: only the element the flag guards is NaN (or, for the first element, the only element of the line).
use edge_aware_sharpen_jobs::*;

const F8_NAN: u32 = 0x7fc0_0111;
const K_NAN: u32 = 0xffc0_0222;

fn ctx(w: i32, s: i32, flag_cell_b: i32, f8: f32, k: f32) -> Ctx {
    Ctx { w, s, nch: 1, flag_cell_b, f0: 1.0, f4: 0.5, f8, fc: 0.25, esh2_job_ctx_a: 0.5, k }
}

fn run(c: &Ctx, ch0: &[f32], cur: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let total = (c.w * c.s) as usize;
    let zero = vec![0.0f32; total];
    let mut a = vec![0.0f32; total];
    let mut t = vec![0.0f32; total];
    let mut b = Bufs { ch: [ch0, &zero, &zero, &zero], cur, a: &mut a, t: &mut t };
    job_2(c, 0, c.s, &mut b).unwrap();
    (a, t)
}

/// One element per line: the first t of each line is the only t. Guide mode, the t path holds two NaN sources (f8 and k) and the planes are finite;
/// the fused step takes the addend (the k derived gain) first.
#[test]
fn first_t_guide_two_nan_sources() {
    let c = ctx(1, 3, 0, f32::from_bits(F8_NAN), f32::from_bits(K_NAN));
    let (_, t) = run(&c, &[0.1, 0.5, 0.9], &[0.3, 0.4, 0.6]);
    assert_eq!(t[1].to_bits(), K_NAN);
    assert_eq!(t[2].to_bits(), K_NAN);
}

/// Constant mode of the same case: the fused step takes the plane distance, then the scale (f8), then the addend (k derived).
#[test]
fn first_t_constant_two_nan_sources() {
    let c = ctx(1, 3, 1, f32::from_bits(F8_NAN), f32::from_bits(K_NAN));
    let (_, t) = run(&c, &[0.1, 0.5, 0.9], &[0.3, 0.4, 0.6]);
    assert_eq!(t[1].to_bits(), F8_NAN);
    assert_eq!(t[2].to_bits(), F8_NAN);
}

/// An infinite plane distance times a zero scale gives a NaN that no operand supplies, in the later elements of a line while the line's first t and
/// every a stay non NaN. The NaN is the canonical 0xffc00000 on every processor.
fn inf_cases(flag_cell_b: i32) {
    let c = ctx(3, 3, flag_cell_b, 0.0, 0.5);
    let inf = f32::INFINITY;
    let ch0 = [0.0, 0.0, 0.0, 0.0, inf, 0.0, 0.0, 0.0, 0.0];
    let cur = [0.3f32; 9];
    let (a, t) = run(&c, &ch0, &cur);
    assert!(a.iter().all(|x| !x.is_nan()));
    assert!(!t[1].is_nan() && !t[2].is_nan());
    // t[j * s + line] for the elements with an infinite distance to the previous line
    assert_eq!(t[3 + 1].to_bits(), 0xffc0_0000);
    assert_eq!(t[3 + 2].to_bits(), 0xffc0_0000);
}

#[test]
fn later_t_fresh_nan_guide() {
    inf_cases(0);
}

#[test]
fn later_t_fresh_nan_constant() {
    inf_cases(1);
}
