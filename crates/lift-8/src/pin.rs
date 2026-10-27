//! Pinned world words (a local extension; the shared harness is untouched). `scripts/pinpatch.py` makes the original read
//! chosen 8 byte values at chosen offsets from the anchor instead of the seeded pseudo-random ones, and writes the
//! list next to the vectors (`<vec>.pins`, lines `data 0xAT kind 0xX`, kind data | heap | int). [`PinEnv`] gives a port the
//! same view: reads of an untouched pinned byte return the pinned value, everything else goes to the [`World`].

use neutral_world::{CallRet, Env, World};
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Pin {
    pub at: u64,
    pub kind: String,
    pub x: u64,
}

thread_local! {
    static PINS: RefCell<Vec<Pin>> = RefCell::new(Vec::new());
}

/// Reads a sidecar file; a missing file means no pins.
pub fn load_pins(path: &Path) -> Vec<Pin> {
    let Ok(text) = std::fs::read_to_string(path) else {
        crate::x86rt::set_allocs(Vec::new());
        return Vec::new();
    };
    let hex = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
    let mut allocs = Vec::new();
    let mut pins = Vec::new();
    for l in text.lines().filter(|l| !l.trim().is_empty()) {
        let t: Vec<&str> = l.split_whitespace().collect();
        if t[0] == "alloc" {
            allocs.push(hex(t[1]));
            continue;
        }
        assert_eq!(t[0], "data", "bad pin line {}", l);
        pins.push(Pin { at: hex(t[1]), kind: t[2].to_string(), x: hex(t[3]) });
    }
    crate::x86rt::set_allocs(allocs);
    pins
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
        let pins = PINS.with(|c| {
            c.borrow()
                .iter()
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
            for (p0, b) in &self.pins {
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
