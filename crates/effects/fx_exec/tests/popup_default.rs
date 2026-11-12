//! An unbound popup parameter of a registered program compiles its registered default, f32(max(v - 1, 0)) for v the
//! registered checkout integer, on every route; a bound value replaces it. No stored data is read.
use fx_eval::{build_effect, Corpus};
use fx_exec::{compile_for, Binding};

fn lane(prog: &str, name: &str, bind: Option<f32>) -> Option<f32> {
    let corpus = Corpus::native();
    let ei = corpus.find_effect(prog, None)?;
    let g = build_effect(&corpus, ei).ok()?;
    let mut b = Binding::defaults();
    b.hc_zero_legacy = true;
    if let Some(x) = bind {
        b.overrides.insert(name.to_string(), vec![x]);
    }
    let cp = compile_for(&g, &b).ok()?;
    cp.params.iter().find(|(n, _)| n == name).map(|(_, l)| f32::from_bits(l[0]))
}

#[test]
fn swishpan_auto_trans_unbound_is_one_and_bound_zero_is_zero() {
    assert_eq!(lane("SwishPan", "auto_trans", None), Some(1.0));
    assert_eq!(lane("SwishPan", "auto_trans", Some(0.0)), Some(0.0));
    assert_eq!(lane("SwishPan", "auto_trans", Some(1.0)), Some(1.0));
}

#[test]
fn plan_route_and_run_route_give_the_same_lane() {
    // the plan tool compiles through fx_eval::compile with default options, fx_run through compile_for
    let corpus = Corpus::native();
    let ei = corpus.find_effect("SwishPan", None).unwrap();
    let g = build_effect(&corpus, ei).unwrap();
    let combos = fx_eval::ComboTable::from_defs();
    let opt = fx_eval::CompileOptions { minimal_context: false, hc_zero_legacy: true, ..Default::default() };
    let cp = fx_eval::compile(&g, &combos, &opt).expect("plan route compiles");
    let plan = cp.params.iter().find(|(n, _)| n == "auto_trans").map(|(_, l)| f32::from_bits(l[0]));
    assert_eq!(plan, lane("SwishPan", "auto_trans", None));
    assert_eq!(plan, Some(1.0));
}

#[test]
fn every_registered_popup_row_gives_its_converted_default() {
    let mut rows = 0;
    let mut compiled = 0;
    let mut skipped: Vec<String> = Vec::new();
    let mut differs: Vec<String> = Vec::new();
    for s in effects_rtdata::styles::STYLES {
        for r in s.style_params {
            let v = match r.default_checkout_integer {
                Some(v) if r.record_name != "-" => v,
                _ => continue,
            };
            rows += 1;
            let want = (v - 1).max(0) as f32;
            assert_eq!(fx_eval::units::registered_popup_default(r.program, r.record_name), Some(want), "{} {}", r.program, r.record_name);
            let text = effects_rtdata::range_defs().find(|d| d.program == r.program && d.parameter == r.record_name).map(|d| d.default);
            if let Some(t) = text {
                if t.trim().parse::<f32>().ok() != Some(want) {
                    println!("TEXT-DEFAULT-DIFFERS {} {}: text {} registered v {} lane {}", r.program, r.record_name, t, v, want);
                    differs.push(format!("{} {}", r.program, r.record_name));
                }
            }
            match lane(r.program, r.record_name, None) {
                Some(x) => {
                    compiled += 1;
                    assert_eq!(x, want, "unbound compile of {} {}", r.program, r.record_name);
                }
                None => skipped.push(format!("{} {}", r.program, r.record_name)),
            }
        }
    }
    println!("popup rows with a registered default: {}; compiled unbound and equal: {}", rows, compiled);
    println!("skipped: {:?}; text default differs: {:?}", skipped, differs);
    assert!(rows >= 20);
    assert_eq!(compiled + skipped.len(), rows);
    // the opacity rows go through the owner-opacity route and combine_new is the layout name of `combine`; they have no unbound lane of their own
    let routed = |n: &str| ["opacity", "input_opacity", "output_opacity", "combine_new"].iter().any(|k| n.ends_with(&format!(" {}", k)));
    let others: Vec<&String> = skipped.iter().filter(|n| !routed(n)).collect();
    assert!(others.is_empty(), "rows whose program does not compile unbound: {:?}", others);
    assert_eq!(skipped.len(), 24, "{:?}", skipped);
    assert!(differs.iter().all(|d| d.starts_with("SwishPan ") || d.starts_with("Dissolve_Shake ")), "a text default other than the auto_trans rows differs: {:?}", differs);
}
