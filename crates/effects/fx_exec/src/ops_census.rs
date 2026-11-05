//! INT24 (W526 (a), W533 (d), W538 (a) 3): the OPS_SCALAR_PAIRS census rows of one compiled plan, the claim-route tripwire
//! OPS-ARITH-SCALAR-SCALAR-UNJUDGED, and the program-text path of a plan node (the operator and its argument expressions beside the id).
//! Census only: nothing here changes a plan or an output bit.
use fx_eval::{CompiledProgram, Genotype, NodeKind, PlanNode};
use std::collections::BTreeMap;

/// One run-primitive of `+`, `*`, `-`, `/` or `dissolve` in a plan: (node idx, op, per-operand plane flag (true = plane, false = non-plane),
/// per-operand counts, nout, program-text path).
#[derive(Clone, Debug, PartialEq)]
pub struct OpsRow {
    pub node: usize,
    pub gid: usize,
    pub op: String,
    pub plane: Vec<bool>,
    pub counts: Vec<i32>,
    pub nout: i32,
    pub path: String,
}

pub const OPS_CLASSES: [&str; 5] = ["+", "*", "-", "/", "dissolve"];

fn by_idx(cp: &CompiledProgram) -> BTreeMap<usize, &PlanNode> {
    cp.nodes.iter().map(|n| (n.idx, n)).collect()
}

/// A node executed as a run-primitive (the executor's own test, exec.rs: func or auto, not a constant).
pub fn is_run_primitive(n: &PlanNode) -> bool {
    (n.kind == "func" || n.kind == "auto") && !(n.b160 || n.folded)
}

/// An operand that is not a plane: a constant node, a folded node or an alias to a constant .
pub fn is_non_plane(n: &PlanNode) -> bool {
    n.kind == "const" || n.b160 || n.folded
}

fn lanes_text(n: &PlanNode) -> String {
    let k = n.elnum.clamp(1, 4) as usize;
    let v: Vec<String> = (0..k).map(|j| format!("{}", f32::from_bits(n.lanes[j]))).collect();
    if k == 1 { v[0].clone() } else { format!("[ {} ]", v.join(" ")) }
}

fn expr(m: &BTreeMap<usize, &PlanNode>, idx: usize, depth: usize) -> String {
    let n = match m.get(&idx) {
        Some(n) => n,
        None => return format!("#{}", idx),
    };
    if let Some(p) = &n.param {
        return p.clone();
    }
    if n.kind == "const" {
        return lanes_text(n);
    }
    if n.kind != "func" && n.kind != "auto" {
        return n.name.clone();
    }
    if depth == 0 || (!n.name.starts_with('_') && !n.name.is_empty() && depth < 3) {
        return n.name.clone();
    }
    let a: Vec<String> = n.args.iter().map(|x| expr(m, *x, depth - 1)).collect();
    format!("( {} {} )", n.prim, a.join(" "))
}

/// The program-text path of node `idx`: "<bound name>: <prim> <argument expressions>" (W533 (d) form), argument expressions to depth 2.
pub fn node_path(cp: &CompiledProgram, idx: usize) -> String {
    let m = by_idx(cp);
    match m.get(&idx) {
        Some(n) => {
            let a: Vec<String> = n.args.iter().map(|x| expr(&m, *x, 2)).collect();
            format!("{}: {} {}", n.name, n.prim, a.join(" "))
        }
        None => format!("#{} (not in the plan)", idx),
    }
}

/// The program-text path of the plan node whose genotype id is `gid` (fold census rows carry genotype ids).
pub fn node_path_gid(cp: &CompiledProgram, gid: usize) -> String {
    match cp.nodes.iter().find(|n| n.gid == gid) {
        Some(n) => node_path(cp, n.idx),
        None => format!("gid {} (folded away, not in the plan)", gid),
    }
}

fn gexpr(g: &Genotype, id: usize, depth: usize) -> String {
    let n = match g.nodes.get(id) {
        Some(n) => n,
        None => return format!("#{}", id),
    };
    if let Some(p) = g.params.iter().find(|p| p.node == Some(id)) {
        return p.name.clone();
    }
    match n.kind {
        NodeKind::Scalar => format!("{}", n.lanes[0]),
        NodeKind::Vector => format!("[ {} ]", (0..n.nlanes.clamp(1, 4)).map(|j| format!("{}", n.lanes[j])).collect::<Vec<_>>().join(" ")),
        NodeKind::Str => n.text.clone(),
        NodeKind::Input | NodeKind::Special => n.name.clone(),
        NodeKind::Function => {
            if depth == 0 || (!n.label.starts_with('_') && !n.label.is_empty() && depth < 3) {
                return n.label.clone();
            }
            let a: Vec<String> = n.children.iter().map(|c| gexpr(g, *c, depth - 1)).collect();
            format!("( {} {} )", n.name, a.join(" "))
        }
    }
}

/// INT25 (INT24 finding 1, W533 (d)): the program-text path of genotype node `gid`: "<label>: <function> <argument expressions>", the
/// arguments to depth 2 (parameters by name, constants by value, inputs by name), read from the genotype, not from the plan.
pub fn genotype_path(g: &Genotype, gid: usize) -> String {
    match g.nodes.get(gid) {
        Some(n) => {
            let a: Vec<String> = n.children.iter().map(|c| gexpr(g, *c, 2)).collect();
            format!("{}: {} {}", n.label, n.name, a.join(" "))
        }
        None => format!("gid {} (not in the genotype)", gid),
    }
}

/// INT25: the path of a fold or decision row by genotype id. A plan node that keeps its arguments gives the plan path; a node the plan keeps
/// without arguments (a folded node) or does not keep gives the genotype path, marked "(genotype path)".
pub fn node_path_fold(cp: &CompiledProgram, g: &Genotype, gid: usize) -> String {
    match cp.nodes.iter().find(|n| n.gid == gid) {
        Some(n) if !n.args.is_empty() => node_path(cp, n.idx),
        _ => format!("{} (genotype path)", genotype_path(g, gid)),
    }
}

/// Every run-primitive of the OPS classes in the plan.
pub fn ops_rows(cp: &CompiledProgram) -> Vec<OpsRow> {
    let m = by_idx(cp);
    let mut out = Vec::new();
    for n in &cp.nodes {
        if !is_run_primitive(n) {
            continue;
        }
        let op = crate::kernels::normalize_class(&n.prim);
        if !OPS_CLASSES.contains(&op) {
            continue;
        }
        let mut plane = Vec::new();
        let mut counts = Vec::new();
        for a in &n.args {
            match m.get(a) {
                Some(x) => {
                    plane.push(!is_non_plane(x));
                    counts.push(x.elnum);
                }
                None => {
                    plane.push(true);
                    counts.push(-1);
                }
            }
        }
        out.push(OpsRow { node: n.idx, gid: n.gid, op: op.to_string(), plane, counts, nout: n.elnum, path: node_path(cp, n.idx) });
    }
    out
}

/// The census line of one row (printed on the 'ae32' route as a diagnostic label).
pub fn census_line(r: &OpsRow) -> String {
    format!(
        "diagnostic OPS-ARITH-RUN\tnode {}\tgid {}\top {}\tflags {}\tcounts {}\tnout {}\tpath {}",
        r.node,
        r.gid,
        r.op,
        r.plane.iter().map(|p| if *p { "P" } else { "S" }).collect::<Vec<_>>().join(","),
        r.counts.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(","),
        r.nout,
        r.path
    )
}

/// W526 (a) 2: the tripwire. A run-primitive of `+`, `*`, `-` or `/` with both operands non-plane is outside ops_arith's judged R cases.
pub fn scalar_scalar_tripwire(program: &str, rows: &[OpsRow]) -> Option<String> {
    rows.iter()
        .find(|r| r.op != "dissolve" && r.plane.len() >= 2 && !r.plane[0] && !r.plane[1])
        .map(|r| {
            format!(
                "OPS-ARITH-SCALAR-SCALAR-UNJUDGED ({}: node {}: {} with both operands non-plane, counts {},{}, nout {}; this operand pair is outside ops_arith's judged R cases; PLR4, W526)",
                program, r.node, r.op, r.counts[0], r.counts[1], r.nout
            )
        })
}
