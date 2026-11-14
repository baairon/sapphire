//! Named constants of the scalar exponential and power routines. Each value is a 32-bit float pattern read once from the
//! original routines' constant cells in the emulated image (MEASURED-EMULATOR-READ); the comments give the role.

// Exponential, special lane.
pub const EXP_X_MAX: u32 = 0x42b1_7217; // above this argument e^x overflows (about 88.72)
pub const EXP_X_MIN: u32 = 0xc2cf_f1b4; // below this argument e^x underflows to zero (about -103.97)
pub const EXP_X_DENORM: u32 = 0xc2ae_ac4f; // below this argument the result is subnormal (about -87.34)
pub const LOG2_E: u32 = 0x3fb8_aa3b; // log2(e)
pub const ROUND_SHIFTER: u32 = 0x4b40_0000; // 1.5 * 2^23, rounds to an integer when added
pub const LN2_HI: u32 = 0x3f31_7200; // ln 2, high part
pub const LN2_LO: u32 = 0x35bf_be8e; // ln 2, low part
pub const EXP_C4: u32 = 0x3c09_50ef; // polynomial coefficient, highest degree
pub const EXP_C3: u32 = 0x3d2b_b1b6;
pub const EXP_C2: u32 = 0x3e2a_a9c6;
pub const EXP_C1: u32 = 0x3eff_fe85;
pub const ONE_PLUS_ULP: u32 = 0x3f80_0001; // 1 + 2^-23, the last two polynomial terms
pub const TWO: u32 = 0x4000_0000;
pub const TWO_POW_M66: u32 = 0x2180_0000;
pub const F32_MAX: u32 = 0x7f7f_ffff;
pub const F32_TINY: u32 = 0x0000_0001; // smallest subnormal
pub const MINUS_ONE: u32 = 0xbf80_0000;
pub const ONE: u32 = 0x3f80_0000;

// Power, main path.
pub const SCALE_2P64: u32 = 0x5f80_0000; // 2^64, lifts a subnormal base
pub const LOG2E_TRUNC: u32 = 0x3fb8_8000; // 738 / 512, log2(e) truncated to nine fraction bits
pub const SPLIT_4097: u32 = 0x4580_0800; // 2^12 + 1, the product-splitting factor
pub const EXP_P3: u32 = 0xbdab_1ea1;
pub const EXP_P2: u32 = 0x3e24_6f69;
pub const EXP_P1: u32 = 0xbeb1_c35d;
pub const EXP_P0: u32 = 0x3a6a_6369;
pub const EXP_SHIFTER: u32 = 0x47c0_0000; // 1.5 * 2^16
pub const EXP_Q2: u32 = 0x3d63_5847;
pub const EXP_Q1: u32 = 0x3e75_fdf0;
pub const TWO_POW_M64: u32 = 0x1f80_0000;
pub const MIN_NORMAL: u32 = 0x0080_0000; // 2^-126
pub const LN2: u32 = 0x3f31_7218;

/// Overflow and underflow limits 2^127 and 2^-126, indexed by the sign of the exponent sum.
pub const LIMITS: [u32; 2] = [0x7f00_0000, 0x0080_0000];
/// +1 and -1, indexed by the result sign.
pub const SIGN_ONE: [u32; 2] = [0x3f80_0000, 0xbf80_0000];
/// +0 and -0, indexed by the result sign.
pub const SIGN_ZERO: [u32; 2] = [0x0000_0000, 0x8000_0000];
