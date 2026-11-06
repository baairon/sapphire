//! INT27 step SI (SIP1 outcome A): Hue_Sat_Bright_Autogen at G0, GC, GD, GN with Source at 3 channels compiles to one folded
//! constant node; the 'ae32' constant route renders its lanes through the destination convert. Bits from SIP1's PLANS.tsv (the original's
//! command-0 constants, MEASURED in the emulator): G0 and GC 00000000, GD 3fc00000, GN c0000000 in lanes 0 to 2.
use fx_exec::exec::{compile_for, execute_ae32_const, Binding, HostFrames};
use fx_exec::hostconv14::{Ae32Dest, Ae32Route};
use fx_exec::srcindep::{route_check, single_const3};
use std::collections::BTreeMap;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

fn hsb() -> fx_eval::Genotype {
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = fx_eval::Corpus::from_programs_tsv(&tsv);
    fx_eval::build_effect(&c, c.find_effect("Hue_Sat_Bright_Autogen", None).unwrap()).unwrap()
}

fn bind(ov: &[(&str, Vec<f32>)]) -> Binding {
    let mut b = Binding::defaults();
    b.const_inputs.insert("Mask".into(), vec![1.0]);
    b.input_elnum.insert("Source".into(), 3);
    for (k, v) in ov {
        b.overrides.insert(k.to_string(), v.clone());
    }
    b
}

fn cases() -> Vec<(&'static str, Binding, u32)> {
    vec![
        ("G0", bind(&[("brightness", vec![0.0]), ("scale_colors", vec![1.0, 1.0, 1.0]), ("offset_darks", vec![0.0])]), 0x0000_0000),
        ("GC", bind(&[("brightness", vec![1.0]), ("scale_colors", vec![0.0, 0.0, 0.0]), ("offset_darks", vec![0.0])]), 0x0000_0000),
        ("GD", bind(&[("brightness", vec![0.850006104]), ("offset_darks", vec![1.5])]), 0x3fc0_0000),
        ("GN", bind(&[("brightness", vec![-0.5]), ("scale_colors", vec![1.0, 1.0, 1.0]), ("offset_darks", vec![-2.0])]), 0xc000_0000),
    ]
}

fn route(w: usize, h: usize) -> Ae32Route {
    Ae32Route { sources: Vec::new(), dest_stack_word: 0, dest: Some(Ae32Dest { planes: 3, mode: 2, prior: vec![0u32; 4 * w * h], copier_source: None }) }
}

fn src3() -> Vec<(String, usize)> {
    vec![("Source".to_string(), 3)]
}

fn check(tag: &str) {
    let g = hsb();
    let (w, h) = (64usize, 32usize);
    for (t, b, want) in cases() {
        if t != tag {
            continue;
        }
        let cp = compile_for(&g, &b).expect("compile");
        let lanes = single_const3(&cp).unwrap_or_else(|| panic!("{}: not a single constant node: {:?}", t, cp.nodes.iter().map(|n| (n.kind, n.name.clone())).collect::<Vec<_>>()));
        assert_eq!(&lanes[..3], &[want; 3], "{} plan lanes", t);
        assert_eq!(route_check(&cp, &src3(), Some(3), "S_HueSatBright", t), Ok(true), "{}", t);
        let o = execute_ae32_const(&cp, &b, &route(w, h), &BTreeMap::new(), w, h, &HostFrames::default()).expect("render");
        assert_eq!(o.exec.dest.channels(), 3, "{}", t);
        for p in &o.exec.dest.planes {
            assert!(p.iter().all(|x| x.to_bits() == want), "{} dest plane bits", t);
        }
        for k in 0..w * h {
            for s in 1..4 {
                assert_eq!(o.dest_words[4 * k + s], want, "{} word {} slot {}", t, k, s);
            }
            assert_eq!(o.dest_words[4 * k], 0, "{} slot 0 keeps the prior", t);
        }
    }
}

#[test]
fn sip1_g0_bits() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    check("G0");
}
#[test]
fn sip1_gc_bits() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    check("GC");
}
#[test]
fn sip1_gd_bits() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    check("GD");
}
#[test]
fn sip1_gn_bits() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    check("GN");
}

#[test]
fn sip1_missing_non_source_input_still_refuses() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = hsb();
    let (_, b, _) = cases().remove(0);
    let cp = compile_for(&g, &b).unwrap();
    let names = vec![("Source".to_string(), 3), ("Foreground".to_string(), 3)];
    assert_eq!(route_check(&cp, &names, Some(3), "S_HueSatBright", "G0"), Err("'ae32' source Foreground is not an input of the compiled plan".to_string()));
    // not class 3 (no elnum 3 set for Source): the old refusal is kept
    assert_eq!(route_check(&cp, &src3(), None, "S_HueSatBright", "G0"), Err("'ae32' source Source is not an input of the compiled plan".to_string()));
}

#[test]
fn sip1_four_planes_refused() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = hsb();
    let (_, b, _) = cases().remove(2);
    let cp = compile_for(&g, &b).unwrap();
    let e = route_check(&cp, &[("Source".to_string(), 4)], Some(3), "S_HueSatBright", "GD").unwrap_err();
    assert_eq!(e, "SOURCE-INDEPENDENT-4CH-UNPORTED (S_HueSatBright GD: the native compiled a Source-independent plan at 4 planes; the original's 4-channel plan reads Source (SIP1); W552)");
}

#[test]
fn sip1_source_reading_plan_is_ordinary() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = hsb();
    let b = bind(&[("brightness", vec![1.0])]);
    let cp = compile_for(&g, &b).unwrap();
    assert!(single_const3(&cp).is_none());
    assert_eq!(route_check(&cp, &src3(), Some(3), "S_HueSatBright", "B1"), Ok(false));
}
