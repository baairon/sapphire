// INT19: LYM2 tests/t27_lym1.rs renamed t31_lym1.rs (the t27 name is INT17s; NEXT.md of INT18, step L).
//! LYM1: the 13 registered programs merged with their layout row (LAY1, W225 (b), W442): extras, hardcodes, parameter counts.
use fx_eval::layoutmerge::{find_layout_text, merge_layout, read_layout};
use fx_eval::{build_effect, Corpus};

// (registered program, expected extras in layout order, :hardcode names, expected record count after the merge or 0 = not measured)
const T: &[(&str, &[&str], &[&str], usize)] = &[
    ("Blur_Color_Autogen", &["soft_borders", "opacity"], &[], 23), // BCJ2's 23-parameter S_Blur record
    ("Blur_Mono_Autogen", &["soft_borders", "opacity"], &[], 0),
    ("Transform_and_Blur_Autogen", &["opacity"], &[], 0),
    ("Blur_Motion_Curves_No_Transform_Autogen", &["opacity"], &[], 0),
    ("Diffuse_Autogen", &[], &[], 0),
    ("Distort_RGB_Autogen", &["opacity"], &["warp_alpha"], 24), // PDV3 pvDR: 24 parameters
    ("Drop_Shadow", &["expand_borders", "opacity"], &[], 0),
    ("Hue_Sat_Bright_Autogen", &[], &[], 0),
    ("Sharpen_Best_Quality_Autogen", &["opacity"], &["show"], 27), // PDV3 pvSH: 27 parameters
    ("Sharpen_Fast_Autogen", &["opacity"], &[], 0),
    ("Time_Warp_RGB_Autogen", &["input_opacity", "output_opacity"], &[], 0),
    ("Trails_Autogen", &["opacity"], &[], 0),
    ("Warp_Perspective_Autogen", &["opacity"], &["comp_premult"], 0),
];

#[test]
fn lym1_thirteen_programs() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let tsv = std::fs::read_to_string(effects_vecdir::path_str("record_extract/programs/programs.tsv")).expect("programs.tsv");
    let c = Corpus::from_programs_tsv(&tsv);
    for (prog, extras, hard, count) in T {
        let rows = find_layout_text(c.layout_texts.iter().map(|x| x.1.as_str()), prog);
        assert_eq!(rows.len(), 1, "{}: layout rows", prog);
        let li = read_layout(rows[0]).unwrap();
        let names: Vec<&str> = li.extras.iter().map(|e| e.name.as_str()).collect();
        assert_eq!(&names, extras, "{}", prog);
        let hn: Vec<&str> = li.hardcodes.iter().map(|h| h.0.as_str()).collect();
        assert_eq!(&hn, hard, "{} hardcode", prog);
        let mut g = build_effect(&c, c.find_effect(prog, None).unwrap()).unwrap();
        let before: Vec<String> = g.params.iter().map(|p| p.name.clone()).collect();
        let rep = merge_layout(&mut g, &li).unwrap_or_else(|e| panic!("{}: {}", prog, e));
        assert_eq!(g.params.len(), before.len() + extras.len() - hard.len(), "{} count", prog);
        for h in &li.hardcodes {
            assert!(!g.params.iter().any(|p| p.name == h.0), "{}: hardcoded {} has no record", prog, h.0);
            assert!(before.contains(&h.0), "{}: hardcoded name {} is a def parameter", prog, h.0);
            let v = rep.hardcoded.iter().find(|x| x.0 == h.0).unwrap().1;
            assert_eq!(v, h.1.parse::<f32>().unwrap());
        }
        // every def parameter appears once; extras sit where the layout row puts them (checked below for the measured styles)
        for b in &before {
            assert!(g.params.iter().any(|p| &p.name == b) || hard.contains(&b.as_str()), "{}: def parameter {} kept", prog, b);
        }
        if *count > 0 {
            assert_eq!(g.params.len(), *count, "{} record count", prog);
        }
    }
}


// Layout-order checks against the recorded host definition lists (PDV3 SETUP json, defs_decoded order after the fixed Load/Save/Matte rows):
// Distort_RGB: ... Amount Blur Lens Rotate Warp Dir Warp Red Warp Green Warp Blue Amount Rel(X,Y) Wrap(X,Y) Filter Blur Matte Invert Matte Matte Use Opacity;
// Sharpen_Best: ... Opacity sits before Mask Use (host ids 52, 540); Blur_Color: Matte Use, Soft Borders, Opacity.
#[test]
fn lym2_layout_order_matches_recorded_host_lists() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let tsv = std::fs::read_to_string(effects_vecdir::path_str("record_extract/programs/programs.tsv")).expect("programs.tsv");
    let c = Corpus::from_programs_tsv(&tsv);
    let order = |prog: &str| -> Vec<String> {
        let rows = find_layout_text(c.layout_texts.iter().map(|x| x.1.as_str()), prog);
        let li = read_layout(rows[0]).unwrap();
        let mut g = build_effect(&c, c.find_effect(prog, None).unwrap()).unwrap();
        merge_layout(&mut g, &li).unwrap().order
    };
    let dr = order("Distort_RGB_Autogen");
    let tail: Vec<&str> = dr.iter().skip(11).map(|s| s.as_str()).collect();
    assert_eq!(tail, ["amount", "blur_lens", "rotate_warp_dir", "warp_red", "warp_green", "warp_blue", "amount_rel", "wrap", "filter", "blur_mask", "invert_mask", "mask_use", "opacity"]);
    assert_eq!(&dr[..11], ["mocha_project", "blur_mocha_mask", "mocha_mask_opacity", "invert_mocha_mask", "resize_mocha_mask", "resize_mocha_rel_x", "resize_mocha_rel_y", "shift_mocha_mask", "bypass_mocha_mask", "show_mocha_mask_only", "combine_masks"]);
    assert!(!dr.iter().any(|n| n == "warp_alpha"));
    let sh = order("Sharpen_Best_Quality_Autogen");
    let po = sh.iter().position(|n| n == "opacity").unwrap();
    let pm = sh.iter().position(|n| n == "mask_use").unwrap();
    assert!(po < pm, "opacity before mask_use (host ids 52 then 540)");
    let bl = order("Blur_Color_Autogen");
    assert_eq!(&bl[bl.len() - 3..], ["mask_use", "soft_borders", "opacity"]);
}
