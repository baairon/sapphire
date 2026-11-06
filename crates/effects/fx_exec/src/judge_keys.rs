//! Judge-interface module: the recorded reader-head numbers that the emulator's records and the claim lines carry, mapped to opaque
//! names here at the input boundary. No arithmetic or output depends on the numbers; every other module uses the names.

pub const R4_HEAD: u64 = 0x7eb3d0;
pub const R3_HEAD: u64 = 0x7e9cc0;
pub const R1_HEAD: u64 = 0x7e8920;
/// The recorded heads that have no ported body here.
pub const UNPORTED_HEADS: [u64; 6] = [0x7eaaa0, 0x7e93c0, 0x7e87f0, 0x7e9f20, 0x7e8a50, 0x7d149a];
/// The variant-B bodies.
pub const VB4_HEAD: u64 = 0x7ea840;
pub const VB3_HEAD: u64 = 0x7e91c0;
pub const VB1_HEAD: u64 = 0x7d1102;

/// Printable name of a reader head id.
pub fn head_name(h: u64) -> &'static str {
    match h {
        R4_HEAD => "R4",
        R3_HEAD => "R3",
        R1_HEAD => "R1",
        VB4_HEAD => "VB4",
        VB3_HEAD => "VB3",
        VB1_HEAD => "VB1",
        _ => "UNPORTED",
    }
}

/// Judge-interface option names: the claim lines carry these names byte for byte; every use refers to the constant.
pub const OPT_CTX_OWNER_MODE: &str = "--ctx-1ae";
pub const OPT_SRC_ROW_GATE: &str = "--src-row58";
pub const OPT_OPACITY_ARM: &str = "--opacity-arm-7b0";
pub const OPT_PROG_FLAG: &str = "--prog-flag-59e";
/// The refusal texts that quote an option name.
pub const MSG_SRC_ROW_GATE: &str = "--src-row58 takes the recorded [row] (decimal)";
pub const MSG_OPACITY_ARM: &str = "--opacity-arm-7b0 takes the recorded [prog] (decimal)";
