//! The host entries get_field, get_frame_width and get_total_frames fold at compile time. get_field is the host field order: 0 on a
//! progressive host, the only one rebuilt, so `if_eq (get_field) 0 a b` keeps arm a and the interlaced frame fetch is pruned; a nonzero
//! field order is refused by name. get_frame_width and get_total_frames take explicit host values (the frame width in pixels and the clip
//! length) and have no default.
use fx_eval::fold::{fold_kind, fold_lanes, FoldKind};
use fx_eval::*;
use std::collections::BTreeMap;

fn gen(texts: &[&str], name: &str) -> Genotype {
    let mut c = Corpus::new();
    for (i, t) in texts.iter().enumerate() {
        let kind = if t.trim_start().starts_with("(defun") { "defun" } else { "def_effect" };
        c.add_text(kind, &format!("t{}", i), "0x0", t).unwrap();
    }
    build_effect(&c, c.find_effect(name, None).unwrap()).unwrap()
}

fn with_host(vals: &[(&str, f32)]) -> CompileOptions {
    let mut host = BTreeMap::new();
    for (k, v) in vals {
        host.insert(k.to_string(), vec![*v]);
    }
    CompileOptions { host, ..Default::default() }
}

// a frame fetch chosen by the field order: progressive hosts take the whole frame, interlaced hosts the field fetch
const FIELDS: &str = "(def_effect FieldsHost ( Source ) ( rgba_ok ) ( k = 1 ) ( if_eq ( get_field ) 0 ( get_frame Source k ) ( interlace_get_frames Source k ) ))";
const WIDTH: &str = "(def_effect WidthHost ( Source ) ( rgba_ok ) ( ) ( * Source ( / 720 ( get_frame_width ) ) ))";
const TOTAL: &str = "(def_effect TotalHost ( Source ) ( rgba_ok ) ( n = 30 ) ( * Source ( / ( get_total_frames ) n ) ))";

#[test]
fn the_three_entries_are_host_folds_with_one_element() {
    for c in ["get_field", "get_frame_width", "get_total_frames"] {
        assert_eq!(fold_kind(c).map(|k| k.0), Some(FoldKind::Host), "{}", c);
        assert_eq!(compile::fixed_out(c), Some(1), "{}", c);
    }
}

#[test]
fn get_field_folds_to_zero_without_a_host_value_and_only_zero_folds() {
    assert_eq!(fold_lanes("get_field", &[], 1, None).unwrap()[0], 0.0f32.to_bits());
    assert_eq!(fold_lanes("get_field", &[], 1, Some(&[0.0])).unwrap()[0], 0.0f32.to_bits());
    assert_eq!(fold_lanes("get_field", &[], 1, Some(&[-0.0])).unwrap()[0], 0.0f32.to_bits());
    for v in [1.0f32, 2.0, -1.0, f32::NAN] {
        assert!(fold_lanes("get_field", &[], 1, Some(&[v])).is_none(), "{}", v);
    }
}

#[test]
fn width_and_total_take_the_host_value_and_have_no_default() {
    for c in ["get_frame_width", "get_total_frames"] {
        assert!(fold_lanes(c, &[], 1, None).is_none(), "{}", c);
        assert_eq!(fold_lanes(c, &[], 1, Some(&[64.0])).unwrap()[0], 64.0f32.to_bits(), "{}", c);
        assert_eq!(fold_lanes(c, &[], 1, Some(&[240.0])).unwrap()[0], 240.0f32.to_bits(), "{}", c);
    }
}

#[test]
fn progressive_field_order_prunes_the_interlaced_fetch() {
    let g = gen(&[FIELDS], "FieldsHost");
    let ct = ComboTable::from_defs();
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert!(cp.folds.iter().any(|f| f.starts_with("get_field#")), "{:?}", cp.folds);
    assert!(!cp.nodes.iter().any(|n| n.name == "get_field" || n.name == "interlace_get_frames"), "{}", cp.plan_tsv());
    assert!(cp.nodes.iter().any(|n| n.name == "get_frame"), "{}", cp.plan_tsv());
    let p = cp.pruned.iter().find(|p| p.class == "if_eq").expect("if_eq pruned");
    assert_eq!(p.arm, 2);
    // an explicit progressive value gives the same plan
    let z = compile(&g, &ct, &with_host(&[("get_field", 0.0)])).unwrap();
    assert_eq!(z.plan_tsv(), cp.plan_tsv());
}

#[test]
fn an_interlaced_field_order_is_refused_by_name() {
    let g = gen(&[FIELDS], "FieldsHost");
    let ct = ComboTable::from_defs();
    for v in [1.0f32, 2.0] {
        let e = compile(&g, &ct, &with_host(&[("get_field", v)])).err().expect("refused");
        assert!(e.msg.starts_with("INTERLACED-FIELDS-NOT-REBUILT"), "{}", e.msg);
    }
}

#[test]
fn frame_width_folds_to_the_host_width() {
    let g = gen(&[WIDTH], "WidthHost");
    let ct = ComboTable::from_defs();
    for w in [64.0f32, 50.0, 1440.0] {
        let cp = compile(&g, &ct, &with_host(&[("get_frame_width", w)])).unwrap();
        assert!(cp.folds.iter().any(|f| f.starts_with("get_frame_width#")), "{:?}", cp.folds);
        assert!(!cp.nodes.iter().any(|n| n.name == "get_frame_width"));
        let want = (720.0f32 / w).to_bits();
        assert!(cp.nodes.iter().any(|n| n.folded && n.lanes[0] == want), "720 / {} missing in {}", w, cp.plan_tsv());
    }
    let cp = compile(&g, &ct, &CompileOptions::default()).unwrap();
    assert!(!cp.folds.iter().any(|f| f.starts_with("get_frame_width#")), "no default width");
}

#[test]
fn total_frames_folds_to_the_clip_length() {
    let g = gen(&[TOTAL], "TotalHost");
    let ct = ComboTable::from_defs();
    for n in [240.0f32, 25.0] {
        let cp = compile(&g, &ct, &with_host(&[("get_total_frames", n)])).unwrap();
        assert!(cp.folds.iter().any(|f| f.starts_with("get_total_frames#")), "{:?}", cp.folds);
        let want = (n / 30.0f32).to_bits();
        assert!(cp.nodes.iter().any(|n| n.folded && n.lanes[0] == want), "{} / 30 missing in {}", n, cp.plan_tsv());
    }
}
