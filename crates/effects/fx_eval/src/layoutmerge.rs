//! LYM1: the registered program as the original's parser sees it (LAY1, W225 (b)): the layout_effect row ALONE is handed to the parser
//!  (stores rec[0] verbatim when the extra string rec[0] is empty); the row names the def_effect and carries
//! `:extra` (new parameters appended to the program's parameter array: PDV3's builder appends every record the parser
//! builds, unconditionally), `:modify` (flags and renames of def parameters) and `:hardcode`.
//!
//! This module implements the `:extra` step only (a ParamRec without a node: the body never references it, the owner arm's loop and the
//! host definition list do). `:modify` flag changes that affect values (min, max, popup, normalize_scale) are carried by the REAL range
//! table and NS_PARAMS.tsv already; `:modify` renames map layout names to def names (the def name is the record name used by the native).
//! `:hardcode` is parsed and reported but NOT applied (see LayoutInfo::hardcodes; HIGH-STAKES, LYM1 README).

use crate::build::{Genotype, NodeKind, ParamRec};
use crate::parse::{flag_info, DefaultVal};

#[derive(Debug, Clone, PartialEq)]
pub enum Sx {
    Atom(String),
    List(Vec<Sx>),
}

fn tokenize(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for ch in s.chars() {
        match ch {
            '(' | ')' => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
                out.push(ch.to_string());
            }
            c if c.is_whitespace() => {
                if !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
            }
            c => cur.push(c),
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn parse_sx(t: &[String], i: &mut usize) -> Result<Sx, String> {
    if *i >= t.len() {
        return Err("unexpected end of layout text".into());
    }
    if t[*i] == "(" {
        *i += 1;
        let mut v = Vec::new();
        loop {
            if *i >= t.len() {
                return Err("unbalanced parentheses in layout text".into());
            }
            if t[*i] == ")" {
                *i += 1;
                return Ok(Sx::List(v));
            }
            v.push(parse_sx(t, i)?);
        }
    } else if t[*i] == ")" {
        Err("stray ')' in layout text".into())
    } else {
        *i += 1;
        Ok(Sx::Atom(t[*i - 1].clone()))
    }
}

pub fn parse_layout(text: &str) -> Result<Vec<Sx>, String> {
    let t = tokenize(text);
    let mut i = 0;
    let top = parse_sx(&t, &mut i)?;
    match top {
        Sx::List(v) => Ok(v),
        _ => Err("layout text is not a list".into()),
    }
}

/// One `:extra name = value ( flags )` entry.
#[derive(Debug, Clone, PartialEq)]
pub struct Extra {
    pub name: String,
    pub default: DefaultVal,
    pub flags: Vec<String>,
}

/// One item of the layout row's parameter list, in layout order (LYM2: genotype-parser.lua create_params iterates `layout.params` in this order).
#[derive(Debug, Clone, PartialEq)]
pub enum LItem {
    /// a bare name: the def parameter of that name
    Bare(String),
    /// `( :modify NAME [CANON] [= VALUE] ( flags ) )`: canonical name = CANON or NAME (the def parameter); `value` true when the item carries `= VALUE`
    /// INT24 (W518 (c)): `text` the raw item text; `value_text` the VALUE of a `NAME = VALUE` item (None when the reader cannot separate them)
    Modify { name: String, canon: String, value: bool, flags: Vec<String>, text: String, value_text: Option<String> },
    Extra(Extra),
    Hardcode { name: String, value: f64 },
    /// INT21 step L (W473 (b)): `:hardcode NAME = [ ... ]`: the lanes as the genotype reader gives a def vector literal of the same text
    HardcodeVec { name: String, lanes: Vec<f64>, text: String },
}

/// INT21 step L (W473 (b)): the label of the three registered rows that carry a vector `:hardcode` (printed with the LYM2_REPORT line).
pub const LAYOUT_HARDCODE_VECTOR_LABEL: &str = "LAYOUT-HARDCODE-VECTOR (Mosaic_Autogen wrap: the value node takes the hardcode's lanes and type; Texture_Chroma_Spiral_Autogen shift (def 1 lanes, hardcode 2; behind LAYOUT-MODIFY-VALUE-UNSUPPORTED on the same row) and Dissolve_Zap bg_rel_color (def 1 lanes, hardcode 3) are refused LAYOUT-HARDCODE-LANES-MISMATCH, MEASURED INT21 t34; these programs' merged plans are not compared with an original; no claimed style carries a vector hardcode, MEASURED LYM2 TEXTS.tsv; the rule INFERRED from the layout item hook and set_hardcoded_arg)";

/// One layout input: `Name` or `( Name Canonical [required] )`.
#[derive(Debug, Clone, PartialEq)]
pub struct LInput {
    pub name: String,
    pub canon: String,
    pub required: bool,
}

#[derive(Debug, Clone, Default)]
pub struct LayoutInfo {
    /// LYM2: the parameter items in layout order
    pub items: Vec<LItem>,
    /// LYM2: the layout row's input group
    pub inputs: Vec<LInput>,
    /// LYM2: the layout row's own effect flags (concatenated after the def's flags by build_genotype_helper pc 110..121)
    pub layout_flags: Vec<String>,
    /// the header names: (style label, def name) or (def name)
    pub header: Vec<String>,
    pub extras: Vec<Extra>,
    /// `:modify` entries as printed (names and flag groups), for the report
    pub modifies: Vec<String>,
    /// `:hardcode name = value` entries (parsed, not applied)
    pub hardcodes: Vec<(String, String)>,
}

fn flat(x: &Sx) -> String {
    match x {
        Sx::Atom(a) => a.clone(),
        Sx::List(v) => format!("( {} )", v.iter().map(flat).collect::<Vec<_>>().join(" ")),
    }
}

/// The layout row's registered def name: `(layout_effect ( Style Def ) ...)` or `(layout_effect Def ...)`.
pub fn header_names(items: &[Sx]) -> Vec<String> {
    match items.get(1) {
        Some(Sx::Atom(a)) => vec![a.clone()],
        Some(Sx::List(v)) => v.iter().filter_map(|x| if let Sx::Atom(a) = x { Some(a.clone()) } else { None }).collect(),
        None => Vec::new(),
    }
}

pub fn read_layout(text: &str) -> Result<LayoutInfo, String> {
    read_layout_sx(parse_layout(text)?)
}

/// The layout reader on an already built token tree (the items of the row's outer list).
pub fn read_layout_sx(items: Vec<Sx>) -> Result<LayoutInfo, String> {
    if items.first() != Some(&Sx::Atom("layout_effect".into())) {
        return Err("not a layout_effect row".into());
    }
    let mut info = LayoutInfo { header: header_names(&items), ..Default::default() };
    if let Some(Sx::List(ins)) = items.get(2) {
        for x in ins {
            match x {
                Sx::Atom(a) => info.inputs.push(LInput { name: a.clone(), canon: a.clone(), required: false }),
                Sx::List(v) => {
                    let at: Vec<&str> = v.iter().filter_map(|y| if let Sx::Atom(a) = y { Some(a.as_str()) } else { None }).collect();
                    let required = at.last() == Some(&"required");
                    let at2: Vec<&str> = at.iter().copied().filter(|a| *a != "required").collect();
                    // proto 58: `( name flag ... )`: with one flag text it is the canonical name (proto 58 pc 57..60)
                    let name = at2.first().copied().unwrap_or("").to_string();
                    let canon = if at2.len() == 1 { name.clone() } else { at2.get(1).copied().unwrap_or("").to_string() };
                    info.inputs.push(LInput { name, canon, required });
                }
            }
        }
    }
    if let Some(Sx::List(fl)) = items.get(3) {
        info.layout_flags = fl.iter().filter_map(|y| if let Sx::Atom(a) = y { Some(a.clone()) } else { None }).collect();
    }
    // (layout_effect HEADER INPUTS FLAGS ( ITEM ... ))
    let body = match items.get(4) {
        Some(Sx::List(v)) => v,
        _ => return Err("layout row has no item list".into()),
    };
    for it in body {
        let l = match it {
            Sx::List(l) => l,
            Sx::Atom(a) => {
                info.items.push(LItem::Bare(a.clone()));
                continue;
            }
        };
        match l.first() {
            Some(Sx::Atom(h)) if h == ":extra" => {
                let name = match l.get(1) {
                    Some(Sx::Atom(a)) => a.clone(),
                    _ => return Err(format!("bad :extra entry {}", flat(it))),
                };
                if l.get(2) != Some(&Sx::Atom("=".into())) {
                    return Err(format!(":extra {} has no '='", name));
                }
                let (default, next) = match l.get(3) {
                    Some(Sx::Atom(a)) if a == "[" => {
                        // vector written with bracket atoms (the layout rows seen have none; kept for completeness)
                        return Err(format!(":extra {}: bracket vector default not supported", name));
                    }
                    Some(Sx::Atom(a)) => (DefaultVal::Scalar(a.parse::<f64>().map_err(|_| format!(":extra {}: default {:?} is not a number", name, a))?), 4),
                    _ => return Err(format!(":extra {}: no default", name)),
                };
                let flags: Vec<String> = match l.get(next) {
                    Some(Sx::List(v)) => v.iter().map(flat).collect(),
                    None => Vec::new(),
                    _ => return Err(format!(":extra {}: bad flag group", name)),
                };
                info.items.push(LItem::Extra(Extra { name: name.clone(), default: default.clone(), flags: flags.clone() }));
                info.extras.push(Extra { name, default, flags });
            }
            Some(Sx::Atom(h)) if h == ":modify" => {
                let text = flat(it);
                info.modifies.push(text.clone());
                let atoms: Vec<&str> = l[1..].iter().filter_map(|y| if let Sx::Atom(a) = y { Some(a.as_str()) } else { None }).collect();
                let value = atoms.contains(&"=");
                let names: Vec<&str> = atoms.iter().copied().filter(|a| *a != "=").take(2).collect();
                let flags: Vec<String> = l[1..].iter().find_map(|y| if let Sx::List(v) = y { Some(v.iter().map(flat).collect()) } else { None }).unwrap_or_default();
                let name = names.first().copied().unwrap_or("").to_string();
                let canon = names.get(1).copied().unwrap_or(&name).to_string();
                let value_text = if atoms.len() == 3 && atoms[1] == "=" { Some(atoms[2].to_string()) } else { None };
                info.items.push(LItem::Modify { name, canon, value, flags, text, value_text });
            }
            Some(Sx::Atom(h)) if h == ":hardcode" => {
                if let (Some(Sx::Atom(n)), Some(Sx::Atom(v))) = (l.get(1), l.get(3)) {
                    if let Ok(value) = v.parse::<f64>() {
                        // the scalar path, unchanged from LYM2
                        info.hardcodes.push((n.clone(), v.clone()));
                        info.items.push(LItem::Hardcode { name: n.clone(), value });
                    } else {
                        // INT21 step L (W473 (b)): a vector literal VALUE goes through the genotype reader's own vector-literal path
                        let text = l[3..].iter().map(flat).collect::<Vec<_>>().join(" ");
                        let prog = info.header.last().cloned().unwrap_or_default();
                        let lanes = if text.starts_with('[') { crate::parse::read_vector_literal(&text).ok() } else { None };
                        match lanes {
                            Some(lanes) => {
                                info.hardcodes.push((n.clone(), text.clone()));
                                info.items.push(LItem::HardcodeVec { name: n.clone(), lanes, text });
                            }
                            None => return Err(format!("LAYOUT-HARDCODE-UNPARSED ({}: {}: {})", prog, n, text)),
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(info)
}

/// Find the layout row registered for `program` (the def name in the row's header). Rows are (kind, name, va, text) of programs.tsv.
pub fn find_layout_text<'a>(rows: impl Iterator<Item = &'a str>, program: &str) -> Vec<&'a str> {
    rows.filter(|t| {
        t.starts_with("(layout_effect") && match read_layout_header(t) {
            Some(h) => h.last().map(|x| x == program).unwrap_or(false),
            None => false,
        }
    })
    .collect()
}

fn read_layout_header(text: &str) -> Option<Vec<String>> {
    parse_layout(text).ok().map(|i| header_names(&i))
}

/// Append the layout row's `:extra` parameters to the genotype (after the def parameters, in layout order). Returns the names added.
/// An extra that already is a def parameter is an error (the parser would build a second record of that name; not seen in the 13 rows).
pub fn apply_extras(g: &mut Genotype, info: &LayoutInfo) -> Result<Vec<String>, String> {
    let mut added = Vec::new();
    for e in &info.extras {
        if g.params.iter().any(|p| p.name == e.name) {
            return Err(format!("LAYOUT-EXTRA-DUPLICATE: {} already is a parameter of {}", e.name, g.name));
        }
        g.params.push(ParamRec { name: e.name.clone(), node: None, default: e.default.clone(), flags: e.flags.clone(), info: flag_info(&e.flags), binding: "LAYOUT-EXTRA", layout_name: e.name.clone() });
        added.push(e.name.clone());
    }
    Ok(added)
}


/// LYM2 report of one merge.
#[derive(Debug, Clone, Default)]
pub struct MergeReport {
    /// parameter names in the merged array order ([prog], create_params order)
    pub order: Vec<String>,
    pub extras: Vec<String>,
    /// (name, value) of each `:hardcode`: the parameter record is not built; the def's value node keeps the hardcoded constant
    pub hardcoded: Vec<(String, f32)>,
    /// INT21 step L: (name, lanes) of each vector `:hardcode` (LAYOUT-HARDCODE-VECTOR)
    pub hardcoded_vec: Vec<(String, Vec<f32>)>,
    /// `:modify` items that carry `= VALUE` (the layout value would replace the def default: not implemented, refused)
    pub modify_values: usize,
    pub modify_count: usize,
    /// the layout row's own flags appended to the genotype flags (share_all for Blur_Color, Blur_Mono and Distort_RGB; cache_alt_frames for Time_Warp_RGB)
    pub layout_flags: Vec<String>,
    /// LYM3: the input group after the merge: (position 1-based, canonical name, layout name, default)
    pub inputs: Vec<(usize, String, String, Option<String>)>,
    /// LYM3: def inputs absent from the layout row, now the parser's default nodes: (canonical name, def position 1-based, constant)
    pub default_nodes: Vec<(String, usize, f32)>,
    /// LYM3: the input group differs from the def's (count, order or names of positions)
    pub inputs_changed: bool,
}

/// LYM2: the registered program as genotype-parser.lua builds it from the layout row (read in interp_recovery s4_lua.out; protos 61, 131, 163, 190, 194 and
/// the C callback set_hardcoded_arg):
/// * create_params (proto 163) iterates `layout.params` IN LAYOUT ORDER; for each item with should_make_param it calls create_param (record appended at
///   ), for a `:hardcode` item it calls set_hardcoded_arg and builds NO record;
/// * create_variable_mappings (proto 131): a layout item that carries a value replaces the variable's parse node (`:hardcode name = 0` makes `name` the
///   constant 0; `:modify` rows in the 13 texts carry no value);
/// * check_layout_consistency (proto 165): a def parameter missing from the layout is a parse error.
/// So the merged array = the layout items in order: def parameters (by canonical name), `:extra` parameters at their layout position, minus the hardcoded ones.
/// INT24 (W518 (c)): the LAYOUT-MODIFY-VALUE-UNSUPPORTED refusal text; `what` is ":modify NAME = VALUE" or the raw item text.
pub fn modify_value_refusal(program: &str, what: &str) -> String {
    format!("LAYOUT-MODIFY-VALUE-UNSUPPORTED ({}: {} in the layout row; the original's handling of a :modify value is not ported; no claimed program carries one, MEASURED IFNEQ_AFTER_Q.tsv against STYLE_MAP.tsv; W518)", program, what)
}

pub fn merge_layout(g: &mut Genotype, info: &LayoutInfo) -> Result<MergeReport, String> {
    let mut rep = MergeReport::default();
    let mut def: Vec<Option<ParamRec>> = std::mem::take(&mut g.params).into_iter().map(Some).collect();
    let mut out: Vec<ParamRec> = Vec::new();
    let take = |def: &mut Vec<Option<ParamRec>>, n: &str| -> Option<ParamRec> {
        let i = def.iter().position(|p| p.as_ref().map(|q| q.name == n).unwrap_or(false))?;
        def[i].take()
    };
    for it in &info.items {
        match it {
            LItem::Bare(n) => match take(&mut def, n) {
                Some(p) => out.push(p),
                None if g.word_defaults.iter().any(|w| w.starts_with(&format!("{}=", n))) => {}
                None => return Err(format!("LAYOUT-PARAM-UNKNOWN: {} is not a parameter of {}", n, g.name)),
            },
            LItem::Modify { name, canon, value, text, value_text, .. } => {
                rep.modify_count += 1;
                if *value {
                    rep.modify_values += 1;
                    // INT24 (W518 (c)): the refusal prints the name and the value (LYM2's reader took the value text as the name), or the raw row text
                    let what = match value_text {
                        Some(v) => format!(":modify {} = {}", name, v),
                        None => text.clone(),
                    };
                    return Err(modify_value_refusal(&g.name, &what));
                }
                match take(&mut def, canon) {
                    Some(mut p) => {
                        // LYM3 (step L2, INT28 hand-applied: LYM3.diff hunk 3 of layoutmerge.rs did not apply over INT24's refusal text): the layout name
                        p.layout_name = name.clone();
                        out.push(p)
                    }
                    None if g.word_defaults.iter().any(|w| w.starts_with(&format!("{}=", canon))) => {}
                    None => return Err(format!("LAYOUT-PARAM-UNKNOWN: {} is not a parameter of {}", canon, g.name)),
                }
            }
            LItem::Extra(e) => {
                if out.iter().any(|p| p.name == e.name) || def.iter().flatten().any(|p| p.name == e.name) {
                    return Err(format!("LAYOUT-EXTRA-DUPLICATE: {} already is a parameter of {}", e.name, g.name));
                }
                out.push(ParamRec { name: e.name.clone(), node: None, default: e.default.clone(), flags: e.flags.clone(), info: flag_info(&e.flags), binding: "LAYOUT-EXTRA", layout_name: e.name.clone() });
                rep.extras.push(e.name.clone());
            }
            LItem::Hardcode { name, value } => {
                let p = take(&mut def, name).ok_or_else(|| format!("LAYOUT-HARDCODE-UNKNOWN: {} is not a parameter of {}", name, g.name))?;
                let ni = p.node.ok_or_else(|| format!("LAYOUT-HARDCODE-NO-NODE: {}", name))?;
                let nd = g.nodes.get_mut(ni).ok_or_else(|| format!("LAYOUT-HARDCODE-NODE-RANGE: {}", name))?;
                if nd.kind != NodeKind::Scalar {
                    return Err(format!("LAYOUT-HARDCODE-NOT-SCALAR: {}", name));
                }
                nd.lanes = [*value as f32, 0.0, 0.0, 0.0];
                nd.nlanes = 1;
                nd.text = format!("{}", value);
                rep.hardcoded.push((name.clone(), *value as f32));
            }
            LItem::HardcodeVec { name, lanes, text } => {
                // INT21 step L (W473 (b)): the target value node takes all the hardcode's lanes and its type (the layout item hook copies the VALUE
                // node's value and value_type; set_hardcoded_arg copies the byte [node field] cells, a count-driven copy)
                let p = take(&mut def, name).ok_or_else(|| format!("LAYOUT-HARDCODE-UNKNOWN: {} is not a parameter of {}", name, g.name))?;
                let ni = p.node.ok_or_else(|| format!("LAYOUT-HARDCODE-NO-NODE: {}", name))?;
                let nd = g.nodes.get_mut(ni).ok_or_else(|| format!("LAYOUT-HARDCODE-NODE-RANGE: {}", name))?;
                let def_lanes = match nd.kind {
                    NodeKind::Scalar | NodeKind::Vector => nd.nlanes,
                    _ => return Err(format!("LAYOUT-HARDCODE-UNPARSED ({}: {}: {})", g.name, name, text)),
                };
                if def_lanes != lanes.len() {
                    return Err(format!("LAYOUT-HARDCODE-LANES-MISMATCH ({}: {}: def {} lanes, hardcode {})", g.name, name, def_lanes, lanes.len()));
                }
                nd.kind = NodeKind::Vector;
                nd.lanes = [0.0; 4];
                for (i, x) in lanes.iter().take(4).enumerate() {
                    nd.lanes[i] = *x as f32;
                }
                nd.nlanes = lanes.len().min(4);
                nd.text = text.clone();
                rep.hardcoded_vec.push((name.clone(), nd.lanes[..nd.nlanes].to_vec()));
            }
        }
    }
    merge_inputs(g, info, &mut rep)?;
    let left: Vec<String> = def.iter().flatten().map(|p| p.name.clone()).collect();
    if !left.is_empty() {
        return Err(format!("LAYOUT-PARAM-MISSING: {} parameter(s) of {} are missing from the layout row: {:?} (check_layout_consistency would stop the plugin)", left.len(), g.name, left));
    }
    rep.order = out.iter().map(|p| p.name.clone()).collect();
    g.params = out;
    // build_genotype_helper (proto 194 pc 110..121): the genotype's flags are the def's flags followed by the layout row's own flags
    for f in &info.layout_flags {
        g.flags.push(f.clone());
    }
    rep.layout_flags = info.layout_flags.clone();
    Ok(rep)
}

/// LYM3: the layout row's input group (build_genotype_helper proto 194, get_effect_input proto 190, build_input_nodes proto 159, get_node_for_input_default
/// proto 158, read in interp_recovery/s4_lua.out by LYM2): the input count, order and names come from the layout row; each layout input takes its canonical
/// name's default from the DEF input of that name; a def input absent from the layout becomes the parser's default node (a numeric default is a scalar
/// constant node). Natively: the layout inputs are renumbered 1.. in layout order (the node keeps its index, its `input_pos` changes), a def input absent
/// from the layout becomes a Scalar node with the default, and `def_input_order` records the def order for tables measured in it.
/// Refused by name: LAYOUT-INPUT-UNKNOWN (no def input of the canonical name), LAYOUT-INPUT-DUPLICATE, LAYOUT-INPUT-DEFAULT-UNSUPPORTED (an absent def input whose
/// default is not a number: the alias case, not met by any of the 13 rows).
pub fn merge_inputs(g: &mut Genotype, info: &LayoutInfo, rep: &mut MergeReport) -> Result<(), String> {
    if info.inputs.is_empty() {
        return Ok(());
    }
    let def_inputs = std::mem::take(&mut g.inputs);
    let def_names: Vec<String> = def_inputs.iter().map(|i| i.name.clone()).collect();
    let mut taken = vec![false; def_inputs.len()];
    let mut out: Vec<crate::build::InputRec> = Vec::new();
    for (k, li) in info.inputs.iter().enumerate() {
        let j = def_inputs
            .iter()
            .position(|d| d.name == li.canon)
            .ok_or_else(|| format!("LAYOUT-INPUT-UNKNOWN: layout input {} (canonical {}) is not an input of {}", li.name, li.canon, g.name))?;
        if taken[j] {
            return Err(format!("LAYOUT-INPUT-DUPLICATE: canonical input {} named twice in the layout row of {}", li.canon, g.name));
        }
        taken[j] = true;
        let mut r = def_inputs[j].clone();
        r.pos = k + 1;
        r.layout_name = li.name.clone();
        if let Some(n) = g.nodes.get_mut(r.node) {
            n.input_pos = k + 1;
        }
        out.push(r);
    }
    for (j, d) in def_inputs.iter().enumerate() {
        if taken[j] {
            continue;
        }
        let v = d
            .default
            .as_deref()
            .and_then(crate::parse::orig_number)
            .ok_or_else(|| format!("LAYOUT-INPUT-DEFAULT-UNSUPPORTED: def input {} of {} is absent from the layout row and its default {:?} is not a number", d.name, g.name, d.default))?;
        let nd = g.nodes.get_mut(d.node).ok_or_else(|| format!("LAYOUT-INPUT-NODE-RANGE: {}", d.name))?;
        nd.kind = NodeKind::Scalar;
        nd.lanes = [v as f32, 0.0, 0.0, 0.0];
        nd.nlanes = 1;
        nd.text = format!("{}", v);
        nd.input_pos = 0;
        nd.name = String::new();
        rep.default_nodes.push((d.name.clone(), j + 1, v as f32));
    }
    let same = out.len() == def_names.len() && out.iter().zip(def_names.iter()).all(|(a, b)| &a.name == b);
    rep.inputs_changed = !same;
    rep.inputs = out.iter().map(|i| (i.pos, i.name.clone(), i.layout_name.clone(), i.default.clone())).collect();
    if !same {
        g.def_input_order = def_names;
    }
    g.inputs = out;
    Ok(())
}

/// LYM3: both names of every record and input of a merged genotype, one TSV line each (`PARAM pos canonical layout` / `INPUT pos canonical layout default`).
pub fn names_tsv(g: &Genotype) -> String {
    let mut s = String::new();
    for (k, p) in g.params.iter().enumerate() {
        s.push_str(&format!("PARAM\t{}\t{}\t{}\t{}\n", g.name, k, p.name, p.layout_name));
    }
    for i in &g.inputs {
        s.push_str(&format!("INPUT\t{}\t{}\t{}\t{}\t{}\n", g.name, i.pos, i.name, i.layout_name, i.default.clone().unwrap_or_else(|| "-".into())));
    }
    s
}

/// LYM3: the layout row registered for `prog` in a corpus (None for a core program or a program without a row).
pub fn resolve_layout(c: &crate::Corpus, prog: &str) -> Result<Option<LayoutInfo>, String> {
    if !c.layout_sx.is_empty() {
        let rows: Vec<&Vec<Sx>> = c.layout_sx.iter().filter(|r| header_names(r).last().map(|x| x == prog).unwrap_or(false)).collect();
        return match rows.len() {
            0 => Ok(None),
            1 => read_layout_sx(rows[0].clone()).map(Some).map_err(|e| format!("LAYOUT-ROW-UNREADABLE {}: {}", prog, e)),
            n => Err(format!("LAYOUT-ROW-AMBIGUOUS: {} layout rows are registered for {}", n, prog)),
        };
    }
    let rows = find_layout_text(c.layout_texts.iter().map(|x| x.1.as_str()), prog);
    match rows.len() {
        0 => Ok(None),
        1 => read_layout(rows[0]).map(Some).map_err(|e| format!("LAYOUT-ROW-UNREADABLE {}: {}", prog, e)),
        n => Err(format!("LAYOUT-ROW-AMBIGUOUS: {} layout rows are registered for {}", n, prog)),
    }
}

/// LYM3: build the registered program of `prog` (layout row merged when one is registered), as fx_run does. Ok((genotype, report)); report None on the def path.
pub fn build_registered(c: &crate::Corpus, ei: usize, prog: &str) -> Result<(Genotype, Option<MergeReport>), String> {
    let mut g = crate::build_effect(c, ei).map_err(|e| format!("{}: {}", prog, e))?;
    match resolve_layout(c, prog)? {
        Some(li) => {
            let rep = merge_layout(&mut g, &li)?;
            Ok((g, Some(rep)))
        }
        None => Ok((g, None)),
    }
}
