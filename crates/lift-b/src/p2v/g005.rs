//! Mechanically lifted functions (scripts/lift_run.py), module g005. Each one is checked against the original
//! in the world harness (tests/world_g005.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe7eea size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7eea<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe7eea;
    loop {
        match pc {
        0xe7eea => {
            // block e7eea
            // e7eea push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7eeb mov rbp,rsp
            rbp = rsp;
            // e7eee pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7eef jmp 0xe7d04
            { let r = { let t = m.env.code_va(0xe7d04); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe807c size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e807c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe807c;
    loop {
        match pc {
        0xe807c => {
            // block e807c
            // e807c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e807d mov rbp,rsp
            rbp = rsp;
            // e8080 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e8081 jmp 0xe7ef4
            { let r = { let t = m.env.code_va(0xe7ef4); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea434 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea434<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xea434;
    loop {
        match pc {
        0xea434 => {
            // block ea434
            // ea434 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea435 mov rbp,rsp
            rbp = rsp;
            // ea438 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea439 jmp 0xea43e
            { let r = { let t = m.env.code_va(0xea43e); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xea598 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ea598<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xea598;
    loop {
        match pc {
        0xea598 => {
            // block ea598
            // ea598 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ea599 mov rbp,rsp
            rbp = rsp;
            // ea59c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ea59d jmp 0xea4f0
            { let r = { let t = m.env.code_va(0xea4f0); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7528 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d7528<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd7528;
    loop {
        match pc {
        0xd7528 => {
            // block d7528
            // d7528 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7529 mov rbp,rsp
            rbp = rsp;
            // d752c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d752e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d752f mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d7532 mov rbx,rdi
            rbx = rdi;
            // d7535 call 0xd8956
            { let r = { let t = m.env.code_va(0xd8956); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d753a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d753c je 0xd757e
            if fl.e() { pc = 0xd757e; continue; }
            pc = 0xd753e;
        }
        0xd753e => {
            // block d753e
            // d753e mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d7541 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // d7546 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d754b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d754e mov rdi,rbx
            rdi = rbx;
            // d7551 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d7554 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7557 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d755a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d755f mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // d7564 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7569 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d756c mov rdi,rbx
            rdi = rbx;
            // d756f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7572 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d7575 mov rdi,rbx
            rdi = rbx;
            // d7578 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd757e;
        }
        0xd757e => {
            // block d757e
            // d757e mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d7580 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7581 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7583 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7584 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6894 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6894<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6894;
    loop {
        match pc {
        0xb6894 => {
            // block b6894
            // b6894 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6895 mov rbp,rsp
            rbp = rsp;
            // b6898 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b689a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b689b mov rbx,rdi
            rbx = rdi;
            // b689e lea rax,[rip+0x8bb8b] # 0x142430
            rax = m.env.data_va(355325);
            // b68a5 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b68a8 cmp BYTE PTR [rbx+0xa],0x0
            fl = sub::<8>(m.r8(rbx.wrapping_add(0xa)) as u64, 0x0u64).1;
            // b68ac je 0xb68b6
            if fl.e() { pc = 0xb68b6; continue; }
            pc = 0xb68ae;
        }
        0xb68ae => {
            // block b68ae
            // b68ae mov rdi,rbx
            rdi = rbx;
            // b68b1 call 0x46b56
            { let r = { let t = m.env.code_va(0x46b56); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb68b6;
        }
        0xb68b6 => {
            // block b68b6
            // b68b6 lea rdi,[rbx+0x258]
            rdi = rbx.wrapping_add(0x258);
            // b68bd call 0xebcd8
            { let r = { let t = m.env.stub_va(0x4a5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b68c2 mov rdi,rbx
            rdi = rbx;
            // b68c5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b68c6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b68c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b68c9 jmp 0x45280
            { let r = { let t = m.env.code_va(0x45280); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
        0xb68ce => {
            // block b68ce
            // b68ce mov r14,rax
            r14 = rax;
            // b68d1 lea rdi,[rbx+0x258]
            rdi = rbx.wrapping_add(0x258);
            // b68d8 call 0xebcd8
            { let r = { let t = m.env.stub_va(0x4a5); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b68dd mov rdi,rbx
            rdi = rbx;
            // b68e0 call 0x45280
            { let r = { let t = m.env.code_va(0x45280); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b68e5 mov rdi,r14
            rdi = r14;
            // b68e8 call 0x10330
            { let r = { let t = m.env.code_va(0x10330); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb66ac size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b66ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb66ac;
    loop {
        match pc {
        0xb66ac => {
            // block b66ac
            // b66ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b66ad mov rbp,rsp
            rbp = rsp;
            // b66b0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b66b2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b66b4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b66b5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b66b6 mov rbx,rdi
            rbx = rdi;
            // b66b9 call 0x4522c
            { let r = { let t = m.env.code_va(0x4522c); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b66be lea rax,[rip+0x8bd6b] # 0x142430
            rax = m.env.data_va(355325);
            // b66c5 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // b66c8 lea r14,[rbx+0x258]
            r14 = rbx.wrapping_add(0x258);
            // b66cf mov QWORD PTR [rbx+0x268],0x0
            m.w64(rbx.wrapping_add(0x268), (0x0u64) as u64);
            // b66da mov QWORD PTR [rbx+0x260],0x0
            m.w64(rbx.wrapping_add(0x260), (0x0u64) as u64);
            // b66e5 mov QWORD PTR [rbx+0x258],0x0
            m.w64(rbx.wrapping_add(0x258), (0x0u64) as u64);
            // b66f0 mov BYTE PTR [rbx+0xa],0x0
            m.w8(rbx.wrapping_add(0xa), (0x0u64) as u8);
            // b66f4 mov rdi,rbx
            rdi = rbx;
            // b66f7 call 0xdc1f4
            { let r = { let t = m.env.code_va(0xdc1f4); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b66fc add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b6700 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b6701 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b6703 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b6705 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b6706 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb6707 => {
            // block b6707
            // b6707 mov r15,rax
            r15 = rax;
            // b670a mov rdi,r14
            rdi = r14;
            // b670d call 0xebcd8
            { let r = { let t = m.env.stub_va(0x4a5); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b6712 mov rdi,rbx
            rdi = rbx;
            // b6715 call 0x45280
            { let r = { let t = m.env.code_va(0x45280); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b671a mov rdi,r15
            rdi = r15;
            // b671d call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd74b0 size=120
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d74b0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd74b0;
    loop {
        match pc {
        0xd74b0 => {
            // block d74b0
            // d74b0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d74b1 mov rbp,rsp
            rbp = rsp;
            // d74b4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d74b5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d74b6 mov rbx,rdi
            rbx = rdi;
            // d74b9 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d74bb mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // d74bd cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // d74c0 ja 0xd7500
            if fl.a() { pc = 0xd7500; continue; }
            pc = 0xd74c2;
        }
        0xd74c2 => {
            // block d74c2
            // d74c2 lea rdx,[rip+0x3f] # 0xd7508
            rdx = m.env.code_va(0xd7508);
            // d74c9 movsxd rcx,DWORD PTR [rdx+rcx*4]
            rcx = sx::<32>(m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64;
            // d74cd add rcx,rdx
            { let (r, f) = add::<64>(rcx, rdx);
              rcx = r;
              fl = f; }
            // d74d0 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd74d2 => {
            // block d74d2
            // d74d2 mov eax,0x1
            rax = (0x1u64) as u32 as u64;
            // d74d7 jmp 0xd74f4
            pc = 0xd74f4; continue;
        }
        0xd74d9 => {
            // block d74d9
            // d74d9 mov edi,DWORD PTR [rbx+0x10]
            rdi = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // d74dc call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d74e1 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // d74e4 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d74e6 jmp 0xd74f4
            pc = 0xd74f4; continue;
        }
        0xd74e8 => {
            // block d74e8
            // d74e8 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // d74ed jmp 0xd74f4
            pc = 0xd74f4; continue;
        }
        0xd74ef => {
            // block d74ef
            // d74ef mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            pc = 0xd74f4;
        }
        0xd74f4 => {
            // block d74f4
            // d74f4 mov rdi,rbx
            rdi = rbx;
            // d74f7 mov esi,eax
            rsi = ((rax as u32 as u64)) as u32 as u64;
            // d74f9 call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d74fe mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xd7500;
        }
        0xd7500 => {
            // block d7500
            // d7500 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d7504 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7505 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7506 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd7b18 size=764
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x949
pub fn f_000d7b18<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd7b18;
    loop {
        match pc {
        0xd7b18 => {
            // block d7b18
            // d7b18 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d7b19 mov rbp,rsp
            rbp = rsp;
            // d7b1c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // d7b1e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d7b20 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // d7b22 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // d7b24 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d7b25 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d7b29 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // d7b2c mov r13,rdi
            r13 = rdi;
            // d7b2f mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d7b33 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7b39 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7b3b je 0xd7db4
            if fl.e() { pc = 0xd7db4; continue; }
            pc = 0xd7b41;
        }
        0xd7b41 => {
            // block d7b41
            // d7b41 mov rdi,r13
            rdi = r13;
            // d7b44 mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d7b47 call 0xd74b0
            { let r = { let t = m.env.code_va(0xd74b0); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7b4c mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d7b50 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7b55 mov edx,0x6
            rdx = (0x6u64) as u32 as u64;
            // d7b5a mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7b5f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7b62 mov rdi,r13
            rdi = r13;
            // d7b65 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7b68 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d7b6c mov rdi,r13
            rdi = r13;
            // d7b6f call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7b75 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d7b79 mov esi,0x3c
            rsi = (0x3cu64) as u32 as u64;
            // d7b7e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d7b80 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7b85 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7b88 mov rdi,r13
            rdi = r13;
            // d7b8b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7b8e mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d7b92 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // d7b98 mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d7b9d mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // d7ba2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7ba7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7baa mov rdi,r13
            rdi = r13;
            // d7bad call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7bb0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d7bb4 mov rdi,r13
            rdi = r13;
            // d7bb7 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7bbd lea ebx,[r14-0x1]
            rbx = (r14.wrapping_add(0xffff_ffff_ffff_ffff)) as u32 as u64;
            // d7bc1 cmp ebx,0x6
            fl = sub::<32>((rbx as u32 as u64), 0x6u64).1;
            // d7bc4 ja 0xd7bdf
            if fl.a() { pc = 0xd7bdf; continue; }
            pc = 0xd7bc6;
        }
        0xd7bc6 => {
            // block d7bc6
            // d7bc6 lea rax,[rip+0x20f] # 0xd7ddc
            rax = m.env.code_va(0xd7ddc);
            // d7bcd movsxd rcx,DWORD PTR [rax+rbx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64;
            // d7bd1 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d7bd4 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd7bd6 => {
            // block d7bd6
            // d7bd6 mov r12d,DWORD PTR [r13+0x368]
            r12 = (m.r32(r13.wrapping_add(0x368)) as u64) as u32 as u64;
            // d7bdd jmp 0xd7be6
            pc = 0xd7be6; continue;
        }
        0xd7bdf => {
            // block d7bdf
            // d7bdf mov r12d,DWORD PTR [r13+0x35c]
            r12 = (m.r32(r13.wrapping_add(0x35c)) as u64) as u32 as u64;
            pc = 0xd7be6;
        }
        0xd7be6 => {
            // block d7be6
            // d7be6 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d7bea mov esi,0x2880
            rsi = (0x2880u64) as u32 as u64;
            // d7bef mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7bf4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7bf7 mov rdi,r13
            rdi = r13;
            // d7bfa mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // d7bfd call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7c00 cmp ebx,0x6
            fl = sub::<32>((rbx as u32 as u64), 0x6u64).1;
            // d7c03 ja 0xd7c22
            if fl.a() { pc = 0xd7c22; continue; }
            pc = 0xd7c05;
        }
        0xd7c05 => {
            // block d7c05
            // d7c05 lea rax,[rip+0x1ec] # 0xd7df8
            rax = m.env.code_va(0xd7df8);
            // d7c0c movsxd rcx,DWORD PTR [rax+rbx*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64;
            // d7c10 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // d7c13 jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xd7c15 => {
            // block d7c15
            // d7c15 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // d7c19 lea rax,[r13+0x344]
            rax = r13.wrapping_add(0x344);
            // d7c20 jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7c22 => {
            // block d7c22
            // d7c22 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // d7c26 lea rax,[r13+0x340]
            rax = r13.wrapping_add(0x340);
            pc = 0xd7c2d;
        }
        0xd7c2d => {
            // block d7c2d
            // d7c2d test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // d7c30 je 0xd7d02
            if fl.e() { pc = 0xd7d02; continue; }
            pc = 0xd7c36;
        }
        0xd7c36 => {
            // block d7c36
            // d7c36 mov eax,DWORD PTR [rax]
            rax = (m.r32(rax) as u64) as u32 as u64;
            // d7c38 mov DWORD PTR [rbp-0x2c],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4), ((rax as u32 as u64)) as u32);
            // d7c3b xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d7c3d xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // d7c40 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            pc = 0xd7c43;
        }
        0xd7c43 => {
            // block d7c43
            // d7c43 mov edi,DWORD PTR [r13+0x10]
            rdi = (m.r32(r13.wrapping_add(0x10)) as u64) as u32 as u64;
            // d7c47 call 0xce3be
            { let r = { let t = m.env.code_va(0xce3be); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7c4c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d7c4e je 0xd7c84
            if fl.e() { pc = 0xd7c84; continue; }
            pc = 0xd7c50;
        }
        0xd7c50 => {
            // block d7c50
            // d7c50 mov eax,DWORD PTR [r13+0x33c]
            rax = (m.r32(r13.wrapping_add(0x33c)) as u64) as u32 as u64;
            // d7c57 imul eax,r14d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r14 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // d7c5b cmp eax,DWORD PTR [r13+0x338]
            fl = sub::<32>((rax as u32 as u64), m.r32(r13.wrapping_add(0x338)) as u64).1;
            // d7c62 jne 0xd7c84
            if fl.ne() { pc = 0xd7c84; continue; }
            pc = 0xd7c64;
        }
        0xd7c64 => {
            // block d7c64
            // d7c64 mov rax,QWORD PTR [rbp-0x38]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // d7c68 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d7c6b jne 0xd7c74
            if fl.ne() { pc = 0xd7c74; continue; }
            pc = 0xd7c6d;
        }
        0xd7c6d => {
            // block d7c6d
            // d7c6d mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d7c72 jmp 0xd7c79
            pc = 0xd7c79; continue;
        }
        0xd7c74 => {
            // block d7c74
            // d7c74 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            pc = 0xd7c79;
        }
        0xd7c79 => {
            // block d7c79
            // d7c79 mov rdi,r13
            rdi = r13;
            // d7c7c call 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7c81 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            pc = 0xd7c84;
        }
        0xd7c84 => {
            // block d7c84
            // d7c84 mov eax,DWORD PTR [r13+0x35c]
            rax = (m.r32(r13.wrapping_add(0x35c)) as u64) as u32 as u64;
            // d7c8b imul eax,r15d
            { let (r, f) = imul::<32>((rax as u32 as u64), (r15 as u32 as u64)); rax = (r) as u32 as u64; fl = f; }
            // d7c8f mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // d7c92 sub esi,eax
            { let (r, f) = sub::<32>((rsi as u32 as u64), (rax as u32 as u64));
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7c94 imul esi,DWORD PTR [r13+0x33c]
            { let (r, f) = imul::<32>((rsi as u32 as u64), m.r32(r13.wrapping_add(0x33c)) as u64); rsi = (r) as u32 as u64; fl = f; }
            // d7c9c add esi,DWORD PTR [rbp-0x2c]
            { let (r, f) = add::<32>((rsi as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7c9f mov rdi,r13
            rdi = r13;
            // d7ca2 call 0xd81de
            { let r = { let t = m.env.code_va(0xd81de); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ca7 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d7cab mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // d7cb0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7cb5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7cb8 mov rdi,r13
            rdi = r13;
            // d7cbb mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // d7cbe call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7cc1 cmp BYTE PTR [r13+0x378],0x0
            fl = sub::<8>(m.r8(r13.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d7cc9 jne 0xd7cf3
            if fl.ne() { pc = 0xd7cf3; continue; }
            pc = 0xd7ccb;
        }
        0xd7ccb => {
            // block d7ccb
            // d7ccb xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d7ccd mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // d7ccf div r12d
            { let (q, r) = div::<32>(rdx, rax, (r12 as u32 as u64)); rax = q; rdx = r; }
            // d7cd2 mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // d7cd4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7cd6 lea rdi,[rip+0x610c1] # 0x138d9e
            rdi = m.env.data_va(316779);
            // d7cdd mov esi,ecx
            rsi = ((rcx as u32 as u64)) as u32 as u64;
            // d7cdf call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7ce4 mov rax,QWORD PTR [rip+0x686f5] # 0x1403e0
            rax = m.r64(m.env.data_va(347053)) as u64;
            // d7ceb mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // d7cee call 0xec1be
            { let r = { let t = m.env.stub_va(0x98b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd7cf3;
        }
        0xd7cf3 => {
            // block d7cf3
            // d7cf3 inc r14d
            { let (r, f) = inc::<32>((r14 as u32 as u64), fl); r14 = (r) as u32 as u64; fl = f; }
            // d7cf6 add ebx,0x64
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x64u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // d7cf9 cmp r12d,r14d
            fl = sub::<32>((r12 as u32 as u64), (r14 as u32 as u64)).1;
            // d7cfc jne 0xd7c43
            if fl.ne() { pc = 0xd7c43; continue; }
            pc = 0xd7d02;
        }
        0xd7d02 => {
            // block d7d02
            // d7d02 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // d7d06 mov esi,0x2881
            rsi = (0x2881u64) as u32 as u64;
            // d7d0b mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d7d10 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d7d13 mov rdi,r13
            rdi = r13;
            // d7d16 mov edx,r12d
            rdx = ((r12 as u32 as u64)) as u32 as u64;
            // d7d19 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7d1c cmp BYTE PTR [r13+0x378],0x0
            fl = sub::<8>(m.r8(r13.wrapping_add(0x378)) as u64, 0x0u64).1;
            // d7d24 jne 0xd7d34
            if fl.ne() { pc = 0xd7d34; continue; }
            pc = 0xd7d26;
        }
        0xd7d26 => {
            // block d7d26
            // d7d26 lea rdi,[rip+0x61085] # 0x138db2
            rdi = m.env.data_va(316799);
            // d7d2d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // d7d2f call 0xec242
            { let r = { let t = m.env.stub_va(0xa0f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xd7d34;
        }
        0xd7d34 => {
            // block d7d34
            // d7d34 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d7d36 mov rdi,r13
            rdi = r13;
            // d7d39 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // d7d3d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d7d3e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // d7d40 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // d7d42 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d7d44 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // d7d46 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d7d47 jmp 0xd7528
            { let r = { let t = m.env.code_va(0xd7528); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
        0xd7d4c => {
            // block d7d4c
            // d7d4c mov r12d,DWORD PTR [r13+0x360]
            r12 = (m.r32(r13.wrapping_add(0x360)) as u64) as u32 as u64;
            // d7d53 jmp 0xd7be6
            pc = 0xd7be6; continue;
        }
        0xd7d58 => {
            // block d7d58
            // d7d58 mov r12d,DWORD PTR [r13+0x364]
            r12 = (m.r32(r13.wrapping_add(0x364)) as u64) as u32 as u64;
            // d7d5f jmp 0xd7be6
            pc = 0xd7be6; continue;
        }
        0xd7d64 => {
            // block d7d64
            // d7d64 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            pc = 0xd7d68;
        }
        0xd7d68 => {
            // block d7d68
            // d7d68 lea rax,[r13+0x354]
            rax = r13.wrapping_add(0x354);
            // d7d6f jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7d74 => {
            // block d7d74
            // d7d74 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            pc = 0xd7d78;
        }
        0xd7d78 => {
            // block d7d78
            // d7d78 lea rax,[r13+0x358]
            rax = r13.wrapping_add(0x358);
            // d7d7f jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7d84 => {
            // block d7d84
            // d7d84 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            pc = 0xd7d88;
        }
        0xd7d88 => {
            // block d7d88
            // d7d88 lea rax,[r13+0x348]
            rax = r13.wrapping_add(0x348);
            // d7d8f jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7d94 => {
            // block d7d94
            // d7d94 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // d7d98 lea rax,[r13+0x34c]
            rax = r13.wrapping_add(0x34c);
            // d7d9f jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7da4 => {
            // block d7da4
            // d7da4 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // d7da8 lea rax,[r13+0x350]
            rax = r13.wrapping_add(0x350);
            // d7daf jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7db4 => {
            // block d7db4
            // d7db4 mov edi,0x8
            rdi = (0x8u64) as u32 as u64;
            // d7db9 call 0xec152
            { let r = { let t = m.env.stub_va(0x91f); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d7dbe lea rcx,[rip+0x6100f] # 0x138dd4
            rcx = m.env.data_va(316833);
            // d7dc5 mov QWORD PTR [rax],rcx
            m.w64(rax, (rcx) as u64);
            // d7dc8 mov rsi,QWORD PTR [rip+0x68409] # 0x1401d8
            rsi = m.r64(m.env.data_va(346533)) as u64;
            // d7dcf xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // d7dd1 mov rdi,rax
            rdi = rax;
            // d7dd4 call 0xec17c
            { let _ = { let t = m.env.stub_va(0x949); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dbf4 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dbf4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2dbf4;
    loop {
        match pc {
        0x2dbf4 => {
            // block 2dbf4
            // 2dbf4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dbf5 mov rbp,rsp
            rbp = rsp;
            // 2dbf8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dbf9 jmp 0xec110
            { let r = { let t = m.env.stub_va(0x8dd); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa3aec size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a3aec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa3aec;
    loop {
        match pc {
        0xa3aec => {
            // block a3aec
            // a3aec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a3aed mov rbp,rsp
            rbp = rsp;
            // a3af0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a3af1 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa4a76 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a4a76<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa4a76;
    loop {
        match pc {
        0xa4a76 => {
            // block a4a76
            // a4a76 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a4a77 mov rbp,rsp
            rbp = rsp;
            // a4a7a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a4a7b jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa529c size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a529c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa529c;
    loop {
        match pc {
        0xa529c => {
            // block a529c
            // a529c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a529d mov rbp,rsp
            rbp = rsp;
            // a52a0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a52a1 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa5d12 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a5d12<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa5d12;
    loop {
        match pc {
        0xa5d12 => {
            // block a5d12
            // a5d12 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a5d13 mov rbp,rsp
            rbp = rsp;
            // a5d16 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a5d17 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa726e size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a726e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa726e;
    loop {
        match pc {
        0xa726e => {
            // block a726e
            // a726e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a726f mov rbp,rsp
            rbp = rsp;
            // a7272 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a7273 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xa8e7c size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000a8e7c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xa8e7c;
    loop {
        match pc {
        0xa8e7c => {
            // block a8e7c
            // a8e7c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // a8e7d mov rbp,rsp
            rbp = rsp;
            // a8e80 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a8e81 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xaa0c8 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000aa0c8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xaa0c8;
    loop {
        match pc {
        0xaa0c8 => {
            // block aa0c8
            // aa0c8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aa0c9 mov rbp,rsp
            rbp = rsp;
            // aa0cc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // aa0cd jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xab2c2 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ab2c2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xab2c2;
    loop {
        match pc {
        0xab2c2 => {
            // block ab2c2
            // ab2c2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ab2c3 mov rbp,rsp
            rbp = rsp;
            // ab2c6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ab2c7 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xaba10 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000aba10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xaba10;
    loop {
        match pc {
        0xaba10 => {
            // block aba10
            // aba10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aba11 mov rbp,rsp
            rbp = rsp;
            // aba14 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // aba15 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xabdb0 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000abdb0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xabdb0;
    loop {
        match pc {
        0xabdb0 => {
            // block abdb0
            // abdb0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // abdb1 mov rbp,rsp
            rbp = rsp;
            // abdb4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // abdb5 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xac150 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ac150<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xac150;
    loop {
        match pc {
        0xac150 => {
            // block ac150
            // ac150 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ac151 mov rbp,rsp
            rbp = rsp;
            // ac154 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ac155 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xaced6 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000aced6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xaced6;
    loop {
        match pc {
        0xaced6 => {
            // block aced6
            // aced6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aced7 mov rbp,rsp
            rbp = rsp;
            // aceda pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // acedb jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xad686 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ad686<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xad686;
    loop {
        match pc {
        0xad686 => {
            // block ad686
            // ad686 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ad687 mov rbp,rsp
            rbp = rsp;
            // ad68a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ad68b jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xae028 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ae028<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xae028;
    loop {
        match pc {
        0xae028 => {
            // block ae028
            // ae028 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ae029 mov rbp,rsp
            rbp = rsp;
            // ae02c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ae02d jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xaeb02 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000aeb02<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xaeb02;
    loop {
        match pc {
        0xaeb02 => {
            // block aeb02
            // aeb02 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aeb03 mov rbp,rsp
            rbp = rsp;
            // aeb06 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // aeb07 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xafd56 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000afd56<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xafd56;
    loop {
        match pc {
        0xafd56 => {
            // block afd56
            // afd56 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // afd57 mov rbp,rsp
            rbp = rsp;
            // afd5a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // afd5b jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb11b6 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b11b6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb11b6;
    loop {
        match pc {
        0xb11b6 => {
            // block b11b6
            // b11b6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b11b7 mov rbp,rsp
            rbp = rsp;
            // b11ba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b11bb jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb22e8 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b22e8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb22e8;
    loop {
        match pc {
        0xb22e8 => {
            // block b22e8
            // b22e8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b22e9 mov rbp,rsp
            rbp = rsp;
            // b22ec pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b22ed jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb2fa6 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b2fa6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb2fa6;
    loop {
        match pc {
        0xb2fa6 => {
            // block b2fa6
            // b2fa6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b2fa7 mov rbp,rsp
            rbp = rsp;
            // b2faa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b2fab jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb3e34 size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b3e34<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb3e34;
    loop {
        match pc {
        0xb3e34 => {
            // block b3e34
            // b3e34 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b3e35 mov rbp,rsp
            rbp = rsp;
            // b3e38 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b3e39 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb488c size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b488c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb488c;
    loop {
        match pc {
        0xb488c => {
            // block b488c
            // b488c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b488d mov rbp,rsp
            rbp = rsp;
            // b4890 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b4891 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb508a size=10
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b508a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb508a;
    loop {
        match pc {
        0xb508a => {
            // block b508a
            // b508a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b508b mov rbp,rsp
            rbp = rsp;
            // b508e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b508f jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14cd6 size=14
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014cd6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x14cd6;
    loop {
        match pc {
        0x14cd6 => {
            // block 14cd6
            // 14cd6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14cd7 mov rbp,rsp
            rbp = rsp;
            // 14cda mov rsi,QWORD PTR [rdi+0x8]
            rsi = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 14cde pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14cdf jmp 0xeb942
            { let r = { let t = m.env.stub_va(0x10f); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc39d0 size=14
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c39d0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc39d0;
    loop {
        match pc {
        0xc39d0 => {
            // block c39d0
            // c39d0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c39d1 mov rbp,rsp
            rbp = rsp;
            // c39d4 mov rsi,QWORD PTR [rdi+0x8]
            rsi = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c39d8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c39d9 jmp 0xebe70
            { let r = { let t = m.env.stub_va(0x63d); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc39ec size=14
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c39ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc39ec;
    loop {
        match pc {
        0xc39ec => {
            // block c39ec
            // c39ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c39ed mov rbp,rsp
            rbp = rsp;
            // c39f0 mov rsi,QWORD PTR [rdi+0x8]
            rsi = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c39f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c39f5 jmp 0xebdf8
            { let r = { let t = m.env.stub_va(0x5c5); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
