//! T23 (INT15 and rule 5; UNT3 PATCH 2, UNT5 PATCH 1 to 3): the checkout rule ported as read (fx_eval::checkout).
//! (1) Diffuse_Autogen under Matte None (presence 1, [owner+0x124b] bit 0x10 clear): exactly blur_mask, invert_mask, mask_use are not checked out,
//!     ba = 1 for those three and -1 for every other record, bc bit 0 clear everywhere (UNT5 MEASURED 17 of 17 records);
//! (2) Blur_Color_Autogen: exactly invert_mask and mask_use (UNT5 bcR);
//! (3) a ':blur_mask' token binds nothing (ba = -1, checked out);
//! (4) presence bit 1 set checks the three out; bc with 0x124b bit 0x10 checks out a show_with_mocha record whose mask is absent;
//! (5) mutant: without the show_with_mask arm the Diffuse_Autogen rows fail (the set differs);
//! With CHECKOUT_DUMP=FILE the per-record table of both registered programs is written for the log comparison (tools/checkout_logs15.py).
use fx_eval::checkout::*;
use fx_eval::*;

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

fn genotype(prog: &str) -> Genotype {
    let c = Corpus::from_programs_tsv(&std::fs::read_to_string(pt()).expect("programs.tsv"));
    build_effect(&c, c.find_effect(prog, None).expect("program row")).unwrap()
}

fn text_genotype(t: &str, name: &str) -> Genotype {
    let mut c = Corpus::new();
    c.add_text("def_effect", name, "0x0", t).unwrap();
    build_effect(&c, c.find_effect(name, None).unwrap()).unwrap()
}

#[test]
fn diffuse_autogen_matte_none() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = genotype("Diffuse_Autogen");
    let t = checkout_table(&g, 1, 0, Arms::READ);
    let nots: Vec<&str> = t.iter().filter(|x| !x.2).map(|x| x.0.as_str()).collect();
    assert_eq!(nots, vec!["blur_mask", "invert_mask", "mask_use"]);
    for (n, b, _) in &t {
        let exp = if ["blur_mask", "invert_mask", "mask_use"].contains(&n.as_str()) { 1 } else { -1 };
        assert_eq!(b.ba, exp, "{}", n);
        assert_eq!(b.bc, 0, "{}", n);
    }
    if let Ok(p) = std::env::var("CHECKOUT_DUMP") {
        let mut s = String::from("program\tparam\tba\tbc\tchecked_out_1af1_124b0\n");
        for prog in ["Diffuse_Autogen", "Blur_Color_Autogen"] {
            for (n, b, c) in checkout_table(&genotype(prog), 1, 0, Arms::READ) {
                s += &format!("{}\t{}\t{}\t{}\t{}\n", prog, n, b.ba, b.bc, c as u8);
            }
        }
        std::fs::write(p, s).unwrap();
    }
}

#[test]
fn blur_color_autogen_matte_none() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = genotype("Blur_Color_Autogen");
    assert_eq!(not_checked_out(&g, 1, 0), vec!["invert_mask".to_string(), "mask_use".to_string()]);
}

#[test]
fn colon_name_binds_nothing() {
    let g = text_genotype("(def_effect T23C ( Source ( Mask 1 ) ) ( ) ( blur_mask = 0 ( :blur_mask ) a = 1 ( show_with_mask ) ) ( * Source ( * blur_mask a ) ))", "T23C");
    let t = checkout_table(&g, 1, 0, Arms::READ);
    assert_eq!(t[0].1.ba, -1);
    assert!(t[0].2);
    assert_eq!(t[1].1.ba, 1);
    assert!(!t[1].2);
}

#[test]
fn presence_and_mocha_bits() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = genotype("Diffuse_Autogen");
    assert!(not_checked_out(&g, 3, 0).is_empty());
    let m = text_genotype("(def_effect T23M ( Source ( Mask 1 ) ) ( ) ( a = 1 ( show_with_mask show_with_mocha ) ) ( * Source a ))", "T23M");
    assert_eq!(not_checked_out(&m, 1, 0), vec!["a".to_string()]);
    assert!(not_checked_out(&m, 1, 0x10).is_empty());
}

#[test]
fn mutant_without_show_with_mask_fails_diffuse() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = genotype("Diffuse_Autogen");
    let mu = Arms { show_with_mask: false, ..Arms::READ };
    let nots: Vec<String> = checkout_table(&g, 1, 0, mu).into_iter().filter(|x| !x.2).map(|x| x.0).collect();
    assert_ne!(nots, not_checked_out(&g, 1, 0));
}
