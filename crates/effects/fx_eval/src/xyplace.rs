//! default-xy writer placement, as the original's word 2 = 1 plans place it.
//!
//! RULE (pass order READ in the planner of neutral_1.asm, listing offsets; each step is the ported pass of this crate):
//! 1. (called): every function-node argument slot whose child is a kind 4 node (Special or Input) gets a fresh clone, one per slot
//!  (compile.rs, READ RULE). The fold runs after it, so a consumer that reads xy through a collapsed node reads the
//!    clone made for the collapsed node's slot (Warp's rotate at angle 0: perspective and perspective_alpha share one clone).
//! 2. (before the fold):; the xy clone is not frame dependent (INFERRED from the records), so `xy - centre` can head a group.
//! 3. (stack word 2 = 1): the preserve analysis (preserve.rs) on the unshared graph, one clone per slot, so a clone
//!    is private to its consumer and the scan passes.
//! 4. call 1, then the group pass with the preserve id in its key (lgpass.rs key_eq, READ),
//!    then call 2 : the Special policy keeps the first clone of a run of equal group ids and clears its table when the group id
//!  changes (compile.rs, READ). Clones in different groups stay separate writers.
//! 5. The plan builder (cpuplan.rs) then places each writer: allocated at top level (a plane, rows at base + row x W x 4: PerRow), allocated in the
//!    line list of the exec block that writes it (one row buffer for every row: OneHead), or inside a preserve group (written in render 1 only and
//!    reused by render 2).
use crate::build::Genotype;
use crate::compile::{compile, ComboTable, CompileError, CompileOptions, CompiledProgram};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    /// allocated outside every exec block (top level or the body of a preserve group): a plane indexed by row
    Plane,
    /// allocated in the line list of the exec block that writes it: one row buffer
    RowBuffer,
}

#[derive(Clone, Debug)]
pub struct XyWriter {
    /// plan node index of the writer (the `:refN` of its `variable 2`)
    pub node: usize,
    /// genotype id (CNode id) of the writer
    pub gid: usize,
    pub place: Place,
    /// inside a preserve group: runs in render 1, render 2 reuses the preserved image
    pub preserve: bool,
    /// (consumer gid, argument slot) of every consumer
    pub consumers: Vec<(usize, usize)>,
}

#[derive(Debug)]
enum T {
    L(Vec<T>),
    A(String),
}

fn parse(ts: &[String], i: &mut usize) -> Vec<T> {
    let mut v = Vec::new();
    while *i < ts.len() {
        let t = &ts[*i];
        *i += 1;
        if t == "(" {
            v.push(T::L(parse(ts, i)));
        } else if t == ")" {
            return v;
        } else {
            v.push(T::A(t.clone()));
        }
    }
    v
}

fn tokens(s: &str) -> Vec<String> {
    let s = s.replace("\\n", " ").replace('(', " ( ").replace(')', " ) ");
    s.split_whitespace().map(|x| x.to_string()).collect()
}

fn refn(a: &str) -> Option<usize> {
    a.strip_prefix(":ref")?.split('.').next()?.parse().ok()
}

/// walk: records allocations (ref -> (in preserve, exec id or None)) and variables (ref, in preserve, exec id)
fn walk(t: &[T], pres: bool, blk: Option<usize>, nblk: &mut usize, alloc: &mut HashMap<usize, (bool, Option<usize>)>, vars: &mut Vec<(usize, bool, Option<usize>)>) {
    let head = match t.first() {
        Some(T::A(h)) => Some(h.as_str()),
        _ => None,
    };
    let at = |k: usize| match t.get(k) {
        Some(T::A(s)) => s.as_str(),
        _ => "",
    };
    match head {
        Some("allocate-images") | Some("allocate-images-preserved") => {
            if let Some(r) = refn(at(1)) {
                alloc.insert(r, (pres, blk));
            }
            return;
        }
        Some("variable") if at(1) == "2" => {
            if let Some(r) = refn(at(2)) {
                vars.push((r, pres, blk));
            }
            return;
        }
        Some("run-primitive") => {
            if let Some(r) = refn(at(3)) {
                RUNS.with(|v| v.borrow_mut().push((r, at(4).to_string(), blk)));
            }
            return;
        }
        _ => {}
    }
    let mut pres = pres;
    let mut blk = blk;
    if head == Some("preserve") {
        pres = true;
    }
    if head == Some("exec-linewise") || head == Some("exec-downsampled") {
        *nblk += 1;
        blk = Some(*nblk);
    }
    let start = if head.is_some() { 1 } else { 0 };
    for c in &t[start..] {
        if let T::L(sub) = c {
            walk(sub, pres, blk, nblk, alloc, vars);
        }
    }
}

thread_local! {
    static RUNS: std::cell::RefCell<Vec<(usize, String, Option<usize>)>> = std::cell::RefCell::new(Vec::new());
    static ALLOC: std::cell::RefCell<HashMap<usize, (bool, Option<usize>)>> = std::cell::RefCell::new(HashMap::new());
}

/// (part 2): the placement of every run-primitive destination of a plan text: (destination ref, primitive, Plane or RowBuffer)
pub fn runs_of_text(text: &str) -> Vec<(usize, String, Place)> {
    RUNS.with(|v| v.borrow_mut().clear());
    let _ = writers_of_text(text);
    let runs = RUNS.with(|v| v.borrow().clone());
    let alloc = ALLOC.with(|a| a.borrow().clone());
    runs.into_iter()
        .map(|(r, p, blk)| {
            let place = match alloc.get(&r) {
                Some(&(_, b)) if b.is_some() && b == blk => Place::RowBuffer,
                _ => Place::Plane,
            };
            (r, p, place)
        })
        .collect()
}

/// compile with the placement model and return, per perspective-class node gid, whether its destination is a row buffer
pub fn persp_places(g: &Genotype, ct: &ComboTable, opt: &CompileOptions) -> Result<HashMap<usize, bool>, CompileError> {
    let (cp, _) = compile_placed(g, ct, opt)?;
    let text = crate::cpuplan::plan_text_mode(&cp, crate::cpuplan::PlanMode::CpuDriver);
    let mut m = HashMap::new();
    for (r, prim, place) in runs_of_text(&text) {
        let p = prim.strip_suffix("_if_needed").unwrap_or(&prim);
        if p == "perspective" || p == "perspective_alpha" {
            if let Some(nd) = cp.nodes.iter().find(|n| n.idx == r) {
                m.insert(nd.gid, place == Place::RowBuffer);
            }
        }
    }
    Ok(m)
}

/// the writers of a plan text (cpuplan's CMD rows or plain command lines), in execution order
pub fn writers_of_text(text: &str) -> Vec<(usize, Place, bool)> {
    let mut alloc = HashMap::new();
    let mut vars = Vec::new();
    let mut nblk = 0usize;
    for line in text.lines() {
        let cmd = if line.starts_with("CMD\t") { line.split('\t').nth(3).unwrap_or("") } else { line };
        let ts = tokens(cmd);
        let mut i = 0;
        let tree = parse(&ts, &mut i);
        for t in &tree {
            if let T::L(sub) = t {
                walk(sub, false, None, &mut nblk, &mut alloc, &mut vars);
            }
        }
    }
    ALLOC.with(|a| *a.borrow_mut() = alloc.clone());
    vars.iter()
        .map(|&(r, pres, blk)| {
            let place = match alloc.get(&r) {
                Some(&(_, b)) if b.is_some() && b == blk => Place::RowBuffer,
                _ => Place::Plane,
            };
            (r, place, pres)
        })
        .collect()
}

/// Compile with the placement model and return the program (writers shared by group, stack word 2 = 1) with its writers.
pub fn compile_placed(g: &Genotype, ct: &ComboTable, opt: &CompileOptions) -> Result<(CompiledProgram, Vec<XyWriter>), CompileError> {
    // pass A: one clone per slot, preserve analysis on it
    let mut oa = opt.clone();
    oa.ctx_word2 = true;
    oa.xy_place = true;
    oa.xy_unshared = true;
    oa.xy_pres_keys = None;
    let cpa = compile(g, ct, &oa)?;
    let mut keys: HashMap<usize, i64> = HashMap::new();
    if crate::preserve::model_refusal(&cpa).is_none() {
        let p = crate::preserve::analyze(&cpa, cpa.ctx_has_data);
        for nd in &cpa.nodes {
            if p.key[nd.idx] >= 0 {
                keys.insert(nd.gid, p.key[nd.idx]);
            }
        }
    }
    // pass B: the group pass with, then the Special sharing per group
    let mut ob = opt.clone();
    ob.ctx_word2 = true;
    ob.xy_place = true;
    ob.xy_unshared = false;
    ob.xy_pres_keys = Some(keys);
    let cp = compile(g, ct, &ob)?;
    let text = crate::cpuplan::plan_text_mode(&cp, crate::cpuplan::PlanMode::CpuDriver);
    let ws = writers_of_text(&text);
    let mut out = Vec::new();
    // hoist duplicates the xy variable: a full-size writer for the hoisted nodes, then the body writer at the small size (plan order)
    let dup = crate::cpuplan::ds_dup_rid(&cp);
    let hoist: Vec<usize> = if crate::cpuplan::ds_factor(&cp) > 1 { crate::cpuplan::ds_hoist(&cp) } else { Vec::new() };
    let has_dup = ws.iter().any(|w| w.0 == dup);
    for (r, place, preserve) in ws {
        let is_dup = r == dup;
        let nd = if is_dup {
            cp.nodes.iter().find(|n| n.kind != "func" && n.kind != "auto" && n.kind != "const" && n.name == "xy")
        } else {
            cp.nodes.iter().find(|n| n.idx == r)
        };
        let Some(nd) = nd else { continue };
        let mut consumers = Vec::new();
        for c in &cp.nodes {
            if c.kind == "func" || c.kind == "auto" {
                let hoisted = hoist.contains(&c.idx);
                if has_dup && hoisted != is_dup {
                    continue;
                }
                for (k, &a) in c.args.iter().enumerate() {
                    let reads = if is_dup { cp.nodes[a].kind != "func" && cp.nodes[a].kind != "auto" && cp.nodes[a].kind != "const" && cp.nodes[a].name == "xy" } else { a == nd.idx };
                    if reads {
                        consumers.push((c.gid, k));
                    }
                }
            }
        }
        out.push(XyWriter { node: nd.idx, gid: nd.gid, place, preserve, consumers });
    }
    if has_dup && std::env::var("XWP2_MUT").map_or(false, |v| v == "dsone") {
        out.truncate(1);
    }
    Ok((cp, out))
}
