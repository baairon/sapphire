//! 's cases for fx_exec::input_class (the transcription of 0x6e7f00 and the caller's class-1 test),
//! plus 's no-default rule (a reached record cell without a value is refused by name).
use fx_exec::input_class::*;

fn base() -> ClassCells {
    ClassCells { ae32_ready_c: 2, ae32_ready_d: 0, prog_mode_word: 1, prog_class_byte: Some(0), prog_flag_byte: Some(0), flag_2: Some(0) }
}
const R0: RowCells = RowCells { fmt_cell_a: 0, b_cell_o: 0, word_cell_a: 0 };

#[test]
fn s_diffuse_default_passes() {
    // Source bound, [1354] = 2, [1358] = 0, [prog+0x590] = 1, format word 0: class 4; the record cells are not reached
    let c = ClassCells { prog_class_byte: None, prog_flag_byte: None, flag_2: None, ..base() };
    assert_eq!(input_class_guard("S_Diffuse", &[(0, "Source")], &c, &R0).unwrap(), vec![(0, "Source".to_string(), 4)]);
}

#[test]
fn opacity_answer_1_refuses_class_3() {
    let c = ClassCells { ae32_ready_d: 3, ..base() };
    let e = input_class_guard("S", &[(0, "Source")], &c, &R0).unwrap_err();
    assert!(e.starts_with("INPUT-CLASS-NOT-PORTED (S input Source: derived class 3"), "{}", e);
    let c1 = ClassCells { ae32_ready_d: 3, prog_class_byte: Some(1), ..base() };
    assert!(input_class_guard("S", &[(0, "Source")], &c1, &R0).is_ok());
}

#[test]
fn format_word_1_refuses_class_3() {
    let r = RowCells { fmt_cell_a: 1, b_cell_o: 0, word_cell_a: 0 };
    let e = input_class_guard("S", &[(0, "Source")], &base(), &r).unwrap_err();
    assert!(e.contains("derived class 3"), "{}", e);
    // bit 0 of +0x23c set makes edi 1 again; format word 2 is masked by 0xfffffffd
    assert!(input_class_guard("S", &[(0, "Source")], &base(), &RowCells { fmt_cell_a: 1, b_cell_o: 1, word_cell_a: 0 }).is_ok());
    assert!(input_class_guard("S", &[(0, "Source")], &base(), &RowCells { fmt_cell_a: 2, b_cell_o: 0, word_cell_a: 0 }).is_ok());
}

#[test]
fn matte_with_59f_flag_refuses_class_3() {
    let z = ClassCells { ae32_ready_c: 0, prog_class_byte: Some(0xff), ..base() };
    assert!(input_class_guard("DS", &[(1, "Matte")], &z, &R0).is_ok());
    let z2 = ClassCells { prog_flag_byte: Some(0x40), ..z };
    assert!(input_class_guard("DS", &[(1, "Matte")], &z2, &R0).unwrap_err().contains("derived class 3"));
}

#[test]
fn class_1_test_refuses_class_1() {
    let c = ClassCells { flag_2: Some(1), ..base() };
    let r = RowCells { fmt_cell_a: 0, b_cell_o: 0, word_cell_a: 3 };
    assert!(input_class_guard("c1", &[(1, "Matte")], &c, &r).unwrap_err().contains("derived class 1"));
    // word 0 or flag 0: no class 1
    assert!(input_class_guard("c1", &[(1, "Matte")], &c, &R0).is_ok());
}

#[test]
fn bumps_with_1354_zero_refuses_by_name() {
    let z = ClassCells { ae32_ready_c: 0, ..base() };
    let e = input_class_guard("u", &[(1, "Bumps")], &z, &R0).unwrap_err();
    assert!(e.starts_with("INPUT-CLASS-NOT-PORTED (u input Bumps: class derivation arm not ported"), "{}", e);
}

#[test]
fn reached_cell_without_record_is_refused() {
    // [prog+0x590] 2 and [1358] 0: bit i of [prog+0x594] is read
    let c = ClassCells { prog_mode_word: 2, prog_class_byte: None, ..base() };
    assert_eq!(input_class_guard("H", &[(0, "Source")], &c, &R0).unwrap_err(), "INPUT-CLASS-CELL-UNRECORDED (H: [prog])");
    // Hue_Sat_Bright_Autogen's record ([prog+0x594] 0x02, Source at position 0): class 3; at position 1 the bit is set: class 4
    let h = ClassCells { prog_mode_word: 2, prog_class_byte: Some(0x02), prog_flag_byte: Some(0x80), ..base() };
    assert_eq!(derive_class(0, "Source", &h, &R0).unwrap(), 3);
    assert_eq!(derive_class(1, "Source", &h, &R0).unwrap(), 4);
    // the class-1 flag is read only for a class-1 name with a positive word
    let f = ClassCells { flag_2: None, ..base() };
    assert_eq!(derive_class(0, "Source", &f, &RowCells { fmt_cell_a: 0, b_cell_o: 0, word_cell_a: 5 }).unwrap(), 4);
    assert_eq!(derive_class(1, "Mask", &f, &RowCells { fmt_cell_a: 0, b_cell_o: 0, word_cell_a: 5 }).unwrap_err(), ClassErr::Unrecorded("*"));
}
