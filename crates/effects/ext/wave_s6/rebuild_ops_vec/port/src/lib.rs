//! ops_vec: native port of the slot 53 plane assemblers of the builtin classes `v2` (vtable, entry),
//! `v3` (vtable, entry) and the sibling `v4` (vtable, entry), listing offsets.
//!
//! Each entry is arithmetic-free. For output channel i = 0..N-1 (N = 2, 3, 4) it calls the host plane service with
//! (mode = i, chan = 0, destination = outs[i], info record) and the last call is a tail jump. The service takes the parameter block
//! `blocks[mode]` of the node (argument i) and fills the destination plane with that block's constant (channel 0) or copies the
//! block's source plane (channel 0). So out[i] = argument i, which is a 1-channel image or constant: "build a vector from scalars".
//! The slot 60 combo block holds ONE entry per class: (1,1)->2, (1,1,1)->3, (1,1,1,1)->4, with no member pass pointer (the first field is 0).
//!
//! Plane service: crate `plane_service` (read-only path dependency). Std only, no unsafe. Host state is explicit (see `assumptions()`).

pub use plane_service::{Action, Block, Host, PlError, Roi, Sel};

use std::sync::atomic::{AtomicU32, Ordering};

pub mod v2;
pub mod v3;
pub mod v4;

static MUT: AtomicU32 = AtomicU32::new(0);

/// Select a mutant (feature `mutants` only; 0 = none). Used by the T2 mutation run, never by the port itself.
pub fn set_mutant(id: u32) {
    MUT.store(id, Ordering::Relaxed);
}

#[inline]
fn m(id: u32) -> bool {
    cfg!(feature = "mutants") && MUT.load(Ordering::Relaxed) == id
}

/// The single row of a class's slot 60 combo block (EV-T COMBOS.tsv rows v2, v3, v4): per-argument channel counts at, the
/// result channel count at and the argument count at. All counts are 1; the entry's member pointer (first field) is 0.
#[derive(Clone, Copy, Debug)]
pub struct Combo {
    pub args: &'static [i32],
    pub out: usize,
}

/// Explicit parameters (no Default): `host` the pool flag of the plane service (PL1-A3), `roi` the info record handed to the
/// entry (flat mode only).
#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub host: Host,
    pub roi: Roi,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VecError {
    /// fewer argument blocks or fewer output planes than the class's channel count
    Shape,
    Plane(PlError),
}

/// INFERRED matching rule of the selector (COMBO_SELECT.md section 2; COMBO-PARITY is the evaluator lane's check): the
/// single row matches when every argument has exactly one channel and the argument count equals the row's length.
pub fn matches_combo(c: &Combo, counts: &[i32]) -> bool {
    counts.len() == c.args.len() && counts.iter().zip(c.args.iter()).all(|(a, b)| a == b)
}

/// The shared body of the three entries: `svc(mode = i, chan = 0, outs[i])` for i in 0..n. Returns the plane service actions in call order.
pub(crate) fn assemble(n: usize, p: &Params, args: &[Block], outs: &mut [&mut [f32]]) -> Result<Vec<Action>, VecError> {
    if args.len() < n || outs.len() < n {
        return Err(VecError::Shape);
    }
    let mut acts = Vec::new();
    // mutant 7 writes one plane too many, mutant 4 one too few, mutant 10 skips the first plane
    let (lo, hi) = (if m(10) { 1 } else { 0 }, if m(4) { n - 1 } else if m(7) && outs.len() > n { n + 1 } else { n });
    for i in lo..hi {
        let mut mode = i as i32;
        let mut chan = 0;
        if m(1) {
            mode = (n - 1 - i) as i32;
        }
        if m(5) {
            mode = 0;
        }
        if m(9) && i == 0 {
            mode = 1;
        }
        if m(2) {
            chan = 1;
        }
        if m(6) {
            chan = i as i32;
        }
        if m(8) && i == n - 1 && i > 0 {
            mode = (i - 1) as i32;
        }
        let mut dst = if m(3) { n - 1 - i } else { i };
        if m(13) && i < 2 {
            dst = 1 - i;
        }
        let a = plane_service::plane_service(&p.host, Sel::Direct { blocks: args }, mode, chan, outs[dst], &p.roi).map_err(VecError::Plane)?;
        acts.push(a);
    }
    Ok(acts)
}

#[derive(Clone, Copy, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub tier: &'static str,
    pub text: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "VA1-A1", tier: "ROUTE", text: "the block getter is the base slot 45 (blocks[mode], a fixed stride from [this]) for v2, v3, v4 (none overrides slot 45); the vtable bytes are uncovered in the data image (lookup_va.tsv U8), the slot target comes from vtables.tsv; the real runs in T2" },
        Assumption { id: "VA1-A2", tier: "CAPTURED-ABSENT", text: "the slot 60 combo entry's member pass pointer (first field) is 0 for all three classes (EV-T dump), so the executor runs slot 53 (and the slot 32 driver finds no class pass); that the executor calls slot 53 with (this, pinfo, destination plane array) is INFERRED from the base-class call shape and the entry's own register use (rdi, rsi, rdx), not measured" },
        Assumption { id: "VA1-A3", tier: "PARAM-UNCAPTURED", text: "pool flag of the plane service (PL1-A3): Host.in_pool, both values run for fills of 512 floats and more" },
        Assumption { id: "VA1-A4", tier: "PARAM-UNCAPTURED", text: "parameter block contents (maxch behind, enable byte, constants, source planes, int/float) are written by the executor's argument binding; explicit inputs, no default (EP-4 measures the writers and the range); in contract every argument block has one channel (the only combo)" },
        Assumption { id: "VA1-A5", tier: "NOT REBUILT", text: "rect modes of the plane service (roi.mode != 0): record only (PL1-A6)" },
        Assumption { id: "VA1-A6", tier: "INFERRED", text: "slot 54 is byte-identical machine code to slot 53 up to the relative call offsets (listing); its GPU role is a name guess of the census; ran equal in T2 as an extra family, not part of the claim" },
        Assumption { id: "VA1-A7", tier: "SEMANTIC-XF", text: "node type PIXEL: decided by the class's own slot table (slot 53 override calling the plane service) and, for v3 in Time_Warp_RGB, by image arguments (element_of of get_frame); v2 in Blur_Motion_Curves has parameter arguments only, so whether the compiler runs it on planes or folds it is FOLDING-UNMEASURED (PLAN-PARITY / EP-3)" },
    ]
}
