#![allow(unused_assignments, unused_variables, unused_mut, unused_parens, unreachable_code, unused_imports, clippy::all)]
use crate::lift_rt::*;
use crate::Mem;

/// lifted from the machine code of @0x2f83a (size=18); checked against the P1 vectors
pub fn f_0002f83a(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0x2f83a;
    loop {
        match pc {
            0x2f83a => {
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
                    let r = (a & b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10710850u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                {
                    rax = (rax & !0xffu64) | (((zf) as u64) & 0xff);
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

/// lifted from the machine code of @0x46b56 (size=18); checked against the P1 vectors
pub fn f_00046b56(m: &mut dyn Mem, p_0: u64) -> u8 {
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
    let mut pc: u64 = 0x46b56;
    loop {
        match pc {
            0x46b56 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0xau64);
                    stm(m, &mut stk, ea, 1, (0x0u64) & 0xffu64);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 2, (0x0u64) & 0xffffu64);
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

/// lifted from the machine code of @0xd5e12 (size=18); checked against the P1 vectors
pub fn f_000d5e12(m: &mut dyn Mem) -> u64 {
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
    let mut pc: u64 = 0xd5e12;
    loop {
        match pc {
            0xd5e12 => {
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
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return rax;
            }
            0xd5e1a => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x28c32 (size=20); checked against the P1 vectors
pub fn f_00028c32(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x28c32;
    loop {
        match pc {
            0x28c32 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = 0x6e74763200000000u64;
                }
                {
                    let ea: u64 = rdi;
                    stm(m, &mut stk, ea, 8, (rax) & 0xffffffffffffffffu64);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x28c45 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x318a0 (size=20); checked against the P1 vectors
pub fn f_000318a0(m: &mut dyn Mem, p_0: u64, p_1: u32) -> u8 {
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
    let mut pc: u64 = 0x318a0;
    loop {
        match pc {
            0x318a0 => {
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
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x318b0; continue; }
                pc = 0x318a9; continue;
            }
            0x318a9 => {
                {
                    let ea: u64 = rdi.wrapping_add(0xcu64);
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0x318b2; continue;
            }
            0x318b0 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x318b2; continue;
            }
            0x318b2 => {
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

/// lifted from the machine code of @0x318c8 (size=20); checked against the P1 vectors
pub fn f_000318c8(m: &mut dyn Mem, p_0: u64, p_1: u32) -> u8 {
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
    let mut pc: u64 = 0x318c8;
    loop {
        match pc {
            0x318c8 => {
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
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x318d8; continue; }
                pc = 0x318d1; continue;
            }
            0x318d1 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x14u64);
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0x318da; continue;
            }
            0x318d8 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x318da; continue;
            }
            0x318da => {
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

/// lifted from the machine code of @0x318f0 (size=20); checked against the P1 vectors
pub fn f_000318f0(m: &mut dyn Mem, p_0: u64, p_1: u32) -> u8 {
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
    let mut pc: u64 = 0x318f0;
    loop {
        match pc {
            0x318f0 => {
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
                    let b = 0x1u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x31900; continue; }
                pc = 0x318f9; continue;
            }
            0x318f9 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x1cu64);
                    stm(m, &mut stk, ea, 4, ((rsi & 0xffffffff)) & 0xffffffffu64);
                }
                {
                    rax = (rax & !0xffu64) | ((0x1u64) & 0xff);
                }
                pc = 0x31902; continue;
            }
            0x31900 => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x31902; continue;
            }
            0x31902 => {
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

/// lifted from the machine code of @0x46d26 (size=20); checked against the P1 vectors
pub fn f_00046d26(m: &mut dyn Mem, p_0: u64) -> u8 {
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
    let mut pc: u64 = 0x46d26;
    loop {
        match pc {
            0x46d26 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x240u64);
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
            0x46d39 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xb9698 (size=20); checked against the P1 vectors
pub fn f_000b9698(m: &mut dyn Mem, p_0: u64) -> u32 {
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
    let mut pc: u64 = 0xb9698;
    loop {
        match pc {
            0xb9698 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x24cu64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xffffffff) as u32;
            }
            0xb96a4 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xe1366 (size=20); checked against the P1 vectors
pub fn f_000e1366(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0xe1366;
    loop {
        match pc {
            0xe1366 => {
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
                    stm(m, &mut stk, ea, 4, (0x0u64) & 0xffffffffu64);
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

/// lifted from the machine code of @0x2ab8c (size=22); checked against the P1 vectors
pub fn f_0002ab8c(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x2ab8c;
    loop {
        match pc {
            0x2ab8c => {
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
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x2aba1 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x2abb8 (size=22); checked against the P1 vectors
pub fn f_0002abb8(m: &mut dyn Mem, p_0: u64) {
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
    let mut pc: u64 = 0x2abb8;
    loop {
        match pc {
            0x2abb8 => {
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
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return ();
            }
            0x2abcd => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0xe55dc (size=22); checked against the P1 vectors
pub fn f_000e55dc(m: &mut dyn Mem, p_0: u64, p_1: u32) -> u8 {
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
    let mut pc: u64 = 0xe55dc;
    loop {
        match pc {
            0xe55dc => {
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
                if !zf { pc = 0xe55e7; continue; }
                pc = 0xe55e4; continue;
            }
            0xe55e4 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x8u64);
                    rsi = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0xe55e7; continue;
            }
            0xe55e7 => {
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0xfffffffbu64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rsi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rsi & 0xffffffff);
                    let b = 0x4u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                {
                    rax = (rax & !0xffu64) | (((cf) as u64) & 0xff);
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

/// lifted from the machine code of @0xb8d3c (size=24); checked against the P1 vectors
pub fn f_000b8d3c(m: &mut dyn Mem, p_0: u64) -> u64 {
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
    rdx = p_0;
    let mut pc: u64 = 0xb8d3c;
    loop {
        match pc {
            0xb8d3c => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    let ea: u64 = rdx.wrapping_add(0x8u64);
                    stm(m, &mut stk, ea, 8, (0xffffffffffffffffu64) & 0xffffffffffffffffu64);
                }
                {
                    let ea: u64 = rdx;
                    stm(m, &mut stk, ea, 8, (0xffffffffffffffffu64) & 0xffffffffffffffffu64);
                }
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
                return rax;
            }
            0xb8d53 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x1369c (size=26); checked against the P1 vectors
pub fn f_0001369c(m: &mut dyn Mem, p_0: u64) -> u32 {
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
    let mut pc: u64 = 0x1369c;
    loop {
        match pc {
            0x1369c => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (0xffffffffu64) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2e8u64);
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x136b4; continue; }
                pc = 0x136ae; continue;
            }
            0x136ae => {
                {
                    let ea: u64 = rdi.wrapping_add(0x2e0u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0x136b4; continue;
            }
            0x136b4 => {
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

/// lifted from the machine code of @0x136b6 (size=26); checked against the P1 vectors
pub fn f_000136b6(m: &mut dyn Mem, p_0: u64) -> u32 {
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
    let mut pc: u64 = 0x136b6;
    loop {
        match pc {
            0x136b6 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (0xffffffffu64) & 0xffff_ffffu64;
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x2e8u64);
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x136ce; continue; }
                pc = 0x136c8; continue;
            }
            0x136c8 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x2e4u64);
                    rax = (ldm(m, &stk, ea, 4)) & 0xffff_ffffu64;
                }
                pc = 0x136ce; continue;
            }
            0x136ce => {
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

/// lifted from the machine code of @0x316ba (size=26); checked against the P1 vectors
pub fn f_000316ba(m: &mut dyn Mem, p_0: u64) -> u8 {
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
    let mut pc: u64 = 0x316ba;
    loop {
        match pc {
            0x316ba => {
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
                    rax = (rax & !0xffu64) | ((ldm(m, &stk, ea, 1)) & 0xff);
                }
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x316c3 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x5cc06 (size=26); checked against the P1 vectors
pub fn f_0005cc06(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0x5cc06;
    loop {
        match pc {
            0x5cc06 => {
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
                    rcx = (rdi.wrapping_sub(0x75u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x5u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x5cc1e; continue; }
                pc = 0x5cc14; continue;
            }
            0x5cc14 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xffffffeeu64;
                    let r = a.wrapping_add(b) & 0xffffffffu64;
                    rdi = (r) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x3u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0x5cc1e; continue; }
                pc = 0x5cc1c; continue;
            }
            0x5cc1c => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x5cc1e; continue;
            }
            0x5cc1e => {
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

/// lifted from the machine code of @0x5a8dc (size=27); checked against the P1 vectors
pub fn f_0005a8dc(m: &mut dyn Mem, p_0: u32) -> u32 {
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
    let mut pc: u64 = 0x5a8dc;
    loop {
        match pc {
            0x5a8dc => {
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
                    let b = 0x10u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5a8f5; continue; }
                pc = 0x5a8e7; continue;
            }
            0x5a8e7 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x11u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5a8f3; continue; }
                pc = 0x5a8ec; continue;
            }
            0x5a8ec => {
                {
                    rax = (0xau64) & 0xffff_ffffu64;
                }
                pc = 0x5a8f5; continue;
            }
            0x5a8f3 => {
                {
                    rax = ((rdi & 0xffffffff)) & 0xffff_ffffu64;
                }
                pc = 0x5a8f5; continue;
            }
            0x5a8f5 => {
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

/// lifted from the machine code of @0x2e598 (size=28); checked against the P1 vectors
pub fn f_0002e598(m: &mut dyn Mem, p_0: u64) -> u8 {
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
    let mut pc: u64 = 0x2e598;
    loop {
        match pc {
            0x2e598 => {
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
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x0u64;
                    let r = a.wrapping_sub(b) & 0xffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x2e5af; continue; }
                pc = 0x2e5a2; continue;
            }
            0x2e5a2 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x18u64);
                    rax = ldm(m, &stk, ea, 8);
                }
                {
                    let ea: u64 = rdi.wrapping_add(0x10u64);
                    let a = rax;
                    let b = ldm(m, &stk, ea, 8);
                    let r = a.wrapping_sub(b) & 0xffffffffffffffffu64;
                    zf = r == 0;
                }
                {
                    rax = (rax & !0xffu64) | (((!zf) as u64) & 0xff);
                }
                pc = 0x2e5b1; continue;
            }
            0x2e5af => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0x2e5b1; continue;
            }
            0x2e5b1 => {
                {
                    let v = ldm(m, &stk, rsp, 8);
                    rsp = rsp.wrapping_add(8);
                    rbp = v;
                }
                return (rax & 0xff) as u8;
            }
            0x2e5b3 => {
                panic!("fell off the end of the function");
            }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x2eb9c (size=28); checked against the P1 vectors
pub fn f_0002eb9c(m: &mut dyn Mem, p_0: u64) -> u64 {
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
    let mut pc: u64 = 0x2eb9c;
    loop {
        match pc {
            0x2eb9c => {
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
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x2ebaf; continue; }
                pc = 0x2eba6; continue;
            }
            0x2eba6 => {
                {
                    let a = rdi;
                    let b = 0x48u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdi;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                pc = 0x2ebb3; continue;
            }
            0x2ebaf => {
                {
                    let ea: u64 = rdi.wrapping_add(0x58u64);
                    rdi = ldm(m, &stk, ea, 8);
                }
                pc = 0x2ebb3; continue;
            }
            0x2ebb3 => {
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
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x5c67c (size=28); checked against the P1 vectors
pub fn f_0005c67c(m: &mut dyn Mem, p_0: u32) -> u32 {
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
    let mut pc: u64 = 0x5c67c;
    loop {
        match pc {
            0x5c67c => {
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
                    let b = 0xdu64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                    zf = r == 0;
                }
                if !cf && !zf { pc = 0x5c694; continue; }
                pc = 0x5c685; continue;
            }
            0x5c685 => {
                {
                    rax = (0x3cc3u64) & 0xffff_ffffu64;
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
            0x5c694 => {
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

/// lifted from the machine code of @0xe6016 (size=28); checked against the P1 vectors
pub fn f_000e6016(m: &mut dyn Mem, p_0: u64) -> u64 {
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
    let mut pc: u64 = 0xe6016;
    loop {
        match pc {
            0xe6016 => {
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
                    let a = ldm(m, &stk, ea, 1);
                    let b = 0x1u64;
                    let r = (a & b) & 0xffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0xe6029; continue; }
                pc = 0xe6020; continue;
            }
            0xe6020 => {
                {
                    let a = rdi;
                    let b = 0x20u64;
                    let r = a.wrapping_add(b) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                {
                    let a = rdi;
                    let r = a.wrapping_add(1) & 0xffffffffffffffffu64;
                    rdi = r;
                }
                pc = 0xe602d; continue;
            }
            0xe6029 => {
                {
                    let ea: u64 = rdi.wrapping_add(0x30u64);
                    rdi = ldm(m, &stk, ea, 8);
                }
                pc = 0xe602d; continue;
            }
            0xe602d => {
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
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// lifted from the machine code of @0x5a8f7 (size=29); checked against the P1 vectors
pub fn f_0005a8f7(m: &mut dyn Mem, p_0: u32) -> u32 {
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
    let mut pc: u64 = 0x5a8f7;
    loop {
        match pc {
            0x5a8f7 => {
                {
                    let v = rbp;
                    rsp = rsp.wrapping_sub(8);
                    stm(m, &mut stk, rsp, 8, v);
                }
                {
                    rbp = rsp;
                }
                {
                    rax = (0x10u64) & 0xffff_ffffu64;
                }
                {
                    let a = (rdi & 0xffffffff);
                    let b = (rdi & 0xffffffff);
                    let r = (a & b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0x5a912; continue; }
                pc = 0x5a904; continue;
            }
            0x5a904 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0xau64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if !zf { pc = 0x5a910; continue; }
                pc = 0x5a909; continue;
            }
            0x5a909 => {
                {
                    rax = (0x11u64) & 0xffff_ffffu64;
                }
                pc = 0x5a912; continue;
            }
            0x5a910 => {
                {
                    rax = ((rdi & 0xffffffff)) & 0xffff_ffffu64;
                }
                pc = 0x5a912; continue;
            }
            0x5a912 => {
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

/// lifted from the machine code of @0xce412 (size=29); checked against the P1 vectors
pub fn f_000ce412(m: &mut dyn Mem, p_0: u32) -> u8 {
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
    let mut pc: u64 = 0xce412;
    loop {
        match pc {
            0xce412 => {
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
                    rcx = (rdi.wrapping_sub(0x10710850u64)) & 0xffff_ffffu64;
                }
                {
                    let a = (rcx & 0xffffffff);
                    let b = 0x2u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    cf = a < b;
                }
                if cf { pc = 0xce42d; continue; }
                pc = 0xce423; continue;
            }
            0xce423 => {
                {
                    let a = (rdi & 0xffffffff);
                    let b = 0x10710800u64;
                    let r = a.wrapping_sub(b) & 0xffffffffu64;
                    zf = r == 0;
                }
                if zf { pc = 0xce42d; continue; }
                pc = 0xce42b; continue;
            }
            0xce42b => {
                {
                    let a = (rax & 0xffffffff);
                    let b = (rax & 0xffffffff);
                    let r = (a ^ b) & 0xffffffffu64;
                    rax = (r) & 0xffff_ffffu64;
                }
                pc = 0xce42d; continue;
            }
            0xce42d => {
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
