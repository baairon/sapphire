//! v3_to_v4_if4_a: native port of the pass functions of the builtin class `v3_to_v4_if4_a` (vtable 0x1b166b0, listing offsets).
//!
//! The class has no slot 53 body. Its pixel code is five member pass functions held in the pass entries of the slot 60 block
//! (stride 0x68, member pointer at +0), called by the pass driver. Every pass only assembles output planes: it calls the host
//! plane service 0x4cc8f0 once per output channel, which fills the destination plane with the block constant or copies the
//! block's source plane (crate `plane_service`). No arithmetic of its own: no FMA, no rcp/rsqrt, no x87, no conversion.
//!
//!   pass A 0x594be0: combos (3|4|1, 1, 4) -> 4 channels: out[c] = arg0 channel min(c, nch0-1) for c = 0..2, out[3] = arg1 channel 0
//!   pass B 0x594c80: combos (3,1,3), (3,1,1): out[c] = arg0 channel c for c = 0..count-1
//!   pass C 0x594d00: combo (1,1,3): out[c] = arg0 channel 0 for c = 0..count-1 (broadcast)
//!   pass D 0x594d80: combo (1,1,2): out[0] = arg0 channel 0, out[1] = arg1 channel 0
//!   pass F 0x595b60: combo (1,1,1): out[c] = arg1 channel min(c, this_nch-1) for c = 0..count-1
//!
//! Std only, no unsafe. All host state is an explicit input (see `assumptions()`).

pub use plane_service::{Action, Block, Host, PlError, Roi, Sel};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pass {
    A,
    B,
    C,
    D,
    F,
}

/// One row of the slot 60 combo block: per-argument channel counts, output count at +0x60 and the pass.
#[derive(Clone, Copy, Debug)]
pub struct Combo {
    pub args: [i32; 3],
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
    Combo { args: [1, 1, 1], out: 1, pass: Pass::F },
    Combo { args: [1, 1, 2], out: 2, pass: Pass::D },
];

/// First table row whose three channel counts equal the arguments' (matching rule not claimed).
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

/// Explicit parameters. `combo` is the selected table row, `host` the pool flag of the plane service, `roi` the info record
/// (flat mode only), `this_nch` the int at this+0x70 that pass F reads (the argument channel count).
#[derive(Clone, Copy, Debug)]
pub struct Params {
    pub combo: usize,
    pub host: Host,
    pub roi: Roi,
    pub this_nch: i32,
}

/// Runs the pass of the selected combo. `args` are the node's parameter blocks as the base getter returns them (blocks[mode]).
/// `outs` are the destination planes. Returns the plane service actions in call order.
pub fn render(p: &Params, args: &[Block], outs: &mut [&mut [f32]]) -> Result<Vec<Action>, VqError> {
    let combo = COMBOS.get(p.combo).ok_or(VqError::BadCombo(p.combo))?;
    render_with_count(p, args, outs, combo.out)
}

/// Same as `render` with the output count (the value the count virtual returns) given explicitly; the table rows use 1..=4.
pub fn render_with_count(p: &Params, args: &[Block], outs: &mut [&mut [f32]], cnt: usize) -> Result<Vec<Action>, VqError> {
    let combo = COMBOS.get(p.combo).ok_or(VqError::BadCombo(p.combo))?;
    if args.len() < 2 || outs.len() < match combo.pass { Pass::A => 4, Pass::D => 2, _ => cnt } {
        return Err(VqError::Shape);
    }
    let mut acts = Vec::new();
    let mut svc = |mode: i32, chan: i32, dst: &mut [f32]| -> Result<(), VqError> {
        let a = plane_service::plane_service(&p.host, Sel::Direct { blocks: args }, mode, chan, dst, &p.roi).map_err(VqError::Plane)?;
        acts.push(a);
        Ok(())
    };
    match combo.pass {
        Pass::A => {
            // r9d = *[block0 + 0x50] - 1; chan = min(c, r9d) signed (cmp, cmovge)
            let mx = args[0].maxch.wrapping_sub(1);
            for c in 0..3usize {
                let ci = c as i32;
                let chan = if ci >= mx { mx } else { ci };
                svc(0, chan, outs[c])?;
            }
            svc(1, 0, outs[3])?;
        }
        Pass::B => {
            for c in 0..cnt {
                svc(0, c as i32, outs[c])?;
            }
        }
        Pass::C => {
            for i in 0..cnt {
                svc(0, 0, outs[i])?;
            }
        }
        Pass::D => {
            svc(0, 0, outs[0])?;
            svc(1, 0, outs[1])?;
        }
        Pass::F => {
            // r13d = [this + 0x70] - 1; edx = (c < r13d) ? c : r13d (cmp r12d,r13d; cmovl); esi = 1
            let mx = p.this_nch.wrapping_sub(1);
            for c in 0..cnt {
                let ci = c as i32;
                let chan = if ci < mx { ci } else { mx };
                svc(1, chan, outs[c])?;
            }
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
        Assumption { id: "A1", tier: "ROUTE", text: "the block getter is the base slot 45 (blocks[mode], stride 0x60 from [this+0x38]); the real getter runs in the emulator run" },
        Assumption { id: "A2", tier: "CALLEE-MODELLED", text: "the output-count virtual (vtable +0xf8) returns [combo+0x60]; the emulator run serves COMBOS[combo].out through a constant hook; the combo index is an explicit parameter" },
        Assumption { id: "A3", tier: "SEMANTIC-XF", text: "the pass entries of the slot 60 block are what the pass driver calls (rdi this, rsi pinfo, rdx destination planes)" },
        Assumption { id: "A4", tier: "PARAM-UNCAPTURED", text: "pool flag of the plane service: Host.in_pool, both values run" },
        Assumption { id: "A5", tier: "PARAM-UNCAPTURED", text: "parameter block contents and the int at this+0x70 (pass F) are written by the argument binding; explicit inputs; this+0x70 meaning (argument channel count) INFERRED, run only with values >= 1" },
        Assumption { id: "A6", tier: "NOT REBUILT", text: "rect modes of the plane service (roi.mode != 0): record only" },
    ]
}
