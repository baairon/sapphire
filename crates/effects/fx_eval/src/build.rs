//! The builder: turns one def_effect into a genotype-equivalent node list.
//!
//! Semantics follow genotype-parser.lua as read in interp_recovery\s4_lua.out (protos 130 to 160, 170, 194):
//! * a genotype node is built once per PARSE NODE (DAG sharing by identity, no structural common-subexpression merge);
//! * identifiers alias the node of the variable they name (parse_node_for_var / dereference_node);
//! * a funcall builds its arguments first, left to right, then allocates its own node (post order numbering);
//! * `values` allocates no node: it maps the call to its argument indices (multi-value);
//! * a destructuring let binds names to the first, second, ... value of a multi-value rhs;
//! * `let` first builds, in the enclosing scope, the identifiers it references but does not define (source order),
//!   then builds only its result lazily in a child scope (unused bindings are never built);
//! * a defun or effect call is inlined in a child scope whose variables are the dereferenced call arguments, then the
//!   special variables are copied from the caller (after the formals, so a formal named x/y/xy/Output is shadowed);
//! * order at the top: inputs, then the body (dest), then the parameters that the body did not already build.
//!
//! Label: EVALUATOR-UNCHECKED (not yet compared with the original's genotype).

use crate::corpus::Corpus;
use crate::parse::{DefaultVal, FlagInfo, PKind};
use std::collections::{HashMap, HashSet};
use std::fmt;

pub const SPECIAL_VARS: [&str; 4] = ["x", "y", "xy", "Output"];
const MAX_DEPTH: usize = 300;
const MAX_NODES: usize = 200_000;
const MAX_INLINE: usize = 64;

#[derive(Debug, Clone)]
pub struct BuildError {
    pub msg: String,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "build error: {}", self.msg)
    }
}

impl std::error::Error for BuildError {}

type R<T> = Result<T, BuildError>;

fn berr<T>(msg: impl Into<String>) -> R<T> {
    Err(BuildError { msg: msg.into() })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Scalar,
    Vector,
    Str,
    Function,
    Input,
    Special,
}

impl NodeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            NodeKind::Scalar => "scalar",
            NodeKind::Vector => "vector",
            NodeKind::Str => "string",
            NodeKind::Function => "function",
            NodeKind::Input => "input",
            NodeKind::Special => "special",
        }
    }
}

#[derive(Debug, Clone)]
pub struct GNode {
    /// 0-based, in allocation order
    pub index: usize,
    pub kind: NodeKind,
    /// the debug label (variable name, "dest", or "_" + function name)
    pub label: String,
    /// function name (Function), variable name (Special), input name (Input), empty otherwise
    pub name: String,
    pub children: Vec<usize>,
    /// constant lanes as f32 (Scalar: 1 lane, Vector: its elements); unused lanes are 0
    pub lanes: [f32; 4],
    pub nlanes: usize,
    pub text: String,
    /// 1-based position as passed to make_input_node
    pub input_pos: usize,
}

#[derive(Debug, Clone)]
pub struct InputRec {
    /// the canonical (def) name: what the program body, the bindings and the tables use
    pub name: String,
    pub pos: usize,
    pub node: usize,
    pub default: Option<String>,
    /// LYM3: the layout row's name for this input (the original's program object holds this name, e.g. Matte for Mask); equals `name` on the def path
    pub layout_name: String,
}

#[derive(Debug, Clone)]
pub struct ParamRec {
    pub name: String,
    pub node: Option<usize>,
    pub default: DefaultVal,
    pub flags: Vec<String>,
    pub info: FlagInfo,
    /// W83: the default is the program text's own default
    pub binding: &'static str,
    /// LYM3: the layout row's name for this record (the original's array holds this name; `:modify NAME CANON` renames); equals `name` on the def path
    pub layout_name: String,
}

#[derive(Debug, Clone)]
pub struct Genotype {
    pub name: String,
    pub va: String,
    pub nodes: Vec<GNode>,
    pub inputs: Vec<InputRec>,
    pub params: Vec<ParamRec>,
    pub dest: Vec<usize>,
    pub flags: Vec<String>,
    /// bare-word parameter defaults (identifier rhs): the original creates no param for them
    pub word_defaults: Vec<String>,
    /// LYM3: the def's input names in def order when the layout row's input group replaced the def's (empty on the def path); tables measured in
    /// def order (PARAM_INPUTS.tsv) are translated through it
    pub def_input_order: Vec<String>,
}

impl Genotype {
    pub fn function_names(&self) -> Vec<&str> {
        self.nodes.iter().filter(|n| n.kind == NodeKind::Function).map(|n| n.name.as_str()).collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Key {
    P(usize),
    Special(String),
    Input(usize),
    Synth(usize),
}

#[derive(Debug, Clone)]
struct VarRef {
    key: Key,
    /// 1-based value index within a multi-value node
    which: usize,
}

enum K {
    Let(usize),
    Ident(String),
    Call { head: String, args: Vec<usize> },
    Scalar(f64),
    Vector(Vec<f64>),
    Str(String),
    Special(String),
    Input,
}

struct Scope {
    parent: Option<usize>,
    chain_vars: bool,
    vars: HashMap<String, VarRef>,
    built: HashMap<Key, Vec<usize>>,
    node_names: HashMap<Key, Vec<String>>,
    value_counts: HashMap<Key, usize>,
}

struct Ctx<'a> {
    c: &'a Corpus,
    scopes: Vec<Scope>,
    nodes: Vec<GNode>,
    synth: Vec<PKind>,
    active: Vec<String>,
    in_progress: HashSet<usize>,
    depth: usize,
    inline_depth: usize,
}

impl<'a> Ctx<'a> {
    fn new_scope(&mut self, parent: Option<usize>, chain_vars: bool) -> usize {
        self.scopes.push(Scope {
            parent,
            chain_vars,
            vars: HashMap::new(),
            built: HashMap::new(),
            node_names: HashMap::new(),
            value_counts: HashMap::new(),
        });
        self.scopes.len() - 1
    }

    fn kind_of(&self, key: &Key) -> R<K> {
        let pk: &PKind = match key {
            Key::P(id) => match self.c.arena.get(*id) {
                Some(k) => k,
                None => return berr("dangling parse node"),
            },
            Key::Synth(i) => match self.synth.get(*i) {
                Some(k) => k,
                None => return berr("dangling synthetic node"),
            },
            Key::Special(n) => return Ok(K::Special(n.clone())),
            Key::Input(_) => return Ok(K::Input),
        };
        Ok(match pk {
            PKind::Let { .. } => match key {
                Key::P(id) => K::Let(*id),
                _ => return berr("let outside the arena"),
            },
            PKind::Ident(n) => K::Ident(n.clone()),
            PKind::Call { head, args } => K::Call { head: head.clone(), args: args.clone() },
            PKind::Scalar(v) => K::Scalar(*v),
            PKind::Vector(v) => K::Vector(v.clone()),
            PKind::Str(s) => K::Str(s.clone()),
        })
    }

    fn lookup_var(&self, s: usize, name: &str) -> Option<VarRef> {
        let mut cur = Some(s);
        while let Some(c) = cur {
            let sc = &self.scopes[c];
            if let Some(v) = sc.vars.get(name) {
                return Some(v.clone());
            }
            if sc.chain_vars {
                cur = sc.parent;
            } else {
                break;
            }
        }
        None
    }

    fn lookup_built(&self, s: usize, key: &Key) -> Option<&Vec<usize>> {
        let mut cur = Some(s);
        while let Some(c) = cur {
            if let Some(v) = self.scopes[c].built.get(key) {
                return Some(v);
            }
            cur = self.scopes[c].parent;
        }
        None
    }

    /// parse_node_for_var: follow identifier aliases to the defining parse node
    fn pnode_for_var(&self, s: usize, name: &str, guard: usize) -> R<VarRef> {
        if guard > MAX_DEPTH {
            return berr(format!("alias cycle through variable {}", name));
        }
        let r = match self.lookup_var(s, name) {
            Some(r) => r,
            None => return berr(format!("undefined variable {}", name)),
        };
        if let Key::P(id) = &r.key {
            if let Some(PKind::Ident(n)) = self.c.arena.get(*id) {
                return self.pnode_for_var(s, n, guard + 1);
            }
        }
        Ok(r)
    }

    fn deref(&self, s: usize, key: &Key) -> R<VarRef> {
        if let Key::P(id) = key {
            if let Some(PKind::Ident(v)) = self.c.arena.get(*id) {
                return self.pnode_for_var(s, v, 0);
            }
        }
        Ok(VarRef { key: key.clone(), which: 1 })
    }

    fn genotype_indices(&self, s: usize, key: &Key) -> R<Vec<usize>> {
        let is_ident = matches!(key, Key::P(id) if matches!(self.c.arena.get(*id), Some(PKind::Ident(_))));
        if is_ident {
            let d = self.deref(s, key)?;
            let l = self.lookup_built(s, &d.key);
            return Ok(l.and_then(|v| v.get(d.which.wrapping_sub(1))).map(|x| vec![*x]).unwrap_or_default());
        }
        Ok(self.lookup_built(s, key).cloned().unwrap_or_default())
    }

    fn set_indices(&mut self, s: usize, key: Key, idx: Vec<usize>) -> R<()> {
        if let Some(old) = self.lookup_built(s, &key) {
            for (i, o) in old.iter().enumerate() {
                if idx.get(i) != Some(o) {
                    return berr("inconsistent re-assignment of genotype indices");
                }
            }
        }
        self.scopes[s].built.insert(key, idx);
        Ok(())
    }

    fn push_node(&mut self, s: usize, key: Key, mut n: GNode) -> R<usize> {
        if self.nodes.len() >= MAX_NODES {
            return berr("too many nodes");
        }
        let idx = self.nodes.len();
        n.index = idx;
        self.nodes.push(n);
        self.set_indices(s, key, vec![idx])?;
        Ok(idx)
    }

    fn blank(kind: NodeKind, label: String) -> GNode {
        GNode {
            index: 0,
            kind,
            label,
            name: String::new(),
            children: Vec::new(),
            lanes: [0.0; 4],
            nlanes: 0,
            text: String::new(),
            input_pos: 0,
        }
    }

    fn build_node(&mut self, s: usize, key: Key, name: Option<String>) -> R<()> {
        self.depth += 1;
        if self.depth > MAX_DEPTH {
            self.depth -= 1;
            return berr("build recursion too deep (cyclic definition?)");
        }
        let r = self.build_node_inner(s, key, name);
        self.depth -= 1;
        r
    }

    fn build_node_inner(&mut self, s: usize, key: Key, name: Option<String>) -> R<()> {
        if !self.genotype_indices(s, &key)?.is_empty() {
            return Ok(());
        }
        let mut name = name;
        if let Some(nn) = self.scopes[s].node_names.get(&key) {
            if let Some(f) = nn.first() {
                name = Some(f.clone());
            }
        }
        let guard_id = if let Key::P(id) = &key { Some(*id) } else { None };
        if let Some(id) = guard_id {
            if !self.in_progress.insert(id) {
                return berr("cyclic reference while building a node");
            }
        }
        let res = self.build_kind(s, &key, name);
        if let Some(id) = guard_id {
            self.in_progress.remove(&id);
        }
        res?;
        if let Some(&exp) = self.scopes[s].value_counts.get(&key) {
            let got = self.genotype_indices(s, &key)?.len();
            if got != exp {
                return berr(format!("Assignment has {} variables, but rhs has {} values.", exp, got));
            }
        }
        Ok(())
    }

    fn build_kind(&mut self, s: usize, key: &Key, name: Option<String>) -> R<()> {
        match self.kind_of(key)? {
            K::Let(pid) => self.build_let(s, key, pid, name),
            K::Ident(v) => {
                let r = self.pnode_for_var(s, &v, 0)?;
                self.build_node(s, r.key, name)
            }
            K::Call { head, args } => self.build_call(s, key, &head, &args, name),
            K::Scalar(v) => {
                let mut n = Self::blank(NodeKind::Scalar, name.unwrap_or_default());
                n.lanes[0] = v as f32;
                n.nlanes = 1;
                self.push_node(s, key.clone(), n)?;
                Ok(())
            }
            K::Vector(v) => {
                let mut n = Self::blank(NodeKind::Vector, name.unwrap_or_default());
                for (i, x) in v.iter().take(4).enumerate() {
                    n.lanes[i] = *x as f32;
                }
                n.nlanes = v.len().min(4);
                self.push_node(s, key.clone(), n)?;
                Ok(())
            }
            K::Str(t) => {
                let mut n = Self::blank(NodeKind::Str, name.unwrap_or_default());
                n.text = t;
                self.push_node(s, key.clone(), n)?;
                Ok(())
            }
            K::Special(v) => {
                let mut n = Self::blank(NodeKind::Special, name.unwrap_or_default());
                n.name = v;
                self.push_node(s, key.clone(), n)?;
                Ok(())
            }
            K::Input => berr("input node referenced before it was built"),
        }
    }

    fn build_funcall_args(&mut self, s: usize, key: &Key, head: &str, args: &[usize]) -> R<Vec<usize>> {
        let names = if head == "values" { self.scopes[s].node_names.get(key).cloned() } else { None };
        let mut out = Vec::with_capacity(args.len());
        for (i, a) in args.iter().enumerate() {
            let ak = Key::P(*a);
            let nm = names.as_ref().and_then(|n| n.get(i)).cloned();
            self.build_node(s, ak.clone(), nm)?;
            let ind = self.genotype_indices(s, &ak)?;
            if ind.len() > 1 {
                return berr(format!("Argument {}, expected single value but got {}", i + 1, ind.len()));
            }
            match ind.first() {
                Some(x) => out.push(*x),
                None => return berr(format!("Argument {} of {} built no node", i + 1, head)),
            }
        }
        Ok(out)
    }

    fn build_call(&mut self, s: usize, key: &Key, head: &str, args: &[usize], name: Option<String>) -> R<()> {
        let arg_idx = self.build_funcall_args(s, key, head, args)?;
        if head == "values" {
            return self.set_indices(s, key.clone(), arg_idx);
        }
        let c: &'a Corpus = self.c;
        if let Some(&ei) = c.effect_idx.get(head) {
            let e = &c.effects[ei];
            let n_in = e.inputs.len();
            let expected = n_in + e.params.len();
            let mut call_args: Vec<Key> = args.iter().map(|a| Key::P(*a)).collect();
            let mut i = call_args.len();
            while i < expected {
                if i < n_in {
                    return berr(format!("effect {}: missing required input {}", e.name, e.inputs[i].name));
                }
                let p = &e.params[i - n_in];
                if !p.flags.iter().any(|f| f == "optional") {
                    break;
                }
                let pk = match &p.default {
                    DefaultVal::Scalar(v) => PKind::Scalar(*v),
                    DefaultVal::Vector(v) => PKind::Vector(v.clone()),
                    _ => return berr(format!("effect {}: parameter {} has no usable default", e.name, p.name)),
                };
                self.synth.push(pk);
                call_args.push(Key::Synth(self.synth.len() - 1));
                i += 1;
            }
            if call_args.len() != expected {
                return berr(format!(
                    "Wrong number of arguments to effect '{}' (got {}, expected {})",
                    e.name,
                    call_args.len(),
                    expected
                ));
            }
            let inputs: Vec<String> = e.inputs.iter().map(|x| x.name.clone()).collect();
            let formals: Vec<String> = e.params.iter().map(|x| x.name.clone()).collect();
            return self.inline(s, key, name, "effect", &e.name, &inputs, &formals, e.body, call_args);
        }
        if let Some(&di) = c.defun_idx.get(head) {
            let d = &c.defuns[di];
            if args.len() != d.args.len() {
                return berr(format!(
                    "Wrong number of arguments to defun '{}' (got {}, expected {})",
                    d.name,
                    args.len(),
                    d.args.len()
                ));
            }
            let call_args: Vec<Key> = args.iter().map(|a| Key::P(*a)).collect();
            return self.inline(s, key, name, "defun", &d.name, &[], &d.args, d.body, call_args);
        }
        let mut n = Self::blank(NodeKind::Function, name.unwrap_or_else(|| format!("_{}", head)));
        n.name = head.to_string();
        n.children = arg_idx;
        self.push_node(s, key.clone(), n)?;
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn inline(
        &mut self,
        s: usize,
        call_key: &Key,
        name: Option<String>,
        what: &str,
        callee: &str,
        input_names: &[String],
        formals: &[String],
        body: usize,
        call_args: Vec<Key>,
    ) -> R<()> {
        if self.active.iter().any(|a| a == callee) {
            return berr(format!("Cyclic call to {} '{}'", what, callee));
        }
        if self.inline_depth >= MAX_INLINE {
            return berr("inlining too deep");
        }
        if call_args.len() != input_names.len() + formals.len() {
            return berr(format!("{} '{}': argument count mismatch", what, callee));
        }
        let mut vars: HashMap<String, VarRef> = HashMap::new();
        for (i, n) in input_names.iter().chain(formals.iter()).enumerate() {
            let r = self.deref(s, &call_args[i])?;
            vars.insert(n.clone(), r);
        }
        for sv in SPECIAL_VARS {
            if let Some(r) = self.lookup_var(s, sv) {
                vars.insert(sv.to_string(), r);
            }
        }
        let ch = self.new_scope(Some(s), false);
        self.scopes[ch].vars = vars;
        self.active.push(callee.to_string());
        self.inline_depth += 1;
        let r = self.build_node(ch, Key::P(body), name);
        self.inline_depth -= 1;
        self.active.pop();
        r?;
        let ind = self.genotype_indices(ch, &Key::P(body))?;
        self.set_indices(s, call_key.clone(), ind)
    }

    fn build_let(&mut self, s: usize, key: &Key, pid: usize, name: Option<String>) -> R<()> {
        let c: &'a Corpus = self.c;
        let (binds, result, nonlocal) = match c.arena.get(pid) {
            Some(PKind::Let { binds, result, nonlocal }) => (binds, *result, nonlocal),
            _ => return berr("not a let node"),
        };
        for v in nonlocal {
            let r = self.pnode_for_var(s, v, 0)?;
            self.build_node(s, r.key, None)?;
        }
        let ch = self.new_scope(Some(s), true);
        for sv in SPECIAL_VARS {
            self.scopes[ch].vars.insert(sv.to_string(), VarRef { key: Key::Special(sv.to_string()), which: 1 });
        }
        for b in binds {
            for (i, n) in b.names.iter().enumerate() {
                self.scopes[ch].vars.insert(n.clone(), VarRef { key: Key::P(b.rhs), which: i + 1 });
            }
            self.scopes[ch].node_names.insert(Key::P(b.rhs), b.names.clone());
            self.scopes[ch].value_counts.insert(Key::P(b.rhs), b.names.len());
        }
        self.build_node(ch, Key::P(result), name)?;
        let ind = self.genotype_indices(ch, &Key::P(result))?;
        self.set_indices(s, key.clone(), ind)
    }
}

/// Build the genotype-equivalent node list of `corpus.effects[effect_index]`.
pub fn build_effect(corpus: &Corpus, effect_index: usize) -> Result<Genotype, BuildError> {
    let e = match corpus.effects.get(effect_index) {
        Some(e) => e,
        None => return berr("no such effect"),
    };
    let mut ctx = Ctx {
        c: corpus,
        scopes: Vec::new(),
        nodes: Vec::new(),
        synth: Vec::new(),
        active: Vec::new(),
        in_progress: HashSet::new(),
        depth: 0,
        inline_depth: 0,
    };
    let s0 = ctx.new_scope(None, false);
    // create_variable_mappings: specials, then parameters, then inputs (inputs shadow parameters of the same name)
    for sv in SPECIAL_VARS {
        ctx.scopes[s0].vars.insert(sv.to_string(), VarRef { key: Key::Special(sv.to_string()), which: 1 });
    }
    for p in &e.params {
        ctx.scopes[s0].vars.insert(p.name.clone(), VarRef { key: Key::P(p.rhs), which: 1 });
        ctx.scopes[s0].node_names.insert(Key::P(p.rhs), vec![p.name.clone()]);
        ctx.scopes[s0].value_counts.insert(Key::P(p.rhs), 1);
    }
    for (i, inp) in e.inputs.iter().enumerate() {
        ctx.scopes[s0].vars.insert(inp.name.clone(), VarRef { key: Key::Input(i), which: 1 });
    }
    // build_input_nodes
    let mut inputs = Vec::new();
    for (i, inp) in e.inputs.iter().enumerate() {
        let mut n = Ctx::blank(NodeKind::Input, inp.name.clone());
        n.name = inp.name.clone();
        n.input_pos = i + 1;
        let idx = ctx.push_node(s0, Key::Input(i), n)?;
        inputs.push(InputRec { name: inp.name.clone(), pos: i + 1, node: idx, default: inp.default.clone(), layout_name: inp.name.clone() });
    }
    // body
    ctx.build_node(s0, Key::P(e.body), Some("dest".to_string()))?;
    let dest = ctx.genotype_indices(s0, &Key::P(e.body))?;
    if dest.is_empty() {
        return berr("the body built no node");
    }
    // build_assignment_nodes: parameters the body did not build, then the parameter records
    let mut params = Vec::new();
    let mut word_defaults = Vec::new();
    for p in &e.params {
        if let DefaultVal::Word(w) = &p.default {
            word_defaults.push(format!("{}={}", p.name, w));
            continue;
        }
        ctx.build_node(s0, Key::P(p.rhs), None)?;
        let r = ctx.pnode_for_var(s0, &p.name, 0)?;
        let node = ctx.lookup_built(s0, &r.key).and_then(|v| v.get(r.which.wrapping_sub(1))).copied();
        params.push(ParamRec {
            name: p.name.clone(),
            node,
            default: p.default.clone(),
            flags: p.flags.clone(),
            info: p.info.clone(),
            binding: "PROGRAM-DEFAULT",
            layout_name: p.name.clone(),
        });
    }
    Ok(Genotype {
        name: e.name.clone(),
        va: e.va.clone(),
        nodes: ctx.nodes,
        inputs,
        params,
        dest,
        flags: e.flags.clone(),
        word_defaults,
        def_input_order: Vec::new(),
    })
}

fn join_usize(v: &[usize]) -> String {
    v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(";")
}

fn lanes_hex(lanes: &[f32; 4], n: usize) -> String {
    lanes.iter().take(n).map(|x| format!("{:08x}", x.to_bits())).collect::<Vec<_>>().join(";")
}

/// Deterministic TSV of one genotype. Columns (tab separated):
/// `GENOTYPE name va n_nodes n_inputs n_params dest flags`
/// `NODE name idx kind fname children nlanes lanes_hex label`
/// `INPUT name pos input_name node default`
/// `PARAM name param node default_kind default flags binding`
pub fn genotype_tsv(g: &Genotype) -> String {
    let mut s = String::new();
    s.push_str(&format!(
        "GENOTYPE\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
        g.name,
        g.va,
        g.nodes.len(),
        g.inputs.len(),
        g.params.len(),
        join_usize(&g.dest),
        g.flags.join(" ")
    ));
    for n in &g.nodes {
        let fname = if n.kind == NodeKind::Str { n.text.as_str() } else { n.name.as_str() };
        s.push_str(&format!(
            "NODE\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            g.name,
            n.index,
            n.kind.as_str(),
            fname,
            join_usize(&n.children),
            n.nlanes,
            lanes_hex(&n.lanes, n.nlanes),
            n.label
        ));
    }
    for i in &g.inputs {
        s.push_str(&format!(
            "INPUT\t{}\t{}\t{}\t{}\t{}\n",
            g.name,
            i.pos,
            i.name,
            i.node,
            i.default.clone().unwrap_or_else(|| "-".into())
        ));
    }
    for p in &g.params {
        let (dk, dv) = match &p.default {
            DefaultVal::Scalar(v) => ("scalar", format!("{:08x}", (*v as f32).to_bits())),
            DefaultVal::Vector(v) => (
                "vector",
                v.iter().map(|x| format!("{:08x}", (*x as f32).to_bits())).collect::<Vec<_>>().join(";"),
            ),
            DefaultVal::Str(t) => ("string", t.clone()),
            DefaultVal::Word(w) => ("word", w.clone()),
        };
        s.push_str(&format!(
            "PARAM\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            g.name,
            p.name,
            p.node.map(|x| x.to_string()).unwrap_or_else(|| "-".into()),
            dk,
            dv,
            p.flags.join(" "),
            p.binding
        ));
    }
    s
}
