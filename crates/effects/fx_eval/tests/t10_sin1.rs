//! SIN1: the original sin / cos fold kernel (Intel SVML sinf4 / cosf4 SSE variants), hand values measured in the original compiler's plans.
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
    if !effects_rtdata::have_tables() {
        return;
    }
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
    assert_eq!(b(sinf_svml(f32::from_bits(0x7149f2ca))), 0xbf4a89b0);
    assert_eq!(b(cosf_svml(f32::from_bits(0x7149f2ca))), 0xbf1c9222);
}

#[test]
fn specials() {
    if !effects_rtdata::have_tables() {
        return;
    }
    assert_eq!(b(sinf_svml(0.0)), 0);
    assert_eq!(b(sinf_svml(-0.0)), 0); // the original computes (-0) - 0 ... + (-0) = +0 (checked natively over all 2^32 inputs)
    assert_eq!(b(cosf_svml(0.0)), 0x3f80_0000);
    assert_eq!(b(sinf_svml(f32::INFINITY)), 0xffc0_0000);
    assert_eq!(b(cosf_svml(f32::NEG_INFINITY)), 0xffc0_0000);
    assert!(sinf_svml(f32::NAN).is_nan());
}
