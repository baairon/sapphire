//! CV3 T1: the native convert pair wired into the executor (host memory in, host memory out).
use convert_pair::AlphaMode;
use fx_exec::cmds::execute_commands_hc;
use fx_exec::*;

fn px(b: f32, g: f32, r: f32, a: f32) -> [f32; 4] {
    [b, g, r, a]
}

#[test]
fn from_source_hand_example() {
    // host (B,G,R,A) = (0.25, 0.5, 0.75, 0.5): planes R,G,B,A = (0.375, 0.25, 0.125, 0.5)
    let mem: Vec<f32> = px(0.25, 0.5, 0.75, 0.5).to_vec();
    let im = host_to_image(&mem, 1, 1).unwrap();
    assert_eq!(im.channels(), 4);
    assert_eq!([im.planes[0][0], im.planes[1][0], im.planes[2][0], im.planes[3][0]], [0.375, 0.25, 0.125, 0.5]);
}

#[test]
fn from_source_removes_non_finite_and_premultiplies_per_row() {
    let nan = f32::from_bits(0x7fc0_0000);
    let mut mem = Vec::new();
    mem.extend_from_slice(&px(nan, f32::INFINITY, 2.0, 0.5)); // B,G removed (+0.0), R = 2*0.5
    mem.extend_from_slice(&px(1.0, 1.0, 1.0, nan)); // alpha removed: everything 0
    mem.extend_from_slice(&px(-0.0, 4.0, 1.0, 1.5)); // alpha above 1 is kept by the reader
    mem.extend_from_slice(&px(1.0, 1.0, 1.0, -0.5));
    let im = host_to_image(&mem, 2, 2).unwrap();
    assert_eq!(im.planes[0], vec![1.0, 0.0, 1.5, -0.5]);
    assert_eq!(im.planes[1][0].to_bits(), 0.0f32.to_bits());
    assert_eq!(im.planes[2][0].to_bits(), 0.0f32.to_bits());
    assert_eq!(im.planes[3], vec![0.5, 0.0, 1.5, -0.5]);
    // -0.0 * 1.5 is -0.0 (sign kept)
    assert_eq!(im.planes[2][2].to_bits(), (-0.0f32 * 1.5).to_bits());
}

#[test]
fn to_dest_modes_hand_example() {
    let img = Image::from_planes(vec![vec![0.1, 0.2], vec![0.3, 0.4], vec![0.5, 0.6], vec![1.5, -0.25]]).unwrap();
    let pre = vec![7.0f32; 8];
    let (m1, w1) = image_to_host(&img, 2, 1, AlphaMode::ClampKeep, &pre).unwrap();
    assert!(w1);
    assert_eq!(m1, vec![0.5, 0.3, 0.1, 1.0, 0.6, 0.4, 0.2, 0.0]);
    let (m2, _) = image_to_host(&img, 2, 1, AlphaMode::ZeroAlpha, &pre).unwrap();
    assert_eq!(m2, vec![0.5, 0.3, 0.1, 0.0, 0.6, 0.4, 0.2, 0.0]);
    let (m3, _) = image_to_host(&img, 2, 1, AlphaMode::One, &pre).unwrap();
    assert_eq!(m3, vec![0.5, 0.3, 0.1, 1.0, 0.6, 0.4, 0.2, 1.0]);
    let (m0, w0) = image_to_host(&img, 2, 1, AlphaMode::NoWrite, &pre).unwrap();
    assert!(!w0);
    assert_eq!(m0, pre);
}

#[test]
fn opaque_round_trip_is_identity() {
    let mem: Vec<f32> = (0..4 * 6).map(|i| if i % 4 == 3 { 1.0 } else { (i as f32) * 0.07 - 0.3 }).collect();
    let im = host_to_image(&mem, 3, 2).unwrap();
    let (back, _) = image_to_host(&im, 3, 2, AlphaMode::ClampKeep, &vec![0.0; 24]).unwrap();
    assert_eq!(back.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), mem.iter().map(|x| x.to_bits()).collect::<Vec<_>>());
}

#[test]
fn shape_errors() {
    assert!(host_to_image(&[0.0; 7], 1, 2).is_err());
    let three = Image::zeros(3, 4);
    assert!(image_to_host(&three, 2, 2, AlphaMode::One, &vec![0.0; 16]).is_err());
}

#[test]
fn command_path_identity_plan_matches_image_path() {
    if !effects_rtdata::have_tables() {
        return;
    }
    // the CPU-driver identity plan of the original (EP-9 NEXT item 6)
    let plan = "(exec-linewise 1 1 ((allocate-images :ref32.2 4 2 cpu)) ((convert-from-source 0 :ref32.2) (convert-to-dest :ref32.2)) ((free-images :ref32.2)))";
    let mem: Vec<f32> = (0..4 * 12).map(|i| ((i * 37 % 17) as f32) * 0.11 - 0.4).collect();
    for mode in [AlphaMode::ClampKeep, AlphaMode::ZeroAlpha, AlphaMode::One, AlphaMode::NoWrite] {
        let hc = HostConvert { src: vec![mem.clone()], mode, dest_prefill: vec![9.0; 48], cell: 1, margin: 0, ae32: None };
        let co = execute_commands_hc(&[plan.to_string()], &hc, 4, 3, &HostFrames::default(), &KParams::standin()).unwrap();
        let im = host_to_image(&mem, 4, 3).unwrap();
        let (want, wrote) = image_to_host(&im, 4, 3, mode, &vec![9.0; 48]).unwrap();
        assert_eq!(co.host_dest.unwrap(), want);
        assert_eq!(co.host_wrote, wrote);
    }
}

#[test]
fn derive_mode_for_hue_sat_bright() {
    // program mode 2 (MEASURED for Hue_Sat_Bright), owner override 0: ctx+0x758 = 0 gives 3, 758 != 0 with dest field 3 gives 1, else 2
    assert_eq!(convert_pair::derive_alpha_mode(0, 2, 0, 0, 4), 3);
    assert_eq!(convert_pair::derive_alpha_mode(0, 2, 1, 3, 4), 1);
    assert_eq!(convert_pair::derive_alpha_mode(0, 2, 1, 2, 4), 2);
    assert_eq!(convert_pair::derive_alpha_mode(1, 2, 0, 0, 4), 1);
    assert_eq!(convert_pair::derive_alpha_mode(1, 2, 0, 0, 3), 3);
}

#[test]
fn cell_gt1_and_margin_are_refused() {
    use fx_exec::{ExecError, HostConvert};
    let base = |cell, margin| HostConvert { src: vec![vec![0.0; 16]], mode: convert_pair::AlphaMode::One, dest_prefill: vec![0.0; 16], cell, margin, ae32: None };
    assert!(base(1, 0).check_claim().is_ok());
    assert!(matches!(base(2, 0).check_claim(), Err(ExecError::CellGt1NotClaimed(2))));
    assert!(matches!(base(1, 1).check_claim(), Err(ExecError::Shape(_))));
}
