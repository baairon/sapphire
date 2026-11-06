use format14_write::*;
#[test]
fn clamp_edges() {
    assert_eq!(clamp01((-0.5f32).to_bits()), 0);
    assert_eq!(clamp01((-0.0f32).to_bits()), (-0.0f32).to_bits());
    assert_eq!(clamp01(2.0f32.to_bits()), 1.0f32.to_bits());
    assert_eq!(clamp01(0x7fc12345), 0x7fc12345);
    assert_eq!(clamp01(0xff800000), 0);
}
#[test]
fn three_plane_mode2_keeps_word0() {
    let p = [vec![1u32, 2], vec![3, 4], vec![5, 6]];
    let r: Vec<&[u32]> = p.iter().map(|v| v.as_slice()).collect();
    let mut d = vec![9u32; 8];
    convert14(3, &r, &mut d, 2, 1, 2).unwrap();
    assert_eq!(d, vec![9, 1, 3, 5, 9, 2, 4, 6]);
}
#[test]
fn four_plane_bad_mode() {
    let p = [vec![1u32], vec![1], vec![1], vec![1]];
    let r: Vec<&[u32]> = p.iter().map(|v| v.as_slice()).collect();
    let mut d = vec![7u32; 4];
    assert_eq!(convert14(4, &r, &mut d, 1, 1, 2), Err(CErr::BadMode(2)));
    assert_eq!(d, vec![7; 4]);
}
#[test]
fn cell_fill_copies_first_pixel() {
    let p = [vec![0x3f000000u32, 0x3f800000]];
    let r: Vec<&[u32]> = p.iter().map(|v| v.as_slice()).collect();
    let mut d = vec![0u32; 20];
    convert14(1, &r, &mut d, 5, 3, 3).unwrap();
    assert_eq!(&d[0..4], &d[4..8]);
    assert_eq!(&d[8..12], &d[0..4]);
    assert_eq!(&d[12..16], &[0x3f800000, 0x3f800000, 0x3f800000, 0x3f800000]);
    assert_eq!(&d[16..20], &d[12..16]);
}
