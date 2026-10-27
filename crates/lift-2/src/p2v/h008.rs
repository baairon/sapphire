//! Mechanically lifted functions (scripts/lift_run.py), module h008. Each one is checked against the original
//! in the world harness (tests/world_h008.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x44cca size=54
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044cca<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44cca;
    loop {
        match pc {
        0x44cca => {
            // block 44cca
            #[cfg(feature = "trace")] eprintln!("f_00044cca pc 44cca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44cca push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44ccb mov rbp,rsp
            rbp = rsp;
            // 44cce sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44cd2 mov r9d,ecx
            r9 = ((rcx as u32 as u64)) as u32 as u64;
            // 44cd5 mov r10,rdx
            r10 = rdx;
            // 44cd8 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 44cda mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 44cdd mov DWORD PTR [rsp],r8d
            m.w32(rsp, ((r8 as u32 as u64)) as u32);
            // 44ce1 mov DWORD PTR [rsp+0x8],0x1
            m.w32(rsp.wrapping_add(0x8), (0x1u64) as u32);
            // 44ce9 mov esi,0x7
            rsi = (0x7u64) as u32 as u64;
            // 44cee mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 44cf3 mov r8,r10
            r8 = r10;
            // 44cf6 call QWORD PTR [rax+0x58]
            { let r = { let t = m.r64(rax.wrapping_add(0x58)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44cf9 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44cfd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44cfe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44b10 size=78
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044b10<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x44b10;
    loop {
        match pc {
        0x44b10 => {
            // block 44b10
            #[cfg(feature = "trace")] eprintln!("f_00044b10 pc 44b10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b10 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44b11 mov rbp,rsp
            rbp = rsp;
            // 44b14 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44b16 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 44b17 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44b1b mov r14,rsi
            r14 = rsi;
            // 44b1e mov rbx,rdi
            rbx = rdi;
            // 44b21 lea rdx,[r14+0x1c]
            rdx = r14.wrapping_add(0x1c);
            // 44b25 lea rsi,[rbp-0x18]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 44b29 call 0xebb46
            { let r = { let t = m.env.stub_va(0x313); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44b2e mov rcx,rax
            rcx = rax;
            // 44b31 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 44b34 test rax,rax
            fl = logic::<64>(rax & rax);
            // 44b37 je 0x44b3d
            if fl.e() { pc = 0x44b3d; continue; }
            pc = 0x44b39;
        }
        0x44b39 => {
            // block 44b39
            #[cfg(feature = "trace")] eprintln!("f_00044b10 pc 44b39 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b39 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 44b3b jmp 0x44b54
            pc = 0x44b54; continue;
        }
        0x44b3d => {
            // block 44b3d
            #[cfg(feature = "trace")] eprintln!("f_00044b10 pc 44b3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b3d mov rsi,QWORD PTR [rbp-0x18]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 44b41 mov rdi,rbx
            rdi = rbx;
            // 44b44 mov rdx,rcx
            rdx = rcx;
            // 44b47 mov rcx,r14
            rcx = r14;
            // 44b4a call 0xebb5e
            { let r = { let t = m.env.stub_va(0x32b); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44b4f mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            // 44b51 mov rax,r14
            rax = r14;
            pc = 0x44b54;
        }
        0x44b54 => {
            // block 44b54
            #[cfg(feature = "trace")] eprintln!("f_00044b10 pc 44b54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44b54 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 44b58 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 44b59 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 44b5b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 44b5c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5226e size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005226e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5226e;
    loop {
        match pc {
        0x5226e => {
            // block 5226e
            #[cfg(feature = "trace")] eprintln!("f_0005226e pc 5226e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5226e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5226f mov rbp,rsp
            rbp = rsp;
            // 52272 mov rdx,rsi
            rdx = rsi;
            // 52275 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 52278 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 5227c mov esi,0x2713
            rsi = (0x2713u64) as u32 as u64;
            // 52281 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 52286 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 52289 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 5228a jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x4242e size=40
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0004242e<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x4242e;
    loop {
        match pc {
        0x4242e => {
            // block 4242e
            #[cfg(feature = "trace")] eprintln!("f_0004242e pc 4242e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4242e push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 4242f mov rbp,rsp
            rbp = rsp;
            // 42432 mov r9,rsi
            r9 = rsi;
            // 42435 mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 42438 mov rax,QWORD PTR [rcx+0x38]
            rax = m.r64(rcx.wrapping_add(0x38)) as u64;
            // 4243c mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 4243e lea rdx,[rip+0xbdfab] # 0x1003f0
            rdx = m.env.data_va(84925);
            // 42445 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 42448 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 4244d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42450 mov rdx,r9
            rdx = r9;
            // 42453 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42454 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28bc0 size=94
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028bc0<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x28bc0;
    loop {
        match pc {
        0x28bc0 => {
            // block 28bc0
            #[cfg(feature = "trace")] eprintln!("f_00028bc0 pc 28bc0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28bc0 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28bc1 mov rbp,rsp
            rbp = rsp;
            // 28bc4 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28bc6 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28bc8 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28bc9 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 28bca mov r15,rsi
            r15 = rsi;
            // 28bcd mov r14,rdi
            r14 = rdi;
            // 28bd0 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 28bd3 add r15,0x8
            { let (r, f) = add::<64>(r15, 0x8u64);
              r15 = r;
              fl = f; }
            // 28bd7 jmp 0x28c0a
            pc = 0x28c0a; continue;
        }
        0x28bd9 => {
            // block 28bd9
            #[cfg(feature = "trace")] eprintln!("f_00028bc0 pc 28bd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28bd9 lea rsi,[rbx+0x1c]
            rsi = rbx.wrapping_add(0x1c);
            // 28bdd mov rdi,r14
            rdi = r14;
            // 28be0 call 0xebada
            { let r = { let t = m.env.stub_va(0x2a7); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28be5 mov rax,QWORD PTR [rbx+0x8]
            rax = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 28be9 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28bec je 0x28bfb
            if fl.e() { pc = 0x28bfb; continue; }
            pc = 0x28bee;
        }
        0x28bee => {
            // block 28bee
            #[cfg(feature = "trace")] eprintln!("f_00028bc0 pc 28bee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28bee mov rbx,rax
            rbx = rax;
            // 28bf1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 28bf4 test rax,rax
            fl = logic::<64>(rax & rax);
            // 28bf7 jne 0x28bee
            if fl.ne() { pc = 0x28bee; continue; }
            pc = 0x28bf9;
        }
        0x28bf9 => {
            // block 28bf9
            #[cfg(feature = "trace")] eprintln!("f_00028bc0 pc 28bf9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28bf9 jmp 0x28c0a
            pc = 0x28c0a; continue;
        }
        0x28bfb => {
            // block 28bfb
            #[cfg(feature = "trace")] eprintln!("f_00028bc0 pc 28bfb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28bfb mov rax,rbx
            rax = rbx;
            pc = 0x28bfe;
        }
        0x28bfe => {
            // block 28bfe
            #[cfg(feature = "trace")] eprintln!("f_00028bc0 pc 28bfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28bfe mov rbx,QWORD PTR [rax+0x10]
            rbx = m.r64(rax.wrapping_add(0x10)) as u64;
            // 28c02 cmp QWORD PTR [rbx],rax
            fl = sub::<64>(m.r64(rbx) as u64, rax).1;
            // 28c05 mov rax,rbx
            rax = rbx;
            // 28c08 jne 0x28bfe
            if fl.ne() { pc = 0x28bfe; continue; }
            pc = 0x28c0a;
        }
        0x28c0a => {
            // block 28c0a
            #[cfg(feature = "trace")] eprintln!("f_00028bc0 pc 28c0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28c0a cmp rbx,r15
            fl = sub::<64>(rbx, r15).1;
            // 28c0d jne 0x28bd9
            if fl.ne() { pc = 0x28bd9; continue; }
            pc = 0x28c0f;
        }
        0x28c0f => {
            // block 28c0f
            #[cfg(feature = "trace")] eprintln!("f_00028bc0 pc 28c0f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28c0f mov rax,r14
            rax = r14;
            // 28c12 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 28c16 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28c17 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28c19 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28c1b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28c1c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2d884 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002d884<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2d884;
    loop {
        match pc {
        0x2d884 => {
            // block 2d884
            #[cfg(feature = "trace")] eprintln!("f_0002d884 pc 2d884 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d884 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2d885 mov rbp,rsp
            rbp = rsp;
            // 2d888 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2d88a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2d88c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2d88e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2d88f sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2d893 mov r15,rsi
            r15 = rsi;
            // 2d896 mov r14,rdi
            r14 = rdi;
            // 2d899 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 2d89d mov rdx,r15
            rdx = r15;
            // 2d8a0 call 0xebab6
            { let r = { let t = m.env.stub_va(0x283); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d8a5 mov r12,rax
            r12 = rax;
            // 2d8a8 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 2d8ac test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 2d8af je 0x2d8b5
            if fl.e() { pc = 0x2d8b5; continue; }
            pc = 0x2d8b1;
        }
        0x2d8b1 => {
            // block 2d8b1
            #[cfg(feature = "trace")] eprintln!("f_0002d884 pc 2d8b1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d8b1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 2d8b3 jmp 0x2d8dc
            pc = 0x2d8dc; continue;
        }
        0x2d8b5 => {
            // block 2d8b5
            #[cfg(feature = "trace")] eprintln!("f_0002d884 pc 2d8b5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d8b5 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 2d8ba call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d8bf mov rbx,rax
            rbx = rax;
            // 2d8c2 mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 2d8c5 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 2d8c8 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 2d8cc mov rdi,r14
            rdi = r14;
            // 2d8cf mov rdx,r12
            rdx = r12;
            // 2d8d2 mov rcx,rbx
            rcx = rbx;
            // 2d8d5 call 0xebac2
            { let r = { let t = m.env.stub_va(0x28f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2d8da mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x2d8dc;
        }
        0x2d8dc => {
            // block 2d8dc
            #[cfg(feature = "trace")] eprintln!("f_0002d884 pc 2d8dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2d8dc mov rax,rbx
            rax = rbx;
            // 2d8df add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 2d8e3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2d8e4 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2d8e6 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2d8e8 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2d8ea pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2d8eb ret
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
            #[cfg(feature = "trace")] eprintln!("f_00001f80 pc 1f80 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0x42ef4 size=116
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00042ef4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x42ef4;
    loop {
        match pc {
        0x42ef4 => {
            // block 42ef4
            #[cfg(feature = "trace")] eprintln!("f_00042ef4 pc 42ef4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42ef4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 42ef5 mov rbp,rsp
            rbp = rsp;
            // 42ef8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 42efa push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 42efc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 42efd push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 42efe mov r14d,edx
            r14 = ((rdx as u32 as u64)) as u32 as u64;
            // 42f01 mov rbx,rdi
            rbx = rdi;
            // 42f04 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f07 call QWORD PTR [rax+0x278]
            { let r = { let t = m.r64(rax.wrapping_add(0x278)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42f0d xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42f10 cmp r14d,0x1ffff
            fl = sub::<32>((r14 as u32 as u64), 0x1ffffu64).1;
            // 42f17 ja 0x42f59
            if fl.a() { pc = 0x42f59; continue; }
            pc = 0x42f19;
        }
        0x42f19 => {
            // block 42f19
            #[cfg(feature = "trace")] eprintln!("f_00042ef4 pc 42f19 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42f19 xor al,0x1
            { let r = (rax as u8 as u64) ^ 0x1u64; let f = logic::<8>(r);
              rax = (rax & !0xff) | ((r) as u8 as u64);
              fl = f; }
            // 42f1b jne 0x42f59
            if fl.ne() { pc = 0x42f59; continue; }
            pc = 0x42f1d;
        }
        0x42f1d => {
            // block 42f1d
            #[cfg(feature = "trace")] eprintln!("f_00042ef4 pc 42f1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42f1d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f20 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 42f23 mov esi,0x903
            rsi = (0x903u64) as u32 as u64;
            // 42f28 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42f2d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42f30 mov rdi,rbx
            rdi = rbx;
            // 42f33 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42f36 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42f39 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 42f3b je 0x42f59
            if fl.e() { pc = 0x42f59; continue; }
            pc = 0x42f3d;
        }
        0x42f3d => {
            // block 42f3d
            #[cfg(feature = "trace")] eprintln!("f_00042ef4 pc 42f3d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42f3d mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 42f40 mov esi,0x907
            rsi = (0x907u64) as u32 as u64;
            // 42f45 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 42f4a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 42f4d mov rdi,rbx
            rdi = rbx;
            // 42f50 mov edx,r14d
            rdx = ((r14 as u32 as u64)) as u32 as u64;
            // 42f53 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 42f56 mov r15b,al
            r15 = (r15 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            pc = 0x42f59;
        }
        0x42f59 => {
            // block 42f59
            #[cfg(feature = "trace")] eprintln!("f_00042ef4 pc 42f59 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42f59 mov al,r15b
            rax = (rax & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 42f5c add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42f60 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42f61 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 42f63 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 42f65 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 42f66 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x3914 size=172
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00003914<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x3914;
    loop {
        match pc {
        0x3914 => {
            // block 3914
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 3914 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3914 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 3915 mov rbp,rsp
            rbp = rsp;
            // 3918 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 391a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 391c push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 391e push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 3920 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 3921 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3922 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 3925 mov r12,rdi
            r12 = rdi;
            // 3928 shl r14d,0xe
            { let (r, f) = shl::<32>((r14 as u32 as u64), 0xeu64, fl); r14 = (r) as u32 as u64; fl = f; }
            // 392c and r14d,0x3fffc000
            { let r = (r14 as u32 as u64) & 0x3fffc000u64; let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 3933 je 0x39ab
            if fl.e() { pc = 0x39ab; continue; }
            pc = 0x3935;
        }
        0x3935 => {
            // block 3935
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 3935 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3935 xor r15d,r15d
            { let r = (r15 as u32 as u64) ^ (r15 as u32 as u64); let f = logic::<32>(r);
              r15 = (r) as u32 as u64;
              fl = f; }
            // 3938 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 393a xor r13d,r13d
            { let r = (r13 as u32 as u64) ^ (r13 as u32 as u64); let f = logic::<32>(r);
              r13 = (r) as u32 as u64;
              fl = f; }
            pc = 0x393d;
        }
        0x393d => {
            // block 393d
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 393d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 393d mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3941 mov esi,0x3b
            rsi = (0x3bu64) as u32 as u64;
            // 3946 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 394b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 394e mov rdi,r12
            rdi = r12;
            // 3951 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 3953 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3956 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 395a mov esi,0x3a
            rsi = (0x3au64) as u32 as u64;
            // 395f mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 3964 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 3969 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 396c mov rdi,r12
            rdi = r12;
            // 396f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3972 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3976 mov rdi,r12
            rdi = r12;
            // 3979 call QWORD PTR [rax+0x9e0]
            { let r = { let t = m.r64(rax.wrapping_add(0x9e0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 397f mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 3983 mov esi,0x3d
            rsi = (0x3du64) as u32 as u64;
            // 3988 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 398d xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 3990 mov rdi,r12
            rdi = r12;
            // 3993 lea rdx,[rbp-0x2c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 3997 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 399a cmp DWORD PTR [rbp-0x2c],0xffffffff
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffd4)) as u64, 0xffffffffu64).1;
            // 399e jne 0x39ae
            if fl.ne() { pc = 0x39ae; continue; }
            pc = 0x39a0;
        }
        0x39a0 => {
            // block 39a0
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39a0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39a0 inc r15d
            { let (r, f) = inc::<32>((r15 as u32 as u64), fl); r15 = (r) as u32 as u64; fl = f; }
            // 39a3 add ebx,0x4
            { let (r, f) = add::<32>((rbx as u32 as u64), 0x4u64);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 39a6 cmp r15d,r14d
            fl = sub::<32>((r15 as u32 as u64), (r14 as u32 as u64)).1;
            // 39a9 jb 0x393d
            if fl.b() { pc = 0x393d; continue; }
            pc = 0x39ab;
        }
        0x39ab => {
            // block 39ab
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39ab mov r13b,0x1
            r13 = (r13 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x39ae;
        }
        0x39ae => {
            // block 39ae
            #[cfg(feature = "trace")] eprintln!("f_00003914 pc 39ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 39ae mov al,r13b
            rax = (rax & !0xff) | (((r13 as u8 as u64)) as u8 as u64);
            // 39b1 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 39b5 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 39b6 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 39b8 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 39ba pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 39bc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 39be pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 39bf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x18098 size=2576
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00018098<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x18098;
    loop {
        match pc {
        0x18098 => {
            // block 18098
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18098 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18098 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 18099 mov rbp,rsp
            rbp = rsp;
            // 1809c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 1809e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 180a0 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 180a2 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 180a4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 180a5 sub rsp,0x148
            { let (r, f) = sub::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 180ac mov QWORD PTR [rbp-0x140],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec0), (rcx) as u64);
            // 180b3 mov r15,rdx
            r15 = rdx;
            // 180b6 mov QWORD PTR [rbp-0x130],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed0), (r15) as u64);
            // 180bd mov QWORD PTR [rbp-0x160],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea0), (rsi) as u64);
            // 180c4 mov QWORD PTR [rbp-0x138],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fec8), (rdi) as u64);
            // 180cb mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 180ce mov QWORD PTR [rbp-0x128],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rax) as u64);
            // 180d5 mov r13,QWORD PTR [rsi]
            r13 = m.r64(rsi) as u64;
            // 180d8 mov r14,QWORD PTR [r15]
            r14 = m.r64(r15) as u64;
            // 180db jmp 0x18127
            pc = 0x18127; continue;
        }
        0x180dd => {
            // block 180dd
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 180dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 180dd lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 180e4 mov QWORD PTR [r15+0x8],rax
            m.w64(r15.wrapping_add(0x8), (rax) as u64);
            // 180e8 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 180ec call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 180f1 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 180f5 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 180fa lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 180fe call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18103 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 18107 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1810c lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 18113 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18118 add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 1811f mov rdi,rbx
            rdi = rbx;
            // 18122 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18127;
        }
        0x18127 => {
            // block 18127
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18127 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18127 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1812b cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 1812e jne 0x180dd
            if fl.ne() { pc = 0x180dd; continue; }
            pc = 0x18130;
        }
        0x18130 => {
            // block 18130
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18130 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18130 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 18137 mov r14,QWORD PTR [rdx]
            r14 = m.r64(rdx) as u64;
            // 1813a jmp 0x1818c
            pc = 0x1818c; continue;
        }
        0x1813c => {
            // block 1813c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1813c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1813c lea rax,[rbx-0xf8]
            rax = rbx.wrapping_add(0xffff_ffff_ffff_ff08);
            // 18143 mov QWORD PTR [rdx+0x8],rax
            m.w64(rdx.wrapping_add(0x8), (rax) as u64);
            // 18147 lea rdi,[rbx-0x38]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffc8);
            // 1814b mov r12,rdx
            r12 = rdx;
            // 1814e call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18153 lea rdi,[rbx-0x50]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 18157 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1815c lea rdi,[rbx-0x68]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff98);
            // 18160 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18165 lea rdi,[rbx-0x80]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff80);
            // 18169 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1816e lea rdi,[rbx-0x98]
            rdi = rbx.wrapping_add(0xffff_ffff_ffff_ff68);
            // 18175 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1817a add rbx,0xffffffffffffff20
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_ff20u64);
              rbx = r;
              fl = f; }
            // 18181 mov rdi,rbx
            rdi = rbx;
            // 18184 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18189 mov rdx,r12
            rdx = r12;
            pc = 0x1818c;
        }
        0x1818c => {
            // block 1818c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1818c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1818c mov rbx,QWORD PTR [rdx+0x8]
            rbx = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 18190 cmp rbx,r14
            fl = sub::<64>(rbx, r14).1;
            // 18193 jne 0x1813c
            if fl.ne() { pc = 0x1813c; continue; }
            pc = 0x18195;
        }
        0x18195 => {
            // block 18195
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18195 mov r14,QWORD PTR [rbp-0x160]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 1819c mov rbx,QWORD PTR [rbp-0x128]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 181a3 mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 181aa jmp 0x181b3
            pc = 0x181b3; continue;
        }
        0x181ac => {
            // block 181ac
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181ac add rbx,0xf8
            { let (r, f) = add::<64>(rbx, 0xf8u64);
              rbx = r;
              fl = f; }
            pc = 0x181b3;
        }
        0x181b3 => {
            // block 181b3
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181b3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181b3 mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // 181ba cmp r13,QWORD PTR [r14+0x8]
            fl = sub::<64>(r13, m.r64(r14.wrapping_add(0x8)) as u64).1;
            // 181be sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 181c1 cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 181c6 jne 0x1835d
            if fl.ne() { pc = 0x1835d; continue; }
            pc = 0x181cc;
        }
        0x181cc => {
            // block 181cc
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181cc add r13,0x90
            { let (r, f) = add::<64>(r13, 0x90u64);
              r13 = r;
              fl = f; }
            pc = 0x181d3;
        }
        0x181d3 => {
            // block 181d3
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181d3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181d3 test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 181d5 jne 0x18900
            if fl.ne() { pc = 0x18900; continue; }
            pc = 0x181db;
        }
        0x181db => {
            // block 181db
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181db mov eax,DWORD PTR [r13-0x90]
            rax = (m.r32(r13.wrapping_add(0xffff_ffff_ffff_ff70)) as u64) as u32 as u64;
            // 181e2 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 181e4 cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 181e7 jb 0x18339
            if fl.b() { pc = 0x18339; continue; }
            pc = 0x181ed;
        }
        0x181ed => {
            // block 181ed
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181ed mov rbx,r15
            rbx = r15;
            // 181f0 mov r15,QWORD PTR [rbx+0x8]
            r15 = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 181f4 cmp r15,QWORD PTR [rbx+0x10]
            fl = sub::<64>(r15, m.r64(rbx.wrapping_add(0x10)) as u64).1;
            // 181f8 je 0x1831a
            if fl.e() { pc = 0x1831a; continue; }
            pc = 0x181fe;
        }
        0x181fe => {
            // block 181fe
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 181fe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 181fe mov rax,QWORD PTR [r13-0x80]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 18202 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 18206 mov rax,QWORD PTR [r13-0x90]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ff70)) as u64;
            // 1820d mov rcx,QWORD PTR [r13-0x88]
            rcx = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            // 18214 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 18218 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 1821b lea rbx,[r15+0x18]
            rbx = r15.wrapping_add(0x18);
            // 1821f lea rsi,[r13-0x78]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18223 mov rdi,rbx
            rdi = rbx;
            // 18226 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1822b mov ax,WORD PTR [r13-0x34]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(0xffff_ffff_ffff_ffcc)) as u64) as u16 as u64);
            // 18230 mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 18235 mov eax,DWORD PTR [r13-0x38]
            rax = (m.r32(r13.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64) as u32 as u64;
            // 18239 mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1823d mov rax,QWORD PTR [r13-0x40]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffc0)) as u64;
            // 18241 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 18245 mov rax,QWORD PTR [r13-0x48]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 18249 mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 1824d mov rax,QWORD PTR [r13-0x50]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 18251 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 18255 mov rax,QWORD PTR [r13-0x60]
            rax = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffa0)) as u64;
            // 18259 mov rcx,QWORD PTR [r13-0x58]
            rcx = m.r64(r13.wrapping_add(0xffff_ffff_ffff_ffa8)) as u64;
            // 1825d mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 18261 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 18265 lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 18269 mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 18270 lea rsi,[r13-0x30]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ffd0);
            // 18274 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18279 mov r12,r14
            r12 = r14;
            // 1827c lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 18280 mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 18287 lea rsi,[r13-0x18]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ffe8);
            // 1828b call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18290 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 18297 lea r14,[r15+0x90]
            r14 = r15.wrapping_add(0x90);
            // 1829e mov rdi,r14
            rdi = r14;
            // 182a1 mov rsi,r13
            rsi = r13;
            // 182a4 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182a9 lea rbx,[r15+0xa8]
            rbx = r15.wrapping_add(0xa8);
            // 182b0 lea rsi,[r13+0x18]
            rsi = r13.wrapping_add(0x18);
            // 182b4 mov rdi,rbx
            rdi = rbx;
            // 182b7 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182bc lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 182c3 lea rsi,[r13+0x30]
            rsi = r13.wrapping_add(0x30);
            // 182c7 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 182cc mov eax,DWORD PTR [r13+0x60]
            rax = (m.r32(r13.wrapping_add(0x60)) as u64) as u32 as u64;
            // 182d0 mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 182d7 mov rax,QWORD PTR [r13+0x58]
            rax = m.r64(r13.wrapping_add(0x58)) as u64;
            // 182db mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 182e2 mov rax,QWORD PTR [r13+0x48]
            rax = m.r64(r13.wrapping_add(0x48)) as u64;
            // 182e6 mov rcx,QWORD PTR [r13+0x50]
            rcx = m.r64(r13.wrapping_add(0x50)) as u64;
            // 182ea mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 182f1 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 182f8 mov r15,QWORD PTR [rbp-0x130]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 182ff add QWORD PTR [r15+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 18307 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 1830e mov r14,r12
            r14 = r12;
            // 18311 mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 18318 jmp 0x18332
            pc = 0x18332; continue;
        }
        0x1831a => {
            // block 1831a
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1831a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1831a lea rsi,[r13-0x90]
            rsi = r13.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18321 mov rdi,rbx
            rdi = rbx;
            // 18324 mov r15,rdx
            r15 = rdx;
            // 18327 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1832c mov rdx,r15
            rdx = r15;
            // 1832f mov r15,rbx
            r15 = rbx;
            pc = 0x18332;
        }
        0x18332 => {
            // block 18332
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18332 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18332 mov rbx,QWORD PTR [rbp-0x128]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            pc = 0x18339;
        }
        0x18339 => {
            // block 18339
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18339 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18339 lea rax,[r13+0x68]
            rax = r13.wrapping_add(0x68);
            // 1833d cmp rax,QWORD PTR [r14+0x8]
            fl = sub::<64>(rax, m.r64(r14.wrapping_add(0x8)) as u64).1;
            // 18341 sete al
            rax = (rax & !0xff) | (((fl.e() as u64)) as u8 as u64);
            // 18344 add r13,0xf8
            { let (r, f) = add::<64>(r13, 0xf8u64);
              r13 = r;
              fl = f; }
            // 1834b cmp rbx,QWORD PTR [r12+0x8]
            fl = sub::<64>(rbx, m.r64(r12.wrapping_add(0x8)) as u64).1;
            // 18350 je 0x181d3
            if fl.e() { pc = 0x181d3; continue; }
            pc = 0x18356;
        }
        0x18356 => {
            // block 18356
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18356 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18356 add r13,0xffffffffffffff70
            { let (r, f) = add::<64>(r13, 0xffff_ffff_ffff_ff70u64);
              r13 = r;
              fl = f; }
            pc = 0x1835d;
        }
        0x1835d => {
            // block 1835d
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1835d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1835d test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 1835f je 0x1848d
            if fl.e() { pc = 0x1848d; continue; }
            pc = 0x18365;
        }
        0x18365 => {
            // block 18365
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18365 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18365 mov r15,QWORD PTR [rdx+0x8]
            r15 = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 18369 cmp r15,QWORD PTR [rdx+0x10]
            fl = sub::<64>(r15, m.r64(rdx.wrapping_add(0x10)) as u64).1;
            // 1836d je 0x186fa
            if fl.e() { pc = 0x186fa; continue; }
            pc = 0x18373;
        }
        0x18373 => {
            // block 18373
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18373 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18373 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 18377 mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 1837b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1837e mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 18382 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 18386 mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 18389 lea rdi,[r15+0x18]
            rdi = r15.wrapping_add(0x18);
            // 1838d mov QWORD PTR [rbp-0x148],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rdi) as u64);
            // 18394 lea rsi,[rbx+0x18]
            rsi = rbx.wrapping_add(0x18);
            // 18398 call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1839d mov ax,WORD PTR [rbx+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 183a1 mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 183a6 mov eax,DWORD PTR [rbx+0x58]
            rax = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 183a9 mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 183ad mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 183b1 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 183b5 mov rax,QWORD PTR [rbx+0x48]
            rax = m.r64(rbx.wrapping_add(0x48)) as u64;
            // 183b9 mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 183bd mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 183c1 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 183c5 mov rax,QWORD PTR [rbx+0x30]
            rax = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 183c9 mov rcx,QWORD PTR [rbx+0x38]
            rcx = m.r64(rbx.wrapping_add(0x38)) as u64;
            // 183cd mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 183d1 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 183d5 lea r14,[r15+0x60]
            r14 = r15.wrapping_add(0x60);
            // 183d9 lea rsi,[rbx+0x60]
            rsi = rbx.wrapping_add(0x60);
            // 183dd mov rdi,r14
            rdi = r14;
            // 183e0 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 183e5 mov QWORD PTR [rbp-0x150],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (r14) as u64);
            // 183ec lea r14,[r15+0x78]
            r14 = r15.wrapping_add(0x78);
            // 183f0 lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 183f4 mov rdi,r14
            rdi = r14;
            // 183f7 call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 183fc mov QWORD PTR [rbp-0x158],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (r14) as u64);
            // 18403 lea r14,[r15+0x90]
            r14 = r15.wrapping_add(0x90);
            // 1840a lea rsi,[rbx+0x90]
            rsi = rbx.wrapping_add(0x90);
            // 18411 mov rdi,r14
            rdi = r14;
            // 18414 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18419 lea r12,[r15+0xa8]
            r12 = r15.wrapping_add(0xa8);
            // 18420 lea rsi,[rbx+0xa8]
            rsi = rbx.wrapping_add(0xa8);
            // 18427 mov rdi,r12
            rdi = r12;
            // 1842a call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1842f lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 18436 lea rsi,[rbx+0xc0]
            rsi = rbx.wrapping_add(0xc0);
            // 1843d call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18442 mov eax,DWORD PTR [rbx+0xf0]
            rax = (m.r32(rbx.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 18448 mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 1844f mov rax,QWORD PTR [rbx+0xe8]
            rax = m.r64(rbx.wrapping_add(0xe8)) as u64;
            // 18456 mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 1845d mov rax,QWORD PTR [rbx+0xd8]
            rax = m.r64(rbx.wrapping_add(0xd8)) as u64;
            // 18464 mov rcx,QWORD PTR [rbx+0xe0]
            rcx = m.r64(rbx.wrapping_add(0xe0)) as u64;
            // 1846b mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 18472 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 18479 mov rdx,QWORD PTR [rbp-0x140]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec0)) as u64;
            // 18480 add QWORD PTR [rdx+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(rdx.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(rdx.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 18488 jmp 0x188e6
            pc = 0x188e6; continue;
        }
        0x1848d => {
            // block 1848d
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1848d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1848d mov r14,rdx
            r14 = rdx;
            // 18490 mov rax,QWORD PTR [r13+0x10]
            rax = m.r64(r13.wrapping_add(0x10)) as u64;
            // 18494 mov QWORD PTR [rbp-0x110],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fef0), (rax) as u64);
            // 1849b mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 1849f mov rcx,QWORD PTR [r13+0x8]
            rcx = m.r64(r13.wrapping_add(0x8)) as u64;
            // 184a3 mov QWORD PTR [rbp-0x118],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee8), (rcx) as u64);
            // 184aa mov QWORD PTR [rbp-0x120],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fee0), (rax) as u64);
            // 184b1 lea rsi,[r13+0x18]
            rsi = r13.wrapping_add(0x18);
            // 184b5 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 184bc call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 184c1 mov ax,WORD PTR [r13+0x5c]
            rax = (rax & !0xffff) | ((m.r16(r13.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 184c6 lea rcx,[rbp-0xf0]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 184cd mov rdx,rcx
            rdx = rcx;
            // 184d0 mov WORD PTR [rdx+0x2c],ax
            m.w16(rdx.wrapping_add(0x2c), ((rax as u16 as u64)) as u16);
            // 184d4 mov eax,DWORD PTR [r13+0x58]
            rax = (m.r32(r13.wrapping_add(0x58)) as u64) as u32 as u64;
            // 184d8 mov DWORD PTR [rdx+0x28],eax
            m.w32(rdx.wrapping_add(0x28), ((rax as u32 as u64)) as u32);
            // 184db mov rax,QWORD PTR [r13+0x50]
            rax = m.r64(r13.wrapping_add(0x50)) as u64;
            // 184df mov QWORD PTR [rdx+0x20],rax
            m.w64(rdx.wrapping_add(0x20), (rax) as u64);
            // 184e3 mov rax,QWORD PTR [r13+0x48]
            rax = m.r64(r13.wrapping_add(0x48)) as u64;
            // 184e7 mov QWORD PTR [rdx+0x18],rax
            m.w64(rdx.wrapping_add(0x18), (rax) as u64);
            // 184eb mov rax,QWORD PTR [r13+0x40]
            rax = m.r64(r13.wrapping_add(0x40)) as u64;
            // 184ef mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 184f3 mov rax,QWORD PTR [r13+0x30]
            rax = m.r64(r13.wrapping_add(0x30)) as u64;
            // 184f7 mov rcx,QWORD PTR [r13+0x38]
            rcx = m.r64(r13.wrapping_add(0x38)) as u64;
            // 184fb mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 184ff mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 18502 lea rsi,[r13+0x60]
            rsi = r13.wrapping_add(0x60);
            // 18506 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 1850d call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18512 lea rsi,[r13+0x78]
            rsi = r13.wrapping_add(0x78);
            // 18516 lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 1851d call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18522 lea rsi,[r13+0x90]
            rsi = r13.wrapping_add(0x90);
            // 18529 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18530 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18535 lea rsi,[r13+0xa8]
            rsi = r13.wrapping_add(0xa8);
            // 1853c lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18540 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18545 lea rsi,[r13+0xc0]
            rsi = r13.wrapping_add(0xc0);
            // 1854c lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 18550 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18555 mov eax,DWORD PTR [r13+0xf0]
            rax = (m.r32(r13.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 1855c lea rcx,[rbp-0x48]
            rcx = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18560 mov rdx,rcx
            rdx = rcx;
            // 18563 mov DWORD PTR [rdx+0x18],eax
            m.w32(rdx.wrapping_add(0x18), ((rax as u32 as u64)) as u32);
            // 18566 mov rax,QWORD PTR [r13+0xe8]
            rax = m.r64(r13.wrapping_add(0xe8)) as u64;
            // 1856d mov QWORD PTR [rdx+0x10],rax
            m.w64(rdx.wrapping_add(0x10), (rax) as u64);
            // 18571 mov rax,QWORD PTR [r13+0xd8]
            rax = m.r64(r13.wrapping_add(0xd8)) as u64;
            // 18578 mov rcx,QWORD PTR [r13+0xe0]
            rcx = m.r64(r13.wrapping_add(0xe0)) as u64;
            // 1857f mov QWORD PTR [rdx+0x8],rcx
            m.w64(rdx.wrapping_add(0x8), (rcx) as u64);
            // 18583 mov QWORD PTR [rdx],rax
            m.w64(rdx, (rax) as u64);
            // 18586 mov eax,DWORD PTR [rbx]
            rax = (m.r32(rbx) as u64) as u32 as u64;
            // 18588 mov ecx,DWORD PTR [rbx+0x4]
            rcx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // 1858b cmp eax,DWORD PTR [rbp-0x120]
            fl = sub::<32>((rax as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64).1;
            // 18591 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 18594 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 18597 lea rdx,[rax+0x1]
            rdx = rax.wrapping_add(0x1);
            // 1859b cmp ecx,DWORD PTR [rbp-0x11c]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee4)) as u64).1;
            // 185a1 cmove rdx,rax
            if fl.e() { rdx = rax; }
            // 185a5 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 185a9 cmp rax,QWORD PTR [rbp-0x110]
            fl = sub::<64>(rax, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64).1;
            // 185b0 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            // 185b3 movzx eax,al
            rax = ((rax as u8 as u64)) as u32 as u64;
            // 185b6 add rax,rdx
            { let (r, f) = add::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 185b9 mov ecx,DWORD PTR [rbx+0x8]
            rcx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // 185bc cmp ecx,DWORD PTR [rbp-0x118]
            fl = sub::<32>((rcx as u32 as u64), m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64).1;
            // 185c2 mov ecx,0x0
            rcx = (0x0u64) as u32 as u64;
            // 185c7 mov rdx,0xffffffffffffffff
            rdx = 0xffff_ffff_ffff_ffffu64;
            // 185ce cmovne rcx,rdx
            if fl.ne() { rcx = rdx; }
            // 185d2 cmp rax,rcx
            fl = sub::<64>(rax, rcx).1;
            // 185d5 je 0x1889a
            if fl.e() { pc = 0x1889a; continue; }
            pc = 0x185db;
        }
        0x185db => {
            // block 185db
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 185db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 185db mov r15,QWORD PTR [r14+0x8]
            r15 = m.r64(r14.wrapping_add(0x8)) as u64;
            // 185df cmp r15,QWORD PTR [r14+0x10]
            fl = sub::<64>(r15, m.r64(r14.wrapping_add(0x10)) as u64).1;
            // 185e3 je 0x18717
            if fl.e() { pc = 0x18717; continue; }
            pc = 0x185e9;
        }
        0x185e9 => {
            // block 185e9
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 185e9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 185e9 mov rax,QWORD PTR [rbx+0x10]
            rax = m.r64(rbx.wrapping_add(0x10)) as u64;
            // 185ed mov QWORD PTR [r15+0x10],rax
            m.w64(r15.wrapping_add(0x10), (rax) as u64);
            // 185f1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 185f4 mov rcx,QWORD PTR [rbx+0x8]
            rcx = m.r64(rbx.wrapping_add(0x8)) as u64;
            // 185f8 mov QWORD PTR [r15+0x8],rcx
            m.w64(r15.wrapping_add(0x8), (rcx) as u64);
            // 185fc mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 185ff lea r12,[r15+0x18]
            r12 = r15.wrapping_add(0x18);
            // 18603 lea rsi,[rbx+0x18]
            rsi = rbx.wrapping_add(0x18);
            // 18607 mov rdi,r12
            rdi = r12;
            // 1860a call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1860f mov ax,WORD PTR [rbx+0x5c]
            rax = (rax & !0xffff) | ((m.r16(rbx.wrapping_add(0x5c)) as u64) as u16 as u64);
            // 18613 mov WORD PTR [r15+0x5c],ax
            m.w16(r15.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 18618 mov eax,DWORD PTR [rbx+0x58]
            rax = (m.r32(rbx.wrapping_add(0x58)) as u64) as u32 as u64;
            // 1861b mov DWORD PTR [r15+0x58],eax
            m.w32(r15.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1861f mov rax,QWORD PTR [rbx+0x50]
            rax = m.r64(rbx.wrapping_add(0x50)) as u64;
            // 18623 mov QWORD PTR [r15+0x50],rax
            m.w64(r15.wrapping_add(0x50), (rax) as u64);
            // 18627 mov rax,QWORD PTR [rbx+0x48]
            rax = m.r64(rbx.wrapping_add(0x48)) as u64;
            // 1862b mov QWORD PTR [r15+0x48],rax
            m.w64(r15.wrapping_add(0x48), (rax) as u64);
            // 1862f mov rax,QWORD PTR [rbx+0x40]
            rax = m.r64(rbx.wrapping_add(0x40)) as u64;
            // 18633 mov QWORD PTR [r15+0x40],rax
            m.w64(r15.wrapping_add(0x40), (rax) as u64);
            // 18637 mov rax,QWORD PTR [rbx+0x30]
            rax = m.r64(rbx.wrapping_add(0x30)) as u64;
            // 1863b mov rcx,QWORD PTR [rbx+0x38]
            rcx = m.r64(rbx.wrapping_add(0x38)) as u64;
            // 1863f mov QWORD PTR [r15+0x38],rcx
            m.w64(r15.wrapping_add(0x38), (rcx) as u64);
            // 18643 mov QWORD PTR [r15+0x30],rax
            m.w64(r15.wrapping_add(0x30), (rax) as u64);
            // 18647 lea rdi,[r15+0x60]
            rdi = r15.wrapping_add(0x60);
            // 1864b mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 18652 lea rsi,[rbx+0x60]
            rsi = rbx.wrapping_add(0x60);
            // 18656 call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1865b lea rdi,[r15+0x78]
            rdi = r15.wrapping_add(0x78);
            // 1865f mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 18666 lea rsi,[rbx+0x78]
            rsi = rbx.wrapping_add(0x78);
            // 1866a call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1866f mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 18676 lea rdi,[r15+0x90]
            rdi = r15.wrapping_add(0x90);
            // 1867d mov QWORD PTR [rbp-0x168],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fe98), (rdi) as u64);
            // 18684 lea rsi,[rbx+0x90]
            rsi = rbx.wrapping_add(0x90);
            // 1868b call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18690 lea r12,[r15+0xa8]
            r12 = r15.wrapping_add(0xa8);
            // 18697 lea rsi,[rbx+0xa8]
            rsi = rbx.wrapping_add(0xa8);
            // 1869e mov rdi,r12
            rdi = r12;
            // 186a1 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 186a6 lea rdi,[r15+0xc0]
            rdi = r15.wrapping_add(0xc0);
            // 186ad lea rsi,[rbx+0xc0]
            rsi = rbx.wrapping_add(0xc0);
            // 186b4 call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 186b9 mov eax,DWORD PTR [rbx+0xf0]
            rax = (m.r32(rbx.wrapping_add(0xf0)) as u64) as u32 as u64;
            // 186bf mov DWORD PTR [r15+0xf0],eax
            m.w32(r15.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 186c6 mov rax,QWORD PTR [rbx+0xe8]
            rax = m.r64(rbx.wrapping_add(0xe8)) as u64;
            // 186cd mov QWORD PTR [r15+0xe8],rax
            m.w64(r15.wrapping_add(0xe8), (rax) as u64);
            // 186d4 mov rax,QWORD PTR [rbx+0xd8]
            rax = m.r64(rbx.wrapping_add(0xd8)) as u64;
            // 186db mov rcx,QWORD PTR [rbx+0xe0]
            rcx = m.r64(rbx.wrapping_add(0xe0)) as u64;
            // 186e2 mov QWORD PTR [r15+0xe0],rcx
            m.w64(r15.wrapping_add(0xe0), (rcx) as u64);
            // 186e9 mov QWORD PTR [r15+0xd8],rax
            m.w64(r15.wrapping_add(0xd8), (rax) as u64);
            // 186f0 add QWORD PTR [r14+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(r14.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(r14.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 186f8 jmp 0x18722
            pc = 0x18722; continue;
        }
        0x186fa => {
            // block 186fa
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 186fa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 186fa mov rdi,rdx
            rdi = rdx;
            // 186fd mov rsi,rbx
            rsi = rbx;
            // 18700 mov r15,rdx
            r15 = rdx;
            // 18703 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18708 mov rdx,r15
            rdx = r15;
            // 1870b mov r15,QWORD PTR [rbp-0x130]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 18712 jmp 0x181ac
            pc = 0x181ac; continue;
        }
        0x18717 => {
            // block 18717
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18717 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18717 mov rdi,r14
            rdi = r14;
            // 1871a mov rsi,rbx
            rsi = rbx;
            // 1871d call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18722;
        }
        0x18722 => {
            // block 18722
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18722 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18722 mov eax,DWORD PTR [rbp-0x120]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64) as u32 as u64;
            // 18728 inc eax
            { let (r, f) = inc::<32>((rax as u32 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 1872a cmp eax,0x2
            fl = sub::<32>((rax as u32 as u64), 0x2u64).1;
            // 1872d jb 0x1889a
            if fl.b() { pc = 0x1889a; continue; }
            pc = 0x18733;
        }
        0x18733 => {
            // block 18733
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18733 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18733 mov rdi,QWORD PTR [rbp-0x130]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1873a mov r12,QWORD PTR [rdi+0x8]
            r12 = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 1873e cmp r12,QWORD PTR [rdi+0x10]
            fl = sub::<64>(r12, m.r64(rdi.wrapping_add(0x10)) as u64).1;
            // 18742 je 0x1888e
            if fl.e() { pc = 0x1888e; continue; }
            pc = 0x18748;
        }
        0x18748 => {
            // block 18748
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18748 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18748 mov rax,QWORD PTR [rbp-0x110]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fef0)) as u64;
            // 1874f mov QWORD PTR [r12+0x10],rax
            m.w64(r12.wrapping_add(0x10), (rax) as u64);
            // 18754 mov rax,QWORD PTR [rbp-0x120]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee0)) as u64;
            // 1875b mov rcx,QWORD PTR [rbp-0x118]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fee8)) as u64;
            // 18762 mov QWORD PTR [r12+0x8],rcx
            m.w64(r12.wrapping_add(0x8), (rcx) as u64);
            // 18767 mov QWORD PTR [r12],rax
            m.w64(r12, (rax) as u64);
            // 1876b lea r15,[r12+0x18]
            r15 = r12.wrapping_add(0x18);
            // 18770 mov rdi,r15
            rdi = r15;
            // 18773 lea rsi,[rbp-0x108]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 1877a call 0xec026
            { let r = { let t = m.env.stub_va(0x7f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1877f lea rax,[rbp-0xf0]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ff10);
            // 18786 mov rcx,rax
            rcx = rax;
            // 18789 mov ax,WORD PTR [rcx+0x2c]
            rax = (rax & !0xffff) | ((m.r16(rcx.wrapping_add(0x2c)) as u64) as u16 as u64);
            // 1878d mov WORD PTR [r12+0x5c],ax
            m.w16(r12.wrapping_add(0x5c), ((rax as u16 as u64)) as u16);
            // 18793 mov eax,DWORD PTR [rcx+0x28]
            rax = (m.r32(rcx.wrapping_add(0x28)) as u64) as u32 as u64;
            // 18796 mov DWORD PTR [r12+0x58],eax
            m.w32(r12.wrapping_add(0x58), ((rax as u32 as u64)) as u32);
            // 1879b mov rax,QWORD PTR [rcx+0x20]
            rax = m.r64(rcx.wrapping_add(0x20)) as u64;
            // 1879f mov QWORD PTR [r12+0x50],rax
            m.w64(r12.wrapping_add(0x50), (rax) as u64);
            // 187a4 mov rax,QWORD PTR [rcx+0x18]
            rax = m.r64(rcx.wrapping_add(0x18)) as u64;
            // 187a8 mov QWORD PTR [r12+0x48],rax
            m.w64(r12.wrapping_add(0x48), (rax) as u64);
            // 187ad mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 187b1 mov QWORD PTR [r12+0x40],rax
            m.w64(r12.wrapping_add(0x40), (rax) as u64);
            // 187b6 mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 187b9 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 187bd mov QWORD PTR [r12+0x38],rcx
            m.w64(r12.wrapping_add(0x38), (rcx) as u64);
            // 187c2 mov QWORD PTR [r12+0x30],rax
            m.w64(r12.wrapping_add(0x30), (rax) as u64);
            // 187c7 lea rdi,[r12+0x60]
            rdi = r12.wrapping_add(0x60);
            // 187cc mov QWORD PTR [rbp-0x150],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb0), (rdi) as u64);
            // 187d3 lea rsi,[rbp-0xc0]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 187da call 0xeba26
            { let r = { let t = m.env.stub_va(0x1f3); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 187df mov QWORD PTR [rbp-0x128],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fed8), (rbx) as u64);
            // 187e6 lea rdi,[r12+0x78]
            rdi = r12.wrapping_add(0x78);
            // 187eb mov QWORD PTR [rbp-0x158],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_fea8), (rdi) as u64);
            // 187f2 lea rsi,[rbp-0xa8]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 187f9 call 0xeba6e
            { let r = { let t = m.env.stub_va(0x23b); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 187fe mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18805 lea rbx,[r12+0x90]
            rbx = r12.wrapping_add(0x90);
            // 1880d mov rdi,rbx
            rdi = rbx;
            // 18810 lea rsi,[rbp-0x90]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18817 call 0xeba4a
            { let r = { let t = m.env.stub_va(0x217); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1881c lea r15,[r12+0xa8]
            r15 = r12.wrapping_add(0xa8);
            // 18824 mov rdi,r15
            rdi = r15;
            // 18827 lea rsi,[rbp-0x78]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 1882b call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18830 lea rdi,[r12+0xc0]
            rdi = r12.wrapping_add(0xc0);
            // 18838 lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 1883c call 0xeba02
            { let r = { let t = m.env.stub_va(0x1cf); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18841 lea rax,[rbp-0x48]
            rax = rbp.wrapping_add(0xffff_ffff_ffff_ffb8);
            // 18845 mov rcx,rax
            rcx = rax;
            // 18848 mov eax,DWORD PTR [rcx+0x18]
            rax = (m.r32(rcx.wrapping_add(0x18)) as u64) as u32 as u64;
            // 1884b mov DWORD PTR [r12+0xf0],eax
            m.w32(r12.wrapping_add(0xf0), ((rax as u32 as u64)) as u32);
            // 18853 mov rax,QWORD PTR [rcx+0x10]
            rax = m.r64(rcx.wrapping_add(0x10)) as u64;
            // 18857 mov QWORD PTR [r12+0xe8],rax
            m.w64(r12.wrapping_add(0xe8), (rax) as u64);
            // 1885f mov rax,QWORD PTR [rcx]
            rax = m.r64(rcx) as u64;
            // 18862 mov rcx,QWORD PTR [rcx+0x8]
            rcx = m.r64(rcx.wrapping_add(0x8)) as u64;
            // 18866 mov QWORD PTR [r12+0xe0],rcx
            m.w64(r12.wrapping_add(0xe0), (rcx) as u64);
            // 1886e mov QWORD PTR [r12+0xd8],rax
            m.w64(r12.wrapping_add(0xd8), (rax) as u64);
            // 18876 mov rax,QWORD PTR [rbp-0x130]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 1887d add QWORD PTR [rax+0x8],0xf8
            { let (r, f) = add::<64>(m.r64(rax.wrapping_add(0x8)) as u64, 0xf8u64);
              m.w64(rax.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 18885 mov rbx,QWORD PTR [rbp-0x128]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed8)) as u64;
            // 1888c jmp 0x1889a
            pc = 0x1889a; continue;
        }
        0x1888e => {
            // block 1888e
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1888e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1888e lea rsi,[rbp-0x120]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_fee0);
            // 18895 call 0xeb9d8
            { let r = { let t = m.env.stub_va(0x1a5); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1889a;
        }
        0x1889a => {
            // block 1889a
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1889a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1889a add r13,0xf8
            { let (r, f) = add::<64>(r13, 0xf8u64);
              r13 = r;
              fl = f; }
            // 188a1 lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 188a5 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188aa lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 188ae call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188b3 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 188ba call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188bf lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 188c6 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188cb lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 188d2 call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188d7 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            // 188de call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 188e3 mov rdx,r14
            rdx = r14;
            pc = 0x188e6;
        }
        0x188e6 => {
            // block 188e6
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 188e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 188e6 mov r15,QWORD PTR [rbp-0x130]
            r15 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fed0)) as u64;
            // 188ed mov r14,QWORD PTR [rbp-0x160]
            r14 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea0)) as u64;
            // 188f4 mov r12,QWORD PTR [rbp-0x138]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fec8)) as u64;
            // 188fb jmp 0x181ac
            pc = 0x181ac; continue;
        }
        0x18900 => {
            // block 18900
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18900 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18900 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 18903 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 18905 cmp rcx,QWORD PTR [r15+0x8]
            fl = sub::<64>(rcx, m.r64(r15.wrapping_add(0x8)) as u64).1;
            // 18909 jne 0x18915
            if fl.ne() { pc = 0x18915; continue; }
            pc = 0x1890b;
        }
        0x1890b => {
            // block 1890b
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1890b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1890b mov rax,QWORD PTR [rdx]
            rax = m.r64(rdx) as u64;
            // 1890e cmp rax,QWORD PTR [rdx+0x8]
            fl = sub::<64>(rax, m.r64(rdx.wrapping_add(0x8)) as u64).1;
            // 18912 setne al
            rax = (rax & !0xff) | (((fl.ne() as u64)) as u8 as u64);
            pc = 0x18915;
        }
        0x18915 => {
            // block 18915
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18915 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18915 add rsp,0x148
            { let (r, f) = add::<64>(rsp, 0x148u64);
              rsp = r;
              fl = f; }
            // 1891c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 1891d pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 1891f pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 18921 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 18923 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 18925 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 18926 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x18927 => {
            // block 18927
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18927 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18927 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 1892e mov r13,rax
            r13 = rax;
            // 18931 jmp 0x189c3
            pc = 0x189c3; continue;
        }
        0x18936 => {
            // block 18936
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18936 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18936 mov QWORD PTR [rbp-0x148],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (rbx) as u64);
            // 1893d mov r13,rax
            r13 = rax;
            // 18940 jmp 0x189b7
            pc = 0x189b7; continue;
        }
        0x18942 => {
            // block 18942
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18942 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18942 mov r13,rax
            r13 = rax;
            // 18945 jmp 0x189ab
            pc = 0x189ab; continue;
        }
        0x18947 => {
            // block 18947
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18947 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18947 mov r13,rax
            r13 = rax;
            // 1894a jmp 0x189a3
            pc = 0x189a3; continue;
        }
        0x1894c => {
            // block 1894c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1894c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1894c mov r13,rax
            r13 = rax;
            // 1894f mov rdi,rbx
            rdi = rbx;
            // 18952 jmp 0x1899e
            pc = 0x1899e; continue;
        }
        0x18954 => {
            // block 18954
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18954 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18954 mov r13,rax
            r13 = rax;
            // 18957 jmp 0x18a94
            pc = 0x18a94; continue;
        }
        0x1895c => {
            // block 1895c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1895c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1895c mov r13,rax
            r13 = rax;
            // 1895f jmp 0x18a88
            pc = 0x18a88; continue;
        }
        0x18964 => {
            // block 18964
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18964 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18964 mov r13,rax
            r13 = rax;
            // 18967 jmp 0x18a7c
            pc = 0x18a7c; continue;
        }
        0x1896c => {
            // block 1896c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1896c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1896c mov r13,rax
            r13 = rax;
            // 1896f jmp 0x18a70
            pc = 0x18a70; continue;
        }
        0x18974 => {
            // block 18974
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18974 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18974 mov r13,rax
            r13 = rax;
            // 18977 jmp 0x18a67
            pc = 0x18a67; continue;
        }
        0x1897c => {
            // block 1897c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1897c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1897c mov r13,rax
            r13 = rax;
            // 1897f jmp 0x18a5e
            pc = 0x18a5e; continue;
        }
        0x18984 => {
            // block 18984
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18984 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18984 mov r13,rax
            r13 = rax;
            // 18987 jmp 0x189c3
            pc = 0x189c3; continue;
        }
        0x18989 => {
            // block 18989
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18989 mov r13,rax
            r13 = rax;
            // 1898c jmp 0x189b7
            pc = 0x189b7; continue;
        }
        0x1898e => {
            // block 1898e
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1898e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1898e mov r13,rax
            r13 = rax;
            // 18991 jmp 0x189ab
            pc = 0x189ab; continue;
        }
        0x18993 => {
            // block 18993
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18993 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18993 mov r13,rax
            r13 = rax;
            // 18996 jmp 0x189a3
            pc = 0x189a3; continue;
        }
        0x18998 => {
            // block 18998
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18998 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18998 mov r13,rax
            r13 = rax;
            // 1899b mov rdi,r12
            rdi = r12;
            pc = 0x1899e;
        }
        0x1899e => {
            // block 1899e
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 1899e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1899e call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189a3;
        }
        0x189a3 => {
            // block 189a3
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189a3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189a3 mov rdi,r14
            rdi = r14;
            // 189a6 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189ab;
        }
        0x189ab => {
            // block 189ab
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189ab mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 189b2 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189b7;
        }
        0x189b7 => {
            // block 189b7
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189b7 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 189be call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189c3;
        }
        0x189c3 => {
            // block 189c3
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189c3 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 189ca jmp 0x18a9b
            pc = 0x18a9b; continue;
        }
        0x189cf => {
            // block 189cf
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189cf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189cf mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 189d6 mov r13,rax
            r13 = rax;
            // 189d9 jmp 0x18a52
            pc = 0x18a52; continue;
        }
        0x189db => {
            // block 189db
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189db mov QWORD PTR [rbp-0x148],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r12) as u64);
            // 189e2 mov r13,rax
            r13 = rax;
            // 189e5 jmp 0x18a46
            pc = 0x18a46; continue;
        }
        0x189e7 => {
            // block 189e7
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189e7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189e7 mov r13,rax
            r13 = rax;
            // 189ea jmp 0x18a3a
            pc = 0x18a3a; continue;
        }
        0x189ec => {
            // block 189ec
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189ec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189ec mov r13,rax
            r13 = rax;
            // 189ef jmp 0x189fc
            pc = 0x189fc; continue;
        }
        0x189f1 => {
            // block 189f1
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189f1 mov r13,rax
            r13 = rax;
            // 189f4 mov rdi,r12
            rdi = r12;
            // 189f7 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x189fc;
        }
        0x189fc => {
            // block 189fc
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 189fc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 189fc mov rdi,QWORD PTR [rbp-0x168]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fe98)) as u64;
            // 18a03 jmp 0x18a35
            pc = 0x18a35; continue;
        }
        0x18a05 => {
            // block 18a05
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a05 mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18a0c mov r13,rax
            r13 = rax;
            // 18a0f jmp 0x18a52
            pc = 0x18a52; continue;
        }
        0x18a11 => {
            // block 18a11
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a11 mov QWORD PTR [rbp-0x148],r15
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_feb8), (r15) as u64);
            // 18a18 mov r13,rax
            r13 = rax;
            // 18a1b jmp 0x18a46
            pc = 0x18a46; continue;
        }
        0x18a1d => {
            // block 18a1d
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a1d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a1d mov r13,rax
            r13 = rax;
            // 18a20 jmp 0x18a3a
            pc = 0x18a3a; continue;
        }
        0x18a22 => {
            // block 18a22
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a22 mov r13,rax
            r13 = rax;
            // 18a25 jmp 0x18a32
            pc = 0x18a32; continue;
        }
        0x18a27 => {
            // block 18a27
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a27 mov r13,rax
            r13 = rax;
            // 18a2a mov rdi,r15
            rdi = r15;
            // 18a2d call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a32;
        }
        0x18a32 => {
            // block 18a32
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a32 mov rdi,rbx
            rdi = rbx;
            pc = 0x18a35;
        }
        0x18a35 => {
            // block 18a35
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a35 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a3a;
        }
        0x18a3a => {
            // block 18a3a
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a3a mov rdi,QWORD PTR [rbp-0x158]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_fea8)) as u64;
            // 18a41 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a46;
        }
        0x18a46 => {
            // block 18a46
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a46 mov rdi,QWORD PTR [rbp-0x150]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb0)) as u64;
            // 18a4d call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a52;
        }
        0x18a52 => {
            // block 18a52
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a52 mov rdi,QWORD PTR [rbp-0x148]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_feb8)) as u64;
            // 18a59 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a5e;
        }
        0x18a5e => {
            // block 18a5e
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a5e lea rdi,[rbp-0x60]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 18a62 call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a67;
        }
        0x18a67 => {
            // block 18a67
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a67 lea rdi,[rbp-0x78]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff88);
            // 18a6b call 0xeb954
            { let r = { let t = m.env.stub_va(0x121); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a70;
        }
        0x18a70 => {
            // block 18a70
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a70 lea rdi,[rbp-0x90]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff70);
            // 18a77 call 0xeb960
            { let r = { let t = m.env.stub_va(0x12d); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a7c;
        }
        0x18a7c => {
            // block 18a7c
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a7c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a7c lea rdi,[rbp-0xa8]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff58);
            // 18a83 call 0xeb966
            { let r = { let t = m.env.stub_va(0x133); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a88;
        }
        0x18a88 => {
            // block 18a88
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a88 lea rdi,[rbp-0xc0]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ff40);
            // 18a8f call 0xeb95a
            { let r = { let t = m.env.stub_va(0x127); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x18a94;
        }
        0x18a94 => {
            // block 18a94
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a94 lea rdi,[rbp-0x108]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_fef8);
            pc = 0x18a9b;
        }
        0x18a9b => {
            // block 18a9b
            #[cfg(feature = "trace")] eprintln!("f_00018098 pc 18a9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 18a9b call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 18aa0 mov rdi,r13
            rdi = r13;
            // 18aa3 call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x5825c size=142
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0005825c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x5825c;
    loop {
        match pc {
        0x5825c => {
            // block 5825c
            #[cfg(feature = "trace")] eprintln!("f_0005825c pc 5825c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 5825c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 5825d mov rbp,rsp
            rbp = rsp;
            // 58260 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 58262 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 58264 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 58265 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 58266 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 58269 mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 5826d lea r15,[rdi+rax*1]
            r15 = rdi.wrapping_add(rax);
            // 58271 mov rcx,QWORD PTR [rip+0xe8138] # 0x1403b0
            rcx = m.r64(m.env.data_va(347005)) as u64;
            // 58278 lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // 5827c mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // 58280 lea r14,[rdi+rax*1+0x80]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x80);
            // 58288 lea rdx,[rcx+0x68]
            rdx = rcx.wrapping_add(0x68);
            // 5828c mov QWORD PTR [rdi+rax*1+0x80],rdx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x80), (rdx) as u64);
            // 58294 add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 58298 mov QWORD PTR [rdi+rax*1+0x10],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x10), (rcx) as u64);
            // 5829d lea rbx,[rdi+rax*1+0x18]
            rbx = rdi.wrapping_add(rax).wrapping_add(0x18);
            // 582a2 mov rcx,QWORD PTR [rip+0xe80ff] # 0x1403a8
            rcx = m.r64(m.env.data_va(346997)) as u64;
            // 582a9 add rcx,0x10
            { let (r, f) = add::<64>(rcx, 0x10u64);
              rcx = r;
              fl = f; }
            // 582ad mov QWORD PTR [rdi+rax*1+0x18],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x18), (rcx) as u64);
            // 582b2 lea rdi,[rdi+rax*1+0x58]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x58);
            // 582b7 call 0xec032
            { let r = { let t = m.env.stub_va(0x7ff); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 582bc mov rdi,rbx
            rdi = rbx;
            // 582bf call 0xec0c8
            { let r = { let t = m.env.stub_va(0x895); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 582c4 mov rsi,QWORD PTR [rip+0xe7f2d] # 0x1401f8
            rsi = m.r64(m.env.data_va(346565)) as u64;
            // 582cb add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 582cf mov rdi,r15
            rdi = r15;
            // 582d2 call 0xec0bc
            { let r = { let t = m.env.stub_va(0x889); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 582d7 mov rdi,r14
            rdi = r14;
            // 582da add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 582de pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 582df pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 582e1 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 582e3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 582e4 jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x43a6c size=44
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043a6c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x43a6c;
    loop {
        match pc {
        0x43a6c => {
            // block 43a6c
            #[cfg(feature = "trace")] eprintln!("f_00043a6c pc 43a6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a6c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43a6d mov rbp,rsp
            rbp = rsp;
            // 43a70 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43a73 jle 0x43a79
            if fl.le() { pc = 0x43a79; continue; }
            pc = 0x43a75;
        }
        0x43a75 => {
            // block 43a75
            #[cfg(feature = "trace")] eprintln!("f_00043a6c pc 43a75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a75 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43a77 jmp 0x43a96
            pc = 0x43a96; continue;
        }
        0x43a79 => {
            // block 43a79
            #[cfg(feature = "trace")] eprintln!("f_00043a6c pc 43a79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a79 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43a7c mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43a7e lea rsi,[rip+0xbc93b] # 0x1003c0
            rsi = m.env.data_va(84877);
            // 43a85 mov esi,DWORD PTR [rsi+rcx*4]
            rsi = (m.r32(rsi.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43a88 movzx edx,dl
            rdx = ((rdx as u8 as u64)) as u32 as u64;
            // 43a8b mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 43a90 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 43a93 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x43a96;
        }
        0x43a96 => {
            // block 43a96
            #[cfg(feature = "trace")] eprintln!("f_00043a6c pc 43a96 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a96 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43a97 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2db52 size=90
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002db52<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x2db52;
    loop {
        match pc {
        0x2db52 => {
            // block 2db52
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db52 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2db53 mov rbp,rsp
            rbp = rsp;
            // 2db56 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 2db5a test rax,rax
            fl = logic::<64>(rax & rax);
            // 2db5d je 0x2db90
            if fl.e() { pc = 0x2db90; continue; }
            pc = 0x2db5f;
        }
        0x2db5f => {
            // block 2db5f
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db5f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db5f mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 2db61 jmp 0x2db66
            pc = 0x2db66; continue;
        }
        0x2db63 => {
            // block 2db63
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db63 mov rax,rdx
            rax = rdx;
            pc = 0x2db66;
        }
        0x2db66 => {
            // block 2db66
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db66 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 2db69 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 2db6b jge 0x2db77
            if fl.ge() { pc = 0x2db77; continue; }
            pc = 0x2db6d;
        }
        0x2db6d => {
            // block 2db6d
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db6d mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 2db70 test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2db73 jne 0x2db63
            if fl.ne() { pc = 0x2db63; continue; }
            pc = 0x2db75;
        }
        0x2db75 => {
            // block 2db75
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db75 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db75 jmp 0x2db9c
            pc = 0x2db9c; continue;
        }
        0x2db77 => {
            // block 2db77
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db77 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 2db79 jge 0x2dba4
            if fl.ge() { pc = 0x2dba4; continue; }
            pc = 0x2db7b;
        }
        0x2db7b => {
            // block 2db7b
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db7b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db7b mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 2db7f test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 2db82 jne 0x2db63
            if fl.ne() { pc = 0x2db63; continue; }
            pc = 0x2db84;
        }
        0x2db84 => {
            // block 2db84
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db84 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2db87 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 2db8b mov rsi,rax
            rsi = rax;
            // 2db8e jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2db90 => {
            // block 2db90
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db90 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db90 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 2db94 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 2db97 mov rsi,rdi
            rsi = rdi;
            // 2db9a jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2db9c => {
            // block 2db9c
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2db9c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2db9c mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 2db9f mov rsi,rax
            rsi = rax;
            // 2dba2 jmp 0x2dba7
            pc = 0x2dba7; continue;
        }
        0x2dba4 => {
            // block 2dba4
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2dba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dba4 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x2dba7;
        }
        0x2dba7 => {
            // block 2dba7
            #[cfg(feature = "trace")] eprintln!("f_0002db52 pc 2dba7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2dba7 mov rax,rsi
            rax = rsi;
            // 2dbaa pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2dbab ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8cbc size=112
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008cbc<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x8cbc;
    loop {
        match pc {
        0x8cbc => {
            // block 8cbc
            #[cfg(feature = "trace")] eprintln!("f_00008cbc pc 8cbc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8cbc push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8cbd mov rbp,rsp
            rbp = rsp;
            // 8cc0 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8cc2 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8cc4 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8cc5 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 8cc6 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 8cc9 mov r14,rsi
            r14 = rsi;
            // 8ccc mov rbx,rdi
            rbx = rdi;
            // 8ccf mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8cd2 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 8cd5 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8cdb test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8cdd je 0x8ce3
            if fl.e() { pc = 0x8ce3; continue; }
            pc = 0x8cdf;
        }
        0x8cdf => {
            // block 8cdf
            #[cfg(feature = "trace")] eprintln!("f_00008cbc pc 8cdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8cdf xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 8ce1 jmp 0x8d21
            pc = 0x8d21; continue;
        }
        0x8ce3 => {
            // block 8ce3
            #[cfg(feature = "trace")] eprintln!("f_00008cbc pc 8ce3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8ce3 mov DWORD PTR [rbp-0x1c],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4), (0x0u64) as u32);
            // 8cea mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8ced mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 8cf0 lea rcx,[rip+0xf6479] # 0xff170
            rcx = m.env.data_va(80189);
            // 8cf7 mov esi,DWORD PTR [rcx+rdx*4]
            rsi = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8cfa lea rcx,[rip+0xf692f] # 0xff630
            rcx = m.env.data_va(81405);
            // 8d01 mov ecx,DWORD PTR [rcx+rdx*4]
            rcx = (m.r32(rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8d04 lea rdi,[rip+0xf6955] # 0xff660
            rdi = m.env.data_va(81453);
            // 8d0b mov r8d,DWORD PTR [rdi+rdx*4]
            r8 = (m.r32(rdi.wrapping_add(rdx.wrapping_mul(4))) as u64) as u32 as u64;
            // 8d0f lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // 8d13 mov rdi,rbx
            rdi = rbx;
            // 8d16 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8d19 cmp DWORD PTR [rbp-0x1c],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x0u64).1;
            // 8d1d setne BYTE PTR [r14]
            m.w8(r14, ((fl.ne() as u64)) as u8);
            pc = 0x8d21;
        }
        0x8d21 => {
            // block 8d21
            #[cfg(feature = "trace")] eprintln!("f_00008cbc pc 8d21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8d21 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 8d25 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8d26 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8d28 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8d2a pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8d2b ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51db4 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051db4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x51db4;
    loop {
        match pc {
        0x51db4 => {
            // block 51db4
            #[cfg(feature = "trace")] eprintln!("f_00051db4 pc 51db4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51db4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51db5 mov rbp,rsp
            rbp = rsp;
            // 51db8 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51dba mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51dbd mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51dc1 mov esi,0x283c
            rsi = (0x283cu64) as u32 as u64;
            // 51dc6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51dcb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51dce pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51dcf jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10690 size=1138
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x94f
pub fn f_00010690<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut x1: u128 = fx[1] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x10690;
    loop {
        match pc {
        0x10690 => {
            // block 10690
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10690 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10691 mov rbp,rsp
            rbp = rsp;
            // 10694 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 10696 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 10698 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 1069a push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 1069c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1069d sub rsp,0x68
            { let (r, f) = sub::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 106a1 mov r15,rdi
            r15 = rdi;
            // 106a4 mov rax,QWORD PTR [r15+0x20]
            rax = m.r64(r15.wrapping_add(0x20)) as u64;
            // 106a8 cmp rax,0x400
            fl = sub::<64>(rax, 0x400u64).1;
            // 106ae jb 0x10752
            if fl.b() { pc = 0x10752; continue; }
            pc = 0x106b4;
        }
        0x106b4 => {
            // block 106b4
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 106b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 106b4 add rax,0xfffffffffffffc00
            { let (r, f) = add::<64>(rax, 0xffff_ffff_ffff_fc00u64);
              rax = r;
              fl = f; }
            // 106ba mov QWORD PTR [r15+0x20],rax
            m.w64(r15.wrapping_add(0x20), (rax) as u64);
            // 106be mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 106c2 mov r12,QWORD PTR [rbx]
            r12 = m.r64(rbx) as u64;
            // 106c5 lea r13,[rbx+0x8]
            r13 = rbx.wrapping_add(0x8);
            // 106c9 mov QWORD PTR [r15+0x8],r13
            m.w64(r15.wrapping_add(0x8), (r13) as u64);
            // 106cd mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 106d1 mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 106d5 cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // 106d8 jne 0x109e6
            if fl.ne() { pc = 0x109e6; continue; }
            pc = 0x106de;
        }
        0x106de => {
            // block 106de
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 106de rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 106de mov QWORD PTR [rbp-0x70],r12
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (r12) as u64);
            // 106e2 mov rdx,QWORD PTR [r15]
            rdx = m.r64(r15) as u64;
            // 106e5 mov rax,r13
            rax = r13;
            // 106e8 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 106eb jbe 0x10859
            if fl.be() { pc = 0x10859; continue; }
            pc = 0x106f1;
        }
        0x106f1 => {
            // block 106f1
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 106f1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 106f1 sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 106f5 lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 106f9 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 106fd lea rax,[rax+rcx*1+0x1]
            rax = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 10702 sar rax,1
            { let (r, f) = sar::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 10705 mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 10709 lea rcx,[rax*8+0x0]
            rcx = rax.wrapping_mul(8);
            // 10711 mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // 10715 mov rdi,rbx
            rdi = rbx;
            // 10718 sub rdi,rcx
            { let (r, f) = sub::<64>(rdi, rcx);
              rdi = r;
              fl = f; }
            // 1071b add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 1071f sub r14,r13
            { let (r, f) = sub::<64>(r14, r13);
              r14 = r;
              fl = f; }
            // 10722 mov r12,r14
            r12 = r14;
            // 10725 shr r12,0x3
            { let (r, f) = shr::<64>(r12, 0x3u64, fl); r12 = r; fl = f; }
            // 10729 mov rsi,r13
            rsi = r13;
            // 1072c mov rdx,r14
            rdx = r14;
            // 1072f call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10734 sub r12,QWORD PTR [rbp-0x80]
            { let (r, f) = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64);
              r12 = r;
              fl = f; }
            // 10738 lea r14,[rbx+r12*8+0x8]
            r14 = rbx.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x8);
            // 1073d mov QWORD PTR [r15+0x10],r14
            m.w64(r15.wrapping_add(0x10), (r14) as u64);
            // 10741 mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 10745 sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 10749 mov r12,QWORD PTR [rbp-0x70]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 1074d jmp 0x109e6
            pc = 0x109e6; continue;
        }
        0x10752 => {
            // block 10752
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10752 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10752 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10756 mov r12,QWORD PTR [r15+0x18]
            r12 = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1075a mov rbx,r14
            rbx = r14;
            // 1075d sub rbx,QWORD PTR [r15+0x8]
            { let (r, f) = sub::<64>(rbx, m.r64(r15.wrapping_add(0x8)) as u64);
              rbx = r;
              fl = f; }
            // 10761 mov rax,r12
            rax = r12;
            // 10764 sub rax,QWORD PTR [r15]
            { let (r, f) = sub::<64>(rax, m.r64(r15) as u64);
              rax = r;
              fl = f; }
            // 10767 cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 1076a jae 0x10794
            if fl.ae() { pc = 0x10794; continue; }
            pc = 0x1076c;
        }
        0x1076c => {
            // block 1076c
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 1076c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1076c mov edi,0x1000
            rdi = (0x1000u64) as u32 as u64;
            // 10771 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10776 cmp r12,r14
            fl = sub::<64>(r12, r14).1;
            // 10779 je 0x10908
            if fl.e() { pc = 0x10908; continue; }
            pc = 0x1077f;
        }
        0x1077f => {
            // block 1077f
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 1077f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1077f mov QWORD PTR [rbp-0x58],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa8), (rax) as u64);
            // 10783 lea rsi,[rbp-0x58]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa8);
            // 10787 mov rdi,r15
            rdi = r15;
            // 1078a call 0xeb846
            { let r = { let t = m.env.stub_va(0x13); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1078f jmp 0x109ee
            pc = 0x109ee; continue;
        }
        0x10794 => {
            // block 10794
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10794 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10794 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10798 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 1079e cmovne r14,rax
            if fl.ne() { r14 = rax; }
            // 107a2 and rbx,0xfffffffffffffff8
            { let r = rbx & 0xffff_ffff_ffff_fff8u64; let f = logic::<64>(r);
              rbx = r;
              fl = f; }
            // 107a6 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 107aa mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 107ae lea rdi,[r14*8+0x0]
            rdi = r14.wrapping_mul(8);
            // 107b6 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107bb mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 107bf add rbx,rax
            { let (r, f) = add::<64>(rbx, rax);
              rbx = r;
              fl = f; }
            // 107c2 mov QWORD PTR [rbp-0x40],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rbx) as u64);
            // 107c6 mov QWORD PTR [rbp-0x48],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rbx) as u64);
            // 107ca lea rax,[rax+r14*8]
            rax = rax.wrapping_add(r14.wrapping_mul(8));
            // 107ce mov QWORD PTR [rbp-0x38],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rax) as u64);
            // 107d2 mov edi,0x1000
            rdi = (0x1000u64) as u32 as u64;
            // 107d7 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107dc mov rbx,rax
            rbx = rax;
            // 107df mov QWORD PTR [rbp-0x68],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff98), (rbx) as u64);
            // 107e3 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 107e7 lea rsi,[rbp-0x68]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ff98);
            // 107eb call 0xeb858
            { let r = { let t = m.env.stub_va(0x25); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 107f0 mov rbx,QWORD PTR [r15+0x10]
            rbx = m.r64(r15.wrapping_add(0x10)) as u64;
            // 107f4 lea r14,[rbp-0x50]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            pc = 0x107f8;
        }
        0x107f8 => {
            // block 107f8
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 107f8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 107f8 mov rax,QWORD PTR [r15+0x8]
            rax = m.r64(r15.wrapping_add(0x8)) as u64;
            // 107fc cmp rbx,rax
            fl = sub::<64>(rbx, rax).1;
            // 107ff je 0x10826
            if fl.e() { pc = 0x10826; continue; }
            pc = 0x10801;
        }
        0x10801 => {
            // block 10801
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10801 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10801 add rbx,0xfffffffffffffff8
            { let (r, f) = add::<64>(rbx, 0xffff_ffff_ffff_fff8u64);
              rbx = r;
              fl = f; }
            // 10805 mov rdi,r14
            rdi = r14;
            // 10808 mov rsi,rbx
            rsi = rbx;
            // 1080b call 0xeb852
            { let r = { let t = m.env.stub_va(0x1f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10810 jmp 0x107f8
            pc = 0x107f8; continue;
        }
        0x10812 => {
            // block 10812
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10812 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10812 mov r14,rax
            r14 = rax;
            pc = 0x10815;
        }
        0x10815 => {
            // block 10815
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10815 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10815 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10819 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1081e mov rdi,r14
            rdi = r14;
            // 10821 call 0xec182
            { let _ = { let t = m.env.stub_va(0x94f); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::default(); }
        }
        0x10826 => {
            // block 10826
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10826 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10826 mov rcx,QWORD PTR [r15]
            rcx = m.r64(r15) as u64;
            // 10829 movaps xmm0,XMMWORD PTR [rbp-0x50]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffb0));
            // 1082d mov QWORD PTR [rbp-0x50],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rcx) as u64);
            // 10831 movups XMMWORD PTR [r15],xmm0
            m.w128(r15, x0);
            // 10835 mov QWORD PTR [rbp-0x48],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rax) as u64);
            // 10839 movaps xmm0,XMMWORD PTR [rbp-0x40]
            x0 = m.r128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0));
            // 1083d movups xmm1,XMMWORD PTR [r15+0x10]
            x1 = m.r128(r15.wrapping_add(0x10));
            // 10842 movups XMMWORD PTR [r15+0x10],xmm0
            m.w128(r15.wrapping_add(0x10), x0);
            // 10847 movaps XMMWORD PTR [rbp-0x40],xmm1
            m.w128(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), x1);
            // 1084b lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 1084f call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10854 jmp 0x109ee
            pc = 0x109ee; continue;
        }
        0x10859 => {
            // block 10859
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10859 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10859 mov QWORD PTR [rbp-0x80],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rcx) as u64);
            // 1085d mov rax,r14
            rax = r14;
            // 10860 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 10863 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10867 mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 1086d cmovne r12,rax
            if fl.ne() { r12 = rax; }
            // 10871 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 1087b and rax,r12
            { let r = rax & r12; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 1087e mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 10882 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10886 mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 1088a lea rdi,[r12*8+0x0]
            rdi = r12.wrapping_mul(8);
            // 10892 mov QWORD PTR [rbp-0x88],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdx) as u64);
            // 10899 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1089e mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 108a2 mov rcx,QWORD PTR [rbp-0x78]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 108a6 lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 108aa mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 108ae mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 108b2 lea rdx,[rax+r12*8]
            rdx = rax.wrapping_add(r12.wrapping_mul(8));
            // 108b6 mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 108ba cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 108bd je 0x109a8
            if fl.e() { pc = 0x109a8; continue; }
            pc = 0x108c3;
        }
        0x108c3 => {
            // block 108c3
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 108c3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 108c3 mov r12,QWORD PTR [rbp-0x70]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x108c7;
        }
        0x108c7 => {
            // block 108c7
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 108c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 108c7 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 108cb mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 108ce add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 108d2 mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 108d6 mov rax,rbx
            rax = rbx;
            // 108d9 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 108dd cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 108e0 mov rbx,r13
            rbx = r13;
            // 108e3 mov r13,rax
            r13 = rax;
            // 108e6 jne 0x108c7
            if fl.ne() { pc = 0x108c7; continue; }
            pc = 0x108e8;
        }
        0x108e8 => {
            // block 108e8
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 108e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 108e8 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 108eb mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 108ef mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 108f3 mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 108f7 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 108fb mov rdi,QWORD PTR [r15+0x18]
            rdi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 108ff mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10903 jmp 0x109ba
            pc = 0x109ba; continue;
        }
        0x10908 => {
            // block 10908
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10908 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10908 mov QWORD PTR [rbp-0x60],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffa0), (rax) as u64);
            // 1090c lea rsi,[rbp-0x60]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffa0);
            // 10910 mov rdi,r15
            rdi = r15;
            // 10913 call 0xeb840
            { let r = { let t = m.env.stub_va(0xd); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10918 mov rbx,QWORD PTR [r15+0x8]
            rbx = m.r64(r15.wrapping_add(0x8)) as u64;
            // 1091c mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 1091f lea r13,[rbx+0x8]
            r13 = rbx.wrapping_add(0x8);
            // 10923 mov QWORD PTR [r15+0x8],r13
            m.w64(r15.wrapping_add(0x8), (r13) as u64);
            // 10927 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 1092b mov rcx,QWORD PTR [r15+0x18]
            rcx = m.r64(r15.wrapping_add(0x18)) as u64;
            // 1092f cmp r14,rcx
            fl = sub::<64>(r14, rcx).1;
            // 10932 jne 0x10adf
            if fl.ne() { pc = 0x10adf; continue; }
            pc = 0x10938;
        }
        0x10938 => {
            // block 10938
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10938 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10938 mov QWORD PTR [rbp-0x70],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff90), (rax) as u64);
            // 1093c mov rdx,QWORD PTR [r15]
            rdx = m.r64(r15) as u64;
            // 1093f mov rax,r13
            rax = r13;
            // 10942 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 10945 jbe 0x109fd
            if fl.be() { pc = 0x109fd; continue; }
            pc = 0x1094b;
        }
        0x1094b => {
            // block 1094b
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 1094b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1094b sar rax,0x3
            { let (r, f) = sar::<64>(rax, 0x3u64, fl); rax = r; fl = f; }
            // 1094f lea rcx,[rax+0x1]
            rcx = rax.wrapping_add(0x1);
            // 10953 shr rcx,0x3f
            { let (r, f) = shr::<64>(rcx, 0x3fu64, fl); rcx = r; fl = f; }
            // 10957 lea rax,[rax+rcx*1+0x1]
            rax = rax.wrapping_add(rcx).wrapping_add(0x1);
            // 1095c sar rax,1
            { let (r, f) = sar::<64>(rax, 0x1u64, fl); rax = r; fl = f; }
            // 1095f mov QWORD PTR [rbp-0x80],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rax) as u64);
            // 10963 lea rcx,[rax*8+0x0]
            rcx = rax.wrapping_mul(8);
            // 1096b mov QWORD PTR [rbp-0x78],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rcx) as u64);
            // 1096f mov rdi,rbx
            rdi = rbx;
            // 10972 sub rdi,rcx
            { let (r, f) = sub::<64>(rdi, rcx);
              rdi = r;
              fl = f; }
            // 10975 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 10979 sub r14,r13
            { let (r, f) = sub::<64>(r14, r13);
              r14 = r;
              fl = f; }
            // 1097c mov r12,r14
            r12 = r14;
            // 1097f shr r12,0x3
            { let (r, f) = shr::<64>(r12, 0x3u64, fl); r12 = r; fl = f; }
            // 10983 mov rsi,r13
            rsi = r13;
            // 10986 mov rdx,r14
            rdx = r14;
            // 10989 call 0xec224
            { let r = { let t = m.env.stub_va(0x9f1); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 1098e sub r12,QWORD PTR [rbp-0x80]
            { let (r, f) = sub::<64>(r12, m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64);
              r12 = r;
              fl = f; }
            // 10992 lea r14,[rbx+r12*8+0x8]
            r14 = rbx.wrapping_add(r12.wrapping_mul(8)).wrapping_add(0x8);
            // 10997 mov QWORD PTR [r15+0x10],r14
            m.w64(r15.wrapping_add(0x10), (r14) as u64);
            // 1099b mov rax,QWORD PTR [rbp-0x78]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 1099f sub QWORD PTR [r15+0x8],rax
            { let (r, f) = sub::<64>(m.r64(r15.wrapping_add(0x8)) as u64, rax);
              m.w64(r15.wrapping_add(0x8), (r) as u64);
              fl = f; }
            // 109a3 jmp 0x10adb
            pc = 0x10adb; continue;
        }
        0x109a8 => {
            // block 109a8
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109a8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109a8 mov rsi,rcx
            rsi = rcx;
            // 109ab mov r12,QWORD PTR [rbp-0x70]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            // 109af mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 109b3 mov rbx,QWORD PTR [rbp-0x88]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x109ba;
        }
        0x109ba => {
            // block 109ba
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109ba mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 109bd mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 109c1 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 109c5 mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 109c9 mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 109cd mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            // 109d1 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 109d5 mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 109d9 lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 109dd call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 109e2 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x109e6;
        }
        0x109e6 => {
            // block 109e6
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109e6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109e6 mov QWORD PTR [r14],r12
            m.w64(r14, (r12) as u64);
            // 109e9 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            pc = 0x109ee;
        }
        0x109ee => {
            // block 109ee
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109ee add rsp,0x68
            { let (r, f) = add::<64>(rsp, 0x68u64);
              rsp = r;
              fl = f; }
            // 109f2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 109f3 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 109f5 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 109f7 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 109f9 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 109fb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 109fc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x109fd => {
            // block 109fd
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 109fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 109fd mov QWORD PTR [rbp-0x80],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff80), (rcx) as u64);
            // 10a01 mov rax,r14
            rax = r14;
            // 10a04 sub rax,rdx
            { let (r, f) = sub::<64>(rax, rdx);
              rax = r;
              fl = f; }
            // 10a07 sar rax,0x2
            { let (r, f) = sar::<64>(rax, 0x2u64, fl); rax = r; fl = f; }
            // 10a0b mov r12d,0x1
            r12 = (0x1u64) as u32 as u64;
            // 10a11 cmovne r12,rax
            if fl.ne() { r12 = rax; }
            // 10a15 movabs rax,0x7ffffffffffffffc
            rax = 0x7fff_ffff_ffff_fffcu64;
            // 10a1f and rax,r12
            { let r = rax & r12; let f = logic::<64>(r);
              rax = r;
              fl = f; }
            // 10a22 mov QWORD PTR [rbp-0x78],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff88), (rax) as u64);
            // 10a26 lea rax,[r15+0x18]
            rax = r15.wrapping_add(0x18);
            // 10a2a mov QWORD PTR [rbp-0x30],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rax) as u64);
            // 10a2e lea rdi,[r12*8+0x0]
            rdi = r12.wrapping_mul(8);
            // 10a36 mov QWORD PTR [rbp-0x88],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ff78), (rdx) as u64);
            // 10a3d call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10a42 mov QWORD PTR [rbp-0x50],rax
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rax) as u64);
            // 10a46 mov rcx,QWORD PTR [rbp-0x78]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff88)) as u64;
            // 10a4a lea rcx,[rax+rcx*2]
            rcx = rax.wrapping_add(rcx.wrapping_mul(2));
            // 10a4e mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10a52 mov QWORD PTR [rbp-0x48],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (rcx) as u64);
            // 10a56 lea rdx,[rax+r12*8]
            rdx = rax.wrapping_add(r12.wrapping_mul(8));
            // 10a5a mov QWORD PTR [rbp-0x38],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdx) as u64);
            // 10a5e cmp r13,r14
            fl = sub::<64>(r13, r14).1;
            // 10a61 je 0x10aa1
            if fl.e() { pc = 0x10aa1; continue; }
            pc = 0x10a63;
        }
        0x10a63 => {
            // block 10a63
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10a63 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10a63 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 10a67 mov QWORD PTR [rcx],rax
            m.w64(rcx, (rax) as u64);
            // 10a6a add rcx,0x8
            { let (r, f) = add::<64>(rcx, 0x8u64);
              rcx = r;
              fl = f; }
            // 10a6e mov QWORD PTR [rbp-0x40],rcx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (rcx) as u64);
            // 10a72 mov rax,rbx
            rax = rbx;
            // 10a75 add rax,0x10
            { let (r, f) = add::<64>(rax, 0x10u64);
              rax = r;
              fl = f; }
            // 10a79 cmp rax,r14
            fl = sub::<64>(rax, r14).1;
            // 10a7c mov rbx,r13
            rbx = r13;
            // 10a7f mov r13,rax
            r13 = rax;
            // 10a82 jne 0x10a63
            if fl.ne() { pc = 0x10a63; continue; }
            pc = 0x10a84;
        }
        0x10a84 => {
            // block 10a84
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10a84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10a84 mov rbx,QWORD PTR [r15]
            rbx = m.r64(r15) as u64;
            // 10a87 mov r13,QWORD PTR [r15+0x8]
            r13 = m.r64(r15.wrapping_add(0x8)) as u64;
            // 10a8b mov rax,QWORD PTR [rbp-0x50]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0)) as u64;
            // 10a8f mov rsi,QWORD PTR [rbp-0x48]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8)) as u64;
            // 10a93 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            // 10a97 mov rdi,QWORD PTR [r15+0x18]
            rdi = m.r64(r15.wrapping_add(0x18)) as u64;
            // 10a9b mov rdx,QWORD PTR [rbp-0x38]
            rdx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // 10a9f jmp 0x10aaf
            pc = 0x10aaf; continue;
        }
        0x10aa1 => {
            // block 10aa1
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10aa1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10aa1 mov rsi,rcx
            rsi = rcx;
            // 10aa4 mov rdi,QWORD PTR [rbp-0x80]
            rdi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff80)) as u64;
            // 10aa8 mov rbx,QWORD PTR [rbp-0x88]
            rbx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff78)) as u64;
            pc = 0x10aaf;
        }
        0x10aaf => {
            // block 10aaf
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10aaf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10aaf mov QWORD PTR [r15],rax
            m.w64(r15, (rax) as u64);
            // 10ab2 mov QWORD PTR [rbp-0x50],rbx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb0), (rbx) as u64);
            // 10ab6 mov QWORD PTR [r15+0x8],rsi
            m.w64(r15.wrapping_add(0x8), (rsi) as u64);
            // 10aba mov QWORD PTR [rbp-0x48],r13
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffb8), (r13) as u64);
            // 10abe mov QWORD PTR [r15+0x10],rcx
            m.w64(r15.wrapping_add(0x10), (rcx) as u64);
            // 10ac2 mov QWORD PTR [rbp-0x40],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc0), (r14) as u64);
            // 10ac6 mov QWORD PTR [r15+0x18],rdx
            m.w64(r15.wrapping_add(0x18), (rdx) as u64);
            // 10aca mov QWORD PTR [rbp-0x38],rdi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rdi) as u64);
            // 10ace lea rdi,[rbp-0x50]
            rdi = rbp.wrapping_add(0xffff_ffff_ffff_ffb0);
            // 10ad2 call 0xeb85e
            { let r = { let t = m.env.stub_va(0x2b); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10ad7 mov r14,QWORD PTR [r15+0x10]
            r14 = m.r64(r15.wrapping_add(0x10)) as u64;
            pc = 0x10adb;
        }
        0x10adb => {
            // block 10adb
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10adb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10adb mov rax,QWORD PTR [rbp-0x70]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ff90)) as u64;
            pc = 0x10adf;
        }
        0x10adf => {
            // block 10adf
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10adf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10adf mov QWORD PTR [r14],rax
            m.w64(r14, (rax) as u64);
            // 10ae2 add QWORD PTR [r15+0x10],0x8
            { let (r, f) = add::<64>(m.r64(r15.wrapping_add(0x10)) as u64, 0x8u64);
              m.w64(r15.wrapping_add(0x10), (r) as u64);
              fl = f; }
            // 10ae7 jmp 0x109ee
            pc = 0x109ee; continue;
        }
        0x10aec => {
            // block 10aec
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10aec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10aec jmp 0x10812
            pc = 0x10812; continue;
        }
        0x10af1 => {
            // block 10af1
            #[cfg(feature = "trace")] eprintln!("f_00010690 pc 10af1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10af1 mov r14,rax
            r14 = rax;
            // 10af4 mov rdi,rbx
            rdi = rbx;
            // 10af7 call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 10afc jmp 0x10815
            pc = 0x10815; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x41fda size=66
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00041fda<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x41fda;
    loop {
        match pc {
        0x41fda => {
            // block 41fda
            #[cfg(feature = "trace")] eprintln!("f_00041fda pc 41fda rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 41fda push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 41fdb mov rbp,rsp
            rbp = rsp;
            // 41fde push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 41fdf push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 41fe0 mov rbx,rsi
            rbx = rsi;
            // 41fe3 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 41fea mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 41fed mov ecx,edx
            rcx = ((rdx as u32 as u64)) as u32 as u64;
            // 41fef lea rdx,[rip+0xbe3ca] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 41ff6 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 41ff9 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 41ffd mov ecx,0x60000000
            rcx = (0x60000000u64) as u32 as u64;
            // 42002 mov r8d,0x1d
            r8 = (0x1du64) as u32 as u64;
            // 42008 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4200b test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 4200d je 0x42014
            if fl.e() { pc = 0x42014; continue; }
            pc = 0x4200f;
        }
        0x4200f => {
            // block 4200f
            #[cfg(feature = "trace")] eprintln!("f_00041fda pc 4200f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4200f mov ecx,DWORD PTR [rbp-0xc]
            rcx = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64) as u32 as u64;
            // 42012 mov DWORD PTR [rbx],ecx
            m.w32(rbx, ((rcx as u32 as u64)) as u32);
            pc = 0x42014;
        }
        0x42014 => {
            // block 42014
            #[cfg(feature = "trace")] eprintln!("f_00041fda pc 42014 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 42014 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 42018 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 42019 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 4201a ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x8dc2 size=110
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00008dc2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x8dc2;
    loop {
        match pc {
        0x8dc2 => {
            // block 8dc2
            #[cfg(feature = "trace")] eprintln!("f_00008dc2 pc 8dc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8dc2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 8dc3 mov rbp,rsp
            rbp = rsp;
            // 8dc6 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 8dc8 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 8dca push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 8dcc push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 8dcd sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8dd1 mov r15,rdx
            r15 = rdx;
            // 8dd4 mov r14d,esi
            r14 = ((rsi as u32 as u64)) as u32 as u64;
            // 8dd7 mov rbx,rdi
            rbx = rdi;
            // 8dda mov DWORD PTR [r15],0x1
            m.w32(r15, (0x1u64) as u32);
            // 8de1 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            // 8de6 lea r12,[rbp-0x24]
            r12 = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            pc = 0x8dea;
        }
        0x8dea => {
            // block 8dea
            #[cfg(feature = "trace")] eprintln!("f_00008dc2 pc 8dea rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8dea mov DWORD PTR [rbp-0x24],0xff
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0xffu64) as u32);
            // 8df1 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // 8df4 mov rdi,rbx
            rdi = rbx;
            // 8df7 mov rdx,r12
            rdx = r12;
            // 8dfa call QWORD PTR [rax+0x10c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x10c8)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 8e00 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 8e02 je 0x8e0a
            if fl.e() { pc = 0x8e0a; continue; }
            pc = 0x8e04;
        }
        0x8e04 => {
            // block 8e04
            #[cfg(feature = "trace")] eprintln!("f_00008dc2 pc 8e04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e04 cmp DWORD PTR [rbp-0x24],r14d
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64, (r14 as u32 as u64)).1;
            // 8e08 je 0x8e21
            if fl.e() { pc = 0x8e21; continue; }
            pc = 0x8e0a;
        }
        0x8e0a => {
            // block 8e0a
            #[cfg(feature = "trace")] eprintln!("f_00008dc2 pc 8e0a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e0a mov esi,DWORD PTR [r15]
            rsi = (m.r32(r15) as u64) as u32 as u64;
            // 8e0d inc esi
            { let (r, f) = inc::<32>((rsi as u32 as u64), fl); rsi = (r) as u32 as u64; fl = f; }
            // 8e0f mov DWORD PTR [r15],esi
            m.w32(r15, ((rsi as u32 as u64)) as u32);
            // 8e12 cmp esi,0x86
            fl = sub::<32>((rsi as u32 as u64), 0x86u64).1;
            // 8e18 jb 0x8dea
            if fl.b() { pc = 0x8dea; continue; }
            pc = 0x8e1a;
        }
        0x8e1a => {
            // block 8e1a
            #[cfg(feature = "trace")] eprintln!("f_00008dc2 pc 8e1a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e1a mov DWORD PTR [r15],0xffffffff
            m.w32(r15, (0xffffffffu64) as u32);
            pc = 0x8e21;
        }
        0x8e21 => {
            // block 8e21
            #[cfg(feature = "trace")] eprintln!("f_00008dc2 pc 8e21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 8e21 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 8e23 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 8e27 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 8e28 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 8e2a pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 8e2c pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 8e2e pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 8e2f ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x51ea4 size=30
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00051ea4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x51ea4;
    loop {
        match pc {
        0x51ea4 => {
            // block 51ea4
            #[cfg(feature = "trace")] eprintln!("f_00051ea4 pc 51ea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 51ea4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 51ea5 mov rbp,rsp
            rbp = rsp;
            // 51ea8 mov edx,esi
            rdx = ((rsi as u32 as u64)) as u32 as u64;
            // 51eaa mov rcx,QWORD PTR [rdi]
            rcx = m.r64(rdi) as u64;
            // 51ead mov rax,QWORD PTR [rcx+0x30]
            rax = m.r64(rcx.wrapping_add(0x30)) as u64;
            // 51eb1 mov esi,0x2840
            rsi = (0x2840u64) as u32 as u64;
            // 51eb6 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 51ebb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 51ebe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 51ebf jmp rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x1710 size=272
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00001710<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
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
    let mut pc: u32 = 0x1710;
    loop {
        match pc {
        0x1710 => {
            // block 1710
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 1710 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1710 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 1711 mov rbp,rsp
            rbp = rsp;
            // 1714 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 1716 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 1717 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 171b mov ebx,edx
            rbx = ((rdx as u32 as u64)) as u32 as u64;
            // 171d mov r14,rdi
            r14 = rdi;
            // 1720 test esi,esi
            fl = logic::<32>((rsi as u32 as u64) & (rsi as u32 as u64));
            // 1722 je 0x1750
            if fl.e() { pc = 0x1750; continue; }
            pc = 0x1724;
        }
        0x1724 => {
            // block 1724
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 1724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1724 mov BYTE PTR [rbp-0x11],0x0
            m.w8(rbp.wrapping_add(0xffff_ffff_ffff_ffef), (0x0u64) as u8);
            // 1728 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 172b lea rsi,[rbp-0x11]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffef);
            // 172f mov rdi,r14
            rdi = r14;
            // 1732 mov edx,ebx
            rdx = ((rbx as u32 as u64)) as u32 as u64;
            // 1734 call QWORD PTR [rax+0x4a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4a8)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 173a cmp BYTE PTR [rbp-0x11],0x0
            fl = sub::<8>(m.r8(rbp.wrapping_add(0xffff_ffff_ffff_ffef)) as u64, 0x0u64).1;
            // 173e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1741 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 1745 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 174a je 0x17a5
            if fl.e() { pc = 0x17a5; continue; }
            pc = 0x174c;
        }
        0x174c => {
            // block 174c
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 174c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 174c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 174e jmp 0x17aa
            pc = 0x17aa; continue;
        }
        0x1750 => {
            // block 1750
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 1750 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1750 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1753 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 1757 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 175c xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 175e cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 1761 jg 0x17d0
            if fl.g() { pc = 0x17d0; continue; }
            pc = 0x1763;
        }
        0x1763 => {
            // block 1763
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 1763 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1763 mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 1768 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 176e mov rdi,r14
            rdi = r14;
            // 1771 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 1773 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1776 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 177b xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 177d mov ecx,0x100000
            rcx = (0x100000u64) as u32 as u64;
            // 1782 mov r8d,0x14
            r8 = (0x14u64) as u32 as u64;
            // 1788 mov rdi,r14
            rdi = r14;
            // 178b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 178e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 1791 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1796 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1798 mov ecx,0x200000
            rcx = (0x200000u64) as u32 as u64;
            // 179d mov r8d,0x15
            r8 = (0x15u64) as u32 as u64;
            // 17a3 jmp 0x1810
            pc = 0x1810; continue;
        }
        0x17a5 => {
            // block 17a5
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 17a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17a5 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            pc = 0x17aa;
        }
        0x17aa => {
            // block 17aa
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 17aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17aa cmp ebx,0x3
            fl = sub::<32>((rbx as u32 as u64), 0x3u64).1;
            // 17ad jg 0x17bc
            if fl.g() { pc = 0x17bc; continue; }
            pc = 0x17af;
        }
        0x17af => {
            // block 17af
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 17af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17af mov ecx,0x8
            rcx = (0x8u64) as u32 as u64;
            // 17b4 mov r8d,0x3
            r8 = (0x3u64) as u32 as u64;
            // 17ba jmp 0x17c7
            pc = 0x17c7; continue;
        }
        0x17bc => {
            // block 17bc
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 17bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17bc mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 17c1 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            pc = 0x17c7;
        }
        0x17c7 => {
            // block 17c7
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 17c7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17c7 mov rdi,r14
            rdi = r14;
            // 17ca call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17cc mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 17ce jmp 0x1816
            pc = 0x1816; continue;
        }
        0x17d0 => {
            // block 17d0
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 17d0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 17d0 mov ecx,0x1000
            rcx = (0x1000u64) as u32 as u64;
            // 17d5 mov r8d,0xc
            r8 = (0xcu64) as u32 as u64;
            // 17db mov rdi,r14
            rdi = r14;
            // 17de call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17e0 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 17e3 mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 17e8 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 17ea mov ecx,0x400000
            rcx = (0x400000u64) as u32 as u64;
            // 17ef mov r8d,0x16
            r8 = (0x16u64) as u32 as u64;
            // 17f5 mov rdi,r14
            rdi = r14;
            // 17f8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 17fb mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 17fe mov esi,0x10b
            rsi = (0x10bu64) as u32 as u64;
            // 1803 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 1805 mov ecx,0x800000
            rcx = (0x800000u64) as u32 as u64;
            // 180a mov r8d,0x17
            r8 = (0x17u64) as u32 as u64;
            pc = 0x1810;
        }
        0x1810 => {
            // block 1810
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 1810 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1810 mov rdi,r14
            rdi = r14;
            // 1813 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x1816;
        }
        0x1816 => {
            // block 1816
            #[cfg(feature = "trace")] eprintln!("f_00001710 pc 1816 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 1816 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 181a pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 181b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 181d pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 181e ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
