//! INT18: snap_16_16 rounds half to even on the 16.16 grid. The half-grid points k + 0.5 for k = 0
//! and k = 2 go to the even neighbour (0 and 2), k = 1 goes up to 2. Asserts written before the first run.
use fx_eval::units::snap_16_16;

const G: f64 = 1.0 / 65536.0;

#[test]
fn snap_half_grid_k0() {
    assert_eq!(snap_16_16(0.5 * G).to_bits(), 0.0f32.to_bits());
}

#[test]
fn snap_half_grid_k2() {
    assert_eq!(snap_16_16(2.5 * G), (2.0 * G) as f32);
}

#[test]
fn snap_half_grid_k1_and_on_grid() {
    assert_eq!(snap_16_16(1.5 * G), (2.0 * G) as f32);
    assert_eq!(snap_16_16(12.0), 12.0f32);
    assert_eq!(snap_16_16(3.25), 3.25f32);
}

// INT18 (PROGRAM-FLAG-READ): [prog+0x59e] bit 4 from the class table. Warp_Perspective_Autogen gets it by calling the core by
// name (one motion_blur vcall per compile in WPG12's records); Diffuse_Autogen has no motion_blur node.
#[test]
fn program_flag_bit4_from_class_table() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }
    let c = fx_eval::Corpus::from_programs_tsv(&std::fs::read_to_string(pt()).expect("programs.tsv"));
    let g = |p: &str| fx_eval::build_effect(&c, c.find_effect(p, None).expect("program row")).unwrap();
    assert!(fx_eval::layout::program_flag_bit4(&g("Warp_Perspective_Autogen")));
    assert!(!fx_eval::layout::program_flag_bit4(&g("Diffuse_Autogen")));
}
