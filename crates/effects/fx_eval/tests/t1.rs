//! T1: hand-evaluated examples per form, determinism, malformed input, the 38 target programs.
use fx_eval::*;
use std::path::PathBuf;

fn big<F: FnOnce() + Send + 'static>(f: F) {
    std::thread::Builder::new().stack_size(128 << 20).spawn(f).unwrap().join().unwrap();
}

fn gen(texts: &[&str], name: &str) -> Result<Genotype, String> {
    let mut c = Corpus::new();
    for (i, t) in texts.iter().enumerate() {
        let kind = if t.trim_start().starts_with("(defun") { "defun" } else { "def_effect" };
        c.add_text(kind, &format!("t{}", i), "0x0", t).map_err(|e| format!("parse: {}", e))?;
    }
    let ei = c.find_effect(name, None).ok_or("no such effect")?;
    build_effect(&c, ei).map_err(|e| e.to_string())
}

fn shape(g: &Genotype) -> Vec<(String, String, Vec<usize>)> {
    g.nodes.iter().map(|n| (n.kind.as_str().to_string(), if n.kind == NodeKind::Scalar { format!("{}", n.lanes[0]) } else { n.name.clone() }, n.children.clone())).collect()
}

#[test]
fn composite_hand() {
    // Composite: inputs Front, Back, then comp_rgba(Back, Front) as the dest
    let g = gen(&["(def_effect Composite ( Front Back ) ( rgba_ok ) ( ) ( comp_rgba Back Front ))"], "Composite").unwrap();
    assert_eq!(g.nodes.len(), 3);
    assert_eq!(g.nodes[0].kind, NodeKind::Input);
    assert_eq!(g.nodes[0].name, "Front");
    assert_eq!(g.nodes[1].name, "Back");
    assert_eq!(g.nodes[2].name, "comp_rgba");
    assert_eq!(g.nodes[2].children, vec![1, 0]);
    assert_eq!(g.dest, vec![2]);
    assert_eq!(g.flags, vec!["rgba_ok".to_string()]);
}

#[test]
fn trailing_newline_is_harmless() {
    let a = gen(&["(def_effect C ( A B ) ( ) ( ) ( comp_rgba B A ))"], "C").unwrap();
    let b = gen(&["(def_effect C ( A B ) ( ) ( ) ( comp_rgba B A ))\n"], "C").unwrap();
    assert_eq!(genotype_tsv(&a), genotype_tsv(&b));
}

#[test]
fn params_are_lazy_and_unused_ones_come_last() {
    let g = gen(&["(def_effect T ( S ) ( ) ( a = 1 b = 2 ) ( f S a ))"], "T").unwrap();
    // S0, a1 (built when the body references it), f2, b3 (built after the body)
    assert_eq!(g.nodes.iter().map(|n| n.label.clone()).collect::<Vec<_>>(), vec!["S", "a", "dest", "b"]);
    assert_eq!(g.nodes[2].children, vec![0, 1]);
    assert_eq!(g.params.iter().map(|p| p.node).collect::<Vec<_>>(), vec![Some(1), Some(3)]);
    assert!(g.params.iter().all(|p| p.binding == "PROGRAM-DEFAULT"));
}

#[test]
fn dag_sharing_is_by_parse_node_not_by_structure() {
    // the same variable twice shares one node; two equal literals are two nodes; two equal calls are two nodes
    let g = gen(&["(def_effect T ( S ) ( ) ( a = 1 ) ( f ( g a ) ( g a ) 2 2 a ))"], "T").unwrap();
    let sh = shape(&g);
    assert_eq!(sh[1].1, "1"); // a
    let gs: Vec<_> = g.nodes.iter().filter(|n| n.name == "g").collect();
    assert_eq!(gs.len(), 2);
    assert_eq!(g.nodes.iter().filter(|n| n.kind == NodeKind::Scalar).count(), 3); // a, 2, 2
    assert_eq!(g.nodes.last().unwrap().children.len(), 5);
}

#[test]
fn special_variable_is_one_shared_node() {
    let g = gen(&["(def_effect T ( S ) ( ) ( ) ( g xy xy ))"], "T").unwrap();
    assert_eq!(g.nodes.len(), 3);
    assert_eq!(g.nodes[1].kind, NodeKind::Special);
    assert_eq!(g.nodes[1].name, "xy");
    assert_eq!(g.nodes[2].children, vec![1, 1]);
}

#[test]
fn destructuring_let_with_values_defun() {
    let g = gen(
        &[
            "(defun two (p q) (values (f p) (g q)))",
            "(def_effect T ( S ) ( ) ( a = 1 b = 2 ) ( let ( u v = ( two a b ) ) ( h v u ) ))",
        ],
        "T",
    )
    .unwrap();
    // S0, a1, b2 (let nonlocals, in source order), f3(a), g4(b), h5(v=4, u=3)
    let names: Vec<_> = g.nodes.iter().map(|n| n.name.clone()).collect();
    assert_eq!(names, vec!["S", "", "", "f", "g", "h"]);
    assert_eq!(g.nodes[3].children, vec![1]);
    assert_eq!(g.nodes[4].children, vec![2]);
    assert_eq!(g.nodes[5].children, vec![4, 3]);
    assert_eq!(g.dest, vec![5]);
}

#[test]
fn destructuring_count_mismatch_is_an_error() {
    let e = gen(
        &["(defun two (p q) (values (f p) (g q)))", "(def_effect T ( S ) ( ) ( a = 1 b = 2 ) ( let ( u v w = ( two a b ) ) ( h v u w ) ))"],
        "T",
    );
    assert!(e.unwrap_err().contains("Assignment has 3 variables, but rhs has 2 values."));
    let e2 = gen(&["(def_effect T ( S ) ( ) ( ) ( let ( u = ( f S ) ) ( g u ( values S S ) ) ))"], "T");
    assert!(e2.unwrap_err().contains("expected single value but got 2"));
}

#[test]
fn two_namespaces_head_versus_variable() {
    // `rotate` is a parameter and a function head in the same program (HS-9)
    let g = gen(&["(def_effect T ( S ) ( ) ( rotate = 5 ) ( rotate S rotate ))"], "T").unwrap();
    let r = g.nodes.last().unwrap();
    assert_eq!(r.kind, NodeKind::Function);
    assert_eq!(r.name, "rotate");
    assert_eq!(g.nodes[1].kind, NodeKind::Scalar);
    assert_eq!(g.nodes[1].label, "rotate");
    assert_eq!(r.children, vec![0, 1]);
    // and inside a defun the formal named rotate does not hide the head
    let g2 = gen(
        &["(defun d (rotate s) (rotate s rotate))", "(def_effect T ( S ) ( ) ( r = 3 ) ( d r S ))"],
        "T",
    )
    .unwrap();
    let last = g2.nodes.last().unwrap();
    assert_eq!(last.name, "rotate");
    assert_eq!(last.children, vec![0, 1]);
}

#[test]
fn let_builds_only_what_the_result_needs() {
    let g = gen(&["(def_effect T ( S ) ( ) ( ) ( let ( unused = ( zzz S ) used = ( yyy S ) ) ( g used ) ))"], "T").unwrap();
    assert!(g.nodes.iter().all(|n| n.name != "zzz"));
    assert!(g.nodes.iter().any(|n| n.name == "yyy"));
}

#[test]
fn defun_inlining_builds_call_arguments_first_and_even_unused_ones() {
    let g = gen(&["(defun d (a b) (f a))", "(def_effect T ( S ) ( ) ( ) ( d ( x1 S ) ( x2 S ) ))"], "T").unwrap();
    let names: Vec<_> = g.nodes.iter().map(|n| n.name.clone()).collect();
    assert_eq!(names, vec!["S", "x1", "x2", "f"]);
    assert_eq!(g.nodes[3].children, vec![1]);
}

#[test]
fn conditional_primitives_are_recorded_not_pruned() {
    let g = gen(&["(def_effect T ( S ) ( ) ( c = 0 ) ( if+ c ( a S ) ( b S ) ))"], "T").unwrap();
    let last = g.nodes.last().unwrap();
    assert_eq!(last.name, "if+");
    assert_eq!(last.children.len(), 3);
    assert!(g.nodes.iter().any(|n| n.name == "a") && g.nodes.iter().any(|n| n.name == "b"));
}

#[test]
fn vector_and_number_literals() {
    let g = gen(&["(def_effect T ( S ) ( ) ( v = [ 1 .5 -360 ] w = -1.25 ) ( f v w [ 2 3 ] ))"], "T").unwrap();
    let v = &g.nodes[1];
    assert_eq!(v.kind, NodeKind::Vector);
    assert_eq!(&v.lanes[..3], &[1.0f32, 0.5, -360.0]);
    assert_eq!(v.nlanes, 3);
    assert_eq!(g.nodes[2].lanes[0], -1.25f32);
    assert_eq!(g.nodes[3].nlanes, 2);
}

#[test]
fn effect_call_inlines_with_optional_defaults() {
    let g = gen(
        &[
            "(def_effect Inner ( A ) ( ) ( k = 2 ( optional ) ) ( f A k ))",
            "(def_effect Outer ( S ) ( ) ( ) ( Inner S ))",
        ],
        "Outer",
    )
    .unwrap();
    let names: Vec<_> = g.nodes.iter().map(|n| n.name.clone()).collect();
    assert_eq!(names, vec!["S", "", "f"]);
    assert_eq!(g.nodes[1].lanes[0], 2.0);
}

#[test]
fn flag_groups_are_opaque_and_flagrefs_kept() {
    let g = gen(&["(def_effect T ( S ) ( g ) ( a = 0 ( :offsetdarks ) p = 1 ( min * int slider_range -30 30 popup Full Half ) ) ( f S a p ))"], "T").unwrap();
    assert_eq!(g.params[0].info.flagrefs, vec![":offsetdarks".to_string()]);
    assert_eq!(g.params[1].info.min.as_deref(), Some("*"));
    assert!(g.params[1].info.int);
    assert_eq!(g.params[1].info.popup, vec!["Full".to_string(), "Half".to_string()]);
}

#[test]
fn number_regex() {
    for ok in ["0", "-360", ".5", "0.2", "12", "-.5", "1.25"] {
        assert!(fx_eval::parse::is_number(ok), "{}", ok);
    }
    for bad in ["", "-", "1.", "1e3", "a1", "--1", "1.2.3", ".", "+1"] {
        assert!(!fx_eval::parse::is_number(bad), "{}", bad);
    }
}

#[test]
fn hue_sat_bright_hand() {
    let defuns = [
        "(defun SCALE_LIGHTS (scale_lights offset_darks) ( * ( / scale_lights ( abs scale_lights ) ) ( max 0 ( - ( abs scale_lights ) offset_darks ) ) ))",
        "(defun SCALE_LIGHTS_OFFSET_DARKS (src scale_lights offset_darks to_vect_type) ( + ( * src ( v3_to_v4_if4 ( SCALE_LIGHTS scale_lights offset_darks ) 1 to_vect_type ) ) ( v3_to_v4_if4 offset_darks 0 to_vect_type ) ))",
        "(def_effect Hue_Sat_Bright ( Source ) ( generates_color ) ( brightness = 1 hue_shift = 0 offset_darks = 0 ( :offsetdarks ) preserve_luma = 0 ( boolean ) saturation = 1 ( :saturation ) scale_colors = [ 1 1 1 ] ( color ) ) ( let ( d0 = ( sat_scale ( if+ preserve_luma ( hue_shift_preserve_luma Source hue_shift ) ( hue_shift Source hue_shift ) ) saturation ) ) ( SCALE_LIGHTS_OFFSET_DARKS d0 ( * brightness scale_colors ) offset_darks Source ) ))",
    ];
    let g = gen(&defuns, "Hue_Sat_Bright").unwrap();
    // hand evaluation (see README): 0 Source; the let's nonlocal references build, in source order, preserve_luma 1,
    // hue_shift 2, saturation 3, brightness 4, scale_colors 5, offset_darks 6; then the result
    let want: Vec<(&str, Vec<usize>)> = vec![
        ("Source", vec![]),
        ("", vec![]),
        ("", vec![]),
        ("", vec![]),
        ("", vec![]),
        ("", vec![]),
        ("", vec![]),
        ("hue_shift_preserve_luma", vec![0, 2]),
        ("hue_shift", vec![0, 2]),
        ("if+", vec![1, 7, 8]),
        ("sat_scale", vec![9, 3]),
        ("*", vec![4, 5]),
        ("abs", vec![11]),
        ("/", vec![11, 12]),
        ("", vec![]),
        ("abs", vec![11]),
        ("-", vec![15, 6]),
        ("max", vec![14, 16]),
        ("*", vec![13, 17]),
        ("", vec![]),
        ("v3_to_v4_if4", vec![18, 19, 0]),
        ("*", vec![10, 20]),
        ("", vec![]),
        ("v3_to_v4_if4", vec![6, 22, 0]),
        ("+", vec![21, 23]),
    ];
    assert_eq!(g.nodes.len(), want.len());
    for (i, (n, (nm, ch))) in g.nodes.iter().zip(want.iter()).enumerate() {
        assert_eq!(&n.name, nm, "node {}", i);
        assert_eq!(&n.children, ch, "node {}", i);
    }
    let lab = |i: usize| g.nodes[i].label.clone();
    assert_eq!(lab(1), "preserve_luma");
    assert_eq!(lab(2), "hue_shift");
    assert_eq!(lab(3), "saturation");
    assert_eq!(lab(4), "brightness");
    assert_eq!(lab(5), "scale_colors");
    assert_eq!(lab(6), "offset_darks");
    assert_eq!(lab(10), "d0");
    assert_eq!(g.dest, vec![24]);
    // namespaces: `hue_shift` is a parameter (node 2) and a function (node 8)
    assert_eq!(g.nodes[2].kind, NodeKind::Scalar);
    assert_eq!(g.nodes[8].kind, NodeKind::Function);
    assert_eq!(g.params.len(), 6);
    assert_eq!(g.params[4].name, "saturation");
}

#[test]
fn determinism() {
    let defuns = ["(defun d (a) (f a a))", "(def_effect T ( S ) ( ) ( x = 1 ) ( let ( p = ( d S ) ) ( g p x xy ) ))"];
    let a = genotype_tsv(&gen(&defuns, "T").unwrap());
    for _ in 0..5 {
        assert_eq!(a, genotype_tsv(&gen(&defuns, "T").unwrap()));
    }
}

#[test]
fn malformed_input_returns_errors_and_never_panics() {
    let bad: Vec<String> = vec![
        "".into(),
        "(".into(),
        ")".into(),
        "(def_effect".into(),
        "(def_effect %s ".into(),
        "(def_effect T ( S ) ( ) ( ) ( f S )".into(),
        "(def_effect T ( S ) ( ) ( ) ( f S )) extra".into(),
        "(def_effect T ( S ) ( ) ( a 1 ) ( f S ))".into(),
        "(def_effect T ( S ) ( ) ( a = ) ( f S ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( 1 + 2 ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( #ref x ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( f [ 1 ] ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( f [ 1 2 3 4 5 ] ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( f \"unterminated ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( let ( = 1 ) S ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( let ( a = 1 ) ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( f undefined_var ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( let ( a = ( f a ) ) a ))".into(),
        "(def_effect T ( S ) ( ) ( ) ( let ( a = b b = a ) a ))".into(),
        "(defun f (x) (f x))".into(),
        "(layout_effect T)".into(),
        "\u{0}\u{1}\u{ffff}(((".into(),
        format!("(def_effect T ( S ) ( ) ( ) {}S{})", "( f ".repeat(10000), " )".repeat(10000)),
        format!("(def_effect T ( S ) ( ) ( ) {}S{})", "( let ( a = ".repeat(5000), " ) a )".repeat(5000)),
    ];
    big(move || {
        for (i, t) in bad.iter().enumerate() {
            let mut c = Corpus::new();
            let r = c.add_text("def_effect", "T", "0x0", t);
            if r.is_ok() {
                // parsed: building must then fail (or succeed) without panic; the cyclic ones must fail
                let ei = c.find_effect("T", None);
                if let Some(ei) = ei {
                    let b = build_effect(&c, ei);
                    if i == 17 || i == 18 {
                        assert!(b.is_err(), "case {} should fail to build", i);
                    }
                }
            }
        }
        // cyclic defun call
        let mut c = Corpus::new();
        c.add_text("defun", "f", "0x0", "(defun f (x) (f x))").unwrap();
        c.add_text("def_effect", "T", "0x0", "(def_effect T ( S ) ( ) ( ) ( f S ))").unwrap();
        let e = build_effect(&c, c.find_effect("T", None).unwrap()).unwrap_err();
        assert!(e.msg.contains("Cyclic call to defun 'f'"), "{}", e);
        // wrong number of defun arguments
        let mut c = Corpus::new();
        c.add_text("defun", "f", "0x0", "(defun f (x y) (g x y))").unwrap();
        c.add_text("def_effect", "T", "0x0", "(def_effect T ( S ) ( ) ( ) ( f S ))").unwrap();
        let e = build_effect(&c, c.find_effect("T", None).unwrap()).unwrap_err();
        assert!(e.msg.contains("Wrong number of arguments to defun 'f' (got 1, expected 2)"), "{}", e);
    });
}

fn effects_dir() -> PathBuf {
    effects_vecdir::root()
}

#[test]
fn the_38_target_programs_parse_and_build() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv", "fx_eval/data"]) {
        return;
    }
    let base = effects_dir();
    let tsv = base.join("record_extract").join("programs").join("programs.tsv");
    // a copy of audit_ev\PROGRAM_FORMS.tsv (read-only input) lives in data\ so the server tree has it
    let forms = effects_vecdir::path("fx_eval/data").join("PROGRAM_FORMS.tsv");
    let t = std::fs::read_to_string(&tsv).unwrap_or_else(|e| panic!("cannot read {:?}: {}", tsv, e));
    let f = std::fs::read_to_string(&forms).unwrap_or_else(|e| panic!("cannot read {:?}: {}", forms, e));
    big(move || {
        let c = Corpus::from_programs_tsv(&t);
        let mut targets = 0;
        let mut fails = Vec::new();
        let mut milestones = 0;
        for (i, line) in f.lines().enumerate() {
            if i == 0 {
                continue;
            }
            let cols: Vec<&str> = line.split('\t').collect();
            if cols.len() < 3 || cols[2] != "1" {
                continue;
            }
            targets += 1;
            match c.find_effect(cols[0], Some(cols[1])) {
                None => fails.push(format!("{} not parsed", cols[0])),
                Some(ei) => match build_effect(&c, ei) {
                    Ok(g) => {
                        if ["Hue_Sat_Bright", "Diffuse", "Trails", "Time_Warp_RGB", "Transform_and_Blur", "Blur_Motion_Curves_No_Transform"].contains(&cols[0]) {
                            milestones += 1;
                            assert!(g.nodes.len() > 3, "{}", cols[0]);
                        }
                    }
                    Err(e) => fails.push(format!("{}: {}", cols[0], e)),
                },
            }
        }
        assert_eq!(targets, 38);
        assert!(fails.is_empty(), "failures: {:?}", fails);
        assert_eq!(milestones, 6);
        // determinism over the whole corpus
        let a: Vec<String> = (0..c.effects.len()).filter_map(|i| build_effect(&c, i).ok()).map(|g| genotype_tsv(&g)).collect();
        let b: Vec<String> = (0..c.effects.len()).filter_map(|i| build_effect(&c, i).ok()).map(|g| genotype_tsv(&g)).collect();
        assert_eq!(a, b);
    });
}
