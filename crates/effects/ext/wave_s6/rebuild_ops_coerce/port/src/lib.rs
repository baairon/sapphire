//! ops_coerce (CO1): native port of the member pass functions that the builtin classes `coerce_up`, `coerce_to_alpha`, `coerce_to` and
//! `coerce_up_a1` store in their slot 60 combo blocks (entry first field; listing offsets, VA = offset).
//!
//! The four classes have NO slot 53 body (slot 53 = shared stub : log + assert) and no GPU twin (slot 54 stub). Their pixel code is
//! seven tiny passes (the same pass functions as v3_to_v4_if4's B and C plus five more), each only assembling output planes
//! through the host plane service: copy a source channel (mode 0 = argument 0) or fill a plane with a constant .
//! No arithmetic: no FMA, no rcp/rsqrt, no x87, no conversion. The node is also resolved at compile time when its operand is already the
//! target width (slot 38 identity collapse) or constant (folded): see README, `slot38_up_collapse`.
//!
//!   C_CELL_A : out[c] = arg0 channel c, c < tag
//!   D_CELL_A : out[c] = arg0 channel 0, c < tag (broadcast)
//!   DE_CELL_A : out[0] = ch0, out[1] = ch1, out[2] = 0.0 (2 -> 3)
//!   E_CELL_A : out[c] = ch0 for c < tag-1, out[tag-1] = 1.0 (1 -> 2 / 1 -> 4 with alpha 1; coerce_up_a1)
//!   EF_CELL_A : out[c] = ch c for c < 3, out[3] = 1.0 (3 -> 4)
//!   F_CELL_A : out[c] = ch0 for c < 4, then out[3] = ch1 (2 -> 4: gray + alpha)
//!   P_CELL_A : out[0] = arg0 channel (maxch0 - 1) (alpha extract: last channel)
//! The constant 1.0 is the CAPTURED float (audit_addr claims.tsv). Std only, no unsafe.

pub use plane_service::{Action, Block, Host, PlError, Roi, Sel};
pub mod judge_keys;
use std::sync::atomic::{AtomicU32, Ordering};

static MUT: AtomicU32 = AtomicU32::new(0);
/// Feature `mutants` only: select a mutant (0 = none) for the T2 mutation run.
pub fn set_mutant(id: u32) {
    MUT.store(id, Ordering::Relaxed);
}
#[inline]
fn m(id: u32) -> bool {
    cfg!(feature = "mutants") && MUT.load(Ordering::Relaxed) == id
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pass {
    C_CELL_A,
    D_CELL_A,
    DE_CELL_A,
    E_CELL_A,
    EF_CELL_A,
    F_CELL_A,
    P_CELL_A,
}

impl Pass {
    pub fn entry(self) -> u32 {
        match self {
            Pass::C_CELL_A => 0,
            Pass::D_CELL_A => 0,
            Pass::DE_CELL_A => 0,
            Pass::E_CELL_A => 0,
            Pass::EF_CELL_A => 0,
            Pass::F_CELL_A => 0,
            Pass::P_CELL_A => 0,
        }
    }
    pub fn from_name(s: &str) -> Option<Pass> {
        Some(match s {
            judge_keys::PASS_NAME_A => Pass::C_CELL_A,
            judge_keys::PASS_NAME_B => Pass::D_CELL_A,
            judge_keys::PASS_NAME_C => Pass::DE_CELL_A,
            judge_keys::PASS_NAME_D => Pass::E_CELL_A,
            judge_keys::PASS_NAME_E => Pass::EF_CELL_A,
            judge_keys::PASS_NAME_F => Pass::F_CELL_A,
            judge_keys::PASS_NAME_G => Pass::P_CELL_A,
            _ => return None,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    CoerceUp,
    CoerceToAlpha,
    CoerceTo,
    CoerceUpA1,
}

/// One row of a slot 60 combo block (COMBOS.tsv, EV-T): operand channel counts, result count (tag at), and the pass (q0 at +0, resolved to a listing offset).
#[derive(Clone, Copy, Debug)]
pub struct Combo {
    pub args: [i32; 2],
    pub nargs: usize,
    pub tag: usize,
    pub pass: Pass,
}

const fn c2(a: i32, b: i32, tag: usize, pass: Pass) -> Combo {
    Combo { args: [a, b], nargs: 2, tag, pass }
}
const fn c1(a: i32, tag: usize, pass: Pass) -> Combo {
    Combo { args: [a, 0], nargs: 1, tag, pass }
}
use Pass::*;

/// coerce_up, block (14 entries, in table order)
pub const UP: [Combo; 14] = [
    c2(1, 2, 2, D_CELL_A), c2(1, 3, 3, D_CELL_A), c2(1, 4, 4, D_CELL_A), c2(3, 4, 4, EF_CELL_A), c2(1, 1, 1, C_CELL_A), c2(2, 1, 2, C_CELL_A), c2(2, 2, 2, C_CELL_A),
    c2(3, 1, 3, C_CELL_A), c2(3, 3, 3, C_CELL_A), c2(4, 1, 4, C_CELL_A), c2(4, 3, 4, C_CELL_A), c2(4, 4, 4, C_CELL_A), c2(2, 3, 3, DE_CELL_A), c2(2, 4, 4, F_CELL_A),
];
/// coerce_up_a1, block 
pub const UP_A1: [Combo; 14] = [
    c2(1, 2, 2, E_CELL_A), c2(1, 3, 3, D_CELL_A), c2(1, 4, 4, E_CELL_A), c2(3, 4, 4, EF_CELL_A), c2(1, 1, 1, C_CELL_A), c2(2, 1, 2, C_CELL_A), c2(2, 2, 2, C_CELL_A),
    c2(3, 1, 3, C_CELL_A), c2(3, 3, 3, C_CELL_A), c2(4, 1, 4, C_CELL_A), c2(4, 3, 4, C_CELL_A), c2(4, 4, 4, C_CELL_A), c2(2, 3, 3, DE_CELL_A), c2(2, 4, 4, F_CELL_A),
];
/// coerce_to_alpha, block 
pub const TO_ALPHA: [Combo; 4] = [c1(1, 1, C_CELL_A), c1(2, 1, P_CELL_A), c1(3, 1, C_CELL_A), c1(4, 1, P_CELL_A)];
/// coerce_to, block 
pub const TO: [Combo; 10] = [
    c2(4, 4, 4, C_CELL_A), c2(3, 4, 4, EF_CELL_A), c2(1, 4, 4, D_CELL_A), c2(4, 3, 3, C_CELL_A), c2(3, 3, 3, C_CELL_A), c2(1, 3, 3, D_CELL_A), c2(1, 2, 2, D_CELL_A),
    c2(1, 1, 1, C_CELL_A), c2(2, 1, 1, C_CELL_A), c2(3, 1, 1, C_CELL_A),
];

pub fn table(c: Class) -> &'static [Combo] {
    match c {
        Class::CoerceUp => &UP,
        Class::CoerceToAlpha => &TO_ALPHA,
        Class::CoerceTo => &TO,
        Class::CoerceUpA1 => &UP_A1,
    }
}

/// The selector (COMBO_SELECT.md): first row whose counts equal the operand counts (plain equality, no wildcard).
pub fn select(c: Class, counts: &[i32]) -> Option<&'static Combo> {
    table(c).iter().find(|e| e.nargs == counts.len() && e.args[..e.nargs] == *counts)
}

/// Slot 38 of the coerce_up family (MEASURED listing): returns 0 (the node is replaced by argument 0) when the combo tag
/// (slot 31) equals argument 0's channel count, else -1. Compile-time identity collapse; not a pass.
pub fn slot38_up_collapse(tag: i32, arg0_count: i32) -> i32 {
    if tag == arg0_count {
        0
    } else {
        -1
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CoError {
    Shape,
    Plane(PlError),
}

/// Explicit parameters (no Default, W65): `tag` = what slot 31 returns (the combo's output count), pool flag of the plane service, flat info record.
#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub pass: Pass,
    pub tag: usize,
    pub host: Host,
    pub roi: Roi,
}

/// The 1.0 constant of (CAPTURED).
pub const ONE: f32 = 1.0;

fn copy(p: &Params, args: &[Block], acts: &mut Vec<Action>, chan: i32, dst: &mut [f32]) -> Result<(), CoError> {
    let a = plane_service::plane_service(&p.host, Sel::Direct { blocks: args }, 0, chan, dst, &p.roi).map_err(CoError::Plane)?;
    acts.push(a);
    Ok(())
}

// (this, dst, info, xmm0): constant fill; modelled as the plane service's block-constant fill (descriptor + dispatcher).
fn fill(p: &Params, acts: &mut Vec<Action>, v: f32, dst: &mut [f32]) -> Result<(), CoError> {
    let vals = [v.to_bits(); 4];
    let planes: [&[f32]; 0] = [];
    let blk = [Block { is_int: false, values: &vals, enable: true, maxch: 1, img_valid: true, planes: &planes }];
    let a = plane_service::plane_service(&p.host, Sel::Direct { blocks: &blk }, 0, 0, dst, &p.roi).map_err(CoError::Plane)?;
    acts.push(a);
    Ok(())
}

/// Runs one pass. `args[0]` is the node's argument-0 parameter block as the base getter returns it; `outs` are the destination planes.
pub fn render(p: &Params, args: &[Block], outs: &mut [&mut [f32]]) -> Result<Vec<Action>, CoError> {
    if args.is_empty() {
        return Err(CoError::Shape);
    }
    let tag = p.tag;
    let need = match p.pass {
        C_CELL_A | D_CELL_A => tag,
        DE_CELL_A => 3,
        E_CELL_A => tag.max(1),
        EF_CELL_A => 4,
        F_CELL_A => 4,
        P_CELL_A => 1,
    };
    if outs.len() < need {
        return Err(CoError::Shape);
    }
    let mut acts = Vec::new();
    match p.pass {
        C_CELL_A => {
            let n = if m(1) { tag.saturating_sub(1) } else { tag };
            for c in 0..n {
                let ch = if m(2) { (n - 1 - c) as i32 } else { c as i32 };
                copy(p, args, &mut acts, ch, outs[c])?;
            }
        }
        D_CELL_A => {
            for c in 0..tag {
                copy(p, args, &mut acts, if m(3) { c as i32 } else { 0 }, outs[c])?;
            }
        }
        DE_CELL_A => {
            let (a, b) = if m(5) { (1, 0) } else { (0, 1) };
            copy(p, args, &mut acts, a, outs[0])?;
            copy(p, args, &mut acts, b, outs[1])?;
            fill(p, &mut acts, if m(4) { ONE } else { 0.0 }, outs[2])?;
        }
        E_CELL_A => {
            // cmp eax,1; jle skips the copy loop (tag <= 1); the copies cover i < tag-1; the fill goes to dst[tag-1]
            let last = if m(7) { tag } else { tag.saturating_sub(1) };
            if tag > 1 {
                for c in 0..last {
                    copy(p, args, &mut acts, 0, outs[c])?;
                }
            }
            fill(p, &mut acts, if m(6) { 0.0 } else { ONE }, outs[tag - 1])?;
        }
        EF_CELL_A => {
            let k = if m(9) { 4 } else { 3 };
            for c in 0..k {
                copy(p, args, &mut acts, c as i32, outs[c])?;
            }
            fill(p, &mut acts, if m(8) { 0.0 } else { ONE }, outs[3])?;
        }
        F_CELL_A => {
            for c in 0..4 {
                copy(p, args, &mut acts, 0, outs[c])?;
            }
            if !m(11) {
                copy(p, args, &mut acts, if m(10) { 0 } else { 1 }, outs[3])?;
            }
        }
        P_CELL_A => {
            let ch = if m(12) {
                args[0].maxch
            } else if m(13) {
                0
            } else {
                args[0].maxch.wrapping_sub(1)
            };
            copy(p, args, &mut acts, ch, outs[0])?;
        }
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
        Assumption { id: "CO1-A1", tier: "SEMANTIC-XF", text: "the member pointers at COMBOS entry first field are what slot 32 calls (call shape rdi this, rsi info, rdx dst planes); INFERRED as in VQ1-A3; pass mode value not measured. The plan dump shows coerce_up as run-primitive on a 1 channel image (probe), so the node is PIXEL" },
        Assumption { id: "CO1-A2", tier: "CALLEE-MODELLED", text: "slot 31 returns the combo tag; T2 serves it through a ret_const hook; the combo (pass, tag) is an explicit parameter" },
        Assumption { id: "CO1-A3", tier: "PARAM-UNCAPTURED", text: "pool flag of the plane service (PL1-A3): Host.in_pool, both values run for fills of 512 floats and more" },
        Assumption { id: "CO1-A4", tier: "PARAM-UNCAPTURED", text: "argument-0 block contents (maxch, enable, constants, source planes) are written by the executor's argument binding; explicit inputs" },
        Assumption { id: "CO1-A5", tier: "CAPTURED", text: "constant 1.0, audit_addr claims.tsv; 0.0 by xorps in DE_CELL_A" },
        Assumption { id: "CO1-A6", tier: "NOT REBUILT", text: "rect modes of the plane service; block getter slot 45 only enters P_CELL_A (real code in T2)" },
        Assumption { id: "CO1-A7", tier: "OPEN", text: "the [this] dword read by coerce_to_alpha's slot 38 is not written by the factory or the base constructor; its writer is unmeasured" },
    ]
}
