//! The constant fold of warp_noise and warp_noise3 composed into the Flicker multiplier, against recorded bindings.
use fx_eval::compile::ArgView2;
use fx_eval::fold::{fold_kind, fold_lanes};
use fx_eval::svml_sin_avx2::sin_rev;

fn c(l: [f32; 4], n: usize) -> ArgView2 {
    ArgView2 { is_const: true, lanes: l, n, elnum: Some(n as i32) }
}
fn s(x: f32) -> ArgView2 {
    c([x; 4], 1)
}
fn f(bits: u32) -> f32 {
    f32::from_bits(bits)
}

/// Host 16.16 value of a text or bound parameter.
fn q(v: f64) -> f32 {
    ((v * 65536.0).round() as f32) / 65536.0
}

/// The Flicker multiplier lanes for one binding: (time, rand_freq, seed field, wave_amp, rand_color_amp).
fn multiplier(time: f32, freq: f32, seed: f32, wave_amp: f32, color_amp: f32) -> [u32; 3] {
    let pos = c([time, 0.0, 0.0, 0.0], 2);
    let w = fold_lanes("warp_noise", &[pos.clone(), s(freq), s(1.0), s(seed)], 1, None).expect("warp_noise folds");
    let seed3 = seed + 0.3333f32;
    let w3 = fold_lanes("warp_noise3", &[pos, s(freq), s(1.0), s(seed3)], 3, None).expect("warp_noise3 folds");
    let amp = q(0.2);
    let wave = sin_rev(time * 5.0 + (0.25f32 - 0.0)) * wave_amp;
    let luma = 1.0f32 * (f(w[0]) - 0.5);
    let mut r = [0u32; 3];
    for k in 0..3 {
        let rand = (luma + color_amp * (f(w3[k]) - 0.5)) * 2.0;
        r[k] = (1.0f32 + (amp * 1.0f32) * (rand + wave)).to_bits();
    }
    r
}

#[test]
fn classes_fold() {
    assert!(fold_kind("warp_noise").is_some());
    assert!(fold_kind("warp_noise3").is_some());
}

#[test]
fn recorded_bindings_fold_to_the_recorded_bits() {
    let seed = 8060.0f32 / 65536.0;
    let wa03 = q(0.3);
    let rows: [(&str, [u32; 3], [u32; 3]); 7] = [
        ("default", multiplier(0.0, 30.0, seed, 0.0, 0.0), [0x3f86e9c8; 3]),
        ("waveamp03", multiplier(0.0, 30.0, seed, wa03, 0.0), [0x3f8e97da; 3]),
        ("randcol1", multiplier(0.0, 30.0, seed, 0.0, 1.0), [0x3f88a855, 0x3f806919, 0x3f8d4a72]),
        ("time7", multiplier(7.0 / 24.0, 30.0, seed, 0.0, 0.0), [0x3f612e85; 3]),
        ("time17", multiplier(17.0 / 24.0, 30.0, seed, 0.0, 0.0), [0x3f631550; 3]),
        ("fs45", multiplier(0.0, 45.0, 0.5, 0.0, 0.0), [0x3f84689b; 3]),
        ("waveamp0", multiplier(0.0, 30.0, seed, 0.0, 0.0), [0x3f86e9c8; 3]),
    ];
    for (name, got, want) in rows {
        assert_eq!(got, want, "{name}: {got:08x?} vs {want:08x?}");
    }
}

#[test]
fn unsupported_shapes_stay_unfolded() {
    let pos1 = s(0.0);
    assert!(fold_lanes("warp_noise", &[pos1, s(30.0), s(1.0), s(0.1)], 1, None).is_none());
    let pos = c([0.0; 4], 2);
    assert!(fold_lanes("warp_noise", &[pos.clone(), s(30.0), s(1.0), s(0.1)], 3, None).is_none());
    assert!(fold_lanes("warp_noise3", &[pos, s(30.0), s(1.0), s(0.1)], 1, None).is_none());
}

fn node(freq: [f32; 2], count: i32, oct: i32, seed: f32) -> warp_noise::Node {
    warp_noise::Node { freq, freq_count: count, octaves: oct, seed }
}

#[test]
fn fold_follows_the_class_entry() {
    let t = warp_noise::Tables::build();
    let pos = c([0.37, 0.21, 0.0, 0.0], 2);
    // octave counts, a two-lane frequency and the 3-lane form
    for oct in [1.0f32, 2.0, 3.0, 9.0, 0.0] {
        let want = warp_noise::eval(&t, &node([30.0, 30.0], 1, warp_noise::cvtt(oct), 0.4), 0.37, 0.21).unwrap();
        let got = fold_lanes("warp_noise", &[pos.clone(), s(30.0), s(oct), s(0.4)], 1, None).unwrap();
        assert_eq!(got[0], want.to_bits(), "octaves {oct}");
        let want3 = warp_noise::eval3(&t, &node([30.0, 30.0], 1, warp_noise::cvtt(oct), 0.4), 0.37, 0.21).unwrap();
        let got3 = fold_lanes("warp_noise3", &[pos.clone(), s(30.0), s(oct), s(0.4)], 3, None).unwrap();
        assert_eq!([got3[0], got3[1], got3[2]], want3.map(f32::to_bits), "octaves {oct}");
    }
    let fq = c([12.0, 40.0, 0.0, 0.0], 2);
    let want = warp_noise::eval(&t, &node([12.0, 40.0], 2, 1, 0.4), 0.37, 0.21).unwrap();
    let got = fold_lanes("warp_noise", &[pos.clone(), fq, s(1.0), s(0.4)], 1, None).unwrap();
    assert_eq!(got[0], want.to_bits());
    // the position lanes are x then y
    let want = warp_noise::eval(&t, &node([30.0, 30.0], 1, 1, 0.4), 0.21, 0.37).unwrap();
    let got = fold_lanes("warp_noise", &[c([0.21, 0.37, 0.0, 0.0], 2), s(30.0), s(1.0), s(0.4)], 1, None).unwrap();
    assert_eq!(got[0], want.to_bits());
}

#[test]
fn short_argument_lists_and_out_of_table_stay_unfolded() {
    let pos = c([0.1, 0.0, 0.0, 0.0], 2);
    assert!(fold_lanes("warp_noise", &[pos.clone(), s(30.0), s(1.0)], 1, None).is_none());
    assert!(fold_lanes("warp_noise3", &[pos], 3, None).is_none());
    let t = warp_noise::Tables::build();
    let mut seen_err = false;
    for x in [1.0e9f32, 1.0e12, 1.0e20, 1.0e30, 3.0e38] {
        let direct = warp_noise::eval(&t, &node([0.5, 0.5], 1, 1, 0.4), x, 0.0);
        let got = fold_lanes("warp_noise", &[c([x, 0.0, 0.0, 0.0], 2), s(0.5), s(1.0), s(0.4)], 1, None);
        assert_eq!(got.is_some(), direct.is_ok(), "x {x}");
        seen_err |= direct.is_err();
    }
    assert!(seen_err, "no out-of-table position found");
}

#[test]
fn value_classes_do_not_panic() {
    let vals = [0.0f32, -0.0, f32::NAN, -f32::NAN, f32::INFINITY, f32::NEG_INFINITY, f32::MIN_POSITIVE / 4.0, 1.0e-40, f32::MAX, -f32::MAX, 1.0e7, -37.5];
    for &x in &vals {
        for &fr in &vals {
            for &sd in &[0.0f32, 0.123, f32::NAN, f32::INFINITY, -1.0e30] {
                let a1 = [c([x, 0.0, 0.0, 0.0], 2), s(fr), s(1.0), s(sd)];
                let _ = fold_lanes("warp_noise", &a1, 1, None);
                let _ = fold_lanes("warp_noise3", &a1, 3, None);
            }
        }
    }
}
