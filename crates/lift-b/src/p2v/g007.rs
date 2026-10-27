//! Mechanically lifted functions (scripts/lift_run.py), module g007. Each one is checked against the original
//! in the world harness (tests/world_g007.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xb9cae size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9cae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9cae;
    loop {
        match pc {
        0xb9cae => {
            // block b9cae
            // b9cae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9caf mov rbp,rsp
            rbp = rsp;
            // b9cb2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9cb5 je 0xb9cc2
            if fl.e() { pc = 0xb9cc2; continue; }
            pc = 0xb9cb7;
        }
        0xb9cb7 => {
            // block b9cb7
            // b9cb7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9cba call QWORD PTR [rax+0x1320]
            { let r = { let t = m.r64(rax.wrapping_add(0x1320)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9cc0 jmp 0xb9cc4
            pc = 0xb9cc4; continue;
        }
        0xb9cc2 => {
            // block b9cc2
            // b9cc2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9cc4;
        }
        0xb9cc4 => {
            // block b9cc4
            // b9cc4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9cc5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9cde size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9cde<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9cde;
    loop {
        match pc {
        0xb9cde => {
            // block b9cde
            // b9cde push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9cdf mov rbp,rsp
            rbp = rsp;
            // b9ce2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9ce5 je 0xb9cf2
            if fl.e() { pc = 0xb9cf2; continue; }
            pc = 0xb9ce7;
        }
        0xb9ce7 => {
            // block b9ce7
            // b9ce7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9cea call QWORD PTR [rax+0x1358]
            { let r = { let t = m.r64(rax.wrapping_add(0x1358)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9cf0 jmp 0xb9cf4
            pc = 0xb9cf4; continue;
        }
        0xb9cf2 => {
            // block b9cf2
            // b9cf2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9cf4;
        }
        0xb9cf4 => {
            // block b9cf4
            // b9cf4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9cf5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9d0e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9d0e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9d0e;
    loop {
        match pc {
        0xb9d0e => {
            // block b9d0e
            // b9d0e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9d0f mov rbp,rsp
            rbp = rsp;
            // b9d12 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9d15 je 0xb9d22
            if fl.e() { pc = 0xb9d22; continue; }
            pc = 0xb9d17;
        }
        0xb9d17 => {
            // block b9d17
            // b9d17 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9d1a call QWORD PTR [rax+0x1388]
            { let r = { let t = m.r64(rax.wrapping_add(0x1388)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9d20 jmp 0xb9d24
            pc = 0xb9d24; continue;
        }
        0xb9d22 => {
            // block b9d22
            // b9d22 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9d24;
        }
        0xb9d24 => {
            // block b9d24
            // b9d24 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9d25 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9d3e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9d3e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9d3e;
    loop {
        match pc {
        0xb9d3e => {
            // block b9d3e
            // b9d3e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9d3f mov rbp,rsp
            rbp = rsp;
            // b9d42 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9d45 je 0xb9d52
            if fl.e() { pc = 0xb9d52; continue; }
            pc = 0xb9d47;
        }
        0xb9d47 => {
            // block b9d47
            // b9d47 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9d4a call QWORD PTR [rax+0x13c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x13c8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9d50 jmp 0xb9d54
            pc = 0xb9d54; continue;
        }
        0xb9d52 => {
            // block b9d52
            // b9d52 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9d54;
        }
        0xb9d54 => {
            // block b9d54
            // b9d54 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9d55 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9d6e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9d6e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9d6e;
    loop {
        match pc {
        0xb9d6e => {
            // block b9d6e
            // b9d6e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9d6f mov rbp,rsp
            rbp = rsp;
            // b9d72 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9d75 je 0xb9d82
            if fl.e() { pc = 0xb9d82; continue; }
            pc = 0xb9d77;
        }
        0xb9d77 => {
            // block b9d77
            // b9d77 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9d7a call QWORD PTR [rax+0x13e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x13e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9d80 jmp 0xb9d84
            pc = 0xb9d84; continue;
        }
        0xb9d82 => {
            // block b9d82
            // b9d82 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9d84;
        }
        0xb9d84 => {
            // block b9d84
            // b9d84 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9d85 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9d9e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9d9e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9d9e;
    loop {
        match pc {
        0xb9d9e => {
            // block b9d9e
            // b9d9e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9d9f mov rbp,rsp
            rbp = rsp;
            // b9da2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9da5 je 0xb9db2
            if fl.e() { pc = 0xb9db2; continue; }
            pc = 0xb9da7;
        }
        0xb9da7 => {
            // block b9da7
            // b9da7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9daa call QWORD PTR [rax+0x1458]
            { let r = { let t = m.r64(rax.wrapping_add(0x1458)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9db0 jmp 0xb9db4
            pc = 0xb9db4; continue;
        }
        0xb9db2 => {
            // block b9db2
            // b9db2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9db4;
        }
        0xb9db4 => {
            // block b9db4
            // b9db4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9db5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9dce size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9dce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9dce;
    loop {
        match pc {
        0xb9dce => {
            // block b9dce
            // b9dce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9dcf mov rbp,rsp
            rbp = rsp;
            // b9dd2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9dd5 je 0xb9de2
            if fl.e() { pc = 0xb9de2; continue; }
            pc = 0xb9dd7;
        }
        0xb9dd7 => {
            // block b9dd7
            // b9dd7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9dda call QWORD PTR [rax+0x1488]
            { let r = { let t = m.r64(rax.wrapping_add(0x1488)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9de0 jmp 0xb9de4
            pc = 0xb9de4; continue;
        }
        0xb9de2 => {
            // block b9de2
            // b9de2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9de4;
        }
        0xb9de4 => {
            // block b9de4
            // b9de4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9de5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9dfe size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9dfe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9dfe;
    loop {
        match pc {
        0xb9dfe => {
            // block b9dfe
            // b9dfe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9dff mov rbp,rsp
            rbp = rsp;
            // b9e02 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9e05 je 0xb9e12
            if fl.e() { pc = 0xb9e12; continue; }
            pc = 0xb9e07;
        }
        0xb9e07 => {
            // block b9e07
            // b9e07 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9e0a call QWORD PTR [rax+0x14e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x14e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9e10 jmp 0xb9e14
            pc = 0xb9e14; continue;
        }
        0xb9e12 => {
            // block b9e12
            // b9e12 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9e14;
        }
        0xb9e14 => {
            // block b9e14
            // b9e14 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9e15 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9e2e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9e2e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9e2e;
    loop {
        match pc {
        0xb9e2e => {
            // block b9e2e
            // b9e2e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9e2f mov rbp,rsp
            rbp = rsp;
            // b9e32 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9e35 je 0xb9e42
            if fl.e() { pc = 0xb9e42; continue; }
            pc = 0xb9e37;
        }
        0xb9e37 => {
            // block b9e37
            // b9e37 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9e3a call QWORD PTR [rax+0x1510]
            { let r = { let t = m.r64(rax.wrapping_add(0x1510)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9e40 jmp 0xb9e44
            pc = 0xb9e44; continue;
        }
        0xb9e42 => {
            // block b9e42
            // b9e42 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9e44;
        }
        0xb9e44 => {
            // block b9e44
            // b9e44 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9e45 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9e5e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9e5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9e5e;
    loop {
        match pc {
        0xb9e5e => {
            // block b9e5e
            // b9e5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9e5f mov rbp,rsp
            rbp = rsp;
            // b9e62 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9e65 je 0xb9e72
            if fl.e() { pc = 0xb9e72; continue; }
            pc = 0xb9e67;
        }
        0xb9e67 => {
            // block b9e67
            // b9e67 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9e6a call QWORD PTR [rax+0x1570]
            { let r = { let t = m.r64(rax.wrapping_add(0x1570)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9e70 jmp 0xb9e74
            pc = 0xb9e74; continue;
        }
        0xb9e72 => {
            // block b9e72
            // b9e72 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9e74;
        }
        0xb9e74 => {
            // block b9e74
            // b9e74 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9e75 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9e8e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9e8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9e8e;
    loop {
        match pc {
        0xb9e8e => {
            // block b9e8e
            // b9e8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9e8f mov rbp,rsp
            rbp = rsp;
            // b9e92 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9e95 je 0xb9ea2
            if fl.e() { pc = 0xb9ea2; continue; }
            pc = 0xb9e97;
        }
        0xb9e97 => {
            // block b9e97
            // b9e97 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9e9a call QWORD PTR [rax+0x15a0]
            { let r = { let t = m.r64(rax.wrapping_add(0x15a0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9ea0 jmp 0xb9ea4
            pc = 0xb9ea4; continue;
        }
        0xb9ea2 => {
            // block b9ea2
            // b9ea2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9ea4;
        }
        0xb9ea4 => {
            // block b9ea4
            // b9ea4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9ea5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9ebe size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9ebe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9ebe;
    loop {
        match pc {
        0xb9ebe => {
            // block b9ebe
            // b9ebe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9ebf mov rbp,rsp
            rbp = rsp;
            // b9ec2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9ec5 je 0xb9ed2
            if fl.e() { pc = 0xb9ed2; continue; }
            pc = 0xb9ec7;
        }
        0xb9ec7 => {
            // block b9ec7
            // b9ec7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9eca call QWORD PTR [rax+0x15d0]
            { let r = { let t = m.r64(rax.wrapping_add(0x15d0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9ed0 jmp 0xb9ed4
            pc = 0xb9ed4; continue;
        }
        0xb9ed2 => {
            // block b9ed2
            // b9ed2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9ed4;
        }
        0xb9ed4 => {
            // block b9ed4
            // b9ed4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9ed5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9eee size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9eee<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9eee;
    loop {
        match pc {
        0xb9eee => {
            // block b9eee
            // b9eee push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9eef mov rbp,rsp
            rbp = rsp;
            // b9ef2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9ef5 je 0xb9f02
            if fl.e() { pc = 0xb9f02; continue; }
            pc = 0xb9ef7;
        }
        0xb9ef7 => {
            // block b9ef7
            // b9ef7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9efa call QWORD PTR [rax+0x1600]
            { let r = { let t = m.r64(rax.wrapping_add(0x1600)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9f00 jmp 0xb9f04
            pc = 0xb9f04; continue;
        }
        0xb9f02 => {
            // block b9f02
            // b9f02 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9f04;
        }
        0xb9f04 => {
            // block b9f04
            // b9f04 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9f05 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9f1e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9f1e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9f1e;
    loop {
        match pc {
        0xb9f1e => {
            // block b9f1e
            // b9f1e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9f1f mov rbp,rsp
            rbp = rsp;
            // b9f22 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9f25 je 0xb9f32
            if fl.e() { pc = 0xb9f32; continue; }
            pc = 0xb9f27;
        }
        0xb9f27 => {
            // block b9f27
            // b9f27 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9f2a call QWORD PTR [rax+0x1630]
            { let r = { let t = m.r64(rax.wrapping_add(0x1630)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9f30 jmp 0xb9f34
            pc = 0xb9f34; continue;
        }
        0xb9f32 => {
            // block b9f32
            // b9f32 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9f34;
        }
        0xb9f34 => {
            // block b9f34
            // b9f34 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9f35 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9fae size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9fae<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9fae;
    loop {
        match pc {
        0xb9fae => {
            // block b9fae
            // b9fae push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9faf mov rbp,rsp
            rbp = rsp;
            // b9fb2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9fb5 je 0xb9fc2
            if fl.e() { pc = 0xb9fc2; continue; }
            pc = 0xb9fb7;
        }
        0xb9fb7 => {
            // block b9fb7
            // b9fb7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9fba call QWORD PTR [rax+0x1760]
            { let r = { let t = m.r64(rax.wrapping_add(0x1760)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9fc0 jmp 0xb9fc4
            pc = 0xb9fc4; continue;
        }
        0xb9fc2 => {
            // block b9fc2
            // b9fc2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9fc4;
        }
        0xb9fc4 => {
            // block b9fc4
            // b9fc4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9fc5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9fde size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9fde<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9fde;
    loop {
        match pc {
        0xb9fde => {
            // block b9fde
            // b9fde push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9fdf mov rbp,rsp
            rbp = rsp;
            // b9fe2 test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // b9fe5 je 0xb9ff2
            if fl.e() { pc = 0xb9ff2; continue; }
            pc = 0xb9fe7;
        }
        0xb9fe7 => {
            // block b9fe7
            // b9fe7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9fea call QWORD PTR [rax+0x1790]
            { let r = { let t = m.r64(rax.wrapping_add(0x1790)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9ff0 jmp 0xb9ff4
            pc = 0xb9ff4; continue;
        }
        0xb9ff2 => {
            // block b9ff2
            // b9ff2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9ff4;
        }
        0xb9ff4 => {
            // block b9ff4
            // b9ff4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9ff5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9710 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9710<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9710;
    loop {
        match pc {
        0xb9710 => {
            // block b9710
            // b9710 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9711 mov rbp,rsp
            rbp = rsp;
            // b9714 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9717 je 0xb9724
            if fl.e() { pc = 0xb9724; continue; }
            pc = 0xb9719;
        }
        0xb9719 => {
            // block b9719
            // b9719 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b971c call QWORD PTR [rax+0x370]
            { let r = { let t = m.r64(rax.wrapping_add(0x370)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9722 jmp 0xb9726
            pc = 0xb9726; continue;
        }
        0xb9724 => {
            // block b9724
            // b9724 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9726;
        }
        0xb9726 => {
            // block b9726
            // b9726 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9727 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9740 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9740<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9740;
    loop {
        match pc {
        0xb9740 => {
            // block b9740
            // b9740 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9741 mov rbp,rsp
            rbp = rsp;
            // b9744 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9747 je 0xb9754
            if fl.e() { pc = 0xb9754; continue; }
            pc = 0xb9749;
        }
        0xb9749 => {
            // block b9749
            // b9749 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b974c call QWORD PTR [rax+0x390]
            { let r = { let t = m.r64(rax.wrapping_add(0x390)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9752 jmp 0xb9756
            pc = 0xb9756; continue;
        }
        0xb9754 => {
            // block b9754
            // b9754 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9756;
        }
        0xb9756 => {
            // block b9756
            // b9756 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9757 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9860 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9860<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9860;
    loop {
        match pc {
        0xb9860 => {
            // block b9860
            // b9860 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9861 mov rbp,rsp
            rbp = rsp;
            // b9864 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9867 je 0xb9874
            if fl.e() { pc = 0xb9874; continue; }
            pc = 0xb9869;
        }
        0xb9869 => {
            // block b9869
            // b9869 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b986c call QWORD PTR [rax+0x4b8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4b8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9872 jmp 0xb9876
            pc = 0xb9876; continue;
        }
        0xb9874 => {
            // block b9874
            // b9874 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9876;
        }
        0xb9876 => {
            // block b9876
            // b9876 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9877 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9890 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9890<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9890;
    loop {
        match pc {
        0xb9890 => {
            // block b9890
            // b9890 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9891 mov rbp,rsp
            rbp = rsp;
            // b9894 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9897 je 0xb98a4
            if fl.e() { pc = 0xb98a4; continue; }
            pc = 0xb9899;
        }
        0xb9899 => {
            // block b9899
            // b9899 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b989c call QWORD PTR [rax+0x4e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b98a2 jmp 0xb98a6
            pc = 0xb98a6; continue;
        }
        0xb98a4 => {
            // block b98a4
            // b98a4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb98a6;
        }
        0xb98a6 => {
            // block b98a6
            // b98a6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b98a7 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb98f0 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b98f0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb98f0;
    loop {
        match pc {
        0xb98f0 => {
            // block b98f0
            // b98f0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b98f1 mov rbp,rsp
            rbp = rsp;
            // b98f4 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b98f7 je 0xb9904
            if fl.e() { pc = 0xb9904; continue; }
            pc = 0xb98f9;
        }
        0xb98f9 => {
            // block b98f9
            // b98f9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b98fc call QWORD PTR [rax+0x548]
            { let r = { let t = m.r64(rax.wrapping_add(0x548)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9902 jmp 0xb9906
            pc = 0xb9906; continue;
        }
        0xb9904 => {
            // block b9904
            // b9904 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9906;
        }
        0xb9906 => {
            // block b9906
            // b9906 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9907 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9980 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9980<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9980;
    loop {
        match pc {
        0xb9980 => {
            // block b9980
            // b9980 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9981 mov rbp,rsp
            rbp = rsp;
            // b9984 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9987 je 0xb9994
            if fl.e() { pc = 0xb9994; continue; }
            pc = 0xb9989;
        }
        0xb9989 => {
            // block b9989
            // b9989 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b998c call QWORD PTR [rax+0x608]
            { let r = { let t = m.r64(rax.wrapping_add(0x608)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9992 jmp 0xb9996
            pc = 0xb9996; continue;
        }
        0xb9994 => {
            // block b9994
            // b9994 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9996;
        }
        0xb9996 => {
            // block b9996
            // b9996 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9997 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9afe size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9afe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9afe;
    loop {
        match pc {
        0xb9afe => {
            // block b9afe
            // b9afe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9aff mov rbp,rsp
            rbp = rsp;
            // b9b02 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9b05 je 0xb9b12
            if fl.e() { pc = 0xb9b12; continue; }
            pc = 0xb9b07;
        }
        0xb9b07 => {
            // block b9b07
            // b9b07 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9b0a call QWORD PTR [rax+0xf48]
            { let r = { let t = m.r64(rax.wrapping_add(0xf48)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9b10 jmp 0xb9b14
            pc = 0xb9b14; continue;
        }
        0xb9b12 => {
            // block b9b12
            // b9b12 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9b14;
        }
        0xb9b14 => {
            // block b9b14
            // b9b14 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9b15 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9b5e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9b5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9b5e;
    loop {
        match pc {
        0xb9b5e => {
            // block b9b5e
            // b9b5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9b5f mov rbp,rsp
            rbp = rsp;
            // b9b62 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9b65 je 0xb9b72
            if fl.e() { pc = 0xb9b72; continue; }
            pc = 0xb9b67;
        }
        0xb9b67 => {
            // block b9b67
            // b9b67 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9b6a call QWORD PTR [rax+0xff0]
            { let r = { let t = m.r64(rax.wrapping_add(0xff0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9b70 jmp 0xb9b74
            pc = 0xb9b74; continue;
        }
        0xb9b72 => {
            // block b9b72
            // b9b72 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9b74;
        }
        0xb9b74 => {
            // block b9b74
            // b9b74 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9b75 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9b8e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9b8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9b8e;
    loop {
        match pc {
        0xb9b8e => {
            // block b9b8e
            // b9b8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9b8f mov rbp,rsp
            rbp = rsp;
            // b9b92 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9b95 je 0xb9ba2
            if fl.e() { pc = 0xb9ba2; continue; }
            pc = 0xb9b97;
        }
        0xb9b97 => {
            // block b9b97
            // b9b97 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9b9a call QWORD PTR [rax+0x1020]
            { let r = { let t = m.r64(rax.wrapping_add(0x1020)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9ba0 jmp 0xb9ba4
            pc = 0xb9ba4; continue;
        }
        0xb9ba2 => {
            // block b9ba2
            // b9ba2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9ba4;
        }
        0xb9ba4 => {
            // block b9ba4
            // b9ba4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9ba5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9f4e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9f4e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9f4e;
    loop {
        match pc {
        0xb9f4e => {
            // block b9f4e
            // b9f4e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9f4f mov rbp,rsp
            rbp = rsp;
            // b9f52 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9f55 je 0xb9f62
            if fl.e() { pc = 0xb9f62; continue; }
            pc = 0xb9f57;
        }
        0xb9f57 => {
            // block b9f57
            // b9f57 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9f5a call QWORD PTR [rax+0x1660]
            { let r = { let t = m.r64(rax.wrapping_add(0x1660)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9f60 jmp 0xb9f64
            pc = 0xb9f64; continue;
        }
        0xb9f62 => {
            // block b9f62
            // b9f62 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9f64;
        }
        0xb9f64 => {
            // block b9f64
            // b9f64 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9f65 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9f7e size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9f7e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9f7e;
    loop {
        match pc {
        0xb9f7e => {
            // block b9f7e
            // b9f7e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9f7f mov rbp,rsp
            rbp = rsp;
            // b9f82 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9f85 je 0xb9f92
            if fl.e() { pc = 0xb9f92; continue; }
            pc = 0xb9f87;
        }
        0xb9f87 => {
            // block b9f87
            // b9f87 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9f8a call QWORD PTR [rax+0x1690]
            { let r = { let t = m.r64(rax.wrapping_add(0x1690)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9f90 jmp 0xb9f94
            pc = 0xb9f94; continue;
        }
        0xb9f92 => {
            // block b9f92
            // b9f92 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9f94;
        }
        0xb9f94 => {
            // block b9f94
            // b9f94 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9f95 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba046 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba046<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba046;
    loop {
        match pc {
        0xba046 => {
            // block ba046
            // ba046 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba047 mov rbp,rsp
            rbp = rsp;
            // ba04a test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // ba04d je 0xba05a
            if fl.e() { pc = 0xba05a; continue; }
            pc = 0xba04f;
        }
        0xba04f => {
            // block ba04f
            // ba04f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ba052 call QWORD PTR [rax+0x1838]
            { let r = { let t = m.r64(rax.wrapping_add(0x1838)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba058 jmp 0xba05c
            pc = 0xba05c; continue;
        }
        0xba05a => {
            // block ba05a
            // ba05a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xba05c;
        }
        0xba05c => {
            // block ba05c
            // ba05c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba05d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb9a22 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9a22<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb9a22;
    loop {
        match pc {
        0xb9a22 => {
            // block b9a22
            // b9a22 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9a23 mov rbp,rsp
            rbp = rsp;
            // b9a26 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b9a29 je 0xb9a36
            if fl.e() { pc = 0xb9a36; continue; }
            pc = 0xb9a2b;
        }
        0xb9a2b => {
            // block b9a2b
            // b9a2b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9a2e call QWORD PTR [rax+0xac0]
            { let r = { let t = m.r64(rax.wrapping_add(0xac0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9a34 jmp 0xb9a38
            pc = 0xb9a38; continue;
        }
        0xb9a36 => {
            // block b9a36
            // b9a36 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9a38;
        }
        0xb9a38 => {
            // block b9a38
            // b9a38 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b9a39 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3a32 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3a32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3a32;
    loop {
        match pc {
        0xc3a32 => {
            // block c3a32
            // c3a32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3a33 mov rbp,rsp
            rbp = rsp;
            // c3a36 mov rsi,QWORD PTR [rdi+0x8]
            rsi = m.r64(rdi.wrapping_add(0x8)) as u64;
            // c3a3a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3a3b jmp 0xebdce
            { let r = { let t = m.env.stub_va(0x59b); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5c9c0 size=25
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005c9c0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5c9c0;
    loop {
        match pc {
        0x5c9c0 => {
            // block 5c9c0
            // 5c9c0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5c9c1 mov rbp,rsp
            rbp = rsp;
            // 5c9c4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5c9c6 cmp edi,0x1
            fl = sub::<32>((rdi as u32 as u64), 0x1u64).1;
            // 5c9c9 ja 0x5c9d7
            if fl.a() { pc = 0x5c9d7; continue; }
            pc = 0x5c9cb;
        }
        0x5c9cb => {
            // block 5c9cb
            // 5c9cb mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // 5c9cd lea rcx,[rip+0xa417c] # 0x100b50
            rcx = m.env.data_va(86813);
            // 5c9d4 mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            pc = 0x5c9d7;
        }
        0x5c9d7 => {
            // block 5c9d7
            // 5c9d7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5c9d8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51d9a size=26
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051d9a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x51d9a;
    loop {
        match pc {
        0x51d9a => {
            // block 51d9a
            // 51d9a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51d9b mov rbp,rsp
            rbp = rsp;
            // 51d9e test rsi,rsi
            fl = logic::<64>(rsi & rsi);
            // 51da1 je 0x51daf
            if fl.e() { pc = 0x51daf; continue; }
            pc = 0x51da3;
        }
        0x51da3 => {
            // block 51da3
            // 51da3 mov eax,DWORD PTR [rip+0x1764df] # 0x1c8288
            rax = (m.r32(m.env.data_va(903765)) as u64) as u32 as u64;
            // 51da9 mov DWORD PTR [rsi],eax
            m.w32(rsi, ((rax as u32 as u64)) as u32);
            // 51dab mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 51dad jmp 0x51db1
            pc = 0x51db1; continue;
        }
        0x51daf => {
            // block 51daf
            // 51daf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x51db1;
        }
        0x51db1 => {
            // block 51db1
            // 51db1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51db2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2dbfe size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002dbfe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2dbfe;
    loop {
        match pc {
        0x2dbfe => {
            // block 2dbfe
            // 2dbfe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2dbff mov rbp,rsp
            rbp = rsp;
            // 2dc02 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2dc03 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2dc04 mov rbx,rdi
            rbx = rdi;
            // 2dc07 call 0xec110
            { let r = { let t = m.env.stub_va(0x8dd); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2dc0c mov rdi,rbx
            rdi = rbx;
            // 2dc0f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2dc13 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2dc14 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dc15 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2eab2 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002eab2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2eab2;
    loop {
        match pc {
        0x2eab2 => {
            // block 2eab2
            // 2eab2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2eab3 mov rbp,rsp
            rbp = rsp;
            // 2eab6 mov rax,QWORD PTR [rdi+0x48]
            rax = m.r64(rdi.wrapping_add(0x48)) as u64;
            // 2eaba xorpd xmm0,xmm0
            x0 = 0;
            // 2eabe test rax,rax
            fl = logic::<64>(rax & rax);
            // 2eac1 je 0x2eac8
            if fl.e() { pc = 0x2eac8; continue; }
            pc = 0x2eac3;
        }
        0x2eac3 => {
            // block 2eac3
            // 2eac3 movsd xmm0,QWORD PTR [rax+0x20]
            x0 = m.r64(rax.wrapping_add(0x20)) as u128;
            pc = 0x2eac8;
        }
        0x2eac8 => {
            // block 2eac8
            // 2eac8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2eac9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5b40a size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005b40a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5b40a;
    loop {
        match pc {
        0x5b40a => {
            // block 5b40a
            // 5b40a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5b40b mov rbp,rsp
            rbp = rsp;
            // 5b40e cmp edi,0x11
            fl = sub::<32>((rdi as u32 as u64), 0x11u64).1;
            // 5b411 ja 0x5b422
            if fl.a() { pc = 0x5b422; continue; }
            pc = 0x5b413;
        }
        0x5b413 => {
            // block 5b413
            // 5b413 movsxd rax,edi
            rax = sx::<32>((rdi as u32 as u64)) as u64;
            // 5b416 lea rcx,[rip+0xa6563] # 0x101980
            rcx = m.env.data_va(90445);
            // 5b41d mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 5b420 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5b421 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x5b422 => {
            // block 5b422
            // 5b422 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5b424 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5b425 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
