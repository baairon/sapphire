//! The tables of the powf routine, computed from their definitions at first use (module `libm_sincos::exact`, a 320 bit
//! fixed-point number): 65 pairs (log2(e) / x rounded to a double, then log2 of the interval start quantised to a multiple of
//! 2^-44) for x = 1 + i / 64 (the logarithm of x / 2 from i = 32 on), two zero qwords, then the 32 entry table of 2^(j / 32)
//! at qword index 132. No file and no stored value is read.

use std::sync::LazyLock;

/// 164 qwords: log table (qwords 0..=129), padding (130 and 131), exp2 table (132..=163).
pub static TBL: LazyLock<[u64; 164]> = LazyLock::new(libm_sincos::exact::powf_table);

/// Index of the first exp2 table qword in `TBL` (byte offset 0x420).
pub const T2_BASE: usize = 0x420 / 8;

/// 
pub const A_LO: u64 = 0x3fa7a3341fac5e29;
/// 
pub const A_HI: u64 = 0x3fc47fd3ffac83b4;
/// 
pub const B_LO: u64 = 0xbfb55046a1437890;
/// 
pub const B_HI: u64 = 0xbfd62e42fefa39ef;
/// 
pub const C_LO: u64 = 0x3f83b2ab6fba4e77;
/// 
pub const C_HI: u64 = 0x3fcebfbdff82c58e;
/// 
pub const D_LO: u64 = 0x3fac6b08d704a0bf;
/// 
pub const D_HI: u64 = 0x3fe62e42fefa39ef;

