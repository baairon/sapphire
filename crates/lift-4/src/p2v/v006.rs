//! Mechanically lifted functions (scripts/lift_run.py), module v006. Each one is checked against the original
//! in the world harness (tests/world_v006.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x2b24 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002b24<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2b24;
    loop {
        match pc {
        0x2b24 => {
            // block 2b24
            #[cfg(feature = "trace")] eprintln!("f_00002b24 pc 2b24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b24 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b25 mov rbp,rsp
            rbp = rsp;
            // 2b28 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2b2a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2b2c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b2d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2b2e mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 2b31 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2b34 mov rbx,rdi
            rbx = rdi;
            // 2b37 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2b3a call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b40 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2b42 je 0x2b48
            if fl.e() { pc = 0x2b48; continue; }
            pc = 0x2b44;
        }
        0x2b44 => {
            // block 2b44
            #[cfg(feature = "trace")] eprintln!("f_00002b24 pc 2b44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b44 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2b46 jmp 0x2b69
            pc = 0x2b69; continue;
        }
        0x2b48 => {
            // block 2b48
            #[cfg(feature = "trace")] eprintln!("f_00002b24 pc 2b48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b48 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2b4b mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 2b4e lea rdx,[rip+0xfc58b] # 0xff0e0
            rdx = m.env.data_va(80045);
            // 2b55 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2b58 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2b5d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2b60 mov rdi,rbx
            rdi = rbx;
            // 2b63 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 2b66 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2b69;
        }
        0x2b69 => {
            // block 2b69
            #[cfg(feature = "trace")] eprintln!("f_00002b24 pc 2b69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b69 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2b6d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2b6e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2b70 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2b72 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2b73 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2b74 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002b74<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2b74;
    loop {
        match pc {
        0x2b74 => {
            // block 2b74
            #[cfg(feature = "trace")] eprintln!("f_00002b74 pc 2b74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b74 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2b75 mov rbp,rsp
            rbp = rsp;
            // 2b78 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2b7a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2b7c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2b7d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2b7e mov r14,rdx
            r14 = rdx;
            // 2b81 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2b84 mov rbx,rdi
            rbx = rdi;
            // 2b87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2b8a call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2b90 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2b92 je 0x2b98
            if fl.e() { pc = 0x2b98; continue; }
            pc = 0x2b94;
        }
        0x2b94 => {
            // block 2b94
            #[cfg(feature = "trace")] eprintln!("f_00002b74 pc 2b94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b94 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2b96 jmp 0x2bb9
            pc = 0x2bb9; continue;
        }
        0x2b98 => {
            // block 2b98
            #[cfg(feature = "trace")] eprintln!("f_00002b74 pc 2b98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2b98 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2b9b mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 2b9e lea rdx,[rip+0xfc53b] # 0xff0e0
            rdx = m.env.data_va(80045);
            // 2ba5 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2ba8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2bad xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2bb0 mov rdi,rbx
            rdi = rbx;
            // 2bb3 mov rdx,r14
            rdx = r14;
            // 2bb6 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2bb9;
        }
        0x2bb9 => {
            // block 2bb9
            #[cfg(feature = "trace")] eprintln!("f_00002b74 pc 2bb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb9 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2bbd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bbe pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bc0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bc2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bc3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2c14 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002c14<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2c14;
    loop {
        match pc {
        0x2c14 => {
            // block 2c14
            #[cfg(feature = "trace")] eprintln!("f_00002c14 pc 2c14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c14 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2c15 mov rbp,rsp
            rbp = rsp;
            // 2c18 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2c1a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2c1c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2c1d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2c1e mov r14,rdx
            r14 = rdx;
            // 2c21 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 2c24 mov rbx,rdi
            rbx = rdi;
            // 2c27 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2c2a call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2c30 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2c32 je 0x2c38
            if fl.e() { pc = 0x2c38; continue; }
            pc = 0x2c34;
        }
        0x2c34 => {
            // block 2c34
            #[cfg(feature = "trace")] eprintln!("f_00002c14 pc 2c34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c34 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2c36 jmp 0x2c59
            pc = 0x2c59; continue;
        }
        0x2c38 => {
            // block 2c38
            #[cfg(feature = "trace")] eprintln!("f_00002c14 pc 2c38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c38 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2c3b mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 2c3e lea rdx,[rip+0xfc4cb] # 0xff110
            rdx = m.env.data_va(80093);
            // 2c45 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2c48 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2c4d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2c50 mov rdi,rbx
            rdi = rbx;
            // 2c53 mov rdx,r14
            rdx = r14;
            // 2c56 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2c59;
        }
        0x2c59 => {
            // block 2c59
            #[cfg(feature = "trace")] eprintln!("f_00002c14 pc 2c59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2c59 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2c5d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2c5e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2c60 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2c62 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2c63 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x698c size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000698c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x698c;
    loop {
        match pc {
        0x698c => {
            // block 698c
            #[cfg(feature = "trace")] eprintln!("f_0000698c pc 698c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 698c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 698d mov rbp,rsp
            rbp = rsp;
            // 6990 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6992 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6994 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6995 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6996 mov r14,rdx
            r14 = rdx;
            // 6999 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 699c mov rbx,rdi
            rbx = rdi;
            // 699f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 69a2 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 69a8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 69aa je 0x69b0
            if fl.e() { pc = 0x69b0; continue; }
            pc = 0x69ac;
        }
        0x69ac => {
            // block 69ac
            #[cfg(feature = "trace")] eprintln!("f_0000698c pc 69ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69ac xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 69ae jmp 0x69d0
            pc = 0x69d0; continue;
        }
        0x69b0 => {
            // block 69b0
            #[cfg(feature = "trace")] eprintln!("f_0000698c pc 69b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69b0 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 69b3 test r15d,r15d
            fl = logic::<32>((r15 as u32 as u64) & (r15 as u32 as u64));
            // 69b6 setne cl
            rcx = (rcx & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 69b9 movzx esi,cl
            rsi = ((rcx as u8 as u64)) as u32 as u64;
            // 69bc or esi,0x44
            { let r = (rsi as u32 as u64) | 0x44u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 69bf mov ecx,0x3ff
            rcx = (0x3ffu64) as u32 as u64;
            // 69c4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 69c7 mov rdi,rbx
            rdi = rbx;
            // 69ca mov rdx,r14
            rdx = r14;
            // 69cd call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x69d0;
        }
        0x69d0 => {
            // block 69d0
            #[cfg(feature = "trace")] eprintln!("f_0000698c pc 69d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 69d0 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 69d4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 69d5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 69d7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 69d9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 69da ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x116d4 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000116d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x116d4;
    loop {
        match pc {
        0x116d4 => {
            // block 116d4
            #[cfg(feature = "trace")] eprintln!("f_000116d4 pc 116d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 116d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 116d5 mov rbp,rsp
            rbp = rsp;
            // 116d8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 116da push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 116db mov r14,rdi
            r14 = rdi;
            // 116de mov rbx,QWORD PTR [rsi+0x30]
            rbx = m.r64(rsi.wrapping_add(0x30)) as u64;
            // 116e2 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 116e5 je 0x1171d
            if fl.e() { pc = 0x1171d; continue; }
            pc = 0x116e7;
        }
        0x116e7 => {
            // block 116e7
            #[cfg(feature = "trace")] eprintln!("f_000116d4 pc 116e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 116e7 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 116ea mov rdi,r14
            rdi = r14;
            // 116ed call QWORD PTR [rax+0x80]
            { let r = { let t = m.r64(rax.wrapping_add(0x80)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 116f3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 116f6 mov rdi,rbx
            rdi = rbx;
            // 116f9 call QWORD PTR [rax+0x20]
            { let r = { let t = m.r64(rax.wrapping_add(0x20)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 116fc mov ebx,eax
            rbx = ((rax as u32 as u64)) as u32 as u64;
            // 116fe mov rdi,QWORD PTR [r14+0x40]
            rdi = m.r64(r14.wrapping_add(0x40)) as u64;
            // 11702 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 11705 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11708 mov rdi,QWORD PTR [r14+0x40]
            rdi = m.r64(r14.wrapping_add(0x40)) as u64;
            // 1170c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1170f mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 11713 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 11715 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 11717 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11718 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1171a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1171b jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x1171d => {
            // block 1171d
            #[cfg(feature = "trace")] eprintln!("f_000116d4 pc 1171d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1171d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 1171f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11720 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11722 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11723 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25c0e size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025c0e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x25c0e;
    loop {
        match pc {
        0x25c0e => {
            // block 25c0e
            #[cfg(feature = "trace")] eprintln!("f_00025c0e pc 25c0e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25c0e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25c0f mov rbp,rsp
            rbp = rsp;
            // 25c12 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25c13 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 25c14 mov rbx,rdi
            rbx = rdi;
            // 25c17 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 25c1f mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 25c26 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 25c2a mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 25c2d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 25c30 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 25c34 je 0x25c3f
            if fl.e() { pc = 0x25c3f; continue; }
            pc = 0x25c36;
        }
        0x25c36 => {
            // block 25c36
            #[cfg(feature = "trace")] eprintln!("f_00025c0e pc 25c36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25c36 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 25c39 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 25c3c mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x25c3f;
        }
        0x25c3f => {
            // block 25c3f
            #[cfg(feature = "trace")] eprintln!("f_00025c0e pc 25c3f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25c3f mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 25c43 mov rsi,rcx
            rsi = rcx;
            // 25c46 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25c4b inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 25c4f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 25c53 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25c54 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25c55 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42d2c size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042d2c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42d2c;
    loop {
        match pc {
        0x42d2c => {
            // block 42d2c
            #[cfg(feature = "trace")] eprintln!("f_00042d2c pc 42d2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d2c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42d2d mov rbp,rsp
            rbp = rsp;
            // 42d30 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42d32 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42d34 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42d35 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42d36 mov r14,rdx
            r14 = rdx;
            // 42d39 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 42d3c mov rbx,rdi
            rbx = rdi;
            // 42d3f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42d42 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42d48 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 42d4a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 42d4c cmp r15d,0x1
            fl = sub::<32>((r15 as u32 as u64), 0x1u64).1;
            // 42d50 jg 0x42d70
            if fl.g() { pc = 0x42d70; continue; }
            pc = 0x42d52;
        }
        0x42d52 => {
            // block 42d52
            #[cfg(feature = "trace")] eprintln!("f_00042d2c pc 42d52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d52 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 42d55 jne 0x42d70
            if fl.ne() { pc = 0x42d70; continue; }
            pc = 0x42d57;
        }
        0x42d57 => {
            // block 42d57
            #[cfg(feature = "trace")] eprintln!("f_00042d2c pc 42d57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d57 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42d5a mov esi,0x902
            rsi = (0x902u64) as u32 as u64;
            // 42d5f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42d64 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42d67 mov rdi,rbx
            rdi = rbx;
            // 42d6a mov rdx,r14
            rdx = r14;
            // 42d6d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x42d70;
        }
        0x42d70 => {
            // block 42d70
            #[cfg(feature = "trace")] eprintln!("f_00042d2c pc 42d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42d70 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42d74 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42d75 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42d77 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42d79 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42d7a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc441e size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c441e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc441e;
    loop {
        match pc {
        0xc441e => {
            // block c441e
            #[cfg(feature = "trace")] eprintln!("f_000c441e pc c441e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c441e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c441f mov rbp,rsp
            rbp = rsp;
            // c4422 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c4423 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c4424 mov rbx,rdi
            rbx = rdi;
            // c4427 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // c442f mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // c4436 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // c443a mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // c443d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c4440 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // c4444 je 0xc444f
            if fl.e() { pc = 0xc444f; continue; }
            pc = 0xc4446;
        }
        0xc4446 => {
            // block c4446
            #[cfg(feature = "trace")] eprintln!("f_000c441e pc c4446 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c4446 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // c4449 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // c444c mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xc444f;
        }
        0xc444f => {
            // block c444f
            #[cfg(feature = "trace")] eprintln!("f_000c441e pc c444f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c444f mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // c4453 mov rsi,rcx
            rsi = rcx;
            // c4456 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c445b inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // c445f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c4463 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c4464 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c4465 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4854 size=80
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4854<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4854;
    loop {
        match pc {
        0xe4854 => {
            // block e4854
            #[cfg(feature = "trace")] eprintln!("f_000e4854 pc e4854 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4854 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4855 mov rbp,rsp
            rbp = rsp;
            // e4858 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // e485c mov r9,QWORD PTR [rdi]
            r9 = m.r64(rdi) as u64;
            // e485f mov rax,QWORD PTR [r9]
            rax = m.r64(r9) as u64;
            // e4862 mov r10,QWORD PTR [rax+0xca8]
            r10 = m.r64(rax.wrapping_add(0xca8)) as u64;
            // e4869 mov eax,DWORD PTR [rdi+0x28]
            rax = (m.r32(rdi.wrapping_add(0x28)) as u64) as u32 as u64;
            // e486c mov edi,DWORD PTR [rdi+0x2c]
            rdi = (m.r32(rdi.wrapping_add(0x2c)) as u64) as u32 as u64;
            // e486f mov DWORD PTR [rbp-0x10],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff0), ((rax as u32 as u64)) as u32);
            // e4872 mov DWORD PTR [rbp-0xc],edi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), ((rdi as u32 as u64)) as u32);
            // e4875 mov DWORD PTR [rbp-0x8],0xffffffff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff8), (0xffffffffu64) as u32);
            // e487c mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // e4883 mov DWORD PTR [rbp-0x20],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), ((rsi as u32 as u64)) as u32);
            // e4886 mov DWORD PTR [rbp-0x1c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rdx as u32 as u64)) as u32);
            // e4889 mov DWORD PTR [rbp-0x18],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), ((rcx as u32 as u64)) as u32);
            // e488c mov DWORD PTR [rbp-0x14],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((r8 as u32 as u64)) as u32);
            // e4890 lea rsi,[rbp-0x10]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // e4894 lea rdx,[rbp-0x20]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe0);
            // e4898 mov rdi,r9
            rdi = r9;
            // e489b call r10
            { let r = { let t = r10; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e489e add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // e48a2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e48a3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11140 size=85
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011140<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11140;
    loop {
        match pc {
        0x11140 => {
            // block 11140
            #[cfg(feature = "trace")] eprintln!("f_00011140 pc 11140 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11140 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11141 mov rbp,rsp
            rbp = rsp;
            // 11144 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11145 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11146 mov rbx,rdi
            rbx = rdi;
            // 11149 mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // 11151 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // 11158 mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // 1115c mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // 1115f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 11162 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // 11166 je 0x11171
            if fl.e() { pc = 0x11171; continue; }
            pc = 0x11168;
        }
        0x11168 => {
            // block 11168
            #[cfg(feature = "trace")] eprintln!("f_00011140 pc 11168 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11168 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 1116b mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1116e mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0x11171;
        }
        0x11171 => {
            // block 11171
            #[cfg(feature = "trace")] eprintln!("f_00011140 pc 11171 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11171 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 11175 mov rsi,rcx
            rsi = rcx;
            // 11178 call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1117d inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // 11181 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 11185 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11186 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11187 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x75f8 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000075f8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x75f8;
    loop {
        match pc {
        0x75f8 => {
            // block 75f8
            #[cfg(feature = "trace")] eprintln!("f_000075f8 pc 75f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 75f8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 75f9 mov rbp,rsp
            rbp = rsp;
            // 75fc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 75fe push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7600 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7601 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 7602 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 7605 mov r14,rsi
            r14 = rsi;
            // 7608 mov rbx,rdi
            rbx = rdi;
            // 760b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 760e mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 7611 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 7617 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7619 je 0x761f
            if fl.e() { pc = 0x761f; continue; }
            pc = 0x761b;
        }
        0x761b => {
            // block 761b
            #[cfg(feature = "trace")] eprintln!("f_000075f8 pc 761b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 761b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 761d jmp 0x7643
            pc = 0x7643; continue;
        }
        0x761f => {
            // block 761f
            #[cfg(feature = "trace")] eprintln!("f_000075f8 pc 761f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 761f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 7622 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 7625 lea rdx,[rip+0xf7e54] # 0xff480
            rdx = m.env.data_va(80973);
            // 762c mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 762f mov ecx,0x40000000
            rcx = (0x40000000u64) as u32 as u64;
            // 7634 mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            // 763a mov rdi,rbx
            rdi = rbx;
            // 763d mov rdx,r14
            rdx = r14;
            // 7640 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x7643;
        }
        0x7643 => {
            // block 7643
            #[cfg(feature = "trace")] eprintln!("f_000075f8 pc 7643 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7643 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 7647 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7648 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 764a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 764c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 764d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x9822 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00009822<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x9822;
    loop {
        match pc {
        0x9822 => {
            // block 9822
            #[cfg(feature = "trace")] eprintln!("f_00009822 pc 9822 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9822 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 9823 mov rbp,rsp
            rbp = rsp;
            // 9826 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9828 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 982a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 982b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 982c mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 982f mov r14,rsi
            r14 = rsi;
            // 9832 mov rbx,rdi
            rbx = rdi;
            // 9835 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 9838 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 983b call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9841 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9843 je 0x9849
            if fl.e() { pc = 0x9849; continue; }
            pc = 0x9845;
        }
        0x9845 => {
            // block 9845
            #[cfg(feature = "trace")] eprintln!("f_00009822 pc 9845 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9845 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 9847 jmp 0x986d
            pc = 0x986d; continue;
        }
        0x9849 => {
            // block 9849
            #[cfg(feature = "trace")] eprintln!("f_00009822 pc 9849 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9849 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 984c mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 984f lea rdx,[rip+0xf5c2a] # 0xff480
            rdx = m.env.data_va(80973);
            // 9856 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 9859 mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 985e mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 9864 mov rdi,rbx
            rdi = rbx;
            // 9867 mov rdx,r14
            rdx = r14;
            // 986a call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x986d;
        }
        0x986d => {
            // block 986d
            #[cfg(feature = "trace")] eprintln!("f_00009822 pc 986d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 986d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 9871 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9872 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9874 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 9876 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9877 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x98ce size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000098ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x98ce;
    loop {
        match pc {
        0x98ce => {
            // block 98ce
            #[cfg(feature = "trace")] eprintln!("f_000098ce pc 98ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 98cf mov rbp,rsp
            rbp = rsp;
            // 98d2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 98d4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 98d6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 98d7 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 98d8 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 98db mov r14,rsi
            r14 = rsi;
            // 98de mov rbx,rdi
            rbx = rdi;
            // 98e1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 98e4 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 98e7 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 98ed test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 98ef je 0x98f5
            if fl.e() { pc = 0x98f5; continue; }
            pc = 0x98f1;
        }
        0x98f1 => {
            // block 98f1
            #[cfg(feature = "trace")] eprintln!("f_000098ce pc 98f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f1 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 98f3 jmp 0x9919
            pc = 0x9919; continue;
        }
        0x98f5 => {
            // block 98f5
            #[cfg(feature = "trace")] eprintln!("f_000098ce pc 98f5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 98f5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 98f8 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 98fb lea rdx,[rip+0xf5b7e] # 0xff480
            rdx = m.env.data_va(80973);
            // 9902 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 9905 mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 990a mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 9910 mov rdi,rbx
            rdi = rbx;
            // 9913 mov rdx,r14
            rdx = r14;
            // 9916 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x9919;
        }
        0x9919 => {
            // block 9919
            #[cfg(feature = "trace")] eprintln!("f_000098ce pc 9919 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9919 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 991d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 991e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9920 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 9922 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9923 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e1ca size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e1ca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e1ca;
    loop {
        match pc {
        0x2e1ca => {
            // block 2e1ca
            #[cfg(feature = "trace")] eprintln!("f_0002e1ca pc 2e1ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e1ca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e1cb mov rbp,rsp
            rbp = rsp;
            // 2e1ce lea rax,[rip+0x11331b] # 0x1414f0
            rax = m.env.data_va(351421);
            // 2e1d5 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // 2e1d8 mov WORD PTR [rdi+0x8],0x0
            m.w16(rdi.wrapping_add(0x8), (0x0u64) as u16);
            // 2e1de mov QWORD PTR [rdi+0x48],0x0
            m.w64(rdi.wrapping_add(0x48), (0x0u64) as u64);
            // 2e1e6 mov QWORD PTR [rdi+0x40],0x0
            m.w64(rdi.wrapping_add(0x40), (0x0u64) as u64);
            // 2e1ee mov QWORD PTR [rdi+0x38],0x0
            m.w64(rdi.wrapping_add(0x38), (0x0u64) as u64);
            // 2e1f6 mov QWORD PTR [rdi+0x30],0x0
            m.w64(rdi.wrapping_add(0x30), (0x0u64) as u64);
            // 2e1fe mov QWORD PTR [rdi+0x28],0x0
            m.w64(rdi.wrapping_add(0x28), (0x0u64) as u64);
            // 2e206 mov QWORD PTR [rdi+0x20],0x0
            m.w64(rdi.wrapping_add(0x20), (0x0u64) as u64);
            // 2e20e mov QWORD PTR [rdi+0x18],0x0
            m.w64(rdi.wrapping_add(0x18), (0x0u64) as u64);
            // 2e216 mov QWORD PTR [rdi+0x10],0x0
            m.w64(rdi.wrapping_add(0x10), (0x0u64) as u64);
            // 2e21e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e21f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6566 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6566<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd6566;
    loop {
        match pc {
        0xd6566 => {
            // block d6566
            #[cfg(feature = "trace")] eprintln!("f_000d6566 pc d6566 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6566 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6567 mov rbp,rsp
            rbp = rsp;
            // d656a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d656b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d656c mov rbx,rdi
            rbx = rdi;
            // d656f mov QWORD PTR [rcx+0x8],0x0
            m.w64(rcx.wrapping_add(0x8), (0x0u64) as u64);
            // d6577 mov QWORD PTR [rcx],0x0
            m.w64(rcx, (0x0u64) as u64);
            // d657e mov QWORD PTR [rcx+0x10],rsi
            m.w64(rcx.wrapping_add(0x10), (rsi) as u64);
            // d6582 mov QWORD PTR [rdx],rcx
            m.w64(rdx, (rcx) as u64);
            // d6585 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d6588 cmp QWORD PTR [rax],0x0
            fl = sub::<64>(m.r64(rax) as u64, 0x0u64).1;
            // d658c je 0xd6597
            if fl.e() { pc = 0xd6597; continue; }
            pc = 0xd658e;
        }
        0xd658e => {
            // block d658e
            #[cfg(feature = "trace")] eprintln!("f_000d6566 pc d658e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d658e mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // d6591 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // d6594 mov rcx,QWORD PTR [rdx]
            rcx = m.r64(rdx) as u64;
            pc = 0xd6597;
        }
        0xd6597 => {
            // block d6597
            #[cfg(feature = "trace")] eprintln!("f_000d6566 pc d6597 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6597 mov rdi,QWORD PTR [rbx+0x8]
            rdi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // d659b mov rsi,rcx
            rsi = rcx;
            // d659e call 0xebbee
            { let r = { let t = m.env.stub_va(0x3bb); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d65a3 inc QWORD PTR [rbx+0x10]
            { let (r, f) = inc::<64>(m.r64(rbx.wrapping_add(0x10)) as u64, fl); m.w64(rbx.wrapping_add(0x10), (r) as u64); fl = f; }
            // d65a7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d65ab pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d65ac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d65ad ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1216 size=86
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e1216<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe1216;
    loop {
        match pc {
        0xe1216 => {
            // block e1216
            #[cfg(feature = "trace")] eprintln!("f_000e1216 pc e1216 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1216 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1217 mov rbp,rsp
            rbp = rsp;
            // e121a movabs rax,0xffffffff0000
            rax = 0x0_ffff_ffff_0000u64;
            // e1224 and rax,rsi
            { let r = rax & rsi; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // e1227 shr rax,0x10
            { let (r, f) = shr::<64>(rax, 0x10u64, fl); rax = r; fl = f; }
            // e122b mov rdx,rax
            rdx = rax;
            // e122e and rdx,0xffc0
            { let r = rdx & 0xffc0u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // e1235 xor ecx,ecx
            { let r = (rcx as u32 as u64) ^ (rcx as u32 as u64); let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e1237 cmp rdx,0x40
            fl = sub::<64>(rdx, 0x40u64).1;
            // e123b jb 0xe124d
            if fl.b() { pc = 0xe124d; continue; }
            pc = 0xe123d;
        }
        0xe123d => {
            // block e123d
            #[cfg(feature = "trace")] eprintln!("f_000e1216 pc e123d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e123d shl rax,0xa
            { let (r, f) = shl::<64>(rax, 0xau64, fl); rax = r; fl = f; }
            // e1241 add eax,0x3ff0000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x3ff0000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // e1246 and eax,0x3fffc00
            { let r = (rax as u32 as u64) & 0x3fffc00u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e124b mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            pc = 0xe124d;
        }
        0xe124d => {
            // block e124d
            #[cfg(feature = "trace")] eprintln!("f_000e1216 pc e124d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e124d movzx eax,si
            rax = ((rsi as u16 as u64)) as u32 as u64;
            // e1250 shr rsi,0xc
            { let (r, f) = shr::<64>(rsi, 0xcu64, fl); rsi = r; fl = f; }
            // e1254 and esi,0xfff00000
            { let r = (rsi as u32 as u64) & 0xfff00000u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e125a or esi,eax
            { let r = (rsi as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e125c or esi,ecx
            { let r = (rsi as u32 as u64) | (rcx as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e125e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1261 mov rax,QWORD PTR [rax+0x1b60]
            rax = m.r64(rax.wrapping_add(0x1b60)) as u64;
            // e1268 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1269 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x156f0 size=92
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000156f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x156f0;
    loop {
        match pc {
        0x156f0 => {
            // block 156f0
            #[cfg(feature = "trace")] eprintln!("f_000156f0 pc 156f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 156f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 156f1 mov rbp,rsp
            rbp = rsp;
            // 156f4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 156f6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 156f7 mov rbx,rdi
            rbx = rdi;
            // 156fa mov rax,QWORD PTR [rip+0x12ab1f] # 0x140220
            rax = m.r64(m.env.data_va(346605)) as u64;
            // 15701 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 15705 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 15708 lea r14,[rip+0x1ac2f9] # 0x1c1a08
            r14 = m.env.data_va(877013);
            // 1570f mov rsi,QWORD PTR [rip+0x1ac2f2] # 0x1c1a08
            rsi = m.r64(m.env.data_va(877013)) as u64;
            // 15716 lea rdi,[rip+0x1ac2e3] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 1571d call 0xeb942
            { let r = { let t = m.env.stub_va(0x10f); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 15722 mov QWORD PTR [rip+0x1ac2e3],0x0 # 0x1c1a10
            m.w64(m.env.data_va(877021), (0x0u64) as u64);
            // 1572d mov QWORD PTR [rip+0x1ac2cc],r14 # 0x1c1a00
            m.w64(m.env.data_va(877005), (r14) as u64);
            // 15734 mov QWORD PTR [rip+0x1ac2c9],0x0 # 0x1c1a08
            m.w64(m.env.data_va(877013), (0x0u64) as u64);
            // 1573f mov rdi,rbx
            rdi = rbx;
            // 15742 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 15743 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 15745 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 15746 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x27375 size=93
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00027375<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x27375;
    loop {
        match pc {
        0x27375 => {
            // block 27375
            #[cfg(feature = "trace")] eprintln!("f_00027375 pc 27375 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 27375 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 27376 mov rbp,rsp
            rbp = rsp;
            // 27379 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2737b push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2737d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2737e push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2737f mov r15,rsi
            r15 = rsi;
            // 27382 mov r14,rdi
            r14 = rdi;
            // 27385 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 27388 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 2738c jmp 0x273bf
            pc = 0x273bf; continue;
        }
        0x2738e => {
            // block 2738e
            #[cfg(feature = "trace")] eprintln!("f_00027375 pc 2738e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2738e lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 27392 mov rdi,r14
            rdi = r14;
            // 27395 call 0xebabc
            { let r = { let t = m.env.stub_va(0x289); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2739a mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 2739e test rax,rax
            fl = logic::<64>(rax & rax);
            // 273a1 je 0x273b0
            if fl.e() { pc = 0x273b0; continue; }
            pc = 0x273a3;
        }
        0x273a3 => {
            // block 273a3
            #[cfg(feature = "trace")] eprintln!("f_00027375 pc 273a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 273a3 mov rbx,rax
            rbx = rax;
            // 273a6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 273a9 test rax,rax
            fl = logic::<64>(rax & rax);
            // 273ac jne 0x273a3
            if fl.ne() { pc = 0x273a3; continue; }
            pc = 0x273ae;
        }
        0x273ae => {
            // block 273ae
            #[cfg(feature = "trace")] eprintln!("f_00027375 pc 273ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 273ae jmp 0x273bf
            pc = 0x273bf; continue;
        }
        0x273b0 => {
            // block 273b0
            #[cfg(feature = "trace")] eprintln!("f_00027375 pc 273b0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 273b0 mov rax,rbx
            rax = rbx;
            pc = 0x273b3;
        }
        0x273b3 => {
            // block 273b3
            #[cfg(feature = "trace")] eprintln!("f_00027375 pc 273b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 273b3 mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 273b7 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 273ba mov rax,rbx
            rax = rbx;
            // 273bd jne 0x273b3
            if fl.ne() { pc = 0x273b3; continue; }
            pc = 0x273bf;
        }
        0x273bf => {
            // block 273bf
            #[cfg(feature = "trace")] eprintln!("f_00027375 pc 273bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 273bf cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 273c2 jne 0x2738e
            if fl.ne() { pc = 0x2738e; continue; }
            pc = 0x273c4;
        }
        0x273c4 => {
            // block 273c4
            #[cfg(feature = "trace")] eprintln!("f_00027375 pc 273c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 273c4 mov rax,r14
            rax = r14;
            // 273c7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 273cb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 273cc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 273ce pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 273d0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 273d1 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x96fc size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000096fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x96fc;
    loop {
        match pc {
        0x96fc => {
            // block 96fc
            #[cfg(feature = "trace")] eprintln!("f_000096fc pc 96fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 96fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 96fd mov rbp,rsp
            rbp = rsp;
            // 9700 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9702 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9704 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9705 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 9706 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 9709 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 970c mov rbx,rdi
            rbx = rdi;
            // 970f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 9712 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 9715 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 971b mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 971d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 971f cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // 9723 ja 0x974e
            if fl.a() { pc = 0x974e; continue; }
            pc = 0x9725;
        }
        0x9725 => {
            // block 9725
            #[cfg(feature = "trace")] eprintln!("f_000096fc pc 9725 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9725 xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 9728 je 0x974e
            if fl.e() { pc = 0x974e; continue; }
            pc = 0x972a;
        }
        0x972a => {
            // block 972a
            #[cfg(feature = "trace")] eprintln!("f_000096fc pc 972a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 972a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 972d mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 9730 lea rdx,[rip+0xf5f99] # 0xff6d0
            rdx = m.env.data_va(81565);
            // 9737 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 973a mov ecx,0x80000000
            rcx = (0x80000000u64) as u32 as u64;
            // 973f mov r8d,0x1f
            r8 = (0x1fu64) as u32 as u64;
            // 9745 mov rdi,rbx
            rdi = rbx;
            // 9748 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 974b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x974e;
        }
        0x974e => {
            // block 974e
            #[cfg(feature = "trace")] eprintln!("f_000096fc pc 974e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 974e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 9752 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 9753 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 9755 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 9757 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 9758 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68252 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068252<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x68252;
    loop {
        match pc {
        0x68252 => {
            // block 68252
            #[cfg(feature = "trace")] eprintln!("f_00068252 pc 68252 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68252 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68253 mov rbp,rsp
            rbp = rsp;
            // 68256 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 68258 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6825a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6825b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6825c mov r15,rdi
            r15 = rdi;
            // 6825f xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 68261 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 68264 ja 0x682a4
            if fl.a() { pc = 0x682a4; continue; }
            pc = 0x68266;
        }
        0x68266 => {
            // block 68266
            #[cfg(feature = "trace")] eprintln!("f_00068252 pc 68266 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68266 test dx,dx
            fl = logic::<16>((rdx as u16 as u64) & (rdx as u16 as u64));
            // 68269 je 0x682a4
            if fl.e() { pc = 0x682a4; continue; }
            pc = 0x6826b;
        }
        0x6826b => {
            // block 6826b
            #[cfg(feature = "trace")] eprintln!("f_00068252 pc 6826b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6826b mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 6826d lea rcx,[rip+0x99a4c] # 0x101cc0
            rcx = m.env.data_va(91277);
            // 68274 mov r14d,DWORD PTR [rcx+rax*4]
            r14 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 68278 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 6827d sub ebx,edx
            { let (r, f) = sub::<32>((rbx as u32 as u64), (rdx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x6827f;
        }
        0x6827f => {
            // block 6827f
            #[cfg(feature = "trace")] eprintln!("f_00068252 pc 6827f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6827f mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 68282 mov edx,0x32
            rdx = (0x32u64) as u32 as u64;
            // 68287 mov rdi,r15
            rdi = r15;
            // 6828a mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 6828d call QWORD PTR [rax+0xc0]
            { let r = { let t = m.r64(rax.wrapping_add(0xc0)) as u64; m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68293 test bx,bx
            fl = logic::<16>((rbx as u16 as u64) & (rbx as u16 as u64));
            // 68296 je 0x682a4
            if fl.e() { pc = 0x682a4; continue; }
            pc = 0x68298;
        }
        0x68298 => {
            // block 68298
            #[cfg(feature = "trace")] eprintln!("f_00068252 pc 68298 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68298 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 6829a xor cl,0x1
            { let r = (rcx as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 6829d lea ebx,[rbx+0x1]
            rbx = (rbx.wrapping_add(0x1)) as u32 as u64;
            // 682a0 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 682a2 je 0x6827f
            if fl.e() { pc = 0x6827f; continue; }
            pc = 0x682a4;
        }
        0x682a4 => {
            // block 682a4
            #[cfg(feature = "trace")] eprintln!("f_00068252 pc 682a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 682a4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 682a8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 682a9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 682ab pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 682ad pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 682ae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5e60 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). self recursion executed natively on the shared stack (f_000d5e60_rec)
pub fn f_000d5e60<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    f_000d5e60_rec(&mut m, [0, ri[3], ri[2], SAVED_RBX, RSP0 - 8, SAVED_RBP, ri[1], ri[0], ri[4], ri[5], 0, 0, SAVED_R12, SAVED_R13, SAVED_R14, SAVED_R15], [fx[0] as u128, fx[1] as u128, fx[2] as u128, fx[3] as u128, fx[4] as u128, fx[5] as u128, fx[6] as u128, fx[7] as u128])
}

fn f_000d5e60_rec<E: Env>(m: &mut Mem<'_, E>, rin: [u64; 16], xin: [u128; 8]) -> Ret {
    let mut rax: u64 = rin[0];
    let mut rcx: u64 = rin[1];
    let mut rdx: u64 = rin[2];
    let mut rbx: u64 = rin[3];
    let mut rsp: u64 = rin[4];
    let mut rbp: u64 = rin[5];
    let mut rsi: u64 = rin[6];
    let mut rdi: u64 = rin[7];
    let mut r8: u64 = rin[8];
    let mut r9: u64 = rin[9];
    let mut r10: u64 = rin[10];
    let mut r11: u64 = rin[11];
    let mut r12: u64 = rin[12];
    let mut r13: u64 = rin[13];
    let mut r14: u64 = rin[14];
    let mut r15: u64 = rin[15];
    let mut x0: u128 = xin[0];
    let mut fl = Fl::default();
    let mut pc: u32 = 0xd5e60;
    loop {
        match pc {
        0xd5e60 => {
            // block d5e60
            #[cfg(feature = "trace")] eprintln!("f_000d5e60 pc d5e60 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5e60 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5e61 mov rbp,rsp
            rbp = rsp;
            // d5e64 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d5e66 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5e67 mov rbx,rsi
            rbx = rsi;
            // d5e6a mov r14,rdi
            r14 = rdi;
            // d5e6d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d5e70 je 0xd5e9e
            if fl.e() { pc = 0xd5e9e; continue; }
            pc = 0xd5e72;
        }
        0xd5e72 => {
            // block d5e72
            #[cfg(feature = "trace")] eprintln!("f_000d5e60 pc d5e72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5e72 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // d5e75 mov rdi,r14
            rdi = r14;
            // d5e78 call 0xd5e60
            { rsp = rsp.wrapping_sub(8); m.w64(rsp, 0x100d5e7d); let r = f_000d5e60_rec(m, [rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15], [x0, 0, 0, 0, 0, 0, 0, 0]); rsp = rsp.wrapping_add(8); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5e7d mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // d5e81 mov rdi,r14
            rdi = r14;
            // d5e84 call 0xd5e60
            { rsp = rsp.wrapping_sub(8); m.w64(rsp, 0x100d5e89); let r = f_000d5e60_rec(m, [rax, rcx, rdx, rbx, rsp, rbp, rsi, rdi, r8, r9, r10, r11, r12, r13, r14, r15], [x0, 0, 0, 0, 0, 0, 0, 0]); rsp = rsp.wrapping_add(8); rax = r.rax; rdx = r.rdx; x0 = r.xmm0; rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5e89 lea rdi,[rbx+0x28]
            rdi = rbx.wrapping_add(0x28);
            // d5e8d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5e92 mov rdi,rbx
            rdi = rbx;
            // d5e95 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5e96 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d5e98 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5e99 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xd5e9e => {
            // block d5e9e
            #[cfg(feature = "trace")] eprintln!("f_000d5e60 pc d5e9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5e9e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5e9f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d5ea1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5ea2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1652 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001652<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1652;
    loop {
        match pc {
        0x1652 => {
            // block 1652
            #[cfg(feature = "trace")] eprintln!("f_00001652 pc 1652 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1652 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1653 mov rbp,rsp
            rbp = rsp;
            // 1656 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1658 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 165a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 165b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 165c mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 165f mov r14,rsi
            r14 = rsi;
            // 1662 mov rbx,rdi
            rbx = rdi;
            // 1665 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1668 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 166e mov rcx,QWORD PTR [rbx]
            rcx = m.r64(rbx) as u64;
            // 1671 mov r9,QWORD PTR [rcx+0x38]
            r9 = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 1675 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // 1678 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 167a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 167c cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // 1680 lea rax,[rip+0xfdbd9] # 0xff260
            rax = m.env.data_va(80429);
            // 1687 mov esi,DWORD PTR [rax+rdx*4]
            rsi = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 168a lea rax,[rip+0xfd98f] # 0xff020
            rax = m.env.data_va(79853);
            // 1691 mov ecx,DWORD PTR [rax+rdx*4]
            rcx = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 1694 lea rax,[rip+0xfd9b5] # 0xff050
            rax = m.env.data_va(79901);
            // 169b mov r8d,DWORD PTR [rax+rdx*4]
            r8 = (m.r32(rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 169f mov rdi,rbx
            rdi = rbx;
            // 16a2 mov rdx,r14
            rdx = r14;
            // 16a5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 16a9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 16aa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 16ac pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 16ae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 16af jmp r9
            { let r = { let t = r9; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x290ce size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000290ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x290ce;
    loop {
        match pc {
        0x290ce => {
            // block 290ce
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 290ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 290ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 290cf mov rbp,rsp
            rbp = rsp;
            // 290d2 mov rax,rsi
            rax = rsi;
            // 290d5 mov rsi,rdi
            rsi = rdi;
            // 290d8 mov rdi,QWORD PTR [rsi]
            rdi = m.r64(rsi) as u64;
            // 290db test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 290de je 0x2912a
            if fl.e() { pc = 0x2912a; continue; }
            pc = 0x290e0;
        }
        0x290e0 => {
            // block 290e0
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 290e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 290e0 mov r8d,DWORD PTR [rsi+0x8]
            r8 = (m.r32(rsi.wrapping_add(0x8)) as u64) as u32 as u64;
            // 290e4 test r8d,r8d
            fl = logic::<32>((r8 as u32 as u64) & (r8 as u32 as u64));
            // 290e7 je 0x2912a
            if fl.e() { pc = 0x2912a; continue; }
            pc = 0x290e9;
        }
        0x290e9 => {
            // block 290e9
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 290e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 290e9 mov rsi,QWORD PTR [rax]
            rsi = m.r64(rax) as u64;
            // 290ec test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 290ef je 0x2912a
            if fl.e() { pc = 0x2912a; continue; }
            pc = 0x290f1;
        }
        0x290f1 => {
            // block 290f1
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 290f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 290f1 mov r9d,DWORD PTR [rax+0x8]
            r9 = (m.r32(rax.wrapping_add(0x8)) as u64) as u32 as u64;
            // 290f5 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 290f7 test r9d,r9d
            fl = logic::<32>((r9 as u32 as u64) & (r9 as u32 as u64));
            // 290fa je 0x2912c
            if fl.e() { pc = 0x2912c; continue; }
            pc = 0x290fc;
        }
        0x290fc => {
            // block 290fc
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 290fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 290fc cmp r9d,r8d
            fl = sub::<32>((r9 as u32 as u64), (r8 as u32 as u64)).1;
            // 290ff jne 0x2912c
            if fl.ne() { pc = 0x2912c; continue; }
            pc = 0x29101;
        }
        0x29101 => {
            // block 29101
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 29101 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29101 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 29103 cmp rsi,rdi
            fl = sub::<64>(rsi, rdi).1;
            // 29106 je 0x2912c
            if fl.e() { pc = 0x2912c; continue; }
            pc = 0x29108;
        }
        0x29108 => {
            // block 29108
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 29108 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29108 sub r8d,edx
            { let (r, f) = sub::<32>((r8 as u32 as u64), (rdx as u32 as u64));
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2910b jbe 0x2912a
            if fl.be() { pc = 0x2912a; continue; }
            pc = 0x2910d;
        }
        0x2910d => {
            // block 2910d
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 2910d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2910d cmp r8d,ecx
            fl = sub::<32>((r8 as u32 as u64), (rcx as u32 as u64)).1;
            // 29110 cmovb ecx,r8d
            if fl.b() { rcx = ((r8 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 29114 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 29116 add rdi,rax
            { let (r, f) = add::<64>(rdi, rax);
              rdi = r;
              fl = f; }
            // 29119 add rsi,rax
            { let (r, f) = add::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 2911c mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            // 2911e call 0xec218
            { let r = { let t = m.env.stub_va(0x9e5); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29123 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 29125 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 29128 jmp 0x2912c
            pc = 0x2912c; continue;
        }
        0x2912a => {
            // block 2912a
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 2912a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2912a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2912c;
        }
        0x2912c => {
            // block 2912c
            #[cfg(feature = "trace")] eprintln!("f_000290ce pc 2912c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2912c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2912d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42fe0 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042fe0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x42fe0;
    loop {
        match pc {
        0x42fe0 => {
            // block 42fe0
            #[cfg(feature = "trace")] eprintln!("f_00042fe0 pc 42fe0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42fe0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42fe1 mov rbp,rsp
            rbp = rsp;
            // 42fe4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42fe6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42fe8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42fe9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42fea mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 42fed mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 42ff0 mov rbx,rdi
            rbx = rdi;
            // 42ff3 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42ff6 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42ffc mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 42ffe xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43000 cmp r14d,0xf
            fl = sub::<32>((r14 as u32 as u64), 0xfu64).1;
            // 43004 ja 0x43034
            if fl.a() { pc = 0x43034; continue; }
            pc = 0x43006;
        }
        0x43006 => {
            // block 43006
            #[cfg(feature = "trace")] eprintln!("f_00042fe0 pc 43006 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43006 test cl,cl
            fl = logic::<8>((rcx as u8 as u64) & (rcx as u8 as u64));
            // 43008 je 0x43034
            if fl.e() { pc = 0x43034; continue; }
            pc = 0x4300a;
        }
        0x4300a => {
            // block 4300a
            #[cfg(feature = "trace")] eprintln!("f_00042fe0 pc 4300a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4300a mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 4300d movzx edx,r15b
            rdx = ((r15 as u8 as u64)) as u32 as u64;
            // 43011 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // 43014 lea rcx,[rip+0xbd4b5] # 0x1004d0
            rcx = m.env.data_va(85149);
            // 4301b mov ecx,DWORD PTR [rcx+rsi*4]
            rcx = (m.r32(rcx.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // 4301e lea rdi,[rip+0xbd4fb] # 0x100520
            rdi = m.env.data_va(85229);
            // 43025 mov r8d,DWORD PTR [rdi+rsi*4]
            r8 = (m.r32(rdi.wrapping_add(rsi.wrapping_mul(4))) as u64) as u32 as u64;
            // 43029 mov esi,0x905
            rsi = (0x905u64) as u32 as u64;
            // 4302e mov rdi,rbx
            rdi = rbx;
            // 43031 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43034;
        }
        0x43034 => {
            // block 43034
            #[cfg(feature = "trace")] eprintln!("f_00042fe0 pc 43034 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43034 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43038 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43039 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 4303b pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 4303d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4303e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4b4c size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4b4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe4b4c;
    loop {
        match pc {
        0xe4b4c => {
            // block e4b4c
            #[cfg(feature = "trace")] eprintln!("f_000e4b4c pc e4b4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4b4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4b4d mov rbp,rsp
            rbp = rsp;
            // e4b50 lea rax,[rip+0x61751] # 0x1462a8
            rax = m.env.data_va(371317);
            // e4b57 mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // e4b5a mov QWORD PTR [rdi+0x90],0x0
            m.w64(rdi.wrapping_add(0x90), (0x0u64) as u64);
            // e4b65 mov BYTE PTR [rdi+0x8c],0x0
            m.w8(rdi.wrapping_add(0x8c), (0x0u64) as u8);
            // e4b6c mov WORD PTR [rdi+0xc],0x0
            m.w16(rdi.wrapping_add(0xc), (0x0u64) as u16);
            // e4b72 mov DWORD PTR [rdi+0x8],0x0
            m.w32(rdi.wrapping_add(0x8), (0x0u64) as u32);
            // e4b79 mov QWORD PTR [rdi+0x48],0x0
            m.w64(rdi.wrapping_add(0x48), (0x0u64) as u64);
            // e4b81 mov QWORD PTR [rdi+0x40],0x0
            m.w64(rdi.wrapping_add(0x40), (0x0u64) as u64);
            // e4b89 mov QWORD PTR [rdi+0x38],0x0
            m.w64(rdi.wrapping_add(0x38), (0x0u64) as u64);
            // e4b91 mov QWORD PTR [rdi+0x30],0x0
            m.w64(rdi.wrapping_add(0x30), (0x0u64) as u64);
            // e4b99 mov QWORD PTR [rdi+0x28],0x0
            m.w64(rdi.wrapping_add(0x28), (0x0u64) as u64);
            // e4ba1 mov QWORD PTR [rdi+0x20],0x0
            m.w64(rdi.wrapping_add(0x20), (0x0u64) as u64);
            // e4ba9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4baa ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe9a44 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e9a44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe9a44;
    loop {
        match pc {
        0xe9a44 => {
            // block e9a44
            #[cfg(feature = "trace")] eprintln!("f_000e9a44 pc e9a44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e9a44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e9a45 mov rbp,rsp
            rbp = rsp;
            // e9a48 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e9a4a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e9a4b mov rbx,rdi
            rbx = rdi;
            // e9a4e mov rax,QWORD PTR [rip+0x5694b] # 0x1403a0
            rax = m.r64(m.env.data_va(346989)) as u64;
            // e9a55 lea rcx,[rax+0x18]
            rcx = rax.wrapping_add(0x18);
            // e9a59 mov QWORD PTR [rbx],rcx
            m.w64(rbx, (rcx) as u64);
            // e9a5c lea r14,[rbx+0x1a8]
            r14 = rbx.wrapping_add(0x1a8);
            // e9a63 add rax,0x40
            { let (r, f) = add::<64>(rax, 0x40u64);
              rax = r;
              fl = f; }
            // e9a67 mov QWORD PTR [rbx+0x1a8],rax
            m.w64(rbx.wrapping_add(0x1a8), (rax) as u64);
            // e9a6e lea rdi,[rbx+0x10]
            rdi = rbx.wrapping_add(0x10);
            // e9a72 call 0xeb912
            { let r = { let t = m.env.stub_va(0xdf); m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9a77 mov rsi,QWORD PTR [rip+0x56772] # 0x1401f0
            rsi = m.r64(m.env.data_va(346557)) as u64;
            // e9a7e add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // e9a82 mov rdi,rbx
            rdi = rbx;
            // e9a85 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9a8a mov rdi,r14
            rdi = r14;
            // e9a8d call 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e9a92 mov rdi,rbx
            rdi = rbx;
            // e9a95 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e9a96 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e9a98 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e9a99 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe092 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e092<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe092;
    loop {
        match pc {
        0xe092 => {
            // block e092
            #[cfg(feature = "trace")] eprintln!("f_0000e092 pc e092 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e092 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e093 mov rbp,rsp
            rbp = rsp;
            // e096 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e098 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e099 mov r14,rsi
            r14 = rsi;
            // e09c mov rbx,rdi
            rbx = rdi;
            // e09f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0a2 mov esi,DWORD PTR [r14]
            rsi = (m.r32(r14) as u64) as u32 as u64;
            // e0a5 call QWORD PTR [rax+0x16e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x16e0)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ab mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ae mov esi,DWORD PTR [r14+0x4]
            rsi = (m.r32(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // e0b2 mov rdi,rbx
            rdi = rbx;
            // e0b5 call QWORD PTR [rax+0x16f0]
            { let r = { let t = m.r64(rax.wrapping_add(0x16f0)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0bb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0be mov esi,DWORD PTR [r14+0x18]
            rsi = (m.r32(r14.wrapping_add(0x18)) as u64) as u32 as u64;
            // e0c2 mov rdi,rbx
            rdi = rbx;
            // e0c5 call QWORD PTR [rax+0x1720]
            { let r = { let t = m.r64(rax.wrapping_add(0x1720)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0cb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0ce movzx esi,BYTE PTR [r14+0x14]
            rsi = (m.r8(r14.wrapping_add(0x14)) as u64) as u32 as u64;
            // e0d3 mov rdi,rbx
            rdi = rbx;
            // e0d6 call QWORD PTR [rax+0x1700]
            { let r = { let t = m.r64(rax.wrapping_add(0x1700)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0dc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0df movzx esi,BYTE PTR [r14+0x15]
            rsi = (m.r8(r14.wrapping_add(0x15)) as u64) as u32 as u64;
            // e0e4 mov rdi,rbx
            rdi = rbx;
            // e0e7 call QWORD PTR [rax+0x1710]
            { let r = { let t = m.r64(rax.wrapping_add(0x1710)) as u64; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ed mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e0ef pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0f0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0f2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0f3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e7e size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002e7e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e7e;
    loop {
        match pc {
        0x2e7e => {
            // block 2e7e
            #[cfg(feature = "trace")] eprintln!("f_00002e7e pc 2e7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e7e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e7f mov rbp,rsp
            rbp = rsp;
            // 2e82 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e84 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e86 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e88 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e89 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 2e8c mov r14,rsi
            r14 = rsi;
            // 2e8f mov rbx,rdi
            rbx = rdi;
            // 2e92 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2e95 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 2e98 call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e9e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2ea0 je 0x2ea6
            if fl.e() { pc = 0x2ea6; continue; }
            pc = 0x2ea2;
        }
        0x2ea2 => {
            // block 2ea2
            #[cfg(feature = "trace")] eprintln!("f_00002e7e pc 2ea2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ea2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2ea4 jmp 0x2edd
            pc = 0x2edd; continue;
        }
        0x2ea6 => {
            // block 2ea6
            #[cfg(feature = "trace")] eprintln!("f_00002e7e pc 2ea6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ea6 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2ea9 mov r12,QWORD PTR [rax+0x38]
            r12 = m.r64(rax.wrapping_add(0x38)) as u64;
            // 2ead mov rdi,rbx
            rdi = rbx;
            // 2eb0 call QWORD PTR [rax+0x1d68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d68)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2eb6 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 2eba xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2ebc test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 2ebe cmove rcx,rdx
            if fl.e() { rcx = rdx; }
            // 2ec2 lea rax,[rip+0xfc277] # 0xff140
            rax = m.env.data_va(80141);
            // 2ec9 mov esi,DWORD PTR [rax+rcx*4]
            rsi = (m.r32(rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2ecc mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 2ed1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 2ed4 mov rdi,rbx
            rdi = rbx;
            // 2ed7 mov rdx,r14
            rdx = r14;
            // 2eda call r12
            { let r = { let t = r12; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2edd;
        }
        0x2edd => {
            // block 2edd
            #[cfg(feature = "trace")] eprintln!("f_00002e7e pc 2edd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2edd pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ede pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2ee0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2ee2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2ee4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ee5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x16012 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00016012<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x16012;
    loop {
        match pc {
        0x16012 => {
            // block 16012
            #[cfg(feature = "trace")] eprintln!("f_00016012 pc 16012 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 16012 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 16013 mov rbp,rsp
            rbp = rsp;
            // 16016 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 16018 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 16019 mov QWORD PTR [rip+0x1ab9ec],0x0 # 0x1c1a10
            m.w64(m.env.data_va(877021), (0x0u64) as u64);
            // 16024 mov QWORD PTR [rip+0x1ab9d9],0x0 # 0x1c1a08
            m.w64(m.env.data_va(877013), (0x0u64) as u64);
            // 1602f lea rsi,[rip+0x1ab9ca] # 0x1c1a00
            rsi = m.env.data_va(877005);
            // 16036 lea rax,[rip+0x1ab9cb] # 0x1c1a08
            rax = m.env.data_va(877013);
            // 1603d mov QWORD PTR [rip+0x1ab9bc],rax # 0x1c1a00
            m.w64(m.env.data_va(877005), (rax) as u64);
            // 16044 lea rdi,[rip+0xffffffffffffec8b] # 0x14cd6
            rdi = m.env.code_va(0x14cd6);
            // 1604b lea r14,[rip+0xfffffffffffe911e] # 0xfffffffffffff170
            r14 = m.env.data_va(-968387);
            // 16052 mov rdx,r14
            rdx = r14;
            // 16055 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1605a lea rbx,[rip+0x1ab9b7] # 0x1c1a18
            rbx = m.env.data_va(877029);
            // 16061 mov rdi,rbx
            rdi = rbx;
            // 16064 call 0x154a4
            { let r = { let t = m.env.code_va(0x154a4); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 16069 lea rdi,[rip+0xffffffffffffec74] # 0x14ce4
            rdi = m.env.code_va(0x14ce4);
            // 16070 mov rsi,rbx
            rsi = rbx;
            // 16073 mov rdx,r14
            rdx = r14;
            // 16076 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 16077 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 16079 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1607a jmp 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e220 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e220<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2e220;
    loop {
        match pc {
        0x2e220 => {
            // block 2e220
            #[cfg(feature = "trace")] eprintln!("f_0002e220 pc 2e220 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e220 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e221 mov rbp,rsp
            rbp = rsp;
            // 2e224 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2e226 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2e228 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2e22a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e22b mov r14,rdi
            r14 = rdi;
            // 2e22e mov rbx,QWORD PTR [r14+0x10]
            rbx = m.r64(r14.wrapping_add(0x10)) as u64;
            // 2e232 mov r12,QWORD PTR [r14+0x18]
            r12 = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2e236 cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 2e239 je 0x2e280
            if fl.e() { pc = 0x2e280; continue; }
            pc = 0x2e23b;
        }
        0x2e23b => {
            // block 2e23b
            #[cfg(feature = "trace")] eprintln!("f_0002e220 pc 2e23b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e23b mov r15,QWORD PTR [rbx]
            r15 = m.r64(rbx) as u64;
            // 2e23e test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2e241 je 0x2e255
            if fl.e() { pc = 0x2e255; continue; }
            pc = 0x2e243;
        }
        0x2e243 => {
            // block 2e243
            #[cfg(feature = "trace")] eprintln!("f_0002e220 pc 2e243 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e243 mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e246 mov rdi,r15
            rdi = r15;
            // 2e249 call QWORD PTR [rax+0x18]
            { let r = { let t = m.r64(rax.wrapping_add(0x18)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e24c mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 2e24f mov rdi,r15
            rdi = r15;
            // 2e252 call QWORD PTR [rax+0x8]
            { let r = { let t = m.r64(rax.wrapping_add(0x8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x2e255;
        }
        0x2e255 => {
            // block 2e255
            #[cfg(feature = "trace")] eprintln!("f_0002e220 pc 2e255 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e255 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2e259 cmp r12,rbx
            fl = sub::<64>(r12, rbx).1;
            // 2e25c jne 0x2e23b
            if fl.ne() { pc = 0x2e23b; continue; }
            pc = 0x2e25e;
        }
        0x2e25e => {
            // block 2e25e
            #[cfg(feature = "trace")] eprintln!("f_0002e220 pc 2e25e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e25e mov rcx,QWORD PTR [r14+0x10]
            rcx = m.r64(r14.wrapping_add(0x10)) as u64;
            // 2e262 mov rax,QWORD PTR [r14+0x18]
            rax = m.r64(r14.wrapping_add(0x18)) as u64;
            // 2e266 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 2e269 je 0x2e280
            if fl.e() { pc = 0x2e280; continue; }
            pc = 0x2e26b;
        }
        0x2e26b => {
            // block 2e26b
            #[cfg(feature = "trace")] eprintln!("f_0002e220 pc 2e26b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e26b lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 2e26f sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 2e272 not rdx
            { let v = !rdx; rdx = v; }
            // 2e275 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 2e279 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 2e27c mov QWORD PTR [r14+0x18],rdx
            m.w64(r14.wrapping_add(0x18), (rdx) as u64);
            pc = 0x2e280;
        }
        0x2e280 => {
            // block 2e280
            #[cfg(feature = "trace")] eprintln!("f_0002e220 pc 2e280 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e280 mov BYTE PTR [r14+0x8],0x0
            m.w8(r14.wrapping_add(0x8), (0x0u64) as u8);
            // 2e285 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e286 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2e288 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2e28a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2e28c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e28d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x316d4 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000316d4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x316d4;
    loop {
        match pc {
        0x316d4 => {
            // block 316d4
            #[cfg(feature = "trace")] eprintln!("f_000316d4 pc 316d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 316d4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 316d5 mov rbp,rsp
            rbp = rsp;
            // 316d8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 316da push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 316db mov rbx,rsi
            rbx = rsi;
            // 316de mov r14,rdi
            r14 = rdi;
            // 316e1 mov QWORD PTR [r14+0x10],0x0
            m.w64(r14.wrapping_add(0x10), (0x0u64) as u64);
            // 316e9 mov QWORD PTR [r14+0x8],0x0
            m.w64(r14.wrapping_add(0x8), (0x0u64) as u64);
            // 316f1 mov QWORD PTR [r14],0x0
            m.w64(r14, (0x0u64) as u64);
            // 316f8 mov rsi,QWORD PTR [rbx+0x8]
            rsi = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 316fc sub rsi,QWORD PTR [rbx]
            { let (r, f) = sub::<64>(rsi, m.r64(rbx) as u64);
              rsi = r;
              fl = f; }
            // 316ff je 0x31723
            if fl.e() { pc = 0x31723; continue; }
            pc = 0x31701;
        }
        0x31701 => {
            // block 31701
            #[cfg(feature = "trace")] eprintln!("f_000316d4 pc 31701 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31701 mov rdi,r14
            rdi = r14;
            // 31704 call 0xebb10
            { let r = { let t = m.env.stub_va(0x2dd); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 31709 mov rsi,QWORD PTR [rbx]
            rsi = m.r64(rbx) as u64;
            // 3170c mov rbx,QWORD PTR [rbx+0x8]
            rbx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 31710 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 31713 mov rdi,QWORD PTR [r14+0x8]
            rdi = m.r64(r14.wrapping_add(0x8)) as u64;
            // 31717 mov rdx,rbx
            rdx = rbx;
            // 3171a call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3171f add QWORD PTR [r14+0x8],rbx
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x8)) as u64, rbx);
              m.w64(r14.wrapping_add(0x8), (r) as u64);
              fl = f; }
            pc = 0x31723;
        }
        0x31723 => {
            // block 31723
            #[cfg(feature = "trace")] eprintln!("f_000316d4 pc 31723 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31723 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 31724 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 31726 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 31727 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x31728 => {
            // block 31728
            #[cfg(feature = "trace")] eprintln!("f_000316d4 pc 31728 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31728 mov rbx,rax
            rbx = rax;
            // 3172b mov rdi,QWORD PTR [r14]
            rdi = m.r64(r14) as u64;
            // 3172e test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 31731 je 0x31742
            if fl.e() { pc = 0x31742; continue; }
            pc = 0x31733;
        }
        0x31733 => {
            // block 31733
            #[cfg(feature = "trace")] eprintln!("f_000316d4 pc 31733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31733 cmp QWORD PTR [r14+0x8],rdi
            fl = sub::<64>(m.r64(r14.wrapping_add(0x8)) as u64, rdi).1;
            // 31737 je 0x3173d
            if fl.e() { pc = 0x3173d; continue; }
            pc = 0x31739;
        }
        0x31739 => {
            // block 31739
            #[cfg(feature = "trace")] eprintln!("f_000316d4 pc 31739 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31739 mov QWORD PTR [r14+0x8],rdi
            m.w64(r14.wrapping_add(0x8), (rdi) as u64);
            pc = 0x3173d;
        }
        0x3173d => {
            // block 3173d
            #[cfg(feature = "trace")] eprintln!("f_000316d4 pc 3173d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3173d call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x31742;
        }
        0x31742 => {
            // block 31742
            #[cfg(feature = "trace")] eprintln!("f_000316d4 pc 31742 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31742 mov rdi,rbx
            rdi = rbx;
            // 31745 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x59643 size=155
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00059643<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x59643;
    loop {
        match pc {
        0x59643 => {
            // block 59643
            #[cfg(feature = "trace")] eprintln!("f_00059643 pc 59643 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 59643 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 59644 mov rbp,rsp
            rbp = rsp;
            // 59647 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 59649 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 5964b push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 5964d push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 5964f push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 59650 sub rsp,0x48
            { let (r, f) = sub::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 59654 mov r15d,r8d
            r15 = ((r8 as u32 as u64)) as u32 as u64;
            // 59657 mov r12d,edx
            r12 = ((rdx as u32 as u64)) as u32 as u64;
            // 5965a mov DWORD PTR [rbp-0x6c],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff94), ((rsi as u32 as u64)) as u32);
            // 5965d mov r13,rdi
            r13 = rdi;
            // 59660 mov ebx,r12d
            rbx = ((r12 as u32 as u64)) as u32 as u64;
            // 59663 and ebx,0xffffff
            { let r = (rbx as u32 as u64) & 0xffffffu64; let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 59669 mov r14d,ebx
            r14 = ((rbx as u32 as u64)) as u32 as u64;
            // 5966c shr r14d,0x10
            { let (r, f) = shr::<32>((r14 as u32 as u64), 0x10u64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 59670 lea rdi,[rbp-0x68]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 59674 call 0x32360
            { let r = { let t = m.env.code_va(0x32360); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 59679 mov DWORD PTR [rbp-0x68],0x870
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (0x870u64) as u32);
            // 59680 test r15b,r15b
            fl = logic::<8>((r15 as u8 as u64) & (r15 as u8 as u64));
            // 59683 mov eax,0x1000
            rax = (0x1000u64) as u32 as u64;
            // 59688 mov ecx,0xf00
            rcx = (0xf00u64) as u32 as u64;
            // 5968d cmovne ecx,eax
            if fl.ne() { rcx = ((rax as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 59690 mov eax,0x870000
            rax = (0x870000u64) as u32 as u64;
            // 59695 mov r8d,0x7e9000
            r8 = (0x7e9000u64) as u32 as u64;
            // 5969b cmovne r8d,eax
            if fl.ne() { r8 = ((rax as u32 as u64)) as u32 as u64; } else { r8 = r8 as u32 as u64; }
            // 5969f mov eax,0x800
            rax = (0x800u64) as u32 as u64;
            // 596a4 mov edx,0x780
            rdx = (0x780u64) as u32 as u64;
            // 596a9 cmovne edx,eax
            if fl.ne() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // 596ac mov DWORD PTR [rbp-0x64],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ff9c), ((rcx as u32 as u64)) as u32);
            // 596af mov DWORD PTR [rbp-0x5c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa4), (0x0u64) as u32);
            // 596b6 mov DWORD PTR [rbp-0x60],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), ((rdx as u32 as u64)) as u32);
            // 596b9 movzx edx,r12b
            rdx = ((r12 as u8 as u64)) as u32 as u64;
            // 596bd movzx esi,bh
            rsi = (((rbx >> 8) as u8 as u64)) as u32 as u64;
            // 596c0 mov rdi,r13
            rdi = r13;
            // 596c3 mov ecx,r14d
            rcx = ((r14 as u32 as u64)) as u32 as u64;
            // 596c6 mov r9d,DWORD PTR [rbp-0x6c]
            r9 = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ff94)) as u64) as u32 as u64;
            // 596ca call 0x59541
            { let r = { let t = m.env.code_va(0x59541); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 596cf add rsp,0x48
            { let (r, f) = add::<64>(rsp, 0x48u64);
              rsp = r;
              fl = f; }
            // 596d3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 596d4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 596d6 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 596d8 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 596da pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 596dc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 596dd ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x32e76 size=194
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032e76<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x32e76;
    loop {
        match pc {
        0x32e76 => {
            // block 32e76
            #[cfg(feature = "trace")] eprintln!("f_00032e76 pc 32e76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32e76 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32e77 mov rbp,rsp
            rbp = rsp;
            // 32e7a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 32e7c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 32e7e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 32e7f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 32e80 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 32e83 mov rbx,rsi
            rbx = rsi;
            // 32e86 mov r14,rdi
            r14 = rdi;
            // 32e89 mov al,BYTE PTR [rip+0x1951f9] # 0x1c8088
            rax = (rax & !0xff) | ((m.r8(m.env.data_va(903253)) as u64) as u8 as u64);
            // 32e8f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 32e91 jne 0x32eea
            if fl.ne() { pc = 0x32eea; continue; }
            pc = 0x32e93;
        }
        0x32e93 => {
            // block 32e93
            #[cfg(feature = "trace")] eprintln!("f_00032e76 pc 32e93 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32e93 lea rdi,[rip+0x1951ee] # 0x1c8088
            rdi = m.env.data_va(903253);
            // 32e9a call 0xec170
            { let r = { let t = m.env.stub_va(0x93d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32e9f test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 32ea1 je 0x32eea
            if fl.e() { pc = 0x32eea; continue; }
            pc = 0x32ea3;
        }
        0x32ea3 => {
            // block 32ea3
            #[cfg(feature = "trace")] eprintln!("f_00032e76 pc 32ea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ea3 mov QWORD PTR [rip+0x1951d2],0x0 # 0x1c8080
            m.w64(m.env.data_va(903245), (0x0u64) as u64);
            // 32eae mov QWORD PTR [rip+0x1951bf],0x0 # 0x1c8078
            m.w64(m.env.data_va(903237), (0x0u64) as u64);
            // 32eb9 mov QWORD PTR [rip+0x1951ac],0x0 # 0x1c8070
            m.w64(m.env.data_va(903229), (0x0u64) as u64);
            // 32ec4 lea rsi,[rip+0x1951a5] # 0x1c8070
            rsi = m.env.data_va(903229);
            // 32ecb mov rdi,QWORD PTR [rip+0x10d2a6] # 0x140178
            rdi = m.r64(m.env.data_va(346437)) as u64;
            // 32ed2 lea rdx,[rip+0xfffffffffffcc297] # 0xfffffffffffff170
            rdx = m.env.data_va(-968387);
            // 32ed9 call 0xec18e
            { let r = { let t = m.env.stub_va(0x95b); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32ede lea rdi,[rip+0x1951a3] # 0x1c8088
            rdi = m.env.data_va(903253);
            // 32ee5 call 0xec176
            { let r = { let t = m.env.stub_va(0x943); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x32eea;
        }
        0x32eea => {
            // block 32eea
            #[cfg(feature = "trace")] eprintln!("f_00032e76 pc 32eea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32eea mov eax,DWORD PTR [rbx+0x18]
            rax = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 32eed cmp rax,0x1f
            fl = sub::<64>(rax, 0x1fu64).1;
            // 32ef1 ja 0x32f1b
            if fl.a() { pc = 0x32f1b; continue; }
            pc = 0x32ef3;
        }
        0x32ef3 => {
            // block 32ef3
            #[cfg(feature = "trace")] eprintln!("f_00032e76 pc 32ef3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32ef3 movzx ecx,r15w
            rcx = ((r15 as u16 as u64)) as u32 as u64;
            // 32ef7 movzx edx,WORD PTR [rbx+0x34]
            rdx = (m.r16(rbx.wrapping_add(0x34)) as u64) as u32 as u64;
            // 32efb cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 32efd jbe 0x32f1b
            if fl.be() { pc = 0x32f1b; continue; }
            pc = 0x32eff;
        }
        0x32eff => {
            // block 32eff
            #[cfg(feature = "trace")] eprintln!("f_00032e76 pc 32eff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32eff lea rax,[rax+rax*2]
            rax = rax.wrapping_add(rax.wrapping_mul(2));
            // 32f03 shl rax,0x5
            { let (r, f) = shl::<64>(rax, 0x5u64, fl); rax = r; fl = f; }
            // 32f07 lea rdx,[rip+0x18eb62] # 0x1c1a70
            rdx = m.env.data_va(877117);
            // 32f0e add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 32f11 lea rax,[rcx+rcx*2]
            rax = rcx.wrapping_add(rcx.wrapping_mul(2));
            // 32f15 lea rsi,[rdx+rax*8]
            rsi = rdx.wrapping_add(rax.wrapping_mul(8));
            // 32f19 jmp 0x32f22
            pc = 0x32f22; continue;
        }
        0x32f1b => {
            // block 32f1b
            #[cfg(feature = "trace")] eprintln!("f_00032e76 pc 32f1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32f1b lea rsi,[rip+0x19514e] # 0x1c8070
            rsi = m.env.data_va(903229);
            pc = 0x32f22;
        }
        0x32f22 => {
            // block 32f22
            #[cfg(feature = "trace")] eprintln!("f_00032e76 pc 32f22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32f22 mov rdi,r14
            rdi = r14;
            // 32f25 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32f2a mov rax,r14
            rax = r14;
            // 32f2d add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 32f31 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32f32 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 32f34 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 32f36 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32f37 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11a9a size=282
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_00011a9a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x11a9a;
    loop {
        match pc {
        0x11a9a => {
            // block 11a9a
            #[cfg(feature = "trace")] eprintln!("f_00011a9a pc 11a9a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11a9a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11a9b mov rbp,rsp
            rbp = rsp;
            // 11a9e push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 11aa0 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 11aa2 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 11aa4 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 11aa6 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11aa7 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 11aab mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 11aaf mov r15,rdi
            r15 = rdi;
            // 11ab2 movabs r12,0x492492492492492
            r12 = 0x492_4924_9249_2492u64;
            // 11abc mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // 11abf mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 11ac3 sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 11ac6 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // 11aca movabs r13,0x6db6db6db6db6db7
            r13 = 0x6db6_db6d_b6db_6db7u64;
            // 11ad4 imul rbx,r13
            { let (r, f) = imul::<64>(rbx, r13); rbx = r; fl = f; }
            // 11ad8 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 11adb cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // 11ade jbe 0x11aeb
            if fl.be() { pc = 0x11aeb; continue; }
            pc = 0x11ae0;
        }
        0x11ae0 => {
            // block 11ae0
            #[cfg(feature = "trace")] eprintln!("f_00011a9a pc 11ae0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11ae0 mov rdi,r15
            rdi = r15;
            // 11ae3 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x11aeb => {
            // block 11aeb
            #[cfg(feature = "trace")] eprintln!("f_00011a9a pc 11aeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11aeb lea rcx,[r15+0x10]
            rcx = r15.wrapping_add(0x10);
            // 11aef mov rdx,QWORD PTR [r15+0x10]
            rdx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 11af3 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 11af6 sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // 11afa imul rdx,r13
            { let (r, f) = imul::<64>(rdx, r13); rdx = r; fl = f; }
            // 11afe movabs rsi,0x249249249249248
            rsi = 0x249_2492_4924_9248u64;
            // 11b08 cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // 11b0b ja 0x11b1a
            if fl.a() { pc = 0x11b1a; continue; }
            pc = 0x11b0d;
        }
        0x11b0d => {
            // block 11b0d
            #[cfg(feature = "trace")] eprintln!("f_00011a9a pc 11b0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11b0d add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // 11b10 cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // 11b13 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // 11b17 mov r12,rdx
            r12 = rdx;
            pc = 0x11b1a;
        }
        0x11b1a => {
            // block 11b1a
            #[cfg(feature = "trace")] eprintln!("f_00011a9a pc 11b1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11b1a mov r14,QWORD PTR [r15+0x8]
            r14 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 11b1e sub r14,rax
            { let (r, f) = sub::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // 11b21 sar r14,0x3
            { let (r, f) = sar::<64>(r14, 0x3u64, fl); r14 = r; fl = f; }
            // 11b25 imul r14,r13
            { let (r, f) = imul::<64>(r14, r13); r14 = r; fl = f; }
            // 11b29 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 11b31 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // 11b35 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 11b37 test r12,r12
            fl = logic::<64>(r12 & r12);
            // 11b3a je 0x11b48
            if fl.e() { pc = 0x11b48; continue; }
            pc = 0x11b3c;
        }
        0x11b3c => {
            // block 11b3c
            #[cfg(feature = "trace")] eprintln!("f_00011a9a pc 11b3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11b3c imul rdi,r12,0x38
            { let (r, f) = imul::<64>(r12, 0x38u64); rdi = r; fl = f; }
            // 11b40 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11b45 mov rbx,rax
            rbx = rax;
            pc = 0x11b48;
        }
        0x11b48 => {
            // block 11b48
            #[cfg(feature = "trace")] eprintln!("f_00011a9a pc 11b48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11b48 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 11b4c imul r14,r14,0x38
            { let (r, f) = imul::<64>(r14, 0x38u64); r14 = r; fl = f; }
            // 11b50 lea rdi,[rbx+r14*1]
            rdi = rbx.wrapping_add(r14);
            // 11b54 mov QWORD PTR [rbp-0x40],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdi) as u64);
            // 11b58 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // 11b5c imul rax,r12,0x38
            { let (r, f) = imul::<64>(r12, 0x38u64); rax = r; fl = f; }
            // 11b60 add rax,rbx
            { let (r, f) = add::<64>(rax, rbx);
              rax = r;
              fl = f; }
            // 11b63 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 11b67 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 11b6b call 0xe06fe
            { let r = { let t = m.env.code_va(0xe06fe); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11b70 lea rax,[r14+rbx*1+0x38]
            rax = r14.wrapping_add(rbx).wrapping_add(0x38);
            // 11b75 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 11b79 lea rsi,[rbp-0x50]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 11b7d mov rdi,r15
            rdi = r15;
            // 11b80 call 0xebf30
            { let r = { let t = m.env.stub_va(0x6fd); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11b85 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 11b89 call 0xeb8ac
            { let r = { let t = m.env.stub_va(0x79); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11b8e add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // 11b92 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11b93 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 11b95 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 11b97 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 11b99 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 11b9b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11b9c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x11b9d => {
            // block 11b9d
            #[cfg(feature = "trace")] eprintln!("f_00011a9a pc 11b9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11b9d mov rbx,rax
            rbx = rax;
            // 11ba0 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 11ba4 call 0xeb8ac
            { let r = { let t = m.env.stub_va(0x79); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11ba9 mov rdi,rbx
            rdi = rbx;
            // 11bac call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a5c2 size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001a5c2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a5c2;
    loop {
        match pc {
        0x1a5c2 => {
            // block 1a5c2
            #[cfg(feature = "trace")] eprintln!("f_0001a5c2 pc 1a5c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a5c2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a5c3 mov rbp,rsp
            rbp = rsp;
            // 1a5c6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1a5c8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a5ca push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1a5cc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1a5ce push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a5cf sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a5d3 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1a5d7 mov r12,rdi
            r12 = rdi;
            // 1a5da mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1a5de mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a5e3 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1a5e6 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1a5ea inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1a5ed mov rax,rbx
            rax = rbx;
            // 1a5f0 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1a5f4 je 0x1a602
            if fl.e() { pc = 0x1a602; continue; }
            pc = 0x1a5f6;
        }
        0x1a5f6 => {
            // block 1a5f6
            #[cfg(feature = "trace")] eprintln!("f_0001a5c2 pc 1a5f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a5f6 mov rdi,r12
            rdi = r12;
            // 1a5f9 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x1a602 => {
            // block 1a602
            #[cfg(feature = "trace")] eprintln!("f_0001a5c2 pc 1a602 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a602 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1a607 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1a60c mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1a610 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1a613 mov rcx,r15
            rcx = r15;
            // 1a616 sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1a61a movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1a624 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1a627 ja 0x1a635
            if fl.a() { pc = 0x1a635; continue; }
            pc = 0x1a629;
        }
        0x1a629 => {
            // block 1a629
            #[cfg(feature = "trace")] eprintln!("f_0001a5c2 pc 1a629 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a629 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1a62c cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1a62f cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1a633 jmp 0x1a63f
            pc = 0x1a63f; continue;
        }
        0x1a635 => {
            // block 1a635
            #[cfg(feature = "trace")] eprintln!("f_0001a5c2 pc 1a635 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a635 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1a63f;
        }
        0x1a63f => {
            // block 1a63f
            #[cfg(feature = "trace")] eprintln!("f_0001a5c2 pc 1a63f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a63f mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a644 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1a648 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1a64b mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1a64f mov r13,r14
            r13 = r14;
            // 1a652 sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1a656 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1a65e mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1a662 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1a664 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1a667 je 0x1a679
            if fl.e() { pc = 0x1a679; continue; }
            pc = 0x1a669;
        }
        0x1a669 => {
            // block 1a669
            #[cfg(feature = "trace")] eprintln!("f_0001a5c2 pc 1a669 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a669 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1a671 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a676 mov rbx,rax
            rbx = rax;
            pc = 0x1a679;
        }
        0x1a679 => {
            // block 1a679
            #[cfg(feature = "trace")] eprintln!("f_0001a5c2 pc 1a679 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a679 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1a67d mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1a681 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1a685 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1a687 mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1a68b lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1a690 mov rdi,rbx
            rdi = rbx;
            // 1a693 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1a697 mov rsi,r15
            rsi = r15;
            // 1a69a mov rdx,r14
            rdx = r14;
            // 1a69d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a6a2 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1a6a6 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1a6aa mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1a6af mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1a6b3 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1a6b7 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1a6bb mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1a6c0 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1a6c4 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1a6c8 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1a6cc lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1a6d0 call 0xeb984
            { let r = { let t = m.env.stub_va(0x151); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a6d5 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a6d9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a6da pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1a6dc pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1a6de pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a6e0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a6e2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a6e3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1a872 size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001a872<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1a872;
    loop {
        match pc {
        0x1a872 => {
            // block 1a872
            #[cfg(feature = "trace")] eprintln!("f_0001a872 pc 1a872 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a872 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1a873 mov rbp,rsp
            rbp = rsp;
            // 1a876 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1a878 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1a87a push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1a87c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1a87e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1a87f sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a883 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1a887 mov r12,rdi
            r12 = rdi;
            // 1a88a mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1a88e mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a893 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1a896 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1a89a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1a89d mov rax,rbx
            rax = rbx;
            // 1a8a0 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1a8a4 je 0x1a8b2
            if fl.e() { pc = 0x1a8b2; continue; }
            pc = 0x1a8a6;
        }
        0x1a8a6 => {
            // block 1a8a6
            #[cfg(feature = "trace")] eprintln!("f_0001a872 pc 1a8a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a8a6 mov rdi,r12
            rdi = r12;
            // 1a8a9 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x1a8b2 => {
            // block 1a8b2
            #[cfg(feature = "trace")] eprintln!("f_0001a872 pc 1a8b2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a8b2 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1a8b7 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1a8bc mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1a8c0 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1a8c3 mov rcx,r15
            rcx = r15;
            // 1a8c6 sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1a8ca movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1a8d4 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1a8d7 ja 0x1a8e5
            if fl.a() { pc = 0x1a8e5; continue; }
            pc = 0x1a8d9;
        }
        0x1a8d9 => {
            // block 1a8d9
            #[cfg(feature = "trace")] eprintln!("f_0001a872 pc 1a8d9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a8d9 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1a8dc cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1a8df cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1a8e3 jmp 0x1a8ef
            pc = 0x1a8ef; continue;
        }
        0x1a8e5 => {
            // block 1a8e5
            #[cfg(feature = "trace")] eprintln!("f_0001a872 pc 1a8e5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a8e5 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1a8ef;
        }
        0x1a8ef => {
            // block 1a8ef
            #[cfg(feature = "trace")] eprintln!("f_0001a872 pc 1a8ef rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a8ef mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1a8f4 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1a8f8 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1a8fb mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1a8ff mov r13,r14
            r13 = r14;
            // 1a902 sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1a906 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1a90e mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1a912 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1a914 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1a917 je 0x1a929
            if fl.e() { pc = 0x1a929; continue; }
            pc = 0x1a919;
        }
        0x1a919 => {
            // block 1a919
            #[cfg(feature = "trace")] eprintln!("f_0001a872 pc 1a919 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a919 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1a921 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a926 mov rbx,rax
            rbx = rax;
            pc = 0x1a929;
        }
        0x1a929 => {
            // block 1a929
            #[cfg(feature = "trace")] eprintln!("f_0001a872 pc 1a929 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1a929 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1a92d mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1a931 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1a935 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1a937 mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1a93b lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1a940 mov rdi,rbx
            rdi = rbx;
            // 1a943 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1a947 mov rsi,r15
            rsi = r15;
            // 1a94a mov rdx,r14
            rdx = r14;
            // 1a94d call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a952 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1a956 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1a95a mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1a95f mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1a963 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1a967 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1a96b mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1a970 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1a974 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1a978 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1a97c lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1a980 call 0xeb972
            { let r = { let t = m.env.stub_va(0x13f); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1a985 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1a989 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1a98a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1a98c pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1a98e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1a990 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1a992 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1a993 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1b0d2 size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001b0d2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1b0d2;
    loop {
        match pc {
        0x1b0d2 => {
            // block 1b0d2
            #[cfg(feature = "trace")] eprintln!("f_0001b0d2 pc 1b0d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b0d2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1b0d3 mov rbp,rsp
            rbp = rsp;
            // 1b0d6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1b0d8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1b0da push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1b0dc push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1b0de push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1b0df sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b0e3 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1b0e7 mov r12,rdi
            r12 = rdi;
            // 1b0ea mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1b0ee mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b0f3 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1b0f6 sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1b0fa inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1b0fd mov rax,rbx
            rax = rbx;
            // 1b100 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1b104 je 0x1b112
            if fl.e() { pc = 0x1b112; continue; }
            pc = 0x1b106;
        }
        0x1b106 => {
            // block 1b106
            #[cfg(feature = "trace")] eprintln!("f_0001b0d2 pc 1b106 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b106 mov rdi,r12
            rdi = r12;
            // 1b109 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x1b112 => {
            // block 1b112
            #[cfg(feature = "trace")] eprintln!("f_0001b0d2 pc 1b112 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b112 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1b117 mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1b11c mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1b120 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1b123 mov rcx,r15
            rcx = r15;
            // 1b126 sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1b12a movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1b134 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1b137 ja 0x1b145
            if fl.a() { pc = 0x1b145; continue; }
            pc = 0x1b139;
        }
        0x1b139 => {
            // block 1b139
            #[cfg(feature = "trace")] eprintln!("f_0001b0d2 pc 1b139 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b139 sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1b13c cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1b13f cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1b143 jmp 0x1b14f
            pc = 0x1b14f; continue;
        }
        0x1b145 => {
            // block 1b145
            #[cfg(feature = "trace")] eprintln!("f_0001b0d2 pc 1b145 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b145 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1b14f;
        }
        0x1b14f => {
            // block 1b14f
            #[cfg(feature = "trace")] eprintln!("f_0001b0d2 pc 1b14f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b14f mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b154 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1b158 sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1b15b mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1b15f mov r13,r14
            r13 = r14;
            // 1b162 sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1b166 mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1b16e mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1b172 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1b174 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1b177 je 0x1b189
            if fl.e() { pc = 0x1b189; continue; }
            pc = 0x1b179;
        }
        0x1b179 => {
            // block 1b179
            #[cfg(feature = "trace")] eprintln!("f_0001b0d2 pc 1b179 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b179 lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1b181 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b186 mov rbx,rax
            rbx = rax;
            pc = 0x1b189;
        }
        0x1b189 => {
            // block 1b189
            #[cfg(feature = "trace")] eprintln!("f_0001b0d2 pc 1b189 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b189 lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1b18d mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1b191 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1b195 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1b197 mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1b19b lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1b1a0 mov rdi,rbx
            rdi = rbx;
            // 1b1a3 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1b1a7 mov rsi,r15
            rsi = r15;
            // 1b1aa mov rdx,r14
            rdx = r14;
            // 1b1ad call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b1b2 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1b1b6 mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1b1ba mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1b1bf mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1b1c3 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1b1c7 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1b1cb mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1b1d0 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1b1d4 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1b1d8 mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1b1dc lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1b1e0 call 0xeb97e
            { let r = { let t = m.env.stub_va(0x14b); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b1e5 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b1e9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1b1ea pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1b1ec pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1b1ee pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1b1f0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1b1f2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1b1f3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1b316 size=290
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_0001b316<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x1b316;
    loop {
        match pc {
        0x1b316 => {
            // block 1b316
            #[cfg(feature = "trace")] eprintln!("f_0001b316 pc 1b316 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b316 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1b317 mov rbp,rsp
            rbp = rsp;
            // 1b31a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1b31c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1b31e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1b320 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1b322 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1b323 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b327 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // 1b32b mov r12,rdi
            r12 = rdi;
            // 1b32e mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // 1b332 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b337 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // 1b33a sar rbx,0x2
            { let (r, f) = sar::<64>(rbx, 0x2u64, fl); rbx = r; fl = f; }
            // 1b33e inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 1b341 mov rax,rbx
            rax = rbx;
            // 1b344 shr rax,0x3e
            { let (r, f) = shr::<64>(rax, 0x3eu64, fl); rax = r; fl = f; }
            // 1b348 je 0x1b356
            if fl.e() { pc = 0x1b356; continue; }
            pc = 0x1b34a;
        }
        0x1b34a => {
            // block 1b34a
            #[cfg(feature = "trace")] eprintln!("f_0001b316 pc 1b34a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b34a mov rdi,r12
            rdi = r12;
            // 1b34d call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0x1b356 => {
            // block 1b356
            #[cfg(feature = "trace")] eprintln!("f_0001b316 pc 1b356 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b356 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // 1b35b mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // 1b360 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // 1b364 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // 1b367 mov rcx,r15
            rcx = r15;
            // 1b36a sar rcx,0x2
            { let (r, f) = sar::<64>(rcx, 0x2u64, fl); rcx = r; fl = f; }
            // 1b36e movabs rdx,0x1ffffffffffffffe
            rdx = 0x1fff_ffff_ffff_fffeu64;
            // 1b378 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 1b37b ja 0x1b389
            if fl.a() { pc = 0x1b389; continue; }
            pc = 0x1b37d;
        }
        0x1b37d => {
            // block 1b37d
            #[cfg(feature = "trace")] eprintln!("f_0001b316 pc 1b37d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b37d sar r15,1
            { let (r, f) = sar::<64>(r15, 0x1u64, fl); r15 = r; fl = f; }
            // 1b380 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // 1b383 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // 1b387 jmp 0x1b393
            pc = 0x1b393; continue;
        }
        0x1b389 => {
            // block 1b389
            #[cfg(feature = "trace")] eprintln!("f_0001b316 pc 1b389 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b389 movabs r15,0x3fffffffffffffff
            r15 = 0x3fff_ffff_ffff_ffffu64;
            pc = 0x1b393;
        }
        0x1b393 => {
            // block 1b393
            #[cfg(feature = "trace")] eprintln!("f_0001b316 pc 1b393 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b393 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // 1b398 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // 1b39c sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // 1b39f mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // 1b3a3 mov r13,r14
            r13 = r14;
            // 1b3a6 sar r13,0x2
            { let (r, f) = sar::<64>(r13, 0x2u64, fl); r13 = r; fl = f; }
            // 1b3aa mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // 1b3b2 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1b3b6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 1b3b8 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 1b3bb je 0x1b3cd
            if fl.e() { pc = 0x1b3cd; continue; }
            pc = 0x1b3bd;
        }
        0x1b3bd => {
            // block 1b3bd
            #[cfg(feature = "trace")] eprintln!("f_0001b316 pc 1b3bd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b3bd lea rdi,[r15*4+0x0]
            rdi = r15.wrapping_mul(4);
            // 1b3c5 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b3ca mov rbx,rax
            rbx = rax;
            pc = 0x1b3cd;
        }
        0x1b3cd => {
            // block 1b3cd
            #[cfg(feature = "trace")] eprintln!("f_0001b316 pc 1b3cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1b3cd lea rax,[rbx+r15*4]
            rax = rbx.wrapping_add(r15.wrapping_mul(4));
            // 1b3d1 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 1b3d5 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 1b3d9 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // 1b3db mov DWORD PTR [rbx+r13*4],eax
            m.w32(rbx.wrapping_add(r13.wrapping_mul(4)), ((rax as u32 as u64)) as u32);
            // 1b3df lea r13,[rbx+r13*4+0x4]
            r13 = rbx.wrapping_add(r13.wrapping_mul(4)).wrapping_add(0x4);
            // 1b3e4 mov rdi,rbx
            rdi = rbx;
            // 1b3e7 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1b3eb mov rsi,r15
            rsi = r15;
            // 1b3ee mov rdx,r14
            rdx = r14;
            // 1b3f1 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b3f6 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 1b3fa mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // 1b3fe mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // 1b403 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1b407 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // 1b40b mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1b40f mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 1b414 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // 1b418 mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 1b41c mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // 1b420 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1b424 call 0xeb984
            { let r = { let t = m.env.stub_va(0x151); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1b429 add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // 1b42d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1b42e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1b430 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 1b432 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 1b434 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 1b436 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1b437 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xdfb36 size=292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000dfb36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xdfb36;
    loop {
        match pc {
        0xdfb36 => {
            // block dfb36
            #[cfg(feature = "trace")] eprintln!("f_000dfb36 pc dfb36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfb36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // dfb37 mov rbp,rsp
            rbp = rsp;
            // dfb3a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // dfb3c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // dfb3e push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // dfb40 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // dfb42 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // dfb43 sub rsp,0x58
            { let (r, f) = sub::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // dfb47 mov QWORD PTR [rbp-0x60],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rsi) as u64);
            // dfb4b mov r12,rdi
            r12 = rdi;
            // dfb4e mov rsi,QWORD PTR [r12]
            rsi = m.r64(r12) as u64;
            // dfb52 mov rbx,QWORD PTR [r12+0x8]
            rbx = m.r64(r12.wrapping_add(0x8)) as u64;
            // dfb57 sub rbx,rsi
            { let (r, f) = sub::<64>(rbx, rsi);
              rbx = r;
              fl = f; }
            // dfb5a sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // dfb5e inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // dfb61 mov rax,rbx
            rax = rbx;
            // dfb64 shr rax,0x3d
            { let (r, f) = shr::<64>(rax, 0x3du64, fl); rax = r; fl = f; }
            // dfb68 je 0xdfb76
            if fl.e() { pc = 0xdfb76; continue; }
            pc = 0xdfb6a;
        }
        0xdfb6a => {
            // block dfb6a
            #[cfg(feature = "trace")] eprintln!("f_000dfb36 pc dfb6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfb6a mov rdi,r12
            rdi = r12;
            // dfb6d call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0xdfb76 => {
            // block dfb76
            #[cfg(feature = "trace")] eprintln!("f_000dfb36 pc dfb76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfb76 lea rax,[r12+0x10]
            rax = r12.wrapping_add(0x10);
            // dfb7b mov r15,QWORD PTR [r12+0x10]
            r15 = m.r64(r12.wrapping_add(0x10)) as u64;
            // dfb80 mov QWORD PTR [rbp-0x68],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (r15) as u64);
            // dfb84 sub r15,rsi
            { let (r, f) = sub::<64>(r15, rsi);
              r15 = r;
              fl = f; }
            // dfb87 mov rcx,r15
            rcx = r15;
            // dfb8a sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // dfb8e movabs rdx,0xffffffffffffffe
            rdx = 0xfff_ffff_ffff_fffeu64;
            // dfb98 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // dfb9b ja 0xdfbaa
            if fl.a() { pc = 0xdfbaa; continue; }
            pc = 0xdfb9d;
        }
        0xdfb9d => {
            // block dfb9d
            #[cfg(feature = "trace")] eprintln!("f_000dfb36 pc dfb9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfb9d sar r15,0x2
            { let (r, f) = sar::<64>(r15, 0x2u64, fl); r15 = r; fl = f; }
            // dfba1 cmp r15,rbx
            fl = sub::<64>(r15, rbx).1;
            // dfba4 cmovb r15,rbx
            if fl.b() { r15 = rbx; }
            // dfba8 jmp 0xdfbb4
            pc = 0xdfbb4; continue;
        }
        0xdfbaa => {
            // block dfbaa
            #[cfg(feature = "trace")] eprintln!("f_000dfb36 pc dfbaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfbaa movabs r15,0x1fffffffffffffff
            r15 = 0x1fff_ffff_ffff_ffffu64;
            pc = 0xdfbb4;
        }
        0xdfbb4 => {
            // block dfbb4
            #[cfg(feature = "trace")] eprintln!("f_000dfb36 pc dfbb4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfbb4 mov r14,QWORD PTR [r12+0x8]
            r14 = m.r64(r12.wrapping_add(0x8)) as u64;
            // dfbb9 mov QWORD PTR [rbp-0x70],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r14) as u64);
            // dfbbd sub r14,rsi
            { let (r, f) = sub::<64>(r14, rsi);
              r14 = r;
              fl = f; }
            // dfbc0 mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // dfbc4 mov r13,r14
            r13 = r14;
            // dfbc7 sar r13,0x3
            { let (r, f) = sar::<64>(r13, 0x3u64, fl); r13 = r; fl = f; }
            // dfbcb mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // dfbd3 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // dfbd7 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // dfbd9 test r15,r15
            fl = logic::<64>(r15 & r15);
            // dfbdc je 0xdfbee
            if fl.e() { pc = 0xdfbee; continue; }
            pc = 0xdfbde;
        }
        0xdfbde => {
            // block dfbde
            #[cfg(feature = "trace")] eprintln!("f_000dfb36 pc dfbde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfbde lea rdi,[r15*8+0x0]
            rdi = r15.wrapping_mul(8);
            // dfbe6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfbeb mov rbx,rax
            rbx = rax;
            pc = 0xdfbee;
        }
        0xdfbee => {
            // block dfbee
            #[cfg(feature = "trace")] eprintln!("f_000dfb36 pc dfbee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // dfbee lea rax,[rbx+r15*8]
            rax = rbx.wrapping_add(r15.wrapping_mul(8));
            // dfbf2 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // dfbf6 mov rax,QWORD PTR [rbp-0x60]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // dfbfa mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // dfbfd mov QWORD PTR [rbx+r13*8],rax
            m.w64(rbx.wrapping_add(r13.wrapping_mul(8)), (rax) as u64);
            // dfc01 lea r13,[rbx+r13*8+0x8]
            r13 = rbx.wrapping_add(r13.wrapping_mul(8)).wrapping_add(0x8);
            // dfc06 mov rdi,rbx
            rdi = rbx;
            // dfc09 mov r15,QWORD PTR [rbp-0x58]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // dfc0d mov rsi,r15
            rsi = r15;
            // dfc10 mov rdx,r14
            rdx = r14;
            // dfc13 call 0xec21e
            { let r = { let t = m.env.stub_va(0x9eb); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfc18 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // dfc1c mov QWORD PTR [rbp-0x48],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r15) as u64);
            // dfc20 mov QWORD PTR [r12+0x8],r13
            m.w64(r12.wrapping_add(0x8), (r13) as u64);
            // dfc25 mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // dfc29 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // dfc2d mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // dfc31 mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // dfc36 mov rax,QWORD PTR [rbp-0x68]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff98)) as u64;
            // dfc3a mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // dfc3e mov QWORD PTR [rbp-0x50],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (r15) as u64);
            // dfc42 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // dfc46 call 0xebedc
            { let r = { let t = m.env.stub_va(0x6a9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // dfc4b add rsp,0x58
            { let (r, f) = add::<64>(rsp, 0x58u64);
              rsp = r;
              fl = f; }
            // dfc4f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // dfc50 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // dfc52 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // dfc54 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // dfc56 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // dfc58 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // dfc59 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe414c size=292
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x781
pub fn f_000e414c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xe414c;
    loop {
        match pc {
        0xe414c => {
            // block e414c
            #[cfg(feature = "trace")] eprintln!("f_000e414c pc e414c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e414c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e414d mov rbp,rsp
            rbp = rsp;
            // e4150 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e4152 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e4154 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // e4156 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e4158 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e4159 sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e415d mov QWORD PTR [rbp-0x58],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rsi) as u64);
            // e4161 mov r15,rdi
            r15 = rdi;
            // e4164 movabs r12,0xaaaaaaaaaaaaaaa
            r12 = 0xaaa_aaaa_aaaa_aaaau64;
            // e416e mov rax,QWORD PTR [r15]
            rax = m.r64(r15) as u64;
            // e4171 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // e4175 sub rbx,rax
            { let (r, f) = sub::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // e4178 sar rbx,0x3
            { let (r, f) = sar::<64>(rbx, 0x3u64, fl); rbx = r; fl = f; }
            // e417c movabs r13,0xaaaaaaaaaaaaaaab
            r13 = 0xaaaa_aaaa_aaaa_aaabu64;
            // e4186 imul rbx,r13
            { let (r, f) = imul::<64>(rbx, r13); rbx = r; fl = f; }
            // e418a inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e418d cmp rbx,r12
            fl = sub::<64>(rbx, r12).1;
            // e4190 jbe 0xe419d
            if fl.be() { pc = 0xe419d; continue; }
            pc = 0xe4192;
        }
        0xe4192 => {
            // block e4192
            #[cfg(feature = "trace")] eprintln!("f_000e414c pc e4192 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4192 mov rdi,r15
            rdi = r15;
            // e4195 call 0xebfb4
            { let _ = { let t = m.env.stub_va(0x781); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::default(); }
        }
        0xe419d => {
            // block e419d
            #[cfg(feature = "trace")] eprintln!("f_000e414c pc e419d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e419d lea rcx,[r15+0x10]
            rcx = r15.wrapping_add(0x10);
            // e41a1 mov rdx,QWORD PTR [r15+0x10]
            rdx = m.r64(r15.wrapping_add(0x10)) as u64;
            // e41a5 sub rdx,rax
            { let (r, f) = sub::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // e41a8 sar rdx,0x3
            { let (r, f) = sar::<64>(rdx, 0x3u64, fl); rdx = r; fl = f; }
            // e41ac imul rdx,r13
            { let (r, f) = imul::<64>(rdx, r13); rdx = r; fl = f; }
            // e41b0 movabs rsi,0x555555555555554
            rsi = 0x555_5555_5555_5554u64;
            // e41ba cmp rdx,rsi
            fl = sub::<64>(rdx, rsi).1;
            // e41bd ja 0xe41cc
            if fl.a() { pc = 0xe41cc; continue; }
            pc = 0xe41bf;
        }
        0xe41bf => {
            // block e41bf
            #[cfg(feature = "trace")] eprintln!("f_000e414c pc e41bf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e41bf add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // e41c2 cmp rdx,rbx
            fl = sub::<64>(rdx, rbx).1;
            // e41c5 cmovb rdx,rbx
            if fl.b() { rdx = rbx; }
            // e41c9 mov r12,rdx
            r12 = rdx;
            pc = 0xe41cc;
        }
        0xe41cc => {
            // block e41cc
            #[cfg(feature = "trace")] eprintln!("f_000e414c pc e41cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e41cc mov r14,QWORD PTR [r15+0x8]
            r14 = m.r64(r15.wrapping_add(0x8)) as u64;
            // e41d0 sub r14,rax
            { let (r, f) = sub::<64>(r14, rax);
              r14 = r;
              fl = f; }
            // e41d3 sar r14,0x3
            { let (r, f) = sar::<64>(r14, 0x3u64, fl); r14 = r; fl = f; }
            // e41d7 imul r14,r13
            { let (r, f) = imul::<64>(r14, r13); r14 = r; fl = f; }
            // e41db mov QWORD PTR [rbp-0x38],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (0x0u64) as u64);
            // e41e3 mov QWORD PTR [rbp-0x30],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rcx) as u64);
            // e41e7 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e41e9 test r12,r12
            fl = logic::<64>(r12 & r12);
            // e41ec je 0xe4202
            if fl.e() { pc = 0xe4202; continue; }
            pc = 0xe41ee;
        }
        0xe41ee => {
            // block e41ee
            #[cfg(feature = "trace")] eprintln!("f_000e414c pc e41ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e41ee lea rax,[r12*8+0x0]
            rax = r12.wrapping_mul(8);
            // e41f6 lea rdi,[rax+rax*2]
            rdi = rax.wrapping_add(rax.wrapping_mul(2));
            // e41fa call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e41ff mov rbx,rax
            rbx = rax;
            pc = 0xe4202;
        }
        0xe4202 => {
            // block e4202
            #[cfg(feature = "trace")] eprintln!("f_000e414c pc e4202 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4202 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // e4206 lea r14,[r14+r14*2]
            r14 = r14.wrapping_add(r14.wrapping_mul(2));
            // e420a lea rdi,[rbx+r14*8]
            rdi = rbx.wrapping_add(r14.wrapping_mul(8));
            // e420e mov QWORD PTR [rbp-0x40],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rdi) as u64);
            // e4212 mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // e4216 lea rax,[r12+r12*2]
            rax = r12.wrapping_add(r12.wrapping_mul(2));
            // e421a lea rax,[rbx+rax*8]
            rax = rbx.wrapping_add(rax.wrapping_mul(8));
            // e421e mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // e4222 mov rsi,QWORD PTR [rbp-0x58]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // e4226 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e422b lea rax,[rbx+r14*8+0x18]
            rax = rbx.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x18);
            // e4230 mov QWORD PTR [rbp-0x40],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rax) as u64);
            // e4234 lea rbx,[rbp-0x50]
            rbx = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e4238 mov rdi,r15
            rdi = r15;
            // e423b mov rsi,rbx
            rsi = rbx;
            // e423e call 0xebe94
            { let r = { let t = m.env.stub_va(0x661); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4243 mov rdi,rbx
            rdi = rbx;
            // e4246 call 0xebef4
            { let r = { let t = m.env.stub_va(0x6c1); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e424b add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // e424f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e4250 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e4252 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // e4254 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e4256 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e4258 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4259 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe425a => {
            // block e425a
            #[cfg(feature = "trace")] eprintln!("f_000e414c pc e425a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e425a mov rbx,rax
            rbx = rax;
            // e425d lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // e4261 call 0xebef4
            { let r = { let t = m.env.stub_va(0x6c1); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e4266 mov rdi,rbx
            rdi = rbx;
            // e4269 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
