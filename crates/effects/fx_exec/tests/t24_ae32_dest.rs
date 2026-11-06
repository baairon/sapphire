//! T24: the 'ae32' destination route through code-14 zero-arm bodies (crate c14) and the row copier (14, 14).
//! R-PARITY (FLOOR PENDING): the bodies' T2 is; this test checks the wiring (record inputs, word layout, prior, copier, refusals).
use fx_exec::hostconv14::*;
use fx_exec::Image;

fn img(planes: Vec<Vec<f32>>) -> Image {
    Image::from_planes(planes).unwrap()
}

fn route(planes: usize, mode: i32, prior: Vec<u32>, copier: Option<usize>, sources: Vec<Ae32Source>) -> Ae32Route {
    Ae32Route { sources, dest_stack_word: 0, dest: Some(Ae32Dest { planes, mode, prior, copier_source: copier }) }
}

#[test]
fn t24_four_planes_mode1_words_and_clamp() {
    let (w, h) = (2, 1);
    let im = img(vec![vec![0.1, 0.2], vec![0.3, 0.4], vec![0.5, 0.6], vec![-0.5, 2.0]]);
    let prior: Vec<u32> = (0..8u32).map(|i| 0xabc0_0000 + i).collect();
    let out = ae32_dest(&route(4, 1, prior, None, vec![]), &im, w, h).unwrap();
    // A,R,G,B per pixel: word 0 = clamp01(alpha), words 1..3 = planes 0..2
    assert_eq!(out[0], 0);
    assert_eq!(out[1], 0.1f32.to_bits());
    assert_eq!(out[3], 0.5f32.to_bits());
    assert_eq!(out[4], 1.0f32.to_bits());
    assert_eq!(out[7], 0.6f32.to_bits());
    // mode 2 on a 4-plane destination is the original's logged no-write: refused by name here
    let e = ae32_dest(&route(4, 2, vec![0; 8], None, vec![]), &im, w, h).unwrap_err();
    assert!(format!("{:?}", e).contains("DEST-MODE-LOGGED"), "{:?}", e);
}

#[test]
fn t24_three_planes_mode2_copier_and_prior() {
    let (w, h) = (2, 2);
    let im = img(vec![vec![1.0; 4], vec![2.0; 4], vec![3.0; 4]]);
    let prior: Vec<u32> = (0..16u32).map(|i| 0x1000 + i).collect();
    // without the copier word 0 keeps the prior (HOST-DEST-PRIOR)
    let out = ae32_dest(&route(3, 2, prior.clone(), None, vec![]), &im, w, h).unwrap();
    assert_eq!(out[0], 0x1000);
    assert_eq!(out[1], 1.0f32.to_bits());
    assert_eq!(out[12], 0x100c);
    // with the copier, word 0 comes from the recorded source row's word 0
    let src: Vec<u32> = (0..16u32).map(|i| 0x7700 + i).collect();
    let s = Ae32Source { words: src, reader_head: R3_HEAD, planes: 3 };
    let out2 = ae32_dest(&route(3, 2, prior.clone(), Some(0), vec![s.clone()]), &im, w, h).unwrap();
    assert_eq!(out2[0], 0x7700);
    assert_eq!(out2[4], 0x7704);
    assert_eq!(out2[14], 2.0f32.to_bits()); // pixel 3 word 2 = plane 1
    // mode 3: word 0 = 1.0, and a copier at a mode other than 2 is refused (order unrecorded)
    let out3 = ae32_dest(&route(3, 3, prior.clone(), None, vec![]), &im, w, h).unwrap();
    assert_eq!(out3[8], 1.0f32.to_bits());
    let e = ae32_dest(&route(3, 3, prior.clone(), Some(0), vec![s]), &im, w, h).unwrap_err();
    assert!(format!("{:?}", e).contains("COPIER-ORDER-UNRECORDED"), "{:?}", e);
    // the recorded plane count must match the image
    let e = ae32_dest(&route(4, 1, prior, None, vec![]), &im, w, h).unwrap_err();
    assert!(format!("{:?}", e).contains("record gives 4"), "{:?}", e);
}

#[test]
fn t24_variant_b_gate_derived() {
    // the 0x6fa810 arm ([owner+0x13e0] == 0) gives (ctx_gate_a, ctx_gate_b) = (0, 1): variant B
    let g = SourceGate::from_owner(false, false, 0);
    assert_eq!(derived_head(4, &g).unwrap(), VB4_HEAD);
    assert_eq!(derived_head(3, &g).unwrap(), VB3_HEAD);
    assert_eq!(derived_head(1, &g).unwrap(), VB1_HEAD);
    // ordinary arm, negotiation bit clear: (1, 0): variant A; set: (0, 1): variant B; row_gate_cell matters only at (0, 0)
    let a = SourceGate::from_owner(true, false, 0);
    assert_eq!(derived_head(4, &a).unwrap(), R4_HEAD);
    assert_eq!(derived_head(4, &SourceGate::from_owner(true, true, 0)).unwrap(), VB4_HEAD);
    // the (0, 0) state is refused by name instead of reading [row+0x58]
    let e00 = derived_head(4, &SourceGate { row_gate_cell: 1, ctx_gate_a: 0, ctx_gate_b: 0 }).unwrap_err();
    assert!(format!("{:?}", e00).contains("SOURCE-GATE-ROW58-UNRESOLVED"));
    let e = derived_source(vec![0; 8], 4, &g, Some(R4_HEAD)).unwrap_err();
    assert!(format!("{:?}", e).contains("SOURCE-VARIANT-MISMATCH"));
    // variant B does not multiply by alpha: colour 2.0 with alpha 0.5 stays 2.0
    let s = derived_source(vec![0.5f32.to_bits(), 2.0f32.to_bits(), 2.0f32.to_bits(), 2.0f32.to_bits()], 4, &g, Some(VB4_HEAD)).unwrap();
    let im = ae32_to_image(&s, 1, 1).unwrap();
    assert_eq!(im.planes[0][0], 2.0);
    assert_eq!(im.planes[3][0], 0.5);
}
