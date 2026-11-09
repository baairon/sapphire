//! Rust-only tests of the NaN flags of job_1's line passes: which written elements make the plain pass hand a line to the NaN-rule pass.
use edge_aware_sharpen_weights::*;

const F8_NAN: u32 = 0x7fc0_0111;
const K_NAN: u32 = 0xffc0_0222;

fn ctx(w: i32, s: i32, flag_cell_b: i32, f8: f32, k: f32) -> Ctx {
    Ctx { w, s, flag_cell_b, f0: 1.0, f4: 0.5, f8, fc: 0.25, esh2_job_ctx_a: 0.5, k }
}

fn run(c: &Ctx, p0: &[f32], p1: &[f32], cur: &[f32]) -> (Vec<f32>, Vec<f32>) {
    let total = (c.w * c.s) as usize;
    let mut a = vec![0.0f32; total];
    let mut t = vec![0.0f32; total];
    let mut b = Bufs { p0, p1, cur, a: &mut a, t: &mut t };
    job_1(c, 0, c.s, 0, &mut b).unwrap();
    (a, t)
}

/// One element per line, two NaN sources (f8 and k), finite planes. Guide mode takes the addend (k derived gain) first.
#[test]
fn first_t_guide_two_nan_sources() {
    let c = ctx(1, 3, 0, f32::from_bits(F8_NAN), f32::from_bits(K_NAN));
    let (_, t) = run(&c, &[0.2, 0.3, 0.4], &[0.1, 0.5, 0.9], &[0.3, 0.4, 0.6]);
    assert_eq!(t[1].to_bits(), K_NAN);
    assert_eq!(t[2].to_bits(), K_NAN);
}

/// Constant mode takes the plane value, then the scale (f8), then the addend.
#[test]
fn first_t_constant_two_nan_sources() {
    let c = ctx(1, 3, 1, f32::from_bits(F8_NAN), f32::from_bits(K_NAN));
    let (_, t) = run(&c, &[0.2, 0.3, 0.4], &[0.1, 0.5, 0.9], &[0.3, 0.4, 0.6]);
    assert_eq!(t[1].to_bits(), F8_NAN);
    assert_eq!(t[2].to_bits(), F8_NAN);
}

/// An infinite plane value times a zero scale gives a NaN no operand supplies, in later elements of a line while the line's first t and every a stay
/// non NaN. The NaN is the canonical 0xffc00000 on every processor.
fn inf_cases(flag_cell_b: i32) {
    let c = ctx(3, 3, flag_cell_b, 0.0, 0.5);
    let inf = f32::INFINITY;
    let p0 = [0.5f32; 9];
    let mut p1 = [0.5f32; 9];
    p1[4] = inf;
    p1[7] = -inf;
    let cur = [0.3f32; 9];
    let (a, t) = run(&c, &p0, &p1, &cur);
    assert!(a.iter().all(|x| !x.is_nan()));
    assert!(!t[1].is_nan() && !t[2].is_nan());
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
