//! Tokenizer and parser of the program language (prefix S-expressions).
//! Source of the forms: audit_ev\LANGUAGE.md sections 1 and 2, genotype-parser.lua protos read in interp_recovery\s4_lua.out.

use std::fmt;

pub const MAX_PARSE_DEPTH: usize = 200;

#[derive(Debug, Clone, PartialEq)]
pub enum Tok {
    LParen,
    RParen,
    LBrack,
    RBrack,
    Num(f64, String),
    Str(String),
    Word(String),
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub msg: String,
    pub pos: usize,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "parse error at char {}: {}", self.pos, self.msg)
    }
}

impl std::error::Error for ParseError {}

fn perr<T>(pos: usize, msg: impl Into<String>) -> Result<T, ParseError> {
    Err(ParseError { msg: msg.into(), pos })
}

/// The grammar's NUMBER token: `^%-?[0-9]*%.?[0-9]+$` (Lua pattern), so `.5` and `-360` are numbers, `1.` is not.
pub fn is_number(w: &str) -> bool {
    let s = w.strip_prefix('-').unwrap_or(w);
    if s.is_empty() {
        return false;
    }
    let mut parts = s.splitn(2, '.');
    let a = parts.next().unwrap_or("");
    match parts.next() {
        None => a.bytes().all(|c| c.is_ascii_digit()),
        Some(b) => {
            a.bytes().all(|c| c.is_ascii_digit()) && !b.is_empty() && b.bytes().all(|c| c.is_ascii_digit())
        }
    }
}

pub fn tokenize(src: &str) -> Result<Vec<(Tok, usize)>, ParseError> {
    let cs: Vec<char> = src.chars().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() || c == '\0' {
            i += 1;
            continue;
        }
        match c {
            '(' => {
                out.push((Tok::LParen, i));
                i += 1;
            }
            ')' => {
                out.push((Tok::RParen, i));
                i += 1;
            }
            '[' => {
                out.push((Tok::LBrack, i));
                i += 1;
            }
            ']' => {
                out.push((Tok::RBrack, i));
                i += 1;
            }
            '"' => {
                let start = i;
                i += 1;
                let mut s = String::new();
                loop {
                    if i >= cs.len() {
                        return perr(start, "unterminated string");
                    }
                    if cs[i] == '"' {
                        i += 1;
                        break;
                    }
                    s.push(cs[i]);
                    i += 1;
                }
                out.push((Tok::Str(s), start));
            }
            _ => {
                let start = i;
                let mut w = String::new();
                while i < cs.len() {
                    let d = cs[i];
                    if d.is_whitespace() || matches!(d, '(' | ')' | '[' | ']' | '"') {
                        break;
                    }
                    w.push(d);
                    i += 1;
                }
                if is_number(&w) {
                    match orig_number(&w).ok_or(()) {
                        Ok(v) => out.push((Tok::Num(v, w), start)),
                        Err(_) => return perr(start, format!("bad number {}", w)),
                    }
                } else {
                    out.push((Tok::Word(w), start));
                }
            }
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------- AST

#[derive(Debug, Clone)]
pub struct Bind {
    pub names: Vec<String>,
    pub rhs: usize,
}

/// A parse node. Identity (the arena index) matters: the builder builds one genotype node per parse node (DAG sharing).
#[derive(Debug, Clone)]
pub enum PKind {
    Scalar(f64),
    Vector(Vec<f64>),
    Str(String),
    Ident(String),
    Call { head: String, args: Vec<usize> },
    /// `nonlocal`: identifier references inside the let (source order, duplicates kept) that the let does not define.
    Let { binds: Vec<Bind>, result: usize, nonlocal: Vec<String> },
}

#[derive(Debug, Default, Clone)]
pub struct Arena {
    pub nodes: Vec<PKind>,
}

impl Arena {
    pub fn push(&mut self, k: PKind) -> usize {
        self.nodes.push(k);
        self.nodes.len() - 1
    }
    pub fn get(&self, id: usize) -> Option<&PKind> {
        self.nodes.get(id)
    }
    pub(crate) fn refs(&self, id: usize, out: &mut Vec<String>) {
        match self.nodes.get(id) {
            Some(PKind::Ident(n)) => out.push(n.clone()),
            Some(PKind::Call { args, .. }) => {
                for a in args {
                    self.refs(*a, out);
                }
            }
            Some(PKind::Let { nonlocal, .. }) => out.extend(nonlocal.iter().cloned()),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DefaultVal {
    Scalar(f64),
    Vector(Vec<f64>),
    Str(String),
    /// bare word default (identifier rhs): the original builds no param for it
    Word(String),
}

#[derive(Debug, Clone, Default)]
pub struct FlagInfo {
    pub min: Option<String>,
    pub max: Option<String>,
    pub popup: Vec<String>,
    pub boolean: bool,
    pub int: bool,
    pub color: bool,
    pub angle: bool,
    /// `:name` flag references (opaque, FLAGS-PLACEHOLDER)
    pub flagrefs: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub default: DefaultVal,
    /// raw flag tokens, in order
    pub flags: Vec<String>,
    pub info: FlagInfo,
    /// the parse node of the default (scalar/vector/string/ident), the node the body's references resolve to
    pub rhs: usize,
}

#[derive(Debug, Clone)]
pub struct InputDef {
    pub name: String,
    /// `( Mask 1 )`: the default (number or word text); None for a required input
    pub default: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EffectDef {
    pub name: String,
    pub va: String,
    pub inputs: Vec<InputDef>,
    pub flags: Vec<String>,
    pub params: Vec<Param>,
    pub body: usize,
}

#[derive(Debug, Clone)]
pub struct DefunDef {
    pub name: String,
    pub va: String,
    pub args: Vec<String>,
    pub body: usize,
}

#[derive(Debug, Clone)]
pub enum Item {
    Effect(EffectDef),
    Defun(DefunDef),
}

const KNOWN_FLAG_EXACT: &[&str] = &[
    "min", "max", "popup", "boolean", "int", "color", "angle", "normalize_scale", "widget", "instance", "show_with_mask",
    "shared", "not_shared", "optional", "disable_motion_blur", "dont_save", "default_to_1_if_postdiv", "old_default",
    "id", "id_insert", "force_slider", "step", "dont_publish", "group_start", "group_end", "string", "file",
];
const KNOWN_FLAG_PREFIX: &[&str] = &[
    "slider_", "big_slider_", "builder_disable", "auto_anim", "screen", "doc_group", "new_column",
];

fn is_known_flag(t: &str) -> bool {
    t.starts_with(':') || KNOWN_FLAG_EXACT.contains(&t) || KNOWN_FLAG_PREFIX.iter().any(|p| t.starts_with(p))
}

/// Best-effort split of a flag group into the value-affecting subset (INFERRED split, LANGUAGE.md section 4).
pub fn flag_info(flags: &[String]) -> FlagInfo {
    let mut fi = FlagInfo::default();
    let mut i = 0;
    while i < flags.len() {
        let t = flags[i].as_str();
        match t {
            "min" => {
                fi.min = flags.get(i + 1).cloned();
                i += 2;
                continue;
            }
            "max" => {
                fi.max = flags.get(i + 1).cloned();
                i += 2;
                continue;
            }
            "popup" => {
                i += 1;
                while i < flags.len() && !is_known_flag(&flags[i]) {
                    fi.popup.push(flags[i].clone());
                    i += 1;
                }
                continue;
            }
            "boolean" => fi.boolean = true,
            "int" => fi.int = true,
            "color" => fi.color = true,
            "angle" => fi.angle = true,
            _ => {
                if t.starts_with(':') {
                    fi.flagrefs.push(t.to_string());
                }
            }
        }
        i += 1;
    }
    fi
}

// ---------------------------------------------------------------- parser

struct P<'a> {
    toks: &'a [(Tok, usize)],
    i: usize,
    depth: usize,
    arena: &'a mut Arena,
    end: usize,
}

fn tok_text(t: &Tok) -> String {
    match t {
        Tok::LParen => "(".into(),
        Tok::RParen => ")".into(),
        Tok::LBrack => "[".into(),
        Tok::RBrack => "]".into(),
        Tok::Num(_, s) => s.clone(),
        Tok::Str(s) => format!("\"{}\"", s),
        Tok::Word(w) => w.clone(),
    }
}

impl<'a> P<'a> {
    fn pos(&self) -> usize {
        self.toks.get(self.i).map(|t| t.1).unwrap_or(self.end)
    }
    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.i).map(|t| &t.0)
    }
    fn next(&mut self) -> Option<Tok> {
        let t = self.toks.get(self.i).map(|t| t.0.clone());
        if t.is_some() {
            self.i += 1;
        }
        t
    }
    fn expect(&mut self, want: Tok, what: &str) -> Result<(), ParseError> {
        let pos = self.pos();
        match self.next() {
            Some(t) if t == want => Ok(()),
            Some(t) => perr(pos, format!("expected {} but found {}", what, tok_text(&t))),
            None => perr(pos, format!("expected {} but reached the end", what)),
        }
    }
    fn word(&mut self, what: &str) -> Result<String, ParseError> {
        let pos = self.pos();
        match self.next() {
            Some(Tok::Word(w)) => Ok(w),
            Some(t) => perr(pos, format!("expected {} but found {}", what, tok_text(&t))),
            None => perr(pos, format!("expected {} but reached the end", what)),
        }
    }

    fn expr(&mut self) -> Result<usize, ParseError> {
        self.depth += 1;
        if self.depth > MAX_PARSE_DEPTH {
            return perr(self.pos(), "nesting too deep");
        }
        let r = self.expr_inner();
        self.depth -= 1;
        r
    }

    fn expr_inner(&mut self) -> Result<usize, ParseError> {
        let pos = self.pos();
        match self.next() {
            None => perr(pos, "expected an expression but reached the end"),
            Some(Tok::Num(v, _)) => Ok(self.arena.push(PKind::Scalar(v))),
            Some(Tok::Str(s)) => Ok(self.arena.push(PKind::Str(s))),
            Some(Tok::Word(w)) => {
                if w.starts_with('#') {
                    return perr(pos, format!("unsupported preprocessor/ref form {}", w));
                }
                if w == "=" {
                    return perr(pos, "unexpected '='");
                }
                Ok(self.arena.push(PKind::Ident(w)))
            }
            Some(Tok::LBrack) => {
                let mut v = Vec::new();
                loop {
                    let p = self.pos();
                    match self.next() {
                        Some(Tok::Num(x, _)) => v.push(x),
                        Some(Tok::RBrack) => break,
                        Some(t) => return perr(p, format!("vector literal element must be a number, found {}", tok_text(&t))),
                        None => return perr(p, "unterminated vector literal"),
                    }
                }
                if v.is_empty() || v.len() > 4 {
                    return perr(pos, format!("vector literal with {} elements (2 to 4 supported)", v.len()));
                }
                Ok(self.arena.push(PKind::Vector(v)))
            }
            Some(Tok::LParen) => {
                let hp = self.pos();
                match self.next() {
                    Some(Tok::Word(h)) => {
                        if h.starts_with('#') {
                            return perr(hp, format!("unsupported form {}", h));
                        }
                        if h == "let" {
                            self.let_form(pos)
                        } else {
                            let mut args = Vec::new();
                            loop {
                                match self.peek() {
                                    Some(Tok::RParen) => {
                                        self.i += 1;
                                        break;
                                    }
                                    None => return perr(self.pos(), "unterminated call"),
                                    _ => args.push(self.expr()?),
                                }
                            }
                            Ok(self.arena.push(PKind::Call { head: h, args }))
                        }
                    }
                    Some(t) => perr(hp, format!("unsupported form: call head must be a symbol (infix?), found {}", tok_text(&t))),
                    None => perr(hp, "unterminated form"),
                }
            }
            Some(t) => perr(pos, format!("unexpected {}", tok_text(&t))),
        }
    }

    fn let_form(&mut self, _pos: usize) -> Result<usize, ParseError> {
        self.expect(Tok::LParen, "'(' after let")?;
        let mut binds: Vec<Bind> = Vec::new();
        loop {
            if let Some(Tok::RParen) = self.peek() {
                self.i += 1;
                break;
            }
            let mut names = Vec::new();
            loop {
                let p = self.pos();
                match self.next() {
                    Some(Tok::Word(w)) if w == "=" => break,
                    Some(Tok::Word(w)) => names.push(w),
                    Some(t) => return perr(p, format!("let binding name expected, found {}", tok_text(&t))),
                    None => return perr(p, "unterminated let binding list"),
                }
            }
            if names.is_empty() {
                return perr(self.pos(), "let binding without a name");
            }
            let rhs = self.expr()?;
            binds.push(Bind { names, rhs });
        }
        let result = self.expr()?;
        self.expect(Tok::RParen, "')' closing let")?;
        let defined: Vec<&String> = binds.iter().flat_map(|b| b.names.iter()).collect();
        let mut all = Vec::new();
        for b in &binds {
            self.arena.refs(b.rhs, &mut all);
        }
        self.arena.refs(result, &mut all);
        let nonlocal: Vec<String> = all.into_iter().filter(|r| !defined.iter().any(|d| *d == r)).collect();
        Ok(self.arena.push(PKind::Let { binds, result, nonlocal }))
    }

    fn flag_group(&mut self) -> Result<Vec<String>, ParseError> {
        // caller consumed the opening '('
        let mut depth = 1usize;
        let mut out = Vec::new();
        loop {
            let p = self.pos();
            match self.next() {
                None => return perr(p, "unterminated flag group"),
                Some(Tok::LParen) => {
                    depth += 1;
                    out.push("(".to_string());
                }
                Some(Tok::RParen) => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(out);
                    }
                    out.push(")".to_string());
                }
                Some(t) => out.push(tok_text(&t)),
            }
        }
    }

    fn inputs(&mut self) -> Result<Vec<InputDef>, ParseError> {
        self.expect(Tok::LParen, "'(' opening the input list")?;
        let mut v = Vec::new();
        loop {
            let p = self.pos();
            match self.next() {
                Some(Tok::RParen) => break,
                Some(Tok::Word(w)) => v.push(InputDef { name: w, default: None }),
                Some(Tok::LParen) => {
                    let name = self.word("input name")?;
                    let p2 = self.pos();
                    let def = match self.next() {
                        Some(Tok::Num(_, s)) => s,
                        Some(Tok::Word(w)) => w,
                        Some(t) => return perr(p2, format!("input default must be a number or word, found {}", tok_text(&t))),
                        None => return perr(p2, "unterminated input"),
                    };
                    self.expect(Tok::RParen, "')' closing input")?;
                    v.push(InputDef { name, default: Some(def) });
                }
                Some(t) => return perr(p, format!("input list: unexpected {}", tok_text(&t))),
                None => return perr(p, "unterminated input list"),
            }
        }
        Ok(v)
    }

    fn flags(&mut self) -> Result<Vec<String>, ParseError> {
        self.expect(Tok::LParen, "'(' opening the effect flags")?;
        self.flag_group()
    }

    fn params(&mut self) -> Result<Vec<Param>, ParseError> {
        self.expect(Tok::LParen, "'(' opening the parameter list")?;
        let mut v = Vec::new();
        loop {
            let p = self.pos();
            match self.next() {
                Some(Tok::RParen) => break,
                Some(Tok::Word(name)) => {
                    let eq = self.word("'=' after parameter name")?;
                    if eq != "=" {
                        return perr(p, format!("parameter {}: expected '=' but found {}", name, eq));
                    }
                    let dp = self.pos();
                    let (default, rhs) = match self.next() {
                        Some(Tok::Num(x, _)) => (DefaultVal::Scalar(x), self.arena.push(PKind::Scalar(x))),
                        Some(Tok::Str(s)) => (DefaultVal::Str(s.clone()), self.arena.push(PKind::Str(s))),
                        Some(Tok::Word(w)) => (DefaultVal::Word(w.clone()), self.arena.push(PKind::Ident(w))),
                        Some(Tok::LBrack) => {
                            let mut x = Vec::new();
                            loop {
                                let q = self.pos();
                                match self.next() {
                                    Some(Tok::Num(n, _)) => x.push(n),
                                    Some(Tok::RBrack) => break,
                                    Some(t) => return perr(q, format!("vector default element must be a number, found {}", tok_text(&t))),
                                    None => return perr(q, "unterminated vector default"),
                                }
                            }
                            if x.is_empty() || x.len() > 4 {
                                return perr(dp, format!("vector default with {} elements", x.len()));
                            }
                            (DefaultVal::Vector(x.clone()), self.arena.push(PKind::Vector(x)))
                        }
                        Some(t) => return perr(dp, format!("parameter {}: unsupported default {}", name, tok_text(&t))),
                        None => return perr(dp, "unterminated parameter list"),
                    };
                    let mut flags = Vec::new();
                    if let Some(Tok::LParen) = self.peek() {
                        self.i += 1;
                        flags = self.flag_group()?;
                    }
                    let info = flag_info(&flags);
                    v.push(Param { name, default, flags, info, rhs });
                }
                Some(t) => return perr(p, format!("parameter list: unexpected {}", tok_text(&t))),
                None => return perr(p, "unterminated parameter list"),
            }
        }
        Ok(v)
    }
}

/// Parse one `(def_effect ...)` or `(defun ...)` text. A trailing newline (the host appends 0x0a) is harmless.
/// `va` is only carried through for reporting.
pub fn parse_item(arena: &mut Arena, text: &str, va: &str) -> Result<Item, ParseError> {
    let toks = tokenize(text)?;
    let end = text.chars().count();
    let mark = arena.nodes.len();
    let mut p = P { toks: &toks, i: 0, depth: 0, arena, end };
    let r = parse_item_inner(&mut p, va);
    if r.is_err() {
        // drop the half-built nodes so a failed parse leaves the arena as it was
        p.arena.nodes.truncate(mark);
    }
    r
}

fn parse_item_inner(p: &mut P, va: &str) -> Result<Item, ParseError> {
    p.expect(Tok::LParen, "'(' opening the form")?;
    let kind = p.word("def_effect or defun")?;
    match kind.as_str() {
        "def_effect" => {
            let name = p.word("effect name")?;
            let inputs = p.inputs()?;
            let flags = p.flags()?;
            let params = p.params()?;
            let body = p.expr()?;
            p.expect(Tok::RParen, "')' closing def_effect")?;
            if p.i != p.toks.len() {
                return perr(p.pos(), "trailing tokens after def_effect");
            }
            Ok(Item::Effect(EffectDef { name, va: va.to_string(), inputs, flags, params, body }))
        }
        "defun" => {
            let name = p.word("defun name")?;
            p.expect(Tok::LParen, "'(' opening the defun argument list")?;
            let mut args = Vec::new();
            loop {
                let pos = p.pos();
                match p.next() {
                    Some(Tok::RParen) => break,
                    Some(Tok::Word(w)) => args.push(w),
                    Some(t) => return perr(pos, format!("defun argument list: unexpected {}", tok_text(&t))),
                    None => return perr(pos, "unterminated defun argument list"),
                }
            }
            let body = p.expr()?;
            p.expect(Tok::RParen, "')' closing defun")?;
            if p.i != p.toks.len() {
                return perr(p.pos(), "trailing tokens after defun");
            }
            Ok(Item::Defun(DefunDef { name, va: va.to_string(), args, body }))
        }
        other => perr(0, format!("unsupported top-level form {}", other)),
    }
}

/// INT21 step L (W473 (b)): a vector literal text read by the genotype reader's own path: the reader's tokenizer (each element a NUMBER token
/// through the W115 path, orig_number) and the same element loop and lane-count rule as a def vector default (1 to 4 elements). The whole text
/// must be one bracketed literal. Used by layoutmerge for a `:hardcode NAME = [ ... ]` value.
pub fn read_vector_literal(text: &str) -> Result<Vec<f64>, ParseError> {
    let toks = tokenize(text)?;
    let mut it = toks.into_iter();
    match it.next() {
        Some((Tok::LBrack, _)) => {}
        Some((t, p)) => return perr(p, format!("vector literal must start with '[', found {}", tok_text(&t))),
        None => return perr(0, "empty vector literal"),
    }
    let mut x = Vec::new();
    loop {
        match it.next() {
            Some((Tok::Num(n, _), _)) => x.push(n),
            Some((Tok::RBrack, _)) => break,
            Some((t, p)) => return perr(p, format!("vector default element must be a number, found {}", tok_text(&t))),
            None => return perr(text.len(), "unterminated vector default"),
        }
    }
    if x.is_empty() || x.len() > 4 {
        return perr(0, format!("vector default with {} elements", x.len()));
    }
    if let Some((t, p)) = it.next() {
        return perr(p, format!("trailing token {} after vector literal", tok_text(&t)));
    }
    Ok(x)
}

/// NV3b (W115, M10): the original's text-to-number path for NUMBER tokens, reproduced generically.
/// The value is accumulated in single precision (f32): the integer part as `v = v * 10 + d`, then for each
/// fractional digit `v += d * scale; scale *= 0.1f`, with `scale` starting at 0.1f; a leading `-` negates.
/// MEASURED-FITTED: this reproduces 181 of 183 parameter-default literals of the 56 EP-3 and EP-3b dumps bit
/// for bit, including the 5 parameters (4 literals) where the nearest f32 differs from the original lane (0.01, 0.05 in two programs,
/// 0.112 and 0.9). The other 2 (`affect_alpha`) are the separate
/// `default_to_1_if_postdiv` rule, not this path. The listing function is NOT located (UNKNOWN); the tokens
/// are `^%-?[0-9]*%.?[0-9]+$`, so no exponent form exists. Returns the f32 result widened to f64 so that a
/// later `as f32` is exact.
pub fn orig_number(text: &str) -> Option<f64> {
    if !is_number(text) {
        return None;
    }
    let (neg, body) = match text.strip_prefix('-') {
        Some(b) => (true, b),
        None => (false, text),
    };
    let (ip, fp) = match body.split_once('.') {
        Some((a, b)) => (a, b),
        None => (body, ""),
    };
    let mut v: f32 = 0.0;
    for c in ip.bytes() {
        v = v * 10.0 + (c - b'0') as f32;
    }
    let mut scale: f32 = 0.1;
    for c in fp.bytes() {
        v += (c - b'0') as f32 * scale;
        scale *= 0.1;
    }
    Some(if neg { -(v as f64) } else { v as f64 })
}

/// NV-3c: the original's string-to-float function, READ from the listing. Two builds of one function: (SSE tier,
/// integer part `v = v*10 + d` with two roundings) and (AVX2 tier, `vfmadd213ss`one rounding; `fma = true`).
/// Skips leading white space, accepts one sign, digits, one '.', fractional digits as `scale *= 0.1f; v += d * scale`
/// (scale starts at 1.0f), an `e` or `E` exponent (`v = (f32)((f64)v * pow(10.0, atoi(rest)))`, then stops), and stops at
/// any other character. Returns None when no digit was seen. All arithmetic in f32 except the exponent step.
pub fn orig_atof(text: &str, fma: bool) -> Option<f32> {
    let b = text.as_bytes();
    let mut i = 0;
    while i < b.len() && (b[i] as char).is_ascii_whitespace() {
        i += 1;
    }
    let mut neg = false;
    if i < b.len() && b[i] == b'-' {
        neg = true;
        i += 1;
    } else if i < b.len() && b[i] == b'+' {
        i += 1;
    }
    let mut v: f32 = 0.0;
    let mut scale: f32 = 1.0;
    let mut dot = false;
    let mut seen = false;
    while i < b.len() {
        let c = b[i];
        if c.is_ascii_digit() {
            seen = true;
            let d = (c - b'0') as f32;
            if dot {
                scale *= 0.1f32;
                v += d * scale;
            } else if fma {
                v = v.mul_add(10.0, d);
            } else {
                v = v * 10.0 + d;
            }
        } else if c == b'.' {
            if dot {
                break;
            }
            dot = true;
        } else if (c & 0xdf) == b'E' {
            let rest = &text[i + 1..];
            let mut e: i32 = 0;
            let rb = rest.as_bytes();
            let (mut k, mut en) = (0, false);
            if k < rb.len() && (rb[k] == b'-' || rb[k] == b'+') {
                en = rb[k] == b'-';
                k += 1;
            }
            while k < rb.len() && rb[k].is_ascii_digit() {
                e = e.saturating_mul(10).saturating_add((rb[k] - b'0') as i32);
                k += 1;
            }
            if en {
                e = -e;
            }
            v = ((v as f64) * 10f64.powf(e as f64)) as f32;
            break;
        } else {
            break;
        }
        i += 1;
    }
    if !seen {
        return None;
    }
    Some(if neg { -v } else { v })
}
