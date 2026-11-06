//! the original sin / cos fold kernel (Intel SVML sinf4 / cosf4 SSE variants), hand values measured in the original compiler's plans.
use fx_eval::svml_trig::{cos_rev, cosf_svml, sin_rev, sinf_svml};

fn b(x: f32) -> u32 { x.to_bits() }

#[test]
fn measured_warp_waves2_values() {
    // EP-3d PLAN_ORIG_Warp_Waves2_a: NODE 33 cos lanes f304353f, NODE 34 sin lanes f404353f, NODE 55 cos eb46773f, NODE 56 sin ee83843e
    assert_eq!(b(cos_rev(0.125)), 0x3f3504f3);
    assert_eq!(b(sin_rev(0.125)), 0x3f3504f4);
    let x15 = f32::from_bits(0x3d2aaaab) ; // 1/24 revolution = 15 degrees (f32(15)/f32(360))
    assert_eq!(b(cos_rev(15.0f32 / 360.0)), 0x3f7746eb);
    assert_eq!(b(sin_rev(15.0f32 / 360.0)), 0x3e8483ee);
    let _ = x15;
}

#[test]
fn emulator_hand_values() {
    // emulated original (tools/emu_run.py quick): input bits -> output bits
    assert_eq!(b(sinf_svml(f32::from_bits(0x3f490fdb))), 0x3f3504f4);
    assert_eq!(b(cosf_svml(f32::from_bits(0x3f490fdb))), 0x3f3504f3);
    assert_eq!(b(sinf_svml(f32::from_bits(0x3e860a92))), 0x3e8483ee);
    assert_eq!(b(cosf_svml(f32::from_bits(0x3e860a92))), 0x3f7746eb);
    assert_eq!(b(sinf_svml(1.0)), 0x3f576aa4);
    assert_eq!(b(cosf_svml(1.0)), 0x3f0a5140);
    assert_eq!(b(sinf_svml(f32::from_bits(0x4640e6b6))), 0xbf344b09);
    assert_eq!(b(cosf_svml(f32::from_bits(0x4640e6b6))), 0x3f35be20);
    assert_eq!(b(sinf_svml(f32::from_bits(0x4e6e6b28))), 0x3f0bbc66);
    assert_eq!(b(cosf_svml(f32::from_bits(0x4e6e6b28))), 0x3f567fc7);
}

/// The copies of the scalar routine's tables, `VEC_DIR/effects/native_tables/svml_{sin,cos}_tab.bin`; None (with a NOT RUN line) when absent.
fn copies(test: &str) -> Option<(Vec<u8>, Vec<u8>)> {
    let rd = |n: &str| std::fs::read(effects_vecdir::path(&format!("native_tables/{}", n))).ok();
    match (rd("svml_sin_tab.bin"), rd("svml_cos_tab.bin")) {
        (Some(a), Some(b)) if a.len() == 2304 && b.len() == 2304 => Some((a, b)),
        _ => {
            println!("NOT RUN {}: VEC_DIR/effects/native_tables/svml_sin_tab.bin and svml_cos_tab.bin are absent", test);
            None
        }
    }
}

#[test]
fn huge_argument_hand_values_need_no_data() {
    let x = f32::from_bits(0x7149f2ca);
    assert_eq!(b(sinf_svml(x)), 0xbf4a89b0);
    assert_eq!(b(cosf_svml(x)), 0xbf1c9222);
}

#[test]
fn computed_regions_equal_the_copies() {
    let Some((st, ct)) = copies("computed_regions_equal_the_copies") else { return };
    let a = libm_sincos::exact::svml_a();
    let w = |t: &[u8], i: usize| u64::from_le_bytes(t[8 * i..8 * i + 8].try_into().unwrap());
    for j in 0..=64usize {
        for h in 0..2 {
            assert_eq!(a.sin_pairs[j][h], w(&st, 2 * j + h), "sin pair {} word {}", j, h);
            assert_eq!(a.cos_pairs[j][h], w(&ct, 2 * j + h), "cos pair {} word {}", j, h);
        }
    }
    for j in 0..15usize {
        for h in 0..2 {
            assert_eq!(a.chunks[j][h], w(&st, 130 + 2 * j + h), "sin chunk {} word {}", j, h);
            assert_eq!(a.chunks[j][h], w(&ct, 130 + 2 * j + h), "cos chunk {} word {}", j, h);
        }
    }
    for t in [&st, &ct] {
        assert_eq!(a.c540, w(t, 0x540 / 8));
        assert_eq!(a.c548, w(t, 0x548 / 8));
        assert_eq!(0xffff_ffff_ff00_0000u64, w(t, 0x550 / 8));
    }
}

#[test]
fn specials() {
    assert_eq!(b(sinf_svml(0.0)), 0);
    assert_eq!(b(sinf_svml(-0.0)), 0); // the original computes (-0) - 0 ... + (-0) = +0 (checked natively over all 2^32 inputs)
    assert_eq!(b(cosf_svml(0.0)), 0x3f80_0000);
    assert_eq!(b(sinf_svml(f32::INFINITY)), 0xffc0_0000);
    assert_eq!(b(cosf_svml(f32::NEG_INFINITY)), 0xffc0_0000);
    assert!(sinf_svml(f32::NAN).is_nan());
}
