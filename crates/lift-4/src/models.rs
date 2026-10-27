//! Write models for opaque stubs (a local extension; the shared harness is untouched). A spec may declare
//! `"models": {"stub:0x7b1": "string_ctor", ...}`; `scripts/pinpatch.py` applies the same rules to the original run and writes
//! the sidecar `<vec>.models` (lines `stub 0x7b1 string_ctor`) that [`load_models`] reads. [`crate::x86rt::Mem::call`] applies the
//! model after the normal call (trace and clobbers unchanged). All models are ASSUMPTIONS about the stubs (high confidence roles
//! from findings/structure/stubs-roles.tsv for 0x7b1, 0x913 and 0x9eb); the ledger notes say so.
//!
//! - `string_ctor`: std::string(const char* src, size_t len) at this = arg0 (libc++ layout). len <= 22: byte 0 = len*2, the chars inline,
//!   the rest zero (24 bytes). Longer (up to 0x400): {cap|1, len, ptr} with cap = (len+16) & !15 and ptr a world heap slot picked by the
//!   call result rdx; chars and a NUL are copied to ptr. Longer than 0x400: nothing is written.
//! - `string_long`: same, but the long layout is built for every length (a HYPOTHETICAL constructor, only to run the long-string paths).
//! - `alloc`: operator new, rax = a world heap pointer (slot picked by the call result rdx).
//! - `retarg0`: rax = the first argument as the caller passed it (a stack address is NOT normalised to STK).
//! - `memcpy`: memmove(dst, src, n) for n <= 0x2000, rax = dst.
//! Readable and writable memory for a model means the emulator stack or the world zone [anchor+0x10000, anchor+WABOVE-64); anything
//! else is skipped (zeros are read, nothing is written), on both sides.

use std::cell::RefCell;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    StringCtor,
    /// Hypothetical constructor that always builds the long layout (exercises the long-string code paths).
    StringLong,
    /// operator new and any other stub or function whose result is used as a pointer: rax is a world heap pointer.
    Alloc,
    Memcpy,
    /// Returns its first argument (the real one: a stack address stays a stack address): operator<< on a stream returns the stream.
    RetArg0,
}

pub const HEAP_SLOTS: u64 = 1 << 16;
pub const STR_MAX: u64 = 0x400;
pub const CPY_MAX: u64 = 0x2000;

thread_local! {
    static MODELS: RefCell<Vec<(bool, u64, Kind)>> = RefCell::new(Vec::new());
    static OUTS: RefCell<Vec<Out>> = RefCell::new(Vec::new());
    static STUB_ZONE: RefCell<u64> = RefCell::new(0);
    /// fleet-4: result values of indirect calls (spec "indres"); the pick is a hash of the call result rdx with two extra "keep" slots.
    static INDRES: RefCell<Vec<(Key, Vec<u64>, bool)>> = RefCell::new(Vec::new());
}

/// Which call targets an out-parameter rule applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    /// Every target that is neither a stub nor a function of the dump (an indirect call through a world word).
    Ind,
    Stub(u64),
    Fn(u64),
}

/// After a call matching `key`, the callee has written pseudo-random values ([`outval`]) into `fields` (relative offset, size) of the
/// object that integer argument `arg` points to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Out {
    pub key: Key,
    pub arg: usize,
    pub fields: Vec<(u64, u64)>,
    /// The callee built a libc++ __tree (std::map / std::set) in the object: {begin, root, size} plus 0..3 world heap nodes.
    pub tree: bool,
    /// fleet-4: the callee writes one of these values into every field (picked by a hash of the call result rdx), instead of `outval`.
    pub vals: Vec<u64>,
}

/// Lines `out <key> <arg> <rel>:<size>,<rel>:<size>` with key `ind`, `stub:0x1d5` or `fn:0x2fdd8`.
pub fn parse_outs(text: &str) -> Vec<Out> {
    text.lines()
        .filter(|l| l.starts_with("out "))
        .map(|l| {
            let t: Vec<&str> = l.split_whitespace().collect();
            let hex = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
            let key = if t[1] == "ind" {
                Key::Ind
            } else if let Some(r) = t[1].strip_prefix("stub:") {
                Key::Stub(hex(r))
            } else if let Some(r) = t[1].strip_prefix("fn:") {
                Key::Fn(hex(r))
            } else {
                panic!("bad out key {}", t[1])
            };
            let tree = t[3] == "tree";
            let fields = if tree {
                Vec::new()
            } else {
                t[3].split(',')
                    .map(|f| {
                        let (a, b) = f.split_once(':').unwrap();
                        (a.parse().unwrap(), b.parse().unwrap())
                    })
                    .collect()
            };
            let vals = t
                .get(4)
                .and_then(|x| x.strip_prefix("vals="))
                .map(|x| x.split(',').map(|v| hex(v)).collect())
                .unwrap_or_default();
            Out { key, arg: t[2].parse().unwrap(), fields, tree, vals }
        })
        .collect()
}

/// Value of the j-th 8 byte chunk written to an out-parameter (identical to pinpatch.py outval): small flag-like numbers, zero, 16 bit
/// words, all ones and full words, picked by a hash of the call result rdx.
pub fn outval(rdx: u64, j: u64) -> u64 {
    let h = neutral_world::mix(rdx, 0x1000 + j);
    match h & 7 {
        0..=2 => (h >> 8) & 0xf,
        3 => 0,
        4 => (h >> 8) & 0xffff,
        5 => u64::MAX,
        _ => neutral_world::mix(h, 7),
    }
}

/// Size of the stub zone of the vector set (header `stub_end`): calls to anchor + zone or above are indirect targets.
pub fn set_stub_zone(z: u64) {
    STUB_ZONE.with(|c| *c.borrow_mut() = z);
}

pub fn stub_zone() -> u64 {
    STUB_ZONE.with(|c| *c.borrow())
}

pub fn set_outs(o: Vec<Out>) {
    OUTS.with(|c| *c.borrow_mut() = o);
}

pub fn outs() -> Vec<Out> {
    OUTS.with(|c| c.borrow().clone())
}

/// Loads the stub models and the out-parameter rules from a `.models` sidecar and installs both. A missing file means none.
pub fn install_from(path: &Path) {
    let text = std::fs::read_to_string(path).unwrap_or_default();
    set_models(parse_models(&text));
    set_outs(parse_outs(&text));
    set_indres(parse_indres(&text));
}

/// Lines `indres 0x10478300,...` (key ind) and `res <ind|stub:0x1d5|fn:0x2fdd8> 0x..,0x..` (fleet-4).
pub fn parse_indres(text: &str) -> Vec<(Key, Vec<u64>, bool)> {
    let vals = |s: &str| -> Vec<u64> { s.split(',').map(|v| u64::from_str_radix(v.trim().trim_start_matches("0x"), 16).unwrap()).collect() };
    let mut out = Vec::new();
    for l in text.lines() {
        if let Some(r) = l.strip_prefix("indres ") {
            out.push((Key::Ind, vals(r), false));
        } else if let Some(r) = l.strip_prefix("res ").or_else(|| l.strip_prefix("resf ")) {
            let fixed = l.starts_with("resf ");
            let (k, v) = r.split_once(' ').unwrap();
            let key = if k == "ind" {
                Key::Ind
            } else if let Some(x) = k.strip_prefix("stub:") {
                Key::Stub(u64::from_str_radix(x.trim_start_matches("0x"), 16).unwrap())
            } else if let Some(x) = k.strip_prefix("fn:") {
                Key::Fn(u64::from_str_radix(x.trim_start_matches("0x"), 16).unwrap())
            } else {
                panic!("bad res key {}", k)
            };
            out.push((key, vals(v), fixed));
        }
    }
    out
}

pub fn set_indres(v: Vec<(Key, Vec<u64>, bool)>) {
    INDRES.with(|c| *c.borrow_mut() = v);
}

pub fn indres() -> Vec<(Key, Vec<u64>, bool)> {
    INDRES.with(|c| c.borrow().clone())
}

/// Lines `stub 0x7b1 string_ctor` or `fn 0x72d00 ptrret`: (is a function of the dump, offset, kind). `out` lines are not models.
pub fn parse_models(text: &str) -> Vec<(bool, u64, Kind)> {
    text.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with("out ") && !l.starts_with("indres ") && !l.starts_with("res ") && !l.starts_with("resf "))
        .map(|l| {
            let t: Vec<&str> = l.split_whitespace().collect();
            assert!(t[0] == "stub" || t[0] == "fn", "bad model line {}", l);
            let rel = u64::from_str_radix(t[1].trim_start_matches("0x"), 16).unwrap();
            let kind = match t[2] {
                "string_ctor" => Kind::StringCtor,
                "string_long" => Kind::StringLong,
                "alloc" | "ptrret" => Kind::Alloc,
                "memcpy" => Kind::Memcpy,
                "retarg0" => Kind::RetArg0,
                k => panic!("unknown model {}", k),
            };
            (t[0] == "fn", rel, kind)
        })
        .collect()
}

/// A missing file means no models.
pub fn load_models(path: &Path) -> Vec<(bool, u64, Kind)> {
    match std::fs::read_to_string(path) {
        Ok(t) => parse_models(&t),
        Err(_) => Vec::new(),
    }
}

pub fn set_models(m: Vec<(bool, u64, Kind)>) {
    MODELS.with(|c| *c.borrow_mut() = m);
}

/// Stub offsets (rel to the end of the dump) that have a model right now.
pub fn current() -> Vec<(bool, u64, Kind)> {
    MODELS.with(|c| c.borrow().clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sidecar_lines_parse() {
        let m = parse_models("stub 0x7b1 string_ctor\nstub 0x913 alloc\nstub 0x9eb memcpy\nfn 0x72d00 ptrret\n");
        assert_eq!(
            m,
            vec![(false, 0x7b1, Kind::StringCtor), (false, 0x913, Kind::Alloc), (false, 0x9eb, Kind::Memcpy), (true, 0x72d00, Kind::Alloc)]
        );
        assert!(parse_models("\n").is_empty());
        let t = "stub 0x7b1 string_ctor\nout ind 2 0:4\nout stub:0x1d5 1 48:16,0:8\nout fn:0x2fdd8 2 0:8,8:8\n";
        assert_eq!(parse_models(t).len(), 1);
        let o = parse_outs(t);
        assert_eq!(o.len(), 3);
        assert_eq!(o[0], Out { key: Key::Ind, arg: 2, fields: vec![(0, 4)], tree: false, vals: vec![] });
        assert_eq!(o[1], Out { key: Key::Stub(0x1d5), arg: 1, fields: vec![(48, 16), (0, 8)], tree: false, vals: vec![] });
        let tt = parse_outs("out fn:0x63dd3 0 tree
");
        assert_eq!(tt, vec![Out { key: Key::Fn(0x63dd3), arg: 0, fields: vec![], tree: true, vals: vec![] }]);
        assert_eq!(o[2].key, Key::Fn(0x2fdd8));
    }

    #[test]
    fn outval_follows_the_hash_classes() {
        let mut seen = [false; 5];
        for r in 0..400u64 {
            let h = neutral_world::mix(r, 0x1000);
            let v = outval(r, 0);
            match h & 7 {
                0..=2 => {
                    assert!(v <= 15);
                    seen[0] = true;
                }
                3 => {
                    assert_eq!(v, 0);
                    seen[1] = true;
                }
                4 => {
                    assert!(v <= 0xffff);
                    seen[2] = true;
                }
                5 => {
                    assert_eq!(v, u64::MAX);
                    seen[3] = true;
                }
                _ => {
                    assert_eq!(v, neutral_world::mix(h, 7));
                    seen[4] = true;
                }
            }
        }
        assert!(seen.iter().all(|x| *x));
    }
}
