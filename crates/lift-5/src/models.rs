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
    /// Global model `all outparam`, not tied to one target (see [`outparam`]).
    OutParam,
    /// Global model `all stackgarbage` (see [`garbage`]).
    StackGarbage,
    /// The callee constructs a std::string at arg0 (length and chars from the call result rdx, see `Mem::apply_model`).
    NewStr,
    /// The callee returns the constant (a pointer to a pinned world object, or a fixed code): sidecar `const:0xVALUE`.
    Const(u64),
    /// The callee returns 1 on every call of the run except the calls whose 0 based index is in the list of the sidecar line
    /// `falseat:I,J,..`, where it returns 0 (a predicate that is false at chosen iterations; the list is [`false_at`]).
    FalseAt,
    /// The callee returns list[mix(world seed, 0x7000 + tag) % len]: sidecar `pick:v,w,..@tag` (one value per world, the same tag gives
    /// the same index for several callees). The payload is the index of the list in [`lists`].
    Pick(usize),
    /// The callee returns list[n % len] on its n-th call of the run (0 based): sidecar `seq:v,w,..`.
    Seq(usize),
}

pub const HEAP_SLOTS: u64 = 1 << 16;
pub const STR_MAX: u64 = 0x400;
pub const CPY_MAX: u64 = 0x2000;

thread_local! {
    static MODELS: RefCell<Vec<(bool, u64, Kind)>> = RefCell::new(Vec::new());
    static OUTPARAM: std::cell::Cell<bool> = std::cell::Cell::new(false);
    static GARBAGE: std::cell::Cell<bool> = std::cell::Cell::new(false);
    static SEED: std::cell::Cell<u64> = std::cell::Cell::new(0);
    static FALSEAT: RefCell<Vec<u64>> = RefCell::new(Vec::new());
    static LISTS: RefCell<Vec<(Vec<u64>, u64)>> = RefCell::new(Vec::new());
}

/// The value lists (and tags) of the `pick` / `seq` models of the current vector set, indexed by the payload of the kind.
pub fn list(i: usize) -> (Vec<u64>, u64) {
    LISTS.with(|c| c.borrow()[i].clone())
}

fn add_list(v: Vec<u64>, tag: u64) -> usize {
    LISTS.with(|c| {
        let mut c = c.borrow_mut();
        c.push((v, tag));
        c.len() - 1
    })
}

/// True when call number `n` (0 based, counted per run) of the `falseat` model returns 0.
pub fn false_at(n: u64) -> bool {
    FALSEAT.with(|c| c.borrow().contains(&n))
}

/// True when the vector set declares the global `stackgarbage` model (sidecar line `all 0x0 stackgarbage`): the stack below the
/// entry frame holds seeded garbage instead of zeros (see [`gw`] and `Mem::new`).
pub fn garbage() -> bool {
    GARBAGE.with(|c| c.get())
}

/// The world seed of the case being run (the test closure sets it before calling the port).
pub fn set_seed(s: u64) {
    SEED.with(|c| c.set(s));
}

pub fn seed() -> u64 {
    SEED.with(|c| c.get())
}

/// Seeded garbage word at the 8 aligned stack address `a`; the same rule as scripts/pinpatch.py gw.
pub fn gw(seed: u64, a: u64) -> u64 {
    let h = mix(seed ^ 0x57ac_c0de, a >> 3);
    let (s, w) = (h & 15, h >> 8);
    match s {
        0..=4 => 0,
        5..=7 => w & 0xff,
        8 | 9 => 1,
        10 | 11 => 0xffff_ffff,
        12 | 13 => w & 0xffff,
        14 => w & 0xffff_ffff,
        _ => mix(h, 99),
    }
}

/// True when the vector set declares the global `outparam` model (sidecar line `all 0x0 outparam`): every integer argument of an
/// opaque call that is a stack address receives a seeded value (fleet-5; see Mem::apply_outparam).
pub fn outparam() -> bool {
    OUTPARAM.with(|c| c.get())
}

fn sm64(x: u64) -> u64 {
    let x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut z = x;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// The world's `mix` (tools/world.py).
pub fn mix(a: u64, b: u64) -> u64 {
    sm64(a ^ sm64(b))
}

/// Value written through the k-th pointer argument by the `outparam` model; the same rule as scripts/pinpatch.py outval.
pub fn outval(rdx: u64, k: u64) -> u32 {
    let v = mix(rdx, 0x2000 + k);
    let (s, w) = (v & 7, v >> 8);
    (match s {
        0 | 1 => 0,
        2 | 3 => 1,
        4 => w & 0xff,
        5 => w & 0xffff,
        6 => 0xffff_ffff,
        _ => w & 0xffff_ffff,
    }) as u32
}

/// Lines `stub 0x7b1 string_ctor` or `fn 0x72d00 ptrret`: (is a function of the dump, offset, kind).
pub fn parse_models(text: &str) -> Vec<(bool, u64, Kind)> {
    LISTS.with(|c| c.borrow_mut().clear());
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .filter(|l| !matches!(l.split_whitespace().next(), Some("indret") | Some("indret_at") | Some("outval") | Some("outreg") | Some("outwords") | Some("stackpin") | Some("modelwords")))
        .map(|l| {
            let t: Vec<&str> = l.split_whitespace().collect();
            assert!(t[0] == "stub" || t[0] == "fn" || t[0] == "all", "bad model line {}", l);
            let rel = u64::from_str_radix(t[1].trim_start_matches("0x"), 16).unwrap();
            let kind = match t[2] {
                "outparam" => Kind::OutParam,
                "stackgarbage" => Kind::StackGarbage,
                "newstr" => Kind::NewStr,
                "string_ctor" => Kind::StringCtor,
                "string_long" => Kind::StringLong,
                "alloc" | "ptrret" => Kind::Alloc,
                "memcpy" => Kind::Memcpy,
                "retarg0" => Kind::RetArg0,
                k if k.starts_with("pick:") => {
                    let (body, tag) = k[5..].split_once('@').unwrap_or((&k[5..], "0"));
                    let v: Vec<u64> = body.split(',').map(|x| u64::from_str_radix(x.trim_start_matches("0x"), 16).unwrap()).collect();
                    Kind::Pick(add_list(v, tag.parse::<u64>().unwrap()))
                }
                k if k.starts_with("seq:") => {
                    let v: Vec<u64> = k[4..].split(',').map(|x| u64::from_str_radix(x.trim_start_matches("0x"), 16).unwrap()).collect();
                    Kind::Seq(add_list(v, 0))
                }
                k if k.starts_with("falseat:") => {
                    let v: Vec<u64> = k[8..].split(',').map(|x| x.parse::<u64>().unwrap()).collect();
                    FALSEAT.with(|c| *c.borrow_mut() = v);
                    Kind::FalseAt
                }
                k if k.starts_with("const:") => Kind::Const(u64::from_str_radix(k[6..].trim_start_matches("0x"), 16).unwrap()),
                k => panic!("unknown model {}", k),
            };
            (t[0] == "fn", rel, kind)
        })
        .collect()
}

/// A missing file means no models.
pub fn load_models(path: &Path) -> Vec<(bool, u64, Kind)> {
    match std::fs::read_to_string(path) {
        Ok(t) => {
            set_cm(parse_cm(&t));
            parse_models(&t)
        }
        Err(_) => {
            set_cm(Cm::default());
            Vec::new()
        }
    }
}

/// Call models beyond the per target ones (fleet-5; scripts/pinpatch.py install_models is the original side):
/// `indret v..` every call to a world address returns list[mix(rdx, 0x4000) % len]; `outval SITE v..` after the call at SITE the stack
/// address arguments receive list[mix(rdx, 0x3000 + k) % len]; `stackpin OFF KIND X` initial stack words at RSP0 + OFF.
#[derive(Debug, Clone, Default)]
pub struct Cm {
    pub indret: Vec<u64>,
    pub indret_at: Vec<(u64, Vec<u64>)>,
    pub outvals: Vec<(u64, Vec<u64>)>,
    /// (site, register number): the outvals / outparam writes at that site go to that argument register only
    pub outreg: Vec<(u64, u64)>,
    /// (site, register number, candidate word lists): see scripts/pinpatch.py install_models, key outwords
    pub outwords: Vec<(u64, u64, Vec<Vec<u64>>)>,
    pub stackpins: Vec<(i64, String, i64)>,
    /// sidecar line `modelwords`: the outwords of a call site are also applied after a modelled callee at that site
    pub modelwords: bool,
}

thread_local! {
    static CM: RefCell<Cm> = RefCell::new(Cm::default());
    static ZONE: std::cell::Cell<(u64, u64)> = std::cell::Cell::new((0, 0));
}

fn hexv(t: &str) -> u64 {
    u64::from_str_radix(t.trim_start_matches("0x"), 16).unwrap()
}

pub fn parse_cm(text: &str) -> Cm {
    let mut cm = Cm::default();
    for l in text.lines() {
        let t: Vec<&str> = l.split_whitespace().collect();
        match t.first().copied() {
            Some("indret") => cm.indret = t[1..].iter().map(|x| hexv(x)).collect(),
            Some("indret_at") => cm.indret_at.push((hexv(t[1]), t[2..].iter().map(|x| hexv(x)).collect())),
            Some("outval") => cm.outvals.push((hexv(t[1]), t[2..].iter().map(|x| hexv(x)).collect())),
            Some("outreg") => cm.outreg.push((hexv(t[1]), t[2].parse::<u64>().unwrap())),
            Some("outwords") => {
                let rest = t[3..].join(" ");
                let lists: Vec<Vec<u64>> = rest.split(';').map(|g| g.split_whitespace().map(|x| hexv(x)).collect()).collect();
                let reg = if let Some(h) = t[2].strip_prefix('a') { (1u64 << 63) | hexv(h) } else { t[2].parse::<u64>().unwrap() };
                cm.outwords.push((hexv(t[1]), reg, lists));
            }
            Some("modelwords") => cm.modelwords = true,
            Some("stackpin") => cm.stackpins.push((t[1].parse::<i64>().unwrap(), t[2].to_string(), t[3].parse::<i64>().unwrap())),
            _ => {}
        }
    }
    cm
}

pub fn set_cm(c: Cm) {
    CM.with(|x| *x.borrow_mut() = c);
}

/// The vector header's anchor and stub zone (set by the test before the check): world addresses start at the page aligned end of the stubs.
pub fn set_zone(anchor: u64, stub_zone: u64) {
    ZONE.with(|z| z.set((anchor, (anchor + stub_zone + 0xfff) & !0xfff)));
}

/// True when `target` lies in the world zone (above the stub table): a virtual call through a world vtable.
pub fn is_world(target: u64) -> bool {
    ZONE.with(|z| z.get().1 != 0 && target >= z.get().1)
}

pub fn indret_pick_at(site: u64, rdx: u64) -> Option<u64> {
    CM.with(|c| {
        let c = c.borrow();
        c.indret_at.iter().find(|(s, _)| *s == site).map(|(_, v)| v[(mix(rdx, 0x4000) % v.len() as u64) as usize])
    })
}

pub fn indret_pick(rdx: u64) -> Option<u64> {
    CM.with(|c| {
        let c = c.borrow();
        if c.indret.is_empty() {
            None
        } else {
            Some(c.indret[(mix(rdx, 0x4000) % c.indret.len() as u64) as usize])
        }
    })
}

pub fn outvals_for(site: u64) -> Option<Vec<u64>> {
    CM.with(|c| c.borrow().outvals.iter().find(|(s, _)| *s == site).map(|(_, v)| v.clone()))
}

pub fn outreg_for(site: u64) -> Option<u64> {
    CM.with(|c| c.borrow().outreg.iter().find(|(s, _)| *s == site).map(|(_, r)| *r))
}

pub fn outwords_for(site: u64) -> Vec<(u64, Vec<Vec<u64>>)> {
    CM.with(|c| c.borrow().outwords.iter().filter(|(s, _, _)| *s == site).map(|(_, r, w)| (*r, w.clone())).collect())
}

pub fn modelwords() -> bool {
    CM.with(|c| c.borrow().modelwords)
}

pub fn stackpins() -> Vec<(i64, String, i64)> {
    CM.with(|c| c.borrow().stackpins.clone())
}

pub fn set_models(m: Vec<(bool, u64, Kind)>) {
    OUTPARAM.with(|c| c.set(m.iter().any(|x| x.2 == Kind::OutParam)));
    GARBAGE.with(|c| c.set(m.iter().any(|x| x.2 == Kind::StackGarbage)));
    MODELS.with(|c| *c.borrow_mut() = m.into_iter().filter(|x| x.2 != Kind::OutParam && x.2 != Kind::StackGarbage).collect());
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
    }
}
