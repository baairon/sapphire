//! T20 (INT8 HC-NO-INTERFACE): a get_curve_val request with neither the keyed nor the unkeyed host-curve interface set is a
//! compile error; HC-ZERO-LEGACY (the named state A option) keeps the zero fold; the unkeyed interface still compiles; a program without
//! get_curve_val is unaffected by the option.
use fx_eval::*;
use std::path::PathBuf;

fn big<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new().stack_size(128 << 20).spawn(f).unwrap().join().unwrap();
}

fn combos() -> ComboTable {
    let p = effects_vecdir::path("fx_eval/data").join("COMBOS.tsv");
    ComboTable::from_tsv(&std::fs::read_to_string(p).unwrap())
}

fn programs() -> Option<Corpus> {
    let base = effects_vecdir::root();
    let t = std::fs::read_to_string(base.join("record_extract").join("programs").join("programs.tsv")).ok()?;
    Some(Corpus::from_programs_tsv(&t))
}

#[test]
fn hc_no_interface_is_an_error_and_legacy_is_named() {
    big(|| {
        let Some(c) = programs() else { return };
        let ct = combos();
        for name in ["Transform_and_Blur", "Blur_Motion_Curves_No_Transform"] {
            let g = build_effect(&c, c.find_effect(name, None).unwrap()).unwrap();
            let e = compile(&g, &ct, &CompileOptions { minimal_context: true, ..Default::default() }).err().expect("HC-NO-INTERFACE must be an error");
            assert!(e.msg.starts_with("HC-NO-INTERFACE"), "{}: {}", name, e.msg);
            let z = compile(&g, &ct, &CompileOptions { minimal_context: true, hc_zero_legacy: true, ..Default::default() }).expect("HC-ZERO-LEGACY compiles");
            assert!(!z.diagnostics.iter().any(|d| d.starts_with("HC-NO-INTERFACE")));
            compile(&g, &ct, &CompileOptions { minimal_context: true, host_curves_unkeyed: true, ..Default::default() }).expect("unkeyed interface compiles");
        }
        let g = build_effect(&c, c.find_effect("Hue_Sat_Bright", None).unwrap()).unwrap();
        let a = compile(&g, &ct, &CompileOptions { minimal_context: true, ..Default::default() }).unwrap().plan_tsv();
        let b = compile(&g, &ct, &CompileOptions { minimal_context: true, hc_zero_legacy: true, ..Default::default() }).unwrap().plan_tsv();
        assert_eq!(a, b, "a program without get_curve_val is unaffected");
    });
}
