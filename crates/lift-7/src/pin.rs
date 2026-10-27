//! Pinned world words (a local extension; the shared harness is untouched). `scripts/pinpatch.py` makes the original read
//! chosen 8 byte values at chosen offsets from the anchor instead of the seeded pseudo-random ones, and writes the
//! list next to the vectors (`<vec>.pins`, lines `data 0xAT kind 0xX`, kind data | heap | int). [`PinEnv`] gives a port the
//! same view: reads of an untouched pinned byte return the pinned value, everything else goes to the [`World`].

use crate::x86rt::{set_case_rets, set_case_wouts, set_retx, OutW, RetX};
use neutral_world::{CallRet, Env, World};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Pin {
    pub at: u64,
    pub kind: String,
    pub x: u64,
}

thread_local! {
    static PINS: RefCell<Vec<Pin>> = RefCell::new(Vec::new());
    /// per case overlays (sidecar lines `pinw CASE REL VALUE` and `retw CASE KEY IDX VALUE`)
    static CASE_PINS: RefCell<HashMap<u64, Vec<Pin>>> = RefCell::new(HashMap::new());
    static CASE_RETS: RefCell<HashMap<u64, Vec<(bool, u64, u64, u64)>>> = RefCell::new(HashMap::new());
    /// sidecar lines `woutc CASE KEY IDX ARG OFF SIZE VALUE`: callee writes into the stack frame after one call of a case
    static CASE_WOUT: RefCell<HashMap<u64, Vec<(bool, u64, u64, u64, u64, u64, u64)>>> = RefCell::new(HashMap::new());
}

fn parse_key(k: &str) -> (bool, u64) {
    // `stub:0xREL`, `fn:0xOFF`, or (older sidecars) a bare hex stub REL
    match k.split_once(':') {
        Some((kind, num)) => (kind == "fn", u64::from_str_radix(num.trim_start_matches("0x"), 16).unwrap()),
        None => (false, u64::from_str_radix(k.trim_start_matches("0x"), 16).unwrap()),
    }
}

/// Reads a sidecar file: pins, raw call results and out-parameter updates; a missing file clears all of them.
pub fn load_sidecar(path: &Path) {
    set_pins(load_pins(path));
    let hex = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
    let mut r = Vec::new();
    let mut w = Vec::new();
    let mut cp: HashMap<u64, Vec<Pin>> = HashMap::new();
    let mut cr: HashMap<u64, Vec<(bool, u64, u64, u64)>> = HashMap::new();
    let mut cw: HashMap<u64, Vec<(bool, u64, u64, u64, u64, u64, u64)>> = HashMap::new();
    if let Ok(text) = std::fs::read_to_string(path) {
        for l in text.lines() {
            let t: Vec<&str> = l.split_whitespace().collect();
            if t.len() == 4 && t[0] == "pinw" {
                cp.entry(t[1].parse().unwrap()).or_default().push(Pin { at: t[2].parse::<i64>().unwrap() as u64, kind: "int".to_string(), x: hex(t[3]) });
                continue;
            }
            if t.len() == 5 && t[0] == "retw" {
                let (is_fn, rel) = parse_key(t[2]);
                cr.entry(t[1].parse().unwrap()).or_default().push((is_fn, rel, t[3].parse().unwrap(), hex(t[4])));
                continue;
            }
            if t.len() == 8 && t[0] == "woutc" {
                let (is_fn, rel) = parse_key(t[2]);
                cw.entry(t[1].parse().unwrap()).or_default().push((is_fn, rel, t[3].parse().unwrap(), t[4].parse().unwrap(), t[5].parse().unwrap(), t[6].parse().unwrap(), hex(t[7])));
                continue;
            }
            if t.len() >= 3 && t[0] == "ret" {
                let (is_fn, rel) = parse_key(t[1]);
                match t[2] {
                    "arg" => r.push(RetX { is_fn, rel, mode: "arg".into(), a: t[3].parse().unwrap(), b: t[4].parse::<i64>().unwrap() as u64, seq: vec![] }),
                    "data" => r.push(RetX { is_fn, rel, mode: "data".into(), a: hex(t[3]), b: 0, seq: vec![] }),
                    "const" => r.push(RetX { is_fn, rel, mode: "const".into(), a: hex(t[3]), b: 0, seq: vec![] }),
                    "seq" => r.push(RetX { is_fn, rel, mode: "seq".into(), a: 0, b: 0, seq: t[3].split(',').map(hex).collect() }),
                    "mix" => r.push(RetX { is_fn, rel, mode: "mix".into(), a: 0, b: 0, seq: t[3].split(',').map(hex).collect() }),
                    k => panic!("bad ret mode {}", k),
                }
            } else if t.len() == 9 && t[0] == "outw" {
                let (is_fn, rel) = parse_key(t[1]);
                let num = |s: &str| s.parse::<i64>().unwrap() as u64;
                w.push(OutW { is_fn, rel, arg: t[2].parse().unwrap(), off: num(t[3]), kind: t[4].to_string(), val: num(t[5]), lo: num(t[6]), n: num(t[7]), sh: num(t[8]) });
            } else if t.len() == 6 && t[0] == "outw" {
                // older sidecars: outw REL ARG OFF KIND VALUE
                let (is_fn, rel) = parse_key(t[1]);
                let num = |s: &str| s.parse::<i64>().unwrap() as u64;
                w.push(OutW { is_fn, rel, arg: t[2].parse().unwrap(), off: num(t[3]), kind: t[4].to_string(), val: num(t[5]), lo: 0, n: 0, sh: 0 });
            }
        }
    }
    set_retx(r, w);
    CASE_PINS.with(|c| *c.borrow_mut() = cp);
    CASE_RETS.with(|c| *c.borrow_mut() = cr);
    CASE_WOUT.with(|c| *c.borrow_mut() = cw);
}

pub fn load_pins(path: &Path) -> Vec<Pin> {
    let Ok(text) = std::fs::read_to_string(path) else { return Vec::new() };
    let hex = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
    text.lines()
        .filter(|l| l.starts_with("data "))
        .map(|l| {
            let t: Vec<&str> = l.split_whitespace().collect();
            assert_eq!(t[0], "data", "bad pin line {}", l);
            Pin { at: hex(t[1]), kind: t[2].to_string(), x: hex(t[3]) }
        })
        .collect()
}

pub fn set_pins(p: Vec<Pin>) {
    PINS.with(|c| *c.borrow_mut() = p);
}

pub struct PinEnv<'a, 'w> {
    w: &'a mut World<'w>,
    /// (absolute address, 8 value bytes)
    pins: Vec<(u64, [u8; 8])>,
    wrote: HashSet<u64>,
}

impl<'a, 'w> PinEnv<'a, 'w> {
    pub fn new(w: &'a mut World<'w>) -> Self {
        set_case_rets(Vec::new());
        set_case_wouts(Vec::new());
        Self::build(w, Vec::new())
    }

    /// Environment for case `idx` of the vector file: the set's pins plus the case's own `pinw` words, and its `retw` call results.
    pub fn new_case(w: &'a mut World<'w>, idx: usize) -> Self {
        let extra = CASE_PINS.with(|c| c.borrow().get(&(idx as u64)).cloned().unwrap_or_default());
        set_case_rets(CASE_RETS.with(|c| c.borrow().get(&(idx as u64)).cloned().unwrap_or_default()));
        set_case_wouts(CASE_WOUT.with(|c| c.borrow().get(&(idx as u64)).cloned().unwrap_or_default()));
        Self::build(w, extra)
    }

    fn build(w: &'a mut World<'w>, extra: Vec<Pin>) -> Self {
        let pins = PINS.with(|c| {
            c.borrow()
                .iter()
                .chain(extra.iter())
                .map(|p| {
                    let v = match p.kind.as_str() {
                        "data" => w.lay.anchor.wrapping_add(p.x),
                        "heap" => w.lay.heap(p.x),
                        "int" => p.x,
                        k => panic!("bad pin kind {}", k),
                    };
                    (w.lay.anchor.wrapping_add(p.at), v.to_le_bytes())
                })
                .collect()
        });
        PinEnv { w, pins, wrote: HashSet::new() }
    }
}

impl<'a, 'w> Env for PinEnv<'a, 'w> {
    fn r8(&mut self, a: u64) -> u8 {
        if !self.wrote.contains(&a) {
            for (p0, b) in self.pins.iter().rev() {
                if a >= *p0 && a < *p0 + 8 {
                    return b[(a - *p0) as usize];
                }
            }
        }
        self.w.r8(a)
    }
    fn w8(&mut self, a: u64, v: u8) {
        self.wrote.insert(a);
        self.w.w8(a, v)
    }
    fn call_f(&mut self, target: u64, iargs: &[u64], fargs: &[u64]) -> CallRet {
        self.w.call_f(target, iargs, fargs)
    }
    fn code_va(&self, off: u64) -> u64 {
        self.w.code_va(off)
    }
    fn data_va(&self, rel: i64) -> u64 {
        self.w.data_va(rel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use neutral_world::parse;

    #[test]
    fn pinned_bytes_read_back_and_writes_win() {
        let text = "world t entry=0x0 n=100 anchor=0x10000064 stub_end=0x10 args=0 rets=0 seed=1 ptrn=6 heap=aligned fetch=byte\ncase 0 ok seed=0x1\nabi ok\nend\n";
        let vf = parse(text).unwrap();
        let mut plain = World::new(&vf.header, &vf.cases[0], None);
        let anchor = plain.lay.anchor;
        let seeded_far = plain.r8(anchor + 0x30000);
        let seeded_near = plain.r8(anchor + 0x30009);
        set_pins(vec![
            Pin { at: 0x30001, kind: "int".to_string(), x: 0x1122334455667788 },
            Pin { at: 0x30100, kind: "data".to_string(), x: 0x30200 },
        ]);
        let mut w = World::new(&vf.header, &vf.cases[0], None);
        let mut p = PinEnv::new(&mut w);
        // an unaligned 8 byte pin: little endian bytes at +0x30001 .. +0x30008, neighbours stay seeded
        assert_eq!(p.r64(anchor + 0x30001), 0x1122334455667788);
        assert_eq!(p.r8(anchor + 0x30002), 0x77);
        assert_eq!(p.r8(anchor + 0x30000), seeded_far);
        assert_eq!(p.r8(anchor + 0x30009), seeded_near);
        // a data pin holds anchor + rel
        assert_eq!(p.r64(anchor + 0x30100), anchor + 0x30200);
        // a store replaces one pinned byte, the others stay pinned
        p.w8(anchor + 0x30003, 0xaa);
        assert_eq!(p.r8(anchor + 0x30003), 0xaa);
        assert_eq!(p.r8(anchor + 0x30002), 0x77);
        assert_eq!(p.r8(anchor + 0x30004), 0x55);
        set_pins(Vec::new());
    }
}

/// Debug wrapper: forwards to another environment and prints every byte store as `W addr value` (env DBG_WRITES).
pub struct LogEnv<'a, E: Env> {
    pub inner: &'a mut E,
}

impl<'a, E: Env> Env for LogEnv<'a, E> {
    fn r8(&mut self, a: u64) -> u8 {
        self.inner.r8(a)
    }
    fn w8(&mut self, a: u64, v: u8) {
        println!("W {:x} {:02x}", a, v);
        self.inner.w8(a, v)
    }
    fn call_f(&mut self, target: u64, iargs: &[u64], fargs: &[u64]) -> CallRet {
        println!("C {:x}", target);
        self.inner.call_f(target, iargs, fargs)
    }
    fn code_va(&self, off: u64) -> u64 {
        self.inner.code_va(off)
    }
    fn data_va(&self, rel: i64) -> u64 {
        self.inner.data_va(rel)
    }
}
