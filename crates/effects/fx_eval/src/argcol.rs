//! Per-argument bits column of compare, report-only. No dependencies.
//! Port of pgm1_argcol_groups/argcol2.py. For a plan text (or NV3D format) it lists, for every run-primitive in command
//! order, the primitive name and each argument as constant lane bits (little-endian bytes) or ":REF" (image references belong
//! to the layout half). A pair is PASS, FAIL (names run-primitive index, primitive and argument index) or UNRESOLVED
//! (any <MISSING>, <EXTRA> or <NONODE> marker on either side).
use std::collections::BTreeMap;

pub struct Node {
    pub name: String,
    pub f: BTreeMap<String, String>,
}

pub type Column = Vec<(String, Vec<String>)>;

fn parse(text: &str) -> (BTreeMap<i64, Node>, Vec<String>, bool) {
    let be = text.starts_with("#NV3D");
    let mut nodes = BTreeMap::new();
    let mut cmds = Vec::new();
    for l in text.lines() {
        let f: Vec<&str> = l.split('\t').collect();
        if f[0] == "NODE" && f.len() >= 3 {
            if let Ok(id) = f[1].parse::<i64>() {
                let mut m = BTreeMap::new();
                for x in &f[3.min(f.len())..] {
                    if let Some((k, v)) = x.split_once('=') {
                        m.insert(k.to_string(), v.to_string());
                    }
                }
                nodes.insert(id, Node { name: f[2].to_string(), f: m });
            }
        } else if f[0] == "CMD" && f.len() >= 3 {
            cmds.push(f.get(3).copied().unwrap_or("").replace("\\n", "\n"));
        }
    }
    (nodes, cmds, be)
}

fn args_of(n: &Node) -> Vec<i64> {
    n.f.get("args").map(|s| s.split(',').filter(|x| !x.is_empty()).filter_map(|x| x.parse().ok()).collect()).unwrap_or_default()
}

fn prim_of(n: &Node) -> String {
    match n.f.get("prim") {
        Some(p) if !p.is_empty() => p.clone(),
        _ => n.name.clone(),
    }
}

/// lane words of a constant node as little-endian byte hex strings
fn words(n: &Node, be: bool) -> Option<Vec<String>> {
    let k: usize = n.f.get("out_ch")?.parse().ok()?;
    let lanes = n.f.get("lanes")?;
    let mut v = Vec::new();
    for i in 0..k {
        let w = lanes.get(8 * i..8 * i + 8)?;
        if be {
            let b: Vec<&str> = (0..4).map(|j| &w[2 * j..2 * j + 2]).collect();
            v.push(b.iter().rev().cloned().collect::<String>());
        } else {
            v.push(w.to_string());
        }
    }
    Some(v)
}

#[derive(Clone, Debug)]
enum G {
    Ref(i64),
    Br(String),
    Clip(String),
}

fn groups(rest: &str) -> Vec<G> {
    let b = rest.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if rest[i..].starts_with(":ref") {
            let mut j = i + 4;
            while j < b.len() && b[j].is_ascii_digit() {
                j += 1;
            }
            let id: i64 = rest[i + 4..j].parse().unwrap_or(-1);
            if j < b.len() && b[j] == b'.' {
                let mut k = j + 1;
                while k < b.len() && b[k].is_ascii_digit() {
                    k += 1;
                }
                if k > j + 1 {
                    out.push(G::Ref(id));
                    i = k;
                    continue;
                }
            }
            i += 1;
        } else if b[i] == b'[' {
            if let Some(e) = rest[i..].find(']') {
                out.push(G::Br(rest[i + 1..i + e].to_string()));
                i += e + 1;
                continue;
            }
            i += 1;
        } else if rest[i..].starts_with("(clip ") {
            if let Some(e) = rest[i..].find(')') {
                out.push(G::Clip(rest[i + 1..i + e].replace(' ', "_")));
                i += e + 1;
                continue;
            }
            i += 1;
        } else {
            i += 1;
        }
    }
    out
}

/// number of first-argument steps from x to r (0 = the argument itself), up to 4; None = not reached
fn alias_steps(nodes: &BTreeMap<i64, Node>, mut x: i64, r: i64) -> Option<usize> {
    for s in 0..4 {
        if x == r {
            return Some(s);
        }
        let n = nodes.get(&x)?;
        let a = args_of(n);
        if a.is_empty() {
            return None;
        }
        x = a[0];
    }
    if x == r {
        Some(4)
    } else {
        None
    }
}

/// the print precision of a printed decimal. The plan printer gives at most 6 significant digits (`%g` style, trailing
/// zeros trimmed, INFERRED from the printed texts such as 0.333333), so the precision is half a unit of the 6th significant digit; a printed 0 is exact.
fn print_tol(p: f32) -> f32 {
    if p == 0.0 || !p.is_finite() {
        return 0.0;
    }
    0.5 * crate::detmath::pow10_f32(crate::detmath::floor_log10(p) - 5)
}

/// the chosen node's constant arguments agree with the printed text within print precision (MAPPING-TEXT-MISMATCH guard)
fn text_consistent(nodes: &BTreeMap<i64, Node>, n: i64, gs: &[G], be: bool) -> bool {
    for (x, g) in args_of(&nodes[&n]).iter().zip(gs) {
        let G::Br(t) = g else { continue };
        let Some(w) = nodes.get(x).and_then(|nn| words(nn, be)) else { return false };
        let toks: Vec<&str> = t.split_whitespace().collect();
        // the printer's sentinel -999 for an argument that is not a constant node (an input image node, e.g. coerce_up's Source); INFERRED from the texts
        if toks.iter().all(|x| *x == "-999") {
            continue;
        }
        if toks.len() != w.len() {
            return false;
        }
        for (hx, tk) in w.iter().zip(toks) {
            let Ok(p) = tk.parse::<f32>() else { return false };
            let bits = u32::from_le_bytes([0, 2, 4, 6].map(|o| u8::from_str_radix(&hx[o..o + 2], 16).unwrap_or(0)));
            let a = f32::from_bits(bits);
            if a.is_nan() && p.is_nan() {
                continue;
            }
            // an integer-typed argument prints the truncated lane (cvttss2si set_arg_const)
            let int_tok = !tk.contains('.') && !tk.contains('e') && !tk.contains('E');
            if int_tok && a.is_finite() && a.trunc() == p {
                continue;
            }
            if !(a == p || (a - p).abs() <= print_tol(p).max(1e-6 * 1f32.max(p.abs()))) {
                return false;
            }
        }
    }
    true
}

fn aliases(nodes: &BTreeMap<i64, Node>, mut x: i64, r: i64) -> bool {
    for _ in 0..4 {
        if x == r {
            return true;
        }
        let Some(n) = nodes.get(&x) else { return false };
        let a = args_of(n);
        if a.is_empty() {
            return false;
        }
        x = a[0];
    }
    x == r
}

fn vals_match(nodes: &BTreeMap<i64, Node>, n: i64, gs: &[G], be: bool) -> bool {
    for (x, g) in args_of(&nodes[&n]).iter().zip(gs) {
        let G::Br(t) = g else { continue };
        let Some(w) = nodes.get(x).and_then(|nn| words(nn, be)) else { return false };
        let pv: Vec<f32> = match t.split_whitespace().map(|s| s.parse::<f32>()).collect() {
            Ok(v) => v,
            Err(_) => return false,
        };
        if pv.len() != w.len() {
            return false;
        }
        for (hx, p) in w.iter().zip(pv) {
            let bits = u32::from_le_bytes([0, 2, 4, 6].map(|o| u8::from_str_radix(&hx[o..o + 2], 16).unwrap_or(0)));
            let a = f32::from_bits(bits);
            let ok = a == p || (a - p).abs() <= 1e-6 * 1f32.max(p.abs()) || (a.is_nan() && p.is_nan());
            if !ok || (a == 0.0 && p == 0.0 && a.is_sign_negative() != p.is_sign_negative()) {
                return false;
            }
        }
    }
    true
}

/// the column and the marker count
pub fn column(text: &str) -> (Column, usize) {
    let (c, m, _) = column_detail(text);
    (c, m)
}

/// the column, the marker count and the detail (guard markers and alias chains used)
pub fn column_detail(text: &str) -> (Column, usize, Vec<String>) {
    let (nodes, cmds, be) = parse(text);
    let mut out = Vec::new();
    let mut mk = 0;
    let mut detail: Vec<String> = Vec::new();
    for c in &cmds {
        let mut pos = 0;
        while let Some(p) = c[pos..].find("(run-primitive ") {
            let i = pos + p;
            let mut depth = 0i32;
            let mut j = i;
            for (k, ch) in c[i..].char_indices() {
                if ch == '(' {
                    depth += 1;
                } else if ch == ')' {
                    depth -= 1;
                    if depth == 0 {
                        j = i + k + 1;
                        break;
                    }
                }
            }
            if j == i {
                j = c.len();
            }
            let body = &c[i + 1..j.saturating_sub(1).max(i + 1)];
            pos = j.max(i + 1);
            // body: run-primitive A B :refD NAME groups...
            let tk: Vec<&str> = body.splitn(6, char::is_whitespace).collect();
            if tk.len() < 6 || !tk[3].starts_with(":ref") {
                out.push(("?".to_string(), vec!["<NONODE>".to_string()]));
                mk += 1;
                continue;
            }
            let dest: i64 = tk[3][4..].split('.').next().and_then(|s| s.parse().ok()).unwrap_or(-1);
            let printed = tk[4];
            let gs = groups(tk[5]);
            let ok = |n: i64| -> bool {
                let nn = &nodes[&n];
                if prim_of(nn).trim_start_matches('_') != printed.trim_start_matches('_') {
                    return false;
                }
                let al = args_of(nn);
                if al.len() != gs.len() {
                    return false;
                }
                al.iter().zip(&gs).all(|(x, g)| match g {
                    G::Ref(r) => aliases(&nodes, *x, *r),
                    _ => true,
                })
            };
            let mut cands: Vec<i64> = nodes.keys().copied().filter(|n| ok(*n)).collect();
            let vm: Vec<i64> = cands.iter().copied().filter(|n| vals_match(&nodes, *n, &gs, be)).collect();
            if !vm.is_empty() {
                cands = vm;
            }
            let ridx = out.len();
            let chosen = if cands.contains(&dest) { Some(dest) } else { cands.first().copied() };
            let mut args: Vec<String> = Vec::new();
            // tie-break guards: more than one candidate left after the printed-text match (and the destination does not decide),
            // or a chosen node whose constant lanes disagree with its printed text beyond print precision, is a marker (the pair becomes UNRESOLVED)
            if chosen.is_some() && !cands.contains(&dest) && cands.len() > 1 {
                args.push("<AMBIGUOUS>".into());
                detail.push(format!("runprim {} {}: {} candidates after the printed-text match", ridx, printed, cands.len()));
            }
            if let Some(n) = chosen {
                if !text_consistent(&nodes, n, &gs, be) {
                    args.push("<MAPPING-TEXT-MISMATCH>".into());
                    detail.push(format!("runprim {} {}: node {} lanes disagree with the printed text (MAPPING-TEXT-MISMATCH)", ridx, printed, n));
                }
                for (k, (x, g)) in args_of(&nodes[&n]).iter().zip(&gs).enumerate() {
                    if let G::Ref(r) = g {
                        if let Some(s) = alias_steps(&nodes, *x, *r) {
                            if s > 0 {
                                detail.push(format!("runprim {} {} arg {}: alias chain {} step(s) from node {} to ref {}", ridx, printed, k, s, x, r));
                            }
                        }
                    }
                }
            }
            match chosen {
                None => args.push("<NONODE>".into()),
                Some(n) => {
                    for (k, a) in args_of(&nodes[&n]).iter().enumerate() {
                        match gs.get(k) {
                            None => args.push("<MISSING>".into()),
                            Some(G::Clip(s)) => args.push(format!("~{}~", s)),
                            Some(G::Ref(_)) => args.push(":REF".into()),
                            Some(G::Br(_)) => match nodes.get(a).and_then(|nn| words(nn, be)) {
                                Some(w) => args.push(format!("{{{}}}", w.join(","))),
                                None => args.push("<NONODE>".into()),
                            },
                        }
                    }
                }
            }
            mk += args.iter().filter(|x| x.starts_with('<')).count();
            out.push((printed.to_string(), args));
        }
    }
    (out, mk, detail)
}

/// (verdict, n_orig, n_native, first difference or detail). Verdicts: PASS, ARGCOL-VACUOUS (equal columns that compare no
/// constant: 0 run-primitives or no constant argument; never a PASS), FAIL, UNRESOLVED (any marker, the two tie-break guards included).
pub fn compare(orig: &str, native: &str) -> (&'static str, usize, usize, String) {
    let (co, mo, dor) = column_detail(orig);
    let (cn, mn, dna) = column_detail(native);
    let mut det: Vec<String> = dor.iter().map(|d| format!("orig {}", d)).collect();
    det.extend(dna.iter().map(|d| format!("native {}", d)));
    let dj = det.join("; ");
    if mo > 0 || mn > 0 {
        return ("UNRESOLVED", co.len(), cn.len(), format!("markers orig={} native={}{}{}", mo, mn, if dj.is_empty() { "" } else { "; " }, dj));
    }
    if co == cn {
        let nconst = co.iter().flat_map(|(_, a)| a.iter()).filter(|x| x.starts_with('{')).count();
        if co.is_empty() || nconst == 0 {
            return ("ARGCOL-VACUOUS", co.len(), cn.len(), format!("{} run-primitives, {} constant arguments{}{}", co.len(), nconst, if dj.is_empty() { "" } else { "; " }, dj));
        }
        return ("PASS", co.len(), cn.len(), dj);
    }
    let mut fd = if co.len() != cn.len() { format!("count {} vs {}", co.len(), cn.len()) } else { String::new() };
    for (i, (x, y)) in co.iter().zip(&cn).enumerate() {
        if x != y {
            let m = x.1.len().max(y.1.len());
            fd = format!("runprim {} ({} | {}): primitive differs", i, x.0, y.0);
            if x.0 == y.0 {
                for j in 0..m {
                    let a = x.1.get(j).map(String::as_str).unwrap_or("<none>");
                    let b = y.1.get(j).map(String::as_str).unwrap_or("<none>");
                    if a != b {
                        fd = format!("runprim {} ({} | {}) arg {}: orig {} native {}", i, x.0, y.0, j, a, b);
                        break;
                    }
                }
            }
            break;
        }
    }
    ("FAIL", co.len(), cn.len(), fd)
}

#[cfg(test)]
mod tests {
    use super::*;
    const ORIG: &str = "#EP3\nNODE\t1\tsrc\tkind=2\tprim=src\tout_ch=2\targs=\tlanes=00000000000000000000000000000000\nNODE\t2\tk\tkind=1\tprim=\tout_ch=1\targs=\tlanes=0000803f000000000000000000000000\nNODE\t3\t_scale\tkind=2\tprim=scale\tout_ch=2\targs=1,2\tlanes=00000000000000000000000000000000\nCMD\t1\texec-linewise\t(exec-linewise 0 0\\n ( (run-primitive 1 -1 :ref9.2 scale :ref1.2 [ 1 ] )\\n )\\n)\n";
    #[test]
    fn pass_and_flipped_lane_fails_naming_argument() {
        let (v, n, _, _) = compare(ORIG, ORIG);
        assert_eq!((v, n), ("PASS", 1));
        // the flipped side prints its own lane (0.75), as a native plan text does
        let flipped = ORIG.replace("0000803f", "0000403f").replace("[ 1 ]", "[ 0.75 ]");
        let (v, _, _, fd) = compare(ORIG, &flipped);
        assert_eq!(v, "FAIL");
        assert!(fd.contains("runprim 0 (scale | scale) arg 1"), "{}", fd);
        assert!(fd.contains("{0000803f}") && fd.contains("{0000403f}"), "{}", fd);
    }
    #[test]
    fn missing_group_is_unresolved() {
        let t = ORIG.replace("[ 1 ] ", "");
        assert_eq!(compare(&t, &t).0, "UNRESOLVED");
    }
    #[test]
    fn int9_vacuous_and_guards() {
        // 0 run-primitives: ARGCOL-VACUOUS, never PASS
        let none = "#EP3\nNODE\t1\tsrc\tkind=2\tprim=src\tout_ch=2\targs=\tlanes=00000000000000000000000000000000\nCMD\t1\tfree-images\t(free-images :ref1.2)\n";
        assert_eq!(compare(none, none).0, "ARGCOL-VACUOUS");
        // a run-primitive with only image references: ARGCOL-VACUOUS
        let refs = "#EP3\nNODE\t1\tsrc\tkind=2\tprim=src\tout_ch=2\targs=\tlanes=00000000000000000000000000000000\nNODE\t3\t_neg\tkind=2\tprim=neg\tout_ch=2\targs=1\tlanes=00000000000000000000000000000000\nCMD\t1\texec-linewise\t(exec-linewise 0 0\\n ( (run-primitive 1 -1 :ref3.2 neg :ref1.2 )\\n )\\n)\n";
        assert_eq!(compare(refs, refs).0, "ARGCOL-VACUOUS");
        // a node whose lane disagrees with its printed text beyond print precision: MAPPING-TEXT-MISMATCH, UNRESOLVED
        let mism = ORIG.replace("[ 1 ]", "[ 1.01 ]");
        let (v, _, _, d) = compare(&mism, &mism);
        assert_eq!(v, "UNRESOLVED");
        assert!(d.contains("MAPPING-TEXT-MISMATCH"), "{}", d);
        // within print precision (0.333333 for 1/3): consistent
        let third = ORIG.replace("0000803f", "abaaaa3e").replace("[ 1 ]", "[ 0.333333 ]");
        assert_eq!(compare(&third, &third).0, "PASS");
        // two candidates with equal printed values and a destination that names neither: AMBIGUOUS, UNRESOLVED
        let amb = "#EP3\nNODE\t1\tsrc\tkind=2\tprim=src\tout_ch=2\targs=\tlanes=00000000000000000000000000000000\nNODE\t2\tk\tkind=1\tprim=\tout_ch=1\targs=\tlanes=0000803f000000000000000000000000\nNODE\t3\t_scale\tkind=2\tprim=scale\tout_ch=2\targs=1,2\tlanes=00000000000000000000000000000000\nNODE\t4\t_scale\tkind=2\tprim=scale\tout_ch=2\targs=1,2\tlanes=00000000000000000000000000000000\nCMD\t1\texec-linewise\t(exec-linewise 0 0\\n ( (run-primitive 1 -1 :ref9.2 scale :ref1.2 [ 1 ] )\\n )\\n)\n";
        let (v, _, _, d) = compare(amb, amb);
        assert_eq!(v, "UNRESOLVED");
        assert!(d.contains("2 candidates"), "{}", d);
    }
}
