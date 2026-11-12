use scaled_screen_blend::*;

/// The quiet bit of an f32 NaN payload.
const QUIET_BIT: u32 = 1 << 22;

fn run(d: &[f32], s: &[f32], k: f32) -> Vec<f32> {
    let mut v = d.to_vec();
    blend(&mut v, s, k);
    v
}

#[test]
fn hand_cases_scaled() {
    // t = 0.5 * 0.5 = 0.25; dst = (1 - 0.25) * 0.5 + 0.25 = 0.625
    assert_eq!(run(&[0.5], &[0.5], 0.5), vec![0.625]);
    // t = 2 * 0.75 = 1.5 >= 1: saturate to 1.0
    assert_eq!(run(&[0.3], &[0.75], 2.0), vec![1.0]);
    // t exactly 1.0 saturates (1.0 > t is false)
    assert_eq!(run(&[0.3], &[0.5], 2.0), vec![1.0]);
    // non-positive and NaN sources leave the destination alone
    let r = run(&[0.1, 0.2, 0.3, 0.4], &[0.0, -0.0, -1.0, f32::NAN], 0.5);
    assert_eq!(r, vec![0.1, 0.2, 0.3, 0.4]);
}

#[test]
fn hand_cases_plain() {
    // s = 0.5, d = 0.5: f = 0.5*0.5+0.5 = 0.75; max(d,f) = 0.75; max(s,.) = 0.75
    assert_eq!(run(&[0.5], &[0.5], 1.0), vec![0.75]);
    // s >= 1 saturates
    assert_eq!(run(&[0.5], &[1.0], 1.0), vec![1.0]);
    assert_eq!(run(&[0.5], &[3.0], 1.0), vec![1.0]);
    // dst negative: f = -1*(0.5)+0.5 = 0; max(d,f)=0; max(0.5,0)=0.5
    assert_eq!(run(&[-1.0], &[0.5], 1.0), vec![0.5]);
}

#[test]
fn nan_scale_takes_scaled_route() {
    let r = run(&[0.5], &[0.5], f32::NAN);
    assert_eq!(r[0], 1.0); // 1.0 > NaN is false: saturate
}

#[test]
fn nan_dst_propagates_in_plain_route() {
    // d NaN: f = NaN; max2(d, f) -> b = f (NaN); max2(s, NaN) -> NaN
    let r = run(&[f32::NAN], &[0.5], 1.0);
    assert!(r[0].is_nan());
}

#[test]
fn widths_one_and_odd_and_empty() {
    assert_eq!(run(&[], &[], 0.5), Vec::<f32>::new());
    assert_eq!(run(&[0.5], &[0.5], 1.0).len(), 1);
    let d: Vec<f32> = (0..37).map(|i| i as f32 / 40.0).collect();
    let s: Vec<f32> = (0..37).map(|i| 1.0 - i as f32 / 40.0).collect();
    let a = run(&d, &s, 0.7);
    let b = run(&d, &s, 0.7);
    assert_eq!(a.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), b.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
}

#[test]
fn length_is_min_of_both() {
    let mut d = vec![0.5f32; 4];
    blend(&mut d, &[0.5, 0.5], 1.0);
    assert_eq!(d[2], 0.5);
    assert_eq!(d[0], 0.75);
}

#[test]
fn assumptions_listed() {
    assert_eq!(assumptions().len(), 2);
}

#[test]
fn single_nan_destination_is_quieted_both_routes() {
    let sn = f32::from_bits(0xff80_1234);
    for scale in [0.5f32, 1.0] {
        let r = run(&[sn, f32::from_bits(0x7fc0_00aa)], &[0.5, 0.5], scale);
        assert_eq!(r[0].to_bits(), 0xffc0_1234, "scale {scale}");
        assert_eq!(r[1].to_bits(), 0x7fc0_00aa, "scale {scale}");
    }
}

#[test]
fn fresh_nan_is_default_nan() {
    // scale -inf: t = -inf, 1 - t = +inf, fma(inf, inf, -inf) is invalid: fresh NaN 0xffc00000
    let r = run(&[f32::INFINITY, f32::NEG_INFINITY], &[0.5, 0.5], f32::NEG_INFINITY);
    assert_eq!(r[0].to_bits(), 0xffc0_0000);
    // d = -inf: -inf * inf + -inf = -inf (no NaN)
    assert_eq!(r[1].to_bits(), 0xff80_0000);
    // plain route: d = inf, s tiny: finite omt, inf result
    let r = run(&[f32::INFINITY], &[0.5], 1.0);
    assert_eq!(r[0], f32::INFINITY);
}

#[test]
fn nan_scale_with_positive_source_saturates() {
    let r = run(&[0.25], &[0.5], f32::from_bits(0x7fc0_0001));
    assert_eq!(r[0], 1.0);
}

#[test]
fn nan_source_never_reaches_a_fused_site() {
    for sb in [0x7fc0_0000u32, 0xffc1_2345, 0x7f80_0001] {
        let r = run(&[f32::from_bits(0x7fc0_0042)], &[f32::from_bits(sb)], 0.5);
        assert_eq!(r[0].to_bits(), 0x7fc0_0042);
    }
}

#[test]
fn two_nan_fused_form_source_order_wins_quieted() {
    // The crate's one fused form, fma213(omt, d, t): the first NaN operand in source order is returned, quieted, for
    // both quiet, both signalling and mixed signs (the route itself never presents two NaN operands; see lib.rs).
    use fx_common::nanop::fma213;
    let q = |b: u32| f32::from_bits(b);
    for (a, b) in [(0x7fc0_0011u32, 0xffc0_0022u32), (0x7f80_0011, 0xff80_0022), (0xff80_0011, 0x7fc0_0022), (0xffc0_0011, 0x7f80_0022)] {
        assert_eq!(fma213(q(a), q(b), 0.5).to_bits(), a | QUIET_BIT, "{a:08x} {b:08x} first and second");
        assert_eq!(fma213(q(a), 0.5, q(b)).to_bits(), a | QUIET_BIT, "{a:08x} {b:08x} first and third");
        assert_eq!(fma213(0.5, q(a), q(b)).to_bits(), a | QUIET_BIT, "{a:08x} {b:08x} second and third");
    }
    assert_eq!(fma213(f32::INFINITY, f32::INFINITY, f32::NEG_INFINITY).to_bits(), 0xffc0_0000);
}

#[test]
fn scaled_route_is_fused_once_rounded() {
    // scale 0.5 is exact; the fused result and the multiply-then-add result differ by one ulp for these pairs
    let cases: [(u32, u32, u32); 3] = [(0x3f1adb84, 0x3e9a0805, 0x3f2a124c), (0x3e9ecfb2, 0x3f043d9a, 0x3efa08a5), (0x3f326fbb, 0x3f24dbc8, 0x3f4b693d)];
    for (d, s, want) in cases {
        let r = run(&[f32::from_bits(d)], &[f32::from_bits(s)], 0.5);
        assert_eq!(r[0].to_bits(), want, "d {d:#x} s {s:#x}");
    }
}
