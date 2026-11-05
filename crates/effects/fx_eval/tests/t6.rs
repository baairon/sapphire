//! NV-3e: the ported linewise grouping pass (listing 0x18fa0, 0x19c70, 0x1a870, 0xb8c0), examples read off the original r8 = 0 eval lists.
use fx_eval::lgpass::{random_access, run_pass, LNode};

fn n(id: usize, f: bool, prim: &str, el: i32, args: &[usize]) -> LNode {
    LNode { id, is_func: f, prim: prim.into(), elnum: el, fmt: if prim == "make_mipmap" { 3 } else { 2 }, args: args.to_vec(), g: -1 }
}

#[test]
fn sharpen_fast_default_hoists_mask_before_source() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // r8 = 1 order: Source, sharp_all(sharpen), Mask, mask(blur), dest(dissolve); original r8 = 0 order: Mask, Source, sharp, mask, dest with lg 1 1 -1 -1 2
    let v = run_pass(vec![n(47, false, "", 3, &[]), n(2, true, "sharpen", 3, &[47, 13, 24]), n(48, false, "", 1, &[]), n(4, true, "blur", 1, &[48, 6, 31, 32, 33]), n(0, true, "dissolve", 3, &[47, 2, 4])]);
    let ids: Vec<usize> = v.iter().map(|x| x.id).collect();
    assert_eq!(ids, vec![48, 47, 2, 4, 0]);
    let g: Vec<i32> = v.iter().map(|x| x.g).collect();
    assert_eq!(g[0], g[1]);
    assert_eq!((g[2], g[3]), (-1, -1));
    assert!(g[4] >= 0 && g[4] != g[0]);
}

#[test]
fn make_mipmap_result_is_not_read_inside_its_own_group() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // Blur_Motion_Curves_No_Transform default: Source, make_mipmap | xy, blur_warp (arg 0 is random access): lg 0 0 1 1
    let v = run_pass(vec![n(63, false, "", 4, &[]), n(1, true, "make_mipmap", 4, &[63, 14, 13]), n(65, false, "", 2, &[]), n(0, true, "blur_warp", 4, &[1, 65, 65, 12, 3, 14, 4, 5])]);
    let g: Vec<i32> = v.iter().map(|x| x.g).collect();
    assert_eq!(g[0], g[1]);
    assert_eq!(g[2], g[3]);
    assert_ne!(g[1], g[2]);
}

#[test]
fn slot_b8_table() {
    if !effects_rtdata::have_tables() {
        return;
    }
    assert!(random_access("warp_rel", 1) && !random_access("warp_rel", 2));
    assert!(random_access("blur_warp", 0) && !random_access("blur_warp", 1));
    assert!(!random_access("hue_shift", 0));
}
