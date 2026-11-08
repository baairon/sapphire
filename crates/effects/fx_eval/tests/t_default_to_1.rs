//! An unbound parameter flagged default_to_1_if_postdiv defaults to 1.0 on every route (minimal context or not);
//! a bound value replaces it; an unflagged parameter keeps its text default. Label DEFAULT-TO-1-READ (the flag is stored unconditionally).
use fx_eval::*;
use std::collections::BTreeMap;

fn combos() -> ComboTable {
    ComboTable::from_defs()
}

const TXT: &str = "(def_effect T_Flag ( Source ) ( ) ( aa = 0 ( default_to_1_if_postdiv ) bb = 0.25 ( ) ) ( * Source ( + aa bb ) ))";

fn lane(minimal: bool, bind: Option<f32>, name: &str) -> f32 {
    let mut c = Corpus::new();
    c.add_text("def_effect", "t0", "0x0", TXT).unwrap();
    let g = build_effect(&c, c.find_effect("T_Flag", None).unwrap()).unwrap();
    let mut ov = BTreeMap::new();
    if let Some(x) = bind {
        ov.insert("aa".to_string(), vec![x]);
    }
    let cp = compile(&g, &combos(), &CompileOptions { overrides: ov, no_fold: true, minimal_context: minimal, ..Default::default() }).unwrap();
    cp.params.iter().find(|(n, _)| n == name).map(|(_, l)| f32::from_bits(l[0])).unwrap()
}

#[test]
fn flagged_default_is_one_on_both_routes() {
    for minimal in [true, false] {
        assert_eq!(lane(minimal, None, "aa"), 1.0, "minimal={}", minimal);
        assert_eq!(lane(minimal, Some(0.5), "aa"), 0.5, "bound, minimal={}", minimal);
        assert_eq!(lane(minimal, None, "bb"), 0.25, "unflagged, minimal={}", minimal);
    }
}
