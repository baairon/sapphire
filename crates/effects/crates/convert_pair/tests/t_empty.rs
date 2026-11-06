//! EMPTY-ARM-EQUIVALENT check of the port (copy of the crate; this file is new). The original's eight empty-range guards all end in an arm that
//! writes nothing and returns (see EMPTY_ARM.md). These tests show that the port, given an empty range (n = 0), also writes and reads nothing:
//! every observable (plane elements, destination floats) is unchanged, no panic, for all four alpha modes and every cell value.
use convert_pair::*;

const MODES: [AlphaMode; 4] = [AlphaMode::NoWrite, AlphaMode::ClampKeep, AlphaMode::ZeroAlpha, AlphaMode::One];

#[test]
fn reader_n0_reads_and_writes_nothing() {
    let mem: Vec<f32> = Vec::new(); // not even one pixel is readable: any read would panic
    let mut planes: [Vec<f32>; 4] = [vec![1.0; 3], vec![2.0; 3], vec![3.0; 3], vec![4.0; 3]];
    convert_from_source_line(&mem, &mut planes, 0);
    for p in planes.iter() {
        assert!(p.is_empty(), "no plane element may exist for an empty range");
    }
}

#[test]
fn writer_n0_writes_nothing_for_every_mode() {
    let planes: [Vec<f32>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()]; // not even one plane element is readable
    for mode in MODES {
        let mut dest = vec![f32::from_bits(0xaaaa_aaaa); 16];
        let before: Vec<u32> = dest.iter().map(|x| x.to_bits()).collect();
        let out = convert_to_dest_line(&planes, ToDestParams { mode }, &mut dest, 0);
        let after: Vec<u32> = dest.iter().map(|x| x.to_bits()).collect();
        assert_eq!(before, after, "mode {:?}: destination changed for n = 0", mode);
        assert_eq!(out, if mode == AlphaMode::NoWrite { DestOutcome::NotWritten } else { DestOutcome::Written });
    }
}

#[test]
fn band_replication_n0_copies_nothing() {
    for cell in 1..=6usize {
        for y in 0..4usize {
            for last_row in y..y + 6 {
                let mut dest = vec![f32::from_bits(0x5555_5555); 8];
                let before = dest.clone();
                replicate_band(&mut dest, 0, y, cell, last_row);
                assert_eq!(dest, before);
            }
        }
    }
}

#[test]
fn whole_frame_with_zero_width_is_a_no_op() {
    for mode in MODES {
        for h in 0..6usize {
            for cell in 1..=6usize {
                let src: Vec<f32> = Vec::new();
                let mut dest: Vec<f32> = Vec::new();
                let _ = convert_pair_frame(&src, 0, h, cell, ToDestParams { mode }, &mut dest);
                assert!(dest.is_empty());
            }
        }
    }
}
