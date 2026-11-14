use optical_flow_interp_maps::kernels::*;

#[test]
fn masked_copy_copies_only_where_flag_is_zero() {
    let flag = [0u32, 1, 0, 0, 5, 0];
    let src = [10u32, 11, 12, 13, 14, 15];
    let mut dst = [99u32; 6];
    k3_masked_copy(3, 3, &flag, &src, &mut dst, 0, 2);
    assert_eq!(dst, [10, 99, 12, 13, 99, 15]);
    let mut d2 = [99u32; 6];
    k3_masked_copy(3, 3, &flag, &src, &mut d2, 1, 1);
    assert_eq!(d2, [99, 99, 99, 13, 99, 15]);
}

#[test]
fn count_zero_words() {
    let m = [0u32, 1, 0, 0, 0, 7];
    assert_eq!(k4_count_zero(3, 3, &m, 0, 2), 4.0);
    assert_eq!(k4_count_zero(3, 3, &m, 0, 0), 0.0);
    assert_eq!(k4_count_zero(0, 3, &m, 0, 2), 0.0);
    assert_eq!(k4_count_zero(1, 3, &m, 0, 2), 2.0);
}

#[test]
fn fill_takes_the_mean_of_flagged_neighbours() {
    // 3x1: centre pixel unfilled; neighbours (0,0) and (2,0) flagged with 2.0 and 4.0 -> mean 3.0
    let (covered, value_in) = ([1u32, 0, 1], [2.0f32.to_bits(), 9.0f32.to_bits(), 4.0f32.to_bits()]);
    let valid_in = [1u32, 0, 1];
    let (mut value_out, mut valid_out) = ([0u32; 3], [0u32; 3]);
    let mut k = K2 { w: 3, h: 1, stride: 3, covered: &covered, value_in: &value_in, value_out: &mut value_out, valid_in: &valid_in, valid_out: &mut valid_out };
    k2_fill(&mut k, 0, 1);
    assert_eq!(f32::from_bits(value_out[1]), 3.0);
    assert_eq!(valid_out, [0, 1, 0]);
    // no flagged neighbour: untouched
    let zero_masks = [0u32; 3];
    let (mut shift_set, mut mask_pair) = ([7u32; 3], [7u32; 3]);
    let mut k = K2 { w: 3, h: 1, stride: 3, covered: &covered, value_in: &value_in, value_out: &mut shift_set, valid_in: &zero_masks, valid_out: &mut mask_pair };
    k2_fill(&mut k, 0, 1);
    assert_eq!(shift_set, [7; 3]);
}

#[test]
fn interp_outputs_hand_evaluated() {
    // w = 2: c = 1.0; scale 0.5: cs = 0.5, 1 - scale = 0.5, gain_rest = 0.5; flow_offset = 0.
    let a = [4.0f32.to_bits(), 2.0f32.to_bits()];
    let b = [8.0f32.to_bits(), 6.0f32.to_bits()];
    let mut o = [[0u32; 2]; 4];
    let [res_a, res_b, res_c, res_d] = &mut o;
    let mut k = K5 { w: 2, stride: 2, scale: 0.5, flow_offset: 0.0, a: &a, b: &b, flow_b_origin: 0, out: [res_a, res_b, res_c, res_d] };
    k5_interp(&mut k, 0, 1);
    let g = |i: usize, j: usize| f32::from_bits(o[i][j]);
    // w = 2 < 8: scalar path. c = 2/2 = 1
    assert_eq!(g(0, 0), -2.0); // -(0.5 * 4)
    assert_eq!(g(2, 0), 2.0); // 4 * 0.5
    assert_eq!(g(1, 0), -4.0); // 1 * (0 - 0.5 * 8)
    assert_eq!(g(3, 0), 4.0); // 1 * (0.5 * 8 + 0)
}

#[test]
fn splat_single_pixel_stays_in_place_with_zero_flow() {
    let z = [0u32; 4];
    let p = [1.0f32.to_bits(), 2.0f32.to_bits(), 3.0f32.to_bits(), 4.0f32.to_bits()];
    let (mut target_flow_x, mut target_flow_y, mut best_cost, mut covered, mut source_done) = ([9u32; 4], [9u32; 4], [0u32; 4], [0u32; 4], [0u32; 4]);
    let mut k = K1 { w: 2, h: 2, stride: 2, nch: 1, scale: 1.0, neg1: -1.0, prev_planes: [&p, &z, &z], next_planes: [&p, &z, &z], a: &z, b: &z, target_flow_x: &mut target_flow_x, target_flow_y: &mut target_flow_y, best_cost: &mut best_cost, covered: &mut covered, source_done: &mut source_done };
    k1_splat(&mut k, 0, 2).unwrap();
    assert_eq!(covered, [1, 1, 1, 1]);
    assert_eq!(source_done, [1, 1, 1, 1]);
    assert_eq!(target_flow_x, [0; 4]);
    assert_eq!(best_cost, [0; 4]); // identical planes: zero residual
}

#[test]
fn chunk_limits_rows() {
    let z = [0u32; 4];
    let (mut target_flow_x, mut target_flow_y, mut best_cost, mut covered, mut source_done) = ([9u32; 4], [9u32; 4], [0u32; 4], [0u32; 4], [0u32; 4]);
    let mut k = K1 { w: 2, h: 2, stride: 2, nch: 0, scale: 1.0, neg1: -1.0, prev_planes: [&z, &z, &z], next_planes: [&z, &z, &z], a: &z, b: &z, target_flow_x: &mut target_flow_x, target_flow_y: &mut target_flow_y, best_cost: &mut best_cost, covered: &mut covered, source_done: &mut source_done };
    k1_splat(&mut k, 1, 1).unwrap();
    assert_eq!(source_done, [0, 0, 1, 1]);
}

#[test]
fn determinism() {
    let a = [f32::NAN.to_bits(), 1.0f32.to_bits(), 3.0f32.to_bits(), 2.0f32.to_bits()];
    let run = || {
        let mut o = [[0u32; 4]; 4];
        let [res_a, res_b, res_c, res_d] = &mut o;
        let mut k = K5 { w: 2, stride: 2, scale: 0.5, flow_offset: -1.0, a: &a, b: &a, flow_b_origin: 0, out: [res_a, res_b, res_c, res_d] };
        k5_interp(&mut k, 0, 2);
        o
    };
    assert_eq!(run(), run());
}

#[test]
fn extent_one_is_refused() {
    for (w, h) in [(1, 7), (50, 1), (1, 1)] {
        let z = vec![0u32; 400];
        let (mut target_flow_x, mut target_flow_y, mut best_cost, mut covered, mut source_done) = (z.clone(), z.clone(), z.clone(), z.clone(), z.clone());
        let mut k = K1 { w, h, stride: w, nch: 3, scale: 1.0, neg1: -1.0, prev_planes: [&z, &z, &z], next_planes: [&z, &z, &z], a: &z, b: &z, target_flow_x: &mut target_flow_x, target_flow_y: &mut target_flow_y, best_cost: &mut best_cost, covered: &mut covered, source_done: &mut source_done };
        assert_eq!(k1_splat(&mut k, 0, h), Err(ExtentOneRefused));
        assert!(source_done.iter().all(|x| *x == 0));
    }
}

#[test]
fn native_op_runs_and_refuses_extent_one() {
    use optical_flow_interp_maps::native::*;
    let (w, h) = (4usize, 3usize);
    let pl: Vec<Vec<f32>> = (0..3).map(|c| (0..w * h).map(|i| (i + c) as f32 * 0.1).collect()).collect();
    let z = vec![0.0f32; w * h];
    let row_hi = interp_maps(w, h, &pl, &pl, &z, &z, 0.5, 0.0).unwrap();
    let r2 = interp_maps(w, h, &pl, &pl, &z, &z, 0.5, 0.0).unwrap();
    assert_eq!(row_hi, r2);
    assert!(row_hi[0].iter().all(|v| *v == 0.0 || *v == -0.0));
    for (w, h) in [(1usize, 7usize), (50, 1), (1, 1)] {
        let p = vec![vec![0.0f32; w * h]; 3];
        let zz = vec![0.0f32; w * h];
        assert_eq!(interp_maps(w, h, &p, &p, &zz, &zz, 0.5, 0.0), Err(NativeError::ExtentOneRefused));
    }
}
