//! the vector `:hardcode` ported generically. Each of the three vector hardcodes of the registered layout rows gives
//! the value node the lanes, lane count and node type that the genotype reader gives a def vector literal of the same text (the def text rebuilt with
//! the hardcode text in place of the def default); every scalar hardcode keeps the result; the two refusals by name.
use fx_eval::layoutmerge::{find_layout_text, merge_layout, read_layout, LItem};
use fx_eval::{build_effect, Corpus, NodeKind};

fn pt() -> &'static str { effects_vecdir::path_str("record_extract/programs/programs.tsv") }

// (registered program, vector hardcode name, the layout row's literal text)
const V: &[(&str, &str, &str)] = &[
    ("Mosaic_Autogen", "wrap", "[ 2 2 ]"),
    ("Texture_Chroma_Spiral_Autogen", "shift", "[ 0 0 ]"),
    ("Dissolve_Zap", "bg_rel_color", "[ 1 1 1 ]"),
];

/// The programs.tsv text with the def row of `prog` taking `lit` as the default of parameter `name` (the reader's own def vector-literal path).
fn with_def_default(tsv: &str, prog: &str, name: &str, lit: &str) -> String {
    let head = format!("def_effect\t{}\t", prog);
    let mut out = Vec::new();
    let mut done = false;
    for line in tsv.lines() {
        if !done && line.starts_with(&head) {
            let key = format!(" {} = [", name);
            let i = line.find(&key).unwrap_or_else(|| panic!("{}: no vector default for {}", prog, name));
            let j = i + key.len() - 1;
            let k = j + line[j..].find(']').expect("closing bracket");
            out.push(format!("{} {} = {}{}", &line[..i], name, lit, &line[k + 1..]));
            done = true;
        } else {
            out.push(line.to_string());
        }
    }
    assert!(done, "{}: def row not found", prog);
    out.join("\n")
}

#[test]
fn t34_vector_hardcode_lanes_equal_reader() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = Corpus::from_programs_tsv(&tsv);
    for (prog, name, lit) in V {
        let rows = find_layout_text(c.layout_texts.iter().map(|x| x.1.as_str()), prog);
        assert_eq!(rows.len(), 1, "{}: layout rows", prog);
        let mut li = read_layout(rows[0]).unwrap_or_else(|e| panic!("{}: {}", prog, e));
        if *prog == "Texture_Chroma_Spiral_Autogen" {
            // FINDING: this row also carries `:modify... = 0`, which merge refuses first by name; the registered merge stays refused.
            let mut g0 = build_effect(&c, c.find_effect(prog, None).unwrap()).unwrap();
            let e = merge_layout(&mut g0, &li).unwrap_err();
            assert!(e.starts_with("LAYOUT-MODIFY-VALUE-UNSUPPORTED"), "{}", e);
            // test-only: the :modify value flag cleared so that the merge reaches the vector hardcode
            for it in li.items.iter_mut() {
                if let LItem::Modify { name, canon, value, .. } = it {
                    // reader takes the value text as the canonical name of a `NAME = VALUE` modify; the test keeps the def name
                    if *value {
                        *canon = name.clone();
                    }
                    *value = false;
                }
            }
        }
        assert!(li.items.iter().any(|it| matches!(it, LItem::HardcodeVec { name: n, text, .. } if n == name && text == lit)), "{}: vector item", prog);
        let mut g = build_effect(&c, c.find_effect(prog, None).unwrap()).unwrap();
        let ni = g.params.iter().find(|p| p.name == *name).and_then(|p| p.node).expect("def node");
        // FINDING: two of the three defs give the parameter a scalar default (Texture_Chroma_Spiral_Autogen `shift = 0`, Dissolve_Zap
        // `bg_rel_color = 0`) while the layout's hardcode is a vector: refused by 's name. Only Mosaic_Autogen (`wrap = [ 2 2 ]`) merges.
        let refusal = match *prog {
            "Texture_Chroma_Spiral_Autogen" => Some("LAYOUT-HARDCODE-LANES-MISMATCH (Texture_Chroma_Spiral_Autogen: shift: def 1 lanes, hardcode 2)"),
            "Dissolve_Zap" => Some("LAYOUT-HARDCODE-LANES-MISMATCH (Dissolve_Zap: bg_rel_color: def 1 lanes, hardcode 3)"),
            _ => None,
        };
        if let Some(r) = refusal {
            let e = merge_layout(&mut g, &li).unwrap_err();
            assert_eq!(e, r);
            continue;
        }
        let rep = merge_layout(&mut g, &li).unwrap_or_else(|e| panic!("{}: {}", prog, e));
        assert!(!g.params.iter().any(|p| p.name == *name), "{}: hardcoded {} has no record", prog, name);
        assert_eq!(rep.hardcoded_vec.len(), 1, "{}", prog);
        let got = &g.nodes[ni];
        // the reader's lanes for the same literal text: the def default replaced by the hardcode text, built by the genotype reader
        let t2 = with_def_default(&tsv, prog, name, lit);
        let c2 = Corpus::from_programs_tsv(&t2);
        let g2 = build_effect(&c2, c2.find_effect(prog, None).unwrap()).unwrap();
        let n2 = g2.params.iter().find(|p| p.name == *name).and_then(|p| p.node).expect("reader node");
        let want = &g2.nodes[n2];
        assert_eq!(want.kind, NodeKind::Vector, "{}: reader type", prog);
        assert_eq!(got.kind, want.kind, "{}: type", prog);
        assert_eq!(got.nlanes, want.nlanes, "{}: lanes", prog);
        let gb: Vec<u32> = got.lanes.iter().map(|x| x.to_bits()).collect();
        let wb: Vec<u32> = want.lanes.iter().map(|x| x.to_bits()).collect();
        assert_eq!(gb, wb, "{}: lane bits", prog);
        assert_eq!(rep.hardcoded_vec[0].1.iter().map(|x| x.to_bits()).collect::<Vec<_>>(), wb[..want.nlanes].to_vec(), "{}: report", prog);
    }
}

#[test]
fn t34_scalar_hardcodes_unchanged() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    // every scalar :hardcode of every registered layout row: the value (text parsed as f64) and an LItem::Hardcode in the same place
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = Corpus::from_programs_tsv(&tsv);
    let (mut scalars, mut vectors, mut unparsed) = (0usize, 0usize, Vec::new());
    for (_, text) in c.layout_texts.iter() {
        match read_layout(text) {
            Ok(li) => {
                let hs: Vec<&LItem> = li.items.iter().filter(|it| matches!(it, LItem::Hardcode { .. } | LItem::HardcodeVec { .. })).collect();
                assert_eq!(hs.len(), li.hardcodes.len());
                for (it, (n, v)) in hs.iter().zip(li.hardcodes.iter()) {
                    match it {
                        LItem::Hardcode { name, value } => {
                            assert_eq!(name, n);
                            assert_eq!(value.to_bits(), v.parse::<f64>().unwrap().to_bits());
                            scalars += 1;
                        }
                        LItem::HardcodeVec { name, text, .. } => {
                            assert_eq!(name, n);
                            assert_eq!(text, v);
                            assert!(v.parse::<f64>().is_err());
                            vectors += 1;
                        }
                        _ => unreachable!(),
                    }
                }
            }
            Err(e) if e.contains(":hardcode") || e.contains("HARDCODE") => {
                assert!(e.starts_with("LAYOUT-HARDCODE-UNPARSED ("), "{}", e);
                unparsed.push(e);
            }
            // rows unreadable for another reason (not a hardcode) are outside this test, as at s15_lym
            Err(_) => {}
        }
    }
    assert_eq!(vectors, 3, "three vector hardcodes");
    assert!(scalars > 0);
    // the two word-valued rows (Blur_Warp, Chroma_Warp: fade_slow_out = slow_out) are refused by name
    assert_eq!(unparsed.len(), 2, "{:?}", unparsed);
    eprintln!("t34: scalar hardcodes {}, vector {}, unparsed rows {:?}", scalars, vectors, unparsed);
}

#[test]
fn t34_refusals_by_name() {
    if !effects_vecdir::have_all(&["record_extract/programs/programs.tsv"]) {
        return;
    }
    let tsv = std::fs::read_to_string(pt()).expect("programs.tsv");
    let c = Corpus::from_programs_tsv(&tsv);
    let rows = find_layout_text(c.layout_texts.iter().map(|x| x.1.as_str()), "Mosaic_Autogen");
    // lane count differs from the def parameter's
    let t3 = rows[0].replacen(":hardcode wrap = [ 2 2 ]", ":hardcode wrap = [ 2 2 2 ]", 1);
    assert_ne!(t3, rows[0]);
    let li = read_layout(&t3).unwrap();
    let mut g = build_effect(&c, c.find_effect("Mosaic_Autogen", None).unwrap()).unwrap();
    let e = merge_layout(&mut g, &li).unwrap_err();
    assert_eq!(e, "LAYOUT-HARDCODE-LANES-MISMATCH (Mosaic_Autogen: wrap: def 2 lanes, hardcode 3)");
    // any other value form
    let tw = rows[0].replacen(":hardcode wrap = [ 2 2 ]", ":hardcode wrap = [ 2 x ]", 1);
    let e = read_layout(&tw).unwrap_err();
    assert_eq!(e, "LAYOUT-HARDCODE-UNPARSED (Mosaic_Autogen: wrap: [ 2 x ])");
}
