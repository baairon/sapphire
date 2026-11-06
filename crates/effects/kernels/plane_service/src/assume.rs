//! Assumption and stand-in list of the plane_service port (prefix -).

#[derive(Clone, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub what: &'static str,
    pub tier: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    let a = |id, what, tier| Assumption { id, what, tier };
    vec![
        a("PL1-A1", "selector cell (captured) = with a rebase fixup: CAPTURED. It is the symbol of the 4 slot vtable; `[this]-0x10 == cell` selects blocks[mode] at this (a fixed stride), else the class virtual at [vtable]. Port input: `Sel`both outcomes run in T2", "CAPTURED, explicit input"),
        a("PL1-A2", "virtual getter [vtable](this, mode) returns the parameter block: modelled as the block passed in `Sel::Virtual` (CALLEE-MODELLED; the real getter is the caller class's slot 45)", "CALLEE-MODELLED"),
        a("PL1-A3", "pool flag cell (captured, zero range in the capture, set at run time): explicit `Host.in_pool`no default, both values in T2", "PARAM-UNCAPTURED"),
        a("PL1-A4", "pool launcher and tile partition modelled as ONE tile covering [0, n) (emulator: serial_for + tile_whole). The fill is tile independent except that every tile applies the zero shortcut itself, so the written bytes do not depend on the tiling (argued from, not compared with several tiles)", "CALLEE-MODELLED"),
        a("PL1-A5", "memcpy and memset modelled as libc semantics (hooks in the emulator, slice copy in Rust); overlapping buffers are outside the contract", "CALLEE-MODELLED"),
        a("PL1-A6", "rect mode (roi.mode != 0): rect fill and rect copy are NOT REBUILT; the port returns the call (value, 24 byte descriptor, source plane, assert flag) and leaves dst untouched; T2 compares the call record only (REC)", "NOT REBUILT, record only"),
        a("PL1-A7", "assert (rect copy, image byte == 0) is assumed to return and execution continues as in the listing; null pointer asserts / log of are outside the contract", "ASSUMED"),
        a("PL1-A8", "copies of at least bytes with the pool flag clear take the pooled memcpy: the emulator runs it with the pool-not-ready cell = 0 (captured zero), where the worker copies the whole range; a multi tile partition (pool ready) is not emulated", "CALLEE-MODELLED"),
        a("PL1-A9", "constants of CAPTURED: = 0x0000000100000000, = 0, = 4, lane 1 = 0, = 4 (lookup_pl.tsv)", "CAPTURED"),
        a("PL1-A10", "feature word 0x9d97ff (AVX2) served through the cell; dispatchers run in the emulator, the AVX and SSE twins are not ported (ROUTE AVX2-ONLY)", "ROUTE"),
        a("PL1-A11", "channel clamp, plane and value indexing outside the arrays (negative channel, short planes, short dst) are outside the contract and return an error", "contract"),
    ]
}
