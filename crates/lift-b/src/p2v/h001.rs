//! Mechanically lifted functions (scripts/lift_run.py), module h001. Each one is checked against the original
//! in the world harness (tests/world_h001.rs).
#![allow(unused_mut, unused_variables, unused_assignments, unused_parens, unreachable_code, unused_imports)]
#![allow(clippy::all)]
use crate::x86rt::*;
use neutral_world::Env;

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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7b18 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7b41 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7bc6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7bd6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7bd6 mov r12d,DWORD PTR [r13+0x368]
            r12 = (m.r32(r13.wrapping_add(0x368)) as u64) as u32 as u64;
            // d7bdd jmp 0xd7be6
            pc = 0xd7be6; continue;
        }
        0xd7bdf => {
            // block d7bdf
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7bdf rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7bdf mov r12d,DWORD PTR [r13+0x35c]
            r12 = (m.r32(r13.wrapping_add(0x35c)) as u64) as u32 as u64;
            pc = 0xd7be6;
        }
        0xd7be6 => {
            // block d7be6
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7be6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c05 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c15 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7c15 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // d7c19 lea rax,[r13+0x344]
            rax = r13.wrapping_add(0x344);
            // d7c20 jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7c22 => {
            // block d7c22
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7c22 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // d7c26 lea rax,[r13+0x340]
            rax = r13.wrapping_add(0x340);
            pc = 0xd7c2d;
        }
        0xd7c2d => {
            // block d7c2d
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c2d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7c2d test r12d,r12d
            fl = logic::<32>((r12 as u32 as u64) & (r12 as u32 as u64));
            // d7c30 je 0xd7d02
            if fl.e() { pc = 0xd7d02; continue; }
            pc = 0xd7c36;
        }
        0xd7c36 => {
            // block d7c36
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c43 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c50 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c6d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7c6d mov esi,0x3
            rsi = (0x3u64) as u32 as u64;
            // d7c72 jmp 0xd7c79
            pc = 0xd7c79; continue;
        }
        0xd7c74 => {
            // block d7c74
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7c74 mov esi,0x1
            rsi = (0x1u64) as u32 as u64;
            pc = 0xd7c79;
        }
        0xd7c79 => {
            // block d7c79
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c79 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7c84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7ccb rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7cf3 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d26 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d4c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d4c mov r12d,DWORD PTR [r13+0x360]
            r12 = (m.r32(r13.wrapping_add(0x360)) as u64) as u32 as u64;
            // d7d53 jmp 0xd7be6
            pc = 0xd7be6; continue;
        }
        0xd7d58 => {
            // block d7d58
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d58 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d58 mov r12d,DWORD PTR [r13+0x364]
            r12 = (m.r32(r13.wrapping_add(0x364)) as u64) as u32 as u64;
            // d7d5f jmp 0xd7be6
            pc = 0xd7be6; continue;
        }
        0xd7d64 => {
            // block d7d64
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d64 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            pc = 0xd7d68;
        }
        0xd7d68 => {
            // block d7d68
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d68 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d68 lea rax,[r13+0x354]
            rax = r13.wrapping_add(0x354);
            // d7d6f jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7d74 => {
            // block d7d74
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d74 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            pc = 0xd7d78;
        }
        0xd7d78 => {
            // block d7d78
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d78 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d78 lea rax,[r13+0x358]
            rax = r13.wrapping_add(0x358);
            // d7d7f jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7d84 => {
            // block d7d84
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d84 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            pc = 0xd7d88;
        }
        0xd7d88 => {
            // block d7d88
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d88 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d88 lea rax,[r13+0x348]
            rax = r13.wrapping_add(0x348);
            // d7d8f jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7d94 => {
            // block d7d94
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7d94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7d94 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // d7d98 lea rax,[r13+0x34c]
            rax = r13.wrapping_add(0x34c);
            // d7d9f jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7da4 => {
            // block d7da4
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7da4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // d7da4 mov QWORD PTR [rbp-0x38],r14
            m.w64(rbp.wrapping_add(0xffff_ffff_ffff_ffc8), (r14) as u64);
            // d7da8 lea rax,[r13+0x350]
            rax = r13.wrapping_add(0x350);
            // d7daf jmp 0xd7c2d
            pc = 0xd7c2d; continue;
        }
        0xd7db4 => {
            // block d7db4
            #[cfg(feature = "trace")] eprintln!("f_000d7b18 pc d7db4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9d3e pc b9d3e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9d3e pc b9d47 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9d47 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9d4a call QWORD PTR [rax+0x13c8]
            { let r = { let t = m.r64(rax.wrapping_add(0x13c8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9d50 jmp 0xb9d54
            pc = 0xb9d54; continue;
        }
        0xb9d52 => {
            // block b9d52
            #[cfg(feature = "trace")] eprintln!("f_000b9d3e pc b9d52 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9d52 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9d54;
        }
        0xb9d54 => {
            // block b9d54
            #[cfg(feature = "trace")] eprintln!("f_000b9d3e pc b9d54 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9d6e pc b9d6e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9d6e pc b9d77 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9d77 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9d7a call QWORD PTR [rax+0x13e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x13e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9d80 jmp 0xb9d84
            pc = 0xb9d84; continue;
        }
        0xb9d82 => {
            // block b9d82
            #[cfg(feature = "trace")] eprintln!("f_000b9d6e pc b9d82 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9d82 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9d84;
        }
        0xb9d84 => {
            // block b9d84
            #[cfg(feature = "trace")] eprintln!("f_000b9d6e pc b9d84 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9d9e pc b9d9e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9d9e pc b9da7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9da7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9daa call QWORD PTR [rax+0x1458]
            { let r = { let t = m.r64(rax.wrapping_add(0x1458)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9db0 jmp 0xb9db4
            pc = 0xb9db4; continue;
        }
        0xb9db2 => {
            // block b9db2
            #[cfg(feature = "trace")] eprintln!("f_000b9d9e pc b9db2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9db2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9db4;
        }
        0xb9db4 => {
            // block b9db4
            #[cfg(feature = "trace")] eprintln!("f_000b9d9e pc b9db4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9dce pc b9dce rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9dce pc b9dd7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9dd7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9dda call QWORD PTR [rax+0x1488]
            { let r = { let t = m.r64(rax.wrapping_add(0x1488)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9de0 jmp 0xb9de4
            pc = 0xb9de4; continue;
        }
        0xb9de2 => {
            // block b9de2
            #[cfg(feature = "trace")] eprintln!("f_000b9dce pc b9de2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9de2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9de4;
        }
        0xb9de4 => {
            // block b9de4
            #[cfg(feature = "trace")] eprintln!("f_000b9dce pc b9de4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9dfe pc b9dfe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9dfe pc b9e07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9e07 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9e0a call QWORD PTR [rax+0x14e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x14e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9e10 jmp 0xb9e14
            pc = 0xb9e14; continue;
        }
        0xb9e12 => {
            // block b9e12
            #[cfg(feature = "trace")] eprintln!("f_000b9dfe pc b9e12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9e12 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9e14;
        }
        0xb9e14 => {
            // block b9e14
            #[cfg(feature = "trace")] eprintln!("f_000b9dfe pc b9e14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9e2e pc b9e2e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9e2e pc b9e37 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9e37 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9e3a call QWORD PTR [rax+0x1510]
            { let r = { let t = m.r64(rax.wrapping_add(0x1510)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9e40 jmp 0xb9e44
            pc = 0xb9e44; continue;
        }
        0xb9e42 => {
            // block b9e42
            #[cfg(feature = "trace")] eprintln!("f_000b9e2e pc b9e42 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9e42 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9e44;
        }
        0xb9e44 => {
            // block b9e44
            #[cfg(feature = "trace")] eprintln!("f_000b9e2e pc b9e44 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9e5e pc b9e5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9e5e pc b9e67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9e67 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9e6a call QWORD PTR [rax+0x1570]
            { let r = { let t = m.r64(rax.wrapping_add(0x1570)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9e70 jmp 0xb9e74
            pc = 0xb9e74; continue;
        }
        0xb9e72 => {
            // block b9e72
            #[cfg(feature = "trace")] eprintln!("f_000b9e5e pc b9e72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9e72 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9e74;
        }
        0xb9e74 => {
            // block b9e74
            #[cfg(feature = "trace")] eprintln!("f_000b9e5e pc b9e74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9e8e pc b9e8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9e8e pc b9e97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9e97 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9e9a call QWORD PTR [rax+0x15a0]
            { let r = { let t = m.r64(rax.wrapping_add(0x15a0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9ea0 jmp 0xb9ea4
            pc = 0xb9ea4; continue;
        }
        0xb9ea2 => {
            // block b9ea2
            #[cfg(feature = "trace")] eprintln!("f_000b9e8e pc b9ea2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9ea2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9ea4;
        }
        0xb9ea4 => {
            // block b9ea4
            #[cfg(feature = "trace")] eprintln!("f_000b9e8e pc b9ea4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9ebe pc b9ebe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9ebe pc b9ec7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9ec7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9eca call QWORD PTR [rax+0x15d0]
            { let r = { let t = m.r64(rax.wrapping_add(0x15d0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9ed0 jmp 0xb9ed4
            pc = 0xb9ed4; continue;
        }
        0xb9ed2 => {
            // block b9ed2
            #[cfg(feature = "trace")] eprintln!("f_000b9ebe pc b9ed2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9ed2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9ed4;
        }
        0xb9ed4 => {
            // block b9ed4
            #[cfg(feature = "trace")] eprintln!("f_000b9ebe pc b9ed4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9eee pc b9eee rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9eee pc b9ef7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9ef7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9efa call QWORD PTR [rax+0x1600]
            { let r = { let t = m.r64(rax.wrapping_add(0x1600)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9f00 jmp 0xb9f04
            pc = 0xb9f04; continue;
        }
        0xb9f02 => {
            // block b9f02
            #[cfg(feature = "trace")] eprintln!("f_000b9eee pc b9f02 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9f02 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9f04;
        }
        0xb9f04 => {
            // block b9f04
            #[cfg(feature = "trace")] eprintln!("f_000b9eee pc b9f04 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9f1e pc b9f1e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9f1e pc b9f27 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9f27 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9f2a call QWORD PTR [rax+0x1630]
            { let r = { let t = m.r64(rax.wrapping_add(0x1630)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9f30 jmp 0xb9f34
            pc = 0xb9f34; continue;
        }
        0xb9f32 => {
            // block b9f32
            #[cfg(feature = "trace")] eprintln!("f_000b9f1e pc b9f32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9f32 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9f34;
        }
        0xb9f34 => {
            // block b9f34
            #[cfg(feature = "trace")] eprintln!("f_000b9f1e pc b9f34 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9fae pc b9fae rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9fae pc b9fb7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9fb7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9fba call QWORD PTR [rax+0x1760]
            { let r = { let t = m.r64(rax.wrapping_add(0x1760)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9fc0 jmp 0xb9fc4
            pc = 0xb9fc4; continue;
        }
        0xb9fc2 => {
            // block b9fc2
            #[cfg(feature = "trace")] eprintln!("f_000b9fae pc b9fc2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9fc2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9fc4;
        }
        0xb9fc4 => {
            // block b9fc4
            #[cfg(feature = "trace")] eprintln!("f_000b9fae pc b9fc4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9fde pc b9fde rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9fde pc b9fe7 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9fe7 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9fea call QWORD PTR [rax+0x1790]
            { let r = { let t = m.r64(rax.wrapping_add(0x1790)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9ff0 jmp 0xb9ff4
            pc = 0xb9ff4; continue;
        }
        0xb9ff2 => {
            // block b9ff2
            #[cfg(feature = "trace")] eprintln!("f_000b9fde pc b9ff2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9ff2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9ff4;
        }
        0xb9ff4 => {
            // block b9ff4
            #[cfg(feature = "trace")] eprintln!("f_000b9fde pc b9ff4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9710 pc b9710 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9710 pc b9719 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9719 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b971c call QWORD PTR [rax+0x370]
            { let r = { let t = m.r64(rax.wrapping_add(0x370)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9722 jmp 0xb9726
            pc = 0xb9726; continue;
        }
        0xb9724 => {
            // block b9724
            #[cfg(feature = "trace")] eprintln!("f_000b9710 pc b9724 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9724 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9726;
        }
        0xb9726 => {
            // block b9726
            #[cfg(feature = "trace")] eprintln!("f_000b9710 pc b9726 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9740 pc b9740 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9740 pc b9749 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9749 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b974c call QWORD PTR [rax+0x390]
            { let r = { let t = m.r64(rax.wrapping_add(0x390)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9752 jmp 0xb9756
            pc = 0xb9756; continue;
        }
        0xb9754 => {
            // block b9754
            #[cfg(feature = "trace")] eprintln!("f_000b9740 pc b9754 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9754 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9756;
        }
        0xb9756 => {
            // block b9756
            #[cfg(feature = "trace")] eprintln!("f_000b9740 pc b9756 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9860 pc b9860 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9860 pc b9869 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9869 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b986c call QWORD PTR [rax+0x4b8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4b8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9872 jmp 0xb9876
            pc = 0xb9876; continue;
        }
        0xb9874 => {
            // block b9874
            #[cfg(feature = "trace")] eprintln!("f_000b9860 pc b9874 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9874 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9876;
        }
        0xb9876 => {
            // block b9876
            #[cfg(feature = "trace")] eprintln!("f_000b9860 pc b9876 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9890 pc b9890 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9890 pc b9899 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9899 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b989c call QWORD PTR [rax+0x4e8]
            { let r = { let t = m.r64(rax.wrapping_add(0x4e8)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b98a2 jmp 0xb98a6
            pc = 0xb98a6; continue;
        }
        0xb98a4 => {
            // block b98a4
            #[cfg(feature = "trace")] eprintln!("f_000b9890 pc b98a4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b98a4 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb98a6;
        }
        0xb98a6 => {
            // block b98a6
            #[cfg(feature = "trace")] eprintln!("f_000b9890 pc b98a6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b98f0 pc b98f0 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b98f0 pc b98f9 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b98f9 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b98fc call QWORD PTR [rax+0x548]
            { let r = { let t = m.r64(rax.wrapping_add(0x548)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9902 jmp 0xb9906
            pc = 0xb9906; continue;
        }
        0xb9904 => {
            // block b9904
            #[cfg(feature = "trace")] eprintln!("f_000b98f0 pc b9904 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9904 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9906;
        }
        0xb9906 => {
            // block b9906
            #[cfg(feature = "trace")] eprintln!("f_000b98f0 pc b9906 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9980 pc b9980 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9980 pc b9989 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9989 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b998c call QWORD PTR [rax+0x608]
            { let r = { let t = m.r64(rax.wrapping_add(0x608)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9992 jmp 0xb9996
            pc = 0xb9996; continue;
        }
        0xb9994 => {
            // block b9994
            #[cfg(feature = "trace")] eprintln!("f_000b9980 pc b9994 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9994 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9996;
        }
        0xb9996 => {
            // block b9996
            #[cfg(feature = "trace")] eprintln!("f_000b9980 pc b9996 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9afe pc b9afe rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9afe pc b9b07 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9b07 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9b0a call QWORD PTR [rax+0xf48]
            { let r = { let t = m.r64(rax.wrapping_add(0xf48)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9b10 jmp 0xb9b14
            pc = 0xb9b14; continue;
        }
        0xb9b12 => {
            // block b9b12
            #[cfg(feature = "trace")] eprintln!("f_000b9afe pc b9b12 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9b12 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9b14;
        }
        0xb9b14 => {
            // block b9b14
            #[cfg(feature = "trace")] eprintln!("f_000b9afe pc b9b14 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9b5e pc b9b5e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9b5e pc b9b67 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9b67 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9b6a call QWORD PTR [rax+0xff0]
            { let r = { let t = m.r64(rax.wrapping_add(0xff0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9b70 jmp 0xb9b74
            pc = 0xb9b74; continue;
        }
        0xb9b72 => {
            // block b9b72
            #[cfg(feature = "trace")] eprintln!("f_000b9b5e pc b9b72 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9b72 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9b74;
        }
        0xb9b74 => {
            // block b9b74
            #[cfg(feature = "trace")] eprintln!("f_000b9b5e pc b9b74 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9b8e pc b9b8e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9b8e pc b9b97 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9b97 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9b9a call QWORD PTR [rax+0x1020]
            { let r = { let t = m.r64(rax.wrapping_add(0x1020)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9ba0 jmp 0xb9ba4
            pc = 0xb9ba4; continue;
        }
        0xb9ba2 => {
            // block b9ba2
            #[cfg(feature = "trace")] eprintln!("f_000b9b8e pc b9ba2 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9ba2 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9ba4;
        }
        0xb9ba4 => {
            // block b9ba4
            #[cfg(feature = "trace")] eprintln!("f_000b9b8e pc b9ba4 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9f4e pc b9f4e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9f4e pc b9f57 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9f57 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9f5a call QWORD PTR [rax+0x1660]
            { let r = { let t = m.r64(rax.wrapping_add(0x1660)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9f60 jmp 0xb9f64
            pc = 0xb9f64; continue;
        }
        0xb9f62 => {
            // block b9f62
            #[cfg(feature = "trace")] eprintln!("f_000b9f4e pc b9f62 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9f62 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9f64;
        }
        0xb9f64 => {
            // block b9f64
            #[cfg(feature = "trace")] eprintln!("f_000b9f4e pc b9f64 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9f7e pc b9f7e rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9f7e pc b9f87 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9f87 mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9f8a call QWORD PTR [rax+0x1690]
            { let r = { let t = m.r64(rax.wrapping_add(0x1690)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9f90 jmp 0xb9f94
            pc = 0xb9f94; continue;
        }
        0xb9f92 => {
            // block b9f92
            #[cfg(feature = "trace")] eprintln!("f_000b9f7e pc b9f92 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9f92 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9f94;
        }
        0xb9f94 => {
            // block b9f94
            #[cfg(feature = "trace")] eprintln!("f_000b9f7e pc b9f94 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000ba046 pc ba046 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000ba046 pc ba04f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba04f mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ba052 call QWORD PTR [rax+0x1838]
            { let r = { let t = m.r64(rax.wrapping_add(0x1838)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba058 jmp 0xba05c
            pc = 0xba05c; continue;
        }
        0xba05a => {
            // block ba05a
            #[cfg(feature = "trace")] eprintln!("f_000ba046 pc ba05a rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba05a xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xba05c;
        }
        0xba05c => {
            // block ba05c
            #[cfg(feature = "trace")] eprintln!("f_000ba046 pc ba05c rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9a22 pc b9a22 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000b9a22 pc b9a2b rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9a2b mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // b9a2e call QWORD PTR [rax+0xac0]
            { let r = { let t = m.r64(rax.wrapping_add(0xac0)) as u64; m.call(t, &[rdi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // b9a34 jmp 0xb9a38
            pc = 0xb9a38; continue;
        }
        0xb9a36 => {
            // block b9a36
            #[cfg(feature = "trace")] eprintln!("f_000b9a22 pc b9a36 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9a36 xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xb9a38;
        }
        0xb9a38 => {
            // block b9a38
            #[cfg(feature = "trace")] eprintln!("f_000b9a22 pc b9a38 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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
            #[cfg(feature = "trace")] eprintln!("f_000c3a32 pc c3a32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
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

/// parity: P2v @0xb9ff6 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000b9ff6<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xb9ff6;
    loop {
        match pc {
        0xb9ff6 => {
            // block b9ff6
            #[cfg(feature = "trace")] eprintln!("f_000b9ff6 pc b9ff6 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9ff6 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // b9ff7 mov rbp,rsp
            rbp = rsp;
            // b9ffa test rdx,rdx
            fl = logic::<64>(rdx & rdx);
            // b9ffd je 0xba00d
            if fl.e() { pc = 0xba00d; continue; }
            pc = 0xb9fff;
        }
        0xb9fff => {
            // block b9fff
            #[cfg(feature = "trace")] eprintln!("f_000b9ff6 pc b9fff rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // b9fff mov rax,QWORD PTR [rdi]
            rax = m.r64(rdi) as u64;
            // ba002 movzx esi,si
            rsi = ((rsi as u16 as u64)) as u32 as u64;
            // ba005 call QWORD PTR [rax+0x17a8]
            { let r = { let t = m.r64(rax.wrapping_add(0x17a8)) as u64; m.call(t, &[rdi, rsi], &[]) }; rax = r.rax; rdx = r.rdx; x0 = (r.xmm0_lo as u128) | ((r.xmm0_hi as u128) << 64); rcx = POISON; rsi = POISON; rdi = POISON; r8 = POISON; r9 = POISON; r10 = POISON; r11 = POISON;  }
            // ba00b jmp 0xba00f
            pc = 0xba00f; continue;
        }
        0xba00d => {
            // block ba00d
            #[cfg(feature = "trace")] eprintln!("f_000b9ff6 pc ba00d rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba00d xor eax,eax
            { let r = (rax as u32 as u64) ^ (rax as u32 as u64); let f = logic::<32>(r);
              rax = (r) as u32 as u64;
              fl = f; }
            pc = 0xba00f;
        }
        0xba00f => {
            // block ba00f
            #[cfg(feature = "trace")] eprintln!("f_000b9ff6 pc ba00f rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // ba00f pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // ba010 ret
            return Ret { rax, rdx, xmm0: x0 };
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}

/// parity: P2v @0xc3e32 size=28
/// Mechanically lifted by scripts/lift_main.py from the instruction stream (registers and flags in locals,
/// memory and calls through Env).
pub fn f_000c3e32<E: Env>(env: &mut E, ri: [u64; 6], fx: [u64; 8], st: &[u64]) -> Ret {
    let mut m = Mem::new(env, st);
    let mut rax: u64 = 0;
    let mut rcx: u64 = ri[3];
    let mut rdx: u64 = ri[2];
    let mut rbx: u64 = SAVED_RBX;
    let mut rbp: u64 = SAVED_RBP;
    let mut rsi: u64 = ri[1];
    let mut rdi: u64 = ri[0];
    let mut r8: u64 = ri[4];
    let mut r9: u64 = ri[5];
    let mut r10: u64 = 0;
    let mut r11: u64 = 0;
    let mut r12: u64 = SAVED_R12;
    let mut r13: u64 = SAVED_R13;
    let mut r14: u64 = SAVED_R14;
    let mut r15: u64 = SAVED_R15;
    let mut rsp: u64 = RSP0 - 8;
    let mut x0: u128 = fx[0] as u128;
    let mut fl = Fl::default();
    let mut pc: u32 = 0xc3e32;
    loop {
        match pc {
        0xc3e32 => {
            // block c3e32
            #[cfg(feature = "trace")] eprintln!("f_000c3e32 pc c3e32 rax {:x} rcx {:x} rdx {:x} rbx {:x} rsi {:x} rdi {:x} r12 {:x} r15 {:x}", rax, rcx, rdx, rbx, rsi, rdi, r12, r15);
            // c3e32 push rbp
            rsp = rsp.wrapping_sub(8);
            m.w64(rsp, rbp);
            // c3e33 mov rbp,rsp
            rbp = rsp;
            // c3e36 lea rax,[rip+0x8039b] # 0x1441d8
            rax = m.env.data_va(362917);
            // c3e3d mov QWORD PTR [rdi],rax
            m.w64(rdi, (rax) as u64);
            // c3e40 mov rsi,QWORD PTR [rdi+0x10]
            rsi = m.r64(rdi.wrapping_add(0x10)) as u64;
            // c3e44 add rdi,0x8
            { let (r, f) = add::<64>(rdi, 0x8u64);
              rdi = r;
              fl = f; }
            // c3e48 pop rbp
            { let v = m.r64(rsp); rsp = rsp.wrapping_add(8); rbp = v; }
            // c3e49 jmp 0xebe22
            { let r = { let t = m.env.stub_va(0x5ef); m.call(t, &[rdi, rsi], &[]) }; return Ret::from_call(r); }
        }
            _ => panic!("bad pc {:#x}", pc),
        }
    }
}
