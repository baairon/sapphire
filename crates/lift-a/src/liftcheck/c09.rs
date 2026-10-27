#![allow(unused_assignments, unused_variables, unused_mut, unused_parens, unreachable_code, unused_imports, clippy::all)]
use crate::lift_rt::*;
use crate::Mem;

/// lifted from the machine code of @0xd14c4 (size=359); checked against the P1 vectors
pub fn f_000d14c4(m: &mut dyn Mem, p_0: u32) -> u32 {
    let mut stk = Stack::new();
    let mut rax: u64 = 0;
    let mut rbx: u64 = 0x1111111111111111u64;
    let mut rcx: u64 = 0;
    let mut rdx: u64 = 0;
    let mut rsi: u64 = 0;
    let mut rdi: u64 = 0;
    let mut rbp: u64 = 0x2222222222222222u64;
    let mut rsp: u64 = RSP_ENTRY;
    let mut r8: u64 = 0;
    let mut r9: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = 0x3333333333333333u64;
    let mut r13: u64 = 0x4444444444444444u64;
    let mut r14: u64 = 0x5555555555555555u64;
    let mut r15: u64 = 0x6666666666666666u64;
    let mut xmm0: u128 = 0;
    let mut xmm1: u128 = 0;
    let mut xmm2: u128 = 0;
    let mut xmm3: u128 = 0;
    let mut xmm4: u128 = 0;
    let mut xmm5: u128 = 0;
    let mut xmm6: u128 = 0;
    let mut xmm7: u128 = 0;
    let mut xmm8: u128 = 0;
    let mut xmm9: u128 = 0;
    let mut xmm10: u128 = 0;
    let mut xmm11: u128 = 0;
    let mut xmm12: u128 = 0;
    let mut xmm13: u128 = 0;
    let mut xmm14: u128 = 0;
    let mut xmm15: u128 = 0;
    let (mut cf, mut zf, mut sf, mut ov, mut pf) = (false, false, false, false, false);
    rdi = p_0 as u64;
    let mut pc: u64 = 0xd14c4;
    loop {
        match pc {
            0xd14c4 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (0x800000u64) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1032294fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd14fd; continue; }
                pc = 0xd14d5; continue;
            }
            0xd14d5 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x102802ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd1546; continue; }
                pc = 0xd14dd; continue;
            }
            0xd14dd => {
                {
                    rcx = (rdi.wrapping_sub(0x10266400u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xd1629; continue; }
                pc = 0xd14ec; continue;
            }
            0xd14ec => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10244800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd1627; continue; }
                pc = 0xd14f8; continue;
            }
            0xd14f8 => {
                pc = 0xd1629; continue;
            }
            0xd14fd => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x106344ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd156b; continue; }
                pc = 0xd1505; continue;
            }
            0xd1505 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105183ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd15a2; continue; }
                pc = 0xd1511; continue;
            }
            0xd1511 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x104020ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd15eb; continue; }
                pc = 0xd151d; continue;
            }
            0xd151d => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10322950u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd1529; continue;
            }
            0xd1529 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10352300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd1535; continue;
            }
            0xd1535 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10378800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd1627; continue; }
                pc = 0xd1541; continue;
            }
            0xd1541 => {
                pc = 0xd1629; continue;
            }
            0xd1546 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x102946ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd15bc; continue; }
                pc = 0xd154e; continue;
            }
            0xd154e => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10280300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd155a; continue;
            }
            0xd155a => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10293000u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd1627; continue; }
                pc = 0xd1566; continue;
            }
            0xd1566 => {
                pc = 0xd1629; continue;
            }
            0xd156b => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x106681ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd15ce; continue; }
                pc = 0xd1573; continue;
            }
            0xd1573 => {
                {
                    rcx = (rdi.wrapping_sub(0x10646700u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xd1629; continue; }
                pc = 0xd1582; continue;
            }
            0xd1582 => {
                {
                    rcx = (rdi.wrapping_sub(0x10646705u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xd1629; continue; }
                pc = 0xd1591; continue;
            }
            0xd1591 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10634500u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd1627; continue; }
                pc = 0xd159d; continue;
            }
            0xd159d => {
                pc = 0xd1629; continue;
            }
            0xd15a2 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105381ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd1605; continue; }
                pc = 0xd15aa; continue;
            }
            0xd15aa => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd15b2; continue;
            }
            0xd15b2 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518450u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd1627; continue; }
                pc = 0xd15ba; continue;
            }
            0xd15ba => {
                pc = 0xd1629; continue;
            }
            0xd15bc => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294700u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd15c4; continue;
            }
            0xd15c4 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294900u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd1627; continue; }
                pc = 0xd15cc; continue;
            }
            0xd15cc => {
                pc = 0xd1629; continue;
            }
            0xd15ce => {
                {
                    rcx = (rdi.wrapping_sub(0x10710850u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xd1629; continue; }
                pc = 0xd15d9; continue;
            }
            0xd15d9 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10668200u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd15e1; continue;
            }
            0xd15e1 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10710800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd15e9; continue;
            }
            0xd15e9 => {
                pc = 0xd1627; continue;
            }
            0xd15eb => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x104782ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd1617; continue; }
                pc = 0xd15f3; continue;
            }
            0xd15f3 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10402100u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd15fb; continue;
            }
            0xd15fb => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10416000u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd1627; continue; }
                pc = 0xd1603; continue;
            }
            0xd1603 => {
                pc = 0xd1629; continue;
            }
            0xd1605 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10538200u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd160d; continue;
            }
            0xd160d => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10565400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd1627; continue; }
                pc = 0xd1615; continue;
            }
            0xd1615 => {
                pc = 0xd1629; continue;
            }
            0xd1617 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd161f; continue;
            }
            0xd161f => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478350u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd1629; continue; }
                pc = 0xd1627; continue;
            }
            0xd1627 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xd1629; continue;
            }
            0xd1629 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xffffffff) as u32;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x65977 (size=376); checked against the P1 vectors
pub fn f_00065977(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32, p_3: u32, p_4: u32) {
    let mut stk = Stack::new();
    let mut rax: u64 = 0;
    let mut rbx: u64 = 0x1111111111111111u64;
    let mut rcx: u64 = 0;
    let mut rdx: u64 = 0;
    let mut rsi: u64 = 0;
    let mut rdi: u64 = 0;
    let mut rbp: u64 = 0x2222222222222222u64;
    let mut rsp: u64 = RSP_ENTRY;
    let mut r8: u64 = 0;
    let mut r9: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = 0x3333333333333333u64;
    let mut r13: u64 = 0x4444444444444444u64;
    let mut r14: u64 = 0x5555555555555555u64;
    let mut r15: u64 = 0x6666666666666666u64;
    let mut xmm0: u128 = 0;
    let mut xmm1: u128 = 0;
    let mut xmm2: u128 = 0;
    let mut xmm3: u128 = 0;
    let mut xmm4: u128 = 0;
    let mut xmm5: u128 = 0;
    let mut xmm6: u128 = 0;
    let mut xmm7: u128 = 0;
    let mut xmm8: u128 = 0;
    let mut xmm9: u128 = 0;
    let mut xmm10: u128 = 0;
    let mut xmm11: u128 = 0;
    let mut xmm12: u128 = 0;
    let mut xmm13: u128 = 0;
    let mut xmm14: u128 = 0;
    let mut xmm15: u128 = 0;
    let (mut cf, mut zf, mut sf, mut ov, mut pf) = (false, false, false, false, false);
    rdi = p_0;
    rsi = p_1;
    rdx = p_2 as u64;
    rcx = p_3 as u64;
    r8 = p_4 as u64;
    let mut pc: u64 = 0x65977;
    loop {
        match pc {
            0x65977 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let v = r15;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    let v = r14;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    let v = rbx;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    ov = false;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if zf || sf != ov { pc = 0x65ae8; continue; }
                pc = 0x65988; continue;
            }
            0x65988 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0xfffffffcu64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (((rcx & 0xffffffff)) as i32 as i64 as u64);
                }
                {
                    let a = rsi;
                    let b = rax;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    r9 = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x65998; continue;
            }
            0x65998 => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x2u64);
                    r10 = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xff);
                    let b = (r8 & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x659fc; continue; }
                pc = 0x659a3; continue;
            }
            0x659a3 => {
                {
                    let a = (r10 & 0xffffffff) as i32 as i128;
                    let b = 16828 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x1u64);
                    r15 = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (r15 & 0xffffffff) as i32 as i128;
                    let b = 33039 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(4));
                    r11 = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff) as i32 as i128;
                    let b = 6416 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    r14 = (rbx.wrapping_add(rdx).wrapping_add(0x8000u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (r14 & 0xffffffff);
                    let c = 16u64;
                    let r = a >> c;
                    r14 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r14 & 0xffffffff);
                    let b = 0x10u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r14 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff) as i32 as i128;
                    let b = -9713 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r15 & 0xffffffff) as i32 as i128;
                    let b = -19070 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff) as i32 as i128;
                    let b = 28784 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    sf = (r >> 31) & 1 != 0;
                    rax = (r) & 0xffff_ffffu64;
                }
                if sf { pc = 0x65a55; continue; }
                pc = 0x659f0; continue;
            }
            0x659f0 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x8000u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 16u64;
                    let sa = ((a << 32) as i64 >> 32) as i128;
                    let r = ((sa >> c) as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rdx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                pc = 0x65a61; continue;
            }
            0x659fc => {
                {
                    let a = (r10 & 0xffffffff) as i32 as i128;
                    let b = 11914 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x1u64);
                    r15 = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (r15 & 0xffffffff) as i32 as i128;
                    let b = 40095 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(4));
                    r11 = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff) as i32 as i128;
                    let b = 4050 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    r14 = (rbx.wrapping_add(rdx).wrapping_add(0x8000u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (r14 & 0xffffffff);
                    let c = 16u64;
                    let r = a >> c;
                    r14 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r14 & 0xffffffff);
                    let b = 0x10u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r14 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff) as i32 as i128;
                    let b = -6388 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r15 & 0xffffffff) as i32 as i128;
                    let b = -21595 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff) as i32 as i128;
                    let b = 28073 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    sf = (r >> 31) & 1 != 0;
                    rax = (r) & 0xffff_ffffu64;
                }
                if sf { pc = 0x65a7d; continue; }
                pc = 0x65a49; continue;
            }
            0x65a49 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x8000u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 16u64;
                    let sa = ((a << 32) as i64 >> 32) as i128;
                    let r = ((sa >> c) as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rdx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                pc = 0x65a89; continue;
            }
            0x65a55 => {
                {
                    rdx = (0x8000u64) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let c = 16u64;
                    let sa = ((a << 32) as i64 >> 32) as i128;
                    let r = ((sa >> c) as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let r = 0u64.wrapping_sub(a) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                pc = 0x65a61; continue;
            }
            0x65a61 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0xffffff80u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff) as i32 as i128;
                    let b = 28784 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r15 & 0xffffffff) as i32 as i128;
                    let b = -24103 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff) as i32 as i128;
                    let b = -4681 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x65aa3; continue;
            }
            0x65a7d => {
                {
                    rdx = (0x8000u64) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let c = 16u64;
                    let sa = ((a << 32) as i64 >> 32) as i128;
                    let r = ((sa >> c) as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let r = 0u64.wrapping_sub(a) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                pc = 0x65a89; continue;
            }
            0x65a89 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0xffffff80u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff) as i32 as i128;
                    let b = 28017 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r15 & 0xffffffff) as i32 as i128;
                    let b = -25349 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff) as i32 as i128;
                    let b = -2566 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x65aa3; continue;
            }
            0x65aa3 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rbx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    sf = (r >> 31) & 1 != 0;
                    rax = (r) & 0xffff_ffffu64;
                }
                if sf { pc = 0x65ab3; continue; }
                pc = 0x65aa7; continue;
            }
            0x65aa7 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x8000u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 16u64;
                    let sa = ((a << 32) as i64 >> 32) as i128;
                    let r = ((sa >> c) as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rbx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                pc = 0x65abf; continue;
            }
            0x65ab3 => {
                {
                    rbx = (0x8000u64) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let c = 16u64;
                    let sa = ((a << 32) as i64 >> 32) as i128;
                    let r = ((sa >> c) as u64) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let r = 0u64.wrapping_sub(a) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                pc = 0x65abf; continue;
            }
            0x65abf => {
                {
                    let a = (rcx & 0xff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x65ad6; continue; }
                pc = 0x65ac4; continue;
            }
            0x65ac4 => {
                {
                    let a = (rbx & 0xffffffff);
                    let b = 0xffffff80u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x1u64);
                    stm(m, &mut stk, ea, 1, ((r14 & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((rbx & 0xff)) & 0xffu64);
                }
                {
                    let a = rsi;
                    let b = 0x3u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                pc = 0x65adc; continue;
            }
            0x65ad6 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 1, ((r14 & 0xff)) & 0xffu64);
                }
                {
                    let a = rsi;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                pc = 0x65adc; continue;
            }
            0x65adc => {
                {
                    let a = rcx;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x65998; continue; }
                pc = 0x65ae8; continue;
            }
            0x65ae8 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbx = v;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    r14 = v;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    r15 = v;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xd20a9 (size=399); checked against the P1 vectors
pub fn f_000d20a9(m: &mut dyn Mem, p_0: u32) -> u32 {
    let mut stk = Stack::new();
    let mut rax: u64 = 0;
    let mut rbx: u64 = 0x1111111111111111u64;
    let mut rcx: u64 = 0;
    let mut rdx: u64 = 0;
    let mut rsi: u64 = 0;
    let mut rdi: u64 = 0;
    let mut rbp: u64 = 0x2222222222222222u64;
    let mut rsp: u64 = RSP_ENTRY;
    let mut r8: u64 = 0;
    let mut r9: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = 0x3333333333333333u64;
    let mut r13: u64 = 0x4444444444444444u64;
    let mut r14: u64 = 0x5555555555555555u64;
    let mut r15: u64 = 0x6666666666666666u64;
    let mut xmm0: u128 = 0;
    let mut xmm1: u128 = 0;
    let mut xmm2: u128 = 0;
    let mut xmm3: u128 = 0;
    let mut xmm4: u128 = 0;
    let mut xmm5: u128 = 0;
    let mut xmm6: u128 = 0;
    let mut xmm7: u128 = 0;
    let mut xmm8: u128 = 0;
    let mut xmm9: u128 = 0;
    let mut xmm10: u128 = 0;
    let mut xmm11: u128 = 0;
    let mut xmm12: u128 = 0;
    let mut xmm13: u128 = 0;
    let mut xmm14: u128 = 0;
    let mut xmm15: u128 = 0;
    let (mut cf, mut zf, mut sf, mut ov, mut pf) = (false, false, false, false, false);
    rdi = p_0 as u64;
    let mut pc: u64 = 0xd20a9;
    loop {
        match pc {
            0xd20a9 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (0xeu64) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1032294fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd20e2; continue; }
                pc = 0xd20ba; continue;
            }
            0xd20ba => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x102802ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd2130; continue; }
                pc = 0xd20c2; continue;
            }
            0xd20c2 => {
                {
                    rax = (rdi.wrapping_sub(0x10266400u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xd21be; continue; }
                pc = 0xd20d1; continue;
            }
            0xd20d1 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10244800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd21be; continue; }
                pc = 0xd20dd; continue;
            }
            0xd20dd => {
                pc = 0xd2231; continue;
            }
            0xd20e2 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x106344ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd2155; continue; }
                pc = 0xd20ea; continue;
            }
            0xd20ea => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105183ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd218c; continue; }
                pc = 0xd20f6; continue;
            }
            0xd20f6 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x104020ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd21df; continue; }
                pc = 0xd2102; continue;
            }
            0xd2102 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10322950u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd21ef; continue; }
                pc = 0xd210e; continue;
            }
            0xd210e => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10352300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd2236; continue; }
                pc = 0xd211a; continue;
            }
            0xd211a => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10378800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd2231; continue; }
                pc = 0xd2126; continue;
            }
            0xd2126 => {
                {
                    rax = (0x16u64) & 0xffff_ffffu64;
                }
                pc = 0xd2236; continue;
            }
            0xd2130 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x102946ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd21ae; continue; }
                pc = 0xd2138; continue;
            }
            0xd2138 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10280300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd2236; continue; }
                pc = 0xd2144; continue;
            }
            0xd2144 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10293000u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd2231; continue; }
                pc = 0xd2150; continue;
            }
            0xd2150 => {
                pc = 0xd21ef; continue;
            }
            0xd2155 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x106681ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd21c5; continue; }
                pc = 0xd215d; continue;
            }
            0xd215d => {
                {
                    rax = (rdi.wrapping_sub(0x10646700u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xd2222; continue; }
                pc = 0xd216c; continue;
            }
            0xd216c => {
                {
                    rax = (rdi.wrapping_sub(0x10646705u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xd2222; continue; }
                pc = 0xd217b; continue;
            }
            0xd217b => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10634500u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd2222; continue; }
                pc = 0xd2187; continue;
            }
            0xd2187 => {
                pc = 0xd2231; continue;
            }
            0xd218c => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105381ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd21f6; continue; }
                pc = 0xd2194; continue;
            }
            0xd2194 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd2222; continue; }
                pc = 0xd21a0; continue;
            }
            0xd21a0 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518450u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd2231; continue; }
                pc = 0xd21ac; continue;
            }
            0xd21ac => {
                pc = 0xd2222; continue;
            }
            0xd21ae => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294700u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd21d8; continue; }
                pc = 0xd21b6; continue;
            }
            0xd21b6 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294900u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd2231; continue; }
                pc = 0xd21be; continue;
            }
            0xd21be => {
                {
                    rax = (0x1eu64) & 0xffff_ffffu64;
                }
                pc = 0xd2236; continue;
            }
            0xd21c5 => {
                {
                    rax = (rdi.wrapping_sub(0x10710850u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xd2222; continue; }
                pc = 0xd21d0; continue;
            }
            0xd21d0 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10668200u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd221a; continue; }
                pc = 0xd21d8; continue;
            }
            0xd21d8 => {
                {
                    rax = (0x36u64) & 0xffff_ffffu64;
                }
                pc = 0xd2236; continue;
            }
            0xd21df => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x104782ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd2208; continue; }
                pc = 0xd21e7; continue;
            }
            0xd21e7 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10402100u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd2229; continue; }
                pc = 0xd21ef; continue;
            }
            0xd21ef => {
                {
                    rax = (0x3eu64) & 0xffff_ffffu64;
                }
                pc = 0xd2236; continue;
            }
            0xd21f6 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10538200u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd2222; continue; }
                pc = 0xd21fe; continue;
            }
            0xd21fe => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10565400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd2231; continue; }
                pc = 0xd2206; continue;
            }
            0xd2206 => {
                pc = 0xd2222; continue;
            }
            0xd2208 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd2222; continue; }
                pc = 0xd2210; continue;
            }
            0xd2210 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478350u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd2231; continue; }
                pc = 0xd2218; continue;
            }
            0xd2218 => {
                pc = 0xd2222; continue;
            }
            0xd221a => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10710800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd2231; continue; }
                pc = 0xd2222; continue;
            }
            0xd2222 => {
                {
                    rax = (0x6du64) & 0xffff_ffffu64;
                }
                pc = 0xd2236; continue;
            }
            0xd2229 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10416000u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd2236; continue; }
                pc = 0xd2231; continue;
            }
            0xd2231 => {
                {
                    rax = (0xfffffffeu64) & 0xffff_ffffu64;
                }
                pc = 0xd2236; continue;
            }
            0xd2236 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xffffffff) as u32;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x5a733 (size=425); checked against the P1 vectors
pub fn f_0005a733(m: &mut dyn Mem, p_0: u32) -> u32 {
    let mut stk = Stack::new();
    let mut rax: u64 = 0;
    let mut rbx: u64 = 0x1111111111111111u64;
    let mut rcx: u64 = 0;
    let mut rdx: u64 = 0;
    let mut rsi: u64 = 0;
    let mut rdi: u64 = 0;
    let mut rbp: u64 = 0x2222222222222222u64;
    let mut rsp: u64 = RSP_ENTRY;
    let mut r8: u64 = 0;
    let mut r9: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = 0x3333333333333333u64;
    let mut r13: u64 = 0x4444444444444444u64;
    let mut r14: u64 = 0x5555555555555555u64;
    let mut r15: u64 = 0x6666666666666666u64;
    let mut xmm0: u128 = 0;
    let mut xmm1: u128 = 0;
    let mut xmm2: u128 = 0;
    let mut xmm3: u128 = 0;
    let mut xmm4: u128 = 0;
    let mut xmm5: u128 = 0;
    let mut xmm6: u128 = 0;
    let mut xmm7: u128 = 0;
    let mut xmm8: u128 = 0;
    let mut xmm9: u128 = 0;
    let mut xmm10: u128 = 0;
    let mut xmm11: u128 = 0;
    let mut xmm12: u128 = 0;
    let mut xmm13: u128 = 0;
    let mut xmm14: u128 = 0;
    let mut xmm15: u128 = 0;
    let (mut cf, mut zf, mut sf, mut ov, mut pf) = (false, false, false, false, false);
    rdi = p_0 as u64;
    let mut pc: u64 = 0x5a733;
    loop {
        match pc {
            0x5a733 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x6du64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                pc = 0x5a73a; continue;
            }
            0x5a73a => {
                if !zf && sf == ov { pc = 0x5a760; continue; }
                pc = 0x5a73c; continue;
            }
            0x5a73c => {
                {
                    rcx = (rdi.wrapping_sub(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x1au64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5a768; continue; }
                pc = 0x5a744; continue;
            }
            0x5a744 => {
                {
                    rax = (0x50u64) & 0xffff_ffffu64;
                }
                {
                    rdx = 0x1005a870u64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(rcx.wrapping_mul(4));
                    rcx = ((ldm(m, &stk, ea, 4)) as i32 as i64 as u64);
                }
                {
                    let a = rcx;
                    let b = rdx;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    ov = (!(a ^ b) & (a ^ r)) >> 63 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 63) & 1 != 0;
                    rcx = r;
                }
                pc = rcx.wrapping_sub(0x1000_0000u64); continue;
            }
            0x5a759 => {
                {
                    rax = (0x64u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a760 => {
                {
                    rax = (rdi.wrapping_sub(0x6eu64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xbu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if cf || zf { pc = 0x5a76c; continue; }
                pc = 0x5a768; continue;
            }
            0x5a768 => {
                {
                    rax = ((rdi & 0xffffffff)) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a76a => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xffffffff) as u32;
            }
            0x5a76c => {
                {
                    rcx = 0x1005a840u64;
                }
                {
                    let ea: u64 = rcx.wrapping_add(rax.wrapping_mul(4));
                    rax = ((ldm(m, &stk, ea, 4)) as i32 as i64 as u64);
                }
                {
                    let a = rax;
                    let b = rcx;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    ov = (!(a ^ b) & (a ^ r)) >> 63 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 63) & 1 != 0;
                    rax = r;
                }
                pc = rax.wrapping_sub(0x1000_0000u64); continue;
            }
            0x5a77c => {
                {
                    rax = (0x69u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a783 => {
                {
                    rax = (0x68u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a78a => {
                {
                    rax = (0x67u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a791 => {
                {
                    rax = (0x6au64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a798 => {
                {
                    rax = (0x6bu64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a79f => {
                {
                    rax = (0x65u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7a6 => {
                {
                    rax = (0x66u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7ad => {
                {
                    rax = (0x60u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7b4 => {
                {
                    rax = (0x61u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7bb => {
                {
                    rax = (0x5eu64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7c2 => {
                {
                    rax = (0x5fu64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7c9 => {
                {
                    rax = (0x51u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7d0 => {
                {
                    rax = (0x5cu64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7d7 => {
                {
                    rax = (0x5du64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7de => {
                {
                    rax = (0x55u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7e5 => {
                {
                    rax = (0x53u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7ef => {
                {
                    rax = (0x54u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a7f9 => {
                {
                    rax = (0x59u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a803 => {
                {
                    rax = (0x5au64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a80d => {
                {
                    rax = (0x56u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a817 => {
                {
                    rax = (0x57u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a821 => {
                {
                    rax = (0x5bu64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a82b => {
                {
                    rax = (0x58u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a835 => {
                {
                    rax = (0x52u64) & 0xffff_ffffu64;
                }
                pc = 0x5a76a; continue;
            }
            0x5a83f => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x325ca (size=446); checked against the P1 vectors
pub fn f_000325ca(m: &mut dyn Mem, p_0: u64) {
    let mut stk = Stack::new();
    let mut rax: u64 = 0;
    let mut rbx: u64 = 0x1111111111111111u64;
    let mut rcx: u64 = 0;
    let mut rdx: u64 = 0;
    let mut rsi: u64 = 0;
    let mut rdi: u64 = 0;
    let mut rbp: u64 = 0x2222222222222222u64;
    let mut rsp: u64 = RSP_ENTRY;
    let mut r8: u64 = 0;
    let mut r9: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = 0x3333333333333333u64;
    let mut r13: u64 = 0x4444444444444444u64;
    let mut r14: u64 = 0x5555555555555555u64;
    let mut r15: u64 = 0x6666666666666666u64;
    let mut xmm0: u128 = 0;
    let mut xmm1: u128 = 0;
    let mut xmm2: u128 = 0;
    let mut xmm3: u128 = 0;
    let mut xmm4: u128 = 0;
    let mut xmm5: u128 = 0;
    let mut xmm6: u128 = 0;
    let mut xmm7: u128 = 0;
    let mut xmm8: u128 = 0;
    let mut xmm9: u128 = 0;
    let mut xmm10: u128 = 0;
    let mut xmm11: u128 = 0;
    let mut xmm12: u128 = 0;
    let mut xmm13: u128 = 0;
    let mut xmm14: u128 = 0;
    let mut xmm15: u128 = 0;
    let (mut cf, mut zf, mut sf, mut ov, mut pf) = (false, false, false, false, false);
    rdi = p_0;
    let mut pc: u64 = 0x325ca;
    loop {
        match pc {
            0x325ca => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x16u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0x325dd; continue; }
                pc = 0x325d6; continue;
            }
            0x325d6 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0xau64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x3260e; continue; }
                pc = 0x325db; continue;
            }
            0x325db => {
                pc = 0x32655; continue;
            }
            0x325dd => {
                {
                    rax = (rcx.wrapping_sub(0x17u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x8u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x3260e; continue; }
                pc = 0x325e5; continue;
            }
            0x325e5 => {
                {
                    rdx = 0x10032764u64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(rax.wrapping_mul(4));
                    rax = ((ldm(m, &stk, ea, 4)) as i32 as i64 as u64);
                }
                {
                    let a = rax;
                    let b = rdx;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                pc = rax.wrapping_sub(0x1000_0000u64); continue;
            }
            0x325f5 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 2, (0x2u64) & 0xffffu64);
                }
                {
                    rax = 0x1400000014u64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x326d5; continue;
            }
            0x3260e => {
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, (0xcu64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 4, (0x20u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 4, (0x3u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 1, (0x0u64) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 2, (0x0u64) & 0xffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                pc = 0x326ce; continue;
            }
            0x32655 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 2, (0x3u64) & 0xffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    r8 = rdi.wrapping_add(0x24u64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    rsi = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = (rsi & 0xffffffff) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0xau64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                {
                    rcx = (rcx & !0xffu64) | (((zf) as u64) & 0xff);
                }
                {
                    let a = (rcx & 0xff);
                    let r = a.wrapping_add(1) & 0xffu64;
                    rcx = (rcx & !0xffu64) | ((r) & 0xff);
                }
                {
                    rax = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = (rcx & 31u64);
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xff);
                    let b = 0x3u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x32689; continue; }
                pc = 0x3267b; continue;
            }
            0x3267b => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let b = (rsi & 0xffffffff) as u32 as u128;
                    if b == 0 { panic!("divide error"); }
                    let n = ((((rdx & 0xffff_ffff) << 32) | (rax & 0xffff_ffff)) as u64) as u128;
                    let q = n / b;
                    let rem = n % b;
                    if (q >> 32) != 0 { panic!("divide overflow"); }
                    rax = (q as u64) & 0xffff_ffff;
                    rdx = (rem as u64) & 0xffff_ffff;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x326d5; continue; }
                pc = 0x32689; continue;
            }
            0x32689 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, (0xcu64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 4, (0x20u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 4, (0x3u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 1, (0x0u64) & 0xffu64);
                }
                {
                    let ea: u64 = r8.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 2, (0x0u64) & 0xffffu64);
                }
                {
                    let ea: u64 = r8.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = r8;
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                pc = 0x326ce; continue;
            }
            0x326ce => {
                {
                    let ea: u64 = rdi.wrapping_add(0x38u64);
                    stm(m, &mut stk, ea, 4, (0x12u64) & 0xffffffffu64);
                }
                pc = 0x326d5; continue;
            }
            0x326d5 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x326d7 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 2, (0x3u64) & 0xffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    rsi = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff) as i32 as i128;
                    let b = (rsi & 0xffffffff) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x1au64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                {
                    rcx = (rcx & !0xffu64) | (((zf) as u64) & 0xff);
                }
                {
                    let a = (rcx & 0xff);
                    let r = a.wrapping_add(1) & 0xffu64;
                    rcx = (rcx & !0xffu64) | ((r) & 0xff);
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = (rcx & 31u64);
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let b = (rsi & 0xffffffff) as u32 as u128;
                    if b == 0 { panic!("divide error"); }
                    let n = ((((rdx & 0xffff_ffff) << 32) | (rax & 0xffff_ffff)) as u64) as u128;
                    let q = n / b;
                    let rem = n % b;
                    if (q >> 32) != 0 { panic!("divide overflow"); }
                    rax = (q as u64) & 0xffff_ffff;
                    rdx = (rem as u64) & 0xffff_ffff;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x326d5; continue; }
                pc = 0x32702; continue;
            }
            0x32702 => {
                {
                    rax = rdi.wrapping_add(0x24u64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, (0xcu64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 4, (0x20u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 4, (0x3u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 1, (0x0u64) & 0xffu64);
                }
                {
                    let ea: u64 = rax.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 2, (0x0u64) & 0xffffu64);
                }
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rax;
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                pc = 0x326ce; continue;
            }
            0x3274c => {
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 2, (0x2u64) & 0xffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 1u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                pc = 0x326d5; continue;
            }
            0x32762 => {
                {
                    let x = (rax & 0xffff);
                    let y = (rax & 0xffff);
                    rax = (rax & !0xffffu64) | ((y) & 0xffff);
                    rax = (rax & !0xffffu64) | ((x) & 0xffff);
                }
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x64cf0 (size=520); checked against the P1 vectors
pub fn f_00064cf0(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64, p_3: u32, p_4: u32) -> u64 {
    let mut stk = Stack::new();
    let mut rax: u64 = 0;
    let mut rbx: u64 = 0x1111111111111111u64;
    let mut rcx: u64 = 0;
    let mut rdx: u64 = 0;
    let mut rsi: u64 = 0;
    let mut rdi: u64 = 0;
    let mut rbp: u64 = 0x2222222222222222u64;
    let mut rsp: u64 = RSP_ENTRY;
    let mut r8: u64 = 0;
    let mut r9: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = 0x3333333333333333u64;
    let mut r13: u64 = 0x4444444444444444u64;
    let mut r14: u64 = 0x5555555555555555u64;
    let mut r15: u64 = 0x6666666666666666u64;
    let mut xmm0: u128 = 0;
    let mut xmm1: u128 = 0;
    let mut xmm2: u128 = 0;
    let mut xmm3: u128 = 0;
    let mut xmm4: u128 = 0;
    let mut xmm5: u128 = 0;
    let mut xmm6: u128 = 0;
    let mut xmm7: u128 = 0;
    let mut xmm8: u128 = 0;
    let mut xmm9: u128 = 0;
    let mut xmm10: u128 = 0;
    let mut xmm11: u128 = 0;
    let mut xmm12: u128 = 0;
    let mut xmm13: u128 = 0;
    let mut xmm14: u128 = 0;
    let mut xmm15: u128 = 0;
    let (mut cf, mut zf, mut sf, mut ov, mut pf) = (false, false, false, false, false);
    rdi = p_0;
    rsi = p_1;
    rdx = p_2 as u64;
    rcx = p_3 as u64;
    r8 = p_4 as u64;
    let mut pc: u64 = 0x64cf0;
    loop {
        match pc {
            0x64cf0 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x30u64);
                    r11 = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x58u64);
                    r9 = ldm(m, &stk, ea, 8);
                }
                {
                    let a = r9;
                    let b = r11;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0x64d08; continue; }
                pc = 0x64d01; continue;
            }
            0x64d01 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x58u64);
                    stm(m, &mut stk, ea, 8, (r11) & 0xffffffffffffffffu64);
                }
                {
                    r9 = r11;
                }
                pc = 0x64d08; continue;
            }
            0x64d08 => {
                {
                    rax = ((r8 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x18u64;
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                    rax = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x64db5; continue; }
                pc = 0x64d14; continue;
            }
            0x64d14 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x1u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x64d22; continue; }
                pc = 0x64d19; continue;
            }
            0x64d19 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x18u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x64db5; continue; }
                pc = 0x64d22; continue;
            }
            0x64d22 => {
                {
                    let a = (r10 & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x64d65; continue; }
                pc = 0x64d29; continue;
            }
            0x64d29 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x64d3d; continue; }
                pc = 0x64d2e; continue;
            }
            0x64d2e => {
                {
                    let ea: u64 = rsi.wrapping_add(0x40u64);
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x64d51; continue; }
                pc = 0x64d34; continue;
            }
            0x64d34 => {
                {
                    rcx = rsi.wrapping_add(0x40u64);
                }
                {
                    let a = rcx;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                pc = 0x64d55; continue;
            }
            0x64d3d => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x1u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x64db5; continue; }
                pc = 0x64d42; continue;
            }
            0x64d42 => {
                {
                    let a = (r8 & 0xff);
                    let b = 0x8u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x64d5d; continue; }
                pc = 0x64d48; continue;
            }
            0x64d48 => {
                {
                    r10 = r11;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x28u64);
                    let a = r10;
                    let b = ldm(m, &stk, ea, 8);
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    r10 = r;
                }
                pc = 0x64d65; continue;
            }
            0x64d51 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x50u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                pc = 0x64d55; continue;
            }
            0x64d55 => {
                {
                    r10 = r9;
                }
                {
                    let a = r10;
                    let b = rcx;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    r10 = r;
                }
                pc = 0x64d65; continue;
            }
            0x64d5d => {
                {
                    let ea: u64 = rsi.wrapping_add(0x18u64);
                    r10 = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    let a = r10;
                    let b = ldm(m, &stk, ea, 8);
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    r10 = r;
                }
                pc = 0x64d65; continue;
            }
            0x64d65 => {
                {
                    let a = r10;
                    let b = rdx;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    sf = (r >> 63) & 1 != 0;
                    r10 = r;
                }
                if sf { pc = 0x64db5; continue; }
                pc = 0x64d6a; continue;
            }
            0x64d6a => {
                {
                    let ea: u64 = rsi.wrapping_add(0x40u64);
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x64d79; continue; }
                pc = 0x64d70; continue;
            }
            0x64d70 => {
                {
                    rcx = rsi.wrapping_add(0x40u64);
                }
                {
                    let a = rcx;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                pc = 0x64d7d; continue;
            }
            0x64d79 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x50u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                pc = 0x64d7d; continue;
            }
            0x64d7d => {
                {
                    rax = r9;
                }
                {
                    let a = rax;
                    let b = rcx;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let a = rax;
                    let b = r10;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 63 & 1 != 0;
                    sf = (r >> 63) & 1 != 0;
                }
                if sf != ov { pc = 0x64db5; continue; }
                pc = 0x64d88; continue;
            }
            0x64d88 => {
                {
                    rcx = ((r8 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x8u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = r10;
                    let b = r10;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x64e44; continue; }
                pc = 0x64d97; continue;
            }
            0x64d97 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x64da2; continue; }
                pc = 0x64d9b; continue;
            }
            0x64d9b => {
                {
                    let ea: u64 = rsi.wrapping_add(0x18u64);
                    let a = ldm(m, &stk, ea, 8);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x64db5; continue; }
                pc = 0x64da2; continue;
            }
            0x64da2 => {
                {
                    let a = (r8 & 0xff);
                    let b = 0x10u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x64e44; continue; }
                pc = 0x64dac; continue;
            }
            0x64dac => {
                {
                    let a = r11;
                    let b = r11;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x64e44; continue; }
                pc = 0x64db5; continue;
            }
            0x64db5 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x78u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x70u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x68u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x60u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x58u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x50u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x48u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x40u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x38u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x30u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x80u64);
                    stm(m, &mut stk, ea, 8, (0xffffffffffffffffu64) & 0xffffffffffffffffu64);
                }
                pc = 0x64e3f; continue;
            }
            0x64e3f => {
                {
                    rax = rdi;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            0x64e44 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x64e57; continue; }
                pc = 0x64e48; continue;
            }
            0x64e48 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = r10;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 8, (r9) & 0xffffffffffffffffu64);
                }
                pc = 0x64e57; continue;
            }
            0x64e57 => {
                {
                    let a = (r8 & 0xff);
                    let b = 0x10u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x64e68; continue; }
                pc = 0x64e5d; continue;
            }
            0x64e5d => {
                {
                    rax = (((r10 & 0xffffffff)) as i32 as i64 as u64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x28u64);
                    let a = rax;
                    let b = ldm(m, &stk, ea, 8);
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x30u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x64e68; continue;
            }
            0x64e68 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x78u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x70u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x68u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x60u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x58u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x50u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x48u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x40u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x38u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x30u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x80u64);
                    stm(m, &mut stk, ea, 8, (r10) & 0xffffffffffffffffu64);
                }
                pc = 0x64e3f; continue;
            }
            0x64ef3 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x5ad38 (size=524); checked against the P1 vectors
pub fn f_0005ad38(m: &mut dyn Mem, p_0: u32, p_1: u32, p_2: u32) -> u64 {
    let mut stk = Stack::new();
    let mut rax: u64 = 0;
    let mut rbx: u64 = 0x1111111111111111u64;
    let mut rcx: u64 = 0;
    let mut rdx: u64 = 0;
    let mut rsi: u64 = 0;
    let mut rdi: u64 = 0;
    let mut rbp: u64 = 0x2222222222222222u64;
    let mut rsp: u64 = RSP_ENTRY;
    let mut r8: u64 = 0;
    let mut r9: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = 0x3333333333333333u64;
    let mut r13: u64 = 0x4444444444444444u64;
    let mut r14: u64 = 0x5555555555555555u64;
    let mut r15: u64 = 0x6666666666666666u64;
    let mut xmm0: u128 = 0;
    let mut xmm1: u128 = 0;
    let mut xmm2: u128 = 0;
    let mut xmm3: u128 = 0;
    let mut xmm4: u128 = 0;
    let mut xmm5: u128 = 0;
    let mut xmm6: u128 = 0;
    let mut xmm7: u128 = 0;
    let mut xmm8: u128 = 0;
    let mut xmm9: u128 = 0;
    let mut xmm10: u128 = 0;
    let mut xmm11: u128 = 0;
    let mut xmm12: u128 = 0;
    let mut xmm13: u128 = 0;
    let mut xmm14: u128 = 0;
    let mut xmm15: u128 = 0;
    let (mut cf, mut zf, mut sf, mut ov, mut pf) = (false, false, false, false, false);
    rdi = p_0 as u64;
    rsi = p_1 as u64;
    rdx = p_2 as u64;
    let mut pc: u64 = 0x5ad38;
    loop {
        match pc {
            0x5ad38 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    rcx = (0xcccccccdu64) & 0xffff_ffffu64;
                }
                {
                    let a = rcx as i64 as i128;
                    let b = rax as i64 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = rcx;
                    let c = 34u64;
                    let r = a >> c;
                    rcx = r;
                }
                {
                    rax = (rcx.wrapping_add(rcx.wrapping_mul(4))) & 0xffff_ffffu64;
                }
                {
                    r8 = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x1u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5ad83; continue; }
                pc = 0x5ad5b; continue;
            }
            0x5ad5b => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xdu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5aed1; continue; }
                pc = 0x5ad68; continue;
            }
            0x5ad68 => {
                {
                    rax = 0x1005aed4u64;
                }
                {
                    let ea: u64 = rax.wrapping_add(rdi.wrapping_mul(4));
                    rsi = ((ldm(m, &stk, ea, 4)) as i32 as i64 as u64);
                }
                {
                    let a = rsi;
                    let b = rax;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                pc = rsi.wrapping_sub(0x1000_0000u64); continue;
            }
            0x5ad78 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 3840 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5ad83 => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5aed1; continue; }
                pc = 0x5ad8b; continue;
            }
            0x5ad8b => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xdu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5aed1; continue; }
                pc = 0x5ad98; continue;
            }
            0x5ad98 => {
                {
                    rax = 0x1005af0cu64;
                }
                {
                    let ea: u64 = rax.wrapping_add(rdi.wrapping_mul(4));
                    rsi = ((ldm(m, &stk, ea, 4)) as i32 as i64 as u64);
                }
                {
                    let a = rsi;
                    let b = rax;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                pc = rsi.wrapping_sub(0x1000_0000u64); continue;
            }
            0x5ada8 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 800 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5adb3 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 1600 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5adbe => {
                {
                    let a = (rcx & 0xffffffff) as i32 as i128;
                    let b = 8008 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5aece; continue; }
                pc = 0x5adcf; continue;
            }
            0x5adcf => {
                {
                    let a = (r8 & 0xffffffff) as i32 as i128;
                    let b = 1602 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let c = 1u64;
                    let r = a >> c;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aece; continue;
            }
            0x5ade1 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 1920 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5adec => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 2000 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5adf7 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 2002 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5ae02 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 3200 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5ae0d => {
                {
                    let a = (rcx & 0xffffffff) as i32 as i128;
                    let b = 16016 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5aece; continue; }
                pc = 0x5ae1e; continue;
            }
            0x5ae1e => {
                {
                    let a = (r8 & 0xffffffff) as i32 as i128;
                    let b = 3203 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let r = a.wrapping_add(1) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aece; continue;
            }
            0x5ae2c => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 4000 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5ae37 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 4004 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5ae42 => {
                {
                    let a = (rcx & 0xffffffff) as i32 as i128;
                    let b = 4004 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5aece; continue; }
                pc = 0x5ae4f; continue;
            }
            0x5ae4f => {
                {
                    let a = (r8 & 0xffffffff) as i32 as i128;
                    let b = 801 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aece; continue;
            }
            0x5ae58 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 6400 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5ae60 => {
                {
                    let a = (rcx & 0xffffffff) as i32 as i128;
                    let b = 32032 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5aece; continue; }
                pc = 0x5ae6d; continue;
            }
            0x5ae6d => {
                {
                    let a = (r8 & 0xffffffff) as i32 as i128;
                    let b = 6406 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aece; continue;
            }
            0x5ae79 => {
                {
                    let a = (rcx & 0xffffffff) as i32 as i128;
                    let b = 4004 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5aece; continue; }
                pc = 0x5ae86; continue;
            }
            0x5ae86 => {
                {
                    let a = (r8 & 0xffffffff) as i32 as i128;
                    let b = 801 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aece; continue;
            }
            0x5ae91 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 960 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5ae99 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 1000 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5aea1 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 1001 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5aea9 => {
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 400 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5aed1; continue;
            }
            0x5aeb1 => {
                {
                    let a = (rcx & 0xffffffff) as i32 as i128;
                    let b = 2002 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5aece; continue; }
                pc = 0x5aebe; continue;
            }
            0x5aebe => {
                {
                    let a = (r8 & 0xffffffff) as i32 as i128;
                    let b = 400 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let c = 1u64;
                    let r = a >> c;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    rax = r8;
                }
                pc = 0x5aece; continue;
            }
            0x5aece => {
                {
                    let a = rax;
                    let b = rcx;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                pc = 0x5aed1; continue;
            }
            0x5aed1 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            0x5aed3 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x1ac5e (size=708); checked against the P1 vectors
pub fn f_0001ac5e(m: &mut dyn Mem, p_0: u64, p_1: u64) {
    let mut stk = Stack::new();
    let mut rax: u64 = 0;
    let mut rbx: u64 = 0x1111111111111111u64;
    let mut rcx: u64 = 0;
    let mut rdx: u64 = 0;
    let mut rsi: u64 = 0;
    let mut rdi: u64 = 0;
    let mut rbp: u64 = 0x2222222222222222u64;
    let mut rsp: u64 = RSP_ENTRY;
    let mut r8: u64 = 0;
    let mut r9: u64 = 0;
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = 0x3333333333333333u64;
    let mut r13: u64 = 0x4444444444444444u64;
    let mut r14: u64 = 0x5555555555555555u64;
    let mut r15: u64 = 0x6666666666666666u64;
    let mut xmm0: u128 = 0;
    let mut xmm1: u128 = 0;
    let mut xmm2: u128 = 0;
    let mut xmm3: u128 = 0;
    let mut xmm4: u128 = 0;
    let mut xmm5: u128 = 0;
    let mut xmm6: u128 = 0;
    let mut xmm7: u128 = 0;
    let mut xmm8: u128 = 0;
    let mut xmm9: u128 = 0;
    let mut xmm10: u128 = 0;
    let mut xmm11: u128 = 0;
    let mut xmm12: u128 = 0;
    let mut xmm13: u128 = 0;
    let mut xmm14: u128 = 0;
    let mut xmm15: u128 = 0;
    let (mut cf, mut zf, mut sf, mut ov, mut pf) = (false, false, false, false, false);
    rdi = p_0;
    rsi = p_1;
    let mut pc: u64 = 0x1ac5e;
    loop {
        match pc {
            0x1ac5e => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi;
                    r9 = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = r9;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x1aeee; continue; }
                pc = 0x1ac72; continue;
            }
            0x1ac72 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    r8 = 0xffffffffffffff08u64;
                }
                pc = 0x1ac7d; continue;
            }
            0x1ac7d => {
                {
                    let ea: u64 = rdx.wrapping_sub(0xe8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xe8u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xf8u64);
                    r10 = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xf0u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xf0u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xf8u64);
                    stm(m, &mut stk, ea, 8, (r10) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xd0u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xd0u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xe0u64);
                    r10 = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xd8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xd8u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xe0u64);
                    stm(m, &mut stk, ea, 8, (r10) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xd0u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xd8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xe0u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x9cu64);
                    rax = (rax & !0xffffu64) | ((ldm(m, &stk, ea, 2)) & 0xffff);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x9cu64);
                    stm(m, &mut stk, ea, 2, ((rax & 0xffff)) & 0xffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xa0u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xa0u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xa8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xa8u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xb0u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xb0u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xb8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xb8u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xc8u64);
                    r10 = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0xc0u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xc0u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0xc8u64);
                    stm(m, &mut stk, ea, 8, (r10) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x88u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x90u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x98u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x98u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x98u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x88u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x88u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x88u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x90u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x98u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x70u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x78u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x80u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x80u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x80u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x70u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x70u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x70u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x78u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x80u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x58u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x60u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x68u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x68u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x68u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x58u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x58u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x58u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x60u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x68u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x40u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x48u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x50u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x50u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x50u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x40u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x40u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x40u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x48u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x50u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x28u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x30u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x38u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x38u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x38u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x28u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x28u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x28u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x30u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x38u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x8u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x8u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x10u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x10u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x20u64);
                    r10 = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_sub(0x18u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    rdx = rdx.wrapping_sub(0xf8u64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x18u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_sub(0x20u64);
                    stm(m, &mut stk, ea, 8, (r10) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rcx;
                    let b = r8;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let a = r9;
                    let b = rdx;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x1ac7d; continue; }
                pc = 0x1aee9; continue;
            }
            0x1aee9 => {
                {
                    let ea: u64 = rdi;
                    r9 = ldm(m, &stk, ea, 8);
                }
                pc = 0x1aef2; continue;
            }
            0x1aeee => {
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                pc = 0x1aef2; continue;
            }
            0x1aef2 => {
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (r9) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x18u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
