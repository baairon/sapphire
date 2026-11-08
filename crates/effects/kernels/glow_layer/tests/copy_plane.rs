use glow_layer::*;

#[test]
fn copy_form1_dense_is_plain_copy() {
    let src: Vec<f32> = (0..12).map(|i| i as f32).collect();
    let mut dst = vec![-1.0f32; 12];
    let r = copy_plane(&mut dst, &src, &RoiRecord::form1(4, 3)).unwrap();
    assert_eq!(dst, src);
    assert_eq!(r, CopyReport { elements: 12, width_mismatch: false });
}

#[test]
fn copy_form2_uses_third_times_height_and_reports_mismatch() {
    let src: Vec<f32> = (0..12).map(|i| i as f32).collect();
    let mut dst = vec![-1.0f32; 12];
    let r = copy_plane(&mut dst, &src, &RoiRecord::form2(4, 3, 2)).unwrap();
    assert_eq!(r.elements, 6);
    assert!(r.width_mismatch);
    assert_eq!(&dst[..6], &src[..6]);
    assert!(dst[6..].iter().all(|&v| v == -1.0));
}

#[test]
fn copy_zero_negative_and_short() {
    let src = vec![1.0f32; 4];
    let mut dst = vec![0.0f32; 4];
    assert_eq!(copy_plane(&mut dst, &src, &RoiRecord::form2(4, 1, 0)).unwrap().elements, 0);
    assert_eq!(copy_plane(&mut dst, &src, &RoiRecord::form2(4, 1, -1)), Err(FxError::Geometry));
    assert_eq!(copy_plane(&mut dst, &src, &RoiRecord::form1(5, 1)), Err(FxError::Stride));
}

#[test]
fn count_wraps_like_the_32_bit_imul() {
    let r = RoiRecord::form2(1, 65536, 65536);
    assert_eq!(r.count(), 0);
}

#[test]
fn roi_pass_matches_in_place_on_dense() {
    let p = BrightParams { thresholds: [0.5, 0.0, 0.0] };
    let src: Vec<f32> = (0..16).map(|i| i as f32 / 8.0 - 0.25).collect();
    let mut a = vec![9.0f32; 16];
    bright_pass_roi(&p, &[&src], &mut [&mut a], &RoiRecord::form1(4, 4)).unwrap();
    let mut b = vec![9.0f32; 16];
    {
        let s = [Plane { data: &src, width: 4, height: 4, stride: 4 }];
        let mut d = [PlaneMut { data: &mut b, width: 4, height: 4, stride: 4 }];
        bright_pass(&p, &s, &mut d).unwrap();
    }
    assert_eq!(a.iter().map(|v| v.to_bits()).collect::<Vec<_>>(), b.iter().map(|v| v.to_bits()).collect::<Vec<_>>());
}
