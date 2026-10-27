//! Mechanically lifted functions (scripts/lift_run.py), module h008. Each one is checked against the original
//! in the world harness (tests/world_h008.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0xb818a size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b818a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb818a;
    loop {
        match pc {
        0xb818a => {
            // block b818a
            #[cfg(feature = "trace")] eprintln!("f_000b818a pc b818a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b818a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b818b mov rbp,rsp
            rbp = rsp;
            // b818e sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b8192 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b8199 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b819c lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b81a0 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // b81a5 mov ecx,0x80000
            rcx = (0x80000u64) as u32 as u64;
            // b81aa mov r8d,0x13
            r8 = (0x13u64) as u32 as u64;
            // b81b0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b81b3 cmp DWORD PTR [rbp-0x4],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64, 0x1u64).1;
            // b81b7 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // b81ba add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b81be pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b81bf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe4754 size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e4754<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe4754;
    loop {
        match pc {
        0xe4754 => {
            // block e4754
            #[cfg(feature = "trace")] eprintln!("f_000e4754 pc e4754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e4754 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e4755 mov rbp,rsp
            rbp = rsp;
            // e4758 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e475c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e475f mov r9,QWORD PTR [rax]
            r9 = m.r64(rax) as u64;
            // e4762 mov esi,DWORD PTR [rdi+0x30]
            rsi = (m.r32(rdi.wrapping_add(0x30)) as u64) as u32 as u64;
            // e4765 add esi,0x4
            { let (r, f) = add::<32>((rsi as u32 as u64), 0x4u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e4768 lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e476c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e4771 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e4774 mov rdi,rax
            rdi = rax;
            // e4777 call QWORD PTR [r9+0x38]
            { let r = { let t = m.r64(r9.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e477b mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e477e test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // e4780 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // e4783 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e4787 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e4788 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7e22 size=60
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7e22<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7e22;
    loop {
        match pc {
        0xb7e22 => {
            // block b7e22
            #[cfg(feature = "trace")] eprintln!("f_000b7e22 pc b7e22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e22 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7e23 mov rbp,rsp
            rbp = rsp;
            // b7e26 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7e27 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // b7e28 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // b7e2a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b7e2d lea rsi,[rbp-0xc]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // b7e31 call QWORD PTR [rax+0xa40]
            { let r = { let t = m.r64(rax.wrapping_add(0xa40)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7e37 cmp ebx,0x1
            fl = sub::<32>((rbx as u32 as u64), 0x1u64).1;
            // b7e3a jne 0xb7e44
            if fl.ne() { pc = 0xb7e44; continue; }
            pc = 0xb7e3c;
        }
        0xb7e3c => {
            // block b7e3c
            #[cfg(feature = "trace")] eprintln!("f_000b7e22 pc b7e3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e3c mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // b7e3f shr eax,0xb
            { let (r, f) = shr::<32>((rax as u32 as u64), 0xbu64, fl); rax = (r) as u32 as u64; fl = f; }
            // b7e42 jmp 0xb7e4e
            pc = 0xb7e4e; continue;
        }
        0xb7e44 => {
            // block b7e44
            #[cfg(feature = "trace")] eprintln!("f_000b7e22 pc b7e44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e44 test ebx,ebx
            fl = logic::<32>((rbx as u32 as u64) & (rbx as u32 as u64));
            // b7e46 jne 0xb7e55
            if fl.ne() { pc = 0xb7e55; continue; }
            pc = 0xb7e48;
        }
        0xb7e48 => {
            // block b7e48
            #[cfg(feature = "trace")] eprintln!("f_000b7e22 pc b7e48 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e48 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // b7e4b shr eax,0x3
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x3u64, fl); rax = (r) as u32 as u64; fl = f; }
            pc = 0xb7e4e;
        }
        0xb7e4e => {
            // block b7e4e
            #[cfg(feature = "trace")] eprintln!("f_000b7e22 pc b7e4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e4e not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // b7e50 and eax,0x1
            { let r = (rax as u32 as u64) & 0x1u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // b7e53 jmp 0xb7e57
            pc = 0xb7e57; continue;
        }
        0xb7e55 => {
            // block b7e55
            #[cfg(feature = "trace")] eprintln!("f_000b7e22 pc b7e55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e55 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb7e57;
        }
        0xb7e57 => {
            // block b7e57
            #[cfg(feature = "trace")] eprintln!("f_000b7e22 pc b7e57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e57 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // b7e5b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7e5c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7e5d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb730a size=62
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b730a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb730a;
    loop {
        match pc {
        0xb730a => {
            // block b730a
            #[cfg(feature = "trace")] eprintln!("f_000b730a pc b730a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b730a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b730b mov rbp,rsp
            rbp = rsp;
            // b730e sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7312 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // b7319 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b731c lea rdx,[rbp-0x4]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // b7320 mov esi,0x30
            rsi = (0x30u64) as u32 as u64;
            // b7325 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b732a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b732d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7330 mov cx,0xffff
            rcx = (rcx & !0xffff) | ((0xffffu64) as u16 as u64);
            // b7334 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7336 je 0xb733e
            if fl.e() { pc = 0xb733e; continue; }
            pc = 0xb7338;
        }
        0xb7338 => {
            // block b7338
            #[cfg(feature = "trace")] eprintln!("f_000b730a pc b7338 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7338 mov eax,DWORD PTR [rbp-0x4]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // b733b movzx ecx,ah
            rcx = (((rax >> 8) as u8 as u64)) as u32 as u64;
            pc = 0xb733e;
        }
        0xb733e => {
            // block b733e
            #[cfg(feature = "trace")] eprintln!("f_000b730a pc b733e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b733e movsx eax,cx
            rax = (sx::<16>((rcx as u16 as u64)) as u64) as u32 as u64;
            // b7341 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7345 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7346 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0b6a size=64
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0b6a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0b6a;
    loop {
        match pc {
        0xe0b6a => {
            // block e0b6a
            #[cfg(feature = "trace")] eprintln!("f_000e0b6a pc e0b6a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0b6a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0b6b mov rbp,rsp
            rbp = rsp;
            // e0b6e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0b70 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0b71 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0b75 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e0b78 mov rbx,rdi
            rbx = rdi;
            // e0b7b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b7e lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0b82 call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0b88 mov esi,DWORD PTR [rbp-0x14]
            rsi = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // e0b8b and esi,0xfffffff0
            { let r = (rsi as u32 as u64) & 0xfffffff0u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0b8e or esi,r14d
            { let r = (rsi as u32 as u64) | (r14 as u32 as u64); let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0b91 mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            // e0b94 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0b97 mov rdi,rbx
            rdi = rbx;
            // e0b9a call QWORD PTR [rax+0x1ac0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0ba0 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0ba4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0ba5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0ba7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0ba8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5c4c size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5c4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd5c4c;
    loop {
        match pc {
        0xd5c4c => {
            // block d5c4c
            #[cfg(feature = "trace")] eprintln!("f_000d5c4c pc d5c4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5c4d mov rbp,rsp
            rbp = rsp;
            // d5c50 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5c51 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d5c52 mov rbx,rdi
            rbx = rdi;
            // d5c55 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5c58 call QWORD PTR [rax+0x100]
            { let r = { let t = m.r64(rax.wrapping_add(0x100)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5c5e test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d5c60 je 0xd5c85
            if fl.e() { pc = 0xd5c85; continue; }
            pc = 0xd5c62;
        }
        0xd5c62 => {
            // block d5c62
            #[cfg(feature = "trace")] eprintln!("f_000d5c4c pc d5c62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c62 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5c65 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d5c69 mov esi,0x40005
            rsi = (0x40005u64) as u32 as u64;
            // d5c6e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5c73 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5c76 mov rdi,rbx
            rdi = rbx;
            // d5c79 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5c7c cmp DWORD PTR [rbp-0xc],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x1u64).1;
            // d5c80 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // d5c83 jmp 0xd5c87
            pc = 0xd5c87; continue;
        }
        0xd5c85 => {
            // block d5c85
            #[cfg(feature = "trace")] eprintln!("f_000d5c4c pc d5c85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c85 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd5c87;
        }
        0xd5c87 => {
            // block d5c87
            #[cfg(feature = "trace")] eprintln!("f_000d5c4c pc d5c87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c87 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d5c8b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5c8c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5c8d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0bee size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0bee<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0bee;
    loop {
        match pc {
        0xe0bee => {
            // block e0bee
            #[cfg(feature = "trace")] eprintln!("f_000e0bee pc e0bee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0bee push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0bef mov rbp,rsp
            rbp = rsp;
            // e0bf2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0bf4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0bf5 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0bf9 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e0bfb mov r14,rdi
            r14 = rdi;
            // e0bfe mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0c01 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0c05 call QWORD PTR [rax+0x1ac8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0c0b mov eax,0xffffff0f
            rax = (0xffffff0fu64) as u32 as u64;
            // e0c10 and eax,DWORD PTR [rbp-0x14]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0c13 shl ebx,0x4
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x4u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0c16 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0c18 mov DWORD PTR [rbp-0x14],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rbx as u32 as u64)) as u32);
            // e0c1b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0c1e mov rdi,r14
            rdi = r14;
            // e0c21 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e0c23 call QWORD PTR [rax+0x1ac0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ac0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0c29 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0c2d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0c2e pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0c30 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0c31 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0e4a size=68
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0e4a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0e4a;
    loop {
        match pc {
        0xe0e4a => {
            // block e0e4a
            #[cfg(feature = "trace")] eprintln!("f_000e0e4a pc e0e4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e4a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0e4b mov rbp,rsp
            rbp = rsp;
            // e0e4e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0e50 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0e51 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e55 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e0e57 mov r14,rdi
            r14 = rdi;
            // e0e5a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0e5d lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0e61 call QWORD PTR [rax+0x1ad8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0e67 mov eax,0x3fffffff
            rax = (0x3fffffffu64) as u32 as u64;
            // e0e6c and eax,DWORD PTR [rbp-0x14]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0e6f shl ebx,0x1e
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x1eu64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e0e72 or ebx,eax
            { let r = (rbx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e0e74 mov DWORD PTR [rbp-0x14],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rbx as u32 as u64)) as u32);
            // e0e77 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // e0e7a mov rdi,r14
            rdi = r14;
            // e0e7d mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e0e7f call QWORD PTR [rax+0x1ad0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ad0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0e85 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e89 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0e8a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0e8c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0e8d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0d78 size=76
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0d78<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0d78;
    loop {
        match pc {
        0xe0d78 => {
            // block e0d78
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0d78 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0d79 mov rbp,rsp
            rbp = rsp;
            // e0d7c sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0d80 mov DWORD PTR [rbp-0x4],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), (0x0u64) as u32);
            // e0d87 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e0d8a lea rsi,[rbp-0x4]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fffc);
            // e0d8e call QWORD PTR [rax+0x1ae8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0d94 mov ecx,DWORD PTR [rbp-0x4]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fffc)) as u64) as u32 as u64;
            // e0d97 shr ecx,0x2
            { let (r, f) = shr::<32>((rcx as u32 as u64), 0x2u64, fl); rcx = (r) as u32 as u64; fl = f; }
            // e0d9a and ecx,0x3
            { let r = (rcx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rcx = (r) as u32 as u64;
              fl = f; }
            // e0d9d mov DWORD PTR [rbp-0x4],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fffc), ((rcx as u32 as u64)) as u32);
            // e0da0 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // e0da5 je 0xe0dbd
            if fl.e() { pc = 0xe0dbd; continue; }
            pc = 0xe0da7;
        }
        0xe0da7 => {
            // block e0da7
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0da7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0da7 cmp ecx,0x1
            fl = sub::<32>((rcx as u32 as u64), 0x1u64).1;
            // e0daa jne 0xe0db0
            if fl.ne() { pc = 0xe0db0; continue; }
            pc = 0xe0dac;
        }
        0xe0dac => {
            // block e0dac
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0dac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0dac xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e0dae jmp 0xe0dbd
            pc = 0xe0dbd; continue;
        }
        0xe0db0 => {
            // block e0db0
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0db0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0db0 cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // e0db3 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // e0db6 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // e0db9 lea eax,[rax+rax*1+0x1]
            rax = (rax.wrapping_add(rax).wrapping_add(0x1)) as u32 as u64;
            pc = 0xe0dbd;
        }
        0xe0dbd => {
            // block e0dbd
            #[cfg(feature = "trace")] eprintln!("f_000e0d78 pc e0dbd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0dbd add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0dc1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0dc2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2f90 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2f90<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2f90;
    loop {
        match pc {
        0xe2f90 => {
            // block e2f90
            #[cfg(feature = "trace")] eprintln!("f_000e2f90 pc e2f90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2f90 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2f91 mov rbp,rsp
            rbp = rsp;
            // e2f94 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2f96 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2f97 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2f9b mov r14,rcx
            r14 = rcx;
            // e2f9e mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e2fa0 mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2fa3 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2fa6 mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // e2faa test edx,edx
            fl = logic::<32>((rdx as u32 as u64) & (rdx as u32 as u64));
            // e2fac je 0xe2fb9
            if fl.e() { pc = 0xe2fb9; continue; }
            pc = 0xe2fae;
        }
        0xe2fae => {
            // block e2fae
            #[cfg(feature = "trace")] eprintln!("f_000e2f90 pc e2fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2fae lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e2fb2 mov esi,0x4001c
            rsi = (0x4001cu64) as u32 as u64;
            // e2fb7 jmp 0xe2fc2
            pc = 0xe2fc2; continue;
        }
        0xe2fb9 => {
            // block e2fb9
            #[cfg(feature = "trace")] eprintln!("f_000e2f90 pc e2fb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2fb9 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e2fbd mov esi,0x4001b
            rsi = (0x4001bu64) as u32 as u64;
            pc = 0xe2fc2;
        }
        0xe2fc2 => {
            // block e2fc2
            #[cfg(feature = "trace")] eprintln!("f_000e2f90 pc e2fc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2fc2 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2fc7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2fca call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2fcc shl ebx,0x3
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x3u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e2fcf mov eax,DWORD PTR [rbp-0x14]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64) as u32 as u64;
            // e2fd2 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2fd4 shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e2fd6 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e2fd9 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e2fdb add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2fdf pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2fe0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2fe2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2fe3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd8328 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d8328<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd8328;
    loop {
        match pc {
        0xd8328 => {
            // block d8328
            #[cfg(feature = "trace")] eprintln!("f_000d8328 pc d8328 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8328 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d8329 mov rbp,rsp
            rbp = rsp;
            // d832c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // d832e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d832f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d8333 mov rbx,rdi
            rbx = rdi;
            // d8336 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d8339 lea r14,[rbp-0x14]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // d833d mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // d8342 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8347 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d834a mov rdx,r14
            rdx = r14;
            // d834d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8350 mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            pc = 0xd8357;
        }
        0xd8357 => {
            // block d8357
            #[cfg(feature = "trace")] eprintln!("f_000d8328 pc d8357 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8357 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d835a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // d835f mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d8364 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d8367 mov rdi,rbx
            rdi = rbx;
            // d836a mov rdx,r14
            rdx = r14;
            // d836d call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d8370 test BYTE PTR [rbp-0x13],0x1
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffed)) as u64 & 0x1u64);
            // d8374 jne 0xd8357
            if fl.ne() { pc = 0xd8357; continue; }
            pc = 0xd8376;
        }
        0xd8376 => {
            // block d8376
            #[cfg(feature = "trace")] eprintln!("f_000d8328 pc d8376 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d8376 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d8378 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // d837c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d837d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // d837f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d8380 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe0dc4 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e0dc4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe0dc4;
    loop {
        match pc {
        0xe0dc4 => {
            // block e0dc4
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0dc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0dc4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e0dc5 mov rbp,rsp
            rbp = rsp;
            // e0dc8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e0dca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e0dcb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0dcf mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e0dd2 mov rbx,rdi
            rbx = rdi;
            // e0dd5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0dd8 lea rsi,[rbp-0x14]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e0ddc call QWORD PTR [rax+0x1ae8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e0de2 mov esi,0xfffffcff
            rsi = (0xfffffcffu64) as u32 as u64;
            // e0de7 and esi,DWORD PTR [rbp-0x14]
            { let r = (rsi as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0dea mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            // e0ded cmp r14d,0x3
            fl = sub::<32>((r14 as u32 as u64), 0x3u64).1;
            // e0df1 je 0xe0e1c
            if fl.e() { pc = 0xe0e1c; continue; }
            pc = 0xe0df3;
        }
        0xe0df3 => {
            // block e0df3
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0df3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0df3 cmp r14d,0x2
            fl = sub::<32>((r14 as u32 as u64), 0x2u64).1;
            // e0df7 jne 0xe0e01
            if fl.ne() { pc = 0xe0e01; continue; }
            pc = 0xe0df9;
        }
        0xe0df9 => {
            // block e0df9
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0df9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0df9 or esi,0x200
            { let r = (rsi as u32 as u64) | 0x200u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            // e0dff jmp 0xe0e0d
            pc = 0xe0e0d; continue;
        }
        0xe0e01 => {
            // block e0e01
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e01 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e01 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // e0e05 jne 0xe0e10
            if fl.ne() { pc = 0xe0e10; continue; }
            pc = 0xe0e07;
        }
        0xe0e07 => {
            // block e0e07
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e07 or esi,0x100
            { let r = (rsi as u32 as u64) | 0x100u64; let f = logic::<32>(r);
              rsi = (r) as u32 as u64;
              fl = f; }
            pc = 0xe0e0d;
        }
        0xe0e0d => {
            // block e0e0d
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e0d mov DWORD PTR [rbp-0x14],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rsi as u32 as u64)) as u32);
            pc = 0xe0e10;
        }
        0xe0e10 => {
            // block e0e10
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e10 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // e0e13 mov rdi,rbx
            rdi = rbx;
            // e0e16 call QWORD PTR [rax+0x1ae0]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ae0)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe0e1c;
        }
        0xe0e1c => {
            // block e0e1c
            #[cfg(feature = "trace")] eprintln!("f_000e0dc4 pc e0e1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e0e1c add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e0e20 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e0e21 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e0e23 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e0e24 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe1dca size=100
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e1dca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe1dca;
    loop {
        match pc {
        0xe1dca => {
            // block e1dca
            #[cfg(feature = "trace")] eprintln!("f_000e1dca pc e1dca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e1dca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e1dcb mov rbp,rsp
            rbp = rsp;
            // e1dce push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e1dd0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e1dd1 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e1dd5 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // e1dd8 mov rbx,rdi
            rbx = rdi;
            // e1ddb mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e1dde mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1de1 lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // e1de5 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e1dea mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e1def xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1df2 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1df5 cmp r14d,0x1
            fl = sub::<32>((r14 as u32 as u64), 0x1u64).1;
            // e1df9 mov eax,0xbfffffff
            rax = (0xbfffffffu64) as u32 as u64;
            // e1dfe mov edx,0x7fffffff
            rdx = (0x7fffffffu64) as u32 as u64;
            // e1e03 cmove edx,eax
            if fl.e() { rdx = ((rax as u32 as u64)) as u32 as u64; } else { rdx = rdx as u32 as u64; }
            // e1e06 and edx,DWORD PTR [rbp-0x14]
            { let r = (rdx as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffec)) as u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e1e09 mov DWORD PTR [rbp-0x14],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), ((rdx as u32 as u64)) as u32);
            // e1e0c mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // e1e0f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e1e12 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e1e17 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e1e1c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e1e1f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e1e22 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e1e24 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e1e28 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e1e29 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e1e2b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e1e2c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xba672 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000ba672<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xba672;
    loop {
        match pc {
        0xba672 => {
            // block ba672
            #[cfg(feature = "trace")] eprintln!("f_000ba672 pc ba672 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba672 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // ba673 mov rbp,rsp
            rbp = rsp;
            // ba676 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // ba678 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // ba67a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // ba67c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // ba67d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ba681 mov r15,rsi
            r15 = rsi;
            // ba684 mov r14,rdi
            r14 = rdi;
            // ba687 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // ba68b mov rdx,r15
            rdx = r15;
            // ba68e call 0xebcde
            { let r = { let t = m.env.stub_va(0x4ab); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba693 mov r12,rax
            r12 = rax;
            // ba696 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // ba69a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // ba69d je 0xba6a3
            if fl.e() { pc = 0xba6a3; continue; }
            pc = 0xba69f;
        }
        0xba69f => {
            // block ba69f
            #[cfg(feature = "trace")] eprintln!("f_000ba672 pc ba69f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba69f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ba6a1 jmp 0xba6ca
            pc = 0xba6ca; continue;
        }
        0xba6a3 => {
            // block ba6a3
            #[cfg(feature = "trace")] eprintln!("f_000ba672 pc ba6a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba6a3 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // ba6a8 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba6ad mov rbx,rax
            rbx = rax;
            // ba6b0 mov al,BYTE PTR [r15]
            rax = (rax & !0xff) | ((m.r8(r15) as u64) as u8 as u64);
            // ba6b3 mov BYTE PTR [rbx+0x19],al
            m.w8(rbx.wrapping_add(0x19), ((rax as u8 as u64)) as u8);
            // ba6b6 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // ba6ba mov rdi,r14
            rdi = r14;
            // ba6bd mov rdx,r12
            rdx = r12;
            // ba6c0 mov rcx,rbx
            rcx = rbx;
            // ba6c3 call 0xebcea
            { let r = { let t = m.env.stub_va(0x4b7); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba6c8 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xba6ca;
        }
        0xba6ca => {
            // block ba6ca
            #[cfg(feature = "trace")] eprintln!("f_000ba672 pc ba6ca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba6ca mov rax,rbx
            rax = rbx;
            // ba6cd add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // ba6d1 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // ba6d2 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // ba6d4 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // ba6d6 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // ba6d8 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba6d9 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb6182 size=106
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b6182<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb6182;
    loop {
        match pc {
        0xb6182 => {
            // block b6182
            #[cfg(feature = "trace")] eprintln!("f_000b6182 pc b6182 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b6182 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b6183 mov rbp,rsp
            rbp = rsp;
            // b6186 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b6188 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b618a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b618c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b618d sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // b6191 mov r14,rdx
            r14 = rdx;
            // b6194 mov r12,rdi
            r12 = rdi;
            // b6197 lea rdx,[rbp-0x28]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // b619b mov rcx,r14
            rcx = r14;
            // b619e call 0xebc90
            { let r = { let t = m.env.stub_va(0x45d); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b61a3 mov r15,rax
            r15 = rax;
            // b61a6 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // b61a9 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b61ac jne 0xb61db
            if fl.ne() { pc = 0xb61db; continue; }
            pc = 0xb61ae;
        }
        0xb61ae => {
            // block b61ae
            #[cfg(feature = "trace")] eprintln!("f_000b6182 pc b61ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b61ae lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b61b2 mov rsi,r12
            rsi = r12;
            // b61b5 mov rdx,r14
            rdx = r14;
            // b61b8 call 0xebcae
            { let r = { let t = m.env.stub_va(0x47b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b61bd mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b61c1 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b61c5 mov rdi,r12
            rdi = r12;
            // b61c8 mov rdx,r15
            rdx = r15;
            // b61cb mov rcx,rbx
            rcx = rbx;
            // b61ce call 0xebcb4
            { let r = { let t = m.env.stub_va(0x481); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b61d3 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            pc = 0xb61db;
        }
        0xb61db => {
            // block b61db
            #[cfg(feature = "trace")] eprintln!("f_000b6182 pc b61db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b61db mov rax,rbx
            rax = rbx;
            // b61de add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // b61e2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b61e3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b61e5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b61e7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b61e9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b61ea ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3ac8 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3ac8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3ac8;
    loop {
        match pc {
        0xc3ac8 => {
            // block c3ac8
            #[cfg(feature = "trace")] eprintln!("f_000c3ac8 pc c3ac8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3ac8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3ac9 mov rbp,rsp
            rbp = rsp;
            // c3acc push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3ace push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3ad0 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3ad2 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3ad3 sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // c3ad7 mov r14,rsi
            r14 = rsi;
            // c3ada mov r12,rdi
            r12 = rdi;
            // c3add lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3ae1 mov rdx,r14
            rdx = r14;
            // c3ae4 call 0xebd3e
            { let r = { let t = m.env.stub_va(0x50b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3ae9 mov r15,rax
            r15 = rax;
            // c3aec mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3aef test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3af2 jne 0xc3b21
            if fl.ne() { pc = 0xc3b21; continue; }
            pc = 0xc3af4;
        }
        0xc3af4 => {
            // block c3af4
            #[cfg(feature = "trace")] eprintln!("f_000c3ac8 pc c3af4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3af4 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // c3af8 mov rsi,r12
            rsi = r12;
            // c3afb mov rdx,r14
            rdx = r14;
            // c3afe call 0xebd44
            { let r = { let t = m.env.stub_va(0x511); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b03 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3b07 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // c3b0b mov rdi,r12
            rdi = r12;
            // c3b0e mov rdx,r15
            rdx = r15;
            // c3b11 mov rcx,rbx
            rcx = rbx;
            // c3b14 call 0xebe64
            { let r = { let t = m.env.stub_va(0x631); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b19 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            pc = 0xc3b21;
        }
        0xc3b21 => {
            // block c3b21
            #[cfg(feature = "trace")] eprintln!("f_000c3ac8 pc c3b21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3b21 add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // c3b25 mov rax,rbx
            rax = rbx;
            // c3b28 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // c3b2c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3b2d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3b2f pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3b31 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3b33 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3b34 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3d56 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3d56<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3d56;
    loop {
        match pc {
        0xc3d56 => {
            // block c3d56
            #[cfg(feature = "trace")] eprintln!("f_000c3d56 pc c3d56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3d56 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3d57 mov rbp,rsp
            rbp = rsp;
            // c3d5a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3d5c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3d5e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3d60 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3d61 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3d65 mov r15,rsi
            r15 = rsi;
            // c3d68 mov r14,rdi
            r14 = rdi;
            // c3d6b lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3d6f mov rdx,r15
            rdx = r15;
            // c3d72 call 0xebd0e
            { let r = { let t = m.env.stub_va(0x4db); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3d77 mov r12,rax
            r12 = rax;
            // c3d7a mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // c3d7e test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3d81 jne 0xc3daf
            if fl.ne() { pc = 0xc3daf; continue; }
            pc = 0xc3d83;
        }
        0xc3d83 => {
            // block c3d83
            #[cfg(feature = "trace")] eprintln!("f_000c3d56 pc c3d83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3d83 mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // c3d88 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3d8d mov rbx,rax
            rbx = rax;
            // c3d90 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // c3d93 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // c3d96 mov DWORD PTR [rbx+0x20],0x0
            m.w32(rbx.wrapping_add(0x20), (0x0u64) as u32);
            // c3d9d mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3da1 mov rdi,r14
            rdi = r14;
            // c3da4 mov rdx,r12
            rdx = r12;
            // c3da7 mov rcx,rbx
            rcx = rbx;
            // c3daa call 0xebdec
            { let r = { let t = m.env.stub_va(0x5b9); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3daf;
        }
        0xc3daf => {
            // block c3daf
            #[cfg(feature = "trace")] eprintln!("f_000c3d56 pc c3daf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3daf add rbx,0x20
            { let (r, f) = add::<64>(rbx, 0x20u64);
              rbx = r;
              fl = f; }
            // c3db3 mov rax,rbx
            rax = rbx;
            // c3db6 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3dba pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3dbb pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3dbd pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3dbf pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3dc1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3dc2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe30c4 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e30c4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe30c4;
    loop {
        match pc {
        0xe30c4 => {
            // block e30c4
            #[cfg(feature = "trace")] eprintln!("f_000e30c4 pc e30c4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e30c4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e30c5 mov rbp,rsp
            rbp = rsp;
            // e30c8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e30ca push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e30cc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e30cd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e30ce mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // e30d1 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // e30d3 mov r15,rdi
            r15 = rdi;
            // e30d6 shl ebx,0x3
            { let (r, f) = shl::<32>((rbx as u32 as u64), 0x3u64, fl); rbx = (r) as u32 as u64; fl = f; }
            // e30d9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e30dc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e30df lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e30e3 mov esi,0x4001d
            rsi = (0x4001du64) as u32 as u64;
            // e30e8 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e30ed xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e30f0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e30f3 mov eax,0xff
            rax = (0xffu64) as u32 as u64;
            // e30f8 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e30fa shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e30fc not eax
            rax = (!(rax as u32 as u64)) as u32 as u64;
            // e30fe and eax,DWORD PTR [rbp-0x1c]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e3101 movzx edx,r14b
            rdx = ((r14 as u8 as u64)) as u32 as u64;
            // e3105 mov cl,bl
            rcx = (rcx & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e3107 shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e3109 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e310b mov DWORD PTR [rbp-0x1c],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rdx as u32 as u64)) as u32);
            // e310e mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e3111 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e3114 mov esi,0x4001d
            rsi = (0x4001du64) as u32 as u64;
            // e3119 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e311e xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e3121 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e3124 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // e3126 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e312a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e312b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e312d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e312f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e3130 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5ee2 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5ee2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5ee2;
    loop {
        match pc {
        0xb5ee2 => {
            // block b5ee2
            #[cfg(feature = "trace")] eprintln!("f_000b5ee2 pc b5ee2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5ee2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5ee3 mov rbp,rsp
            rbp = rsp;
            // b5ee6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5ee8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5eea push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b5eec push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b5eed sub rsp,0x20
            { let (r, f) = sub::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // b5ef1 mov r15,rsi
            r15 = rsi;
            // b5ef4 mov r14,rdi
            r14 = rdi;
            // b5ef7 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // b5efb mov rdx,r15
            rdx = r15;
            // b5efe call 0xebc96
            { let r = { let t = m.env.stub_va(0x463); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f03 mov r12,rax
            r12 = rax;
            // b5f06 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // b5f0a test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b5f0d je 0xb5f13
            if fl.e() { pc = 0xb5f13; continue; }
            pc = 0xb5f0f;
        }
        0xb5f0f => {
            // block b5f0f
            #[cfg(feature = "trace")] eprintln!("f_000b5ee2 pc b5f0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f0f xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b5f11 jmp 0xb5f42
            pc = 0xb5f42; continue;
        }
        0xb5f13 => {
            // block b5f13
            #[cfg(feature = "trace")] eprintln!("f_000b5ee2 pc b5f13 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f13 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b5f17 mov rsi,r14
            rsi = r14;
            // b5f1a mov rdx,r15
            rdx = r15;
            // b5f1d call 0xebcae
            { let r = { let t = m.env.stub_va(0x47b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f22 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b5f26 mov rbx,QWORD PTR [rbp-0x40]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // b5f2a mov rdi,r14
            rdi = r14;
            // b5f2d mov rdx,r12
            rdx = r12;
            // b5f30 mov rcx,rbx
            rcx = rbx;
            // b5f33 call 0xebcb4
            { let r = { let t = m.env.stub_va(0x481); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5f38 mov QWORD PTR [rbp-0x40],0x0
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x0u64) as u64);
            // b5f40 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xb5f42;
        }
        0xb5f42 => {
            // block b5f42
            #[cfg(feature = "trace")] eprintln!("f_000b5ee2 pc b5f42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5f42 mov rax,rbx
            rax = rbx;
            // b5f45 add rsp,0x20
            { let (r, f) = add::<64>(rsp, 0x20u64);
              rsp = r;
              fl = f; }
            // b5f49 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b5f4a pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b5f4c pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b5f4e pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b5f50 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b5f51 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2d84 size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2d84<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2d84;
    loop {
        match pc {
        0xe2d84 => {
            // block e2d84
            #[cfg(feature = "trace")] eprintln!("f_000e2d84 pc e2d84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2d84 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2d85 mov rbp,rsp
            rbp = rsp;
            // e2d88 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2d8a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2d8c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2d8e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2d8f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2d93 mov r14,rcx
            r14 = rcx;
            // e2d96 mov r15,rdx
            r15 = rdx;
            // e2d99 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // e2d9c mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2d9f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2da2 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // e2da6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2da8 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2dad mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2db2 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2db5 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2db8 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2dba je 0xe2de5
            if fl.e() { pc = 0xe2de5; continue; }
            pc = 0xe2dbc;
        }
        0xe2dbc => {
            // block e2dbc
            #[cfg(feature = "trace")] eprintln!("f_000e2d84 pc e2dbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2dbc add r12d,r12d
            { let (r, f) = add::<32>((r12 as u32 as u64), (r12 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // e2dbf mov eax,0x3
            rax = (0x3u64) as u32 as u64;
            // e2dc4 mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2dc7 shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e2dc9 and eax,DWORD PTR [rbp-0x24]
            { let r = (rax as u32 as u64) & m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // e2dcc mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2dcf shr eax,cl
            { let (r, f) = shr::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // e2dd1 mov DWORD PTR [rbp-0x24],eax
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rax as u32 as u64)) as u32);
            // e2dd4 mov cl,al
            rcx = (rcx & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // e2dd6 shr cl,1
            { let (r, f) = shr::<8>((rcx as u8 as u64), 0x1u64, fl); rcx = (rcx & !0xff) | ((r) as u8 as u64); fl = f; }
            // e2dd8 and cl,0x1
            { let r = (rcx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rcx = (rcx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2ddb mov BYTE PTR [r15],cl
            m.w8(r15, ((rcx as u8 as u64)) as u8);
            // e2dde and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2de0 mov BYTE PTR [r14],al
            m.w8(r14, ((rax as u8 as u64)) as u8);
            // e2de3 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe2de5;
        }
        0xe2de5 => {
            // block e2de5
            #[cfg(feature = "trace")] eprintln!("f_000e2d84 pc e2de5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2de5 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2de7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2deb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2dec pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2dee pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2df0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2df2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2df3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb5770 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b5770<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb5770;
    loop {
        match pc {
        0xb5770 => {
            // block b5770
            #[cfg(feature = "trace")] eprintln!("f_000b5770 pc b5770 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b5770 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b5771 mov rbp,rsp
            rbp = rsp;
            // b5774 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b5776 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b5778 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b577a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b577b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b577f mov r15,rsi
            r15 = rsi;
            // b5782 mov r14,rdi
            r14 = rdi;
            // b5785 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // b5789 mov rdx,r15
            rdx = r15;
            // b578c call 0xebbf4
            { let r = { let t = m.env.stub_va(0x3c1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b5791 mov r12,rax
            r12 = rax;
            // b5794 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // b5798 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // b579b jne 0xb57d0
            if fl.ne() { pc = 0xb57d0; continue; }
            pc = 0xb579d;
        }
        0xb579d => {
            // block b579d
            #[cfg(feature = "trace")] eprintln!("f_000b5770 pc b579d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b579d mov edi,0x28
            rdi = (0x28u64) as u32 as u64;
            // b57a2 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b57a7 mov rbx,rax
            rbx = rax;
            // b57aa mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // b57ad mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // b57b0 mov DWORD PTR [rbx+0x20],0x0
            m.w32(rbx.wrapping_add(0x20), (0x0u64) as u32);
            // b57b7 mov DWORD PTR [rbx+0x24],0x0
            m.w32(rbx.wrapping_add(0x24), (0x0u64) as u32);
            // b57be mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // b57c2 mov rdi,r14
            rdi = r14;
            // b57c5 mov rdx,r12
            rdx = r12;
            // b57c8 mov rcx,rbx
            rcx = rbx;
            // b57cb call 0xebc12
            { let r = { let t = m.env.stub_va(0x3df); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb57d0;
        }
        0xb57d0 => {
            // block b57d0
            #[cfg(feature = "trace")] eprintln!("f_000b5770 pc b57d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b57d0 add rbx,0x20
            { let (r, f) = add::<64>(rbx, 0x20u64);
              rbx = r;
              fl = f; }
            // b57d4 mov rax,rbx
            rax = rbx;
            // b57d7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b57db pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b57dc pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b57de pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b57e0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b57e2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b57e3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd36d8 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d36d8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd36d8;
    loop {
        match pc {
        0xd36d8 => {
            // block d36d8
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d36d8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d36d8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d36d9 mov rbp,rsp
            rbp = rsp;
            // d36dc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d36dd sub rsp,0x38
            { let (r, f) = sub::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // d36e1 mov rbx,rdx
            rbx = rdx;
            // d36e4 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // d36e7 je 0xd3744
            if fl.e() { pc = 0xd3744; continue; }
            pc = 0xd36e9;
        }
        0xd36e9 => {
            // block d36e9
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d36e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d36e9 xorps xmm0,xmm0
            x0 = 0;
            // d36ec movaps XMMWORD PTR [rbp-0x20],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), x0);
            // d36f0 movaps XMMWORD PTR [rbp-0x30],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), x0);
            // d36f4 movaps XMMWORD PTR [rbp-0x40],xmm0
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x0);
            // d36f8 mov DWORD PTR [rbp-0x40],0x2
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (0x2u64) as u32);
            // d36ff mov DWORD PTR [rbp-0x3c],0x30
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffc4), (0x30u64) as u32);
            // d3706 mov DWORD PTR [rbp-0x28],esi
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), ((rsi as u32 as u64)) as u32);
            // d3709 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // d3710 mov DWORD PTR [rbp-0x20],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0), (0x0u64) as u32);
            // d3717 mov DWORD PTR [rbp-0x1c],ecx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), ((rcx as u32 as u64)) as u32);
            // d371a mov DWORD PTR [rbp-0x18],r8d
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe8), ((r8 as u32 as u64)) as u32);
            // d371e mov BYTE PTR [rbp-0x14],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u8);
            // d3722 mov BYTE PTR [rbp-0x13],0x1
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffed), (0x1u64) as u8);
            // d3726 mov BYTE PTR [rbp-0x12],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffee), (0x0u64) as u8);
            // d372a mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // d372d lea rsi,[rbp-0x40]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // d3731 call QWORD PTR [rax+0x168]
            { let r = { let t = m.r64(rax.wrapping_add(0x168)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d3737 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // d3739 je 0xd3744
            if fl.e() { pc = 0xd3744; continue; }
            pc = 0xd373b;
        }
        0xd373b => {
            // block d373b
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d373b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d373b mov eax,DWORD PTR [rbp-0x20]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe0)) as u64) as u32 as u64;
            // d373e mov DWORD PTR [rbx],eax
            m.w32(rbx, ((rax as u32 as u64)) as u32);
            // d3740 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d3742 jmp 0xd3746
            pc = 0xd3746; continue;
        }
        0xd3744 => {
            // block d3744
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d3744 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3744 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd3746;
        }
        0xd3746 => {
            // block d3746
            #[cfg(feature = "trace")] eprintln!("f_000d36d8 pc d3746 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d3746 add rsp,0x38
            { let (r, f) = add::<64>(rsp, 0x38u64);
              rsp = r;
              fl = f; }
            // d374a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d374b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d374c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2e82 size=118
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2e82<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2e82;
    loop {
        match pc {
        0xe2e82 => {
            // block e2e82
            #[cfg(feature = "trace")] eprintln!("f_000e2e82 pc e2e82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2e82 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2e83 mov rbp,rsp
            rbp = rsp;
            // e2e86 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2e88 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2e8a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // e2e8c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2e8d sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2e91 mov r14,rcx
            r14 = rcx;
            // e2e94 mov r15,rdx
            r15 = rdx;
            // e2e97 mov r12d,esi
            r12 = ((rsi as u32 as u64)) as u32 as u64;
            // e2e9a mov rdi,QWORD PTR [rdi]
            rdi = m.r64(rdi) as u64;
            // e2e9d mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2ea0 lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // e2ea4 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // e2ea6 mov esi,0x4001a
            rsi = (0x4001au64) as u32 as u64;
            // e2eab mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2eb0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2eb3 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2eb6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // e2eb8 je 0xe2ee8
            if fl.e() { pc = 0xe2ee8; continue; }
            pc = 0xe2eba;
        }
        0xe2eba => {
            // block e2eba
            #[cfg(feature = "trace")] eprintln!("f_000e2e82 pc e2eba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2eba mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // e2ebd shr eax,0x8
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x8u64, fl); rax = (r) as u32 as u64; fl = f; }
            // e2ec0 add r12d,r12d
            { let (r, f) = add::<32>((r12 as u32 as u64), (r12 as u32 as u64));
              r12 = (r) as u32 as u64;
              fl = f; }
            // e2ec3 mov edx,0x3
            rdx = (0x3u64) as u32 as u64;
            // e2ec8 mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2ecb shl edx,cl
            { let (r, f) = shl::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e2ecd and edx,eax
            { let r = (rdx as u32 as u64) & (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // e2ecf mov cl,r12b
            rcx = (rcx & !0xff) | (((r12 as u8 as u64)) as u8 as u64);
            // e2ed2 shr edx,cl
            { let (r, f) = shr::<32>((rdx as u32 as u64), (rcx as u8 as u64), fl); rdx = (r) as u32 as u64; fl = f; }
            // e2ed4 mov DWORD PTR [rbp-0x24],edx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), ((rdx as u32 as u64)) as u32);
            // e2ed7 mov al,dl
            rax = (rax & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // e2ed9 shr al,1
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x1u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // e2edb and al,0x1
            { let r = (rax as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2edd mov BYTE PTR [r15],al
            m.w8(r15, ((rax as u8 as u64)) as u8);
            // e2ee0 and dl,0x1
            { let r = (rdx as u8 as u64) & 0x1u64; let f = logic::<8>(r);
              rdx = (rdx & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // e2ee3 mov BYTE PTR [r14],dl
            m.w8(r14, ((rdx as u8 as u64)) as u8);
            // e2ee6 mov bl,0x1
            rbx = (rbx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xe2ee8;
        }
        0xe2ee8 => {
            // block e2ee8
            #[cfg(feature = "trace")] eprintln!("f_000e2e82 pc e2ee8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2ee8 mov al,bl
            rax = (rax & !0xff) | (((rbx as u8 as u64)) as u8 as u64);
            // e2eea add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // e2eee pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2eef pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // e2ef1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2ef3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2ef5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2ef6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3bd0 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3bd0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3bd0;
    loop {
        match pc {
        0xc3bd0 => {
            // block c3bd0
            #[cfg(feature = "trace")] eprintln!("f_000c3bd0 pc c3bd0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3bd0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3bd1 mov rbp,rsp
            rbp = rsp;
            // c3bd4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3bd6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3bd8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3bda push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3bdb sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3bdf mov r12,rsi
            r12 = rsi;
            // c3be2 mov r14,rdi
            r14 = rdi;
            // c3be5 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3be9 mov rdx,r12
            rdx = r12;
            // c3bec call 0xebd32
            { let r = { let t = m.env.stub_va(0x4ff); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3bf1 mov r15,rax
            r15 = rax;
            // c3bf4 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3bf7 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3bfa jne 0xc3c3a
            if fl.ne() { pc = 0xc3c3a; continue; }
            pc = 0xc3bfc;
        }
        0xc3bfc => {
            // block c3bfc
            #[cfg(feature = "trace")] eprintln!("f_000c3bd0 pc c3bfc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3bfc mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c3c01 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3c06 mov rbx,rax
            rbx = rax;
            // c3c09 mov eax,DWORD PTR [r12]
            rax = (m.r32(r12) as u64) as u32 as u64;
            // c3c0d mov DWORD PTR [rbx+0x20],eax
            m.w32(rbx.wrapping_add(0x20), ((rax as u32 as u64)) as u32);
            // c3c10 mov QWORD PTR [rbx+0x38],0x0
            m.w64(rbx.wrapping_add(0x38), (0x0u64) as u64);
            // c3c18 mov QWORD PTR [rbx+0x30],0x0
            m.w64(rbx.wrapping_add(0x30), (0x0u64) as u64);
            // c3c20 mov QWORD PTR [rbx+0x28],0x0
            m.w64(rbx.wrapping_add(0x28), (0x0u64) as u64);
            // c3c28 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3c2c mov rdi,r14
            rdi = r14;
            // c3c2f mov rdx,r15
            rdx = r15;
            // c3c32 mov rcx,rbx
            rcx = rbx;
            // c3c35 call 0xebe4c
            { let r = { let t = m.env.stub_va(0x619); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3c3a;
        }
        0xc3c3a => {
            // block c3c3a
            #[cfg(feature = "trace")] eprintln!("f_000c3bd0 pc c3c3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3c3a add rbx,0x28
            { let (r, f) = add::<64>(rbx, 0x28u64);
              rbx = r;
              fl = f; }
            // c3c3e mov rax,rbx
            rax = rbx;
            // c3c41 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3c45 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3c46 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3c48 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3c4a pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3c4c pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3c4d ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xe2c64 size=148
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000e2c64<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xe2c64;
    loop {
        match pc {
        0xe2c64 => {
            // block e2c64
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c64 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // e2c65 mov rbp,rsp
            rbp = rsp;
            // e2c68 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // e2c6a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // e2c6c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // e2c6d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // e2c6e mov r14d,r8d
            r14 = ((r8 as u32 as u64)) as u32 as u64;
            // e2c71 mov r15,rdi
            r15 = rdi;
            // e2c74 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // e2c7b mov ebx,0x43864
            rbx = (0x43864u64) as u32 as u64;
            // e2c80 cmp ecx,0x3
            fl = sub::<32>((rcx as u32 as u64), 0x3u64).1;
            // e2c83 ja 0xe2c9c
            if fl.a() { pc = 0xe2c9c; continue; }
            pc = 0xe2c85;
        }
        0xe2c85 => {
            // block e2c85
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c85 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c85 cmp esi,0x1
            fl = sub::<32>((rsi as u32 as u64), 0x1u64).1;
            // e2c88 ja 0xe2c9c
            if fl.a() { pc = 0xe2c9c; continue; }
            pc = 0xe2c8a;
        }
        0xe2c8a => {
            // block e2c8a
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c8a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c8a cmp edx,0x7
            fl = sub::<32>((rdx as u32 as u64), 0x7u64).1;
            // e2c8d ja 0xe2c9c
            if fl.a() { pc = 0xe2c9c; continue; }
            pc = 0xe2c8f;
        }
        0xe2c8f => {
            // block e2c8f
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c8f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c8f lea eax,[rdx+rsi*4]
            rax = (rdx.wrapping_add(rsi.wrapping_mul(4))) as u32 as u64;
            // e2c92 lea eax,[rcx+rax*4]
            rax = (rcx.wrapping_add(rax.wrapping_mul(4))) as u32 as u64;
            // e2c95 lea ebx,[rax+rax*2+0x43864]
            rbx = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x43864)) as u32 as u64;
            pc = 0xe2c9c;
        }
        0xe2c9c => {
            // block e2c9c
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2c9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2c9c mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e2c9f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2ca2 lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // e2ca6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2cab xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2cae mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2cb0 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2cb3 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // e2cb7 je 0xe2cec
            if fl.e() { pc = 0xe2cec; continue; }
            pc = 0xe2cb9;
        }
        0xe2cb9 => {
            // block e2cb9
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2cb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2cb9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e2cbc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2cbf mov edx,0x80000000
            rdx = (0x80000000u64) as u32 as u64;
            // e2cc4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2cc9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2ccc mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2cce call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // e2cd1 movzx eax,r14b
            rax = ((r14 as u8 as u64)) as u32 as u64;
            // e2cd5 lea edx,[rax+rax*1+0x1]
            rdx = (rax.wrapping_add(rax).wrapping_add(0x1)) as u32 as u64;
            // e2cd9 mov rdi,QWORD PTR [r15]
            rdi = m.r64(r15) as u64;
            // e2cdc mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // e2cdf mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // e2ce4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // e2ce7 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // e2ce9 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xe2cec;
        }
        0xe2cec => {
            // block e2cec
            #[cfg(feature = "trace")] eprintln!("f_000e2c64 pc e2cec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // e2cec add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // e2cf0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // e2cf1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // e2cf3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // e2cf5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // e2cf6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3b36 size=154
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3b36<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3b36;
    loop {
        match pc {
        0xc3b36 => {
            // block c3b36
            #[cfg(feature = "trace")] eprintln!("f_000c3b36 pc c3b36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3b36 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3b37 mov rbp,rsp
            rbp = rsp;
            // c3b3a push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3b3c push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3b3e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c3b40 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c3b41 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3b45 mov r12,rsi
            r12 = rsi;
            // c3b48 mov r14,rdi
            r14 = rdi;
            // c3b4b lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // c3b4f mov rdx,r12
            rdx = r12;
            // c3b52 call 0xebd3e
            { let r = { let t = m.env.stub_va(0x50b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b57 mov r15,rax
            r15 = rax;
            // c3b5a mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // c3b5d test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // c3b60 jne 0xc3bbc
            if fl.ne() { pc = 0xc3bbc; continue; }
            pc = 0xc3b62;
        }
        0xc3b62 => {
            // block c3b62
            #[cfg(feature = "trace")] eprintln!("f_000c3b36 pc c3b62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3b62 mov edi,0x40
            rdi = (0x40u64) as u32 as u64;
            // c3b67 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3b6c mov rbx,rax
            rbx = rax;
            // c3b6f mov rax,QWORD PTR [r12+0x10]
            rax = m.r64(r12.wrapping_add(0x10)) as u64;
            // c3b74 mov QWORD PTR [rbx+0x30],rax
            m.w64(rbx.wrapping_add(0x30), (rax) as u64);
            // c3b78 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // c3b7c mov rcx,QWORD PTR [r12+0x8]
            rcx = m.r64(r12.wrapping_add(0x8)) as u64;
            // c3b81 mov QWORD PTR [rbx+0x28],rcx
            m.w64(rbx.wrapping_add(0x28), (rcx) as u64);
            // c3b85 mov QWORD PTR [rbx+0x20],rax
            m.w64(rbx.wrapping_add(0x20), (rax) as u64);
            // c3b89 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // c3b92 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // c3b9b mov QWORD PTR [r12],0x0
            m.w64(r12, (0x0u64) as u64);
            // c3ba3 mov DWORD PTR [rbx+0x38],0x0
            m.w32(rbx.wrapping_add(0x38), (0x0u64) as u32);
            // c3baa mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // c3bae mov rdi,r14
            rdi = r14;
            // c3bb1 mov rdx,r15
            rdx = r15;
            // c3bb4 mov rcx,rbx
            rcx = rbx;
            // c3bb7 call 0xebe64
            { let r = { let t = m.env.stub_va(0x631); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xc3bbc;
        }
        0xc3bbc => {
            // block c3bbc
            #[cfg(feature = "trace")] eprintln!("f_000c3b36 pc c3bbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3bbc add rbx,0x38
            { let (r, f) = add::<64>(rbx, 0x38u64);
              rbx = r;
              fl = f; }
            // c3bc0 mov rax,rbx
            rax = rbx;
            // c3bc3 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // c3bc7 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c3bc8 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c3bca pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c3bcc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c3bce pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3bcf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xd5c8e size=156
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000d5c8e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xd5c8e;
    loop {
        match pc {
        0xd5c8e => {
            // block d5c8e
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5c8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5c8e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // d5c8f mov rbp,rsp
            rbp = rsp;
            // d5c92 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // d5c93 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // d5c94 mov rbx,rdi
            rbx = rdi;
            // d5c97 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // d5c9e mov DWORD PTR [rbp-0x10],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff0), (0x0u64) as u32);
            // d5ca5 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5ca8 lea rdx,[rbp-0x10]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff0);
            // d5cac mov esi,0x32
            rsi = (0x32u64) as u32 as u64;
            // d5cb1 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5cb6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5cb9 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cbc mov ecx,DWORD PTR [rbp-0x10]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff0)) as u64) as u32 as u64;
            // d5cbf cmp ecx,0x106466ff
            fl = sub::<32>((rcx as u32 as u64), 0x106466ffu64).1;
            // d5cc5 jg 0xd5cfe
            if fl.g() { pc = 0xd5cfe; continue; }
            pc = 0xd5cc7;
        }
        0xd5cc7 => {
            // block d5cc7
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cc7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cc7 cmp ecx,0x10518400
            fl = sub::<32>((rcx as u32 as u64), 0x10518400u64).1;
            // d5ccd je 0xd5cd7
            if fl.e() { pc = 0xd5cd7; continue; }
            pc = 0xd5ccf;
        }
        0xd5ccf => {
            // block d5ccf
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5ccf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5ccf cmp ecx,0x10518450
            fl = sub::<32>((rcx as u32 as u64), 0x10518450u64).1;
            // d5cd5 jne 0xd5d21
            if fl.ne() { pc = 0xd5d21; continue; }
            pc = 0xd5cd7;
        }
        0xd5cd7 => {
            // block d5cd7
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cd7 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // d5cda lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // d5cde mov esi,0x40020
            rsi = (0x40020u64) as u32 as u64;
            // d5ce3 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // d5ce8 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // d5ceb mov rdi,rbx
            rdi = rbx;
            // d5cee call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // d5cf1 mov eax,DWORD PTR [rbp-0xc]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // d5cf4 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // d5cf6 cmp eax,0x1
            fl = sub::<32>((rax as u32 as u64), 0x1u64).1;
            // d5cf9 seta al
            rax = (rax & !0xff) | (((fl.a() as u64)) as u8 as u64);
            // d5cfc jmp 0xd5d23
            pc = 0xd5d23; continue;
        }
        0xd5cfe => {
            // block d5cfe
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5cfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5cfe mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // d5d00 lea edx,[rcx-0x10646700]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_9900)) as u32 as u64;
            // d5d06 cmp edx,0x4
            fl = sub::<32>((rdx as u32 as u64), 0x4u64).1;
            // d5d09 jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d0b;
        }
        0xd5d0b => {
            // block d5d0b
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d0b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d0b lea edx,[rcx-0x10646705]
            rdx = (rcx.wrapping_add(0xffff_ffff_ef9b_98fb)) as u32 as u64;
            // d5d11 cmp edx,0x2
            fl = sub::<32>((rdx as u32 as u64), 0x2u64).1;
            // d5d14 jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d16;
        }
        0xd5d16 => {
            // block d5d16
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d16 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d16 add ecx,0xef8ef7b0
            { let (r, f) = add::<32>((rcx as u32 as u64), 0xef8ef7b0u64);
              rcx = (r) as u32 as u64;
              fl = f; }
            // d5d1c cmp ecx,0x2
            fl = sub::<32>((rcx as u32 as u64), 0x2u64).1;
            // d5d1f jb 0xd5d23
            if fl.b() { pc = 0xd5d23; continue; }
            pc = 0xd5d21;
        }
        0xd5d21 => {
            // block d5d21
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d21 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xd5d23;
        }
        0xd5d23 => {
            // block d5d23
            #[cfg(feature = "trace")] eprintln!("f_000d5c8e pc d5d23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d5d23 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // d5d27 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // d5d28 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // d5d29 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3901 size=207
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3901<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xc3901;
    loop {
        match pc {
        0xc3901 => {
            // block c3901
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3901 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3901 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3902 mov rbp,rsp
            rbp = rsp;
            // c3905 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c3907 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c3909 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // c390b push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // c390d push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c390e sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // c3912 mov r14,rsi
            r14 = rsi;
            // c3915 mov rbx,rdi
            rbx = rdi;
            // c3918 mov r13,QWORD PTR [r14]
            r13 = m.r64(r14) as u64;
            // c391b add r14,0x8
            { let (r, f) = add::<64>(r14, 0x8u64);
              r14 = r;
              fl = f; }
            // c391f cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // c3922 je 0xc39a9
            if fl.e() { pc = 0xc39a9; continue; }
            pc = 0xc3928;
        }
        0xc3928 => {
            // block c3928
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3928 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3928 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // c392c jmp 0xc3945
            pc = 0xc3945; continue;
        }
        0xc392e => {
            // block c392e
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c392e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c392e mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // c3933 mov rdi,rbx
            rdi = rbx;
            // c3936 lea rsi,[rip+0x66e26] # 0x12a763
            rsi = m.env.data_va(257840);
            // c393d call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3942 mov r13,r12
            r13 = r12;
            pc = 0xc3945;
        }
        0xc3945 => {
            // block c3945
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3945 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3945 mov esi,DWORD PTR [r13+0x1c]
            rsi = (m.r32(r13.wrapping_add(0x1c)) as u64) as u32 as u64;
            // c3949 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // c394e mov rdi,r15
            rdi = r15;
            // c3951 call 0x5f77c
            { let r = { let t = m.env.code_va(0x5f77c); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3956 movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // c395a test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // c395d jne 0xc3968
            if fl.ne() { pc = 0xc3968; continue; }
            pc = 0xc395f;
        }
        0xc395f => {
            // block c395f
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c395f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c395f shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // c3962 lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // c3966 jmp 0xc3970
            pc = 0xc3970; continue;
        }
        0xc3968 => {
            // block c3968
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3968 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3968 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // c396c mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xc3970;
        }
        0xc3970 => {
            // block c3970
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3970 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3970 mov rdi,rbx
            rdi = rbx;
            // c3973 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3978 mov rdi,r15
            rdi = r15;
            // c397b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c3980 mov rax,QWORD PTR [r13+0x8]
            rax = m.r64(r13.wrapping_add(0x8)) as u64;
            // c3984 test rax,rax
            fl = logic::<64>(rax & rax);
            // c3987 je 0xc3997
            if fl.e() { pc = 0xc3997; continue; }
            pc = 0xc3989;
        }
        0xc3989 => {
            // block c3989
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3989 mov r12,rax
            r12 = rax;
            // c398c mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // c3990 test rax,rax
            fl = logic::<64>(rax & rax);
            // c3993 jne 0xc3989
            if fl.ne() { pc = 0xc3989; continue; }
            pc = 0xc3995;
        }
        0xc3995 => {
            // block c3995
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3995 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3995 jmp 0xc39a4
            pc = 0xc39a4; continue;
        }
        0xc3997 => {
            // block c3997
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c3997 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3997 mov r12,QWORD PTR [r13+0x10]
            r12 = m.r64(r13.wrapping_add(0x10)) as u64;
            // c399b cmp QWORD PTR [r12],r13
            fl = sub::<64>(m.r64(r12) as u64, r13).1;
            // c399f mov r13,r12
            r13 = r12;
            // c39a2 jne 0xc3997
            if fl.ne() { pc = 0xc3997; continue; }
            pc = 0xc39a4;
        }
        0xc39a4 => {
            // block c39a4
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c39a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c39a4 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // c39a7 jne 0xc392e
            if fl.ne() { pc = 0xc392e; continue; }
            pc = 0xc39a9;
        }
        0xc39a9 => {
            // block c39a9
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c39a9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c39a9 mov rax,rbx
            rax = rbx;
            // c39ac add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // c39b0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c39b1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // c39b3 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // c39b5 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c39b7 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c39b9 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c39ba ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xc39bb => {
            // block c39bb
            #[cfg(feature = "trace")] eprintln!("f_000c3901 pc c39bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c39bb mov rbx,rax
            rbx = rax;
            // c39be lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // c39c2 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c39c7 mov rdi,rbx
            rdi = rbx;
            // c39ca call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb7e5e size=210
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b7e5e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb7e5e;
    loop {
        match pc {
        0xb7e5e => {
            // block b7e5e
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7e5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e5e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b7e5f mov rbp,rsp
            rbp = rsp;
            // b7e62 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b7e64 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b7e65 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7e69 mov rbx,rdi
            rbx = rdi;
            // b7e6c mov DWORD PTR [rbp-0x14],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffec), (0x0u64) as u32);
            // b7e73 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7e76 call QWORD PTR [rax+0x108]
            { let r = { let t = m.r64(rax.wrapping_add(0x108)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7e7c xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b7e7f test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7e81 je 0xb7f23
            if fl.e() { pc = 0xb7f23; continue; }
            pc = 0xb7e87;
        }
        0xb7e87 => {
            // block b7e87
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7e87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7e87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // b7e8a lea rdx,[rbp-0x14]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffec);
            // b7e8e xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // b7e91 mov esi,0x18
            rsi = (0x18u64) as u32 as u64;
            // b7e96 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // b7e9b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b7e9e mov rdi,rbx
            rdi = rbx;
            // b7ea1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b7ea4 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // b7ea6 je 0xb7f23
            if fl.e() { pc = 0xb7f23; continue; }
            pc = 0xb7ea8;
        }
        0xb7ea8 => {
            // block b7ea8
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ea8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ea8 mov eax,DWORD PTR [rbx+0x10]
            rax = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b7eab cmp eax,0x104782ff
            fl = sub::<32>((rax as u32 as u64), 0x104782ffu64).1;
            // b7eb0 jg 0xb7ed2
            if fl.g() { pc = 0xb7ed2; continue; }
            pc = 0xb7eb2;
        }
        0xb7eb2 => {
            // block b7eb2
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7eb2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7eb2 cmp eax,0x1032294f
            fl = sub::<32>((rax as u32 as u64), 0x1032294fu64).1;
            // b7eb7 jg 0xb7ee9
            if fl.g() { pc = 0xb7ee9; continue; }
            pc = 0xb7eb9;
        }
        0xb7eb9 => {
            // block b7eb9
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7eb9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7eb9 cmp eax,0x10266400
            fl = sub::<32>((rax as u32 as u64), 0x10266400u64).1;
            // b7ebe jne 0xb7f09
            if fl.ne() { pc = 0xb7f09; continue; }
            pc = 0xb7ec0;
        }
        0xb7ec0 => {
            // block b7ec0
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ec0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ec0 mov al,BYTE PTR [rbp-0x11]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u8 as u64);
            // b7ec3 and al,0x8
            { let r = (rax as u8 as u64) & 0x8u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7ec5 shr al,0x3
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x3u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // b7ec8 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b7ecb lea r14d,[rax+rax*4+0x1]
            r14 = (rax.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1)) as u32 as u64;
            // b7ed0 jmp 0xb7f23
            pc = 0xb7f23; continue;
        }
        0xb7ed2 => {
            // block b7ed2
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ed2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ed2 cmp eax,0x10478300
            fl = sub::<32>((rax as u32 as u64), 0x10478300u64).1;
            // b7ed7 je 0xb7f10
            if fl.e() { pc = 0xb7f10; continue; }
            pc = 0xb7ed9;
        }
        0xb7ed9 => {
            // block b7ed9
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ed9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ed9 cmp eax,0x10518400
            fl = sub::<32>((rax as u32 as u64), 0x10518400u64).1;
            // b7ede je 0xb7f10
            if fl.e() { pc = 0xb7f10; continue; }
            pc = 0xb7ee0;
        }
        0xb7ee0 => {
            // block b7ee0
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ee0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ee0 cmp eax,0x10518450
            fl = sub::<32>((rax as u32 as u64), 0x10518450u64).1;
            // b7ee5 jne 0xb7f23
            if fl.ne() { pc = 0xb7f23; continue; }
            pc = 0xb7ee7;
        }
        0xb7ee7 => {
            // block b7ee7
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ee7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ee7 jmp 0xb7f10
            pc = 0xb7f10; continue;
        }
        0xb7ee9 => {
            // block b7ee9
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ee9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ee9 cmp eax,0x10322950
            fl = sub::<32>((rax as u32 as u64), 0x10322950u64).1;
            // b7eee je 0xb7f10
            if fl.e() { pc = 0xb7f10; continue; }
            pc = 0xb7ef0;
        }
        0xb7ef0 => {
            // block b7ef0
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ef0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ef0 cmp eax,0x10352300
            fl = sub::<32>((rax as u32 as u64), 0x10352300u64).1;
            // b7ef5 jne 0xb7f23
            if fl.ne() { pc = 0xb7f23; continue; }
            pc = 0xb7ef7;
        }
        0xb7ef7 => {
            // block b7ef7
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7ef7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7ef7 mov al,BYTE PTR [rbp-0x11]
            rax = (rax & !0xff) | ((m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64) as u8 as u64);
            // b7efa and al,0x8
            { let r = (rax as u8 as u64) & 0x8u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // b7efc shr al,0x3
            { let (r, f) = shr::<8>((rax as u8 as u64), 0x3u64, fl); rax = (rax & !0xff) | ((r) as u8 as u64); fl = f; }
            // b7eff movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // b7f02 lea r14d,[rax+rax*2+0x1]
            r14 = (rax.wrapping_add(rax.wrapping_mul(2)).wrapping_add(0x1)) as u32 as u64;
            // b7f07 jmp 0xb7f23
            pc = 0xb7f23; continue;
        }
        0xb7f09 => {
            // block b7f09
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7f09 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7f09 cmp eax,0x10294700
            fl = sub::<32>((rax as u32 as u64), 0x10294700u64).1;
            // b7f0e jne 0xb7f23
            if fl.ne() { pc = 0xb7f23; continue; }
            pc = 0xb7f10;
        }
        0xb7f10 => {
            // block b7f10
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7f10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7f10 test BYTE PTR [rbp-0x11],0x8
            fl = logic::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64 & 0x8u64);
            // b7f14 mov eax,0x2
            rax = (0x2u64) as u32 as u64;
            // b7f19 mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // b7f1f cmove r14d,eax
            if fl.e() { r14 = ((rax as u32 as u64)) as u32 as u64; } else { r14 = r14 as u32 as u64; }
            pc = 0xb7f23;
        }
        0xb7f23 => {
            // block b7f23
            #[cfg(feature = "trace")] eprintln!("f_000b7e5e pc b7f23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b7f23 mov eax,r14d
            rax = ((r14 as u32 as u64)) as u32 as u64;
            // b7f26 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // b7f2a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b7f2b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b7f2d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b7f2e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xb8e79 size=241
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b8e79<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0xb8e79;
    loop {
        match pc {
        0xb8e79 => {
            // block b8e79
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8e79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8e79 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b8e7a mov rbp,rsp
            rbp = rsp;
            // b8e7d push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // b8e7f push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // b8e81 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // b8e83 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // b8e85 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // b8e86 sub rsp,0x28
            { let (r, f) = sub::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b8e8a mov r12,rsi
            r12 = rsi;
            // b8e8d mov QWORD PTR [rbp-0x48],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rdi) as u64);
            // b8e91 mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8e95 lea r13,[r12+0x8]
            r13 = r12.wrapping_add(0x8);
            // b8e9a cmp rcx,r13
            fl = sub::<64>(rcx, r13).1;
            // b8e9d je 0xb8f43
            if fl.e() { pc = 0xb8f43; continue; }
            pc = 0xb8ea3;
        }
        0xb8ea3 => {
            // block b8ea3
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8ea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ea3 lea r15,[rbp-0x40]
            r15 = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8ea7 mov r14,rcx
            r14 = rcx;
            // b8eaa jmp 0xb8eb3
            pc = 0xb8eb3; continue;
        }
        0xb8eac => {
            // block b8eac
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8eac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8eac mov rcx,QWORD PTR [r12]
            rcx = m.r64(r12) as u64;
            // b8eb0 mov r14,rax
            r14 = rax;
            pc = 0xb8eb3;
        }
        0xb8eb3 => {
            // block b8eb3
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8eb3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8eb3 cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // b8eb6 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // b8eb9 lea rsi,[rip+0x7fe08] # 0x138cc8
            rsi = m.env.data_va(316565);
            // b8ec0 lea rcx,[rip+0x715d6] # 0x12a49d
            rcx = m.env.data_va(257130);
            // b8ec7 cmovne rsi,rcx
            if fl.ne() { rsi = rcx; }
            // b8ecb movzx edx,al
            rdx = ((rax as u8 as u64)) as u32 as u64;
            // b8ece add rdx,rdx
            { let (r, f) = add::<64>(rdx, rdx);
              rdx = r;
              fl = f; }
            // b8ed1 mov rdi,QWORD PTR [rbp-0x48]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b8ed5 call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8eda mov rbx,rax
            rbx = rax;
            // b8edd mov esi,DWORD PTR [r14+0x1c]
            rsi = (m.r32(r14.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b8ee1 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // b8ee6 mov rdi,r15
            rdi = r15;
            // b8ee9 call 0x60ec3
            { let r = { let t = m.env.code_va(0x60ec3); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8eee movzx edx,BYTE PTR [rbp-0x40]
            rdx = (m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64) as u32 as u64;
            // b8ef2 test dl,0x1
            fl = logic::<8>((rdx as u8 as u64) & 0x1u64);
            // b8ef5 jne 0xb8f00
            if fl.ne() { pc = 0xb8f00; continue; }
            pc = 0xb8ef7;
        }
        0xb8ef7 => {
            // block b8ef7
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8ef7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8ef7 shr rdx,1
            { let (r, f) = shr::<64>(rdx, 0x1u64, fl); rdx = r; fl = f; }
            // b8efa lea rsi,[rbp-0x3f]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffc1);
            // b8efe jmp 0xb8f08
            pc = 0xb8f08; continue;
        }
        0xb8f00 => {
            // block b8f00
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8f00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f00 mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // b8f04 mov rsi,QWORD PTR [rbp-0x30]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            pc = 0xb8f08;
        }
        0xb8f08 => {
            // block b8f08
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8f08 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f08 mov rdi,rbx
            rdi = rbx;
            // b8f0b call 0xebb88
            { let r = { let t = m.env.stub_va(0x355); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8f10 mov rdi,r15
            rdi = r15;
            // b8f13 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8f18 mov rcx,QWORD PTR [r14+0x8]
            rcx = m.r64(r14.wrapping_add(0x8)) as u64;
            // b8f1c test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b8f1f je 0xb8f2e
            if fl.e() { pc = 0xb8f2e; continue; }
            pc = 0xb8f21;
        }
        0xb8f21 => {
            // block b8f21
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8f21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f21 mov rax,rcx
            rax = rcx;
            // b8f24 mov rcx,QWORD PTR [rax]
            rcx = m.r64(rax) as u64;
            // b8f27 test rcx,rcx
            fl = logic::<64>(rcx & rcx);
            // b8f2a jne 0xb8f21
            if fl.ne() { pc = 0xb8f21; continue; }
            pc = 0xb8f2c;
        }
        0xb8f2c => {
            // block b8f2c
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8f2c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f2c jmp 0xb8f3a
            pc = 0xb8f3a; continue;
        }
        0xb8f2e => {
            // block b8f2e
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8f2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f2e mov rax,QWORD PTR [r14+0x10]
            rax = m.r64(r14.wrapping_add(0x10)) as u64;
            // b8f32 cmp QWORD PTR [rax],r14
            fl = sub::<64>(m.r64(rax) as u64, r14).1;
            // b8f35 mov r14,rax
            r14 = rax;
            // b8f38 jne 0xb8f2e
            if fl.ne() { pc = 0xb8f2e; continue; }
            pc = 0xb8f3a;
        }
        0xb8f3a => {
            // block b8f3a
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8f3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f3a cmp rax,r13
            fl = sub::<64>(rax, r13).1;
            // b8f3d jne 0xb8eac
            if fl.ne() { pc = 0xb8eac; continue; }
            pc = 0xb8f43;
        }
        0xb8f43 => {
            // block b8f43
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8f43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f43 mov rax,QWORD PTR [rbp-0x48]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // b8f47 add rsp,0x28
            { let (r, f) = add::<64>(rsp, 0x28u64);
              rsp = r;
              fl = f; }
            // b8f4b pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b8f4c pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // b8f4e pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // b8f50 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b8f52 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // b8f54 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b8f55 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0xb8f56 => {
            // block b8f56
            #[cfg(feature = "trace")] eprintln!("f_000b8e79 pc b8f56 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b8f56 mov rbx,rax
            rbx = rax;
            // b8f59 lea rdi,[rbp-0x40]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffc0);
            // b8f5d call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b8f62 mov rdi,rbx
            rdi = rbx;
            // b8f65 call 0xec182
            { let r = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            panic!("fell off the end of the function");
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
