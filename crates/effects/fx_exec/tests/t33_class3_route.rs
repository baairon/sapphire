//! INT20 step G3: the route pieces the 'ae32' class-3 path runs (readers14b::read3b for head 0x7e91c0, c14::convert14 with 3 planes at
//! mode 2, c14::copy_word0 for the row copier) give the same bits as convert_pair's floored class-3 pair on the special-value suite.
//! So the floor of convert_pair (t_class3.rs, mutants with SHAs) covers the code the route executes.

const SPECIAL: [u32; 16] = [
    0x7fc0_0000, 0x7f80_0001, 0xffc0_0000, 0x7f80_0000, 0xff80_0000, 0x8000_0000, 0x0000_0000, 0x0000_0001, 0x807f_ffff, 0x3fc0_0000,
    0xbe80_0000, 0x7f7f_ffff, 0x3f80_0000, 0x3e80_0000, 0x4020_0000, 0x3f40_0000,
];

#[test]
fn route_pieces_equal_floored_pair() {
    let mut mem = Vec::new();
    for a in 0..16 {
        for c in 0..16 {
            mem.extend_from_slice(&[SPECIAL[a], SPECIAL[c], SPECIAL[(c * 5 + 3) % 16], SPECIAL[(c * 7 + a) % 16]]);
        }
    }
    let n = mem.len() / 4;
    // floored pair
    let mut pl = Vec::new();
    convert_pair::convert_from_source_line_3_b(&mem, &mut pl, n);
    let mut d1 = vec![0xaaaa_aaaau32; 4 * n];
    convert_pair::convert_to_dest_line_3_m2(&pl, &mem, &mut d1, n).unwrap();
    // route pieces
    let (mut a, mut b, mut c) = (vec![0u32; n], vec![0u32; n], vec![0u32; n]);
    {
        let mut p: [&mut [u32]; 3] = [&mut a[..], &mut b[..], &mut c[..]];
        readers14b::read3b(&mem, &mut p, 0, n as i32, false).unwrap();
    }
    assert_eq!((&a, &b, &c), (&pl[0], &pl[1], &pl[2]), "read3b equals variant B");
    let mut d2 = vec![0xaaaa_aaaau32; 4 * n];
    c14::convert14(3, &[&a[..], &b[..], &c[..]], &mut d2, n as i32, 1, 2).unwrap();
    for i in 0..n {
        assert_eq!(d2[4 * i], 0xaaaa_aaaa, "the mode-2 body leaves slot 0");
    }
    c14::copy_word0(&mem, &mut d2, n as i32).unwrap();
    assert_eq!(d1, d2, "route destination equals the floored pair");
}

#[test]
fn class_to_planes() {
    assert_eq!(convert_pair::class3_planes(), 3);
}
