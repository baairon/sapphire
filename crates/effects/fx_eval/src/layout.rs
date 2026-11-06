//! NV-3c: command layout of the plan (F13), derived from the emitter and its helpers and measured
//! against the layout half of the 56 fitted dumps. Labels in README.md of this folder.
//!
//! Rules (address, status):
//! - run-primitive command flag = 1 unless node byte is set (r8d = [node] == 0). The byte is set by the
//!   propagation pass : a node whose class flag word (slot 49) has bit 0x2000 sets the byte on its image arguments,
//!   and a node with the byte set sets it on its image arguments (transitive). ([node] != -1 also sets it; never seen: UNKNOWN.)
//! - in place: reuses an argument image as the destination when the class flag word has bit 0x4, the argument image
//!   has the same element count, cpu/gpu byte and format as the destination, and this node is the argument's last consumer
//!   (or the root). The first qualifying argument in order wins. The per-argument predicate [vt](i) is approximated by
//!   "the argument is an image" (INFERRED).
//! - frees: after the run, every dying image argument that was not reused as the destination, in argument order.
//! - cleanup records: one judge_keys::PLAN_CLEANUP_CMD (the recorded host cleanup command name) per allocate-images command, at the end (INFERRED count rule, measured on the dumps).
//! - get_frame writes a cpu image; the consumers see a gpu twin (allocate, copy-images, free of the cpu image).
//! - inputs: allocate + convert-from-source <slot> when first needed as an image; an input used only as the clip argument of
//!   get_frame prints `(clip <slot> <elnum>)` and is never converted.
use crate::compile::CompiledProgram;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub fn class_flags() -> &'static HashMap<String, u32> {
    static M: OnceLock<HashMap<String, u32>> = OnceLock::new();
    M.get_or_init(|| effects_rtdata::classes::CLASS_FLAGS.iter().map(|(n, f)| (n.to_string(), *f)).collect())
}

/// NV-3g rule A (RULE-FITTED, evidence tools/argprop.py): a marked `monochrome` does not pass the command-flag mark on to its
/// argument. 6 of 6 observations in the originals (PixelSortLinear a, b, nf1, nf2 and PixelSortRadial nf1, nf2) show the argument unmarked
/// (flag 1) while the node itself is marked (flag 0); every other (primitive, argument) pair seen (16 other (primitive, argument) pairs, 373 observations) propagates.
/// Cause INFERRED: the argument cell type byte read by the image-argument predicate (slot 9, : `[cell] == 0`) is non-zero for monochrome.
pub fn mark_blocked(prim: &str) -> bool {
    let p = prim.strip_suffix("_if_needed").unwrap_or(prim);
    p == "monochrome"
}

/// INT18 (PROGRAM-FLAG-READ): [prog] bit 4 is set when some function node of the parsed program (called programs and defuns
/// included, as the genotype holds them) has a class flag word with 0x40000 (writer); only motion_blur has it.
pub fn program_flag_bit4(g: &crate::build::Genotype) -> bool {
    g.nodes.iter().any(|n| n.kind == crate::build::NodeKind::Function && flags_of(&n.name) & 0x40000 != 0)
}

pub fn flags_of(prim: &str) -> u32 {
    let p = prim.strip_suffix("_if_needed").unwrap_or(prim);
    *class_flags().get(p).unwrap_or(&0)
}

#[derive(Clone, Debug)]
struct Img {
    id: usize,
    cpu: bool,
    fmt: u8,
    el: i32,
    noreuse: bool,
}

fn rf(i: &Img) -> String {
    format!(":ref{}.{}{}", i.id, i.fmt, if i.cpu { "" } else { "g" })
}

/// Convert-from-source slot of a host input (INFERRED from the dumps: Front and Source 0, Back and Z_Buffer 1).
pub fn input_slot(name: &str, pos: usize) -> usize {
    if pos >= 1 {
        return pos - 1;
    }
    match name {
        "Back" | "Z_Buffer" | "Background" => 1,
        _ => 0,
    }
}

fn push(c: &mut Vec<(String, String)>, t: String) {
    let op = t.trim_start_matches('(').split_whitespace().next().unwrap_or("").to_string();
    c.push((op, t));
}

/// Command list as (class, text); the cleanup records have the class judge_keys::PLAN_CLEANUP_CMD and empty text.
pub fn layout_cmds(cp: &CompiledProgram, fmt_g6: &dyn Fn(f64) -> String) -> Vec<(String, String)> {
    let mut cmds: Vec<(String, String)> = Vec::new();
    let is_out = |i: usize| cp.nodes[i].kind == "special" && cp.nodes[i].name == "Output";
    let mut last: HashMap<usize, usize> = HashMap::new();
    for n in &cp.nodes {
        for &a in &n.args {
            last.insert(a, n.idx);
        }
    }
    let mut need_img: HashSet<usize> = HashSet::new();
    for n in &cp.nodes {
        if n.kind == "func" || n.kind == "auto" {
            for (k, &a) in n.args.iter().enumerate() {
                if cp.nodes[a].kind == "input" && !(n.prim == "get_frame" && k == 0) {
                    need_img.insert(a);
                }
            }
        }
    }
    if cp.nodes[cp.root].kind == "input" {
        need_img.insert(cp.root);
    }
    // the propagation 
    let mut mark: HashSet<usize> = HashSet::new();
    loop {
        let mut ch = false;
        for n in &cp.nodes {
            if (n.kind == "func" || n.kind == "auto") && (mark.contains(&n.idx) || flags_of(&n.prim) & 0x2000 != 0) {
                for &a in &n.args {
                    if (cp.nodes[a].kind == "func" || cp.nodes[a].kind == "auto") && !mark_blocked(&n.prim) && mark.insert(a) {
                        ch = true;
                    }
                }
            }
        }
        if !ch {
            break;
        }
    }
    let mut cur: HashMap<usize, Img> = HashMap::new();
    let mut nalloc = 0usize;
    for n in &cp.nodes {
        match n.kind {
            "special" if n.name == "Output" => {}
            "input" | "special" if n.name == "xy" => {
                let im = Img { id: n.idx, cpu: false, fmt: 2, el: n.elnum.max(1), noreuse: true };
                push(&mut cmds, format!("(allocate-images {} {} 2 gpu)", rf(&im), im.el));
                nalloc += 1;
                push(&mut cmds, format!("(variable 2 {})", rf(&im)));
                cur.insert(n.idx, im);
            }
            "input" | "special" => {
                if need_img.contains(&n.idx) {
                    let im = Img { id: n.idx, cpu: false, fmt: 2, el: n.elnum.max(1), noreuse: false };
                    push(&mut cmds, format!("(allocate-images {} {} 2 gpu)", rf(&im), im.el));
                    nalloc += 1;
                    push(&mut cmds, format!("(convert-from-source {} {})", input_slot(&n.name, n.input_pos), rf(&im)));
                    cur.insert(n.idx, im);
                }
            }
            "const" => {}
            _ => {
                let prim = n.prim.strip_suffix("_if_needed").unwrap_or(&n.prim).to_string();
                let fl = flags_of(&prim);
                for (k, &x) in n.args.iter().enumerate() {
                    let an = &cp.nodes[x];
                    if an.kind == "const" && k == 0 && matches!(prim.as_str(), "blur" | "sharpen" | "edge_aware_blur" | "edge_aware_sharpen") && !an.name.starts_with("__") && an.param.is_none() && !cur.contains_key(&x) {
                        let im = Img { id: x, cpu: false, fmt: 2, el: an.elnum.clamp(1, 4), noreuse: true };
                        push(&mut cmds, format!("(allocate-images {} {} 2 gpu)", rf(&im), im.el));
                        nalloc += 1;
                        let l: Vec<String> = (0..im.el as usize).map(|i| fmt_g6(f32::from_bits(an.lanes[i]) as f64)).collect();
                        push(&mut cmds, format!("(constant [ {} ] {})", l.join(" "), rf(&im)));
                        cur.insert(x, im);
                    }
                }
                let want_cpu = prim == "get_frame";
                let want_fmt: u8 = if prim == "make_mipmap" && n.idx != cp.root { 3 } else { 2 };
                let el = n.elnum.max(1);
                let mut dest: Option<Img> = None;
                if fl & 0x4 != 0 {
                    for &x in &n.args {
                        if let Some(a) = cur.get(&x) {
                            if !a.noreuse && a.el == el && a.cpu == want_cpu && a.fmt == want_fmt && last.get(&x) == Some(&n.idx) {
                                dest = Some(a.clone());
                                break;
                            }
                        }
                    }
                }
                let inplace = dest.is_some();
                let dest = match dest {
                    Some(d) => d,
                    None => {
                        let d = Img { id: n.idx, cpu: want_cpu, fmt: want_fmt, el, noreuse: false };
                        push(&mut cmds, format!("(allocate-images {} {} {} {})", rf(&d), d.el, d.fmt, if d.cpu { "cpu" } else { "gpu" }));
                        nalloc += 1;
                        d
                    }
                };
                let mut a: Vec<String> = Vec::new();
                for (k, &x) in n.args.iter().enumerate() {
                    let an = &cp.nodes[x];
                    if is_out(x) {
                        a.push("[ -999 ]".to_string());
                    } else if let Some(im) = cur.get(&x) {
                        a.push(rf(im));
                    } else if an.kind == "input" && prim == "get_frame" && k == 0 {
                        a.push(format!("(clip {} {})", input_slot(&an.name, an.input_pos), an.elnum.max(1)));
                    } else if an.kind == "const" {
                        let l: Vec<String> = (0..an.elnum.clamp(1, 4) as usize).map(|i| fmt_g6(f32::from_bits(an.lanes[i]) as f64)).collect();
                        a.push(format!("[ {} ]", l.join(" ")));
                    } else {
                        a.push(rf(&Img { id: x, cpu: false, fmt: 2, el: 0, noreuse: false }));
                    }
                }
                let flag = if mark.contains(&n.idx) { 0 } else { 1 };
                push(&mut cmds, format!("(run-primitive {} -1 {} {} {} )", flag, rf(&dest), prim, a.join(" ")));
                let mut freed: HashSet<usize> = HashSet::new();
                for &x in &n.args {
                    if let Some(im) = cur.get(&x).cloned() {
                        if last.get(&x) == Some(&n.idx) && !(inplace && im.id == dest.id && im.cpu == dest.cpu) && freed.insert(x) {
                            push(&mut cmds, format!("(free-images {})", rf(&im)));
                        }
                    }
                }
                let mut out = dest;
                if want_cpu {
                    let g = Img { id: out.id, cpu: false, fmt: 2, el: out.el, noreuse: false };
                    push(&mut cmds, format!("(allocate-images {} {} 2 gpu)", rf(&g), g.el));
                    nalloc += 1;
                    push(&mut cmds, format!("(copy-images {} {})", rf(&g), rf(&out)));
                    push(&mut cmds, format!("(free-images {})", rf(&out)));
                    out = g;
                }
                cur.insert(n.idx, out);
            }
        }
    }
    let root = &cp.nodes[cp.root];
    if root.kind != "const" {
        if let Some(im) = cur.get(&root.idx).cloned() {
            push(&mut cmds, format!("(convert-to-dest {})", rf(&im)));
            push(&mut cmds, format!("(free-images {})", rf(&im)));
        }
    }
    for _ in 0..nalloc {
        cmds.push((crate::judge_keys::PLAN_CLEANUP_CMD.to_string(), String::new()));
    }
    cmds
}
