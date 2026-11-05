//! INT20 step G3: the floor suite of convert_pair's class-3 pair (variant B head 0x7e91c0 and the mode-2 three-plane body 0x82e5ca with
//! the alpha slot from the host source row). Expected bits come from the rule as stated from the listing text (RDR10): plane k = slot k+1
//! if (bits & 0x7fffffff) < 0x7f800000 else +0.0; dest slots 1..3 = planes, slot 0 = source slot 0 (bit copy). Special values in every
//! slot, slot 0 included: NaN (quiet, signalling, negative), both infinities, -0.0, +0.0, denormals, above 1, below 0, max finite.
//! Every mutant of the CONVERT_PAIR_G3_MUT switch must fail this test (FLOOR-MET when all 5 are killed).
use convert_pair::*;

const SPECIAL: [u32; 16] = [
    0x7fc0_0000, 0x7f80_0001, 0xffc0_0000, 0x7f80_0000, 0xff80_0000, 0x8000_0000, 0x0000_0000, 0x0000_0001, 0x807f_ffff, 0x3fc0_0000,
    0xbe80_0000, 0x7f7f_ffff, 0x3f80_0000, 0x3e80_0000, 0x4020_0000, 0x3f40_0000,
];

fn suite() -> Vec<u32> {
    // every ordered combination over slots (alpha, r, g, b) drawn with distinct strides from SPECIAL: 16 * 16 pixels
    let mut v = Vec::new();
    for a in 0..16 {
        for c in 0..16 {
            v.extend_from_slice(&[SPECIAL[a], SPECIAL[c], SPECIAL[(c * 5 + 3) % 16], SPECIAL[(c * 7 + a) % 16]]);
        }
    }
    v
}

fn fin(b: u32) -> u32 {
    if (b & 0x7fff_ffff) < 0x7f80_0000 {
        b
    } else {
        0
    }
}

#[test]
fn class3_pair_floor_suite() {
    let mem = suite();
    let n = mem.len() / 4;
    assert_eq!(class3_planes(), 3, "a class-3 input has 3 planes");
    let mut planes = Vec::new();
    convert_from_source_line_3_b(&mem, &mut planes, n);
    assert_eq!(planes.len(), 3, "variant B writes 3 rows");
    let mut bad = 0usize;
    for i in 0..n {
        for k in 0..3 {
            if planes[k][i] != fin(mem[4 * i + 1 + k]) {
                bad += 1;
            }
        }
    }
    assert_eq!(bad, 0, "source planes differ from the variant B rule");
    let mut dest = vec![0xaaaa_aaaau32; 4 * n];
    convert_to_dest_line_3_m2(&planes, &mem, &mut dest, n).unwrap();
    for i in 0..n {
        assert_eq!(dest[4 * i], mem[4 * i], "pixel {} slot 0 is the source row's slot 0", i);
        for k in 1..4 {
            assert_eq!(dest[4 * i + k], fin(mem[4 * i + k]), "pixel {} slot {}", i, k);
        }
    }
}

#[test]
fn class3_dest_refuses_four_planes() {
    let p = vec![vec![0u32; 1]; 4];
    assert!(convert_to_dest_line_3_m2(&p, &[0; 4], &mut [0; 4], 1).unwrap_err().starts_with("CLASS3-PLANES-MISMATCH"));
}
