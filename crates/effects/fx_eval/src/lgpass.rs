//! NV-3e: the linewise grouping pass of the plan builder, ported from the listing (function, helpers,
//! ). Listing offsets (VA = offset). Every rule below is STATIC (read from the code) unless marked.
//!
//! Input: the node array of the program in compile (eval) order, as the original holds it when the compile argument r8 = 0
//! (so the per-node gpu byte is 0 for every node). Output: the array in its final order and the group id `lg` of each
//! node; -1 = not linewise. Ids are NOT renumbered (the original leaves holes).
//!
//! Pass 1 : walk the array; a node is ELIGIBLE iff it is not a function node, or its class answers
//!   get_flag(1) = false (virtual slot with argument 1 = class flag word & 1; CLASS_FLAGS.tsv bit 0x1 = non linewise).
//!   Consecutive eligible nodes whose keys are equal to the previous node's share one group id; a
//!   new id otherwise. Ineligible nodes get -1. KEY fields: UNKNOWN meaning, ASSUMED equal for every node (see `key`).
//! Pass 2 : next free id = max id + 1.
//! Pass 3 (tail): for every node with id >= 0, take the FIRST image argument k for
//!   which the class slot (random access argument) answers true; scan the array backwards from the node for the nearest
//!   node j that is an argument of this node at a random access position; every node from j+1 on that has the node's id gets
//!   a fresh id (the node may not run in the group that produced its random access input).
//! Pass 4 = : for every run C of equal ids (from the second run on) hoist C backwards over the runs before it as long
//!   as C does not consume the passed nodes; with r14 the start of the earliest run reached, merge C into
//!   the run P before it and/or the run X it was moved before, through the predicate (`can_merge`).
//! Pass 5 = : last consumer of every image (array order).
//! Pass 6 : memory budget split of every run of at least two nodes (see `budget_split`).

use crate::layout::flags_of;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct LNode {
    pub id: usize,
    pub is_func: bool,
    pub prim: String,
    pub elnum: i32,
    pub fmt: u8,
    pub args: Vec<usize>,
    pub g: i32,
}

fn base(prim: &str) -> &str {
    prim.strip_suffix("_if_needed").unwrap_or(prim)
}

/// class slot with argument 1: class flag word & 1 (slot 5 =, STATIC; flag words in CLASS_FLAGS.tsv)
pub fn class_nonlinewise(prim: &str) -> bool {
    flags_of(base(prim)) & 1 != 0
}

/// class slot (random access argument k). Default = class flag word & 1; overrides (STATIC, slot 23 scan of all 320 classes):
/// k == 0 for blur_warp, blur_warp_bidirectional, caustic, chroma_warp*, emboss, flys_eye, hotspot_mask, median, repeat_warp*;
/// k < 2 for warp, warp_no_pass_arg, warp_rel.
pub fn random_access(prim: &str, k: i64) -> bool {
    match base(prim) {
        "blur_warp" | "blur_warp_bidirectional" | "caustic" | "chroma_warp" | "chroma_warp_dod_hack" | "chroma_warp_filter" | "emboss" | "flys_eye" | "hotspot_mask" | "median" | "repeat_warp" | "repeat_warp_filter" => k == 0,
        "warp" | "warp_no_pass_arg" | "warp_rel" => (0..2).contains(&k),
        p => flags_of(p) & 1 != 0,
    }
}

/// size units of an image (switch on [node], table : fmt 0 -> el*1, 1 -> el*2, 2 -> el*2, 3 -> el*8, 4 -> el*4, 5 -> 0, else el*2)
pub fn size_of(n: &LNode) -> i32 {
    let el = n.elnum;
    match n.fmt {
        0 => el,
        1 | 2 => el * 2,
        3 => el * 8,
        4 => el * 4,
        5 => 0,
        _ => el * 2,
    }
}

struct Ctx {
    nodes: Vec<LNode>,
    /// the group id per node id (absent = -1); equal keys are required for the same linewise group 
    keys: HashMap<usize, i64>,
}

impl Ctx {
    fn n(&self) -> usize {
        self.nodes.len()
    }
    fn pos(&self, id: usize) -> Option<usize> {
        self.nodes.iter().position(|x| x.id == id)
    }
    /// image argument predicate (class slot, APPROXIMATED as "the argument node is in the array", as NV-3c did)
    fn is_img_arg(&self, n: &LNode, k: usize) -> bool {
        n.is_func && self.pos(n.args[k]).is_some()
    }
    fn run_end(&self, i: usize) -> usize {
        let g = self.nodes[i].g;
        let mut j = i;
        while j < self.n() && self.nodes[j].g == g {
            j += 1;
        }
        j
    }
    fn key_eq(&self, a: usize, b: usize) -> bool {
        // (= -1) and (= 0) are uniform in the modelled programs; is the preserve group id (, READ)
        self.keys.get(&self.nodes[a].id).copied().unwrap_or(-1) == self.keys.get(&self.nodes[b].id).copied().unwrap_or(-1)
    }
    /// does any node in [s,e) take node `target_id` as image argument
    fn uses(&self, s: usize, e: usize, target_id: usize) -> bool {
        (s..e).any(|i| {
            let nd = &self.nodes[i];
            nd.is_func && (0..nd.args.len()).any(|k| self.is_img_arg(nd, k) && nd.args[k] == target_id)
        })
    }
    /// 
    fn can_merge(&self, a: (i64, i64), b: (i64, i64)) -> bool {
        if a.0 == -1 || b.0 == -1 || a.1 <= 0 || b.1 <= 0 {
            return false;
        }
        let (a0, a1, b0, b1) = (a.0 as usize, a.1 as usize, b.0 as usize, b.1 as usize);
        if self.nodes[a0].g < 0 || self.nodes[b0].g < 0 {
            return false;
        }
        if !self.key_eq(a1 - 1, b0) {
            return false;
        }
        let ga = self.nodes[a0].g;
        for i in b0..b1 {
            let nd = &self.nodes[i];
            if !nd.is_func {
                continue;
            }
            for k in 0..nd.args.len() {
                if !self.is_img_arg(nd, k) {
                    continue;
                }
                if random_access(&nd.prim, k as i64) {
                    if let Some(p) = self.pos(nd.args[k]) {
                        if self.nodes[p].g == ga {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

pub fn run_pass(input: Vec<LNode>) -> Vec<LNode> {
    run_pass_keys(input, HashMap::new())
}

/// the pass with the group ids per node id
pub fn run_pass_keys(input: Vec<LNode>, keys: HashMap<usize, i64>) -> Vec<LNode> {
    let mut c = Ctx { nodes: input, keys };
    let n = c.n();
    // pass 1
    let mut next: i32 = 0;
    let mut cur: i32 = -1;
    for i in 0..n {
        let elig = !c.nodes[i].is_func || !class_nonlinewise(&c.nodes[i].prim);
        let g;
        if !elig {
            g = -1;
        } else if cur < 0 {
            g = next;
            next += 1;
        } else if i > 0 && c.key_eq(i - 1, i) {
            g = cur;
        } else {
            g = next;
            next += 1;
        }
        c.nodes[i].g = g;
        cur = g;
    }
    // pass 2
    let mut nextid = c.nodes.iter().map(|x| x.g + 1).max().unwrap_or(0).max(0);
    // pass 3
    for i in 0..n {
        if c.nodes[i].g < 0 {
            continue;
        }
        let nd = c.nodes[i].clone();
        if !nd.is_func {
            continue;
        }
        let kra = (0..nd.args.len()).find(|&k| c.is_img_arg(&nd, k) && random_access(&nd.prim, k as i64));
        if kra.is_none() {
            continue;
        }
        let mut j = i as i64 - 1;
        while j >= 0 {
            let jid = c.nodes[j as usize].id;
            let kp = nd.args.iter().position(|&a| a == jid).map(|x| x as i64).unwrap_or(-1);
            if random_access(&nd.prim, kp) {
                let newid = nextid;
                nextid += 1;
                let gi = c.nodes[i].g;
                let mut t = (j + 1) as usize;
                while t < n && c.nodes[t].g == gi {
                    c.nodes[t].g = newid;
                    t += 1;
                }
                break;
            }
            j -= 1;
        }
    }
    // pass 4
    hoist(&mut c);
    // pass 2 again
    let mut nextid = c.nodes.iter().map(|x| x.g + 1).max().unwrap_or(0).max(0);
    // pass 5 + 6
    budget_split(&mut c, &mut nextid);
    c.nodes
}

fn hoist(c: &mut Ctx) {
    let n = c.n();
    if n == 0 {
        return;
    }
    let e0 = c.run_end(0);
    if e0 >= n {
        return;
    }
    let mut s = e0;
    let mut e = c.run_end(s);
    loop {
        let nxt = if e < n { Some(c.run_end(e)) } else { None };
        hoist_one(c, s, e);
        match nxt {
            None => break,
            Some(r) => {
                s = e;
                e = r;
            }
        }
    }
}

fn hoist_one(c: &mut Ctx, s: usize, e: usize) {
    if s == 0 {
        return;
    }
    // the node before C must not be consumed by C
    let prev_id = c.nodes[s - 1].id;
    if c.uses(s, e, prev_id) {
        return;
    }
    if !c.key_eq(s - 1, s) {
        return;
    }
    let mut r14 = s;
    let mut rsi = s;
    loop {
        rsi -= 1;
        if rsi == 0 {
            r14 = 0;
        } else if c.nodes[rsi].g != c.nodes[rsi - 1].g {
            r14 = rsi;
        }
        if rsi == 0 {
            break;
        }
        let cand = c.nodes[rsi - 1].id;
        if c.uses(s, e, cand) {
            break;
        }
        if !c.key_eq(rsi - 1, s) {
            break;
        }
    }
    if r14 >= s {
        return;
    }
    let xend = c.run_end(r14);
    let xr = (r14 as i64, xend as i64);
    let pr = if r14 > 0 {
        let gp = c.nodes[r14 - 1].g;
        let mut ps = r14 - 1;
        while ps > 0 && c.nodes[ps - 1].g == gp {
            ps -= 1;
        }
        (ps as i64, r14 as i64)
    } else {
        (-1, -1)
    };
    let cr = (s as i64, e as i64);
    let first = c.can_merge(pr, cr);
    let mut second = c.can_merge(cr, xr);
    if first && second {
        second = c.can_merge(pr, xr);
    }
    if !first && !second {
        return;
    }
    // move C to r14
    let blk: Vec<LNode> = c.nodes[r14..e].to_vec();
    let clen = e - s;
    let mut mv: Vec<LNode> = Vec::with_capacity(e - r14);
    mv.extend_from_slice(&blk[(s - r14)..]);
    mv.extend_from_slice(&blk[..(s - r14)]);
    for (t, nd) in mv.into_iter().enumerate() {
        c.nodes[r14 + t] = nd;
    }
    let r9 = r14 + clen;
    if c.nodes[r14].g < 0 {
        return;
    }
    if first {
        let gp = c.nodes[r14 - 1].g;
        for t in r14..r9 {
            c.nodes[t].g = gp;
        }
    }
    if second {
        let gx = c.nodes[r9].g;
        let gc = c.nodes[r9 - 1].g;
        let mut t = r9;
        while t < c.n() && c.nodes[t].g == gx {
            c.nodes[t].g = gc;
            t += 1;
        }
    }
}

/// consumers of node id (distinct array nodes taking it as image argument), with the argument position of the first match
fn consumers(c: &Ctx, id: usize) -> Vec<(usize, usize)> {
    let mut v = Vec::new();
    for (i, nd) in c.nodes.iter().enumerate() {
        if !nd.is_func {
            continue;
        }
        for k in 0..nd.args.len() {
            if nd.args[k] == id && c.is_img_arg(nd, k) {
                v.push((i, k));
                break;
            }
        }
    }
    v
}

/// image is local to its group: every consumer has the image's id and none reads it at a random access position
pub fn is_group_local(c_nodes: &[LNode], id: usize) -> bool {
    let c = Ctx { nodes: c_nodes.to_vec(), keys: HashMap::new() };
    local(&c, id)
}

fn local(c: &Ctx, id: usize) -> bool {
    let p = match c.pos(id) {
        Some(p) => p,
        None => return false,
    };
    let g = c.nodes[p].g;
    for (ci, k) in consumers(c, id) {
        if c.nodes[ci].g != g || random_access(&c.nodes[ci].prim, k as i64) {
            return false;
        }
    }
    true
}

fn budget_split(c: &mut Ctx, nextid: &mut i32) {
    let n = c.n();
    // last consumer 
    let mut last: HashMap<usize, usize> = HashMap::new();
    for (i, nd) in c.nodes.iter().enumerate() {
        if !nd.is_func {
            continue;
        }
        for k in 0..nd.args.len() {
            if c.is_img_arg(nd, k) {
                last.insert(nd.args[k], i);
            }
        }
    }
    let mut s = 0usize;
    while s < n {
        let e = c.run_end(s);
        if c.nodes[s].g >= 0 && e >= s + 2 {
            let mut best = 0i32;
            let mut bestidx: i64 = -1;
            let mut cab = 0i32;
            let mut created = 0i32;
            let mut freed = 0i32;
            for idx in s..e - 1 {
                let nd = c.nodes[idx].clone();
                if nd.g >= 0 && local(c, nd.id) {
                    created += size_of(&nd);
                }
                if nd.is_func {
                    for k in 0..nd.args.len() {
                        if !c.is_img_arg(&nd, k) {
                            continue;
                        }
                        let aid = nd.args[k];
                        if last.get(&aid) != Some(&idx) {
                            continue;
                        }
                        let ap = c.pos(aid).unwrap();
                        let an = c.nodes[ap].clone();
                        if an.g < 0 || !local(c, aid) {
                            freed += size_of(&an);
                        }
                    }
                }
                let d = freed - created;
                if d > best {
                    best = d;
                    bestidx = idx as i64;
                    cab = created;
                }
            }
            if std::env::var("LG_TRACE").is_ok() {
                let tr: Vec<String> = (s..e).map(|t| format!("{}:{}:g{}:el{}f{}", c.nodes[t].id, c.nodes[t].prim, c.nodes[t].g, c.nodes[t].elnum, c.nodes[t].fmt)).collect();
                eprintln!("LGTRACE	run={}..{}	best={}	bestidx={}	created={}	freed={}	{}", s, e, best, bestidx, created, freed, tr.join(" "));
            }
            if best > 6 && bestidx >= 0 {
                let from = bestidx as usize + 1;
                if e > from {
                    let nid = *nextid;
                    *nextid += 1;
                    for t in from..e {
                        c.nodes[t].g = nid;
                    }
                }
            }
        }
        s = e;
    }
}
