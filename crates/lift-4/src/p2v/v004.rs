//! Mechanically lifted functions (scripts/lift_run.py), module v004. Each one is checked against the original
//! in the world harness (tests/world_v004.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xc938 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c938<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc938;
    loop {
        match pc {
        0xc938 => {
            // block c938
            #[cfg(feature = "trace")] eprintln!("f_0000c938 pc c938 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c938 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c939 mov rbp,rsp
            rbp = rsp;
            // c93c mov rdx,rsi
            rdx = rsi;
            // c93f mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c942 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // c946 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // c94b mov ecx,0x1e00
            rcx = (0x1e00u64) as u32 as u64;
            // c950 mov r8d,0x9
            r8 = (0x9u64) as u32 as u64;
            // c956 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c957 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc9bc size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c9bc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc9bc;
    loop {
        match pc {
        0xc9bc => {
            // block c9bc
            #[cfg(feature = "trace")] eprintln!("f_0000c9bc pc c9bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c9bc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c9bd mov rbp,rsp
            rbp = rsp;
            // c9c0 mov rdx,rsi
            rdx = rsi;
            // c9c3 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c9c6 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // c9ca mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // c9cf mov ecx,0x30
            rcx = (0x30u64) as u32 as u64;
            // c9d4 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // c9da pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c9db jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xca20 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ca20<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xca20;
    loop {
        match pc {
        0xca20 => {
            // block ca20
            #[cfg(feature = "trace")] eprintln!("f_0000ca20 pc ca20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ca20 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ca21 mov rbp,rsp
            rbp = rsp;
            // ca24 mov rdx,rsi
            rdx = rsi;
            // ca27 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ca2a mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // ca2e mov esi,0x7e
            rsi = (0x7eu64) as u32 as u64;
            // ca33 mov ecx,0x800
            rcx = (0x800u64) as u32 as u64;
            // ca38 mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // ca3e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ca3f jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xca84 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ca84<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xca84;
    loop {
        match pc {
        0xca84 => {
            // block ca84
            #[cfg(feature = "trace")] eprintln!("f_0000ca84 pc ca84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ca84 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ca85 mov rbp,rsp
            rbp = rsp;
            // ca88 mov rdx,rsi
            rdx = rsi;
            // ca8b mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ca8e mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // ca92 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // ca97 mov ecx,0x4000
            rcx = (0x4000u64) as u32 as u64;
            // ca9c mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // caa2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // caa3 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcb44 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cb44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcb44;
    loop {
        match pc {
        0xcb44 => {
            // block cb44
            #[cfg(feature = "trace")] eprintln!("f_0000cb44 pc cb44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cb44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cb45 mov rbp,rsp
            rbp = rsp;
            // cb48 mov rdx,rsi
            rdx = rsi;
            // cb4b mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cb4e mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cb52 mov esi,0xc0
            rsi = (0xc0u64) as u32 as u64;
            // cb57 mov ecx,0xf0
            rcx = (0xf0u64) as u32 as u64;
            // cb5c mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // cb62 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cb63 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcbc8 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cbc8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcbc8;
    loop {
        match pc {
        0xcbc8 => {
            // block cbc8
            #[cfg(feature = "trace")] eprintln!("f_0000cbc8 pc cbc8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cbc8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cbc9 mov rbp,rsp
            rbp = rsp;
            // cbcc mov rdx,rsi
            rdx = rsi;
            // cbcf mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cbd2 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cbd6 mov esi,0xc0
            rsi = (0xc0u64) as u32 as u64;
            // cbdb mov ecx,0x20
            rcx = (0x20u64) as u32 as u64;
            // cbe0 mov r8d,0x5
            r8 = (0x5u64) as u32 as u64;
            // cbe6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cbe7 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcc4c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cc4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcc4c;
    loop {
        match pc {
        0xcc4c => {
            // block cc4c
            #[cfg(feature = "trace")] eprintln!("f_0000cc4c pc cc4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cc4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cc4d mov rbp,rsp
            rbp = rsp;
            // cc50 mov rdx,rsi
            rdx = rsi;
            // cc53 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cc56 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cc5a mov esi,0xc0
            rsi = (0xc0u64) as u32 as u64;
            // cc5f mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // cc64 mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // cc6a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cc6b jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xccd0 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ccd0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xccd0;
    loop {
        match pc {
        0xccd0 => {
            // block ccd0
            #[cfg(feature = "trace")] eprintln!("f_0000ccd0 pc ccd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ccd0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ccd1 mov rbp,rsp
            rbp = rsp;
            // ccd4 mov rdx,rsi
            rdx = rsi;
            // ccd7 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ccda mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // ccde mov esi,0xc0
            rsi = (0xc0u64) as u32 as u64;
            // cce3 mov ecx,0xff0000
            rcx = (0xff0000u64) as u32 as u64;
            // cce8 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // ccee pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ccef jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcd90 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cd90<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcd90;
    loop {
        match pc {
        0xcd90 => {
            // block cd90
            #[cfg(feature = "trace")] eprintln!("f_0000cd90 pc cd90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // cd90 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cd91 mov rbp,rsp
            rbp = rsp;
            // cd94 mov rdx,rsi
            rdx = rsi;
            // cd97 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cd9a mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cd9e mov esi,0x80
            rsi = (0x80u64) as u32 as u64;
            // cda3 mov ecx,0xe000
            rcx = (0xe000u64) as u32 as u64;
            // cda8 mov r8d,0xd
            r8 = (0xdu64) as u32 as u64;
            // cdae pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cdaf jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd328 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000d328<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd328;
    loop {
        match pc {
        0xd328 => {
            // block d328
            #[cfg(feature = "trace")] eprintln!("f_0000d328 pc d328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d328 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d329 mov rbp,rsp
            rbp = rsp;
            // d32c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d32f mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // d333 movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // d337 mov esi,0x5f
            rsi = (0x5fu64) as u32 as u64;
            // d33c mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // d341 mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // d347 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d348 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1a4 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e1a4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe1a4;
    loop {
        match pc {
        0xe1a4 => {
            // block e1a4
            #[cfg(feature = "trace")] eprintln!("f_0000e1a4 pc e1a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1a4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1a5 mov rbp,rsp
            rbp = rsp;
            // e1a8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1ab mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e1af movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // e1b3 mov esi,0x162
            rsi = (0x162u64) as u32 as u64;
            // e1b8 mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // e1bd mov r8d,0x2
            r8 = (0x2u64) as u32 as u64;
            // e1c3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1c4 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe258 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e258<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe258;
    loop {
        match pc {
        0xe258 => {
            // block e258
            #[cfg(feature = "trace")] eprintln!("f_0000e258 pc e258 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e258 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e259 mov rbp,rsp
            rbp = rsp;
            // e25c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e25f mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e263 movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // e267 mov esi,0x166
            rsi = (0x166u64) as u32 as u64;
            // e26c mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // e271 mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // e277 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e278 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe30c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e30c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe30c;
    loop {
        match pc {
        0xe30c => {
            // block e30c
            #[cfg(feature = "trace")] eprintln!("f_0000e30c pc e30c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e30c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e30d mov rbp,rsp
            rbp = rsp;
            // e310 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e313 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e317 movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // e31b mov esi,0x166
            rsi = (0x166u64) as u32 as u64;
            // e320 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // e325 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // e32b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e32c jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1005c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001005c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1005c;
    loop {
        match pc {
        0x1005c => {
            // block 1005c
            #[cfg(feature = "trace")] eprintln!("f_0001005c pc 1005c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1005c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1005d mov rbp,rsp
            rbp = rsp;
            // 10060 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 10063 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 10067 movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // 1006b mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // 10070 mov ecx,0x1000000
            rcx = (0x1000000u64) as u32 as u64;
            // 10075 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 1007b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1007c jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6804c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0006804c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6804c;
    loop {
        match pc {
        0x6804c => {
            // block 6804c
            #[cfg(feature = "trace")] eprintln!("f_0006804c pc 6804c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6804c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6804d mov rbp,rsp
            rbp = rsp;
            // 68050 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 68052 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 68055 mov r8,QWORD PTR [rcx+0xb0]
            r8 = m.r64(rcx.wrapping_add(0xb0)) as u64;
            // 6805c mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 6805e lea rcx,[rdi+rcx*8+0x20]
            rcx = rdi.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20);
            // 68063 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 68068 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 6806a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6806b jmp r8
            { let r = { let t = r8; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x681be size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000681be<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x681be;
    loop {
        match pc {
        0x681be => {
            // block 681be
            #[cfg(feature = "trace")] eprintln!("f_000681be pc 681be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 681be push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 681bf mov rbp,rsp
            rbp = rsp;
            // 681c2 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 681c4 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 681c7 mov rcx,QWORD PTR [rcx+0xbb8]
            rcx = m.r64(rcx.wrapping_add(0xbb8)) as u64;
            // 681ce mov edx,edx
            rdx = ((rdx as u32 as u64)) as u32 as u64;
            // 681d0 lea rsi,[rip+0x99ae9] # 0x101cc0
            rsi = m.env.data_va(91277);
            // 681d7 mov esi,DWORD PTR [rsi+rdx*4]
            rsi = (m.r32(rsi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 681da mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 681dc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 681dd jmp rcx
            { let r = { let t = rcx; m.call_r(t, &[rdi, rsi, rdx, rcx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe7c30 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7c30<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe7c30;
    loop {
        match pc {
        0xe7c30 => {
            // block e7c30
            #[cfg(feature = "trace")] eprintln!("f_000e7c30 pc e7c30 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c30 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7c31 mov rbp,rsp
            rbp = rsp;
            // e7c34 cmp edi,0xb
            fl = sub::<32>((rdi as u32 as u64), 0xbu64).1;
            // e7c37 ja 0xe7c49
            if fl.a() { pc = 0xe7c49; continue; }
            pc = 0xe7c39;
        }
        0xe7c39 => {
            // block e7c39
            #[cfg(feature = "trace")] eprintln!("f_000e7c30 pc e7c39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c39 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // e7c3c lea rcx,[rip+0x5e68d] # 0x1462d0
            rcx = m.env.data_va(371357);
            // e7c43 mov rax,QWORD PTR [rcx+rax*8]
            rax = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // e7c47 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7c48 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe7c49 => {
            // block e7c49
            #[cfg(feature = "trace")] eprintln!("f_000e7c30 pc e7c49 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c49 lea rax,[rip+0x47c3d] # 0x12f88d
            rax = m.env.data_va(278618);
            // e7c50 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7c51 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe7c74 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7c74<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe7c74;
    loop {
        match pc {
        0xe7c74 => {
            // block e7c74
            #[cfg(feature = "trace")] eprintln!("f_000e7c74 pc e7c74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c74 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7c75 mov rbp,rsp
            rbp = rsp;
            // e7c78 cmp edi,0x11
            fl = sub::<32>((rdi as u32 as u64), 0x11u64).1;
            // e7c7b ja 0xe7c8d
            if fl.a() { pc = 0xe7c8d; continue; }
            pc = 0xe7c7d;
        }
        0xe7c7d => {
            // block e7c7d
            #[cfg(feature = "trace")] eprintln!("f_000e7c74 pc e7c7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c7d movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // e7c80 lea rcx,[rip+0x5e7a9] # 0x146430
            rcx = m.env.data_va(371709);
            // e7c87 mov rax,QWORD PTR [rcx+rax*8]
            rax = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // e7c8b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7c8c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe7c8d => {
            // block e7c8d
            #[cfg(feature = "trace")] eprintln!("f_000e7c74 pc e7c8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7c8d lea rax,[rip+0x51bd6] # 0x13986a
            rax = m.env.data_va(319543);
            // e7c94 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7c95 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2914 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002914<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2914;
    loop {
        match pc {
        0x2914 => {
            // block 2914
            #[cfg(feature = "trace")] eprintln!("f_00002914 pc 2914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2914 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2915 mov rbp,rsp
            rbp = rsp;
            // 2918 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 291b ja 0x2935
            if fl.a() { pc = 0x2935; continue; }
            pc = 0x291d;
        }
        0x291d => {
            // block 291d
            #[cfg(feature = "trace")] eprintln!("f_00002914 pc 291d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 291d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 2920 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 2925 mov ecx,0x300000
            rcx = (0x300000u64) as u32 as u64;
            // 292a mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 2930 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2933 jmp 0x2937
            pc = 0x2937; continue;
        }
        0x2935 => {
            // block 2935
            #[cfg(feature = "trace")] eprintln!("f_00002914 pc 2935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2935 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2937;
        }
        0x2937 => {
            // block 2937
            #[cfg(feature = "trace")] eprintln!("f_00002914 pc 2937 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2937 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2938 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4332e size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004332e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4332e;
    loop {
        match pc {
        0x4332e => {
            // block 4332e
            #[cfg(feature = "trace")] eprintln!("f_0004332e pc 4332e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4332e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4332f mov rbp,rsp
            rbp = rsp;
            // 43332 movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // 43336 xor edx,0x1
            { let r = (rdx as u32 as u64) ^ 0x1u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 43339 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 4333c mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 43340 mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // 43345 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // 4334a mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // 43350 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43351 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a42c size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a42c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5a42c;
    loop {
        match pc {
        0x5a42c => {
            // block 5a42c
            #[cfg(feature = "trace")] eprintln!("f_0005a42c pc 5a42c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a42c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a42d mov rbp,rsp
            rbp = rsp;
            // 5a430 dec edi
            { let (r, f) = dec::<32>((rdi as u32 as u64), fl); rdi = (r) as u32 as u64; fl = f; }
            // 5a432 cmp edi,0x11
            fl = sub::<32>((rdi as u32 as u64), 0x11u64).1;
            // 5a435 ja 0x5a448
            if fl.a() { pc = 0x5a448; continue; }
            pc = 0x5a437;
        }
        0x5a437 => {
            // block 5a437
            #[cfg(feature = "trace")] eprintln!("f_0005a42c pc 5a437 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a437 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 5a43a lea rcx,[rip+0xa67ef] # 0x100c30
            rcx = m.env.data_va(87037);
            // 5a441 movsd xmm0,QWORD PTR [rcx+rax*8]
            x0 = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u128;
            // 5a446 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a447 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5a448 => {
            // block 5a448
            #[cfg(feature = "trace")] eprintln!("f_0005a42c pc 5a448 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a448 movsd xmm0,QWORD PTR [rip+0xa6328] # 0x100778
            x0 = m.r64(m.env.data_va(85829)) as u128;
            // 5a450 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a451 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd6d74 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d6d74<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd6d74;
    loop {
        match pc {
        0xd6d74 => {
            // block d6d74
            #[cfg(feature = "trace")] eprintln!("f_000d6d74 pc d6d74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6d74 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d6d75 mov rbp,rsp
            rbp = rsp;
            // d6d78 mov BYTE PTR [rdi+0x378],0x1
            m.w8(rdi.wrapping_add(0x378), (0x1u64) as u8);
            // d6d7f mov rdi,QWORD PTR [rdi+0x6b8]
            rdi = m.r64(rdi.wrapping_add(0x6b8)) as u64;
            // d6d86 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // d6d89 je 0xd6d97
            if fl.e() { pc = 0xd6d97; continue; }
            pc = 0xd6d8b;
        }
        0xd6d8b => {
            // block d6d8b
            #[cfg(feature = "trace")] eprintln!("f_000d6d74 pc d6d8b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6d8b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d6d8e mov rax,QWORD PTR [rax+0x40]
            rax = m.r64(rax.wrapping_add(0x40)) as u64;
            // d6d92 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // d6d94 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6d95 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0xd6d97 => {
            // block d6d97
            #[cfg(feature = "trace")] eprintln!("f_000d6d74 pc d6d97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d6d97 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d6d98 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x42456 size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042456<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x42456;
    loop {
        match pc {
        0x42456 => {
            // block 42456
            #[cfg(feature = "trace")] eprintln!("f_00042456 pc 42456 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42456 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42457 mov rbp,rsp
            rbp = rsp;
            // 4245a mov r9d,esi
            r9 = ((rsi as u32 as u64)) as u32 as u64;
            // 4245d mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 42460 mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 42464 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 42466 lea rdx,[rip+0xbdfb3] # 0x100420
            rdx = m.env.data_va(84973);
            // 4246d mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42470 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42475 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42478 mov edx,r9d
            rdx = ((r9 as u32 as u64)) as u32 as u64;
            // 4247b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4247c jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x424a6 size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000424a6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x424a6;
    loop {
        match pc {
        0x424a6 => {
            // block 424a6
            #[cfg(feature = "trace")] eprintln!("f_000424a6 pc 424a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 424a6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 424a7 mov rbp,rsp
            rbp = rsp;
            // 424aa mov r9,rsi
            r9 = rsi;
            // 424ad mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 424b0 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 424b4 mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 424b6 lea rdx,[rip+0xbdfb3] # 0x100470
            rdx = m.env.data_va(85053);
            // 424bd mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 424c0 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 424c5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 424c8 mov rdx,r9
            rdx = r9;
            // 424cb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 424cc jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8, r9], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43cba size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043cba<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43cba;
    loop {
        match pc {
        0x43cba => {
            // block 43cba
            #[cfg(feature = "trace")] eprintln!("f_00043cba pc 43cba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43cba push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43cbb mov rbp,rsp
            rbp = rsp;
            // 43cbe mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43cc1 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 43cc5 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43cc7 lea rsi,[rip+0xbc7a2] # 0x100470
            rsi = m.env.data_va(85053);
            // 43cce mov esi,DWORD PTR [rsi+rcx*4]
            rsi = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43cd1 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 43cd4 mov ecx,0x20000
            rcx = (0x20000u64) as u32 as u64;
            // 43cd9 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 43cdf pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43ce0 jmp rax
            { let r = { let t = rax; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5a70b size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005a70b<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5a70b;
    loop {
        match pc {
        0x5a70b => {
            // block 5a70b
            #[cfg(feature = "trace")] eprintln!("f_0005a70b pc 5a70b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a70b push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5a70c mov rbp,rsp
            rbp = rsp;
            // 5a70f lea eax,[rdi-0x50]
            rax = (rdi.wrapping_add(0xffff_ffff_ffff_ffb0)) as u32 as u64;
            // 5a712 cmp eax,0x1b
            fl = sub::<32>((rax as u32 as u64), 0x1bu64).1;
            // 5a715 ja 0x5a721
            if fl.a() { pc = 0x5a721; continue; }
            pc = 0x5a717;
        }
        0x5a717 => {
            // block 5a717
            #[cfg(feature = "trace")] eprintln!("f_0005a70b pc 5a717 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a717 mov ecx,0xff3ffff
            rcx = (0xff3ffffu64) as u32 as u64;
            // 5a71c bt ecx,eax
            fl.cf = bit::<32>((rcx as u32 as u64), (rax as u32 as u64));
            // 5a71f jb 0x5a725
            if fl.b() { pc = 0x5a725; continue; }
            pc = 0x5a721;
        }
        0x5a721 => {
            // block 5a721
            #[cfg(feature = "trace")] eprintln!("f_0005a70b pc 5a721 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a721 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5a723 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a724 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5a725 => {
            // block 5a725
            #[cfg(feature = "trace")] eprintln!("f_0005a70b pc 5a725 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5a725 cdqe
            rax = rax as u32 as i32 as i64 as u64;
            // 5a727 lea rcx,[rip+0xa6bf2] # 0x101320
            rcx = m.env.data_va(88813);
            // 5a72e mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5a731 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5a732 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe5a8e size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e5a8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe5a8e;
    loop {
        match pc {
        0xe5a8e => {
            // block e5a8e
            #[cfg(feature = "trace")] eprintln!("f_000e5a8e pc e5a8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e5a8f mov rbp,rsp
            rbp = rsp;
            // e5a92 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // e5a94 jne 0xe5a99
            if fl.ne() { pc = 0xe5a99; continue; }
            pc = 0xe5a96;
        }
        0xe5a96 => {
            // block e5a96
            #[cfg(feature = "trace")] eprintln!("f_000e5a8e pc e5a96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a96 mov esi,DWORD PTR [rdi+0x8]
            rsi = (m.r32(rdi.wrapping_add(0x8)) as u64) as u32 as u64;
            pc = 0xe5a99;
        }
        0xe5a99 => {
            // block e5a99
            #[cfg(feature = "trace")] eprintln!("f_000e5a8e pc e5a99 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5a99 dec esi
            { let (r, f) = dec::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // e5a9b cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // e5a9e ja 0xe5aaf
            if fl.a() { pc = 0xe5aaf; continue; }
            pc = 0xe5aa0;
        }
        0xe5aa0 => {
            // block e5aa0
            #[cfg(feature = "trace")] eprintln!("f_000e5a8e pc e5aa0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5aa0 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // e5aa3 lea rcx,[rip+0x44236] # 0x129ce0
            rcx = m.env.data_va(255149);
            // e5aaa mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // e5aad pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5aae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xe5aaf => {
            // block e5aaf
            #[cfg(feature = "trace")] eprintln!("f_000e5a8e pc e5aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e5aaf mov eax,0x1e
            rax = (0x1eu64) as u32 as u64;
            // e5ab4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5ab5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2de4e size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002de4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2de4e;
    loop {
        match pc {
        0x2de4e => {
            // block 2de4e
            #[cfg(feature = "trace")] eprintln!("f_0002de4e pc 2de4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2de4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2de4f mov rbp,rsp
            rbp = rsp;
            // 2de52 lea rdi,[rip+0x11928f] # 0x1470e8
            rdi = m.env.data_va(374965);
            // 2de59 lea rsi,[rip+0xfffffffffffffee6] # 0x2dd46
            rsi = m.env.code_va(0x2dd46);
            // 2de60 call 0xec266
            { let r = { let t = m.env.stub_va(0xa33); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2de65 mov rax,QWORD PTR [rip+0x193bbc] # 0x1c1a28
            rax = m.r64(m.env.data_va(877045)) as u64;
            // 2de6c test rax,rax
            fl = logic::<64>(rax & rax);
            // 2de6f je 0x2de74
            if fl.e() { pc = 0x2de74; continue; }
            pc = 0x2de71;
        }
        0x2de71 => {
            // block 2de71
            #[cfg(feature = "trace")] eprintln!("f_0002de4e pc 2de71 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2de71 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2de72 jmp rax
            { let r = { let t = rax; m.call_r(t, &[], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x2de74 => {
            // block 2de74
            #[cfg(feature = "trace")] eprintln!("f_0002de4e pc 2de74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2de74 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2de76 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2de77 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11930 size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011930<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x11930;
    loop {
        match pc {
        0x11930 => {
            // block 11930
            #[cfg(feature = "trace")] eprintln!("f_00011930 pc 11930 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11930 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11931 mov rbp,rsp
            rbp = rsp;
            // 11934 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11935 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11936 mov ebx,0xffffffff
            rbx = (0xffffffffu64) as u32 as u64;
            // 1193b lock xadd DWORD PTR [rdi+0x10],ebx
            { let a = m.r32(rdi.wrapping_add(0x10)) as u64; let b = (rbx as u32 as u64); let (r, f) = add::<32>(a, b); rbx = (a) as u32 as u64; m.w32(rdi.wrapping_add(0x10), (r) as u32); fl = f; }
            // 11940 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 11943 je 0x11950
            if fl.e() { pc = 0x11950; continue; }
            pc = 0x11945;
        }
        0x11945 => {
            // block 11945
            #[cfg(feature = "trace")] eprintln!("f_00011930 pc 11945 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11945 cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // 11948 jne 0x11950
            if fl.ne() { pc = 0x11950; continue; }
            pc = 0x1194a;
        }
        0x1194a => {
            // block 1194a
            #[cfg(feature = "trace")] eprintln!("f_00011930 pc 1194a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1194a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1194d call QWORD PTR [rax+0x60]
            { let r = { let t = m.r64(rax.wrapping_add(0x60)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x11950;
        }
        0x11950 => {
            // block 11950
            #[cfg(feature = "trace")] eprintln!("f_00011930 pc 11950 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11950 dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 11952 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // 11954 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 11958 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 11959 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1195a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ea5c size=46
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002ea5c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2ea5c;
    loop {
        match pc {
        0x2ea5c => {
            // block 2ea5c
            #[cfg(feature = "trace")] eprintln!("f_0002ea5c pc 2ea5c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ea5c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ea5d mov rbp,rsp
            rbp = rsp;
            // 2ea60 mov rax,QWORD PTR [rdi+0x48]
            rax = m.r64(rdi.wrapping_add(0x48)) as u64;
            // 2ea64 test rax,rax
            fl = logic::<64>(rax & rax);
            // 2ea67 je 0x2ea86
            if fl.e() { pc = 0x2ea86; continue; }
            pc = 0x2ea69;
        }
        0x2ea69 => {
            // block 2ea69
            #[cfg(feature = "trace")] eprintln!("f_0002ea5c pc 2ea69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ea69 mov rsi,QWORD PTR [rax+0x10]
            rsi = m.r64(rax.wrapping_add(0x10)) as u64;
            // 2ea6d test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 2ea70 je 0x2ea86
            if fl.e() { pc = 0x2ea86; continue; }
            pc = 0x2ea72;
        }
        0x2ea72 => {
            // block 2ea72
            #[cfg(feature = "trace")] eprintln!("f_0002ea5c pc 2ea72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ea72 cmp BYTE PTR [rsi],0x0
            fl = sub::<8>(m.r8(rsi) as u64, 0x0u64).1;
            // 2ea75 je 0x2ea86
            if fl.e() { pc = 0x2ea86; continue; }
            pc = 0x2ea77;
        }
        0x2ea77 => {
            // block 2ea77
            #[cfg(feature = "trace")] eprintln!("f_0002ea5c pc 2ea77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ea77 mov rdi,rdx
            rdi = rdx;
            // 2ea7a mov rdx,rcx
            rdx = rcx;
            // 2ea7d call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call_r(t, &[rdi, rsi, rdx], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ea82 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2ea84 jmp 0x2ea88
            pc = 0x2ea88; continue;
        }
        0x2ea86 => {
            // block 2ea86
            #[cfg(feature = "trace")] eprintln!("f_0002ea5c pc 2ea86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ea86 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2ea88;
        }
        0x2ea88 => {
            // block 2ea88
            #[cfg(feature = "trace")] eprintln!("f_0002ea5c pc 2ea88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ea88 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ea89 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c83e size=47
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c83e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c83e;
    loop {
        match pc {
        0x5c83e => {
            // block 5c83e
            #[cfg(feature = "trace")] eprintln!("f_0005c83e pc 5c83e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c83e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c83f mov rbp,rsp
            rbp = rsp;
            // 5c842 mov eax,0x1b
            rax = (0x1bu64) as u32 as u64;
            // 5c847 cmp edi,0x9
            fl = sub::<32>((rdi as u32 as u64), 0x9u64).1;
            // 5c84a jg 0x5c86b
            if fl.g() { pc = 0x5c86b; continue; }
            pc = 0x5c84c;
        }
        0x5c84c => {
            // block 5c84c
            #[cfg(feature = "trace")] eprintln!("f_0005c83e pc 5c84c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c84c cmp edi,0xa
            fl = sub::<32>((rdi as u32 as u64), 0xau64).1;
            // 5c84f ja 0x5c86b
            if fl.a() { pc = 0x5c86b; continue; }
            pc = 0x5c851;
        }
        0x5c851 => {
            // block 5c851
            #[cfg(feature = "trace")] eprintln!("f_0005c83e pc 5c851 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c851 mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5c853 lea rcx,[rip+0xa41c6] # 0x100a20
            rcx = m.env.data_va(86509);
            // 5c85a lea rdx,[rip+0xa418f] # 0x1009f0
            rdx = m.env.data_va(86461);
            // 5c861 test sil,sil
            fl = logic::<8>((rsi as u8 as u64) & (rsi as u8 as u64));
            // 5c864 cmovne rdx,rcx
            if fl.ne() { rdx = rcx; }
            // 5c868 mov eax,DWORD PTR [rdx+rax*4]
            rax = (m.r32(rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c86b;
        }
        0x5c86b => {
            // block 5c86b
            #[cfg(feature = "trace")] eprintln!("f_0005c83e pc 5c86b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5c86b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c86c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43904 size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043904<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43904;
    loop {
        match pc {
        0x43904 => {
            // block 43904
            #[cfg(feature = "trace")] eprintln!("f_00043904 pc 43904 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43904 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43905 mov rbp,rsp
            rbp = rsp;
            // 43908 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 4390b jle 0x43911
            if fl.le() { pc = 0x43911; continue; }
            pc = 0x4390d;
        }
        0x4390d => {
            // block 4390d
            #[cfg(feature = "trace")] eprintln!("f_00043904 pc 4390d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4390d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 4390f jmp 0x43931
            pc = 0x43931; continue;
        }
        0x43911 => {
            // block 43911
            #[cfg(feature = "trace")] eprintln!("f_00043904 pc 43911 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43911 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43914 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43916 lea rsi,[rip+0xbcaa3] # 0x1003c0
            rsi = m.env.data_va(84877);
            // 4391d mov esi,DWORD PTR [rsi+rcx*4]
            rsi = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43920 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 43923 mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // 43928 mov r8d,0x2
            r8 = (0x2u64) as u32 as u64;
            // 4392e call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43931;
        }
        0x43931 => {
            // block 43931
            #[cfg(feature = "trace")] eprintln!("f_00043904 pc 43931 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43931 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43932 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x439f4 size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000439f4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x439f4;
    loop {
        match pc {
        0x439f4 => {
            // block 439f4
            #[cfg(feature = "trace")] eprintln!("f_000439f4 pc 439f4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 439f4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 439f5 mov rbp,rsp
            rbp = rsp;
            // 439f8 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 439fb jle 0x43a01
            if fl.le() { pc = 0x43a01; continue; }
            pc = 0x439fd;
        }
        0x439fd => {
            // block 439fd
            #[cfg(feature = "trace")] eprintln!("f_000439f4 pc 439fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 439fd xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 439ff jmp 0x43a21
            pc = 0x43a21; continue;
        }
        0x43a01 => {
            // block 43a01
            #[cfg(feature = "trace")] eprintln!("f_000439f4 pc 43a01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a01 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43a04 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43a06 lea rsi,[rip+0xbc9b3] # 0x1003c0
            rsi = m.env.data_va(84877);
            // 43a0d mov esi,DWORD PTR [rsi+rcx*4]
            rsi = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43a10 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 43a13 mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // 43a18 mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // 43a1e call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi, rsi, rdx, rcx, r8], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43a21;
        }
        0x43a21 => {
            // block 43a21
            #[cfg(feature = "trace")] eprintln!("f_000439f4 pc 43a21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a21 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43a22 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba4e0 size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba4e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba4e0;
    loop {
        match pc {
        0xba4e0 => {
            // block ba4e0
            #[cfg(feature = "trace")] eprintln!("f_000ba4e0 pc ba4e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba4e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba4e1 mov rbp,rsp
            rbp = rsp;
            // ba4e4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba4e5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // ba4e6 mov rbx,rdi
            rbx = rdi;
            // ba4e9 mov rax,QWORD PTR [rip+0x85eb8] # 0x1403a8
            rax = m.r64(m.env.data_va(346997)) as u64;
            // ba4f0 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // ba4f4 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // ba4f7 lea rdi,[rbx+0x40]
            rdi = rbx.wrapping_add(0x40);
            // ba4fb call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba500 mov rdi,rbx
            rdi = rbx;
            // ba503 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // ba507 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba508 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba509 jmp 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe061a size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e061a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe061a;
    loop {
        match pc {
        0xe061a => {
            // block e061a
            #[cfg(feature = "trace")] eprintln!("f_000e061a pc e061a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e061a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e061b mov rbp,rsp
            rbp = rsp;
            // e061e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e061f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e0620 mov rbx,rdi
            rbx = rdi;
            // e0623 lea rax,[rip+0x65b9e] # 0x1461c8
            rax = m.env.data_va(371093);
            // e062a mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // e062d mov rdi,QWORD PTR [rbx+0x10]
            rdi = m.r64(rbx.wrapping_add(0x10)) as u64;
            // e0631 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // e0634 je 0xe063b
            if fl.e() { pc = 0xe063b; continue; }
            pc = 0xe0636;
        }
        0xe0636 => {
            // block e0636
            #[cfg(feature = "trace")] eprintln!("f_000e061a pc e0636 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0636 call 0xec1d0
            { let r = { let t = m.env.stub_va(0x99d); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe063b;
        }
        0xe063b => {
            // block e063b
            #[cfg(feature = "trace")] eprintln!("f_000e061a pc e063b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e063b mov rdi,rbx
            rdi = rbx;
            // e063e add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e0642 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0643 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0644 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14a58 size=50
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014a58<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x14a58;
    loop {
        match pc {
        0x14a58 => {
            // block 14a58
            #[cfg(feature = "trace")] eprintln!("f_00014a58 pc 14a58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a58 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14a59 mov rbp,rsp
            rbp = rsp;
            // 14a5c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 14a5f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14a62 lea rdi,[rip+0x1acf97] # 0x1c1a00
            rdi = m.env.data_va(877005);
            // 14a69 mov rsi,rax
            rsi = rax;
            // 14a6c call 0xeb93c
            { let r = { let t = m.env.stub_va(0x109); m.call_r(t, &[rdi, rsi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14a71 mov rcx,rax
            rcx = rax;
            // 14a74 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 14a79 lea rdx,[rip+0x1acf88] # 0x1c1a08
            rdx = m.env.data_va(877013);
            // 14a80 cmp rcx,rdx
            fl = sub::<64>(rcx, rdx).1;
            // 14a83 je 0x14a88
            if fl.e() { pc = 0x14a88; continue; }
            pc = 0x14a85;
        }
        0x14a85 => {
            // block 14a85
            #[cfg(feature = "trace")] eprintln!("f_00014a58 pc 14a85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a85 mov eax,DWORD PTR [rcx+0x38]
            rax = (m.r32(rcx.wrapping_add(0x38)) as u64) as u32 as u64;
            pc = 0x14a88;
        }
        0x14a88 => {
            // block 14a88
            #[cfg(feature = "trace")] eprintln!("f_00014a58 pc 14a88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14a88 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14a89 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2e084 size=52
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002e084<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2e084;
    loop {
        match pc {
        0x2e084 => {
            // block 2e084
            #[cfg(feature = "trace")] eprintln!("f_0002e084 pc 2e084 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2e084 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2e085 mov rbp,rsp
            rbp = rsp;
            // 2e088 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2e089 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2e08a mov rbx,rdi
            rbx = rdi;
            // 2e08d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 2e090 call QWORD PTR [rax+0xa8]
            { let r = { let t = m.r64(rax.wrapping_add(0xa8)) as u64; m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e096 movsxd rax,DWORD PTR [rbx+0x18]
            rax = sx::<32>(m.r32(rbx.wrapping_add(0x18)) as u64) as u64;
            // 2e09a movsxd rdi,DWORD PTR [rbx+0x14]
            rdi = sx::<32>(m.r32(rbx.wrapping_add(0x14)) as u64) as u64;
            // 2e09e imul rdi,rax
            { let (r, f) = imul::<64>(rdi, rax); rdi = r; fl = f; }
            // 2e0a2 call 0xec20c
            { let r = { let t = m.env.stub_va(0x9d9); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2e0a7 mov QWORD PTR [rbx+0x8],rax
            m.w64(rbx.wrapping_add(0x8), (rax) as u64);
            // 2e0ab mov BYTE PTR [rbx+0x61],0x1
            m.w8(rbx.wrapping_add(0x61), (0x1u64) as u8);
            // 2e0af xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 2e0b1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2e0b5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2e0b6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2e0b7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1065a size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001065a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1065a;
    loop {
        match pc {
        0x1065a => {
            // block 1065a
            #[cfg(feature = "trace")] eprintln!("f_0001065a pc 1065a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1065a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1065b mov rbp,rsp
            rbp = rsp;
            // 1065e mov rcx,QWORD PTR [rdi+0x8]
            rcx = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 10662 mov rax,QWORD PTR [rdi+0x10]
            rax = m.r64(rdi.wrapping_add(0x10)) as u64;
            // 10666 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 10669 je 0x10680
            if fl.e() { pc = 0x10680; continue; }
            pc = 0x1066b;
        }
        0x1066b => {
            // block 1066b
            #[cfg(feature = "trace")] eprintln!("f_0001065a pc 1066b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1066b lea rdx,[rax-0x8]
            rdx = rax.wrapping_add(0xffff_ffff_ffff_fff8);
            // 1066f sub rdx,rcx
            { let (r, f) = sub::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 10672 not rdx
            { let v = !rdx; rdx = v; }
            // 10675 and rdx,0xfffffffffffffff8
            { let r = rdx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 10679 add rdx,rax
            { let (r, f) = add::<64>(rdx, rax);
              rdx = r;
              fl = f; }
            // 1067c mov QWORD PTR [rdi+0x10],rdx
            m.w64(rdi.wrapping_add(0x10), (rdx) as u64);
            pc = 0x10680;
        }
        0x10680 => {
            // block 10680
            #[cfg(feature = "trace")] eprintln!("f_0001065a pc 10680 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10680 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // 10683 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 10686 je 0x1068e
            if fl.e() { pc = 0x1068e; continue; }
            pc = 0x10688;
        }
        0x10688 => {
            // block 10688
            #[cfg(feature = "trace")] eprintln!("f_0001065a pc 10688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10688 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10689 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x1068e => {
            // block 1068e
            #[cfg(feature = "trace")] eprintln!("f_0001065a pc 1068e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1068e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1068f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1999e size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0001999e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1999e;
    loop {
        match pc {
        0x1999e => {
            // block 1999e
            #[cfg(feature = "trace")] eprintln!("f_0001999e pc 1999e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1999e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1999f mov rbp,rsp
            rbp = rsp;
            // 199a2 mov rax,rdi
            rax = rdi;
            // 199a5 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 199a8 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 199ab je 0x199d1
            if fl.e() { pc = 0x199d1; continue; }
            pc = 0x199ad;
        }
        0x199ad => {
            // block 199ad
            #[cfg(feature = "trace")] eprintln!("f_0001999e pc 199ad rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 199ad mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 199b1 cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // 199b4 je 0x199cb
            if fl.e() { pc = 0x199cb; continue; }
            pc = 0x199b6;
        }
        0x199b6 => {
            // block 199b6
            #[cfg(feature = "trace")] eprintln!("f_0001999e pc 199b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 199b6 lea rdx,[rcx-0x4]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 199ba sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 199bd not rdx
            { let v = !rdx; rdx = v; }
            // 199c0 and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 199c4 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 199c7 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            pc = 0x199cb;
        }
        0x199cb => {
            // block 199cb
            #[cfg(feature = "trace")] eprintln!("f_0001999e pc 199cb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 199cb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 199cc jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x199d1 => {
            // block 199d1
            #[cfg(feature = "trace")] eprintln!("f_0001999e pc 199d1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 199d1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 199d2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x19a0a size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00019a0a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x19a0a;
    loop {
        match pc {
        0x19a0a => {
            // block 19a0a
            #[cfg(feature = "trace")] eprintln!("f_00019a0a pc 19a0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a0a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 19a0b mov rbp,rsp
            rbp = rsp;
            // 19a0e mov rax,rdi
            rax = rdi;
            // 19a11 mov rdi,QWORD PTR [rax]
            rdi = m.r64(rax) as u64;
            // 19a14 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 19a17 je 0x19a3d
            if fl.e() { pc = 0x19a3d; continue; }
            pc = 0x19a19;
        }
        0x19a19 => {
            // block 19a19
            #[cfg(feature = "trace")] eprintln!("f_00019a0a pc 19a19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a19 mov rcx,QWORD PTR [rax+0x8]
            rcx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 19a1d cmp rcx,rdi
            fl = sub::<64>(rcx, rdi).1;
            // 19a20 je 0x19a37
            if fl.e() { pc = 0x19a37; continue; }
            pc = 0x19a22;
        }
        0x19a22 => {
            // block 19a22
            #[cfg(feature = "trace")] eprintln!("f_00019a0a pc 19a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a22 lea rdx,[rcx-0x4]
            rdx = rcx.wrapping_add(0xffff_ffff_ffff_fffc);
            // 19a26 sub rdx,rdi
            { let (r, f) = sub::<64>(rdx, rdi);
              rdx = r;
              fl = f; }
            // 19a29 not rdx
            { let v = !rdx; rdx = v; }
            // 19a2c and rdx,0xfffffffffffffffc
            { let r = rdx & 0xffff_ffff_ffff_fffcu64; let f = logic::<64>(r);
              rdx = r;
              fl = f; }
            // 19a30 add rdx,rcx
            { let (r, f) = add::<64>(rdx, rcx);
              rdx = r;
              fl = f; }
            // 19a33 mov QWORD PTR [rax+0x8],rdx
            m.w64(rax.wrapping_add(0x8), (rdx) as u64);
            pc = 0x19a37;
        }
        0x19a37 => {
            // block 19a37
            #[cfg(feature = "trace")] eprintln!("f_00019a0a pc 19a37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a37 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19a38 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call_r(t, &[rdi], &[], [rdi, rsi, rdx, rcx, r8, r9]) }; return Ret::from_call(r); }
        }
        0x19a3d => {
            // block 19a3d
            #[cfg(feature = "trace")] eprintln!("f_00019a0a pc 19a3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 19a3d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 19a3e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
