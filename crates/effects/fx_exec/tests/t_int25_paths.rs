//! INT25 step s25_p (INT24 finding 1): the program-text path of a folded node is read from the genotype. Real compile of
//! Hue_Sat_Bright_Autogen at FLD2's binding A (Source 3 channels), whose two v3_to_v4_if4 nodes (genotype 67 and 70) fold.
use fx_exec::ops_census::{genotype_path, node_path_fold, node_path_gid};

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

fn hsb() -> fx_eval::Genotype {
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = fx_eval::Corpus::from_programs_tsv(&tsv);
    fx_eval::build_effect(&c, c.find_effect("Hue_Sat_Bright_Autogen", None).unwrap()).unwrap()
}

#[test]
fn folded_v3_nodes_get_genotype_paths() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = hsb();
    let mut o = fx_eval::CompileOptions::default();
    o.const_inputs.insert("Mask".into(), vec![1.0]);
    o.input_elnum.insert("Source".into(), 3);
    o.overrides.insert("scale_colors".into(), vec![0.8125, 1.0625, 0.9375]);
    o.overrides.insert("brightness".into(), vec![1.0]);
    let _ = fx_eval::fold::v3_take();
    let cp = fx_eval::compile(&g, &fx_exec::exec::combo_table(), &o).expect("compile");
    let v3 = fx_eval::fold::v3_take();
    let gids: Vec<usize> = v3.iter().map(|x| x.0).collect();
    assert!(gids.contains(&67), "{:?}", gids);
    for x in &v3 {
        let p = node_path_fold(&cp, &g, x.0);
        assert!(!p.contains("folded away") && !p.contains("not in the"), "{}", p);
        assert!(p.starts_with("_v3_to_v4_if4: v3_to_v4_if4 "), "{}", p);
        // three argument expressions, none empty
        let args = p.trim_start_matches("_v3_to_v4_if4: v3_to_v4_if4 ").trim_end_matches(" (genotype path)");
        assert!(!args.trim().is_empty(), "{}", p);
        println!("FOLD-PATH\tgid {}\t{}\told {}", x.0, p, node_path_gid(&cp, x.0));
    }
    // genotype 67 = v3_to_v4_if4 of ( * ... ), the scalar 1 and the input Source
    assert!(node_path_fold(&cp, &g, 67).ends_with(" 1 Source (genotype path)"), "{}", node_path_fold(&cp, &g, 67));
}

#[test]
fn genotype_path_names_parameters_and_unknown_ids() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let g = hsb();
    // genotype node 67 = v3_to_v4_if4 of nodes 65 (`*`), 66 (the scalar 1) and 0 (input Source) (INT21 gdump_tree line 74)
    assert_eq!(g.nodes[67].children, vec![65, 66, 0]);
    let p = genotype_path(&g, 67);
    assert!(p.starts_with("_v3_to_v4_if4: v3_to_v4_if4 ( * "), "{}", p);
    assert!(p.ends_with(" 1 Source"), "{}", p);
    // genotype node 70 = v3_to_v4_if4 of offset_darks (a parameter, by name), the scalar 0 and Source
    assert_eq!(genotype_path(&g, 70), "_v3_to_v4_if4: v3_to_v4_if4 offset_darks 0 Source");
    assert_eq!(genotype_path(&g, 100000), "gid 100000 (not in the genotype)");
}
