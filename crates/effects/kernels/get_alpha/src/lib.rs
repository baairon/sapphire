//! get_alpha: native port of the slot 52 body 0x553e50 of the builtin class `get_alpha` (vtable 0x1b11910, listing offsets).
//!
//! 0x553e50(this = rdi, pinfo = rsi, outs = rdx) reads the int at this+0x40 (the argument's channel count) and writes one plane:
//!   count == 2: host plane service 0x4cc8f0(this, mode 0, channel 1, dst = outs[0], pinfo)
//!   count == 4: host plane service 0x4cc8f0(this, mode 0, channel 3, dst = outs[0], pinfo)
//!   otherwise : fill 0x4ccaf0(this, dst = outs[0], pinfo, value = 1.0f from the constant cell 0x15e850c), which is the
//!               plane service's constant fill (descriptor 0x5b9bd0, dispatcher 0x6758e0)
//! No arithmetic of its own: no FMA, no rcp/rsqrt, no x87, no conversion. Std only, no unsafe.

pub use plane_service::{Action, Block, Host, PlError, Roi, Sel};

/// Bits of the constant cell 0x15e850c (1.0f).
pub const ONE_BITS: u32 = 0x3f80_0000;

#[derive(Clone, Copy, Debug)]
pub struct Params {
    /// int at this+0x40
    pub count: i32,
    pub host: Host,
    pub roi: Roi,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GaError {
    /// no output plane, or fewer than 1 argument block
    Shape,
    Plane(PlError),
}

/// Runs the body. `args` are the parameter blocks as the base getter returns them (blocks[mode]); `outs[0]` is the destination.
pub fn render(p: &Params, args: &[Block], outs: &mut [&mut [f32]]) -> Result<Action, GaError> {
    if outs.is_empty() || args.is_empty() {
        return Err(GaError::Shape);
    }
    let chan = if p.count == 2 {
        1
    } else if p.count == 4 {
        3
    } else {
        return fill_one(p, outs[0]);
    };
    plane_service::plane_service(&p.host, Sel::Direct { blocks: args }, 0, chan, outs[0], &p.roi).map_err(GaError::Plane)
}

/// 0x4ccaf0 with value 1.0: the constant fill path of the plane service, entered with an always-enabled constant block.
fn fill_one(p: &Params, dst: &mut [f32]) -> Result<Action, GaError> {
    let vals = [ONE_BITS];
    let planes: [&[f32]; 0] = [];
    let blk = Block { is_int: false, values: &vals, enable: true, maxch: 0, img_valid: true, planes: &planes };
    plane_service::plane_service(&p.host, Sel::Virtual { block: blk }, 0, 0, dst, &p.roi).map_err(GaError::Plane)
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
        Assumption { id: "A2", tier: "SEMANTIC-XF", text: "0x553e50 is the executing body of the class (slot 52 is the only overridden pixel slot, no slot 53/54, no member pass pointer); its caller and the meaning of this+0x40 (argument channel count) are INFERRED" },
        Assumption { id: "A3", tier: "PARAM-UNCAPTURED", text: "pool flag of the plane service: Host.in_pool, both values run" },
        Assumption { id: "A4", tier: "PARAM-UNCAPTURED", text: "parameter block contents are written by the argument binding; explicit inputs" },
        Assumption { id: "A5", tier: "NOT REBUILT", text: "rect modes of the plane service (roi.mode != 0): record only; the CPU route only (the GPU twin is not ported)" },
    ]
}
