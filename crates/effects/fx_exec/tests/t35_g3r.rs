//! INT21 step G3R (W476 (d), W481 (a)): R3 0x7e9cc0 on the class-3 path.
//! 1. Distort_RGB_Autogen's cells ([prog+0x590] 2, [prog+0x594] 0, [prog+0x59f] 0x80, [owner+0x1354] 1, [owner+0x1358] 0; DRR2 RECORD.tsv) give
//!    class 3 for Source, class3_planes() = 3, and (3 planes, gate A) derives head 0x7e9cc0.
//!    INT24 (W515 (i), INT21-A4): gate cells (ctx+0x76c, +0x770) = (1, 0): fixture cells INFERRED from DRR2 RECORD.tsv owner cells.
//! 2. The route's reader dispatch (hostconv14::ae32_to_image) on DRR2's dr2h source gives DRR2's dumped 0x7e9cc0 exit rows bit for bit (2 calls,
//!    3 planes x 64 words; fixture fixtures/g3r_dr2h, MANIFEST.sha256; RECORD-RUN, POSTHOC information under W132). The routing mutant
//!    (feature g3r_mut_r3b) must fail this test.
//! 3. G3's variant-B line in convert_pair equals readers14b's read3b on the convert_pair class-3 floor suite (W476 (d) 4, check 11).
use fx_exec::hostconv14::{ae32_to_image, derived_head, Ae32Source, SourceGate, R3_HEAD};
use fx_exec::input_class::{guard_one, ClassCells, RowCells};

fn fx() -> &'static str { effects_vecdir::path_str("fixtures/g3r_dr2h") }

#[test]
fn t35_distort_cells_give_class3_planes3_head_r3() {
    let cells = ClassCells { ae32_ready_c: 1, ae32_ready_d: 0, prog_mode_word: 2, prog_class_byte: Some(0), prog_flag_byte: Some(0x80), flag_2: None };
    let rows = RowCells { fmt_cell_a: 0, b_cell_o: 0, word_cell_a: 0 };
    let c = guard_one("Distort_RGB_Autogen", 0, "Source", &cells, &rows);
    assert_eq!(c, Ok(3), "class of Distort's Source");
    assert_eq!(convert_pair::class3_planes(), 3);
    let g = SourceGate { row_gate_cell: 0, ctx_gate_a: 1, ctx_gate_b: 0 };
    assert_eq!(derived_head(3, &g).unwrap(), R3_HEAD);
}

fn words_le(b: &[u8]) -> Vec<u32> {
    b.chunks(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

#[test]
fn t35_r3_dr2h_rows() {
    if !effects_vecdir::have_all(&["fixtures/g3r_dr2h"]) {
        return;
    }
    let src = words_le(&std::fs::read(format!("{}/SRC.bin", fx())).unwrap());
    let (w, h) = (64usize, 32usize);
    assert_eq!(src.len(), 4 * w * h);
    let img = ae32_to_image(&Ae32Source { words: src, reader_head: R3_HEAD, planes: 3 }, w, h).unwrap();
    assert_eq!(img.planes.len(), 3);
    let rows = std::fs::read_to_string(format!("{}/ROWS.tsv", fx())).unwrap();
    let (mut n, mut bad) = (0usize, 0usize);
    for l in rows.lines().skip(1) {
        let f: Vec<&str> = l.split('\t').collect();
        let y: usize = f[1].parse().unwrap();
        let k: usize = f[3].parse().unwrap();
        let want: Vec<u32> = f[4].split(',').map(|x| u32::from_str_radix(x, 16).unwrap()).collect();
        for (x, wv) in want.iter().enumerate() {
            n += 1;
            if img.planes[k][w * y + x].to_bits() != *wv {
                bad += 1;
            }
        }
    }
    assert_eq!(n, 384, "2 calls x 3 planes x 64 words");
    assert_eq!(bad, 0, "R3 on the route differs from DRR2's dumped 0x7e9cc0 rows in {} of {} words", bad, n);
}

#[test]
fn t35_variant_b_line_equals_read3b() {
    const SPECIAL: [u32; 16] = [
        0x7fc0_0000, 0x7f80_0001, 0xffc0_0000, 0x7f80_0000, 0xff80_0000, 0x8000_0000, 0x0000_0000, 0x0000_0001, 0x807f_ffff, 0x3fc0_0000,
        0xbe80_0000, 0x7f7f_ffff, 0x3f80_0000, 0x3e80_0000, 0x4020_0000, 0x3f40_0000,
    ];
    // the convert_pair class-3 floor suite (crates/convert_pair/tests/t_class3.rs suite())
    let mut mem = Vec::new();
    for a in 0..16 {
        for c in 0..16 {
            mem.extend_from_slice(&[SPECIAL[a], SPECIAL[c], SPECIAL[(c * 5 + 3) % 16], SPECIAL[(c * 7 + a) % 16]]);
        }
    }
    let n = mem.len() / 4;
    let mut g3 = Vec::new();
    convert_pair::convert_from_source_line_3_b(&mem, &mut g3, n);
    let (mut p0, mut p1, mut p2) = (vec![0u32; n], vec![0u32; n], vec![0u32; n]);
    {
        let mut p: [&mut [u32]; 3] = [&mut p0, &mut p1, &mut p2];
        readers14b::read3b(&mem, &mut p, 0, n as i32, false).unwrap();
    }
    assert_eq!(g3.len(), 3);
    assert_eq!(g3[0], p0, "plane 0");
    assert_eq!(g3[1], p1, "plane 1");
    assert_eq!(g3[2], p2, "plane 2");
}
