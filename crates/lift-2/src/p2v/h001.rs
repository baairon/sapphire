//! Mechanically lifted functions (scripts/lift_run.py), module h001. Each one is checked against the original
//! in the world harness (tests/world_h001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

/// parity: P2v @0x43a98 size=70
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00043a98<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x43a98;
    loop {
        match pc {
        0x43a98 => {
            // block 43a98
            #[cfg(feature = "trace")] eprintln!("f_00043a98 pc 43a98 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43a98 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 43a99 mov rbp,rsp
            rbp = rsp;
            // 43a9c push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 43a9d push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 43a9e mov rbx,rdx
            rbx = rdx;
            // 43aa1 cmp esi,0x7
            fl = sub::<32>((rsi as u32 as u64), 0x7u64).1;
            // 43aa4 jle 0x43aaa
            if fl.le() { pc = 0x43aaa; continue; }
            pc = 0x43aa6;
        }
        0x43aa6 => {
            // block 43aa6
            #[cfg(feature = "trace")] eprintln!("f_00043a98 pc 43aa6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43aa6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 43aa8 jmp 0x43ad6
            pc = 0x43ad6; continue;
        }
        0x43aaa => {
            // block 43aaa
            #[cfg(feature = "trace")] eprintln!("f_00043a98 pc 43aaa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43aaa mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 43ab1 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 43ab4 mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 43ab6 lea rdx,[rip+0xbc903] # 0x1003c0
            rdx = m.env.data_va(84877);
            // 43abd mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 43ac0 lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 43ac4 mov ecx,0x1
            rcx = (0x1u64) as u32 as u64;
            // 43ac9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 43acc call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 43acf cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 43ad3 setne BYTE PTR [rbx]
            m.w8(rbx, ((fl.ne() as u64)) as u8);
            pc = 0x43ad6;
        }
        0x43ad6 => {
            // block 43ad6
            #[cfg(feature = "trace")] eprintln!("f_00043a98 pc 43ad6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 43ad6 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 43ada pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 43adb pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 43adc ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x6dbe size=376
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00006dbe<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x6dbe;
    loop {
        match pc {
        0x6dbe => {
            // block 6dbe
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6dbe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6dbe push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 6dbf mov rbp,rsp
            rbp = rsp;
            // 6dc2 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 6dc4 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 6dc6 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 6dc8 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 6dca push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 6dcb push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 6dcc mov ebx,ecx
            rbx = ((rcx as u32 as u64)) as u32 as u64;
            // 6dce mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // 6dd1 mov r13d,esi
            r13 = ((rsi as u32 as u64)) as u32 as u64;
            // 6dd4 mov r12,rdi
            r12 = rdi;
            // 6dd7 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6ddb mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 6ddd call QWORD PTR [rax+0x1d30]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6de3 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6de5 jne 0x6e8e
            if fl.ne() { pc = 0x6e8e; continue; }
            pc = 0x6deb;
        }
        0x6deb => {
            // block 6deb
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6deb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6deb test r13d,r13d
            fl = logic::<32>((r13 as u32 as u64) & (r13 as u32 as u64));
            // 6dee mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6df2 mov rax,QWORD PTR [rax+0x30]
            rax = m.r64(rax.wrapping_add(0x30)) as u64;
            // 6df6 movzx ebx,bx
            rbx = ((rbx as u16 as u64)) as u32 as u64;
            // 6df9 je 0x6ea3
            if fl.e() { pc = 0x6ea3; continue; }
            pc = 0x6dff;
        }
        0x6dff => {
            // block 6dff
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6dff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6dff lea rcx,[rip+0xf85ea] # 0xff3f0
            rcx = m.env.data_va(80829);
            // 6e06 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6e09 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6e0c mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6e11 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6e14 mov rdi,r12
            rdi = r12;
            // 6e17 mov edx,r13d
            rdx = ((r13 as u32 as u64)) as u32 as u64;
            // 6e1a call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6e1c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6e1e je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6e20;
        }
        0x6e20 => {
            // block 6e20
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e20 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e20 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6e24 lea rcx,[rip+0xf85f5] # 0xff420
            rcx = m.env.data_va(80877);
            // 6e2b mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6e2e xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6e31 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6e36 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6e39 mov rdi,r12
            rdi = r12;
            // 6e3c mov edx,r15d
            rdx = ((r15 as u32 as u64)) as u32 as u64;
            // 6e3f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6e42 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6e44 je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6e46;
        }
        0x6e46 => {
            // block 6e46
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e46 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e46 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6e4a lea rcx,[rip+0xf856f] # 0xff3c0
            rcx = m.env.data_va(80781);
            // 6e51 mov ebx,DWORD PTR [rcx+rbx*4]
            rbx = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6e54 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 6e59 mov ecx,0x8000000
            rcx = (0x8000000u64) as u32 as u64;
            // 6e5e mov r8d,0x1b
            r8 = (0x1bu64) as u32 as u64;
            // 6e64 mov rdi,r12
            rdi = r12;
            // 6e67 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 6e69 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6e6c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6e6e je 0x6e8e
            if fl.e() { pc = 0x6e8e; continue; }
            pc = 0x6e70;
        }
        0x6e70 => {
            // block 6e70
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e70 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6e74 mov edx,0x1
            rdx = (0x1u64) as u32 as u64;
            // 6e79 mov ecx,0x4000000
            rcx = (0x4000000u64) as u32 as u64;
            // 6e7e mov r8d,0x1a
            r8 = (0x1au64) as u32 as u64;
            // 6e84 mov rdi,r12
            rdi = r12;
            // 6e87 mov esi,ebx
            rsi = ((rbx as u32 as u64)) as u32 as u64;
            // 6e89 jmp 0x6f2b
            pc = 0x6f2b; continue;
        }
        0x6e8e => {
            // block 6e8e
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e8e xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            pc = 0x6e91;
        }
        0x6e91 => {
            // block 6e91
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6e91 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6e91 mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // 6e94 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 6e98 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 6e99 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 6e9b pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 6e9d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 6e9f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 6ea1 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6ea2 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6ea3 => {
            // block 6ea3
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6ea3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6ea3 lea rcx,[rip+0xf8516] # 0xff3c0
            rcx = m.env.data_va(80781);
            // 6eaa mov r15d,DWORD PTR [rcx+rbx*4]
            r15 = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6eae xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6eb1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6eb3 mov ecx,0x8000000
            rcx = (0x8000000u64) as u32 as u64;
            // 6eb8 mov r8d,0x1b
            r8 = (0x1bu64) as u32 as u64;
            // 6ebe mov rdi,r12
            rdi = r12;
            // 6ec1 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 6ec4 call rax
            { let r = { let t = rax; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6ec6 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6ec8 je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6eca;
        }
        0x6eca => {
            // block 6eca
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6eca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6eca mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6ece xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6ed1 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6ed3 mov ecx,0x4000000
            rcx = (0x4000000u64) as u32 as u64;
            // 6ed8 mov r8d,0x1a
            r8 = (0x1au64) as u32 as u64;
            // 6ede mov rdi,r12
            rdi = r12;
            // 6ee1 mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // 6ee4 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6ee7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6ee9 je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6eeb;
        }
        0x6eeb => {
            // block 6eeb
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6eeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6eeb mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6eef lea rcx,[rip+0xf84fa] # 0xff3f0
            rcx = m.env.data_va(80829);
            // 6ef6 mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6ef9 xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 6efc xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6efe mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6f03 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6f06 mov rdi,r12
            rdi = r12;
            // 6f09 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6f0c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 6f0e je 0x6e91
            if fl.e() { pc = 0x6e91; continue; }
            pc = 0x6f10;
        }
        0x6f10 => {
            // block 6f10
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6f10 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6f10 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 6f14 lea rcx,[rip+0xf8505] # 0xff420
            rcx = m.env.data_va(80877);
            // 6f1b mov esi,DWORD PTR [rcx+rbx*4]
            rsi = (m.r32(rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 6f1e xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 6f20 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 6f25 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 6f28 mov rdi,r12
            rdi = r12;
            pc = 0x6f2b;
        }
        0x6f2b => {
            // block 6f2b
            #[cfg(feature = "trace")] eprintln!("f_00006dbe pc 6f2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6f2b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 6f2e mov r14b,al
            r14 = (r14 & !0xff) | (((rax as u8 as u64)) as u8 as u64);
            // 6f31 jmp 0x6e91
            pc = 0x6e91; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x28ea8 size=88
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00028ea8<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x28ea8;
    loop {
        match pc {
        0x28ea8 => {
            // block 28ea8
            #[cfg(feature = "trace")] eprintln!("f_00028ea8 pc 28ea8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ea8 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 28ea9 mov rbp,rsp
            rbp = rsp;
            // 28eac push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 28eae push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 28eb0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 28eb1 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 28eb2 mov r14,rdx
            r14 = rdx;
            // 28eb5 mov r15,rsi
            r15 = rsi;
            // 28eb8 mov rbx,rdi
            rbx = rdi;
            // 28ebb mov eax,DWORD PTR [rbx+0xc]
            rax = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // 28ebe test al,0x1
            fl = logic::<8>((rax as u8 as u64) & 0x1u64);
            // 28ec0 je 0x28eec
            if fl.e() { pc = 0x28eec; continue; }
            pc = 0x28ec2;
        }
        0x28ec2 => {
            // block 28ec2
            #[cfg(feature = "trace")] eprintln!("f_00028ea8 pc 28ec2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ec2 mov rdi,QWORD PTR [rbx]
            rdi = m.r64(rbx) as u64;
            // 28ec5 test rdi,rdi
            fl = logic::<64>(rdi & rdi);
            // 28ec8 je 0x28ed8
            if fl.e() { pc = 0x28ed8; continue; }
            pc = 0x28eca;
        }
        0x28eca => {
            // block 28eca
            #[cfg(feature = "trace")] eprintln!("f_00028ea8 pc 28eca rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28eca cmp DWORD PTR [rbx+0x8],0x0
            fl = sub::<32>(m.r32(rbx.wrapping_add(0x8)) as u64, 0x0u64).1;
            // 28ece je 0x28ed8
            if fl.e() { pc = 0x28ed8; continue; }
            pc = 0x28ed0;
        }
        0x28ed0 => {
            // block 28ed0
            #[cfg(feature = "trace")] eprintln!("f_00028ea8 pc 28ed0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ed0 call 0xec134
            { let r = { let t = m.env.stub_va(0x901); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 28ed5 mov eax,DWORD PTR [rbx+0xc]
            rax = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            pc = 0x28ed8;
        }
        0x28ed8 => {
            // block 28ed8
            #[cfg(feature = "trace")] eprintln!("f_00028ea8 pc 28ed8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28ed8 mov QWORD PTR [rbx],0x0
            m.w64(rbx, (0x0u64) as u64);
            // 28edf mov DWORD PTR [rbx+0x8],0x0
            m.w32(rbx.wrapping_add(0x8), (0x0u64) as u32);
            // 28ee6 and eax,0xfffffffe
            { let r = (rax as u32 as u64) & 0xfffffffeu64; let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 28ee9 mov DWORD PTR [rbx+0xc],eax
            m.w32(rbx.wrapping_add(0xc), ((rax as u32 as u64)) as u32);
            pc = 0x28eec;
        }
        0x28eec => {
            // block 28eec
            #[cfg(feature = "trace")] eprintln!("f_00028ea8 pc 28eec rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 28eec mov QWORD PTR [rbx],r15
            m.w64(rbx, (r15) as u64);
            // 28eef mov DWORD PTR [rbx+0x8],r14d
            m.w32(rbx.wrapping_add(0x8), ((r14 as u32 as u64)) as u32);
            // 28ef3 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 28ef5 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 28ef9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 28efa pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 28efc pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 28efe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 28eff ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x9b4 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000009b4<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x9b4;
    loop {
        match pc {
        0x9b4 => {
            // block 9b4
            #[cfg(feature = "trace")] eprintln!("f_000009b4 pc 9b4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9b4 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 9b5 mov rbp,rsp
            rbp = rsp;
            // 9b8 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 9ba push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 9bc push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 9be push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 9c0 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 9c1 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 9c5 mov QWORD PTR [rbp-0x38],r9
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r9) as u64);
            // 9c9 mov r15,r8
            r15 = r8;
            // 9cc mov r12,rcx
            r12 = rcx;
            // 9cf mov r13,rdx
            r13 = rdx;
            // 9d2 mov rbx,rsi
            rbx = rsi;
            // 9d5 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 9d8 lea rsi,[rbp-0x2c]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd4);
            // 9dc xor r14d,r14d
            { let r = (r14 as u32 as u64) ^ (r14 as u32 as u64); let f = logic::<32>(r);
              r14 = (r) as u32 as u64;
              fl = f; }
            // 9df xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 9e1 call QWORD PTR [rax+0x3b0]
            { let r = { let t = m.r64(rax.wrapping_add(0x3b0)) as u64; m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 9e7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 9e9 je 0xa1f
            if fl.e() { pc = 0xa1f; continue; }
            pc = 0x9eb;
        }
        0x9eb => {
            // block 9eb
            #[cfg(feature = "trace")] eprintln!("f_000009b4 pc 9eb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 9eb mov rax,QWORD PTR [rbp+0x10]
            rax = m.r64(rbp.wrapping_add(0x10)) as u64;
            // 9ef mov DWORD PTR [rbx],0x1000
            m.w32(rbx, (0x1000u64) as u32);
            // 9f5 mov DWORD PTR [r13+0x0],0x800
            m.w32(r13, (0x800u64) as u32);
            // 9fd mov DWORD PTR [r12],0x1800
            m.w32(r12, (0x1800u64) as u32);
            // a05 mov DWORD PTR [r15],0x800
            m.w32(r15, (0x800u64) as u32);
            // a0c mov rcx,QWORD PTR [rbp-0x38]
            rcx = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            // a10 mov DWORD PTR [rcx],0x400
            m.w32(rcx, (0x400u64) as u32);
            // a16 mov DWORD PTR [rax],0xc00
            m.w32(rax, (0xc00u64) as u32);
            // a1c mov r14b,0x1
            r14 = (r14 & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0xa1f;
        }
        0xa1f => {
            // block a1f
            #[cfg(feature = "trace")] eprintln!("f_000009b4 pc a1f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // a1f mov al,r14b
            rax = (rax & !0xff) | (((r14 as u8 as u64)) as u8 as u64);
            // a22 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // a26 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // a27 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // a29 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // a2b pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // a2d pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // a2f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // a30 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x7d3c size=644
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00007d3c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut pc: u32 = 0x7d3c;
    loop {
        match pc {
        0x7d3c => {
            // block 7d3c
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d3c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 7d3d mov rbp,rsp
            rbp = rsp;
            // 7d40 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 7d42 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 7d44 push r13
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r13);
            // 7d46 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 7d48 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 7d49 sub rsp,0x18
            { let (r, f) = sub::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7d4d mov r12,rsi
            r12 = rsi;
            // 7d50 mov r13,rdi
            r13 = rdi;
            // 7d53 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7d57 mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 7d5c sub rsi,rax
            { let (r, f) = sub::<64>(rsi, rax);
              rsi = r;
              fl = f; }
            // 7d5f sar rsi,0x3
            { let (r, f) = sar::<64>(rsi, 0x3u64, fl); rsi = r; fl = f; }
            // 7d63 cmp rsi,0x400
            fl = sub::<64>(rsi, 0x400u64).1;
            // 7d6a jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7d70;
        }
        0x7d70 => {
            // block 7d70
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d70 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d70 mov rsi,QWORD PTR [rdx+0x8]
            rsi = m.r64(rdx.wrapping_add(0x8)) as u64;
            // 7d74 sub rsi,QWORD PTR [rdx]
            { let (r, f) = sub::<64>(rsi, m.r64(rdx) as u64);
              rsi = r;
              fl = f; }
            // 7d77 sar rsi,0x3
            { let (r, f) = sar::<64>(rsi, 0x3u64, fl); rsi = r; fl = f; }
            // 7d7b cmp rsi,0x400
            fl = sub::<64>(rsi, 0x400u64).1;
            // 7d82 jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7d88;
        }
        0x7d88 => {
            // block 7d88
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7d88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7d88 mov rsi,rcx
            rsi = rcx;
            // 7d8b mov rcx,QWORD PTR [rsi+0x8]
            rcx = m.r64(rsi.wrapping_add(0x8)) as u64;
            // 7d8f sub rcx,QWORD PTR [rsi]
            { let (r, f) = sub::<64>(rcx, m.r64(rsi) as u64);
              rcx = r;
              fl = f; }
            // 7d92 sar rcx,0x3
            { let (r, f) = sar::<64>(rcx, 0x3u64, fl); rcx = r; fl = f; }
            // 7d96 cmp rcx,0x400
            fl = sub::<64>(rcx, 0x400u64).1;
            // 7d9d jb 0x7fae
            if fl.b() { pc = 0x7fae; continue; }
            pc = 0x7da3;
        }
        0x7da3 => {
            // block 7da3
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7da3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7da3 mov QWORD PTR [rbp-0x30],rdx
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0), (rdx) as u64);
            // 7da7 mov QWORD PTR [rbp-0x38],rsi
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (rsi) as u64);
            // 7dab mov ebx,0x8
            rbx = (0x8u64) as u32 as u64;
            // 7db0 mov r14d,0x1
            r14 = (0x1u64) as u32 as u64;
            // 7db6 movsd xmm1,QWORD PTR [rip+0xf6b1a] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7dbe mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            // 7dc4 jmp 0x7dd9
            pc = 0x7dd9; continue;
        }
        0x7dc6 => {
            // block 7dc6
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dc6 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7dca add rbx,0x10
            { let (r, f) = add::<64>(rbx, 0x10u64);
              rbx = r;
              fl = f; }
            // 7dce inc r14
            { let (r, f) = inc::<64>(r14, fl); r14 = r; fl = f; }
            // 7dd1 movsd xmm1,QWORD PTR [rip+0xf6aff] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            pc = 0x7dd9;
        }
        0x7dd9 => {
            // block 7dd9
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dd9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dd9 movsd xmm0,QWORD PTR [rax+rbx*1-0x8]
            x0 = m.r64(rax.wrapping_add(rbx).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7ddf addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7de3 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7de8 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7dea mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7def js 0x7dfd
            if fl.s() { pc = 0x7dfd; continue; }
            pc = 0x7df1;
        }
        0x7df1 => {
            // block 7df1
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7df1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7df1 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7df7 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7dfb mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7dfd;
        }
        0x7dfd => {
            // block 7dfd
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7dfd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7dfd lea rsi,[r14+0x1ff]
            rsi = r14.wrapping_add(0x1ff);
            // 7e04 movsd xmm0,QWORD PTR [rax+rbx*1]
            x0 = m.r64(rax.wrapping_add(rbx)) as u128;
            // 7e09 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7e0d cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7e12 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7e14 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7e16 js 0x7e24
            if fl.s() { pc = 0x7e24; continue; }
            pc = 0x7e18;
        }
        0x7e18 => {
            // block 7e18
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e18 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7e1e cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7e22 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7e24;
        }
        0x7e24 => {
            // block 7e24
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e24 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e24 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7e27 shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7e2a add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7e2c mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7e30 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7e35 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7e38 mov rdi,r13
            rdi = r13;
            // 7e3b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7e3e cmp r14,0x1ff
            fl = sub::<64>(r14, 0x1ffu64).1;
            // 7e45 ja 0x7e4f
            if fl.a() { pc = 0x7e4f; continue; }
            pc = 0x7e47;
        }
        0x7e47 => {
            // block 7e47
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e47 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7e49 jne 0x7dc6
            if fl.ne() { pc = 0x7dc6; continue; }
            pc = 0x7e4f;
        }
        0x7e4f => {
            // block 7e4f
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e4f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e4f mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // 7e55 mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 7e5a test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7e5c je 0x7efc
            if fl.e() { pc = 0x7efc; continue; }
            pc = 0x7e62;
        }
        0x7e62 => {
            // block 7e62
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e62 mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            // 7e68 mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7e6c;
        }
        0x7e6c => {
            // block 7e6c
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e6c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e6c mov rax,QWORD PTR [rbp-0x30]
            rax = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd0)) as u64;
            // 7e70 mov rax,QWORD PTR [rax]
            rax = m.r64(rax) as u64;
            // 7e73 movsd xmm0,QWORD PTR [rax+r14*1-0x8]
            x0 = m.r64(rax.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7e7a movsd xmm1,QWORD PTR [rip+0xf6a56] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7e82 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7e86 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7e8b test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7e8d mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7e92 js 0x7ea0
            if fl.s() { pc = 0x7ea0; continue; }
            pc = 0x7e94;
        }
        0x7e94 => {
            // block 7e94
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7e94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7e94 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7e9a cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7e9e mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7ea0;
        }
        0x7ea0 => {
            // block 7ea0
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ea0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ea0 lea rsi,[rbx+0x3ff]
            rsi = rbx.wrapping_add(0x3ff);
            // 7ea7 movsd xmm0,QWORD PTR [rax+r14*1]
            x0 = m.r64(rax.wrapping_add(r14)) as u128;
            // 7ead addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7eb1 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7eb6 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7eb8 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7eba js 0x7ec8
            if fl.s() { pc = 0x7ec8; continue; }
            pc = 0x7ebc;
        }
        0x7ebc => {
            // block 7ebc
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ebc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ebc cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7ec2 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7ec6 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7ec8;
        }
        0x7ec8 => {
            // block 7ec8
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7ec8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7ec8 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7ecb shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7ece add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7ed0 mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7ed4 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7ed9 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7edc mov rdi,r13
            rdi = r13;
            // 7edf call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7ee2 cmp rbx,0x1ff
            fl = sub::<64>(rbx, 0x1ffu64).1;
            // 7ee9 ja 0x7f00
            if fl.a() { pc = 0x7f00; continue; }
            pc = 0x7eeb;
        }
        0x7eeb => {
            // block 7eeb
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7eeb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7eeb add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // 7eef inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7ef2 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7ef4 jne 0x7e6c
            if fl.ne() { pc = 0x7e6c; continue; }
            pc = 0x7efa;
        }
        0x7efa => {
            // block 7efa
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7efa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7efa jmp 0x7f00
            pc = 0x7f00; continue;
        }
        0x7efc => {
            // block 7efc
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7efc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7efc mov r12,QWORD PTR [rbp-0x38]
            r12 = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8)) as u64;
            pc = 0x7f00;
        }
        0x7f00 => {
            // block 7f00
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f00 movsd xmm1,QWORD PTR [rip+0xf69d0] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7f08 mov r14d,0x8
            r14 = (0x8u64) as u32 as u64;
            // 7f0e mov ebx,0x1
            rbx = (0x1u64) as u32 as u64;
            // 7f13 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7f15 je 0x7fae
            if fl.e() { pc = 0x7fae; continue; }
            pc = 0x7f1b;
        }
        0x7f1b => {
            // block 7f1b
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f1b mov r15d,0x3ff
            r15 = (0x3ffu64) as u32 as u64;
            pc = 0x7f21;
        }
        0x7f21 => {
            // block 7f21
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f21 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f21 mov rax,QWORD PTR [r12]
            rax = m.r64(r12) as u64;
            // 7f25 movsd xmm0,QWORD PTR [rax+r14*1-0x8]
            x0 = m.r64(rax.wrapping_add(r14).wrapping_add(0xffff_ffff_ffff_fff8)) as u128;
            // 7f2c addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7f30 cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7f35 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7f37 mov edx,0x0
            rdx = (0x0u64) as u32 as u64;
            // 7f3c js 0x7f4a
            if fl.s() { pc = 0x7f4a; continue; }
            pc = 0x7f3e;
        }
        0x7f3e => {
            // block 7f3e
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f3e cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7f44 cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7f48 mov edx,ecx
            rdx = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7f4a;
        }
        0x7f4a => {
            // block 7f4a
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f4a lea rsi,[rbx+0x5ff]
            rsi = rbx.wrapping_add(0x5ff);
            // 7f51 movsd xmm0,QWORD PTR [rax+r14*1]
            x0 = m.r64(rax.wrapping_add(r14)) as u128;
            // 7f57 addsd xmm0,xmm1
            x0 = set_lo64(x0, (f64_of(x0) + f64_of(x1)).to_bits());
            // 7f5b cvttsd2si rcx,xmm0
            rcx = cvtt64(f64_of(x0)) as u64;
            // 7f60 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 7f62 test ecx,ecx
            fl = logic::<32>((rcx as u32 as u64) & (rcx as u32 as u64));
            // 7f64 js 0x7f72
            if fl.s() { pc = 0x7f72; continue; }
            pc = 0x7f66;
        }
        0x7f66 => {
            // block 7f66
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f66 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f66 cmp ecx,0x3ff
            fl = sub::<32>((rcx as u32 as u64), 0x3ffu64).1;
            // 7f6c cmovg ecx,r15d
            if fl.g() { rcx = ((r15 as u32 as u64)) as u32 as u64; } else { rcx = rcx as u32 as u64; }
            // 7f70 mov eax,ecx
            rax = ((rcx as u32 as u64)) as u32 as u64;
            pc = 0x7f72;
        }
        0x7f72 => {
            // block 7f72
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f72 shl eax,0x16
            { let (r, f) = shl::<32>((rax as u32 as u64), 0x16u64, fl); rax = (r) as u32 as u64; fl = f; }
            // 7f75 shl edx,0x6
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0x6u64, fl); rdx = (r) as u32 as u64; fl = f; }
            // 7f78 add edx,eax
            { let (r, f) = add::<32>((rdx as u32 as u64), (rax as u32 as u64));
              rdx = (r) as u32 as u64;
              fl = f; }
            // 7f7a mov rax,QWORD PTR [r13+0x0]
            rax = m.r64(r13) as u64;
            // 7f7e mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 7f83 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 7f86 mov rdi,r13
            rdi = r13;
            // 7f89 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON; x1 = POISON as u128; }
            // 7f8c cmp rbx,0x1ff
            fl = sub::<64>(rbx, 0x1ffu64).1;
            // 7f93 ja 0x7fb0
            if fl.a() { pc = 0x7fb0; continue; }
            pc = 0x7f95;
        }
        0x7f95 => {
            // block 7f95
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7f95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7f95 add r14,0x10
            { let (r, f) = add::<64>(r14, 0x10u64);
              r14 = r;
              fl = f; }
            // 7f99 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 7f9c test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 7f9e movsd xmm1,QWORD PTR [rip+0xf6932] # 0xfe8d8
            x1 = m.r64(m.env.data_va(77989)) as u128;
            // 7fa6 jne 0x7f21
            if fl.ne() { pc = 0x7f21; continue; }
            pc = 0x7fac;
        }
        0x7fac => {
            // block 7fac
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fac jmp 0x7fb0
            pc = 0x7fb0; continue;
        }
        0x7fae => {
            // block 7fae
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fae xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x7fb0;
        }
        0x7fb0 => {
            // block 7fb0
            #[cfg(feature = "trace")] eprintln!("f_00007d3c pc 7fb0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 7fb0 add rsp,0x18
            { let (r, f) = add::<64>(rsp, 0x18u64);
              rsp = r;
              fl = f; }
            // 7fb4 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 7fb5 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 7fb7 pop r13
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r13 = v; }
            // 7fb9 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 7fbb pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 7fbd pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 7fbe ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x14d94 size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00014d94<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x14d94;
    loop {
        match pc {
        0x14d94 => {
            // block 14d94
            #[cfg(feature = "trace")] eprintln!("f_00014d94 pc 14d94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 14d94 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 14d95 mov rbp,rsp
            rbp = rsp;
            // 14d98 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 14d9a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 14d9b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 14d9e mov rax,QWORD PTR [rax-0x18]
            rax = m.r64(rax.wrapping_add(0xffff_ffff_ffff_ffe8)) as u64;
            // 14da2 lea rbx,[rdi+rax*1]
            rbx = rdi.wrapping_add(rax);
            // 14da6 mov rcx,QWORD PTR [rip+0x12b5f3] # 0x1403a0
            rcx = m.r64(m.env.data_va(346989)) as u64;
            // 14dad lea rdx,[rcx+0x18]
            rdx = rcx.wrapping_add(0x18);
            // 14db1 mov QWORD PTR [rdi+rax*1],rdx
            m.w64(rdi.wrapping_add(rax), (rdx) as u64);
            // 14db5 lea r14,[rdi+rax*1+0x1a8]
            r14 = rdi.wrapping_add(rax).wrapping_add(0x1a8);
            // 14dbd add rcx,0x40
            { let (r, f) = add::<64>(rcx, 0x40u64);
              rcx = r;
              fl = f; }
            // 14dc1 mov QWORD PTR [rdi+rax*1+0x1a8],rcx
            m.w64(rdi.wrapping_add(rax).wrapping_add(0x1a8), (rcx) as u64);
            // 14dc9 lea rdi,[rdi+rax*1+0x10]
            rdi = rdi.wrapping_add(rax).wrapping_add(0x10);
            // 14dce call 0xeb912
            { let r = { let t = m.env.stub_va(0xdf); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14dd3 mov rsi,QWORD PTR [rip+0x12b416] # 0x1401f0
            rsi = m.r64(m.env.data_va(346557)) as u64;
            // 14dda add rsi,0x8
            { let (r, f) = add::<64>(rsi, 0x8u64);
              rsi = r;
              fl = f; }
            // 14dde mov rdi,rbx
            rdi = rbx;
            // 14de1 call 0xec056
            { let r = { let t = m.env.stub_va(0x823); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 14de6 mov rdi,r14
            rdi = r14;
            // 14de9 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 14dea pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 14dec pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 14ded jmp 0xec116
            { let r = { let t = m.env.stub_va(0x8e3); m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x32210 size=84
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00032210<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x32210;
    loop {
        match pc {
        0x32210 => {
            // block 32210
            #[cfg(feature = "trace")] eprintln!("f_00032210 pc 32210 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32210 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 32211 mov rbp,rsp
            rbp = rsp;
            // 32214 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 32216 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 32218 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 32219 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 3221a mov r14,rsi
            r14 = rsi;
            // 3221d mov r15,rdi
            r15 = rdi;
            // 32220 mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 32225 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3222a mov rbx,rax
            rbx = rax;
            // 3222d mov eax,DWORD PTR [r14]
            rax = (m.r32(r14) as u64) as u32 as u64;
            // 32230 mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 32233 mov rdi,r15
            rdi = r15;
            // 32236 mov rsi,rbx
            rsi = rbx;
            // 32239 call 0xebb28
            { let r = { let t = m.env.stub_va(0x2f5); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 3223e mov r14,rax
            r14 = rax;
            // 32241 mov r15b,dl
            r15 = (r15 & !0xff) | (((rdx as u8 as u64)) as u8 as u64);
            // 32244 test r15b,0x1
            fl = logic::<8>((r15 as u8 as u64) & 0x1u64);
            // 32248 jne 0x32252
            if fl.ne() { pc = 0x32252; continue; }
            pc = 0x3224a;
        }
        0x3224a => {
            // block 3224a
            #[cfg(feature = "trace")] eprintln!("f_00032210 pc 3224a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 3224a mov rdi,rbx
            rdi = rbx;
            // 3224d call 0xec13a
            { let r = { let t = m.env.stub_va(0x907); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x32252;
        }
        0x32252 => {
            // block 32252
            #[cfg(feature = "trace")] eprintln!("f_00032210 pc 32252 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 32252 mov rax,r14
            rax = r14;
            // 32255 mov dl,r15b
            rdx = (rdx & !0xff) | (((r15 as u8 as u64)) as u8 as u64);
            // 32258 add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 3225c pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 3225d pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 3225f pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 32261 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 32262 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x443d2 size=48
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000443d2<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x443d2;
    loop {
        match pc {
        0x443d2 => {
            // block 443d2
            #[cfg(feature = "trace")] eprintln!("f_000443d2 pc 443d2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 443d2 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 443d3 mov rbp,rsp
            rbp = rsp;
            // 443d6 lea edx,[rcx+rdx*4]
            rdx = (rcx.wrapping_add(rdx.wrapping_mul(4))) as u32 as u64;
            // 443d9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 443dc mov r9,QWORD PTR [rax+0x30]
            r9 = m.r64(rax.wrapping_add(0x30)) as u64;
            // 443e0 mov eax,esi
            rax = ((rsi as u32 as u64)) as u32 as u64;
            // 443e2 lea rcx,[rip+0xbc2a7] # 0x100690
            rcx = m.env.data_va(85597);
            // 443e9 mov r8d,DWORD PTR [rcx+rax*4]
            r8 = (m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u32 as u64;
            // 443ed mov eax,0xf
            rax = (0xfu64) as u32 as u64;
            // 443f2 mov cl,r8b
            rcx = (rcx & !0xff) | (((r8 as u8 as u64)) as u8 as u64);
            // 443f5 shl eax,cl
            { let (r, f) = shl::<32>((rax as u32 as u64), (rcx as u8 as u64), fl); rax = (r) as u32 as u64; fl = f; }
            // 443f7 mov esi,0xbe
            rsi = (0xbeu64) as u32 as u64;
            // 443fc mov ecx,eax
            rcx = ((rax as u32 as u64)) as u32 as u64;
            // 443fe pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 443ff jmp r9
            { let r = { let t = r9; m.call(t, &[rdi, rsi, rdx, rcx, r8, r9], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x633ff size=789
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000633ff<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x633ff;
    loop {
        match pc {
        0x633ff => {
            // block 633ff
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 633ff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 633ff push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 63400 mov rbp,rsp
            rbp = rsp;
            // 63403 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 63404 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 63405 mov rbx,rdi
            rbx = rdi;
            // 63408 cmp esi,0x106344ff
            fl = sub::<32>((rsi as u32 as u64), 0x106344ffu64).1;
            // 6340e jg 0x63438
            if fl.g() { pc = 0x63438; continue; }
            pc = 0x63410;
        }
        0x63410 => {
            // block 63410
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63410 cmp esi,0x105183ff
            fl = sub::<32>((rsi as u32 as u64), 0x105183ffu64).1;
            // 63416 jg 0x63461
            if fl.g() { pc = 0x63461; continue; }
            pc = 0x63418;
        }
        0x63418 => {
            // block 63418
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63418 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63418 cmp esi,0x104020ff
            fl = sub::<32>((rsi as u32 as u64), 0x104020ffu64).1;
            // 6341e jg 0x634aa
            if fl.g() { pc = 0x634aa; continue; }
            pc = 0x63424;
        }
        0x63424 => {
            // block 63424
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63424 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63424 cmp esi,0x102447ff
            fl = sub::<32>((rsi as u32 as u64), 0x102447ffu64).1;
            // 6342a jg 0x6353a
            if fl.g() { pc = 0x6353a; continue; }
            pc = 0x63430;
        }
        0x63430 => {
            // block 63430
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63430 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63430 cmp esi,0xffffffff
            fl = sub::<32>((rsi as u32 as u64), 0xffffffffu64).1;
            // 63433 jmp 0x636db
            pc = 0x636db; continue;
        }
        0x63438 => {
            // block 63438
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63438 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63438 cmp esi,0x107107ff
            fl = sub::<32>((rsi as u32 as u64), 0x107107ffu64).1;
            // 6343e jg 0x6348a
            if fl.g() { pc = 0x6348a; continue; }
            pc = 0x63440;
        }
        0x63440 => {
            // block 63440
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63440 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63440 cmp esi,0x106466ff
            fl = sub::<32>((rsi as u32 as u64), 0x106466ffu64).1;
            // 63446 jg 0x634ce
            if fl.g() { pc = 0x634ce; continue; }
            pc = 0x6344c;
        }
        0x6344c => {
            // block 6344c
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6344c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6344c cmp esi,0x10634500
            fl = sub::<32>((rsi as u32 as u64), 0x10634500u64).1;
            // 63452 jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x63458;
        }
        0x63458 => {
            // block 63458
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63458 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63458 lea rsi,[rip+0xce3d3] # 0x131832
            rsi = m.env.data_va(286719);
            // 6345f jmp 0x63480
            pc = 0x63480; continue;
        }
        0x63461 => {
            // block 63461
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63461 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63461 cmp esi,0x105381ff
            fl = sub::<32>((rsi as u32 as u64), 0x105381ffu64).1;
            // 63467 jg 0x634f9
            if fl.g() { pc = 0x634f9; continue; }
            pc = 0x6346d;
        }
        0x6346d => {
            // block 6346d
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6346d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6346d cmp esi,0x10518400
            fl = sub::<32>((rsi as u32 as u64), 0x10518400u64).1;
            // 63473 jne 0x6356a
            if fl.ne() { pc = 0x6356a; continue; }
            pc = 0x63479;
        }
        0x63479 => {
            // block 63479
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63479 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63479 lea rsi,[rip+0xce41a] # 0x13189a
            rsi = m.env.data_va(286823);
            pc = 0x63480;
        }
        0x63480 => {
            // block 63480
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63480 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63480 mov edx,0xf
            rdx = (0xfu64) as u32 as u64;
            // 63485 jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x6348a => {
            // block 6348a
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6348a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6348a cmp esi,0x10710800
            fl = sub::<32>((rsi as u32 as u64), 0x10710800u64).1;
            // 63490 je 0x63511
            if fl.e() { pc = 0x63511; continue; }
            pc = 0x63492;
        }
        0x63492 => {
            // block 63492
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63492 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63492 cmp esi,0x10710850
            fl = sub::<32>((rsi as u32 as u64), 0x10710850u64).1;
            // 63498 jne 0x63522
            if fl.ne() { pc = 0x63522; continue; }
            pc = 0x6349e;
        }
        0x6349e => {
            // block 6349e
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6349e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6349e lea rsi,[rip+0xce4a1] # 0x131946
            rsi = m.env.data_va(286995);
            // 634a5 jmp 0x63665
            pc = 0x63665; continue;
        }
        0x634aa => {
            // block 634aa
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 634aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 634aa cmp esi,0x104782ff
            fl = sub::<32>((rsi as u32 as u64), 0x104782ffu64).1;
            // 634b0 jg 0x63582
            if fl.g() { pc = 0x63582; continue; }
            pc = 0x634b6;
        }
        0x634b6 => {
            // block 634b6
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 634b6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 634b6 cmp esi,0x10402100
            fl = sub::<32>((rsi as u32 as u64), 0x10402100u64).1;
            // 634bc jne 0x635ae
            if fl.ne() { pc = 0x635ae; continue; }
            pc = 0x634c2;
        }
        0x634c2 => {
            // block 634c2
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 634c2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 634c2 lea rsi,[rip+0xce328] # 0x1317f1
            rsi = m.env.data_va(286654);
            // 634c9 jmp 0x63673
            pc = 0x63673; continue;
        }
        0x634ce => {
            // block 634ce
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 634ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 634ce add esi,0xef9b9900
            { let (r, f) = add::<32>((rsi as u32 as u64), 0xef9b9900u64);
              rsi = (r) as u32 as u64;
              fl = f; }
            // 634d4 cmp esi,0x6
            fl = sub::<32>((rsi as u32 as u64), 0x6u64).1;
            // 634d7 ja 0x636db
            if fl.a() { pc = 0x636db; continue; }
            pc = 0x634dd;
        }
        0x634dd => {
            // block 634dd
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 634dd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 634dd lea rax,[rip+0x214] # 0x636f8
            rax = m.env.code_va(0x636f8);
            // 634e4 movsxd rcx,DWORD PTR [rax+rsi*4]
            rcx = sx::<32>(m.r32(rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64;
            // 634e8 add rcx,rax
            { let (r, f) = add::<64>(rcx, rax);
              rcx = r;
              fl = f; }
            // 634eb jmp rcx
            pc = (rcx.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0x634ed => {
            // block 634ed
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 634ed rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 634ed lea rsi,[rip+0xce3c5] # 0x1318b9
            rsi = m.env.data_va(286854);
            // 634f4 jmp 0x63681
            pc = 0x63681; continue;
        }
        0x634f9 => {
            // block 634f9
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 634f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 634f9 cmp esi,0x10538200
            fl = sub::<32>((rsi as u32 as u64), 0x10538200u64).1;
            // 634ff jne 0x63596
            if fl.ne() { pc = 0x63596; continue; }
            pc = 0x63505;
        }
        0x63505 => {
            // block 63505
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63505 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63505 lea rsi,[rip+0xce318] # 0x131824
            rsi = m.env.data_va(286705);
            // 6350c jmp 0x63681
            pc = 0x63681; continue;
        }
        0x63511 => {
            // block 63511
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63511 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63511 lea rsi,[rip+0xce424] # 0x13193c
            rsi = m.env.data_va(286985);
            // 63518 mov edx,0x9
            rdx = (0x9u64) as u32 as u64;
            // 6351d jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x63522 => {
            // block 63522
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63522 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63522 cmp esi,0x10710851
            fl = sub::<32>((rsi as u32 as u64), 0x10710851u64).1;
            // 63528 jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x6352e;
        }
        0x6352e => {
            // block 6352e
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6352e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6352e lea rsi,[rip+0xce420] # 0x131955
            rsi = m.env.data_va(287010);
            // 63535 jmp 0x63665
            pc = 0x63665; continue;
        }
        0x6353a => {
            // block 6353a
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6353a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6353a cmp esi,0x1032294f
            fl = sub::<32>((rsi as u32 as u64), 0x1032294fu64).1;
            // 63540 jg 0x635c6
            if fl.g() { pc = 0x635c6; continue; }
            pc = 0x63546;
        }
        0x63546 => {
            // block 63546
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63546 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63546 cmp esi,0x102802ff
            fl = sub::<32>((rsi as u32 as u64), 0x102802ffu64).1;
            // 6354c jg 0x635f7
            if fl.g() { pc = 0x635f7; continue; }
            pc = 0x63552;
        }
        0x63552 => {
            // block 63552
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63552 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63552 cmp esi,0x10244800
            fl = sub::<32>((rsi as u32 as u64), 0x10244800u64).1;
            // 63558 jne 0x63688
            if fl.ne() { pc = 0x63688; continue; }
            pc = 0x6355e;
        }
        0x6355e => {
            // block 6355e
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6355e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6355e lea rsi,[rip+0xce26f] # 0x1317d4
            rsi = m.env.data_va(286625);
            // 63565 jmp 0x63480
            pc = 0x63480; continue;
        }
        0x6356a => {
            // block 6356a
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6356a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6356a cmp esi,0x10518450
            fl = sub::<32>((rsi as u32 as u64), 0x10518450u64).1;
            // 63570 jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x63576;
        }
        0x63576 => {
            // block 63576
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63576 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63576 lea rsi,[rip+0xce32d] # 0x1318aa
            rsi = m.env.data_va(286839);
            // 6357d jmp 0x63665
            pc = 0x63665; continue;
        }
        0x63582 => {
            // block 63582
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63582 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63582 cmp esi,0x10478300
            fl = sub::<32>((rsi as u32 as u64), 0x10478300u64).1;
            // 63588 jne 0x635e2
            if fl.ne() { pc = 0x635e2; continue; }
            pc = 0x6358a;
        }
        0x6358a => {
            // block 6358a
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6358a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6358a lea rsi,[rip+0xce2b1] # 0x131842
            rsi = m.env.data_va(286735);
            // 63591 jmp 0x636be
            pc = 0x636be; continue;
        }
        0x63596 => {
            // block 63596
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63596 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63596 cmp esi,0x10565400
            fl = sub::<32>((rsi as u32 as u64), 0x10565400u64).1;
            // 6359c jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x635a2;
        }
        0x635a2 => {
            // block 635a2
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635a2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635a2 lea rsi,[rip+0xce26d] # 0x131816
            rsi = m.env.data_va(286691);
            // 635a9 jmp 0x63681
            pc = 0x63681; continue;
        }
        0x635ae => {
            // block 635ae
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635ae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635ae cmp esi,0x10416000
            fl = sub::<32>((rsi as u32 as u64), 0x10416000u64).1;
            // 635b4 jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x635ba;
        }
        0x635ba => {
            // block 635ba
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635ba lea rsi,[rip+0xce36d] # 0x13192e
            rsi = m.env.data_va(286971);
            // 635c1 jmp 0x63681
            pc = 0x63681; continue;
        }
        0x635c6 => {
            // block 635c6
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635c6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635c6 cmp esi,0x10322950
            fl = sub::<32>((rsi as u32 as u64), 0x10322950u64).1;
            // 635cc je 0x6361b
            if fl.e() { pc = 0x6361b; continue; }
            pc = 0x635ce;
        }
        0x635ce => {
            // block 635ce
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635ce cmp esi,0x10352300
            fl = sub::<32>((rsi as u32 as u64), 0x10352300u64).1;
            // 635d4 jne 0x63627
            if fl.ne() { pc = 0x63627; continue; }
            pc = 0x635d6;
        }
        0x635d6 => {
            // block 635d6
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635d6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635d6 lea rsi,[rip+0xce334] # 0x131911
            rsi = m.env.data_va(286942);
            // 635dd jmp 0x63480
            pc = 0x63480; continue;
        }
        0x635e2 => {
            // block 635e2
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635e2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635e2 cmp esi,0x10478350
            fl = sub::<32>((rsi as u32 as u64), 0x10478350u64).1;
            // 635e8 jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x635ee;
        }
        0x635ee => {
            // block 635ee
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635ee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635ee lea rsi,[rip+0xce25a] # 0x13184f
            rsi = m.env.data_va(286748);
            // 635f5 jmp 0x63654
            pc = 0x63654; continue;
        }
        0x635f7 => {
            // block 635f7
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 635f7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 635f7 cmp esi,0x102946ff
            fl = sub::<32>((rsi as u32 as u64), 0x102946ffu64).1;
            // 635fd jg 0x63699
            if fl.g() { pc = 0x63699; continue; }
            pc = 0x63603;
        }
        0x63603 => {
            // block 63603
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63603 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63603 cmp esi,0x10280300
            fl = sub::<32>((rsi as u32 as u64), 0x10280300u64).1;
            // 63609 jne 0x636af
            if fl.ne() { pc = 0x636af; continue; }
            pc = 0x6360f;
        }
        0x6360f => {
            // block 6360f
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6360f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6360f lea rsi,[rip+0xce24a] # 0x131860
            rsi = m.env.data_va(286765);
            // 63616 jmp 0x636d4
            pc = 0x636d4; continue;
        }
        0x6361b => {
            // block 6361b
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6361b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6361b lea rsi,[rip+0xce26b] # 0x13188d
            rsi = m.env.data_va(286810);
            // 63622 jmp 0x636be
            pc = 0x636be; continue;
        }
        0x63627 => {
            // block 63627
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63627 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63627 cmp esi,0x10378800
            fl = sub::<32>((rsi as u32 as u64), 0x10378800u64).1;
            // 6362d jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x63633;
        }
        0x63633 => {
            // block 63633
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63633 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63633 lea rsi,[rip+0xce239] # 0x131873
            rsi = m.env.data_va(286784);
            // 6363a jmp 0x63681
            pc = 0x63681; continue;
        }
        0x6363c => {
            // block 6363c
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6363c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6363c lea rsi,[rip+0xce284] # 0x1318c7
            rsi = m.env.data_va(286868);
            // 63643 mov edx,0x17
            rdx = (0x17u64) as u32 as u64;
            // 63648 jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x6364d => {
            // block 6364d
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6364d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6364d lea rsi,[rip+0xce28b] # 0x1318df
            rsi = m.env.data_va(286892);
            pc = 0x63654;
        }
        0x63654 => {
            // block 63654
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63654 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63654 mov edx,0x10
            rdx = (0x10u64) as u32 as u64;
            // 63659 jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x6365e => {
            // block 6365e
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6365e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6365e lea rsi,[rip+0xce28b] # 0x1318f0
            rsi = m.env.data_va(286909);
            pc = 0x63665;
        }
        0x63665 => {
            // block 63665
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63665 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63665 mov edx,0xe
            rdx = (0xeu64) as u32 as u64;
            // 6366a jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x6366c => {
            // block 6366c
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6366c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6366c lea rsi,[rip+0xce28c] # 0x1318ff
            rsi = m.env.data_va(286924);
            pc = 0x63673;
        }
        0x63673 => {
            // block 63673
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63673 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63673 mov edx,0x11
            rdx = (0x11u64) as u32 as u64;
            // 63678 jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x6367a => {
            // block 6367a
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 6367a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6367a lea rsi,[rip+0xce2e3] # 0x131964
            rsi = m.env.data_va(287025);
            pc = 0x63681;
        }
        0x63681 => {
            // block 63681
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63681 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63681 mov edx,0xd
            rdx = (0xdu64) as u32 as u64;
            // 63686 jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x63688 => {
            // block 63688
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63688 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63688 cmp esi,0x10266400
            fl = sub::<32>((rsi as u32 as u64), 0x10266400u64).1;
            // 6368e jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x63690;
        }
        0x63690 => {
            // block 63690
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63690 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63690 lea rsi,[rip+0xce28a] # 0x131921
            rsi = m.env.data_va(286958);
            // 63697 jmp 0x636be
            pc = 0x636be; continue;
        }
        0x63699 => {
            // block 63699
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 63699 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 63699 cmp esi,0x10294700
            fl = sub::<32>((rsi as u32 as u64), 0x10294700u64).1;
            // 6369f jne 0x636c5
            if fl.ne() { pc = 0x636c5; continue; }
            pc = 0x636a1;
        }
        0x636a1 => {
            // block 636a1
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 636a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636a1 lea rsi,[rip+0xce1d9] # 0x131881
            rsi = m.env.data_va(286798);
            // 636a8 mov edx,0xb
            rdx = (0xbu64) as u32 as u64;
            // 636ad jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x636af => {
            // block 636af
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 636af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636af cmp esi,0x10293000
            fl = sub::<32>((rsi as u32 as u64), 0x10293000u64).1;
            // 636b5 jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x636b7;
        }
        0x636b7 => {
            // block 636b7
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 636b7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636b7 lea rsi,[rip+0xce126] # 0x1317e4
            rsi = m.env.data_va(286641);
            pc = 0x636be;
        }
        0x636be => {
            // block 636be
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 636be rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636be mov edx,0xc
            rdx = (0xcu64) as u32 as u64;
            // 636c3 jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x636c5 => {
            // block 636c5
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 636c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636c5 cmp esi,0x10294900
            fl = sub::<32>((rsi as u32 as u64), 0x10294900u64).1;
            // 636cb jne 0x636db
            if fl.ne() { pc = 0x636db; continue; }
            pc = 0x636cd;
        }
        0x636cd => {
            // block 636cd
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 636cd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636cd lea rsi,[rip+0xce12f] # 0x131803
            rsi = m.env.data_va(286672);
            pc = 0x636d4;
        }
        0x636d4 => {
            // block 636d4
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 636d4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636d4 mov edx,0x12
            rdx = (0x12u64) as u32 as u64;
            // 636d9 jmp 0x636e4
            pc = 0x636e4; continue;
        }
        0x636db => {
            // block 636db
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 636db rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636db lea rsi,[rip+0xd55e6] # 0x138cc8
            rsi = m.env.data_va(316565);
            // 636e2 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            pc = 0x636e4;
        }
        0x636e4 => {
            // block 636e4
            #[cfg(feature = "trace")] eprintln!("f_000633ff pc 636e4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 636e4 mov rdi,rbx
            rdi = rbx;
            // 636e7 call 0xebfe4
            { let r = { let t = m.env.stub_va(0x7b1); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 636ec mov rax,rbx
            rax = rbx;
            // 636ef add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 636f3 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 636f4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 636f5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc188 size=96
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000c188<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc188;
    loop {
        match pc {
        0xc188 => {
            // block c188
            #[cfg(feature = "trace")] eprintln!("f_0000c188 pc c188 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c188 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c189 mov rbp,rsp
            rbp = rsp;
            // c18c push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // c18e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // c190 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // c191 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // c192 mov r15d,edx
            r15 = ((rdx as u32 as u64)) as u32 as u64;
            // c195 mov r14,rsi
            r14 = rsi;
            // c198 mov rbx,rdi
            rbx = rdi;
            // c19b mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c19e mov esi,r15d
            rsi = ((r15 as u32 as u64)) as u32 as u64;
            // c1a1 call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c1a7 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // c1a9 je 0xc1af
            if fl.e() { pc = 0xc1af; continue; }
            pc = 0xc1ab;
        }
        0xc1ab => {
            // block c1ab
            #[cfg(feature = "trace")] eprintln!("f_0000c188 pc c1ab rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1ab xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // c1ad jmp 0xc1dc
            pc = 0xc1dc; continue;
        }
        0xc1af => {
            // block c1af
            #[cfg(feature = "trace")] eprintln!("f_0000c188 pc c1af rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1af mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // c1b2 mov ecx,r15d
            rcx = ((r15 as u32 as u64)) as u32 as u64;
            // c1b5 lea rdx,[rip+0xf32c4] # 0xff480
            rdx = m.env.data_va(80973);
            // c1bc mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // c1bf lea rdx,[rbp-0x1c]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffe4);
            // c1c3 mov ecx,0x10000000
            rcx = (0x10000000u64) as u32 as u64;
            // c1c8 mov r8d,0x1c
            r8 = (0x1cu64) as u32 as u64;
            // c1ce mov rdi,rbx
            rdi = rbx;
            // c1d1 call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // c1d4 cmp DWORD PTR [rbp-0x1c],0x1
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffe4)) as u64, 0x1u64).1;
            // c1d8 sete BYTE PTR [r14]
            m.w8(r14, ((fl.e() as u64)) as u8);
            pc = 0xc1dc;
        }
        0xc1dc => {
            // block c1dc
            #[cfg(feature = "trace")] eprintln!("f_0000c188 pc c1dc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c1dc add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // c1e0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // c1e1 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // c1e3 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // c1e5 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c1e6 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xaa1c size=2756
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000aa1c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xaa1c;
    loop {
        match pc {
        0xaa1c => {
            // block aa1c
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc aa1c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa1c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // aa1d mov rbp,rsp
            rbp = rsp;
            // aa20 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // aa22 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // aa23 mov ebx,esi
            rbx = ((rsi as u32 as u64)) as u32 as u64;
            // aa25 mov r14,rdi
            r14 = rdi;
            // aa28 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aa2b call QWORD PTR [rax+0x1d28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1d28)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa31 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // aa33 jne 0xaa3c
            if fl.ne() { pc = 0xaa3c; continue; }
            pc = 0xaa35;
        }
        0xaa35 => {
            // block aa35
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc aa35 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa35 mov eax,ebx
            rax = ((rbx as u32 as u64)) as u32 as u64;
            // aa37 cmp ebx,0x7
            fl = sub::<32>((rbx as u32 as u64), 0x7u64).1;
            // aa3a jbe 0xaa43
            if fl.be() { pc = 0xaa43; continue; }
            pc = 0xaa3c;
        }
        0xaa3c => {
            // block aa3c
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc aa3c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa3c xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // aa3e jmp 0xb4bb
            pc = 0xb4bb; continue;
        }
        0xaa43 => {
            // block aa43
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc aa43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa43 lea rbx,[rbp+0x10]
            rbx = rbp.wrapping_add(0x10);
            // aa47 lea rcx,[rip+0xa72] # 0xb4c0
            rcx = m.env.code_va(0xb4c0);
            // aa4e movsxd rax,DWORD PTR [rcx+rax*4]
            rax = sx::<32>(m.r32(rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
            // aa52 add rax,rcx
            { let (r, f) = add::<64>(rax, rcx);
              rax = r;
              fl = f; }
            // aa55 jmp rax
            pc = (rax.wrapping_sub(0x1000_0000)) as u32; continue;
        }
        0xaa57 => {
            // block aa57
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc aa57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aa57 mov edx,DWORD PTR [rbx]
            rdx = (m.r32(rbx) as u64) as u32 as u64;
            // aa59 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // aa5b shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // aa5e and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aa61 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // aa64 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aa66 mov DWORD PTR [rbx],edx
            m.w32(rbx, ((rdx as u32 as u64)) as u32);
            // aa68 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aa6b mov esi,0x8e
            rsi = (0x8eu64) as u32 as u64;
            // aa70 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // aa75 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // aa78 mov rdi,r14
            rdi = r14;
            // aa7b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa7e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aa81 mov edx,DWORD PTR [rbx+0x4]
            rdx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // aa84 mov esi,0x8e
            rsi = (0x8eu64) as u32 as u64;
            // aa89 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // aa8e mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // aa94 mov rdi,r14
            rdi = r14;
            // aa97 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aa9a mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // aa9d mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // aa9f shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // aaa2 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aaa5 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // aaa8 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aaaa mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // aaad mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aab0 mov esi,0x8f
            rsi = (0x8fu64) as u32 as u64;
            // aab5 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // aaba xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // aabd mov rdi,r14
            rdi = r14;
            // aac0 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aac3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aac6 mov edx,DWORD PTR [rbx+0xc]
            rdx = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // aac9 mov esi,0x8f
            rsi = (0x8fu64) as u32 as u64;
            // aace mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // aad3 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // aad9 mov rdi,r14
            rdi = r14;
            // aadc call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aadf mov edx,DWORD PTR [rbx+0x10]
            rdx = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // aae2 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // aae4 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // aae7 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aaea shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // aaed or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aaef mov DWORD PTR [rbx+0x10],edx
            m.w32(rbx.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // aaf2 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aaf5 mov esi,0x90
            rsi = (0x90u64) as u32 as u64;
            // aafa mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // aaff xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ab02 mov rdi,r14
            rdi = r14;
            // ab05 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab08 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ab0b mov edx,DWORD PTR [rbx+0x14]
            rdx = (m.r32(rbx.wrapping_add(0x14)) as u64) as u32 as u64;
            // ab0e mov esi,0x90
            rsi = (0x90u64) as u32 as u64;
            // ab13 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // ab18 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // ab1e mov rdi,r14
            rdi = r14;
            // ab21 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab24 mov edx,DWORD PTR [rbx+0x18]
            rdx = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // ab27 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // ab29 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // ab2c and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ab2f shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ab32 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ab34 mov DWORD PTR [rbx+0x18],edx
            m.w32(rbx.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // ab37 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ab3a mov esi,0x91
            rsi = (0x91u64) as u32 as u64;
            // ab3f mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ab44 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ab47 mov rdi,r14
            rdi = r14;
            // ab4a call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab4d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ab50 mov edx,DWORD PTR [rbx+0x1c]
            rdx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // ab53 mov esi,0x91
            rsi = (0x91u64) as u32 as u64;
            // ab58 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // ab5d mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // ab63 mov rdi,r14
            rdi = r14;
            // ab66 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab69 mov edx,DWORD PTR [rbx+0x20]
            rdx = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // ab6c mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // ab6e shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // ab71 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ab74 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ab77 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ab79 mov DWORD PTR [rbx+0x20],edx
            m.w32(rbx.wrapping_add(0x20), ((rdx as u32 as u64)) as u32);
            // ab7c mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ab7f mov esi,0x92
            rsi = (0x92u64) as u32 as u64;
            // ab84 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ab89 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ab8c mov rdi,r14
            rdi = r14;
            // ab8f call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ab92 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ab95 mov edx,DWORD PTR [rbx+0x24]
            rdx = (m.r32(rbx.wrapping_add(0x24)) as u64) as u32 as u64;
            // ab98 mov esi,0x92
            rsi = (0x92u64) as u32 as u64;
            // ab9d jmp 0xb4aa
            pc = 0xb4aa; continue;
        }
        0xaba2 => {
            // block aba2
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc aba2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aba2 mov edx,DWORD PTR [rbx]
            rdx = (m.r32(rbx) as u64) as u32 as u64;
            // aba4 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // aba6 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // aba9 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // abac shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // abaf or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // abb1 mov DWORD PTR [rbx],edx
            m.w32(rbx, ((rdx as u32 as u64)) as u32);
            // abb3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // abb6 mov esi,0x93
            rsi = (0x93u64) as u32 as u64;
            // abbb mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // abc0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // abc3 mov rdi,r14
            rdi = r14;
            // abc6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abc9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // abcc mov edx,DWORD PTR [rbx+0x4]
            rdx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // abcf mov esi,0x93
            rsi = (0x93u64) as u32 as u64;
            // abd4 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // abd9 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // abdf mov rdi,r14
            rdi = r14;
            // abe2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // abe5 mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // abe8 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // abea shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // abed and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // abf0 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // abf3 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // abf5 mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // abf8 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // abfb mov esi,0x94
            rsi = (0x94u64) as u32 as u64;
            // ac00 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ac05 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ac08 mov rdi,r14
            rdi = r14;
            // ac0b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ac0e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ac11 mov edx,DWORD PTR [rbx+0xc]
            rdx = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // ac14 mov esi,0x94
            rsi = (0x94u64) as u32 as u64;
            // ac19 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // ac1e mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // ac24 mov rdi,r14
            rdi = r14;
            // ac27 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ac2a mov edx,DWORD PTR [rbx+0x10]
            rdx = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // ac2d mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // ac2f shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // ac32 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ac35 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ac38 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ac3a mov DWORD PTR [rbx+0x10],edx
            m.w32(rbx.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // ac3d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ac40 mov esi,0x95
            rsi = (0x95u64) as u32 as u64;
            // ac45 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ac4a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ac4d mov rdi,r14
            rdi = r14;
            // ac50 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ac53 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ac56 mov edx,DWORD PTR [rbx+0x14]
            rdx = (m.r32(rbx.wrapping_add(0x14)) as u64) as u32 as u64;
            // ac59 mov esi,0x95
            rsi = (0x95u64) as u32 as u64;
            // ac5e mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // ac63 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // ac69 mov rdi,r14
            rdi = r14;
            // ac6c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ac6f mov edx,DWORD PTR [rbx+0x18]
            rdx = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // ac72 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // ac74 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // ac77 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ac7a shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ac7d or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ac7f mov DWORD PTR [rbx+0x18],edx
            m.w32(rbx.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // ac82 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ac85 mov esi,0x96
            rsi = (0x96u64) as u32 as u64;
            // ac8a mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ac8f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ac92 mov rdi,r14
            rdi = r14;
            // ac95 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ac98 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ac9b mov edx,DWORD PTR [rbx+0x1c]
            rdx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // ac9e mov esi,0x96
            rsi = (0x96u64) as u32 as u64;
            // aca3 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // aca8 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // acae mov rdi,r14
            rdi = r14;
            // acb1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // acb4 mov edx,DWORD PTR [rbx+0x20]
            rdx = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // acb7 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // acb9 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // acbc and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // acbf shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // acc2 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // acc4 mov DWORD PTR [rbx+0x20],edx
            m.w32(rbx.wrapping_add(0x20), ((rdx as u32 as u64)) as u32);
            // acc7 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // acca mov esi,0x97
            rsi = (0x97u64) as u32 as u64;
            // accf mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // acd4 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // acd7 mov rdi,r14
            rdi = r14;
            // acda call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // acdd mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ace0 mov edx,DWORD PTR [rbx+0x24]
            rdx = (m.r32(rbx.wrapping_add(0x24)) as u64) as u32 as u64;
            // ace3 mov esi,0x97
            rsi = (0x97u64) as u32 as u64;
            // ace8 jmp 0xb4aa
            pc = 0xb4aa; continue;
        }
        0xaced => {
            // block aced
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc aced rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // aced mov edx,DWORD PTR [rbx]
            rdx = (m.r32(rbx) as u64) as u32 as u64;
            // acef mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // acf1 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // acf4 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // acf7 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // acfa or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // acfc mov DWORD PTR [rbx],edx
            m.w32(rbx, ((rdx as u32 as u64)) as u32);
            // acfe mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ad01 mov esi,0x123
            rsi = (0x123u64) as u32 as u64;
            // ad06 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ad0b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ad0e mov rdi,r14
            rdi = r14;
            // ad11 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad14 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ad17 mov edx,DWORD PTR [rbx+0x4]
            rdx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // ad1a mov esi,0x123
            rsi = (0x123u64) as u32 as u64;
            // ad1f mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // ad24 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // ad2a mov rdi,r14
            rdi = r14;
            // ad2d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad30 mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // ad33 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // ad35 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // ad38 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ad3b shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ad3e or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ad40 mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // ad43 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ad46 mov esi,0x124
            rsi = (0x124u64) as u32 as u64;
            // ad4b mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ad50 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ad53 mov rdi,r14
            rdi = r14;
            // ad56 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad59 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ad5c mov edx,DWORD PTR [rbx+0xc]
            rdx = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // ad5f mov esi,0x124
            rsi = (0x124u64) as u32 as u64;
            // ad64 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // ad69 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // ad6f mov rdi,r14
            rdi = r14;
            // ad72 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad75 mov edx,DWORD PTR [rbx+0x10]
            rdx = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // ad78 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // ad7a shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // ad7d and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ad80 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ad83 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ad85 mov DWORD PTR [rbx+0x10],edx
            m.w32(rbx.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // ad88 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ad8b mov esi,0x125
            rsi = (0x125u64) as u32 as u64;
            // ad90 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ad95 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ad98 mov rdi,r14
            rdi = r14;
            // ad9b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ad9e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ada1 mov edx,DWORD PTR [rbx+0x14]
            rdx = (m.r32(rbx.wrapping_add(0x14)) as u64) as u32 as u64;
            // ada4 mov esi,0x125
            rsi = (0x125u64) as u32 as u64;
            // ada9 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // adae mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // adb4 mov rdi,r14
            rdi = r14;
            // adb7 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // adba mov edx,DWORD PTR [rbx+0x18]
            rdx = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // adbd mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // adbf shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // adc2 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // adc5 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // adc8 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // adca mov DWORD PTR [rbx+0x18],edx
            m.w32(rbx.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // adcd mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // add0 mov esi,0x126
            rsi = (0x126u64) as u32 as u64;
            // add5 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // adda xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // addd mov rdi,r14
            rdi = r14;
            // ade0 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ade3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ade6 mov edx,DWORD PTR [rbx+0x1c]
            rdx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // ade9 mov esi,0x126
            rsi = (0x126u64) as u32 as u64;
            // adee mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // adf3 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // adf9 mov rdi,r14
            rdi = r14;
            // adfc call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // adff mov edx,DWORD PTR [rbx+0x20]
            rdx = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // ae02 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // ae04 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // ae07 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ae0a shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ae0d or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ae0f mov DWORD PTR [rbx+0x20],edx
            m.w32(rbx.wrapping_add(0x20), ((rdx as u32 as u64)) as u32);
            // ae12 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ae15 mov esi,0x127
            rsi = (0x127u64) as u32 as u64;
            // ae1a mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ae1f xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ae22 mov rdi,r14
            rdi = r14;
            // ae25 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ae28 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ae2b mov edx,DWORD PTR [rbx+0x24]
            rdx = (m.r32(rbx.wrapping_add(0x24)) as u64) as u32 as u64;
            // ae2e mov esi,0x127
            rsi = (0x127u64) as u32 as u64;
            // ae33 jmp 0xb4aa
            pc = 0xb4aa; continue;
        }
        0xae38 => {
            // block ae38
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc ae38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ae38 mov edx,DWORD PTR [rbx]
            rdx = (m.r32(rbx) as u64) as u32 as u64;
            // ae3a mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // ae3c shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // ae3f and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ae42 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ae45 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ae47 mov DWORD PTR [rbx],edx
            m.w32(rbx, ((rdx as u32 as u64)) as u32);
            // ae49 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ae4c mov esi,0x128
            rsi = (0x128u64) as u32 as u64;
            // ae51 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ae56 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ae59 mov rdi,r14
            rdi = r14;
            // ae5c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ae5f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ae62 mov edx,DWORD PTR [rbx+0x4]
            rdx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // ae65 mov esi,0x128
            rsi = (0x128u64) as u32 as u64;
            // ae6a mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // ae6f mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // ae75 mov rdi,r14
            rdi = r14;
            // ae78 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ae7b mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // ae7e mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // ae80 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // ae83 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ae86 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // ae89 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // ae8b mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // ae8e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // ae91 mov esi,0x129
            rsi = (0x129u64) as u32 as u64;
            // ae96 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // ae9b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // ae9e mov rdi,r14
            rdi = r14;
            // aea1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aea4 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aea7 mov edx,DWORD PTR [rbx+0xc]
            rdx = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // aeaa mov esi,0x129
            rsi = (0x129u64) as u32 as u64;
            // aeaf mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // aeb4 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // aeba mov rdi,r14
            rdi = r14;
            // aebd call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aec0 mov edx,DWORD PTR [rbx+0x10]
            rdx = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // aec3 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // aec5 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // aec8 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aecb shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // aece or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // aed0 mov DWORD PTR [rbx+0x10],edx
            m.w32(rbx.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // aed3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aed6 mov esi,0x12a
            rsi = (0x12au64) as u32 as u64;
            // aedb mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // aee0 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // aee3 mov rdi,r14
            rdi = r14;
            // aee6 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // aee9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aeec mov edx,DWORD PTR [rbx+0x14]
            rdx = (m.r32(rbx.wrapping_add(0x14)) as u64) as u32 as u64;
            // aeef mov esi,0x12a
            rsi = (0x12au64) as u32 as u64;
            // aef4 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // aef9 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // aeff mov rdi,r14
            rdi = r14;
            // af02 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af05 mov edx,DWORD PTR [rbx+0x18]
            rdx = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // af08 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // af0a shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // af0d and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af10 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // af13 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af15 mov DWORD PTR [rbx+0x18],edx
            m.w32(rbx.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // af18 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // af1b mov esi,0x12b
            rsi = (0x12bu64) as u32 as u64;
            // af20 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // af25 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // af28 mov rdi,r14
            rdi = r14;
            // af2b call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af2e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // af31 mov edx,DWORD PTR [rbx+0x1c]
            rdx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // af34 mov esi,0x12b
            rsi = (0x12bu64) as u32 as u64;
            // af39 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // af3e mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // af44 mov rdi,r14
            rdi = r14;
            // af47 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af4a mov edx,DWORD PTR [rbx+0x20]
            rdx = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // af4d mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // af4f shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // af52 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af55 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // af58 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af5a mov DWORD PTR [rbx+0x20],edx
            m.w32(rbx.wrapping_add(0x20), ((rdx as u32 as u64)) as u32);
            // af5d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // af60 mov esi,0x12c
            rsi = (0x12cu64) as u32 as u64;
            // af65 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // af6a xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // af6d mov rdi,r14
            rdi = r14;
            // af70 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // af73 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // af76 mov edx,DWORD PTR [rbx+0x24]
            rdx = (m.r32(rbx.wrapping_add(0x24)) as u64) as u32 as u64;
            // af79 mov esi,0x12c
            rsi = (0x12cu64) as u32 as u64;
            // af7e jmp 0xb4aa
            pc = 0xb4aa; continue;
        }
        0xaf83 => {
            // block af83
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc af83 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // af83 mov edx,DWORD PTR [rbx]
            rdx = (m.r32(rbx) as u64) as u32 as u64;
            // af85 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // af87 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // af8a and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af8d shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // af90 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // af92 mov DWORD PTR [rbx],edx
            m.w32(rbx, ((rdx as u32 as u64)) as u32);
            // af94 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // af97 mov esi,0x15b
            rsi = (0x15bu64) as u32 as u64;
            // af9c mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // afa1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // afa4 mov rdi,r14
            rdi = r14;
            // afa7 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afaa mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // afad mov edx,DWORD PTR [rbx+0x4]
            rdx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // afb0 mov esi,0x15b
            rsi = (0x15bu64) as u32 as u64;
            // afb5 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // afba mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // afc0 mov rdi,r14
            rdi = r14;
            // afc3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afc6 mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // afc9 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // afcb shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // afce and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // afd1 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // afd4 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // afd6 mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // afd9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // afdc mov esi,0x15c
            rsi = (0x15cu64) as u32 as u64;
            // afe1 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // afe6 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // afe9 mov rdi,r14
            rdi = r14;
            // afec call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // afef mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // aff2 mov edx,DWORD PTR [rbx+0xc]
            rdx = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // aff5 mov esi,0x15c
            rsi = (0x15cu64) as u32 as u64;
            // affa mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // afff mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b005 mov rdi,r14
            rdi = r14;
            // b008 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b00b mov edx,DWORD PTR [rbx+0x10]
            rdx = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b00e mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b010 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b013 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b016 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b019 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b01b mov DWORD PTR [rbx+0x10],edx
            m.w32(rbx.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // b01e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b021 mov esi,0x15d
            rsi = (0x15du64) as u32 as u64;
            // b026 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b02b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b02e mov rdi,r14
            rdi = r14;
            // b031 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b034 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b037 mov edx,DWORD PTR [rbx+0x14]
            rdx = (m.r32(rbx.wrapping_add(0x14)) as u64) as u32 as u64;
            // b03a mov esi,0x15d
            rsi = (0x15du64) as u32 as u64;
            // b03f mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b044 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b04a mov rdi,r14
            rdi = r14;
            // b04d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b050 mov edx,DWORD PTR [rbx+0x18]
            rdx = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // b053 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b055 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b058 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b05b shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b05e or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b060 mov DWORD PTR [rbx+0x18],edx
            m.w32(rbx.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // b063 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b066 mov esi,0x15e
            rsi = (0x15eu64) as u32 as u64;
            // b06b mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b070 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b073 mov rdi,r14
            rdi = r14;
            // b076 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b079 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b07c mov edx,DWORD PTR [rbx+0x1c]
            rdx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b07f mov esi,0x15e
            rsi = (0x15eu64) as u32 as u64;
            // b084 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b089 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b08f mov rdi,r14
            rdi = r14;
            // b092 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b095 mov edx,DWORD PTR [rbx+0x20]
            rdx = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b098 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b09a shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b09d and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0a0 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b0a3 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0a5 mov DWORD PTR [rbx+0x20],edx
            m.w32(rbx.wrapping_add(0x20), ((rdx as u32 as u64)) as u32);
            // b0a8 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b0ab mov esi,0x15f
            rsi = (0x15fu64) as u32 as u64;
            // b0b0 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b0b5 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b0b8 mov rdi,r14
            rdi = r14;
            // b0bb call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0be mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b0c1 mov edx,DWORD PTR [rbx+0x24]
            rdx = (m.r32(rbx.wrapping_add(0x24)) as u64) as u32 as u64;
            // b0c4 mov esi,0x15f
            rsi = (0x15fu64) as u32 as u64;
            // b0c9 jmp 0xb4aa
            pc = 0xb4aa; continue;
        }
        0xb0ce => {
            // block b0ce
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc b0ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b0ce mov edx,DWORD PTR [rbx]
            rdx = (m.r32(rbx) as u64) as u32 as u64;
            // b0d0 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b0d2 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b0d5 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0d8 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b0db or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b0dd mov DWORD PTR [rbx],edx
            m.w32(rbx, ((rdx as u32 as u64)) as u32);
            // b0df mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b0e2 mov esi,0x1cc
            rsi = (0x1ccu64) as u32 as u64;
            // b0e7 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b0ec xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b0ef mov rdi,r14
            rdi = r14;
            // b0f2 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b0f5 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b0f8 mov edx,DWORD PTR [rbx+0x4]
            rdx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // b0fb mov esi,0x1cc
            rsi = (0x1ccu64) as u32 as u64;
            // b100 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b105 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b10b mov rdi,r14
            rdi = r14;
            // b10e call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b111 mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // b114 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b116 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b119 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b11c shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b11f or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b121 mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b124 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b127 mov esi,0x1cd
            rsi = (0x1cdu64) as u32 as u64;
            // b12c mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b131 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b134 mov rdi,r14
            rdi = r14;
            // b137 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b13a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b13d mov edx,DWORD PTR [rbx+0xc]
            rdx = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // b140 mov esi,0x1cd
            rsi = (0x1cdu64) as u32 as u64;
            // b145 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b14a mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b150 mov rdi,r14
            rdi = r14;
            // b153 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b156 mov edx,DWORD PTR [rbx+0x10]
            rdx = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b159 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b15b shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b15e and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b161 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b164 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b166 mov DWORD PTR [rbx+0x10],edx
            m.w32(rbx.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // b169 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b16c mov esi,0x1ce
            rsi = (0x1ceu64) as u32 as u64;
            // b171 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b176 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b179 mov rdi,r14
            rdi = r14;
            // b17c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b17f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b182 mov edx,DWORD PTR [rbx+0x14]
            rdx = (m.r32(rbx.wrapping_add(0x14)) as u64) as u32 as u64;
            // b185 mov esi,0x1ce
            rsi = (0x1ceu64) as u32 as u64;
            // b18a mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b18f mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b195 mov rdi,r14
            rdi = r14;
            // b198 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b19b mov edx,DWORD PTR [rbx+0x18]
            rdx = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // b19e mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b1a0 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b1a3 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b1a6 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b1a9 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b1ab mov DWORD PTR [rbx+0x18],edx
            m.w32(rbx.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // b1ae mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b1b1 mov esi,0x1cf
            rsi = (0x1cfu64) as u32 as u64;
            // b1b6 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b1bb xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b1be mov rdi,r14
            rdi = r14;
            // b1c1 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1c4 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b1c7 mov edx,DWORD PTR [rbx+0x1c]
            rdx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b1ca mov esi,0x1cf
            rsi = (0x1cfu64) as u32 as u64;
            // b1cf mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b1d4 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b1da mov rdi,r14
            rdi = r14;
            // b1dd call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b1e0 mov edx,DWORD PTR [rbx+0x20]
            rdx = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b1e3 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b1e5 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b1e8 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b1eb shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b1ee or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b1f0 mov DWORD PTR [rbx+0x20],edx
            m.w32(rbx.wrapping_add(0x20), ((rdx as u32 as u64)) as u32);
            // b1f3 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b1f6 mov esi,0x1d0
            rsi = (0x1d0u64) as u32 as u64;
            // b1fb mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b200 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b203 mov rdi,r14
            rdi = r14;
            // b206 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b209 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b20c mov edx,DWORD PTR [rbx+0x24]
            rdx = (m.r32(rbx.wrapping_add(0x24)) as u64) as u32 as u64;
            // b20f mov esi,0x1d0
            rsi = (0x1d0u64) as u32 as u64;
            // b214 jmp 0xb4aa
            pc = 0xb4aa; continue;
        }
        0xb219 => {
            // block b219
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc b219 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b219 mov edx,DWORD PTR [rbx]
            rdx = (m.r32(rbx) as u64) as u32 as u64;
            // b21b mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b21d shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b220 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b223 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b226 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b228 mov DWORD PTR [rbx],edx
            m.w32(rbx, ((rdx as u32 as u64)) as u32);
            // b22a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b22d mov esi,0x1d1
            rsi = (0x1d1u64) as u32 as u64;
            // b232 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b237 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b23a mov rdi,r14
            rdi = r14;
            // b23d call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b240 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b243 mov edx,DWORD PTR [rbx+0x4]
            rdx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // b246 mov esi,0x1d1
            rsi = (0x1d1u64) as u32 as u64;
            // b24b mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b250 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b256 mov rdi,r14
            rdi = r14;
            // b259 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b25c mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // b25f mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b261 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b264 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b267 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b26a or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b26c mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b26f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b272 mov esi,0x1d2
            rsi = (0x1d2u64) as u32 as u64;
            // b277 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b27c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b27f mov rdi,r14
            rdi = r14;
            // b282 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b285 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b288 mov edx,DWORD PTR [rbx+0xc]
            rdx = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // b28b mov esi,0x1d2
            rsi = (0x1d2u64) as u32 as u64;
            // b290 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b295 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b29b mov rdi,r14
            rdi = r14;
            // b29e call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2a1 mov edx,DWORD PTR [rbx+0x10]
            rdx = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b2a4 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b2a6 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b2a9 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b2ac shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b2af or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b2b1 mov DWORD PTR [rbx+0x10],edx
            m.w32(rbx.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // b2b4 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b2b7 mov esi,0x1d3
            rsi = (0x1d3u64) as u32 as u64;
            // b2bc mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b2c1 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b2c4 mov rdi,r14
            rdi = r14;
            // b2c7 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2ca mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b2cd mov edx,DWORD PTR [rbx+0x14]
            rdx = (m.r32(rbx.wrapping_add(0x14)) as u64) as u32 as u64;
            // b2d0 mov esi,0x1d3
            rsi = (0x1d3u64) as u32 as u64;
            // b2d5 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b2da mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b2e0 mov rdi,r14
            rdi = r14;
            // b2e3 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b2e6 mov edx,DWORD PTR [rbx+0x18]
            rdx = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // b2e9 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b2eb shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b2ee and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b2f1 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b2f4 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b2f6 mov DWORD PTR [rbx+0x18],edx
            m.w32(rbx.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // b2f9 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b2fc mov esi,0x1d4
            rsi = (0x1d4u64) as u32 as u64;
            // b301 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b306 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b309 mov rdi,r14
            rdi = r14;
            // b30c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b30f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b312 mov edx,DWORD PTR [rbx+0x1c]
            rdx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b315 mov esi,0x1d4
            rsi = (0x1d4u64) as u32 as u64;
            // b31a mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b31f mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b325 mov rdi,r14
            rdi = r14;
            // b328 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b32b mov edx,DWORD PTR [rbx+0x20]
            rdx = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b32e mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b330 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b333 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b336 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b339 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b33b mov DWORD PTR [rbx+0x20],edx
            m.w32(rbx.wrapping_add(0x20), ((rdx as u32 as u64)) as u32);
            // b33e mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b341 mov esi,0x1d5
            rsi = (0x1d5u64) as u32 as u64;
            // b346 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b34b xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b34e mov rdi,r14
            rdi = r14;
            // b351 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b354 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b357 mov edx,DWORD PTR [rbx+0x24]
            rdx = (m.r32(rbx.wrapping_add(0x24)) as u64) as u32 as u64;
            // b35a mov esi,0x1d5
            rsi = (0x1d5u64) as u32 as u64;
            // b35f jmp 0xb4aa
            pc = 0xb4aa; continue;
        }
        0xb364 => {
            // block b364
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc b364 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b364 mov edx,DWORD PTR [rbx]
            rdx = (m.r32(rbx) as u64) as u32 as u64;
            // b366 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b368 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b36b and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b36e shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b371 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b373 mov DWORD PTR [rbx],edx
            m.w32(rbx, ((rdx as u32 as u64)) as u32);
            // b375 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b378 mov esi,0x1d6
            rsi = (0x1d6u64) as u32 as u64;
            // b37d mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b382 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b385 mov rdi,r14
            rdi = r14;
            // b388 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b38b mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b38e mov edx,DWORD PTR [rbx+0x4]
            rdx = (m.r32(rbx.wrapping_add(0x4)) as u64) as u32 as u64;
            // b391 mov esi,0x1d6
            rsi = (0x1d6u64) as u32 as u64;
            // b396 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b39b mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b3a1 mov rdi,r14
            rdi = r14;
            // b3a4 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3a7 mov edx,DWORD PTR [rbx+0x8]
            rdx = (m.r32(rbx.wrapping_add(0x8)) as u64) as u32 as u64;
            // b3aa mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b3ac shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b3af and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b3b2 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b3b5 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b3b7 mov DWORD PTR [rbx+0x8],edx
            m.w32(rbx.wrapping_add(0x8), ((rdx as u32 as u64)) as u32);
            // b3ba mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b3bd mov esi,0x1d7
            rsi = (0x1d7u64) as u32 as u64;
            // b3c2 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b3c7 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b3ca mov rdi,r14
            rdi = r14;
            // b3cd call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3d0 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b3d3 mov edx,DWORD PTR [rbx+0xc]
            rdx = (m.r32(rbx.wrapping_add(0xc)) as u64) as u32 as u64;
            // b3d6 mov esi,0x1d7
            rsi = (0x1d7u64) as u32 as u64;
            // b3db mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b3e0 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b3e6 mov rdi,r14
            rdi = r14;
            // b3e9 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b3ec mov edx,DWORD PTR [rbx+0x10]
            rdx = (m.r32(rbx.wrapping_add(0x10)) as u64) as u32 as u64;
            // b3ef mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b3f1 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b3f4 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b3f7 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b3fa or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b3fc mov DWORD PTR [rbx+0x10],edx
            m.w32(rbx.wrapping_add(0x10), ((rdx as u32 as u64)) as u32);
            // b3ff mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b402 mov esi,0x1d8
            rsi = (0x1d8u64) as u32 as u64;
            // b407 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b40c xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b40f mov rdi,r14
            rdi = r14;
            // b412 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b415 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b418 mov edx,DWORD PTR [rbx+0x14]
            rdx = (m.r32(rbx.wrapping_add(0x14)) as u64) as u32 as u64;
            // b41b mov esi,0x1d8
            rsi = (0x1d8u64) as u32 as u64;
            // b420 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b425 mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b42b mov rdi,r14
            rdi = r14;
            // b42e call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b431 mov edx,DWORD PTR [rbx+0x18]
            rdx = (m.r32(rbx.wrapping_add(0x18)) as u64) as u32 as u64;
            // b434 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b436 shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b439 and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b43c shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b43f or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b441 mov DWORD PTR [rbx+0x18],edx
            m.w32(rbx.wrapping_add(0x18), ((rdx as u32 as u64)) as u32);
            // b444 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b447 mov esi,0x1d9
            rsi = (0x1d9u64) as u32 as u64;
            // b44c mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b451 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b454 mov rdi,r14
            rdi = r14;
            // b457 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b45a mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b45d mov edx,DWORD PTR [rbx+0x1c]
            rdx = (m.r32(rbx.wrapping_add(0x1c)) as u64) as u32 as u64;
            // b460 mov esi,0x1d9
            rsi = (0x1d9u64) as u32 as u64;
            // b465 mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b46a mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b470 mov rdi,r14
            rdi = r14;
            // b473 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b476 mov edx,DWORD PTR [rbx+0x20]
            rdx = (m.r32(rbx.wrapping_add(0x20)) as u64) as u32 as u64;
            // b479 mov eax,edx
            rax = ((rdx as u32 as u64)) as u32 as u64;
            // b47b shr eax,0x2
            { let (r, f) = shr::<32>((rax as u32 as u64), 0x2u64, fl); rax = (r) as u32 as u64; fl = f; }
            // b47e and edx,0x3
            { let r = (rdx as u32 as u64) & 0x3u64; let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b481 shl edx,0xb
            { let (r, f) = shl::<32>((rdx as u32 as u64), 0xbu64, fl); rdx = (r) as u32 as u64; fl = f; }
            // b484 or edx,eax
            { let r = (rdx as u32 as u64) | (rax as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // b486 mov DWORD PTR [rbx+0x20],edx
            m.w32(rbx.wrapping_add(0x20), ((rdx as u32 as u64)) as u32);
            // b489 mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b48c mov esi,0x1da
            rsi = (0x1dau64) as u32 as u64;
            // b491 mov ecx,0x1fff
            rcx = (0x1fffu64) as u32 as u64;
            // b496 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // b499 mov rdi,r14
            rdi = r14;
            // b49c call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b49f mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // b4a2 mov edx,DWORD PTR [rbx+0x24]
            rdx = (m.r32(rbx.wrapping_add(0x24)) as u64) as u32 as u64;
            // b4a5 mov esi,0x1da
            rsi = (0x1dau64) as u32 as u64;
            pc = 0xb4aa;
        }
        0xb4aa => {
            // block b4aa
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc b4aa rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4aa mov ecx,0x7ffc000
            rcx = (0x7ffc000u64) as u32 as u64;
            // b4af mov r8d,0xe
            r8 = (0xeu64) as u32 as u64;
            // b4b5 mov rdi,r14
            rdi = r14;
            // b4b8 call QWORD PTR [rax+0x30]
            { let r = { let t = m.r64(rax.wrapping_add(0x30)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0xb4bb;
        }
        0xb4bb => {
            // block b4bb
            #[cfg(feature = "trace")] eprintln!("f_0000aa1c pc b4bb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b4bb pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // b4bc pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // b4be pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // b4bf ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x64410 size=475
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00064410<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
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
    let mut x2: u128 = fx[2] as u128;
    let mut x3: u128 = fx[3] as u128;
    let mut x4: u128 = fx[4] as u128;
    let mut x5: u128 = fx[5] as u128;
    let mut x6: u128 = fx[6] as u128;
    let mut x7: u128 = fx[7] as u128;
    let mut x8: u128 = 0;
    let mut x9: u128 = 0;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x64410;
    loop {
        match pc {
        0x64410 => {
            // block 64410
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64410 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64410 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 64411 mov rbp,rsp
            rbp = rsp;
            // 64414 movss xmm0,DWORD PTR [rdi]
            x0 = m.r32(rdi) as u128;
            // 64418 xorps xmm1,xmm1
            x1 = 0;
            // 6441b ucomiss xmm1,xmm0
            fl = ucomi(f32_of(x1) as f64, f32_of(x0) as f64);
            // 6441e ja 0x64429
            if fl.a() { pc = 0x64429; continue; }
            pc = 0x64420;
        }
        0x64420 => {
            // block 64420
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64420 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64420 ucomiss xmm0,DWORD PTR [rip+0x9c38d] # 0x1007b4
            fl = ucomi(f32_of(x0) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 64427 jbe 0x6442d
            if fl.be() { pc = 0x6442d; continue; }
            pc = 0x64429;
        }
        0x64429 => {
            // block 64429
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64429 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64429 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x6442b;
        }
        0x6442b => {
            // block 6442b
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6442b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6442b pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 6442c ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x6442d => {
            // block 6442d
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6442d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6442d movss xmm1,DWORD PTR [rdi+0x4]
            x1 = m.r32(rdi.wrapping_add(0x4)) as u128;
            // 64432 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 64434 xorps xmm2,xmm2
            x2 = 0;
            // 64437 ucomiss xmm2,xmm1
            fl = ucomi(f32_of(x2) as f64, f32_of(x1) as f64);
            // 6443a ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6443c;
        }
        0x6443c => {
            // block 6443c
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6443c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6443c ucomiss xmm1,DWORD PTR [rip+0x9c371] # 0x1007b4
            fl = ucomi(f32_of(x1) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 64443 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64445;
        }
        0x64445 => {
            // block 64445
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64445 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64445 movss xmm2,DWORD PTR [rdi+0x8]
            x2 = m.r32(rdi.wrapping_add(0x8)) as u128;
            // 6444a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6444c xorps xmm3,xmm3
            x3 = 0;
            // 6444f ucomiss xmm3,xmm2
            fl = ucomi(f32_of(x3) as f64, f32_of(x2) as f64);
            // 64452 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64454;
        }
        0x64454 => {
            // block 64454
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64454 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64454 ucomiss xmm2,DWORD PTR [rip+0x9c359] # 0x1007b4
            fl = ucomi(f32_of(x2) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 6445b ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6445d;
        }
        0x6445d => {
            // block 6445d
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6445d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6445d movss xmm3,DWORD PTR [rdi+0xc]
            x3 = m.r32(rdi.wrapping_add(0xc)) as u128;
            // 64462 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 64464 xorps xmm4,xmm4
            x4 = 0;
            // 64467 ucomiss xmm4,xmm3
            fl = ucomi(f32_of(x4) as f64, f32_of(x3) as f64);
            // 6446a ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6446c;
        }
        0x6446c => {
            // block 6446c
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6446c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6446c ucomiss xmm3,DWORD PTR [rip+0x9c341] # 0x1007b4
            fl = ucomi(f32_of(x3) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 64473 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64475;
        }
        0x64475 => {
            // block 64475
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64475 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64475 movss xmm4,DWORD PTR [rdi+0x10]
            x4 = m.r32(rdi.wrapping_add(0x10)) as u128;
            // 6447a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 6447c xorps xmm5,xmm5
            x5 = 0;
            // 6447f ucomiss xmm5,xmm4
            fl = ucomi(f32_of(x5) as f64, f32_of(x4) as f64);
            // 64482 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64484;
        }
        0x64484 => {
            // block 64484
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64484 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64484 ucomiss xmm4,DWORD PTR [rip+0x9c329] # 0x1007b4
            fl = ucomi(f32_of(x4) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 6448b ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6448d;
        }
        0x6448d => {
            // block 6448d
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6448d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6448d movss xmm5,DWORD PTR [rdi+0x14]
            x5 = m.r32(rdi.wrapping_add(0x14)) as u128;
            // 64492 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 64494 xorps xmm6,xmm6
            x6 = 0;
            // 64497 ucomiss xmm6,xmm5
            fl = ucomi(f32_of(x6) as f64, f32_of(x5) as f64);
            // 6449a ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x6449c;
        }
        0x6449c => {
            // block 6449c
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 6449c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 6449c ucomiss xmm5,DWORD PTR [rip+0x9c311] # 0x1007b4
            fl = ucomi(f32_of(x5) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 644a3 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644a5;
        }
        0x644a5 => {
            // block 644a5
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644a5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644a5 movss xmm6,DWORD PTR [rdi+0x18]
            x6 = m.r32(rdi.wrapping_add(0x18)) as u128;
            // 644aa xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 644ac xorps xmm7,xmm7
            x7 = 0;
            // 644af ucomiss xmm7,xmm6
            fl = ucomi(f32_of(x7) as f64, f32_of(x6) as f64);
            // 644b2 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644b8;
        }
        0x644b8 => {
            // block 644b8
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644b8 ucomiss xmm6,DWORD PTR [rip+0x9c2f5] # 0x1007b4
            fl = ucomi(f32_of(x6) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 644bf ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644c5;
        }
        0x644c5 => {
            // block 644c5
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644c5 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644c5 movss xmm9,DWORD PTR [rdi+0x1c]
            x9 = m.r32(rdi.wrapping_add(0x1c)) as u128;
            // 644cb xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 644cd xorps xmm7,xmm7
            x7 = 0;
            // 644d0 ucomiss xmm7,xmm9
            fl = ucomi(f32_of(x7) as f64, f32_of(x9) as f64);
            // 644d4 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644da;
        }
        0x644da => {
            // block 644da
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644da rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644da ucomiss xmm9,DWORD PTR [rip+0x9c2d2] # 0x1007b4
            fl = ucomi(f32_of(x9) as f64, f32_of((m.r32(m.env.data_va(85889)) as u128)) as f64);
            // 644e2 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644e8;
        }
        0x644e8 => {
            // block 644e8
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644e8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644e8 movss xmm8,DWORD PTR [rdi+0x24]
            x8 = m.r32(rdi.wrapping_add(0x24)) as u128;
            // 644ee xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            // 644f0 xorps xmm7,xmm7
            x7 = 0;
            // 644f3 ucomiss xmm7,xmm8
            fl = ucomi(f32_of(x7) as f64, f32_of(x8) as f64);
            // 644f7 ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x644fd;
        }
        0x644fd => {
            // block 644fd
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 644fd rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 644fd cvtss2sd xmm8,xmm8
            x8 = set_lo64(x8, (f32_of(x8) as f64).to_bits());
            // 64502 ucomisd xmm8,QWORD PTR [rip+0x9c28d] # 0x100798
            fl = ucomi(f64_of(x8), f64_of((m.r64(m.env.data_va(85861)) as u128)));
            // 6450b ja 0x6442b
            if fl.a() { pc = 0x6442b; continue; }
            pc = 0x64511;
        }
        0x64511 => {
            // block 64511
            #[cfg(feature = "trace")] eprintln!("f_00064410 pc 64511 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 64511 xorps xmm7,xmm7
            x7 = 0;
            // 64514 cvtss2sd xmm7,xmm0
            x7 = set_lo64(x7, (f32_of(x0) as f64).to_bits());
            // 64518 movsd xmm0,QWORD PTR [rip+0x9c280] # 0x1007a0
            x0 = m.r64(m.env.data_va(85869)) as u128;
            // 64520 divsd xmm7,xmm0
            x7 = set_lo64(x7, (f64_of(x7) / f64_of(x0)).to_bits());
            // 64524 cvttsd2si eax,xmm7
            rax = (cvtt32(f64_of(x7)) as u64) as u32 as u64;
            // 64528 mov WORD PTR [rsi],ax
            m.w16(rsi, ((rax as u16 as u64)) as u16);
            // 6452b cvtss2sd xmm1,xmm1
            x1 = set_lo64(x1, (f32_of(x1) as f64).to_bits());
            // 6452f divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 64533 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 64537 mov WORD PTR [rsi+0x2],ax
            m.w16(rsi.wrapping_add(0x2), ((rax as u16 as u64)) as u16);
            // 6453b xorps xmm1,xmm1
            x1 = 0;
            // 6453e cvtss2sd xmm1,xmm2
            x1 = set_lo64(x1, (f32_of(x2) as f64).to_bits());
            // 64542 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 64546 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 6454a mov WORD PTR [rsi+0x4],ax
            m.w16(rsi.wrapping_add(0x4), ((rax as u16 as u64)) as u16);
            // 6454e xorps xmm1,xmm1
            x1 = 0;
            // 64551 cvtss2sd xmm1,xmm3
            x1 = set_lo64(x1, (f32_of(x3) as f64).to_bits());
            // 64555 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 64559 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 6455d mov WORD PTR [rsi+0x6],ax
            m.w16(rsi.wrapping_add(0x6), ((rax as u16 as u64)) as u16);
            // 64561 xorps xmm1,xmm1
            x1 = 0;
            // 64564 cvtss2sd xmm1,xmm4
            x1 = set_lo64(x1, (f32_of(x4) as f64).to_bits());
            // 64568 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 6456c cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 64570 mov WORD PTR [rsi+0x8],ax
            m.w16(rsi.wrapping_add(0x8), ((rax as u16 as u64)) as u16);
            // 64574 xorps xmm1,xmm1
            x1 = 0;
            // 64577 cvtss2sd xmm1,xmm5
            x1 = set_lo64(x1, (f32_of(x5) as f64).to_bits());
            // 6457b divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 6457f cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 64583 mov WORD PTR [rsi+0xa],ax
            m.w16(rsi.wrapping_add(0xa), ((rax as u16 as u64)) as u16);
            // 64587 xorps xmm1,xmm1
            x1 = 0;
            // 6458a cvtss2sd xmm1,xmm6
            x1 = set_lo64(x1, (f32_of(x6) as f64).to_bits());
            // 6458e divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 64592 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 64596 mov WORD PTR [rsi+0xc],ax
            m.w16(rsi.wrapping_add(0xc), ((rax as u16 as u64)) as u16);
            // 6459a xorps xmm1,xmm1
            x1 = 0;
            // 6459d cvtss2sd xmm1,xmm9
            x1 = set_lo64(x1, (f32_of(x9) as f64).to_bits());
            // 645a2 divsd xmm1,xmm0
            x1 = set_lo64(x1, (f64_of(x1) / f64_of(x0)).to_bits());
            // 645a6 cvttsd2si eax,xmm1
            rax = (cvtt32(f64_of(x1)) as u64) as u32 as u64;
            // 645aa mov WORD PTR [rsi+0xe],ax
            m.w16(rsi.wrapping_add(0xe), ((rax as u16 as u64)) as u16);
            // 645ae divsd xmm8,QWORD PTR [rip+0x9c1f1] # 0x1007a8
            x8 = set_lo64(x8, (f64_of(x8) / f64_of((m.r64(m.env.data_va(85877)) as u128))).to_bits());
            // 645b7 cvttsd2si eax,xmm8
            rax = (cvtt32(f64_of(x8)) as u64) as u32 as u64;
            // 645bc mov WORD PTR [rsi+0x12],ax
            m.w16(rsi.wrapping_add(0x12), ((rax as u16 as u64)) as u16);
            // 645c0 mov ax,WORD PTR [rdi+0x20]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x20)) as u64) as u16 as u64);
            // 645c4 mov WORD PTR [rsi+0x10],ax
            m.w16(rsi.wrapping_add(0x10), ((rax as u16 as u64)) as u16);
            // 645c8 mov ax,WORD PTR [rdi+0x28]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x28)) as u64) as u16 as u64);
            // 645cc mov WORD PTR [rsi+0x14],ax
            m.w16(rsi.wrapping_add(0x14), ((rax as u16 as u64)) as u16);
            // 645d0 mov ax,WORD PTR [rdi+0x2a]
            rax = (rax & !0xffff) | ((m.r16(rdi.wrapping_add(0x2a)) as u64) as u16 as u64);
            // 645d4 mov WORD PTR [rsi+0x16],ax
            m.w16(rsi.wrapping_add(0x16), ((rax as u16 as u64)) as u16);
            // 645d8 mov al,BYTE PTR [rdi+0x2c]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x2c)) as u64) as u8 as u64);
            // 645db mov BYTE PTR [rsi+0x18],al
            m.w8(rsi.wrapping_add(0x18), ((rax as u8 as u64)) as u8);
            // 645de mov al,BYTE PTR [rdi+0x2d]
            rax = (rax & !0xff) | ((m.r8(rdi.wrapping_add(0x2d)) as u64) as u8 as u64);
            // 645e1 mov BYTE PTR [rsi+0x19],al
            m.w8(rsi.wrapping_add(0x19), ((rax as u8 as u64)) as u8);
            // 645e4 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 645e6 jmp 0x6442b
            pc = 0x6442b; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x2bb44 size=126
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0002bb44<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x2bb44;
    loop {
        match pc {
        0x2bb44 => {
            // block 2bb44
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb44 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 2bb45 mov rbp,rsp
            rbp = rsp;
            // 2bb48 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 2bb4a push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 2bb4c push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 2bb4e push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2bb4f mov r12,rsi
            r12 = rsi;
            // 2bb52 mov r14,rdi
            r14 = rdi;
            // 2bb55 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 2bb5a mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 2bb5f mov rdi,r12
            rdi = r12;
            // 2bb62 call 0xebcd2
            { let r = { let t = m.env.stub_va(0x49f); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bb67 mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 2bb70 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 2bb74 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 2bb7d mov r15,QWORD PTR [r14+0x50]
            r15 = m.r64(r14.wrapping_add(0x50)) as u64;
            // 2bb81 test r15,r15
            fl = logic::<64>(r15 & r15);
            // 2bb84 je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb86;
        }
        0x2bb86 => {
            // block 2bb86
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb86 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb86 cmp DWORD PTR [r14+0x58],0x0
            fl = sub::<32>(m.r32(r14.wrapping_add(0x58)) as u64, 0x0u64).1;
            // 2bb8b je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb8d;
        }
        0x2bb8d => {
            // block 2bb8d
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb8d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb8d mov eax,DWORD PTR [r14+0x4c]
            rax = (m.r32(r14.wrapping_add(0x4c)) as u64) as u32 as u64;
            // 2bb91 test eax,eax
            fl = logic::<32>((rax as u32 as u64) & (rax as u32 as u64));
            // 2bb93 je 0x2bb9b
            if fl.e() { pc = 0x2bb9b; continue; }
            pc = 0x2bb95;
        }
        0x2bb95 => {
            // block 2bb95
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb95 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb95 cmp eax,DWORD PTR [r14+0x20]
            fl = sub::<32>((rax as u32 as u64), m.r32(r14.wrapping_add(0x20)) as u64).1;
            // 2bb99 jbe 0x2bba6
            if fl.be() { pc = 0x2bba6; continue; }
            pc = 0x2bb9b;
        }
        0x2bb9b => {
            // block 2bb9b
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb9b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb9b xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bb9d;
        }
        0x2bb9d => {
            // block 2bb9d
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bb9d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bb9d pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 2bb9e pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 2bba0 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 2bba2 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 2bba4 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 2bba5 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
        0x2bba6 => {
            // block 2bba6
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bba6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bba6 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            pc = 0x2bba8;
        }
        0x2bba8 => {
            // block 2bba8
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bba8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bba8 mov esi,DWORD PTR [r15+rbx*4]
            rsi = (m.r32(r15.wrapping_add(rbx.wrapping_mul(4))) as u64) as u32 as u64;
            // 2bbac mov rdi,r12
            rdi = r12;
            // 2bbaf call 0x60955
            { let r = { let t = m.env.code_va(0x60955); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 2bbb4 inc rbx
            { let (r, f) = inc::<64>(rbx, fl); rbx = r; fl = f; }
            // 2bbb7 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 2bbb9 cmp ebx,DWORD PTR [r14+0x58]
            fl = sub::<32>((rbx as u32 as u64), m.r32(r14.wrapping_add(0x58)) as u64).1;
            // 2bbbd jb 0x2bba8
            if fl.b() { pc = 0x2bba8; continue; }
            pc = 0x2bbbf;
        }
        0x2bbbf => {
            // block 2bbbf
            #[cfg(feature = "trace")] eprintln!("f_0002bb44 pc 2bbbf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 2bbbf jmp 0x2bb9d
            pc = 0x2bb9d; continue;
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x44100 size=202
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00044100<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x44100;
    loop {
        match pc {
        0x44100 => {
            // block 44100
            #[cfg(feature = "trace")] eprintln!("f_00044100 pc 44100 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44100 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 44101 mov rbp,rsp
            rbp = rsp;
            // 44104 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 44106 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 44108 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 4410a push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 4410b sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 4410f mov r12,rdx
            r12 = rdx;
            // 44112 mov r15d,esi
            r15 = ((rsi as u32 as u64)) as u32 as u64;
            // 44115 mov r14,rdi
            r14 = rdi;
            // 44118 lea rbx,[r12+0x8]
            rbx = r12.wrapping_add(0x8);
            // 4411d mov rsi,QWORD PTR [r12+0x8]
            rsi = m.r64(r12.wrapping_add(0x8)) as u64;
            // 44122 mov rdi,r12
            rdi = r12;
            // 44125 call 0xebb6a
            { let r = { let t = m.env.stub_va(0x337); m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 4412a mov QWORD PTR [r12+0x10],0x0
            m.w64(r12.wrapping_add(0x10), (0x0u64) as u64);
            // 44133 mov QWORD PTR [r12],rbx
            m.w64(r12, (rbx) as u64);
            // 44137 mov QWORD PTR [r12+0x8],0x0
            m.w64(r12.wrapping_add(0x8), (0x0u64) as u64);
            // 44140 cmp r15d,0x7
            fl = sub::<32>((r15 as u32 as u64), 0x7u64).1;
            // 44144 ja 0x441ba
            if fl.a() { pc = 0x441ba; continue; }
            pc = 0x44146;
        }
        0x44146 => {
            // block 44146
            #[cfg(feature = "trace")] eprintln!("f_00044100 pc 44146 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44146 mov DWORD PTR [rbp-0x24],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc), (0x0u64) as u32);
            // 4414d mov rax,QWORD PTR [r14]
            rax = m.r64(r14) as u64;
            // 44150 mov r15d,r15d
            r15 = ((r15 as u32 as u64)) as u32 as u64;
            // 44153 lea rcx,[rip+0xbc4f6] # 0x100650
            rcx = m.env.data_va(85533);
            // 4415a mov esi,DWORD PTR [rcx+r15*4]
            rsi = (m.r32(rcx.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // 4415e lea rdx,[rbp-0x24]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_ffdc);
            // 44162 xor ebx,ebx
            { let r = (rbx as u32 as u64) ^ (rbx as u32 as u64); let f = logic::<32>(r);
              rbx = (r) as u32 as u64;
              fl = f; }
            // 44164 mov ecx,0xffffffff
            rcx = (0xffffffffu64) as u32 as u64;
            // 44169 xor r8d,r8d
            { let r = (r8 as u32 as u64) ^ (r8 as u32 as u64); let f = logic::<32>(r);
              r8 = (r) as u32 as u64;
              fl = f; }
            // 4416c mov rdi,r14
            rdi = r14;
            // 4416f call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 44172 test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 44174 mov eax,0x0
            rax = (0x0u64) as u32 as u64;
            // 44179 je 0x441bc
            if fl.e() { pc = 0x441bc; continue; }
            pc = 0x4417b;
        }
        0x4417b => {
            // block 4417b
            #[cfg(feature = "trace")] eprintln!("f_00044100 pc 4417b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 4417b lea rax,[rip+0xbc4ee] # 0x100670
            rax = m.env.data_va(85565);
            // 44182 mov r15d,DWORD PTR [rax+r15*4]
            r15 = (m.r32(rax.wrapping_add(r15.wrapping_mul(4))) as u64) as u32 as u64;
            // 44186 mov DWORD PTR [rbp-0x28],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), (0x0u64) as u32);
            // 4418d shl r15d,0x3
            { let (r, f) = shl::<32>((r15 as u32 as u64), 0x3u64, fl); r15 = (r) as u32 as u64; fl = f; }
            // 44191 lea r14,[rbp-0x28]
            r14 = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            pc = 0x44195;
        }
        0x44195 => {
            // block 44195
            #[cfg(feature = "trace")] eprintln!("f_00044100 pc 44195 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 44195 mov eax,DWORD PTR [rbp-0x24]
            rax = (m.r32(rbp.wrapping_add(0xffff_ffff_ffff_ffdc)) as u64) as u32 as u64;
            // 44198 lea ecx,[r15+rbx*1]
            rcx = (r15.wrapping_add(rbx)) as u32 as u64;
            // 4419c bt eax,ecx
            fl.cf = bit::<32>((rax as u32 as u64), (rcx as u32 as u64));
            // 4419f jae 0x441ac
            if fl.ae() { pc = 0x441ac; continue; }
            pc = 0x441a1;
        }
        0x441a1 => {
            // block 441a1
            #[cfg(feature = "trace")] eprintln!("f_00044100 pc 441a1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 441a1 mov rdi,r12
            rdi = r12;
            // 441a4 mov rsi,r14
            rsi = r14;
            // 441a7 call 0xebb52
            { let r = { let t = m.env.stub_va(0x31f); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            pc = 0x441ac;
        }
        0x441ac => {
            // block 441ac
            #[cfg(feature = "trace")] eprintln!("f_00044100 pc 441ac rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 441ac inc ebx
            { let (r, f) = inc::<32>((rbx as u32 as u64), fl); rbx = (r) as u32 as u64; fl = f; }
            // 441ae mov DWORD PTR [rbp-0x28],ebx
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_ffd8), ((rbx as u32 as u64)) as u32);
            // 441b1 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 441b3 cmp ebx,0x8
            fl = sub::<32>((rbx as u32 as u64), 0x8u64).1;
            // 441b6 jb 0x44195
            if fl.b() { pc = 0x44195; continue; }
            pc = 0x441b8;
        }
        0x441b8 => {
            // block 441b8
            #[cfg(feature = "trace")] eprintln!("f_00044100 pc 441b8 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 441b8 jmp 0x441bc
            pc = 0x441bc; continue;
        }
        0x441ba => {
            // block 441ba
            #[cfg(feature = "trace")] eprintln!("f_00044100 pc 441ba rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 441ba xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x441bc;
        }
        0x441bc => {
            // block 441bc
            #[cfg(feature = "trace")] eprintln!("f_00044100 pc 441bc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 441bc add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 441c0 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 441c1 pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 441c3 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 441c5 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 441c7 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 441c8 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25d00 size=98
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025d00<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x25d00;
    loop {
        match pc {
        0x25d00 => {
            // block 25d00
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d00 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d00 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25d01 mov rbp,rsp
            rbp = rsp;
            // 25d04 mov rax,QWORD PTR [rdi+0x8]
            rax = m.r64(rdi.wrapping_add(0x8)) as u64;
            // 25d08 test rax,rax
            fl = logic::<64>(rax & rax);
            // 25d0b je 0x25d3e
            if fl.e() { pc = 0x25d3e; continue; }
            pc = 0x25d0d;
        }
        0x25d0d => {
            // block 25d0d
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d0d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d0d mov ecx,DWORD PTR [rdx]
            rcx = (m.r32(rdx) as u64) as u32 as u64;
            // 25d0f jmp 0x25d14
            pc = 0x25d14; continue;
        }
        0x25d11 => {
            // block 25d11
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d11 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d11 mov rax,rdx
            rax = rdx;
            pc = 0x25d14;
        }
        0x25d14 => {
            // block 25d14
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d14 mov edx,DWORD PTR [rax+0x1c]
            rdx = (m.r32(rax.wrapping_add(0x1c)) as u64) as u32 as u64;
            // 25d17 cmp ecx,edx
            fl = sub::<32>((rcx as u32 as u64), (rdx as u32 as u64)).1;
            // 25d19 jge 0x25d25
            if fl.ge() { pc = 0x25d25; continue; }
            pc = 0x25d1b;
        }
        0x25d1b => {
            // block 25d1b
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d1b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d1b mov rdx,QWORD PTR [rax]
            rdx = m.r64(rax) as u64;
            // 25d1e test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 25d21 jne 0x25d11
            if fl.ne() { pc = 0x25d11; continue; }
            pc = 0x25d23;
        }
        0x25d23 => {
            // block 25d23
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d23 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d23 jmp 0x25d4a
            pc = 0x25d4a; continue;
        }
        0x25d25 => {
            // block 25d25
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d25 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d25 cmp edx,ecx
            fl = sub::<32>((rdx as u32 as u64), (rcx as u32 as u64)).1;
            // 25d27 jge 0x25d52
            if fl.ge() { pc = 0x25d52; continue; }
            pc = 0x25d29;
        }
        0x25d29 => {
            // block 25d29
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d29 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d29 mov rdx,QWORD PTR [rax+0x8]
            rdx = m.r64(rax.wrapping_add(0x8)) as u64;
            // 25d2d test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // 25d30 jne 0x25d11
            if fl.ne() { pc = 0x25d11; continue; }
            pc = 0x25d32;
        }
        0x25d32 => {
            // block 25d32
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d32 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 25d35 add rax,0x8
            { let (r, f) = add::<64>(rax, 0x8u64);
              rax = r;
              fl = f; }
            // 25d39 mov rsi,rax
            rsi = rax;
            // 25d3c jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d3e => {
            // block 25d3e
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d3e add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // 25d42 mov QWORD PTR [rsi],rdi
            m.w64(rsi, (rdi) as u64);
            // 25d45 mov rsi,rdi
            rsi = rdi;
            // 25d48 jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d4a => {
            // block 25d4a
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d4a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d4a mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            // 25d4d mov rsi,rax
            rsi = rax;
            // 25d50 jmp 0x25d55
            pc = 0x25d55; continue;
        }
        0x25d52 => {
            // block 25d52
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d52 mov QWORD PTR [rsi],rax
            m.w64(rsi, (rax) as u64);
            pc = 0x25d55;
        }
        0x25d55 => {
            // block 25d55
            #[cfg(feature = "trace")] eprintln!("f_00025d00 pc 25d55 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25d55 mov rax,rsi
            rax = rsi;
            // 25d58 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25d59 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xfd3a size=254
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000fd3a<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xfd3a;
    loop {
        match pc {
        0xfd3a => {
            // block fd3a
            #[cfg(feature = "trace")] eprintln!("f_0000fd3a pc fd3a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // fd3a push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // fd3b mov rbp,rsp
            rbp = rsp;
            // fd3e push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // fd40 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // fd41 mov r14,rsi
            r14 = rsi;
            // fd44 mov rbx,rdi
            rbx = rdi;
            // fd47 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd4a movzx esi,WORD PTR [r14]
            rsi = (m.r16(r14) as u64) as u32 as u64;
            // fd4e call QWORD PTR [rax+0x1c08]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c08)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd54 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd57 movzx esi,WORD PTR [r14+0x2]
            rsi = (m.r16(r14.wrapping_add(0x2)) as u64) as u32 as u64;
            // fd5c mov rdi,rbx
            rdi = rbx;
            // fd5f call QWORD PTR [rax+0x1c18]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c18)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd65 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd68 movzx esi,WORD PTR [r14+0x4]
            rsi = (m.r16(r14.wrapping_add(0x4)) as u64) as u32 as u64;
            // fd6d mov rdi,rbx
            rdi = rbx;
            // fd70 call QWORD PTR [rax+0x1c28]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c28)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd76 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd79 movzx esi,WORD PTR [r14+0x6]
            rsi = (m.r16(r14.wrapping_add(0x6)) as u64) as u32 as u64;
            // fd7e mov rdi,rbx
            rdi = rbx;
            // fd81 call QWORD PTR [rax+0x1c38]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c38)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd87 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd8a movzx esi,WORD PTR [r14+0x8]
            rsi = (m.r16(r14.wrapping_add(0x8)) as u64) as u32 as u64;
            // fd8f mov rdi,rbx
            rdi = rbx;
            // fd92 call QWORD PTR [rax+0x1c48]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c48)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fd98 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fd9b movzx esi,WORD PTR [r14+0xa]
            rsi = (m.r16(r14.wrapping_add(0xa)) as u64) as u32 as u64;
            // fda0 mov rdi,rbx
            rdi = rbx;
            // fda3 call QWORD PTR [rax+0x1c58]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c58)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fda9 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fdac movzx esi,WORD PTR [r14+0xc]
            rsi = (m.r16(r14.wrapping_add(0xc)) as u64) as u32 as u64;
            // fdb1 mov rdi,rbx
            rdi = rbx;
            // fdb4 call QWORD PTR [rax+0x1c68]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c68)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fdba mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fdbd movzx esi,WORD PTR [r14+0xe]
            rsi = (m.r16(r14.wrapping_add(0xe)) as u64) as u32 as u64;
            // fdc2 mov rdi,rbx
            rdi = rbx;
            // fdc5 call QWORD PTR [rax+0x1c78]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c78)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fdcb mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fdce movzx esi,WORD PTR [r14+0x10]
            rsi = (m.r16(r14.wrapping_add(0x10)) as u64) as u32 as u64;
            // fdd3 mov rdi,rbx
            rdi = rbx;
            // fdd6 call QWORD PTR [rax+0x1c88]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c88)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fddc mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fddf movzx esi,WORD PTR [r14+0x12]
            rsi = (m.r16(r14.wrapping_add(0x12)) as u64) as u32 as u64;
            // fde4 mov rdi,rbx
            rdi = rbx;
            // fde7 call QWORD PTR [rax+0x1c98]
            { let r = { let t = m.r64(rax.wrapping_add(0x1c98)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fded mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fdf0 movzx esi,WORD PTR [r14+0x14]
            rsi = (m.r16(r14.wrapping_add(0x14)) as u64) as u32 as u64;
            // fdf5 mov rdi,rbx
            rdi = rbx;
            // fdf8 call QWORD PTR [rax+0x1ca8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1ca8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fdfe mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fe01 movzx esi,WORD PTR [r14+0x16]
            rsi = (m.r16(r14.wrapping_add(0x16)) as u64) as u32 as u64;
            // fe06 mov rdi,rbx
            rdi = rbx;
            // fe09 call QWORD PTR [rax+0x1cb8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1cb8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fe0f mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fe12 movzx esi,BYTE PTR [r14+0x18]
            rsi = (m.r8(r14.wrapping_add(0x18)) as u64) as u32 as u64;
            // fe17 mov rdi,rbx
            rdi = rbx;
            // fe1a call QWORD PTR [rax+0x1cc8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1cc8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fe20 mov rax,QWORD PTR [rbx]
            rax = m.r64(rbx) as u64;
            // fe23 movzx esi,BYTE PTR [r14+0x19]
            rsi = (m.r8(r14.wrapping_add(0x19)) as u64) as u32 as u64;
            // fe28 mov rdi,rbx
            rdi = rbx;
            // fe2b call QWORD PTR [rax+0x1cd8]
            { let r = { let t = m.r64(rax.wrapping_add(0x1cd8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // fe31 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // fe33 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // fe34 pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // fe36 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // fe37 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x298c size=74
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_0000298c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x298c;
    loop {
        match pc {
        0x298c => {
            // block 298c
            #[cfg(feature = "trace")] eprintln!("f_0000298c pc 298c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 298c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 298d mov rbp,rsp
            rbp = rsp;
            // 2990 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 2991 push rax
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rax);
            // 2992 mov rbx,rdx
            rbx = rdx;
            // 2995 mov DWORD PTR [rbp-0xc],0x0
            m.w32(rbp.wrapping_add(0xffff_ffff_ffff_fff4), (0x0u64) as u32);
            // 299c mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 299f mov ecx,esi
            rcx = ((rsi as u32 as u64)) as u32 as u64;
            // 29a1 lea rdx,[rip+0xfc6d8] # 0xff080
            rdx = m.env.data_va(79949);
            // 29a8 mov esi,DWORD PTR [rdx+rcx*4]
            rsi = (m.r32(rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u32 as u64;
            // 29ab lea rdx,[rbp-0xc]
            rdx = rbp.wrapping_add(0xffff_ffff_ffff_fff4);
            // 29af mov ecx,0x80
            rcx = (0x80u64) as u32 as u64;
            // 29b4 mov r8d,0x7
            r8 = (0x7u64) as u32 as u64;
            // 29ba call QWORD PTR [rax+0x38]
            { let r = { let t = m.r64(rax.wrapping_add(0x38)) as u64; m.call(t, &[rdi, rsi, rdx, rcx, r8], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 29bd test al,al
            fl = logic::<8>((rax as u8 as u64) & (rax as u8 as u64));
            // 29bf je 0x29cc
            if fl.e() { pc = 0x29cc; continue; }
            pc = 0x29c1;
        }
        0x29c1 => {
            // block 29c1
            #[cfg(feature = "trace")] eprintln!("f_0000298c pc 29c1 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29c1 cmp DWORD PTR [rbp-0xc],0x0
            fl = sub::<32>(m.r32(rbp.wrapping_add(0xffff_ffff_ffff_fff4)) as u64, 0x0u64).1;
            // 29c5 sete BYTE PTR [rbx]
            m.w8(rbx, ((fl.e() as u64)) as u8);
            // 29c8 mov al,0x1
            rax = (rax & !0xff) | ((0x1u64) as u8 as u64);
            // 29ca jmp 0x29ce
            pc = 0x29ce; continue;
        }
        0x29cc => {
            // block 29cc
            #[cfg(feature = "trace")] eprintln!("f_0000298c pc 29cc rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29cc xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0x29ce;
        }
        0x29ce => {
            // block 29ce
            #[cfg(feature = "trace")] eprintln!("f_0000298c pc 29ce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 29ce add rsp,0x8
            { let (r, f) = add::<64>(rsp, 0x8u64);
              rsp = r;
              fl = f; }
            // 29d2 pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 29d3 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 29d4 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x25b4c size=104
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00025b4c<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x25b4c;
    loop {
        match pc {
        0x25b4c => {
            // block 25b4c
            #[cfg(feature = "trace")] eprintln!("f_00025b4c pc 25b4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25b4c push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 25b4d mov rbp,rsp
            rbp = rsp;
            // 25b50 push r15
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r15);
            // 25b52 push r14
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r14);
            // 25b54 push r12
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, r12);
            // 25b56 push rbx
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbx);
            // 25b57 sub rsp,0x10
            { let (r, f) = sub::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 25b5b mov r15,rsi
            r15 = rsi;
            // 25b5e mov r14,rdi
            r14 = rdi;
            // 25b61 lea rsi,[rbp-0x28]
            rsi = rbp.wrapping_add(0xffff_ffff_ffff_ffd8);
            // 25b65 mov rdx,r15
            rdx = r15;
            // 25b68 call 0xeb9ae
            { let r = { let t = m.env.stub_va(0x17b); m.call(t, &[rdi, rsi, rdx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25b6d mov r12,rax
            r12 = rax;
            // 25b70 mov rbx,QWORD PTR [r12]
            rbx = m.r64(r12) as u64;
            // 25b74 test rbx,rbx
            fl = logic::<64>(rbx & rbx);
            // 25b77 je 0x25b7d
            if fl.e() { pc = 0x25b7d; continue; }
            pc = 0x25b79;
        }
        0x25b79 => {
            // block 25b79
            #[cfg(feature = "trace")] eprintln!("f_00025b4c pc 25b79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25b79 xor edx,edx
            { let r = (rdx as u32 as u64) ^ (rdx as u32 as u64); let f = logic::<32>(r);
              rdx = (r) as u32 as u64;
              fl = f; }
            // 25b7b jmp 0x25ba4
            pc = 0x25ba4; continue;
        }
        0x25b7d => {
            // block 25b7d
            #[cfg(feature = "trace")] eprintln!("f_00025b4c pc 25b7d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25b7d mov edi,0x20
            rdi = (0x20u64) as u32 as u64;
            // 25b82 call 0xec146
            { let r = { let t = m.env.stub_va(0x913); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25b87 mov rbx,rax
            rbx = rax;
            // 25b8a mov eax,DWORD PTR [r15]
            rax = (m.r32(r15) as u64) as u32 as u64;
            // 25b8d mov DWORD PTR [rbx+0x1c],eax
            m.w32(rbx.wrapping_add(0x1c), ((rax as u32 as u64)) as u32);
            // 25b90 mov rsi,QWORD PTR [rbp-0x28]
            rsi = m.r64(rbp.wrapping_add(0xffff_ffff_ffff_ffd8)) as u64;
            // 25b94 mov rdi,r14
            rdi = r14;
            // 25b97 mov rdx,r12
            rdx = r12;
            // 25b9a mov rcx,rbx
            rcx = rbx;
            // 25b9d call 0xeb9ba
            { let r = { let t = m.env.stub_va(0x187); m.call(t, &[rdi, rsi, rdx, rcx], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 25ba2 mov dl,0x1
            rdx = (rdx & !0xff) | ((0x1u64) as u8 as u64);
            pc = 0x25ba4;
        }
        0x25ba4 => {
            // block 25ba4
            #[cfg(feature = "trace")] eprintln!("f_00025b4c pc 25ba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 25ba4 mov rax,rbx
            rax = rbx;
            // 25ba7 add rsp,0x10
            { let (r, f) = add::<64>(rsp, 0x10u64);
              rsp = r;
              fl = f; }
            // 25bab pop rbx
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbx = v; }
            // 25bac pop r12
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r12 = v; }
            // 25bae pop r14
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r14 = v; }
            // 25bb0 pop r15
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); r15 = v; }
            // 25bb2 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 25bb3 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x55a04 size=16
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_00055a04<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x55a04;
    loop {
        match pc {
        0x55a04 => {
            // block 55a04
            #[cfg(feature = "trace")] eprintln!("f_00055a04 pc 55a04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 55a04 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 55a05 mov rbp,rsp
            rbp = rsp;
            // 55a08 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // 55a0b mov rax,QWORD PTR [rax+0x38]
            rax = m.r64(rax.wrapping_add(0x38)) as u64;
            // 55a0f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // 55a10 jmp rax
            { let r = { let t = rax; m.call(t, &[rdi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0x10330 size=24
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env). noreturn calls: stub:0x8fb
pub fn f_00010330<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0x10330;
    loop {
        match pc {
        0x10330 => {
            // block 10330
            #[cfg(feature = "trace")] eprintln!("f_00010330 pc 10330 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // 10330 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // 10331 mov rbp,rsp
            rbp = rsp;
            // 10334 call 0xec158
            { let r = { let t = m.env.stub_va(0x925); m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // 10339 call 0xec12e
            { let _ = { let t = m.env.stub_va(0x8fb); m.call(t, &[], &[]) }; return Ret::default(); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
