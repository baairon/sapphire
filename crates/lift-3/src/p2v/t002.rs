//! Mechanically lifted functions (scripts/lift_run.py), module t002. Each one is checked against the original
//! in the world harness (tests/world_t002.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x31108 size=32
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00031108<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x31108;
    loop {
        match pc {
        0x31108 => {
            // block 31108
            #[cfg(feature = "trace")] eprintln!("f_00031108 pc 31108 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31108 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 31109 mov rbp,rsp
            rbp = rsp;
            // 3110c cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // 3110f ja 0x31120
            if fl.a() { pc = 0x31120; continue; }
            pc = 0x31111;
        }
        0x31111 => {
            // block 31111
            #[cfg(feature = "trace")] eprintln!("f_00031108 pc 31111 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31111 movsxd rax,esi
            rax = sx::<32>((rsi as u32 as u64)) as u64;
            // 31114 lea rcx,[rip+0xcece5] # 0xffe00
            rcx = m.env.data_va(83405);
            // 3111b mov eax,DWORD PTR [rcx+rax*4]
            rax = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 3111e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 3111f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x31120 => {
            // block 31120
            #[cfg(feature = "trace")] eprintln!("f_00031108 pc 31120 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 31120 mov eax,0xffffffff
            rax = (0xffffffffu64) as u32 as u64;
            // 31125 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 31126 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x32d7a size=32
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032d7a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x32d7a;
    loop {
        match pc {
        0x32d7a => {
            // block 32d7a
            #[cfg(feature = "trace")] eprintln!("f_00032d7a pc 32d7a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32d7a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32d7b mov rbp,rsp
            rbp = rsp;
            // 32d7e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 32d7f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 32d80 mov rbx,rdi
            rbx = rdi;
            // 32d83 movzx ecx,cl
            rcx = ((rcx as u8 as u64)) as u32 as u64;
            // 32d86 movzx r8d,r8b
            r8 = ((r8 as u8 as u64)) as u32 as u64;
            // 32d8a call 0x32c72
            { let r = { let t = m.env.code_va(0x32c72); m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 32d8f mov rax,rbx
            rax = rbx;
            // 32d92 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 32d96 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 32d97 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32d98 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x524e0 size=32
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000524e0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x524e0;
    loop {
        match pc {
        0x524e0 => {
            // block 524e0
            #[cfg(feature = "trace")] eprintln!("f_000524e0 pc 524e0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 524e0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 524e1 mov rbp,rsp
            rbp = rsp;
            // 524e4 lea rdi,[rip+0x175bbd] # 0x1c80a8
            rdi = m.env.data_va(903285);
            // 524eb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 524ec jmp 0x559a8
            { let r = { let t = m.env.code_va(0x559a8); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x67a size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000067a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x67a;
    loop {
        match pc {
        0x67a => {
            // block 67a
            #[cfg(feature = "trace")] eprintln!("f_0000067a pc 67a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 67a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 67b mov rbp,rsp
            rbp = rsp;
            // 67e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 67f push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 680 mov rbx,rsi
            rbx = rsi;
            // 683 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 686 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68c mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // 68e mov rsi,rbx
            rsi = rbx;
            // 691 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 695 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 696 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 697 jmp 0x19556
            { let r = { let t = m.env.code_va(0x19556); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0c76 size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0c76<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0c76;
    loop {
        match pc {
        0xe0c76 => {
            // block e0c76
            #[cfg(feature = "trace")] eprintln!("f_000e0c76 pc e0c76 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0c76 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0c77 mov rbp,rsp
            rbp = rsp;
            // e0c7a sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0c7e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0c81 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0c85 call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0c8b mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0c8e and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0c91 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0c95 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0c96 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0e8e size=34
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0e8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0e8e;
    loop {
        match pc {
        0xe0e8e => {
            // block e0e8e
            #[cfg(feature = "trace")] eprintln!("f_000e0e8e pc e0e8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0e8f mov rbp,rsp
            rbp = rsp;
            // e0e92 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e96 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0e99 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0e9d call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ea3 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0ea6 shr eax,0x1e
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x1eu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0ea9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0ead pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0eae ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2acbe size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002acbe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2acbe;
    loop {
        match pc {
        0x2acbe => {
            // block 2acbe
            #[cfg(feature = "trace")] eprintln!("f_0002acbe pc 2acbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2acbe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2acbf mov rbp,rsp
            rbp = rsp;
            // 2acc2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2acc3 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2acc4 mov rbx,rdi
            rbx = rdi;
            // 2acc7 call 0x2ace2
            { let r = { let t = m.env.code_va(0x2ace2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2accc add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2acd0 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2acd2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2acd4 mov rdi,rbx
            rdi = rbx;
            // 2acd7 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2acdb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2acdc pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2acdd jmp 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2ad16 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002ad16<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2ad16;
    loop {
        match pc {
        0x2ad16 => {
            // block 2ad16
            #[cfg(feature = "trace")] eprintln!("f_0002ad16 pc 2ad16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2ad16 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2ad17 mov rbp,rsp
            rbp = rsp;
            // 2ad1a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2ad1b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2ad1c mov rbx,rdi
            rbx = rdi;
            // 2ad1f call 0x2ace2
            { let r = { let t = m.env.code_va(0x2ace2); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2ad24 add rbx,0x8
            { let (r, f) = add::<64>(rbx, 0x8u64);
              rbx = r;
              fl = f; }
            // 2ad28 xor esi,esi
            { let r = (rsi as u32 as u64) ^ (rsi as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 2ad2a xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2ad2c mov rdi,rbx
            rdi = rbx;
            // 2ad2f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2ad33 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2ad34 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2ad35 jmp 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x683dc size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000683dc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x683dc;
    loop {
        match pc {
        0x683dc => {
            // block 683dc
            #[cfg(feature = "trace")] eprintln!("f_000683dc pc 683dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 683dc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 683dd mov rbp,rsp
            rbp = rsp;
            // 683e0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 683e1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 683e2 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 683e4 mov rbx,rdi
            rbx = rdi;
            // 683e7 lea rsi,[rip+0x15ff92] # 0x1c8380
            rsi = m.env.data_va(904013);
            // 683ee mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 683f0 call 0x68aa8
            { let r = { let t = m.env.code_va(0x68aa8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 683f5 mov rax,rbx
            rax = rbx;
            // 683f8 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 683fc pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 683fd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 683fe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0c98 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0c98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0c98;
    loop {
        match pc {
        0xe0c98 => {
            // block e0c98
            #[cfg(feature = "trace")] eprintln!("f_000e0c98 pc e0c98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0c98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0c99 mov rbp,rsp
            rbp = rsp;
            // e0c9c sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0ca0 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0ca3 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0ca7 call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0cad mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0cb0 shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0cb3 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0cb6 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0cba pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0cbb ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0cbc size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0cbc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0cbc;
    loop {
        match pc {
        0xe0cbc => {
            // block e0cbc
            #[cfg(feature = "trace")] eprintln!("f_000e0cbc pc e0cbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0cbc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0cbd mov rbp,rsp
            rbp = rsp;
            // e0cc0 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0cc4 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0cc7 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0ccb call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0cd1 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0cd4 shr eax,0x4
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x4u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0cd7 and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0cda add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0cde pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0cdf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0ce0 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0ce0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0ce0;
    loop {
        match pc {
        0xe0ce0 => {
            // block e0ce0
            #[cfg(feature = "trace")] eprintln!("f_000e0ce0 pc e0ce0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0ce0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0ce1 mov rbp,rsp
            rbp = rsp;
            // e0ce4 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0ce8 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0ceb lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0cef call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0cf5 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0cf8 shr eax,0xc
            { let (r, f) = shr::<32>((rax as u32 as u64), 0xcu64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0cfb and eax,0xf
            { let r = (rax as u32 as u64) & 0xfu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0cfe add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0d02 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0d03 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0e26 size=36
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0e26<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0e26;
    loop {
        match pc {
        0xe0e26 => {
            // block e0e26
            #[cfg(feature = "trace")] eprintln!("f_000e0e26 pc e0e26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e26 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0e27 mov rbp,rsp
            rbp = rsp;
            // e0e2a sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e2e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0e31 lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0e35 call QWORD PTR [rax+0x1ae8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0e3b mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0e3e shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e0e41 and eax,0x3
            { let r = (rax as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0e44 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0e49 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x452a8 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000452a8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x452a8;
    loop {
        match pc {
        0x452a8 => {
            // block 452a8
            #[cfg(feature = "trace")] eprintln!("f_000452a8 pc 452a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 452a8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 452a9 mov rbp,rsp
            rbp = rsp;
            // 452ac push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 452ad push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 452ae mov rbx,rdi
            rbx = rdi;
            // 452b1 lea rax,[rip+0xfc458] # 0x141710
            rax = m.env.data_va(351965);
            // 452b8 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 452bb call 0xd4ae8
            { let r = { let t = m.env.code_va(0xd4ae8); m.call(t, &[], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 452c0 mov rdi,rbx
            rdi = rbx;
            // 452c3 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 452c7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 452c8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 452c9 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68488 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068488<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68488;
    loop {
        match pc {
        0x68488 => {
            // block 68488
            #[cfg(feature = "trace")] eprintln!("f_00068488 pc 68488 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68488 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68489 mov rbp,rsp
            rbp = rsp;
            // 6848c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6848d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6848e mov rax,rsi
            rax = rsi;
            // 68491 mov rbx,rdi
            rbx = rdi;
            // 68494 lea rsi,[rip+0x15fee5] # 0x1c8380
            rsi = m.env.data_va(904013);
            // 6849b mov rdx,rax
            rdx = rax;
            // 6849e call 0x69328
            { let r = { let t = m.env.code_va(0x69328); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 684a3 mov rax,rbx
            rax = rbx;
            // 684a6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 684aa pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 684ab pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 684ac ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68500 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068500<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68500;
    loop {
        match pc {
        0x68500 => {
            // block 68500
            #[cfg(feature = "trace")] eprintln!("f_00068500 pc 68500 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68500 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68501 mov rbp,rsp
            rbp = rsp;
            // 68504 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 68505 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 68506 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 68508 mov rbx,rdi
            rbx = rdi;
            // 6850b movzx ecx,dl
            rcx = ((rdx as u8 as u64)) as u32 as u64;
            // 6850e lea rsi,[rip+0x15fe6b] # 0x1c8380
            rsi = m.env.data_va(904013);
            // 68515 mov edx,eax
            rdx = ((rax as u32 as u64)) as u32 as u64;
            // 68517 call 0x6947e
            { let r = { let t = m.env.code_va(0x6947e); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6851c mov rax,rbx
            rax = rbx;
            // 6851f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 68523 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 68524 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 68525 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7ffc size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7ffc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7ffc;
    loop {
        match pc {
        0xb7ffc => {
            // block b7ffc
            #[cfg(feature = "trace")] eprintln!("f_000b7ffc pc b7ffc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ffc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7ffd mov rbp,rsp
            rbp = rsp;
            // b8000 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8002 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8003 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b8006 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b8008 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b800b call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8011 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8013 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b8015 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b8018 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8019 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b801b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b801c jmp 0xd162b
            { let r = { let t = m.env.code_va(0xd162b); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8022 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8022<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb8022;
    loop {
        match pc {
        0xb8022 => {
            // block b8022
            #[cfg(feature = "trace")] eprintln!("f_000b8022 pc b8022 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8022 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8023 mov rbp,rsp
            rbp = rsp;
            // b8026 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8028 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8029 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b802c mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b802e mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b8031 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8037 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8039 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b803b mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b803e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b803f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8041 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8042 jmp 0xd1b6c
            { let r = { let t = m.env.code_va(0xd1b6c); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8048 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8048<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb8048;
    loop {
        match pc {
        0xb8048 => {
            // block b8048
            #[cfg(feature = "trace")] eprintln!("f_000b8048 pc b8048 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8048 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8049 mov rbp,rsp
            rbp = rsp;
            // b804c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b804e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b804f mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b8052 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b8054 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b8057 call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b805d mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b805f mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b8061 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b8064 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8065 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8067 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8068 jmp 0xd2238
            { let r = { let t = m.env.code_va(0xd2238); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb806e size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b806e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb806e;
    loop {
        match pc {
        0xb806e => {
            // block b806e
            #[cfg(feature = "trace")] eprintln!("f_000b806e pc b806e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b806e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b806f mov rbp,rsp
            rbp = rsp;
            // b8072 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8074 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8075 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // b8078 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b807a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b807d call QWORD PTR [rax+0x198]
            { let r = { let t = m.r64(rax.wrapping_add(0x198)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8083 mov edi,eax
            rdi = ((rax as u32 as u64)) as u32 as u64;
            // b8085 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // b8087 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // b808a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b808b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b808d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b808e jmp 0xd2245
            { let r = { let t = m.env.code_va(0xd2245); m.call(t, &[rdi, rsi, rdx], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe62fe size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e62fe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe62fe;
    loop {
        match pc {
        0xe62fe => {
            // block e62fe
            #[cfg(feature = "trace")] eprintln!("f_000e62fe pc e62fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e62fe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e62ff mov rbp,rsp
            rbp = rsp;
            // e6302 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e6304 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e6305 mov r14,rsi
            r14 = rsi;
            // e6308 mov rbx,rdi
            rbx = rdi;
            // e630b call 0xe6324
            { let r = { let t = m.env.code_va(0xe6324); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e6310 lea rsi,[rbx+0x38]
            rsi = rbx.wrapping_add(0x38);
            // e6314 mov rdi,r14
            rdi = r14;
            // e6317 call 0xec038
            { let r = { let t = m.env.stub_va(0x805); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e631c mov al,BYTE PTR [rbx+0xc]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0xc)) as u64) as u8 as u64);
            // e631f pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e6320 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e6322 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e6323 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe6428 size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e6428<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe6428;
    loop {
        match pc {
        0xe6428 => {
            // block e6428
            #[cfg(feature = "trace")] eprintln!("f_000e6428 pc e6428 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6428 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e6429 mov rbp,rsp
            rbp = rsp;
            // e642c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e642e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e642f mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e6431 mov r14,rdi
            r14 = rdi;
            // e6434 call 0xe6324
            { let r = { let t = m.env.code_va(0xe6324); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e6439 dec ebx
            { let (r, f) = dec::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // e643b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e643d cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // e6440 ja 0xe6449
            if fl.a() { pc = 0xe6449; continue; }
            pc = 0xe6442;
        }
        0xe6442 => {
            // block e6442
            #[cfg(feature = "trace")] eprintln!("f_000e6428 pc e6442 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6442 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e6444 mov eax,DWORD PTR [r14+rax*4+0x60]
            rax = (m.r32(r14.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x60)) as u64) as u32 as u64;
            pc = 0xe6449;
        }
        0xe6449 => {
            // block e6449
            #[cfg(feature = "trace")] eprintln!("f_000e6428 pc e6449 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6449 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e644a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e644c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e644d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe7cde size=38
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e7cde<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe7cde;
    loop {
        match pc {
        0xe7cde => {
            // block e7cde
            #[cfg(feature = "trace")] eprintln!("f_000e7cde pc e7cde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7cde push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e7cdf mov rbp,rsp
            rbp = rsp;
            // e7ce2 cmp edi,0x1ff
            fl = sub::<32>((rdi as u32 as u64), 0x1ffu64).1;
            // e7ce8 ja 0xe7cf9
            if fl.a() { pc = 0xe7cf9; continue; }
            pc = 0xe7cea;
        }
        0xe7cea => {
            // block e7cea
            #[cfg(feature = "trace")] eprintln!("f_000e7cde pc e7cea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7cea mov eax,edi
            rax = ((rdi as u32 as u64)) as u32 as u64;
            // e7cec lea rcx,[rip+0xd8cfd] # 0x1c09f0
            rcx = m.env.data_va(872893);
            // e7cf3 mov rax,QWORD PTR [rcx+rax*8]
            rax = m.r64(rcx.wrapping_add(rax.wrapping_mul(8))) as u64;
            // e7cf7 jmp 0xe7d00
            pc = 0xe7d00; continue;
        }
        0xe7cf9 => {
            // block e7cf9
            #[cfg(feature = "trace")] eprintln!("f_000e7cde pc e7cf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7cf9 lea rax,[rip+0x50fc8] # 0x138cc8
            rax = m.env.data_va(316565);
            pc = 0xe7d00;
        }
        0xe7d00 => {
            // block e7d00
            #[cfg(feature = "trace")] eprintln!("f_000e7cde pc e7d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e7d00 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e7d01 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2964 size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00002964<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2964;
    loop {
        match pc {
        0x2964 => {
            // block 2964
            #[cfg(feature = "trace")] eprintln!("f_00002964 pc 2964 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2964 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2965 mov rbp,rsp
            rbp = rsp;
            // 2968 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 296b mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 296f mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 2971 lea rdx,[rip+0xfc708] # 0xff080
            rdx = m.env.data_va(79949);
            // 2978 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 297b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 297d mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // 2982 mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // 2988 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2989 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x524a6 size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000524a6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x524a6;
    loop {
        match pc {
        0x524a6 => {
            // block 524a6
            #[cfg(feature = "trace")] eprintln!("f_000524a6 pc 524a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 524a6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 524a7 mov rbp,rsp
            rbp = rsp;
            // 524aa sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 524ae lea rdi,[rip+0x175d73] # 0x1c8228
            rdi = m.env.data_va(903669);
            // 524b5 mov QWORD PTR [rbp-0x8],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff8), (rdi) as u64);
            // 524b9 call 0x566f8
            { let r = { let t = m.env.code_va(0x566f8); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 524be lea rdi,[rbp-0x8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fff8);
            // 524c2 call 0x56744
            { let r = { let t = m.env.code_va(0x56744); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 524c7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 524cb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 524cc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5ab58 size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005ab58<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5ab58;
    loop {
        match pc {
        0x5ab58 => {
            // block 5ab58
            #[cfg(feature = "trace")] eprintln!("f_0005ab58 pc 5ab58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab58 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5ab59 mov rbp,rsp
            rbp = rsp;
            // 5ab5c mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 5ab5e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 5ab60 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 5ab62 je 0x5ab69
            if fl.e() { pc = 0x5ab69; continue; }
            pc = 0x5ab64;
        }
        0x5ab64 => {
            // block 5ab64
            #[cfg(feature = "trace")] eprintln!("f_0005ab58 pc 5ab64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab64 movzx edx,cl
            rdx = ((rcx as u8 as u64)) as u32 as u64;
            // 5ab67 inc edx
            { let (r, f) = inc::<32>((rdx as u32 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            pc = 0x5ab69;
        }
        0x5ab69 => {
            // block 5ab69
            #[cfg(feature = "trace")] eprintln!("f_0005ab58 pc 5ab69 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab69 call 0x5a96d
            { let r = { let t = m.env.code_va(0x5a96d); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 5ab6e test ax,0xfff
            fl = logic::<16>((rax as u16 as u64) & 0xfffu64);
            // 5ab72 je 0x5ab7e
            if fl.e() { pc = 0x5ab7e; continue; }
            pc = 0x5ab74;
        }
        0x5ab74 => {
            // block 5ab74
            #[cfg(feature = "trace")] eprintln!("f_0005ab58 pc 5ab74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab74 add eax,0x1000
            { let (r, f) = add::<32>((rax as u32 as u64), 0x1000u64);
              rax = (r) as u32 as u64;
              fl = f; }
            // 5ab79 and eax,0xfffff000
            { let r = (rax as u32 as u64) & 0xfffff000u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x5ab7e;
        }
        0x5ab7e => {
            // block 5ab7e
            #[cfg(feature = "trace")] eprintln!("f_0005ab58 pc 5ab7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5ab7e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5ab7f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14c16 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014c16<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x14c16;
    loop {
        match pc {
        0x14c16 => {
            // block 14c16
            #[cfg(feature = "trace")] eprintln!("f_00014c16 pc 14c16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14c16 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14c17 mov rbp,rsp
            rbp = rsp;
            // 14c1a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14c1b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 14c1c mov rbx,rdi
            rbx = rdi;
            // 14c1f call 0x14a8a
            { let r = { let t = m.env.code_va(0x14a8a); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14c24 mov rdi,rbx
            rdi = rbx;
            // 14c27 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 14c2b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14c2c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14c2d jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2f810 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002f810<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2f810;
    loop {
        match pc {
        0x2f810 => {
            // block 2f810
            #[cfg(feature = "trace")] eprintln!("f_0002f810 pc 2f810 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2f810 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2f811 mov rbp,rsp
            rbp = rsp;
            // 2f814 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2f815 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2f816 mov rbx,rdi
            rbx = rdi;
            // 2f819 lea rax,[rip+0x111e70] # 0x141690
            rax = m.env.data_va(351837);
            // 2f820 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 2f823 lea rdi,[rbx+0x18]
            rdi = rbx.wrapping_add(0x18);
            // 2f827 call 0xb68ee
            { let r = { let t = m.env.code_va(0xb68ee); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2f82c mov rdi,rbx
            rdi = rbx;
            // 2f82f add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 2f833 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2f834 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2f835 jmp 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x68526 size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00068526<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x68526;
    loop {
        match pc {
        0x68526 => {
            // block 68526
            #[cfg(feature = "trace")] eprintln!("f_00068526 pc 68526 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 68526 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 68527 mov rbp,rsp
            rbp = rsp;
            // 6852a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6852b push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6852c mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // 6852e mov rcx,rsi
            rcx = rsi;
            // 68531 mov rbx,rdi
            rbx = rdi;
            // 68534 lea rsi,[rip+0x15fe45] # 0x1c8380
            rsi = m.env.data_va(904013);
            // 6853b mov rdx,rcx
            rdx = rcx;
            // 6853e mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 68540 call 0x697d6
            { let r = { let t = m.env.code_va(0x697d6); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 68545 mov rax,rbx
            rax = rbx;
            // 68548 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6854c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6854d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6854e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe45ce size=42
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e45ce<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe45ce;
    loop {
        match pc {
        0xe45ce => {
            // block e45ce
            #[cfg(feature = "trace")] eprintln!("f_000e45ce pc e45ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e45ce push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e45cf mov rbp,rsp
            rbp = rsp;
            // e45d2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e45d4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e45d5 mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // e45d8 mov rbx,rdi
            rbx = rdi;
            // e45db lea rdi,[rbx+0x34]
            rdi = rbx.wrapping_add(0x34);
            // e45df mov edx,0x1004
            rdx = (0x1004u64) as u32 as u64;
            // e45e4 call 0xec2cc
            { let r = { let t = m.env.stub_va(0xa99); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e45e9 mov rdi,rbx
            rdi = rbx;
            // e45ec mov esi,r14d
            rsi = ((r14 as u32 as u64)) as u32 as u64;
            // e45ef pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e45f0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e45f2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e45f3 jmp 0xe435c
            { let r = { let t = m.env.code_va(0xe435c); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe49c4 size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e49c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe49c4;
    loop {
        match pc {
        0xe49c4 => {
            // block e49c4
            #[cfg(feature = "trace")] eprintln!("f_000e49c4 pc e49c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e49c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e49c5 mov rbp,rsp
            rbp = rsp;
            // e49c8 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e49cc mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e49cf mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e49d2 lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e49d6 mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // e49db mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e49e0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e49e3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e49e6 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e49e9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e49ed pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e49ee ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe6032 size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e6032<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe6032;
    loop {
        match pc {
        0xe6032 => {
            // block e6032
            #[cfg(feature = "trace")] eprintln!("f_000e6032 pc e6032 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6032 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e6033 mov rbp,rsp
            rbp = rsp;
            // e6036 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e6037 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e6038 mov rbx,rdi
            rbx = rdi;
            // e603b mov ecx,DWORD PTR [rbx+0x5c]
            rcx = (m.r32(rbx.wrapping_add(0x5c)) as u64) as u32 as u64;
            // e603e mov r8d,DWORD PTR [rbx+0x58]
            r8 = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // e6042 mov r9d,DWORD PTR [rbx+0x54]
            r9 = (m.r32(rbx.wrapping_add(0x54)) as u64) as u32 as u64;
            // e6046 mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // e6049 mov eax,DWORD PTR [rbx+0x50]
            rax = (m.r32(rbx.wrapping_add(0x50)) as u64) as u32 as u64;
            // e604c mov DWORD PTR [rsp],eax
            m.w32(rsp, ((rax as u32 as u64)) as u32);
            // e604f call 0xe605e
            { let r = { let t = m.env.code_va(0xe605e); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e6054 mov al,BYTE PTR [rbx+0xc]
            rax = (rax & !0xff) | ((m.r8(rbx.wrapping_add(0xc)) as u64) as u8 as u64);
            // e6057 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e605b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e605c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e605d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe63fc size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e63fc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe63fc;
    loop {
        match pc {
        0xe63fc => {
            // block e63fc
            #[cfg(feature = "trace")] eprintln!("f_000e63fc pc e63fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e63fc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e63fd mov rbp,rsp
            rbp = rsp;
            // e6400 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e6401 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e6402 mov rbx,rdi
            rbx = rdi;
            // e6405 call 0xe6324
            { let r = { let t = m.env.code_va(0xe6324); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e640a test BYTE PTR [rbx+0x38],0x1
            fl = logic::<8>(m.r8(rbx.wrapping_add(0x38)) as u64 & 0x1u64);
            // e640e jne 0xe6419
            if fl.ne() { pc = 0xe6419; continue; }
            pc = 0xe6410;
        }
        0xe6410 => {
            // block e6410
            #[cfg(feature = "trace")] eprintln!("f_000e63fc pc e6410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6410 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // e6414 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // e6417 jmp 0xe641d
            pc = 0xe641d; continue;
        }
        0xe6419 => {
            // block e6419
            #[cfg(feature = "trace")] eprintln!("f_000e63fc pc e6419 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e6419 mov rbx,QWORD PTR [rbx+0x48]
            rbx = m.r64(rbx.wrapping_add(0x48)) as u64;
            pc = 0xe641d;
        }
        0xe641d => {
            // block e641d
            #[cfg(feature = "trace")] eprintln!("f_000e63fc pc e641d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e641d mov rax,rbx
            rax = rbx;
            // e6420 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e6424 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e6425 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e6426 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe65ca size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e65ca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe65ca;
    loop {
        match pc {
        0xe65ca => {
            // block e65ca
            #[cfg(feature = "trace")] eprintln!("f_000e65ca pc e65ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e65ca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e65cb mov rbp,rsp
            rbp = rsp;
            // e65ce push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e65d0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e65d1 mov r14,rdi
            r14 = rdi;
            // e65d4 call 0xe6324
            { let r = { let t = m.env.code_va(0xe6324); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e65d9 mov eax,DWORD PTR [r14+0x6c]
            rax = (m.r32(r14.wrapping_add(0x6c)) as u64) as u32 as u64;
            // e65dd lea ebx,[rax+rax*4]
            rbx = (rax.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e65e0 add ebx,ebx
            { let (r, f) = add::<32>((rbx as u32 as u64), (rbx as u32 as u64));
              rbx = (r) as u32 as u64;
              fl = f; }
            // e65e2 mov rdi,r14
            rdi = r14;
            // e65e5 call 0xe6324
            { let r = { let t = m.env.code_va(0xe6324); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e65ea add ebx,DWORD PTR [r14+0x68]
            { let (r, f) = add::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x68)) as u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e65ee mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // e65f0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e65f1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e65f3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e65f4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28f1c size=46
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028f1c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28f1c;
    loop {
        match pc {
        0x28f1c => {
            // block 28f1c
            #[cfg(feature = "trace")] eprintln!("f_00028f1c pc 28f1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28f1c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28f1d mov rbp,rsp
            rbp = rsp;
            // 28f20 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28f22 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28f23 mov r14d,ecx
            r14 = ((rcx as u32 as u64)) as u32 as u64;
            // 28f26 mov rbx,rdi
            rbx = rdi;
            // 28f29 call 0x28ea8
            { let r = { let t = m.env.code_va(0x28ea8); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28f2e mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 28f31 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 28f34 je 0x28f42
            if fl.e() { pc = 0x28f42; continue; }
            pc = 0x28f36;
        }
        0x28f36 => {
            // block 28f36
            #[cfg(feature = "trace")] eprintln!("f_00028f1c pc 28f36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28f36 mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 28f39 movzx esi,r14b
            rsi = ((r14 as u8 as u64)) as u32 as u64;
            // 28f3d call 0xec22a
            { let r = { let t = m.env.stub_va(0x9f7); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x28f42;
        }
        0x28f42 => {
            // block 28f42
            #[cfg(feature = "trace")] eprintln!("f_00028f1c pc 28f42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28f42 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 28f44 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28f45 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28f47 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28f48 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb796e size=46
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b796e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb796e;
    loop {
        match pc {
        0xb796e => {
            // block b796e
            #[cfg(feature = "trace")] eprintln!("f_000b796e pc b796e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b796e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b796f mov rbp,rsp
            rbp = rsp;
            // b7972 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7974 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7975 mov rbx,rdi
            rbx = rdi;
            // b7978 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b797b call QWORD PTR [rax+0x1d18]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d18)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7981 mov r14d,eax
            r14 = ((rax as u32 as u64)) as u32 as u64;
            // b7984 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7987 mov rdi,rbx
            rdi = rbx;
            // b798a call QWORD PTR [rax+0x1d20]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d20)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7990 shl rax,0x20
            { let (r, f) = shl::<64>(rax, 0x20u64, fl); rax = r; fl = f; }
            // b7994 or rax,r14
            { let r = rax | r14; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // b7997 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7998 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b799a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b799b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe52f6 size=46
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e52f6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe52f6;
    loop {
        match pc {
        0xe52f6 => {
            // block e52f6
            #[cfg(feature = "trace")] eprintln!("f_000e52f6 pc e52f6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e52f6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e52f7 mov rbp,rsp
            rbp = rsp;
            // e52fa sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e52fe mov eax,DWORD PTR [rsi]
            rax = (m.r32(rsi) as u64) as u32 as u64;
            // e5300 mov ecx,DWORD PTR [rsi+0x4]
            rcx = (m.r32(rsi.wrapping_add(0x4)) as u64) as u32 as u64;
            // e5303 shl rcx,0x20
            { let (r, f) = shl::<64>(rcx, 0x20u64, fl); rcx = r; fl = f; }
            // e5307 or rcx,rax
            { let r = rcx | rax; let f = logic::<64>(r);
              rcx = r;
              fl = f; }
            // e530a mov eax,DWORD PTR [rsi+0x8]
            rax = (m.r32(rsi.wrapping_add(0x8)) as u64) as u32 as u64;
            // e530d mov QWORD PTR [rbp-0x10],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (rcx) as u64);
            // e5311 mov DWORD PTR [rbp-0x8],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff8), ((rax as u32 as u64)) as u32);
            // e5314 lea rsi,[rbp-0x10]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // e5318 call 0xe4fc6
            { let r = { let t = m.env.code_va(0xe4fc6); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e531d add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e5321 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e5322 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x11252 size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00011252<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x11252;
    loop {
        match pc {
        0x11252 => {
            // block 11252
            #[cfg(feature = "trace")] eprintln!("f_00011252 pc 11252 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 11252 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 11253 mov rbp,rsp
            rbp = rsp;
            // 11256 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 11257 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 11258 mov rbx,rdi
            rbx = rdi;
            // 1125b call 0xe0758
            { let r = { let t = m.env.code_va(0xe0758); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 11260 lea rax,[rip+0x130039] # 0x1412a0
            rax = m.env.data_va(350829);
            // 11267 mov QWORD PTR [rbx],rax
            m.w64(rbx, (rax) as u64);
            // 1126a mov QWORD PTR [rbx+0x40],0x0
            m.w64(rbx.wrapping_add(0x40), (0x0u64) as u64);
            // 11272 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // 1127a add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 1127e pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1127f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 11280 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe478a size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e478a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe478a;
    loop {
        match pc {
        0xe478a => {
            // block e478a
            #[cfg(feature = "trace")] eprintln!("f_000e478a pc e478a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e478a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e478b mov rbp,rsp
            rbp = rsp;
            // e478e sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e4792 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e4795 mov r9,QWORD PTR [rax]
            r9 = m.r64(rax) as u64;
            // e4798 mov esi,DWORD PTR [rdi+0x30]
            rsi = (m.r32(rdi.wrapping_add(0x30)) as u64) as u32 as u64;
            // e479b add esi,0x4
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x4u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e479e lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e47a2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e47a7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e47aa mov rdi,rax
            rdi = rax;
            // e47ad call QWORD PTR [r9+0x38]
            { let r = { let t = m.r64(r9.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e47b1 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e47b4 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e47b8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e47b9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44d68 size=50
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044d68<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44d68;
    loop {
        match pc {
        0x44d68 => {
            // block 44d68
            #[cfg(feature = "trace")] eprintln!("f_00044d68 pc 44d68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44d68 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44d69 mov rbp,rsp
            rbp = rsp;
            // 44d6c sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44d70 mov r8,rdx
            r8 = rdx;
            // 44d73 mov r10d,esi
            r10 = ((rsi as u32 as u64)) as u32 as u64;
            // 44d76 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 44d79 mov DWORD PTR [rsp],ecx
            m.w32(rsp, ((rcx as u32 as u64)) as u32);
            // 44d7c mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 44d84 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 44d89 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 44d8b xor r9d,r9d
            { let r = (r9 as u32 as u64) ^ (r9 as u32 as u64); let f = logic::<32>(r);
              r9 = (r) as u32 as u64;
              fl = f; }
            // 44d8e mov ecx,r10d
            rcx = ((r10 as u32 as u64)) as u32 as u64;
            // 44d91 call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44d94 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44d98 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44d99 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
