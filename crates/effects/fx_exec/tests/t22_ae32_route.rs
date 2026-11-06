//! T22 (INT9): the 'ae32' source route through FMT3a's readers, the recorded head decides the body,
//! unported heads and the destination (until FMT3c) are refused by name, TABLE-NULL-ON-ROUTE is the native state.
use fx_exec::hostconv14::*;

fn mem(w: usize, h: usize) -> Vec<u32> {
    // A,R,G,B per pixel; alpha 0.5, colours i, -i, NaN for the last pixel's blue
    let mut v = Vec::new();
    for i in 0..w * h {
        let b = if i == w * h - 1 { f32::NAN.to_bits() } else { (i as f32 * 0.25).to_bits() };
        v.extend_from_slice(&[0.5f32.to_bits(), (i as f32).to_bits(), (-(i as f32)).to_bits(), b]);
    }
    v
}

#[test]
fn r4_r3_r1_bodies_by_recorded_head() {
    let (w, h) = (3, 2);
    let m = mem(w, h);
    let im = ae32_to_image(&Ae32Source { words: m.clone(), reader_head: R4_HEAD, planes: 4 }, w, h).unwrap();
    assert_eq!(im.planes.len(), 4);
    for i in 0..w * h {
        assert_eq!(im.planes[0][i].to_bits(), (i as f32 * 0.5).to_bits());
        assert_eq!(im.planes[1][i].to_bits(), (-(i as f32) * 0.5).to_bits());
        assert_eq!(im.planes[3][i].to_bits(), 0.5f32.to_bits());
    }
    // the non-finite sample becomes +0.0 before the product (0xb74750)
    assert_eq!(im.planes[2][w * h - 1].to_bits(), 0);
    let im3 = ae32_to_image(&Ae32Source { words: m.clone(), reader_head: R3_HEAD, planes: 3 }, w, h).unwrap();
    assert_eq!(im3.planes.len(), 3);
    assert_eq!(im3.planes[0], im.planes[0]);
    let im1 = ae32_to_image(&Ae32Source { words: m, reader_head: R1_HEAD, planes: 1 }, w, h).unwrap();
    assert_eq!(im1.planes[0][0].to_bits(), 0.5f32.to_bits());
}

#[test]
fn unported_heads_plane_mismatch_and_dest_are_refused() {
    let m = mem(2, 1);
    for hd in UNPORTED_HEADS {
        let e = ae32_to_image(&Ae32Source { words: m.clone(), reader_head: hd, planes: 4 }, 2, 1).unwrap_err();
        assert!(format!("{:?}", e).contains("READER-BODY-NOT-PORTED"), "{:?}", e);
    }
    let e = ae32_to_image(&Ae32Source { words: m.clone(), reader_head: R4_HEAD, planes: 3 }, 2, 1).unwrap_err();
    assert!(format!("{:?}", e).contains("writes 4 planes"));
    // INT10: the destination is merged (FMT3c); without its record it is refused by name
    let one = fx_exec::Image::from_planes(vec![vec![0.0f32; 2]; 4]).unwrap();
    let e = ae32_dest(&Ae32Route { sources: vec![], dest_stack_word: 0, dest: None }, &one, 2, 1).unwrap_err();
    assert!(format!("{:?}", e).contains("DEST-RECORD-REQUIRED"));
    let e = ae32_dest(&Ae32Route { sources: vec![], dest_stack_word: 1, dest: None }, &one, 2, 1).unwrap_err();
    assert!(format!("{:?}", e).contains("HOST-GATED"));
    assert!(CURVE_TABLE_ON_ROUTE.is_none());
    assert_eq!(STACK_WORD_ON_ROUTE, 0);
}
