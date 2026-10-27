//! Mechanically lifted functions (scripts/lift_run.py), module g011. Each one is checked against the original
//! in the world harness (tests/world_g011.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xe84ac size=32
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e84ac<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe84ac;
    loop {
        match pc {
        0xe84ac => {
            // block e84ac
            // e84ac push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e84ad mov rbp,rsp
            rbp = rsp;
            // e84b0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e84b1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e84b2 mov rbx,rdi
            rbx = rdi;
            // e84b5 add rsi,0x2f0
            { let (r, f) = add::<64>(rsi, 0x2f0u64);
              rsi = r;
              fl = f; }
            // e84bc call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e84c1 mov rax,rbx
            rax = rbx;
            // e84c4 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e84c8 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e84c9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e84ca ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe84ec size=32
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e84ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe84ec;
    loop {
        match pc {
        0xe84ec => {
            // block e84ec
            // e84ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e84ed mov rbp,rsp
            rbp = rsp;
            // e84f0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e84f1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e84f2 mov rbx,rdi
            rbx = rdi;
            // e84f5 add rsi,0x2d8
            { let (r, f) = add::<64>(rsi, 0x2d8u64);
              rsi = r;
              fl = f; }
            // e84fc call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e8501 mov rax,rbx
            rax = rbx;
            // e8504 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e8508 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e8509 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e850a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xbba12 size=33
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000bba12<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xbba12;
    loop {
        match pc {
        0xbba12 => {
            // block bba12
            // bba12 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // bba13 mov rbp,rsp
            rbp = rsp;
            // bba16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // bba17 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // bba18 mov rbx,rdi
            rbx = rdi;
            // bba1b mov rax,QWORD PTR [rsi]
            rax = m.r64(rsi) as u64;
            // bba1e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // bba20 mov rdi,rsi
            rdi = rsi;
            // bba23 mov rsi,rbx
            rsi = rbx;
            // bba26 call QWORD PTR [rax+0x50]
            { let r = { let t = m.r64(rax.wrapping_add(0x50)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // bba29 mov rax,rbx
            rax = rbx;
            // bba2c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // bba30 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // bba31 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // bba32 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1f80 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001f80<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1f80;
    loop {
        match pc {
        0x1f80 => {
            // block 1f80
            // 1f80 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1f81 mov rbp,rsp
            rbp = rsp;
            // 1f84 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 1f87 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 1f8b mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 1f8d lea rsi,[rip+0xfd0ec] # 0xff080
            rsi = m.env.data_va(79949);
            // 1f94 mov esi,DWORD PTR [rsi+rcx*4]
            rsi = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 1f97 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 1f9c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 1f9f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 1fa0 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x507e size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000507e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x507e;
    loop {
        match pc {
        0x507e => {
            // block 507e
            // 507e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 507f mov rbp,rsp
            rbp = rsp;
            // 5082 mov rdx,rsi
            rdx = rsi;
            // 5085 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 5088 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 508c mov esi,0x9
            rsi = (0x9u64) as u32 as u64;
            // 5091 mov ecx,0x1800
            rcx = (0x1800u64) as u32 as u64;
            // 5096 mov r8d,0xb
            r8 = (0xbu64) as u32 as u64;
            // 509c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 509d jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6bb2 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006bb2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6bb2;
    loop {
        match pc {
        0x6bb2 => {
            // block 6bb2
            // 6bb2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6bb3 mov rbp,rsp
            rbp = rsp;
            // 6bb6 mov rdx,rsi
            rdx = rsi;
            // 6bb9 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 6bbc mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 6bc0 mov esi,0x30
            rsi = (0x30u64) as u32 as u64;
            // 6bc5 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // 6bca mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // 6bd0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6bd1 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6bf6 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006bf6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6bf6;
    loop {
        match pc {
        0x6bf6 => {
            // block 6bf6
            // 6bf6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6bf7 mov rbp,rsp
            rbp = rsp;
            // 6bfa mov rdx,rsi
            rdx = rsi;
            // 6bfd mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 6c00 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 6c04 mov esi,0x30
            rsi = (0x30u64) as u32 as u64;
            // 6c09 mov ecx,0xff00
            rcx = (0xff00u64) as u32 as u64;
            // 6c0e mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // 6c14 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6c15 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6d5a size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006d5a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6d5a;
    loop {
        match pc {
        0x6d5a => {
            // block 6d5a
            // 6d5a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6d5b mov rbp,rsp
            rbp = rsp;
            // 6d5e mov rdx,rsi
            rdx = rsi;
            // 6d61 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 6d64 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 6d68 mov esi,0x81
            rsi = (0x81u64) as u32 as u64;
            // 6d6d mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // 6d72 mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // 6d78 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6d79 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6fe4 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006fe4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x6fe4;
    loop {
        match pc {
        0x6fe4 => {
            // block 6fe4
            // 6fe4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6fe5 mov rbp,rsp
            rbp = rsp;
            // 6fe8 mov rdx,rsi
            rdx = rsi;
            // 6feb mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 6fee mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 6ff2 mov esi,0x83
            rsi = (0x83u64) as u32 as u64;
            // 6ff7 mov ecx,0x700
            rcx = (0x700u64) as u32 as u64;
            // 6ffc mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // 7002 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7003 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7068 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007068<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x7068;
    loop {
        match pc {
        0x7068 => {
            // block 7068
            // 7068 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7069 mov rbp,rsp
            rbp = rsp;
            // 706c mov rdx,rsi
            rdx = rsi;
            // 706f mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 7072 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 7076 mov esi,0x83
            rsi = (0x83u64) as u32 as u64;
            // 707b mov ecx,0x78000000
            rcx = (0x78000000u64) as u32 as u64;
            // 7080 mov r8d,0x1b
            r8 = (0x1bu64) as u32 as u64;
            // 7086 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7087 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7128 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007128<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x7128;
    loop {
        match pc {
        0x7128 => {
            // block 7128
            // 7128 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7129 mov rbp,rsp
            rbp = rsp;
            // 712c mov rdx,rsi
            rdx = rsi;
            // 712f mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 7132 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 7136 mov esi,0x83
            rsi = (0x83u64) as u32 as u64;
            // 713b mov ecx,0x30
            rcx = (0x30u64) as u32 as u64;
            // 7140 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // 7146 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7147 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x71c4 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000071c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x71c4;
    loop {
        match pc {
        0x71c4 => {
            // block 71c4
            // 71c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 71c5 mov rbp,rsp
            rbp = rsp;
            // 71c8 mov rdx,rsi
            rdx = rsi;
            // 71cb mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 71ce mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 71d2 mov esi,0x83
            rsi = (0x83u64) as u32 as u64;
            // 71d7 mov ecx,0x700000
            rcx = (0x700000u64) as u32 as u64;
            // 71dc mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 71e2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 71e3 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7248 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007248<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x7248;
    loop {
        match pc {
        0x7248 => {
            // block 7248
            // 7248 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7249 mov rbp,rsp
            rbp = rsp;
            // 724c mov rdx,rsi
            rdx = rsi;
            // 724f mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 7252 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 7256 mov esi,0x83
            rsi = (0x83u64) as u32 as u64;
            // 725b mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // 7260 mov r8d,0x6
            r8 = (0x6u64) as u32 as u64;
            // 7266 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7267 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x72cc size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000072cc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x72cc;
    loop {
        match pc {
        0x72cc => {
            // block 72cc
            // 72cc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 72cd mov rbp,rsp
            rbp = rsp;
            // 72d0 mov rdx,rsi
            rdx = rsi;
            // 72d3 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 72d6 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 72da mov esi,0x83
            rsi = (0x83u64) as u32 as u64;
            // 72df mov ecx,0x30000
            rcx = (0x30000u64) as u32 as u64;
            // 72e4 mov r8d,0x11
            r8 = (0x11u64) as u32 as u64;
            // 72ea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 72eb jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x738c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000738c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x738c;
    loop {
        match pc {
        0x738c => {
            // block 738c
            // 738c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 738d mov rbp,rsp
            rbp = rsp;
            // 7390 mov rdx,rsi
            rdx = rsi;
            // 7393 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 7396 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 739a mov esi,0xa8
            rsi = (0xa8u64) as u32 as u64;
            // 739f mov ecx,0x7000
            rcx = (0x7000u64) as u32 as u64;
            // 73a4 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            // 73aa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 73ab jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7410 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007410<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x7410;
    loop {
        match pc {
        0x7410 => {
            // block 7410
            // 7410 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7411 mov rbp,rsp
            rbp = rsp;
            // 7414 mov rdx,rsi
            rdx = rsi;
            // 7417 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 741a mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 741e mov esi,0xa8
            rsi = (0xa8u64) as u32 as u64;
            // 7423 mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // 7428 mov r8d,0x6
            r8 = (0x6u64) as u32 as u64;
            // 742e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 742f jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc648 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c648<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc648;
    loop {
        match pc {
        0xc648 => {
            // block c648
            // c648 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c649 mov rbp,rsp
            rbp = rsp;
            // c64c mov rdx,rsi
            rdx = rsi;
            // c64f mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c652 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // c656 mov esi,0x55
            rsi = (0x55u64) as u32 as u64;
            // c65b mov ecx,0xfff0000
            rcx = (0xfff0000u64) as u32 as u64;
            // c660 mov r8d,0x10
            r8 = (0x10u64) as u32 as u64;
            // c666 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c667 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc6c8 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c6c8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc6c8;
    loop {
        match pc {
        0xc6c8 => {
            // block c6c8
            // c6c8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c6c9 mov rbp,rsp
            rbp = rsp;
            // c6cc mov rdx,rsi
            rdx = rsi;
            // c6cf mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c6d2 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // c6d6 mov esi,0x7e
            rsi = (0x7eu64) as u32 as u64;
            // c6db mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // c6e0 mov r8d,0x2
            r8 = (0x2u64) as u32 as u64;
            // c6e6 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c6e7 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc74c size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c74c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc74c;
    loop {
        match pc {
        0xc74c => {
            // block c74c
            // c74c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c74d mov rbp,rsp
            rbp = rsp;
            // c750 mov rdx,rsi
            rdx = rsi;
            // c753 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c756 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // c75a mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // c75f mov ecx,0x30
            rcx = (0x30u64) as u32 as u64;
            // c764 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // c76a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c76b jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc7ec size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c7ec<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc7ec;
    loop {
        match pc {
        0xc7ec => {
            // block c7ec
            // c7ec push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c7ed mov rbp,rsp
            rbp = rsp;
            // c7f0 mov rdx,rsi
            rdx = rsi;
            // c7f3 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c7f6 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // c7fa mov esi,0x7c
            rsi = (0x7cu64) as u32 as u64;
            // c7ff mov ecx,0xf0
            rcx = (0xf0u64) as u32 as u64;
            // c804 mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // c80a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c80b jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc8f6 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c8f6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc8f6;
    loop {
        match pc {
        0xc8f6 => {
            // block c8f6
            // c8f6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c8f7 mov rbp,rsp
            rbp = rsp;
            // c8fa mov rdx,rsi
            rdx = rsi;
            // c8fd mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c900 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // c904 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // c909 mov ecx,0xc0000
            rcx = (0xc0000u64) as u32 as u64;
            // c90e mov r8d,0x12
            r8 = (0x12u64) as u32 as u64;
            // c914 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c915 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc97a size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c97a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc97a;
    loop {
        match pc {
        0xc97a => {
            // block c97a
            // c97a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c97b mov rbp,rsp
            rbp = rsp;
            // c97e mov rdx,rsi
            rdx = rsi;
            // c981 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // c984 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // c988 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // c98d mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // c992 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // c998 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c999 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc9fe size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c9fe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc9fe;
    loop {
        match pc {
        0xc9fe => {
            // block c9fe
            // c9fe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c9ff mov rbp,rsp
            rbp = rsp;
            // ca02 mov rdx,rsi
            rdx = rsi;
            // ca05 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ca08 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // ca0c mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // ca11 mov ecx,0x100
            rcx = (0x100u64) as u32 as u64;
            // ca16 mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // ca1c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ca1d jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xca42 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000ca42<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xca42;
    loop {
        match pc {
        0xca42 => {
            // block ca42
            // ca42 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ca43 mov rbp,rsp
            rbp = rsp;
            // ca46 mov rdx,rsi
            rdx = rsi;
            // ca49 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // ca4c mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // ca50 mov esi,0x7e
            rsi = (0x7eu64) as u32 as u64;
            // ca55 mov ecx,0x400
            rcx = (0x400u64) as u32 as u64;
            // ca5a mov r8d,0xa
            r8 = (0xau64) as u32 as u64;
            // ca60 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ca61 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcac6 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cac6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcac6;
    loop {
        match pc {
        0xcac6 => {
            // block cac6
            // cac6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cac7 mov rbp,rsp
            rbp = rsp;
            // caca mov rdx,rsi
            rdx = rsi;
            // cacd mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cad0 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cad4 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // cad9 mov ecx,0x40000000
            rcx = (0x40000000u64) as u32 as u64;
            // cade mov r8d,0x1e
            r8 = (0x1eu64) as u32 as u64;
            // cae4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cae5 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcb86 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cb86<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcb86;
    loop {
        match pc {
        0xcb86 => {
            // block cb86
            // cb86 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cb87 mov rbp,rsp
            rbp = rsp;
            // cb8a mov rdx,rsi
            rdx = rsi;
            // cb8d mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cb90 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cb94 mov esi,0xc0
            rsi = (0xc0u64) as u32 as u64;
            // cb99 mov ecx,0x10
            rcx = (0x10u64) as u32 as u64;
            // cb9e mov r8d,0x4
            r8 = (0x4u64) as u32 as u64;
            // cba4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cba5 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcc0a size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cc0a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcc0a;
    loop {
        match pc {
        0xcc0a => {
            // block cc0a
            // cc0a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cc0b mov rbp,rsp
            rbp = rsp;
            // cc0e mov rdx,rsi
            rdx = rsi;
            // cc11 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cc14 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cc18 mov esi,0xc0
            rsi = (0xc0u64) as u32 as u64;
            // cc1d mov ecx,0x40
            rcx = (0x40u64) as u32 as u64;
            // cc22 mov r8d,0x6
            r8 = (0x6u64) as u32 as u64;
            // cc28 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cc29 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcc8e size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cc8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcc8e;
    loop {
        match pc {
        0xcc8e => {
            // block cc8e
            // cc8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cc8f mov rbp,rsp
            rbp = rsp;
            // cc92 mov rdx,rsi
            rdx = rsi;
            // cc95 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cc98 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cc9c mov esi,0xc0
            rsi = (0xc0u64) as u32 as u64;
            // cca1 mov ecx,0x700
            rcx = (0x700u64) as u32 as u64;
            // cca6 mov r8d,0x8
            r8 = (0x8u64) as u32 as u64;
            // ccac pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ccad jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcd12 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cd12<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcd12;
    loop {
        match pc {
        0xcd12 => {
            // block cd12
            // cd12 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cd13 mov rbp,rsp
            rbp = rsp;
            // cd16 mov rdx,rsi
            rdx = rsi;
            // cd19 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cd1c mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cd20 mov esi,0xc0
            rsi = (0xc0u64) as u32 as u64;
            // cd25 mov ecx,0xff000000
            rcx = (0xff000000u64) as u32 as u64;
            // cd2a mov r8d,0x18
            r8 = (0x18u64) as u32 as u64;
            // cd30 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cd31 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xcdd2 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000cdd2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xcdd2;
    loop {
        match pc {
        0xcdd2 => {
            // block cdd2
            // cdd2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // cdd3 mov rbp,rsp
            rbp = rsp;
            // cdd6 mov rdx,rsi
            rdx = rsi;
            // cdd9 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // cddc mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // cde0 mov esi,0x80
            rsi = (0x80u64) as u32 as u64;
            // cde5 mov ecx,0x1e00000
            rcx = (0x1e00000u64) as u32 as u64;
            // cdea mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            // cdf0 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // cdf1 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43404 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043404<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43404;
    loop {
        match pc {
        0x43404 => {
            // block 43404
            // 43404 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43405 mov rbp,rsp
            rbp = rsp;
            // 43408 mov rdx,rsi
            rdx = rsi;
            // 4340b mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 4340e mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 43412 mov esi,0x7d
            rsi = (0x7du64) as u32 as u64;
            // 43417 mov ecx,0x20000000
            rcx = (0x20000000u64) as u32 as u64;
            // 4341c mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 43422 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43423 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe14a size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e14a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe14a;
    loop {
        match pc {
        0xe14a => {
            // block e14a
            // e14a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e14b mov rbp,rsp
            rbp = rsp;
            // e14e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e151 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e155 movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // e159 mov esi,0x162
            rsi = (0x162u64) as u32 as u64;
            // e15e mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // e163 mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // e169 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e16a jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1fe size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e1fe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe1fe;
    loop {
        match pc {
        0xe1fe => {
            // block e1fe
            // e1fe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1ff mov rbp,rsp
            rbp = rsp;
            // e202 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e205 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e209 movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // e20d mov esi,0x162
            rsi = (0x162u64) as u32 as u64;
            // e212 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // e217 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // e21d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e21e jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2b2 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000e2b2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2b2;
    loop {
        match pc {
        0xe2b2 => {
            // block e2b2
            // e2b2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2b3 mov rbp,rsp
            rbp = rsp;
            // e2b6 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2b9 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // e2bd movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // e2c1 mov esi,0x166
            rsi = (0x166u64) as u32 as u64;
            // e2c6 mov ecx,0x4
            rcx = (0x4u64) as u32 as u64;
            // e2cb mov r8d,0x2
            r8 = (0x2u64) as u32 as u64;
            // e2d1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2d2 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10002 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00010002<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x10002;
    loop {
        match pc {
        0x10002 => {
            // block 10002
            // 10002 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10003 mov rbp,rsp
            rbp = rsp;
            // 10006 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 10009 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 1000d movzx edx,sil
            rdx = ((rsi as u8 as u64)) as u32 as u64;
            // 10011 mov esi,0x7f
            rsi = (0x7fu64) as u32 as u64;
            // 10016 mov ecx,0x2
            rcx = (0x2u64) as u32 as u64;
            // 1001b mov r8d,0x1
            r8 = (0x1u64) as u32 as u64;
            // 10021 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 10022 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
