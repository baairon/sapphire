use diagonal_box_sum::*;
use fx_common::nanop::NanOps;

/// The quiet bit of an f32 NaN payload.
const QUIET_BIT: u32 = 1 << 22;

fn p(w: i32, h: i32, radii: [i32; 3], shift: i32, kind: Diagonal) -> Params {
    Params { w, h, radii, shift, phases: [0; 5], start: 0, count: w + h - 1, kind }
}

#[test]
fn radius_one_all_ones_anti() {
    let mut img = vec![1.0f32; 9];
    box_sum_diagonals(&mut img, &p(3, 3, [1, 0, 0], 0, Diagonal::Anti)).unwrap();
    let s = scale_factor([1, 0, 0], Diagonal::Anti);
    let lens = [[1.0f32, 0.0, 0.0], [0.0, 0.0, 0.0]];
    let _ = lens;
    // anti-diagonal sums with zero padding: lengths 1,2,3,2,1 give window sums 1, 2/2, 2/3/2, 2/2, 1
    let want = |v: f32| s.fmul(v);
    assert_eq!(img[0].to_bits(), want(1.0).to_bits());
    assert_eq!(img[1].to_bits(), want(2.0).to_bits());
    assert_eq!(img[3].to_bits(), want(2.0).to_bits());
    assert_eq!(img[2].to_bits(), want(2.0).to_bits());
    assert_eq!(img[4].to_bits(), want(3.0).to_bits());
    assert_eq!(img[6].to_bits(), want(2.0).to_bits());
    assert_eq!(img[5].to_bits(), want(2.0).to_bits());
    assert_eq!(img[7].to_bits(), want(2.0).to_bits());
    assert_eq!(img[8].to_bits(), want(1.0).to_bits());
}

#[test]
fn radius_one_all_ones_main() {
    let mut img = vec![1.0f32; 9];
    box_sum_diagonals(&mut img, &p(3, 3, [1, 0, 0], 0, Diagonal::Main)).unwrap();
    let s = scale_factor([1, 0, 0], Diagonal::Main);
    let want = |v: f32| s.fmul(v);
    // main diagonal has 3 elements: 2, 3, 2; the off diagonals have 2: 2, 2; corners 1
    assert_eq!(img[0].to_bits(), want(2.0).to_bits());
    assert_eq!(img[4].to_bits(), want(3.0).to_bits());
    assert_eq!(img[8].to_bits(), want(2.0).to_bits());
    assert_eq!(img[1].to_bits(), want(2.0).to_bits());
    assert_eq!(img[2].to_bits(), want(1.0).to_bits());
    assert_eq!(img[6].to_bits(), want(1.0).to_bits());
}

#[test]
fn single_element_and_no_pass() {
    let mut img = vec![2.5f32];
    box_sum_diagonals(&mut img, &p(1, 1, [3, 0, 0], 0, Diagonal::Anti)).unwrap();
    assert_eq!(img[0].to_bits(), scale_factor([3, 0, 0], Diagonal::Anti).fmul(2.5).to_bits());
    let mut img = vec![2.5f32, -1.0];
    box_sum_diagonals(&mut img, &p(2, 1, [0, -4, 0], 0, Diagonal::Anti)).unwrap();
    let s = scale_factor([0, -4, 0], Diagonal::Anti);
    assert_eq!(img[0].to_bits(), s.fmul(2.5).to_bits());
}

#[test]
fn height_one_and_width_one() {
    let mut a = vec![1.0f32, 2.0, 3.0, 4.0];
    box_sum_diagonals(&mut a, &p(4, 1, [1, 0, 0], 0, Diagonal::Main)).unwrap();
    assert_eq!(a[0], scale_factor([1, 0, 0], Diagonal::Main) * 1.0);
    let mut b = vec![1.0f32, 2.0, 3.0];
    box_sum_diagonals(&mut b, &p(1, 3, [1, 0, 0], 0, Diagonal::Anti)).unwrap();
    assert!(b.iter().all(|v| v.is_finite()));
}

#[test]
fn shift_blend_zero_and_taps() {
    // 3x3 anti, no radii (scale 1), shift 1: diagonal of length 3 holds a b c -> 0.5*b, 0.5*(a+c), 0.5*b
    let mut img: Vec<f32> = (1..=9).map(|v| v as f32).collect();
    box_sum_diagonals(&mut img, &p(3, 3, [0, 0, 0], 1, Diagonal::Anti)).unwrap();
    // middle anti-diagonal: (0,2)=3,(1,1)=5,(2,0)=7 -> 2.5, 5.0, 2.5
    assert_eq!(img[2], 2.5);
    assert_eq!(img[4], 5.0);
    assert_eq!(img[6], 2.5);
    // length-1 diagonal with shift >= len gives +0
    assert_eq!(img[0].to_bits(), 0);
}

#[test]
fn nan_propagation_and_determinism() {
    let mut img = vec![1.0f32; 16];
    img[5] = f32::from_bits(0x7fc0_1234);
    let mut a = img.clone();
    box_sum_diagonals(&mut a, &p(4, 4, [1, 1, 0], 0, Diagonal::Anti)).unwrap();
    assert!(a.iter().any(|v| v.is_nan()));
    let mut b = img.clone();
    box_sum_diagonals(&mut b, &p(4, 4, [1, 1, 0], 0, Diagonal::Anti)).unwrap();
    assert_eq!(a.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), b.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    let nan = a.iter().find(|v| v.is_nan()).unwrap().to_bits();
    assert_eq!(nan & QUIET_BIT, QUIET_BIT);
}

#[test]
fn error_paths() {
    let mut img = vec![0.0f32; 4];
    assert_eq!(box_sum_diagonals(&mut img, &p(0, 4, [1, 0, 0], 0, Diagonal::Anti)), Err(BoxSumError::BadSize));
    assert_eq!(box_sum_diagonals(&mut img, &p(3, 3, [1, 0, 0], 0, Diagonal::Anti)), Err(BoxSumError::BadLength));
    let mut q = p(2, 2, [1, 0, 0], 0, Diagonal::Anti);
    q.count = 0;
    assert_eq!(box_sum_diagonals(&mut img, &q), Ok(()));
    q.count = 4;
    assert_eq!(box_sum_diagonals(&mut img, &q), Err(BoxSumError::BadBand));
    let mut q = p(2, 2, [1, 0, 0], i32::MIN, Diagonal::Anti);
    q.count = 3;
    assert_eq!(box_sum_diagonals(&mut img, &q), Err(BoxSumError::BadShift));
}

#[test]
fn band_split_equals_whole() {
    let src: Vec<f32> = (0..35).map(|i| ((i * 7919) % 101) as f32 * 0.37).collect();
    for kind in [Diagonal::Anti, Diagonal::Main] {
        let mut whole = src.clone();
        box_sum_diagonals(&mut whole, &p(7, 5, [2, 1, 1], 2, kind)).unwrap();
        let mut parts = src.clone();
        for (s, c) in [(0, 3), (3, 6), (9, 2)] {
            let mut q = p(7, 5, [2, 1, 1], 2, kind);
            q.start = s;
            q.count = c;
            box_sum_diagonals(&mut parts, &q).unwrap();
        }
        assert_eq!(whole.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), parts.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
    }
}

// RULE-PINs: two NaNs of distinct payload at one site; the first operand in source order wins (quieted).
fn nan_plane(w: usize, h: usize) -> Vec<f32> {
    (0..w * h).map(|i| f32::from_bits(0x7fc0_0000 | ((i as u32 + 1) & 0x3f_ffff))).collect()
}

fn pq(p: &Params, w: usize, h: usize) -> Vec<f32> {
    let mut img = nan_plane(w, h);
    box_sum_diagonals(&mut img, p).unwrap();
    img
}

#[test]
fn rule_pin_middle_form_difference_first() {
    // 7x7, radius 1: the running sum holds the payload of the first window element; the first middle update takes the payload of
    // the entering element (the difference is the first operand of the add).
    for (kind, want_cell) in [(Diagonal::Anti, (2usize, 4usize, 3usize, 3usize)), (Diagonal::Main, (2, 2, 3, 3))] {
        let img = pq(&p(7, 7, [1, 0, 0], 0, kind), 7, 7);
        let (r, c, sr, sc) = want_cell;
        let src = 0x7fc0_0000 | ((sr * 7 + sc + 1) as u32);
        assert_eq!(img[r * 7 + c].to_bits(), src, "{kind:?}");
    }
}

#[test]
fn rule_pin_blend_right_tap_first() {
    for (kind, (r, c, sr, sc)) in [(Diagonal::Anti, (1usize, 5usize, 2usize, 4usize)), (Diagonal::Main, (1, 1, 2, 2))] {
        let img = pq(&p(7, 7, [0, 0, 0], 1, kind), 7, 7);
        let src = 0x7fc0_0000 | ((sr * 7 + sc + 1) as u32);
        assert_eq!(img[r * 7 + c].to_bits(), src, "{kind:?}");
    }
}

#[test]
fn rule_pin_vector_accumulator_old_first() {
    // 255x255, radius 127: two 64-float blocks; the second block adds into the accumulator (accumulator first).
    for (kind, (r, c)) in [(Diagonal::Anti, (0usize, 254usize)), (Diagonal::Main, (0, 0))] {
        let mut q = p(255, 255, [127, 0, 0], 0, kind);
        q.start = 254;
        q.count = 1;
        let img = pq(&q, 255, 255);
        let src = 0x7fc0_0000 | ((r * 255 + c + 1) as u32);
        assert_eq!(img[r * 255 + c].to_bits(), src, "{kind:?}");
    }
}
