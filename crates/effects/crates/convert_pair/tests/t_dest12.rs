// CV12 unit tests of the 1- and 2-plane destination arms. Expected values are the ones the emulated original produced in the T2 runs
// (writer edx per (planes, mode) is also MEASURED in BLE1's 90 original runs: Blur_Mono edx 2 in mode 1, edx 1 in modes 2 and 3).
use convert_pair::*;

fn p4(g: &[f32], a: &[f32]) -> [Vec<f32>; 4] {
    [g.to_vec(), a.to_vec(), Vec::new(), Vec::new()]
}

fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

#[test]
fn writer_args_table() {
    // (image planes, to_dest_params_a, mode) -> (edx, mode)
    assert_eq!(writer_args(1, 4, 1), (1, 3));
    assert_eq!(writer_args(2, 4, 1), (2, 1));
    assert_eq!(writer_args(3, 4, 1), (3, 3));
    assert_eq!(writer_args(4, 4, 1), (4, 1));
    for m in [0i8, 2, 3] {
        assert_eq!(writer_args(1, 4, m), (1, m));
        assert_eq!(writer_args(2, 4, m), (1, m));
        assert_eq!(writer_args(3, 4, m), (3, m));
        assert_eq!(writer_args(4, 4, m), (3, m));
    }
    // min with ctx+0x64
    assert_eq!(writer_args(4, 2, 1), (2, 1));
    assert_eq!(writer_args(4, 1, 1), (1, 1)); // the image plane count (even) decides the mode rewrite, not n
    assert_eq!(writer_args(3, 1, 1), (1, 3));
    assert_eq!(writer_args(2, 1, 2), (1, 2));
    assert_eq!(writer_args(1, 8, 2), (1, 2));
}

#[test]
fn one_plane_modes() {
    let g = [0.25f32, -0.0, f32::NAN, f32::INFINITY, 1.5, -7.0, 1e-40];
    let planes = p4(&g, &[]);
    let n = g.len();
    for (mode, alpha) in [(3i8, 1.0f32), (2, 0.0)] {
        let mut d = vec![f32::from_bits(0xaaaa_aaaa); 4 * n];
        let (edx, o) = convert_to_dest_line_n12(1, 4, mode, &planes, &mut d, n);
        assert_eq!((edx, o), (1, Dest12::Written));
        for i in 0..n {
            assert_eq!(d[4 * i].to_bits(), g[i].to_bits());
            assert_eq!(d[4 * i + 1].to_bits(), g[i].to_bits());
            assert_eq!(d[4 * i + 2].to_bits(), g[i].to_bits());
            assert_eq!(d[4 * i + 3].to_bits(), alpha.to_bits());
        }
    }
    // mode 1 on one plane becomes mode 3 (0xd00a7)
    let mut d = vec![0f32; 4 * n];
    let (edx, o) = convert_to_dest_line_n12(1, 4, 1, &planes, &mut d, n);
    assert_eq!((edx, o), (1, Dest12::Written));
    assert_eq!(d[3], 1.0);
    // mode 0 writes nothing
    let mut d = vec![f32::from_bits(0xaaaa_aaaa); 4 * n];
    let (edx, o) = convert_to_dest_line_n12(1, 4, 0, &planes, &mut d, n);
    assert_eq!((edx, o), (1, Dest12::NotWritten));
    assert!(d.iter().all(|x| x.to_bits() == 0xaaaa_aaaa));
}

#[test]
fn two_plane_mode1_clamps_alpha_and_copies_gray() {
    let g = [0.5f32, 2.0, -3.0, f32::NAN, 0.0, 9.0, -0.0, 0.1];
    let a = [0.25f32, 1.5, -0.5, 0.75, f32::NAN, f32::INFINITY, -0.0, f32::NEG_INFINITY];
    let want_a = [0.25f32, 1.0, 0.0, 0.75, f32::NAN, 1.0, -0.0, 0.0];
    let planes = p4(&g, &a);
    let n = g.len();
    let mut d = vec![f32::from_bits(0xaaaa_aaaa); 4 * n];
    let (edx, o) = convert_to_dest_line_n12(2, 4, 1, &planes, &mut d, n);
    assert_eq!((edx, o), (2, Dest12::Written));
    for i in 0..n {
        for c in 0..3 {
            assert_eq!(d[4 * i + c].to_bits(), g[i].to_bits());
        }
        assert_eq!(d[4 * i + 3].to_bits(), want_a[i].to_bits(), "alpha {}", i);
    }
}

#[test]
fn two_plane_other_modes_use_the_one_plane_arm() {
    let g = [0.5f32, 2.0];
    let a = [0.25f32, 0.75];
    let planes = p4(&g, &a);
    for (mode, alpha) in [(3i8, 1.0f32), (2, 0.0)] {
        let mut d = vec![0f32; 8];
        let (edx, o) = convert_to_dest_line_n12(2, 4, mode, &planes, &mut d, 2);
        assert_eq!((edx, o), (1, Dest12::Written));
        assert_eq!(bits(&d), bits(&[0.5, 0.5, 0.5, alpha, 2.0, 2.0, 2.0, alpha]));
    }
}

#[test]
fn arms_cfbb0_never_selects_are_refused() {
    let planes = p4(&[0.5], &[0.5]);
    let mut d = vec![0f32; 4];
    assert_eq!(convert_to_dest_line_2(&planes, ToDestParams { mode: AlphaMode::One }, &mut d, 1), Dest12::NotReachable);
    assert_eq!(convert_to_dest_line_2(&planes, ToDestParams { mode: AlphaMode::NoWrite }, &mut d, 1), Dest12::NotReachable);
}

#[test]
fn frame_with_band_replication() {
    let rows: Vec<[Vec<f32>; 4]> = (0..4).map(|y| p4(&[y as f32 * 0.1 + 0.1, 0.5], &[0.9, 2.0])).collect();
    let mut d = vec![0f32; 4 * 2 * 4];
    let calls = convert_dest_frame_n12(2, 4, 1, &rows, 2, 4, 2, &mut d);
    assert_eq!(calls, 2);
    // rows 0 and 1 equal (row 1 replicated from row 0), rows 2 and 3 equal
    assert_eq!(bits(&d[0..8]), bits(&d[8..16]));
    assert_eq!(bits(&d[16..24]), bits(&d[24..32]));
    assert_eq!(d[7], 1.0);
}

#[test]
fn one_plane_mode1_reached_through_a_small_ctx64_clamps_the_gray_value() {
    let g = [0.25f32, 2.0, -1.0, f32::NAN, -0.0];
    let planes = p4(&g, &[]);
    let mut d = vec![0f32; 4 * g.len()];
    let (edx, o) = convert_to_dest_line_n12(4, 1, 1, &planes, &mut d, g.len());
    assert_eq!((edx, o), (1, Dest12::Written));
    let want = [0.25f32, 1.0, 0.0, f32::NAN, -0.0];
    for i in 0..g.len() {
        assert_eq!(d[4 * i].to_bits(), g[i].to_bits());
        assert_eq!(d[4 * i + 3].to_bits(), want[i].to_bits());
    }
}
