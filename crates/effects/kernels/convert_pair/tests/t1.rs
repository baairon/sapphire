use convert_pair::*;

fn planes_of(mem: &[f32], n: usize) -> [Vec<f32>; 4] {
    let mut p: [Vec<f32>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];
    convert_from_source_line(mem, &mut p, n);
    p
}

#[test]
fn reader_premultiplies_and_reorders() {
    // memory B,G,R,A = 0.5, 0.25, 1.0, 0.5 -> planes R,G,B,A = 0.5, 0.125, 0.25, 0.5
    let p = planes_of(&[0.5, 0.25, 1.0, 0.5], 1);
    assert_eq!((p[0][0], p[1][0], p[2][0], p[3][0]), (0.5, 0.125, 0.25, 0.5));
}

#[test]
fn reader_replaces_non_finite_samples_by_zero_before_the_multiply() {
    let p = planes_of(&[1.0, -2.0, f32::INFINITY, f32::NAN], 1);
    assert_eq!(p[3][0].to_bits(), 0.0f32.to_bits());
    assert_eq!(p[0][0].to_bits(), 0.0f32.to_bits()); // inf -> +0.0, * +0.0
    assert_eq!(p[1][0].to_bits(), (-0.0f32).to_bits()); // -2.0 * +0.0 = -0.0
    let q = planes_of(&[1.0, 1.0, f32::NAN, 0.5], 1);
    assert_eq!(q[0][0].to_bits(), 0.0f32.to_bits());
    let r = planes_of(&[1.0, 1.0, 1.0, f32::NEG_INFINITY], 1);
    assert_eq!(r[3][0].to_bits(), 0.0f32.to_bits());
}

#[test]
fn reader_overflow_gives_infinity_not_nan() {
    let p = planes_of(&[0.0, 0.0, 3.0e38, 4.0], 1);
    assert_eq!(p[0][0], f32::INFINITY);
    assert!(!p[1][0].is_nan());
}

#[test]
fn alpha_clamp_table() {
    assert_eq!(clamp_alpha(-0.5).to_bits(), 0.0f32.to_bits());
    assert_eq!(clamp_alpha(2.0), 1.0);
    assert_eq!(clamp_alpha(0.5), 0.5);
    assert_eq!(clamp_alpha(-0.0).to_bits(), (-0.0f32).to_bits());
    assert_eq!(clamp_alpha(1.0), 1.0);
    let n = f32::from_bits(0x7fc0_1234);
    assert_eq!(clamp_alpha(n).to_bits(), 0x7fc0_1234);
    assert_eq!(clamp_alpha(f32::INFINITY), 1.0);
    assert_eq!(clamp_alpha(f32::NEG_INFINITY).to_bits(), 0.0f32.to_bits());
    assert_eq!(clamp_alpha(1.0e-40).to_bits(), 1.0e-40f32.to_bits());
}

#[test]
fn writer_modes() {
    let planes: [Vec<f32>; 4] = [vec![0.1], vec![0.2], vec![0.3], vec![7.0]];
    let mut d = vec![f32::from_bits(0xaaaa_aaaa); 4];
    let o = convert_to_dest_line(&planes, ToDestParams { mode: AlphaMode::ClampKeep }, &mut d, 1);
    assert_eq!(o, DestOutcome::Written);
    assert_eq!(d, vec![0.3, 0.2, 0.1, 1.0]);
    let mut d = vec![f32::from_bits(0xaaaa_aaaa); 4];
    let o = convert_to_dest_line(&planes, ToDestParams { mode: AlphaMode::NoWrite }, &mut d, 1);
    assert_eq!(o, DestOutcome::NotWritten);
    assert!(d.iter().all(|x| x.to_bits() == 0xaaaa_aaaa));
    let mut d = vec![0.0; 4];
    convert_to_dest_line(&planes, ToDestParams { mode: AlphaMode::One }, &mut d, 1);
    assert_eq!(d[3], 1.0);
    convert_to_dest_line(&planes, ToDestParams { mode: AlphaMode::ZeroAlpha }, &mut d, 1);
    assert_eq!(d[3], 0.0);
}

#[test]
fn alpha_mode_derivation() {
    if let Ok(v) = std::env::var("CV_MUTANT") {
        convert_pair::set_mutant(v.parse().unwrap());
    }
    // the program record of Hue_Sat_Bright holds 2 (MEASURED); the harness leaves ctx+0x758 = 0 -> mode 3
    assert_eq!(derive_alpha_mode(0, 2, 0, 0, 4), 3);
    assert_eq!(derive_alpha_mode(0, 2, 1, 3, 4), 1);
    assert_eq!(derive_alpha_mode(0, 2, 1, 4, 4), 2);
    assert_eq!(derive_alpha_mode(0, 1, 0, 0, 4), 1);
    assert_eq!(derive_alpha_mode(0, 1, 0, 0, 3), 3);
    assert_eq!(derive_alpha_mode(1, 2, 0, 0, 4), 1); // owner override wins
    assert_eq!(derive_alpha_mode(0, 0, 0, 0, 4), 0);
}

#[test]
fn band_replication_limits() {
    let w = 2usize;
    let h = 6usize;
    let mut src = vec![0.0f32; 4 * w * h];
    for y in 0..h {
        for i in 0..4 * w {
            src[4 * w * y + i] = (y * 10 + 1) as f32 * 0.25;
        }
    }
    let mut d = vec![f32::from_bits(0xaaaa_aaaa); 4 * w * h];
    let calls = convert_pair_frame(&src, w, h, 4, ToDestParams { mode: AlphaMode::One }, &mut d);
    assert_eq!(calls, 2);
    // rows 0..3 equal the converted row 0 (alpha forced to 1 and colour premultiplied by the input alpha), rows 4..5 equal converted row 4
    for y in 1..4 {
        assert_eq!(d[4 * w * y..4 * w * (y + 1)], d[0..4 * w]);
    }
    assert_eq!(d[4 * w * 5..4 * w * 6], d[4 * w * 4..4 * w * 5]);
    assert_ne!(d[0..4 * w], d[4 * w * 4..4 * w * 5]);
}

#[test]
fn opaque_roundtrip_is_identity_for_finite_inputs() {
    let mem = [0.1f32, 0.2, 0.3, 1.0, 0.4, 0.5, 0.6, 1.0];
    let p = planes_of(&mem, 2);
    let mut d = vec![0.0f32; 8];
    convert_to_dest_line(&p, ToDestParams { mode: AlphaMode::ClampKeep }, &mut d, 2);
    assert_eq!(&d[..], &mem[..]);
}

#[test]
fn deterministic_and_finite_in_finite_out() {
    let mem: Vec<f32> = (0..64).map(|i| (i as f32) * 0.037 - 0.3).collect();
    let a = planes_of(&mem, 16);
    let b = planes_of(&mem, 16);
    assert_eq!(a, b);
    assert!(a.iter().all(|p| p.iter().all(|x| x.is_finite())));
}
