//! T22 (INT9, Batch 41 rule 4, W323; WPG15 PATCH 2): the INPUT-PRESENCE-MASK is one native input. Both readers derive from it:
//! the bind pass (bit = node index minus 3 = declared position, INFERRED) and the range pass (bit = record field +0xba = declared position).
//! (1) presence 0 equals the state A form (range mask 0, no image binding); (2) presence 1 with the Mask channel count equals the state A
//! form with range mask 1 and Mask bound as an image; (3) the plans at 0 and 1 differ (the bind pass reads the bit);
//! (4) MIXED-MASK, INPUT-CHANNELS-REQUIRED and PRESENCE-MASK-WIDTH are errors; the named option --mixed-mask admits a mixed state.
use fx_eval::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn combos() -> ComboTable {
    let p = effects_vecdir::path("fx_eval/data").join("COMBOS.tsv");
    ComboTable::from_tsv(&std::fs::read_to_string(p).unwrap())
}

// Warp_Perspective's input order: Mask is declared first (position 0), Source second (position 1, no default).
const WP: &str = "(def_effect T22_Mask ( ( Mask 1 ) Source ) ( ) ( amount = 0.5 ) ( * Source ( * Mask amount ) ))";

fn genotype() -> Genotype {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", WP).unwrap();
    build_effect(&c, c.find_effect("T22_Mask", None).unwrap()).unwrap()
}

fn plan(o: &CompileOptions) -> Result<String, String> {
    let cp = compile(&genotype(), &combos(), o).map_err(|e| e.msg)?;
    Ok(format!("{:?}|{:?}", cp.nodes, cp.params))
}

fn base() -> CompileOptions {
    CompileOptions { minimal_context: true, ..Default::default() }
}

#[test]
fn presence_zero_equals_state_a_mask_zero() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    let legacy = plan(&base()).unwrap();
    let pm = plan(&CompileOptions { presence_mask: Some(0), ..base() }).unwrap();
    assert_eq!(legacy, pm);
}

#[test]
fn presence_one_equals_state_a_mask_one_with_mask_image() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    let mut be = BTreeMap::new();
    be.insert("Mask".to_string(), 4);
    let legacy = plan(&CompileOptions { bound_input_mask: 1, bound_elnum: be.clone(), ..base() }).unwrap();
    let pm = plan(&CompileOptions { presence_mask: Some(1), bound_elnum: be, ..base() }).unwrap();
    assert_eq!(legacy, pm);
    assert_ne!(pm, plan(&base()).unwrap(), "the bind pass reads bit 0: Mask stays an image at presence 1");
}

#[test]
fn mixed_and_missing_channel_states_are_errors() {
    if !effects_vecdir::have_all(&["fx_eval/data"]) {
        return;
    }
    let mut be = BTreeMap::new();
    be.insert("Mask".to_string(), 4);
    // range mask beside the presence byte, different value
    let e = plan(&CompileOptions { presence_mask: Some(1), bound_input_mask: 3, bound_elnum: be.clone(), ..base() }).unwrap_err();
    assert!(e.starts_with("MIXED-MASK"), "{}", e);
    // image binding with the bit clear
    let e = plan(&CompileOptions { presence_mask: Some(0), bound_elnum: be.clone(), ..base() }).unwrap_err();
    assert!(e.starts_with("MIXED-MASK"), "{}", e);
    // set bit for an input with a default and no channel count
    let e = plan(&CompileOptions { presence_mask: Some(1), ..base() }).unwrap_err();
    assert!(e.starts_with("INPUT-CHANNELS-REQUIRED"), "{}", e);
    // a bit above the declared inputs
    let e = plan(&CompileOptions { presence_mask: Some(4), ..base() }).unwrap_err();
    assert!(e.starts_with("PRESENCE-MASK-WIDTH"), "{}", e);
    // the named option admits the mixed state (attribution only) and keeps the given range mask
    assert!(plan(&CompileOptions { presence_mask: Some(0), bound_elnum: be, mixed_mask: true, ..base() }).is_ok());
}

#[test]
fn bit_rows_name_both_readers() {
    let rows = compile::presence_bit_rows(&genotype());
    assert_eq!(rows.len(), 2);
    assert_eq!((rows[0].0, rows[0].1.as_str(), rows[0].3, rows[0].4, rows[0].5), (0, "Mask", 3, Some(0), Some(0)));
    assert_eq!((rows[1].0, rows[1].1.as_str(), rows[1].3, rows[1].4, rows[1].5), (1, "Source", 4, Some(1), Some(1)));
}
