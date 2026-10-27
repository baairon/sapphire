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
//! - `find`: std::map::find, rax = arg0 + 8 (end()) for odd rdx, else a world heap node.
//! - `findnpos`: std::string::find, rax = u64::MAX unless rdx & 3 == 0, then (rdx >> 8) & 0xf.
//! - `memcpy`: memmove(dst, src, n) for n <= 0x2000, rax = dst.
//! - `memset`: memset(dst, c, n) for n <= 0x2000, rax = dst.
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
    /// memset(dst, c, n) for n <= 0x2000, rax = dst.
    Memset,
    /// std::getline: returns the stream; the iostate byte at stream + 0x30 becomes 6 (eof|fail) from call number K = mix(seed, 0x6cd) % 4 on.
    Getline,
    /// The identity on the first floating argument (xmm0 keeps its value): floor / ceil / round of an integral value.
    Fid,
    /// Returns its first argument (the real one: a stack address stays a stack address): operator<< on a stream returns the stream.
    RetArg0,
    /// std::map::find: rax = this + 8 (the end iterator) when the call result rdx is odd, else a world heap node.
    Find,
    /// std::string::find: npos (-1) unless the call result rdx has two zero low bits, then a small position (rdx >> 8) & 0xf.
    FindNpos,
    /// locale::use_facet(id): rax = the fixed world data block FACET_REL (its vtable words are pinned by the spec: fleet-6 scripts/modelcand.py facet_pins).
    Facet,
}

/// Anchor relative offset of the facet object returned by the `facet` model (kept in step with scripts/pinpatch.py FACET_REL).
pub const FACET_REL: u64 = 0x880000;

pub const HEAP_SLOTS: u64 = 1 << 16;
pub const STR_MAX: u64 = 0x400;
pub const CPY_MAX: u64 = 0x2000;

thread_local! {
    static RETC: RefCell<Vec<RetC>> = RefCell::new(Vec::new());
    static MODELS: RefCell<Vec<(bool, u64, Kind)>> = RefCell::new(Vec::new());
    static OUTS: RefCell<Vec<Out>> = RefCell::new(Vec::new());
    static STUB_ZONE: RefCell<u64> = RefCell::new(0);
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
    /// (relative offset, size, constants the code compares the value with)
    pub fields: Vec<(u64, u64, Vec<u64>)>,
    /// The callee built a libc++ __tree (std::map / std::set) in the object: {begin, root, size} plus 0..3 world heap nodes.
    pub tree: bool,
    /// The callee wrote a std::vector header {begin, end, cap}: begin a world heap slot, end = begin + 0..16, cap = end + 0..16 (sidecar `vec`).
    pub vec: bool,
    /// The callee built a libc++ __tree with this many chain nodes (right leaning chain n0 -> n1 -> ...); sidecar `chain:N`.
    pub chain: u64,
    /// Vector header with the begin field at this offset and the end field right after it (begin = heap slot, end = begin + stride * 0..3); sidecar `vecs:OFF:STRIDE`.
    pub vecs_off: u64,
    pub vecs_stride: u64,
    /// A balanced binary search tree with this many nodes (sidecar `bst:N`).
    pub bst: u64,
    /// An inconsistent header: begin == end node, no root, size N (sidecar `chainz:N`).
    pub chainz: u64,
    /// The rule only applies when the argument is a stack address (sidecar token `stack`).
    pub stack: bool,
    /// Half of the full words written are world heap pointers (sidecar token `heapv`).
    pub heapv: bool,
    /// Only the listed constants are written (sidecar token `cst`).
    pub cst: bool,
    /// The rule fires only when the argument address modulo 8 equals this value (sidecar `amod:N`); u64::MAX = always.
    pub amod: u64,
    /// Tree rules may also build the four node tree R(A, B(C)) (sidecar token `tree4`).
    pub tree4: bool,
}

/// In one call of eight (rdx >> 4 & 7 == 0) the target returns one of the constants the code compares its result with: number (rdx >> 8) % n.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetC {
    pub key: Key,
    pub consts: Vec<u64>,
}

fn parse_key(s: &str) -> Key {
    let hex = |s: &str| u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap();
    if s == "ind" {
        Key::Ind
    } else if let Some(r) = s.strip_prefix("stub:") {
        Key::Stub(hex(r))
    } else if let Some(r) = s.strip_prefix("fn:") {
        Key::Fn(hex(r))
    } else {
        panic!("bad key {}", s)
    }
}

/// Lines `ret <key> <c1>/<c2>/...` (decimal constants).
pub fn parse_retc(text: &str) -> Vec<RetC> {
    text.lines()
        .filter(|l| l.starts_with("ret "))
        .map(|l| {
            let t: Vec<&str> = l.split_whitespace().collect();
            RetC { key: parse_key(t[1]), consts: t[2].split('/').map(|x| x.parse().unwrap()).collect() }
        })
        .collect()
}

pub fn retcs() -> Vec<RetC> {
    RETC.with(|c| c.borrow().clone())
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
            let vec = t[3] == "vec";
            let chain: u64 = t[3].strip_prefix("chain:").map(|x| x.parse().unwrap()).unwrap_or(0);
            let (vecs_off, vecs_stride): (u64, u64) = match t[3].strip_prefix("vecs:") { Some(r) => { let p: Vec<&str> = r.split(':').collect(); (p[0].parse().unwrap(), p[1].parse().unwrap()) } None => (0, 0) };
            let bst: u64 = t[3].strip_prefix("bst:").map(|x| x.parse().unwrap()).unwrap_or(0);
            let chainz: u64 = t[3].strip_prefix("chainz:").map(|x| x.parse().unwrap()).unwrap_or(0);
            let fields = if tree || vec || chain > 0 || bst > 0 || chainz > 0 || vecs_stride > 0 {
                Vec::new()
            } else {
                t[3].split(',')
                    .map(|f| {
                        let p: Vec<&str> = f.split(':').collect();
                        let cs: Vec<u64> = if p.len() > 2 { p[2].split('/').map(|x| x.parse().unwrap()).collect() } else { Vec::new() };
                        (p[0].parse().unwrap(), p[1].parse().unwrap(), cs)
                    })
                    .collect()
            };
            Out { key, arg: t[2].parse().unwrap(), fields, tree, vec, chain, vecs_off, vecs_stride, bst, chainz, stack: t[4..].contains(&"stack"), heapv: t[4..].contains(&"heapv"), tree4: t[4..].contains(&"tree4"), cst: t[4..].contains(&"cst"), amod: t[4..].iter().find_map(|x| x.strip_prefix("amod:")).map(|x| x.parse().unwrap()).unwrap_or(u64::MAX) }
        })
        .collect()
}

/// Value of the j-th 8 byte chunk written to an out-parameter (identical to pinpatch.py outval): small flag-like numbers, zero, 16 bit
/// words, all ones and full words, picked by a hash of the call result rdx.
pub fn outval(rdx: u64, j: u64, consts: &[u64], heap_base: Option<u64>, only: bool) -> u64 {
    let h = neutral_world::mix(rdx, 0x1000 + j);
    if only && !consts.is_empty() {
        return consts[((h >> 16) % consts.len() as u64) as usize];
    }
    match h & 7 {
        0..=2 => (h >> 8) & 0xf,
        3 => 0,
        4 => (h >> 8) & 0xffff,
        5 => u64::MAX,
        6 if !consts.is_empty() => consts[((h >> 16) % consts.len() as u64) as usize],
        _ => match heap_base {
            Some(hb) if (h >> 3) & 1 == 1 => hb + 64 * ((h >> 8) % HEAP_SLOTS),
            _ => neutral_world::mix(h, 7),
        },
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
    RETC.with(|c| *c.borrow_mut() = parse_retc(&text));
}

/// Lines `stub 0x7b1 string_ctor` or `fn 0x72d00 ptrret`: (is a function of the dump, offset, kind). `out` lines are not models.
pub fn parse_models(text: &str) -> Vec<(bool, u64, Kind)> {
    text.lines()
        .filter(|l| !l.trim().is_empty() && !l.starts_with("out ") && !l.starts_with("ret "))
        .map(|l| {
            let t: Vec<&str> = l.split_whitespace().collect();
            assert!(t[0] == "stub" || t[0] == "fn", "bad model line {}", l);
            let rel = u64::from_str_radix(t[1].trim_start_matches("0x"), 16).unwrap();
            let kind = match t[2] {
                "string_ctor" => Kind::StringCtor,
                "string_long" => Kind::StringLong,
                "alloc" | "ptrret" => Kind::Alloc,
                "memcpy" => Kind::Memcpy,
                "memset" => Kind::Memset,
                "getline" => Kind::Getline,
                "fid" => Kind::Fid,
                "retarg0" => Kind::RetArg0,
                "find" => Kind::Find,
                "findnpos" => Kind::FindNpos,
                "facet" => Kind::Facet,
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
        let t = "stub 0x7b1 string_ctor\nout ind 2 0:4\nout stub:0x1d5 1 48:16,0:8\nout fn:0x2fdd8 2 0:8,8:8\nout fn:0x2f282 3 0:1:24/25/23\n";
        assert_eq!(parse_models(t).len(), 1);
        let o = parse_outs(t);
        assert_eq!(o.len(), 4);
        assert_eq!(o[3].fields, vec![(0, 1, vec![24, 25, 23])]);
        assert_eq!(o[0], Out { key: Key::Ind, arg: 2, fields: vec![(0, 4, vec![])], tree: false, vec: false, chain: 0, vecs_off: 0, vecs_stride: 0, bst: 0, chainz: 0, stack: false, heapv: false, tree4: false, cst: false, amod: u64::MAX });
        assert_eq!(o[1], Out { key: Key::Stub(0x1d5), arg: 1, fields: vec![(48, 16, vec![]), (0, 8, vec![])], tree: false, vec: false, chain: 0, vecs_off: 0, vecs_stride: 0, bst: 0, chainz: 0, stack: false, heapv: false, tree4: false, cst: false, amod: u64::MAX });
        let tt = parse_outs("out fn:0x63dd3 0 tree
");
        assert_eq!(tt, vec![Out { key: Key::Fn(0x63dd3), arg: 0, fields: vec![], tree: true, vec: false, chain: 0, vecs_off: 0, vecs_stride: 0, bst: 0, chainz: 0, stack: false, heapv: false, tree4: false, cst: false, amod: u64::MAX }]);
        assert_eq!(o[2].key, Key::Fn(0x2fdd8));
        let st = parse_outs("out stub:0x121 0 0:16,16:8 stack heapv
out ind 2 0:4
");
        assert!(st[0].stack && st[0].heapv && !st[1].stack && !st[1].heapv);
        assert_eq!(st[0].fields, vec![(0, 16, vec![]), (16, 8, vec![])]);
    }

    #[test]
    fn ret_lines_parse() {
        let t = "stub 0x7b1 string_ctor\nret ind 273122048/273122128\nret fn:0xce76c 2\nout ind 2 0:4\n";
        assert_eq!(parse_models(t).len(), 1);
        assert_eq!(parse_outs(t).len(), 1);
        let r = parse_retc(t);
        assert_eq!(r, vec![RetC { key: Key::Ind, consts: vec![273122048, 273122128] }, RetC { key: Key::Fn(0xce76c), consts: vec![2] }]);
    }

    #[test]
    fn outval_follows_the_hash_classes() {
        let mut seen = [false; 5];
        for r in 0..400u64 {
            let h = neutral_world::mix(r, 0x1000);
            let v = outval(r, 0, &[], None, false);
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
