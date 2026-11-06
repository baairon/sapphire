//! the unbound fixed-slider (host type 2) default is the registered 16.16 value f32(trunc(d x 65536) / 65536)
//! of the text default d for parameters outside the normalize_scale table; bound values are taken as given (the harness's snap_16_16);
//! the normalize_scale path is unchanged; a negative off-grid default refuses PARAM-DEFAULT-OFFGRID-NEGATIVE. Mutants (fx_eval features):
//! df_round_mut, df_f32text_mut, df_bound_mut, df_ns_mut; each fails at least one test here. Also TICKMAP-INDEX-NOT-REGISTERED.
use fx_eval::hostcurve::{fold, fold_with_table, HostCurveValues};
use fx_eval::units::{ns_of, snap_16_16, style_param_row};
static RECORDS: effects_rtdata::Table = effects_rtdata::Table::new("data/TICKMAP_PARAM_RECORDS.tsv");
static STYLE_PARAM_INDEX: effects_rtdata::Table = effects_rtdata::Table::new("data/STYLE_PARAM_INDEX.tsv");
use fx_eval::*;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn combos() -> ComboTable {
    let p = effects_vecdir::path("fx_eval/data").join("COMBOS.tsv");
    ComboTable::from_tsv(&std::fs::read_to_string(p).unwrap())
}

fn corpus() -> Corpus {
    let tsv = std::fs::read_to_string(effects_vecdir::path_str("record_extract/programs/programs.tsv")).expect("programs.tsv");
    Corpus::from_programs_tsv(&tsv)
}

fn params_of(c: &Corpus, prog: &str, sets: &[(&str, f32)]) -> BTreeMap<String, Vec<u32>> {
    let ei = c.find_effect(prog, None).unwrap_or_else(|| panic!("{} not in programs.tsv", prog));
    // the registered genotype (layout row merged): parameters carry their layout names, the keys of STYLE_PARAM_INDEX.tsv
    let (g, _) = fx_eval::layoutmerge::build_registered(c, ei, prog).unwrap();
    let lname = |n: &str| g.params.iter().find(|q| q.name == n).map(|q| if q.layout_name.is_empty() { q.name.clone() } else { q.layout_name.clone() }).unwrap();
    let mut ov = BTreeMap::new();
    for (k, v) in sets {
        ov.insert(k.to_string(), vec![*v]);
    }
    let cp = compile(&g, &combos(), &CompileOptions { overrides: ov, no_fold: true, ..Default::default() }).unwrap_or_else(|e| panic!("{}: {}", prog, e.msg));
    // keyed by layout name
    cp.params.into_iter().map(|(n, l)| (lname(&n), l)).collect()
}

#[test]
fn prev_brightness_unbound_is_registered_16_16() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let p = params_of(&corpus(), "Trails_Autogen", &[]);
    assert_eq!(p["prev_brightness"][0], 0x3f4ccc00, "{:08x}", p["prev_brightness"][0]);
}

#[test]
fn edge_threshold_unbound_is_registered_16_16() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let c = corpus();
    // Sharpen_Fast_Autogen's text has no edge_threshold (its index row is registration only)
    for prog in ["Sharpen_Best_Quality_Autogen"] {
        let p = params_of(&c, prog, &[]);
        assert_eq!(p["edge_threshold"][0], 0x3e199800, "{} {:08x}", prog, p["edge_threshold"][0]);
    }
}

#[test]
fn prev_brightness_bound_is_taken_as_given() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let c = corpus();
    // the harness binds a host value by snap_16_16: 0.8 is 52429 / 65536
    let h = snap_16_16(0.8);
    assert_eq!(h.to_bits(), 0x3f4ccd00);
    let p = params_of(&c, "Trails_Autogen", &[("fade", h)]);
    assert_eq!(p["prev_brightness"][0], 0x3f4ccd00, "{:08x}", p["prev_brightness"][0]);
    // a bound value off the grid is not re-derived (the rule is for unbound parameters only)
    let p = params_of(&c, "Trails_Autogen", &[("fade", 0.8)]);
    assert_eq!(p["prev_brightness"][0], 0x3f4ccccd, "{:08x}", p["prev_brightness"][0]);
}

#[test]
fn blur_mask_unbound_unchanged_normalize_scale_path() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    assert!(ns_of("Trails_Autogen", "blur_mask").is_some());
    assert_eq!(style_param_row("Trails_Autogen", "blur_mask").map(|r| r.host_type), Some(2));
    let p = params_of(&corpus(), "Trails_Autogen", &[]);
    // the parse-time lane of the text 0.05 (as on s31_l2); the rule would give trunc(0.05 x 65536) / 65536 = 0x3d4cc000
    assert_eq!(p["blur_mask"][0], 0x3d4cccce, "{:08x}", p["blur_mask"][0]);
}

const TR: &str = "(def_effect Trails_Autogen ( Source ) ( ) ( prev_brightness = DEF ) ( * Source prev_brightness ))";

fn constructed(d: &str) -> Result<Vec<u32>, String> {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", &TR.replace("DEF", d)).unwrap();
    let ei = c.find_effect("Trails_Autogen", None).unwrap();
    let g = build_effect(&c, ei).unwrap();
    let cp = compile(&g, &combos(), &CompileOptions { no_fold: true, ranges_off: true, ..Default::default() }).map_err(|e| e.msg)?;
    Ok(cp.params.into_iter().find(|x| x.0 == "prev_brightness").unwrap().1)
}

#[test]
fn offgrid_negative_default_refused_on_constructed_text() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let e = constructed("-0.3").unwrap_err();
    assert!(e.starts_with("PARAM-DEFAULT-OFFGRID-NEGATIVE (Trails_Autogen prev_brightness: the unbound fixed-slider default -0.3 is negative and off the 16.16 grid;"), "{}", e);
    // on the grid a negative default is exact, and a positive off-grid one truncates
    assert_eq!(constructed("-0.5").unwrap()[0], (-0.5f32).to_bits());
    assert_eq!(constructed("0.8").unwrap()[0], 0x3f4ccc00);
}

/// for every non-normalize_scale host type 2 parameter of the claimed programs that the program text reads and whose
/// registered value bits are recorded, the native's unbound lane x 65536 equals the bits.
#[test]
fn table_unbound_lane_equals_registered_bits() {
    if !effects_rtdata::have_tables() || !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let c = corpus();
    let mut hdr: Vec<&str> = Vec::new();
    let mut want: BTreeMap<String, Vec<(String, i64)>> = BTreeMap::new();
    for l in STYLE_PARAM_INDEX.lines() {
        if l.starts_with('#') {
            continue;
        }
        let f: Vec<&str> = l.split('\t').collect();
        if f[0] == "program" {
            hdr = f;
            continue;
        }
        let col = |n: &str| f[hdr.iter().position(|h| *h == n).unwrap()];
        if col("host_type") != "2" || col("record_name") == "-" || ns_of(col("program"), col("record_name")).is_some() {
            continue;
        }
        if let Ok(b) = col("registered_value_bits").parse::<i64>() {
            want.entry(col("program").to_string()).or_default().push((col("record_name").to_string(), b));
        }
    }
    let mut checked = 0;
    let mut bad = Vec::new();
    for (prog, rows) in &want {
        let p = params_of(&c, prog, &[]);
        for (name, bits) in rows {
            if let Some(l) = p.get(name) {
                checked += 1;
                let v = f32::from_bits(l[0]) as f64 * 65536.0;
                if v != *bits as f64 {
                    bad.push(format!("{} {}: lane {:08x} x 65536 = {}, registered {}", prog, name, l[0], v, bits));
                }
            }
        }
    }
    println!("DF-TABLE checked {} rows", checked);
    assert!(checked >= 40, "only {} rows checked", checked);
    assert!(bad.is_empty(), "{:?}", bad);
}

/// the table index of a claimed style's parameter must be its registered host index.
#[test]
fn tickmap_index_not_registered_refused() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let h = HostCurveValues::default();
    // the shipped table carries index 23; the refusal is kept on a CONSTRUCTED table, the shipped text with
    // Transform_and_Blur_Autogen's rotate row set back to the old index 0 row
    let old = RECORDS.replace("Transform_and_Blur_Autogen\trotate\t1\t1\t1\t0\t0\t6\t23", "Transform_and_Blur_Autogen\trotate\t1\t1\t1\t0\t0\t6\t0");
    assert_ne!(old.as_str(), &*RECORDS);
    let mut d = Vec::new();
    assert!(fold_with_table(&old, "Transform_and_Blur_Autogen", "rotate", 0.0, &[0.0], 1, &h, &mut d).is_none());
    assert_eq!(d[0], "TICKMAP-INDEX-NOT-REGISTERED (Transform_and_Blur_Autogen rotate: table index 0, the style's registered index 23)");
    // a core program has no registered index row: the check does not apply and the next refusal is reached
    let mut d = Vec::new();
    assert!(fold("Transform_and_Blur", "rotate", 0.0, &[0.0], 1, &h, &mut d).is_none());
    assert!(d[0].starts_with("TICKMAP-HOST-FIELD-UNSOURCED (time_frames)"), "{:?}", d);
}
