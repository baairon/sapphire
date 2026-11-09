use copy_alpha::*;

fn run(mode: i32, nchan: i32, n: usize, a: &Source, b: &Source) -> Result<Vec<Vec<f32>>, Error> {
    let mut outs: Vec<Vec<f32>> = (0..nchan.max(0)).map(|_| vec![f32::from_bits(0x7fc0ffee); n]).collect();
    let mut refs: Vec<&mut [f32]> = outs.iter_mut().map(|v| v.as_mut_slice()).collect();
    copy_alpha(&Params { mode, nchan, n, ..Params::default() }, a, b, &mut refs)?;
    Ok(outs)
}

#[test]
fn plane_colours_and_alpha_from_second() {
    let a: Vec<Vec<f32>> = (0..4).map(|c| (0..5).map(|i| (c * 10 + i) as f32).collect()).collect();
    let b: Vec<Vec<f32>> = (0..4).map(|c| (0..5).map(|i| (100 + c * 10 + i) as f32).collect()).collect();
    let ar: Vec<&[f32]> = a.iter().map(|v| v.as_slice()).collect();
    let br: Vec<&[f32]> = b.iter().map(|v| v.as_slice()).collect();
    let o = run(4, 4, 5, &Source::Planes { planes: &ar, last: 3 }, &Source::Planes { planes: &br, last: 3 }).unwrap();
    assert_eq!(o[0], a[0]);
    assert_eq!(o[1], a[1]);
    assert_eq!(o[2], a[2]);
    assert_eq!(o[3], b[3]);
}

#[test]
fn two_channel_mode_takes_second_channel_of_b() {
    let a = vec![vec![1.0f32, 2.0], vec![3.0, 4.0]];
    let b = vec![vec![5.0f32, 6.0], vec![7.0, 8.0]];
    let ar: Vec<&[f32]> = a.iter().map(|v| v.as_slice()).collect();
    let br: Vec<&[f32]> = b.iter().map(|v| v.as_slice()).collect();
    let o = run(2, 2, 2, &Source::Planes { planes: &ar, last: 1 }, &Source::Planes { planes: &br, last: 1 }).unwrap();
    assert_eq!(o[0], a[0]);
    assert_eq!(o[1], b[1]);
}

#[test]
fn no_alpha_mode_copies_all_channels_from_first() {
    let a = vec![vec![1.0f32], vec![2.0], vec![3.0]];
    let b = vec![vec![9.0f32], vec![9.0], vec![9.0]];
    let ar: Vec<&[f32]> = a.iter().map(|v| v.as_slice()).collect();
    let br: Vec<&[f32]> = b.iter().map(|v| v.as_slice()).collect();
    let o = run(3, 3, 1, &Source::Planes { planes: &ar, last: 2 }, &Source::Planes { planes: &br, last: 2 }).unwrap();
    assert_eq!(o, a);
}

#[test]
fn scalar_float_and_int_fill_width_one_and_odd() {
    let fv = [0x7fc0_1234u32, 0xff80_0000, 0x0000_0001, 0x8000_0000];
    for n in [1usize, 3, 4, 7] {
        let o = run(4, 4, n, &Source::Scalar { values: &fv, is_int: false, last: 3 }, &Source::Scalar { values: &fv, is_int: false, last: 3 }).unwrap();
        for c in 0..4 {
            assert!(o[c].iter().all(|v| v.to_bits() == fv[c] || (c == 3 && v.to_bits() == fv[3])));
        }
        assert!(o[3].iter().all(|v| v.to_bits() == fv[3]));
    }
    let iv = [0xffff_ffffu32, 16777217, 0x7fff_ffff, 5];
    let o = run(4, 4, 2, &Source::Scalar { values: &iv, is_int: true, last: 3 }, &Source::Scalar { values: &iv, is_int: true, last: 3 }).unwrap();
    assert_eq!(o[0][0], -1.0);
    assert_eq!(o[1][0], 16777216.0); // ties to even
    assert_eq!(o[2][1], 2147483648.0);
    assert_eq!(o[3][0], 5.0);
}

#[test]
fn channel_clamp_to_last() {
    let a = vec![vec![1.0f32, 1.0], vec![2.0, 2.0]];
    let ar: Vec<&[f32]> = a.iter().map(|v| v.as_slice()).collect();
    let o = run(0, 4, 2, &Source::Planes { planes: &ar, last: 1 }, &Source::Planes { planes: &ar, last: 1 }).unwrap();
    assert_eq!(o[2], a[1]);
    assert_eq!(o[3], a[1]);
}

#[test]
fn nan_payload_preserved_in_plane_copy() {
    let p = [f32::from_bits(0xffc1_2345), f32::from_bits(0x7fa0_0001)];
    let planes = [&p[..]];
    let o = run(0, 1, 2, &Source::Planes { planes: &planes, last: 0 }, &Source::Planes { planes: &planes, last: 0 }).unwrap();
    assert_eq!(o[0][0].to_bits(), 0xffc1_2345);
    assert_eq!(o[0][1].to_bits(), 0x7fa0_0001);
}

#[test]
fn determinism_and_errors() {
    let p = [1.0f32, 2.0];
    let planes = [&p[..]];
    let s = Source::Planes { planes: &planes, last: 0 };
    assert_eq!(run(0, 1, 2, &s, &s), run(0, 1, 2, &s, &s));
    assert_eq!(run(4, 0, 2, &s, &s), Err(Error::NoChannels));
    let short = [1.0f32];
    let sp = [&short[..]];
    assert_eq!(run(0, 1, 2, &Source::Planes { planes: &sp, last: 0 }, &s), Err(Error::PlaneShort));
    assert_eq!(run(0, 1, 2, &Source::Planes { planes: &planes, last: -1 }, &s), Err(Error::ChannelRange));
    let mut small = vec![0.0f32; 1];
    let mut refs: Vec<&mut [f32]> = vec![small.as_mut_slice()];
    assert_eq!(copy_alpha(&Params { mode: 0, nchan: 1, n: 2, ..Params::default() }, &s, &s, &mut refs), Err(Error::OutputShape));
    assert!(!assumptions().is_empty());
}

#[test]
fn pooled_fill_clears_negative_zero() {
    let neg = [0x8000_0000u32, 0x7fc0_0001, 0x0000_0000, 5];
    let s = Source::Scalar { values: &neg, is_int: false, last: 3 };
    let go = |n: usize, idle: bool, ch_mode: i32| {
        let mut outs: Vec<Vec<f32>> = vec![vec![f32::from_bits(0x7fc0ffee); n]];
        let mut refs: Vec<&mut [f32]> = outs.iter_mut().map(|v| v.as_mut_slice()).collect();
        copy_alpha(&Params { mode: ch_mode, nchan: 1, n, pool_idle: idle }, &s, &s, &mut refs).unwrap();
        outs.remove(0)
    };
    assert!(go(512, true, 0).iter().all(|x| x.to_bits() == 0));
    assert!(go(511, true, 0).iter().all(|x| x.to_bits() == 0x8000_0000));
    assert!(go(512, false, 0).iter().all(|x| x.to_bits() == 0x8000_0000));
    // NaN payloads are stored unchanged by the pool as well
    let nan = [0x7fc0_0001u32, 0, 0, 0];
    let s2 = Source::Scalar { values: &nan, is_int: false, last: 3 };
    let mut o = vec![0f32; 600];
    copy_alpha(&Params { mode: 0, nchan: 1, n: 600, pool_idle: true }, &s2, &s2, &mut [o.as_mut_slice()]).unwrap();
    assert!(o.iter().all(|x| x.to_bits() == 0x7fc0_0001));
    // count 0 writes nothing
    let mut z = vec![f32::from_bits(0x1234_5678); 4];
    copy_alpha(&Params { mode: 0, nchan: 1, n: 0, pool_idle: true }, &s2, &s2, &mut [z.as_mut_slice()]).unwrap();
    assert!(z.iter().all(|x| x.to_bits() == 0x1234_5678));
}

#[test]
fn colour_loop_never_reads_the_alpha_channel_of_the_first_input() {
    // The first input holds only three channel words; the alpha output comes from the second input.
    // A colour loop that also covered the alpha channel would look up a fourth word of the first input.
    let a_vals = [1u32, 2, 3];
    let b_vals = [0u32, 0, 0, 0x8000_0000];
    let a = Source::Scalar { values: &a_vals, is_int: false, last: 3 };
    let b = Source::Scalar { values: &b_vals, is_int: false, last: 3 };
    let o = run(4, 4, 3, &a, &b).expect("alpha channel of the first input is not read");
    assert_eq!(o[3][0].to_bits(), 0x8000_0000);
    assert_eq!(o[2][0].to_bits(), 3);
}
