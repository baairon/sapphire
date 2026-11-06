//! Device free tests (every OS): the kernel transcriptions run by the simulator in shuffled work item orders give
//! the same bits as the plain Rust expressions the CPU kernels use, over the adversarial corpus.

use fx_device::kernels::rs;
use fx_device::selfcheck::{adversarial_f32, chain_pairs, SIZES};
use fx_device::sim::{launch_1d, same_for_every_order, Global};

#[test]
fn one_minus_and_copy_do_not_depend_on_the_work_item_order() {
    let words = adversarial_f32(3000, 5);
    for &n in SIZES {
        let src: Vec<f32> = (0..n).map(|i| words[i % words.len()]).collect();
        let bits: Vec<u32> = src.iter().map(|f| f.to_bits()).collect();
        let om = same_for_every_order(|seed| {
            let mut out = Global::new(n, 0u32);
            launch_1d(n, seed, |g| out.set(g, rs::one_minus_f32_item(&src, g).to_bits()));
            out.into_vec()
        });
        let cp = same_for_every_order(|seed| {
            let mut out = Global::new(n, 0u32);
            launch_1d(n, seed, |g| out.set(g, rs::copy_f32_item(&bits, g)));
            out.into_vec()
        });
        assert_eq!(cp, bits);
        for i in 0..n {
            assert_eq!(om[i], (1.0f32 - src[i]).to_bits(), "n={n} i={i}");
        }
    }
}

#[test]
fn the_nan_scan_sums_to_the_host_answer() {
    let words = adversarial_f32(500, 6);
    let flag = same_for_every_order(|seed| {
        let mut any = 0u32;
        launch_1d(words.len(), seed, |g| any |= rs::any_nan_f32_item(&words, g));
        any
    });
    assert_eq!(flag, 1);
    let clean = [1.0f32, f32::INFINITY];
    let mut any = 0;
    launch_1d(2, 0, |g| any |= rs::any_nan_f32_item(&clean, g));
    assert_eq!(any, 0);
}

#[test]
fn the_chain_transcription_is_unfused() {
    for (x, k) in chain_pairs(30, 9) {
        let mut v = 0.0f32;
        for _ in 0..64 {
            v = v * k + x;
        }
        assert_eq!(rs::chain_f32_item(x, k).to_bits(), v.to_bits());
    }
}

#[test]
fn the_prelude_conversions_match_the_x86_cvtt_rules() {
    use fx_device::prelude::rs::*;
    assert_eq!(cvtt_i32_f(f32::NAN), i32::MIN);
    assert_eq!(cvtt_i32_f(2147483648.0), i32::MIN);
    assert_eq!(cvtt_i32_f(-2147483648.0), i32::MIN);
    assert_eq!(cvtt_i32_f(-1.9), -1);
    assert_eq!(cvtt_i32_f(2147483520.0), 2147483520);
    assert_eq!(cvtt_i64_f(f32::INFINITY), i64::MIN);
    assert_eq!(cvtt_i32_d(-2147483648.9), i32::MIN);
    assert_eq!(cvtt_i32_d(-2147483647.9), -2147483647);
    assert_eq!(cvtt_i64_d(f64::NAN), i64::MIN);
}
