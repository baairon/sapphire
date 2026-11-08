//! Hand-evaluated cases from the listing of 0x553e50, widths 0 and 1, odd and even, NaN and signed zero, determinism, errors.
use get_alpha::*;

const SENT: u32 = 0x7fc0ffee;

fn blk<'a>(vals: &'a [u32], planes: &'a [&'a [f32]], enable: bool, maxch: i32) -> Block<'a> {
    Block { is_int: false, values: vals, enable, maxch, img_valid: true, planes }
}

fn run(count: i32, n: i32, pool: bool, b: &Block, len: usize) -> (Vec<f32>, Result<Action, GaError>) {
    let mut out = vec![f32::from_bits(SENT); len];
    let p = Params { count, host: Host { in_pool: pool }, roi: Roi { mode: 0, n, port_blk_a: 0, port_blk_b: 0, port_blk_c: 0 } };
    let r = {
        let mut refs: Vec<&mut [f32]> = vec![out.as_mut_slice()];
        render(&p, std::slice::from_ref(b), &mut refs)
    };
    (out, r)
}

fn planes() -> Vec<Vec<f32>> {
    vec![vec![1.0, 2.0, 3.0], vec![10.0, 20.0, 30.0], vec![100.0, 200.0, 300.0], vec![-1.0, -0.0, f32::NAN]]
}

#[test]
fn count_two_takes_channel_one_and_four_takes_channel_three() {
    let pv = planes();
    let pr: Vec<&[f32]> = pv.iter().map(|p| p.as_slice()).collect();
    let vals = [0u32; 4];
    let b = blk(&vals, &pr, false, 3);
    let (o, r) = run(2, 3, false, &b, 4);
    assert!(r.is_ok());
    assert_eq!(&o[..3], &[10.0, 20.0, 30.0]);
    assert_eq!(o[3].to_bits(), SENT);
    let (o, _) = run(4, 3, false, &b, 4);
    assert_eq!(o[0], -1.0);
    assert_eq!(o[1].to_bits(), 0x8000_0000);
    assert!(o[2].is_nan());
}

#[test]
fn other_counts_fill_one() {
    let pv = planes();
    let pr: Vec<&[f32]> = pv.iter().map(|p| p.as_slice()).collect();
    let vals = [0u32; 4];
    let b = blk(&vals, &pr, false, 3);
    for count in [0, 1, 3, 5, -1, i32::MAX, i32::MIN] {
        let (o, r) = run(count, 5, false, &b, 6);
        assert!(matches!(r, Ok(Action::FillInline { value: 0x3f80_0000, count: 5 })));
        assert_eq!(&o[..5], &[1.0; 5]);
        assert_eq!(o[5].to_bits(), SENT);
    }
}

#[test]
fn channel_is_clamped_by_the_block_maxch_and_constant_blocks_fill() {
    let pv = planes();
    let pr: Vec<&[f32]> = pv.iter().map(|p| p.as_slice()).collect();
    let vals = [0u32; 4];
    let b = blk(&vals, &pr, false, 0);
    let (o, _) = run(4, 3, false, &b, 3);
    assert_eq!(&o[..3], &[1.0, 2.0, 3.0]);
    let cv = [0x8000_0000u32, 0x4000_0000, 0x7fc0_1234, 0x3f00_0000];
    let b = blk(&cv, &pr, true, 3);
    let (o, _) = run(4, 2, false, &b, 2);
    assert_eq!(o[0].to_bits(), 0x3f00_0000);
    let (o, _) = run(2, 2, false, &b, 2);
    assert_eq!(o[0].to_bits(), 0x4000_0000);
}

#[test]
fn width_zero_one_odd_even_and_pooled_fill() {
    let pv = planes();
    let pr: Vec<&[f32]> = pv.iter().map(|p| p.as_slice()).collect();
    let vals = [0u32; 4];
    let b = blk(&vals, &pr, false, 3);
    let (o, r) = run(0, 0, false, &b, 2);
    assert_eq!(r, Ok(Action::FillNone));
    assert_eq!(o[0].to_bits(), SENT);
    let (o, _) = run(0, 1, false, &b, 2);
    assert_eq!((o[0], o[1].to_bits()), (1.0, SENT));
    let (o, r) = run(0, 600, false, &b, 600);
    assert!(matches!(r, Ok(Action::FillPooledValue { value: 0x3f80_0000, count: 600 })));
    assert!(o.iter().all(|x| *x == 1.0));
    let (_, r) = run(0, 600, true, &b, 600);
    assert!(matches!(r, Ok(Action::FillInline { .. })));
}

#[test]
fn deterministic_and_errors() {
    let pv = planes();
    let pr: Vec<&[f32]> = pv.iter().map(|p| p.as_slice()).collect();
    let vals = [0u32; 4];
    let b = blk(&vals, &pr, false, 3);
    let x = run(2, 3, false, &b, 3).0;
    let y = run(2, 3, false, &b, 3).0;
    assert_eq!(x.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), y.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    let mut refs: Vec<&mut [f32]> = vec![];
    let p = Params { count: 2, host: Host { in_pool: false }, roi: Roi { mode: 0, n: 1, port_blk_a: 0, port_blk_b: 0, port_blk_c: 0 } };
    assert_eq!(render(&p, std::slice::from_ref(&b), &mut refs), Err(GaError::Shape));
    let nb = blk(&vals, &pr, false, -1);
    let (_, r) = run(2, 3, false, &nb, 3);
    assert_eq!(r, Err(GaError::Plane(PlError::NegativeChannel)));
    let (_, r) = run(2, 8, false, &b, 3);
    assert!(matches!(r, Err(GaError::Plane(PlError::SrcShort))));
    assert!(!assumptions().is_empty());
}
