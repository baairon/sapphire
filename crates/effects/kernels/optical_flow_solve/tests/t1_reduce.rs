// T1 for module reduce: hand-evaluated sums, fills and border zeroing, NaN propagation, determinism, edge widths.
#[path = "support/small_side.rs"]
mod small_side;
use optical_flow_solve::reduce::zero_border;
use small_side::*;

fn ramp(n: usize) -> Vec<f32> {
    (0..n).map(|i| i as f32).collect()
}

#[test]
fn sum_width_one_height_one() {
    let acc = sum_plane(&[3.0f32], 0, 0, 1, 1, 1);
    assert_eq!(acc.vx_0, 3.0);
    assert_eq!(finish(&acc, 1), (3.0, 1.0, 3.0));
}

#[test]
fn sum_scalar_row_small_width() {
    let acc = sum_plane(&ramp(31), 0, 0, 31, 31, 1);
    assert_eq!(acc.vx_0, 465.0);
    assert_eq!(acc.v, [[0.0; 4]; 8]);
}

#[test]
fn sum_vector_row_aligned() {
    // w = 40, phase 0: end = 32, one 32 block, then two 4-blocks into v[0], no scalar tail
    let acc = sum_plane(&ramp(40), 0, 0, 40, 40, 1);
    assert_eq!(acc.v[0], [68.0, 71.0, 74.0, 77.0]);
    assert_eq!(acc.v[7], [28.0, 29.0, 30.0, 31.0]);
    assert_eq!(acc.vx_0, 0.0);
}

#[test]
fn sum_vector_row_phase_four() {
    // phase 4: peel 3, end = 35, block 3..34, 4-block 35..38, scalar 39
    let acc = sum_plane(&ramp(40), 4, 0, 40, 40, 1);
    assert_eq!(acc.v[0], [38.0, 40.0, 42.0, 44.0]);
    assert_eq!(acc.v[7], [31.0, 32.0, 33.0, 34.0]);
    assert_eq!(acc.vx_0, 42.0);
}

#[test]
fn sum_odd_phase_is_scalar() {
    let acc = sum_plane(&ramp(40), 2, 0, 40, 40, 1);
    assert_eq!(acc.vx_0, 780.0);
    assert_eq!(acc.v, [[0.0; 4]; 8]);
}

#[test]
fn finish_fold_and_mean() {
    let mut acc = Acc::new();
    for r in 0..8 {
        acc.v[r] = [r as f32, 1.0, 2.0, 4.0];
    }
    acc.vx_0 = 0.5;
    // lanes: lane0 = 0+..+7 = 28, lane1 = 8, lane2 = 16, lane3 = 32; total = (28 + 16) + (8 + 32) + 0.5
    let (t, c, m) = finish(&acc, 4);
    assert_eq!((t, c, m), (84.5, 4.0, 21.125));
}

#[test]
fn nan_propagates_and_is_quiet() {
    let mut a = ramp(40);
    a[5] = f32::from_bits(0x7fa0_0001);
    let acc = sum_plane(&a, 0, 0, 40, 40, 1);
    let (_, _, m) = finish(&acc, 40);
    assert_eq!(m.to_bits() & 0xffc0_0000, 0x7fc0_0000);
    let (_, _, z) = finish(&Acc::new(), 0);
    assert_eq!(z.to_bits(), 0xffc0_0000); // 0 / 0 is the default NaN
}

#[test]
fn deterministic() {
    let a: Vec<f32> = (0..200).map(|i| (i as f32) * 0.37 - 11.0).collect();
    assert_eq!(sum_plane(&a, 8, 3, 50, 45, 3), sum_plane(&a, 8, 3, 50, 45, 3));
}

#[test]
fn fill_rows_only() {
    let mut p = vec![9.0f32; 20];
    fill_plane(&mut p, 1, 6, 4, 3, 1.5);
    let e = [9.0, 1.5, 1.5, 1.5, 1.5, 9.0, 9.0, 1.5, 1.5, 1.5, 1.5, 9.0, 9.0, 1.5, 1.5, 1.5, 1.5, 9.0, 9.0, 9.0];
    assert_eq!(p, e);
}

#[test]
fn border_zero_rows() {
    let mut p = vec![1.0f32; 30]; // 5 wide, 6 high, stride 5
    zero_border(&mut p, 5, 6, 5, 0, 6);
    for y in 0..6 {
        for x in 0..5 {
            let want = if y == 0 || y == 5 || x == 0 || x == 4 { 0.0 } else { 1.0 };
            assert_eq!(p[y * 5 + x], want, "{y} {x}");
        }
    }
    let mut q = vec![1.0f32; 30];
    zero_border(&mut q, 5, 6, 5, 2, 1); // one interior row
    assert_eq!(q.iter().filter(|v| **v == 0.0).count(), 2);
    zero_border(&mut q, 5, 6, 5, 3, 0); // empty range
    assert_eq!(q.iter().filter(|v| **v == 0.0).count(), 2);
}

#[test]
fn border_width_one() {
    let mut p = vec![1.0f32; 3];
    zero_border(&mut p, 1, 3, 1, 0, 3);
    assert_eq!(p, [0.0, 0.0, 0.0]);
}

#[test]
#[should_panic]
fn sum_out_of_range_panics() {
    let _ = sum_plane(&[1.0f32; 3], 0, 0, 3, 8, 1);
}
