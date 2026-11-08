use alpha_from_rgb_max::*;

fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}
fn par(mode: i32, nch: i32, scale: f32, nout: i32, n: usize) -> Params {
    Params { mode, nch, scale, nout, n }
}
fn run3(p: &Params, ins: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let refs: Vec<&[f32]> = ins.iter().map(|v| v.as_slice()).collect();
    let mut outs: Vec<Vec<f32>> = (0..4).map(|_| vec![f32::from_bits(0x7fc0ffee); p.n]).collect();
    let mut m: Vec<&mut [f32]> = outs.iter_mut().map(|v| v.as_mut_slice()).collect();
    run(p, &Source::Planes(&refs), &mut m).unwrap();
    outs
}

#[test]
fn max_times_scale_hand() {
    let ins = vec![vec![0.1, 0.9, -1.0], vec![0.5, 0.2, -2.0], vec![0.3, 0.4, -3.0]];
    let o = run3(&par(4, 3, 2.0, 4, 3), &ins);
    assert_eq!(o[3], vec![1.0, 1.8, -2.0]);
    assert_eq!(o[0], ins[0]);
    assert_eq!(o[2], ins[2]);
}

#[test]
fn scale_one_is_plain_max_and_zero_fills() {
    let ins = vec![vec![0.1, 0.9], vec![0.5, 0.2], vec![0.3, 0.4]];
    assert_eq!(run3(&par(4, 3, 1.0, 4, 2), &ins)[3], vec![0.5, 0.9]);
    assert_eq!(run3(&par(4, 3, 0.0, 4, 2), &ins)[3], vec![0.0, 0.0]);
    assert_eq!(run3(&par(4, 3, -0.0, 4, 2), &ins)[3], vec![0.0, 0.0]);
}

#[test]
fn single_channel_input() {
    let ins = vec![vec![0.25, -0.5, 4.0]];
    let o = run3(&par(4, 1, 2.0, 4, 3), &ins);
    assert_eq!(o[0], ins[0]);
    assert_eq!(o[1], ins[0]);
    assert_eq!(o[2], ins[0]);
    assert_eq!(o[3], vec![0.5, -1.0, 8.0]);
    assert_eq!(run3(&par(4, 1, 1.0, 4, 3), &ins)[3], vec![1.0; 3]);
    // nout 2: only channel 0 is copied; channel 3 reads the untouched channel 2 (sentinel NaN)
    let o = run3(&par(4, 1, 2.0, 2, 3), &ins);
    assert_eq!(o[0], ins[0]);
    assert!(o[1].iter().all(|x| x.to_bits() == 0x7fc0ffee));
    assert!(o[3].iter().all(|x| x.to_bits() == 0x7fc0ffee | 0));
}

#[test]
fn non_alpha_mode_copies_with_clamp() {
    let ins = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let o = run3(&par(0, 2, 5.0, 3, 2), &ins);
    assert_eq!(o[0], ins[0]);
    assert_eq!(o[1], ins[1]);
    assert_eq!(o[2], ins[1]);
    assert!(o[3].iter().all(|x| x.to_bits() == 0x7fc0ffee));
}

#[test]
fn width_one_and_odd_even() {
    for n in [1usize, 2, 3, 7, 8, 9, 16, 17, 31, 32, 33, 100] {
        let ins: Vec<Vec<f32>> = (0..3).map(|c| (0..n).map(|i| ((i * 7 + c * 3) % 11) as f32 * 0.5 - 2.0).collect()).collect();
        let o = run3(&par(4, 3, 0.5, 4, n), &ins);
        for i in 0..n {
            let m = ins[0][i].max(ins[1][i]).max(ins[2][i]);
            assert_eq!(o[3][i], 0.5 * m);
        }
    }
}

#[test]
fn nan_rules() {
    let nan_p = f32::from_bits(0x7fc01234);
    let nan_n = f32::from_bits(0xffc05678);
    // max returns the second operand when either is NaN
    assert_eq!(imax(nan_p, 1.0), 1.0);
    assert_eq!(imax(1.0, nan_n).to_bits(), nan_n.to_bits());
    assert_eq!(imax(0.0, -0.0).to_bits(), (-0.0f32).to_bits());
    assert_eq!(imax(-0.0, 0.0).to_bits(), 0.0f32.to_bits());
    // multiply: first NaN operand quieted
    assert_eq!(imul(f32::from_bits(0x7f800001), 2.0).to_bits(), 0x7fc00001);
    assert_eq!(imul(f32::from_bits(0x7fc00000), f32::from_bits(0xffc12345)).to_bits(), 0x7fc00000);
    assert_eq!(imul(f32::from_bits(0x7fa00001), f32::from_bits(0xffc12345)).to_bits(), 0x7fe00001);
    assert_eq!(imul(2.0, nan_n).to_bits(), nan_n.to_bits());
    assert_eq!(imul(f32::INFINITY, 0.0).to_bits(), 0xffc00000);
    // scale NaN is not zero and not one: product path, result NaN
    let ins = vec![vec![1.0], vec![2.0], vec![3.0]];
    assert_eq!(run3(&par(4, 3, nan_p, 4, 1), &ins)[3][0].to_bits(), nan_p.to_bits());
}

#[test]
fn const_source() {
    let mut o: Vec<Vec<f32>> = (0..4).map(|_| vec![0.0; 3]).collect();
    let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
    let bits = [(-5i32) as u32, 7, 16777217];
    run(&par(4, 3, 1.0, 4, 3), &Source::Const { bits: &bits, is_int: true }, &mut m).unwrap();
    assert_eq!(o[0], vec![-5.0; 3]);
    assert_eq!(o[2], vec![16777216.0; 3]);
    assert_eq!(o[3], vec![16777216.0; 3]);
}

#[test]
fn determinism_and_errors() {
    let ins = vec![vec![0.3f32; 5], vec![0.1; 5], vec![0.2; 5]];
    assert_eq!(bits(&run3(&par(4, 3, 0.7, 4, 5), &ins)[3]), bits(&run3(&par(4, 3, 0.7, 4, 5), &ins)[3]));
    let refs: Vec<&[f32]> = ins.iter().map(|v| v.as_slice()).collect();
    let mut o: Vec<Vec<f32>> = (0..4).map(|_| vec![0.0; 5]).collect();
    let mut m: Vec<&mut [f32]> = o.iter_mut().map(|v| v.as_mut_slice()).collect();
    assert_eq!(run(&par(4, 0, 1.0, 4, 5), &Source::Planes(&refs), &mut m), Err(Error::NoChannels));
    assert_eq!(run(&par(4, 3, 1.0, 4, 9), &Source::Planes(&refs), &mut m), Err(Error::ShortPlane));
    let mut m3: Vec<&mut [f32]> = o.iter_mut().take(3).map(|v| v.as_mut_slice()).collect();
    assert_eq!(run(&par(4, 3, 1.0, 4, 5), &Source::Planes(&refs), &mut m3), Err(Error::OutputPlanes));
    assert!(!assumptions().is_empty());
}
