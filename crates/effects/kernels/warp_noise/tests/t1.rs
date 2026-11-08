use fx_common::shake_natives::hash_2;
use warp_noise::*;

fn tables() -> Tables {
    Tables::build()
}

#[test]
fn weight_table_ends() {
    let t = tables();
    assert_eq!(t.weights[0], 1.0);
    // the first zero entry sits at the limit index, so a tap at exactly the limit adds nothing
    assert_eq!(t.weights[4537], 0.0);
    assert!(t.weights[4536] > 0.0);
    assert_eq!(t.limit.to_bits(), 0x400f_fc00);
}

/// Hand evaluation at the origin, one octave: the nine taps have squared distances 2, 1, 2 / 1, 0, 1 / 2, 1, 2.
#[test]
fn origin_one_octave_by_hand() {
    let t = tables();
    let seed = 503;
    let cell = |cx: i32, cy: i32| {
        let h = hash_2(&t.hash, ((cy as u32) << 4) ^ (seed as u32), cx as u32);
        (((h & 0x1fff) as i32 - 4096) as f32) * f32::from_bits(0x3980_0000)
    };
    let w = |d2: f32| t.weights[(((d2 * 4096.0) as i32) >> 1) as usize];
    let (mut sw, mut acc) = (0.0f32, 0.0f32);
    for jy in 0..3 {
        for ix in 0..3 {
            let d2 = [1.0f32, 0.0, 1.0][jy] + [1.0f32, 0.0, 1.0][ix];
            if t.limit > d2 {
                sw += w(d2);
                acc = w(d2) * cell(ix as i32 - 1, jy as i32 - 1) + acc;
            }
        }
    }
    let want = ((acc.mul_add(1.0 / sw, 0.0) as f64).mul_add(0.5, 0.5)) as f32;
    let got = noise1(&t, 1, seed, 0.0, 0.0, 1.0, 0.0).unwrap();
    assert_eq!(got.to_bits(), want.to_bits());
}

#[test]
fn range_is_unit_for_finite_inputs() {
    let t = tables();
    for k in 0..500 {
        let v = noise1(&t, 1, 503, k as f32 * 0.37, k as f32 * 0.11 - 5.0, 1.0, 0.0).unwrap();
        assert!((0.0..=1.0).contains(&v), "{v}");
    }
}

#[test]
fn deterministic_and_pure() {
    let t = tables();
    let a = noise1(&t, 3, 7, 1.25, -2.5, 1.0, 0.0).unwrap();
    let b = noise1(&t, 3, 7, 1.25, -2.5, 1.0, 0.0).unwrap();
    assert_eq!(a.to_bits(), b.to_bits());
}

#[test]
fn nan_coordinate_gives_default_nan() {
    let t = tables();
    // no tap is inside the limit, the weight sum is zero and 1 / 0 * 0 is the default NaN
    let v = noise1(&t, 1, 503, f32::NAN, 0.0, 1.0, 0.0).unwrap();
    assert_eq!(v.to_bits(), 0xffc0_0000);
}

#[test]
fn overflowing_coordinate_reports_out_of_table() {
    let t = tables();
    assert!(noise1(&t, 1, 503, f32::MAX, 1.0, 1.0, 0.0).is_err());
}

#[test]
fn non_positive_octaves_skip_the_loop() {
    let t = tables();
    // oct 0: the output cell stays as it was, then the remap 0.5 * v + 0.5 applies
    let v = noise1(&t, 0, 503, 1.0, 1.0, 1.0, 0.25).unwrap();
    assert_eq!(v, 0.625);
}

#[test]
fn entry_clamps_octaves_and_scales_by_two_freq() {
    let t = tables();
    let n = Node { freq: [30.0, 0.0], freq_count: 1, octaves: 1, seed: 0.123 };
    let direct = noise1(&t, 1, 503, 0.5 * 60.0, 0.0, 1.0, 0.0).unwrap();
    assert_eq!(eval(&t, &n, 0.5, 0.0).unwrap().to_bits(), direct.to_bits());
    let hi = Node { octaves: 99, ..n };
    let eight = Node { octaves: 8, ..n };
    assert_eq!(eval(&t, &hi, 0.3, 0.1).unwrap().to_bits(), eval(&t, &eight, 0.3, 0.1).unwrap().to_bits());
    let neg = Node { octaves: -4, ..n };
    assert_eq!(eval(&t, &neg, 0.3, 0.1).unwrap().to_bits(), eval(&t, &n, 0.3, 0.1).unwrap().to_bits());
}

#[test]
fn second_frequency_lane_scales_y() {
    let t = tables();
    let n = Node { freq: [10.0, 40.0], freq_count: 2, octaves: 1, seed: 0.5 };
    let direct = noise1(&t, 1, 2048, 0.7 * 20.0, 80.0 * 0.2, 1.0, 0.0).unwrap();
    assert_eq!(eval(&t, &n, 0.7, 0.2).unwrap().to_bits(), direct.to_bits());
}

#[test]
fn assumptions_are_listed() {
    assert_eq!(assumptions().len(), 3);
}

#[test]
fn three_channel_range_and_determinism() {
    let t = tables();
    let n = Node { freq: [60.0, 60.0], freq_count: 1, octaves: 1, seed: 0.4 };
    for k in 0..40 {
        let x = k as f32 * 0.0137;
        let a = eval3(&t, &n, x, 0.0).unwrap();
        assert_eq!(a.map(f32::to_bits), eval3(&t, &n, x, 0.0).unwrap().map(f32::to_bits));
        for v in a {
            assert!((0.0..=1.0).contains(&v));
        }
    }
}

#[test]
fn three_channel_nan_coordinate_is_nan() {
    let t = tables();
    let n = Node { freq: [60.0, 60.0], freq_count: 1, octaves: 1, seed: 0.4 };
    assert!(eval3(&t, &n, f32::NAN, 0.0).unwrap().iter().all(|v| v.is_nan()));
}

/// Every weight from index 4537 on is zero, so the exact value of the limit between 2.2153 and 2.2497 changes no tap.
#[test]
fn weight_tail_is_zero_up_to_the_limit_index() {
    let t = tables();
    assert!(t.weights[4537..].iter().all(|w| *w == 0.0));
    assert_eq!(((t.limit * 4096.0) as i32 >> 1) as usize, 4607);
}

/// Every NaN the evaluation returns without a NaN operand is the x86 default NaN, whatever the processor makes of 0 * inf and inf - inf.
#[test]
fn infinite_coordinates_give_default_nan() {
    let t = tables();
    for (x, y) in [(f32::INFINITY, 0.0), (f32::NEG_INFINITY, 0.0), (0.0, f32::INFINITY), (0.0, f32::NEG_INFINITY), (f32::INFINITY, f32::NEG_INFINITY)] {
        let v = noise1(&t, 2, 503, x, y, 1.0, 0.0).unwrap();
        assert_eq!(v.to_bits(), 0xffc0_0000, "{x} {y}");
        let w = noise3(&t, 2, 503, x, y, 1.0, [0.0; 3]).unwrap();
        assert_eq!(w.map(f32::to_bits), [0xffc0_0000; 3], "{x} {y}");
    }
}

#[test]
fn three_channel_nan_coordinate_gives_default_nan() {
    let t = tables();
    let w = noise3(&t, 1, 503, f32::NAN, 0.0, 1.0, [0.0; 3]).unwrap();
    assert_eq!(w.map(f32::to_bits), [0xffc0_0000; 3]);
}

/// A zero amplitude over a zero weight sum is 0 / 0, a fresh NaN.
#[test]
fn zero_amplitude_over_empty_taps_gives_default_nan() {
    let t = tables();
    assert_eq!(noise1(&t, 1, 503, f32::NAN, 0.0, 0.0, 0.0).unwrap().to_bits(), 0xffc0_0000);
    assert_eq!(noise3(&t, 1, 503, f32::NAN, 0.0, 0.0, [0.0; 3]).unwrap().map(f32::to_bits), [0xffc0_0000; 3]);
}

/// A NaN already in the output cell is the accumulating operand: it wins over the fresh NaN of 0 * inf and keeps its payload and sign.
#[test]
fn output_cell_nan_wins_over_a_fresh_nan() {
    let t = tables();
    for bits in [0x7fc0_1234u32, 0xffc0_0042, 0x7f80_0001] {
        let v = noise1(&t, 1, 503, f32::NAN, 0.0, 1.0, f32::from_bits(bits)).unwrap();
        assert_eq!(v.to_bits(), bits | 0x0040_0000, "{bits:#x}");
        let w = noise3(&t, 1, 503, f32::NAN, 0.0, 1.0, [f32::from_bits(bits); 3]).unwrap();
        assert_eq!(w.map(f32::to_bits), [bits | 0x0040_0000; 3], "{bits:#x}");
    }
}
