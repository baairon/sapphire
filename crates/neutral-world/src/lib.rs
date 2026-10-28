//! Rust side of the world-parametric parity harness. std only.
//!
//! The dumps lack data sections and imports, so a port is written against the [`Env`] trait: all
//! memory outside the function's own locals goes through `Env::r*`/`w*`, and every call (import,
//! indirect call, callee inside the dump) goes through `Env::call`. [`World`] implements `Env` with
//! the same deterministic pseudo-random model as tools/world.py, so a correct port reproduces the
//! original's return values, final memory writes and call trace in every seeded world.

use std::collections::{BTreeSet, HashMap};
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};
use std::path::Path;

pub const A0: u64 = 0x1000_0000;
pub const HDR: u64 = 0x10_0000;
pub const HEAP_REL: u64 = 0x1000_0000;
pub const HEAP_SLOTS: u64 = 1 << 16;
pub const WABOVE: u64 = HEAP_REL + 64 * HEAP_SLOTS + 0x10_0000;
/// Value a port passes for a pointer into its own stack frame (the oracle normalizes stack pointers).
pub const STK: u64 = 0x5afe_57ac;
const STACK_BASE: u64 = 0x7ff0_0000;
const STACK_TOP: u64 = STACK_BASE + 0x10_0000;

pub fn sm64(x: u64) -> u64 {
    let x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

pub fn mix(a: u64, b: u64) -> u64 {
    sm64(a ^ sm64(b))
}

fn page_up(n: u64) -> u64 {
    (n + 0xfff) & !0xfff
}

fn norm(v: u64) -> u64 {
    if (STACK_BASE..STACK_TOP + 0x1000).contains(&v) {
        STK
    } else {
        v
    }
}

#[derive(Debug, Clone)]
pub struct Layout {
    pub n: u64,
    pub anchor: u64,
    pub stub_end_abs: u64,
    pub world_hi: u64,
    pub world_lo: u64,
    pub ptrn: u64,
    /// Address of heap slot 0. Old vectors use anchor + HEAP_REL; vectors with `heap=aligned` use a page-aligned base.
    pub heap_base: u64,
    /// Per case pinned world words (aligned address, value); empty in every earlier vector set.
    pub pins: Vec<(u64, u64)>,
}

impl Layout {
    pub fn new(n: u64, stub_zone: u64, ptrn: u64) -> Layout {
        let anchor = A0 + n;
        Layout { n, anchor, stub_end_abs: anchor + stub_zone, world_hi: anchor + WABOVE, world_lo: A0 - HDR, ptrn, heap_base: anchor + HEAP_REL, pins: Vec::new() }
    }

    /// Page-aligned heap base, so heap objects and pointer-like words are aligned like real heap blocks.
    pub fn with_aligned_heap(mut self) -> Layout {
        self.heap_base = (self.anchor + HEAP_REL + 0xfff) & !0xfff;
        self
    }

    pub fn in_world(&self, a: u64) -> bool {
        (self.world_lo <= a && a < A0) || (self.stub_end_abs <= a && a < self.world_hi)
    }

    pub fn heap(&self, slot: u64) -> u64 {
        self.heap_base + slot * 64
    }

    pub fn word(&self, seed: u64, a8: u64) -> u64 {
        for &(pa, pv) in &self.pins {
            if pa == a8 {
                return pv;
            }
        }
        let h = mix(seed, a8.wrapping_sub(self.anchor));
        let sel = h & 15;
        let p = self.ptrn;
        if sel < p {
            self.heap((h >> 8) % HEAP_SLOTS)
        } else if sel < p + 3 {
            (h >> 8) & 0xff
        } else if sel < p + 5 {
            (h >> 8) & 0xffff
        } else if sel < p + 7 {
            0
        } else {
            h
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceEv {
    pub target: u64,
    pub iargs: Vec<u64>,
    pub fargs: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallRet {
    pub rax: u64,
    pub rdx: u64,
    pub xmm0_lo: u64,
    pub xmm0_hi: u64,
}

/// Raised (without invoking the panic hook) when a port touches memory the model does not provide.
#[derive(Debug)]
pub struct Fault(pub String);

fn fault(msg: String) -> ! {
    resume_unwind(Box::new(Fault(msg)))
}

/// Everything a port may do outside its own locals.
pub trait Env {
    fn r8(&mut self, a: u64) -> u8;
    fn w8(&mut self, a: u64, v: u8);
    fn call_f(&mut self, target: u64, iargs: &[u64], fargs: &[u64]) -> CallRet;
    /// Absolute address of the code at file offset `off` of the dump.
    fn code_va(&self, off: u64) -> u64;
    /// Absolute address of a target given relative to the end of the dump (`rel_to_end`).
    fn data_va(&self, rel: i64) -> u64;

    /// Call with integer arguments only. The slice length must equal the arity declared in the spec.
    fn call(&mut self, target: u64, iargs: &[u64]) -> CallRet {
        self.call_f(target, iargs, &[])
    }
    fn stub_va(&self, rel: u64) -> u64 {
        self.data_va(rel as i64)
    }
    fn r16(&mut self, a: u64) -> u16 {
        u16::from_le_bytes([self.r8(a), self.r8(a + 1)])
    }
    fn r32(&mut self, a: u64) -> u32 {
        let mut b = [0u8; 4];
        for (i, x) in b.iter_mut().enumerate() {
            *x = self.r8(a + i as u64);
        }
        u32::from_le_bytes(b)
    }
    fn r64(&mut self, a: u64) -> u64 {
        let mut b = [0u8; 8];
        for (i, x) in b.iter_mut().enumerate() {
            *x = self.r8(a + i as u64);
        }
        u64::from_le_bytes(b)
    }
    fn r128(&mut self, a: u64) -> u128 {
        let lo = self.r64(a) as u128;
        let hi = self.r64(a + 8) as u128;
        lo | (hi << 64)
    }
    fn rf32(&mut self, a: u64) -> f32 {
        f32::from_bits(self.r32(a))
    }
    fn rf64(&mut self, a: u64) -> f64 {
        f64::from_bits(self.r64(a))
    }
    fn w16(&mut self, a: u64, v: u16) {
        for (i, x) in v.to_le_bytes().iter().enumerate() {
            self.w8(a + i as u64, *x);
        }
    }
    fn w32(&mut self, a: u64, v: u32) {
        for (i, x) in v.to_le_bytes().iter().enumerate() {
            self.w8(a + i as u64, *x);
        }
    }
    fn w64(&mut self, a: u64, v: u64) {
        for (i, x) in v.to_le_bytes().iter().enumerate() {
            self.w8(a + i as u64, *x);
        }
    }
    fn w128(&mut self, a: u64, v: u128) {
        self.w64(a, v as u64);
        self.w64(a + 8, (v >> 64) as u64);
    }
    fn wf32(&mut self, a: u64, v: f32) {
        self.w32(a, v.to_bits());
    }
    fn wf64(&mut self, a: u64, v: f64) {
        self.w64(a, v.to_bits());
    }
}

#[derive(Debug, Clone)]
pub struct Header {
    pub name: String,
    pub entry: u64,
    pub n: u64,
    pub anchor: u64,
    pub stub_zone: u64,
    pub seed: u64,
    pub ptrn: u64,
    pub aligned_heap: bool,
    /// Vectors with `fetch=byte`: a call into a world page turns only the target byte into `ret`. Without it the
    /// whole page became `ret` bytes (older vectors).
    pub fetch_byte: bool,
    /// `ovr TARGET ARG DELTA` lines: a call to TARGET returns argument ARG plus DELTA in rax (assumption declared in the spec).
    pub ret_overrides: Vec<(u64, usize, i64)>,
}

#[derive(Debug, Clone)]
pub enum WArg {
    Int { bits: u32, value: u64 },
    F32(u32),
    F64(u64),
    Buf { size: usize, off: usize, ptr: u64, data: Vec<u8> },
}

#[derive(Debug, Clone)]
pub struct WCase {
    pub index: usize,
    pub status: String,
    pub seed: u64,
    pub abi_ok: bool,
    pub args: Vec<WArg>,
    pub rets: Vec<(String, u32, u128)>,
    pub writes: Vec<(u64, Vec<u8>)>,
    pub trace: Vec<TraceEv>,
    /// The original never returned (a declared noreturn call ended the run): return values are undefined.
    pub noreturn: bool,
    /// `pin ADDR VALUE` lines of the case: the world word at ADDR reads as VALUE.
    pub pins: Vec<(u64, u64)>,
}

impl WCase {
    pub fn ok(&self) -> bool {
        self.status == "ok"
    }

    /// Integer argument value, or the pointer (start + offset) of a buffer argument.
    pub fn int(&self, i: usize) -> u64 {
        match &self.args[i] {
            WArg::Int { value, .. } => *value,
            WArg::Buf { ptr, off, .. } => *ptr + *off as u64,
            other => panic!("arg {} is not an int or buffer: {:?}", i, other),
        }
    }

    pub fn f32_bits(&self, i: usize) -> u32 {
        match &self.args[i] {
            WArg::F32(b) => *b,
            other => panic!("arg {} is not an f32: {:?}", i, other),
        }
    }

    pub fn f64_bits(&self, i: usize) -> u64 {
        match &self.args[i] {
            WArg::F64(b) => *b,
            other => panic!("arg {} is not an f64: {:?}", i, other),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WorldVec {
    pub header: Header,
    pub cases: Vec<WCase>,
}

fn hex(s: &str) -> Result<u128, String> {
    let h = s.strip_prefix("0x").ok_or_else(|| format!("expected 0x prefix: {}", s))?;
    u128::from_str_radix(h, 16).map_err(|e| format!("bad hex {}: {}", s, e))
}

fn hex_bytes(s: &str) -> Result<Vec<u8>, String> {
    let h = s.strip_prefix("0x").ok_or_else(|| format!("expected 0x prefix: {}", s))?;
    if h.len() % 2 != 0 {
        return Err("odd hex length".to_string());
    }
    (0..h.len()).step_by(2).map(|i| u8::from_str_radix(&h[i..i + 2], 16).map_err(|e| e.to_string())).collect()
}

fn kv_hex(t: &str, key: &str) -> Option<u64> {
    t.strip_prefix(key).and_then(|v| v.strip_prefix("0x")).and_then(|v| u64::from_str_radix(v, 16).ok())
}

pub fn parse(text: &str) -> Result<WorldVec, String> {
    let mut header: Option<Header> = None;
    let mut cases = Vec::new();
    let mut cur: Option<WCase> = None;
    for (ln, line) in text.lines().enumerate() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let at = |m: String| format!("line {}: {}", ln + 1, m);
        let toks: Vec<&str> = line.split_whitespace().collect();
        match toks[0] {
            "world" => {
                let mut h = Header { name: toks[1].to_string(), entry: 0, n: 0, anchor: 0, stub_zone: 0, seed: 0, ptrn: 6, aligned_heap: false, fetch_byte: false, ret_overrides: Vec::new() };
                for t in &toks[2..] {
                    if let Some(v) = kv_hex(t, "entry=") {
                        h.entry = v;
                    } else if let Some(v) = kv_hex(t, "anchor=") {
                        h.anchor = v;
                    } else if let Some(v) = kv_hex(t, "stub_end=") {
                        h.stub_zone = v;
                    } else if let Some(v) = t.strip_prefix("n=") {
                        h.n = v.parse().map_err(|_| at("bad n".into()))?;
                    } else if let Some(v) = t.strip_prefix("seed=") {
                        h.seed = v.parse().map_err(|_| at("bad seed".into()))?;
                    } else if *t == "heap=aligned" {
                        h.aligned_heap = true;
                    } else if *t == "fetch=byte" {
                        h.fetch_byte = true;
                    } else if let Some(v) = t.strip_prefix("ptrn=") {
                        h.ptrn = v.parse().map_err(|_| at("bad ptrn".into()))?;
                    }
                }
                header = Some(h);
            }
            "case" => {
                let index: usize = toks[1].parse().map_err(|_| at("bad case index".into()))?;
                let last = toks.last().unwrap();
                let seed = kv_hex(last, "seed=").ok_or_else(|| at("missing seed".into()))?;
                cur = Some(WCase {
                    index,
                    status: toks[2..toks.len() - 1].join(" "),
                    seed,
                    abi_ok: true,
                    args: Vec::new(),
                    rets: Vec::new(),
                    writes: Vec::new(),
                    trace: Vec::new(),
                    noreturn: false,
                    pins: Vec::new(),
                });
            }
            "a" => {
                let c = cur.as_mut().ok_or_else(|| at("a outside case".into()))?;
                let arg = match toks[2] {
                    "int" => WArg::Int { bits: toks[3].parse().map_err(|_| at("bad bits".into()))?, value: hex(toks[4]).map_err(at)? as u64 },
                    "f32" => WArg::F32(hex(toks[3]).map_err(at)? as u32),
                    "f64" => WArg::F64(hex(toks[3]).map_err(at)? as u64),
                    "buf" => {
                        let size: usize = toks[3].parse().map_err(|_| at("bad size".into()))?;
                        let off: usize = toks[4].parse().map_err(|_| at("bad off".into()))?;
                        let ptr = kv_hex(toks[6], "ptr=").ok_or_else(|| at("missing ptr".into()))?;
                        let data = hex_bytes(toks.get(7).copied().unwrap_or("0x")).map_err(at)?;
                        WArg::Buf { size, off, ptr, data }
                    }
                    other => return Err(at(format!("unknown arg kind {}", other))),
                };
                c.args.push(arg);
            }
            "abi" => {
                cur.as_mut().ok_or_else(|| at("abi outside case".into()))?.abi_ok = toks.get(1) == Some(&"ok");
            }
            "r" => {
                let c = cur.as_mut().ok_or_else(|| at("r outside case".into()))?;
                let bits = (toks[3].len().saturating_sub(2) * 4) as u32;
                c.rets.push((toks[2].to_string(), bits, hex(toks[3]).map_err(at)?));
            }
            "w" => {
                let c = cur.as_mut().ok_or_else(|| at("w outside case".into()))?;
                c.writes.push((hex(toks[1]).map_err(at)? as u64, hex_bytes(toks[2]).map_err(at)?));
            }
            "t" => {
                let c = cur.as_mut().ok_or_else(|| at("t outside case".into()))?;
                let target = hex(toks[2]).map_err(at)? as u64;
                let ni: usize = toks[4].parse().map_err(|_| at("bad arity".into()))?;
                let iargs = toks[5..5 + ni].iter().map(|t| hex(t).map(|v| v as u64)).collect::<Result<Vec<_>, _>>().map_err(at)?;
                let nf: usize = toks[6 + ni].parse().map_err(|_| at("bad farity".into()))?;
                let fargs = toks[7 + ni..7 + ni + nf].iter().map(|t| hex(t).map(|v| v as u64)).collect::<Result<Vec<_>, _>>().map_err(at)?;
                c.trace.push(TraceEv { target, iargs, fargs });
            }
            "ovr" => {
                let h = header.as_mut().ok_or_else(|| at("ovr before the world header".into()))?;
                let t = hex(toks[1]).map_err(at)? as u64;
                let ai: usize = toks[2].parse().map_err(|_| at("bad ovr arg".into()))?;
                let d: i64 = toks[3].parse().map_err(|_| at("bad ovr delta".into()))?;
                h.ret_overrides.push((t, ai, d));
            }
            "x" => {
                cur.as_mut().ok_or_else(|| at("x outside case".into()))?.noreturn = toks.get(1) == Some(&"noreturn");
            }
            "pin" => {
                let c = cur.as_mut().ok_or_else(|| at("pin outside case".into()))?;
                c.pins.push((hex(toks[1]).map_err(at)? as u64, hex(toks[2]).map_err(at)? as u64));
            }
            "end" => cases.push(cur.take().ok_or_else(|| at("end outside case".into()))?),
            other => return Err(at(format!("unknown tag {}", other))),
        }
    }
    Ok(WorldVec { header: header.ok_or("missing world header")?, cases })
}

pub fn load(path: &Path) -> Result<WorldVec, String> {
    let text = std::fs::read_to_string(path).map_err(|e| format!("{}: {}", path.display(), e))?;
    parse(&text)
}

pub struct World<'a> {
    pub lay: Layout,
    seed: u64,
    code: Option<&'a [u8]>,
    bufs: Vec<(u64, Vec<u8>)>,
    overlay: HashMap<u64, u8>,
    written: BTreeSet<u64>,
    counts: HashMap<u64, u64>,
    trace: Vec<TraceEv>,
    fetch_page_wide: bool,
    fetched_pages: BTreeSet<u64>,
    ovr: HashMap<u64, (usize, i64)>,
}

impl<'a> World<'a> {
    pub fn new(h: &Header, c: &WCase, code: Option<&'a [u8]>) -> World<'a> {
        let mut lay = Layout::new(h.n, h.stub_zone, h.ptrn);
        if h.aligned_heap {
            lay = lay.with_aligned_heap();
        }
        lay.pins = c.pins.clone();
        let mut bufs = Vec::new();
        for a in &c.args {
            if let WArg::Buf { size, ptr, data, .. } = a {
                if *size == 0 {
                    continue;
                }
                let rsz = page_up(*size as u64);
                let mut v = vec![0u8; (rsz - *size as u64) as usize];
                v.extend_from_slice(data);
                bufs.push((*ptr + *size as u64 - rsz, v));
            }
        }
        World { lay, seed: c.seed, code, bufs, overlay: HashMap::new(), written: BTreeSet::new(), counts: HashMap::new(), trace: Vec::new(), fetch_page_wide: !h.fetch_byte, fetched_pages: BTreeSet::new(), ovr: h.ret_overrides.iter().map(|&(t, a, d)| (t, (a, d))).collect() }
    }

    fn init_byte(&self, a: u64) -> u8 {
        for (base, data) in &self.bufs {
            if a >= *base && a < *base + data.len() as u64 {
                return data[(a - *base) as usize];
            }
        }
        if a >= A0 && a < self.lay.anchor {
            return match self.code {
                Some(c) => c[(a - A0) as usize],
                None => fault(format!("code read at {:#x} but no code image was given", a)),
            };
        }
        if a >= self.lay.anchor && a < self.lay.stub_end_abs {
            return 0xc3;
        }
        if self.lay.in_world(a) {
            return (self.lay.word(self.seed, a & !7) >> (8 * (a & 7))) as u8;
        }
        if a >= STACK_BASE && a < STACK_TOP + 0x1000 {
            // The original runs on a zero-initialised stack; reads outside the port's own locals see zeros.
            return 0;
        }
        fault(format!("access outside modeled memory at {:#x}", a))
    }

    fn writable(&self, a: u64) -> bool {
        self.lay.in_world(a) || self.bufs.iter().any(|(b, d)| a >= *b && a < *b + d.len() as u64)
    }

    /// Final non-stack writes (bytes that differ from the initial world, coalesced) and the call trace.
    pub fn finish(&self) -> (Vec<(u64, Vec<u8>)>, Vec<TraceEv>) {
        let mut runs: Vec<(u64, Vec<u8>)> = Vec::new();
        for &a in &self.written {
            let v = self.overlay[&a];
            if v == self.init_byte(a) {
                continue;
            }
            match runs.last_mut() {
                Some((s, d)) if *s + d.len() as u64 == a => d.push(v),
                _ => runs.push((a, vec![v])),
            }
        }
        (runs, self.trace.clone())
    }
}

impl<'a> Env for World<'a> {
    fn r8(&mut self, a: u64) -> u8 {
        match self.overlay.get(&a) {
            Some(v) => *v,
            None if self.fetch_page_wide && self.fetched_pages.contains(&(a & !0xfff)) => 0xc3,
            None => self.init_byte(a),
        }
    }

    fn w8(&mut self, a: u64, v: u8) {
        if !self.writable(a) {
            fault(format!("write to non-writable address {:#x}", a));
        }
        self.overlay.insert(a, v);
        self.written.insert(a);
    }

    fn call_f(&mut self, target: u64, iargs: &[u64], fargs: &[u64]) -> CallRet {
        if target >= self.lay.stub_end_abs && target < self.lay.world_hi {
            // The original executes a `ret` there: only that byte (or, in old vectors, the whole page) changes.
            if self.fetch_page_wide {
                let p = target & !0xfff;
                self.fetched_pages.insert(p);
                for (a, v) in self.overlay.iter_mut() {
                    if *a & !0xfff == p {
                        *v = 0xc3;
                    }
                }
            } else {
                self.overlay.insert(target, 0xc3);
            }
        }
        let idx = *self.counts.get(&target).unwrap_or(&0);
        self.counts.insert(target, idx + 1);
        let ni: Vec<u64> = iargs.iter().map(|&v| norm(v)).collect();
        self.trace.push(TraceEv { target, iargs: ni.clone(), fargs: fargs.to_vec() });
        let mut ah = 0u64;
        for &a in ni.iter().chain(fargs.iter()) {
            ah = mix(ah, a);
        }
        let h = mix(mix(mix(self.seed, target), idx), ah);
        let mut rax = match h & 7 {
            0 | 1 => 0,
            2 | 3 => (h >> 8) & 0xff,
            4 | 5 => self.lay.heap((h >> 8) % HEAP_SLOTS),
            _ => mix(h, 11),
        };
        if let Some(&(ai, d)) = self.ovr.get(&target) {
            if ai < ni.len() {
                rax = ni[ai].wrapping_add(d as u64);
            }
        }
        CallRet { rax, rdx: mix(h, 12), xmm0_lo: mix(h, 13), xmm0_hi: mix(h, 14) }
    }

    fn code_va(&self, off: u64) -> u64 {
        A0 + off
    }

    fn data_va(&self, rel: i64) -> u64 {
        (self.lay.anchor as i64 + rel) as u64
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    pub name: String,
    pub total: usize,
    pub compared: usize,
    pub skipped: usize,
    pub failed: usize,
    /// Cases where the Rust port touched memory the model does not provide (counted as failures).
    pub rust_faults: usize,
    pub mismatches: Vec<String>,
}

impl Report {
    pub fn assert_ok(&self, min_compared: usize) {
        if self.failed > 0 || self.compared < min_compared {
            panic!(
                "{}: {} of {} compared worlds failed ({} skipped, need at least {} compared)\n{}",
                self.name,
                self.failed,
                self.compared,
                self.skipped,
                min_compared,
                self.mismatches.join("\n")
            );
        }
    }
}

fn f32_same(a: u32, b: u32) -> bool {
    a == b || (f32::from_bits(a).is_nan() && f32::from_bits(b).is_nan())
}

fn f64_same(a: u64, b: u64) -> bool {
    a == b || (f64::from_bits(a).is_nan() && f64::from_bits(b).is_nan())
}

/// Runs `f` once per ok world. `f` returns one value per `ret` of the spec (narrow values zero-extended).
pub fn check<'a>(vf: &WorldVec, code: Option<&'a [u8]>, mut f: impl FnMut(&mut World<'a>, &WCase) -> Vec<u128>) -> Report {
    let mut rep = Report { name: vf.header.name.clone(), total: vf.cases.len(), compared: 0, skipped: 0, failed: 0, rust_faults: 0, mismatches: Vec::new() };
    for c in &vf.cases {
        if !c.ok() {
            rep.skipped += 1;
            continue;
        }
        rep.compared += 1;
        let mut w = World::new(&vf.header, c, code);
        let mut errs: Vec<String> = Vec::new();
        match catch_unwind(AssertUnwindSafe(|| f(&mut w, c))) {
            Err(payload) => {
                rep.rust_faults += 1;
                let msg = match payload.downcast_ref::<Fault>() {
                    Some(Fault(m)) => format!("port faulted: {}", m),
                    None => "port panicked".to_string(),
                };
                errs.push(msg);
            }
            Ok(rets) => {
                if c.noreturn {
                    // the original never returns, so there are no return values to compare
                } else if rets.len() != c.rets.len() {
                    errs.push(format!("expected {} return values, got {}", c.rets.len(), rets.len()));
                } else {
                    for (j, (g, (spec, bits, want))) in rets.iter().zip(&c.rets).enumerate() {
                        let mask = if *bits >= 128 { u128::MAX } else { (1u128 << bits) - 1 };
                        let g = *g & mask;
                        let same = if spec.ends_with(":f32") {
                            f32_same(g as u32, *want as u32)
                        } else if spec.ends_with(":f64") {
                            f64_same(g as u64, *want as u64)
                        } else {
                            g == *want
                        };
                        if !same {
                            errs.push(format!("ret {} ({}) got 0x{:x} want 0x{:x}", j, spec, g, want));
                        }
                    }
                }
                let (runs, trace) = w.finish();
                if runs != c.writes {
                    let k = runs.iter().zip(&c.writes).position(|(a, b)| a != b).unwrap_or(runs.len().min(c.writes.len()));
                    errs.push(format!(
                        "memory writes differ at run {} (got {} runs, want {}): got {:?} want {:?}",
                        k,
                        runs.len(),
                        c.writes.len(),
                        runs.get(k).map(|r| (format!("{:#x}", r.0), r.1.len())),
                        c.writes.get(k).map(|r| (format!("{:#x}", r.0), r.1.len()))
                    ));
                }
                if trace != c.trace {
                    let k = trace.iter().zip(&c.trace).position(|(a, b)| a != b).unwrap_or(trace.len().min(c.trace.len()));
                    errs.push(format!(
                        "call trace differs at event {} (got {} calls, want {}): got {:x?} want {:x?}",
                        k,
                        trace.len(),
                        c.trace.len(),
                        trace.get(k),
                        c.trace.get(k)
                    ));
                }
            }
        }
        if !errs.is_empty() {
            rep.failed += 1;
            if rep.mismatches.len() < 20 {
                rep.mismatches.push(format!("world {} (seed 0x{:x}): {}", c.index, c.seed, errs.join("; ")));
            }
        }
    }
    rep
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_matches_python_reference_values() {
        // Reference values computed with tools/world.py (sm64 and mix).
        assert_eq!(sm64(0), 0xe220a8397b1dcdaf);
        assert_eq!(mix(1, 2), mix(1, 2));
        assert_ne!(mix(1, 2), mix(2, 1));
    }

    #[test]
    fn layout_zones() {
        let l = Layout::new(0x40, 0x1000, 6);
        assert_eq!(l.anchor, A0 + 0x40);
        assert!(l.in_world(A0 - 1));
        assert!(!l.in_world(A0));
        assert!(!l.in_world(l.anchor));
        assert!(l.in_world(l.stub_end_abs));
        assert!(!l.in_world(l.world_hi));
    }
}
