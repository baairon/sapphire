//! Arithmetic core of the scalar power routine for a finite non-zero base and a finite exponent: x^y as the product of
//! three table-driven logarithm stages, an extended-precision product with y, and a two-table exponential.
//!
//! The stages are written in the order the original evaluates them: a split of x into exponent and mantissa, three
//! table-lookup logarithm steps, a compensated sum and a split product with y, then a polynomial and a two-table
//! exponential, with the result scaled by the exponent sum or cut to a signed limit. Every float operation goes
//! through the first-operand NaN rule, and each `//` origin line names the address range a block was derived from.
#![allow(clippy::all)]
use crate::scalar_consts::*;
use crate::tables::ScalarTables;
use fx_common::nanop::NanOps;

/// `s_c`: bits of |x| (scaled by 2^64 when x is subnormal, with `sel_a` = -64 then, else 0); `y`: bits of the exponent; `ysign`: its sign bit (0 or 1);
/// `yexp`: its biased exponent field; `sign_factor`: the sign factor (+1 or -1) of the result. Returns the result bits.
pub(crate) fn pow_core(t: &ScalarTables, s_c: u32, sel_a: u32, y: u32, ysign: u32, yexp: u32, sign_factor: f32) -> u32 {
    // origin: 135c4ce to 135c7b0
    let y_bits = y;
    let y_sign = ysign;
    let exp_acc = yexp;
    let x_abs = f32::from_bits(s_c);
    let exp_a = (s_c >> 16) & 0xffff;
    let exp_a = exp_a & 0x7f80;
    let exp_a = exp_a >> 0x7;
    let word_c = s_c;
    let exp_a = exp_a << 0x17;
    let word_c = word_c & 0x7fffff;
    let exp_a = exp_a | word_c;
    let word_d = (x_abs.to_bits() >> 16) & 0xffff;
    let exp_a = exp_a.wrapping_add(0xc0c80000);
    let word_d = word_d & 0xffff807f;
    let word_d = word_d | 0xffff3f80;
    let reduced_bits: u32 = (x_abs.to_bits() & !(0xffffu32 << 16)) | ((word_d & 0xffff) << 16);
    let exp_a = ((exp_a as i32) >> 0x17) as u32;
    let exp_a = exp_a.wrapping_add(sel_a);
    let sel_a = reduced_bits;
    let sel_a = sel_a & 0x780000;
    let sel_a = sel_a.wrapping_add(0x80000);
    let sel_a = sel_a >> 0x14;
    let lo2 = f32::from_bits(reduced_bits);
    let exp_f = (exp_a as i32) as f32;
    let look3 = f32::from_bits(t.rcp1[(sel_a) as usize]);
    let sel_a = sel_a.wrapping_add(sel_a);
    let prod4 = lo2.fmul(look3);
    let sel_b = prod4.to_bits();
    let sel_b = sel_b & 0x1e0000;
    let sel_b = sel_b.wrapping_add(0x20000);
    let sel_b = sel_b >> 0x12;
    let look5 = f32::from_bits(t.rcp2[(sel_b) as usize]);
    let sel_b = sel_b.wrapping_add(sel_b);
    let prod6 = prod4.fmul(look5);
    let prod7 = look3.fmul(look5);
    let sel_c = prod6.to_bits();
    let look8 = f32::from_bits(t.pair1[(sel_a) as usize]);
    let sel_c = sel_c & 0x7c000;
    let sum9 = look8.fadd(exp_f);
    let sel_c = sel_c.wrapping_add(0x4000);
    let sel_c = sel_c >> 0xf;
    let look10 = f32::from_bits(t.pair1[(sel_a + 1) as usize]);
    let look11 = f32::from_bits(t.rcp3[(sel_c) as usize]);
    let sel_c = sel_c.wrapping_add(sel_c);
    let look12 = f32::from_bits(t.pair2[(sel_b) as usize]);
    let look13 = f32::from_bits(t.pair2[(sel_b + 1) as usize]);
    let prod14 = prod6.fmul(look11);
    let prod15 = prod7.fmul(look11);
    let look16 = f32::from_bits(t.pair3[(sel_c) as usize]);
    let look17 = f32::from_bits(t.pair3[(sel_c + 1) as usize]);
    let diff18 = exp_f.fsub(sum9);
    let sum19 = look8.fadd(diff18);
    let sum20 = sum9.fadd(look12);
    let diff21 = sum9.fsub(sum20);
    let sum22 = diff21.fadd(look12);
    let sum23 = sum20.fadd(look16);
    let sum24 = sum19.fadd(sum22);
    let cst25 = f32::from_bits(SPLIT_4097);
    let diff26 = sum20.fsub(sum23);
    let diff27 = prod14.fsub(f32::from_bits(LOG2E_TRUNC));
    let sum28 = diff26.fadd(look16);
    let sum29 = sum24.fadd(sum28);
    let sum30 = sum29.fadd(look10);
    let prod31 = lo2.fmul(cst25);
    let sum32 = sum30.fadd(look13);
    let sum33 = sum32.fadd(look17);
    let diff34 = prod31.fsub(f32::from_bits(reduced_bits));
    let diff35 = prod31.fsub(diff34);
    let diff36 = lo2.fsub(diff35);
    let prod37 = cst25.fmul(prod15);
    let diff38 = prod37.fsub(prod15);
    let diff39 = prod37.fsub(diff38);
    let diff40 = prod15.fsub(diff39);
    let prod41 = diff35.fmul(diff39);
    let prod42 = diff39.fmul(diff36);
    let diff43 = prod41.fsub(prod14);
    let sum44 = diff43.fadd(prod42);
    let sum45 = sum23.fadd(diff27);
    let prod46 = diff35.fmul(diff40);
    let diff47 = sum23.fsub(sum45);
    let prod48 = diff36.fmul(diff40);
    let sum49 = sum44.fadd(prod46);
    let sum50 = diff47.fadd(diff27);
    let sum51 = prod48.fadd(sum49);
    let exp_b = (sum45.to_bits() >> 16) & 0xffff;
    let exp_b = exp_b & 0x7f80;
    let exp_b = exp_b >> 0x7;
    let exp_acc = exp_acc.wrapping_add(exp_b);
    if (exp_acc as i32) >= 0x109 {
        let bsign = (sum45.to_bits() >> 31) & 1;
        let cst52 = f32::from_bits(LIMITS[(y_sign ^ bsign) as usize]);
        let prod53 = cst52.fmul(cst52);
        let prod54 = prod53.fmul(sign_factor);
        return prod54.to_bits();
    }
    if (exp_acc as i32) <= 0xc0 {
        // origin: 135c7ce to 135c7f4
        let const_bits_a: u32 = 0x3f800000;
        let lo56 = f32::from_bits(const_bits_a);
        let sum57 = lo56.fadd(f32::from_bits(MIN_NORMAL));
        let prod58 = sign_factor.fmul(sum57);
        return prod58.to_bits();
    }
    // origin: 135c900 to 135cb40
    let cst59 = f32::from_bits(EXP_P3);
    let sum60 = diff27.fadd(sum51);
    let sum61 = sum51.fadd(sum50);
    let prod62 = cst59.fmul(sum60);
    let sum63 = sum33.fadd(sum61);
    let sum64 = prod62.fadd(f32::from_bits(EXP_P2));
    let prod65 = sum64.fmul(sum60);
    let sum66 = prod65.fadd(f32::from_bits(EXP_P1));
    let prod67 = sum66.fmul(sum60);
    let sum68 = prod67.fadd(f32::from_bits(EXP_P0));
    let prod69 = sum68.fmul(sum60);
    let sum70 = sum45.fadd(prod69);
    let diff71 = sum45.fsub(sum70);
    let sum72 = diff71.fadd(prod69);
    let sum73 = sum70.fadd(sum63);
    let diff74 = sum70.fsub(sum73);
    let sum75 = diff74.fadd(sum63);
    let prod76 = cst25.fmul(sum73);
    let sum77 = sum75.fadd(sum72);
    let y_f = f32::from_bits(y_bits);
    let diff78 = prod76.fsub(sum73);
    let prod79 = cst25.fmul(y_f);
    let prod80 = sum77.fmul(f32::from_bits(y_bits));
    let diff81 = prod76.fsub(diff78);
    let diff82 = sum73.fsub(diff81);
    let cst83 = f32::from_bits(EXP_SHIFTER);
    let diff84 = prod79.fsub(f32::from_bits(y_bits));
    let diff85 = prod79.fsub(diff84);
    let diff86 = y_f.fsub(diff85);
    let prod87 = diff85.fmul(diff81);
    let prod88 = diff85.fmul(diff82);
    let sum89 = cst83.fadd(prod87);
    let prod90 = diff86.fmul(diff82);
    let prod91 = diff81.fmul(diff86);
    let sum92 = prod90.fadd(prod88);
    let sum93 = sum92.fadd(prod91);
    let sum94 = prod80.fadd(sum93);
    let diff95 = sum89.fsub(cst83);
    let cst96 = f32::from_bits(EXP_Q2);
    let diff97 = prod87.fsub(diff95);
    let word_d = sum89.to_bits();
    let word_c = word_d;
    let word_d = word_d & 0x7f;
    let sum98 = sum94.fadd(diff97);
    let prod99 = cst96.fmul(sum98);
    let word_d = word_d.wrapping_add(word_d);
    let sum100 = prod99.fadd(f32::from_bits(EXP_Q1));
    let prod101 = sum100.fmul(sum98);
    let word_c = word_c << 0xa;
    let sum102 = prod101.fadd(f32::from_bits(LN2));
    let prod103 = sum102.fmul(sum98);
    let look104 = f32::from_bits(t.exp2[(word_d) as usize]);
    let prod105 = prod103.fmul(look104);
    let word_c = ((word_c as i32) >> 0x11) as u32;
    let sum106 = prod105.fadd(f32::from_bits(t.exp2[(word_d + 1) as usize]));
    let sum107 = look104.fadd(sum106);
    let s_c = sum107.to_bits();
    let word_d = (s_c >> 16) & 0xffff;
    let word_a = word_d;
    let word_a = word_a & 0x7f80;
    let word_a = word_a >> 0x7;
    let word_a = word_a.wrapping_add(0xffffff81);
    let word_a = word_a.wrapping_add(word_c);
    let exp_sum = word_a as i32;
    if exp_sum >= 0x80 {
        // origin: 135cd4f to 135cd7d
        let const_bits_b: u32 = 0x7f000000;
        let lo109 = f32::from_bits(const_bits_b);
        let lo110 = f32::from_bits(const_bits_b);
        let prod111 = lo109.fmul(lo110);
        let prod112 = sign_factor.fmul(prod111);
        return prod112.to_bits();
    }
    if exp_sum >= -126 {
        // origin: 135cb52 to 135cb78
        let word_a = word_a.wrapping_add(0x7f);
        let word_d = word_d & 0xffff807f;
        let word_a = word_a & 0xff;
        let word_a = word_a << 0x7;
        let word_d = word_d | word_a;
        let s_c = (s_c & !(0xffffu32 << 16)) | ((word_d & 0xffff) << 16);
        let cst113 = f32::from_bits(s_c);
        let prod114 = cst113.fmul(sign_factor);
        return prod114.to_bits();
    }
    if exp_sum >= -136 {
        // origin: 135cb8e to 135cc89
        let word_c = word_c.wrapping_add(0x40);
        let word_a = 0x3f800000;
        let word_a_copy_a: u32 = word_a;
        let word_c = word_c.wrapping_add(0x7f);
        let word_a = word_a >> 0x10;
        let diff116 = look104.fsub(sum107);
        let word_d = word_a;
        let sum117 = diff116.fadd(sum106);
        let word_d = word_d & 0xffff807f;
        let prod118 = cst25.fmul(sum107);
        let word_c = word_c & 0xff;
        let diff119 = prod118.fsub(sum107);
        let word_c = word_c << 0x7;
        let diff120 = prod118.fsub(diff119);
        let word_d = word_d | word_c;
        let spliced_bits_a: u32 = (word_a_copy_a & !(0xffffu32 << 16)) | ((word_d & 0xffff) << 16);
        let diff122 = sum107.fsub(diff120);
        let lo123 = f32::from_bits(spliced_bits_a);
        let sum124 = sum117.fadd(diff122);
        let prod125 = diff120.fmul(lo123);
        let prod126 = sum124.fmul(lo123);
        let min_normal_bits_a: u32 = MIN_NORMAL;
        let sum128 = prod125.fadd(prod126);
        let lo129 = f32::from_bits(min_normal_bits_a);
        let lo130 = f32::from_bits(min_normal_bits_a);
        let prod131 = lo129.fmul(lo130);
        let prod132 = sum128.fmul(f32::from_bits(TWO_POW_M64));
        let sum133 = prod132.fadd(prod131);
        let prod134 = sign_factor.fmul(sum133);
        return prod134.to_bits();
    }
    if exp_sum >= -159 {
        // origin: 135cc9b to 135cd0b
        let word_a = 0x3f800000;
        let word_c = word_c.wrapping_add(0xbf);
        let word_a_copy_b: u32 = word_a;
        let word_a = word_a >> 0x10;
        let word_d = word_a;
        let word_c = word_c & 0xff;
        let word_d = word_d & 0xffff807f;
        let word_c = word_c << 0x7;
        let word_d = word_d | word_c;
        let spliced_bits_b: u32 = (word_a_copy_b & !(0xffffu32 << 16)) | ((word_d & 0xffff) << 16);
        let cst137 = f32::from_bits(TWO_POW_M64);
        let lo138 = f32::from_bits(spliced_bits_b);
        let prod139 = sign_factor.fmul(cst137);
        let prod140 = sum107.fmul(lo138);
        let min_normal_bits_b: u32 = MIN_NORMAL;
        let lo142 = f32::from_bits(min_normal_bits_b);
        let prod143 = prod139.fmul(prod140);
        let lo144 = f32::from_bits(min_normal_bits_b);
        let prod145 = lo144.fmul(lo142);
        let sum146 = prod143.fadd(prod145);
        return sum146.to_bits();
    }
    // origin: 135cd16 to 135cd44
    let min_normal_bits_c: u32 = MIN_NORMAL;
    let lo148 = f32::from_bits(min_normal_bits_c);
    let lo149 = f32::from_bits(min_normal_bits_c);
    let prod150 = lo149.fmul(lo148);
    let prod151 = sign_factor.fmul(prod150);
    return prod151.to_bits();
}
