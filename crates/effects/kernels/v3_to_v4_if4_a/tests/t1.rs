//! Hand-evaluated cases from the listing, width 1 and 0, odd and even widths, NaN and signed-zero propagation, determinism, errors.
use v3_to_v4_if4_a::*;

const SENT: u32 = 0x7fc0ffee;

struct B {
    vals: Vec<u32>,
    planes: Vec<Vec<f32>>,
    enable: bool,
    maxch: i32,
    is_int: bool,
}

impl B {
    fn fill(v: [f32; 4], maxch: i32) -> B {
        B { vals: v.iter().map(|x| x.to_bits()).collect(), planes: vec![vec![0.0; 8]; 4], enable: true, maxch, is_int: false }
    }
    fn copy(p: [Vec<f32>; 4], maxch: i32) -> B {
        B { vals: vec![0; 4], planes: p.to_vec(), enable: false, maxch, is_int: false }
    }
}

fn run(combo: usize, n: i32, nch: i32, pool: bool, b: [&B; 3]) -> (Vec<Vec<f32>>, Result<Vec<Action>, VqError>) {
    let pl: Vec<Vec<&[f32]>> = b.iter().map(|x| x.planes.iter().map(|p| p.as_slice()).collect()).collect();
    let blocks: Vec<Block> = b
        .iter()
        .enumerate()
        .map(|(i, x)| Block { is_int: x.is_int, values: &x.vals, enable: x.enable, maxch: x.maxch, img_valid: true, planes: &pl[i] })
        .collect();
    let mut outs: Vec<Vec<f32>> = (0..4).map(|_| vec![f32::from_bits(SENT); 8]).collect();
    let p = Params { combo, host: Host { in_pool: pool }, roi: Roi { mode: 0, n, port_blk_a: 0, port_blk_b: 0, port_blk_c: 0 }, this_nch: nch };
    let r = {
        let mut refs: Vec<&mut [f32]> = outs.iter_mut().map(|v| v.as_mut_slice()).collect();
        render(&p, &blocks, &mut refs)
    };
    (outs, r)
}

fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

#[test]
fn table_row_6_is_pass_f_and_the_rest_match_the_listing() {
    assert_eq!(COMBOS[6].pass, Pass::F);
    assert_eq!(COMBOS[5].pass, Pass::B);
    assert_eq!(COMBOS[4].pass, Pass::C);
    assert_eq!(COMBOS[7].pass, Pass::D);
    assert_eq!(select_combo([1, 1, 1]), Some(6));
    assert_eq!(select_combo([9, 9, 9]), None);
}

#[test]
fn pass_f_copies_channel_zero_of_block_one() {
    let a = B::fill([1.0; 4], 0);
    let b = B::copy([vec![5.0, 6.0, 7.0], vec![8.0; 3], vec![9.0; 3], vec![10.0; 3]], 3);
    let c = B::fill([0.0; 4], 0);
    for nch in [1, 2, 4] {
        let (o, r) = run(6, 3, nch, false, [&a, &b, &c]);
        assert!(r.is_ok());
        assert_eq!(&o[0][..3], &[5.0, 6.0, 7.0]);
        assert_eq!(o[0][3].to_bits(), SENT);
        assert_eq!(o[1][0].to_bits(), SENT);
    }
}

#[test]
fn pass_f_fills_from_block_one_constant_channel_clamped_by_maxch() {
    let a = B::fill([1.0; 4], 0);
    let b = B::fill([-0.0, 2.0, 3.0, 4.0], 0);
    let c = B::fill([0.0; 4], 0);
    let (o, r) = run(6, 2, 2, false, [&a, &b, &c]);
    assert!(r.is_ok());
    assert_eq!(bits(&o[0][..2]), vec![0x8000_0000; 2]);
}

#[test]
fn pass_f_nonpositive_this_nch_is_a_service_error() {
    let a = B::fill([1.0; 4], 0);
    let b = B::fill([1.0; 4], 3);
    let c = B::fill([0.0; 4], 0);
    let (_, r) = run(6, 2, 0, false, [&a, &b, &c]);
    assert_eq!(r, Err(VqError::Plane(PlError::NegativeChannel)));
}

#[test]
fn pass_a_clamps_color_channel_and_copies_alpha_from_block_one() {
    let a = B::copy([vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0], vec![7.0, 8.0]], 1);
    let b = B::copy([vec![-1.0, -2.0], vec![0.0; 2], vec![0.0; 2], vec![0.0; 2]], 1);
    let c = B::fill([0.0; 4], 0);
    // maxch 1 -> mx = 0: all three colour planes take channel 0 (width 2)
    let (o, r) = run(0, 2, 1, false, [&a, &b, &c]);
    assert!(r.is_ok());
    assert_eq!(&o[0][..2], &[1.0, 2.0]);
    assert_eq!(&o[1][..2], &[1.0, 2.0]);
    assert_eq!(&o[2][..2], &[1.0, 2.0]);
    assert_eq!(&o[3][..2], &[-1.0, -2.0]);
}

#[test]
fn pass_b_c_d_hand_cases() {
    let a = B::copy([vec![1.0], vec![2.0], vec![3.0], vec![4.0]], 3);
    let b = B::copy([vec![9.0], vec![0.0], vec![0.0], vec![0.0]], 3);
    let c = B::fill([0.0; 4], 0);
    let (o, _) = run(3, 1, 1, false, [&a, &b, &c]);
    assert_eq!((o[0][0], o[1][0], o[2][0]), (1.0, 2.0, 3.0));
    assert_eq!(o[3][0].to_bits(), SENT);
    let (o, _) = run(4, 1, 1, false, [&a, &b, &c]);
    assert_eq!((o[0][0], o[1][0], o[2][0]), (1.0, 1.0, 1.0));
    let (o, _) = run(7, 1, 1, false, [&a, &b, &c]);
    assert_eq!((o[0][0], o[1][0]), (1.0, 9.0));
    assert_eq!(o[2][0].to_bits(), SENT);
}

#[test]
fn width_zero_writes_nothing_and_nan_signs_survive() {
    let nan_neg = f32::from_bits(0xffc0_0000);
    let a = B::copy([vec![nan_neg; 5], vec![f32::from_bits(0x8000_0000); 5], vec![0.0; 5], vec![0.0; 5]], 3);
    let c = B::fill([0.0; 4], 0);
    let (o, r) = run(5, 0, 1, false, [&a, &a, &c]);
    assert!(r.is_ok());
    assert_eq!(o[0][0].to_bits(), SENT);
    let (o, _) = run(5, 5, 1, false, [&a, &a, &c]);
    assert_eq!(o[0][0].to_bits(), 0xffc0_0000);
    let (o, _) = run(6, 5, 1, false, [&a, &a, &c]);
    assert_eq!(o[0][4].to_bits(), 0xffc0_0000);
}

#[test]
fn deterministic_and_error_paths() {
    let a = B::copy([vec![1.5; 7], vec![2.5; 7], vec![3.5; 7], vec![4.5; 7]], 3);
    let c = B::fill([0.0; 4], 0);
    let x = run(2, 7, 1, true, [&a, &a, &c]);
    let y = run(2, 7, 1, true, [&a, &a, &c]);
    assert_eq!(x.0.iter().map(|v| bits(v)).collect::<Vec<_>>(), y.0.iter().map(|v| bits(v)).collect::<Vec<_>>());
    assert_eq!(run(99, 1, 1, false, [&a, &a, &c]).1, Err(VqError::BadCombo(99)));
    // odd width 7 and even width 8 both copy completely
    let (o, _) = run(7, 7, 1, false, [&a, &a, &c]);
    assert_eq!(&o[0][..7], &[1.5; 7]);
    assert_eq!(o[0][7].to_bits(), SENT);
    assert!(!assumptions().is_empty());
}

#[test]
fn fill_rect_mode_is_recorded_not_executed() {
    let a = B::fill([1.0; 4], 0);
    let c = B::fill([0.0; 4], 0);
    let pl: Vec<Vec<&[f32]>> = [&a, &a, &c].iter().map(|x| x.planes.iter().map(|p| p.as_slice()).collect()).collect();
    let blocks: Vec<Block> = [&a, &a, &c].iter().enumerate().map(|(i, x)| Block { is_int: false, values: &x.vals, enable: true, maxch: 0, img_valid: true, planes: &pl[i] }).collect();
    let mut outs: Vec<Vec<f32>> = (0..4).map(|_| vec![0.0; 4]).collect();
    let p = Params { combo: 6, host: Host { in_pool: false }, roi: Roi { mode: 1, n: 4, port_blk_a: 0, port_blk_b: 0, port_blk_c: 0 }, this_nch: 1 };
    let mut refs: Vec<&mut [f32]> = outs.iter_mut().map(|v| v.as_mut_slice()).collect();
    let r = render(&p, &blocks, &mut refs).unwrap();
    assert!(matches!(r[0], Action::FillRect { .. }));
}
