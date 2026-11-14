use optical_flow_pyramid::*;

#[test]
fn first_on_empty_table() {
    assert_eq!(LevelTable::default().first(), -1);
}

#[test]
fn first_on_three_levels() {
    let t = LevelTable { levels: vec![LevelRecord::default(); 3], min_side: 8, image_table: 0 };
    assert_eq!(t.first(), 2);
}

#[test]
fn defaults_are_zero() {
    let r = LevelRecord::default();
    assert_eq!((r.width, r.height, r.stride, r.rect_word3, r.elem_bytes, r.index), (0, 0, 0, 0, 0, 0));
    assert_eq!(r.copied_plane, 0);
    let t = LevelTable::default();
    assert!(t.levels.is_empty() && t.min_side == 0 && t.image_table == 0);
}
