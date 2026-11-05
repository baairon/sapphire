//! v3_to_v4_if4: native port of the class pass functions of the builtin class `v3_to_v4_if4` (vtable, listing offsets).
//!
//! The class has no slot 53 body (slot 53 is the base stub that logs and asserts). Its pixel code is four member pass
//! functions kept in the pass entries of the slot 60 block (a fixed stride, member pointer in the first field), called by the pass driver slot 32
//! . Every pass only assembles output planes: it calls the host plane service once per output channel, which
//! fills the destination plane with the block constant or copies the block's source plane (crate `plane_service`, read-only path
//! dependency). No arithmetic of its own: no FMA, no rcp/rsqrt, no x87, no conversion in the passes.
//!
//!   pass A : combos (3|4|1, 1, 4) -> 4 channels: out[c] = arg0 channel min(c, nch0-1) for c = 0..2, out[3] = arg1 channel 0
//!   pass B : combos (3,1,3), (3,1,1), (1,1,1): out[c] = arg0 channel c for c = 0..count-1
//!   pass C : combo (1,1,3): out[c] = arg0 channel 0 for c = 0..count-1 (broadcast)
//!   pass D : combo (1,1,2): out[0] = arg0 channel 0, out[1] = arg1 channel 0
//!
//! Std only, no unsafe. All host state is an explicit input (see `assumptions()`).

pub use plane_service::{Action, Block, Host, PlError, Roi, Sel};

use std::sync::atomic::{AtomicU32, Ordering};

static MUT: AtomicU32 = AtomicU32::new(0);

/// Select a mutant (feature `mutants` only; 0 = none). Used by the T2 mutation run, never by the port itself.
pub fn set_mutant(id: u32) {
    MUT.store(id, Ordering::Relaxed);
}

#[inline]
fn m(id: u32) -> bool {
    cfg!(feature = "mutants") && MUT.load(Ordering::Relaxed) == id
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pass {
    /// 
    A,
    /// 
    B,
    /// 
    C,
    /// 
    D,
}

/// One row of the slot 60 combo block (EV-T COMBOS.tsv rows 0..7: ints at, result count at, argument count at).
#[derive(Clone, Copy, Debug)]
pub struct Combo {
    /// per-argument channel counts (to_vect_type's image and the two value arguments), as the combo table stores them
    pub args: [i32; 3],
    /// output channel count = `[combo]`which the pass reads back through slot 31 
    pub out: usize,
    pub pass: Pass,
}

pub const COMBOS: [Combo; 8] = [
    Combo { args: [3, 1, 4], out: 4, pass: Pass::A },
    Combo { args: [4, 1, 4], out: 4, pass: Pass::A },
    Combo { args: [1, 1, 4], out: 4, pass: Pass::A },
    Combo { args: [3, 1, 3], out: 3, pass: Pass::B },
    Combo { args: [1, 1, 3], out: 3, pass: Pass::C },
    Combo { args: [3, 1, 1], out: 1, pass: Pass::B },
    Combo { args: [1, 1, 1], out: 1, pass: Pass::B },
    Combo { args: [1, 1, 2], out: 2, pass: Pass::D },
];

/// First table row whose three channel counts equal the arguments' (INFERRED matching rule, not claimed: COMBO-PARITY is the evaluator lane's check).
pub fn select_combo(elnums: [i32; 3]) -> Option<usize> {
    COMBOS.iter().position(|c| c.args == elnums)
}

#[derive(Clone, Debug, PartialEq)]
pub enum VqError {
    BadCombo(usize),
    /// fewer than 2 argument blocks, or fewer output planes than the combo's channel count
    Shape,
    Plane(PlError),
}

/// Explicit parameters (no Default, W65). `combo` is the table row the executor selected (slot 31 result is `COMBOS[combo].out`),
/// `host` the pool flag of the plane service (PL1-A3), `roi` the info record handed to the passes (flat mode only).
#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub combo: usize,
    pub host: Host,
    pub roi: Roi,
}

/// Runs the pass of the selected combo. `args` are the node's parameter blocks as the base getter returns them
/// (blocks[mode], mode 0 = value argument, mode 1 = alpha argument; the third block, to_vect_type, is never read by a pass).
/// `outs` are the destination planes. Returns the plane service actions in call order.
pub fn render(p: &Params, args: &[Block], outs: &mut [&mut [f32]]) -> Result<Vec<Action>, VqError> {
    let combo = COMBOS.get(p.combo).ok_or(VqError::BadCombo(p.combo))?;
    if args.len() < 2 || outs.len() < combo.out {
        return Err(VqError::Shape);
    }
    let mut acts = Vec::new();
    let mut svc = |mode: i32, chan: i32, dst: &mut [f32]| -> Result<(), VqError> {
        let a = plane_service::plane_service(&p.host, Sel::Direct { blocks: args }, mode, chan, dst, &p.roi).map_err(VqError::Plane)?;
        acts.push(a);
        Ok(())
    };
    let cnt = if m(6) { combo.out.saturating_sub(1) } else { combo.out };
    match combo_pass(p.combo).ok_or(VqError::BadCombo(p.combo))? {
        Pass::A => {
            // r9d = *[block0] - 1; chan = min(c, r9d) signed (cmp, cmovge)
            let mx = if m(2) { args[0].maxch } else { args[0].maxch.wrapping_sub(1) };
            let ncol = if m(5) { 2 } else { 3 };
            for c in 0..ncol {
                let ci = c as i32;
                let chan = if m(1) {
                    ci
                } else if m(13) {
                    ci.min(mx.max(0))
                } else if ci >= mx {
                    mx
                } else {
                    ci
                };
                svc(if m(15) { 1 } else { 0 }, chan, outs[c])?;
            }
            // esi = 1, edx = 0, rcx = [r14]
            let (am, ac, ad) = (if m(4) { 0 } else { 1 }, if m(3) { 1 } else { 0 }, if m(14) { 2 } else { 3 });
            svc(am, ac, outs[ad])?;
        }
        Pass::B => {
            // do { svc(esi = 0, edx = r12d = c, rcx = outs[c]) } while (++c < slot31)
            for c in 0..cnt {
                let chan = if m(12) { (cnt - 1 - c) as i32 } else { c as i32 };
                svc(0, chan, outs[c])?;
            }
        }
        Pass::C => {
            // svc(esi = 0, edx = 0, rcx = outs[i]) for i < slot31
            for i in 0..cnt {
                svc(0, if m(7) { i as i32 } else { 0 }, outs[i])?;
            }
        }
        Pass::D => {
            // svc(0, 0, outs[0]); svc(esi = 1, edx = 0, rcx = outs[1])
            let (m0, m1) = if m(8) { (1, 0) } else { (0, 1) };
            svc(m0, 0, outs[0])?;
            svc(m1, if m(9) { 1 } else { 0 }, outs[1])?;
        }
    }
    Ok(acts)
}

/// Mutant 10 changes the combo table (pass C row read as pass B); it is applied by `combo_pass` for the T2 driver.
pub fn combo_pass(i: usize) -> Option<Pass> {
    let p = COMBOS.get(i)?.pass;
    Some(if m(10) && p == Pass::C { Pass::B } else { p })
}

#[derive(Clone, Copy, Debug)]
pub struct Assumption {
    pub id: &'static str,
    pub tier: &'static str,
    pub text: &'static str,
}

pub fn assumptions() -> Vec<Assumption> {
    vec![
        Assumption { id: "VQ1-A1", tier: "ROUTE", text: "the block getter is the base slot 45 (blocks[mode], a fixed stride from [this]); the vtable bytes are uncovered in the data image, the slot target comes from vtables.tsv; the real runs in T2" },
        Assumption { id: "VQ1-A2", tier: "CALLEE-MODELLED", text: "slot 31 returns [combo]; the pass re-reads it as the output channel count; T2 serves COMBOS[combo].out through a ret_const hook; the combo index is an explicit parameter, all 8 rows run" },
        Assumption { id: "VQ1-A3", tier: "SEMANTIC-XF", text: "the passes in the pass entries (first field member pointer, second field adjustment 0) are what slot 32 calls; INFERRED from the call shape (rdi this, rsi pinfo, rdx destination planes, ecx flag); pass mode value not measured" },
        Assumption { id: "VQ1-A4", tier: "PARAM-UNCAPTURED", text: "pool flag of the plane service (PL1-A3): Host.in_pool, both values run for fills of 512 floats and more" },
        Assumption { id: "VQ1-A5", tier: "PARAM-UNCAPTURED", text: "parameter block contents (maxch, enable, constants, source planes, int/float) are written by the executor's argument binding; explicit inputs, no default (EP-4 measures the writers and the range)" },
        Assumption { id: "VQ1-A6", tier: "NOT REBUILT", text: "rect modes of the plane service (roi.mode != 0): record only (PL1-A6)" },
    ]
}
