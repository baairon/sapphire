#![allow(unused_assignments, unused_variables, unused_mut, unused_parens, unreachable_code, unused_imports, clippy::all)]
use crate::lift_rt::*;
use crate::Mem;

/// lifted from the machine code of @0x2ceb8 (size=118); checked against the P1 vectors
pub fn f_0002ceb8(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u64, p_3: u32, p_4: u32) {
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
    rsi = p_1 as u64;
    rdx = p_2 as u64;
    rcx = p_3 as u64;
    r8 = p_4 as u64;
    let mut pc: u64 = 0x2ceb8;
    loop {
        match pc {
            0x2ceb8 => {
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
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, (0x58u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 4, (0x8u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = ((r8 & 0xff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 8, (rdx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x30u64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x44u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x3cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x54u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x2cf2d => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x330d0 (size=126); checked against the P1 vectors
pub fn f_000330d0(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32, p_3: u32) -> u64 {
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
    rsi = p_1 as u64;
    rdx = p_2 as u64;
    rcx = p_3 as u64;
    let mut pc: u64 = 0x330d0;
    loop {
        match pc {
            0x330d0 => {
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
                    r9 = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if cf || zf { pc = 0x3314b; continue; }
                pc = 0x330de; continue;
            }
            0x330de => {
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    r10 = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    r8 = ((rcx & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if cf || zf { pc = 0x3314b; continue; }
                pc = 0x330ee; continue;
            }
            0x330ee => {
                {
                    let a = (rcx & 0xffff);
                    let b = (rcx & 0xffff);
                    let r = (a & b) & 0xffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x3313f; continue; }
                pc = 0x330f3; continue;
            }
            0x330f3 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffff);
                    let b = (r10 & 0xffff);
                    let r = (a & b) & 0xffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x33102; continue; }
                pc = 0x330fb; continue;
            }
            0x330fb => {
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff) as i32 as i128;
                    let b = (r9 & 0xffffffff) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x33102; continue;
            }
            0x33102 => {
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x33137; continue; }
                pc = 0x33108; continue;
            }
            0x33108 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    r10 = ((r10 & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x3311b; continue; }
                pc = 0x33114; continue;
            }
            0x33114 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff) as i32 as i128;
                    let b = (r9 & 0xffffffff) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x3311b; continue;
            }
            0x3311b => {
                {
                    let a = rax;
                    let b = rcx;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x3313c; continue; }
                pc = 0x33124; continue;
            }
            0x33124 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x33134; continue; }
                pc = 0x3312c; continue;
            }
            0x3312c => {
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    let a = (r9 & 0xffffffff) as i32 as i128;
                    let b = ldm(m, &stk, ea, 4) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = r9;
                }
                pc = 0x33134; continue;
            }
            0x33134 => {
                {
                    let a = rcx;
                    let b = rax;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                pc = 0x33137; continue;
            }
            0x33137 => {
                {
                    let a = rsi;
                    let b = rcx;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                pc = 0x3313f; continue;
            }
            0x3313c => {
                {
                    let a = rsi;
                    let b = rax;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                pc = 0x3313f; continue;
            }
            0x3313f => {
                {
                    let ea: u64 = rdi.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x24u64);
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = ldm(m, &stk, ea, 4) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = rdx;
                    let b = rsi;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdx = r;
                }
                {
                    rax = rdx;
                }
                pc = 0x3314b; continue;
            }
            0x3314b => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            0x3314d => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xc8b1a (size=131); checked against the P1 vectors
pub fn f_000c8b1a(m: &mut dyn Mem, p_0: u64, p_1: u64) {
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
    let mut pc: u64 = 0xc8b1a;
    loop {
        match pc {
            0xc8b1a => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2u64);
                    r9 = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff) as i32 as i128;
                    let b = 11914 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x1u64);
                    rdx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = 40095 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi;
                    rdi = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff) as i32 as i128;
                    let b = 4050 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 14u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x40u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 2, ((rax & 0xffff)) & 0xffffu64);
                }
                {
                    let a = (r9 & 0xffffffff) as i32 as i128;
                    let b = -6388 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = -21595 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff) as i32 as i128;
                    let b = 28073 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 14u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x200u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 2, ((rax & 0xffff)) & 0xffffu64);
                }
                {
                    let a = (r9 & 0xffffffff) as i32 as i128;
                    let b = 28017 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = -25349 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff) as i32 as i128;
                    let b = -2566 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 14u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x200u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 2, ((rax & 0xffff)) & 0xffffu64);
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

/// lifted from the machine code of @0x2a154 (size=132); checked against the P1 vectors
pub fn f_0002a154(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0x2a154;
    loop {
        match pc {
            0x2a154 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rsi;
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x30u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x14u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x44u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x24u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x54u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x58u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x38u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x5cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x3cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x64u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = 0x8u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let c = cf as u64;
                    let r = a.wrapping_sub(b).wrapping_sub(c) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x40u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x60u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x44u64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 2u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x48u64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 3u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4cu64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 4u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x50u64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 5u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x54u64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 6u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x58u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x2a1d7 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x2a1d8 (size=134); checked against the P1 vectors
pub fn f_0002a1d8(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0x2a1d8;
    loop {
        match pc {
            0x2a1d8 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rsi;
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x30u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x14u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x24u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x44u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x34u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x54u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x38u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x58u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x3cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x5cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x64u64);
                    stm(m, &mut stk, ea, 4, (0x8u64) & 0xffffffffu64);
                }
                {
                    rax = (0x1u64) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x44u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2a256; continue; }
                pc = 0x2a21d; continue;
            }
            0x2a21d => {
                {
                    rax = (0x4u64) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x48u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2a256; continue; }
                pc = 0x2a228; continue;
            }
            0x2a228 => {
                {
                    rax = (0x8u64) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4cu64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2a256; continue; }
                pc = 0x2a233; continue;
            }
            0x2a233 => {
                {
                    rax = (0x10u64) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x50u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2a256; continue; }
                pc = 0x2a23e; continue;
            }
            0x2a23e => {
                {
                    rax = (0x20u64) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x54u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2a256; continue; }
                pc = 0x2a249; continue;
            }
            0x2a249 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x58u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                {
                    rax = (rax & !0xffu64) | (((!zf) as u64) & 0xff);
                }
                {
                    rax = ((rax & 0xff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 6u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x2a256; continue;
            }
            0x2a256 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x60u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x2a25d => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xcaeb8 (size=136); checked against the P1 vectors
pub fn f_000caeb8(m: &mut dyn Mem, p_0: u32, p_1: u32) -> u8 {
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
    let mut pc: u64 = 0xcaeb8;
    loop {
        match pc {
            0xcaeb8 => {
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
                    let b = 0x103522ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcaede; continue; }
                pc = 0xcaec4; continue;
            }
            0xcaec4 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x102802ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcaeff; continue; }
                pc = 0xcaecc; continue;
            }
            0xcaecc => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10266400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcaef6; continue; }
                pc = 0xcaed4; continue;
            }
            0xcaed4 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10266401u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcaf3c; continue; }
                pc = 0xcaedc; continue;
            }
            0xcaedc => {
                pc = 0xcaef6; continue;
            }
            0xcaede => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1047834fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcaf1b; continue; }
                pc = 0xcaee6; continue;
            }
            0xcaee6 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10352300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcaf07; continue; }
                pc = 0xcaeee; continue;
            }
            0xcaeee => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10378800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcaf3c; continue; }
                pc = 0xcaef6; continue;
            }
            0xcaef6 => {
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x1bu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xcaf3c; continue; }
                pc = 0xcaefd; continue;
            }
            0xcaefd => {
                pc = 0xcaf3e; continue;
            }
            0xcaeff => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10280300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcaf2d; continue; }
                pc = 0xcaf07; continue;
            }
            0xcaf07 => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0xdu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xcaf3c; continue; }
                pc = 0xcaf0c; continue;
            }
            0xcaf0c => {
                {
                    rax = (0x100fu64) & 0xffff_ffffu64;
                }
                {
                    rcx = (rcx & !0xffu64) | (((rsi & 0xff)) & 0xff);
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = (rcx & 31u64);
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xcaf3e; continue;
            }
            0xcaf1b => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478350u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcaf35; continue; }
                pc = 0xcaf23; continue;
            }
            0xcaf23 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518450u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcaf3c; continue; }
                pc = 0xcaf2b; continue;
            }
            0xcaf2b => {
                pc = 0xcaf35; continue;
            }
            0xcaf2d => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294700u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcaf3c; continue; }
                pc = 0xcaf35; continue;
            }
            0xcaf35 => {
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x1eu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcaf3e; continue; }
                pc = 0xcaf3c; continue;
            }
            0xcaf3c => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xcaf3e; continue;
            }
            0xcaf3e => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xcbb7b (size=136); checked against the P1 vectors
pub fn f_000cbb7b(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0xcbb7b;
    loop {
        match pc {
            0xcbb7b => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x106344ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcbba3; continue; }
                pc = 0xcbb89; continue;
            }
            0xcbb89 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105183ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcbbc0; continue; }
                pc = 0xcbb91; continue;
            }
            0xcbb91 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbc01; continue; }
                pc = 0xcbb99; continue;
            }
            0xcbb99 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478350u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcbbff; continue; }
                pc = 0xcbba1; continue;
            }
            0xcbba1 => {
                pc = 0xcbc01; continue;
            }
            0xcbba3 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x107107ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcbbda; continue; }
                pc = 0xcbbab; continue;
            }
            0xcbbab => {
                {
                    rcx = (rdi.wrapping_sub(0x10646700u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x7u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcbc01; continue; }
                pc = 0xcbbb6; continue;
            }
            0xcbbb6 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10634500u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcbbff; continue; }
                pc = 0xcbbbe; continue;
            }
            0xcbbbe => {
                pc = 0xcbc01; continue;
            }
            0xcbbc0 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105381ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcbbef; continue; }
                pc = 0xcbbc8; continue;
            }
            0xcbbc8 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbc01; continue; }
                pc = 0xcbbd0; continue;
            }
            0xcbbd0 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518450u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcbbff; continue; }
                pc = 0xcbbd8; continue;
            }
            0xcbbd8 => {
                pc = 0xcbc01; continue;
            }
            0xcbbda => {
                {
                    rcx = (rdi.wrapping_sub(0x10710850u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcbc01; continue; }
                pc = 0xcbbe5; continue;
            }
            0xcbbe5 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10710800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbc01; continue; }
                pc = 0xcbbed; continue;
            }
            0xcbbed => {
                pc = 0xcbbff; continue;
            }
            0xcbbef => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10538200u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbc01; continue; }
                pc = 0xcbbf7; continue;
            }
            0xcbbf7 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10565400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbc01; continue; }
                pc = 0xcbbff; continue;
            }
            0xcbbff => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xcbc01; continue;
            }
            0xcbc01 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x2a034 (size=144); checked against the P1 vectors
pub fn f_0002a034(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    rsi = p_1 as u64;
    let mut pc: u64 = 0x2a034;
    loop {
        match pc {
            0x2a034 => {
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
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, (0x70u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 4, (0x8u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x5cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x54u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x44u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x3cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x64u64);
                    stm(m, &mut stk, ea, 8, (0x8u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x6cu64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x2a0c3 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x2a0c4 (size=144); checked against the P1 vectors
pub fn f_0002a0c4(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    rsi = p_1 as u64;
    let mut pc: u64 = 0x2a0c4;
    loop {
        match pc {
            0x2a0c4 => {
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
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, (0x70u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 4, (0x8u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x5cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x54u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x44u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x3cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x64u64);
                    stm(m, &mut stk, ea, 8, (0x8u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x6cu64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x2a153 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xea210 (size=146); checked against the P1 vectors
pub fn f_000ea210(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0xea210;
    loop {
        match pc {
            0xea210 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x18cu64);
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x10u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xea29c; continue; }
                pc = 0xea21d; continue;
            }
            0xea21d => {
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
                    let ea: u64 = rdi.wrapping_add(0x60u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = 0x9u64;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xea25f; continue; }
                pc = 0xea23f; continue;
            }
            0xea23f => {
                {
                    let ea: u64 = rdi.wrapping_add(0x192u64);
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xea279; continue; }
                pc = 0xea248; continue;
            }
            0xea248 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x40u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    rax = rax.wrapping_add(rcx).wrapping_sub(0x1u64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x30u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x38u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0xea292; continue;
            }
            0xea25f => {
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
                pc = 0xea292; continue;
            }
            0xea279 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x68u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x70u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    rcx = rcx.wrapping_add(rax).wrapping_sub(0x1u64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x30u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x38u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                pc = 0xea292; continue;
            }
            0xea292 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x18cu64);
                    stm(m, &mut stk, ea, 4, (0x10u64) & 0xffffffffu64);
                }
                pc = 0xea29c; continue;
            }
            0xea29c => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0xea29e => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xe5efc (size=150); checked against the P1 vectors
pub fn f_000e5efc(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32, p_3: u32) {
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
    rsi = p_1 as u64;
    rdx = p_2 as u64;
    rcx = p_3 as u64;
    let mut pc: u64 = 0xe5efc;
    loop {
        match pc {
            0xe5efc => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    r8 = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0xfbffffffu64;
                    let r = (a & b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4000000u64;
                    let r = (a | b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xff);
                    let b = (rdx & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                {
                    let v = if zf { (r8 & 0xffffffff) } else { (rax & 0xffffffff) };
                    rax = (v) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0xfffffffbu64;
                    let r = (a & b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe5f5a; continue; }
                pc = 0xe5f2c; continue;
            }
            0xe5f2c => {
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    r8 = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0xf7ffffffu64;
                    let r = (a & b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0x8000000u64;
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xff);
                    let b = (rsi & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                {
                    let v = if zf { (r8 & 0xffffffff) } else { (rdx & 0xffffffff) };
                    rdx = (v) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (rcx & 0xff);
                    let b = (rcx & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe5f7e; continue; }
                pc = 0xe5f53; continue;
            }
            0xe5f53 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x800u64;
                    let r = (a | b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5f8a; continue;
            }
            0xe5f5a => {
                {
                    rdx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0xfffff7ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x800u64;
                    let r = (a | b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xff);
                    let b = (rsi & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                {
                    let v = if zf { (rdx & 0xffffffff) } else { (rax & 0xffffffff) };
                    rax = (v) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (rcx & 0xff);
                    let b = (rcx & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe5f85; continue; }
                pc = 0xe5f77; continue;
            }
            0xe5f77 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x8000000u64;
                    let r = (a | b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5f8a; continue;
            }
            0xe5f7e => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xfffff7ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5f8a; continue;
            }
            0xe5f85 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xf7ffffffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5f8a; continue;
            }
            0xe5f8a => {
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
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

/// lifted from the machine code of @0x51636 (size=154); checked against the P1 vectors
pub fn f_00051636(m: &mut dyn Mem, p_0: u64, p_1: u64) {
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
    rsi = p_0;
    rdx = p_1;
    let mut pc: u64 = 0x51636;
    loop {
        match pc {
            0x51636 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rsi;
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0xcu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x14u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x1cu64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x1cu64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x2cu64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x2cu64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x3cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x3cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x40u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x40u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x44u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x44u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x48u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x48u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x58u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x58u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x60u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x60u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x70u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x70u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x80u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x80u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x84u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x84u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x88u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x88u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8cu64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x8cu64);
                    st128m(m, &mut stk, ea, xmm0);
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

/// lifted from the machine code of @0x52332 (size=154); checked against the P1 vectors
pub fn f_00052332(m: &mut dyn Mem, p_0: u64, p_1: u64) {
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
    rsi = p_0;
    rdx = p_1;
    let mut pc: u64 = 0x52332;
    loop {
        match pc {
            0x52332 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rsi;
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0xcu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x14u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x1cu64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x1cu64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x2cu64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x2cu64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x3cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x3cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x40u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x40u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x44u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x44u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x48u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x48u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x58u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x58u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x60u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x60u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x70u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x70u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x80u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x80u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x84u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x84u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x88u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x88u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8cu64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x8cu64);
                    st128m(m, &mut stk, ea, xmm0);
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

/// lifted from the machine code of @0x299ea (size=170); checked against the P1 vectors
pub fn f_000299ea(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0x299ea;
    loop {
        match pc {
            0x299ea => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x18u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x44u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x48u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x2cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x30u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x78u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x34u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x80u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x3cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4cu64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x94u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x5cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x98u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x60u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x9cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x64u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xa0u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x68u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xa4u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x6cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xa8u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x70u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xb4u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xb8u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = rcx;
                    let c = 32u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = rcx;
                    let b = rax;
                    let r = (a | b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xbcu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x40u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x48u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xcbf2b (size=171); checked against the P1 vectors
pub fn f_000cbf2b(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0xcbf2b;
    loop {
        match pc {
            0xcbf2b => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1032294fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcbf5e; continue; }
                pc = 0xcbf39; continue;
            }
            0xcbf39 => {
                {
                    rcx = (rdi.wrapping_sub(0x10266400u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcbfd4; continue; }
                pc = 0xcbf48; continue;
            }
            0xcbf48 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294700u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbfd4; continue; }
                pc = 0xcbf54; continue;
            }
            0xcbf54 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294900u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcbfd2; continue; }
                pc = 0xcbf5c; continue;
            }
            0xcbf5c => {
                pc = 0xcbfd4; continue;
            }
            0xcbf5e => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x106466ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcbf88; continue; }
                pc = 0xcbf66; continue;
            }
            0xcbf66 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x104782ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcbfa8; continue; }
                pc = 0xcbf6e; continue;
            }
            0xcbf6e => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10322950u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbfd4; continue; }
                pc = 0xcbf76; continue;
            }
            0xcbf76 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10378800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbfd4; continue; }
                pc = 0xcbf7e; continue;
            }
            0xcbf7e => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10402100u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcbfd2; continue; }
                pc = 0xcbf86; continue;
            }
            0xcbf86 => {
                pc = 0xcbfd4; continue;
            }
            0xcbf88 => {
                {
                    rcx = (rdi.wrapping_sub(0x10646700u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x7u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcbfd4; continue; }
                pc = 0xcbf93; continue;
            }
            0xcbf93 => {
                {
                    rcx = (rdi.wrapping_sub(0x10710850u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcbfd4; continue; }
                pc = 0xcbf9e; continue;
            }
            0xcbf9e => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10710800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbfd4; continue; }
                pc = 0xcbfa6; continue;
            }
            0xcbfa6 => {
                pc = 0xcbfd2; continue;
            }
            0xcbfa8 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105183ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcbfc2; continue; }
                pc = 0xcbfb0; continue;
            }
            0xcbfb0 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbfd4; continue; }
                pc = 0xcbfb8; continue;
            }
            0xcbfb8 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478350u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcbfd2; continue; }
                pc = 0xcbfc0; continue;
            }
            0xcbfc0 => {
                pc = 0xcbfd4; continue;
            }
            0xcbfc2 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbfd4; continue; }
                pc = 0xcbfca; continue;
            }
            0xcbfca => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518450u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcbfd4; continue; }
                pc = 0xcbfd2; continue;
            }
            0xcbfd2 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xcbfd4; continue;
            }
            0xcbfd4 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x29928 (size=194); checked against the P1 vectors
pub fn f_00029928(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0x29928;
    loop {
        match pc {
            0x29928 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x20u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0xcu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x10u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x18u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x28u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x44u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x2cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x48u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x30u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x34u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x78u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x3cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x80u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4cu64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x5cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x94u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x60u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x98u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x64u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x9cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x68u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xa0u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x6cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xa4u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x70u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xa8u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x40u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x44u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xb4u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xb8u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x50u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x299e5; continue; }
                pc = 0x299cc; continue;
            }
            0x299cc => {
                {
                    let ea: u64 = rdi.wrapping_add(0x58u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = 0xcu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x299e5; continue; }
                pc = 0x299d2; continue;
            }
            0x299d2 => {
                {
                    let a = rdi;
                    let b = 0xb4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rax;
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                pc = 0x299e5; continue;
            }
            0x299e5 => {
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x299e9 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x58853 (size=214); checked against the P1 vectors
pub fn f_00058853(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x58853;
    loop {
        match pc {
            0x58853 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x58927; continue; }
                pc = 0x5885f; continue;
            }
            0x5885f => {
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    r11 = (0x5u64) & 0xffff_ffffu64;
                }
                pc = 0x58868; continue;
            }
            0x58868 => {
                {
                    rax = (r11.wrapping_sub(0x5u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    r9 = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_sub(0x4u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    r10 = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_sub(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 20u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = ((r8 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (r11.wrapping_sub(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rcx = (r11.wrapping_sub(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    rcx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = ((r11 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 20u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = (r8.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (r11.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rcx = (r11.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    rcx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 20u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = (r8.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (r11.wrapping_add(0x4u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rcx = (r11.wrapping_add(0x5u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    rcx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_add(0x6u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 20u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = (r8.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_add(0xcu64)) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff);
                    let b = 0x7u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r11 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                {
                    r11 = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                if cf { pc = 0x58868; continue; }
                pc = 0x58927; continue;
            }
            0x58927 => {
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

/// lifted from the machine code of @0x59033 (size=214); checked against the P1 vectors
pub fn f_00059033(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x59033;
    loop {
        match pc {
            0x59033 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x59107; continue; }
                pc = 0x5903f; continue;
            }
            0x5903f => {
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    r11 = (0x5u64) & 0xffff_ffffu64;
                }
                pc = 0x59048; continue;
            }
            0x59048 => {
                {
                    rax = (r11.wrapping_sub(0x5u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    r9 = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_sub(0x4u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    r10 = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_sub(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 20u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = ((r8 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (r11.wrapping_sub(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rcx = (r11.wrapping_sub(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    rcx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = ((r11 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 20u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = (r8.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (r11.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rcx = (r11.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    rcx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 20u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = (r8.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (r11.wrapping_add(0x4u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rcx = (r11.wrapping_add(0x5u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    rcx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_add(0x6u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 20u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = (r8.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r11.wrapping_add(0xcu64)) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff);
                    let b = 0x7u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r11 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                {
                    r11 = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                if cf { pc = 0x59048; continue; }
                pc = 0x59107; continue;
            }
            0x59107 => {
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

/// lifted from the machine code of @0xe6a78 (size=216); checked against the P1 vectors
pub fn f_000e6a78(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32, p_3: u32) {
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
    rsi = p_0;
    rdx = p_1 as u64;
    rcx = p_2 as u64;
    r8 = p_3 as u64;
    let mut pc: u64 = 0xe6a78;
    loop {
        match pc {
            0xe6a78 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rsi;
                    r9 = ldm(m, &stk, ea, 8);
                }
                {
                    rdx = (((rdx & 0xffffffff)) as i32 as i64 as u64);
                }
                {
                    let a = rdx as i64 as i128;
                    let b = 1431655766 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    rax = rdi;
                }
                {
                    let a = rax;
                    let c = 63u64;
                    let r = a >> c;
                    rax = r;
                }
                {
                    let a = rdi;
                    let c = 32u64;
                    let r = a >> c;
                    rdi = r;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (rdi.wrapping_add(rdi.wrapping_mul(2))) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe6ae1; continue; }
                pc = 0xe6aa0; continue;
            }
            0xe6aa0 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0x1u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe6b1a; continue; }
                pc = 0xe6aa5; continue;
            }
            0xe6aa5 => {
                {
                    rax = ((rcx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 4u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = r9.wrapping_add(0x2u64);
                    rdx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0xfu64;
                    let r = (a & b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = r9.wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 4u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xff);
                    let b = 0x3fu64;
                    let r = (a & b) & 0xffu64;
                    rcx = (rcx & !0xffu64) | ((r) & 0xff);
                }
                {
                    let ea: u64 = r9.wrapping_add(0x3u64);
                    stm(m, &mut stk, ea, 1, ((rcx & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = r9.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 1, ((r8 & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = r9.wrapping_add(0x5u64);
                    rax = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xfcu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let c = 8u64;
                    let r = a >> c;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0x3u64;
                    let r = (a & b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = r9.wrapping_add(0x5u64);
                    stm(m, &mut stk, ea, 1, ((r8 & 0xff)) & 0xffu64);
                }
                pc = 0xe6b14; continue;
            }
            0xe6ae1 => {
                {
                    rax = (rcx.wrapping_mul(4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = r9.wrapping_add(0x1u64);
                    rdx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0x3u64;
                    let r = (a & b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = r9.wrapping_add(0x1u64);
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
                }
                {
                    rax = ((r8 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 4u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 6u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0xfu64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = r9.wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((rcx & 0xff)) & 0xffu64);
                }
                {
                    let a = (r8 & 0xffffffff);
                    let c = 4u64;
                    let r = a >> c;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xff);
                    let b = 0x3fu64;
                    let r = (a & b) & 0xffu64;
                    r8 = (r8 & !0xffu64) | ((r) & 0xff);
                }
                {
                    let ea: u64 = r9.wrapping_add(0x3u64);
                    stm(m, &mut stk, ea, 1, ((r8 & 0xff)) & 0xffu64);
                }
                pc = 0xe6b14; continue;
            }
            0xe6b14 => {
                {
                    let ea: u64 = rsi;
                    let a = ldm(m, &stk, ea, 8);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    stm(m, &mut stk, ea, 8, (r) & 0xffffffffffffffffu64);
                }
                pc = 0xe6b4e; continue;
            }
            0xe6b1a => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe6b4e; continue; }
                pc = 0xe6b1e; continue;
            }
            0xe6b1e => {
                {
                    let ea: u64 = r9;
                    stm(m, &mut stk, ea, 1, ((rcx & 0xff)) & 0xffu64);
                }
                {
                    rax = (r8.wrapping_mul(4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 8u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x3u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = r9.wrapping_add(0x1u64);
                    stm(m, &mut stk, ea, 1, ((rcx & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = r9.wrapping_add(0x2u64);
                    rax = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xf0u64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let c = 6u64;
                    let r = a >> c;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0xfu64;
                    let r = (a & b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = r9.wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((r8 & 0xff)) & 0xffu64);
                }
                pc = 0xe6b4e; continue;
            }
            0xe6b4e => {
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

/// lifted from the machine code of @0x657a6 (size=235); checked against the P1 vectors
pub fn f_000657a6(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) -> u8 {
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
    let mut pc: u64 = 0x657a6;
    loop {
        match pc {
            0x657a6 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let v = rbx;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = rdi;
                    let b = rdi;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x6588e; continue; }
                pc = 0x657b6; continue;
            }
            0x657b6 => {
                {
                    let a = rsi;
                    let b = rsi;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x6588e; continue; }
                pc = 0x657bf; continue;
            }
            0x657bf => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x6588e; continue; }
                pc = 0x657c7; continue;
            }
            0x657c7 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                if zf { pc = 0x6588e; continue; }
                pc = 0x657d1; continue;
            }
            0x657d1 => {
                {
                    let a = (r9 & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                pc = 0x657d7; continue;
            }
            0x657d7 => {
                {
                    r11 = ((r9 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11);
                    r10 = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0x1u64);
                    rcx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 12u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0x2u64);
                    rbx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let c = 22u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    r10 = ((r8 & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(r10.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rbx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0x3u64);
                    rcx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0x4u64);
                    rbx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let c = 12u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0x5u64);
                    rcx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 22u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rbx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rbx = ((r10 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a | b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rbx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0x6u64);
                    rcx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0x7u64);
                    rbx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let c = 12u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0x8u64);
                    rcx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 22u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rbx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rbx = ((r10 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let b = 0x2u64;
                    let r = (a | b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rbx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0x9u64);
                    rcx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0xau64);
                    rbx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let c = 12u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rbx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rbx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r11).wrapping_add(0xbu64);
                    rcx = (ldm(m, &stk, ea, 1)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 22u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rbx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = 0x3u64;
                    let r = (a | b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(r10.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    r8 = (r8.wrapping_add(0x4u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = 0xcu64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    r9 = ((r9 & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x657d7; continue; }
                pc = 0x6588e; continue;
            }
            0x6588e => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbx = v;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x5a478 (size=284); checked against the P1 vectors
pub fn f_0005a478(m: &mut dyn Mem, p_0: u32, p_1: u64, p_2: u64) -> u8 {
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
    rsi = p_1;
    rdx = p_2;
    let mut pc: u64 = 0x5a478;
    loop {
        match pc {
            0x5a478 => {
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
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x11u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if cf || zf { pc = 0x5a496; continue; }
                pc = 0x5a483; continue;
            }
            0x5a483 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5a54a; continue;
            }
            0x5a496 => {
                {
                    rax = 0x1005a54cu64;
                }
                {
                    let ea: u64 = rax.wrapping_add(rdi.wrapping_mul(4));
                    rcx = ((ldm(m, &stk, ea, 4)) as i32 as i64 as u64);
                }
                {
                    let a = rcx;
                    let b = rax;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                pc = rcx.wrapping_sub(0x1000_0000u64); continue;
            }
            0x5a4a6 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x3cu64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a4b1 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0xea60u64) & 0xffffffffu64);
                }
                pc = 0x5a542; continue;
            }
            0x5a4bc => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x1eu64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a4c4 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x7530u64) & 0xffffffffu64);
                }
                pc = 0x5a542; continue;
            }
            0x5a4cc => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x19u64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a4d4 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x18u64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a4dc => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x5dc0u64) & 0xffffffffu64);
                }
                pc = 0x5a542; continue;
            }
            0x5a4e4 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x32u64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a4ec => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x30u64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a4f4 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0xbb80u64) & 0xffffffffu64);
                }
                pc = 0x5a542; continue;
            }
            0x5a4fc => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x78u64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a504 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x1d4c0u64) & 0xffffffffu64);
                }
                pc = 0x5a542; continue;
            }
            0x5a50c => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0xfu64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a514 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x3a98u64) & 0xffffffffu64);
                }
                pc = 0x5a542; continue;
            }
            0x5a51c => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x13u64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a524 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x4a38u64) & 0xffffffffu64);
                }
                pc = 0x5a542; continue;
            }
            0x5a52c => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x12u64) & 0xffffffffu64);
                }
                pc = 0x5a532; continue;
            }
            0x5a532 => {
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, (0x1u64) & 0xffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0x5a54a; continue;
            }
            0x5a53c => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x4650u64) & 0xffffffffu64);
                }
                pc = 0x5a542; continue;
            }
            0x5a542 => {
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, (0x3e9u64) & 0xffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0x5a54a; continue;
            }
            0x5a54a => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xdf5c4 (size=288); checked against the P1 vectors
pub fn f_000df5c4(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64, p_3: u64) {
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
    rdx = p_2;
    rcx = p_3;
    let mut pc: u64 = 0xdf5c4;
    loop {
        match pc {
            0xdf5c4 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x274u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = rax;
                    let b = 0x20u64;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0xdf65e; continue; }
                pc = 0xdf5d8; continue;
            }
            0xdf5d8 => {
                {
                    rdi = 0x100df660u64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(4));
                    rax = ((ldm(m, &stk, ea, 4)) as i32 as i64 as u64);
                }
                {
                    let a = rax;
                    let b = rdi;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                pc = rax.wrapping_sub(0x1000_0000u64); continue;
            }
            0xdf5e8 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rcx;
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                pc = 0xdf65e; continue;
            }
            0xdf5fc => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0xf00u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, (0xf00u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rcx;
                    stm(m, &mut stk, ea, 4, (0x870u64) & 0xffffffffu64);
                }
                pc = 0xdf65e; continue;
            }
            0xdf610 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0xf00u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, (0x780u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rcx;
                    stm(m, &mut stk, ea, 4, (0x870u64) & 0xffffffffu64);
                }
                pc = 0xdf65e; continue;
            }
            0xdf624 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0xf00u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, (0xb40u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rcx;
                    stm(m, &mut stk, ea, 4, (0x870u64) & 0xffffffffu64);
                }
                pc = 0xdf65e; continue;
            }
            0xdf638 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0xf00u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, (0xa00u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rcx;
                    stm(m, &mut stk, ea, 4, (0x870u64) & 0xffffffffu64);
                }
                pc = 0xdf65e; continue;
            }
            0xdf64c => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0xf00u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 4, (0x1680u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rcx;
                    stm(m, &mut stk, ea, 4, (0x870u64) & 0xffffffffu64);
                }
                pc = 0xdf65e; continue;
            }
            0xdf65e => {
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

/// lifted from the machine code of @0x591eb (size=321); checked against the P1 vectors
pub fn f_000591eb(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32) {
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
    rsi = p_1 as u64;
    rdx = p_2 as u64;
    let mut pc: u64 = 0x591eb;
    loop {
        match pc {
            0x591eb => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x6u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5930d; continue; }
                pc = 0x591f8; continue;
            }
            0x591f8 => {
                {
                    rax = 0x10059310u64;
                }
                {
                    let ea: u64 = rax.wrapping_add(rsi.wrapping_mul(4));
                    rcx = ((ldm(m, &stk, ea, 4)) as i32 as i64 as u64);
                }
                {
                    let a = rcx;
                    let b = rax;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                pc = rcx.wrapping_sub(0x1000_0000u64); continue;
            }
            0x59208 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x5930d; continue; }
                pc = 0x59210; continue;
            }
            0x59210 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x59212; continue;
            }
            0x59212 => {
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x200u64) & 0xffffu64);
                }
                {
                    rcx = (rax.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x40u64) & 0xffffu64);
                }
                {
                    rcx = (rax.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x200u64) & 0xffffu64);
                }
                {
                    rcx = (rax.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x40u64) & 0xffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x59212; continue; }
                pc = 0x5923c; continue;
            }
            0x5923c => {
                pc = 0x5930d; continue;
            }
            0x59241 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x5930d; continue; }
                pc = 0x59249; continue;
            }
            0x59249 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5924b; continue;
            }
            0x5924b => {
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x200u64) & 0xffffu64);
                }
                {
                    rcx = (rax.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x200u64) & 0xffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x5924b; continue; }
                pc = 0x59263; continue;
            }
            0x59263 => {
                pc = 0x5930d; continue;
            }
            0x59268 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x5930d; continue; }
                pc = 0x59270; continue;
            }
            0x59270 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x59272; continue;
            }
            0x59272 => {
                {
                    rcx = (rax.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x40u64) & 0xffffu64);
                }
                {
                    rcx = (rax.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x200u64) & 0xffffu64);
                }
                {
                    rcx = (rax.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x40u64) & 0xffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x59272; continue; }
                pc = 0x59294; continue;
            }
            0x59294 => {
                pc = 0x5930d; continue;
            }
            0x59296 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x5930d; continue; }
                pc = 0x5929a; continue;
            }
            0x5929a => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5929c; continue;
            }
            0x5929c => {
                {
                    rcx = (rax.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x200u64) & 0xffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x5929c; continue; }
                pc = 0x592ac; continue;
            }
            0x592ac => {
                pc = 0x5930d; continue;
            }
            0x592ae => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x5930d; continue; }
                pc = 0x592b2; continue;
            }
            0x592b2 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x592b4; continue;
            }
            0x592b4 => {
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x200u64) & 0xffffu64);
                }
                {
                    rcx = (rax.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x40u64) & 0xffffu64);
                }
                {
                    rcx = (rax.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x40u64) & 0xffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x592b4; continue; }
                pc = 0x592d5; continue;
            }
            0x592d5 => {
                pc = 0x5930d; continue;
            }
            0x592d7 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x5930d; continue; }
                pc = 0x592db; continue;
            }
            0x592db => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x592dd; continue;
            }
            0x592dd => {
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x200u64) & 0xffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x592dd; continue; }
                pc = 0x592ec; continue;
            }
            0x592ec => {
                pc = 0x5930d; continue;
            }
            0x592ee => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x5930d; continue; }
                pc = 0x592f2; continue;
            }
            0x592f2 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x592f4; continue;
            }
            0x592f4 => {
                {
                    rcx = (rax.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x40u64) & 0xffffu64);
                }
                {
                    rcx = (rax.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, (0x40u64) & 0xffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x592f4; continue; }
                pc = 0x5930d; continue;
            }
            0x5930d => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x5930f => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xccd87 (size=322); checked against the P1 vectors
pub fn f_000ccd87(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0xccd87;
    loop {
        match pc {
            0xccd87 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1032294fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xccdbd; continue; }
                pc = 0xccd95; continue;
            }
            0xccd95 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x102802ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcce02; continue; }
                pc = 0xccd9d; continue;
            }
            0xccd9d => {
                {
                    rcx = (rdi.wrapping_sub(0x10266400u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xccec7; continue; }
                pc = 0xccdac; continue;
            }
            0xccdac => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10244800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xccec5; continue; }
                pc = 0xccdb8; continue;
            }
            0xccdb8 => {
                pc = 0xccec7; continue;
            }
            0xccdbd => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x106344ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcce27; continue; }
                pc = 0xccdc5; continue;
            }
            0xccdc5 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105183ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcce48; continue; }
                pc = 0xccdcd; continue;
            }
            0xccdcd => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x104020ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcce89; continue; }
                pc = 0xccdd9; continue;
            }
            0xccdd9 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10322950u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xccde5; continue;
            }
            0xccde5 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10352300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xccdf1; continue;
            }
            0xccdf1 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10378800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xccec5; continue; }
                pc = 0xccdfd; continue;
            }
            0xccdfd => {
                pc = 0xccec7; continue;
            }
            0xcce02 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x102946ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcce62; continue; }
                pc = 0xcce0a; continue;
            }
            0xcce0a => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10280300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xcce16; continue;
            }
            0xcce16 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10293000u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xccec5; continue; }
                pc = 0xcce22; continue;
            }
            0xcce22 => {
                pc = 0xccec7; continue;
            }
            0xcce27 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x107107ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcce74; continue; }
                pc = 0xcce2f; continue;
            }
            0xcce2f => {
                {
                    rcx = (rdi.wrapping_sub(0x10646700u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x7u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xccec7; continue; }
                pc = 0xcce3e; continue;
            }
            0xcce3e => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10634500u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xccec5; continue; }
                pc = 0xcce46; continue;
            }
            0xcce46 => {
                pc = 0xccec7; continue;
            }
            0xcce48 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105381ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xccea3; continue; }
                pc = 0xcce50; continue;
            }
            0xcce50 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xcce58; continue;
            }
            0xcce58 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518450u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xccec5; continue; }
                pc = 0xcce60; continue;
            }
            0xcce60 => {
                pc = 0xccec7; continue;
            }
            0xcce62 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294700u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xcce6a; continue;
            }
            0xcce6a => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294900u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xccec5; continue; }
                pc = 0xcce72; continue;
            }
            0xcce72 => {
                pc = 0xccec7; continue;
            }
            0xcce74 => {
                {
                    rcx = (rdi.wrapping_sub(0x10710850u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xccec7; continue; }
                pc = 0xcce7f; continue;
            }
            0xcce7f => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10710800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xcce87; continue;
            }
            0xcce87 => {
                pc = 0xccec5; continue;
            }
            0xcce89 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x104782ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcceb5; continue; }
                pc = 0xcce91; continue;
            }
            0xcce91 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10402100u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xcce99; continue;
            }
            0xcce99 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10416000u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xccec5; continue; }
                pc = 0xccea1; continue;
            }
            0xccea1 => {
                pc = 0xccec7; continue;
            }
            0xccea3 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10538200u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xcceab; continue;
            }
            0xcceab => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10565400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xccec5; continue; }
                pc = 0xcceb3; continue;
            }
            0xcceb3 => {
                pc = 0xccec7; continue;
            }
            0xcceb5 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xccebd; continue;
            }
            0xccebd => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478350u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xccec7; continue; }
                pc = 0xccec5; continue;
            }
            0xccec5 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xccec7; continue;
            }
            0xccec7 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xccec9 (size=341); checked against the P1 vectors
pub fn f_000ccec9(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0xccec9;
    loop {
        match pc {
            0xccec9 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1032294fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcceff; continue; }
                pc = 0xcced7; continue;
            }
            0xcced7 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x102802ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xccf48; continue; }
                pc = 0xccedf; continue;
            }
            0xccedf => {
                {
                    rcx = (rdi.wrapping_sub(0x10266400u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcd01c; continue; }
                pc = 0xcceee; continue;
            }
            0xcceee => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10244800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcd01a; continue; }
                pc = 0xccefa; continue;
            }
            0xccefa => {
                pc = 0xcd01c; continue;
            }
            0xcceff => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x106344ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xccf6d; continue; }
                pc = 0xccf07; continue;
            }
            0xccf07 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105183ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xccf95; continue; }
                pc = 0xccf13; continue;
            }
            0xccf13 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x104020ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xccfde; continue; }
                pc = 0xccf1f; continue;
            }
            0xccf1f => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10322950u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xccf2b; continue;
            }
            0xccf2b => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10352300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xccf37; continue;
            }
            0xccf37 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10378800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcd01a; continue; }
                pc = 0xccf43; continue;
            }
            0xccf43 => {
                pc = 0xcd01c; continue;
            }
            0xccf48 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x102946ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xccfaf; continue; }
                pc = 0xccf50; continue;
            }
            0xccf50 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10280300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xccf5c; continue;
            }
            0xccf5c => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10293000u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcd01a; continue; }
                pc = 0xccf68; continue;
            }
            0xccf68 => {
                pc = 0xcd01c; continue;
            }
            0xccf6d => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x106681ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xccfc1; continue; }
                pc = 0xccf75; continue;
            }
            0xccf75 => {
                {
                    rcx = (rdi.wrapping_sub(0x10646700u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x7u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcd01c; continue; }
                pc = 0xccf84; continue;
            }
            0xccf84 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10634500u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcd01a; continue; }
                pc = 0xccf90; continue;
            }
            0xccf90 => {
                pc = 0xcd01c; continue;
            }
            0xccf95 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105381ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xccff8; continue; }
                pc = 0xccf9d; continue;
            }
            0xccf9d => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xccfa5; continue;
            }
            0xccfa5 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518450u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcd01a; continue; }
                pc = 0xccfad; continue;
            }
            0xccfad => {
                pc = 0xcd01c; continue;
            }
            0xccfaf => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294700u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xccfb7; continue;
            }
            0xccfb7 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294900u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcd01a; continue; }
                pc = 0xccfbf; continue;
            }
            0xccfbf => {
                pc = 0xcd01c; continue;
            }
            0xccfc1 => {
                {
                    rcx = (rdi.wrapping_sub(0x10710850u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcd01c; continue; }
                pc = 0xccfcc; continue;
            }
            0xccfcc => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10668200u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xccfd4; continue;
            }
            0xccfd4 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10710800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xccfdc; continue;
            }
            0xccfdc => {
                pc = 0xcd01a; continue;
            }
            0xccfde => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x104782ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcd00a; continue; }
                pc = 0xccfe6; continue;
            }
            0xccfe6 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10402100u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xccfee; continue;
            }
            0xccfee => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10416000u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcd01a; continue; }
                pc = 0xccff6; continue;
            }
            0xccff6 => {
                pc = 0xcd01c; continue;
            }
            0xccff8 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10538200u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xcd000; continue;
            }
            0xcd000 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10565400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcd01a; continue; }
                pc = 0xcd008; continue;
            }
            0xcd008 => {
                pc = 0xcd01c; continue;
            }
            0xcd00a => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xcd012; continue;
            }
            0xcd012 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478350u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcd01c; continue; }
                pc = 0xcd01a; continue;
            }
            0xcd01a => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xcd01c; continue;
            }
            0xcd01c => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
