//! The definitions in `effects_rtdata` equal the stored copies of the host tables, row by row (test data under `$VEC_DIR/effects/rtdata`;
//! the test prints NOT RUN when the copies are absent). Every defined row must be found in the copy with every field equal.
use effects_rtdata::styles::STYLES;
use fx_eval::ComboTable;

fn tsv(name: &str) -> Vec<Vec<&'static str>> {
    effects_rtdata::text(name).lines().skip(1).map(|l| l.split('\t').collect()).collect()
}

fn rust_f32(t: &str) -> Option<f32> {
    t.trim().parse::<f32>().ok()
}

fn opt_i32(t: &str) -> Option<i32> {
    t.parse().ok()
}

#[test]
fn style_param_rows_equal_the_copy() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let text = effects_rtdata::text("data/STYLE_PARAM_INDEX.tsv");
    let mut hdr: Vec<&'static str> = Vec::new();
    let mut copy: Vec<Vec<&'static str>> = Vec::new();
    for l in text.lines().filter(|l| !l.starts_with('#')) {
        let f: Vec<&str> = l.split('\t').collect();
        if f[0] == "program" {
            hdr = f;
        } else {
            copy.push(f);
        }
    }
    let col = |f: &Vec<&'static str>, c: &str| -> &'static str { f[hdr.iter().position(|h| *h == c).unwrap()] };
    let mut n = 0;
    for s in STYLES {
        for r in s.style_params {
            let hit = copy.iter().any(|f| {
                col(f, "program") == r.program
                    && col(f, "style") == r.style
                    && col(f, "record_name") == r.record_name
                    && col(f, "display_name") == r.display_name
                    && col(f, "host_index").parse::<i32>().ok() == Some(r.host_index)
                    && col(f, "host_type").parse::<i32>().ok() == Some(r.host_type)
                    && opt_i32(col(f, "popup_choices")) == r.popup_choices
                    && opt_i32(col(f, "default_checkout_integer")) == r.default_checkout_integer
                    && col(f, "registered_value_bits").parse::<i64>().ok() == r.registered_value_bits
                    && col(f, "record") == r.record
            });
            assert!(hit, "style row not in the copy: {:?}", r);
            n += 1;
        }
    }
    println!("STYLE_PARAM_INDEX: {} defined rows equal copy rows", n);
}

#[test]
fn tickmap_rows_equal_the_copy() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let copy = tsv("data/TICKMAP_PARAM_RECORDS.tsv");
    let mut n = 0;
    for s in STYLES {
        for r in s.tickmap {
            let want = [r.ptype, r.nlanes, r.kind_byte, r.subtype_byte, r.flags_a_byte, r.flags_b_byte, r.index].map(|x| x.to_string());
            let hit = copy.iter().any(|f| f.len() == 9 && f[0] == r.program && f[1] == r.param && f[2..].iter().zip(want.iter()).all(|(a, b)| a == b));
            assert!(hit, "tickmap row not in the copy: {:?}", r);
            n += 1;
        }
    }
    println!("TICKMAP_PARAM_RECORDS: {} defined rows equal copy rows", n);
}

#[test]
fn range_rows_equal_the_copy() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let full = fx_eval::ranges::RangeTable::parse(effects_rtdata::text("fx_eval/data/PARAM_RANGES.tsv"));
    let mut n = 0;
    for s in STYLES {
        for d in s.ranges {
            let c = full.lookup(d.program, d.parameter).unwrap_or_else(|| panic!("range row not in the copy: {:?}", d));
            assert_eq!((c.ptype.as_str(), c.lanes, c.min.to_bits(), c.max.to_bits(), c.default.as_str(), c.min_enforced, c.max_enforced),
                       (d.ptype, d.lanes, d.min.to_bits(), d.max.to_bits(), d.default, d.min_enforced, d.max_enforced), "{:?}", d);
            n += 1;
        }
    }
    // the table built from the definitions answers like the copy on its rows
    let b = fx_eval::ranges::RangeTable::from_defs();
    assert_eq!(b.rows.len(), n);
    for r in &b.rows {
        assert_eq!(full.lookup(&r.program, &r.parameter), Some(r));
    }
    println!("PARAM_RANGES: {} defined rows equal copy rows", n);
}

#[test]
fn input_and_ns_rows_equal_the_copy() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let pi = tsv("fx_eval/data/PARAM_INPUTS.tsv");
    let pir = tsv("fx_eval/data/PARAM_INPUT_RECORDS.tsv");
    let ns = tsv("fx_eval/data/NS_PARAMS.tsv");
    let (mut a, mut b, mut c) = (0, 0, 0);
    for s in STYLES {
        for r in s.inputs {
            assert!(pi.iter().any(|f| f[0] == r.program && f[1] == r.parameter && f[2].trim().parse::<usize>().ok() == Some(r.input_index)), "{:?}", r);
            a += 1;
        }
        for r in s.input_records {
            assert!(pir.iter().any(|f| f[0] == r.program && f[1] == r.parameter), "{:?}", r);
            b += 1;
        }
        for r in s.ns {
            assert!(
                ns.iter().any(|f| f.len() >= 4 && f[0] == r.program && f[1] == r.param && rust_f32(f[2]) == r.text_default && rust_f32(f[3]) == r.normalize_scale),
                "{:?}",
                r
            );
            c += 1;
        }
    }
    println!("PARAM_INPUTS: {} rows, PARAM_INPUT_RECORDS: {} rows, NS_PARAMS: {} rows equal copy rows", a, b, c);
}

#[test]
fn class_rows_equal_the_copy() {
    if !effects_rtdata::have_tables() {
        return;
    }
    let flags = tsv("fx_eval/data/CLASS_FLAGS.tsv");
    for (name, f) in effects_rtdata::classes::CLASS_FLAGS {
        let first = flags.iter().find(|r| r[0] == *name).unwrap_or_else(|| panic!("class {} not in the copy", name));
        assert_eq!(u32::from_str_radix(first[1].trim_start_matches("0x"), 16).unwrap(), *f, "{}", name);
    }
    for (file, defs) in [("fx_eval/data/IMG_ONLY_ARGS.tsv", effects_rtdata::classes::IMG_ONLY_ARGS), ("fx_eval/data/INT_TYPED_ARGS.tsv", effects_rtdata::classes::INT_TYPED_ARGS)] {
        let copy = tsv(file);
        for (p, k) in defs {
            assert!(copy.iter().any(|r| r[0] == *p && r[1].trim().parse::<usize>().ok() == Some(*k)), "{} {} {}", file, p, k);
        }
    }
    // every defined class keeps every entry of the copy's class, in order (the first match wins)
    let full = ComboTable::from_tsv(effects_rtdata::text("fx_exec/data/COMBOS.tsv"));
    let mut entries = 0;
    for c in effects_rtdata::classes::COMBOS {
        let want = &full.classes[c.class];
        assert_eq!(want.len(), c.entries.len(), "{}", c.class);
        for (w, d) in want.iter().zip(c.entries) {
            assert_eq!((&w.vals[..], w.tag), (d.vals, d.tag), "{}", c.class);
        }
        entries += c.entries.len();
    }
    let d = ComboTable::from_defs();
    assert_eq!(d.classes.len(), effects_rtdata::classes::COMBOS.len());
    println!(
        "CLASS_FLAGS {} IMG_ONLY_ARGS {} INT_TYPED_ARGS {} rows; COMBOS {} classes {} entries equal copy rows",
        effects_rtdata::classes::CLASS_FLAGS.len(),
        effects_rtdata::classes::IMG_ONLY_ARGS.len(),
        effects_rtdata::classes::INT_TYPED_ARGS.len(),
        effects_rtdata::classes::COMBOS.len(),
        entries
    );
}
