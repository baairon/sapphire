//! LYM3: the layout row's input group natively (W449 (e)), both names per record (W449 (f) 1), and a mutant per rule.
use fx_eval::layoutmerge::{build_registered, merge_layout, read_layout, resolve_layout};
use fx_eval::{build_effect, Corpus, NodeKind};

fn corpus() -> Corpus {
    let tsv = std::fs::read_to_string(effects_vecdir::path_str("record_extract/programs/programs.tsv")).expect("programs.tsv");
    Corpus::from_programs_tsv(&tsv)
}

// (registered program, layout input names in layout order, canonical names in layout order)
const INPUTS: &[(&str, &[&str], &[&str])] = &[
    ("Blur_Color_Autogen", &["Source", "Matte"], &["Source", "Mask"]),
    ("Blur_Mono_Autogen", &["Source", "Matte"], &["Source", "Mask"]),
    ("Transform_and_Blur_Autogen", &["Source", "Mask"], &["Source", "Mask"]),
    ("Blur_Motion_Curves_No_Transform_Autogen", &["Source", "Mask"], &["Source", "Mask"]),
    ("Diffuse_Autogen", &["Source", "Matte"], &["Source", "Mask"]),
    ("Distort_RGB_Autogen", &["Source", "Lens", "Matte"], &["Source", "Lens", "Mask"]),
    ("Drop_Shadow", &["Foreground", "Background", "Matte"], &["Foreground", "Background", "Matte"]),
    ("Hue_Sat_Bright_Autogen", &["Source", "Mask"], &["Source", "Mask"]),
    ("Sharpen_Best_Quality_Autogen", &["Source", "Matte"], &["Source", "Mask"]),
    ("Sharpen_Fast_Autogen", &["Source", "Matte"], &["Source", "Mask"]),
    ("Time_Warp_RGB_Autogen", &["Source", "Mask"], &["Source", "Mask"]),
    ("Trails_Autogen", &["Source", "Mask"], &["Source", "Mask"]),
    ("Warp_Perspective_Autogen", &["Source", "Matte"], &["Source", "Mask"]),
];

#[test]
fn lym3_input_group_of_the_13_rows() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let c = corpus();
    for (prog, lnames, cnames) in INPUTS {
        let ei = c.find_effect(prog, None).unwrap();
        let (g, rep) = build_registered(&c, ei, prog).unwrap_or_else(|e| panic!("{}: {}", prog, e));
        let rep = rep.expect("a layout row");
        let got_l: Vec<&str> = g.inputs.iter().map(|i| i.layout_name.as_str()).collect();
        let got_c: Vec<&str> = g.inputs.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(&got_l, lnames, "{} layout names", prog);
        assert_eq!(&got_c, cnames, "{} canonical names", prog);
        for (k, i) in g.inputs.iter().enumerate() {
            assert_eq!(i.pos, k + 1, "{} position", prog);
            assert_eq!(g.nodes[i.node].input_pos, k + 1, "{} node position", prog);
            assert_eq!(g.nodes[i.node].kind, NodeKind::Input);
        }
        // only Warp_Perspective's group differs from the def's (INPUTS.tsv of LYM2)
        assert_eq!(rep.inputs_changed, *prog == "Warp_Perspective_Autogen", "{} inputs_changed", prog);
        assert_eq!(rep.default_nodes.is_empty(), *prog != "Warp_Perspective_Autogen", "{} default nodes", prog);
        // every parameter keeps a layout name; the :modify renames differ from the canonical name
        assert!(g.params.iter().all(|p| !p.layout_name.is_empty()));
    }
}

#[test]
fn lym3_warp_perspective_group() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let c = corpus();
    let prog = "Warp_Perspective_Autogen";
    let ei = c.find_effect(prog, None).unwrap();
    let def = build_effect(&c, ei).unwrap();
    let dnames: Vec<&str> = def.inputs.iter().map(|i| i.name.as_str()).collect();
    assert_eq!(dnames, ["Mask", "Source", "SparksMatte", "Back"], "the def's order");
    let (g, rep) = build_registered(&c, ei, prog).unwrap();
    let rep = rep.unwrap();
    assert_eq!(g.inputs.len(), 2);
    assert_eq!(g.def_input_order, ["Mask", "Source", "SparksMatte", "Back"]);
    // SparksMatte and Back are the parser's default nodes: the constants 1 and 0
    assert_eq!(rep.default_nodes, vec![("SparksMatte".to_string(), 3, 1.0f32), ("Back".to_string(), 4, 0.0f32)]);
    for (nm, v) in [("SparksMatte", 1.0f32), ("Back", 0.0f32)] {
        let d = def.inputs.iter().find(|i| i.name == nm).unwrap();
        let n = &g.nodes[d.node];
        assert_eq!(n.kind, NodeKind::Scalar, "{}", nm);
        assert_eq!(n.lanes[0], v);
        assert_eq!(n.input_pos, 0);
    }
    // the node count and the node indices are unchanged by the input renumbering
    assert_eq!(g.nodes.len(), def.nodes.len());
    // the Mask input keeps its default 1 and sits at position 2, labelled Matte
    let m = g.inputs.iter().find(|i| i.name == "Mask").unwrap();
    assert_eq!((m.pos, m.layout_name.as_str(), m.default.as_deref()), (2, "Matte", Some("1")));
    let s = g.inputs.iter().find(|i| i.name == "Source").unwrap();
    assert_eq!((s.pos, s.layout_name.as_str()), (1, "Source"));
}

#[test]
fn lym3_record_names_both() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let c = corpus();
    // Distort_RGB_Autogen lmDR3: the original's array holds the layout names; the native keeps the canonical name for binding
    let prog = "Distort_RGB_Autogen";
    let (g, _) = build_registered(&c, c.find_effect(prog, None).unwrap(), prog).unwrap();
    let want = [
        ("mocha_project", "mocha_project"),
        ("blur_mocha_mask", "blur_mocha"),
        ("mocha_mask_opacity", "mocha_opacity"),
        ("invert_mocha_mask", "invert_mocha"),
        ("resize_mocha_mask", "resize_mocha"),
        ("resize_mocha_rel_x", "resize_rel_x"),
        ("resize_mocha_rel_y", "resize_rel_y"),
        ("shift_mocha_mask", "shift_mocha"),
        ("bypass_mocha_mask", "bypass_mocha"),
        ("show_mocha_mask_only", "show_mocha_only"),
        ("combine_masks", "combine_masks"),
    ];
    for (k, (cn, ln)) in want.iter().enumerate() {
        assert_eq!((g.params[k].name.as_str(), g.params[k].layout_name.as_str()), (*cn, *ln), "record {}", k);
    }
    // lmDR3's recorded array: the layout names of the 24 records (MEASURED, LYM2 rec/)
    let rec = [
        "mocha_project", "blur_mocha", "mocha_opacity", "invert_mocha", "resize_mocha", "resize_rel_x", "resize_rel_y", "shift_mocha", "bypass_mocha", "show_mocha_only", "combine_masks",
        "amount", "blur_lens", "rotate_warp_dir", "warp_red", "warp_green", "warp_blue", "amount_rel", "wrap", "filter", "blur_matte", "invert_matte", "matte_use", "opacity",
    ];
    let got: Vec<&str> = g.params.iter().map(|p| p.layout_name.as_str()).collect();
    assert_eq!(got, rec, "layout names equal the recorded array lmDR3");
}

#[test]
fn lym3_mutants_are_refused_or_differ() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let c = corpus();
    let prog = "Warp_Perspective_Autogen";
    let ei = c.find_effect(prog, None).unwrap();
    let mut g = build_effect(&c, ei).unwrap();
    let rows = fx_eval::layoutmerge::find_layout_text(c.layout_texts.iter().map(|x| x.1.as_str()), prog);
    let text = rows[0].to_string();
    // mutant 1: a layout input with a canonical name the def does not have
    let bad = text.replacen("( Matte Mask )", "( Matte Nope )", 1);
    assert_ne!(bad, text, "the row text carries ( Matte Mask )");
    let li = read_layout(&bad).unwrap();
    let e = merge_layout(&mut g, &li).unwrap_err();
    assert!(e.starts_with("LAYOUT-INPUT-UNKNOWN"), "{}", e);
    // mutant 2: the same canonical input twice
    let mut g = build_effect(&c, ei).unwrap();
    let dup = text.replacen("( Matte Mask )", "( Matte Source )", 1);
    let li = read_layout(&dup).unwrap();
    let e = merge_layout(&mut g, &li).unwrap_err();
    assert!(e.starts_with("LAYOUT-INPUT-DUPLICATE"), "{}", e);
    // mutant 3: dropping Mask from the layout leaves a def input whose default (1) is a number: still the constant 1 (a default node), 1 input
    let mut g = build_effect(&c, ei).unwrap();
    let one = text.replacen("Source ( Matte Mask )", "Source", 1);
    if one != text {
        let li = read_layout(&one).unwrap();
        let rep = merge_layout(&mut g, &li).unwrap();
        assert_eq!(g.inputs.len(), 1);
        assert!(rep.default_nodes.iter().any(|d| d.0 == "Mask" && d.2 == 1.0));
    }
    // the core program has no layout row: the def path is unchanged
    let core = "Warp_Perspective";
    assert!(resolve_layout(&c, core).unwrap().is_none());
    let (g, rep) = build_registered(&c, c.find_effect(core, None).unwrap(), core).unwrap();
    assert!(rep.is_none());
    let n: Vec<&str> = g.inputs.iter().map(|i| i.name.as_str()).collect();
    assert_eq!(n, ["Mask", "Source", "SparksMatte", "Back"]);
}
