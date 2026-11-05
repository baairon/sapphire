//! STK2: the node-grouping pass (stack word 2 = 1) as a PRESERVE-group model on the native node graph.
//!
//! Source of every rule: listing neutral_1.asm (offsets, VA = offset). Labels: READ = instruction read; INFERRED = reasoning;
//! ASSUMED = a stand-in listed in GROUPING2.md (HIGH-STAKES ASSUMPTIONS).
//!
//! The planner runs, when [compiler] (stack word 2) is nonzero (guard), two walks of the node tree through the generic
//! walker (READ): the callback in post order (children first) and the callback in pre order (the walker stores the DFS
//! parent in the visitor cell [v] before each callback). A third walk, runs for every word 2 and marks 
//! on every node with a group id != -1 and on all its qualifying descendants (READ).
//!
//!  (READ): [node] = 1; for a function node (kind == 2) over its qualifying arguments c: a child c with [c] != 0 and
//! [c] == 0 (a literal constant) is skipped; otherwise [node] = 0 if [c] == 0 or [c] != [node].
//!  is nonzero only below a `motion_blur` class node (string cell = "motion_blur", READ in place). This model refuses
//! programs that contain such a node (STACK-WORD2-MOTION-BLUR-NOT-MODELLED), so = 0 everywhere and the child test can never clear the
//! flag (INFERRED from the refusal): the flag is 1 for every visited node.
//!
//!  (READ), per node in pre order: return unless flag != 0 and == -1 and != 1; if the DFS parent exists and its
//!  >= 0 the node JOINS it ([node] = [parent]); else for a function node with == 0 and == 0 it runs
//! the head test: the nested walk (c = 1, post order, visited bytes) with the visitor (vtable cell slot 2, READ in
//! place) which fails if any visited node has >= 0, marks every visited node except the head in Q and counts R[x]++ for every
//! qualifying argument slot of every visited node; the 6-slot test (all consumer formats other than the head's own format
//! must have count 0 in the histogram [+ slot * 24], filled by from class slot); and the scan (every Q
//! node with R < (distinct consumers, a752) must be a kind-1 node with == 0). A head gets = 1 and
//!  = counter++ (counter starts at 0, [walker]).
//!
//!  (READ): input node: the has-data flag the owner cell; function node: class flag bit 0x100 or 0x80 (class slot)
//! or any qualifying child's; constant: parameter record bits (ASSUMED 0).
//!
//! ASSUMED (GROUPING2.md): the qualifying-argument predicate (class slot) is "the argument is an image": a function, an input with data
//! that is not a donor or clip argument, or a constant materialised as an image (a pass clone, = 1) at an image-only argument.

use crate::compile::CompiledProgram;
use crate::cpuplan::{image_only_arg, is_donor_arg};
use std::collections::{HashMap, HashSet};

pub struct Pres {
    ///  per node index, -1 = none
    pub key: Vec<i64>,
    ///  per node index (head of a group)
    pub head: Vec<bool>,
    /// nodes marked by through a group id (the node and its qualifying descendants)
    pub mark: HashSet<usize>,
    ///  per node index (diagnostics)
    pub fx_eval_pres_a: Vec<bool>,
    /// DFS order of the pre-order walk (diagnostics)
    pub order: Vec<usize>,
}

/// Programs the model does not cover are refused by name instead of being planned on a guess.
pub fn model_refusal(cp: &CompiledProgram) -> Option<String> {
    if cp.nodes.iter().any(|n| (n.kind == "func" || n.kind == "auto") && n.prim.strip_suffix("_if_needed").unwrap_or(&n.prim) == "motion_blur") {
        return Some("STACK-WORD2-MOTION-BLUR-NOT-MODELLED: a motion_blur node sets the class field whose use in is not modelled".into());
    }
    if crate::cpuplan::ds_factor(cp) > 1 {
        return Some("STACK-WORD2-DOWNSAMPLE-NOT-MODELLED: the preserve group inside an exec-downsampled command is not modelled".into());
    }
    None
}

fn kind_of(k: &str) -> u8 {
    match k {
        "func" | "auto" => 2,
        "input" | "special" => 4,
        _ => 1,
    }
}

/// qualifying arguments (class slot, ASSUMED image arguments): (argument position, node index)
fn qual(cp: &CompiledProgram, n: usize) -> Vec<(usize, usize)> {
    let nd = &cp.nodes[n];
    if !(nd.kind == "func" || nd.kind == "auto") {
        return vec![];
    }
    let mut v = Vec::new();
    for (k, &a) in nd.args.iter().enumerate() {
        let an = &cp.nodes[a];
        let ok = match an.kind {
            "func" | "auto" => true,
            "input" | "special" => !(an.kind == "special" && an.name == "Output") && !is_donor_arg(&nd.prim, k, an.kind) && !(nd.prim.strip_suffix("_if_needed").unwrap_or(&nd.prim) == "get_frame" && k == 0),
            "const" => image_only_arg(&nd.prim, k),
            _ => false,
        };
        if ok {
            v.push((k, a));
        }
    }
    v
}

fn flag_of_class(prim: &str, bits: u32) -> bool {
    crate::layout::flags_of(prim) & bits != 0
}

/// has-data flag of an input node: bit (declared position + 3) of the context word (INT9 presence_bit_rows: node index of declared input j is j + 3)
fn has_data(cp: &CompiledProgram, n: usize, hd: u32) -> bool {
    let nd = &cp.nodes[n];
    if nd.kind == "special" {
        // xy variable: ASSUMED frame dependent (conservative: never preserved) on the existing path; XWP2 (W746): under the placement model the
        // xy clone is not frame dependent (INFERRED from the recorded plans: TB, Blur_Only and Warp SR preserve groups hold a `variable 2`)
        return !cp.xy_place;
    }
    let j = nd.input_pos.saturating_sub(1);
    (hd >> (j + 3)) & 1 != 0
}

pub fn analyze(cp: &CompiledProgram, hd: u32) -> Pres {
    let nn = cp.nodes.len();
    let fmtv = crate::cpuplan::fmt_pass_pub(cp);
    let mut key = vec![-1i64; nn];
    let mut head = vec![false; nn];
    // live nodes, pre order, DFS parent (generic walker : pre order, qualifying arguments in order, visited bytes)
    let mut order: Vec<usize> = Vec::new();
    let mut parent: Vec<Option<usize>> = vec![None; nn];
    let mut seen = vec![false; nn];
    fn pre(cp: &CompiledProgram, n: usize, p: Option<usize>, seen: &mut Vec<bool>, parent: &mut Vec<Option<usize>>, order: &mut Vec<usize>) {
        if seen[n] {
            return;
        }
        seen[n] = true;
        parent[n] = p;
        order.push(n);
        for (_, a) in qual(cp, n) {
            pre(cp, a, Some(n), seen, parent, order);
        }
    }
    pre(cp, cp.root, None, &mut seen, &mut parent, &mut order);
    //  bottom up
    let mut fx_eval_pres_a = vec![false; nn];
    let mut done = vec![false; nn];
    fn up(cp: &CompiledProgram, n: usize, hd: u32, c: &mut Vec<bool>, done: &mut Vec<bool>) {
        if done[n] {
            return;
        }
        done[n] = true;
        let nd = &cp.nodes[n];
        let mut v = match kind_of(nd.kind) {
            4 => has_data(cp, n, hd),
            2 => {
                let p = nd.prim.strip_suffix("_if_needed").unwrap_or(&nd.prim);
                flag_of_class(p, 0x100) || flag_of_class(p, 0x80)
            }
            _ => false,
        };
        for (_, a) in qual(cp, n) {
            up(cp, a, hd, c, done);
            if c[a] {
                v = true;
            }
        }
        // XWP2: is set by before the fold (planner order READ), so a folded constant keeps its unfolded chain's value
        if cp.xy_place && *cp.fx_eval_pres_a_pre.get(&nd.gid).unwrap_or(&false) {
            v = true;
        }
        c[n] = v;
    }
    up(cp, cp.root, hd, &mut fx_eval_pres_a, &mut done);
    // post order: flag = 1; a function node clears it when a non-literal child has flag 0 or a different class (always 0 here)
    let mut flag = vec![true; nn];
    for &n in order.iter().rev() {
        // reverse pre order visits every node after all nodes below it on its DFS path; a DAG child reached by another parent was visited earlier or later,
        // but flags only depend on the flags of children, all 1 when is uniform, so the order cannot change the result
        for (_, a) in qual(cp, n) {
            let literal = cp.nodes[a].b160 && !(cp.nodes[a].kind == "const");
            let _ = literal;
            #[cfg(not(feature = "mut_childclear"))]
            {
                if !flag[a] {
                    flag[n] = false;
                }
            }
        }
    }
    // consumers over qualifying edges of the live graph: distinct consumers and the formats they ask for (the histogram of)
    let mut cons: Vec<HashSet<usize>> = vec![HashSet::new(); nn];
    let mut want: Vec<Vec<u32>> = vec![Vec::new(); nn];
    for &c in order.iter() {
        let p = &cp.nodes[c].prim;
        for (k, a) in qual(cp, c) {
            cons[a].insert(c);
            want[a].push(crate::cpuplan::fmt_s22_pub(p, k, fmtv[c]));
        }
    }
    let mut counter = 0i64;
    for &n in order.iter() {
        // entry test: flag != 0 (above), == -1 (always, no multi-pass class is modelled), != 1
        if !flag[n] {
            continue;
        }
        // JOIN: the DFS parent carries a group id
        let join = !cfg!(feature = "mut_join") && parent[n].map(|p| key[p] >= 0).unwrap_or(false);
        if join {
            key[n] = key[parent[n].unwrap()];
            continue;
        }
        let nd = &cp.nodes[n];
        if kind_of(nd.kind) != 2 || fx_eval_pres_a[n] || nd.b160 {
            continue;
        }
        // nested walk: post order, visited bytes, children = qualifying arguments, parent = Some(head) for every visited node but the head
        let mut vis = vec![false; nn];
        let mut q = vec![false; nn];
        let mut r = vec![0usize; nn];
        fn nest(cp: &CompiledProgram, n: usize, root: usize, vis: &mut Vec<bool>, q: &mut Vec<bool>, r: &mut Vec<usize>) {
            if vis[n] {
                return;
            }
            for (_, a) in qual(cp, n) {
                nest(cp, a, root, vis, q, r);
            }
            if n != root {
                q[n] = true;
            }
            for (_, a) in qual(cp, n) {
                r[a] += 1;
            }
            vis[n] = true;
        }
        // the visited byte is set after the children, so a node reached again during its own subtree cannot occur in a DAG
        nest(cp, n, n, &mut vis, &mut q, &mut r);
        // 6-slot test: every consumer of the head asks for the head's own format
        let own = fmtv[n];
        if want[n].iter().any(|&f| f != own) {
            continue;
        }
        // scan : Q nodes referenced from outside the group must be literal constants (kind 1, == 0)
        let mut ok = true;
        for i in 0..nn {
            if q[i] && r[i] < cons[i].len() {
                // a constant image clone is private to its consumer (pass clones per argument, ASSUMED), so it is exempt like a kind-1 node
                if !(kind_of(cp.nodes[i].kind) == 1) {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            continue;
        }
        key[n] = counter;
        counter += 1;
        head[n] = true;
    }
    // on every node with a group id and on its qualifying descendants (pre order propagation)
    let mut mark: HashSet<usize> = HashSet::new();
    for &n in order.iter() {
        if key[n] >= 0 {
            mark.insert(n);
        }
        if mark.contains(&n) {
            for (_, a) in qual(cp, n) {
                mark.insert(a);
            }
        }
    }
    Pres { key, head, mark, fx_eval_pres_a, order }
}

/// the preserve command text: the commands of the group in emission order inside one container (READ: CL1 plan, command 10)
pub fn wrap_text(head_ref: &str, inner: &[String]) -> String {
    format!("(preserve {}\\n  ( {}\\n  )\\n)", head_ref, inner.join("\\n    "))
}

#[allow(dead_code)]
fn _unused(_: &HashMap<usize, usize>) {}
