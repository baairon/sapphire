//! M3P: the host-datum derivation of [ctx] at the arm (MODE3_DERIVATION.md line 7), and the cells it reads.
//! Rule (MEASURED from the listing by): ctx = 1 exactly when the 590 value is 2 AND desc == 2 AND (in[idx] == 2 OR NOT(in[idx] != 0 AND in[idx] != 0))
//! AND the input world's data pointer differs from the output world's (after the [desc] row-pointer correction); else 0.
//! Host datum: the two data pointers. An in-place host gives equal pointers. `--host-inplace 1` is that datum.

/// The cells the rule reads on the verified render path.
#[derive(Clone, Copy, Debug)]
pub struct CellsMode {
    pub cells_mode_d: u32,
    pub dest_desc_mode: u32,
    pub cells_mode_c: u32,
    pub cells_mode_a: u32,
    pub cells_mode_b: u32,
    pub dest_desc_word: u32,
}

/// ctx for the cells and the pointer compare (`ptrs_equal`: input data pointer == output data pointer).
pub fn fx_run_cell_f_rule(c: &CellsMode, ptrs_equal: bool) -> Result<u32, String> {
    if c.cells_mode_d != 2 || c.dest_desc_mode != 2 {
        return Ok(0);
    }
    if c.dest_desc_word != 0 {
        return Err(format!("DEST_DESC_WORD-ROW-POINTER-CORRECTION-UNPORTED (desc = {}: the bottom-up row pointer correction is not ported; zero in every record)", c.dest_desc_word));
    }
    let both = c.cells_mode_a != 0 && c.cells_mode_b != 0;
    if !(c.cells_mode_c == 2 || !both) {
        return Ok(0);
    }
    Ok(if ptrs_equal { 0 } else { 1 })
}

/// The claim-route cells: desc = 2 by the writer (fifth argument 0 in all 13 callers), in[idx] = 2 and desc = 0
/// (claim-route constants record rj1M3F), in[idx] = the host world's width and height (64, 32 at 64x32).
pub fn claim_cells(cells_mode_d: u32, w: usize, h: usize) -> CellsMode {
    CellsMode { cells_mode_d, dest_desc_mode: 2, cells_mode_c: 2, cells_mode_a: w as u32, cells_mode_b: h as u32, dest_desc_word: 0 }
}

/// (ctx, desc) the mode derivation reads, from the host datum.
pub fn derive(cells_mode_d: u32, host_inplace: bool, w: usize, h: usize) -> Result<(u32, u32), String> {
    let c = claim_cells(cells_mode_d, w, h);
    Ok((fx_run_cell_f_rule(&c, host_inplace)?, c.dest_desc_mode))
}

/// the programs whose own counted records show the five terms (desc, desc, in[idx],
///). record (Hue_Sat_Bright_Autogen, rj1M3F, 64x32), the Time_Warp_RGB_Autogen O2 records (64x32 and 50x7, FIVE_TERMS.tsv) and the Flicker records at 64x32 and 50x7 (read at the descriptor mode test, the pointer comparison and the input row cells: terms 2, 2, 2, widths nonzero, pointers different).
pub const TERMS_RECORDED: &[&str] = &["Hue_Sat_Bright_Autogen", "Time_Warp_RGB_Autogen", "Flicker"];

/// The label for a program whose own record of the terms is owed (None for a program in TERMS_RECORDED).
pub fn terms_unrecorded_label(program: &str) -> Option<String> {
    if TERMS_RECORDED.contains(&program) {
        return None;
    }
    Some(format!("label MODE_SELECT-TERMS-UNRECORDED ({}: ctx derived with in[idx] = 2 and desc = 0 from record; this style's own record of the terms is owed; M3P)", program))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn terms_label_hsb_recorded() {
        assert!(terms_unrecorded_label("Hue_Sat_Bright_Autogen").is_none());
    }
    #[test]
    fn terms_label_time_warp_recorded() {
        assert!(terms_unrecorded_label("Time_Warp_RGB_Autogen").is_none());
    }
    #[test]
    fn terms_label_flicker_recorded() {
        assert!(terms_unrecorded_label("Flicker").is_none());
    }
    #[test]
    fn terms_label_other_style_unrecorded() {
        let t = terms_unrecorded_label("Trails_Autogen").unwrap();
        assert!(t.starts_with("label MODE_SELECT-TERMS-UNRECORDED (Trails_Autogen: ctx derived"));
    }
    fn base() -> CellsMode {
        CellsMode { cells_mode_d: 2, dest_desc_mode: 2, cells_mode_c: 2, cells_mode_a: 64, cells_mode_b: 32, dest_desc_word: 0 }
    }
    #[test]
    fn inplace_gives_zero_distinct_gives_one() {
        assert_eq!(fx_run_cell_f_rule(&base(), true).unwrap(), 0);
        assert_eq!(fx_run_cell_f_rule(&base(), false).unwrap(), 1);
    }
    #[test]
    fn other_590_or_desc24_gives_zero() {
        let mut c = base();
        c.cells_mode_d = 3;
        assert_eq!(fx_run_cell_f_rule(&c, false).unwrap(), 0);
        let mut c = base();
        c.dest_desc_mode = 1;
        assert_eq!(fx_run_cell_f_rule(&c, false).unwrap(), 0);
    }
    #[test]
    fn row_cells_term() {
        let mut c = base();
        c.cells_mode_c = 0; // not 2: both extents nonzero gives 0
        assert_eq!(fx_run_cell_f_rule(&c, false).unwrap(), 0);
        c.cells_mode_a = 0; // empty extent: falls through to the pointer compare
        assert_eq!(fx_run_cell_f_rule(&c, false).unwrap(), 1);
        assert_eq!(fx_run_cell_f_rule(&c, true).unwrap(), 0);
    }
    #[test]
    fn dest_desc_word_nonzero_refused() {
        let mut c = base();
        c.dest_desc_word = 8;
        assert!(fx_run_cell_f_rule(&c, true).unwrap_err().starts_with("DEST_DESC_WORD-ROW-POINTER"));
    }
    #[test]
    fn claim_derive() {
        assert_eq!(derive(2, true, 64, 32).unwrap(), (0, 2));
        assert_eq!(derive(2, false, 64, 32).unwrap(), (1, 2));
    }
}
