//! NV-3: plan emission in the text form of the original's compiled plans (EP-3 `PLAN_ORIG_*.txt`),
//! the canonical run list used for the compare, and the compare tool.
//!
//! Emission models the plan at the RUN level: every surviving function node is one `run-primitive`
//! in evaluation order. Buffer allocation, in-place reuse, cpu/gpu staging, `copy-images` and the
//! execute-time argument combos (R9, command stage) are NOT ported: UNKNOWN.
//! The compare therefore normalises both sides to value ids (`R<k>` = the k-th run, `IN:<name>` = an
//! input), so buffer reuse and `constant` images drop out. Constants print with 6 significant digits.
//!
//! The original plans (audit_ep3) were produced by running the original compiler in the emulator on the
//! approved original binary (Ale approved it as a read-only data source). A run-level match per branch set is EVALUATOR-PLAN-PARITY.

use crate::compile::CompiledProgram;
use std::collections::HashMap;

/// 6-significant-digit decimal, shortest form (`%g`-like), for f32 or f64 inputs.
pub fn fmt_g6(x: f64) -> String {
    if x == 0.0 {
        return if x.is_sign_negative() { "-0".into() } else { "0".into() };
    }
    // NV-3f: the original prints a NaN as `nan` (printf %g; Shake_Normal `[ nan nan ]`, MEASURED in the layout dumps); inf as `inf` / `-inf`
    if x.is_nan() {
        return "nan".into();
    }
    if !x.is_finite() {
        return format!("{}", x);
    }
    let s = format!("{:.5e}", x);
    // DS1: C %g rule: exponent < -4 or >= 6 prints in exponent form (MEASURED: the original prints -8.33333e-05 where the old text had -0.0000833333)
    if let Some((m, e)) = s.split_once('e') {
        if let Ok(e) = e.parse::<i32>() {
            if e < -4 || e >= 6 {
                let mut m = m.to_string();
                if m.contains('.') {
                    while m.ends_with('0') {
                        m.pop();
                    }
                    if m.ends_with('.') {
                        m.pop();
                    }
                }
                return format!("{}e{}{:02}", m, if e < 0 { '-' } else { '+' }, e.abs());
            }
        }
    }
    let v: f64 = s.parse().unwrap_or(x);
    format!("{}", v)
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Canon {
    /// `prim<TAB>tok;tok;...` per run, in order
    pub runs: Vec<(String, Vec<String>)>,
    pub dest: String,
}

impl Canon {
    pub fn to_text(&self) -> String {
        let mut s = String::from("CANON\n");
        for (i, (p, t)) in self.runs.iter().enumerate() {
            s.push_str(&format!("RUN\t{}\t{}\t{}\n", i, p, t.join(";")));
        }
        s.push_str(&format!("DEST\t{}\n", self.dest));
        s
    }

    pub fn from_text(t: &str) -> Canon {
        let mut c = Canon::default();
        for l in t.lines() {
            let f: Vec<&str> = l.split('\t').collect();
            if f[0] == "RUN" && f.len() >= 4 {
                c.runs.push((f[2].to_string(), if f[3].is_empty() { vec![] } else { f[3].split(';').map(|s| s.to_string()).collect() }));
            } else if f[0] == "DEST" && f.len() >= 2 {
                c.dest = f[1].to_string();
            }
        }
        c
    }
}

fn norm_const(tok: &str) -> String {
    // "[ 0.2 0.2 ]" -> "[0.2,0.2]"
    let inner = tok.trim_start_matches('[').trim_end_matches(']');
    let v: Vec<String> = inner.split_whitespace().map(|x| x.parse::<f64>().map(fmt_g6).unwrap_or_else(|_| x.to_string())).collect();
    format!("[{}]", v.join(","))
}

fn tokens(s: &str) -> Vec<String> {
    let b: Vec<char> = s.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < b.len() {
        let c = b[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '[' {
            let st = i;
            while i < b.len() && b[i] != ']' {
                i += 1;
            }
            i += 1;
            out.push(b[st..i.min(b.len())].iter().collect());
        } else if c == '(' {
            let st = i;
            let mut d = 0;
            while i < b.len() {
                if b[i] == '(' {
                    d += 1;
                } else if b[i] == ')' {
                    d -= 1;
                    if d == 0 {
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
            out.push(b[st..i.min(b.len())].iter().collect());
        } else if c == ')' {
            break;
        } else {
            let st = i;
            while i < b.len() && !b[i].is_whitespace() && b[i] != ')' {
                i += 1;
            }
            out.push(b[st..i].iter().collect());
        }
    }
    out
}

fn ref_id(r: &str) -> String {
    r.trim_start_matches(":ref").split('.').next().unwrap_or("").to_string()
}

/// Canonical run list of a plan in the EP-3 text form (CMD rows + EVALTEXT).
pub fn canon_orig(text: &str) -> Canon {
    // eval-list names by ref id
    let mut names: HashMap<String, String> = HashMap::new();
    let mut cmds: Vec<String> = Vec::new();
    for l in text.lines() {
        let f: Vec<&str> = l.split('\t').collect();
        if f[0] == "EVALTEXT" && f.len() > 1 {
            for seg in f[1].split("\\n") {
                if let (Some(a), Some(b)) = (seg.find('('), seg.find(')')) {
                    if b > a {
                        let name = seg[..a].trim().to_string();
                        let id = seg[a + 1..b].to_string();
                        if id.chars().all(|c| c.is_ascii_digit()) && !name.is_empty() {
                            names.insert(id, name);
                        }
                    }
                }
            }
        } else if f[0] == "CMD" && f.len() > 3 {
            cmds.push(f[3].to_string());
        }
    }
    let mut val: HashMap<String, String> = HashMap::new();
    let mut c = Canon::default();
    let is_input_name = |n: &str| n.chars().next().map(|ch| ch.is_ascii_uppercase()).unwrap_or(false) || n == "xy";
    let get = |val: &HashMap<String, String>, r: &str| val.get(r).cloned().unwrap_or_else(|| format!("?{}", r));
    for cmd in &cmds {
        let prefixes = ["(convert-from-source ", "(variable ", "(constant ", "(copy-images ", "(convert-to-dest ", "(run-primitive "];
        let mut pos = 0;
        while pos < cmd.len() {
            let mut best: Option<(usize, &str)> = None;
            for p in prefixes.iter() {
                if let Some(i) = cmd[pos..].find(p) {
                    if best.map(|(b, _)| pos + i < b).unwrap_or(true) {
                        best = Some((pos + i, p));
                    }
                }
            }
            let (at, p) = match best {
                Some(x) => x,
                None => break,
            };
            let rest = &cmd[at + p.len()..];
            let tk = tokens(rest);
            pos = at + p.len();
            match p.trim() {
                "(convert-from-source" | "(variable" => {
                    if tk.len() >= 2 {
                        let id = ref_id(&tk[1]);
                        let nm = names.get(&id).cloned().unwrap_or_else(|| format!("src{}", tk[0]));
                        val.insert(tk[1].clone(), format!("IN:{}", nm));
                    }
                }
                "(constant" => {
                    if tk.len() >= 2 {
                        let id = ref_id(&tk[1]);
                        let _ = (&id, &is_input_name);
                        let t = match None::<&String> {
                            Some(n) if is_input_name(n) => format!("IN:{}", n),
                            _ => norm_const(&tk[0]),
                        };
                        val.insert(tk[1].clone(), t);
                    }
                }
                "(copy-images" => {
                    if tk.len() >= 2 {
                        let v = get(&val, &tk[1]);
                        val.insert(tk[0].clone(), v);
                    }
                }
                "(convert-to-dest" => {
                    if let Some(r) = tk.first() {
                        c.dest = get(&val, r);
                    }
                }
                "(run-primitive" => {
                    if tk.len() >= 4 {
                        let out = tk[2].clone();
                        let name = tk[3].clone();
                        let args: Vec<String> = tk[4..]
                            .iter()
                            .map(|t| {
                                if t.starts_with(":ref") {
                                    get(&val, t)
                                } else if t.starts_with('[') {
                                    norm_const(t)
                                } else if t.starts_with("(clip") {
                                    "CLIP".to_string()
                                } else {
                                    t.clone()
                                }
                            })
                            .collect();
                        let mut args = args;
                        if name == "get_frame" && !args.is_empty() {
                            args[0] = "CLIP".to_string();
                        }
                        let k = c.runs.len();
                        c.runs.push((name, args));
                        val.insert(out, format!("R{}", k));
                    }
                }
                _ => {}
            }
        }
    }
    c
}

/// Emit our plan in the EP-3 text form (run level; see the module note).
/// NV-3c: plan text with the full command layout (F13, see `layout.rs`).
pub fn plan_text(cp: &CompiledProgram) -> String {
    let mut s = String::new();
    let mut eval: Vec<String> = Vec::new();
    for n in &cp.nodes {
        match n.kind {
            "special" if n.name == "Output" => {}
            "input" | "special" => eval.push(format!("{}({})", n.name, n.idx)),
            "const" => {}
            _ => eval.push(format!("_{}({})", n.prim, n.idx)),
        }
    }
    let cmds = crate::layout::layout_cmds(cp, &fmt_g6);
    s.push_str(&format!("#NV3\tprogram={}\ttag={}\tcommands={}\tnodes={}\n", cp.program, if cp.paramset.is_empty() { "default" } else { &cp.paramset }, cmds.len(), cp.nodes.len()));
    for (i, (op, c)) in cmds.iter().enumerate() {
        s.push_str(&format!("CMD\t{}\t{}\t{}\t-\n", i, op, c));
    }
    for n in &cp.nodes {
        let lanes: Vec<String> = n.lanes.iter().map(|b| format!("{:08x}", b)).collect();
        s.push_str(&format!("NODE\t{}\t{}\tkind={}\tprim={}\tout_ch={}\targs={}\tlanes={}\tb160={}\tfolded={}\n", n.idx, n.name, n.kind, if n.prim == "-" { "" } else { &n.prim }, n.elnum, n.args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(","), lanes.join(""), n.b160 as u8, n.folded as u8));
    }
    s.push_str(&format!("EVALTEXT\tEval list:\\n{}\n", eval.join("\\n")));
    s
}

/// The NV-3b run-level emission (one buffer per run); kept for reference only.
#[allow(dead_code)]
pub fn plan_text_runlevel(cp: &CompiledProgram) -> String {
    let mut s = String::new();
    let rf = |i: usize| format!(":ref{}.2g", i);
    let mut cmds: Vec<String> = Vec::new();
    let mut eval: Vec<String> = Vec::new();
    let mut slot = 0;
    // last use of each node as an argument (free-images after it)
    let mut last: HashMap<usize, usize> = HashMap::new();
    for n in &cp.nodes {
        for &a in &n.args {
            last.insert(a, n.idx);
        }
    }
    for n in &cp.nodes {
        match n.kind {
            "special" if n.name == "Output" => {}
            "input" | "special" => {
                cmds.push(format!("(allocate-images {} {} 2 gpu)", rf(n.idx), n.elnum.max(1)));
                cmds.push(format!("(convert-from-source {} {})", slot, rf(n.idx)));
                slot += 1;
                eval.push(format!("{}({})", n.name, n.idx));
            }
            "const" => {}
            _ => {
                let mut a: Vec<String> = Vec::new();
                for &x in &n.args {
                    let an = &cp.nodes[x];
                    if an.kind == "special" && an.name == "Output" {
                        a.push("[ -999 ]".to_string());
                    } else if an.kind == "const" {
                        let l: Vec<String> = (0..an.elnum.clamp(1, 4) as usize).map(|i| fmt_g6(f32::from_bits(an.lanes[i]) as f64)).collect();
                        a.push(format!("[ {} ]", l.join(" ")));
                    } else {
                        a.push(rf(x));
                    }
                }
                cmds.push(format!("(allocate-images {} {} 2 gpu)", rf(n.idx), n.elnum.max(1)));
                cmds.push(format!("(run-primitive 1 -1 {} {} {} )", rf(n.idx), n.prim.strip_suffix("_if_needed").unwrap_or(&n.prim), a.join(" ")));
                for &x in &n.args {
                    if cp.nodes[x].kind != "const" && !(cp.nodes[x].kind == "special" && cp.nodes[x].name == "Output") && last.get(&x) == Some(&n.idx) {
                        cmds.push(format!("(free-images {})", rf(x)));
                    }
                }
                eval.push(format!("_{}({})", n.prim, n.idx));
            }
        }
    }
    let root = &cp.nodes[cp.root];
    if root.kind != "const" {
        cmds.push(format!("(convert-to-dest {})", rf(root.idx)));
        cmds.push(format!("(free-images {})", rf(root.idx)));
    }
    s.push_str(&format!("#NV3\tprogram={}\ttag={}\tcommands={}\tnodes={}\n", cp.program, if cp.paramset.is_empty() { "default" } else { &cp.paramset }, cmds.len(), cp.nodes.len()));
    for (i, c) in cmds.iter().enumerate() {
        let op = c.trim_start_matches('(').split_whitespace().next().unwrap_or("");
        s.push_str(&format!("CMD\t{}\t{}\t{}\t-\n", i, op, c));
    }
    for n in &cp.nodes {
        let lanes: Vec<String> = n.lanes.iter().map(|b| format!("{:08x}", b)).collect();
        s.push_str(&format!("NODE\t{}\t{}\tkind={}\tprim={}\tout_ch={}\targs={}\tlanes={}\tb160={}\tfolded={}\n", n.idx, n.name, n.kind, if n.prim == "-" { "" } else { &n.prim }, n.elnum, n.args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(","), lanes.join(""), n.b160 as u8, n.folded as u8));
    }
    s.push_str(&format!("EVALTEXT\tEval list:\\n{}\n", eval.join("\\n")));
    s
}

pub fn canon_mine(cp: &CompiledProgram) -> Canon {
    canon_orig(&plan_text(cp))
}

/// Parse any plan file: EP-3 text form (CMD rows) or CANON text.
pub fn canon_any(text: &str) -> Canon {
    if text.starts_with("CANON") {
        Canon::from_text(text)
    } else {
        canon_orig(text)
    }
}

/// Differences of `mine` against `orig`; empty = equal. `CLIP` tokens in the original match anything.
pub fn diff(mine: &Canon, orig: &Canon) -> Vec<String> {
    let mut d = Vec::new();
    if mine.runs.len() != orig.runs.len() {
        d.push(format!("run count: mine {} vs original {}", mine.runs.len(), orig.runs.len()));
    }
    for (i, (m, o)) in mine.runs.iter().zip(orig.runs.iter()).enumerate() {
        if m.0 != o.0 {
            d.push(format!("run {}: prim mine `{}` vs original `{}`", i, m.0, o.0));
            continue;
        }
        if m.1.len() != o.1.len() {
            d.push(format!("run {} ({}): arity mine {} vs original {} ({:?} vs {:?})", i, m.0, m.1.len(), o.1.len(), m.1, o.1));
            continue;
        }
        for (j, (a, b)) in m.1.iter().zip(o.1.iter()).enumerate() {
            if a != b && b != "CLIP" && a != "CLIP" {
                d.push(format!("run {} ({}) arg {}: mine `{}` vs original `{}`", i, m.0, j, a, b));
            }
        }
    }
    if mine.dest != orig.dest {
        d.push(format!("dest: mine `{}` vs original `{}`", mine.dest, orig.dest));
    }
    d
}

pub fn canon_mine_mode(cp: &CompiledProgram, mode: crate::cpuplan::PlanMode) -> Canon {
    canon_orig(&crate::cpuplan::plan_text_mode(cp, mode))
}
