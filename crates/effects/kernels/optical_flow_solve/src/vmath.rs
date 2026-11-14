//! The vector power routine (listing 0x1367900, two arguments) and the vector exponential routine (listing 0x1364400),
//! AVX2/FMA variants, lane by lane in the listing's operation order, with the scalar special-lane routines.
//!
//! Naming (inferred from the constants and structure): the first routine computes x^y per lane, the second e^x per lane.
//! ROUTE AVX2-ONLY. APPROX-MODEL: the reciprocal estimate at 0x136790f is modelled as the exact 1/x (one rounding).
use crate::consts::*;
use crate::scalar::{expf_scalar_bits, powf_scalar_bits};
use crate::tables::atables;
use fx_common::nanop::{self as nn, NanOps};

fn f(b: u32) -> f32 {
    f32::from_bits(b)
}

/// One lane of the power routine, x^y.
pub fn powf_lane(xb: u32, yb: u32) -> u32 {
    let t = atables();
    let mut tmp = [0f32; 14];
    let (x, y) = (f(xb), f(yb));
    let rcp = 1.0f32.fdiv(x); // vrcpps, APPROX-MODEL
    let rcp_biased = rcp.to_bits().wrapping_add(A_080);
    let x_negative = x < f(A_000);
    let x_out_of_range = !(x <= f(A_040));
    let rcp_masked = rcp_biased & A_0C0;
    let exp_field = rcp_masked >> 23;
    let table_index = (rcp_masked >> 20) as usize & 7;
    tmp[11] = (exp_field as i32) as f32;
    tmp[2] = nn::fms231(f(A_100), f(rcp_masked), x);
    tmp[10] = f(t.log_head[table_index]);
    let mask = x_negative | x_out_of_range;
    tmp[7] = tmp[10].fsub(tmp[11]);
    tmp[3] = tmp[2].fmul(tmp[2]);
    tmp[6] = nn::fma213(f(A_240), tmp[2], f(A_280));
    tmp[5] = nn::fma213(f(A_300), tmp[2], f(A_340));
    tmp[10] = nn::fma213(f(A_1C0), tmp[2], f(A_200));
    tmp[10] = nn::fma213(tmp[10], tmp[3], tmp[6]);
    tmp[4] = f(t.log_tail[table_index]);
    tmp[13] = nn::fma231(tmp[7], tmp[2], f(A_2C0));
    tmp[10] = nn::fma213(tmp[10], tmp[2], f(A_3C0));
    tmp[7] = tmp[13].fsub(tmp[7]);
    tmp[10] = nn::fma213(tmp[10], tmp[3], tmp[5]);
    tmp[7] = nn::fms231(tmp[7], tmp[2], f(A_2C0));
    tmp[11] = nn::fma231(tmp[13], tmp[3], f(A_380));
    tmp[5] = tmp[4].fadd(tmp[7]);
    tmp[6] = tmp[11].fmul(y);
    tmp[13] = tmp[11].fsub(tmp[13]);
    tmp[11] = nn::fms213(tmp[11], y, tmp[6]);
    tmp[3] = nn::fms132(tmp[3], tmp[13], f(A_380));
    tmp[3] = tmp[5].fadd(tmp[3]);
    tmp[10] = nn::fma213(tmp[10], tmp[2], tmp[3]);
    let product_mag_bits = tmp[6].to_bits() & A_400;
    tmp[3] = nn::fma213(y, tmp[10], tmp[6]);
    let product_out_of_range = !(f(product_mag_bits) <= f(A_440));
    tmp[2] = tmp[3].fsub(tmp[6]);
    let mask = mask | product_out_of_range;
    tmp[1] = tmp[3].fadd(f(A_480));
    tmp[10] = nn::fms213(tmp[10], y, tmp[2]);
    let scale_bits = tmp[1].to_bits() << 20;
    tmp[4] = tmp[1].fsub(f(A_480));
    tmp[10] = tmp[10].fadd(tmp[11]);
    tmp[2] = tmp[3].fsub(tmp[4]);
    tmp[7] = nn::fma213(f(A_500), tmp[2], f(A_540));
    tmp[6] = tmp[2].fadd(tmp[10]);
    tmp[13] = f(t.exp_table[tmp[1].to_bits() as usize & 7]);
    tmp[7] = nn::fma213(tmp[7], tmp[6], f(A_5C0));
    tmp[11] = tmp[13].fmul(tmp[6]);
    tmp[11] = nn::fma213(tmp[11], tmp[7], tmp[13]);
    tmp[1] = f(scale_bits & A_580);
    let out = tmp[11].fmul(tmp[1]).to_bits();
    if mask { powf_scalar_bits(xb, yb, out) } else { out }
}

/// One lane of the exponential routine, e^x.
pub fn expf_lane(xb: u32) -> u32 {
    let x = f(xb);
    let mut tmp = [0f32; 8];
    tmp[1] = nn::fma213(f(B_000), x, f(B_040));
    tmp[4] = tmp[1].fsub(f(B_040));
    tmp[7] = nn::fnma231(x, f(B_080), tmp[4]);
    let scale_bits = tmp[1].to_bits() << 23;
    tmp[4] = nn::fnma132(tmp[4], tmp[7], f(B_0C0));
    tmp[3] = nn::fma213(f(B_280), tmp[4], f(B_240));
    tmp[3] = nn::fma213(tmp[3], tmp[4], f(B_200));
    tmp[3] = nn::fma213(tmp[3], tmp[4], f(B_1C0));
    tmp[3] = nn::fma213(tmp[3], tmp[4], f(B_180));
    tmp[3] = nn::fma213(tmp[3], tmp[4], f(B_140));
    let big = ((xb & B_2C0) as i32) > (B_300 as i32);
    let out = scale_bits.wrapping_add(tmp[3].to_bits());
    if big { expf_scalar_bits(xb, out) } else { out }
}

/// The power routine on 8 lanes.
pub fn powf8(x: [u32; 8], y: [u32; 8]) -> [u32; 8] {
    let mut o = [0u32; 8];
    for i in 0..8 {
        o[i] = powf_lane(x[i], y[i]);
    }
    o
}

/// The exponential routine on 8 lanes.
pub fn expf8(x: [u32; 8]) -> [u32; 8] {
    let mut o = [0u32; 8];
    for i in 0..8 {
        o[i] = expf_lane(x[i]);
    }
    o
}
