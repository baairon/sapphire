#![allow(unused_assignments, unused_variables, unused_mut, unused_parens, unreachable_code, unused_imports, clippy::all)]
use crate::lift_rt::*;
use crate::Mem;

/// lifted from the machine code of @0x5cbbd (size=30); checked against the P1 vectors
pub fn f_0005cbbd(m: &mut dyn Mem, p_0: u32) -> u32 {
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
    let mut pc: u64 = 0x5cbbd;
    loop {
        match pc {
            0x5cbbd => {
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
                    let b = 0x8u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5cbd7; continue; }
                pc = 0x5cbc8; continue;
            }
            0x5cbc8 => {
                {
                    rax = (0x137u64) & 0xffff_ffffu64;
                }
                {
                    rcx = (rcx & !0xffu64) | (((rdi & 0xff)) & 0xff);
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
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xffffffff) as u32;
            }
            0x5cbd7 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
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

/// lifted from the machine code of @0x5cab0 (size=31); checked against the P1 vectors
pub fn f_0005cab0(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0x5cab0;
    loop {
        match pc {
            0x5cab0 => {
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
                    let b = 0x20u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5cacb; continue; }
                pc = 0x5cabb; continue;
            }
            0x5cabb => {
                {
                    rax = (0x7ffffff8u64) & 0xffff_ffffu64;
                }
                {
                    rcx = (rcx & !0xffu64) | (((rdi & 0xff)) & 0xff);
                }
                {
                    let a = rax;
                    let c = (rcx & 63u64);
                    let r = a >> c;
                    rax = r;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x5cacb => {
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

/// lifted from the machine code of @0x5caed (size=31); checked against the P1 vectors
pub fn f_0005caed(m: &mut dyn Mem, p_0: u32) -> u32 {
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
    let mut pc: u64 = 0x5caed;
    loop {
        match pc {
            0x5caed => {
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
                    let b = 0xfffffffeu64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x16u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5cb08; continue; }
                pc = 0x5caf9; continue;
            }
            0x5caf9 => {
                {
                    rax = (0x5c6c37u64) & 0xffff_ffffu64;
                }
                {
                    rcx = (rcx & !0xffu64) | (((rdi & 0xff)) & 0xff);
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
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xffffffff) as u32;
            }
            0x5cb08 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
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

/// lifted from the machine code of @0x5cb2b (size=31); checked against the P1 vectors
pub fn f_0005cb2b(m: &mut dyn Mem, p_0: u32) -> u32 {
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
    let mut pc: u64 = 0x5cb2b;
    loop {
        match pc {
            0x5cb2b => {
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
                    let b = 0xfffffffeu64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x14u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5cb46; continue; }
                pc = 0x5cb37; continue;
            }
            0x5cb37 => {
                {
                    rax = (0x181013u64) & 0xffff_ffffu64;
                }
                {
                    rcx = (rcx & !0xffu64) | (((rdi & 0xff)) & 0xff);
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
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xffffffff) as u32;
            }
            0x5cb46 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
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

/// lifted from the machine code of @0xba91e (size=32); checked against the P1 vectors
pub fn f_000ba91e(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0xba91e;
    loop {
        match pc {
            0xba91e => {
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
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0xcu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
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
            0xba93d => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xe139c (size=32); checked against the P1 vectors
pub fn f_000e139c(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0xe139c;
    loop {
        match pc {
            0xe139c => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    let a = (rax & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe13b8; continue; }
                pc = 0xe13a8; continue;
            }
            0xe13a8 => {
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
                if !zf { pc = 0xe13b8; continue; }
                pc = 0xe13ae; continue;
            }
            0xe13ae => {
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    let a = (rcx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe13ba; continue; }
                pc = 0xe13b8; continue;
            }
            0xe13b8 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe13ba; continue;
            }
            0xe13ba => {
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

/// lifted from the machine code of @0xe137a (size=34); checked against the P1 vectors
pub fn f_000e137a(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0xe137a;
    loop {
        match pc {
            0xe137a => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    let a = (rax & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe1398; continue; }
                pc = 0xe1386; continue;
            }
            0xe1386 => {
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
                if !zf { pc = 0xe1398; continue; }
                pc = 0xe138c; continue;
            }
            0xe138c => {
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
                if !zf { pc = 0xe1398; continue; }
                pc = 0xe1394; continue;
            }
            0xe1394 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe139a; continue;
            }
            0xe1398 => {
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0xe139a; continue;
            }
            0xe139a => {
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

/// lifted from the machine code of @0x5932c (size=35); checked against the P1 vectors
pub fn f_0005932c(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0x5932c;
    loop {
        match pc {
            0x5932c => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let a = (rsi & 0xffff);
                    let b = (rsi & 0xffff);
                    let r = (a & b) & 0xffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5934d; continue; }
                pc = 0x59335; continue;
            }
            0x59335 => {
                {
                    rax = ((rsi & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x5933d; continue;
            }
            0x5933d => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 4, (0x400200u64) & 0xffffffffu64);
                }
                {
                    let a = rcx;
                    let b = 0x2u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = rcx;
                    let b = rax;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 63 & 1 != 0;
                    sf = (r >> 63) & 1 != 0;
                }
                if sf != ov { pc = 0x5933d; continue; }
                pc = 0x5934d; continue;
            }
            0x5934d => {
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

/// lifted from the machine code of @0x674ee (size=35); checked against the P1 vectors
pub fn f_000674ee(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x674ee;
    loop {
        match pc {
            0x674ee => {
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
                    let b = 0x7fffffffu64;
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x6750f; continue; }
                pc = 0x674fa; continue;
            }
            0x674fa => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                pc = 0x674fc; continue;
            }
            0x674fc => {
                {
                    let ea: u64 = rdi;
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 2u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 1, ((rax & 0xff)) & 0xffu64);
                }
                {
                    let a = rdi;
                    let b = 0x2u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rsi;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if !zf { pc = 0x674fc; continue; }
                pc = 0x6750f; continue;
            }
            0x6750f => {
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

/// lifted from the machine code of @0xc621b (size=35); checked against the P1 vectors
pub fn f_000c621b(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0xc621b;
    loop {
        match pc {
            0xc621b => {
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
                    let b = 0x7fffffffu64;
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc623c; continue; }
                pc = 0xc6227; continue;
            }
            0xc6227 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                pc = 0xc6229; continue;
            }
            0xc6229 => {
                {
                    let ea: u64 = rdi;
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 2u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 1, ((rax & 0xff)) & 0xffu64);
                }
                {
                    let a = rdi;
                    let b = 0x2u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rsi;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if !zf { pc = 0xc6229; continue; }
                pc = 0xc623c; continue;
            }
            0xc623c => {
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

/// lifted from the machine code of @0x25e30 (size=36); checked against the P1 vectors
pub fn f_00025e30(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x25e30;
    loop {
        match pc {
            0x25e30 => {
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
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
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

/// lifted from the machine code of @0x25e78 (size=36); checked against the P1 vectors
pub fn f_00025e78(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x25e78;
    loop {
        match pc {
            0x25e78 => {
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
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
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

/// lifted from the machine code of @0x26008 (size=36); checked against the P1 vectors
pub fn f_00026008(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x26008;
    loop {
        match pc {
            0x26008 => {
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
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
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

/// lifted from the machine code of @0xe62da (size=36); checked against the P1 vectors
pub fn f_000e62da(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0xe62da;
    loop {
        match pc {
            0xe62da => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x80u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    rax = (rax & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0xe62fd => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x66bff (size=37); checked against the P1 vectors
pub fn f_00066bff(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0x66bff;
    loop {
        match pc {
            0x66bff => {
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
                if zf { pc = 0x66c22; continue; }
                pc = 0x66c07; continue;
            }
            0x66c07 => {
                {
                    rax = rdi;
                }
                pc = 0x66c0a; continue;
            }
            0x66c0a => {
                {
                    let ea: u64 = rdi;
                    rcx = (rcx & !0xffffu64) | ((ldm(m, &stk, ea, 2)) & 0xffff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2u64);
                    rdx = (rdx & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rax;
                    stm(m, &mut stk, ea, 2, ((rcx & 0xffff)) & 0xffffu64);
                }
                {
                    let ea: u64 = rax.wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
                }
                {
                    let a = rdi;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rax;
                    let b = 0x3u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    zf = r == 0;
                    rsi = (r) & 0xffff_ffffu64;
                }
                if !zf { pc = 0x66c0a; continue; }
                pc = 0x66c22; continue;
            }
            0x66c22 => {
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

/// lifted from the machine code of @0x2ac1e (size=40); checked against the P1 vectors
pub fn f_0002ac1e(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32, p_3: u32, p_4: u32) {
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
    let mut pc: u64 = 0x2ac1e;
    loop {
        match pc {
            0x2ac1e => {
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
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x1u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if cf || zf { pc = 0x2ac35; continue; }
                pc = 0x2ac29; continue;
            }
            0x2ac29 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 4, ((r8 & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x2ac35 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
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

/// lifted from the machine code of @0x67592 (size=40); checked against the P1 vectors
pub fn f_00067592(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0x67592;
    loop {
        match pc {
            0x67592 => {
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
                if zf { pc = 0x675b8; continue; }
                pc = 0x6759d; continue;
            }
            0x6759d => {
                {
                    rsi = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x675a1; continue;
            }
            0x675a1 => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    rdx = (rdx & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x2u64);
                    rax = (rax & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    stm(m, &mut stk, ea, 1, ((rax & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
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
                if cf { pc = 0x675a1; continue; }
                pc = 0x675b8; continue;
            }
            0x675b8 => {
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

/// lifted from the machine code of @0xc7a73 (size=40); checked against the P1 vectors
pub fn f_000c7a73(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0xc7a73;
    loop {
        match pc {
            0xc7a73 => {
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
                if zf { pc = 0xc7a99; continue; }
                pc = 0xc7a7e; continue;
            }
            0xc7a7e => {
                {
                    rsi = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0xc7a82; continue;
            }
            0xc7a82 => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    rdx = (rdx & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x2u64);
                    rax = (rax & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx);
                    stm(m, &mut stk, ea, 1, ((rax & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx).wrapping_add(0x2u64);
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
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
                if cf { pc = 0xc7a82; continue; }
                pc = 0xc7a99; continue;
            }
            0xc7a99 => {
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

/// lifted from the machine code of @0x28fb0 (size=42); checked against the P1 vectors
pub fn f_00028fb0(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32) -> u64 {
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
    let mut pc: u64 = 0x28fb0;
    loop {
        match pc {
            0x28fb0 => {
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
                if zf { pc = 0x28fd8; continue; }
                pc = 0x28fbe; continue;
            }
            0x28fbe => {
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rdi = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
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
                if zf { pc = 0x28fd8; continue; }
                pc = 0x28fc7; continue;
            }
            0x28fc7 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                    rdi = (r) & 0xffff_ffffu64;
                }
                if cf || zf { pc = 0x28fd8; continue; }
                pc = 0x28fcb; continue;
            }
            0x28fcb => {
                {
                    let a = (rdx & 0xff);
                    let b = (rdx & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                {
                    let v = if !zf { (rdi & 0xffffffff) } else { (rsi & 0xffffffff) };
                    rsi = (v) & 0xffff_ffffu64;
                }
                {
                    rax = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = rcx;
                    let b = rax;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    rax = rcx;
                }
                pc = 0x28fd8; continue;
            }
            0x28fd8 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x46c6e (size=42); checked against the P1 vectors
pub fn f_00046c6e(m: &mut dyn Mem, p_0: u64) -> u8 {
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
    let mut pc: u64 = 0x46c6e;
    loop {
        match pc {
            0x46c6e => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x220u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x218u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x210u64);
                    stm(m, &mut stk, ea, 8, (0x0u64) & 0xffffffffffffffffu64);
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
            0x46c97 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x6744c (size=42); checked against the P1 vectors
pub fn f_0006744c(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x6744c;
    loop {
        match pc {
            0x6744c => {
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
                if zf { pc = 0x67474; continue; }
                pc = 0x67454; continue;
            }
            0x67454 => {
                {
                    let ea: u64 = rdi;
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2u64);
                    rcx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x3ac00000u64;
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = rsi;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    let a = rdi;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if !zf { pc = 0x67454; continue; }
                pc = 0x67474; continue;
            }
            0x67474 => {
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

/// lifted from the machine code of @0xe5dee (size=42); checked against the P1 vectors
pub fn f_000e5dee(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0xe5dee;
    loop {
        match pc {
            0xe5dee => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 1, ((rsi & 0xff)) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0xfffffbffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x400u64;
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
                    let v = if zf { (rcx & 0xffffffff) } else { (rax & 0xffffffff) };
                    rax = (v) & 0xffff_ffffu64;
                }
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
            0xe5e17 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xe5f92 (size=42); checked against the P1 vectors
pub fn f_000e5f92(m: &mut dyn Mem, p_0: u64) -> u8 {
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
    let mut pc: u64 = 0xe5f92;
    loop {
        match pc {
            0xe5f92 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xfffffffbu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = rdi.wrapping_add(0x88u64);
                }
                {
                    let a = rdi;
                    let b = 0x84u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                {
                    let v = if zf { rcx } else { rdi };
                    rdi = v;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x3u64);
                    rax = (rax & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let a = (rax & 0xff);
                    let b = 0x8u64;
                    let r = (a & b) & 0xffu64;
                    rax = (rax & !0xffu64) | ((r) & 0xff);
                }
                {
                    let a = (rax & 0xff);
                    let c = 3u64;
                    let r = a >> c;
                    rax = (rax & !0xffu64) | ((r) & 0xff);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0xe5fbb => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x591c0 (size=43); checked against the P1 vectors
pub fn f_000591c0(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x591c0;
    loop {
        match pc {
            0x591c0 => {
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
                if zf { pc = 0x591e9; continue; }
                pc = 0x591c8; continue;
            }
            0x591c8 => {
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
                    let b = 0x3u64;
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
                    let r = a.wrapping_add(b) & 0xffffffffu64;
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
                if !zf { pc = 0x591c8; continue; }
                pc = 0x591e9; continue;
            }
            0x591e9 => {
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

/// lifted from the machine code of @0x28c46 (size=44); checked against the P1 vectors
pub fn f_00028c46(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x28c46;
    loop {
        match pc {
            0x28c46 => {
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
                    stm(m, &mut stk, ea, 8, (rsi) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
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
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x28c71 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
