//! The flow stencil at an interior row, with distinct rows of plane C: the vertical difference is c[row] - c[row - 1], and the
//! words are computable by hand (small integers, every step exact).
use optical_flow::flow_stencil_rows;

/// b[x] = x, d[x] = x, c[row][x] = (row + 1) * x (so the vertical difference at row 1 is x), scale 0.5.
/// Interior element o: g = b[o] - b[o-1] = 1 (the last element takes -b[o-1]), out = (g + x) * 0.5 + d.
#[test]
fn interior_row_uses_the_vertical_difference_of_plane_c() {
    for width in [9i32, 33, 100] {
        let rows = 3;
        let w = width as usize;
        let b: Vec<f32> = (0..3 * w).map(|i| (i % w) as f32).collect();
        let d = b.clone();
        let c: Vec<f32> = (0..3 * w).map(|i| ((i / w + 1) * (i % w)) as f32).collect();
        for phase in [0usize, 4, 32] {
            let mut out = vec![0.0f32; 3 * w];
            flow_stencil_rows(&mut out, &b, &c, &d, width, rows, width, 0.5, 1, 1, phase);
            // element 0: (b0 + (c1[0] - c0[0])) * 0.5 + d0 = 0
            assert_eq!(out[w].to_bits(), 0.0f32.to_bits(), "width {} phase {}", width, phase);
            for o in 1..w {
                let x = o as f32;
                let g = if o == w - 1 { -((o - 1) as f32) } else { 1.0 };
                let want = (g + x) * 0.5 + x;
                assert_eq!(out[w + o].to_bits(), want.to_bits(), "width {} phase {} o {}", width, phase, o);
            }
            // the rows outside the call are untouched
            assert!(out[..w].iter().chain(out[2 * w..].iter()).all(|v| *v == 0.0));
        }
    }
}
