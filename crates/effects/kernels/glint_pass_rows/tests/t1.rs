use glint_pass_rows::*;

fn phases() -> Alignment {
    Alignment { scratch: [0, 0], radii: 0 }
}
fn run1(src: &[f32], w: usize, rows: usize, cfg: &Config) -> Vec<f32> {
    let mut dst = vec![f32::from_bits(0x7fc0ffee); w * rows];
    run(cfg, &phases(), src, &mut dst, w, rows).unwrap();
    dst
}

#[test]
fn no_pass_is_copy() {
    let s = [1.0f32, 2.0, 3.0];
    let d = run1(&s, 3, 1, &Config { edge: 0, radii: &[], shift: 0 });
    assert_eq!(d, s);
}

#[test]
fn zero_pad_radius_one() {
    // window of 3, scale 1/3
    let s = [1.0f32, 2.0, 3.0, 4.0];
    let d = run1(&s, 4, 1, &Config { edge: 0, radii: &[1], shift: 0 });
    let sc = (1.0f64 / 3.0) as f32;
    let want = [3.0f32, 6.0, 9.0, 7.0].map(|x| x * sc);
    assert_eq!(d, want);
}

#[test]
fn mirror_radius_one() {
    let s = [1.0f32, 2.0, 3.0, 4.0];
    let d = run1(&s, 4, 1, &Config { edge: 1, radii: &[1], shift: 0 });
    let sc = (1.0f64 / 3.0) as f32;
    let want = [4.0f32, 6.0, 9.0, 11.0].map(|x| x * sc);
    assert_eq!(d, want);
}

#[test]
fn width_one_and_height_one() {
    let d = run1(&[5.0], 1, 1, &Config { edge: 1, radii: &[0, -2], shift: 0 });
    assert_eq!(d, [5.0 * ((1.0f64 / -3.0) as f32)]);
    let d = run1(&[1.0, 2.0, 3.0], 1, 3, &Config { edge: 0, radii: &[], shift: 0 });
    assert_eq!(d, [1.0, 2.0, 3.0]);
}

#[test]
fn shift_blend_two_tap() {
    let s = [1.0f32, 2.0, 3.0, 4.0, 5.0];
    let d = run1(&s, 5, 1, &Config { edge: 0, radii: &[], shift: 2 });
    assert_eq!(d, [1.5, 2.0, 3.0, 1.0, 1.5]);
    let d = run1(&s, 5, 1, &Config { edge: 0, radii: &[], shift: -9 });
    assert_eq!(d, [0.0; 5]);
    let d = run1(&s, 5, 1, &Config { edge: 0, radii: &[], shift: 4 });
    assert_eq!(d, [2.5, 0.0, 0.0, 0.0, 0.5]);
}

#[test]
fn errors() {
    let mut dst = [0.0f32; 4];
    assert_eq!(run(&Config { edge: 2, radii: &[1], shift: 0 }, &phases(), &[0.0; 4], &mut dst, 4, 1), Err(Error::BadEdge(2)));
    assert_eq!(run(&Config { edge: 0, radii: &[2], shift: 0 }, &phases(), &[0.0; 4], &mut dst, 4, 1), Err(Error::RadiusTooLarge { radius: 2, width: 4 }));
    assert_eq!(run(&Config { edge: 0, radii: &[], shift: 0 }, &phases(), &[0.0; 2], &mut dst, 4, 1), Err(Error::ShortBuffer));
    // an unused bad edge mode is fine
    assert!(run(&Config { edge: 7, radii: &[0], shift: 0 }, &phases(), &[0.0; 4], &mut dst, 4, 1).is_ok());
}

#[test]
fn nan_propagates_and_is_deterministic() {
    let mut s = vec![1.0f32; 9];
    s[4] = f32::from_bits(0x7fc00001);
    let c = Config { edge: 1, radii: &[2, 1], shift: 0 };
    let a = run1(&s, 9, 1, &c);
    let b = run1(&s, 9, 1, &c);
    assert_eq!(a.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), b.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
    assert!(a.iter().any(|x| x.is_nan()));
}

#[test]
fn scale_product_orders() {
    // large radii overflow the product to infinity: scale 0
    let radii = vec![0x7fffffffi32 / 2; 40];
    let mut dst = [1.0f32; 3];
    run(&Config { edge: 0, radii: &radii, shift: 0 }, &phases(), &[1.0; 3], &mut dst, 3, 1).ok();
}

#[test]
fn alignment_changes_only_the_order() {
    let s: Vec<f32> = (0..200).map(|i| ((i * 37 % 101) as f32) * 0.37 - 11.0).collect();
    let c = Config { edge: 0, radii: &[70], shift: 0 };
    let mut dist_a = vec![0f32; 200];
    let mut dist_b = vec![0f32; 200];
    run(&c, &Alignment { scratch: [0, 0], radii: 0 }, &s, &mut dist_a, 200, 1).unwrap();
    run(&c, &Alignment { scratch: [16, 16], radii: 0 }, &s, &mut dist_b, 200, 1).unwrap();
    for (a, b) in dist_a.iter().zip(&dist_b) {
        assert!((a - b).abs() < 1e-3);
    }
}

#[test]
fn assumptions_listed() {
    assert_eq!(assumptions().len(), 4);
    assert!(assumptions().iter().all(|a| !a.starts_with("NAN-X86-MODEL") && !a.starts_with("VEC-HALF-CONST")));
}

fn fb(b: u32) -> f32 {
    f32::from_bits(b)
}
const QA: u32 = 0x7fc0_0011;
const QB: u32 = 0xffc0_0022;
const SA: u32 = 0x7f80_0033;
const SB: u32 = 0xff80_0044;

#[test]
fn two_nan_blend_add_first_operand_wins() {
    // w=5, m=1: dst[2] = 0.5 * (buf[3] + buf[1]); the add's first source is buf[i+m]
    for (x, y) in [(QA, QB), (QB, QA), (SA, SB), (SB, SA), (QA, SB), (SA, QB), (SB, QA)] {
        let s = [0.0f32, fb(y), 0.0, fb(x), 0.0];
        let d = run1(&s, 5, 1, &Config { edge: 0, radii: &[], shift: 1 });
        assert_eq!(d[2].to_bits(), x | (1u32 << 22), "x {x:08x} y {y:08x}");
    }
}

#[test]
fn two_nan_scale_multiply_first_operand_wins() {
    // the final scale multiplies (scale, sample): the scale is finite here, so the sample NaN passes quieted with its payload
    let s = [fb(SA), 1.0, 1.0, 1.0];
    let d = run1(&s, 4, 1, &Config { edge: 0, radii: &[], shift: 0 });
    assert_eq!(d[0].to_bits(), SA | (1u32 << 22));
}

#[test]
fn two_nan_window_sum_and_fused_edge() {
    // mirrored edge radius 1: acc = fms213(2.0, s, src[1]) with s = src[0] + src[1]; both NaN: the add keeps src[0], the fused op keeps s
    let s = [fb(QA), fb(QB), 1.0, 1.0];
    let d = run1(&s, 4, 1, &Config { edge: 1, radii: &[1], shift: 0 });
    assert_eq!(d[0].to_bits(), QA);
    let s = [fb(SB), fb(SA), 1.0, 1.0];
    let d = run1(&s, 4, 1, &Config { edge: 1, radii: &[1], shift: 0 });
    assert_eq!(d[0].to_bits(), SB | (1u32 << 22));
    // only the subtracted operand NaN
    let s = [1.0, fb(SA), 1.0, 1.0];
    let d = run1(&s, 4, 1, &Config { edge: 1, radii: &[1], shift: 0 });
    assert!(d[0].is_nan());
}

#[test]
fn fresh_nan_is_default() {
    let s = [f32::INFINITY, f32::NEG_INFINITY, 1.0, 1.0];
    let d = run1(&s, 4, 1, &Config { edge: 0, radii: &[1], shift: 0 });
    assert!(d.iter().any(|x| x.is_nan()));
    assert!(d.iter().filter(|x| x.is_nan()).all(|x| x.to_bits() == 0xffc0_0000));
    let d = run1(&[f32::INFINITY, 1.0, f32::NEG_INFINITY], 3, 1, &Config { edge: 0, radii: &[], shift: 1 });
    assert_eq!(d[1].to_bits(), 0xffc0_0000);
}

#[test]
fn single_nan_and_finite_unchanged() {
    let s = [1.0f32, fb(SA), 3.0, 4.0, 5.0];
    let d = run1(&s, 5, 1, &Config { edge: 0, radii: &[], shift: 1 });
    assert_eq!(d[0].to_bits(), SA | (1u32 << 22));
    assert_eq!(d[3], 4.0);
}

#[test]
fn mirrored_left_edge_is_fused() {
    // mirrored edge radius 1: acc = 2.0 * s - src[1] with s = src[0] + src[1] and one rounding. Here 2.0 * s overflows binary32
    // (4e38) while 2.0 * s - 1e38 is finite, so a rounded multiply followed by a subtract would give infinity.
    let big = 1.0e38f32;
    let s = [big, big, 0.0, 0.0];
    let d = run1(&s, 4, 1, &Config { edge: 1, radii: &[1], shift: 0 });
    let sum = big + big;
    let acc = (2.0f64 * sum as f64 - big as f64) as f32;
    assert!(acc.is_finite());
    let scale = (1.0f64 / 3.0) as f32;
    assert_eq!(d[0].to_bits(), (scale * acc).to_bits());
    assert!(d[0].is_finite());
}
