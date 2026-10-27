//! Pinned world words (a local extension; the shared harness is untouched). `scripts/pinpatch.py` makes the original read
//! chosen 8 byte values at chosen offsets from the anchor instead of the seeded pseudo-random ones, and writes the
//! list next to the vectors (`<vec>.pins`, lines `data 0xAT kind 0xX`, kind data | heap | int). [`PinEnv`] gives a port the
//! same view: reads of an untouched pinned byte return the pinned value, everything else goes to the [`World`].

use neutral_world::{CallRet, Env, Fault, Report, WCase, World, WorldVec};
use std::cell::RefCell;
use std::collections::{BTreeMap, HashSet};
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Pin {
    pub at: u64,
    pub kind: String,
    pub x: u64,
}

thread_local! {
    static PINS: RefCell<Vec<Pin>> = RefCell::new(Vec::new());
    /// Final writes of the last [`PinEnv`] (bytes that differ from the pinned initial world, coalesced), set when it is dropped.
    static LAST_RUNS: RefCell<Option<Vec<(u64, Vec<u8>)>>> = RefCell::new(None);
}

/// Reads a sidecar file; a missing file means no pins.
pub fn load_pins(path: &Path) -> Vec<Pin> {
    let Ok(text) = std::fs::read_to_string(path) else { return Vec::new() };
    let hex = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
    text.lines()
        .filter(|l| !l.trim().is_empty())
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
    /// address -> (value before the first store, latest stored value), the initial world seen through the pins
    base: BTreeMap<u64, (u8, u8)>,
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
        PinEnv { w, pins, wrote: HashSet::new(), base: BTreeMap::new() }
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
        if let Some(e) = self.base.get_mut(&a) {
            e.1 = v;
        } else {
            let b0 = self.r8(a);
            self.base.insert(a, (b0, v));
        }
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

impl<'a, 'w> Drop for PinEnv<'a, 'w> {
    fn drop(&mut self) {
        let mut runs: Vec<(u64, Vec<u8>)> = Vec::new();
        for (&a, &(b0, v)) in &self.base {
            if v == b0 {
                continue;
            }
            match runs.last_mut() {
                Some((s, d)) if *s + d.len() as u64 == a => d.push(v),
                _ => runs.push((a, vec![v])),
            }
        }
        LAST_RUNS.with(|c| *c.borrow_mut() = Some(runs));
    }
}

fn f32_same(a: u32, b: u32) -> bool {
    a == b || (f32::from_bits(a).is_nan() && f32::from_bits(b).is_nan())
}

fn f64_same(a: u64, b: u64) -> bool {
    a == b || (f64::from_bits(a).is_nan() && f64::from_bits(b).is_nan())
}

/// `neutral_world::check` for ports that run on a [`PinEnv`]: the original compares its final memory with the initial world
/// including the pinned words, so the expected write runs are computed against that baseline instead of the seeded one
/// (a store of the pinned value is no write, a store of the seeded value over a pin is one).
pub fn check_pinned<'a>(vf: &WorldVec, code: Option<&'a [u8]>, mut f: impl FnMut(&mut World<'a>, &WCase) -> Vec<u128>) -> Report {
    let mut rep = Report { name: vf.header.name.clone(), total: vf.cases.len(), compared: 0, skipped: 0, failed: 0, rust_faults: 0, mismatches: Vec::new() };
    for c in &vf.cases {
        if !c.ok() {
            rep.skipped += 1;
            continue;
        }
        rep.compared += 1;
        let mut w = World::new(&vf.header, c, code);
        let mut errs: Vec<String> = Vec::new();
        LAST_RUNS.with(|l| *l.borrow_mut() = None);
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
                let (runs0, trace) = w.finish();
                let runs = LAST_RUNS.with(|l| l.borrow_mut().take()).unwrap_or(runs0);
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
