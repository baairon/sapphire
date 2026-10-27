#![allow(unused_assignments, unused_variables, unused_mut, unused_parens, unreachable_code, unused_imports, clippy::all)]
use crate::lift_rt::*;
use crate::Mem;

/// lifted from the machine code of @0x25bb4 (size=90); checked against the P1 vectors
pub fn f_00025bb4(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0x25bb4;
    loop {
        match pc {
            0x25bb4 => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x25bf2; continue; }
                pc = 0x25bc1; continue;
            }
            0x25bc1 => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0x25bc8; continue;
            }
            0x25bc5 => {
                {
                    rax = rdx;
                }
                pc = 0x25bc8; continue;
            }
            0x25bc8 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x25bd9; continue; }
                pc = 0x25bcf; continue;
            }
            0x25bcf => {
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
                if !zf { pc = 0x25bc5; continue; }
                pc = 0x25bd7; continue;
            }
            0x25bd7 => {
                pc = 0x25bfe; continue;
            }
            0x25bd9 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x25c06; continue; }
                pc = 0x25bdd; continue;
            }
            0x25bdd => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x25bc5; continue; }
                pc = 0x25be6; continue;
            }
            0x25be6 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0x25c09; continue;
            }
            0x25bf2 => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0x25c09; continue;
            }
            0x25bfe => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0x25c09; continue;
            }
            0x25c06 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x25c09; continue;
            }
            0x25c09 => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0x2d8ec (size=90); checked against the P1 vectors
pub fn f_0002d8ec(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0x2d8ec;
    loop {
        match pc {
            0x2d8ec => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2d92a; continue; }
                pc = 0x2d8f9; continue;
            }
            0x2d8f9 => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0x2d900; continue;
            }
            0x2d8fd => {
                {
                    rax = rdx;
                }
                pc = 0x2d900; continue;
            }
            0x2d900 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x2d911; continue; }
                pc = 0x2d907; continue;
            }
            0x2d907 => {
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
                if !zf { pc = 0x2d8fd; continue; }
                pc = 0x2d90f; continue;
            }
            0x2d90f => {
                pc = 0x2d936; continue;
            }
            0x2d911 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x2d93e; continue; }
                pc = 0x2d915; continue;
            }
            0x2d915 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2d8fd; continue; }
                pc = 0x2d91e; continue;
            }
            0x2d91e => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0x2d941; continue;
            }
            0x2d92a => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0x2d941; continue;
            }
            0x2d936 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0x2d941; continue;
            }
            0x2d93e => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x2d941; continue;
            }
            0x2d941 => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0x655d6 (size=90); checked against the P1 vectors
pub fn f_000655d6(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0x655d6;
    loop {
        match pc {
            0x655d6 => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x65614; continue; }
                pc = 0x655e3; continue;
            }
            0x655e3 => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0x655ea; continue;
            }
            0x655e7 => {
                {
                    rax = rdx;
                }
                pc = 0x655ea; continue;
            }
            0x655ea => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x655fb; continue; }
                pc = 0x655f1; continue;
            }
            0x655f1 => {
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
                if !zf { pc = 0x655e7; continue; }
                pc = 0x655f9; continue;
            }
            0x655f9 => {
                pc = 0x65620; continue;
            }
            0x655fb => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x65628; continue; }
                pc = 0x655ff; continue;
            }
            0x655ff => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x655e7; continue; }
                pc = 0x65608; continue;
            }
            0x65608 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0x6562b; continue;
            }
            0x65614 => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0x6562b; continue;
            }
            0x65620 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0x6562b; continue;
            }
            0x65628 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x6562b; continue;
            }
            0x6562b => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0xc4bca (size=90); checked against the P1 vectors
pub fn f_000c4bca(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0xc4bca;
    loop {
        match pc {
            0xc4bca => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc4c08; continue; }
                pc = 0xc4bd7; continue;
            }
            0xc4bd7 => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0xc4bde; continue;
            }
            0xc4bdb => {
                {
                    rax = rdx;
                }
                pc = 0xc4bde; continue;
            }
            0xc4bde => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc4bef; continue; }
                pc = 0xc4be5; continue;
            }
            0xc4be5 => {
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
                if !zf { pc = 0xc4bdb; continue; }
                pc = 0xc4bed; continue;
            }
            0xc4bed => {
                pc = 0xc4c14; continue;
            }
            0xc4bef => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc4c1c; continue; }
                pc = 0xc4bf3; continue;
            }
            0xc4bf3 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xc4bdb; continue; }
                pc = 0xc4bfc; continue;
            }
            0xc4bfc => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0xc4c1f; continue;
            }
            0xc4c08 => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0xc4c1f; continue;
            }
            0xc4c14 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0xc4c1f; continue;
            }
            0xc4c1c => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0xc4c1f; continue;
            }
            0xc4c1f => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0xc5076 (size=90); checked against the P1 vectors
pub fn f_000c5076(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0xc5076;
    loop {
        match pc {
            0xc5076 => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc50b4; continue; }
                pc = 0xc5083; continue;
            }
            0xc5083 => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0xc508a; continue;
            }
            0xc5087 => {
                {
                    rax = rdx;
                }
                pc = 0xc508a; continue;
            }
            0xc508a => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc509b; continue; }
                pc = 0xc5091; continue;
            }
            0xc5091 => {
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
                if !zf { pc = 0xc5087; continue; }
                pc = 0xc5099; continue;
            }
            0xc5099 => {
                pc = 0xc50c0; continue;
            }
            0xc509b => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc50c8; continue; }
                pc = 0xc509f; continue;
            }
            0xc509f => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xc5087; continue; }
                pc = 0xc50a8; continue;
            }
            0xc50a8 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0xc50cb; continue;
            }
            0xc50b4 => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0xc50cb; continue;
            }
            0xc50c0 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0xc50cb; continue;
            }
            0xc50c8 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0xc50cb; continue;
            }
            0xc50cb => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0xc5820 (size=90); checked against the P1 vectors
pub fn f_000c5820(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0xc5820;
    loop {
        match pc {
            0xc5820 => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc585e; continue; }
                pc = 0xc582d; continue;
            }
            0xc582d => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0xc5834; continue;
            }
            0xc5831 => {
                {
                    rax = rdx;
                }
                pc = 0xc5834; continue;
            }
            0xc5834 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc5845; continue; }
                pc = 0xc583b; continue;
            }
            0xc583b => {
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
                if !zf { pc = 0xc5831; continue; }
                pc = 0xc5843; continue;
            }
            0xc5843 => {
                pc = 0xc586a; continue;
            }
            0xc5845 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc5872; continue; }
                pc = 0xc5849; continue;
            }
            0xc5849 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xc5831; continue; }
                pc = 0xc5852; continue;
            }
            0xc5852 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0xc5875; continue;
            }
            0xc585e => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0xc5875; continue;
            }
            0xc586a => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0xc5875; continue;
            }
            0xc5872 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0xc5875; continue;
            }
            0xc5875 => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0x2da88 (size=98); checked against the P1 vectors
pub fn f_0002da88(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0x2da88;
    loop {
        match pc {
            0x2da88 => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2dac6; continue; }
                pc = 0x2da95; continue;
            }
            0x2da95 => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0x2da9c; continue;
            }
            0x2da99 => {
                {
                    rax = rdx;
                }
                pc = 0x2da9c; continue;
            }
            0x2da9c => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x2daad; continue; }
                pc = 0x2daa3; continue;
            }
            0x2daa3 => {
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
                if !zf { pc = 0x2da99; continue; }
                pc = 0x2daab; continue;
            }
            0x2daab => {
                pc = 0x2dad2; continue;
            }
            0x2daad => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0x2dada; continue; }
                pc = 0x2dab1; continue;
            }
            0x2dab1 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2da99; continue; }
                pc = 0x2daba; continue;
            }
            0x2daba => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0x2dadd; continue;
            }
            0x2dac6 => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0x2dadd; continue;
            }
            0x2dad2 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0x2dadd; continue;
            }
            0x2dada => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x2dadd; continue;
            }
            0x2dadd => {
                {
                    rax = rsi;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            0x2dae2 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xc53ec (size=90); checked against the P1 vectors
pub fn f_000c53ec(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0xc53ec;
    loop {
        match pc {
            0xc53ec => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xc542a; continue; }
                pc = 0xc53f9; continue;
            }
            0xc53f9 => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0xc5400; continue;
            }
            0xc53fd => {
                {
                    rax = rdx;
                }
                pc = 0xc5400; continue;
            }
            0xc5400 => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc5411; continue; }
                pc = 0xc5407; continue;
            }
            0xc5407 => {
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
                if !zf { pc = 0xc53fd; continue; }
                pc = 0xc540f; continue;
            }
            0xc540f => {
                pc = 0xc5436; continue;
            }
            0xc5411 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xc543e; continue; }
                pc = 0xc5415; continue;
            }
            0xc5415 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xc53fd; continue; }
                pc = 0xc541e; continue;
            }
            0xc541e => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0xc5441; continue;
            }
            0xc542a => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0xc5441; continue;
            }
            0xc5436 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0xc5441; continue;
            }
            0xc543e => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0xc5441; continue;
            }
            0xc5441 => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0xd650c (size=90); checked against the P1 vectors
pub fn f_000d650c(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0xd650c;
    loop {
        match pc {
            0xd650c => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xd654a; continue; }
                pc = 0xd6519; continue;
            }
            0xd6519 => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0xd6520; continue;
            }
            0xd651d => {
                {
                    rax = rdx;
                }
                pc = 0xd6520; continue;
            }
            0xd6520 => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xd6531; continue; }
                pc = 0xd6527; continue;
            }
            0xd6527 => {
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
                if !zf { pc = 0xd651d; continue; }
                pc = 0xd652f; continue;
            }
            0xd652f => {
                pc = 0xd6556; continue;
            }
            0xd6531 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if sf == ov { pc = 0xd655e; continue; }
                pc = 0xd6535; continue;
            }
            0xd6535 => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xd651d; continue; }
                pc = 0xd653e; continue;
            }
            0xd653e => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0xd6561; continue;
            }
            0xd654a => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0xd6561; continue;
            }
            0xd6556 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0xd6561; continue;
            }
            0xd655e => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0xd6561; continue;
            }
            0xd6561 => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0x322b2 (size=90); checked against the P1 vectors
pub fn f_000322b2(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0x322b2;
    loop {
        match pc {
            0x322b2 => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x322f0; continue; }
                pc = 0x322bf; continue;
            }
            0x322bf => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0x322c6; continue;
            }
            0x322c3 => {
                {
                    rax = rdx;
                }
                pc = 0x322c6; continue;
            }
            0x322c6 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0x322d7; continue; }
                pc = 0x322cd; continue;
            }
            0x322cd => {
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
                if !zf { pc = 0x322c3; continue; }
                pc = 0x322d5; continue;
            }
            0x322d5 => {
                pc = 0x322fc; continue;
            }
            0x322d7 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0x32304; continue; }
                pc = 0x322db; continue;
            }
            0x322db => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x322c3; continue; }
                pc = 0x322e4; continue;
            }
            0x322e4 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0x32307; continue;
            }
            0x322f0 => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0x32307; continue;
            }
            0x322fc => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0x32307; continue;
            }
            0x32304 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0x32307; continue;
            }
            0x32307 => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0xb5432 (size=90); checked against the P1 vectors
pub fn f_000b5432(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0xb5432;
    loop {
        match pc {
            0xb5432 => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xb5470; continue; }
                pc = 0xb543f; continue;
            }
            0xb543f => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0xb5446; continue;
            }
            0xb5443 => {
                {
                    rax = rdx;
                }
                pc = 0xb5446; continue;
            }
            0xb5446 => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb5457; continue; }
                pc = 0xb544d; continue;
            }
            0xb544d => {
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
                if !zf { pc = 0xb5443; continue; }
                pc = 0xb5455; continue;
            }
            0xb5455 => {
                pc = 0xb547c; continue;
            }
            0xb5457 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb5484; continue; }
                pc = 0xb545b; continue;
            }
            0xb545b => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdx = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdx;
                    let b = rdx;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xb5443; continue; }
                pc = 0xb5464; continue;
            }
            0xb5464 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0xb5487; continue;
            }
            0xb5470 => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0xb5487; continue;
            }
            0xb547c => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0xb5487; continue;
            }
            0xb5484 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0xb5487; continue;
            }
            0xb5487 => {
                {
                    rax = rsi;
                }
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

/// lifted from the machine code of @0xb5846 (size=104); checked against the P1 vectors
pub fn f_000b5846(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u64) -> u64 {
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
    let mut pc: u64 = 0xb5846;
    loop {
        match pc {
            0xb5846 => {
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
                    let a = rax;
                    let b = rax;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xb5891; continue; }
                pc = 0xb5853; continue;
            }
            0xb5853 => {
                {
                    let ea: u64 = rdx;
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x4u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0xb585d; continue;
            }
            0xb585a => {
                {
                    rax = rdi;
                }
                pc = 0xb585d; continue;
            }
            0xb585d => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rdi = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rdi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xb586d; continue; }
                pc = 0xb5864; continue;
            }
            0xb5864 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xb587c; continue; }
                pc = 0xb5868; continue;
            }
            0xb5868 => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = (rdx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb5877; continue; }
                pc = 0xb586d; continue;
            }
            0xb586d => {
                {
                    let ea: u64 = rax;
                    rdi = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = rdi;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xb585a; continue; }
                pc = 0xb5875; continue;
            }
            0xb5875 => {
                pc = 0xb589d; continue;
            }
            0xb5877 => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb58a5; continue; }
                pc = 0xb587c; continue;
            }
            0xb587c => {
                {
                    let ea: u64 = rax.wrapping_add(0x8u64);
                    rdi = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = rdi;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xb585a; continue; }
                pc = 0xb5885; continue;
            }
            0xb5885 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let a = rax;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rax = r;
                }
                {
                    rsi = rax;
                }
                pc = 0xb58a8; continue;
            }
            0xb5891 => {
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rdi) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rdi;
                }
                pc = 0xb58a8; continue;
            }
            0xb589d => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    rsi = rax;
                }
                pc = 0xb58a8; continue;
            }
            0xb58a5 => {
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                pc = 0xb58a8; continue;
            }
            0xb58a8 => {
                {
                    rax = rsi;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            0xb58ad => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xb57e4 (size=98); checked against the P1 vectors
pub fn f_000b57e4(m: &mut dyn Mem, p_0: u64, p_1: u64) -> u64 {
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
    let mut pc: u64 = 0xb57e4;
    loop {
        match pc {
            0xb57e4 => {
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
                    r8 = ldm(m, &stk, ea, 8);
                }
                {
                    let a = rdi;
                    let b = 0x8u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = r8;
                    let b = r8;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xb5840; continue; }
                pc = 0xb57f5; continue;
            }
            0xb57f5 => {
                {
                    let ea: u64 = rsi;
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(0x4u64);
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rax = rdi;
                }
                pc = 0xb57fd; continue;
            }
            0xb57fd => {
                {
                    r9 = rax;
                }
                {
                    rax = r8;
                }
                pc = 0xb5803; continue;
            }
            0xb5803 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rsi = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xb5813; continue; }
                pc = 0xb580a; continue;
            }
            0xb580a => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xb581e; continue; }
                pc = 0xb580e; continue;
            }
            0xb580e => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = ldm(m, &stk, ea, 4);
                    let b = (rcx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb581e; continue; }
                pc = 0xb5813; continue;
            }
            0xb5813 => {
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
                if !zf { pc = 0xb5803; continue; }
                pc = 0xb581c; continue;
            }
            0xb581c => {
                pc = 0xb5828; continue;
            }
            0xb581e => {
                {
                    let ea: u64 = rax;
                    r8 = ldm(m, &stk, ea, 8);
                }
                {
                    let a = r8;
                    let b = r8;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xb57fd; continue; }
                pc = 0xb5826; continue;
            }
            0xb5826 => {
                pc = 0xb582b; continue;
            }
            0xb5828 => {
                {
                    rax = r9;
                }
                pc = 0xb582b; continue;
            }
            0xb582b => {
                {
                    let a = rax;
                    let b = rdi;
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xb5840; continue; }
                pc = 0xb5830; continue;
            }
            0xb5830 => {
                {
                    let ea: u64 = rax.wrapping_add(0x1cu64);
                    rsi = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xb5840; continue; }
                pc = 0xb5837; continue;
            }
            0xb5837 => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xb5843; continue; }
                pc = 0xb583b; continue;
            }
            0xb583b => {
                {
                    let ea: u64 = rax.wrapping_add(0x20u64);
                    let a = (rcx & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if !cf { pc = 0xb5843; continue; }
                pc = 0xb5840; continue;
            }
            0xb5840 => {
                {
                    rax = rdi;
                }
                pc = 0xb5843; continue;
            }
            0xb5843 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            0xb5845 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x587f6 (size=93); checked against the P1 vectors
pub fn f_000587f6(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x587f6;
    loop {
        match pc {
            0x587f6 => {
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
                if zf { pc = 0x58851; continue; }
                pc = 0x587fe; continue;
            }
            0x587fe => {
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                pc = 0x58804; continue;
            }
            0x58804 => {
                {
                    rax = ((r8 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(4));
                    r11 = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    r9 = ((r11 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = ((r10 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((r9 & 0xffff)) & 0xffffu64);
                }
                {
                    rcx = ((r11 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 10u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r10.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rax.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((rcx & 0xffff)) & 0xffffu64);
                }
                {
                    let a = (r11 & 0xffffffff);
                    let c = 20u64;
                    let r = a >> c;
                    r11 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    r11 = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r10.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rax.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((r11 & 0xffff)) & 0xffffu64);
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let r = a.wrapping_add(1) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x58804; continue; }
                pc = 0x58851; continue;
            }
            0x58851 => {
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

/// lifted from the machine code of @0x58fd6 (size=93); checked against the P1 vectors
pub fn f_00058fd6(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) {
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
    let mut pc: u64 = 0x58fd6;
    loop {
        match pc {
            0x58fd6 => {
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
                if zf { pc = 0x59031; continue; }
                pc = 0x58fde; continue;
            }
            0x58fde => {
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                pc = 0x58fe4; continue;
            }
            0x58fe4 => {
                {
                    rax = ((r8 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax.wrapping_mul(4));
                    r11 = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    r9 = ((r11 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    rcx = ((r10 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rcx.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((r9 & 0xffff)) & 0xffffu64);
                }
                {
                    rcx = ((r11 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let c = 10u64;
                    let r = a >> c;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r10.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rax.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((rcx & 0xffff)) & 0xffffu64);
                }
                {
                    let a = (r11 & 0xffffffff);
                    let c = 20u64;
                    let r = a >> c;
                    r11 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff);
                    let b = 0x3ffu64;
                    let r = (a & b) & 0xffffffffu64;
                    r11 = (r) & 0xffff_ffffu64;
                }
                {
                    rax = (r10.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi.wrapping_add(rax.wrapping_mul(2));
                    stm(m, &mut stk, ea, 2, ((r11 & 0xffff)) & 0xffffu64);
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let r = a.wrapping_add(1) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x58fe4; continue; }
                pc = 0x59031; continue;
            }
            0x59031 => {
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

/// lifted from the machine code of @0x65914 (size=99); checked against the P1 vectors
pub fn f_00065914(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32) -> u8 {
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
    let mut pc: u64 = 0x65914;
    loop {
        match pc {
            0x65914 => {
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
                    let a = rdi;
                    let b = rdi;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x65975; continue; }
                pc = 0x6591f; continue;
            }
            0x6591f => {
                {
                    let a = rsi;
                    let b = rsi;
                    let r = (a & b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x65975; continue; }
                pc = 0x65924; continue;
            }
            0x65924 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x65975; continue; }
                pc = 0x65928; continue;
            }
            0x65928 => {
                {
                    let ea: u64 = rdi;
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    r8 = ((rax & 0xff)) & 0xffff_ffffu64;
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
                    let b = 0x3f00u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    r8 = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let c = 14u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0xc00000u64;
                    let r = (a & b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    r8 = (r) & 0xffff_ffffu64;
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
                    let b = 0xf0000u64;
                    let r = (a & b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
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
                    let b = 0xf0000000u64;
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
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, ((rax & 0xffffffff)) & 0xffffffffu64);
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
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                if !zf { pc = 0x65928; continue; }
                pc = 0x65975; continue;
            }
            0x65975 => {
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

/// lifted from the machine code of @0x2602c (size=100); checked against the P1 vectors
pub fn f_0002602c(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32) {
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
    let mut pc: u64 = 0x2602c;
    loop {
        match pc {
            0x2602c => {
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
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x2608f => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x26090 (size=100); checked against the P1 vectors
pub fn f_00026090(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32) {
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
    let mut pc: u64 = 0x26090;
    loop {
        match pc {
            0x26090 => {
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
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x260f3 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xe644e (size=104); checked against the P1 vectors
pub fn f_000e644e(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32) -> u8 {
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
    let mut pc: u64 = 0xe644e;
    loop {
        match pc {
            0xe644e => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (rsi.wrapping_sub(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x7u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0xe648a; continue; }
                pc = 0xe645a; continue;
            }
            0xe645a => {
                {
                    rcx = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x60u64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0xe648e; continue; }
                pc = 0xe6465; continue;
            }
            0xe6465 => {
                {
                    rcx = (rax.wrapping_mul(8).wrapping_add(0x4u64)) & 0xffff_ffffu64;
                }
                {
                    rax = (0xfu64) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = (rcx & 31u64);
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    rax = (!a) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    let a = (rax & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0xfu64;
                    let r = (a & b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let c = (rcx & 31u64);
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x84u64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                pc = 0xe64b1; continue;
            }
            0xe648a => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xe64b3; continue;
            }
            0xe648e => {
                {
                    rcx = (rsi.wrapping_mul(8).wrapping_sub(0x24u64)) & 0xffff_ffffu64;
                }
                {
                    rax = (0xfu64) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = (rcx & 31u64);
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    rax = (!a) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    let a = (rax & 0xffffffff);
                    let b = ldm(m, &stk, ea, 4);
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0xfu64;
                    let r = (a & b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let c = (rcx & 31u64);
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x88u64);
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                pc = 0xe64b1; continue;
            }
            0xe64b1 => {
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0xe64b3; continue;
            }
            0xe64b3 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0xe64b5 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x5a594 (size=108); checked against the P1 vectors
pub fn f_0005a594(m: &mut dyn Mem, p_0: u32, p_1: u32) -> u32 {
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
    let mut pc: u64 = 0x5a594;
    loop {
        match pc {
            0x5a594 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (0xcu64) & 0xffff_ffffu64;
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
                if !cf && !zf { pc = 0x5a5d7; continue; }
                pc = 0x5a5a4; continue;
            }
            0x5a5a4 => {
                {
                    rcx = 0x1005a5dcu64;
                }
                {
                    let ea: u64 = rcx.wrapping_add(rdi.wrapping_mul(4));
                    rdx = ((ldm(m, &stk, ea, 4)) as i32 as i64 as u64);
                }
                {
                    let a = rdx;
                    let b = rcx;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdx = r;
                }
                pc = rdx.wrapping_sub(0x1000_0000u64); continue;
            }
            0x5a5b4 => {
                {
                    rax = ((rsi & 0xff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5a5d7; continue;
            }
            0x5a5bd => {
                {
                    rax = (0x2u64) & 0xffff_ffffu64;
                }
                pc = 0x5a5d7; continue;
            }
            0x5a5c4 => {
                {
                    rax = (0x3u64) & 0xffff_ffffu64;
                }
                pc = 0x5a5d7; continue;
            }
            0x5a5cb => {
                {
                    rax = (0x1u64) & 0xffff_ffffu64;
                }
                pc = 0x5a5d7; continue;
            }
            0x5a5d2 => {
                {
                    rax = (0x5u64) & 0xffff_ffffu64;
                }
                pc = 0x5a5d7; continue;
            }
            0x5a5d7 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xffffffff) as u32;
            }
            0x5a5d9 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x59541 (size=109); checked against the P1 vectors
pub fn f_00059541(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32, p_3: u32, p_4: u32, p_5: u32) {
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
    r9 = p_5 as u64;
    let mut pc: u64 = 0x59541;
    loop {
        match pc {
            0x59541 => {
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
                    r8 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = 0x1u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5957e; continue; }
                pc = 0x5954e; continue;
            }
            0x5954e => {
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x595ac; continue; }
                pc = 0x59553; continue;
            }
            0x59553 => {
                {
                    let a = (r9 & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                pc = 0x59556; continue;
            }
            0x59556 => {
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
                if cf { pc = 0x59556; continue; }
                pc = 0x5957c; continue;
            }
            0x5957c => {
                pc = 0x595ac; continue;
            }
            0x5957e => {
                {
                    let a = (r8 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x595ac; continue; }
                pc = 0x59583; continue;
            }
            0x59583 => {
                {
                    let a = (r9 & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                pc = 0x59586; continue;
            }
            0x59586 => {
                {
                    rax = ((r9 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax);
                    stm(m, &mut stk, ea, 1, ((rsi & 0xff)) & 0xffu64);
                }
                {
                    rax = (r9.wrapping_add(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax);
                    stm(m, &mut stk, ea, 1, ((rdx & 0xff)) & 0xffu64);
                }
                {
                    rax = (r9.wrapping_add(0x2u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax);
                    stm(m, &mut stk, ea, 1, ((rsi & 0xff)) & 0xffu64);
                }
                {
                    rax = (r9.wrapping_add(0x3u64)) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rax);
                    stm(m, &mut stk, ea, 1, ((rcx & 0xff)) & 0xffu64);
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
                if cf { pc = 0x59586; continue; }
                pc = 0x595ac; continue;
            }
            0x595ac => {
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

/// lifted from the machine code of @0xcc285 (size=109); checked against the P1 vectors
pub fn f_000cc285(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0xcc285;
    loop {
        match pc {
            0xcc285 => {
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
                    let b = 0x106466ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcc2a5; continue; }
                pc = 0xcc293; continue;
            }
            0xcc293 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x1032294fu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcc2b2; continue; }
                pc = 0xcc29b; continue;
            }
            0xcc29b => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10294700u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcc2ee; continue; }
                pc = 0xcc2a3; continue;
            }
            0xcc2a3 => {
                pc = 0xcc2f0; continue;
            }
            0xcc2a5 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xef9b9900u64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x7u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xcc2f0; continue; }
                pc = 0xcc2b0; continue;
            }
            0xcc2b0 => {
                pc = 0xcc2ee; continue;
            }
            0xcc2b2 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x103787ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcc2c4; continue; }
                pc = 0xcc2ba; continue;
            }
            0xcc2ba => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10322950u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcc2ee; continue; }
                pc = 0xcc2c2; continue;
            }
            0xcc2c2 => {
                pc = 0xcc2f0; continue;
            }
            0xcc2c4 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x105183ffu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    ov = ((a ^ b) & (a ^ r)) >> 31 & 1 != 0;
                    zf = r == 0;
                    sf = (r >> 31) & 1 != 0;
                }
                if !zf && sf == ov { pc = 0xcc2de; continue; }
                pc = 0xcc2cc; continue;
            }
            0xcc2cc => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10378800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcc2f0; continue; }
                pc = 0xcc2d4; continue;
            }
            0xcc2d4 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10478350u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xcc2ee; continue; }
                pc = 0xcc2dc; continue;
            }
            0xcc2dc => {
                pc = 0xcc2f0; continue;
            }
            0xcc2de => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518400u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcc2f0; continue; }
                pc = 0xcc2e6; continue;
            }
            0xcc2e6 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10518450u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xcc2f0; continue; }
                pc = 0xcc2ee; continue;
            }
            0xcc2ee => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xcc2f0; continue;
            }
            0xcc2f0 => {
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

/// lifted from the machine code of @0x3314e (size=110); checked against the P1 vectors
pub fn f_0003314e(m: &mut dyn Mem, p_0: u64, p_1: u32, p_2: u32) -> u32 {
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
    let mut pc: u64 = 0x3314e;
    loop {
        match pc {
            0x3314e => {
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
                    rcx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    rax = (0xffffffffu64) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if cf || zf { pc = 0x331ba; continue; }
                pc = 0x3315d; continue;
            }
            0x3315d => {
                {
                    let ea: u64 = rdi.wrapping_add(0x34u64);
                    r9 = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    r8 = ((rdx & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = (r8 & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if cf || zf { pc = 0x331ba; continue; }
                pc = 0x3316b; continue;
            }
            0x3316b => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffff);
                    let b = (rdx & 0xffff);
                    let r = (a & b) & 0xffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x331b0; continue; }
                pc = 0x33172; continue;
            }
            0x33172 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffff);
                    let b = (r9 & 0xffff);
                    let r = (a & b) & 0xffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x33180; continue; }
                pc = 0x3317a; continue;
            }
            0x3317a => {
                {
                    let ea: u64 = rdi.wrapping_add(0x24u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff) as i32 as i128;
                    let b = (rcx & 0xffffffff) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x33180; continue;
            }
            0x33180 => {
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x331b0; continue; }
                pc = 0x33186; continue;
            }
            0x33186 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    r9 = ((r9 & 0xffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x33198; continue; }
                pc = 0x33192; continue;
            }
            0x33192 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x28u64);
                    rdx = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff) as i32 as i128;
                    let b = (rcx & 0xffffffff) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                pc = 0x33198; continue;
            }
            0x33198 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r8 & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x331b0; continue; }
                pc = 0x331a0; continue;
            }
            0x331a0 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x331ae; continue; }
                pc = 0x331a8; continue;
            }
            0x331a8 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x2cu64);
                    let a = (rcx & 0xffffffff) as i32 as i128;
                    let b = ldm(m, &stk, ea, 4) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                {
                    rdx = ((rcx & 0xffffffff)) & 0xffff_ffffu64;
                }
                pc = 0x331ae; continue;
            }
            0x331ae => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x331b0; continue;
            }
            0x331b0 => {
                {
                    let ea: u64 = rdi.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x24u64);
                    let a = (rsi & 0xffffffff) as i32 as i128;
                    let b = ldm(m, &stk, ea, 4) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    rsi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rsi = (r) & 0xffff_ffffu64;
                }
                {
                    rax = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                pc = 0x331ba; continue;
            }
            0x331ba => {
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

/// lifted from the machine code of @0x676f9 (size=111); checked against the P1 vectors
pub fn f_000676f9(m: &mut dyn Mem, p_0: u64, p_1: u32) {
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
    let mut pc: u64 = 0x676f9;
    loop {
        match pc {
            0x676f9 => {
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
                if zf { pc = 0x67766; continue; }
                pc = 0x67701; continue;
            }
            0x67701 => {
                {
                    r8 = ((rsi & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = (rcx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rcx = (r) & 0xffff_ffffu64;
                }
                pc = 0x67706; continue;
            }
            0x67706 => {
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x4u64);
                    rdx = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    r9 = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let c = 14u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r9 & 0xffffffff);
                    let b = 0xff0000u64;
                    let r = (a & b) & 0xffffffffu64;
                    r9 = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x2u64);
                    r11 = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    r10 = ((r11 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let c = 6u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = 0xff00u64;
                    let r = (a & b) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(8));
                    rax = (ldm(m, &stk, ea, 2)) & 0xffff_ffffu64;
                }
                {
                    rsi = ((rax & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let c = 2u64;
                    let r = a >> c;
                    rsi = (r) & 0xffff_ffffu64;
                }
                {
                    rsi = ((rsi & 0xff)) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = 0x3u64;
                    let r = (a & b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let c = 28u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff);
                    let b = 0x3u64;
                    let r = (a & b) & 0xffffffffu64;
                    r11 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r11 & 0xffffffff);
                    let c = 26u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    r11 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let b = 0x3u64;
                    let r = (a & b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rax & 0xffffffff);
                    let c = 24u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (r11 & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rsi & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a | b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(rcx.wrapping_mul(4));
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
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
                if !zf { pc = 0x67706; continue; }
                pc = 0x67766; continue;
            }
            0x67766 => {
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

/// lifted from the machine code of @0x5c039 (size=112); checked against the P1 vectors
pub fn f_0005c039(m: &mut dyn Mem, p_0: u64, p_1: u64, p_2: u32, p_3: u32, p_4: u32, p_5: u32) -> u32 {
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
    r9 = p_5 as u64;
    let mut pc: u64 = 0x5c039;
    loop {
        match pc {
            0x5c039 => {
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
                    r10 = ((rdx & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5c097; continue; }
                pc = 0x5c04a; continue;
            }
            0x5c04a => {
                {
                    r14 = (r9.wrapping_sub(0x1u64)) & 0xffff_ffffu64;
                }
                {
                    let a = r14;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    r14 = r;
                }
                {
                    let ea: u64 = rsi;
                    r11 = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let a = (r15 & 0xffffffff);
                    let b = (r15 & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    r15 = (r) & 0xffff_ffffu64;
                }
                pc = 0x5c057; continue;
            }
            0x5c057 => {
                {
                    let a = (rdx & 0xffffffff);
                    let b = (rdx & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                {
                    rax = ((r11 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    let b = (rcx & 0xffffffff) as u32 as u128;
                    if b == 0 { panic!("divide error"); }
                    let n = ((((rdx & 0xffff_ffff) << 32) | (rax & 0xffff_ffff)) as u64) as u128;
                    let q = n / b;
                    let rem = n % b;
                    if (q >> 32) != 0 { panic!("divide overflow"); }
                    rax = (q as u64) & 0xffff_ffff;
                    rdx = (rem as u64) & 0xffff_ffff;
                }
                {
                    let a = (r8 & 0xff);
                    let b = (r8 & 0xff);
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5c06a; continue; }
                pc = 0x5c063; continue;
            }
            0x5c063 => {
                {
                    rdx = (((rdx & 0xffffffff) as u32).swap_bytes() as u64) & 0xffff_ffffu64;
                }
                {
                    let a = (rdx & 0xffffffff);
                    let c = 16u64;
                    let r = a >> c;
                    rdx = (r) & 0xffff_ffffu64;
                }
                pc = 0x5c06d; continue;
            }
            0x5c06a => {
                {
                    let a = (rdx & 0xffffffff);
                    let c = 16u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    rdx = (r) & 0xffff_ffffu64;
                }
                pc = 0x5c06d; continue;
            }
            0x5c06d => {
                {
                    let a = (r9 & 0xffffffff);
                    let b = (r9 & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                {
                    rax = ((r9 & 0xffffffff)) & 0xffff_ffffu64;
                }
                {
                    rbx = rdi;
                }
                if zf { pc = 0x5c089; continue; }
                pc = 0x5c078; continue;
            }
            0x5c078 => {
                {
                    let ea: u64 = rbx;
                    stm(m, &mut stk, ea, 4, ((rdx & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = rbx;
                    let b = 0x4u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rbx = r;
                }
                {
                    let a = (rax & 0xffffffff);
                    let r = a.wrapping_sub(1) & 0xffffffffu64;
                    zf = r == 0;
                    rax = (r) & 0xffff_ffffu64;
                }
                if !zf { pc = 0x5c078; continue; }
                pc = 0x5c082; continue;
            }
            0x5c082 => {
                {
                    rdi = rdi.wrapping_add(r14.wrapping_mul(4));
                }
                {
                    let ea: u64 = rsi;
                    r11 = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0x5c089; continue;
            }
            0x5c089 => {
                {
                    let a = (r11 & 0xffffffff);
                    let r = a.wrapping_add(1) & 0xffffffffu64;
                    r11 = (r) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rsi;
                    stm(m, &mut stk, ea, 4, ((r11 & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    let a = (r15 & 0xffffffff);
                    let r = a.wrapping_add(1) & 0xffffffffu64;
                    r15 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r15 & 0xffffffff);
                    let b = (r10 & 0xffffffff);
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5c057; continue; }
                pc = 0x5c097; continue;
            }
            0x5c097 => {
                {
                    let a = (r10 & 0xffffffff) as i32 as i128;
                    let b = (r9 & 0xffffffff) as i32 as i128;
                    let p = a * b;
                    let r = (p as u64) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (r10 & 0xffffffff);
                    let c = 2u64;
                    let wide = (a as u128) << c;
                    let r = (wide as u64) & 0xffffffffu64;
                    r10 = (r) & 0xffff_ffffu64;
                }
                {
                    rax = ((r10 & 0xffffffff)) & 0xffff_ffffu64;
                }
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
                return (rax & 0xffffffff) as u32;
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
