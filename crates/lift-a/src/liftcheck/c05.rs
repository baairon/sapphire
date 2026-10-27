#![allow(unused_assignments, unused_variables, unused_mut, unused_parens, unreachable_code, unused_imports, clippy::all)]
use crate::lift_rt::*;
use crate::Mem;

/// lifted from the machine code of @0x59192 (size=46); checked against the P1 vectors
pub fn f_00059192(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x59192;
    loop {
        match pc {
            0x59192 => {
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
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x591be; continue; }
                pc = 0x5919a; continue;
            }
            0x5919a => {
                {
                    let ea: u64 = rsi;
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
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
                    let b = 0x3f0u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 20u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xfu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 2, ((rax & 0xffff)) & 0xffffu64);
                }
                {
                    let a = rsi;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    let a = rdi;
                    let b = 0x2u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if !zf { pc = 0x5919a; continue; }
                pc = 0x591be; continue;
            }
            0x591be => {
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

/// lifted from the machine code of @0x5cc20 (size=48); checked against the P1 vectors
pub fn f_0005cc20(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0x5cc20;
    loop {
        match pc {
            0x5cc20 => {
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
                    let b = 0x6du64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0x5cc44; continue; }
                pc = 0x5cc2b; continue;
            }
            0x5cc2b => {
                {
                    rcx = ((rdi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x21u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5cc4c; continue; }
                pc = 0x5cc32; continue;
            }
            0x5cc32 => {
                {
                    rdx = 0x307e27f3eu64;
                }
                {
                    let a = rdx;
                    cf = (a >> (rcx & 63)) & 1 != 0;
                }
                if cf { pc = 0x5cc4e; continue; }
                pc = 0x5cc42; continue;
            }
            0x5cc42 => {
                pc = 0x5cc4c; continue;
            }
            0x5cc44 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xffffff92u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x5u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x5cc4e; continue; }
                pc = 0x5cc4c; continue;
            }
            0x5cc4c => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5cc4e; continue;
            }
            0x5cc4e => {
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

/// lifted from the machine code of @0xe5574 (size=48); checked against the P1 vectors
pub fn f_000e5574(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0xe5574;
    loop {
        match pc {
            0xe5574 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0xcu64);
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe55a0; continue; }
                pc = 0xe557e; continue;
            }
            0xe557e => {
                {
                    let ea: u64 = rdi.wrapping_add(0x50u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x50u64);
                    let a = (rax & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe55a0; continue; }
                pc = 0xe5586; continue;
            }
            0xe5586 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x54u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x54u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe55a0; continue; }
                pc = 0xe558e; continue;
            }
            0xe558e => {
                {
                    let ea: u64 = rsi.wrapping_add(0x58u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x58u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe55a0; continue; }
                pc = 0xe5596; continue;
            }
            0xe5596 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x5cu64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x5cu64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe55a2; continue; }
                pc = 0xe55a0; continue;
            }
            0xe55a0 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe55a2; continue;
            }
            0xe55a2 => {
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

/// lifted from the machine code of @0x59372 (size=49); checked against the P1 vectors
pub fn f_00059372(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32, p_3: u32, p_4: u32) {
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
    let mut pc: u64 = 0x59372;
    loop {
        match pc {
            0x59372 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let a = (r8 & 0xffff);
                    let b = (r8 & 0xffff);
                    let r = (a & b) & 0xffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x593a1; continue; }
                pc = 0x5937c; continue;
            }
            0x5937c => {
                {
                    r8 = ((r8 & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let a = r8;
                    let b = r8;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    r8 = r;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x59385; continue;
            }
            0x59385 => {
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((rdx & 0xffff)) & 0xffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 2, ((rsi & 0xffff)) & 0xffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 2, ((rcx & 0xffff)) & 0xffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x6u64);
                    stm(m, &mut stk, ea, 2, ((rsi & 0xffff)) & 0xffffu64);
                }
                {
                    let a = rax;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let a = rax;
                    let b = r8;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 63 & 1 != 0;
                    sf = (r >> 63) & 1 != 0;
                }
                if sf != ov { pc = 0x59385; continue; }
                pc = 0x593a1; continue;
            }
            0x593a1 => {
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

/// lifted from the machine code of @0x5cb4a (size=49); checked against the P1 vectors
pub fn f_0005cb4a(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0x5cb4a;
    loop {
        match pc {
            0x5cb4a => {
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
                    let b = 0x6du64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0x5cb6f; continue; }
                pc = 0x5cb55; continue;
            }
            0x5cb55 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xfffffff3u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x37u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5cb77; continue; }
                pc = 0x5cb5d; continue;
            }
            0x5cb5d => {
                {
                    rcx = 0xf800000000600fu64;
                }
                {
                    let a = rcx;
                    cf = (a >> (rdi & 63)) & 1 != 0;
                }
                if cf { pc = 0x5cb79; continue; }
                pc = 0x5cb6d; continue;
            }
            0x5cb6d => {
                pc = 0x5cb77; continue;
            }
            0x5cb6f => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xffffff92u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xcu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x5cb79; continue; }
                pc = 0x5cb77; continue;
            }
            0x5cb77 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5cb79; continue;
            }
            0x5cb79 => {
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

/// lifted from the machine code of @0x67628 (size=50); checked against the P1 vectors
pub fn f_00067628(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x67628;
    loop {
        match pc {
            0x67628 => {
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
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x67658; continue; }
                pc = 0x67630; continue;
            }
            0x67630 => {
                {
                    let a = rdi;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                pc = 0x67634; continue;
            }
            0x67634 => {
                {
                    let ea: u64 = rdi.wrapping_sub(0x4u64);
                    r8 = (r8 & !0xffffu64) | ((ldm(m, &stk, ea, 2)) & 0xffff);
                }
                {
                    let ea: u64 = rdi.wrapping_sub(0x2u64);
                    rcx = (rcx & !0xffffu64) | ((ldm(m, &stk, ea, 2)) & 0xffff);
                }
                {
                    let ea: u64 = rdi;
                    rax = (rax & !0xffffu64) | ((ldm(m, &stk, ea, 2)) & 0xffff);
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 2, ((rax & 0xffff)) & 0xffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 2, ((rcx & 0xffff)) & 0xffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 2, ((r8 & 0xffff)) & 0xffffu64);
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rsi;
                    let b = 0x6u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if !zf { pc = 0x67634; continue; }
                pc = 0x67658; continue;
            }
            0x67658 => {
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

/// lifted from the machine code of @0x6768e (size=50); checked against the P1 vectors
pub fn f_0006768e(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0x6768e;
    loop {
        match pc {
            0x6768e => {
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
                    let b = (rsi & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x676be; continue; }
                pc = 0x67696; continue;
            }
            0x67696 => {
                {
                    r8 = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x6769b; continue;
            }
            0x6769b => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(8));
                    rdx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let c = 20u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x2u64);
                    rsi = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rsi = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4u64);
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = rcx;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x6769b; continue; }
                pc = 0x676be; continue;
            }
            0x676be => {
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

/// lifted from the machine code of @0x675ba (size=52); checked against the P1 vectors
pub fn f_000675ba(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x675ba;
    loop {
        match pc {
            0x675ba => {
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
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x675ec; continue; }
                pc = 0x675c5; continue;
            }
            0x675c5 => {
                {
                    r8 = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x675ca; continue;
            }
            0x675ca => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    r9 = (r9 & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x1u64);
                    rax = (rax & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x2u64);
                    rdx = (rdx & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x1u64);
                    stm(m, &mut stk, ea, 1, ((rax & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((r9 & 0xff)) & 0xffu64);
                }
                {
                    let a = rcx;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = rsi;
                    let b = 0x3u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    let a = rcx;
                    let b = r8;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x675ca; continue; }
                pc = 0x675ec; continue;
            }
            0x675ec => {
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

/// lifted from the machine code of @0x6765a (size=52); checked against the P1 vectors
pub fn f_0006765a(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x6765a;
    loop {
        match pc {
            0x6765a => {
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
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x6768c; continue; }
                pc = 0x67665; continue;
            }
            0x67665 => {
                {
                    r8 = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x6766a; continue;
            }
            0x6766a => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    rdx = (rdx & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x1u64);
                    rax = (rax & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x2u64);
                    r9 = (r9 & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x1u64);
                    stm(m, &mut stk, ea, 1, ((rax & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((r9 & 0xff)) & 0xffu64);
                }
                {
                    let a = rcx;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = rsi;
                    let b = 0x3u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    let a = rcx;
                    let b = r8;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x6766a; continue; }
                pc = 0x6768c; continue;
            }
            0x6768c => {
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

/// lifted from the machine code of @0xc61e7 (size=52); checked against the P1 vectors
pub fn f_000c61e7(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32, p_3: u32, p_4: u32) {
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
    let mut pc: u64 = 0xc61e7;
    loop {
        match pc {
            0xc61e7 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    r8 = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0xc6219; continue; }
                pc = 0xc61f0; continue;
            }
            0xc61f0 => {
                {
                    let a = (r9 & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                pc = 0xc61f3; continue;
            }
            0xc61f3 => {
                {
                    rax = ((r9 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax);
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
                }
                {
                    rax = (r9.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax);
                    stm(m, &mut stk, ea, 1, ((rsi & 0xff)) & 0xffu64);
                }
                {
                    rax = (r9.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax);
                    stm(m, &mut stk, ea, 1, ((rcx & 0xff)) & 0xffu64);
                }
                {
                    rax = (r9.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax);
                    stm(m, &mut stk, ea, 1, ((rsi & 0xff)) & 0xffu64);
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xc61f3; continue; }
                pc = 0xc6219; continue;
            }
            0xc6219 => {
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

/// lifted from the machine code of @0x2612a (size=53); checked against the P1 vectors
pub fn f_0002612a(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32) {
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
    let mut pc: u64 = 0x2612a;
    loop {
        match pc {
            0x2612a => {
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
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
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
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
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

/// lifted from the machine code of @0x32e40 (size=54); checked against the P1 vectors
pub fn f_00032e40(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0x32e40;
    loop {
        match pc {
            0x32e40 => {
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
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi;
                    let a = (rax & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x32e71; continue; }
                pc = 0x32e4a; continue;
            }
            0x32e4a => {
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    let a = (rax & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x32e71; continue; }
                pc = 0x32e52; continue;
            }
            0x32e52 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x34u64);
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    rcx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x32e71; continue; }
                pc = 0x32e5e; continue;
            }
            0x32e5e => {
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0xcu64);
                    let a = (rax & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x32e71; continue; }
                pc = 0x32e66; continue;
            }
            0x32e66 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x24u64);
                    let a = (rax & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                {
                    rax = (rax & !0xffu64) | (((zf) as u64) & 0xff);
                }
                pc = 0x32e73; continue;
            }
            0x32e71 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x32e73; continue;
            }
            0x32e73 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x32e75 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xc61b1 (size=54); checked against the P1 vectors
pub fn f_000c61b1(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32, p_3: u32, p_4: u32) {
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
    let mut pc: u64 = 0xc61b1;
    loop {
        match pc {
            0xc61b1 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    r8 = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0xc61e5; continue; }
                pc = 0xc61ba; continue;
            }
            0xc61ba => {
                {
                    let a = (r9 & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                pc = 0xc61bd; continue;
            }
            0xc61bd => {
                {
                    rax = ((r9 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((rdx & 0xffff)) & 0xffffu64);
                }
                {
                    rax = (r9.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((rsi & 0xffff)) & 0xffffu64);
                }
                {
                    rax = (r9.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((rcx & 0xffff)) & 0xffffu64);
                }
                {
                    rax = (r9.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((rsi & 0xffff)) & 0xffffu64);
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xc61bd; continue; }
                pc = 0xc61e5; continue;
            }
            0xc61e5 => {
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

/// lifted from the machine code of @0x676c0 (size=57); checked against the P1 vectors
pub fn f_000676c0(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32) {
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
    let mut pc: u64 = 0x676c0;
    loop {
        match pc {
            0x676c0 => {
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
                    let b = (rsi & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x676f7; continue; }
                pc = 0x676c8; continue;
            }
            0x676c8 => {
                {
                    r8 = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x676cd; continue;
            }
            0x676cd => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4u64);
                    rsi = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let c = 22u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rsi = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x2u64);
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 12u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(8));
                    rsi = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rsi = (rax.wrapping_add(rsi.wrapping_mul(4))) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xff);
                    let b = (rdx & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x676ec; continue; }
                pc = 0x676ea; continue;
            }
            0x676ea => {
                {
                    rsi = (((rsi & 0xffffffff) as u32).swap_bytes() as u64) & 0xffff_ffffu64;
                }
                pc = 0x676ec; continue;
            }
            0x676ec => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = rcx;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x676cd; continue; }
                pc = 0x676f7; continue;
            }
            0x676f7 => {
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

/// lifted from the machine code of @0x2f04d (size=58); checked against the P1 vectors
pub fn f_0002f04d(m: &mut dyn Mem, p_0: u32, p_1: u32) -> u32 {
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
    let mut pc: u64 = 0x2f04d;
    loop {
        match pc {
            0x2f04d => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = (rdi & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2f085; continue; }
                pc = 0x2f057; continue;
            }
            0x2f057 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x20000u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x2f06e; continue; }
                pc = 0x2f05f; continue;
            }
            0x2f05f => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x1ffffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x2f076; continue; }
                pc = 0x2f067; continue;
            }
            0x2f067 => {
                {
                    let a = (rsi & 0xffffffff);
                    let c = 12u64;
                    let r = a >> c;
                    rsi = (r) & 0xffff_ffffu64;
                }
                {
                    rax = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                pc = 0x2f085; continue;
            }
            0x2f06e => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let b = (rdi & 0xffffffff) as u32 as u128;
                    if b == 0 { panic!("divide error"); }
                    let n = ((((rdx & 0xffff_ffff) << 32) | (rax & 0xffff_ffff)) as u64) as u128;
                    let q = n / b;
                    let rem = n % b;
                    if (q >> 32) != 0 { panic!("divide overflow"); }
                    rax = (q as u64) & 0xffff_ffff;
                    rdx = (rem as u64) & 0xffff_ffff;
                }
                pc = 0x2f085; continue;
            }
            0x2f076 => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0xfffe0000u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rsi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let b = (rdi & 0xffffffff) as u32 as u128;
                    if b == 0 { panic!("divide error"); }
                    let n = ((((rdx & 0xffff_ffff) << 32) | (rax & 0xffff_ffff)) as u64) as u128;
                    let q = n / b;
                    let rem = n % b;
                    if (q >> 32) != 0 { panic!("divide overflow"); }
                    rax = (q as u64) & 0xffff_ffff;
                    rdx = (rem as u64) & 0xffff_ffff;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x20u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x2f085; continue;
            }
            0x2f085 => {
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

/// lifted from the machine code of @0x5ca76 (size=58); checked against the P1 vectors
pub fn f_0005ca76(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0x5ca76;
    loop {
        match pc {
            0x5ca76 => {
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
                    let b = 0x4fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0x5ca9a; continue; }
                pc = 0x5ca81; continue;
            }
            0x5ca81 => {
                {
                    rcx = ((rdi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x25u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5caac; continue; }
                pc = 0x5ca88; continue;
            }
            0x5ca88 => {
                {
                    rdx = 0x30780180c0u64;
                }
                {
                    let a = rdx;
                    cf = (a >> (rcx & 63)) & 1 != 0;
                }
                if cf { pc = 0x5caae; continue; }
                pc = 0x5ca98; continue;
            }
            0x5ca98 => {
                pc = 0x5caac; continue;
            }
            0x5ca9a => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xffffffb0u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x13u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5caac; continue; }
                pc = 0x5caa2; continue;
            }
            0x5caa2 => {
                {
                    rcx = (0xcc1c7u64) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    cf = (a >> ((rdi & 0xffffffff) & 31)) & 1 != 0;
                }
                if cf { pc = 0x5caae; continue; }
                pc = 0x5caac; continue;
            }
            0x5caac => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5caae; continue;
            }
            0x5caae => {
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

/// lifted from the machine code of @0x2ac46 (size=60); checked against the P1 vectors
pub fn f_0002ac46(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x2ac46;
    loop {
        match pc {
            0x2ac46 => {
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
                    stm(m, &mut stk, ea, 4, (0x4u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x2ac81 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x67556 (size=60); checked against the P1 vectors
pub fn f_00067556(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0x67556;
    loop {
        match pc {
            0x67556 => {
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
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rsi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x67590; continue; }
                pc = 0x67561; continue;
            }
            0x67561 => {
                {
                    rsi = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x67565; continue;
            }
            0x67565 => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    r8 = (r8 & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x1u64);
                    r9 = (r9 & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x2u64);
                    rdx = (rdx & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x3u64);
                    rax = (rax & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    stm(m, &mut stk, ea, 1, ((rax & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x1u64);
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((r9 & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x3u64);
                    stm(m, &mut stk, ea, 1, ((r8 & 0xff)) & 0xffu64);
                }
                {
                    let a = rcx;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = rcx;
                    let b = rsi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x67565; continue; }
                pc = 0x67590; continue;
            }
            0x67590 => {
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

/// lifted from the machine code of @0xe5dae (size=64); checked against the P1 vectors
pub fn f_000e5dae(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32) {
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
    let mut pc: u64 = 0xe5dae;
    loop {
        match pc {
            0xe5dae => {
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
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe5dca; continue; }
                pc = 0xe5db6; continue;
            }
            0xe5db6 => {
                {
                    let ea: u64 = rdi.wrapping_add(0xfu64);
                    stm(m, &mut stk, ea, 1, ((rsi & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xff);
                    let b = (rsi & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe5dde; continue; }
                pc = 0xe5dc5; continue;
            }
            0xe5dc5 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x10u64;
                    let r = (a | b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5de6; continue;
            }
            0xe5dca => {
                {
                    let ea: u64 = rdi.wrapping_add(0xeu64);
                    stm(m, &mut stk, ea, 1, ((rsi & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xff);
                    let b = (rsi & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe5de3; continue; }
                pc = 0xe5dd9; continue;
            }
            0xe5dd9 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x20u64;
                    let r = (a | b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5de6; continue;
            }
            0xe5dde => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xffffffefu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5de6; continue;
            }
            0xe5de3 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xffffffdfu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5de6; continue;
            }
            0xe5de6 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
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

/// lifted from the machine code of @0x2c3b6 (size=65); checked against the P1 vectors
pub fn f_0002c3b6(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0x2c3b6;
    loop {
        match pc {
            0x2c3b6 => {
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
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi;
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x2c3d8; continue; }
                pc = 0x2c3c4; continue;
            }
            0x2c3c4 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x2c3d6; continue; }
                pc = 0x2c3c8; continue;
            }
            0x2c3c8 => {
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x2c3d8; continue; }
                pc = 0x2c3d2; continue;
            }
            0x2c3d2 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0x2c3da; continue; }
                pc = 0x2c3d6; continue;
            }
            0x2c3d6 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x2c3d8; continue;
            }
            0x2c3d8 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x2c3da => {
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x2c3d8; continue; }
                pc = 0x2c3e4; continue;
            }
            0x2c3e4 => {
                {
                    let ea: u64 = rsi.wrapping_add(0xcu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                {
                    let a = (rsi & 0xff);
                    let b = (rsi & 0xff);
                    let c = cf as u64;
                    let r = a.wrapping_sub(b).wrapping_sub(c) & 0xffu64;
                    rsi = (rsi & !0xffu64) | ((r) & 0xff);
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                {
                    rax = (rax & !0xffu64) | (((!cf) as u64) & 0xff);
                }
                {
                    let a = (rax & 0xff);
                    let b = (rsi & 0xff);
                    let r = (a & b) & 0xffu64;
                    rax = (rax & !0xffu64) | ((r) & 0xff);
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

/// lifted from the machine code of @0x515f4 (size=66); checked against the P1 vectors
pub fn f_000515f4(m: &mut dyn Mem, p_0: u64, p_1: u64) {
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
    let mut pc: u64 = 0x515f4;
    loop {
        match pc {
            0x515f4 => {
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
                    let ea: u64 = rdx;
                    st128m(m, &mut stk, ea, xmm0);
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
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x18u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x20u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x20u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x30u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x30u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x40u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x40u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x50u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x50u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x51635 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x522f0 (size=66); checked against the P1 vectors
pub fn f_000522f0(m: &mut dyn Mem, p_0: u64, p_1: u64) {
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
    let mut pc: u64 = 0x522f0;
    loop {
        match pc {
            0x522f0 => {
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
                    let ea: u64 = rdx;
                    st128m(m, &mut stk, ea, xmm0);
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
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x18u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x18u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x20u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x20u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x30u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x30u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x40u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x40u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x50u64);
                    xmm0 = ld128m(m, &stk, ea);
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x50u64);
                    st128m(m, &mut stk, ea, xmm0);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x52331 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x67511 (size=69); checked against the P1 vectors
pub fn f_00067511(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0x67511;
    loop {
        match pc {
            0x67511 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
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
                    let a = (rsi & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    zf = r == 0;
                    rsi = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x67551; continue; }
                pc = 0x6751c; continue;
            }
            0x6751c => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x6751e; continue;
            }
            0x6751e => {
                {
                    r10 = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r10);
                    r8 = (r8 & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    rdx = (rax.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rdx);
                    r9 = (r9 & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    rcx = (rax.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    r11 = (r11 & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    rbx = (rax.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rbx);
                    r14 = (r14 & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(r10);
                    stm(m, &mut stk, ea, 1, ((r9 & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rdx);
                    stm(m, &mut stk, ea, 1, ((r8 & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    stm(m, &mut stk, ea, 1, ((r14 & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rbx);
                    stm(m, &mut stk, ea, 1, ((r11 & 0xff)) & 0xffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x6751e; continue; }
                pc = 0x67551; continue;
            }
            0x67551 => {
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
                    rbp = v;
                }
                return ();
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x2b6f4 (size=72); checked against the P1 vectors
pub fn f_0002b6f4(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32, p_3: u32, p_4: u32) -> u8 {
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
    let mut pc: u64 = 0x2b6f4;
    loop {
        match pc {
            0x2b6f4 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2b71e; continue; }
                pc = 0x2b6fe; continue;
            }
            0x2b6fe => {
                {
                    let ea: u64 = rdi.wrapping_add(0x208u64);
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x2b722; continue; }
                pc = 0x2b709; continue;
            }
            0x2b709 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x20cu64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x210u64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x214u64);
                    stm(m, &mut stk, ea, 4, ((r8 & 0xffffffff)) & 0xffffffffu64);
                }
                pc = 0x2b738; continue;
            }
            0x2b71e => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x2b73a; continue;
            }
            0x2b722 => {
                {
                    let a = rdi;
                    let b = 0x208u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                pc = 0x2b738; continue;
            }
            0x2b738 => {
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0x2b73a; continue;
            }
            0x2b73a => {
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

/// lifted from the machine code of @0x2ea9e (size=20); checked against the P1 vectors
pub fn f_0002ea9e(m: &mut dyn Mem, p_0: u64) -> u32 {
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
    let mut pc: u64 = 0x2ea9e;
    loop {
        match pc {
            0x2ea9e => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x48u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = rcx;
                    let b = rcx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2eab0; continue; }
                pc = 0x2eaad; continue;
            }
            0x2eaad => {
                {
                    let ea: u64 = rcx.wrapping_add(0x1cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0x2eab0; continue;
            }
            0x2eab0 => {
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
