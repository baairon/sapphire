//! NV-3d: the CPU-driver plan (compile argument r8 = 0, driver), natively.
//!
//! Labels: MEASURED = read off the 60 original r8 = 0 dumps in `orig_r0\` (EP-5 `ep5_cplan.py` harness with the compile
//! argument r8 = 0, program flag byte `prog`STATIC, copied per node to `node`).
//! FITTED = rule chosen while looking at those 60 dumps (not a prediction until the held-out batch is scored).
//! UNKNOWN = not reproduced. The run list (primitives, arguments, constant lanes) is the same as in the gpu-first mode;
//! this file only decides the LAYOUT: which commands go into an `exec-linewise` block, where images are allocated and freed,
//! which image a command writes (in place or not), the command flag, and the trailing cleanup records.
//!
//! Model (see README.md of this folder):
//! 1. Node classes. LINEWISE (L): convert-from-source inputs, materialised constants, and every primitive except the
//!    NONLINEWISE set (blur, sharpen, edge_aware_sharpen, glow, z_glow, multiwarp, get_frame, get_prev_dest, save_prev_dest;
//!    MEASURED as the only primitives seen at top level in 60 plans; the original decides this per class through a virtual
//!    call (pass, class slot with argument 1), so classes not in the dumps are UNKNOWN).
//! 2. Schedule (FITTED, the original orders by the group pass whose budget and merge rules are NOT ported):
//!    walk the node list; an N node is its own command; an L node opens a block that also takes later L nodes whose arguments
//!    are all done or in the block, when the node is attached to the block (has an argument in it) or no N node was skipped
//!    since the block opened.
//! 3. Images: one image per node (the node's compiler index is the id, format 3 for make_mipmap outputs, 2 otherwise,
//!    `cpu` token, no `g`). An image whose producer and all consumers sit in one block is BLOCK-LOCAL (allocated and freed in
//!    the block's lists in production order); every other image is allocated at top level right before its producing command
//!    and freed right after its last consumer command in consumer argument order. IN PLACE (FITTED): a node writes into the image
//!    of an argument that is top level, whose last use is this node and that has the same element count; block-local images are never reused.
//! 4. `exec-linewise a a`: a = 1 when the block holds convert-to-dest, else 0.
//! 5. One cleanup record (the recorded host cleanup command name, judge_keys::PLAN_CLEANUP_CMD) per allocate-images, at the end.
//! 6. Command flag (FITTED, shared with the F13 flag rule that NV-3c ports): 0 when the node feeds, through any chain, a
//!    non-linewise node (save_prev_dest excluded) or a warp family node; 1 otherwise.

use crate::compile::CompiledProgram;
use crate::plan::fmt_g6;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanMode {
    /// compile r8 = 1, driver (EP-3 EP-4 plans)
    GpuFirst,
    /// compile r8 = 0, driver (W120 target)
    CpuDriver,
}

/// AG1: argument positions that only donate a type (fold.rs `can_fold` exemption): coerce_up and coerce_to arg 1, v3_to_v4_if4 arg 2 and up.
pub fn donor_pos(prim: &str, k: usize) -> bool {
    match prim.strip_suffix("_if_needed").unwrap_or(prim) {
        "coerce_up" | "coerce_to" => k >= 1,
        "v3_to_v4_if4" => k >= 2,
        // NV-3f (class E): argument 0 of time_warp is the CLIP of the input (original text `time_warp (clip 0 4) f shift`), not an image dependency: MEASURED in TimeDisplace a and b
        "time_warp" => k == 0,
        _ => false,
    }
}

/// a donor that is an input or special variable (not a computed node): no image dependency, printed as the -999 constant (AG1, MEASURED on
/// Time_Warp_RGB_Autogen: coerce_up arg 1 = Source prints `[ -999 -999 -999 -999 ]`, no convert-from-source)
pub(crate) fn is_donor_arg(prim: &str, k: usize, kind: &str) -> bool {
    !old("DONOR") && donor_pos(prim, k) && matches!(kind, "input" | "special")
}

/// DS1: the exec-downsampled factor of a compiled program: 2^blur_res when the program has a constant node named `blur_res` (popup
/// Full / Half / Quarter, record range 0..2, MEASURED in AG1 and NEG2), else 1. `NV3E_DS=old` switches the rule off (NV-3e layout).
pub fn ds_factor(cp: &CompiledProgram) -> u32 {
    if old("DS") {
        return 1;
    }
    for n in &cp.nodes {
        if n.kind == "const" && n.name == "blur_res" {
            let v = f32::from_bits(n.lanes[0]);
            if v.is_finite() && v >= 1.0 {
                return 1u32 << (v.min(2.0) as u32);
            }
            return 1;
        }
    }
    1
}

/// FLD1 (INFERRED from DSW4's set_arg_const read and the -0 into blur_warp argument 6 of rc and bc): a constant lane bound to an INTEGER
/// typed class argument ([desc] = 1) is stored as cvttss2si(lane) (truncation toward zero; -0 becomes +0), so the plan shows the truncated integer.
/// The pairs are defined in effects_rtdata::classes::INT_TYPED_ARGS (from the descriptor dumps). Switch (mixed typing between instances) is excluded by the table.
pub fn int_typed_arg(prim: &str, k: usize) -> bool {
    if std::env::var_os("FLD1_NO_INTTYPE").is_some() {
        return false;
    }
    effects_rtdata::int_typed_arg(prim.strip_suffix("_if_needed").unwrap_or(prim), k)
}

/// DSW4: argument `k` of `prim` is image only (descriptor kind field = 1, constant field = 0)
pub fn image_only_arg(prim: &str, k: usize) -> bool {
    effects_rtdata::img_only_arg(prim.strip_suffix("_if_needed").unwrap_or(prim), k)
}

pub fn nonlinewise(prim: &str) -> bool {
    matches!(prim.strip_suffix("_if_needed").unwrap_or(prim), "blur" | "sharpen" | "edge_aware_sharpen" | "glow" | "z_glow" | "multiwarp" | "get_frame" | "get_prev_dest" | "save_prev_dest")
}

/// classes seen writing in place in the r8 = 0 and r8 = 1 dumps (FITTED allow list; interp never did)
fn inplace_ok(prim: &str) -> bool {
    matches!(prim.strip_suffix("_if_needed").unwrap_or(prim), "*" | "+" | "-" | "/" | "abs" | "blur" | "get_prev_dest" | "hue_shift" | "hue_shift_preserve_luma" | "max" | "min" | "ncos" | "one_minus" | "sat_scale" | "save_prev_dest" | "screen_super" | "sharpen")
}

fn random_access(prim: &str) -> bool {
    matches!(prim, "warp_rel" | "warp" | "blur_warp" | "chroma_warp" | "multiwarp")
}


/// FLD1 (INFERRED from the node rows of cd, md and ta of RNG1's batch, plus SHE3's h3): when a folded FUNCTION constant is bound to an image-only
/// class argument (DSW4's rule), the original appends a copy node of the same class and lanes (kind 2, flag set) that stands as the materialised
/// image. This returns the lanes of those copies, one per (consumer, argument) binding. Parameter constants (kind 1) are not counted.
pub fn materialised_copies(cp: &CompiledProgram) -> Vec<[u32; 4]> {
    let nodes = &cp.nodes;
    let mut v = Vec::new();
    // INT4 reconcile (W-DSW5): the (consumer, argument) copies are the clone nodes of the compile pass (cp.clones); only the root copy stays here (FLD1).
    // the root: convert-to-dest reads an image (MEASURED ca and ma: root alias to a folded blur, copy node 103 appended; param roots (n2, f10) are kind 1 and not counted)
    let r = &nodes[cp.root];
    if r.kind == "const" && r.folded && !r.cls.is_empty() {
        v.push(r.lanes);
    }
    v
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum IK {
    Input,
    Const,
    Run,
    Dest,
}

#[derive(Clone, Debug)]
struct Item {
    k: IK,
    n: usize,
    lw: bool,
    deps: Vec<usize>, // item indices (images read)
    /// the id printed in `:refN` (the node index; the DS1 duplicate of the xy variable gets a fresh one)
    rid: usize,
    /// DS1: (node argument, item) replacement of an argument by the duplicate xy item
    alias: Option<(usize, usize)>,
}

/// NV-3g rule M (RULE-INFERRED, DigitalDamage nf1 and nf2 against Rays, Warp_Chroma, Warp_Perspective, InfiniteZoom): a make_mipmap image has the mip-chain format 3 only when a consumer reads it at a random access position (warp family); a mipmap used only linewise (DigitalDamage: diff, copy_alpha in the same block) is format 2 like every other image
fn fcode_n(cp: &CompiledProgram, idx: usize) -> u32 {
    let nd = &cp.nodes[idx];
    // WPG12 PATCH 1 (FORMAT-READ, pass): the format comes from the propagation pass; rule M is the named ablation WPG12_OLDFMT=1 (off in counted runs)
    if nd.prim == "make_mipmap" && idx != cp.root && std::env::var_os("WPG12_OLDFMT").is_none() {
        return fmt_pass(cp)[idx];
    }
    if nd.prim == "make_mipmap" && idx != cp.root {
        let ra = cp.nodes.iter().any(|m| (m.kind == "func" || m.kind == "auto") && m.args.iter().enumerate().any(|(k, &a)| a == idx && (random_access(m.prim.strip_suffix("_if_needed").unwrap_or(&m.prim)) || crate::lgpass::random_access(&m.prim, k as i64))));
        return if ra { 3 } else { 2 };
    }
    2
}

/// WPG12 (READ): the format propagation pass (walk: preorder, children in argument order, a node is marked visited after its
/// children). Slot 22 (vtable) of the consumer class gives the format of argument k from the consumer's own format; slot 29 says
/// whether a class accepts a format. CPU path only (= 0). Classes: make_mipmap slot22 (k = 0 gives 2, else 3), slot29 
/// (2 or 3); the warp family slot22 (k = 0 gives 3, else the
/// consumer format); every other class of the in-scope programs uses the base slots (the consumer format) and (accepts 2).
/// A node whose gate fails, and the root, is reset to the best accepted format (2).
fn fmt_s22(prim: &str, k: usize, f: u32) -> u32 {
    match prim {
        "make_mipmap" => if k == 0 { 2 } else { 3 },
        "warp" | "warp_no_pass_arg" | "warp_rel" | "multiwarp" | "repeat_warp" | "repeat_warp_filter" | "chroma_warp" | "chroma_warp_filter"
        | "chroma_warp_dod_hack" | "flys_eye" | "blur_warp" | "blur_warp_bidirectional" => if k == 0 { 3 } else { f },
        _ => f,
    }
}

fn fmt_s29(prim: &str, f: u32) -> bool {
    if prim == "make_mipmap" { f == 2 || f == 3 } else { f == 2 }
}

pub(crate) fn fmt_pass_pub(cp: &CompiledProgram) -> Vec<u32> {
    fmt_pass(cp)
}

pub(crate) fn fmt_s22_pub(prim: &str, k: usize, f: u32) -> u32 {
    fmt_s22(prim, k, f)
}

fn fmt_pass(cp: &CompiledProgram) -> Vec<u32> {
    let kinds: Vec<u8> = cp.nodes.iter().map(|nd| match nd.kind { "input" => 1, "special" | "const" => 2, _ => 0 }).collect();
    let prims: Vec<String> = cp.nodes.iter().map(|nd| nd.prim.clone()).collect();
    let args: Vec<Vec<usize>> = cp.nodes.iter().map(|nd| nd.args.clone()).collect();
    fmt_pass_gen(cp.root, &kinds, &prims, &args)
}

/// kinds: 0 function, 1 input leaf, 2 other leaf (special or constant)
pub fn fmt_pass_gen(root: usize, kinds: &[u8], prims: &[String], args: &[Vec<usize>]) -> Vec<u32> {
    let n = kinds.len();
    let mut fmt = vec![0u32; n];
    let mut visited = vec![false; n];
    fn visit(i: usize, parent: bool, kinds: &[u8], prims: &[String], args: &[Vec<usize>], fmt: &mut Vec<u32>, visited: &mut Vec<bool>) {
        if i >= fmt.len() || visited[i] {
            return;
        }
        let gate = parent && match kinds[i] {
            0 => fmt_s29(&prims[i], fmt[i]),
            1 => fmt[i] == 2,
            _ => fmt[i] == 1 || fmt[i] == 2,
        };
        if !gate {
            fmt[i] = 2;
        }
        if kinds[i] == 0 {
            for (k, &a) in args[i].iter().enumerate() {
                if a < fmt.len() && !visited[a] {
                    fmt[a] = fmt_s22(&prims[i], k, fmt[i]);
                }
            }
            for &a in args[i].iter() {
                visit(a, true, kinds, prims, args, fmt, visited);
            }
        }
        visited[i] = true;
    }
    visit(root, false, kinds, prims, args, &mut fmt, &mut visited);
    fmt
}


fn fcode(prim: &str, root: bool) -> u32 {
    if prim == "make_mipmap" && !root {
        3
    } else {
        2
    }
}

pub fn plan_text_cpu(cp: &CompiledProgram) -> String {
    let nodes = &cp.nodes;
    let nn = nodes.len();
    let is_out = |i: usize| nodes[i].kind == "special" && nodes[i].name == "Output";
    let is_func = |i: usize| !matches!(nodes[i].kind, "input" | "special" | "const");
    // materialised constants (DSW4, rule READ + MEASURED): a constant argument bound to a class argument whose descriptor does not allow a scalar
    // ([desc field] == 0, image only; set_arg_const refuses it) is materialised as a `constant` image command.
    // Table data/IMG_ONLY_ARGS.tsv comes from the descriptors of every class object after the original compile (tools/desc_dump.py).
    let mut mat = vec![false; nn];
    for n in nodes.iter() {
        if is_func(n.idx) {
            for (k, &a) in n.args.iter().enumerate() {
                if nodes[a].kind == "const" && image_only_arg(&n.prim, k) {
                    mat[a] = true;
                }
            }
        }
    }
    // DSW3 (MEASURED shape, Drop_Shadow n2: the whole program collapses to the constant [0 0 0]): a constant ROOT is materialised as a constant image that
    // convert-to-dest reads (original: allocate, (constant [ .. ] :ref), convert-to-dest, one exec-linewise command)
    if nodes[cp.root].kind == "const" {
        mat[cp.root] = true;
    }
    // items
    let mut items: Vec<Item> = Vec::new();
    let mut item_of: HashMap<usize, usize> = HashMap::new();
    // DS1: hoisted nodes (FITTED, see `ds_hoist`) run at full resolution before the exec-downsampled command, on their own copy of the xy variable
    let hoist: Vec<usize> = if ds_factor(cp) > 1 { ds_hoist(cp) } else { Vec::new() };
    let xy_idx: Option<usize> = nodes.iter().find(|m| m.kind != "func" && m.kind != "auto" && m.kind != "const" && m.name == "xy").map(|m| m.idx);
    let dup_rid = nodes.len() + 1000;
    let mut dup_item: Option<usize> = None;
    let order: Vec<(usize, bool)> = if hoist.is_empty() {
        nodes.iter().map(|m| (m.idx, false)).collect()
    } else {
        let is_in = |m: &crate::compile::PlanNode| matches!(m.kind, "input" | "special") && m.name != "xy" && !is_out(m.idx);
        let mut o: Vec<(usize, bool)> = nodes.iter().filter(|m| is_in(m)).map(|m| (m.idx, false)).collect();
        o.push((xy_idx.unwrap_or(0), true));
        o.extend(nodes.iter().filter(|m| hoist.contains(&m.idx)).map(|m| (m.idx, false)));
        o.extend(nodes.iter().filter(|m| !is_in(m) && !hoist.contains(&m.idx)).map(|m| (m.idx, false)));
        o
    };
    for &(nidx, isdup) in order.iter() {
        let n = &nodes[nidx];
        if is_out(n.idx) {
            continue;
        }
        if isdup {
            dup_item = Some(items.len());
            item_of.insert(dup_rid, items.len());
            items.push(Item { k: IK::Input, n: n.idx, lw: true, deps: vec![], rid: dup_rid, alias: None });
            continue;
        }
        match n.kind {
            "input" | "special" => {
                // an input read only by get_frame is a clip descriptor `(clip slot elnum)`, not an image (MEASURED, Time_Warp_RGB)
                // AG1: an input used only as a donor argument (coerce_up arg 1, v3_to_v4_if4 arg 2) is not an image either
                let only_clip = nodes.iter().filter(|m| is_func(m.idx) && m.args.contains(&n.idx)).all(|m| m.prim == "get_frame" || (!old("DONOR") && m.args.iter().enumerate().all(|(k, &a)| a != n.idx || donor_pos(&m.prim, k)))) && nodes.iter().any(|m| is_func(m.idx) && m.args.contains(&n.idx));
                if only_clip {
                    continue;
                }
                item_of.insert(n.idx, items.len());
                items.push(Item { k: IK::Input, n: n.idx, lw: true, deps: vec![], rid: n.idx, alias: None });
            }
            "const" => {
                if mat[n.idx] {
                    item_of.insert(n.idx, items.len());
                    items.push(Item { k: IK::Const, n: n.idx, lw: true, deps: vec![], rid: n.idx, alias: None });
                }
            }
            _ => {
                let hoisted = hoist.contains(&n.idx);
                let alias = if hoisted && xy_idx.is_some() && dup_item.is_some() && n.args.contains(&xy_idx.unwrap()) { Some((xy_idx.unwrap(), dup_item.unwrap())) } else { None };
                let deps: Vec<usize> = n
                    .args
                    .iter()
                    .enumerate()
                    .filter(|(k, &a)| !(is_donor_arg(&n.prim, *k, nodes[a].kind)))
                    .filter_map(|(_, a)| match alias {
                        Some((x, d)) if *a == x => Some(d),
                        _ => item_of.get(a).copied(),
                    })
                    .collect();
                item_of.insert(n.idx, items.len());
                items.push(Item { k: IK::Run, n: n.idx, lw: !nonlinewise(&n.prim), deps, rid: n.idx, alias });
            }
        }
    }
    // STK2 (stack word 2 = 1): the preserve-group model (preserve.rs); programs it does not cover run the word 2 = 0 path (plancmp refuses them by name)
    let pres: Option<crate::preserve::Pres> = if cp.ctx_word2 && crate::preserve::model_refusal(cp).is_none() { Some(crate::preserve::analyze(cp, cp.ctx_has_data)) } else { None };
    let mut mark = mark_set(cp);
    if let Some(p) = pres.as_ref() {
        mark.extend(p.mark.iter().copied());
    }
    let root = &nodes[cp.root];
    {
        if let Some(&ri) = item_of.get(&root.idx) {
            items.push(Item { k: IK::Dest, n: root.idx, lw: true, deps: vec![ri], rid: root.idx, alias: None });
        }
    }
    let ni = items.len();
    // flag: feeds a non-linewise (not save_prev_dest) or warp family node
    let mut feeds_rand = vec![false; nn];
    for _ in 0..nn {
        let mut ch = false;
        for n in nodes.iter() {
            if !is_func(n.idx) {
                continue;
            }
            let p = n.prim.strip_suffix("_if_needed").unwrap_or(&n.prim);
            let hits = (nonlinewise(p) && p != "save_prev_dest") || random_access(p);
            for &a in &n.args {
                if (hits || feeds_rand[n.idx]) && !feeds_rand[a] {
                    feeds_rand[a] = true;
                    ch = true;
                }
            }
        }
        if !ch {
            break;
        }
    }
    // NV-3e: schedule = the ported linewise pass (lgpass.rs) on the node array in compile order
    let mut lnodes: Vec<crate::lgpass::LNode> = Vec::new();
    for it in items.iter() {
        if it.k == IK::Dest {
            continue;
        }
        let nd = &nodes[it.n];
        lnodes.push(crate::lgpass::LNode {
            id: it.rid,
            // DSW4 (f03, MEASURED + READ): a FOLDED function node keeps node kind 2 and its class in the original (NODE row of the f03 mocha_render
            // node 88: kind=2, b160=1), so pass 1 asks its class flag word: mocha_render (flags 0xb1, bit 0 set) is not linewise and the materialised constant stands
            // in its own command, not in the block of its consumer
            is_func: it.k == IK::Run || (it.k == IK::Const && nd.folded && !nd.cls.is_empty()),
            prim: if it.k == IK::Run { nd.prim.clone() } else if it.k == IK::Const && nd.folded { nd.cls.clone() } else { String::new() },
            elnum: nd.elnum.max(1),
            // NV-3g rule R (RULE-FITTED, Rays b/nf1/nf2; H1 [best > created] was tried and REFUTED by MASK1 Transform_and_Blur_Autogen mk4_m1): the xy variable image counts as format 4 (size el * 4) in the memory budget of the linewise pass
            fmt: if nd.kind == "special" && nd.name == "xy" { 4 } else { fcode_n(cp, nd.idx) as u8 },
            args: if it.k == IK::Run { nd.args.iter().enumerate().filter(|(k, &a)| !is_donor_arg(&nd.prim, *k, nodes[a].kind)).map(|(_, &a)| match it.alias { Some((x, _)) if a == x => dup_rid, _ => a }).collect() } else { vec![] },
            g: -1,
        });
    }
    let lkeys: HashMap<usize, i64> = match pres.as_ref() {
        Some(p) => items.iter().filter(|it| it.k != IK::Dest && p.key[it.n] >= 0).map(|it| (it.rid, p.key[it.n])).collect(),
        None => HashMap::new(),
    };
    let lres = crate::lgpass::run_pass_keys(lnodes, lkeys);
    let mut cmds: Vec<(bool, Vec<usize>)> = Vec::new(); // (is_block, items)
    {
        let mut k = 0usize;
        while k < lres.len() {
            let g = lres[k].g;
            let mut e = k + 1;
            if g >= 0 {
                while e < lres.len() && lres[e].g == g {
                    e += 1;
                }
            }
            let its: Vec<usize> = (k..e).map(|t| *item_of.get(&lres[t].id).unwrap()).collect();
            cmds.push((g >= 0, its));
            k = e;
        }
        // convert-to-dest joins the block of the root node when that block is the last command, else it stands alone
        if let Some(di) = (0..ni).find(|&i| items[i].k == IK::Dest) {
            let ri = items[di].deps[0];
            match cmds.last_mut() {
                Some((true, its)) if its.contains(&ri) => its.push(di),
                _ => cmds.push((true, vec![di])),
            }
        }
    }
    // DS1: exec-downsampled. The convert-from-source inputs and the convert-to-dest stay at full resolution OUTSIDE the command (MEASURED
    // T09 / B09 / AG1 a2 / NEG2 plans); everything else is the body of the command, scheduled as before.
    let dsf = ds_factor(cp);
    let cmds_full = cmds.clone();
    let mut ds_dest: Option<usize> = None;
    let mut kpre = 0usize;
    // DS3: the tail items leave the command body (see `ds_tail_split`)
    let (tail_nodes, body_root_node) = ds_tail_split(cp);
    let tail_items: Vec<usize> = (0..ni).filter(|&i| items[i].k == IK::Run && tail_nodes.contains(&items[i].n)).collect();
    // items that run at full resolution before the command: the convert-from-source inputs, the duplicate xy variable, the hoisted nodes
    let pre_set: Vec<usize> = (0..ni).filter(|&i| (items[i].k == IK::Input && nodes[items[i].n].name != "xy") || (items[i].k == IK::Input && items[i].rid != items[i].n) || (items[i].k == IK::Run && hoist.contains(&items[i].n))).collect();
    if dsf > 1 {
        ds_dest = (0..ni).find(|&i| items[i].k == IK::Dest);
        let mut pre_l: Vec<(bool, Vec<usize>)> = Vec::new();
        let mut reg_l: Vec<(bool, Vec<usize>)> = Vec::new();
        for (blk, its) in cmds.iter() {
            let p: Vec<usize> = its.iter().copied().filter(|i| pre_set.contains(i)).collect();
            let r: Vec<usize> = its.iter().copied().filter(|i| !pre_set.contains(i) && Some(*i) != ds_dest && !tail_items.contains(i)).collect();
            if !p.is_empty() {
                pre_l.push((*blk, p));
            }
            if !r.is_empty() {
                reg_l.push((*blk, r));
            }
        }
        kpre = pre_l.len();
        pre_l.extend(reg_l);
        cmds = pre_l;
    }
    // block of each item
    let mut cmd_of = vec![usize::MAX; ni];
    for (ci, (_, its)) in cmds.iter().enumerate() {
        for &i in its {
            cmd_of[i] = ci;
        }
    }
    // consumers
    let mut cons: Vec<Vec<usize>> = vec![vec![]; ni];
    for (i, it) in items.iter().enumerate() {
        for &d in &it.deps {
            cons[d].push(i);
        }
    }
    let mut local = vec![false; ni];
    for i in 0..ni {
        if matches!(items[i].k, IK::Dest) || tail_items.contains(&i) {
            continue;
        }
        local[i] = items[i].lw && cmds[cmd_of[i]].0 && !cons[i].is_empty() && cons[i].iter().all(|&c| cmd_of[c] == cmd_of[i]) && crate::lgpass::is_group_local(&lres, items[i].rid);
    }
    let mut gpos = vec![0usize; ni];
    {
        let mut c = 0;
        for (_, its) in cmds.iter() {
            for &i in its {
                gpos[i] = c;
                c += 1;
            }
        }
    }
    let mut lastc = vec![0usize; ni];
    for i in 0..ni {
        for &d in &items[i].deps {
            if gpos[i] > lastc[d] {
                lastc[d] = gpos[i];
            }
        }
    }
    // STK2: a member of a preserve group is never reused in place and its image is never reused (ASSUMED: the preserved image must survive)
    let pm_item = |i: usize| pres.as_ref().map_or(false, |p| p.key[items[i].n] >= 0);
    let is_head = |i: usize| pres.as_ref().map_or(false, |p| p.head[items[i].n]);
    // storage with in place
    let mut storage: Vec<usize> = (0..ni).collect();
    let mut remaining: Vec<usize> = (0..ni).map(|i| cons[i].len()).collect();
    for (_, its) in cmds.iter() {
        for &i in its {
            if items[i].k == IK::Run {
                let me = &nodes[items[i].n];
                for &d in &items[i].deps {
                    let s = storage[d];
                    let occ = items[i].deps.iter().filter(|&&x| storage[x] == s).count();
                    if !local[s] && !local[i] && !pm_item(i) && !pm_item(s) && items[d].k != IK::Const && (if old("LASTC") { remaining[s] == occ } else { lastc[d] == gpos[i] }) && (nodes[items[d].n].elnum == me.elnum || me.prim == "save_prev_dest") && inplace_rule(&me.prim) {
                        storage[i] = s;
                        break;
                    }
                }
            }
            for &d in &items[i].deps {
                let s = storage[d];
                remaining[s] = remaining[s].saturating_sub(1);
            }
        }
    }
    // last consumer command of each storage
    let mut last_cmd: HashMap<usize, usize> = HashMap::new();
    for (i, it) in items.iter().enumerate() {
        for &d in &it.deps {
            let e = last_cmd.entry(storage[d]).or_insert(0);
            if cmd_of[i] > *e {
                *e = cmd_of[i];
            }
        }
    }
    let rf = |s: usize| format!(":ref{}.{}", items[s].rid, fcode_n(cp, nodes[items[s].n].idx));
    let alloc_txt = |s: usize| {
        let nd = &nodes[items[s].n];
        format!("({} :ref{}.{} {} {} cpu)", if is_head(s) { "allocate-images-preserved" } else { "allocate-images" }, items[s].rid, fcode_n(cp, nd.idx), nd.elnum.max(1), fcode_n(cp, nd.idx))
    };
    // flags and body command text
    let mut slot = 0;
    let mut slot_of: HashMap<usize, usize> = HashMap::new();
    for n in nodes.iter() {
        if matches!(n.kind, "input" | "special") && !is_out(n.idx) {
            // UNKNOWN input numbering (F13): Back is host input 1 in Composite (MEASURED), the rest count up from 0
            if !old("SLOT") {
                slot_of.insert(n.idx, crate::layout::input_slot(&n.name, n.input_pos));
            } else if n.name == "Back" {
                slot_of.insert(n.idx, 1);
            } else {
                slot_of.insert(n.idx, slot);
                slot += 1;
            }
        }
    }
    let body = |i: usize| -> String {
        let it = &items[i];
        let nd = &nodes[it.n];
        match it.k {
            IK::Input => {
                if nd.name == "xy" {
                    format!("(variable 2 {})", rf(i))
                } else {
                    format!("(convert-from-source {} {})", slot_of.get(&nd.idx).copied().unwrap_or(0), rf(i))
                }
            }
            IK::Const => {
                let l: Vec<String> = (0..nd.elnum.clamp(1, 4) as usize).map(|k| fmt_g6(f32::from_bits(nd.lanes[k]) as f64)).collect();
                format!("(constant [ {} ] {})", l.join(" "), rf(i))
            }
            IK::Dest => format!("(convert-to-dest {})", rf(storage[it.deps[0]])),
            IK::Run => {
                let mut a: Vec<String> = Vec::new();
                for (k, &x) in nd.args.iter().enumerate() {
                    let an = &nodes[x];
                    if nd.prim.strip_suffix("_if_needed").unwrap_or(&nd.prim) == "time_warp" && k == 0 && matches!(an.kind, "input" | "special") {
                        a.push(format!("(clip {} {})", slot_of.get(&an.idx).copied().unwrap_or(0), an.elnum));
                    } else if is_donor_arg(&nd.prim, k, an.kind) && !(an.kind == "special" && an.name == "Output") {
                        // AG1 DONOR: the type donor prints as a constant -999 per channel of the donor and is not an image dependency
                        a.push(format!("[ {} ]", vec!["-999"; an.elnum.clamp(1, 4) as usize].join(" ")));
                    } else if an.kind == "special" && an.name == "Output" {
                        a.push("[ -999 ]".to_string());
                    } else if let Some(&ai) = (match it.alias { Some((ax, ad)) if x == ax => Some(ad), _ => item_of.get(&x).copied() }).as_ref() {
                        a.push(rf(storage[ai]));
                    } else if matches!(an.kind, "input" | "special") {
                        a.push(format!("(clip {} {})", slot_of.get(&an.idx).copied().unwrap_or(0), an.elnum));
                    } else {
                        let ity = int_typed_arg(&nd.prim, k);
                        let l: Vec<String> = (0..an.elnum.clamp(1, 4) as usize)
                            .map(|j| {
                                let mut x = f32::from_bits(an.lanes[j]);
                                if ity {
                                    x = crate::compile::cvtt_i32(x) as f32;
                                }
                                fmt_g6(x as f64)
                            })
                            .collect();
                        a.push(format!("[ {} ]", l.join(" ")));
                    }
                }
                let flag = if (if old("FLAG") { feeds_rand[nd.idx] } else { mark.contains(&nd.idx) }) { 0 } else { 1 };
                format!("(run-primitive {} -1 {} {} {} )", flag, rf(storage[i]), nd.prim.strip_suffix("_if_needed").unwrap_or(&nd.prim), a.join(" "))
            }
        }
    };
    // emit
    let mut out: Vec<String> = Vec::new();
    let mut n_alloc = 0usize;
    let mut last_use: HashMap<usize, usize> = HashMap::new();
    for i in 0..ni {
        for &d in &items[i].deps {
            let e = last_use.entry(storage[d]).or_insert(0);
            if gpos[i] > *e {
                *e = gpos[i];
            }
        }
    }
    let mut freed: HashMap<usize, bool> = HashMap::new();
    let mut allocated: HashMap<usize, bool> = HashMap::new();
    // DS1: images allocated outside the exec-downsampled command: the convert-from-source inputs and the destination of the command
    let ds_dest_store: Option<usize> = match (ds_dest, body_root_node.and_then(|b| item_of.get(&b).copied())) {
        (Some(_), Some(bi)) if !tail_items.is_empty() => Some(storage[bi]),
        (Some(d), _) => Some(storage[items[d].deps[0]]),
        _ => None,
    };
    let mut bounds: Vec<usize> = Vec::new();
    if dsf > 1 {
        if let Some(s) = ds_dest_store {
            allocated.insert(s, true);
        }
    }
    // STK2: the group id of every command (uniform by construction of the keyed pass); a group split into several runs of commands is not wrapped
    let pgk: Vec<i64> = cmds
        .iter()
        .map(|(_, its)| {
            let ks: Vec<i64> = its.iter().filter(|&&i| items[i].k != IK::Dest).map(|&i| pres.as_ref().map_or(-1, |p| p.key[items[i].n])).collect();
            if !ks.is_empty() && ks.iter().all(|&k| k == ks[0]) { ks[0] } else { -1 }
        })
        .collect();
    let mut pwrap: std::collections::HashSet<i64> = std::collections::HashSet::new();
    for &g in pgk.iter().filter(|&&g| g >= 0) {
        let runs = (0..cmds.len()).filter(|&c| pgk[c] == g && (c == 0 || pgk[c - 1] != g)).count();
        if runs == 1 {
            pwrap.insert(g);
        }
    }
    let mut pstart: HashMap<i64, usize> = HashMap::new();
    for (ci, (is_block, its)) in cmds.iter().enumerate() {
        if pgk[ci] >= 0 && (ci == 0 || pgk[ci - 1] != pgk[ci]) {
            pstart.insert(pgk[ci], out.len());
        }
        // top level allocations of images produced here
        for &i in its {
            if items[i].k == IK::Dest || storage[i] != i {
                continue;
            }
            if !local[i] && !allocated.contains_key(&i) {
                allocated.insert(i, true);
                out.push(alloc_txt(i));
                n_alloc += 1;
            }
        }
        if *is_block {
            let mut al: Vec<String> = Vec::new();
            let mut fr: Vec<String> = Vec::new();
            let mut bd: Vec<String> = Vec::new();
            let mut has_dest = false;
            for &i in its {
                if items[i].k == IK::Dest {
                    has_dest = true;
                } else if local[i] && storage[i] == i {
                    al.push(alloc_txt(i));
                    n_alloc += 1;
                }
                bd.push(body(i));
            }
            let mut fdone: std::collections::HashSet<usize> = std::collections::HashSet::new();
            for &i in its {
                for &d in &items[i].deps {
                    let s = storage[d];
                    if local[s] && last_use.get(&s) == Some(&gpos[i]) && fdone.insert(s) {
                        fr.push(format!("(free-images {})", rf(s)));
                    }
                }
            }
            let h = if has_dest { "1 1" } else { "0 0" };
            let wrap = |v: &Vec<String>| if v.is_empty() { "( )".to_string() } else { format!("( {}\\n  )", v.join("\\n    ")) };
            out.push(format!("(exec-linewise {}\\n  {}\\n  {}\\n  {}\\n)", h, wrap(&al), wrap(&bd), wrap(&fr)));
        } else {
            out.push(body(its[0]));
        }
        // top level frees after the last consumer command
        for &i in its {
            for &d in &items[i].deps {
                let s = storage[d];
                if !local[s] && last_cmd.get(&s) == Some(&ci) && !freed.contains_key(&s) {
                    freed.insert(s, true);
                    out.push(format!("({} {})", if is_head(s) { "unlock-preserved-images" } else { "free-images" }, rf(s)));
                }
            }
        }
        // STK2: close a preserve container after the last command of its group (the group's commands are collected in emission order)
        if pgk[ci] >= 0 && (ci + 1 == cmds.len() || pgk[ci + 1] != pgk[ci]) && pwrap.contains(&pgk[ci]) {
            if let (Some(&s0), Some(h)) = (pstart.get(&pgk[ci]), (0..ni).find(|&i| is_head(i) && pres.as_ref().map_or(false, |p| p.key[items[i].n] == pgk[ci]))) {
                let inner = out.split_off(s0);
                out.push(crate::preserve::wrap_text(&rf(h), &inner));
            }
        }
        bounds.push(out.len());
    }
    if dsf > 1 {
        // top level: allocate and convert each input, allocate the destination, the command around the body, convert-to-dest, free
        let wrap1 = |v: &Vec<String>| if v.is_empty() { "( )".to_string() } else { format!("( {}\\n  )", v.join("\\n    ")) };
        let none: Vec<String> = Vec::new();
        let split = if kpre == 0 { 0 } else { bounds[kpre - 1] };
        let mut top: Vec<String> = out[..split].to_vec();
        let region: Vec<String> = out[split..].to_vec();
        if let Some(st) = ds_dest_store {
            top.push(alloc_txt(st));
            n_alloc += 1;
            // images made before the command and read inside it, in production order
            let srcs: Vec<String> = (0..ni)
                .filter(|&i| storage[i] == i && pre_set.contains(&i) && (0..ni).any(|c| items[c].deps.iter().any(|&d| storage[d] == i) && cmd_of[c] != usize::MAX && cmd_of[c] >= kpre))
                .map(|i| format!("{} ", rf(i)))
                .collect();
            top.push(format!("(exec-downsampled {} {}\\n({})\\n()\\n  ( {}\\n  )\\n)", dsf, rf(st), srcs.join(""), region.join("\\n    ")));
            if tail_items.is_empty() {
                top.push(format!("(exec-linewise 1 1\\n  {}\\n  {}\\n  {}\\n)", wrap1(&none), wrap1(&vec![body(ds_dest.unwrap())]), wrap1(&none)));
            } else {
                // DS3: the tail in its own exec-linewise on fresh images, then convert-to-dest; the command destination is freed afterwards
                let al: Vec<String> = tail_items.iter().map(|&i| alloc_txt(i)).collect();
                let mut bd: Vec<String> = tail_items.iter().map(|&i| body(i)).collect();
                bd.push(body(ds_dest.unwrap()));
                let fr: Vec<String> = tail_items.iter().map(|&i| format!("(free-images {})", rf(i))).collect();
                n_alloc += tail_items.len();
                top.push(format!("(exec-linewise 1 1\\n  {}\\n  {}\\n  {}\\n)", wrap1(&al), wrap1(&bd), wrap1(&fr)));
            }
            top.push(format!("(free-images {})", rf(st)));
        }
        out = top;
    }
    let mut s = String::new();
    s.push_str(&format!("#NV3D\tprogram={}\ttag={}\tmode=cpu-driver\tcommands={}\tnodes={}\n", cp.program, if cp.paramset.is_empty() { "default" } else { &cp.paramset }, out.len() + n_alloc, nn));
    let mut k = 0;
    for c in out.iter() {
        let op = c.trim_start_matches('(').split_whitespace().next().unwrap_or("");
        s.push_str(&format!("CMD\t{}\t{}\t{}\t-\n", k, op, c));
        k += 1;
    }
    for _ in 0..n_alloc {
        s.push_str(&format!("CMD\t{}\t{}\t\t-\n", k, crate::judge_keys::PLAN_CLEANUP_CMD));
        k += 1;
    }
    for n in nodes.iter() {
        let lanes: Vec<String> = n.lanes.iter().map(|b| format!("{:08x}", b)).collect();
        s.push_str(&format!("NODE\t{}\t{}\tkind={}\tprim={}\tout_ch={}\targs={}\tlanes={}\t{}={}\tfolded={}\n", n.idx, n.name, n.kind, if n.prim == "-" { "" } else { &n.prim }, n.elnum, n.args.iter().map(|a| a.to_string()).collect::<Vec<_>>().join(","), lanes.join(""), crate::judge_keys::PLAN_KEY_NODE_FLAG, n.b160 as u8, n.folded as u8));
    }
    // DSW4 (f03, MEASURED on the DSW3 f03 original: nodes 27 and 88 are both `m1` folded to [0]): a folded constant that an image-only argument needs as an image
    // AND that has a consumer at a scalar-capable argument exists twice in the original (the literal node and a clone appended at the end that carries the image).
    // INFERRED: the clone is made only when both kinds of use exist (a sole image-only use keeps the single node: older corpora match with one row).
    {
        let mut extra = nn + 2000;
        for nd in cp.clones.iter().filter(|c| c.folded) {
            let lanes: Vec<String> = nd.lanes.iter().map(|b| format!("{:08x}", b)).collect();
            s.push_str(&format!("NODE\t{}\t{}\tkind=2\tprim=\tout_ch={}\targs=\tlanes={}\t{}=1\tfolded=1\n", extra, nd.name, nd.elnum, lanes.join(""), crate::judge_keys::PLAN_KEY_NODE_FLAG));
            extra += 1;
        }
    }
    if dup_item.is_some() {
        // DS1: the duplicate of the xy variable the hoisted nodes read (id past the node ids), named like its original for the canonical compare
        s.push_str(&format!("NODE\t{}\txy\tkind=special\tprim=\tout_ch=2\targs=\tlanes={}\t{}=0\tfolded=0\n", dup_rid, "0".repeat(32), crate::judge_keys::PLAN_KEY_NODE_FLAG));
    }
    let mut eval: Vec<String> = Vec::new();
    for (_, its) in cmds_full.iter() {
        for &i in its {
            let nd = &nodes[items[i].n];
            match items[i].k {
                IK::Input | IK::Const => eval.push(format!("{}({})", nd.name, items[i].rid)),
                IK::Run => eval.push(format!("_{}({})", nd.prim, nd.idx)),
                IK::Dest => {}
            }
        }
    }
    s.push_str(&format!("EVALTEXT\tEval list:\\n{}\n", eval.join("\\n")));
    s
}

pub fn plan_text_mode(cp: &CompiledProgram, mode: PlanMode) -> String {
    match mode {
        PlanMode::GpuFirst => crate::plan::plan_text(cp),
        PlanMode::CpuDriver => plan_text_cpu(cp),
    }
}

/// NV3E_<NAME>=old selects the NV-3d FITTED rule (used only to measure the before/after of each replaced rule)
fn old(name: &str) -> bool {
    std::env::var(format!("NV3E_{}", name)).map(|v| v == "old").unwrap_or(false)
}

fn inplace_rule(prim: &str) -> bool {
    if old("INPLACE") {
        inplace_ok(prim)
    } else {
        crate::layout::flags_of(prim) & 0x4 != 0
    }
}

/// command flag, propagation (NV-3c, layout.rs): ids of func nodes whose run-primitive flag is 0
fn mark_set(cp: &CompiledProgram) -> std::collections::HashSet<usize> {
    let mut mark = std::collections::HashSet::new();
    loop {
        let mut ch = false;
        for n in &cp.nodes {
            if (n.kind == "func" || n.kind == "auto") && (mark.contains(&n.idx) || crate::layout::flags_of(&n.prim) & 0x2000 != 0) {
                for &a in &n.args {
                    if (cp.nodes[a].kind == "func" || cp.nodes[a].kind == "auto") && !crate::layout::mark_blocked(&n.prim) && mark.insert(a) {
                        ch = true;
                    }
                }
            }
        }
        if !ch {
            break;
        }
    }
    mark
}

/// DS3 (W953): the printed id of the duplicate xy variable that DS1's hoist reads at full size (the plan builder gives it a fresh id above every node index).
pub fn ds_dup_rid(cp: &CompiledProgram) -> usize {
    cp.nodes.len() + 1000
}

/// DS3: the compiler's tail of an exec-downsampled program, read from the program's own node kinds only. The compiler appends its tail nodes
/// with the kind `auto`; the chain from the final root down the first image argument, up to the first node whose kind is not `auto`, is
/// the tail, and that node is the body root. The tail runs after the command, in its own linewise command, on a fresh image (the recorded
/// plans of every exec-downsampled Blur Motion Curves and Transform and Blur row, see the E1 plan_text of f_BR3T_50x7 and the eight others).
/// Returns (tail nodes in execution order, computed and unfolded only; body root node). Empty tail unless the factor is above 1, and also
/// empty when the chain does not end in a computed node. Env DS3_MUT=inbody (mutant) restores the old plan and executor order.
pub fn ds_tail_split(cp: &CompiledProgram) -> (Vec<usize>, Option<usize>) {
    if ds_factor(cp) <= 1 || std::env::var("DS3_MUT").map_or(false, |v| v == "inbody") {
        return (Vec::new(), None);
    }
    let comp = |i: usize| (cp.nodes[i].kind == "func" || cp.nodes[i].kind == "auto") && !cp.nodes[i].b160 && !cp.nodes[i].folded;
    let mut v: Vec<usize> = Vec::new();
    let mut cur = cp.root;
    let mut guard = 0;
    while cp.nodes[cur].kind == "auto" && guard < 64 {
        guard += 1;
        if comp(cur) {
            v.push(cur);
        }
        match cp.nodes[cur].args.first() {
            Some(&a) => cur = a,
            None => return (Vec::new(), None),
        }
    }
    if v.is_empty() || !comp(cur) {
        return (Vec::new(), None);
    }
    v.reverse();
    (v, Some(cur))
}

pub fn ds_tail(cp: &CompiledProgram) -> Vec<usize> {
    ds_tail_split(cp).0
}

/// DS1 FITTED RULE (not read from the compiler, which does this in the pass around): the nodes that the original evaluates at FULL
/// resolution before the exec-downsampled command. MEASURED on Blur_Motion_Curves_No_Transform at blur_res 1 and 2 (EP-10 B09, NEG2 B09 / B15):
/// the vector `dv = xy*s1 - xy*s0` (nodes `*`, `*`, `-`) is computed at full size on its own copy of the xy variable, then downsampled with the
/// source image, while `xy - dv` and `xy + dv` run inside the command on the small xy. Transform_and_Blur has no such node (everything inside).
/// Rule: a computed node g whose leaves are only xy and constants (pure) is hoisted when it is an argument of a computed node that also takes
/// the xy variable directly; the pure computed arguments of a hoisted node are hoisted too.
pub fn ds_hoist(cp: &CompiledProgram) -> Vec<usize> {
    let nodes = &cp.nodes;
    let is_comp = |i: usize| (nodes[i].kind == "func" || nodes[i].kind == "auto") && !nodes[i].b160 && !nodes[i].folded;
    let is_xy = |i: usize| nodes[i].kind != "func" && nodes[i].kind != "auto" && nodes[i].kind != "const" && nodes[i].name == "xy";
    let is_constlike = |i: usize| nodes[i].kind == "const" || ((nodes[i].kind == "func" || nodes[i].kind == "auto") && (nodes[i].b160 || nodes[i].folded));
    fn pure(cp: &CompiledProgram, i: usize, depth: usize) -> bool {
        let nodes = &cp.nodes;
        if depth > 64 {
            return false;
        }
        let n = &nodes[i];
        if !((n.kind == "func" || n.kind == "auto") && !n.b160 && !n.folded) {
            return false;
        }
        n.args.iter().all(|&a| {
            let m = &nodes[a];
            m.kind == "const" || ((m.kind == "func" || m.kind == "auto") && (m.b160 || m.folded)) || (m.kind != "func" && m.kind != "auto" && m.name == "xy") || pure(cp, a, depth + 1)
        })
    }
    let mut h: Vec<usize> = Vec::new();
    for m in nodes.iter() {
        if !is_comp(m.idx) || !m.args.iter().any(|&a| is_xy(a)) {
            continue;
        }
        for &g in &m.args {
            if is_comp(g) && pure(cp, g, 0) && !h.contains(&g) {
                h.push(g);
            }
        }
    }
    let mut k = 0;
    while k < h.len() {
        let g = h[k];
        for &a in &nodes[g].args {
            if is_comp(a) && !is_constlike(a) && pure(cp, a, 0) && !h.contains(&a) {
                h.push(a);
            }
        }
        k += 1;
    }
    h.sort();
    h
}
