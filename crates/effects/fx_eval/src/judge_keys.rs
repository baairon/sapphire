//! Judge-interface module (W689 (a), W823 (d) 4): the option name and the record key that the claim lines and the emulator's records
//! carry byte for byte; every other line uses the names.

pub const OPT_CTX_OWNER_MODE: &str = "--ctx-1ae";
/// The `unit` record field of the parameter value (32-bit).
pub const REC_UNIT_KEY: &str = "rec6c";
/// The record keys of the host point record (four signed size fields), byte for byte as the emulator's records carry them.
pub const REC_POINT_KEY_A: &str = "s11c";
pub const REC_POINT_KEY_B: &str = "s120";
pub const REC_POINT_KEY_C: &str = "s124";
pub const REC_POINT_KEY_D: &str = "s128";
/// The plan text key of the node line (PLAN_ORIG_*.txt and the plan text of cpuplan.rs carry it byte for byte).
pub const PLAN_KEY_NODE_FLAG: &str = "b160";
/// The host cleanup command name that the original's recorded plan dumps carry byte for byte (fx_eval/data/orig, orig_hc and fixtures/pdv1 PLAN_ORIG_*.txt); the plan text of cpuplan.rs and the command layout of layout.rs write it by this name (W113 per-image cleanup records, W900).
pub const PLAN_CLEANUP_CMD: &str = "cmd-0x1aa0500";
