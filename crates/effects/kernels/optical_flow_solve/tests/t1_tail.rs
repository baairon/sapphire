//! Hand-evaluated cases of the scalar median callback and of the ordinary tail blocks.
use optical_flow_solve::body::{Callback, LaunchArgs, Level};
use optical_flow_solve::median::*;
use optical_flow_solve::tail::*;

fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|f| f.to_bits()).collect()
}

#[test]
fn median_of_a_ramp_centre_is_the_middle_value() {
    let src = bits(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);
    let mut dst = vec![0u32; 9];
    median_rows(&src, &mut dst, 3, 3, 3, 0, 3).unwrap();
    assert_eq!(f32::from_bits(dst[4]), 5.0);
}

#[test]
fn median_of_a_flat_plane_is_flat_for_odd_and_even_widths() {
    for w in [2i32, 3, 4, 5, 8, 9] {
        for h in [1i32, 2, 3, 4] {
            let n = (w * h + 2) as usize;
            let src = vec![0.75f32.to_bits(); n];
            let mut dst = vec![0u32; n];
            median_rows(&src, &mut dst, w, h, w, 0, h).unwrap();
            assert!(dst[..(w * h) as usize].iter().all(|b| *b == 0.75f32.to_bits()), "w {w} h {h}");
        }
    }
}

#[test]
fn width_one_height_one_reads_the_pitch_padding() {
    let src = bits(&[2.0, 2.0, 0.0, 0.0]);
    let mut dst = vec![0u32; 4];
    median_rows(&src, &mut dst, 1, 1, 2, 0, 1).unwrap();
    assert_eq!(f32::from_bits(dst[0]), 2.0);
}

#[test]
fn nan_plane_gives_nan_and_is_deterministic() {
    let src = vec![0x7fc0_1234u32; 16];
    let (mut a, mut b) = (vec![0u32; 16], vec![0u32; 16]);
    median_rows(&src, &mut a, 4, 4, 4, 0, 4).unwrap();
    median_rows(&src, &mut b, 4, 4, 4, 0, 4).unwrap();
    assert_eq!(a, b);
    assert!(a.iter().all(|v| f32::from_bits(*v).is_nan()));
}

#[test]
fn signed_zero_follows_the_operand_order() {
    // all inputs compare equal: minss and maxss return the second operand, so the result is one of the inputs, bit for bit
    let src: Vec<u32> = (0..16).map(|i| if i % 2 == 0 { 0 } else { 0x8000_0000 }).collect();
    let mut dst = vec![1u32; 16];
    median_rows(&src, &mut dst, 4, 4, 4, 0, 4).unwrap();
    assert!(dst.iter().all(|v| *v == 0 || *v == 0x8000_0000));
}

#[test]
fn row_range_and_error_paths() {
    let src = bits(&[1.0; 9]);
    let mut dst = vec![0u32; 9];
    median_rows(&src, &mut dst, 3, 3, 3, 1, 1).unwrap();
    assert_eq!(&dst[0..3], &[0, 0, 0]);
    assert_ne!(dst[3], 0);
    median_rows(&src, &mut dst, 3, 3, 3, 2, 0).unwrap();
    let mut small = vec![0u32; 2];
    assert!(median_rows(&src, &mut small, 3, 3, 3, 0, 3).is_err());
}

#[derive(Default)]
struct Rec {
    launches: Vec<Callback>,
    copies: Vec<(u64, u64)>,
}

impl TailHost for Rec {
    fn launch(&mut self, args: LaunchArgs) {
        self.launches.push(args.callback());
    }
    fn copy_back(&mut self, plane: u64, median_out: u64, _pitch: u64, _head: [u64; 3]) {
        self.copies.push((plane, median_out));
    }
}

fn level(cols: u32, rows: u32) -> Level {
    let p = |i: u64| 0x1000 * i;
    Level {
        width: cols as i32,
        height: rows as i32,
        stride: cols as i32,
        rect_word3: 0,
        elem_bytes: p(2),
        index: p(3) as u32,
        image_a: p(4),
        image_b: p(5),
        grad_x: p(6),
        grad_y: p(7),
        warped_image: p(8),
        warped_grad_x: p(9),
        warped_grad_y: p(10),
        coeff: p(11),
        flow_u: p(12),
        flow_v: p(13),
        flow_u_next: p(14),
        flow_v_next: p(15),
        nest1_t: p(16),
        nest1_u: p(17),
        nest2_t: p(18),
        nest2_u: p(19),
        relax_first: p(20),
        relax_second: p(21),
        median_out: p(22),
        copied_plane: p(23),
    }
}

#[test]
fn narrow_image_leaves_without_a_call() {
    let mut h = Rec::default();
    median_pair(&level(2, 9), &mut h);
    assert!(h.launches.is_empty() && h.copies.is_empty());
}

#[test]
fn interior_image_launches_the_median_and_copies_twice() {
    let mut h = Rec::default();
    let l = level(3, 3);
    median_pair(&l, &mut h);
    assert_eq!(h.launches, vec![Callback::Median, Callback::Median]);
    assert_eq!(h.copies, vec![(l.flow_u, l.median_out), (l.flow_v, l.median_out)]);
}
