#![allow(unused_assignments, unused_variables, unused_mut, unused_parens, unreachable_code, unused_imports, clippy::all)]
use crate::lift_rt::*;
use crate::Mem;

/// lifted from the machine code of @0x28f4a (size=34); checked against the P1 vectors
pub fn f_00028f4a(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0x28f4a;
    loop {
        match pc {
            0x28f4a => {
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
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x28f69; continue; }
                pc = 0x28f56; continue;
            }
            0x28f56 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 1u64;
                    let r = a >> c;
                    if c != 0 {
                        zf = r == 0;
                    }
                    rcx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x28f69; continue; }
                pc = 0x28f5d; continue;
            }
            0x28f5d => {
                {
                    let ea: u64 = rax;
                    stm(m, &mut stk, ea, 2, ((rsi & 0xffff)) & 0xffffu64);
                }
                {
                    let a = rax;
                    let b = 0x2u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let a = rcx;
                    let r = a.wrapping_sub(1) & 0xffffffffffffffffu64;
                    zf = r == 0;
                    rcx = r;
                }
                if !zf { pc = 0x28f5d; continue; }
                pc = 0x28f69; continue;
            }
            0x28f69 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x28f6b => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x28f8e (size=34); checked against the P1 vectors
pub fn f_00028f8e(m: &mut dyn Mem, p_0: u64, p_1: u64) {
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
    let mut pc: u64 = 0x28f8e;
    loop {
        match pc {
            0x28f8e => {
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
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x28fae; continue; }
                pc = 0x28f9a; continue;
            }
            0x28f9a => {
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 3u64;
                    let r = a >> c;
                    if c != 0 {
                        zf = r == 0;
                    }
                    rcx = (r) & 0xffff_ffffu64;
                }
                if zf { pc = 0x28fae; continue; }
                pc = 0x28fa2; continue;
            }
            0x28fa2 => {
                {
                    let ea: u64 = rax;
                    stm(m, &mut stk, ea, 8, (rsi) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let a = rcx;
                    let r = a.wrapping_sub(1) & 0xffffffffffffffffu64;
                    zf = r == 0;
                    rcx = r;
                }
                if !zf { pc = 0x28fa2; continue; }
                pc = 0x28fae; continue;
            }
            0x28fae => {
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

/// lifted from the machine code of @0x2ca4c (size=34); checked against the P1 vectors
pub fn f_0002ca4c(m: &mut dyn Mem, p_0: u32, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0x2ca4c;
    loop {
        match pc {
            0x2ca4c => {
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
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rcx;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2ca6c; continue; }
                pc = 0x2ca5c; continue;
            }
            0x2ca5c => {
                {
                    let ea: u64 = rax;
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rdi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2ca6c; continue; }
                pc = 0x2ca60; continue;
            }
            0x2ca60 => {
                {
                    let a = rax;
                    let b = 0x10u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let a = rcx;
                    let b = rax;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2ca5c; continue; }
                pc = 0x2ca69; continue;
            }
            0x2ca69 => {
                {
                    rax = rcx;
                }
                pc = 0x2ca6c; continue;
            }
            0x2ca6c => {
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

/// lifted from the machine code of @0x2b622 (size=48); checked against the P1 vectors
pub fn f_0002b622(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) -> u8 {
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
    let mut pc: u64 = 0x2b622;
    loop {
        match pc {
            0x2b622 => {
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
                    let a = (rdx & 0xffffffff);
                    let b = 0x1au64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x2b64f; continue; }
                pc = 0x2b62d; continue;
            }
            0x2b62d => {
                {
                    let ea: u64 = rdi.wrapping_add(0xc0u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rcx;
                    let b = rcx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2b64f; continue; }
                pc = 0x2b639; continue;
            }
            0x2b639 => {
                {
                    rax = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    rax = rax.wrapping_add(rax.wrapping_mul(2));
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi;
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_add(rax.wrapping_mul(4));
                    stm(m, &mut stk, ea, 8, (rdx) & 0xffffffffffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0x2b64f; continue;
            }
            0x2b64f => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x2b651 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x2b77c (size=68); checked against the P1 vectors
pub fn f_0002b77c(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) -> u8 {
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
    let mut pc: u64 = 0x2b77c;
    loop {
        match pc {
            0x2b77c => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x16cu64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, (0xffffffffu64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    stm(m, &mut stk, ea, 4, (0xffffffffu64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, (0xffffffffu64) & 0xffffffffu64);
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0x1au64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x2b7bd; continue; }
                pc = 0x2b7a2; continue;
            }
            0x2b7a2 => {
                {
                    let a = rcx;
                    let b = rcx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2b7bd; continue; }
                pc = 0x2b7a7; continue;
            }
            0x2b7a7 => {
                {
                    rax = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    rax = rax.wrapping_add(rax.wrapping_mul(2));
                }
                {
                    let ea: u64 = rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rcx.wrapping_add(rax.wrapping_mul(4));
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0x2b7bd; continue;
            }
            0x2b7bd => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x2b7bf => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x2d990 (size=72); checked against the P1 vectors
pub fn f_0002d990(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x2d990;
    loop {
        match pc {
            0x2d990 => {
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
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rax;
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let a = rcx;
                    let b = rcx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2d9a8; continue; }
                pc = 0x2d9a4; continue;
            }
            0x2d9a4 => {
                {
                    let ea: u64 = rcx.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                pc = 0x2d9a8; continue;
            }
            0x2d9a8 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rax.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx;
                    let a = ldm(m, &stk, ea, 8);
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2d9bf; continue; }
                pc = 0x2d9b9; continue;
            }
            0x2d9b9 => {
                {
                    let ea: u64 = rcx.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x2d9c2; continue;
            }
            0x2d9bf => {
                {
                    let ea: u64 = rcx;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x2d9c2; continue;
            }
            0x2d9c2 => {
                {
                    let ea: u64 = rax;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x2d9cb => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x11195 (size=59); checked against the P1 vectors
pub fn f_00011195(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x11195;
    loop {
        match pc {
            0x11195 => {
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
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let a = rcx;
                    let b = rcx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x111ac; continue; }
                pc = 0x111a8; continue;
            }
            0x111a8 => {
                {
                    let ea: u64 = rcx.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                pc = 0x111ac; continue;
            }
            0x111ac => {
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rax.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 8, (rcx) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx;
                    let a = ldm(m, &stk, ea, 8);
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x111c3; continue; }
                pc = 0x111bd; continue;
            }
            0x111bd => {
                {
                    let ea: u64 = rcx.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x111c6; continue;
            }
            0x111c3 => {
                {
                    let ea: u64 = rcx;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x111c6; continue;
            }
            0x111c6 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
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

/// lifted from the machine code of @0xb5106 (size=72); checked against the P1 vectors
pub fn f_000b5106(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0xb5106;
    loop {
        match pc {
            0xb5106 => {
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
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xb5149; continue; }
                pc = 0xb5117; continue;
            }
            0xb5117 => {
                {
                    let ea: u64 = rsi;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rsi = rdi;
                }
                pc = 0xb511c; continue;
            }
            0xb511c => {
                {
                    rax = rdx;
                }
                pc = 0xb511f; continue;
            }
            0xb511f => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb512f; continue; }
                pc = 0xb5124; continue;
            }
            0xb5124 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xb511f; continue; }
                pc = 0xb512d; continue;
            }
            0xb512d => {
                pc = 0xb513c; continue;
            }
            0xb512f => {
                {
                    let ea: u64 = rax;
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                {
                    rsi = rax;
                }
                if !zf { pc = 0xb511c; continue; }
                pc = 0xb513a; continue;
            }
            0xb513a => {
                pc = 0xb513f; continue;
            }
            0xb513c => {
                {
                    rax = rsi;
                }
                pc = 0xb513f; continue;
            }
            0xb513f => {
                {
                    let a = rax;
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xb5149; continue; }
                pc = 0xb5144; continue;
            }
            0xb5144 => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = (rcx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb514c; continue; }
                pc = 0xb5149; continue;
            }
            0xb5149 => {
                {
                    rax = rdi;
                }
                pc = 0xb514c; continue;
            }
            0xb514c => {
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

/// lifted from the machine code of @0xb5ba2 (size=72); checked against the P1 vectors
pub fn f_000b5ba2(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0xb5ba2;
    loop {
        match pc {
            0xb5ba2 => {
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
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xb5be5; continue; }
                pc = 0xb5bb3; continue;
            }
            0xb5bb3 => {
                {
                    let ea: u64 = rsi;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rsi = rdi;
                }
                pc = 0xb5bb8; continue;
            }
            0xb5bb8 => {
                {
                    rax = rdx;
                }
                pc = 0xb5bbb; continue;
            }
            0xb5bbb => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb5bcb; continue; }
                pc = 0xb5bc0; continue;
            }
            0xb5bc0 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xb5bbb; continue; }
                pc = 0xb5bc9; continue;
            }
            0xb5bc9 => {
                pc = 0xb5bd8; continue;
            }
            0xb5bcb => {
                {
                    let ea: u64 = rax;
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                {
                    rsi = rax;
                }
                if !zf { pc = 0xb5bb8; continue; }
                pc = 0xb5bd6; continue;
            }
            0xb5bd6 => {
                pc = 0xb5bdb; continue;
            }
            0xb5bd8 => {
                {
                    rax = rsi;
                }
                pc = 0xb5bdb; continue;
            }
            0xb5bdb => {
                {
                    let a = rax;
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xb5be5; continue; }
                pc = 0xb5be0; continue;
            }
            0xb5be0 => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = (rcx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb5be8; continue; }
                pc = 0xb5be5; continue;
            }
            0xb5be5 => {
                {
                    rax = rdi;
                }
                pc = 0xb5be8; continue;
            }
            0xb5be8 => {
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

/// lifted from the machine code of @0xc47da (size=72); checked against the P1 vectors
pub fn f_000c47da(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0xc47da;
    loop {
        match pc {
            0xc47da => {
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
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc481d; continue; }
                pc = 0xc47eb; continue;
            }
            0xc47eb => {
                {
                    let ea: u64 = rsi;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rsi = rdi;
                }
                pc = 0xc47f0; continue;
            }
            0xc47f0 => {
                {
                    rax = rdx;
                }
                pc = 0xc47f3; continue;
            }
            0xc47f3 => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xc4803; continue; }
                pc = 0xc47f8; continue;
            }
            0xc47f8 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xc47f3; continue; }
                pc = 0xc4801; continue;
            }
            0xc4801 => {
                pc = 0xc4810; continue;
            }
            0xc4803 => {
                {
                    let ea: u64 = rax;
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                {
                    rsi = rax;
                }
                if !zf { pc = 0xc47f0; continue; }
                pc = 0xc480e; continue;
            }
            0xc480e => {
                pc = 0xc4813; continue;
            }
            0xc4810 => {
                {
                    rax = rsi;
                }
                pc = 0xc4813; continue;
            }
            0xc4813 => {
                {
                    let a = rax;
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc481d; continue; }
                pc = 0xc4818; continue;
            }
            0xc4818 => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = (rcx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xc4820; continue; }
                pc = 0xc481d; continue;
            }
            0xc481d => {
                {
                    rax = rdi;
                }
                pc = 0xc4820; continue;
            }
            0xc4820 => {
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

/// lifted from the machine code of @0xc446e (size=72); checked against the P1 vectors
pub fn f_000c446e(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0xc446e;
    loop {
        match pc {
            0xc446e => {
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
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc44b1; continue; }
                pc = 0xc447f; continue;
            }
            0xc447f => {
                {
                    let ea: u64 = rsi;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rsi = rdi;
                }
                pc = 0xc4484; continue;
            }
            0xc4484 => {
                {
                    rax = rdx;
                }
                pc = 0xc4487; continue;
            }
            0xc4487 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xc4497; continue; }
                pc = 0xc448c; continue;
            }
            0xc448c => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xc4487; continue; }
                pc = 0xc4495; continue;
            }
            0xc4495 => {
                pc = 0xc44a4; continue;
            }
            0xc4497 => {
                {
                    let ea: u64 = rax;
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                {
                    rsi = rax;
                }
                if !zf { pc = 0xc4484; continue; }
                pc = 0xc44a2; continue;
            }
            0xc44a2 => {
                pc = 0xc44a7; continue;
            }
            0xc44a4 => {
                {
                    rax = rsi;
                }
                pc = 0xc44a7; continue;
            }
            0xc44a7 => {
                {
                    let a = rax;
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc44b1; continue; }
                pc = 0xc44ac; continue;
            }
            0xc44ac => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    let a = (rcx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xc44b4; continue; }
                pc = 0xc44b1; continue;
            }
            0xc44b1 => {
                {
                    rax = rdi;
                }
                pc = 0xc44b4; continue;
            }
            0xc44b4 => {
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

/// lifted from the machine code of @0xc4cb4 (size=72); checked against the P1 vectors
pub fn f_000c4cb4(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0xc4cb4;
    loop {
        match pc {
            0xc4cb4 => {
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
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc4cf7; continue; }
                pc = 0xc4cc5; continue;
            }
            0xc4cc5 => {
                {
                    let ea: u64 = rsi;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rsi = rdi;
                }
                pc = 0xc4cca; continue;
            }
            0xc4cca => {
                {
                    rax = rdx;
                }
                pc = 0xc4ccd; continue;
            }
            0xc4ccd => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc4cdd; continue; }
                pc = 0xc4cd2; continue;
            }
            0xc4cd2 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xc4ccd; continue; }
                pc = 0xc4cdb; continue;
            }
            0xc4cdb => {
                pc = 0xc4cea; continue;
            }
            0xc4cdd => {
                {
                    let ea: u64 = rax;
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                {
                    rsi = rax;
                }
                if !zf { pc = 0xc4cca; continue; }
                pc = 0xc4ce8; continue;
            }
            0xc4ce8 => {
                pc = 0xc4ced; continue;
            }
            0xc4cea => {
                {
                    rax = rsi;
                }
                pc = 0xc4ced; continue;
            }
            0xc4ced => {
                {
                    let a = rax;
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc4cf7; continue; }
                pc = 0xc4cf2; continue;
            }
            0xc4cf2 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    let a = (rcx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc4cfa; continue; }
                pc = 0xc4cf7; continue;
            }
            0xc4cf7 => {
                {
                    rax = rdi;
                }
                pc = 0xc4cfa; continue;
            }
            0xc4cfa => {
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

/// lifted from the machine code of @0x25d9c (size=74); checked against the P1 vectors
pub fn f_00025d9c(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0x25d9c;
    loop {
        match pc {
            0x25d9c => {
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
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x25ddf; continue; }
                pc = 0x25dad; continue;
            }
            0x25dad => {
                {
                    let ea: u64 = rsi;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rsi = rdi;
                }
                pc = 0x25db2; continue;
            }
            0x25db2 => {
                {
                    rax = rdx;
                }
                pc = 0x25db5; continue;
            }
            0x25db5 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x25dc5; continue; }
                pc = 0x25dba; continue;
            }
            0x25dba => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x25db5; continue; }
                pc = 0x25dc3; continue;
            }
            0x25dc3 => {
                pc = 0x25dd2; continue;
            }
            0x25dc5 => {
                {
                    let ea: u64 = rax;
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                {
                    rsi = rax;
                }
                if !zf { pc = 0x25db2; continue; }
                pc = 0x25dd0; continue;
            }
            0x25dd0 => {
                pc = 0x25dd5; continue;
            }
            0x25dd2 => {
                {
                    rax = rsi;
                }
                pc = 0x25dd5; continue;
            }
            0x25dd5 => {
                {
                    let a = rax;
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x25ddf; continue; }
                pc = 0x25dda; continue;
            }
            0x25dda => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    let a = (rcx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x25de2; continue; }
                pc = 0x25ddf; continue;
            }
            0x25ddf => {
                {
                    rax = rdi;
                }
                pc = 0x25de2; continue;
            }
            0x25de2 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            0x25de4 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xc60f5 (size=73); checked against the P1 vectors
pub fn f_000c60f5(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32, p_3: u32) {
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
    let mut pc: u64 = 0xc60f5;
    loop {
        match pc {
            0xc60f5 => {
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
                if zf { pc = 0xc613c; continue; }
                pc = 0xc60fd; continue;
            }
            0xc60fd => {
                {
                    let a = rdi;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                pc = 0xc6101; continue;
            }
            0xc6101 => {
                {
                    let a = (rcx & 0xff);
                    let b = (rcx & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xc6130; continue; }
                pc = 0xc6105; continue;
            }
            0xc6105 => {
                {
                    let ea: u64 = rsi;
                    r8 = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rax = ((r8 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 22u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 2, ((rax & 0xffff)) & 0xffffu64);
                }
                {
                    rax = ((r8 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 12u64;
                    let r = a >> c;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_sub(0x2u64);
                    stm(m, &mut stk, ea, 2, ((rax & 0xffff)) & 0xffffu64);
                }
                {
                    let a = (r8 & 0xffffffff);
                    let c = 2u64;
                    let r = a >> c;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_sub(0x4u64);
                    stm(m, &mut stk, ea, 2, ((r8 & 0xffff)) & 0xffffu64);
                }
                pc = 0xc6130; continue;
            }
            0xc6130 => {
                {
                    let a = rsi;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if !zf { pc = 0xc6101; continue; }
                pc = 0xc613c; continue;
            }
            0xc613c => {
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

/// lifted from the machine code of @0xd10a5 (size=73); checked against the P1 vectors
pub fn f_000d10a5(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0xd10a5;
    loop {
        match pc {
            0xd10a5 => {
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
                    let b = 0x104782ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd10bd; continue; }
                pc = 0xd10b3; continue;
            }
            0xd10b3 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10378800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd10ea; continue; }
                pc = 0xd10bb; continue;
            }
            0xd10bb => {
                pc = 0xd10ec; continue;
            }
            0xd10bd => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1047834fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xd10cf; continue; }
                pc = 0xd10c5; continue;
            }
            0xd10c5 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478300u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd10ea; continue; }
                pc = 0xd10cd; continue;
            }
            0xd10cd => {
                pc = 0xd10ec; continue;
            }
            0xd10cf => {
                {
                    rcx = (rdi.wrapping_sub(0x10710850u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xd10ec; continue; }
                pc = 0xd10da; continue;
            }
            0xd10da => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478350u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd10ec; continue; }
                pc = 0xd10e2; continue;
            }
            0xd10e2 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10710800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd10ec; continue; }
                pc = 0xd10ea; continue;
            }
            0xd10ea => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xd10ec; continue;
            }
            0xd10ec => {
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

/// lifted from the machine code of @0x58c54 (size=78); checked against the P1 vectors
pub fn f_00058c54(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0x58c54;
    loop {
        match pc {
            0x58c54 => {
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
                if zf { pc = 0x58ca0; continue; }
                pc = 0x58c5c; continue;
            }
            0x58c5c => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x58c5e; continue;
            }
            0x58c5e => {
                {
                    r8 = ((rax & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r8.wrapping_mul(4));
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    r9 = ((rcx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let c = 26u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    rdx = ((rcx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let c = 22u64;
                    let r = a >> c;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    r10 = ((rdx & 0xff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    rdx = ((rcx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let c = 10u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0xff0000u64;
                    let r = (a & b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 6u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0xff00u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(r8.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (rax.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    rcx = ((rax & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x58c5e; continue; }
                pc = 0x58ca0; continue;
            }
            0x58ca0 => {
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

/// lifted from the machine code of @0x2b652 (size=82); checked against the P1 vectors
pub fn f_0002b652(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u8 {
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
    let mut pc: u64 = 0x2b652;
    loop {
        match pc {
            0x2b652 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xc0u64);
                    rcx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rcx;
                    let b = rcx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2b6a0; continue; }
                pc = 0x2b662; continue;
            }
            0x2b662 => {
                {
                    let ea: u64 = rdi.wrapping_add(0xc8u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rdx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    rdi = (0xaaaaaaabu64) & 0xffff_ffffu64;
                }
                {
                    let a = rdi as i64 as i128;
                    let b = rdx as i64 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdi;
                    let c = 35u64;
                    let r = a >> c;
                    rdi = r;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x14fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                {
                    rdx = (0x1bu64) & 0xffff_ffffu64;
                }
                {
                    let v = if cf || zf { (rdi & 0xffffffff) } else { (rdx & 0xffffffff) };
                    rdx = (v) & 0xffff_ffffu64;
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2b6a2; continue; }
                pc = 0x2b68a; continue;
            }
            0x2b68a => {
                {
                    let ea: u64 = rsi.wrapping_add(0x8u64);
                    rdi = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rcx.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 4, ((rdi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rsi;
                    rdi = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    let a = rcx;
                    let b = 0xcu64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rcx = r;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    zf = r == 0;
                    rdx = (r) & 0xffff_ffffu64;
                }
                if !zf { pc = 0x2b68a; continue; }
                pc = 0x2b69e; continue;
            }
            0x2b69e => {
                pc = 0x2b6a2; continue;
            }
            0x2b6a0 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x2b6a2; continue;
            }
            0x2b6a2 => {
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

/// lifted from the machine code of @0x2dc4a (size=82); checked against the P1 vectors
pub fn f_0002dc4a(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0x2dc4a;
    loop {
        match pc {
            0x2dc4a => {
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
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2dc8d; continue; }
                pc = 0x2dc5b; continue;
            }
            0x2dc5b => {
                {
                    let ea: u64 = rsi;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rsi = rdi;
                }
                pc = 0x2dc60; continue;
            }
            0x2dc60 => {
                {
                    rax = rdx;
                }
                pc = 0x2dc63; continue;
            }
            0x2dc63 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x2dc73; continue; }
                pc = 0x2dc68; continue;
            }
            0x2dc68 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2dc63; continue; }
                pc = 0x2dc71; continue;
            }
            0x2dc71 => {
                pc = 0x2dc80; continue;
            }
            0x2dc73 => {
                {
                    let ea: u64 = rax;
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                {
                    rsi = rax;
                }
                if !zf { pc = 0x2dc60; continue; }
                pc = 0x2dc7e; continue;
            }
            0x2dc7e => {
                pc = 0x2dc83; continue;
            }
            0x2dc80 => {
                {
                    rax = rsi;
                }
                pc = 0x2dc83; continue;
            }
            0x2dc83 => {
                {
                    let a = rax;
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2dc8d; continue; }
                pc = 0x2dc88; continue;
            }
            0x2dc88 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    let a = (rcx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x2dc90; continue; }
                pc = 0x2dc8d; continue;
            }
            0x2dc8d => {
                {
                    rax = rdi;
                }
                pc = 0x2dc90; continue;
            }
            0x2dc90 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            0x2dc92 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x32360 (size=82); checked against the P1 vectors
pub fn f_00032360(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x32360;
    loop {
        match pc {
            0x32360 => {
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
                {
                    let ea: u64 = rdi.wrapping_add(0x38u64);
                    stm(m, &mut stk, ea, 4, (0x12u64) & 0xffffffffu64);
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

/// lifted from the machine code of @0x32d9a (size=82); checked against the P1 vectors
pub fn f_00032d9a(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x32d9a;
    loop {
        match pc {
            0x32d9a => {
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
                {
                    let ea: u64 = rdi.wrapping_add(0x38u64);
                    stm(m, &mut stk, ea, 4, (0x12u64) & 0xffffffffu64);
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

/// lifted from the machine code of @0x2a25e (size=84); checked against the P1 vectors
pub fn f_0002a25e(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x2a25e;
    loop {
        match pc {
            0x2a25e => {
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
                    stm(m, &mut stk, ea, 4, (0x12u64) & 0xffffffffu64);
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
                    stm(m, &mut stk, ea, 4, (0x8u64) & 0xffffffffu64);
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

/// lifted from the machine code of @0x3230c (size=84); checked against the P1 vectors
pub fn f_0003230c(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32, p_3: u32, p_4: u32) {
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
    let mut pc: u64 = 0x3230c;
    loop {
        match pc {
            0x3230c => {
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
                    stm(m, &mut stk, ea, 2, (0x1u64) & 0xffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x38u64);
                    stm(m, &mut stk, ea, 4, (0x12u64) & 0xffffffffu64);
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    stm(m, &mut stk, ea, 4, ((rcx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x30u64);
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
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

/// lifted from the machine code of @0xe5ea8 (size=84); checked against the P1 vectors
pub fn f_000e5ea8(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0xe5ea8;
    loop {
        match pc {
            0xe5ea8 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
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
                    let a = (rax & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe5ed1; continue; }
                pc = 0xe5eba; continue;
            }
            0xe5eba => {
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe5ee2; continue; }
                pc = 0xe5ec4; continue;
            }
            0xe5ec4 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x8000000u64;
                    let r = (a | b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                pc = 0xe5efa; continue;
            }
            0xe5ed1 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xe5eef; continue; }
                pc = 0xe5edb; continue;
            }
            0xe5edb => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x8000000u64;
                    let r = (a | b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5ef4; continue;
            }
            0xe5ee2 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xf7ffffffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                pc = 0xe5efa; continue;
            }
            0xe5eef => {
                {
                    let a = (rax & 0xffffffff);
                    let b = 0xf7ffffffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe5ef4; continue;
            }
            0xe5ef4 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
                }
                pc = 0xe5efa; continue;
            }
            0xe5efa => {
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

/// lifted from the machine code of @0x2cd04 (size=88); checked against the P1 vectors
pub fn f_0002cd04(m: &mut dyn Mem, p_0: u64, p_1: u32) -> (u64, u64) {
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
    let mut pc: u64 = 0x2cd04;
    loop {
        match pc {
            0x2cd04 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rdx = (0xffffffffu64) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x4cu64);
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
                if zf { pc = 0x2cd55; continue; }
                pc = 0x2cd18; continue;
            }
            0x2cd18 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x54u64);
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
                if zf { pc = 0x2cd55; continue; }
                pc = 0x2cd21; continue;
            }
            0x2cd21 => {
                {
                    let a = (rdi & 0xffffffff);
                    let c = 4u64;
                    let r = a >> c;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    rsi = ((rsi & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    rdi = ((rdi & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = (rdi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0x2cd55; continue; }
                pc = 0x2cd30; continue;
            }
            0x2cd30 => {
                {
                    let a = rsi;
                    let c = 4u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffffffffffu64;
                    rsi = r;
                }
                {
                    let ea: u64 = rcx.wrapping_add(rsi);
                    rdi = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rcx.wrapping_add(rsi).wrapping_add(0x8u64);
                    rsi = ldm(m, &stk, ea, 8);
                }
                {
                    rax = 0xffffffff00000000u64;
                }
                {
                    let a = rax;
                    let b = rdi;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let a = rdx;
                    let b = rdi;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    rdx = r;
                }
                {
                    rcx = rdx;
                }
                {
                    rdx = rsi;
                }
                pc = 0x2cd57; continue;
            }
            0x2cd55 => {
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x2cd57; continue;
            }
            0x2cd57 => {
                {
                    let a = rax;
                    let b = rcx;
                    let r = (a | b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax, rdx);
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x5c8c1 (size=89); checked against the P1 vectors
pub fn f_0005c8c1(m: &mut dyn Mem, p_0: u32, p_1: u32) -> u32 {
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
    let mut pc: u64 = 0x5c8c1;
    loop {
        match pc {
            0x5c8c1 => {
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
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5c911; continue; }
                pc = 0x5c8cb; continue;
            }
            0x5c8cb => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0x5c8d9; continue; }
                pc = 0x5c8d2; continue;
            }
            0x5c8d2 => {
                if !zf { pc = 0x5c911; continue; }
                pc = 0x5c8d4; continue;
            }
            0x5c8d4 => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x1cu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                pc = 0x5c90e; continue;
            }
            0x5c8d9 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x5c8e3; continue; }
                pc = 0x5c8de; continue;
            }
            0x5c8de => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x1du64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                pc = 0x5c90e; continue;
            }
            0x5c8e3 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1cu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0x5c8f2; continue; }
                pc = 0x5c8e8; continue;
            }
            0x5c8e8 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5c8fc; continue; }
                pc = 0x5c8ed; continue;
            }
            0x5c8ed => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x1eu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                pc = 0x5c90e; continue;
            }
            0x5c8f2 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1du64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5c906; continue; }
                pc = 0x5c8f7; continue;
            }
            0x5c8f7 => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                pc = 0x5c90e; continue;
            }
            0x5c8fc => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1cu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5c911; continue; }
                pc = 0x5c901; continue;
            }
            0x5c901 => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x1u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                pc = 0x5c90e; continue;
            }
            0x5c906 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1eu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5c911; continue; }
                pc = 0x5c90b; continue;
            }
            0x5c90b => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                pc = 0x5c90e; continue;
            }
            0x5c90e => {
                {
                    rax = (rax & !0xffu64) | (((zf) as u64) & 0xff);
                }
                pc = 0x5c911; continue;
            }
            0x5c911 => {
                {
                    let a = (rax & 0xff);
                    let b = (rax & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                {
                    let v = if !zf { (rsi & 0xffffffff) } else { (rdi & 0xffffffff) };
                    rdi = (v) & 0xffff_ffffu64;
                }
                {
                    rax = ((rdi & 0xffffffff)) & 0xffff_ffffu64;
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
